#![no_std]
//! Semaphores
//!
//! This module implements ipc_sem functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel ipc

use kernel_types::*;
use libc::{c_int, c_uint, c_void, c_ulong, size_t};

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn ipc_sem_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn ipc_sem_exit() {
}

#[no_mangle]
pub static IPC_SEM_INITIALIZED: bool = false;
