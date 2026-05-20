#![no_std]
//! pipe syscall
//!
//! This module implements sys_pipe functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel syscalls

use kernel_types::*;
use libc::{c_int, c_uint, c_void, c_ulong, size_t};

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn sys_pipe_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn sys_pipe_exit() {
}

#[no_mangle]
pub static SYS_PIPE_INITIALIZED: bool = false;
