//! IPv6 XFRM state management module
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use core::ffi::c_int;

// Constants from C
pub const AF_INET6: c_int = 10;
pub const IPPROTO_IPV6: c_int = 41;

// Type definitions
#[repr(C)]
struct sockaddr {
    // Placeholder for actual struct - defined in Linux kernel
    sa_family: c_int,
}

#[repr(C)]
struct sk_buff {
    // Placeholder for actual struct - defined in Linux kernel
}

#[repr(C)]
struct xfrm_state {
    // Placeholder for actual struct - defined in Linux kernel
}

// Function pointer types
type OutputFn = extern "C" fn(*mut xfrm_state, *mut sk_buff) -> c_int;
type TransportFinishFn = extern "C" fn(*mut sk_buff, *mut xfrm_state) -> c_int;
type LocalErrorFn = extern "C" fn(*mut sk_buff, *mut sockaddr, *mut xfrm_state) -> c_int;

#[repr(C)]
struct xfrm_state_afinfo {
    family: c_int,
    proto: c_int,
    output: OutputFn,
    transport_finish: TransportFinishFn,
    local_error: LocalErrorFn,
}

// External functions from kernel
extern "C" {
    fn xfrm_state_register_afinfo(info: *mut xfrm_state_afinfo) -> c_int;
    fn xfrm_state_unregister_afinfo(info: *mut xfrm_state_afinfo);
}

// External functions from other modules
extern "C" {
    fn xfrm6_output(x: *mut xfrm_state, skb: *mut sk_buff) -> c_int;
    fn xfrm6_transport_finish(skb: *mut sk_buff, x: *mut xfrm_state) -> c_int;
    fn xfrm6_local_error(skb: *mut sk_buff, addr: *mut sockaddr, x: *mut xfrm_state) -> c_int;
}

// Static variable - must be mutable to match C behavior
static mut xfrm6_state_afinfo: xfrm_state_afinfo = xfrm_state_afinfo {
    family: AF_INET6,
    proto: IPPROTO_IPV6,
    output: xfrm6_output,
    transport_finish: xfrm6_transport_finish,
    local_error: xfrm6_local_error,
};

// Function implementations
#[no_mangle]
pub unsafe extern "C" fn xfrm6_state_init() -> c_int {
    // SAFETY: The static variable is properly initialized and mutable
    xfrm_state_register_afinfo(&mut xfrm6_state_afinfo)
}

#[no_mangle]
pub unsafe extern "C" fn xfrm6_state_fini() {
    // SAFETY: The static variable is properly initialized and mutable
    xfrm_state_unregister_afinfo(&mut xfrm6_state_afinfo)
}

#[cfg(test)]
mod tests {
    // No tests for this simple module
}