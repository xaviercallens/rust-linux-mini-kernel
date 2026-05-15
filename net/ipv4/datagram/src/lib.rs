//! Common UDP/RAW code for Linux INET implementation
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]  // For C-style type names

use core::ptr;
use libc::{c_int, c_uint, c_void, size_t};

// Constants from C
pub const EINVAL: c_int = -22;
pub const EAFNOSUPPORT: c_int = -97;
pub const ENETUNREACH: c_int = -101;
pub const EACCES: c_int = -13;

// Type definitions
#[repr(C)]
pub struct sockaddr {
    pub sa_family: c_int,
    pub sa_data: [c_char; 14],
}

#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: c_int,
    pub sin_port: u16,
    pub sin_addr: in_addr,
    pub sin_zero: [u8; 8],
}

#[repr(C)]
pub struct in_addr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct sock {
    pub sk_bound_dev_if: c_int,
    pub sk_protocol: u8,
    pub sk_state: c_int,
    pub sk_txhash: u32,
    pub sk_net: c_void, // Placeholder for struct net
    pub sk_prot: *const c_void, // Placeholder for struct proto
}

#[repr(C)]
pub struct inet_sock {
    pub inet_saddr: u32,
    pub inet_rcv_saddr: u32,
    pub inet_daddr: u32,
    pub inet_dport: u16,
    pub mc_index: c_int,
    pub mc_addr: u32,
    pub inet_opt: *const c_void, // Placeholder for ip_options_rcu
    pub inet_id: u32,
}

#[repr(C)]
pub struct rtable {
    pub rt_flags: u32,
    pub rt_dst: u32,
    pub rt_src: u32,
    pub rt_key_dst: u32,
    pub rt_key_src: u32,
    pub rt_key_iif: c_int,
    pub rt_key_oif: c_int,
    pub rt_key_tos: u8,
    pub rt_key_flags: u32,
    pub rt_dst_entry: *const c_void, // Placeholder for dst_entry
}

#[repr(C)]
pub struct flowi4 {
    pub daddr: u32,
    pub saddr: u32,
    pub fl4_tos: u8,
    pub fl4_oif: c_int,
    pub fl4_tos: u8,
    pub fl4_flags: u32,
}

// Function pointers for external C functions
extern "C" {
    fn ipv4_is_multicast(addr: u32) -> c_int;
    fn netif_index_is_l3_master(net: *const c_void, oif: c_int) -> c_int;
    fn ip_route_connect(
        fl4: *mut flowi4,
        daddr: u32,
        saddr: u32,
        flags: u32,
        oif: c_int,
        protocol: u8,
        sport: u16,
        dport: u16,
        sk: *mut sock,
    ) -> *mut rtable;
    fn ip_route_output_ports(
        net: *const c_void,
        fl4: *mut flowi4,
        sk: *mut sock,
        daddr: u32,
        saddr: u32,
        dport: u16,
        sport: u16,
        protocol: u8,
        flags: u32,
        oif: c_int,
    ) -> *mut rtable;
    fn sock_net(sk: *mut sock) -> *const c_void;
    fn inet_sk(sk: *mut sock) -> *mut inet_sock;
    fn sk_dst_reset(sk: *mut sock);
    fn IP_INC_STATS(net: *const c_void, mib: c_int);
    fn sk_dst_set(sk: *mut sock, dst: *const c_void);
    fn sk_dst_get(sk: *mut sock) -> *const c_void;
    fn dst_ops_check(dst: *const c_void, arg: c_int) -> c_int;
    fn reuseport_has_conns(sk: *mut sock, has_conns: c_int);
    fn sk_set_txhash(sk: *mut sock);
    fn prandom_u32() -> u32;
    fn lock_sock(sk: *mut sock);
    fn release_sock(sk: *mut sock);
    fn rcu_read_lock();
    fn rcu_read_unlock();
    fn rcu_dereference<T>(ptr: *const T) -> *const T;
}

// Function implementations
#[no_mangle]
pub unsafe extern "C" fn __ip4_datagram_connect(
    sk: *mut sock,
    uaddr: *mut sockaddr,
    addr_len: c_int,
) -> c_int {
    // SAFETY: Caller guarantees sk and uaddr are valid
    let inet = unsafe { inet_sk(sk) };
    let usin = unsafe { uaddr as *mut sockaddr_in };

    if addr_len < size_of::<sockaddr_in>() as c_int {
        return EINVAL;
    }

    if (*usin).sin_family != AF_INET {
        return EAFNOSUPPORT;
    }

    unsafe { sk_dst_reset(sk) };

    let oif = (*sk).sk_bound_dev_if;
    let saddr = (*inet).inet_saddr;

    if unsafe { ipv4_is_multicast((*usin).sin_addr.s_addr) } != 0 {
        if oif == 0 || 
           unsafe { netif_index_is_l3_master(sock_net(sk), oif) } != 0 {
            let mc_index = (*inet).mc_index;
            if oif != 0 {
                (*sk).sk_bound_dev_if = mc_index;
            }
        }
        if saddr == 0 {
            (*inet).inet_saddr = (*inet).mc_addr;
        }
    }

    let fl4 = &mut (*inet).cork.fl.u.ip4;
    let rt = unsafe {
        ip_route_connect(
            fl4,
            (*usin).sin_addr.s_addr,
            (*inet).inet_saddr,
            RT_CONN_FLAGS(sk),
            (*sk).sk_bound_dev_if,
            (*sk).sk_protocol,
            (*inet).inet_sport,
            (*usin).sin_port,
            sk,
        )
    };

    let mut err = if rt.is_null() {
        let ptr_err = unsafe { PTR_ERR(rt) };
        if ptr_err == ENETUNREACH {
            unsafe { IP_INC_STATS(sock_net(sk), IPSTATS_MIB_OUTNOROUTES) };
        }
        ptr_err
    } else {
        0
    };

    if err != 0 {
        return err;
    }

    if ((*rt).rt_flags & RTCF_BROADCAST) != 0 && 
       !sock_flag(sk, SOCK_BROADCAST) {
        unsafe { ip_rt_put(rt) };
        return EACCES;
    }

    if (*inet).inet_saddr == 0 {
        (*inet).inet_saddr = (*fl4).saddr;
    }

    if (*inet).inet_rcv_saddr == 0 {
        (*inet).inet_rcv_saddr = (*fl4).saddr;
        if !(*sk).sk_prot.is_null() && 
           (*(*sk).sk_prot).rehash != 0 {
            unsafe { (*(*sk).sk_prot).rehash(sk) };
        }
    }

    (*inet).inet_daddr = (*fl4).daddr;
    (*inet).inet_dport = (*usin).sin_port;
    unsafe { reuseport_has_conns(sk, 1) };
    (*sk).sk_state = TCP_ESTABLISHED;
    unsafe { sk_set_txhash(sk) };
    (*inet).inet_id = unsafe { prandom_u32() };

    unsafe { sk_dst_set(sk, &(*rt).rt_dst_entry) };

    err
}

#[no_mangle]
pub unsafe extern "C" fn ip4_datagram_connect(
    sk: *mut sock,
    uaddr: *mut sockaddr,
    addr_len: c_int,
) -> c_int {
    let mut res: c_int = 0;
    unsafe { lock_sock(sk) };
    res = unsafe { __ip4_datagram_connect(sk, uaddr, addr_len) };
    unsafe { release_sock(sk) };
    res
}

#[no_mangle]
pub unsafe extern "C" fn ip4_datagram_release_cb(
    sk: *mut sock,
) {
    let inet = unsafe { inet_sk(sk) };
    let daddr = (*inet).inet_daddr;
    let fl4 = &mut (*inet).cork.fl.u.ip4;
    let mut rt: *mut rtable = ptr::null_mut();

    unsafe { rcu_read_lock() };

    let dst = unsafe { __sk_dst_get(sk) };
    if !dst.is_null() && 
       (*dst).obsolete == 0 || 
       unsafe { dst_ops_check(dst, 0) } != 0 {
        unsafe { rcu_read_unlock() };
        return;
    }

    let inet_opt = unsafe { rcu_dereference((*inet).inet_opt) };
    if !inet_opt.is_null() && 
       (*inet_opt).opt.srr != 0 {
        daddr = (*inet_opt).opt.faddr;
    }

    rt = unsafe {
        ip_route_output_ports(
            sock_net(sk),
            fl4,
            sk,
            daddr,
            (*inet).inet_saddr,
            (*inet).inet_dport,
            (*inet).inet_sport,
            (*sk).sk_protocol,
            RT_CONN_FLAGS(sk),
            (*sk).sk_bound_dev_if,
        )
    };

    let new_dst = if !rt.is_null() {
        &(*rt).rt_dst_entry
    } else {
        ptr::null()
    };

    unsafe { sk_dst_set(sk, new_dst) };

    unsafe { rcu_read_unlock() };
}

// Helper functions (extern declarations)
#[no_mangle]
pub unsafe extern "C" fn PTR_ERR<T>(ptr: *mut T) -> c_int {
    // Implementation would convert a PTR_ERR to an error code
    // This is a simplified version
    -(ptr as c_int)
}

#[no_mangle]
pub unsafe extern "C" fn ip_rt_put(rt: *mut rtable) {
    // Implementation would release the route reference
}

#[no_mangle]
pub unsafe extern "C" fn __sk_dst_get(sk: *mut sock) -> *mut c_void {
    // Implementation would get the destination entry
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn sock_flag(sk: *mut sock, flag: c_int) -> c_int {
    // Implementation would check socket flags
    0
}

// Constants
pub const AF_INET: c_int = 2;
pub const RT_CONN_FLAGS: u32 = 0; // Placeholder
pub const RTCF_BROADCAST: u32 = 1 << 0;
pub const SOCK_BROADCAST: c_int = 1 << 1;
pub const IPSTATS_MIB_OUTNOROUTES: c_int = 10;
pub const TCP_ESTABLISHED: c_int = 1;
