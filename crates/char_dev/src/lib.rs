#![no_std]
//! Character device operations
//!
//! This module implements char_dev functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel fs

use kernel_types::*;
use libc::{c_int, c_uint, c_void, c_ulong, size_t};

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn char_dev_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn char_dev_exit() {
}

#[no_mangle]
pub static CHAR_DEV_INITIALIZED: bool = false;
