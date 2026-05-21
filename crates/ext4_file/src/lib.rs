#![no_std]
//! ext4 file operations
//!
//! This module implements ext4_file functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel fs/ext4

use libc::c_int;

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn ext4_file_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn ext4_file_exit() {
}

#[no_mangle]
pub static EXT4_FILE_INITIALIZED: bool = false;
