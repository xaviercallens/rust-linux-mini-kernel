#![no_std]
//! TTY core
//!
//! This module implements driver_tty_core functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel drivers/char/tty

use libc::c_int;

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn driver_tty_core_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn driver_tty_core_exit() {
}

#[no_mangle]
pub static DRIVER_TTY_CORE_INITIALIZED: bool = false;
