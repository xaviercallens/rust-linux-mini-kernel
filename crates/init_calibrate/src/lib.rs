#![allow(clippy::all, clippy::pedantic)]
#![no_std]
//! Delay calibration
//!
//! This module implements init_calibrate functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel init

use core::ffi::c_int;

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn init_calibrate_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn init_calibrate_exit() {
}

#[no_mangle]
pub static INIT_CALIBRATE_INITIALIZED: bool = false;
