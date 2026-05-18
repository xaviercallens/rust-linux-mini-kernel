#![no_std]
#![no_main]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(clippy::all)]

use core::ffi::{c_char, c_int, c_void};

mod kernel_types {
    pub type size_t = usize;
    pub type c_size_t = usize;
    pub type socklen_t = u32;
}

use kernel_types::*;

pub const IPPROTO_UDP: u8 = 17;
pub const IPPROTO_IPV6: u8 = 41;
pub const IPPROTO_IPIP: u8 = 4;
pub const IPPROTO_UDPLITE: u8 = 136;

pub const EINVAL: c_int = -22;
pub const ENOENT: c_int = -2;
pub const EOPNOTSUPP: c_int = -95;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct in6_addr {
    pub s6_addr: [u8; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iphdr {
    pub version: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct udphdr {
    pub source: u16,
    pub dest: u16,
    pub len: u16,
    pub check: u16,
}

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_tunnel_encap {
    pub dport: u16,
    pub flags: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flowi6 {
    pub saddr: in6_addr,
    pub daddr: in6_addr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct guehdr {
    pub version: u8,
    pub control: u8,
    pub hlen: u8,
    pub proto_ctype: u8,
}

#[repr(C)]
pub struct inet6_skb_parm {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inet6_protocol {
    pub err_handler:
        extern "C" fn(*mut sk_buff, *mut inet6_skb_parm, u8, u8, c_int, u32) -> c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip6_tnl_encap_ops {
    pub encap_hlen: extern "C" fn(*const ip_tunnel_encap) -> c_int,
    pub build_header:
        extern "C" fn(*mut sk_buff, *const ip_tunnel_encap, *mut u8, *mut flowi6) -> c_int,
    pub err_handler:
        extern "C" fn(*mut sk_buff, *mut inet6_skb_parm, u8, u8, c_int, u32) -> c_int,
}

unsafe extern "C" {
    fn skb_push(skb: *mut sk_buff, len: size_t) -> *mut c_void;
    fn skb_reset_transport_header(skb: *mut sk_buff);
    fn udp_hdr(skb: *mut sk_buff) -> *mut udphdr;
    fn udp6_set_csum(
        flag: c_int,
        skb: *mut sk_buff,
        saddr: *const in6_addr,
        daddr: *const in6_addr,
        len: size_t,
    );
    fn skb_len(skb: *const sk_buff) -> size_t;
    fn __fou_build_header(
        skb: *mut sk_buff,
        e: *const ip_tunnel_encap,
        protocol: *mut u8,
        sport: *mut u16,
        type_: c_int,
    ) -> c_int;
    fn __gue_build_header(
        skb: *mut sk_buff,
        e: *const ip_tunnel_encap,
        protocol: *mut u8,
        sport: *mut u16,
        type_: c_int,
    ) -> c_int;
    fn pskb_may_pull(skb: *mut sk_buff, len: size_t) -> c_int;
    fn validate_gue_flags(gueh: *const guehdr, optlen: size_t) -> c_int;
    fn ip6_tnl_encap_add_ops(ops: *const ip6_tnl_encap_ops, encap_type: c_int) -> c_int;
    fn ip6_tnl_encap_del_ops(ops: *const ip6_tnl_encap_ops, encap_type: c_int);
    fn pr_err(fmt: *const c_char, ...);
}

const TUNNEL_ENCAP_FLAG_CSUM6: u16 = 0x0001;
const SKB_GSO_UDP_TUNNEL: c_int = 0;
const SKB_GSO_UDP_TUNNEL_CSUM: c_int = 1;
const FOU_ENCAP: c_int = 1;
const GUE_ENCAP: c_int = 2;

fn fou6_build_udp(
    skb: *mut sk_buff,
    e: *const ip_tunnel_encap,
    fl6: *const flowi6,
    protocol: *mut u8,
    sport: u16,
) {
    unsafe {
        let _ = skb_push(skb, core::mem::size_of::<udphdr>()) as *mut udphdr;
        skb_reset_transport_header(skb);

        let uh = udp_hdr(skb);
        (*uh).dest = (*e).dport;
        (*uh).source = sport;
        (*uh).len = (skb_len(skb) as u16).to_be();

        udp6_set_csum(
            if ((*e).flags & TUNNEL_ENCAP_FLAG_CSUM6) == 0 { 1 } else { 0 },
            skb,
            &(*fl6).saddr,
            &(*fl6).daddr,
            skb_len(skb),
        );

        *protocol = IPPROTO_UDP;
    }
}

extern "C" fn fou6_build_header(
    skb: *mut sk_buff,
    e: *const ip_tunnel_encap,
    protocol: *mut u8,
    fl6: *mut flowi6,
) -> c_int {
    unsafe {
        let mut sport = 0u16;
        let type_ = if ((*e).flags & TUNNEL_ENCAP_FLAG_CSUM6) != 0 {
            SKB_GSO_UDP_TUNNEL_CSUM
        } else {
            SKB_GSO_UDP_TUNNEL
        };

        let err = __fou_build_header(skb, e, protocol, &mut sport, type_);
        if err != 0 {
            return err;
        }

        fou6_build_udp(skb, e, fl6, protocol, sport);
        0
    }
}

extern "C" fn gue6_build_header(
    skb: *mut sk_buff,
    e: *const ip_tunnel_encap,
    protocol: *mut u8,
    fl6: *mut flowi6,
) -> c_int {
    unsafe {
        let mut sport = 0u16;
        let type_ = if ((*e).flags & TUNNEL_ENCAP_FLAG_CSUM6) != 0 {
            SKB_GSO_UDP_TUNNEL_CSUM
        } else {
            SKB_GSO_UDP_TUNNEL
        };

        let err = __gue_build_header(skb, e, protocol, &mut sport, type_);
        if err != 0 {
            return err;
        }

        fou6_build_udp(skb, e, fl6, protocol, sport);
        0
    }
}

extern "C" fn fou6_encap_hlen(_e: *const ip_tunnel_encap) -> c_int {
    core::mem::size_of::<udphdr>() as c_int
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}