#![no_std]
//! /sys filesystem
//!
//! This module implements sysfs_dir functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel fs/sysfs

use kernel_types::*;
use libc::{c_int, c_uint, c_void, c_ulong, size_t};

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn sysfs_dir_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn sysfs_dir_exit() {
}

#[no_mangle]
pub static SYSFS_DIR_INITIALIZED: bool = false;
