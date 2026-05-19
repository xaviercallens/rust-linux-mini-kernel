#![no_std]
//! Command line parsing
//!
//! This module implements cmdline functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel lib

use kernel_types::*;
use libc::{c_int, c_uint, c_void, c_ulong, size_t};

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn cmdline_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn cmdline_exit() {
}

#[no_mangle]
pub static CMDLINE_INITIALIZED: bool = false;
