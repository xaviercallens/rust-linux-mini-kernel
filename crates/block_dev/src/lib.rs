#![no_std]
//! Block device operations
//!
//! This module implements block_dev functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel fs

use core::ffi::c_int;

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn block_dev_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn block_dev_exit() {
}

#[no_mangle]
pub static BLOCK_DEV_INITIALIZED: bool = false;
