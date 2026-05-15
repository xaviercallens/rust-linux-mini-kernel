//! Common UDP/RAW code for Linux INET implementation
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use core::ptr;
use core::ffi::c_int;

// Constants from C
pub const EINVAL: c_int = -22;
pub const EAFNOSUPPORT: c_int = -97;
pub const ENETUNREACH: c_int = -101;
pub const EACCES: c_int = -13;

// Type definitions
#[repr(C)]
pub struct sockaddr {
    pub sa_family: c_int,
    pub sa_data: [u8; 14],
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
    pub sk_prot: *const sk_prot_ops,
    pub sk_net: *const net,
    pub sk_dst_cache: *mut dst_entry,
}

#[repr(C)]
pub struct inet_sock {
    pub inet_saddr: u32,
    pub inet_rcv_saddr: u32,
    pub inet_daddr: u32,
    pub inet_dport: u16,
    pub mc_index: c_int,
    pub mc_addr: u32,
    pub inet_opt: *mut ip_options_rcu,
    pub inet_sport: u16,
    pub inet_id: u32,
}

#[repr(C)]
pub struct sk_prot_ops {
    pub rehash: Option<unsafe extern "C" fn(*mut sock)>,
}

#[repr(C)]
pub struct net {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ip_options_rcu {
    pub opt: ip_options,
}

#[repr(C)]
pub struct ip_options {
    pub srr: u8,
    pub faddr: u32,
}

#[repr(C)]
pub struct flowi4 {
    pub saddr: u32,
    pub daddr: u32,
    _private: [u8; 0],
}

#[repr(C)]
pub struct rtable {
    pub rt_flags: u32,
    pub dst: dst_entry,
}

#[repr(C)]
pub struct dst_entry {
    pub obsolete: u8,
    pub ops: *const dst_ops,
}

#[repr(C)]
pub struct dst_ops {
    pub check: Option<unsafe extern "C" fn(*mut dst_entry, c_int) -> *mut dst_entry>,
}

// Function pointers for C functions
type ip_route_connect_fn = unsafe extern "C" fn(
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

type ip_route_output_ports_fn = unsafe extern "C" fn(
    net: *mut net,
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

type sk_dst_set_fn = unsafe extern "C" fn(sk: *mut sock, dst: *mut dst_entry);

type sk_dst_reset_fn = unsafe extern "C" fn(sk: *mut sock);

type sock_flag_fn = unsafe extern "C" fn(sk: *mut sock, flag: c_int) -> c_int;

type IP_INC_STATS_fn = unsafe extern "C" fn(net: *mut net, mib: c_int);

// Extern declarations for C functions
extern "C" {
    fn ipv4_is_multicast(addr: u32) -> c_int;
    fn netif_index_is_l3_master(net: *mut net, oif: c_int) -> c_int;
    fn prandom_u32() -> u32;
}

// Function implementations
/// Connect a datagram socket to a remote address
///
/// # Safety
/// - `sk` must be a valid pointer to a sock struct
/// - `uaddr` must be a valid pointer to a sockaddr struct
/// - Caller must handle locking for thread safety
///
/// # Returns
/// 0 on success, negative error code on failure
#[no_mangle]
pub unsafe extern "C" fn __ip4_datagram_connect(
    sk: *mut sock,
    uaddr: *mut sockaddr,
    addr_len: c_int,
) -> c_int {
    if sk.is_null() || uaddr.is_null() {
        return EINVAL;
    }

    let inet = (sk as *mut inet_sock).as_mut().unwrap();
    let usin = uaddr as *mut sockaddr_in;

    if addr_len < core::mem::size_of::<sockaddr_in>() as c_int {
        return EINVAL;
    }

    if (*usin).sin_family != AF_INET {
        return EAFNOSUPPORT;
    }

    // SAFETY: Caller guarantees sk is valid
    unsafe {
        // Reset destination cache
        extern "C" {
            fn sk_dst_reset(sk: *mut sock);
        }
        sk_dst_reset(sk);
    }

    let oif = (*sk).sk_bound_dev_if;
    let saddr = (*inet).inet_saddr;

    if unsafe { ipv4_is_multicast((*usin).sin_addr.s_addr) } != 0 {
        if oif == 0 || unsafe { netif_index_is_l3_master((*sk).sk_net, oif) } != 0 {
            let mc_index = (*inet).mc_index;
            if oif == 0 {
                (*sk).sk_bound_dev_if = mc_index;
            }
        }
        if saddr == 0 {
            (*inet).inet_saddr = (*inet).mc_addr;
        }
    }

    let fl4 = &mut (*inet).cork.fl.u.ip4;
    let rt = unsafe {
        extern "C" {
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
        }
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

    if rt.is_null() {
        let err = -1;
        return err;
    }

    if (*rt).rt_flags & RTCF_BROADCAST != 0 && 
       unsafe { sock_flag(sk, SOCK_BROADCAST) } == 0 {
        unsafe {
            extern "C" {
                fn ip_rt_put(rt: *mut rtable);
            }
            ip_rt_put(rt);
        }
        return EACCES;
    }

    if (*inet).inet_saddr == 0 {
        (*inet).inet_saddr = (*fl4).saddr;
    }

    if (*inet).inet_rcv_saddr == 0 {
        (*inet).inet_rcv_saddr = (*fl4).saddr;
        if let Some(rehash) = (*(*sk).sk_prot).rehash {
            rehash(sk);
        }
    }

    (*inet).inet_daddr = (*fl4).daddr;
    (*inet).inet_dport = (*usin).sin_port;

    (*sk).sk_state = TCP_ESTABLISHED;
    unsafe {
        extern "C" {
            fn sk_set_txhash(sk: *mut sock);
        }
        sk_set_txhash(sk);
    }

    (*inet).inet_id = unsafe { prandom_u32() };

    unsafe {
        extern "C" {
            fn sk_dst_set(sk: *mut sock, dst: *mut dst_entry);
        }
        sk_dst_set(sk, &(*rt).dst as *mut _);
    }

    0
}

/// Connect a datagram socket with locking
///
/// # Safety
/// - `sk` must be a valid pointer to a sock struct
/// - `uaddr` must be a valid pointer to a sockaddr struct
///
/// # Returns
/// 0 on success, negative error code on failure
#[no_mangle]
pub unsafe extern "C" fn ip4_datagram_connect(
    sk: *mut sock,
    uaddr: *mut sockaddr,
    addr_len: c_int,
) -> c_int {
    extern "C" {
        fn lock_sock(sk: *mut sock);
        fn release_sock(sk: *mut sock);
    }

    lock_sock(sk);
    let res = __ip4_datagram_connect(sk, uaddr, addr_len);
    release_sock(sk);
    res
}

/// Release callback for datagram sockets
///
/// # Safety
/// - `sk` must be a valid pointer to a sock struct
#[no_mangle]
pub unsafe extern "C" fn ip4_datagram_release_cb(sk: *mut sock) {
    let inet = (sk as *mut inet_sock).as_ref().unwrap();
    let daddr = (*inet).inet_daddr;

    extern "C" {
        fn rcu_read_lock();
        fn rcu_read_unlock();
        fn __sk_dst_get(sk: *mut sock) -> *mut dst_entry;
        fn sk_dst_set(sk: *mut sock, dst: *mut dst_entry);
        fn ip_route_output_ports(
            net: *mut net,
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
    }

    rcu_read_lock();

    let dst = __sk_dst_get(sk);
    if !dst.is_null() && (*dst).obsolete == 0 || 
       (*(*dst).ops).check.unwrap()(dst, 0) != ptr::null_mut() {
        rcu_read_unlock();
        return;
    }

    let inet_opt = (*inet).inet_opt;
    let mut final_daddr = daddr;
    if !inet_opt.is_null() && (*inet_opt).opt.srr != 0 {
        final_daddr = (*inet_opt).opt.faddr;
    }

    let fl4 = &mut (*inet).cork.fl.u.ip4;
    let rt = ip_route_output_ports(
        (*sk).sk_net,
        fl4,
        sk,
        final_daddr,
        (*inet).inet_saddr,
        (*inet).inet_dport,
        (*inet).inet_sport,
        (*sk).sk_protocol,
        RT_CONN_FLAGS(sk),
        (*sk).sk_bound_dev_if,
    );

    let new_dst = if !rt.is_null() {
        &(*rt).dst as *mut _
    } else {
        ptr::null_mut()
    };

    sk_dst_set(sk, new_dst);

    rcu_read_unlock();
}

// Constants for C compatibility
pub const AF_INET: c_int = 2;
pub const RT_CONN_FLAGS: unsafe extern "C" fn(*mut sock) -> u32 = |_sk| 0;
pub const RTCF_BROADCAST: u32 = 1 << 0;
pub const SOCK_BROADCAST: c_int = 1;
pub const TCP_ESTABLISHED: c_int = 1;