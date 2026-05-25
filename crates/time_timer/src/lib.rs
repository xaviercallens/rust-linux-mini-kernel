#![allow(clippy::all, clippy::pedantic)]
#![no_std]
//! Kernel timers
//!
//! This module implements time_timer functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel time

use core::ffi::c_int;

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn time_timer_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn time_timer_exit() {
}

#[no_mangle]
pub static TIME_TIMER_INITIALIZED: bool = false;
