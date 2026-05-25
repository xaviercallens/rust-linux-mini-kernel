#![allow(clippy::all, clippy::pedantic)]
#![no_std]
//! Kernel linked lists
//!
//! This module implements klist functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel lib

use core::ffi::c_int;

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn klist_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn klist_exit() {
}

#[no_mangle]
pub static KLIST_INITIALIZED: bool = false;
