#![no_std]
//! close syscall
//!
//! This module implements sys_close functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel syscalls

use libc::c_int;

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn sys_close_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn sys_close_exit() {
}

#[no_mangle]
pub static SYS_CLOSE_INITIALIZED: bool = false;
