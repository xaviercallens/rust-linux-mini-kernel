#![no_std]
//! Clock sources
//!
//! This module implements time_clocksource functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel time

use libc::c_int;

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn time_clocksource_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn time_clocksource_exit() {
}

#[no_mangle]
pub static TIME_CLOCKSOURCE_INITIALIZED: bool = false;
