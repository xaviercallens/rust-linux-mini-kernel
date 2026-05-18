#![no_std]
#![no_main]
#![allow(non_camel_case_types)]

use core::ffi::{c_int, c_void};
use core::ptr;
use kernel_types::*;

pub const EINVAL: c_int = 22;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flowi6 {
    pub flowi6_oif: u32,
    pub flowi6_mark: u32,
    pub flowi6_uid: u32,
    pub daddr: [u8; 16],
    pub saddr: [u8; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip6_rt_info {
    pub daddr: [u8; 16],
    pub saddr: [u8; 16],
    pub mark: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_bridge_frag_data {
    pub _priv: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_queue_entry_state {
    pub hook: u32,
    pub net: *mut c_void,
    pub sk: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_queue_entry {
    pub state: nf_queue_entry_state,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_ipv6_ops {
    pub route_me_harder:
        unsafe extern "C" fn(net: *mut c_void, sk_partial: *mut c_void, skb: *mut c_void) -> c_int,
    pub route: unsafe extern "C" fn(
        net: *mut c_void,
        dst: *mut *mut c_void,
        fl: *mut c_void,
        strict: c_int,
    ) -> c_int,
    pub fragment: extern "C" fn(
        net: *mut c_void,
        sk: *mut c_void,
        skb: *mut c_void,
        data: *mut nf_bridge_frag_data,
        output: extern "C" fn(
            net: *mut c_void,
            sk: *mut c_void,
            data: *mut nf_bridge_frag_data,
            skb: *mut c_void,
        ) -> c_int,
    ) -> c_int,
    pub reroute: unsafe extern "C" fn(skb: *mut c_void, entry: *const nf_queue_entry) -> c_int,
    pub route_input: extern "C" fn(skb: *mut c_void) -> c_int,
    pub br_fragment: extern "C" fn(
        net: *mut c_void,
        sk: *mut c_void,
        skb: *mut c_void,
        data: *mut nf_bridge_frag_data,
        output: extern "C" fn(
            net: *mut c_void,
            sk: *mut c_void,
            data: *mut nf_bridge_frag_data,
            skb: *mut c_void,
        ) -> c_int,
    ) -> c_int,
}

unsafe extern "C" {
    fn nf_queue_entry_reroute(entry: *const nf_queue_entry) -> *const ip6_rt_info;
}

#[no_mangle]
pub unsafe extern "C" fn ip6_route_me_harder(
    net: *mut c_void,
    sk_partial: *mut c_void,
    skb: *mut c_void,
) -> c_int {
    if net.is_null() || sk_partial.is_null() || skb.is_null() {
        return -EINVAL;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn nf_ip6_reroute(skb: *mut c_void, entry: *const nf_queue_entry) -> c_int {
    if skb.is_null() || entry.is_null() {
        return -EINVAL;
    }

    let rt_info = nf_queue_entry_reroute(entry);
    if rt_info.is_null() {
        return 0;
    }

    if (*entry).state.hook == 3 {
        return ip6_route_me_harder((*entry).state.net, (*entry).state.sk, skb);
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn __nf_ip6_route(
    net: *mut c_void,
    dst: *mut *mut c_void,
    fl: *mut c_void,
    strict: c_int,
) -> c_int {
    if net.is_null() || dst.is_null() || fl.is_null() {
        return -EINVAL;
    }

    let _ = strict;
    *dst = ptr::null_mut();
    0
}

#[no_mangle]
pub static nf_ipv6_ops_instance: nf_ipv6_ops = nf_ipv6_ops {
    route_me_harder: ip6_route_me_harder,
    route: __nf_ip6_route,
    fragment: nf_ip6_fragment_stub,
    reroute: nf_ip6_reroute,
    route_input: nf_ip6_route_input_stub,
    br_fragment: nf_ip6_br_fragment_stub,
};

#[no_mangle]
pub extern "C" fn nf_ip6_fragment_stub(
    _net: *mut c_void,
    _sk: *mut c_void,
    _skb: *mut c_void,
    _data: *mut nf_bridge_frag_data,
    _output: extern "C" fn(
        net: *mut c_void,
        sk: *mut c_void,
        data: *mut nf_bridge_frag_data,
        skb: *mut c_void,
    ) -> c_int,
) -> c_int {
    0
}

#[no_mangle]
pub extern "C" fn nf_ip6_br_fragment_stub(
    _net: *mut c_void,
    _sk: *mut c_void,
    _skb: *mut c_void,
    _data: *mut nf_bridge_frag_data,
    _output: extern "C" fn(
        net: *mut c_void,
        sk: *mut c_void,
        data: *mut nf_bridge_frag_data,
        skb: *mut c_void,
    ) -> c_int,
) -> c_int {
    0
}

#[no_mangle]
pub extern "C" fn nf_ip6_route_input_stub(_skb: *mut c_void) -> c_int {
    0
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}