#![no_std]
//! Character type functions
//!
//! This module implements ctype functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel lib

use core::ffi::c_int;

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn ctype_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn ctype_exit() {
}

#[no_mangle]
pub static CTYPE_INITIALIZED: bool = false;
