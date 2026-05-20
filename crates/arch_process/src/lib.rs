#![no_std]
//! Process arch code
//!
//! This module implements arch_process functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel arch/x86/kernel

use kernel_types::*;
use libc::{c_int, c_uint, c_void, c_ulong, size_t};

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn arch_process_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn arch_process_exit() {
}

#[no_mangle]
pub static ARCH_PROCESS_INITIALIZED: bool = false;
