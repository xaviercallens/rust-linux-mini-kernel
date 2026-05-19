#![no_std]
//! Reference counting
//!
//! This module implements kref functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel lib

use kernel_types::*;
use libc::{c_int, c_uint, c_void, c_ulong, size_t};

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn kref_init() -> c_int {
    // TODO: Initialize kref subsystem
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn kref_exit() {
    // TODO: Cleanup kref subsystem
}

#[no_mangle]
pub static KREF_INITIALIZED: bool = false;
