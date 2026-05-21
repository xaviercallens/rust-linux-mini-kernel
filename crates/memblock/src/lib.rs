#![no_std]
//! Early memory allocator
//!
//! This module implements memblock functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel mm/memblock.c

use libc::c_int;

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn memblock_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn memblock_exit() {
}

// Placeholder exports for FFI compatibility
#[no_mangle]
pub static MEMBLOCK_INITIALIZED: bool = false;
