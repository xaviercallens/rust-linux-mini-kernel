#![no_std]
//! Path resolution
//!
//! This module implements vfs_namei functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel fs/vfs

use libc::c_int;

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn vfs_namei_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn vfs_namei_exit() {
}

#[no_mangle]
pub static VFS_NAMEI_INITIALIZED: bool = false;
