#![allow(clippy::all, clippy::pedantic)]
#![no_std]
//! Page I/O for swapping
//!
//! This module implements page_io functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel mm/page_io.c

use core::ffi::c_int;

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn page_io_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn page_io_exit() {
}

// Placeholder exports for FFI compatibility
#[no_mangle]
pub static PAGE_IO_INITIALIZED: bool = false;
