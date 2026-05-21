#![no_std]
//! Message queues
//!
//! This module implements ipc_msg functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel ipc

use libc::c_int;

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn ipc_msg_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn ipc_msg_exit() {
}

#[no_mangle]
pub static IPC_MSG_INITIALIZED: bool = false;
