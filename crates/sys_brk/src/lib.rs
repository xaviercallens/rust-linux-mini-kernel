#![no_std]
//! brk syscall
//!
//! This module implements sys_brk functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel syscalls

use kernel_types::*;
use libc::{c_int, c_uint, c_void, c_ulong, size_t};

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn sys_brk_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn sys_brk_exit() {
}

#[no_mangle]
pub static SYS_BRK_INITIALIZED: bool = false;
