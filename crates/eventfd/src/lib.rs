#![no_std]
//! Event file descriptor
//!
//! This module implements eventfd functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel fs

use core::ffi::c_int;

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn eventfd_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn eventfd_exit() {
}

#[no_mangle]
pub static EVENTFD_INITIALIZED: bool = false;
