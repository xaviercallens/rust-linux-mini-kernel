#![allow(clippy::all, clippy::pedantic)]
#![no_std]
//! Anonymous inodes
//!
//! This module implements anon_inodes functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel fs

use core::ffi::c_int;

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn anon_inodes_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn anon_inodes_exit() {
}

#[no_mangle]
pub static ANON_INODES_INITIALIZED: bool = false;
