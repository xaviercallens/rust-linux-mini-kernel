#![no_std]
//! PCI bus driver
//!
//! This module implements driver_pci_core functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel drivers/pci

use libc::c_int;

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn driver_pci_core_init() -> c_int {
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn driver_pci_core_exit() {
}

#[no_mangle]
pub static DRIVER_PCI_CORE_INITIALIZED: bool = false;
