//! UDP Diagnostic Module for Linux Kernel
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use core::ffi::c_int;
use core::ffi::c_uint;
use core::ffi::c_void;
use core::ptr;
use core::mem;

// Constants from C
pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ENOENT: c_int = -6;
pub const AF_INET: c_int = 2;
pub const AF_INET6: c_int = 10;
pub const AF_UNSPEC: c_int = 0;

// Type definitions
#[repr(C)]
pub struct sock {
    sk_family: c_int,
    sk_refcnt: c_int,
    sk_state: c_int,
    sk_net: c_void,
    sk_wmem_alloc: c_int,
}

#[repr(C)]
pub struct sk_buff {
    sk: *mut sock,
}

#[repr(C)]
pub struct netlink_callback {
    skb: *mut sk_buff,
    args: [c_int; 2],
    data: *mut c_void,
}

#[repr(C)]
pub struct inet_diag_req_v2 {
    sdiag_family: c_int,
    idiag_states: c_int,
    id: inet_diag_msg,
}

#[repr(C)]
pub struct inet_diag_msg {
    idiag_sport: u16,
    idiag_dport: u16,
    idiag_family: c_int,
    idiag_state: c_int,
    idiag_if: c_int,
    idiag_cookie: [c_int; 2],
}

#[repr(C)]
pub struct udp_table {
    mask: c_int,
    hash: *mut udp_hslot,
}

#[repr(C)]
pub struct udp_hslot {
    head: c_void,
    lock: c_void,
}

#[repr(C)]
pub struct inet_diag_handler {
    dump: extern "C" fn(*mut sk_buff, *mut netlink_callback, *const inet_diag_req_v2),
    dump_one: extern "C" fn(*mut netlink_callback, *const inet_diag_req_v2) -> c_int,
    idiag_get_info: extern "C" fn(*mut sock, *mut inet_diag_msg, *mut c_void),
    idiag_type: c_int,
    idiag_info_size: c_int,
    destroy: Option<extern "C" fn(*mut sk_buff, *const inet_diag_req_v2) -> c_int>,
}

// Extern declarations for C functions
extern "C" {
    fn inet_diag_bc_sk(bc: *mut c_void, sk: *mut sock) -> c_int;
    fn inet_sk_diag_fill(sk: *mut sock, info: *mut c_void, skb: *mut sk_buff, 
                         cb: *mut netlink_callback, req: *const inet_diag_req_v2, 
                         flags: c_int, net_admin: c_int) -> c_int;
    fn __udp4_lib_lookup(net: *mut c_void, saddr: u32, sport: u16, 
                         daddr: u32, dport: u16, ifindex: c_int, 
                         flags: c_int, table: *mut udp_table, 
                         result: *mut *mut sock) -> *mut sock;
    fn __udp6_lib_lookup(net: *mut c_void, saddr: *mut c_void, sport: u16, 
                         daddr: *mut c_void, dport: u16, ifindex: c_int, 
                         flags: c_int, table: *mut udp_table, 
                         result: *mut *mut sock) -> *mut sock;
    fn refcount_inc_not_zero(refcnt: *mut c_int) -> c_int;
    fn sock_diag_check_cookie(sk: *mut sock, cookie: [c_int; 2]) -> c_int;
    fn nla_total_size(len: c_int) -> c_int;
    fn inet_diag_msg_attrs_size() -> c_int;
    fn netlink_unicast(nlsk: *mut c_void, skb: *mut sk_buff, portid: c_int, flags: c_int) -> c_int;
    fn sock_put(sk: *mut sock);
    fn rcu_read_lock();
    fn rcu_read_unlock();
    fn spin_lock_bh(lock: *mut c_void);
    fn spin_unlock_bh(lock: *mut c_void);
    fn inet_diag_register(handler: *mut inet_diag_handler) -> c_int;
    fn inet_diag_unregister(handler: *mut inet_diag_handler);
    fn netlink_net_capable(skb: *mut sk_buff, cap: c_int) -> c_int;
}

// Function implementations
fn sk_diag_dump(sk: *mut sock, skb: *mut sk_buff, cb: *mut netlink_callback, 
                req: *const inet_diag_req_v2, bc: *mut c_void, net_admin: c_int) -> c_int {
    unsafe {
        if inet_diag_bc_sk(bc, sk) == 0 {
            return 0;
        }
        return inet_sk_diag_fill(sk, ptr::null_mut(), skb, cb, req, 1 << 3, net_admin);
    }
}

fn udp_dump_one(tbl: *mut udp_table, cb: *mut netlink_callback, req: *const inet_diag_req_v2) -> c_int {
    unsafe {
        let in_skb = (*cb).skb;
        let mut err = 0;
        let mut sk: *mut sock = ptr::null_mut();
        let net = (*(*in_skb).sk).sk_net;

        rcu_read_lock();
        if (*req).sdiag_family == AF_INET {
            sk = __udp4_lib_lookup(net, (*req).id.idiag_src[0], (*req).id.idiag_sport,
                                  (*req).id.idiag_dst[0], (*req).id.idiag_dport,
                                  (*req).id.idiag_if, 0, tbl, ptr::null_mut());
        }
        // IPv6 handling would be added here
        rcu_read_unlock();

        if sk.is_null() {
            return -ENOENT;
        }

        err = sock_diag_check_cookie(sk, (*req).id.idiag_cookie);
        if err != 0 {
            return err;
        }

        err = -ENOMEM;
        let size = nla_total_size(mem::size_of::<inet_diag_msg>() as c_int) +
                  inet_diag_msg_attrs_size() +
                  nla_total_size(mem::size_of::<inet_diag_msg>() as c_int) + 64;
        let rep = libc::malloc(size as usize) as *mut sk_buff;
        if rep.is_null() {
            return -ENOMEM;
        }

        err = inet_sk_diag_fill(sk, ptr::null_mut(), rep, cb, req, 0,
                               netlink_net_capable(in_skb, 24 /* CAP_NET_ADMIN */));
        if err < 0 {
            libc::free(rep as *mut c_void);
            return err;
        }

        err = netlink_unicast(ptr::null_mut(), rep, (*in_skb).portid, 1 /* MSG_DONTWAIT */);
        if err > 0 {
            err = 0;
        }
        sock_put(sk);
        return err;
    }
}

fn udp_dump(table: *mut udp_table, skb: *mut sk_buff, cb: *mut netlink_callback, r: *const inet_diag_req_v2) {
    unsafe {
        let net_admin = netlink_net_capable((*cb).skb, 24 /* CAP_NET_ADMIN */);
        let net = (*(*skb).sk).sk_net;
        let cb_data = (*cb).data as *mut c_void;
        let bc = (*cb_data).cast::<c_void>();
        let mut slot = (*cb).args[0];
        let mut num = (*cb).args[1];

        while slot <= (*table).mask {
            let hslot = &mut *(*table).hash.offset(slot as isize);
            let mut sk: *mut sock = ptr::null_mut();

            if hlist_empty(&hslot.head) {
                slot += 1;
                continue;
            }

            spin_lock_bh(&mut hslot.lock);
            sk_for_each(sk, &hslot.head) {
                if !net_eq(sock_net(sk), net) {
                    continue;
                }
                if num < (*cb).args[1] {
                    num += 1;
                    continue;
                }
                if !((*r).idiag_states & (1 << (*sk).sk_state)) {
                    num += 1;
                    continue;
                }
                if (*r).sdiag_family != AF_UNSPEC && (*sk).sk_family != (*r).sdiag_family {
                    num += 1;
                    continue;
                }
                // Additional checks would be added here

                let res = sk_diag_dump(sk, skb, cb, r, bc, net_admin);
                if res < 0 {
                    spin_unlock_bh(&mut hslot.lock);
                    return;
                }
                num += 1;
            }
            spin_unlock_bh(&mut hslot.lock);
            slot += 1;
        }
        (*cb).args[0] = slot;
        (*cb).args[1] = num;
    }
}

#[no_mangle]
pub extern "C" fn udp_diag_dump(skb: *mut sk_buff, cb: *mut netlink_callback, r: *const inet_diag_req_v2) {
    unsafe {
        let table = &mut *(ptr::null_mut::<udp_table>());
        udp_dump(table, skb, cb, r);
    }
}

#[no_mangle]
pub extern "C" fn udp_diag_dump_one(cb: *mut netlink_callback, req: *const inet_diag_req_v2) -> c_int {
    unsafe {
        let table = &mut *(ptr::null_mut::<udp_table>());
        udp_dump_one(table, cb, req)
    }
}

#[no_mangle]
pub extern "C" fn udp_diag_get_info(sk: *mut sock, r: *mut inet_diag_msg, info: *mut c_void) {
    unsafe {
        (*r).idiag_rqueue = udp_rqueue_get(sk);
        (*r).idiag_wqueue = (*sk).sk_wmem_alloc;
    }
}

#[cfg(feature = "CONFIG_INET_DIAG_DESTROY")]
#[no_mangle]
pub extern "C" fn udp_diag_destroy(in_skb: *mut sk_buff, req: *const inet_diag_req_v2) -> c_int {
    unsafe {
        let table = &mut *(ptr::null_mut::<udp_table>());
        __udp_diag_destroy(in_skb, req, table)
    }
}

// Additional functions for udplite and module init/exit would be added here

// Module initialization
#[no_mangle]
pub extern "C" fn udp_diag_init() -> c_int {
    unsafe {
        let mut err = 0;
        let handler = &mut *(ptr::null_mut::<inet_diag_handler>());
        handler.dump = Some(udp_diag_dump);
        handler.dump_one = Some(udp_diag_dump_one);
        handler.idiag_get_info = Some(udp_diag_get_info);
        handler.idiag_type = 17; // IPPROTO_UDP
        handler.idiag_info_size = 0;
        
        err = inet_diag_register(handler);
        if err != 0 {
            return err;
        }
        
        // Register udplite handler
        let udplite_handler = &mut *(ptr::null_mut::<inet_diag_handler>());
        udplite_handler.dump = Some(udplite_diag_dump);
        udplite_handler.dump_one = Some(udplite_diag_dump_one);
        udplite_handler.idiag_get_info = Some(udp_diag_get_info);
        udplite_handler.idiag_type = 136; // IPPROTO_UDPLITE
        udplite_handler.idiag_info_size = 0;
        
        err = inet_diag_register(udplite_handler);
        if err != 0 {
            inet_diag_unregister(handler);
        }
        return err;
    }
}

#[no_mangle]
pub extern "C" fn udp_diag_exit() {
    unsafe {
        inet_diag_unregister(&mut *(ptr::null_mut::<inet_diag_handler>()));
        inet_diag_unregister(&mut *(ptr::null_mut::<inet_diag_handler>()));
    }
}
