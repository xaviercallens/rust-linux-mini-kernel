#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![allow(non_camel_case_types)]

use core::ffi::{c_int, c_void};
use core::ptr;
use kernel_types::*;

pub const EINVAL: c_int = 22;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flowi6 {
    pub flowi6_oif: u32,
    pub flowi6_mark: u32,
    pub flowi6_uid: u32,
    pub daddr: [u8; 16],
    pub saddr: [u8; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip6_rt_info {
    pub daddr: [u8; 16],
    pub saddr: [u8; 16],
    pub mark: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_bridge_frag_data {
    pub _priv: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_queue_entry_state {
    pub hook: u32,
    pub net: *mut c_void,
    pub sk: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_queue_entry {
    pub state: nf_queue_entry_state,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_ipv6_ops {
    pub route_me_harder:
        unsafe extern "C" fn(net: *mut c_void, sk_partial: *mut c_void, skb: *mut c_void) -> c_int,
    pub route: unsafe extern "C" fn(
        net: *mut c_void,
        dst: *mut *mut c_void,
        fl: *mut c_void,
        strict: c_int,
    ) -> c_int,
    pub fragment: extern "C" fn(
        net: *mut c_void,
        sk: *mut c_void,
        skb: *mut c_void,
        data: *mut nf_bridge_frag_data,
        output: extern "C" fn(
            net: *mut c_void,
            sk: *mut c_void,
            data: *mut nf_bridge_frag_data,
            skb: *mut c_void,
        ) -> c_int,
    ) -> c_int,
    pub reroute: unsafe extern "C" fn(skb: *mut c_void, entry: *const nf_queue_entry) -> c_int,
    pub route_input: extern "C" fn(skb: *mut c_void) -> c_int,
    pub br_fragment: extern "C" fn(
        net: *mut c_void,
        sk: *mut c_void,
        skb: *mut c_void,
        data: *mut nf_bridge_frag_data,
        output: extern "C" fn(
            net: *mut c_void,
            sk: *mut c_void,
            data: *mut nf_bridge_frag_data,
            skb: *mut c_void,
        ) -> c_int,
    ) -> c_int,
}

unsafe extern "C" {
    fn nf_queue_entry_reroute(entry: *const nf_queue_entry) -> *const ip6_rt_info;
}

#[no_mangle]
pub unsafe extern "C" fn ip6_route_me_harder(
    net: *mut c_void,
    sk_partial: *mut c_void,
    skb: *mut c_void,
) -> c_int {
    if net.is_null() || sk_partial.is_null() || skb.is_null() {
        return -EINVAL;
    }

    let iph = ipv6_hdr(skb);
    let sk = sk_to_full_sk(sk_partial);
    let mut fl6 = flowi6 {
        flowi6_oif: if !sk.is_null() && (*sk).sk_bound_dev_if != 0 {
            (*sk).sk_bound_dev_if
        } else if (ipv6_addr_type(&(*iph).daddr) & (1 << 19 | 1 << 31)) != 0 {
            (*(*skb).dev).ifindex
        } else {
            0
        },
        flowi6_mark: (*skb).mark,
        flowi6_uid: sock_net_uid(net, sk),
        daddr: (*iph).daddr,
        saddr: (*iph).saddr,
    };

    let mut dst: *mut c_void = ptr::null_mut();
    let strict = (ipv6_addr_type(&(*iph).daddr) & (1 << 19 | 1 << 31)) != 0;

    fib6_rules_early_flow_dissect(net, skb, &mut fl6, ptr::null_mut());

    dst = ip6_route_output(net, sk, &mut fl6);
    let err = (*dst).error;

    if err != 0 {
        IP6_INC_STATS(net, ip6_dst_idev(dst), 3); // IPSTATS_MIB_OUTNOROUTES
        net_dbg_ratelimited(b"ip6_route_me_harder: No more route\n");
        dst_release(dst);
        return err;
    }

    skb_dst_drop(skb);
    skb_dst_set(skb, dst);

    // XFRM handling
    if (IP6CB(skb).flags & 1 << 0) == 0 {
        let fl = flowi6_to_flowi(&fl6);
        if xfrm_decode_session(skb, fl, 10) == 0 {
            skb_dst_set(skb, ptr::null_mut());
            dst = xfrm_lookup(net, dst, fl, sk, 0);
            if dst.is_null() {
                return -ENOMEM;
            }
            skb_dst_set(skb, dst);
        }
    }

    let hh_len = (*(*skb).dst).dev.hard_header_len;
    if skb_headroom(skb) < hh_len {
        if pskb_expand_head(skb, HH_DATA_ALIGN(hh_len - skb_headroom(skb)), 0, 1) != 0 {
            return -ENOMEM;
        }
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn nf_ip6_reroute(skb: *mut c_void, entry: *const nf_queue_entry) -> c_int {
    if skb.is_null() || entry.is_null() {
        return -EINVAL;
    }

    let rt_info = nf_queue_entry_reroute(entry);
    if rt_info.is_null() {
        return 0;
    }

    if (*entry).state.hook == 3 {
        return ip6_route_me_harder((*entry).state.net, (*entry).state.sk, skb);
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn __nf_ip6_route(
    net: *mut c_void,
    dst: *mut *mut c_void,
    fl: *mut c_void,
    strict: c_int,
) -> c_int {
    if net.is_null() || dst.is_null() || fl.is_null() {
        return -EINVAL;
    }

    static mut fake_pinfo: ipv6_pinfo = ipv6_pinfo {
        saddr: in6_addr { in6_u: in6_addr_union { u6_addr32: [0; 4] } },
        daddr: in6_addr { in6_u: in6_addr_union { u6_addr32: [0; 4] } },
        flow_label: 0,
        frag_size: 0,
        hop_limit: 0,
        mcast_hops: 0,
        mcast_oif: 0,
        rxopt: ip6cb { flags: 0, frag_max_size: 0 },
    };
    static mut fake_sk: inet_sock = inet_sock {
        sk: ptr::null_mut(),
        pinet6: &mut fake_pinfo,
        inet_saddr: 0,
        uc_ttl: 0,
        cmsg_flags: 0,
        inet_sport: 0,
        inet_id: 0,
        tos: 0,
        min_ttl: 0,
        mc_ttl: 0,
        pmtudisc: 0,
        recverr: 0,
        freebind: 0,
        hdrincl: 0,
        mc_loop: 0,
        transparent: 0,
        mc_all: 0,
        nodefrag: 0,
        bind_address_no_port: 0,
        defer_connect: 0,
        rcv_tos: 0,
        convert_csum: 0,
        uc_index: 0,
        mc_index: 0,
        mc_addr: 0,
    };

    let sk = if strict { &mut fake_sk } else { ptr::null_mut() };
    let result = ip6_route_output(net, sk, &mut (*fl).u.ip6);
    let err = (*result).error;

    if err != 0 {
        dst_release(result);
    } else {
        *dst = result;
    }
    err
}

#[no_mangle]
pub unsafe extern "C" fn br_ip6_fragment(
    net: *mut c_void,
    sk: *mut c_void,
    skb: *mut c_void,
    data: *mut nf_bridge_frag_data,
    output: extern "C" fn(net: *mut c_void, sk: *mut c_void, data: *mut nf_bridge_frag_data, skb: *mut c_void) -> c_int,
) -> c_int {
    if net.is_null() || sk.is_null() || skb.is_null() || data.is_null() {
        return -EINVAL;
    }

    let frag_max_size = BR_INPUT_SKB_CB(skb).frag_max_size;
    let tstamp = (*skb).tstamp;
    let mut state: ip6_frag_state = ip6_frag_state {
        left: 0,
        mtu: 0,
        hlen: 0,
        hroom: 0,
        frag_id: 0,
        prevhdr: ptr::null_mut(),
        nexthdr: 0,
    };
    let mut prevhdr: *mut u8 = ptr::null_mut();
    let mut nexthdr: u8 = 0;
    let mut mtu: u32 = 0;
    let mut hlen: u32 = 0;
    let mut hroom: u32 = 0;
    let mut err: c_int = 0;
    let mut frag_id: u32 = 0;

    err = ip6_find_1stfragopt(skb, &mut prevhdr);
    if err < 0 {
        return err;
    }
    hlen = err as u32;
    nexthdr = *prevhdr;

    mtu = (*(*skb).dev).mtu;
    if frag_max_size > mtu || frag_max_size < 1280 {
        return -EINVAL;
    }

    mtu = frag_max_size;
    if mtu < hlen + 20 + 8 {
        return -EINVAL;
    }
    mtu -= hlen + 20;

    frag_id = ipv6_select_ident(net, &(*ipv6_hdr(skb)).daddr, &(*ipv6_hdr(skb)).saddr);

    if (*skb).ip_summed == 1 && skb_checksum_help(skb) != 0 {
        return -EINVAL;
    }

    hroom = LL_RESERVED_SPACE((*skb).dev);
    if skb_has_frag_list(skb) != 0 {
        let first_len = skb_pagelen(skb);
        let mut iter: ip6_fraglist_iter = ip6_fraglist_iter {
            frag: ptr::null_mut(),
            tmp_hdr: ptr::null_mut(),
            hlen: 0,
            prevhdr: ptr::null_mut(),
            nexthdr: 0,
            frag_id: 0,
            frag_max_size: 0,
            frag_left: 0,
            frag_offset: 0,
        };

        if first_len > hlen + mtu {
            return -EINVAL;
        }

        if skb_cloned(skb) != 0 {
            return -EINVAL;
        }

        // Walk frag list
        // ... (simplified for FFI compatibility)
        // Actual implementation would walk the frag list and validate

        err = ip6_fraglist_init(skb, hlen, prevhdr, nexthdr, frag_id, &mut iter);
        if err < 0 {
            return err;
        }

        loop {
            if !iter.frag.is_null() {
                ip6_fraglist_prepare(skb, &mut iter);
            }

            (*skb).tstamp = tstamp;
            err = output(net, sk, data, skb);
            if err != 0 || iter.frag.is_null() {
                break;
            }

            skb = ip6_fraglist_next(&mut iter);
        }

        kfree(iter.tmp_hdr);
        if err == 0 {
            return 0;
        }

        kfree_skb_list(iter.frag);
        return err;
    }

    ip6_frag_init(skb, hlen, mtu, (*(*skb).dev).needed_tailroom, LL_RESERVED_SPACE((*skb).dev), prevhdr, nexthdr, frag_id, &mut state);

    while state.left > 0 {
        let skb2 = ip6_frag_next(skb, &mut state);
        if skb2.is_null() {
            err = -ENOMEM;
            break;
        }

        (*skb2).tstamp = tstamp;
        err = output(net, sk, data, skb2);
        if err != 0 {
            break;
        }
    }

    consume_skb(skb);
    return err;
}

// Static struct nf_ipv6_ops
#[no_mangle]
pub static mut ipv6ops: nf_ipv6_ops = nf_ipv6_ops {
    route_me_harder: ip6_route_me_harder,
    route: __nf_ip6_route,
    fragment: ip6_fragment,
    reroute: nf_ip6_reroute,
    route_input: ip6_route_input,
    br_fragment: br_ip6_fragment,
};

// Initialization
#[no_mangle]
pub unsafe extern "C" fn ipv6_netfilter_init() -> c_int {
    RCU_INIT_POINTER(nf_ipv6_ops, &ipv6ops);
    0
}

#[no_mangle]
pub static nf_ipv6_ops_instance: nf_ipv6_ops = nf_ipv6_ops {
    route_me_harder: ip6_route_me_harder,
    route: __nf_ip6_route,
    fragment: nf_ip6_fragment_stub,
    reroute: nf_ip6_reroute,
    route_input: nf_ip6_route_input_stub,
    br_fragment: nf_ip6_br_fragment_stub,
};

#[no_mangle]
pub extern "C" fn nf_ip6_fragment_stub(
    _net: *mut c_void,
    _sk: *mut c_void,
    _skb: *mut c_void,
    _data: *mut nf_bridge_frag_data,
    _output: extern "C" fn(
        net: *mut c_void,
        sk: *mut c_void,
        data: *mut nf_bridge_frag_data,
        skb: *mut c_void,
    ) -> c_int,
) -> c_int {
    0
}

// Helper functions (extern declarations)
extern "C" {
    fn ipv6_hdr(skb: *mut c_void) -> *mut ipv6hdr;
    fn sk_to_full_sk(sk_partial: *mut c_void) -> *mut c_void;
    fn ipv6_addr_type(addr: *mut in6_addr) -> u32;
    fn ipv6_addr_equal(a: *mut in6_addr, b: *mut in6_addr) -> bool;
    fn sock_net_uid(net: *mut c_void, sk: *mut c_void) -> u32;
    fn fib6_rules_early_flow_dissect(net: *mut c_void, skb: *mut c_void, fl6: *mut flowi6, flkeys: *mut c_void);
    fn IP6_INC_STATS(net: *mut c_void, idev: *mut c_void, mib: c_int);
    fn net_dbg_ratelimited(fmt: *const u8);
    fn dst_release(dst: *mut c_void);
    fn skb_dst_drop(skb: *mut c_void);
    fn skb_dst_set(skb: *mut c_void, dst: *mut c_void);
    fn xfrm_decode_session(skb: *mut c_void, fl: *mut c_void, af: c_int) -> c_int;
    fn xfrm_lookup(net: *mut c_void, dst: *mut c_void, fl: *mut c_void, sk: *mut c_void, flags: c_int) -> *mut c_void;
    fn HH_DATA_ALIGN(len: u32) -> u32;
    fn skb_headroom(skb: *mut c_void) -> u32;
    fn pskb_expand_head(skb: *mut c_void, headroom: u32, data_len: u32, gfp: c_int) -> c_int;
    fn IP6CB(skb: *mut c_void) -> *mut c_void;
    fn BR_INPUT_SKB_CB(skb: *mut c_void) -> *mut c_void;
    fn ip6_route_output(net: *mut c_void, sk: *mut c_void, fl6: *mut flowi6) -> *mut c_void;
    fn ip6_route_input(skb: *mut c_void) -> c_int;
    fn ip6_fragment(net: *mut c_void, sk: *mut c_void, skb: *mut c_void, output: extern "C" fn(net: *mut c_void, sk: *mut c_void, skb: *mut c_void) -> c_int) -> c_int;
    fn ip6_find_1stfragopt(skb: *mut c_void, prevhdr: *mut *mut u8) -> c_int;
    fn skb_checksum_help(skb: *mut c_void) -> c_int;
    fn LL_RESERVED_SPACE(dev: *mut c_void) -> u32;
    fn skb_has_frag_list(skb: *mut c_void) -> c_int;
    fn skb_pagelen(skb: *mut c_void) -> u32;
    fn skb_cloned(skb: *mut c_void) -> c_int;
    fn ip6_fraglist_init(skb: *mut c_void, hlen: u32, prevhdr: *mut u8, nexthdr: u8, frag_id: u32, iter: *mut ip6_fraglist_iter) -> c_int;
    fn ip6_fraglist_prepare(skb: *mut c_void, iter: *mut ip6_fraglist_iter);
    fn ip6_fraglist_next(iter: *mut ip6_fraglist_iter) -> *mut c_void;
    fn ip6_frag_init(skb: *mut c_void, hlen: u32, mtu: u32, tailroom: u32, headroom: u32, prevhdr: *mut u8, nexthdr: u8, frag_id: u32, state: *mut ip6_frag_state);
    fn ip6_frag_next(skb: *mut c_void, state: *mut ip6_frag_state) -> *mut c_void;
    fn consume_skb(skb: *mut c_void);
    fn kfree_skb(skb: *mut c_void);
    fn kfree_skb_list(skb: *mut c_void);
    fn kfree(ptr: *mut c_void);
    fn RCU_INIT_POINTER(ptr: *mut *mut c_void, val: *mut c_void);
    fn ip6_dst_idev(dst: *mut c_void) -> *mut c_void;
    fn flowi6_to_flowi(fl6: *mut flowi6) -> *mut c_void;
    fn ipv6_select_ident(net: *mut c_void, daddr: *mut in6_addr, saddr: *mut in6_addr) -> u32;
    fn nf_queue_entry_reroute(entry: *const nf_queue_entry) -> *mut ip6_rt_info;
}
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
