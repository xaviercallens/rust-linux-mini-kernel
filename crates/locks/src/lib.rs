#![no_std]
//! File locking
//!
//! This module implements locks functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel fs

use kernel_types::*;
use libc::{c_int, c_uint, c_void, c_ulong, size_t};

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn locks_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn locks_exit() {
}

#[no_mangle]
pub static LOCKS_INITIALIZED: bool = false;
