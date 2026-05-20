#![no_std]
//! ext4 block allocator
//!
//! This module implements ext4_balloc functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel fs/ext4

use kernel_types::*;
use libc::{c_int, c_uint, c_void, c_ulong, size_t};

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn ext4_balloc_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn ext4_balloc_exit() {
}

#[no_mangle]
pub static EXT4_BALLOC_INITIALIZED: bool = false;
