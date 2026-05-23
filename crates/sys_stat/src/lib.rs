#![no_std]
//! stat syscall
//!
//! This module implements sys_stat functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel syscalls

use core::ffi::c_int;

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn sys_stat_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn sys_stat_exit() {
}

#[no_mangle]
pub static SYS_STAT_INITIALIZED: bool = false;
