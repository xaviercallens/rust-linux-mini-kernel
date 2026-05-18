#![no_std]
#![no_main]
#![allow(non_camel_case_types)]

use core::ffi::c_void;
use core::panic::PanicInfo;
use kernel_types::*;

pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ENOSYS: c_int = -38;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flowi6 {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Inet6SkbParm {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Icmp6Hdr {
    pub icmp6_type: u8,
    pub icmp6_code: u8,
    pub icmp6_cksum: u16,
}

#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    loop {
        core::hint::spin_loop();
    }
}

#[no_mangle]
pub unsafe extern "C" fn rust_eh_personality() {}

#[no_mangle]
pub unsafe extern "C" fn icmpv6_err(
    skb: *mut sk_buff,
    opt: *mut Inet6SkbParm,
    _type: u8,
    _code: u8,
    _offset: c_int,
    _info: u32,
) -> c_int {
    if skb.is_null() || opt.is_null() {
        return EINVAL;
    }
    0
}

fn icmpv6_xrlim_allow(sk: *mut c_void, _type: u8, fl6: *mut flowi6) -> bool {
    if sk.is_null() || fl6.is_null() {
        return false;
    }
    true
}

fn is_ineligible(skb: *const sk_buff) -> bool {
    skb.is_null()
}

#[no_mangle]
pub unsafe extern "C" fn icmp6_send(
    skb: *mut sk_buff,
    _type: u8,
    _code: u8,
    _info: u32,
) -> c_int {
    if skb.is_null() {
        return EINVAL;
    }
    if is_ineligible(skb as *const sk_buff) {
        return EINVAL;
    }
    let _ = icmpv6_xrlim_allow(core::ptr::null_mut(), _type, core::ptr::null_mut());
    0
}

#[no_mangle]
pub unsafe extern "C" fn ip6_err_gen_icmpv6_unreach(
    skb: *mut sk_buff,
    _type: u8,
    _code: u8,
) -> c_int {
    if skb.is_null() {
        return EINVAL;
    }
    0
}