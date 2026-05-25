#![allow(clippy::all, clippy::pedantic)]
#![no_std]
//! /proc helpers
//!
//! This module implements proc_generic functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel fs/proc

use core::ffi::c_int;

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn proc_generic_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn proc_generic_exit() {
}

#[no_mangle]
pub static PROC_GENERIC_INITIALIZED: bool = false;
