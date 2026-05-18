#![no_std]
#![no_main]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use core::ffi::{c_int, c_void};
use core::ptr;
use kernel_types::*;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}

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

type netdev_features_t = u32;

#[repr(C)]
pub struct sk_buff {
    _priv: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    next: *mut list_head,
    prev: *mut list_head,
}

#[repr(C)]
pub struct gre_base_hdr {
    flags: u16,
    protocol: u16,
}

#[repr(C)]
pub struct packet_offload_callbacks {
    gso_segment: Option<extern "C" fn(*mut sk_buff, netdev_features_t) -> *mut sk_buff>,
    gro_receive: Option<extern "C" fn(*mut list_head, *mut sk_buff) -> *mut sk_buff>,
    gro_complete: Option<extern "C" fn(*mut sk_buff, c_int) -> c_int>,
}

#[repr(C)]
pub struct packet_offload {
    callbacks: packet_offload_callbacks,
}

unsafe extern "C" {
    fn skb_inner_mac_header(skb: *const sk_buff) -> usize;
    fn skb_transport_header(skb: *const sk_buff) -> usize;
    fn skb_get_protocol(skb: *const sk_buff) -> u16;
    fn skb_set_protocol(skb: *mut sk_buff, protocol: u16);

    fn skb_get_encapsulation(skb: *const sk_buff) -> c_int;
    fn skb_set_encapsulation(skb: *mut sk_buff, val: c_int);

    fn skb_get_inner_network_offset(skb: *const sk_buff) -> u16;
    fn skb_get_inner_protocol(skb: *const sk_buff) -> u16;

    fn skb_get_mac_header(skb: *const sk_buff) -> u16;
    fn skb_get_mac_len(skb: *const sk_buff) -> u16;
    fn skb_set_mac_len(skb: *mut sk_buff, val: u16);

    fn skb_get_ip_summed(skb: *const sk_buff) -> c_int;
    fn skb_set_encap_hdr_csum(skb: *mut sk_buff, val: c_int);

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

#[unsafe(no_mangle)]
pub unsafe extern "C" fn gre_gso_segment(
    skb: *mut sk_buff,
    features: netdev_features_t,
) -> *mut sk_buff {
    let tnl_hlen = unsafe { skb_inner_mac_header(skb) - skb_transport_header(skb) };
    let need_csum = unsafe { skb_get_ip_summed(skb) == CHECKSUM_PARTIAL };

    if unsafe { skb_get_encapsulation(skb) } == 0 {
        return ptr::null_mut();
    }

    if tnl_hlen < core::mem::size_of::<gre_base_hdr>() {
        return ptr::null_mut();
    }

    if unsafe { pskb_may_pull(skb, tnl_hlen) } == 0 {
        return ptr::null_mut();
    }

    unsafe {
        skb_set_encapsulation(skb, 0);
        __skb_pull(skb, tnl_hlen);
        skb_reset_mac_header(skb);
        skb_set_network_header(skb, skb_get_inner_network_offset(skb));
        skb_set_mac_len(skb, skb_get_inner_network_offset(skb));
        skb_set_protocol(skb, skb_get_inner_protocol(skb));
        skb_set_encap_hdr_csum(skb, if need_csum { 1 } else { 0 });
        skb_set_transport_header(skb, skb_get_inner_network_offset(skb) as usize);
        skb_mac_gso_segment(skb, features)
    }
}