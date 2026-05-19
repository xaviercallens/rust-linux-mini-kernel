#![no_std]
//! Platform setup (x86_64) - Rust Linux Mini Kernel

#[allow(non_camel_case_types)]
type c_int = i32;

#[no_mangle]
pub unsafe extern "C" fn arch_setup_init() -> c_int {
    core::arch::asm!("cli", options(nomem, nostack));
    0
}

#[no_mangle]
pub unsafe extern "C" fn arch_setup_exit() {}

#[no_mangle]
pub static mut ARCH_SETUP_INITIALIZED: bool = false;
