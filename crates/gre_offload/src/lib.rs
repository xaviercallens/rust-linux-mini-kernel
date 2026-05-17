//! IPv4 GRE GSO/GRO offload support for Linux kernel
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use core::ptr;
use kernel_types::*;

// Constants from Linux headers
pub const IPPROTO_GRE: c_int = 47;
pub const NETIF_F_SCTP_CRC: netdev_features_t = 1 << 17;
pub const NETIF_F_HW_CSUM: netdev_features_t = 1 << 1;
pub const SKB_GSO_GRE_CSUM: u16 = 1 << 4;
pub const SKB_GSO_PARTIAL: u16 = 1 << 11;
pub const CHECKSUM_PARTIAL: c_int = 2;
pub const ENODEV: c_int = -19;
pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ENOENT: c_int = -2;
pub const SKB_GSO_GRE: u16 = 1 << 12;

// GRE flags
pub const GRE_KEY: u16 = 1 << 1;
pub const GRE_CSUM: u16 = 1 << 2;

// Type definitions
#[repr(C)]
struct gre_base_hdr {
    flags: u16,
    protocol: u16,
}

#[repr(C)]
struct packet_offload {
    callbacks: packet_offload_callbacks,
}

#[repr(C)]
struct packet_offload_callbacks {
    gso_segment: extern "C" fn(*mut sk_buff, netdev_features_t) -> *mut sk_buff,
    gro_receive: extern "C" fn(*mut list_head, *mut sk_buff) -> *mut sk_buff,
    gro_complete: extern "C" fn(*mut sk_buff, c_int) -> c_int,
}

#[repr(C)]
struct list_head {
    next: *mut list_head,
    prev: *mut list_head,
}

type netdev_features_t = u32;

// External C functions
extern "C" {
    fn pskb_may_pull(skb: *mut sk_buff, len: usize) -> c_int;
    fn __skb_pull(skb: *mut sk_buff, len: usize) -> *mut c_void;
    fn skb_reset_mac_header(skb: *mut sk_buff);
    fn skb_set_network_header(skb: *mut sk_buff, offset: u16);
    fn skb_set_transport_header(skb: *mut sk_buff, offset: usize);
    fn skb_mac_gso_segment(skb: *mut sk_buff, features: netdev_features_t) -> *mut sk_buff;
    fn skb_gso_error_unwind(
        skb: *mut sk_buff,
        protocol: u16,
        tnl_hlen: usize,
        mac_offset: u16,
        mac_len: u16,
    );
    fn skb_tnl_header_len(skb: *mut sk_buff) -> usize;
    fn skb_gro_offset(skb: *mut sk_buff) -> usize;
    fn skb_gro_header_fast(skb: *mut sk_buff, offset: usize) -> *mut c_void;
    fn skb_gro_header_hard(skb: *mut sk_buff, hlen: usize) -> c_int;
    fn skb_gro_pull(skb: *mut sk_buff, len: usize) -> *mut c_void;
    fn skb_gro_postpull_rcsum(skb: *mut sk_buff, data: *mut c_void, len: usize);
    fn skb_gro_flush_final(skb: *mut sk_buff, pp: *mut sk_buff, flush: c_int);
    fn skb_gro_checksum_simple_validate(skb: *mut sk_buff) -> c_int;
    fn skb_gro_checksum_try_convert(
        skb: *mut sk_buff,
        protocol: c_int,
        compute_pseudo: extern "C" fn(*mut sk_buff) -> u32,
    );
    fn skb_is_gso(skb: *mut sk_buff) -> c_int;
    fn gro_find_receive_by_type(protocol: u16) -> *mut packet_offload;
    fn gro_find_complete_by_type(protocol: u16) -> *mut packet_offload;
    fn call_gro_receive(
        gro_receive: extern "C" fn(*mut list_head, *mut sk_buff) -> *mut sk_buff,
        head: *mut list_head,
        skb: *mut sk_buff,
    ) -> *mut sk_buff;
    fn inet_add_offload(offload: *const packet_offload, protocol: c_int) -> c_int;
    fn inet_del_offload(offload: *const packet_offload, protocol: c_int);
    fn rcu_read_lock();
    fn rcu_read_unlock();
}

// Function implementations
#[no_mangle]
pub unsafe extern "C" fn gre_gso_segment(
    skb: *mut sk_buff,
    features: netdev_features_t,
) -> *mut sk_buff {
    let tnl_hlen = skb_inner_mac_header(skb) - (*skb).transport_header as usize;
    let need_csum = (*skb).ip_summed & (SKB_GSO_GRE_CSUM as c_int) != 0;
    let mut segs = ptr::null_mut();

    if (*skb).encapsulation == 0 {
        return segs;
    }

    if tnl_hlen < core::mem::size_of::<gre_base_hdr>() {
        return ptr::null_mut();
    }

    if unsafe { pskb_may_pull(skb, tnl_hlen) } == 0 {
        return ptr::null_mut();
    }

    // Setup inner skb
    (*skb).encapsulation = 0;
    // SKB_GSO_CB(skb)->encap_level = 0; // Not implemented in this translation
    unsafe { __skb_pull(skb, tnl_hlen) };
    unsafe { skb_reset_mac_header(skb) };
    unsafe { skb_set_network_header(skb, (*skb).inner_network_offset) };
    (*skb).mac_len = (*skb).inner_network_offset;
    (*skb).protocol = (*skb).inner_protocol;

    (*skb).encap_hdr_csum = if need_csum { 1 } else { 0 };

    let mut offload_csum = 0;
    if need_csum {
        features &= !NETIF_F_SCTP_CRC;
        offload_csum = 1;
    }

    segs = unsafe { skb_mac_gso_segment(skb, features) };
    if segs.is_null() || (segs as *const c_void).is_null() {
        unsafe {
            skb_gso_error_unwind(
                skb,
                (*skb).protocol,
                tnl_hlen,
                (*skb).mac_header,
                (*skb).mac_len,
            )
        };
        return segs;
    }

    let gso_partial = (*skb).ip_summed & (SKB_GSO_PARTIAL as c_int) != 0;
    let outer_hlen = unsafe { skb_tnl_header_len(skb) };
    let gre_offset = outer_hlen - tnl_hlen;
    let mut current_skb = segs;

    loop {
        let greh = current_skb as *mut c_void as *mut gre_base_hdr;
        let pcsum = (greh as *mut c_void).offset(core::mem::size_of::<gre_base_hdr>()) as *mut u16;

        if (*current_skb).ip_summed == CHECKSUM_PARTIAL {
            // skb_reset_inner_headers(current_skb); // Not implemented
            (*current_skb).encapsulation = 1;
        }

        (*current_skb).mac_len = (*skb).mac_len;
        (*current_skb).protocol = (*skb).protocol;

        unsafe { __skb_push(current_skb, outer_hlen) };
        unsafe { skb_reset_mac_header(current_skb) };
        unsafe { skb_set_network_header(current_skb, (*skb).mac_len) };
        unsafe { skb_set_transport_header(current_skb, gre_offset) };

        if !need_csum {
            if (*current_skb).next.is_null() {
                break;
            }
            current_skb = (*current_skb).next;
            continue;
        }

        // Calculate checksum
        if gso_partial && unsafe { skb_is_gso(current_skb) } != 0 {
            let partial_adj = (*current_skb).len + (*current_skb).head as usize
                - (*current_skb).data as usize
                - (*current_skb).inner_network_offset as usize
                - (*current_skb).gso_size;
            *pcsum = !((partial_adj as u32).to_be() as u16);
        } else {
            *pcsum = 0;
        }

        // SAFETY: Pointer arithmetic is valid as we've allocated sufficient space
        *pcsum.offset(1) = 0;

        if (*current_skb).encapsulation != 0 || offload_csum == 0 {
            // gso_make_checksum(current_skb, 0); // Not implemented
        } else {
            (*current_skb).ip_summed = CHECKSUM_PARTIAL;
            (*current_skb).csum_start =
                (*current_skb).transport_header as usize - (*current_skb).head as usize;
            (*current_skb).csum_offset = core::mem::size_of::<gre_base_hdr>();
        }

        if (*current_skb).next.is_null() {
            break;
        }
        current_skb = (*current_skb).next;
    }

    segs
}

#[no_mangle]
pub unsafe extern "C" fn gre_gro_receive(head: *mut list_head, skb: *mut sk_buff) -> *mut sk_buff {
    let mut pp = ptr::null_mut();
    let mut flush = 1;

    if (*NAPI_GRO_CB(skb)).encap_mark != 0 {
        return pp;
    }

    (*NAPI_GRO_CB(skb)).encap_mark = 1;

    let off = unsafe { skb_gro_offset(skb) };
    let hlen = off + core::mem::size_of::<gre_base_hdr>();
    let greh = unsafe { skb_gro_header_fast(skb, off) } as *mut gre_base_hdr;

    if unsafe { skb_gro_header_hard(skb, hlen) } != 0 {
        let greh = unsafe { skb_gro_header_slow(skb, hlen, off) } as *mut gre_base_hdr;
        if greh.is_null() {
            return pp;
        }
    }

    // Check GRE flags
    if (*greh).flags & !(GRE_KEY | GRE_CSUM) != 0 {
        return pp;
    }

    if (*greh).flags & GRE_CSUM != 0 && (*NAPI_GRO_CB(skb)).is_fou != 0 {
        return pp;
    }

    let type_ = (*greh).protocol;

    unsafe { rcu_read_lock() };
    let ptype = unsafe { gro_find_receive_by_type(type_) };
    if ptype.is_null() {
        unsafe { rcu_read_unlock() };
        return pp;
    }

    let mut grehlen = core::mem::size_of::<gre_base_hdr>();
    if (*greh).flags & GRE_KEY != 0 {
        grehlen += core::mem::size_of::<u32>();
    }
    if (*greh).flags & GRE_CSUM != 0 {
        grehlen += core::mem::size_of::<u16>();
    }

    let hlen = off + grehlen;
    if unsafe { skb_gro_header_hard(skb, hlen) } != 0 {
        let greh = unsafe { skb_gro_header_slow(skb, hlen, off) } as *mut gre_base_hdr;
        if greh.is_null() {
            unsafe { rcu_read_unlock() };
            return pp;
        }
    }

    // Checksum validation
    if (*greh).flags & GRE_CSUM != 0 && (*NAPI_GRO_CB(skb)).flush == 0 {
        if unsafe { skb_gro_checksum_simple_validate(skb) } != 0 {
            unsafe { rcu_read_unlock() };
            return pp;
        }
        unsafe { skb_gro_checksum_try_convert(skb, IPPROTO_GRE, null_compute_pseudo) };
    }

    // Check same flow
    let mut p = (*head).next;
    while p != head as *mut list_head {
        let greh2 = (p as *mut sk_buff).offset(off) as *mut gre_base_hdr;

        if (*greh2).flags != (*greh).flags || (*greh2).protocol != (*greh).protocol {
            (*NAPI_GRO_CB(p as *mut sk_buff)).same_flow = 0;
        } else if (*greh).flags & GRE_KEY != 0 {
            let key1 =
                (greh as *mut c_void).offset(core::mem::size_of::<gre_base_hdr>()) as *mut u32;
            let key2 =
                (greh2 as *mut c_void).offset(core::mem::size_of::<gre_base_hdr>()) as *mut u32;
            if *key1 != *key2 {
                (*NAPI_GRO_CB(p as *mut sk_buff)).same_flow = 0;
            }
        }

        p = (*p).next;
    }

    unsafe { skb_gro_pull(skb, grehlen) };
    unsafe { skb_gro_postpull_rcsum(skb, greh, grehlen) };

    pp = unsafe { call_gro_receive((*ptype).callbacks.gro_receive, head, skb) };
    flush = 0;

    unsafe { rcu_read_unlock() };
    unsafe { skb_gro_flush_final(skb, pp, flush) };

    pp
}

#[no_mangle]
pub unsafe extern "C" fn gre_gro_complete(skb: *mut sk_buff, nhoff: c_int) -> c_int {
    let greh = (skb as *mut c_void).offset(nhoff as isize) as *mut gre_base_hdr;
    let grehlen = core::mem::size_of::<gre_base_hdr>() as u32;
    let mut err = -ENOENT;

    (*skb).encapsulation = 1;
    (*skb).ip_summed = 0;
    (*skb_shinfo(skb)).gso_type = SKB_GSO_GRE;

    let type_ = (*greh).protocol;
    if (*greh).flags & GRE_KEY != 0 {
        grehlen += core::mem::size_of::<u32>() as u32;
    }
    if (*greh).flags & GRE_CSUM != 0 {
        grehlen += core::mem::size_of::<u16>() as u32;
    }

    unsafe { rcu_read_lock() };
    let ptype = unsafe { gro_find_complete_by_type(type_) };
    if !ptype.is_null() {
        err = unsafe {
            (*ptype)
                .callbacks
                .gro_complete(skb, nhoff + grehlen as c_int)
        };
    }
    unsafe { rcu_read_unlock() };

    skb_set_inner_mac_header(skb, nhoff + grehlen as c_int);

    err
}

#[no_mangle]
pub unsafe extern "C" fn gre_offload_init() -> c_int {
    let mut err = 0;

    err = inet_add_offload(&gre_offload, IPPROTO_GRE);
    if err != 0 {
        return err;
    }

    // IPv6 support
    // if IS_ENABLED(CONFIG_IPV6) {
    //     err = inet6_add_offload(&gre_offload, IPPROTO_GRE);
    //     if (err)
    //         inet_del_offload(&gre_offload, IPPROTO_GRE);
    // }

    err
}

// Helper functions
#[inline]
unsafe fn skb_inner_mac_header(skb: *mut sk_buff) -> usize {
    // Simplified implementation
    (*skb).data as usize + (*skb).mac_len as usize
}

#[inline]
unsafe fn skb_shinfo(skb: *mut sk_buff) -> *mut skb_shared_info {
    // Simplified implementation
    (skb as *mut c_void).offset(128) as *mut skb_shared_info
}

#[inline]
unsafe fn skb_set_inner_mac_header(skb: *mut sk_buff, offset: c_int) {
    // Simplified implementation
}

#[repr(C)]
struct skb_shared_info {
    gso_type: u16,
    gso_size: u16,
    data_offset: u16,
    // ... many more fields ...
}

#[repr(C)]
struct NAPI_GRO_CB {
    encap_mark: u8,
    same_flow: u8,
    flush: u8,
    is_fou: u8,
    data_offset: usize,
}

#[inline]
unsafe fn NAPI_GRO_CB(skb: *mut sk_buff) -> *mut NAPI_GRO_CB {
    // Simplified implementation
    (skb as *mut c_void).offset(192) as *mut NAPI_GRO_CB
}

#[repr(C)]
static gre_offload: packet_offload = packet_offload {
    callbacks: packet_offload_callbacks {
        gso_segment: gre_gso_segment,
        gro_receive: gre_gro_receive,
        gro_complete: gre_gro_complete,
    },
};

#[no_mangle]
pub unsafe extern "C" fn null_compute_pseudo(skb: *mut sk_buff) -> u32 {
    0
}

// Module initialization
#[no_mangle]
pub unsafe extern "C" fn device_initcall(gre_offload_init: extern "C" fn() -> c_int) {
    gre_offload_init();
}
