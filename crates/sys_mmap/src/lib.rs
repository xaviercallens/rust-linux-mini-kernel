#![no_std]
//! mmap syscall
//!
//! This module implements sys_mmap functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel syscalls

use core::ffi::c_int;

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn sys_mmap_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn sys_mmap_exit() {
}

#[no_mangle]
pub static SYS_MMAP_INITIALIZED: bool = false;
