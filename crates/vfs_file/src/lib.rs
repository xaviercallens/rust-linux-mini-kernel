#![no_std]
//! File operations
//!
//! This module implements vfs_file functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel fs/vfs

use libc::c_int;

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn vfs_file_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn vfs_file_exit() {
}

#[no_mangle]
pub static VFS_FILE_INITIALIZED: bool = false;
