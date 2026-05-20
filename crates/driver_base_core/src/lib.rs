#![no_std]
//! Device model core
//!
//! This module implements driver_base_core functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel drivers/base

use kernel_types::*;
use libc::{c_int, c_uint, c_void, c_ulong, size_t};

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn driver_base_core_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn driver_base_core_exit() {
}

#[no_mangle]
pub static DRIVER_BASE_CORE_INITIALIZED: bool = false;
