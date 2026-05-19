#![no_std]
//! Page table setup
//!
//! This module implements arch_pgtable functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel arch/x86/mm

use kernel_types::*;
use libc::{c_int, c_uint, c_void, c_ulong, size_t};

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn arch_pgtable_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn arch_pgtable_exit() {
}

#[no_mangle]
pub static ARCH_PGTABLE_INITIALIZED: bool = false;
