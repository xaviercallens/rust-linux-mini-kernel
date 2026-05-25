#![allow(clippy::all, clippy::pedantic)]
#![no_std]
//! NTP support
//!
//! This module implements time_ntp functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel time

use core::ffi::c_int;

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn time_ntp_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn time_ntp_exit() {
}

#[no_mangle]
pub static TIME_NTP_INITIALIZED: bool = false;
