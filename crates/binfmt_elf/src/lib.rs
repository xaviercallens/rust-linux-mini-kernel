#![no_std]
//! ELF binary format
//!
//! This module implements binfmt_elf functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel fs

use kernel_types::*;
use libc::{c_int, c_uint, c_void, c_ulong, size_t};

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn binfmt_elf_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn binfmt_elf_exit() {
}

#[no_mangle]
pub static BINFMT_ELF_INITIALIZED: bool = false;
