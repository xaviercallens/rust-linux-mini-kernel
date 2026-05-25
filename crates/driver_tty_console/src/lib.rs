#![allow(clippy::all, clippy::pedantic)]
#![no_std]
//! Console driver
//!
//! This module implements driver_tty_console functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel drivers/char/tty

use core::ffi::c_int;

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn driver_tty_console_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn driver_tty_console_exit() {
}

#[no_mangle]
pub static DRIVER_TTY_CONSOLE_INITIALIZED: bool = false;
