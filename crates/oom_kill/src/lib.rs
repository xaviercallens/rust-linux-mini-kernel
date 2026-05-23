#![no_std]
//! Out-of-memory killer
//!
//! This module implements oom_kill functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel mm/oom_kill.c

use core::ffi::c_int;

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn oom_kill_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn oom_kill_exit() {
}

// Placeholder exports for FFI compatibility
#[no_mangle]
pub static OOM_KILL_INITIALIZED: bool = false;
