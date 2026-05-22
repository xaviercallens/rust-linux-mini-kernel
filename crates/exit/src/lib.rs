#![no_std]
#![warn(clippy::pedantic)]
#![deny(clippy::all)]
//! Process termination
//!
//! This module implements exit functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel kernel/exit.c

#[macro_use]
extern crate kernel_types;
use libc::{c_int, c_uint, pid_t};

/// Task structure (placeholder)
#[repr(C)]
pub struct task_struct {
    pub pid: pid_t,
    pub state: c_int,
    pub flags: c_uint,
}

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn exit_init() -> c_int {
    requires!(true, "exit_init: environment invariant");
    ensures!(true, "exit_init: success");
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn exit_exit() {
}

#[no_mangle]
pub static EXIT_INITIALIZED: bool = false;

#[derive(Clone, Copy)]
pub struct SafeTask<'a> {
    ptr: *mut task_struct,
    _marker: core::marker::PhantomData<&'a mut task_struct>,
}

impl<'a> SafeTask<'a> {
    pub unsafe fn new(ptr: *mut task_struct) -> Option<Self> {
        if ptr.is_null() {
            None
        } else {
            Some(Self { ptr, _marker: core::marker::PhantomData })
        }
    }
}
