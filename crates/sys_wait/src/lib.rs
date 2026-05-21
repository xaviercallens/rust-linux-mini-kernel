#![no_std]
//! wait syscall
//!
//! This module implements sys_wait functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel syscalls

use libc::c_int;

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn sys_wait_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn sys_wait_exit() {
}

#[no_mangle]
pub static SYS_WAIT_INITIALIZED: bool = false;
