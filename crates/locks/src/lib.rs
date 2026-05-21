#![no_std]
//! File locking
//!
//! This module implements locks functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel fs

use libc::c_int;

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
