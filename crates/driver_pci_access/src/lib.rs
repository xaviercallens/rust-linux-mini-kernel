#![no_std]
//! PCI config space
//!
//! This module implements driver_pci_access functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel drivers/pci

use kernel_types::*;
use libc::{c_int, c_uint, c_void, c_ulong, size_t};

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn driver_pci_access_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn driver_pci_access_exit() {
}

#[no_mangle]
pub static DRIVER_PCI_ACCESS_INITIALIZED: bool = false;
