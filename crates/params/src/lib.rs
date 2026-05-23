#![no_std]
//! Kernel parameters
//!
//! This module implements params functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel kernel

use core::ffi::c_int;

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn params_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn params_exit() {
}

#[no_mangle]
pub static PARAMS_INITIALIZED: bool = false;
