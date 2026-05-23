#![no_std]
//! APIC management
//!
//! This module implements arch_apic functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel arch/x86/kernel

use core::ffi::c_int;

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn arch_apic_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn arch_apic_exit() {
}

#[no_mangle]
pub static ARCH_APIC_INITIALIZED: bool = false;
