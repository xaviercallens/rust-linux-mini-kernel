#![no_std]
#![no_main]
#![allow(non_camel_case_types)]

use core::ffi::{c_int, c_void};
use core::panic::PanicInfo;
use core::ptr;
use kernel_types::*;

pub const IPPROTO_COMP: c_int = 108;
pub const IPPROTO_IPV6: c_int = 41;
pub const AF_INET6: c_int = 10;
pub const EINVAL: c_int = 22;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_comp_hdr {
    pub cpi: __be16,
}

#[repr(C)]
pub struct inet6_skb_parm {
    _priv: [u8; 0],
}

#[repr(C)]
pub struct xfrm_state {
    _priv: [u8; 0],
}

#[repr(C)]
pub struct sk_buff {
    _priv: [u8; 0],
}

#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    loop {}
}

#[no_mangle]
pub unsafe extern "C" fn rust_eh_personality() {}

#[no_mangle]
pub unsafe extern "C" fn ipcomp6_err(
    _skb: *mut sk_buff,
    _opt: *mut inet6_skb_parm,
    _type: u8,
    _code: u8,
    _offset: c_int,
    _info: __be32,
) -> c_int {
    0
}

#[no_mangle]
pub unsafe extern "C" fn ipcomp6_tunnel_create(_x: *mut xfrm_state) -> *mut xfrm_state {
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn ipcomp6_tunnel_attach(x: *mut xfrm_state) -> c_int {
    if x.is_null() {
        return -EINVAL;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn ipcomp6_init_state(x: *mut xfrm_state) -> c_int {
    if x.is_null() {
        return -EINVAL;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn ipcomp6_destroy(_x: *mut xfrm_state) {}

#[no_mangle]
pub unsafe extern "C" fn ipcomp6_get_mtu(_x: *mut xfrm_state, mtu: u32) -> u32 {
    mtu
}

#[no_mangle]
pub unsafe extern "C" fn ipcomp6_input(_x: *mut xfrm_state, _skb: *mut sk_buff) -> c_int {
    0
}

#[no_mangle]
pub unsafe extern "C" fn ipcomp6_output(_x: *mut xfrm_state, _skb: *mut sk_buff) -> c_int {
    0
}

#[no_mangle]
pub unsafe extern "C" fn ipcomp6_output_tail(
    _x: *mut xfrm_state,
    _skb: *mut sk_buff,
) -> *mut c_void {
    ptr::null_mut()
}