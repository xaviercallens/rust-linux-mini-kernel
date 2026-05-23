#![no_std]
//! IRQ chip management
//!
//! This module implements irq_chip functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel kernel/irq

use core::ffi::c_int;

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn irq_chip_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn irq_chip_exit() {
}

#[no_mangle]
pub static IRQ_CHIP_INITIALIZED: bool = false;
