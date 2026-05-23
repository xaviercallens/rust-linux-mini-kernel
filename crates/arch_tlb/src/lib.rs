#![no_std]
//! TLB management
//!
//! This module implements arch_tlb functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel arch/x86/mm

use core::ffi::c_int;

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn arch_tlb_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn arch_tlb_exit() {
}

#[no_mangle]
pub static ARCH_TLB_INITIALIZED: bool = false;
