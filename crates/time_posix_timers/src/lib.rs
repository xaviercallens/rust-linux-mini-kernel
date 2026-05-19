#![no_std]
//! POSIX timer APIs
//!
//! This module implements time_posix_timers functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel time

use kernel_types::*;
use libc::{c_int, c_uint, c_void, c_ulong, size_t};

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn time_posix_timers_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn time_posix_timers_exit() {
}

#[no_mangle]
pub static TIME_POSIX_TIMERS_INITIALIZED: bool = false;
