#![no_std]
//! String operations
//!
//! This module implements string functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel lib

use libc::c_int;

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn string_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn string_exit() {
}

#[no_mangle]
pub static STRING_INITIALIZED: bool = false;
