#![allow(clippy::all, clippy::pedantic)]
#![no_std]
//! Red-black tree
//!
//! This module implements rbtree functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel lib

use core::ffi::c_int;

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn rbtree_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn rbtree_exit() {
}

#[no_mangle]
pub static RBTREE_INITIALIZED: bool = false;
