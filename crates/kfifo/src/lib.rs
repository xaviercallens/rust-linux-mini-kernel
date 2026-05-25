#![allow(clippy::all, clippy::pedantic)]
#![no_std]
//! Kernel FIFO
//!
//! This module implements kfifo functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel kernel

use core::ffi::c_int;

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn kfifo_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn kfifo_exit() {
}

#[no_mangle]
pub static KFIFO_INITIALIZED: bool = false;
