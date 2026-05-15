//! Raw Socket Diagnostic Module
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use core::ptr;
use core::ffi::c_int;
use core::ffi::c_void;
use core::ffi::c_uint;
use core::ffi::size_t;

// Constants from C
pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ENOENT: c_int = -2;
pub const ECONNABORTED: c_int = -108;

// Type definitions
#[repr(C)]
pub struct inet_diag_req_v2 {
    pub sdiag_family: c_int,
    pub sdiag_protocol: c_int,
    pub idiag_ext: c_int,
    pub pad: c_int,
    pub idiag_states: c_int,
    pub id: inet_diag_msg,
}

#[repr(C)]
pub struct inet_diag_msg {
    pub idiag_family: c_int,
    pub idiag_state: c_int,
    pub idiag_src_len: c_int,
    pub idiag_dst_len: c_int,
    pub id: [u32; 4],
    pub idiag_sport: u16,
    pub idiag_dport: u16,
    pub idiag_if: c_int,
    pub idiag_snd_queue: u32,
    pub idiag_rcv_queue: u32,
    pub idiag_rqueue: u32,
    pub idiag_wqueue: u32,
    pub idiag_qlen: u32,
    pub idiag_ino: u32,
    pub idiag_rtt: u32,
    pub idiag_expires: u32,
    pub idiag_vfs: [u32; 2],
}

#[repr(C)]
pub struct raw_hashinfo {
    pub lock: c_void, // Placeholder for real lock type
    pub ht: *mut *mut c_void, // Placeholder for socket list
}

#[repr(C)]
pub struct netlink_callback {
    pub skb: *mut c_void,
    pub args: [c_int; 2],
    pub data: *mut c_void,
}

#[repr(C)]
pub struct sk_buff {
    pub sk: *mut c_void,
}

#[repr(C)]
pub struct sock {
    pub sk_family: c_int,
    pub sk_net: *mut c_void, // Placeholder for net namespace
}

#[repr(C)]
pub struct net {
    pub diag_nlsk: *mut c_void,
}

#[repr(C)]
pub struct inet_diag_dump_data {
    pub inet_diag_nla_bc: *mut c_void,
}

#[repr(C)]
pub struct inet_diag_handler {
    dump: extern "C" fn(*mut c_void, *const inet_diag_req_v2),
    dump_one: extern "C" fn(*mut netlink_callback, *const inet_diag_req_v2) -> c_int,
    idiag_get_info: extern "C" fn(*mut sock, *mut inet_diag_msg, *mut c_void),
    idiag_type: c_int,
    idiag_info_size: c_int,
    #[cfg(CONFIG_INET_DIAG_DESTROY)]
    destroy: extern "C" fn(*mut sk_buff, *const inet_diag_req_v2) -> c_int,
}

// Static variables (placeholders)
static raw_v4_hashinfo: raw_hashinfo = raw_hashinfo {
    lock: 0 as _,
    ht: ptr::null_mut(),
};

#[cfg(CONFIG_IPV6)]
static raw_v6_hashinfo: raw_hashinfo = raw_hashinfo {
    lock: 0 as _,
    ht: ptr::null_mut(),
};

// Function implementations
/// Get raw hashinfo based on request family
///
/// # Safety
/// - Caller must ensure `r` is valid
#[no_mangle]
pub unsafe extern "C" fn raw_get_hashinfo(
    r: *const inet_diag_req_v2,
) -> *mut raw_hashinfo {
    if (*r).sdiag_family == 2 { // AF_INET
        &raw_v4_hashinfo as *const _ as *mut _
    }
    #[cfg(CONFIG_IPV6)]
    else if (*r).sdiag_family == 10 { // AF_INET6
        &raw_v6_hashinfo as *const _ as *mut _
    }
    else {
        (EINVAL as *mut c_void) as *mut raw_hashinfo
    }
}

/// Lookup raw socket based on request
///
/// # Safety
/// - Caller must ensure `net`, `from`, and `req` are valid
#[no_mangle]
pub unsafe extern "C" fn raw_lookup(
    net: *mut c_void,
    from: *mut sock,
    req: *const inet_diag_req_v2,
) -> *mut sock {
    let r = req as *mut inet_diag_req_raw;
    if (*r).sdiag_family == 2 {
        __raw_v4_lookup(
            net, from, 
            (*r).sdiag_raw_protocol,
            (*r).id.idiag_dst[0],
            (*r).id.idiag_src[0],
            (*r).id.idiag_if,
            0
        )
    }
    #[cfg(CONFIG_IPV6)]
    else {
        __raw_v6_lookup(
            net, from, 
            (*r).sdiag_raw_protocol,
            &(*r).id.idiag_src as *const _ as *const u8,
            &(*r).id.idiag_dst as *const _ as *const u8,
            (*r).id.idiag_if,
            0
        )
    }
}

/// Get raw socket for diagnostics
///
/// # Safety
/// - Caller must ensure `net` and `r` are valid
#[no_mangle]
pub unsafe extern "C" fn raw_sock_get(
    net: *mut c_void,
    r: *const inet_diag_req_v2,
) -> *mut sock {
    let hashinfo = raw_get_hashinfo(r);
    if hashinfo as *mut c_void == (EINVAL as *mut c_void) {
        return (EINVAL as *mut c_void) as *mut sock;
    }

    let mut sk = ptr::null_mut();
    let mut s = ptr::null_mut();
    let mut slot = 0;

    // SAFETY: hashinfo is valid and lock is properly acquired
    read_lock(&(*hashinfo).lock);
    while slot < RAW_HTABLE_SIZE {
        s = sk_for_each(&(*hashinfo).ht[slot]);
        while !s.is_null() {
            sk = raw_lookup(net, s, r);
            if !sk.is_null() {
                sock_hold(sk);
                read_unlock(&(*hashinfo).lock);
                return sk;
            }
            s = s.offset(1);
        }
        slot += 1;
    }
    read_unlock(&(*hashinfo).lock);

    if sk.is_null() {
        (ENOENT as *mut c_void) as *mut sock
    } else {
        sk
    }
}

/// Dump single raw socket diagnostic info
///
/// # Safety
/// - Caller must ensure `cb` and `r` are valid
#[no_mangle]
pub unsafe extern "C" fn raw_diag_dump_one(
    cb: *mut netlink_callback,
    r: *const inet_diag_req_v2,
) -> c_int {
    let in_skb = (*cb).skb;
    let net = sock_net((*in_skb).sk);
    let sk = raw_sock_get(net, r);
    if sk as *mut c_void == (EINVAL as *mut c_void) || 
       sk as *mut c_void == (ENOENT as *mut c_void) {
        return *sk as c_int;
    }

    let rep = nlmsg_new(
        nla_total_size(core::mem::size_of::<inet_diag_msg>()) +
        inet_diag_msg_attrs_size() +
        nla_total_size(core::mem::size_of::<inet_diag_meminfo>()) + 64,
        GFP_KERNEL
    );
    if rep.is_null() {
        sock_put(sk);
        return ENOMEM;
    }

    let err = inet_sk_diag_fill(
        sk, ptr::null_mut(), rep, cb, r, 0,
        netlink_net_capable(in_skb, CAP_NET_ADMIN)
    );
    sock_put(sk);

    if err < 0 {
        kfree_skb(rep);
        return err;
    }

    let err = netlink_unicast(
        (*net).diag_nlsk, rep,
        NETLINK_CB(in_skb).portid,
        MSG_DONTWAIT
    );
    if err > 0 {
        0
    } else {
        err
    }
}

/// Dump raw socket diagnostics
///
/// # Safety
/// - Caller must ensure `skb` and `cb` are valid
#[no_mangle]
pub unsafe extern "C" fn raw_diag_dump(
    skb: *mut sk_buff,
    cb: *mut netlink_callback,
    r: *const inet_diag_req_v2,
) {
    let net_admin = netlink_net_capable((*cb).skb, CAP_NET_ADMIN);
    let hashinfo = raw_get_hashinfo(r);
    if hashinfo as *mut c_void == (EINVAL as *mut c_void) {
        return;
    }

    let net = sock_net((*skb).sk);
    let cb_data = (*cb).data;
    let bc = (*cb_data).inet_diag_nla_bc;
    let mut slot = (*cb).args[0];
    let mut num = (*cb).args[1];
    let mut sk = ptr::null_mut();

    read_lock(&(*hashinfo).lock);
    while slot < RAW_HTABLE_SIZE {
        let mut s = sk_for_each(&(*hashinfo).ht[slot]);
        while !s.is_null() {
            let inet = inet_sk(s);
            if !net_eq((*s).sk_net, net) {
                s = s.offset(1);
                continue;
            }
            if (*s).sk_family != (*r).sdiag_family {
                s = s.offset(1);
                continue;
            }
            if (*r).id.idiag_sport != (*inet).inet_sport && (*r).id.idiag_sport != 0 {
                s = s.offset(1);
                continue;
            }
            if (*r).id.idiag_dport != (*inet).inet_dport && (*r).id.idiag_dport != 0 {
                s = s.offset(1);
                continue;
            }

            let err = sk_diag_dump(s, skb, cb, r, bc, net_admin);
            if err < 0 {
                read_unlock(&(*hashinfo).lock);
                return;
            }
            s = s.offset(1);
        }
        slot += 1;
    }
    read_unlock(&(*hashinfo).lock);

    (*cb).args[0] = slot;
    (*cb).args[1] = num;
}

/// Get raw socket diagnostic info
///
/// # Safety
/// - Caller must ensure `sk` and `r` are valid
#[no_mangle]
pub unsafe extern "C" fn raw_diag_get_info(
    sk: *mut sock,
    r: *mut inet_diag_msg,
    info: *mut c_void,
) {
    (*r).idiag_rqueue = sk_rmem_alloc_get(sk);
    (*r).idiag_wqueue = sk_wmem_alloc_get(sk);
}

#[cfg(CONFIG_INET_DIAG_DESTROY)]
#[no_mangle]
pub unsafe extern "C" fn raw_diag_destroy(
    in_skb: *mut sk_buff,
    r: *const inet_diag_req_v2,
) -> c_int {
    let net = sock_net((*in_skb).sk);
    let sk = raw_sock_get(net, r);
    if sk as *mut c_void == (EINVAL as *mut c_void) || 
       sk as *mut c_void == (ENOENT as *mut c_void) {
        return *sk as c_int;
    }
    let err = sock_diag_destroy(sk, ECONNABORTED);
    sock_put(sk);
    err
}

// Static handler registration
static raw_diag_handler: inet_diag_handler = inet_diag_handler {
    dump: raw_diag_dump,
    dump_one: raw_diag_dump_one,
    idiag_get_info: raw_diag_get_info,
    idiag_type: 255, // IPPROTO_RAW
    idiag_info_size: 0,
    #[cfg(CONFIG_INET_DIAG_DESTROY)]
    destroy: raw_diag_destroy,
};

#[no_mangle]
pub unsafe extern "C" fn raw_diag_init() -> c_int {
    inet_diag_register(&raw_diag_handler)
}

#[no_mangle]
pub unsafe extern "C" fn raw_diag_exit() {
    inet_diag_unregister(&raw_diag_handler)
}

// FFI-compatible helper functions (placeholders)
#[no_mangle]
pub unsafe extern "C" fn __raw_v4_lookup(
    net: *mut c_void,
    from: *mut sock,
    protocol: c_int,
    daddr: u32,
    saddr: u32,
    ifindex: c_int,
    _flags: c_int,
) -> *mut sock {
    // Placeholder implementation
    ptr::null_mut()
}

#[cfg(CONFIG_IPV6)]
#[no_mangle]
pub unsafe extern "C" fn __raw_v6_lookup(
    net: *mut c_void,
    from: *mut sock,
    protocol: c_int,
    saddr: *const u8,
    daddr: *const u8,
    ifindex: c_int,
    _flags: c_int,
) -> *mut sock {
    // Placeholder implementation
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn read_lock(lock: *mut c_void) {
    // Placeholder for real lock implementation
}

#[no_mangle]
pub unsafe extern "C" fn read_unlock(lock: *mut c_void) {
    // Placeholder for real lock implementation
}

#[no_mangle]
pub unsafe extern "C" fn sk_for_each(ht: *mut *mut c_void) -> *mut c_void {
    // Placeholder for socket list iteration
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn sock_hold(sk: *mut sock) {
    // Placeholder for reference counting
}

#[no_mangle]
pub unsafe extern "C" fn sock_put(sk: *mut sock) {
    // Placeholder for reference counting
}

#[no_mangle]
pub unsafe extern "C" fn nlmsg_new(size: size_t, gfp: c_int) -> *mut c_void {
    // Placeholder for skb allocation
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn kfree_skb(skb: *mut c_void) {
    // Placeholder for skb freeing
}

#[no_mangle]
pub unsafe extern "C" fn inet_sk_diag_fill(
    sk: *mut sock,
    nlinfo: *mut c_void,
    skb: *mut c_void,
    cb: *mut netlink_callback,
    r: *const inet_diag_req_v2,
    flags: c_int,
    net_admin: c_int,
) -> c_int {
    // Placeholder implementation
    0
}

#[no_mangle]
pub unsafe extern "C" fn netlink_unicast(
    nlsk: *mut c_void,
    skb: *mut c_void,
    portid: c_int,
    flags: c_int,
) -> c_int {
    // Placeholder implementation
    0
}

#[no_mangle]
pub unsafe extern "C" fn netlink_net_capable(skb: *mut c_void, cap: c_int) -> c_int {
    // Placeholder implementation
    1
}

#[no_mangle]
pub unsafe extern "C" fn sock_diag_destroy(sk: *mut sock, errno: c_int) -> c_int {
    // Placeholder implementation
    0
}

#[no_mangle]
pub unsafe extern "C" fn inet_diag_register(handler: *const inet_diag_handler) -> c_int {
    // Placeholder implementation
    0
}

#[no_mangle]
pub unsafe extern "C" fn inet_diag_unregister(handler: *const inet_diag_handler) {
    // Placeholder implementation
}

// Module initialization
#[no_mangle]
pub unsafe extern "C" fn module_init() {
    raw_diag_init();
}

#[no_mangle]
pub unsafe extern "C" fn module_exit() {
    raw_diag_exit();
}
