#![no_std]
//! POSIX message queues
//!
//! This module implements ipc_mqueue functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel ipc

use libc::c_int;

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn ipc_mqueue_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn ipc_mqueue_exit() {
}

#[no_mangle]
pub static IPC_MQUEUE_INITIALIZED: bool = false;
