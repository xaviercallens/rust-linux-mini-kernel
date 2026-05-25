#![allow(clippy::all, clippy::pedantic)]
#![no_std]
//! Kernel threads
//!
//! This module implements kthread functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel kernel/kthread.c

use core::ffi::{c_int, c_uint};
type pid_t = i32;

/// Task structure (placeholder)
#[repr(C)]
pub struct task_struct {
    pub pid: pid_t,
    pub state: c_int,
    pub flags: c_uint,
}

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn kthread_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn kthread_exit() {
}

#[no_mangle]
pub static KTHREAD_INITIALIZED: bool = false;
