#![no_std]
//! start_kernel() entry point - Rust Linux Mini Kernel

#[allow(non_camel_case_types)]
type c_int = i32;

extern "C" {
    fn printk_init() -> c_int;
    fn printk_str(s: *const u8, len: usize);
    fn arch_setup_init() -> c_int;
}

#[inline]
unsafe fn print(msg: &[u8]) {
    printk_str(msg.as_ptr(), msg.len());
}

#[no_mangle]
pub unsafe extern "C" fn start_kernel() -> ! {
    printk_init();
    print(b"Rust Linux Mini Kernel v8.2.0 booting...\n");

    if arch_setup_init() != 0 {
        print(b"PANIC: arch_setup_init failed\n");
        loop {}
    }

    print(b"Architecture initialized\n");
    print(b"Kernel panic - Phase 1 boot complete!\n");

    loop { core::arch::asm!("hlt", options(nomem, nostack)); }
}

#[no_mangle]
pub unsafe extern "C" fn init_main_init() -> c_int { 0 }

#[no_mangle]
pub unsafe extern "C" fn init_main_exit() {}

#[no_mangle]
pub static mut INIT_MAIN_INITIALIZED: bool = false;
