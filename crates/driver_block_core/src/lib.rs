#![no_std]
//! Block device core
//!
//! This module implements driver_block_core functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel drivers/block

use core::ffi::c_int;

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn driver_block_core_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn driver_block_core_exit() {
}

#[no_mangle]
pub static DRIVER_BLOCK_CORE_INITIALIZED: bool = false;
