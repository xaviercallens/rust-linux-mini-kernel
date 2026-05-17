//! IPv6 Routing Protocol for Low-Power and Lossy Networks (RPL)
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]  // For C-style type names

use core::ptr;
use libc::{c_int, c_uint, c_void, size_t};

// Constants from C
const IPV6_RPL_BEST_ADDR_COMPRESSION: u8 = 15;

// Type definitions
#[repr(C)]
#[derive(Copy, Clone)]
pub struct in6_addr {
    s6_addr: [u8; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipv6_rpl_sr_hdr {
    pub nexthdr: u8,
    pub hdrlen: u8,
    pub pad: u8,
    pub rsvd: u8,
    pub type_: u8,
    pub segments_left: u8,
    pub cmpri: u8,
    pub cmpre: u8,
    pub rpl_segaddr: [in6_addr; 1], // Flexible array member
}

// Function implementations
#[no_mangle]
pub unsafe extern "C" fn ipv6_rpl_addr_decompress(
    dst: *mut in6_addr,
    daddr: *const in6_addr,
    post: *const c_void,
    pfx: u8,
) {
    // SAFETY: Caller guarantees valid pointers and pfx <= 16
    ptr::copy_nonoverlapping(daddr as *const u8, dst as *mut u8, pfx as usize);
    let dst_post = dst as *mut u8.add(pfx as usize);
    let post_bytes = post as *const u8;
    let tail_len = (16 - pfx) as usize;
    ptr::copy_nonoverlapping(post_bytes, dst_post, tail_len);
}

#[no_mangle]
pub unsafe extern "C" fn ipv6_rpl_addr_compress(
    dst: *mut c_void,
    addr: *const in6_addr,
    pfx: u8,
) {
    // SAFETY: Caller guarantees valid pointers and pfx <= 16
    let src = addr as *const u8.add(pfx as usize);
    ptr::copy_nonoverlapping(src, dst as *mut u8, (16 - pfx) as usize);
}

#[no_mangle]
pub unsafe extern "C" fn ipv6_rpl_segdata_pos(
    hdr: *const ipv6_rpl_sr_hdr,
    i: c_int,
) -> *mut c_void {
    let cmpri = (*hdr).cmpri;
    let offset = i as usize * (16 - cmpri as usize);
    // SAFETY: hdr is valid and points to a valid ipv6_rpl_sr_hdr
    (hdr as *const u8).add(core::mem::size_of::<ipv6_rpl_sr_hdr>() + offset) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn ipv6_rpl_srh_size(
    n: u8,
    cmpri: u8,
    cmpre: u8,
) -> size_t {
    let seg_len = (n as usize * (16 - cmpri as usize)) + (16 - cmpre as usize);
    seg_len as size_t
}

#[no_mangle]
pub unsafe extern "C" fn ipv6_rpl_srh_decompress(
    outhdr: *mut ipv6_rpl_sr_hdr,
    inhdr: *const ipv6_rpl_sr_hdr,
    daddr: *const in6_addr,
    n: u8,
) {
    let outhdr = outhdr.as_mut().unwrap();
    let inhdr = inhdr.as_ref().unwrap();
    
    outhdr.nexthdr = inhdr.nexthdr;
    outhdr.hdrlen = (((n + 1) * 16) >> 3) as u8;
    outhdr.pad = 0;
    outhdr.type_ = inhdr.type_;
    outhdr.segments_left = inhdr.segments_left;
    outhdr.cmpri = 0;
    outhdr.cmpre = 0;
    
    for i in 0..n {
        let dst = &mut outhdr.rpl_segaddr[i as usize];
        let post = ipv6_rpl_segdata_pos(inhdr as *const _, i as c_int);
        ipv6_rpl_addr_decompress(dst as *mut _, daddr, post, inhdr.cmpri);
    }
    
    let dst = &mut outhdr.rpl_segaddr[n as usize];
    let post = ipv6_rpl_segdata_pos(inhdr as *const _, n as c_int);
    ipv6_rpl_addr_decompress(dst as *mut _, daddr, post, inhdr.cmpre);
}

#[no_mangle]
pub unsafe extern "C" fn ipv6_rpl_srh_compress(
    outhdr: *mut ipv6_rpl_sr_hdr,
    inhdr: *const ipv6_rpl_sr_hdr,
    daddr: *const in6_addr,
    n: u8,
) {
    let inhdr = inhdr.as_ref().unwrap();
    let mut cmpri = 0;
    let mut cmpre = 0;
    
    // Calculate compression values
    for plen in 0..16 {
        let mut mismatch = false;
        for i in 0..n {
            if daddr.as_ref().unwrap().s6_addr[plen] != 
               inhdr.rpl_segaddr[i as usize].s6_addr[plen] {
                cmpri = plen as u8;
                mismatch = true;
                break;
            }
        }
        if mismatch {
            break;
        }
    }
    
    for plen in 0..16 {
        if daddr.as_ref().unwrap().s6_addr[plen] != 
           inhdr.rpl_segaddr[n as usize].s6_addr[plen] {
            cmpre = plen as u8;
            break;
        }
    }
    
    let outhdr = outhdr.as_mut().unwrap();
    outhdr.nexthdr = inhdr.nexthdr;
    let seg_len = (n as usize * (16 - cmpri as usize)) + (16 - cmpre as usize);
    outhdr.hdrlen = (seg_len >> 3) as u8;
    
    if seg_len & 0x7 != 0 {
        outhdr.hdrlen += 1;
        outhdr.pad = (8 - (seg_len & 0x7)) as u8;
    } else {
        outhdr.pad = 0;
    }
    
    outhdr.type_ = inhdr.type_;
    outhdr.segments_left = inhdr.segments_left;
    outhdr.cmpri = cmpri;
    outhdr.cmpre = cmpre;
    
    for i in 0..n {
        let pos = ipv6_rpl_segdata_pos(outhdr as *mut _, i as c_int);
        ipv6_rpl_addr_compress(pos, &inhdr.rpl_segaddr[i as usize], cmpri);
    }
    
    let pos = ipv6_rpl_segdata_pos(outhdr as *mut _, n as c_int);
    ipv6_rpl_addr_compress(pos, &inhdr.rpl_segaddr[n as usize], cmpre);
}

// Tests (conditional compilation)
#[cfg(test)]
mod tests {
    use super::*;
    use core::ptr;

    #[test]
    fn test_address_compression() {
        let mut dst = in6_addr { s6_addr: [0; 16] };
        let daddr = in6_addr {
            s6_addr: [1; 16],
        };
        let post = [2; 16];
        unsafe {
            ipv6_rpl_addr_decompress(&mut dst as *mut _, &daddr as *const _, post.as_ptr() as *const _, 8);
            assert_eq!(dst.s6_addr[0..8], [1; 8]);
            assert_eq!(dst.s6_addr[8..], [2; 8]);
        }
    }

    #[test]
    fn test_srh_size() {
        let size = unsafe { ipv6_rpl_srh_size(3, 8, 8) };
        assert_eq!(size, (3 * 8 + 8) as size_t);
    }
}