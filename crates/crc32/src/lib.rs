#![no_std]
//! CRC32 checksums
//!
//! This module implements crc32 functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel lib

use core::ffi::c_int;

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn crc32_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn crc32_exit() {
}

#[no_mangle]
pub static CRC32_INITIALIZED: bool = false;
