#![no_std]
//! Platform devices
//!
//! This module implements driver_base_platform functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel drivers/base

use kernel_types::*;
use libc::{c_int, c_uint, c_void, c_ulong, size_t};

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn driver_base_platform_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn driver_base_platform_exit() {
}

#[no_mangle]
pub static DRIVER_BASE_PLATFORM_INITIALIZED: bool = false;
