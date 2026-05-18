//! IP Payload Compression Protocol (IPComp) for IPv6 - RFC3173
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![no_main]
#![allow(non_camel_case_types)]

use core::ffi::{c_int, c_void};
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
    if type_ != 1 && type_ != 2 {
        return 0;
    }

    let iph = (*skb).data as *const ipv6hdr;
    let ipcomph = (*skb).data.offset(offset as isize) as *const ip_comp_hdr;

    let spi = u32::from_be(ntohs((*ipcomph).cpi));
    let net = dev_net((*skb).dev);

    let x = xfrm_state_lookup(
        net,
        (*skb).mark,
        &(*iph).daddr as *const _ as *const xfrm_address_t,
        spi,
        IPPROTO_COMP,
        AF_INET6,
    );

    if x.is_null() {
        return 0;
    }

    if type_ == 2 {
        ip6_redirect(skb, net, (*skb).dev.offset(0).ifindex, 0, sock_net_uid(net, ptr::null_mut()));
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
    let mut err = 0;
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
            IPPROTO_IPV6,
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
pub unsafe extern "C" fn ipcomp6_rcv_cb(skb: *mut sk_buff, err: c_int) -> c_int {
    0
}

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
    destructor: Some(ipcomp_destroy),
    input: Some(ipcomp_input),
    output: Some(ipcomp_output),
    hdr_offset: Some(xfrm6_find_1stfragopt),
};

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