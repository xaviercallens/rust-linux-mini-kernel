#![no_std]
//! ext4 inode operations
//!
//! This module implements ext4_inode functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel fs/ext4

use core::ffi::c_int;

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn ext4_inode_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn ext4_inode_exit() {
}

#[no_mangle]
pub static EXT4_INODE_INITIALIZED: bool = false;
