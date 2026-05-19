#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![no_builtins]
#![allow(non_camel_case_types)]

use core::ptr;
use kernel_types::*;

pub const ENOMEM: c_int = 12;
pub const ENOENT: c_int = 2;
pub const EINPROGRESS: c_int = 115;

pub type __be16 = u16;
pub type __be32 = u32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sk_buff {
    _private: [u8; 0],
}

// Type definitions

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
    let fq = &mut *fq;
    let skb = &mut *skb;
    let dev = &mut *dev;

    inet_frag_kill(&mut fq.q);

    let ecn = ip6_frag_ecn(ipv6_hdr(skb));
    if ecn == 0xff {
        return -1;
    }

    let reasm_data = inet_frag_reasm_prepare(&mut fq.q, skb, prev_tail);
    if reasm_data.is_null() {
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
        saddr: (*hdr).saddr.in6_u.u6_addr8,
        daddr: (*hdr).daddr.in6_u.u6_addr8,
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
        *prob_offset = (core::mem::offset_of!(frag_hdr, frag_off) as u32) - skb_network_header_offset(skb);
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
            // Handle error case
            return -1;
        }
        fq.q.flags |= 1 << 1; // INET_FRAG_LAST_IN
        fq.q.len = end;
    } else {
        if end & 0x7 != 0 {
            *prob_offset = core::mem::offset_of!(ipv6hdr, payload_len) as u32;
            return -1;
        }
        if end > fq.q.len {
            if fq.q.flags & (1 << 1) != 0 { // INET_FRAG_LAST_IN
                // Handle error case
                return -1;
            }
            fq.q.len = end;
        }
    }

    if end == offset {
        // Handle error case
        return -1;
    }

    if !pskb_pull(skb, (fhdr as *mut _ as usize) - (skb.data as usize)) {
        // Handle error case
        return -1;
    }

    if pskb_trim_rcsum(skb, end - offset) != 0 {
        // Handle error case
        return -1;
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
        fq.nhoffset = nhoff;
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
    -EINPROGRESS
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
    fn ipv6_hdr(skb: *mut sk_buff) -> *mut ipv6hdr;
    fn ipv6_addr_type(addr: *const [u8; 16]) -> u32;
    fn inet_frag_find(fqdir: *mut inet_frags, key: *const frag_v6_compare_key) -> *mut inet_frag_queue;
    fn skb_network_offset(skb: *mut sk_buff) -> c_int;
    fn skb_network_header_offset(skb: *mut sk_buff) -> c_int;
    fn skb_dst(skb: *mut sk_buff) -> *mut sk_buff;
    fn skb_dst_drop(skb: *mut sk_buff);
}
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
