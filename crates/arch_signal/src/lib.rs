#![no_std]
//! Signal arch code
//!
//! This module implements arch_signal functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel arch/x86/kernel

use libc::c_int;

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn arch_signal_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn arch_signal_exit() {
}

#[no_mangle]
pub static ARCH_SIGNAL_INITIALIZED: bool = false;
