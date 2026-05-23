#![no_std]
//! Inode operations
//!
//! This module implements vfs_inode functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel fs/vfs

use core::ffi::c_int;

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn vfs_inode_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn vfs_inode_exit() {
}

#[no_mangle]
pub static VFS_INODE_INITIALIZED: bool = false;
