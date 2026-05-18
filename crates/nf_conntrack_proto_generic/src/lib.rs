
//! This module provides FFI-compatible Rust bindings for the Linux kernel's
//! generic protocol connection tracking implementation. It maintains ABI
//! compatibility with the original C implementation for netfilter/conntrack.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use core::mem::size_of;
use core::ptr;
use kernel_types::*;

// Constants from C
pub const HZ: c_int = 100; // System clock ticks per second
pub const CTA_TIMEOUT_GENERIC_TIMEOUT: c_int = 1;
pub const CTA_TIMEOUT_GENERIC_MAX: c_int = 2;
pub const ENOSPC: c_int = -12;
pub const EINVAL: c_int = -22;

// Type definitions
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nlattr {
    _unused: [u8; 0],
} // Opaque type - actual layout defined in kernel headers

#[repr(C)]
struct NfGenericNet {
    timeout: c_uint,
}

#[repr(C)]
struct NlaPolicy {
    type_: c_uint,
}

#[repr(C)]
struct NfCtnlTimeout {
    nlattr_to_obj: Option<
        unsafe extern "C" fn(tb: *mut *mut nlattr, net: *mut c_void, data: *mut c_void) -> c_int,
    >,
    obj_to_nlattr: Option<unsafe extern "C" fn(skb: *mut c_void, data: *const c_void) -> c_int>,
    nlattr_max: c_int,
    obj_size: size_t,
    nla_policy: *const NlaPolicy,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct NfConntrackL4proto {
    l4proto: u8,
    #[cfg(CONFIG_NF_CONNTRACK_TIMEOUT)]
    ctnl_timeout: NfCtnlTimeout,
}

// Global data
#[no_mangle]
pub static NF_CONNTRACK_L4PROTO_GENERIC: NfConntrackL4proto = NfConntrackL4proto {
    l4proto: 255,
    #[cfg(CONFIG_NF_CONNTRACK_TIMEOUT)]
    ctnl_timeout: NfCtnlTimeout {
        nlattr_to_obj: Some(generic_timeout_nlattr_to_obj),
        obj_to_nlattr: Some(generic_timeout_obj_to_nlattr),
        nlattr_max: CTA_TIMEOUT_GENERIC_MAX,
        obj_size: size_of::<c_uint>(),
        nla_policy: &GENERIC_TIMEOUT_NLA_POLICY as *const NlaPolicy,
    },
};

// Static data
#[cfg(CONFIG_NF_CONNTRACK_TIMEOUT)]
#[no_mangle]
static GENERIC_TIMEOUT_NLA_POLICY: [NlaPolicy; CTA_TIMEOUT_GENERIC_MAX as usize + 1] = {
    let mut arr = [NlaPolicy { type_: 0 }; CTA_TIMEOUT_GENERIC_MAX as usize + 1];
    arr[CTA_TIMEOUT_GENERIC_TIMEOUT as usize] = NlaPolicy { type_: 1 }; // NLA_U32
    arr
};

// Extern declarations for kernel functions
extern "C" {
    fn nf_generic_pernet(net: *mut c_void) -> *mut NfGenericNet;
    fn nla_get_be32(attr: *const nlattr) -> u32;
    fn nla_put_be32(skb: *mut c_void, type_: c_int, data: u32) -> c_int;
}

// Function implementations
#[no_mangle]
pub unsafe extern "C" fn nf_conntrack_generic_init_net(net: *mut c_void) {
    let gn = nf_generic_pernet(net);
    (*gn).timeout = NF_CT_GENERIC_TIMEOUT();
}

#[no_mangle]
pub unsafe extern "C" fn generic_timeout_nlattr_to_obj(
    tb: *mut *mut nlattr,
    net: *mut c_void,
    data: *mut c_void,
) -> c_int {
    let gn = nf_generic_pernet(net);
    let timeout = data as *mut c_uint;

    // SAFETY: Caller guarantees valid net pointer
    let gn_timeout = &mut (*gn).timeout;

    if timeout.is_null() {
        return EINVAL;
    }

    let attr_index = CTA_TIMEOUT_GENERIC_TIMEOUT as isize;
    let attr = *tb.offset(attr_index);

    if !attr.is_null() {
        let value = nla_get_be32(attr);
        *timeout = libc::ntohl(value) * HZ as u32;
    } else {
        *timeout = *gn_timeout;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn generic_timeout_obj_to_nlattr(
    skb: *mut c_void,
    data: *const c_void,
) -> c_int {
    let timeout = data as *const c_uint;
    let timeout_val = *timeout;

    // SAFETY: Caller guarantees valid skb pointer
    if nla_put_be32(
        skb,
        CTA_TIMEOUT_GENERIC_TIMEOUT,
        libc::htonl(timeout_val / HZ as u32),
    ) != 0
    {
        return ENOSPC;
    }

    0
}

// Constants
#[no_mangle]
pub static NF_CT_GENERIC_TIMEOUT: unsafe extern "C" fn() -> c_uint =
    || -> c_uint { 600 * HZ as u32 };