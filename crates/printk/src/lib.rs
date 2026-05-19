#![no_std]
//! Kernel logging
//!
//! This module implements printk functionality for the Rust Linux Mini Kernel.
//! Based on Linux kernel kernel
//!
//! Phase 1 implementation: Simple serial port output via QEMU
//! Uses external assembly helpers for x86 port I/O

// Type alias for C compatibility
#[allow(non_camel_case_types)]
type c_int = i32;

// External assembly functions for port I/O (defined in entry.s or separate asm file)
extern "C" {
    fn x86_out8(port: u16, value: u8);
    fn x86_in8(port: u16) -> u8;
}

// Serial port I/O addresses (COM1)
const SERIAL_PORT: u16 = 0x3F8;
const SERIAL_DATA: u16 = SERIAL_PORT;
const SERIAL_STATUS: u16 = SERIAL_PORT + 5;

/// Write a byte to the serial port
#[inline]
unsafe fn serial_write_byte(byte: u8) {
    // Wait for transmit buffer to be empty (bit 5 of line status register)
    while (x86_in8(SERIAL_STATUS) & 0x20) == 0 {}
    x86_out8(SERIAL_DATA, byte);
}

/// Print a string to serial console
#[no_mangle]
pub unsafe extern "C" fn printk_str(s: *const u8, len: usize) {
    if s.is_null() {
        return;
    }

    let slice = core::slice::from_raw_parts(s, len);
    for &byte in slice {
        serial_write_byte(byte);
    }
}

/// Print a null-terminated C string
#[no_mangle]
pub unsafe extern "C" fn printk_cstr(s: *const u8) {
    if s.is_null() {
        return;
    }

    let mut ptr = s;
    while *ptr != 0 {
        serial_write_byte(*ptr);
        ptr = ptr.add(1);
    }
}

/// Module initialization - setup serial port
#[no_mangle]
pub unsafe extern "C" fn printk_init() -> c_int {
    // Initialize COM1 serial port (8N1, 9600 baud)
    x86_out8(SERIAL_PORT + 1, 0x00);    // Disable interrupts
    x86_out8(SERIAL_PORT + 3, 0x80);    // Enable DLAB (set baud rate divisor)
    x86_out8(SERIAL_PORT + 0, 0x0C);    // Divisor low byte (9600 baud)
    x86_out8(SERIAL_PORT + 1, 0x00);    // Divisor high byte
    x86_out8(SERIAL_PORT + 3, 0x03);    // 8 bits, no parity, one stop bit
    x86_out8(SERIAL_PORT + 2, 0xC7);    // Enable FIFO, clear with 14-byte threshold
    x86_out8(SERIAL_PORT + 4, 0x0B);    // IRQs enabled, RTS/DSR set

    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn printk_exit() {
    // Nothing to cleanup for serial port
}

#[no_mangle]
pub static mut PRINTK_INITIALIZED: bool = false;
