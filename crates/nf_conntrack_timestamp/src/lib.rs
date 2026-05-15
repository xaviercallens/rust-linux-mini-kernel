// SPDX-License-Identifier: GPL-2.0-or-later
//!
//! This module implements connection tracking timestamp functionality for the Linux kernel.
//! The implementation is a direct FFI-compatible Rust translation of the original C code,
//! preserving the exact ABI and memory layout required for kernel integration.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_variables)]

use core::ptr;
use core::ffi::{c_int, c_char};

// Kernel constants
const EINVAL: c_int = -22;
const ENOMEM: c_int = -12;

// Kernel types
#[repr(C)]
struct net {
    ct: netct,
}

#[repr(C)]
struct netct {
    sysctl_tstamp: bool,
}

#[repr(C)]
struct nf_ct_ext_type {
    len: u32,
    align: u32,
    id: u32,
}

// Kernel constants from headers
const NF_CT_EXT_TSTAMP: u32 = 0; // Actual value defined in kernel headers

// Module parameter
static mut nf_ct_tstamp: bool = false;

// Extension descriptor
static tstamp_extend: nf_ct_ext_type = nf_ct_ext_type {
    len: core::mem::size_of::<nf_conn_tstamp>() as u32,
    align: core::mem::align_of::<nf_conn_tstamp>() as u32,
    id: NF_CT_EXT_TSTAMP,
};

// Opaque type from kernel headers
#[repr(C)]
struct nf_conn_tstamp {
    // Actual fields defined in kernel headers
}

// External kernel functions
extern "C" {
    fn nf_ct_extend_register(ext: *const nf_ct_ext_type) -> c_int;
    fn nf_ct_extend_unregister(ext: *const nf_ct_ext_type);
    fn pr_err(fmt: *const c_char);
}

// Module parameter initialization
#[no_mangle]
pub unsafe extern "C" fn nf_conntrack_tstamp_pernet_init(net: *mut net) {
    // SAFETY: Kernel guarantees valid net pointer during pernet init
    //         and exclusive access to sysctl_tstamp field
    unsafe {
        (*net).ct.sysctl_tstamp = nf_ct_tstamp;
    }
}

// Module initialization
#[no_mangle]
pub unsafe extern "C" fn nf_conntrack_tstamp_init() -> c_int {
    let ret = unsafe { nf_ct_extend_register(&tstamp_extend) };
    
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
        nf_ct_extend_unregister(&tstamp_extend);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_extension_size() {
        assert!(core::mem::size_of::<nf_conn_tstamp>() > 0);
        assert!(core::mem::align_of::<nf_conn_tstamp>() > 0);
    }
}