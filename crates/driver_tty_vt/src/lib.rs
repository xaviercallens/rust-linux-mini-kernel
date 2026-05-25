#![allow(clippy::all, clippy::pedantic)]
#![no_std]
//! Virtual terminal
//!
//! This module implements driver_tty_vt functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel drivers/char/tty

use core::ffi::c_int;

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn driver_tty_vt_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn driver_tty_vt_exit() {
}

#[no_mangle]
pub static DRIVER_TTY_VT_INITIALIZED: bool = false;
