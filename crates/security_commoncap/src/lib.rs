#![no_std]
//! Capability handling
//!
//! This module implements security_commoncap functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel security

use kernel_types::*;
use libc::{c_int, c_uint, c_void, c_ulong, size_t};

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn security_commoncap_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn security_commoncap_exit() {
}

#[no_mangle]
pub static SECURITY_COMMONCAP_INITIALIZED: bool = false;
