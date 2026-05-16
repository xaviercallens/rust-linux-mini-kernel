//! IPv6 FIB Notifier Module
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use core::ffi::c_int;
use core::ffi::c_uint;
use core::ffi::c_void;
use core::ptr;

// Constants from C
pub const AF_INET6: c_int = 10;
pub const ENOMEM: c_int = -12;
pub const EINVAL: c_int = -22;

// Type definitions
#[repr(C)]
struct net;

#[repr(C)]
struct ipv6_net {
    notifier_ops: *mut fib_notifier_ops,
}

#[repr(C)]
struct net {
    ipv6: ipv6_net,
}

#[repr(C)]
struct notifier_block;

#[repr(C)]
struct fib_notifier_info {
    family: c_int,
    // Other fields (not accessed in this module)
}

#[repr(C)]
struct fib_notifier_ops {
    family: c_int,
    fib_seq_read: unsafe extern "C" fn(*mut net) -> c_uint,
    fib_dump: unsafe extern "C" fn(*mut net, *mut notifier_block, *mut c_void) -> c_int,
    owner: *mut c_void,
}

// Function pointers for FFI compatibility
extern "C" {
    fn call_fib_notifier(
        nb: *mut notifier_block,
        event_type: c_int,
        info: *mut fib_notifier_info,
    ) -> c_int;
    fn call_fib_notifiers(net: *mut net, event_type: c_int, info: *mut fib_notifier_info) -> c_int;
    fn fib_notifier_ops_register(
        ops: *const fib_notifier_ops,
        net: *mut net,
    ) -> *mut fib_notifier_ops;
    fn fib_notifier_ops_unregister(ops: *mut fib_notifier_ops);
    fn fib6_tables_seq_read(net: *mut net) -> c_uint;
    fn fib6_rules_seq_read(net: *mut net) -> c_uint;
    fn fib6_rules_dump(net: *mut net, nb: *mut notifier_block, extack: *mut c_void) -> c_int;
    fn fib6_tables_dump(net: *mut net, nb: *mut notifier_block, extack: *mut c_void) -> c_int;
    fn IS_ERR(ptr: *mut fib_notifier_ops) -> bool;
    fn PTR_ERR(ptr: *mut fib_notifier_ops) -> c_int;
}

// Static data
static FIB6_NOTIFIER_OPS_TEMPLATE: fib_notifier_ops = fib_notifier_ops {
    family: AF_INET6,
    fib_seq_read: fib6_seq_read,
    fib_dump: fib6_dump,
    owner: 0 as *mut c_void,
};

// Function implementations
#[no_mangle]
pub unsafe extern "C" fn call_fib6_notifier(
    nb: *mut notifier_block,
    event_type: c_int,
    info: *mut fib_notifier_info,
) -> c_int {
    // SAFETY: info is not null (validated by kernel)
    (*info).family = AF_INET6;
    call_fib_notifier(nb, event_type, info)
}

#[no_mangle]
pub unsafe extern "C" fn call_fib6_notifiers(
    net: *mut net,
    event_type: c_int,
    info: *mut fib_notifier_info,
) -> c_int {
    // SAFETY: info is not null (validated by kernel)
    (*info).family = AF_INET6;
    call_fib_notifiers(net, event_type, info)
}

#[no_mangle]
pub unsafe extern "C" fn fib6_seq_read(net: *mut net) -> c_uint {
    let tables_seq = fib6_tables_seq_read(net);
    let rules_seq = fib6_rules_seq_read(net);
    tables_seq + rules_seq
}

#[no_mangle]
pub unsafe extern "C" fn fib6_dump(
    net: *mut net,
    nb: *mut notifier_block,
    extack: *mut c_void,
) -> c_int {
    let mut err = fib6_rules_dump(net, nb, extack);
    if err != 0 {
        return err;
    }
    fib6_tables_dump(net, nb, extack)
}

#[no_mangle]
pub unsafe extern "C" fn fib6_notifier_init(net: *mut net) -> c_int {
    let ops = fib_notifier_ops_register(&FIB6_NOTIFIER_OPS_TEMPLATE, net);
    if IS_ERR(ops) {
        return PTR_ERR(ops);
    }
    // SAFETY: net is valid and has an ipv6 field with notifier_ops
    (*net).ipv6.notifier_ops = ops;
    0
}

#[no_mangle]
pub unsafe extern "C" fn fib6_notifier_exit(net: *mut net) {
    let ops = (*net).ipv6.notifier_ops;
    fib_notifier_ops_unregister(ops);
}

// Tests (conditional compilation)
#[cfg(test)]
mod tests {
    #[test]
    fn placeholder() {
        // No tests implemented for kernel module
    }
}
