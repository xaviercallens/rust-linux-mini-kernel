#![allow(clippy::all, clippy::pedantic)]
#![no_std]
//! Tick handling
//!
//! This module implements time_tick functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel time

use core::ffi::c_int;

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn time_tick_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn time_tick_exit() {
}

#[no_mangle]
pub static TIME_TICK_INITIALIZED: bool = false;
