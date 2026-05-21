#![no_std]
//! /dev filesystem
//!
//! This module implements devtmpfs functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel fs

use libc::c_int;

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn devtmpfs_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn devtmpfs_exit() {
}

#[no_mangle]
pub static DEVTMPFS_INITIALIZED: bool = false;
