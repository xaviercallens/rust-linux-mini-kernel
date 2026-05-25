#![allow(clippy::all, clippy::pedantic)]
#![no_std]
//! High-resolution timers
//!
//! This module implements time_hrtimer functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel time

use core::ffi::c_int;

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn time_hrtimer_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn time_hrtimer_exit() {
}

#[no_mangle]
pub static TIME_HRTIMER_INITIALIZED: bool = false;
