#![no_std]
//! String formatting
//!
//! This module implements vsprintf functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel lib

use core::ffi::c_int;

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn vsprintf_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn vsprintf_exit() {
}

#[no_mangle]
pub static VSPRINTF_INITIALIZED: bool = false;
