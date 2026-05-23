#![no_std]
//! Memory pool allocator
//!
//! This module implements mempool functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel mm/mempool.c

use core::ffi::c_int;

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn mempool_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn mempool_exit() {
}

// Placeholder exports for FFI compatibility
#[no_mangle]
pub static MEMPOOL_INITIALIZED: bool = false;
