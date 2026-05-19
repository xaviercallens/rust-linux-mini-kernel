#![no_std]
//! Kernel logging - Phase 1: Serial port output via QEMU

#[allow(non_camel_case_types)]
type c_int = i32;

extern "C" {
    fn x86_out8(port: u16, value: u8);
    fn x86_in8(port: u16) -> u8;
}

const SERIAL_PORT: u16 = 0x3F8;

#[inline]
unsafe fn serial_write_byte(byte: u8) {
    while (x86_in8(SERIAL_PORT + 5) & 0x20) == 0 {}
    x86_out8(SERIAL_PORT, byte);
}

#[no_mangle]
pub unsafe extern "C" fn printk_str(s: *const u8, len: usize) {
    if !s.is_null() {
        core::slice::from_raw_parts(s, len).iter().for_each(|&b| serial_write_byte(b));
    }
}

#[no_mangle]
pub unsafe extern "C" fn printk_cstr(s: *const u8) {
    if !s.is_null() {
        let mut p = s;
        while *p != 0 { serial_write_byte(*p); p = p.add(1); }
    }
}

#[no_mangle]
pub unsafe extern "C" fn printk_init() -> c_int {
    x86_out8(SERIAL_PORT + 1, 0x00);
    x86_out8(SERIAL_PORT + 3, 0x80);
    x86_out8(SERIAL_PORT, 0x0C);
    x86_out8(SERIAL_PORT + 1, 0x00);
    x86_out8(SERIAL_PORT + 3, 0x03);
    x86_out8(SERIAL_PORT + 2, 0xC7);
    x86_out8(SERIAL_PORT + 4, 0x0B);
    0
}

#[no_mangle]
pub unsafe extern "C" fn printk_exit() {}

#[no_mangle]
pub static mut PRINTK_INITIALIZED: bool = false;
