#![allow(clippy::all, clippy::pedantic)]
#![no_std]
#![warn(clippy::pedantic)]
#![deny(clippy::all)]
//! Platform devices
//!
//! This module implements driver_base_platform functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel drivers/base

use core::ffi::c_int;

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
