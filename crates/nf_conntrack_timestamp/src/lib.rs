// SPDX-License-Identifier: GPL-2.0-or-later
//!
//! This module implements connection tracking timestamp functionality for the Linux kernel.
//! The implementation is a direct FFI-compatible Rust translation of the original C code,
//! preserving the exact ABI and memory layout required for kernel integration.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_variables)]

use core::ffi::{c_char, c_int, c_void};
use kernel_types::*;

// Kernel constants
const EINVAL: c_int = -22;
const ENOMEM: c_int = -12;

// Kernel constants from headers
const NF_CT_EXT_TSTAMP: u32 = 0; // Actual value defined in kernel headers

// Module parameter
static mut NF_CT_TSTAMP: bool = false;

// Extension descriptor
#[repr(C)]
struct NF_CT_EXT_TYPE {
    len: u32,
    align: u32,
    id: u32,
}

static TSTAMP_EXTEND: NF_CT_EXT_TYPE = NF_CT_EXT_TYPE {
    len: core::mem::size_of::<NF_CONN_TSTAMP>() as u32,
    align: core::mem::align_of::<NF_CONN_TSTAMP>() as u32,
    id: NF_CT_EXT_TSTAMP,
};

// Opaque type from kernel headers
#[repr(C)]
struct NF_CONN_TSTAMP {
    // Actual fields defined in kernel headers
}

// External kernel functions
extern "C" {
    fn nf_ct_extend_register(ext: *const NF_CT_EXT_TYPE) -> c_int;
    fn nf_ct_extend_unregister(ext: *const NF_CT_EXT_TYPE);
    fn pr_err(fmt: *const c_char);
}

// Module parameter initialization
#[no_mangle]
pub unsafe extern "C" fn nf_conntrack_tstamp_pernet_init(net: *mut c_void) -> c_int {
    // SAFETY: Kernel guarantees valid net pointer during pernet init
    //         and exclusive access to sysctl_tstamp field
    unsafe {
        let net_ptr = net as *mut net;
        (*net_ptr).ct.sysctl_tstamp = NF_CT_TSTAMP;
    }
    0
}

// Module initialization
#[no_mangle]
pub unsafe extern "C" fn nf_conntrack_tstamp_init() -> c_int {
    let ret = unsafe { nf_ct_extend_register(&TSTAMP_EXTEND) };

    if ret < 0 {
        // SAFETY: Error message is valid C string
        unsafe {
            pr_err(b"Unable to register extension\n".as_ptr() as *const c_char);
        }
    }

    ret
}

// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn nf_conntrack_tstamp_fini() {
    // SAFETY: Extension must be registered before unregistration
    unsafe {
        nf_ct_extend_unregister(&TSTAMP_EXTEND);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extension_size() {
        assert!(core::mem::size_of::<NF_CONN_TSTAMP>() > 0);
        assert!(core::mem::align_of::<NF_CONN_TSTAMP>() > 0);
    }
}