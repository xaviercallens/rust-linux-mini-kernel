#![no_std]
//! Kernel panic
//!
//! This module implements panic functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel kernel

use libc::c_int;

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn panic_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn panic_exit() {
}

#[no_mangle]
pub static PANIC_INITIALIZED: bool = false;
