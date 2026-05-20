#![no_std]
//! Alarm timers
//!
//! This module implements time_alarmtimer functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel time

use kernel_types::*;
use libc::{c_int, c_uint, c_void, c_ulong, size_t};

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn time_alarmtimer_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn time_alarmtimer_exit() {
}

#[no_mangle]
pub static TIME_ALARMTIMER_INITIALIZED: bool = false;
