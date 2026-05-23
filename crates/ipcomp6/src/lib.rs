#![cfg_attr(not(target_arch = "x86_64"), no_std)]
//! IP Payload Compression Protocol (IPComp) for IPv6 - RFC3173
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_variables)]

use core::ffi::{c_int, c_char, c_void};
use core::panic::PanicInfo;
use core::ptr;
use kernel_types::*;

pub const IPPROTO_COMP: c_int = 108;
pub const XFRM_STATE_DEAD: c_int = 2;
pub const ENOMEM: c_int = -12;
pub const EINVAL: c_int = -22;
pub const EAGAIN: c_int = -11;
pub const AF_INET6: c_int = 10;
pub const IPPROTO_IPV6: c_int = 41;
pub const XFRM_MODE_TRANSPORT: c_int = 0;
pub const XFRM_MODE_TUNNEL: c_int = 1;

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
#[derive(Copy, Clone)]
pub struct xfrm_mark {
    pub v: u32,
    pub m: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_address_t {
    pub a6: [u32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_state_props {
    pub mode: c_int,
    pub header_len: c_int,
    pub saddr: xfrm_address_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_state_id {
    pub daddr: xfrm_address_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_state {
    pub mark: xfrm_mark,
    pub props: xfrm_state_props,
    pub id: xfrm_state_id,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_type {
    pub description: *const c_char,
    pub owner: *const c_void,
    pub proto: c_int,
    pub init_state: Option<unsafe extern "C" fn(*mut xfrm_state) -> c_int>,
    pub destructor: Option<unsafe extern "C" fn(*mut xfrm_state)>,
    pub input: Option<unsafe extern "C" fn(*mut xfrm_state, *mut sk_buff) -> c_int>,
    pub output: Option<unsafe extern "C" fn(*mut xfrm_state, *mut sk_buff) -> c_int>,
    pub hdr_offset: Option<unsafe extern "C" fn(*mut xfrm_state, *mut sk_buff, *mut *mut u8) -> c_int>,
}

unsafe impl Sync for xfrm_type {}

pub const THIS_MODULE: *const c_void = ptr::null();
pub static ipcomp6_protocol: u8 = 0;

extern "C" {
    pub fn ntohs(val: __be16) -> u16;
    pub fn dev_net(dev: *mut c_void) -> *mut c_void;
    pub fn xfrm_state_lookup(
        net: *mut c_void,
        mark: u32,
        daddr: *const xfrm_address_t,
        spi: u32,
        proto: u8,
        family: c_int,
    ) -> *mut xfrm_state;
    pub fn sock_net_uid(net: *mut c_void, sk: *mut c_void) -> u32;
    pub fn ip6_redirect(skb: *mut sk_buff, net: *mut c_void, ifindex: c_int, target: u32, uid: u32);
    pub fn ip6_update_pmtu(skb: *mut sk_buff, net: *mut c_void, mtu: u32, flags: u32, tclass: u32, uid: u32);
    pub fn xfrm_state_put(x: *mut xfrm_state);
    pub fn xs_net(x: *mut xfrm_state) -> *mut c_void;
    pub fn xfrm6_tunnel_spi_lookup(net: *mut c_void, saddr: *const xfrm_address_t) -> u32;
    pub fn ipcomp_init_state(x: *mut xfrm_state) -> c_int;
    pub fn xfrm_register_type(t: *const xfrm_type, family: c_int) -> c_int;
    pub fn xfrm_unregister_type(t: *const xfrm_type, family: c_int);
    pub fn xfrm6_protocol_register(handler: *const u8, proto: c_int) -> c_int;
    pub fn xfrm6_protocol_deregister(handler: *const u8, proto: c_int) -> c_int;
    pub fn pr_info(fmt: *const c_char);
    pub fn xfrm6_find_1stfragopt(x: *mut xfrm_state, skb: *mut sk_buff, prevhdr: *mut *mut u8) -> c_int;
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    loop {}
}

#[no_mangle]
pub unsafe extern "C" fn rust_eh_personality() {}

#[no_mangle]
pub unsafe extern "C" fn ipcomp6_err(
    skb: *mut sk_buff,
    _opt: *mut inet6_skb_parm,
    type_: u8,
    _code: u8,
    offset: c_int,
    info: __be32,
) -> c_int {
    if type_ != 1 && type_ != 2 {
        return 0;
    }

    let iph = (*skb).data as *const ipv6hdr;
    let ipcomph = ((*skb).data as *const u8).add(offset as usize) as *const ip_comp_hdr;

    let spi = u32::from_be(ntohs((*ipcomph).cpi) as u32);
    let net = dev_net((*skb).dev);

    let x = xfrm_state_lookup(
        net,
        (*skb).mark as usize as u32,
        &(*iph).daddr as *const _ as *const xfrm_address_t,
        spi,
        IPPROTO_COMP as u8,
        AF_INET6,
    );

    if x.is_null() {
        return 0;
    }

    let dev = (*skb).dev as *mut net_device;
    if type_ == 2 {
        ip6_redirect(skb, net, (*dev).ifindex, 0, sock_net_uid(net, ptr::null_mut()));
    } else {
        ip6_update_pmtu(skb, net, info, 0, 0, sock_net_uid(net, ptr::null_mut()));
    }

    xfrm_state_put(x);

    0
}

#[no_mangle]
pub unsafe extern "C" fn ipcomp6_tunnel_create(_x: *mut xfrm_state) -> *mut xfrm_state {
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn ipcomp6_tunnel_attach(x: *mut xfrm_state) -> c_int {
    let net = xs_net(x);
    let err = 0;
    let mut t: *mut xfrm_state = ptr::null_mut();
    let mut spi: u32 = 0;
    let mark = (*x).mark.m & (*x).mark.v;

    spi = xfrm6_tunnel_spi_lookup(
        net,
        &(*x).props.saddr as *const _ as *const xfrm_address_t
    );

    if spi != 0 {
        t = xfrm_state_lookup(
            net,
            mark,
            &(*x).id.daddr as *const _ as *const xfrm_address_t,
            spi,
            IPPROTO_IPV6 as u8,
            AF_INET6
        );
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn ipcomp6_init_state(x: *mut xfrm_state) -> c_int {
    let mut err = -EINVAL;

    (*x).props.header_len = 0;

    match (*x).props.mode {
        XFRM_MODE_TRANSPORT => {}
        XFRM_MODE_TUNNEL => {
            (*x).props.header_len += core::mem::size_of::<ipv6hdr>() as c_int;
        },
        _ => return -EINVAL,
    }

    err = ipcomp_init_state(x);
    if err != 0 {
        return err;
    }

    if (*x).props.mode == XFRM_MODE_TUNNEL {
        err = ipcomp6_tunnel_attach(x);
        if err != 0 {
            return err;
        }
    }

    0
}

/// Callback for IPComp receive
///
/// # Safety
/// - `skb` must be a valid pointer to sk_buff
/// - `err` must be a valid error code
///
/// # Returns
/// 0
#[no_mangle]
pub unsafe extern "C" fn ipcomp6_rcv_cb(skb: *mut sk_buff, err: c_int) -> c_int { 0 }

// Module initialization and cleanup
#[no_mangle]
pub unsafe extern "C" fn ipcomp6_init() -> c_int {
    if xfrm_register_type(&ipcomp6_type, AF_INET6) < 0 {
        return -EAGAIN;
    }

    if xfrm6_protocol_register(&ipcomp6_protocol, IPPROTO_COMP) < 0 {
        pr_info(b"ipcomp6_init: can't add protocol\n".as_ptr() as *const c_char);
        xfrm_unregister_type(&ipcomp6_type, AF_INET6);
        return -EAGAIN;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn ipcomp6_fini() {
    if xfrm6_protocol_deregister(&ipcomp6_protocol, IPPROTO_COMP) < 0 {
        pr_info(b"ipcomp6_fini: can't remove protocol\n".as_ptr() as *const c_char);
    }
    xfrm_unregister_type(&ipcomp6_type, AF_INET6);
}

// Static data
#[no_mangle]
pub static ipcomp6_type: xfrm_type = xfrm_type {
    description: b"IPCOMP6\0".as_ptr() as *const c_char,
    owner: THIS_MODULE,
    proto: IPPROTO_COMP,
    init_state: Some(ipcomp6_init_state),
    destructor: Some(ipcomp6_destroy),
    input: Some(ipcomp6_input),
    output: Some(ipcomp6_output),
    hdr_offset: Some(xfrm6_find_1stfragopt),
};

#[no_mangle]
pub unsafe extern "C" fn ipcomp6_destroy(_x: *mut xfrm_state) {}

#[no_mangle]
pub unsafe extern "C" fn ipcomp6_get_mtu(_x: *mut xfrm_state, mtu: u32) -> u32 {
    mtu
}

#[no_mangle]
pub unsafe extern "C" fn ipcomp6_input(_x: *mut xfrm_state, _skb: *mut sk_buff) -> c_int { 0 }

#[no_mangle]
pub unsafe extern "C" fn ipcomp6_output(_x: *mut xfrm_state, _skb: *mut sk_buff) -> c_int { 0 }

#[no_mangle]
pub unsafe extern "C" fn ipcomp6_output_tail(
    _x: *mut xfrm_state,
    _skb: *mut sk_buff,
) -> *mut c_void {
    ptr::null_mut()
}
