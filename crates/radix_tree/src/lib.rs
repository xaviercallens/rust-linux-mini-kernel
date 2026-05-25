#![allow(clippy::all, clippy::pedantic)]
#![no_std]
//! Radix tree
//!
//! This module implements radix_tree functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel lib

use core::ffi::c_int;

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn radix_tree_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn radix_tree_exit() {
}

#[no_mangle]
pub static RADIX_TREE_INITIALIZED: bool = false;
