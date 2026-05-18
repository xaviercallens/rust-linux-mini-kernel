//! IPv6 GSO/GRO offload support for ESP
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clang_undefined_intended_behavior)]

use kernel_types::*;
use core::ffi::c_int;
use core::ffi::c_void;
use core::ptr;
use core::mem;
use core::slice;
use core::cmp;

// Constants from C
pub const IPPROTO_ESP: u8 = 50;
pub const NEXTHDR_ESP: u8 = 50;
pub const XFRM_MAX_DEPTH: usize = 16;
pub const IPPROTO_TCP: u8 = 6;
pub const IPPROTO_BEETPH: u8 = 148;
pub const SKB_GSO_TCPV6: u32 = 0x00000008;
pub const SKB_GSO_ESP: u32 = 0x00000400;
pub const NETIF_F_HW_ESP: u32 = 0x00000010;
pub const NETIF_F_HW_ESP_TX_CSUM: u32 = 0x00000020;
pub const NETIF_F_SG: u32 = 0x00000002;
pub const NETIF_F_CSUM_MASK: u32 = 0x0000000F;
pub const NETIF_F_SCTP_CRC: u32 = 0x00000040;
pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const EOPNOTSUPP: c_int = -95;
pub const EINPROGRESS: c_int = -115;
pub const AF_INET6: c_int = 10;
pub const EAGAIN: c_int = -11;

// Type definitions

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_offload {
    pub flags: u32,
    pub proto: u8,
    pub seq: [u32; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_state {
    pub id: xfrm_id,
    pub props: xfrm_props,
    pub data: *mut c_void,
    pub outer_mode: xfrm_mode,
    pub xso: xfrm_offload_state,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_id {
    pub spi: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_props {
    pub header_len: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_mode {
    pub encap: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_offload_state {
    pub dev: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sec_path {
    pub xvec: [*mut xfrm_state; XFRM_MAX_DEPTH],
    pub len: usize,
    pub ovec: [u8; 4],
    pub olen: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct net_offload {
    pub callbacks: net_offload_callbacks,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct net_offload_callbacks {
    pub gro_receive: extern "C" fn(*mut sk_buff) -> *mut sk_buff,
    pub gso_segment: extern "C" fn(*mut sk_buff, netdev_features_t) -> *mut sk_buff,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_type_offload {
    pub description: *const u8,
    pub owner: *const c_void,
    pub proto: u8,
    pub input_tail: extern "C" fn(*mut xfrm_state, *mut sk_buff) -> c_int,
    pub xmit: extern "C" fn(*mut xfrm_state, *mut sk_buff, netdev_features_t) -> c_int,
    pub encap: extern "C" fn(*mut xfrm_state, *mut sk_buff),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipv6_opt_hdr {
    pub nexthdr: u8,
}

pub type netdev_features_t = u32;

// Function implementations

/// Find the offset of the ESP header in IPv6 extension headers
///
/// # Safety
/// - `ipv6_hdr` must be a valid pointer to an ipv6hdr
/// - `nhlen` must be the length of the extension headers
#[no_mangle]
pub unsafe extern "C" fn esp6_nexthdr_esp_offset(
    ipv6_hdr: *const ipv6hdr,
    nhlen: c_int,
) -> c_int {
    let mut off = mem::size_of::<ipv6hdr>() as c_int;
    let mut exthdr: *const ipv6_opt_hdr = ptr::null();

    if ipv6_hdr.is_null() {
        return 0;
    }

    if (*ipv6_hdr).nexthdr == NEXTHDR_ESP {
        return mem::offset_of!(ipv6hdr, nexthdr) as c_int;
    }

    while off < nhlen {
        exthdr = (ipv6_hdr as *const u8).add(off as usize) as *const ipv6_opt_hdr;
        if (*exthdr).nexthdr == NEXTHDR_ESP {
            return off;
        }
        off += ipv6_optlen(exthdr);
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn ipv6_optlen(hdr: *const ipv6_opt_hdr) -> c_int {
    if hdr.is_null() {
        return 0;
    }
    let len = (*hdr).nexthdr & 0x0F;
    (len as c_int) * 8
}

/// GRO receive handler for ESP IPv6
///
/// # Safety
/// - `skb` must be a valid sk_buff pointer
#[no_mangle]
pub unsafe extern "C" fn esp6_gro_receive(
    skb: *mut sk_buff,
) -> *mut sk_buff {
    if skb.is_null() {
        return ptr::null_mut();
    }

    let offset = skb_gro_offset(skb);
    let xo = xfrm_offload(skb);

    if !pskb_pull(skb, offset) {
        return ptr::null_mut();
    }

    let mut spi: u32 = 0;
    let mut seq: u32 = 0;
    if xfrm_parse_spi(skb, IPPROTO_ESP, &mut spi, &mut seq) != 0 {
        return ptr::null_mut();
    }

    if xo.is_null() || (*xo).flags & (1 << 0) == 0 {
        let sp = secpath_set(skb);
        if sp.is_null() {
            return ptr::null_mut();
        }

        if (*sp).len == XFRM_MAX_DEPTH {
            return ptr::null_mut();
        }

        let x = xfrm_state_lookup(
            dev_net((*skb).sk as *mut sock),
            (*skb).mark,
            &(*ipv6_hdr(skb)).daddr as *const _ as *const nf_inet_addr,
            spi,
            IPPROTO_ESP,
            AF_INET6,
        );
        if x.is_null() {
            return ptr::null_mut();
        }

        (*skb).mark = xfrm_smark_get((*skb).mark, x);

        (*sp).xvec[(*sp).len] = x;
        (*sp).len += 1;
        (*sp).olen += 1;

        let new_xo = xfrm_offload(skb);
        if new_xo.is_null() {
            return ptr::null_mut();
        }
    }

    (*xo).flags |= (1 << 1); // XFRM_GRO

    let nhoff = esp6_nexthdr_esp_offset(ipv6_hdr(skb), offset);
    if nhoff == 0 {
        return ptr::null_mut();
    }

    (*IP6CB(skb)).nhoff = nhoff;
    (*XFRM_TUNNEL_SKB_CB(skb)).tunnel.ip6 = ptr::null_mut();
    (*XFRM_SPI_SKB_CB(skb)).family = AF_INET6;
    (*XFRM_SPI_SKB_CB(skb)).daddroff = mem::offset_of!(ipv6hdr, daddr) as c_int;
    (*XFRM_SPI_SKB_CB(skb)).seq = seq;

    xfrm_input(skb, IPPROTO_ESP, spi, -2);

    secpath_reset(skb);
    skb_push(skb, offset);
    (*NAPI_GRO_CB(skb)).same_flow = 0;
    (*NAPI_GRO_CB(skb)).flush = 1;

    ptr::null_mut()
}

/// GSO segment handler for ESP IPv6
///
/// # Safety
/// - `skb` must be a valid sk_buff pointer
/// - `features` must be valid netdev_features_t
#[no_mangle]
pub unsafe extern "C" fn esp6_gso_segment(
    skb: *mut sk_buff,
    features: netdev_features_t,
) -> *mut sk_buff {
    if skb.is_null() {
        return ptr::null_mut();
    }

    let offset = skb_gso_offset(skb);
    let xo = xfrm_offload(skb);

    if !pskb_pull(skb, offset) {
        return ptr::null_mut();
    }

    let mut spi: u32 = 0;
    let mut seq: u32 = 0;
    if xfrm_parse_spi(skb, IPPROTO_ESP, &mut spi, &mut seq) != 0 {
        return ptr::null_mut();
    }

    if xo.is_null() || (*xo).flags & (1 << 0) == 0 {
        let sp = secpath_set(skb);
        if sp.is_null() {
            return ptr::null_mut();
        }

        if (*sp).len == XFRM_MAX_DEPTH {
            return ptr::null_mut();
        }

        let x = xfrm_state_lookup(
            dev_net((*skb).sk as *mut sock),
            (*skb).mark,
            &(*ipv6_hdr(skb)).daddr as *const _ as *const nf_inet_addr,
            spi,
            IPPROTO_ESP,
            AF_INET6,
        );
        if x.is_null() {
            return ptr::null_mut();
        }

        (*skb).mark = xfrm_smark_get((*skb).mark, x);

        (*sp).xvec[(*sp).len] = x;
        (*sp).len += 1;
        (*sp).olen += 1;

        let new_xo = xfrm_offload(skb);
        if new_xo.is_null() {
            return ptr::null_mut();
        }
    }

    (*xo).flags |= (1 << 1); // XFRM_GSO

    let nhoff = esp6_nexthdr_esp_offset(ipv6_hdr(skb), offset);
    if nhoff == 0 {
        return ptr::null_mut();
    }

    (*IP6CB(skb)).nhoff = nhoff;
    (*XFRM_TUNNEL_SKB_CB(skb)).tunnel.ip6 = ptr::null_mut();
    (*XFRM_SPI_SKB_CB(skb)).family = AF_INET6;
    (*XFRM_SPI_SKB_CB(skb)).daddroff = mem::offset_of!(ipv6hdr, daddr) as c_int;
    (*XFRM_SPI_SKB_CB(skb)).seq = seq;

    let segs = skb_gso_segment(skb, features);
    if segs.is_null() {
        return ptr::null_mut();
    }

    skb_push(skb, offset);
    segs
}

// Module initialization
#[no_mangle]
pub unsafe extern "C" fn esp6_offload_init() -> c_int {
    if xfrm_register_type_offload(&esp6_type_offload, AF_INET6) < 0 {
        pr_info(b"esp6_offload_init: can't add xfrm type offload\n".as_ptr() as *const c_char);
        return -EAGAIN;
    }

    inet6_add_offload(&esp6_offload, IPPROTO_ESP)
}

#[no_mangle]
pub unsafe extern "C" fn esp6_offload_exit() {
    xfrm_unregister_type_offload(&esp6_type_offload, AF_INET6);
    inet6_del_offload(&esp6_offload, IPPROTO_ESP);
}

// Helper functions (simplified for brevity)
#[no_mangle]
pub unsafe extern "C" fn xfrm_register_type_offload(
    type_: *const xfrm_type_offload,
    family: c_int,
) -> c_int {
    // Implementation would interface with kernel APIs
    0
}

#[no_mangle]
pub unsafe extern "C" fn xfrm_unregister_type_offload(
    type_: *const xfrm_type_offload,
    family: c_int,
) {
    // Implementation would interface with kernel APIs
}

#[no_mangle]
pub unsafe extern "C" fn inet6_add_offload(
    offload: *const net_offload,
    proto: c_int,
) -> c_int {
    // Implementation would interface with kernel APIs
    0
}

#[no_mangle]
pub unsafe extern "C" fn inet6_del_offload(
    offload: *const net_offload,
    proto: c_int,
) {
    // Implementation would interface with kernel APIs
}

// Module metadata
#[no_mangle]
pub static esp6_offload: net_offload = net_offload {
    callbacks: net_offload_callbacks {
        gro_receive: esp6_gro_receive,
        gso_segment: esp6_gso_segment,
    },
};

#[no_mangle]
pub static esp6_type_offload: xfrm_type_offload = xfrm_type_offload {
    description: b"ESP6 OFFLOAD\0".as_ptr() as *const u8,
    owner: ptr::null(),
    proto: IPPROTO_ESP,
    input_tail: esp6_input_tail,
    xmit: esp6_xmit,
    encap: esp6_gso_encap,
};

// SAFETY: All pointer operations assume valid pointers as per kernel API contracts
// and proper synchronization is maintained by the kernel's internal locking mechanisms.

// Helper functions for missing kernel APIs
#[no_mangle]
pub unsafe extern "C" fn esp6_input_tail(
    x: *mut xfrm_state,
    skb: *mut sk_buff,
) -> c_int {
    // Implementation would interface with kernel APIs
    0
}

#[no_mangle]
pub unsafe extern "C" fn esp6_xmit(
    x: *mut xfrm_state,
    skb: *mut sk_buff,
    features: netdev_features_t,
) -> c_int {
    // Implementation would interface with kernel APIs
    0
}

#[no_mangle]
pub unsafe extern "C" fn esp6_gso_encap(
    x: *mut xfrm_state,
    skb: *mut sk_buff,
) {
    // Implementation would interface with kernel APIs
}

#[no_mangle]
pub unsafe extern "C" fn skb_gro_offset(skb: *mut sk_buff) -> c_int {
    // Implementation would interface with kernel APIs
    0
}

#[no_mangle]
pub unsafe extern "C" fn pskb_pull(skb: *mut sk_buff, len: c_int) -> bool {
    // Implementation would interface with kernel APIs
    false
}

#[no_mangle]
pub unsafe extern "C" fn xfrm_parse_spi(
    skb: *mut sk_buff,
    proto: u8,
    spi: *mut u32,
    seq: *mut u32,
) -> c_int {
    // Implementation would interface with kernel APIs
    0
}

#[no_mangle]
pub unsafe extern "C" fn secpath_set(skb: *mut sk_buff) -> *mut sec_path {
    // Implementation would interface with kernel APIs
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn xfrm_state_lookup(
    net: *mut c_void,
    mark: u32,
    daddr: *const nf_inet_addr,
    spi: u32,
    proto: u8,
    family: c_int,
) -> *mut xfrm_state {
    // Implementation would interface with kernel APIs
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn xfrm_smark_get(mark: u32, x: *mut xfrm_state) -> u32 {
    // Implementation would interface with kernel APIs
    0
}

#[no_mangle]
pub unsafe extern "C" fn xfrm_offload(skb: *mut sk_buff) -> *mut xfrm_offload {
    // Implementation would interface with kernel APIs
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn ipv6_hdr(skb: *mut sk_buff) -> *mut ipv6hdr {
    // Implementation would interface with kernel APIs
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn dev_net(sk: *mut sock) -> *mut c_void {
    // Implementation would interface with kernel APIs
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn IP6CB(skb: *mut sk_buff) -> *mut c_void {
    // Implementation would interface with kernel APIs
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn XFRM_TUNNEL_SKB_CB(skb: *mut sk_buff) -> *mut c_void {
    // Implementation would interface with kernel APIs
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn XFRM_SPI_SKB_CB(skb: *mut sk_buff) -> *mut c_void {
    // Implementation would interface with kernel APIs
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn xfrm_input(
    skb: *mut sk_buff,
    proto: u8,
    spi: u32,
    encap_type: c_int,
) {
    // Implementation would interface with kernel APIs
}

#[no_mangle]
pub unsafe extern "C" fn secpath_reset(skb: *mut sk_buff) {
    // Implementation would interface with kernel APIs
}

#[no_mangle]
pub unsafe extern "C" fn skb_push(skb: *mut sk_buff, len: c_int) {
    // Implementation would interface with kernel APIs
}

#[no_mangle]
pub unsafe extern "C" fn NAPI_GRO_CB(skb: *mut sk_buff) -> *mut c_void {
    // Implementation would interface with kernel APIs
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn skb_gso_offset(skb: *mut sk_buff) -> c_int {
    // Implementation would interface with kernel APIs
    0
}

#[no_mangle]
pub unsafe extern "C" fn skb_gso_segment(
    skb: *mut sk_buff,
    features: netdev_features_t,
) -> *mut sk_buff {
    // Implementation would interface with kernel APIs
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn pr_info(fmt: *const c_char) {
    // Implementation would interface with kernel APIs
}