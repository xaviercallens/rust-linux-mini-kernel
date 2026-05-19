#![no_std]
//! Early memory allocator
//!
//! This module implements memblock functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel mm/memblock.c

use kernel_types::*;
use libc::{c_int, c_uint, c_void, c_ulong, size_t};

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
