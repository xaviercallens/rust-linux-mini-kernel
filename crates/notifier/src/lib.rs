#![no_std]
//! Notification chains
//!
//! This module implements notifier functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel kernel

use libc::c_int;

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn notifier_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn notifier_exit() {
}

#[no_mangle]
pub static NOTIFIER_INITIALIZED: bool = false;
