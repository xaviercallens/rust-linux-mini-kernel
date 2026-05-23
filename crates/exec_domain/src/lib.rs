#![no_std]
//! Execution domain
//!
//! This module implements exec_domain functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel fs

use core::ffi::c_int;

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn exec_domain_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn exec_domain_exit() {
}

#[no_mangle]
pub static EXEC_DOMAIN_INITIALIZED: bool = false;
