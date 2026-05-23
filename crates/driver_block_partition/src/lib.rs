#![no_std]
//! Partition handling
//!
//! This module implements driver_block_partition functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel drivers/block

use core::ffi::c_int;

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn driver_block_partition_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn driver_block_partition_exit() {
}

#[no_mangle]
pub static DRIVER_BLOCK_PARTITION_INITIALIZED: bool = false;
