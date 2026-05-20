#![no_std]
//! System call table
//!
//! This module implements syscall_table functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel syscalls

use kernel_types::*;
use libc::{c_int, c_uint, c_void, c_ulong, size_t};

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn syscall_table_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn syscall_table_exit() {
}

#[no_mangle]
pub static SYSCALL_TABLE_INITIALIZED: bool = false;
