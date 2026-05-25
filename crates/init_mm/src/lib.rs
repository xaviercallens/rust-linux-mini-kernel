#![allow(clippy::all, clippy::pedantic)]
#![no_std]
//! Memory manager initialization
//!
//! This module implements init_mm functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel mm/init_mm.c

use core::ffi::c_int;

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn init_mm_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn init_mm_exit() {
}

// Placeholder exports for FFI compatibility
#[no_mangle]
pub static INIT_MM_INITIALIZED: bool = false;
