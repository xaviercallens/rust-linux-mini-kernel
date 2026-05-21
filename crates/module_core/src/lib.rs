#![no_std]
//! Module loading
//!
//! This module implements module_core functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel kernel

use libc::c_int;

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn module_core_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn module_core_exit() {
}

#[no_mangle]
pub static MODULE_CORE_INITIALIZED: bool = false;
