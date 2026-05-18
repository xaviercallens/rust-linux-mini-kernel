#![no_std]
#![no_main]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use core::ffi::c_int;
use core::ptr;
use core::sync::atomic::{AtomicPtr, Ordering};
use kernel_types::*;

pub const IPPROTO_ESP: u8 = 50;
pub const IPPROTO_AH: u8 = 51;
pub const IPPROTO_COMP: u8 = 108;

pub const INET6_PROTO_NOPOLICY: c_int = 1 << 0;
pub const ICMPV6_DEST_UNREACH: c_int = 1;
pub const ICMPV6_PORT_UNREACH: c_int = 4;

pub const EINVAL: c_int = -22;

pub const AF_INET6: c_int = 10;

unsafe extern "C" {
    fn icmpv6_send(skb: *mut sk_buff, type_: c_int, code: c_int, info: u32);
    fn kfree_skb(skb: *mut sk_buff);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm6_protocol {
    pub next: *mut xfrm6_protocol,
    pub priority: c_int,
    pub handler: extern "C" fn(*mut sk_buff) -> c_int,
    pub input_handler: extern "C" fn(*mut sk_buff, c_int, u32, c_int) -> c_int,
    pub cb_handler: extern "C" fn(*mut sk_buff, c_int) -> c_int,
    pub err_handler: extern "C" fn(*mut sk_buff, *mut sk_buff, u8, u8, c_int, u32) -> c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inet6_protocol {
    pub handler: extern "C" fn(*mut sk_buff) -> c_int,
    pub err_handler: extern "C" fn(*mut sk_buff, *mut sk_buff, u8, u8, c_int, u32) -> c_int,
    pub flags: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_input_afinfo {
    pub family: c_int,
    pub callback: extern "C" fn(*mut sk_buff, u8, c_int) -> c_int,
}

static esp6_handlers: AtomicPtr<xfrm6_protocol> = AtomicPtr::new(ptr::null_mut());
static ah6_handlers: AtomicPtr<xfrm6_protocol> = AtomicPtr::new(ptr::null_mut());
static ipcomp6_handlers: AtomicPtr<xfrm6_protocol> = AtomicPtr::new(ptr::null_mut());

unsafe fn proto_handlers(protocol: u8) -> *const AtomicPtr<xfrm6_protocol> {
    match protocol {
        IPPROTO_ESP => &esp6_handlers as *const AtomicPtr<xfrm6_protocol>,
        IPPROTO_AH => &ah6_handlers as *const AtomicPtr<xfrm6_protocol>,
        IPPROTO_COMP => &ipcomp6_handlers as *const AtomicPtr<xfrm6_protocol>,
        _ => ptr::null(),
    }
}

#[no_mangle]
pub extern "C" fn xfrm6_esp_rcv(_skb: *mut sk_buff) -> c_int {
    0
}

#[no_mangle]
pub extern "C" fn xfrm6_esp_err(
    _skb: *mut sk_buff,
    _opt: *mut sk_buff,
    _type_: u8,
    _code: u8,
    _offset: c_int,
    _info: u32,
) -> c_int {
    0
}

#[no_mangle]
pub extern "C" fn xfrm6_ah_rcv(_skb: *mut sk_buff) -> c_int {
    0
}

#[no_mangle]
pub extern "C" fn xfrm6_ah_err(
    _skb: *mut sk_buff,
    _opt: *mut sk_buff,
    _type_: u8,
    _code: u8,
    _offset: c_int,
    _info: u32,
) -> c_int {
    0
}

#[no_mangle]
pub extern "C" fn xfrm6_ipcomp_rcv(_skb: *mut sk_buff) -> c_int {
    0
}

#[no_mangle]
pub extern "C" fn xfrm6_ipcomp_err(
    _skb: *mut sk_buff,
    _opt: *mut sk_buff,
    _type_: u8,
    _code: u8,
    _offset: c_int,
    _info: u32,
) -> c_int {
    0
}

#[no_mangle]
pub unsafe extern "C" fn xfrm6_rcv_cb(skb: *mut sk_buff, protocol: u8, err: c_int) -> c_int {
    let headp = proto_handlers(protocol);
    if headp.is_null() {
        return 0;
    }

    let mut handler = (*headp).load(Ordering::Acquire);
    while !handler.is_null() {
        let ret = ((*handler).cb_handler)(skb, err);
        if ret <= 0 {
            return ret;
        }
        handler = (*handler).next;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn xfrm6_rcv_encap(
    skb: *mut sk_buff,
    nexthdr: c_int,
    spi: u32,
    encap_type: c_int,
) -> c_int {
    let headp = proto_handlers(nexthdr as u8);
    if !headp.is_null() {
        let mut handler = (*headp).load(Ordering::Acquire);
        while !handler.is_null() {
            let ret = ((*handler).input_handler)(skb, nexthdr, spi, encap_type);
            if ret != EINVAL {
                return ret;
            }
            handler = (*handler).next;
        }
    }

    icmpv6_send(skb, ICMPV6_DEST_UNREACH, ICMPV6_PORT_UNREACH, 0);
    kfree_skb(skb);
    0
}

#[no_mangle]
pub unsafe extern "C" fn xfrm6_protocol_register(
    handler: *mut xfrm6_protocol,
    protocol: u8,
) -> c_int {
    let headp = proto_handlers(protocol);
    if headp.is_null() || handler.is_null() {
        return EINVAL;
    }

    let old = (*headp).load(Ordering::Acquire);
    (*handler).next = old;
    (*headp).store(handler, Ordering::Release);
    0
}

#[no_mangle]
pub unsafe extern "C" fn xfrm6_protocol_deregister(
    handler: *mut xfrm6_protocol,
    protocol: u8,
) -> c_int {
    let headp = proto_handlers(protocol);
    if headp.is_null() || handler.is_null() {
        return EINVAL;
    }

    let mut cur = (*headp).load(Ordering::Acquire);
    let mut prev: *mut xfrm6_protocol = ptr::null_mut();

    while !cur.is_null() {
        if cur == handler {
            let next = (*cur).next;
            if prev.is_null() {
                (*headp).store(next, Ordering::Release);
            } else {
                (*prev).next = next;
            }
            return 0;
        }
        prev = cur;
        cur = (*cur).next;
    }

    EINVAL
}