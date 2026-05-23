#![no_std]
//! /proc filesystem
//!
//! This module implements proc_base functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel fs/proc

use core::ffi::c_int;

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn proc_base_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn proc_base_exit() {
}

#[no_mangle]
pub static PROC_BASE_INITIALIZED: bool = false;
