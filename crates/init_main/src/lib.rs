#![no_std]
//! start_kernel() entry point
//!
//! This module implements init_main functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel init

// Type alias for C compatibility
#[allow(non_camel_case_types)]
type c_int = i32;

// External functions from other modules
extern "C" {
    fn printk_init() -> c_int;
    fn printk_str(s: *const u8, len: usize);
    fn arch_setup_init() -> c_int;
}

/// Main kernel entry point - called from assembly
#[no_mangle]
pub unsafe extern "C" fn start_kernel() -> ! {
    // Initialize serial console first so we can print
    printk_init();

    // Print boot banner
    let banner = b"Rust Linux Mini Kernel v8.2.0 booting...\n";
    printk_str(banner.as_ptr(), banner.len());

    // Initialize architecture-specific setup
    if arch_setup_init() != 0 {
        let error = b"PANIC: arch_setup_init failed\n";
        printk_str(error.as_ptr(), error.len());
        loop {}
    }

    let msg = b"Architecture initialized\n";
    printk_str(msg.as_ptr(), msg.len());

    // Success message
    let success = b"Kernel panic - Phase 1 boot complete!\n";
    printk_str(success.as_ptr(), success.len());

    // Halt - Phase 1 stops here intentionally
    loop {
        core::arch::asm!("hlt", options(nomem, nostack));
    }
}

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn init_main_init() -> c_int {
    // This module doesn't need separate init - start_kernel is the entry
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn init_main_exit() {
    // Never reached - kernel doesn't exit
}

#[no_mangle]
pub static mut INIT_MAIN_INITIALIZED: bool = false;
