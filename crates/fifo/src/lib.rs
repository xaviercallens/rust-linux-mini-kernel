#![allow(clippy::all, clippy::pedantic)]
#![no_std]
//! FIFO operations
//!
//! This module implements fifo functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel fs

use core::ffi::c_int;

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn fifo_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn fifo_exit() {
}

#[no_mangle]
pub static FIFO_INITIALIZED: bool = false;
