#![no_std]
//! I/O memory mapping
//!
//! This module implements arch_ioremap functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel arch/x86/mm

use kernel_types::*;
use libc::{c_int, c_uint, c_void, c_ulong, size_t};

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn arch_ioremap_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn arch_ioremap_exit() {
}

#[no_mangle]
pub static ARCH_IOREMAP_INITIALIZED: bool = false;
