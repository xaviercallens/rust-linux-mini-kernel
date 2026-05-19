#![no_std]
//! dup syscall
//!
//! This module implements sys_dup functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel syscalls

use kernel_types::*;
use libc::{c_int, c_uint, c_void, c_ulong, size_t};

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn sys_dup_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn sys_dup_exit() {
}

#[no_mangle]
pub static SYS_DUP_INITIALIZED: bool = false;
