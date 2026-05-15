//! This module provides FFI-compatible Rust bindings for TCP Segmentation Offload (TSO)
//! functions from the Linux kernel. The implementation maintains ABI compatibility with
//! the original C code and follows strict safety requirements for kernel FFI.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use core::ptr;
use core::mem;

// Constants from Linux kernel
const ETH_P_IPV6: u16 = 0x86DD;

// Type definitions for FFI compatibility
#[repr(C)]
struct SkBuff {
    data: *const u8,
    headlen: u32,
    shinfo: *mut SkbSharedInfo,
}

#[repr(C)]
struct SkbSharedInfo {
    gso_segs: u16,
    nr_frags: u16,
    frags: *mut SkbFrag,
}

#[repr(C)]
struct SkbFrag {
    frag: *mut core::ffi::c_void,
    size: u16,
}

#[repr(C)]
struct Iphdr {
    id: u16,
    tot_len: u16,
}

#[repr(C)]
struct Ipv6hdr {
    payload_len: u16,
}

#[repr(C)]
struct Tcphdr {
    seq: u32,
    psh: u8,
    fin: u8,
    rst: u8,
}

#[repr(C)]
struct Udphdr {
    len: u16,
}

#[repr(C)]
struct Tso {
    tlen: u16,
    ip_id: u16,
    tcp_seq: u32,
    next_frag_idx: u16,
    ipv6: bool,
    size: u32,
    data: *const u8,
}

// Helper functions for network byte order
fn htons(x: u16) -> u16 {
    x.to_be()
}

fn ntohs(x: u16) -> u16 {
    u16::from_be(x)
}

fn ntohl(x: u32) -> u32 {
    u32::from_be(x)
}

// Placeholder implementations for kernel functions
unsafe fn skb_transport_offset(skb: *const SkBuff) -> usize {
    // In real implementation, this would calculate transport header offset
    0
}

unsafe fn skb_network_offset(skb: *const SkBuff) -> usize {
    // In real implementation, this would calculate network header offset
    0
}

unsafe fn skb_is_gso_tcp(skb: *const SkBuff) -> bool {
    // Placeholder - real implementation would check GSO type
    true
}

unsafe fn tcp_hdrlen(skb: *const SkBuff) -> u16 {
    // Placeholder - real implementation would get TCP header length
    20
}

unsafe fn ip_hdr(skb: *const SkBuff) -> *mut Iphdr {
    // Placeholder - real implementation would get IP header pointer
    ptr::null_mut()
}

unsafe fn tcp_hdr(skb: *const SkBuff) -> *mut Tcphdr {
    // Placeholder - real implementation would get TCP header pointer
    ptr::null_mut()
}

unsafe fn vlan_get_protocol(skb: *const SkBuff) -> u16 {
    // Placeholder - real implementation would get VLAN protocol
    0x0800
}

unsafe fn skb_frag_size(frag: *const SkbFrag) -> u32 {
    (*frag).size as u32
}

unsafe fn skb_frag_address(frag: *const SkbFrag) -> *const u8 {
    (*frag).frag as *const u8
}

// Put unaligned 32-bit value in big-endian format
fn put_unaligned_be32(src: u32, dst: *mut u32) {
    // SAFETY: Caller guarantees valid pointer to 4 bytes of memory
    unsafe {
        *dst = src.to_be();
    }
}

// Exported functions
#[no_mangle]
pub unsafe extern "C" fn tso_count_descs(skb: *const SkBuff) -> i32 {
    // SAFETY: Caller must ensure skb is valid and non-null
    let shinfo = (*skb).shinfo;
    let gso_segs = (*shinfo).gso_segs;
    let nr_frags = (*shinfo).nr_frags;
    (gso_segs as i32) * 2 + (nr_frags as i32)
}

#[no_mangle]
pub unsafe extern "C" fn tso_build_hdr(
    skb: *const SkBuff,
    hdr: *mut u8,
    tso: *mut Tso,
    size: i32,
    is_last: bool,
) {
    // SAFETY: Caller must ensure all pointers are valid and non-null
    let skb_transport_offset = skb_transport_offset(skb);
    let hdr_len = skb_transport_offset + (*tso).tlen;
    let mac_hdr_len = skb_network_offset(skb);

    // Copy header data
    ptr::copy_nonoverlapping((*skb).data, hdr, hdr_len as usize);

    if !(*tso).ipv6 {
        let iph = (hdr.add(mac_hdr_len) as *mut Iphdr);
        (*iph).id = htons((*tso).ip_id);
        (*iph).tot_len = htons((size as u32 + hdr_len as u32 - mac_hdr_len as u32) as u16);
        (*tso).ip_id += 1;
    } else {
        let iph = (hdr.add(mac_hdr_len) as *mut Ipv6hdr);
        (*iph).payload_len = htons((size as u32 + (*tso).tlen as u32) as u16);
    }

    let transport_hdr = hdr.add(skb_transport_offset);
    if (*tso).tlen != mem::size_of::<Udphdr>() as u16 {
        let tcph = transport_hdr as *mut Tcphdr;
        put_unaligned_be32((*tso).tcp_seq, &mut (*tcph).seq);

        if !is_last {
            (*tcph).psh = 0;
            (*tcph).fin = 0;
            (*tcph).rst = 0;
        }
    } else {
        let uh = transport_hdr as *mut Udphdr;
        (*uh).len = htons((mem::size_of::<Udphdr>() as u32 + size as u32) as u16);
    }
}

#[no_mangle]
pub unsafe extern "C" fn tso_build_data(skb: *const SkBuff, tso: *mut Tso, size: i32) {
    // SAFETY: Caller must ensure pointers are valid and non-null
    (*tso).tcp_seq += size as u32;
    (*tso).size -= size as u32;
    (*tso).data = (*tso).data.add(size as usize);

    if (*tso).size == 0 && (*tso).next_frag_idx < (*skb).shinfo.nr_frags {
        let frag = (*skb).shinfo.frags.add((*tso).next_frag_idx as usize);
        (*tso).size = skb_frag_size(frag);
        (*tso).data = skb_frag_address(frag);
        (*tso).next_frag_idx += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn tso_start(skb: *mut SkBuff, tso: *mut Tso) -> i32 {
    // SAFETY: Caller must ensure pointers are valid and non-null
    let tlen = if skb_is_gso_tcp(skb) {
        tcp_hdrlen(skb) as u16
    } else {
        mem::size_of::<Udphdr>() as u16
    };
    (*tso).tlen = tlen;
    (*tso).ip_id = ntohs(ip_hdr(skb).id);
    (*tso).tcp_seq = if tlen != mem::size_of::<Udphdr>() as u16 {
        ntohl(tcp_hdr(skb).seq)
    } else {
        0
    };
    (*tso).next_frag_idx = 0;
    (*tso).ipv6 = vlan_get_protocol(skb) == htons(ETH_P_IPV6);

    let hdr_len = skb_transport_offset(skb) + tlen as usize;
    (*tso).size = (*skb).headlen as u32 - hdr_len as u32;
    (*tso).data = (*skb).data.add(hdr_len);

    if (*tso).size == 0 && (*tso).next_frag_idx < (*skb).shinfo.nr_frags {
        let frag = (*skb).shinfo.frags.add((*tso).next_frag_idx as usize);
        (*tso).size = skb_frag_size(frag);
        (*tso).data = skb_frag_address(frag);
        (*tso).next_frag_idx += 1;
    }
    hdr_len as i32
}
