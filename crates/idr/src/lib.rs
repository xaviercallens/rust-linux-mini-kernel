#![allow(clippy::all, clippy::pedantic)]
#![no_std]
//! ID allocation
//!
//! This module implements idr functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel lib

use core::ffi::c_int;

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn idr_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn idr_exit() {
}

#[no_mangle]
pub static IDR_INITIALIZED: bool = false;
