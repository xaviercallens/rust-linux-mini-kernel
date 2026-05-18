#![no_std]
#![no_main]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use core::ffi::{c_int, c_void};
use core::mem;

mod kernel_types {
    pub type size_t = usize;
    pub type c_size_t = usize;
    pub type socklen_t = u32;
}
use kernel_types::*;

pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ETH_P_IP: c_int = 0x0800;
pub const ETH_P_IPV6: c_int = 0x86DD;
pub const IPPROTO_IPV6: c_int = 41;
pub const IPPROTO_IPIP: c_int = 4;
pub const IPPROTO_ETHERNET: c_int = 143;
pub const IPV6_FLOWLABEL_MASK: u32 = 0x000F_FFFF;
pub const NEXTHDR_ROUTING: u8 = 43;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct in6_addr {
    pub s6_addr: [u8; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipv6hdr {
    pub priority: u8,
    pub version: u8,
    pub flow_lbl: u32,
    pub payload_len: u16,
    pub nexthdr: u8,
    pub hop_limit: u8,
    pub saddr: in6_addr,
    pub daddr: in6_addr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipv6_sr_hdr {
    pub nexthdr: u8,
    pub hdrlen: u8,
    pub m: u8,
    pub reserved: u8,
    pub first_segment: u8,
    pub segments: [in6_addr; 1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct seg6_iptunnel_encap {
    pub mode: c_int,
    pub srh: *mut ipv6_sr_hdr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dst_cache {
    _private: [u8; 1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct seg6_lwt {
    pub cache: dst_cache,
    pub tuninfo: [seg6_iptunnel_encap; 1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lwtunnel_state {
    pub data: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sk_buff {
    pub protocol: u16,
    _private: [u8; 1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct net {
    _private: [u8; 1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct net_device {
    _private: [u8; 1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dst_entry {
    pub dev: *mut net_device,
    pub lwtstate: *mut lwtunnel_state,
}

unsafe extern "C" {
    fn skb_dst(skb: *mut sk_buff) -> *mut dst_entry;
    fn dev_net(dev: *mut net_device) -> *mut net;
    fn ipv6_hdr(skb: *mut sk_buff) -> *mut ipv6hdr;
    fn skb_cow_head(skb: *mut sk_buff, headroom: c_int) -> c_int;
    fn skb_push(skb: *mut sk_buff, len: c_int);
    fn skb_reset_network_header(skb: *mut sk_buff);
    fn skb_mac_header_rebuild(skb: *mut sk_buff);
    fn seg6_make_flowlabel(net: *mut net, skb: *mut sk_buff, inner_hdr: *const ipv6hdr) -> u32;
    fn htons(v: u16) -> u16;
    fn ip6_flow_hdr(hdr: *mut ipv6hdr, tclass: u8, flowlabel: u32);
    fn ip6_tclass(flowinfo: u32) -> u8;
    fn ip6_flowinfo(hdr: *const ipv6hdr) -> u32;
    fn ip6_dst_hoplimit(dst: *mut dst_entry) -> u8;
    fn IP6CB(skb: *mut sk_buff) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn set_tun_src(net: *mut net, dev: *mut net_device, daddr: *mut in6_addr, saddr: *mut in6_addr);
    fn skb_postpush_rcsum(skb: *mut sk_buff, start: *mut ipv6hdr, len: c_int);
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}

#[no_mangle]
pub unsafe extern "C" fn seg6_lwt_headroom(tuninfo: *const seg6_iptunnel_encap) -> size_t {
    if tuninfo.is_null() {
        return 0;
    }

    let mode = (*tuninfo).mode;
    let mut head: size_t = 0;

    match mode {
        0 => {}
        1 => head = mem::size_of::<ipv6hdr>(),
        2 => return 0,
        _ => {}
    }

    let hdrlen = (*tuninfo)
        .srh
        .as_ref()
        .map_or(0, |srh| ((srh.hdrlen as size_t) + 1) << 3);

    hdrlen + head
}

#[no_mangle]
pub unsafe extern "C" fn seg6_do_srh_encap(
    skb: *mut sk_buff,
    osrh: *mut ipv6_sr_hdr,
    _proto: c_int,
) -> c_int {
    if skb.is_null() || osrh.is_null() {
        return EINVAL;
    }

    let dst = skb_dst(skb);
    if dst.is_null() || (*dst).dev.is_null() {
        return EINVAL;
    }

    let net = dev_net((*dst).dev);
    if net.is_null() {
        return EINVAL;
    }

    let inner_hdr_ptr = ipv6_hdr(skb);
    if inner_hdr_ptr.is_null() {
        return EINVAL;
    }

    let osrh_ref = &*osrh;
    let hdrlen = ((osrh_ref.hdrlen as size_t) + 1) << 3;
    let tot_len = hdrlen + mem::size_of::<ipv6hdr>();

    let err = skb_cow_head(skb, tot_len as c_int);
    if err != 0 {
        return err;
    }

    let inner_hdr = &*inner_hdr_ptr;
    let flowlabel = seg6_make_flowlabel(net, skb, inner_hdr as *const ipv6hdr);

    skb_push(skb, tot_len as c_int);
    skb_reset_network_header(skb);
    skb_mac_header_rebuild(skb);

    let hdr = ipv6_hdr(skb);
    if hdr.is_null() {
        return EINVAL;
    }

    if (*skb).protocol == htons(ETH_P_IPV6 as u16) {
        ip6_flow_hdr(hdr, ip6_tclass(ip6_flowinfo(inner_hdr as *const ipv6hdr)), flowlabel);
        (*hdr).hop_limit = inner_hdr.hop_limit;
    } else {
        ip6_flow_hdr(hdr, 0, flowlabel);
        let ndst = skb_dst(skb);
        if ndst.is_null() {
            return EINVAL;
        }
        (*hdr).hop_limit = ip6_dst_hoplimit(ndst);
        memset(IP6CB(skb), 0, mem::size_of::<*mut c_void>() as size_t);
    }

    0
}