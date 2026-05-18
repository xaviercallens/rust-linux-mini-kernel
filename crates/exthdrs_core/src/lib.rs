#![no_std]
#![no_main]
#![allow(non_camel_case_types)]
#![allow(clippy::all)]

use core::ffi::c_void;
use core::mem;
use core::ptr;
use kernel_types::*;

pub const NEXTHDR_HOP: u8 = 0;
pub const NEXTHDR_ROUTING: u8 = 43;
pub const NEXTHDR_FRAGMENT: u8 = 44;
pub const NEXTHDR_AUTH: u8 = 51;
pub const NEXTHDR_NONE: u8 = 59;
pub const NEXTHDR_DEST: u8 = 60;

pub const EBADMSG: c_int = -74;
pub const ENOENT: c_int = -2;
pub const ENOMEM: c_int = -12;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipv6_opt_hdr {
    pub nexthdr: u8,
    pub hdrlen: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipv6_rt_hdr {
    pub nexthdr: u8,
    pub hdrlen: u8,
    pub type_: u8,
    pub segments_left: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct frag_hdr {
    pub nexthdr: u8,
    pub reserved: u8,
    pub frag_off: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipv6hdr {
    pub priority_version: u32,
    pub payload_len: u16,
    pub nexthdr: u8,
    pub hop_limit: u8,
    pub saddr: [u8; 16],
    pub daddr: [u8; 16],
}

unsafe extern "C" {
    fn skb_header_pointer(
        skb: *const c_void,
        offset: c_int,
        len: c_int,
        buffer: *mut c_void,
    ) -> *const c_void;

    fn skb_network_header(skb: *const c_void) -> *const u8;
    fn skb_tail_pointer(skb: *const c_void) -> *const u8;
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_eh_personality() {}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ipv6_ext_hdr(nexthdr: u8) -> bool {
    (nexthdr == NEXTHDR_HOP)
        || (nexthdr == NEXTHDR_ROUTING)
        || (nexthdr == NEXTHDR_FRAGMENT)
        || (nexthdr == NEXTHDR_AUTH)
        || (nexthdr == NEXTHDR_NONE)
        || (nexthdr == NEXTHDR_DEST)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ipv6_authlen(hp: *const ipv6_opt_hdr) -> c_int {
    let hdrlen = (*hp).hdrlen as c_int;
    (hdrlen + 2) * 4
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ipv6_optlen(hp: *const ipv6_opt_hdr) -> c_int {
    let hdrlen = (*hp).hdrlen as c_int;
    (hdrlen + 1) << 3
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ipv6_skip_exthdr(
    skb: *const c_void,
    mut start: c_int,
    nexthdrp: *mut u8,
    frag_offp: *mut u16,
) -> c_int {
    let mut nexthdr = *nexthdrp;
    *frag_offp = 0;

    while ipv6_ext_hdr(nexthdr) {
        let mut hdr: ipv6_opt_hdr = mem::zeroed();
        let hp = skb_header_pointer(
            skb,
            start,
            mem::size_of::<ipv6_opt_hdr>() as c_int,
            (&mut hdr as *mut ipv6_opt_hdr).cast::<c_void>(),
        );
        if hp.is_null() {
            return -1;
        }

        if nexthdr == NEXTHDR_NONE {
            return -1;
        }

        let hp = hp.cast::<ipv6_opt_hdr>();
        let hdrlen: c_int;

        if nexthdr == NEXTHDR_FRAGMENT {
            let mut frag: frag_hdr = mem::zeroed();
            let fhp = skb_header_pointer(
                skb,
                start,
                mem::size_of::<frag_hdr>() as c_int,
                (&mut frag as *mut frag_hdr).cast::<c_void>(),
            );
            if fhp.is_null() {
                return -1;
            }

            let fhp = fhp.cast::<frag_hdr>();
            *frag_offp = (*fhp).frag_off;

            if ((*frag_offp as u32) & 0xFFF8) != 0 {
                break;
            }
            hdrlen = 8;
        } else if nexthdr == NEXTHDR_AUTH {
            hdrlen = ipv6_authlen(hp);
        } else {
            hdrlen = ipv6_optlen(hp);
        }

        nexthdr = (*hp).nexthdr;
        start += hdrlen;
    }

    *nexthdrp = nexthdr;
    start
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ipv6_find_tlv(skb: *const c_void, offset: c_int, type_: c_int) -> c_int {
    let nh = skb_network_header(skb);
    if nh.is_null() {
        return -1;
    }

    let tail = skb_tail_pointer(skb);
    if tail.is_null() || (tail as usize) < (nh as usize) {
        return -1;
    }

    let packet_len = (tail as usize) - (nh as usize);
    let mut off = offset as usize;

    if off + 2 > packet_len {
        return -1;
    }

    let hdr = (nh as usize + off) as *const ipv6_opt_hdr;
    let hdr_bytes = (((*hdr).hdrlen as usize) + 1) << 3;

    if off + hdr_bytes > packet_len || hdr_bytes < 2 {
        return -1;
    }

    off += 2;
    let mut left = hdr_bytes - 2;

    while left > 0 {
        if off >= packet_len {
            return -1;
        }

        let opttype = ptr::read((nh as usize + off) as *const u8) as c_int;

        if opttype == type_ {
            return off as c_int;
        }

        let optlen: usize = if opttype == 0 {
            1
        } else {
            if off + 1 >= packet_len {
                return -1;
            }
            (ptr::read((nh as usize + off + 1) as *const u8) as usize) + 2
        };

        if optlen == 0 || optlen > left {
            return -1;
        }

        off += optlen;
        left -= optlen;
    }

    -1
}