#![cfg_attr(not(test), no_std)]
//! Platform setup (x86_64) - Rust Linux Mini Kernel

#[allow(non_camel_case_types)]
type c_int = i32;

/// Initialize architecture-specific setup (x86_64).
///
/// Disables interrupts to ensure a safe boot environment.
///
/// # Safety
///
/// - Uses inline assembly to disable interrupts
/// - Must be called early in boot process
/// - Assumes x86_64 architecture
#[no_mangle]
pub unsafe extern "C" fn arch_setup_init() -> c_int {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    core::arch::asm!("cli", options(nomem, nostack));
    0
}

/// Cleanup function for arch_setup subsystem.
///
/// # Safety
///
/// - Currently a no-op, safe to call at any time
#[no_mangle]
pub unsafe extern "C" fn arch_setup_exit() {}

#[no_mangle]
pub static mut ARCH_SETUP_INITIALIZED: bool = false;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arch_setup_init() {
        unsafe {
            let result = arch_setup_init();
            assert_eq!(result, 0, "arch_setup_init should return 0");
        }
        // Note: The 'cli' instruction in the real implementation disables interrupts,
        // but in test mode (with std), this is just a no-op inline asm
    }

    #[test]
    fn test_arch_setup_exit_is_safe() {
        unsafe {
            arch_setup_exit(); // Should not panic
        }
    }

    #[test]
    fn test_arch_setup_initialized_flag() {
        unsafe {
            let initial = ARCH_SETUP_INITIALIZED;
            ARCH_SETUP_INITIALIZED = true;
            assert!(ARCH_SETUP_INITIALIZED);
            ARCH_SETUP_INITIALIZED = false;
            assert!(!ARCH_SETUP_INITIALIZED);
            ARCH_SETUP_INITIALIZED = initial; // Restore
        }
    }

    #[test]
    fn test_arch_setup_multiple_calls() {
        unsafe {
            // Should be safe to call multiple times
            let result1 = arch_setup_init();
            let result2 = arch_setup_init();
            assert_eq!(result1, 0);
            assert_eq!(result2, 0);
        }
    }
}
