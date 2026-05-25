#![allow(clippy::all, clippy::pedantic)]
#![no_std]
//! Per-CPU memory allocator
//!
//! This module implements percpu functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel mm/percpu.c

use core::ffi::c_int;

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn percpu_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn percpu_exit() {
}

// Placeholder exports for FFI compatibility
#[no_mangle]
pub static PERCPU_INITIALIZED: bool = false;
