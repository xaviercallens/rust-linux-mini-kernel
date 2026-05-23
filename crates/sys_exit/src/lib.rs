#![no_std]
//! exit syscall
//!
//! This module implements sys_exit functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel syscalls

use core::ffi::c_int;

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn sys_exit_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn sys_exit_exit() {
}

#[no_mangle]
pub static SYS_EXIT_INITIALIZED: bool = false;
