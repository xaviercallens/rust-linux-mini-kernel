//! IPv6 XFRM state management module
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use core::ffi::c_int;
use core::panic::PanicInfo;
use kernel_types::*;

pub const AF_INET6: c_int = 10;
pub const IPPROTO_IPV6: c_int = 41;

// Function pointer types
type OutputFn = extern "C" fn(*mut c_void, *mut c_void) -> c_int;
type TransportFinishFn = extern "C" fn(*mut c_void, *mut c_void) -> c_int;
type LocalErrorFn = extern "C" fn(*mut c_void, *mut sockaddr, *mut c_void) -> c_int;

#[repr(C)]
pub struct xfrm_state {
    _priv: [u8; 0],
}

#[repr(C)]
pub struct sk_buff {
    _priv: [u8; 0],
}

#[repr(C)]
pub struct sockaddr {
    _priv: [u8; 0],
}

#[repr(C)]
pub struct xfrm_state_afinfo {
    family: c_int,
    proto: c_int,
    output: OutputFn,
    transport_finish: TransportFinishFn,
    local_error: LocalErrorFn,
}

unsafe extern "C" {
    fn xfrm_state_register_afinfo(info: *mut xfrm_state_afinfo) -> c_int;
    fn xfrm_state_unregister_afinfo(info: *mut xfrm_state_afinfo);

// External functions from other modules
extern "C" {
    fn xfrm6_output(x: *mut c_void, skb: *mut c_void) -> c_int;
    fn xfrm6_transport_finish(skb: *mut c_void, x: *mut c_void) -> c_int;
    fn xfrm6_local_error(skb: *mut c_void, addr: *mut sockaddr, x: *mut c_void) -> c_int;
}

static mut XFRM6_STATE_AFINFO: xfrm_state_afinfo = xfrm_state_afinfo {
    family: AF_INET6,
    proto: IPPROTO_IPV6,
    output: xfrm6_output,
    transport_finish: xfrm6_transport_finish,
    local_error: xfrm6_local_error,
};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xfrm6_state_init() -> c_int {
    unsafe { xfrm_state_register_afinfo(&raw mut XFRM6_STATE_AFINFO) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xfrm6_state_fini() {
    unsafe { xfrm_state_unregister_afinfo(&raw mut XFRM6_STATE_AFINFO) }
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    loop {}
}
```