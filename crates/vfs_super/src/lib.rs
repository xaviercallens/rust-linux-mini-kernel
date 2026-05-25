#![allow(clippy::all, clippy::pedantic)]
#![no_std]
//! Superblock operations
//!
//! This module implements vfs_super functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel fs/vfs

use core::ffi::c_int;

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn vfs_super_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn vfs_super_exit() {
}

#[no_mangle]
pub static VFS_SUPER_INITIALIZED: bool = false;
