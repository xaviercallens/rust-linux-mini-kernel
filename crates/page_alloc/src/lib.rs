#![no_std]
//! Physical page allocator (buddy system)
//!
//! This module implements page_alloc functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel mm/page_alloc.c

use kernel_types::*;
use libc::{c_int, c_uint, c_void, c_ulong, size_t};

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn page_alloc_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn page_alloc_exit() {
}

// Placeholder exports for FFI compatibility
#[no_mangle]
pub static PAGE_ALLOC_INITIALIZED: bool = false;
