//! IPv6 fragment reassembly
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]  // For C-style type names

use core::ptr;
use libc::{c_int, c_uint, c_ulong, size_t};

// Constants from C
pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const EINPROGRESS: c_int = -115;
pub const ENOENT: c_int = -2;

// Type definitions
#[repr(C)]
pub struct ipv6hdr {
    pub payload_len: u16,
    pub nexthdr: u8,
    pub hop_limit: u8,
    pub saddr: [u8; 16],
    pub daddr: [u8; 16],
}

#[repr(C)]
pub struct frag_hdr {
    pub frag_off: u16,
}

#[repr(C)]
pub struct sk_buff {
    pub data: *mut u8,
    pub head: *mut u8,
    pub mac_header: *mut u8,
    pub network_header: *mut u8,
    pub transport_header: *mut u8,
    pub dev: *mut net_device,
    pub _skb_refdst: c_ulong,
    pub tstamp: u64,
    pub truesize: size_t,
}

#[repr(C)]
pub struct net_device {
    pub ifindex: c_int,
}

#[repr(C)]
pub struct inet_frags {
    _private: [u8; 0],
}

#[repr(C)]
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
pub struct frag_queue {
    q: inet_frag_queue,
    iif: c_int,
    nhoffset: c_int,
    ecn: u8,
}

#[repr(C)]
pub struct frag_v6_compare_key {
    id: __be32,
    saddr: [u8; 16],
    daddr: [u8; 16],
    user: c_int,
    iif: c_int,
}

pub type __be32 = u32;

#[repr(C)]
pub struct net {
    ipv6: ipv6_net,
}

#[repr(C)]
pub struct ipv6_net {
    fqdir: *mut inet_frags,
}

#[repr(C)]
pub struct IP6CB {
    nhoff: c_int,
    flags: c_int,
    frag_max_size: c_int,
}

// Function implementations
/// Extract ECN from IPv6 header
///
/// # Safety
/// - `ipv6h` must be a valid pointer to a `ipv6hdr`
#[no_mangle]
pub unsafe extern "C" fn ip6_frag_ecn(
    ipv6h: *const ipv6hdr,
) -> u8 {
    let dsfield = ipv6_get_dsfield(ipv6h);
    1 << (dsfield & 0x03) // INET_ECN_MASK is 0x03
}

/// Reassemble IPv6 fragments
///
/// # Safety
/// - `fq` must be a valid pointer to a `frag_queue`
/// - `skb` must be a valid pointer to a `sk_buff`
/// - `prev_tail` must be a valid pointer to a `sk_buff`
/// - `dev` must be a valid pointer to a `net_device`
#[no_mangle]
pub unsafe extern "C" fn ip6_frag_reasm(
    fq: *mut frag_queue,
    skb: *mut sk_buff,
    prev_tail: *mut sk_buff,
    dev: *mut net_device,
) -> c_int {
    let fq = &mut *fq;
    let skb = &mut *skb;
    let dev = &mut *dev;

    inet_frag_kill(&mut fq.q);

    let ecn = ip6_frag_ecn(&ipv6_hdr(skb));
    if ecn == 0xff {
        return -1;
    }

    let reasm_data = inet_frag_reasm_prepare(&mut fq.q, skb, prev_tail);
    if reasm_data.is_null() {
        return -ENOMEM;
    }

    let payload_len = ((skb.data.offset_from(skb.network_header) as usize) -
                       core::mem::size_of::<ipv6hdr>() + fq.q.len as usize -
                       core::mem::size_of::<frag_hdr>()) as u16;
    if payload_len > 0xFFFF {
        return -1;
    }

    let nhoff = fq.nhoff;
    let network_header = skb.network_header;
    let transport_header = skb.transport_header;
    let data = skb.data;

    // Move headers and payload
    let frag_hdr_size = core::mem::size_of::<frag_hdr>() as isize;
    let network_header_new = network_header.add(frag_hdr_size);
    let transport_header_new = transport_header.add(frag_hdr_size);
    let data_new = data.add(frag_hdr_size);

    // Copy data
    ptr::copy_nonoverlapping(network_header, network_header_new, frag_hdr_size as usize);
    ptr::copy_nonoverlapping(network_header, transport_header_new, frag_hdr_size as usize);
    ptr::copy_nonoverlapping(data, data_new, (skb.data.offset(data) as usize) - frag_hdr_size);

    if skb.mac_header != ptr::null_mut() {
        *skb.mac_header = (*skb.mac_header).add(frag_hdr_size);
    }

    skb.network_header = network_header_new;
    skb.transport_header = transport_header_new;
    skb.data = data_new;

    inet_frag_reasm_finish(&mut fq.q, skb, reasm_data, true);

    (*skb).dev = dev;
    let ipv6h = &mut *ipv6_hdr(skb);
    ipv6h.payload_len = payload_len;
    ipv6_change_dsfield(ipv6h, 0xff, ecn);
    IP6CB(skb).nhoff = nhoff;
    IP6CB(skb).flags |= 1 << 0; // IP6SKB_FRAGMENTED
    IP6CB(skb).frag_max_size = payload_len as c_int + core::mem::size_of::<ipv6hdr>() as c_int;

    // Calculate checksum
    skb_postpush_rcsum(skb, network_header_new, skb_network_header_len(skb));

    // Statistics
    let net = (*fq.q.fqdir).net;
    let dev_stats = __in6_dev_stats_get((*dev).ifindex, skb);
    __IP6_INC_STATS(net, dev_stats, 0); // IPSTATS_MIB_REASMOKS

    fq.q.rb_fragments = 0;
    fq.q.fragments_tail = ptr::null_mut();
    fq.q.last_run_head = ptr::null_mut();
    1
}

/// Timer handler for fragment expiration
///
/// # Safety
/// - `t` must be a valid pointer to a `timer_list`
#[no_mangle]
pub unsafe extern "C" fn ip6_frag_expire(
    t: *mut timer_list,
) {
    let frag = from_timer(t);
    let fq = container_of(frag, frag_queue, q);
    ip6frag_expire_frag_queue((*fq).q.fqdir.net, fq);
}

/// Find fragment queue
///
/// # Safety
/// - `net` must be a valid pointer to a `net`
/// - `id` must be a valid `__be32`
/// - `hdr` must be a valid pointer to a `ipv6hdr`
#[no_mangle]
pub unsafe extern "C" fn fq_find(
    net: *mut net,
    id: __be32,
    hdr: *const ipv6hdr,
    iif: c_int,
) -> *mut frag_queue {
    let mut key = frag_v6_compare_key {
        id,
        saddr: (*hdr).saddr,
        daddr: (*hdr).daddr,
        user: 1, // IP6_DEFRAG_LOCAL_DELIVER
        iif,
    };

    if !(ipv6_addr_type(&key.daddr) & (1 << 0 | 1 << 1)) {
        key.iif = 0;
    }

    let q = inet_frag_find((*net).ipv6.fqdir, &key);
    if q.is_null() {
        return ptr::null_mut();
    }

    container_of(q, frag_queue, q)
}

/// Queue a fragment for reassembly
///
/// # Safety
/// - `fq` must be a valid pointer to a `frag_queue`
/// - `skb` must be a valid pointer to a `sk_buff`
/// - `fhdr` must be a valid pointer to a `frag_hdr`
#[no_mangle]
pub unsafe extern "C" fn ip6_frag_queue(
    fq: *mut frag_queue,
    skb: *mut sk_buff,
    fhdr: *mut frag_hdr,
    nhoff: c_int,
    prob_offset: *mut u32,
) -> c_int {
    let fq = &mut *fq;
    let skb = &mut *skb;
    let fhdr = &mut *fhdr;

    if fq.q.flags & 1 != 0 { // INET_FRAG_COMPLETE
        return -ENOENT;
    }

    let offset = ntohs(fhdr.frag_off) & !0x7;
    let payload_len = ntohs((*ipv6_hdr(skb)).payload_len);
    let payload_offset = (fhdr as *mut _ as usize) - (skb.data as usize);
    let end = offset + (payload_len - payload_offset as u16);

    if end > 0xFFFF {
        *prob_offset = (offsetof!(frag_hdr, frag_off) as u32) - skb_network_header_offset(skb);
        return -1;
    }

    let ecn = ip6_frag_ecn(ipv6_hdr(skb));

    if skb.ip_summed == 1 { // CHECKSUM_COMPLETE
        let nh = skb_network_header(skb);
        let nh_len = (fhdr as *mut _ as usize) - (nh as usize);
        skb.csum = csum_sub(skb.csum, csum_partial(nh, nh_len, 0));
    }

    if fhdr.frag_off & htons(1) == 0 { // IP6_MF
        if end < fq.q.len || (fq.q.flags & (1 << 1) != 0 && end != fq.q.len) { // INET_FRAG_LAST_IN
        }
        fq.q.flags |= 1 << 1; // INET_FRAG_LAST_IN
        fq.q.len = end;
    } else {
        if end & 0x7 != 0 {
            *prob_offset = offsetof!(ipv6hdr, payload_len) as u32;
            return -1;
        }
        if end > fq.q.len {
            if fq.q.flags & (1 << 1) != 0 { // INET_FRAG_LAST_IN
            }
            fq.q.len = end;
        }
    }

    if end == offset {
    }

    if !pskb_pull(skb, (fhdr as *mut _ as usize) - (skb.data as usize)) {
    }

    if pskb_trim_rcsum(skb, end - offset) != 0 {
    }

    let dev = skb.dev;
    let prev_tail = fq.q.fragments_tail;
    let err = inet_frag_queue_insert(&mut fq.q, skb, offset, end);
    if err != 0 {
        if err == 1 { // IPFRAG_DUP
            kfree_skb(skb);
            return -EINVAL;
        }
        __IP6_INC_STATS(dev_net(skb_dst(skb)), ip6_dst_idev(skb_dst(skb)), 1); // IPSTATS_MIB_REASM_OVERLAPS
    }

    if !dev.is_null() {
        fq.iif = (*dev).ifindex;
    }

    fq.q.stamp = skb.tstamp;
    fq.q.meat += skb.truesize;
    fq.ecn |= ecn;
    add_frag_mem_limit(fq.q.fqdir, skb.truesize);

    let fragsize = -skb_network_offset(skb) + skb.len as c_int;
    if fragsize > fq.q.max_size {
        fq.q.max_size = fragsize;
    }

    if offset == 0 {
        fq.nhoff = nhoff;
        fq.q.flags |= 1 << 0; // INET_FRAG_FIRST_IN
    }

    if fq.q.flags == (1 << 0 | 1 << 1) && fq.q.meat == fq.q.len {
        let orefdst = skb._skb_refdst;
        skb._skb_refdst = 0;
        let err = ip6_frag_reasm(fq, skb, prev_tail, dev);
        skb._skb_refdst = orefdst;
        return err;
    }

    skb_dst_drop(skb);
    -EINPROGRESS;

    inet_frag_kill(&mut fq.q);
    __IP6_INC_STATS(dev_net(skb_dst(skb)), ip6_dst_idev(skb_dst(skb)), 1); // IPSTATS_MIB_REASMFAILS
    kfree_skb(skb);
    -EINVAL;
}

// Helper functions (extern declarations)
extern "C" {
    fn ntohs(x: u16) -> u16;
    fn htons(x: u16) -> u16;
    fn ipv6_get_dsfield(ipv6h: *const ipv6hdr) -> u8;
    fn ipv6_change_dsfield(ipv6h: *mut ipv6hdr, mask: u8, val: u8);
    fn inet_frag_kill(q: *mut inet_frag_queue);
    fn inet_frag_reasm_prepare(q: *mut inet_frag_queue, skb: *mut sk_buff, prev_tail: *mut sk_buff) -> *mut c_void;
    fn inet_frag_reasm_finish(q: *mut inet_frag_queue, skb: *mut sk_buff, reasm_data: *mut c_void, flag: bool);
    fn skb_postpush_rcsum(skb: *mut sk_buff, data: *mut u8, len: c_int);
    fn skb_network_header(skb: *mut sk_buff) -> *mut u8;
    fn skb_network_header_len(skb: *mut sk_buff) -> c_int;
    fn pskb_pull(skb: *mut sk_buff, len: c_int) -> bool;
    fn pskb_trim_rcsum(skb: *mut sk_buff, len: c_int) -> c_int;
    fn inet_frag_queue_insert(q: *mut inet_frag_queue, skb: *mut sk_buff, offset: c_int, end: c_int) -> c_int;
    fn kfree_skb(skb: *mut sk_buff);
    fn add_frag_mem_limit(fqdir: *mut inet_frags, size: size_t);
    fn dev_net(skb_dst: *mut sk_buff) -> *mut net;
    fn ip6_dst_idev(skb_dst: *mut sk_buff) -> *mut c_void;
    fn __in6_dev_stats_get(ifindex: c_int, skb: *mut sk_buff) -> *mut c_void;
    fn __IP6_INC_STATS(net: *mut net, stats: *mut c_void, mib: c_int);
    fn ip6frag_expire_frag_queue(net: *mut net, fq: *mut frag_queue);
    fn container_of(ptr: *mut c_void, container_type: *mut c_void, member: *mut c_void) -> *mut c_void;
    fn from_timer(t: *mut timer_list) -> *mut inet_frag_queue;
}

// SAFETY: These functions are assumed to be provided by the Linux kernel
// and their implementations are not included here.