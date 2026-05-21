#![no_std]
//! chdir syscall
//!
//! This module implements sys_chdir functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel syscalls

use libc::c_int;

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn sys_chdir_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn sys_chdir_exit() {
}

#[no_mangle]
pub static SYS_CHDIR_INITIALIZED: bool = false;
