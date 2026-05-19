#![no_std]
//! Signal file descriptor
//!
//! This module implements signalfd functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel fs

use kernel_types::*;
use libc::{c_int, c_uint, c_void, c_ulong, size_t};

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn signalfd_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn signalfd_exit() {
}

#[no_mangle]
pub static SIGNALFD_INITIALIZED: bool = false;
