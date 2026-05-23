#![no_std]
//! Kernel version
//!
//! This module implements init_version functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel init

use core::ffi::c_int;

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn init_version_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn init_version_exit() {
}

#[no_mangle]
pub static INIT_VERSION_INITIALIZED: bool = false;
