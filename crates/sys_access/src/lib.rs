#![allow(clippy::all, clippy::pedantic)]
#![no_std]
//! access syscall
//!
//! This module implements sys_access functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel syscalls

use core::ffi::c_int;

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn sys_access_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn sys_access_exit() {
}

#[no_mangle]
pub static SYS_ACCESS_INITIALIZED: bool = false;
