#![no_std]
#![no_main]
#![allow(non_camel_case_types)]

use core::ffi::{c_int, c_uint, c_void};
use core::panic::PanicInfo;
use kernel_types::*;

pub type socklen_t = u32;
pub type size_t = usize;

pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ENOSYS: c_int = -38;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tlvtype_proc {
    pub type_: c_int,
    pub func: Option<unsafe extern "C" fn(skb: *mut c_void, offset: c_int) -> bool>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipv6_destopt_hao {
    pub length: u8,
    pub addr: [u8; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipv6_sr_hdr {
    pub nexthdr: u8,
    pub hdrlen: u8,
    pub segments_left: u16,
    pub reserved: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inet6_skb_parm {
    pub lastopt: c_int,
    pub dst1: c_int,
    pub dsthao: *mut c_void,
    pub srcrt: c_int,
    pub nhoff: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inet6_dev {
    pub cnf: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct net {
    pub ipv6: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dst_entry {
    pub dev: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sk_buff {
    pub dev: *mut c_void,
    pub dst: *mut dst_entry,
    pub ip_summed: c_int,
    pub tstamp: c_int,
    pub head: *mut u8,
    pub data: *mut u8,
    pub network_header: u16,
    pub transport_header: u16,
    pub len: c_uint,
    pub cb: [u8; 48],
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
    fn icmpv6_param_prob(skb: *mut c_void, code: c_int, ptr: c_int);
    fn kfree_skb(skb: *mut c_void);
    fn pskb_may_pull(skb: *mut c_void, len: c_int) -> bool;
    fn skb_cloned(skb: *mut c_void) -> bool;
    fn pskb_expand_head(skb: *mut c_void, headroom: c_int, tailroom: c_int, flags: c_int) -> c_int;
    fn xfrm6_input_addr(skb: *mut c_void, dst: *mut c_void, src: *mut c_void, proto: c_int) -> c_int;
    fn __IP6_INC_STATS(net: *mut net, idev: *mut inet6_dev, mib: c_int);
    fn __skb_tunnel_rx(skb: *mut c_void, dev: *mut c_void, net: *mut net);
    fn netif_rx(skb: *mut c_void) -> c_int;
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_eh_personality() {}

#[inline]
unsafe fn skb_network_header_ptr(skb: *mut sk_buff) -> *mut u8 {
    (*skb).head.add((*skb).network_header as usize)
}

#[inline]
unsafe fn skb_transport_header_ptr(skb: *mut sk_buff) -> *mut u8 {
    (*skb).head.add((*skb).transport_header as usize)
}

#[inline]
unsafe fn skb_headlen(skb: *mut sk_buff) -> usize {
    (*skb).len as usize
}

#[inline]
unsafe fn ipv6_hdr(skb: *mut sk_buff) -> *mut ipv6hdr {
    skb_network_header_ptr(skb) as *mut ipv6hdr
}

#[inline]
unsafe fn ip6cb(skb: *mut sk_buff) -> *mut inet6_skb_parm {
    (*skb).cb.as_mut_ptr() as *mut inet6_skb_parm
}

#[inline]
fn ipv6_addr_is_multicast(addr: *const u8) -> bool {
    unsafe { (*addr & 0xFF) == 0xFF }
}

unsafe fn ip6_tlvopt_unknown(skb: *mut sk_buff, optoff: c_int, disallow_unknowns: bool) -> bool {
    if disallow_unknowns {
        kfree_skb(skb as *mut c_void);
        return false;
    }

    let nh = ipv6_hdr(skb);
    let th = skb_transport_header_ptr(skb);
    let offset = (optoff as isize) - ((*skb).transport_header as isize);
    if offset < 0 {
        kfree_skb(skb as *mut c_void);
        return false;
    }

    let opt_type = ((*th.offset(offset) & 0xC0) >> 6) as u8;

    match opt_type {
        0 => true,
        1 => {
            kfree_skb(skb as *mut c_void);
            false
        }
        2 | 3 => {
            if !ipv6_addr_is_multicast((*nh).daddr.as_ptr()) {
                icmpv6_param_prob(skb as *mut c_void, 4, optoff);
            }
            kfree_skb(skb as *mut c_void);
            false
        }
        _ => false,
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ip6_parse_tlv(
    procs: *const tlvtype_proc,
    skb: *mut c_void,
    _max_count: c_int,
) -> bool {
    if skb.is_null() || procs.is_null() {
        return false;
    }

    let skb = skb as *mut sk_buff;
    let _ = ip6cb(skb);
    let th = skb_transport_header_ptr(skb);
    if th.is_null() {
        return false;
    }

    let hdrlen = ((*th.add(1) as c_int + 1) << 3) as c_int;
    if hdrlen < 2 {
        kfree_skb(skb as *mut c_void);
        return false;
    }

    if ((*skb).transport_header as usize + hdrlen as usize) > skb_headlen(skb) {
        kfree_skb(skb as *mut c_void);
        return false;
    }

    let _ = pskb_may_pull(skb as *mut c_void, hdrlen);
    let _ = skb_cloned(skb as *mut c_void);
    let _ = pskb_expand_head(skb as *mut c_void, 0, 0, 0);
    let _ = xfrm6_input_addr(skb as *mut c_void, core::ptr::null_mut(), core::ptr::null_mut(), 0);
    let _ = netif_rx(skb as *mut c_void);

    ip6_tlvopt_unknown(skb, (*skb).transport_header as c_int, false)
}