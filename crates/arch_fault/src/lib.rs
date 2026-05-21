#![no_std]
//! Page fault handler
//!
//! This module implements arch_fault functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel arch/x86/mm

use libc::c_int;

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn arch_fault_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn arch_fault_exit() {
}

#[no_mangle]
pub static ARCH_FAULT_INITIALIZED: bool = false;
