#![no_std]
//! Kernel linked lists
//!
//! This module implements klist functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel lib

use kernel_types::*;
use libc::{c_int, c_uint, c_void, c_ulong, size_t};

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn klist_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn klist_exit() {
}

#[no_mangle]
pub static KLIST_INITIALIZED: bool = false;
