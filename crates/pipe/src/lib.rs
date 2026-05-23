#![no_std]
//! Pipe operations
//!
//! This module implements pipe functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel fs

use core::ffi::c_int;

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn pipe_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn pipe_exit() {
}

#[no_mangle]
pub static PIPE_INITIALIZED: bool = false;
