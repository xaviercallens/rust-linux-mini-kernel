#![allow(clippy::all, clippy::pedantic)]
#![no_std]
//! Page table setup
//!
//! This module implements arch_pgtable functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel arch/x86/mm

use core::ffi::c_int;

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
