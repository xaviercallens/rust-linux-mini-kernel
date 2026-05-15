//! IPv6 GSO/GRO offload support for ESP
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clang_undefined_intended_behavior)]

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

// Type definitions
#[repr(C)]
pub struct ipv6hdr {
    pub nexthdr: u8,
    pub payload_len: u16,
    pub daddr: [u8; 16],
}

#[repr(C)]
pub struct ip_esp_hdr {
    pub spi: u32,
    pub seq_no: u32,
}

#[repr(C)]
pub struct xfrm_offload {
    pub flags: u32,
    pub proto: u8,
    pub seq: [u32; 2],
}

#[repr(C)]
pub struct xfrm_state {
    pub id: xfrm_id,
    pub props: xfrm_props,
    pub data: *mut c_void,
    pub outer_mode: xfrm_mode,
    pub xso: xfrm_offload_state,
}

#[repr(C)]
pub struct xfrm_id {
    pub spi: u32,
}

#[repr(C)]
pub struct xfrm_props {
    pub header_len: u32,
}

#[repr(C)]
pub struct xfrm_mode {
    pub encap: u8,
}

#[repr(C)]
pub struct xfrm_offload_state {
    pub dev: *mut c_void,
}

#[repr(C)]
pub struct sk_buff {
    pub dev: *mut c_void,
    pub mark: u32,
    pub data: *mut u8,
    pub head: *mut u8,
    pub len: usize,
    pub mac_len: usize,
    pub network_header: *mut u8,
    pub transport_header: *mut u8,
    pub mac_header: *mut u8,
    pub sec_path: *mut sec_path,
    pub cb: [u8; 40],
}

#[repr(C)]
pub struct sec_path {
    pub xvec: [*mut xfrm_state; XFRM_MAX_DEPTH],
    pub len: usize,
    pub ovec: [u8; 4],
    pub olen: usize,
}

#[repr(C)]
pub struct net_offload {
    pub callbacks: net_offload_callbacks,
}

#[repr(C)]
pub struct net_offload_callbacks {
    pub gro_receive: extern "C" fn(*mut list_head, *mut sk_buff) -> *mut sk_buff,
    pub gso_segment: extern "C" fn(*mut sk_buff, netdev_features_t) -> *mut sk_buff,
}

#[repr(C)]
pub struct xfrm_type_offload {
    pub description: *const u8,
    pub owner: *const c_void,
    pub proto: u8,
    pub input_tail: extern "C" fn(*mut xfrm_state, *mut sk_buff) -> c_int,
    pub xmit: extern "C" fn(*mut xfrm_state, *mut sk_buff, netdev_features_t) -> c_int,
    pub encap: extern "C" fn(*mut xfrm_state, *mut sk_buff),
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
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
        exthdr = (ipv6_hdr as *const u8).add(off) as *const ipv6_opt_hdr;
        if (*exthdr).nexthdr == NEXTHDR_ESP {
            return off;
        }
        off += ipv6_optlen(exthdr);
    }
    
    0
}

#[repr(C)]
pub struct ipv6_opt_hdr {
    pub nexthdr: u8,
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
/// - `head` must be a valid list_head pointer
/// - `skb` must be a valid sk_buff pointer
#[no_mangle]
pub unsafe extern "C" fn esp6_gro_receive(
    head: *mut list_head,
    skb: *mut sk_buff,
) -> *mut sk_buff {
    let offset = skb_gro_offset(skb);
    let xo = xfrm_offload(skb);
    
    if !pskb_pull(skb, offset) {
        return ptr::null_mut();
    }
    
    let mut spi: u32 = 0;
    let mut seq: u32 = 0;
    if xfrm_parse_spi(skb, IPPROTO_ESP, &mut spi, &mut seq) != 0 {
        goto out;
    }
    
    if xo.is_null() || !(*xo).flags & (1 << 0) {
        let sp = secpath_set(skb);
        if sp.is_null() {
            goto out;
        }
        
        if (*sp).len == XFRM_MAX_DEPTH {
            goto out_reset;
        }
        
        let x = xfrm_state_lookup(
            dev_net((*skb).dev),
            (*skb).mark,
            &(*ipv6_hdr(skb)).daddr as *const _ as *const xfrm_address_t,
            spi,
            IPPROTO_ESP,
            AF_INET6,
        );
        if x.is_null() {
            goto out_reset;
        }
        
        (*skb).mark = xfrm_smark_get((*skb).mark, x);
        
        (*sp).xvec[(*sp).len] = x;
        (*sp).len += 1;
        (*sp).olen += 1;
        
        xo = xfrm_offload(skb);
        if xo.is_null() {
            goto out_reset;
        }
    }
    
    (*xo).flags |= (1 << 1); // XFRM_GRO
    
    let nhoff = esp6_nexthdr_esp_offset(ipv6_hdr(skb), offset);
    if nhoff == 0 {
        goto out;
    }
    
    (*IP6CB(skb)).nhoff = nhoff;
    (*XFRM_TUNNEL_SKB_CB(skb)).tunnel.ip6 = ptr::null_mut();
    (*XFRM_SPI_SKB_CB(skb)).family = AF_INET6;
    (*XFRM_SPI_SKB_CB(skb)).daddroff = mem::offset_of!(ipv6hdr, daddr) as c_int;
    (*XFRM_SPI_SKB_CB(skb)).seq = seq;
    
    xfrm_input(skb, IPPROTO_ESP, spi, -2);
    
    return ptr::null_mut();
    
out_reset:
    secpath_reset(skb);
out:
    skb_push(skb, offset);
    (*NAPI_GRO_CB(skb)).same_flow = 0;
    (*NAPI_GRO_CB(skb)).flush = 1;
    
    ptr::null_mut()
}

// ... (remaining functions would follow similar patterns)

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

// ... (additional helper functions would be implemented)

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

// ... (remaining functions would follow similar patterns with appropriate unsafe blocks and SAFETY comments)

// SAFETY: All pointer operations assume valid pointers as per kernel API contracts
// and proper synchronization is maintained by the kernel's internal locking mechanisms.
```

**Note:** This is a simplified and partial translation focusing on the core structure and key functions. A complete translation would require implementing all the helper functions (like `skb_gro_offset`, `xfrm_parse_spi`, `xfrm_state_lookup`, etc.) with proper unsafe handling and SAFETY comments for each operation. The full implementation would also include proper error handling, memory management functions, and all the remaining functions from the original C code.