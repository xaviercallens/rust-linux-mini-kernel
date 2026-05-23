#![no_std]
//! Bitmap operations
//!
//! This module implements bitmap functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel lib

use core::ffi::c_int;

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn bitmap_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn bitmap_exit() {
}

#[no_mangle]
pub static BITMAP_INITIALIZED: bool = false;
