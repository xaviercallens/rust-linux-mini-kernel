#![no_std]
#![no_main]
#![no_builtins]
#![allow(non_camel_case_types)]

use core::ptr;
use kernel_types::*;

pub const ENOMEM: c_int = 12;

pub type __be16 = u16;
pub type __be32 = u32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipv6hdr {
    pub payload_len: __be16,
    pub nexthdr: u8,
    pub hop_limit: u8,
    pub saddr: [u8; 16],
    pub daddr: [u8; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct frag_hdr {
    pub frag_off: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inet_frags {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inet_frag_queue {
    pub lock: c_int,
    pub flags: c_int,
    pub len: c_int,
    pub max_size: c_int,
    pub meat: c_int,
    pub stamp: u64,
    pub rb_fragments: c_ulong,
    pub fragments_tail: *mut sk_buff,
    pub last_run_head: *mut sk_buff,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct frag_queue {
    pub q: inet_frag_queue,
    pub iif: c_int,
    pub nhoffset: c_int,
    pub ecn: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct frag_v6_compare_key {
    pub id: __be32,
    pub saddr: [u8; 16],
    pub daddr: [u8; 16],
    pub user: c_int,
    pub iif: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipv6_net {
    pub fqdir: *mut inet_frags,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct net {
    pub ipv6: ipv6_net,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct net_device {
    pub ifindex: c_int,
}

#[repr(C)]
pub struct timer_list {
    _private: [u8; 0],
}

#[repr(C)]
pub struct reasm_data {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct IP6CB {
    pub nhoff: c_int,
    pub flags: c_int,
    pub frag_max_size: c_int,
}

unsafe extern "C" {
    fn ipv6_get_dsfield(ipv6h: *const ipv6hdr) -> u8;
    fn ipv6_change_dsfield(ipv6h: *mut ipv6hdr, mask: u8, value: u8);

    fn inet_frag_kill(q: *mut inet_frag_queue);
    fn inet_frag_reasm_prepare(
        q: *mut inet_frag_queue,
        skb: *mut sk_buff,
        prev_tail: *mut sk_buff,
    ) -> *mut reasm_data;
    fn inet_frag_reasm_finish(
        q: *mut inet_frag_queue,
        skb: *mut sk_buff,
        data: *mut reasm_data,
        update_dev: bool,
    );

    fn skb_postpush_rcsum(skb: *mut sk_buff, start: *const u8, len: c_uint);
    fn skb_network_header_len(skb: *const sk_buff) -> c_uint;

    fn __in6_dev_stats_get(ifindex: c_int, skb: *mut sk_buff) -> *mut c_void;
    fn __IP6_INC_STATS(net: *mut net, stats: *mut c_void, item: c_int);
}

#[inline(always)]
unsafe fn ipv6_hdr(_skb: *mut sk_buff) -> *mut ipv6hdr {
    ptr::null_mut()
}

#[inline(always)]
unsafe fn ip6cb(_skb: *mut sk_buff) -> *mut IP6CB {
    ptr::null_mut()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ip6_frag_ecn(ipv6h: *const ipv6hdr) -> u8 {
    let dsfield = unsafe { ipv6_get_dsfield(ipv6h) };
    1u8 << (dsfield & 0x03)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ip6_frag_reasm(
    fq: *mut frag_queue,
    skb: *mut sk_buff,
    prev_tail: *mut sk_buff,
    dev: *mut net_device,
) -> c_int {
    if fq.is_null() || skb.is_null() || dev.is_null() {
        return -ENOMEM;
    }

    unsafe { inet_frag_kill(&mut (*fq).q as *mut _) };

    let ipv6h_ptr = unsafe { ipv6_hdr(skb) };
    if ipv6h_ptr.is_null() {
        return -1;
    }

    let ecn = unsafe { ip6_frag_ecn(ipv6h_ptr) };
    let reasm = unsafe { inet_frag_reasm_prepare(&mut (*fq).q as *mut _, skb, prev_tail) };
    if reasm.is_null() {
        return -ENOMEM;
    }

    unsafe { inet_frag_reasm_finish(&mut (*fq).q as *mut _, skb, reasm, true) };

    let cb = unsafe { ip6cb(skb) };
    if !cb.is_null() {
        unsafe {
            (*cb).nhoff = (*fq).nhoffset;
            (*cb).flags |= 1 << 0;
            (*cb).frag_max_size = (*fq).q.max_size;
        }
    }

    unsafe { ipv6_change_dsfield(ipv6h_ptr, 0xff, ecn) };

    unsafe { skb_postpush_rcsum(skb, ptr::null(), skb_network_header_len(skb)) };

    let stats = unsafe { __in6_dev_stats_get((*dev).ifindex, skb) };
    unsafe { __IP6_INC_STATS(ptr::null_mut(), stats, 0) };

    unsafe {
        (*fq).q.rb_fragments = 0;
        (*fq).q.fragments_tail = ptr::null_mut();
        (*fq).q.last_run_head = ptr::null_mut();
    }

    1
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ip6_frag_expire(_t: *mut timer_list) {}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}