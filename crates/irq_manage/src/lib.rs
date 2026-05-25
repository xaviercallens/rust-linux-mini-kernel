#![allow(clippy::all, clippy::pedantic)]
#![no_std]
//! IRQ resource management
//!
//! This module implements irq_manage functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel kernel/irq

use core::ffi::c_int;

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn irq_manage_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn irq_manage_exit() {
}

#[no_mangle]
pub static IRQ_MANAGE_INITIALIZED: bool = false;
