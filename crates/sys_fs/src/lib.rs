#![no_std]
//! Filesystem syscalls
//!
//! This module implements sys_fs functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel syscalls

use core::ffi::c_int;

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn sys_fs_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn sys_fs_exit() {
}

#[no_mangle]
pub static SYS_FS_INITIALIZED: bool = false;
