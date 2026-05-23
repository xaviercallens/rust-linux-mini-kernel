#![no_std]
//! /dev/mem, /dev/null
//!
//! This module implements driver_char_mem functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel drivers/char

use core::ffi::c_int;

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn driver_char_mem_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn driver_char_mem_exit() {
}

#[no_mangle]
pub static DRIVER_CHAR_MEM_INITIALIZED: bool = false;
