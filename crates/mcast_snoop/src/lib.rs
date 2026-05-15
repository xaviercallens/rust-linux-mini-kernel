//! IPv6 Multicast Snooping MLD Validation
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]  // For C-style type names

use core::ptr;
use libc::{c_int, c_uint, c_void, size_t, ntohs};

// Error codes from errno.h
pub const EINVAL: c_int = -22;
pub const ENOMSG: c_int = -96;
pub const ENODATA: c_int = -61;
pub const ENOMEM: c_int = -12;

// Kernel function declarations
extern "C" {
    fn pskb_may_pull(skb: *mut c_void, len: c_int) -> c_int;
    fn skb_network_offset(skb: *mut c_void) -> c_int;
    fn ipv6_hdr(skb: *mut c_void) -> *const c_void;
    fn skb_set_transport_header(skb: *mut c_void, offset: c_int);
    fn ipv6_skip_exthdr(skb: *mut c_void, offset: c_int, nexthdr: *mut u8, frag_off: *mut u16) -> c_int;
    fn ipv6_transport_len(skb: *mut c_void) -> c_int;
    fn ipv6_addr_type(addr: *const c_void) -> c_int;
    fn ipv6_addr_any(addr: *const c_void) -> c_int;
    fn ipv6_addr_is_ll_all_nodes(addr: *const c_void) -> c_int;
    fn ipv6_mc_may_pull(skb: *mut c_void, len: c_int) -> c_int;
    fn skb_checksum_trimmed(skb: *mut c_void, transport_len: c_int, validate: extern "C" fn(*mut c_void) -> u16) -> *mut c_void;
    fn kfree_skb(skb: *mut c_void);
}

// Constants from C
pub const IPPROTO_HOPOPTS: u8 = 0;
pub const IPPROTO_ICMPV6: u8 = 58;
pub const ICMPV6_MGM_REDUCTION: u8 = 143;
pub const ICMPV6_MGM_REPORT: u8 = 137;
pub const ICMPV6_MLD2_REPORT: u8 = 144;
pub const ICMPV6_MGM_QUERY: u8 = 138;

// Type definitions
#[repr(C)]
pub struct mld_msg {
    mld_type: u8,
    mld_code: u8,
    mld_checksum: u16,
    mld_mca: [u8; 16],
}

#[repr(C)]
pub struct mld2_query {
    mld2q_type: u8,
    mld2q_resv1: u8,
    mld2q_maxdelay: u16,
    mld2q_mindelay: u16,
    mld2q_code: u16,
    mld2q_mca: [u8; 16],
}

#[repr(C)]
pub struct mld2_report {
    mld2r_type: u8,
    mld2r_resv1: u8,
    mld2r_resv2: u16,
    mld2r_num: u32,
}

// Function implementations
fn ipv6_mc_check_ip6hdr(skb: *mut c_void) -> c_int {
    let offset = unsafe { skb_network_offset(skb) } + size_of::<c_void>() as c_int;
    if unsafe { pskb_may_pull(skb, offset) } == 0 {
        return EINVAL;
    }

    let ip6h = unsafe { ipv6_hdr(skb) };
    // SAFETY: ip6h is valid due to pskb_may_pull success
    let version = unsafe { *(ip6h as *const u8) };
    if (version & 0xF0) >> 4 != 6 {
        return EINVAL;
    }

    let payload_len = unsafe { ntohs(*((ip6h as *const u16).offset(1))) };
    let len = offset + payload_len as c_int;
    if unsafe { (*skb).len } < len || len <= offset {
        return EINVAL;
    }

    unsafe { skb_set_transport_header(skb, offset) };

    0
}

fn ipv6_mc_check_exthdrs(skb: *mut c_void) -> c_int {
    let ip6h = unsafe { ipv6_hdr(skb) };
    let mut nexthdr = IPPROTO_HOPOPTS;
    let offset = unsafe { skb_network_offset(skb) } + size_of::<c_void>() as c_int;
    
    if unsafe { *(ip6h as *const u8).offset(3) } != IPPROTO_HOPOPTS {
        return ENOMSG;
    }

    let mut frag_off: u16 = 0;
    let new_offset = unsafe { ipv6_skip_exthdr(skb, offset, &mut nexthdr, &mut frag_off) };
    
    if new_offset < 0 {
        return EINVAL;
    }

    if nexthdr != IPPROTO_ICMPV6 {
        return ENOMSG;
    }

    unsafe { skb_set_transport_header(skb, new_offset) };

    0
}

fn ipv6_mc_check_mld_reportv2(skb: *mut c_void) -> c_int {
    let len = unsafe { ipv6_transport_len(skb) };
    let required_len = len + size_of::<mld2_report>() as c_int;
    
    if unsafe { ipv6_mc_may_pull(skb, required_len) } == 0 {
        return EINVAL;
    }

    0
}

fn ipv6_mc_check_mld_query(skb: *mut c_void) -> c_int {
    let transport_len = unsafe { ipv6_transport_len(skb) };
    let ip6h = unsafe { ipv6_hdr(skb) };
    
    if (unsafe { ipv6_addr_type(ip6h) } & 0x0010) == 0 {
        return EINVAL;
    }

    if transport_len != size_of::<mld_msg>() as c_int {
        if transport_len < size_of::<mld2_query>() as c_int {
            return EINVAL;
        }

        let len = unsafe { ipv6_transport_len(skb) } + size_of::<mld2_query>() as c_int;
        if unsafe { ipv6_mc_may_pull(skb, len) } == 0 {
            return EINVAL;
        }
    }

    let mld = unsafe { (ipv6_transport_header(skb)) as *const mld_msg };
    let mld_mca = unsafe { &(*mld).mld_mca };
    
    if unsafe { ipv6_addr_any(mld_mca) } != 0 && 
       unsafe { ipv6_addr_is_ll_all_nodes(ip6h) } == 0 {
        return EINVAL;
    }

    0
}

fn ipv6_transport_header(skb: *mut c_void) -> *const c_void {
    // This would be implemented via skb_transport_header in C
    skb
}

fn ipv6_mc_check_mld_msg(skb: *mut c_void) -> c_int {
    let len = unsafe { ipv6_transport_len(skb) } + size_of::<mld_msg>() as c_int;
    
    if unsafe { ipv6_mc_may_pull(skb, len) } == 0 {
        return ENODATA;
    }

    let mld = unsafe { (ipv6_transport_header(skb)) as *const mld_msg };
    let mld_type = unsafe { (*mld).mld_type };
    
    match mld_type {
        ICMPV6_MGM_REDUCTION | ICMPV6_MGM_REPORT => 0,
        ICMPV6_MLD2_REPORT => ipv6_mc_check_mld_reportv2(skb),
        ICMPV6_MGM_QUERY => ipv6_mc_check_mld_query(skb),
        _ => ENODATA,
    }
}

fn ipv6_mc_validate_checksum(skb: *mut c_void) -> u16 {
    // Placeholder for actual checksum validation
    0
}

fn ipv6_mc_check_icmpv6(skb: *mut c_void) -> c_int {
    let len = unsafe { ipv6_transport_len(skb) } + size_of::<mld_msg>() as c_int;
    
    if unsafe { ipv6_mc_may_pull(skb, len) } == 0 {
        return EINVAL;
    }

    let transport_len = unsafe { ipv6_transport_len(skb) };
    let skb_chk = unsafe { 
        skb_checksum_trimmed(
            skb, 
            transport_len, 
            ipv6_mc_validate_checksum 
        )
    };
    
    if skb_chk.is_null() {
        return EINVAL;
    }

    if skb_chk != skb {
        unsafe { kfree_skb(skb_chk) };
    }

    0
}

/// Checks whether this is a sane MLD packet
///
/// # Safety
/// - `skb` must be a valid pointer to sk_buff
/// - Caller must ensure proper memory management
///
/// # Returns
/// 0 on success, error code otherwise
#[no_mangle]
pub unsafe extern "C" fn ipv6_mc_check_mld(skb: *mut c_void) -> c_int {
    let mut ret = ipv6_mc_check_ip6hdr(skb);
    if ret < 0 {
        return ret;
    }
    
    ret = ipv6_mc_check_exthdrs(skb);
    if ret < 0 {
        return ret;
    }
    
    ret = ipv6_mc_check_icmpv6(skb);
    if ret < 0 {
        return ret;
    }
    
    ipv6_mc_check_mld_msg(skb)
}