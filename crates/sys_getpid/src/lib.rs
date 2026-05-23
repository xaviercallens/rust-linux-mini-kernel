#![no_std]
//! getpid syscall
//!
//! This module implements sys_getpid functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel syscalls

use core::ffi::c_int;

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn sys_getpid_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn sys_getpid_exit() {
}

#[no_mangle]
pub static SYS_GETPID_INITIALIZED: bool = false;
