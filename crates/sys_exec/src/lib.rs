#![no_std]
//! exec syscall
//!
//! This module implements sys_exec functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel syscalls

use libc::c_int;

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn sys_exec_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn sys_exec_exit() {
}

#[no_mangle]
pub static SYS_EXEC_INITIALIZED: bool = false;
