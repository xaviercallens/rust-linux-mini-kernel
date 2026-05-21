#![no_std]
//! ext4 extents
//!
//! This module implements ext4_extents functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel fs/ext4

use libc::c_int;

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn ext4_extents_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn ext4_extents_exit() {
}

#[no_mangle]
pub static EXT4_EXTENTS_INITIALIZED: bool = false;
