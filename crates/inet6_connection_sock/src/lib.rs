//! IPv6 Connection Socket Implementation
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation
//! for IPv6 connection-oriented socket operations. Maintains ABI compatibility
//! with all exported symbols for direct replacement in kernel modules.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use core::ffi::c_void;
use core::ffi::c_int;
use core::ffi::c_uint;
use core::mem;
use core::ptr;
use kernel_types::*;

// Constants from C
pub const ENOMEM: c_int = -12;
pub const EINVAL: c_int = -22;

// Function declarations for external C functions
extern "C" {
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn rcu_read_lock();
    fn rcu_read_unlock();
    fn rcu_dereference<T>(ptr: *const T) -> *const T;
    fn fl6_update_dst(fl6: *mut flowi6, opt: *const c_void, final_p: *mut in6_addr) -> *mut in6_addr;
    fn ip6_dst_lookup_flow(net: *const c_void, sk: *const sock, fl6: *mut flowi6, final_p: *mut in6_addr) -> *mut dst_entry;
    fn ip6_dst_store(sk: *mut sock, dst: *mut dst_entry, _: *mut c_void, _: *mut c_void);
    fn __sk_dst_check(sk: *mut sock, cookie: u32) -> *mut dst_entry;
    fn ip6_xmit(sk: *mut sock, skb: *mut sk_buff, fl: *const flowi6, mark: c_int, opt: *const c_void, tclass: u8, priority: c_int) -> c_int;
    fn security_req_classify_flow(req: *const request_sock, fl_common: *mut c_void);
    fn security_sk_classify_flow(sk: *mut sock, fl_common: *mut c_void);
    fn sock_net(sk: *const sock) -> *const c_void;
    fn ipv6_iface_scope_id(addr: *const in6_addr, dev_if: c_int) -> u32;
}

// Function implementations
#[no_mangle]
pub unsafe extern "C" fn inet6_csk_route_req(
    sk: *const sock,
    fl6: *mut flowi6,
    req: *const request_sock,
    proto: u8,
) -> *mut dst_entry {
    let ireq = &*(req as *const inet_request_sock);
    let np = &*(sk as *const ipv6_pinfo);

    // SAFETY: Caller guarantees fl6 is valid and properly aligned
    unsafe {
        let _ = memset(fl6 as *mut c_void, 0, mem::size_of::<flowi6>() as _);
    }

    (*fl6).flowi6_proto = proto;
    (*fl6).daddr = (*ireq).ir_v6_rmt_addr;

    unsafe {
        rcu_read_lock();
        let final_p = fl6_update_dst(fl6, rcu_dereference(np.opt), &mut in6_addr { in6_u: in6_addr_union { u6_addr8: [0; 16] } });
        rcu_read_unlock();

        (*fl6).saddr = (*ireq).ir_v6_loc_addr;
        (*fl6).flowi6_oif = (*ireq).ir_iif;
        (*fl6).flowi6_mark = (*ireq).ir_mark;
        (*fl6).fl6_dport = (*ireq).ir_rmt_port;
        (*fl6).fl6_sport = (*ireq).ir_num.to_be();
        (*fl6).flowi6_uid = (*sk).sk_uid;
        security_req_classify_flow(req, &mut (*fl6) as *mut _ as *mut c_void);
    }

    let dst = ip6_dst_lookup_flow(sock_net(sk), sk, fl6, &mut in6_addr { in6_u: in6_addr_union { u6_addr8: [0; 16] } });
    if (dst as usize) < 0x00007FF000000000 {
        return ptr::null_mut();
    }

    dst
}

#[no_mangle]
pub unsafe extern "C" fn inet6_csk_addr2sockaddr(
    sk: *mut sock,
    uaddr: *mut sockaddr_in6,
) {
    let sin6 = &mut *uaddr;
    sin6.sin6_family = 10; // AF_INET6
    sin6.sin6_addr = (*sk).sk_v6_daddr;
    sin6.sin6_port = (*inet_sk(sk)).inet_dport;
    sin6.sin6_flowinfo = 0;
    sin6.sin6_scope_id = ipv6_iface_scope_id(&sin6.sin6_addr, (*sk).sk_bound_dev_if);
}

#[no_mangle]
pub unsafe extern "C" fn inet6_csk_xmit(
    sk: *mut sock,
    skb: *mut sk_buff,
    fl_unused: *mut c_void,
) -> c_int {
    let np = &*(sk as *const ipv6_pinfo);
    let mut fl6 = flowi6 {
        flowi6_proto: (*sk).sk_protocol,
        daddr: (*sk).sk_v6_daddr,
        saddr: (*np).saddr,
        flowlabel: (*np).flow_label,
        flowi6_oif: (*sk).sk_bound_dev_if,
        flowi6_mark: (*sk).sk_mark,
        fl6_sport: (*inet_sk(sk)).inet_sport,
        fl6_dport: (*inet_sk(sk)).inet_dport,
        flowi6_uid: (*sk).sk_uid,
    };

    unsafe {
        rcu_read_lock();
        let final_p = fl6_update_dst(&mut fl6, rcu_dereference(np.opt), &mut in6_addr { in6_u: in6_addr_union { u6_addr8: [0; 16] } });
        rcu_read_unlock();
    }

    let dst = inet6_csk_route_socket(sk, &mut fl6);
    if (dst as usize) < 0x00007FF000000000 {
        (*sk).sk_err_soft = -(dst as isize);
        (*sk).sk_route_caps = 0;
        ptr::null_mut::<sk_buff>();
        return -(dst as isize);
    }

    unsafe {
        skb_dst_set_noref(skb, dst);

        // Restore final destination back after routing done
        (*fl6).daddr = (*sk).sk_v6_daddr;

        let res = ip6_xmit(
            sk,
            skb,
            &fl6,
            (*sk).sk_mark,
            rcu_dereference(np.opt),
            (*np).tclass,
            (*sk).sk_priority,
        );
        rcu_read_unlock();
        res
    }
}

#[no_mangle]
pub unsafe extern "C" fn inet6_csk_update_pmtu(
    sk: *mut sock,
    mtu: u32,
) -> *mut dst_entry {
    let mut fl6 = flowi6 {
        flowi6_proto: (*sk).sk_protocol,
        daddr: (*sk).sk_v6_daddr,
        saddr: (*sk as *const ipv6_pinfo).saddr,
        flowlabel: (*sk as *const ipv6_pinfo).flow_label,
        flowi6_oif: (*sk).sk_bound_dev_if,
        flowi6_mark: (*sk).sk_mark,
        fl6_sport: (*inet_sk(sk)).inet_sport,
        fl6_dport: (*inet_sk(sk)).inet_dport,
        flowi6_uid: (*sk).sk_uid,
    };

    let dst = inet6_csk_route_socket(sk, &mut fl6);
    if (dst as usize) < 0x00007FF000000000 {
        return ptr::null_mut();
    }

    unsafe {
        (*(*dst).ops).update_pmtu.expect("update_pmtu function pointer")(dst, sk, ptr::null_mut(), mtu, 1);
    }

    let dst = inet6_csk_route_socket(sk, &mut fl6);
    if (dst as usize) < 0x00007FF000000000 {
        return ptr::null_mut();
    }

    dst
}

// Helper functions
#[inline]
unsafe fn inet_sk(sk: *mut sock) -> *mut inet_sock {
    (sk as *mut u8).offset(0) as *mut inet_sock
}

#[inline]
unsafe fn inet6_sk(sk: *mut sock) -> *mut ipv6_pinfo {
    (sk as *mut u8).offset(0) as *mut ipv6_pinfo
}

#[no_mangle]
unsafe extern "C" fn __inet6_csk_dst_check(sk: *mut sock, cookie: u32) -> *mut dst_entry {
    __sk_dst_check(sk, cookie)
}

#[no_mangle]
unsafe extern "C" fn inet6_csk_route_socket(
    sk: *mut sock,
    fl6: *mut flowi6,
) -> *mut dst_entry {
    let inet = inet_sk(sk);
    let np = inet6_sk(sk);
    let mut final_p = in6_addr { in6_u: in6_addr_union { u6_addr8: [0; 16] } };

    // SAFETY: Caller guarantees fl6 is valid and properly aligned
    unsafe {
        let _ = memset(fl6 as *mut c_void, 0, mem::size_of::<flowi6>() as _);
    }

    (*fl6).flowi6_proto = (*sk).sk_protocol;
    (*fl6).daddr = (*sk).sk_v6_daddr;
    (*fl6).saddr = (*np).saddr;
    (*fl6).flowlabel = (*np).flow_label;
    // IP6_ECN_flow_xmit - no-op in this translation
    (*fl6).flowi6_oif = (*sk).sk_bound_dev_if;
    (*fl6).flowi6_mark = (*sk).sk_mark;
    (*fl6).fl6_sport = (*inet).inet_sport;
    (*fl6).fl6_dport = (*inet).inet_dport;
    (*fl6).flowi6_uid = (*sk).sk_uid;
    security_sk_classify_flow(sk, &mut (*fl6) as *mut _ as *mut c_void);

    unsafe {
        rcu_read_lock();
        let final_p = fl6_update_dst(fl6, rcu_dereference(np.opt), &mut final_p);
        rcu_read_unlock();
    }

    let dst = __inet6_csk_dst_check(sk, (*np).dst_cookie);
    if dst.is_null() {
        let dst = ip6_dst_lookup_flow(sock_net(sk), sk, fl6, &mut final_p);

        if !dst.is_null() {
            ip6_dst_store(sk, dst, ptr::null_mut(), ptr::null_mut());
        }
        dst
    } else {
        dst
    }
}

// Exported symbols
#[no_mangle]
pub unsafe extern "C" fn inet6_csk_route_req_export() {
    // Symbol export marker
}

#[no_mangle]
pub unsafe extern "C" fn inet6_csk_addr2sockaddr_export() {
    // Symbol export marker
}

#[no_mangle]
pub unsafe extern "C" fn inet6_csk_xmit_export() {
    // Symbol export marker
}

#[no_mangle]
pub unsafe extern "C" fn inet6_csk_update_pmtu_export() {
    // Symbol export marker
}
