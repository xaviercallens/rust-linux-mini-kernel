#![no_std]
#![no_main]
#![allow(non_camel_case_types)]

use core::ptr;
use kernel_types::{c_int, c_void};

type size_t = usize;
type c_size_t = usize;
type socklen_t = u32;

const IPV6_RPL_BEST_ADDR_COMPRESSION: u8 = 15;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct in6_addr {
    pub s6_addr: [u8; 16],
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
    pub rpl_segaddr: [in6_addr; 1],
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}

#[no_mangle]
pub unsafe extern "C" fn rust_eh_personality() {}

#[no_mangle]
pub unsafe extern "C" fn ipv6_rpl_addr_decompress(
    dst: *mut in6_addr,
    daddr: *const in6_addr,
    post: *const c_void,
    pfx: u8,
) {
    ptr::copy_nonoverlapping(daddr as *const u8, dst as *mut u8, pfx as usize);
    let dst_post = (dst as *mut u8).add(pfx as usize);
    let post_bytes = post as *const u8;
    let tail_len = (16 - pfx) as usize;
    ptr::copy_nonoverlapping(post_bytes, dst_post, tail_len);
}

#[no_mangle]
pub unsafe extern "C" fn ipv6_rpl_addr_compress(dst: *mut c_void, addr: *const in6_addr, pfx: u8) {
    let src = (addr as *const u8).add(pfx as usize);
    ptr::copy_nonoverlapping(src, dst as *mut u8, (16 - pfx) as usize);
}

#[no_mangle]
pub unsafe extern "C" fn ipv6_rpl_segdata_pos(hdr: *const ipv6_rpl_sr_hdr, i: c_int) -> *mut c_void {
    let cmpri = (*hdr).cmpri;
    let offset = i as usize * (16 - cmpri as usize);
    (hdr as *const u8).add(core::mem::size_of::<ipv6_rpl_sr_hdr>() + offset) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn ipv6_rpl_srh_size(n: u8, cmpri: u8, cmpre: u8) -> size_t {
    (n as usize * (16 - cmpri as usize)) + (16 - cmpre as usize)
}

#[no_mangle]
pub unsafe extern "C" fn ipv6_rpl_srh_decompress(
    outhdr: *mut ipv6_rpl_sr_hdr,
    inhdr: *const ipv6_rpl_sr_hdr,
    daddr: *const in6_addr,
    n: u8,
) {
    let outhdr_ref = &mut *outhdr;
    let inhdr_ref = &*inhdr;

    outhdr_ref.nexthdr = inhdr_ref.nexthdr;
    outhdr_ref.hdrlen = (((n + 1) * 16) >> 3) as u8;
    outhdr_ref.pad = 0;
    outhdr_ref.rsvd = inhdr_ref.rsvd;
    outhdr_ref.type_ = inhdr_ref.type_;
    outhdr_ref.segments_left = inhdr_ref.segments_left;
    outhdr_ref.cmpri = 0;
    outhdr_ref.cmpre = 0;

    let out_base = core::ptr::addr_of_mut!(outhdr_ref.rpl_segaddr[0]);

    for i in 0..n {
        let dst = out_base.add(i as usize);
        let post = ipv6_rpl_segdata_pos(inhdr, i as c_int);
        ipv6_rpl_addr_decompress(dst, daddr, post, inhdr_ref.cmpri);
    }

    let dst = out_base.add(n as usize);
    let post = ipv6_rpl_segdata_pos(inhdr, n as c_int);
    ipv6_rpl_addr_decompress(dst, daddr, post, inhdr_ref.cmpre);
}

#[no_mangle]
pub unsafe extern "C" fn ipv6_rpl_srh_compress(
    outhdr: *mut ipv6_rpl_sr_hdr,
    inhdr: *const ipv6_rpl_sr_hdr,
    daddr: *const in6_addr,
    n: u8,
) {
    let inhdr_ref = &*inhdr;
    let daddr_ref = &*daddr;

    let in_base = core::ptr::addr_of!(inhdr_ref.rpl_segaddr[0]);
    let mut cmpri: u8 = IPV6_RPL_BEST_ADDR_COMPRESSION;
    let mut cmpre: u8 = IPV6_RPL_BEST_ADDR_COMPRESSION;

    for plen in 0..16usize {
        let mut mismatch = false;
        for i in 0..n as usize {
            let seg = &*in_base.add(i);
            if daddr_ref.s6_addr[plen] != seg.s6_addr[plen] {
                cmpri = plen as u8;
                mismatch = true;
                break;
            }
        }
        if mismatch {
            break;
        }
        if plen == 15 {
            cmpri = 16;
        }
    }

    let last_seg = &*in_base.add(n as usize);
    for plen in 0..16usize {
        if daddr_ref.s6_addr[plen] != last_seg.s6_addr[plen] {
            cmpre = plen as u8;
            break;
        }
        if plen == 15 {
            cmpre = 16;
        }
    }

    let outhdr_ref = &mut *outhdr;
    outhdr_ref.nexthdr = inhdr_ref.nexthdr;
    let seg_len = (n as usize * (16 - cmpri as usize)) + (16 - cmpre as usize);
    outhdr_ref.hdrlen = (seg_len >> 3) as u8;

    if (seg_len & 0x7) != 0 {
        outhdr_ref.hdrlen = outhdr_ref.hdrlen.wrapping_add(1);
        outhdr_ref.pad = (8 - (seg_len & 0x7)) as u8;
    } else {
        outhdr_ref.pad = 0;
    }

    outhdr_ref.rsvd = inhdr_ref.rsvd;
    outhdr_ref.type_ = inhdr_ref.type_;
    outhdr_ref.segments_left = inhdr_ref.segments_left;
    outhdr_ref.cmpri = cmpri;
    outhdr_ref.cmpre = cmpre;

    for i in 0..n {
        let seg = &*in_base.add(i as usize);
        let pos = ipv6_rpl_segdata_pos(outhdr as *const ipv6_rpl_sr_hdr, i as c_int);
        ipv6_rpl_addr_compress(pos, seg, cmpri);
    }

    let seg = &*in_base.add(n as usize);
    let pos = ipv6_rpl_segdata_pos(outhdr as *const ipv6_rpl_sr_hdr, n as c_int);
    ipv6_rpl_addr_compress(pos, seg, cmpre);
}