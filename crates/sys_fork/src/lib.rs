#![allow(clippy::all, clippy::pedantic)]
#![no_std]
//! fork syscall
//!
//! This module implements sys_fork functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel syscalls

use core::ffi::c_int;

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn sys_fork_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn sys_fork_exit() {
}

#[no_mangle]
pub static SYS_FORK_INITIALIZED: bool = false;
