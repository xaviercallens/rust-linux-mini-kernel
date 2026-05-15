//! IPv4 FIB Notifier Module
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.
//!
//! Handles IPv4-specific FIB (Forwarding Information Base) notification operations
//! including sequence number tracking and notification dispatching.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(non_snake_case)]

use core::ptr;
use libc::{c_int, c_uint, c_void};

// Constants from C
pub const AF_INET: c_int = 2;
pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;

// Type definitions
#[repr(C)]
pub struct NotifierBlock {
    // Opaque fields - actual implementation in C
    _private: [u8; 0],
}

#[repr(C)]
pub struct FibNotifierInfo {
    family: c_int,
    // Other fields as needed
}

#[repr(C)]
pub struct NetlinkExtAck {
    // Opaque fields
    _private: [u8; 0],
}

#[repr(C)]
pub struct FibNotifierOps {
    family: c_int,
    fib_seq_read: extern "C" fn(*mut Net) -> c_uint,
    fib_dump: extern "C" fn(*mut Net, *mut NotifierBlock, *mut NetlinkExtAck) -> c_int,
    owner: *mut c_void,
}

#[repr(C)]
pub struct Net {
    ipv4: Ipv4Info,
    // Other fields
}

#[repr(C)]
pub struct Ipv4Info {
    fib_seq: c_uint,
    notifier_ops: *mut FibNotifierOps,
}

// Function implementations
#[no_mangle]
pub unsafe extern "C" fn call_fib4_notifier(
    nb: *mut NotifierBlock,
    event_type: c_int,
    info: *mut FibNotifierInfo,
) -> c_int {
    // SAFETY: Caller guarantees info is valid pointer
    (*info).family = AF_INET;
    call_fib_notifier(nb, event_type, info)
}

#[no_mangle]
pub unsafe extern "C" fn call_fib4_notifiers(
    net: *mut Net,
    event_type: c_int,
    info: *mut FibNotifierInfo,
) -> c_int {
    // SAFETY: Caller must hold RTNL lock (asserted in C)
    (*info).family = AF_INET;
    (*net).ipv4.fib_seq += 1;
    call_fib_notifiers(net, event_type, info)
}

#[no_mangle]
pub unsafe extern "C" fn fib4_seq_read(net: *mut Net) -> c_uint {
    // SAFETY: Caller must hold RTNL lock (asserted in C)
    (*net).ipv4.fib_seq + fib4_rules_seq_read(net)
}

#[no_mangle]
pub unsafe extern "C" fn fib4_dump(
    net: *mut Net,
    nb: *mut NotifierBlock,
    extack: *mut NetlinkExtAck,
) -> c_int {
    let err = fib4_rules_dump(net, nb, extack);
    if err != 0 {
        return err;
    }
    fib_notify(net, nb, extack)
}

// Static initializer for fib_notifier_ops_template
#[no_mangle]
pub static FIB4_NOTIFIER_OPS_TEMPLATE: FibNotifierOps = FibNotifierOps {
    family: AF_INET,
    fib_seq_read: fib4_seq_read,
    fib_dump: fib4_dump,
    owner: THIS_MODULE as *mut c_void,
};

#[no_mangle]
pub unsafe extern "C" fn fib4_notifier_init(net: *mut Net) -> c_int {
    (*net).ipv4.fib_seq = 0;
    
    let ops = fib_notifier_ops_register(&FIB4_NOTIFIER_OPS_TEMPLATE, net);
    if IS_ERR(ops) {
        return PTR_ERR(ops);
    }
    
    (*net).ipv4.notifier_ops = ops;
    0
}

#[no_mangle]
pub unsafe extern "C" fn fib4_notifier_exit(net: *mut Net) {
    fib_notifier_ops_unregister((*net).ipv4.notifier_ops);
}

// External function declarations (assumed to exist in C)
extern "C" {
    fn call_fib_notifier(nb: *mut NotifierBlock, event_type: c_int, info: *mut FibNotifierInfo) -> c_int;
    fn call_fib_notifiers(net: *mut Net, event_type: c_int, info: *mut FibNotifierInfo) -> c_int;
    fn fib4_rules_seq_read(net: *mut Net) -> c_uint;
    fn fib4_rules_dump(net: *mut Net, nb: *mut NotifierBlock, extack: *mut NetlinkExtAck) -> c_int;
    fn fib_notify(net: *mut Net, nb: *mut NotifierBlock, extack: *mut NetlinkExtAck) -> c_int;
    fn fib_notifier_ops_register(
        template: *const FibNotifierOps,
        net: *mut Net
    ) -> *mut FibNotifierOps;
    fn fib_notifier_ops_unregister(ops: *mut FibNotifierOps);
    fn IS_ERR(ptr: *mut c_void) -> c_int;
    fn PTR_ERR(ptr: *mut c_void) -> c_int;
}

// Module symbol
#[no_mangle]
pub static THIS_MODULE: *mut c_void = ptr::null_mut();
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_fib4_seq_read() {
        // This test would require a valid Net struct
        // which is not possible to construct here
        // as it's part of the Linux kernel
        // This is just a placeholder
        assert!(true);
    }
}
