```rust
#![no_std]
#![no_main]
#![allow(non_camel_case_types)]

use core::ffi::{c_int, c_uint};
use core::panic::PanicInfo;

pub mod kernel_types {
    pub type size_t = usize;
    pub type c_size_t = usize;
    pub type socklen_t = u32;
}

use kernel_types::*;

pub const EMSGSIZE: c_int = -90;
pub const XFRM_MODE_TUNNEL: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_state_props {
    pub mode: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_state {
    pub props: xfrm_state_props,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dst_entry {
    pub xfrm: *mut xfrm_state,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sock {
    pub sk_bound_dev_if: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct net {
    _priv: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flowi6 {
    pub flowi6_oif: c_int,
    pub daddr: u32,
    pub fl6_dport: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipv6hdr {
    pub daddr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sk_buff {
    pub sk: *mut sock,
    pub len: u32,
    pub protocol: u16,
    pub encapsulation: u8,
    _pad: [u8; 0],
}

extern "C" {
    fn ip6_find_1stfragopt(skb: *mut sk_buff, prevhdr: *mut *mut u8) -> c_int;
    fn ipv6_local_rxpmtu(sk: *mut sock, fl: *mut flowi6, mtu: c_uint);
    fn ipv6_local_error(sk: *mut sock, code: c_int, fl: *mut flowi6, mtu: c_uint);
    fn xfrm_output(sk: *mut sock, skb: *mut sk_buff) -> c_int;
    fn ip6_fragment(
        net: *mut net,
        sk: *mut sock,
        skb: *mut sk_buff,
        okfn: extern "C" fn(*mut net, *mut sock, *mut sk_buff) -> c_int,
    ) -> c_int;
    fn dst_output(net: *mut net, sk: *mut sock, skb: *mut sk_buff) -> c_int;
    fn skb_dst(skb: *mut sk_buff) -> *mut dst_entry;
    fn ipv6_hdr(skb: *mut sk_buff) -> *mut ipv6hdr;
    fn inner_ipv6_hdr(skb: *mut sk_buff) -> *mut ipv6hdr;
}

#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xfrm6_find_1stfragopt(
    _x: *mut xfrm_state,
    skb: *mut sk_buff,
    prevhdr: *mut *mut u8,
) -> c_int {
    ip6_find_1stfragopt(skb, prevhdr)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xfrm6_local_rxpmtu(skb: *mut sk_buff, mtu: c_uint) {
    let sk = (*skb).sk;
    if sk.is_null() {
        return;
    }

    let mut fl6 = flowi6 {
        flowi6_oif: (*sk).sk_bound_dev_if,
        daddr: (*ipv6_hdr(skb)).daddr,
        fl6_dport: 0,
    };

    ipv6_local_rxpmtu(sk, &mut fl6, mtu);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xfrm6_local_error(skb: *mut sk_buff, mtu: c_uint) {
    let sk = (*skb).sk;
    if sk.is_null() {
        return;
    }

    let hdr = if (*skb).encapsulation != 0 {
        inner_ipv6_hdr(skb)
    } else {
        ipv6_hdr(skb)
    };

    let mut fl6 = flowi6 {
        flowi6_oif: (*sk).sk_bound_dev_if,
        daddr: (*hdr).daddr,
        fl6_dport: 0,
    };

    ipv6_local_error(sk, EMSGSIZE, &mut fl6, mtu);
}

extern "C" fn __xfrm6_output_finish(net: *mut net, sk: *mut sock, skb: *mut sk_buff) -> c_int {
    let _ = net;
    unsafe { xfrm_output(sk, skb) }
}

extern "C" fn __xfrm6_output(net: *mut net, sk: *mut sock, skb: *mut sk_buff) -> c_int {
    unsafe {
        let dst = skb_dst(skb);
        if dst.is_null() {
            return xfrm_output(sk, skb);
        }

        let x = (*dst).xfrm;
        if x.is_null() {
            return dst_output(net, sk, skb);
        }

        if (*x).props.mode != XFRM_MODE_TUNNEL {
            return xfrm_output(sk, skb);
        }

        ip6_fragment(net, sk, skb, __xfrm6_output_finish)
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn xfrm6_output(net: *mut net, sk: *mut sock, skb: *mut sk_buff) -> c_int {
    __xfrm6_output(net, sk, skb)
}
```