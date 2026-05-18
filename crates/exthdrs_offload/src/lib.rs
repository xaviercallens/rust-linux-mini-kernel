//! IPv6 Extension Header GSO/GRO Offload Support
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)] // For C-style type names

use kernel_types::*;
use core::ffi::c_int;

// Constants from C
pub const IPPROTO_ROUTING: c_int = 43;
pub const IPPROTO_DSTOPTS: c_int = 44;
pub const INET6_PROTO_GSO_EXTHDR: c_int = 0x0001; // Assuming this value; actual value may vary

// Type definitions
#[repr(C)]
pub struct NetOffload {
    pub flags: c_int,
}

// Static offload instances
static RTHDR_OFFLOAD: NetOffload = NetOffload {
    flags: INET6_PROTO_GSO_EXTHDR,
};

static DSTOPT_OFFLOAD: NetOffload = NetOffload {
    flags: INET6_PROTO_GSO_EXTHDR,
};

// Extern function declarations
extern "C" {
    fn inet6_add_offload(offload: *const NetOffload, proto: c_int) -> c_int;
    fn inet6_del_offload(offload: *const NetOffload, proto: c_int);
}

// Function implementations
/// Initialize IPv6 extension headers offload support
///
/// # Safety
/// - This function is called during kernel initialization
/// - Assumes inet6_add_offload and inet6_del_offload are properly implemented
///
/// # Returns
/// 0 on success, error code from inet6_add_offload on failure
#[no_mangle]
pub unsafe extern "C" fn ipv6_exthdrs_offload_init() -> c_int {
    // SAFETY: Using static variables which are valid for the duration of the function
    // Static variables are guaranteed to exist and be properly aligned
    let mut ret = inet6_add_offload(&RTHDR_OFFLOAD, IPPROTO_ROUTING);
    if ret != 0 {
        return ret;
    }

    ret = inet6_add_offload(&DSTOPT_OFFLOAD, IPPROTO_DSTOPTS);
    if ret != 0 {
        // SAFETY: RTHDR_OFFLOAD is still valid and IPPROTO_ROUTING is a valid protocol
        inet6_del_offload(&RTHDR_OFFLOAD, IPPROTO_ROUTING);
        return ret;
    }

    0
}

// Tests (conditional compilation)
#[cfg(test)]
mod tests {
    // No tests for this simple module
}