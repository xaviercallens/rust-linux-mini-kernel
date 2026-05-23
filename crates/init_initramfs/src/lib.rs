#![no_std]
//! Initial RAM filesystem
//!
//! This module implements init_initramfs functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel init

use core::ffi::c_int;

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn init_initramfs_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn init_initramfs_exit() {
}

#[no_mangle]
pub static INIT_INITRAMFS_INITIALIZED: bool = false;
