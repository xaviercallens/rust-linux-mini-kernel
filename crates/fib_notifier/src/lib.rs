use kernel_types::*;

//! IPv4 FIB Notifier Implementation
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use core::ptr;
use libc::{c_int, c_uint, c_void};

// Constants from C
pub const AF_INET: c_int = 2;
pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;

// Type definitions
#[repr(C)]
pub struct NotifierBlock {
    // Opaque structure - fields not accessed directly
}

#[repr(C)]
pub struct FIBNotifierInfo {
    family: c_int,
    // Other fields not accessed in this implementation
}

#[repr(C)]
pub struct Net {
    ipv4: NetIPv4,
}

#[repr(C)]
pub struct NetIPv4 {
    fib_seq: c_uint,
    notifier_ops: *mut FIBNotifierOps,
}

#[repr(C)]
pub struct FIBNotifierOps {
    family: c_int,
    fib_seq_read: Option<unsafe extern "C" fn(net: *mut Net) -> c_uint>,
    fib_dump: Option<
        unsafe extern "C" fn(net: *mut Net, nb: *mut NotifierBlock, extack: *mut c_void) -> c_int,
    >,
    owner: *mut c_void,
}

#[repr(C)]
pub struct NetlinkExtAck {
    // Opaque structure - fields not accessed directly
}

// External function declarations
extern "C" {
    fn call_fib_notifier(
        nb: *mut NotifierBlock,
        event_type: c_int,
        info: *mut FIBNotifierInfo,
    ) -> c_int;
    fn call_fib_notifiers(net: *mut Net, event_type: c_int, info: *mut FIBNotifierInfo) -> c_int;
    fn fib4_rules_seq_read(net: *mut Net) -> c_uint;
    fn fib4_rules_dump(net: *mut Net, nb: *mut NotifierBlock, extack: *mut c_void) -> c_int;
    fn fib_notify(net: *mut Net, nb: *mut NotifierBlock, extack: *mut c_void) -> c_int;
    fn fib_notifier_ops_register(ops: *const FIBNotifierOps, net: *mut Net) -> *mut FIBNotifierOps;
    fn fib_notifier_ops_unregister(ops: *mut FIBNotifierOps);
}

// Static data
static FIB4_NOTIFIER_OPS_TEMPLATE: FIBNotifierOps = FIBNotifierOps {
    family: AF_INET,
    fib_seq_read: Some(fib4_seq_read),
    fib_dump: Some(fib4_dump),
    owner: ptr::null_mut(),
};

// Function implementations
#[no_mangle]
pub unsafe extern "C" fn call_fib4_notifier(
    nb: *mut NotifierBlock,
    event_type: c_int,
    info: *mut FIBNotifierInfo,
) -> c_int {
    // SAFETY: Caller must ensure info is valid
    (*info).family = AF_INET;
    call_fib_notifier(nb, event_type, info)
}

#[no_mangle]
pub unsafe extern "C" fn call_fib4_notifiers(
    net: *mut Net,
    event_type: c_int,
    info: *mut FIBNotifierInfo,
) -> c_int {
    // ASSERT_RTNL() - RTNL lock must be held by caller
    (*info).family = AF_INET;
    (*(*net).ipv4).fib_seq += 1;
    call_fib_notifiers(net, event_type, info)
}

#[no_mangle]
pub unsafe extern "C" fn fib4_seq_read(net: *mut Net) -> c_uint {
    // ASSERT_RTNL() - RTNL lock must be held by caller
    (*(*net).ipv4).fib_seq + fib4_rules_seq_read(net)
}

#[no_mangle]
pub unsafe extern "C" fn fib4_dump(
    net: *mut Net,
    nb: *mut NotifierBlock,
    extack: *mut c_void,
) -> c_int {
    let mut err: c_int = 0;
    err = fib4_rules_dump(net, nb, extack);
    if err != 0 {
        return err;
    }
    fib_notify(net, nb, extack)
}

#[no_mangle]
pub unsafe extern "C" fn fib4_notifier_init(net: *mut Net) -> c_int {
    (*(*net).ipv4).fib_seq = 0;

    let ops = fib_notifier_ops_register(&FIB4_NOTIFIER_OPS_TEMPLATE as *const FIBNotifierOps, net);
    if ops.is_null() {
        return PTR_ERR(ops);
    }

    (*(*net).ipv4).notifier_ops = ops;
    0
}

#[no_mangle]
pub unsafe extern "C" fn fib4_notifier_exit(net: *mut Net) {
    let ops = (*(*net).ipv4).notifier_ops;
    fib_notifier_ops_unregister(ops);
}

// Helper function for error handling
unsafe fn PTR_ERR<T>(ptr: *mut T) -> c_int {
    (ptr as *mut c_void as *mut c_int).offset_from(ptr::null_mut() as *mut c_void) as c_int
}
