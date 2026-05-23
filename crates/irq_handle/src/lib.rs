#![no_std]
//! IRQ handling core
//!
//! This module implements irq_handle functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel kernel/irq

use core::ffi::c_int;

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn irq_handle_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn irq_handle_exit() {
}

#[no_mangle]
pub static IRQ_HANDLE_INITIALIZED: bool = false;
