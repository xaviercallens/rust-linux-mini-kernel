#![allow(clippy::all, clippy::pedantic)]
#![no_std]
//! Command line parsing
//!
//! This module implements cmdline functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel lib

use core::ffi::c_int;

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn cmdline_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn cmdline_exit() {
}

#[no_mangle]
pub static CMDLINE_INITIALIZED: bool = false;
