#![no_std]
//! Bus subsystem
//!
//! This module implements driver_base_bus functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel drivers/base

use kernel_types::*;
use libc::{c_int, c_uint, c_void, c_ulong, size_t};

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn driver_base_bus_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn driver_base_bus_exit() {
}

#[no_mangle]
pub static DRIVER_BASE_BUS_INITIALIZED: bool = false;
