#![allow(clippy::all, clippy::pedantic)]
#![no_std]
//! /dev/random, /dev/urandom
//!
//! This module implements driver_char_random functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel drivers/char

use core::ffi::c_int;

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn driver_char_random_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn driver_char_random_exit() {
}

#[no_mangle]
pub static DRIVER_CHAR_RANDOM_INITIALIZED: bool = false;
