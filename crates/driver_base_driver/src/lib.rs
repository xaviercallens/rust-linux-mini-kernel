#![no_std]
//! Driver model
//!
//! This module implements driver_base_driver functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel drivers/base

use libc::c_int;

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn driver_base_driver_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn driver_base_driver_exit() {
}

#[no_mangle]
pub static DRIVER_BASE_DRIVER_INITIALIZED: bool = false;
