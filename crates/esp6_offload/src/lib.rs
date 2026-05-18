#![no_std]
#![no_main]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clang_undefined_intended_behavior)]

use core::ffi::{c_int, c_void};
use core::{mem, ptr};
use kernel_types::*;

pub type size_t = usize;
pub type c_size_t = usize;
pub type socklen_t = u32;
pub type netdev_features_t = u32;

pub const IPPROTO_ESP: u8 = 50;
pub const NEXTHDR_ESP: u8 = 50;
pub const XFRM_MAX_DEPTH: usize = 16;
pub const AF_INET6: c_int = 10;

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
pub const EAGAIN: c_int = -11;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_esp_hdr {
    pub spi: u32,
    pub seq_no: u32,
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
pub struct xfrm_state {
    pub id: xfrm_id,
    pub props: xfrm_props,
    pub data: *mut c_void,
    pub outer_mode: xfrm_mode,
    pub xso: xfrm_offload_state,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_offload {
    pub flags: u32,
    pub proto: u8,
    pub seq: [u32; 2],
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
pub struct ipv6_opt_hdr {
    pub nexthdr: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_address_t {
    pub a6: [u32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct net_offload_callbacks {
    pub gro_receive: extern "C" fn(*mut list_head, *mut sk_buff) -> *mut sk_buff,
    pub gso_segment: extern "C" fn(*mut sk_buff, netdev_features_t) -> *mut sk_buff,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct net_offload {
    pub callbacks: net_offload_callbacks,
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

#[no_mangle]
pub unsafe extern "C" fn ipv6_optlen(hdr: *const ipv6_opt_hdr) -> c_int {
    if hdr.is_null() {
        return 0;
    }
    let len = (*hdr).nexthdr & 0x0F;
    (len as c_int) * 8
}

#[no_mangle]
pub unsafe extern "C" fn esp6_nexthdr_esp_offset(ipv6_hdr: *const ipv6hdr, nhlen: c_int) -> c_int {
    let mut off = mem::size_of::<ipv6hdr>() as c_int;

    if ipv6_hdr.is_null() {
        return 0;
    }

    if (*ipv6_hdr).nexthdr == NEXTHDR_ESP {
        return mem::offset_of!(ipv6hdr, nexthdr) as c_int;
    }

    while off < nhlen {
        let exthdr = (ipv6_hdr as *const u8).add(off as usize) as *const ipv6_opt_hdr;
        if (*exthdr).nexthdr == NEXTHDR_ESP {
            return off;
        }
        let optlen = ipv6_optlen(exthdr);
        if optlen <= 0 {
            break;
        }
        off += optlen;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn esp6_gro_receive(_head: *mut list_head, skb: *mut sk_buff) -> *mut sk_buff {
    if skb.is_null() {
        return ptr::null_mut();
    }
    skb
}

#[no_mangle]
pub unsafe extern "C" fn esp6_gso_segment(skb: *mut sk_buff, _features: netdev_features_t) -> *mut sk_buff {
    skb
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}