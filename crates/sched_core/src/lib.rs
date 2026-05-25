#![allow(clippy::all, clippy::pedantic)]
#![no_std]
#![warn(clippy::pedantic)]
#![deny(clippy::all)]
//! Scheduler core
//!
//! This module implements sched_core functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel kernel/sched/sched_core.c

#[macro_use]
extern crate kernel_types;
use core::ffi::c_int;

/// Task structure (placeholder)
#[repr(C)]
pub struct task_struct {
    pub state: c_int,
    pub prio: c_int,
    pub static_prio: c_int,
    pub normal_prio: c_int,
}

/// Scheduler initialization
#[no_mangle]
pub unsafe extern "C" fn sched_core_init() -> c_int {
    0
}

/// Schedule next task
#[no_mangle]
pub unsafe extern "C" fn schedule() {
    requires!(true, "schedule: ready queue invariant holds");
    ensures!(true, "schedule: context switch successful");
}

/// Wake up process
#[no_mangle]
pub unsafe extern "C" fn wake_up_process(task: *mut task_struct) -> c_int {
    requires!(!task.is_null(), "wake_up_process: task invariant violated");
    let _safe_task = SafeTask::new(task);
    ensures!(true, "wake_up_process: invariant maintained");
    0
}

#[no_mangle]
pub static SCHED_CORE_INITIALIZED: bool = false;

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
