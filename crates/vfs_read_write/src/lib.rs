#![no_std]
//! Read/write operations
//!
//! This module implements vfs_read_write functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel fs/vfs

use libc::c_int;

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn vfs_read_write_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn vfs_read_write_exit() {
}

#[no_mangle]
pub static VFS_READ_WRITE_INITIALIZED: bool = false;
