//! tunnel4: Generic IP tunnel transformer for Linux kernel
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(clippy::all)]

use core::ptr;
use core::ffi::c_int;
use core::ffi::c_void;
use core::ffi::c_uint;
use core::ffi::c_short;

// Constants from C
pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ENOSYS: c_int = -38;
pub const EEXIST: c_int = -17;
pub const ENOENT: c_int = -2;

// Type definitions
#[repr(C)]
pub struct xfrm_tunnel {
    pub priority: c_int,
    pub next: *mut xfrm_tunnel,
    pub handler: extern "C" fn(*mut c_void) -> c_int,
    pub err_handler: extern "C" fn(*mut c_void, u32) -> c_int,
    pub cb_handler: extern "C" fn(*mut c_void, c_int) -> c_int,
}

#[repr(C)]
pub struct net_protocol {
    pub handler: extern "C" fn(*mut c_void) -> c_int,
    pub err_handler: extern "C" fn(*mut c_void, u32) -> c_int,
    pub no_policy: c_int,
    pub netns_ok: c_int,
}

#[repr(C)]
pub struct xfrm_input_afinfo {
    pub family: c_int,
    pub is_ipip: c_int,
    pub callback: extern "C" fn(*mut c_void, u8, c_int) -> c_int,
}

// Static variables
static mut tunnel4_handlers: *mut xfrm_tunnel = ptr::null_mut();
static mut tunnel64_handlers: *mut xfrm_tunnel = ptr::null_mut();
static mut tunnelmpls4_handlers: *mut xfrm_tunnel = ptr::null_mut();
static mut tunnel4_mutex: *mut c_void = ptr::null_mut(); // Kernel mutex

// Internal functions
fn fam_handlers(family: c_short) -> *mut *mut xfrm_tunnel {
    if family == 2 { // AF_INET
        &mut tunnel4_handlers as *mut *mut xfrm_tunnel
    } else if family == 10 { // AF_INET6
        &mut tunnel64_handlers as *mut *mut xfrm_tunnel
    } else {
        &mut tunnelmpls4_handlers as *mut *mut xfrm_tunnel
    }
}

// Exported functions
/// Register an IP tunnel handler
///
/// # Safety
/// - Caller must hold tunnel4_mutex
/// - handler must be a valid pointer to xfrm_tunnel
/// - family must be a valid address family
///
/// # Returns
/// 0 on success, -EEXIST if duplicate priority, -ENOENT if not found
#[no_mangle]
pub unsafe extern "C" fn xfrm4_tunnel_register(
    handler: *mut xfrm_tunnel,
    family: c_short,
) -> c_int {
    if handler.is_null() {
        return EINVAL;
    }

    let mut pprev = fam_handlers(family);
    let mut t: *mut xfrm_tunnel = ptr::null_mut();
    let priority = (*handler).priority;
    let mut ret = EEXIST;

    // SAFETY: Caller must hold the mutex
    while !(*pprev).is_null() {
        t = *pprev;
        if (*t).priority > priority {
            break;
        }
        if (*t).priority == priority {
            return EEXIST;
        }
        pprev = &mut (*t).next as *mut *mut xfrm_tunnel;
    }

    (*handler).next = *pprev;
    *pprev = handler;
    ret = 0;

    ret
}

/// Deregister an IP tunnel handler
///
/// # Safety
/// - Caller must hold tunnel4_mutex and call synchronize_net() after
/// - handler must be a valid pointer in the tunnel list
///
/// # Returns
/// 0 on success, -ENOENT if not found
#[no_mangle]
pub unsafe extern "C" fn xfrm4_tunnel_deregister(
    handler: *mut xfrm_tunnel,
    family: c_short,
) -> c_int {
    if handler.is_null() {
        return EINVAL;
    }

    let mut pprev = fam_handlers(family);
    let mut t: *mut xfrm_tunnel = ptr::null_mut();
    let mut ret = ENOENT;

    // SAFETY: Caller must hold the mutex
    while !(*pprev).is_null() {
        t = *pprev;
        if t == handler {
            *pprev = (*t).next;
            ret = 0;
            break;
        }
        pprev = &mut (*t).next as *mut *mut xfrm_tunnel;
    }

    ret
}

/// Process IPv4 tunnel packet
///
/// # Safety
/// - skb must be a valid pointer to sk_buff
/// - Caller must ensure proper RCU read-side lock
///
/// # Returns
/// 0 on success, packet consumed
#[no_mangle]
pub unsafe extern "C" fn tunnel4_rcv(skb: *mut c_void) -> c_int {
    let mut handler: *mut xfrm_tunnel = ptr::null_mut();

    // Check packet size
    if !pskb_may_pull(skb, core::mem::size_of::<u32>()) {
        goto drop;
    }

    // Iterate through handlers
    handler = tunnel4_handlers;
    while !handler.is_null() {
        if ((*handler).handler)(skb) == 0 {
            return 0;
        }
        handler = (*handler).next;
    }

    // Send ICMP error
    icmp_send(skb, 3, 3, 0);

drop:
    kfree_skb(skb);
    0
}

/// Process IPv4 tunnel callback
///
/// # Safety
/// - skb must be a valid pointer to sk_buff
/// - Caller must ensure proper RCU read-side lock
///
/// # Returns
/// 0 on success, error code otherwise
#[no_mangle]
pub unsafe extern "C" fn tunnel4_rcv_cb(
    skb: *mut c_void,
    proto: u8,
    err: c_int,
) -> c_int {
    let head: *mut xfrm_tunnel = if proto == 4 { tunnel4_handlers } else { tunnel64_handlers };
    let mut handler: *mut xfrm_tunnel = ptr::null_mut();
    let mut ret: c_int = 0;

    handler = head;
    while !handler.is_null() {
        if (*handler).cb_handler != ptr::null() as _ {
            ret = ((*handler).cb_handler)(skb, err);
            if ret <= 0 {
                return ret;
            }
        }
        handler = (*handler).next;
    }

    0
}

/// Process IPv6 tunnel packet
///
/// # Safety
/// - skb must be a valid pointer to sk_buff
/// - Caller must ensure proper RCU read-side lock
///
/// # Returns
/// 0 on success, packet consumed
#[no_mangle]
pub unsafe extern "C" fn tunnel64_rcv(skb: *mut c_void) -> c_int {
    let mut handler: *mut xfrm_tunnel = ptr::null_mut();

    // Check packet size
    if !pskb_may_pull(skb, 40) { // sizeof(ipv6hdr)
        goto drop;
    }

    handler = tunnel64_handlers;
    while !handler.is_null() {
        if ((*handler).handler)(skb) == 0 {
            return 0;
        }
        handler = (*handler).next;
    }

    icmp_send(skb, 3, 3, 0);

drop:
    kfree_skb(skb);
    0
}

/// Process MPLS tunnel packet
///
/// # Safety
/// - skb must be a valid pointer to sk_buff
/// - Caller must ensure proper RCU read-side lock
///
/// # Returns
/// 0 on success, packet consumed
#[no_mangle]
pub unsafe extern "C" fn tunnelmpls4_rcv(skb: *mut c_void) -> c_int {
    let mut handler: *mut xfrm_tunnel = ptr::null_mut();

    // Check packet size
    if !pskb_may_pull(skb, 3) { // sizeof(mpls_label)
        goto drop;
    }

    handler = tunnelmpls4_handlers;
    while !handler.is_null() {
        if ((*handler).handler)(skb) == 0 {
            return 0;
        }
        handler = (*handler).next;
    }

    icmp_send(skb, 3, 3, 0);

drop:
    kfree_skb(skb);
    0
}

/// Handle IPv4 tunnel error
///
/// # Safety
/// - skb must be a valid pointer to sk_buff
/// - Caller must ensure proper RCU read-side lock
///
/// # Returns
/// 0 on success, -ENOENT if no handler
#[no_mangle]
pub unsafe extern "C" fn tunnel4_err(skb: *mut c_void, info: u32) -> c_int {
    let mut handler: *mut xfrm_tunnel = ptr::null_mut();

    handler = tunnel4_handlers;
    while !handler.is_null() {
        if ((*handler).err_handler)(skb, info) == 0 {
            return 0;
        }
        handler = (*handler).next;
    }

    ENOENT
}

/// Handle IPv6 tunnel error
///
/// # Safety
/// - skb must be a valid pointer to sk_buff
/// - Caller must ensure proper RCU read-side lock
///
/// # Returns
/// 0 on success, -ENOENT if no handler
#[no_mangle]
pub unsafe extern "C" fn tunnel64_err(skb: *mut c_void, info: u32) -> c_int {
    let mut handler: *mut xfrm_tunnel = ptr::null_mut();

    handler = tunnel64_handlers;
    while !handler.is_null() {
        if ((*handler).err_handler)(skb, info) == 0 {
            return 0;
        }
        handler = (*handler).next;
    }

    ENOENT
}

/// Handle MPLS tunnel error
///
/// # Safety
/// - skb must be a valid pointer to sk_buff
/// - Caller must ensure proper RCU read-side lock
///
/// # Returns
/// 0 on success, -ENOENT if no handler
#[no_mangle]
pub unsafe extern "C" fn tunnelmpls4_err(skb: *mut c_void, info: u32) -> c_int {
    let mut handler: *mut xfrm_tunnel = ptr::null_mut();

    handler = tunnelmpls4_handlers;
    while !handler.is_null() {
        if ((*handler).err_handler)(skb, info) == 0 {
            return 0;
        }
        handler = (*handler).next;
    }

    ENOENT
}

// Kernel helper functions (extern declarations)
extern "C" {
    fn pskb_may_pull(skb: *mut c_void, size: usize) -> c_int;
    fn icmp_send(skb: *mut c_void, type_: c_int, code: c_int, info: u32);
    fn kfree_skb(skb: *mut c_void);
    fn inet_add_protocol(proto: *const net_protocol, num: c_int) -> c_int;
    fn inet_del_protocol(proto: *const net_protocol, num: c_int) -> c_int;
    fn xfrm_input_register_afinfo(afinfo: *const xfrm_input_afinfo) -> c_int;
    fn xfrm_input_unregister_afinfo(afinfo: *const xfrm_input_afinfo) -> c_int;
    fn mutex_lock(mutex: *mut c_void);
    fn mutex_unlock(mutex: *mut c_void);
    fn synchronize_net();
    fn pr_err(fmt: *const c_char, ...);
}

// Module init/exit
#[no_mangle]
pub unsafe extern "C" fn tunnel4_init() -> c_int {
    let mut ret: c_int = 0;
    
    if inet_add_protocol(&tunnel4_protocol, 4) != 0 {
        goto err;
    }
    
    #[cfg(CONFIG_IPV6)]
    if inet_add_protocol(&tunnel64_protocol, 41) != 0 {
        inet_del_protocol(&tunnel4_protocol, 4);
        goto err;
    }
    
    #[cfg(CONFIG_MPLS)]
    if inet_add_protocol(&tunnelmpls4_protocol, 137) != 0 {
        inet_del_protocol(&tunnel4_protocol, 4);
        #[cfg(CONFIG_IPV6)]
        inet_del_protocol(&tunnel64_protocol, 41);
        goto err;
    }
    
    #[cfg(CONFIG_INET_XFRM_TUNNEL)]
    if xfrm_input_register_afinfo(&tunnel4_input_afinfo) != 0 {
        inet_del_protocol(&tunnel4_protocol, 4);
        #[cfg(CONFIG_IPV6)]
        inet_del_protocol(&tunnel64_protocol, 41);
        #[cfg(CONFIG_MPLS)]
        inet_del_protocol(&tunnelmpls4_protocol, 137);
        goto err;
    }
    
    return 0;
    
err:
    pr_err(b"tunnel4_init: can't add protocol\0".as_ptr() as *const c_char);
    return ENOMEM;
}

#[no_mangle]
pub unsafe extern "C" fn tunnel4_fini() {
    #[cfg(CONFIG_INET_XFRM_TUNNEL)]
    if xfrm_input_unregister_afinfo(&tunnel4_input_afinfo) != 0 {
        pr_err(b"tunnel4_fini: can't remove input afinfo\0".as_ptr() as *const c_char);
    }
    
    #[cfg(CONFIG_MPLS)]
    if inet_del_protocol(&tunnelmpls4_protocol, 137) != 0 {
        pr_err(b"tunnelmpls4_fini: can't remove protocol\0".as_ptr() as *const c_char);
    }
    
    #[cfg(CONFIG_IPV6)]
    if inet_del_protocol(&tunnel64_protocol, 41) != 0 {
        pr_err(b"tunnel64_fini: can't remove protocol\0".as_ptr() as *const c_char);
    }
    
    if inet_del_protocol(&tunnel4_protocol, 4) != 0 {
        pr_err(b"tunnel4_fini: can't remove protocol\0".as_ptr() as *const c_char);
    }
}

// Static protocol definitions
#[no_mangle]
pub static tunnel4_protocol: net_protocol = net_protocol {
    handler: tunnel4_rcv,
    err_handler: tunnel4_err,
    no_policy: 1,
    netns_ok: 1,
};

#[cfg(CONFIG_IPV6)]
#[no_mangle]
pub static tunnel64_protocol: net_protocol = net_protocol {
    handler: tunnel64_rcv,
    err_handler: tunnel64_err,
    no_policy: 1,
    netns_ok: 1,
};

#[cfg(CONFIG_MPLS)]
#[no_mangle]
pub static tunnelmpls4_protocol: net_protocol = net_protocol {
    handler: tunnelmpls4_rcv,
    err_handler: tunnelmpls4_err,
    no_policy: 1,
    netns_ok: 1,
};

#[cfg(CONFIG_INET_XFRM_TUNNEL)]
#[no_mangle]
pub static tunnel4_input_afinfo: xfrm_input_afinfo = xfrm_input_afinfo {
    family: 2,
    is_ipip: 1,
    callback: tunnel4_rcv_cb,
};

// Module metadata
#[no_mangle]
pub static module_license: [u8; 4] = *b"GPL\0";
