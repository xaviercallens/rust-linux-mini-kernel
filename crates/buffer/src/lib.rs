#![no_std]
//! Buffer cache
//!
//! This module implements buffer functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel fs

use core::ffi::c_int;

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn buffer_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn buffer_exit() {
}

#[no_mangle]
pub static BUFFER_INITIALIZED: bool = false;
