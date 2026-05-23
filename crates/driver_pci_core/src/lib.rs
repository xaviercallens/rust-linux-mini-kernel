#![no_std]
#![warn(clippy::pedantic)]
#![deny(clippy::all)]
//! PCI bus driver
//!
//! This module implements driver_pci_core functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel drivers/pci

use core::ffi::c_int;

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

/// A safe wrapper around raw `pci_dev` pointers.
#[repr(transparent)]
pub struct SafePciDevice(*mut core::ffi::c_void);

impl SafePciDevice {
    /// Creates a new `SafePciDevice` from a raw pointer.
    ///
    /// # Safety
    /// The caller must ensure that the pointer is valid and properly aligned.
    #[must_use]
    pub unsafe fn new(ptr: *mut core::ffi::c_void) -> Self {
        Self(ptr)
    }

    /// Returns the underlying raw pointer.
    #[must_use]
    pub fn as_ptr(&self) -> *mut core::ffi::c_void {
        self.0
    }
}
