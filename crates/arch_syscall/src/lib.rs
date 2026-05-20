#![no_std]
//! Syscall entry
//!
//! This module implements arch_syscall functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel arch/x86/entry

use kernel_types::*;
use libc::{c_int, c_uint, c_void, c_ulong, size_t};

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn arch_syscall_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn arch_syscall_exit() {
}

#[no_mangle]
pub static ARCH_SYSCALL_INITIALIZED: bool = false;
