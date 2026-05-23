#![no_std]
//! Boot header
//!
//! This module implements arch_boot_header functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel arch/x86/boot

use core::ffi::c_int;

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn arch_boot_header_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn arch_boot_header_exit() {
}

#[no_mangle]
pub static ARCH_BOOT_HEADER_INITIALIZED: bool = false;
