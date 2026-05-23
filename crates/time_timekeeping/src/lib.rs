#![no_std]
//! System time
//!
//! This module implements time_timekeeping functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel time

use core::ffi::c_int;

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn time_timekeeping_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn time_timekeeping_exit() {
}

#[no_mangle]
pub static TIME_TIMEKEEPING_INITIALIZED: bool = false;
