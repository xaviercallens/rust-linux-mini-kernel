//! SR-IPv6 implementation for Linux kernel
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]  // For C-style type names

use core::ptr;
use libc::{c_int, c_uint, c_ulong, c_void, size_t};

// Constants from C
pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ENOSYS: c_int = -38;

// Type definitions
#[repr(C)]
struct seg6_local_lwtunnel_ops {
    build_state: Option<unsafe extern "C" fn(*mut seg6_local_lwt, *const c_void, *mut c_void) -> c_int>,
    destroy_state: Option<unsafe extern "C" fn(*mut seg6_local_lwt)>,
}

#[repr(C)]
struct seg6_action_desc {
    action: c_int,
    attrs: c_ulong,
    optattrs: c_ulong,
    input: Option<unsafe extern "C" fn(*mut c_void, *mut seg6_local_lwt) -> c_int>,
    static_headroom: c_int,
    slwt_ops: seg6_local_lwtunnel_ops,
}

#[repr(C)]
struct bpf_lwt_prog {
    prog: *mut c_void,
    name: *mut c_char,
}

#[repr(C)]
enum seg6_end_dt_mode {
    DT_INVALID_MODE = -1,
    DT_LEGACY_MODE = 0,
    DT_VRF_MODE = 1,
}

#[repr(C)]
struct seg6_end_dt_info {
    mode: seg6_end_dt_mode,
    net: *mut c_void,
    vrf_ifindex: c_int,
    vrf_table: c_int,
    proto: u16,
    family: u16,
    hdrlen: c_int,
}

#[repr(C)]
struct pcpu_seg6_local_counters {
    packets: u64,
    bytes: u64,
    errors: u64,
    syncp: u64_stats_sync,
}

#[repr(C)]
struct seg6_local_counters {
    packets: u64,
    bytes: u64,
    errors: u64,
}

#[repr(C)]
struct seg6_local_lwt {
    action: c_int,
    srh: *mut ipv6_sr_hdr,
    table: c_int,
    nh4: in_addr,
    nh6: in6_addr,
    iif: c_int,
    oif: c_int,
    bpf: bpf_lwt_prog,
    #[cfg(CONFIG_NET_L3_MASTER_DEV)]
    dt_info: seg6_end_dt_info,
    pcpu_counters: *mut pcpu_seg6_local_counters,
    headroom: c_int,
    desc: *mut seg6_action_desc,
    parsed_optattrs: c_ulong,
}

#[repr(C)]
struct lwtunnel_state {
    data: *mut c_void,
}

#[repr(C)]
struct sk_buff {
    dev: *mut c_void,
    data: *mut u8,
    len: c_int,
    mark: c_int,
    // ... other fields as needed
}

#[repr(C)]
struct ipv6hdr {
    saddr: in6_addr,
    daddr: in6_addr,
    nexthdr: u8,
    // ... other fields as needed
}

#[repr(C)]
struct ipv6_sr_hdr {
    hdrlen: u8,
    segments_left: u8,
    segments: [in6_addr; 0], // Flexible array member
}

// Function implementations
#[no_mangle]
pub unsafe extern "C" fn seg6_local_lwtunnel(lwt: *mut lwtunnel_state) -> *mut seg6_local_lwt {
    // SAFETY: lwt is valid pointer as per kernel API
    &mut (*lwt).data as *mut seg6_local_lwt
}

#[no_mangle]
pub unsafe extern "C" fn get_srh(skb: *mut sk_buff, flags: c_int) -> *mut ipv6_sr_hdr {
    let mut srhoff: c_int = 0;
    
    if ipv6_find_hdr(skb, &mut srhoff as *mut c_int, IPPROTO_ROUTING, ptr::null_mut(), &flags) < 0 {
        return ptr::null_mut();
    }

    if !pskb_may_pull(skb, srhoff + size_of::<ipv6_sr_hdr>()) {
        return ptr::null_mut();
    }

    let srh = (skb_data(skb) as *mut u8).offset(srhoff as isize) as *mut ipv6_sr_hdr;

    let len = ((*srh).hdrlen + 1) << 3;
    if !pskb_may_pull(skb, srhoff + len) {
        return ptr::null_mut();
    }

    // Reload srh after pull
    let srh = (skb_data(skb) as *mut u8).offset(srhoff as isize) as *mut ipv6_sr_hdr;

    if !seg6_validate_srh(srh, len, true) {
        return ptr::null_mut();
    }

    srh
}

#[no_mangle]
pub unsafe extern "C" fn get_and_validate_srh(skb: *mut sk_buff) -> *mut ipv6_sr_hdr {
    let srh = get_srh(skb, IP6_FH_F_SKIP_RH);
    if srh.is_null() {
        return ptr::null_mut();
    }

    #[cfg(CONFIG_IPV6_SEG6_HMAC)]
    if !seg6_hmac_validate_skb(skb) {
        return ptr::null_mut();
    }

    srh
}

#[no_mangle]
pub unsafe extern "C" fn decap_and_validate(skb: *mut sk_buff, proto: c_int) -> bool {
    let mut srh = get_srh(skb, 0);
    if !srh.is_null() && (*srh).segments_left > 0 {
        return false;
    }

    #[cfg(CONFIG_IPV6_SEG6_HMAC)]
    if !srh.is_null() && !seg6_hmac_validate_skb(skb) {
        return false;
    }

    let mut off: c_int = 0;
    if ipv6_find_hdr(skb, &mut off as *mut c_int, proto, ptr::null_mut(), ptr::null_mut()) < 0 {
        return false;
    }

    if !pskb_pull(skb, off as size_t) {
        return false;
    }

    skb_postpull_rcsum(skb, skb_network_header(skb), off as size_t);

    skb_reset_network_header(skb);
    skb_reset_transport_header(skb);
    if iptunnel_pull_offloads(skb) < 0 {
        return false;
    }

    true
}

#[no_mangle]
pub unsafe extern "C" fn advance_nextseg(srh: *mut ipv6_sr_hdr, daddr: *mut in6_addr) {
    (*srh).segments_left -= 1;
    let addr = &(*srh).segments[(*srh).segments_left as usize];
    *daddr = *addr;
}

#[no_mangle]
pub unsafe extern "C" fn seg6_lookup_any_nexthop(
    skb: *mut sk_buff,
    nhaddr: *mut in6_addr,
    tbl_id: u32,
    local_delivery: bool
) -> c_int {
    let net = dev_net((*skb).dev);
    let hdr = ipv6_hdr(skb);
    let mut fl6 = Default::default();
    
    fl6.flowi6_iif = (*skb).dev->ifindex;
    fl6.daddr = if !nhaddr.is_null() { (*nhaddr).s6_addr } else { (*hdr).daddr.s6_addr };
    fl6.saddr = (*hdr).saddr.s6_addr;
    fl6.flowlabel = ip6_flowinfo(hdr);
    fl6.flowi6_mark = (*skb).mark;
    fl6.flowi6_proto = (*hdr).nexthdr;

    if !nhaddr.is_null() {
        fl6.flowi6_flags = FLOWI_FLAG_KNOWN_NH;
    }

    let mut dst: *mut dst_entry = ptr::null_mut();
    
    if tbl_id == 0 {
        dst = ip6_route_input_lookup(net, (*skb).dev, &fl6, skb, RT6_LOOKUP_F_HAS_SADDR);
    } else {
        let table = fib6_get_table(net, tbl_id);
        if !table.is_null() {
            dst = ip6_pol_route(net, table, 0, &fl6, skb, RT6_LOOKUP_F_HAS_SADDR);
        }
    }

    let dev_flags = if !local_delivery { IFF_LOOPBACK } else { 0 };
    
    if !dst.is_null() && ( (*dst).dev->flags & dev_flags ) != 0 && (*dst).error == 0 {
        dst_release(dst);
        dst = ptr::null_mut();
    }

    if dst.is_null() {
        dst = &(*net).ipv6.ip6_blk_hole_entry.dst;
        dst_hold(dst);
    }

    skb_dst_drop(skb);
    skb_dst_set(skb, dst);
    (*dst).error
}

#[no_mangle]
pub unsafe extern "C" fn seg6_lookup_nexthop(skb: *mut sk_buff, nhaddr: *mut in6_addr, tbl_id: u32) -> c_int {
    seg6_lookup_any_nexthop(skb, nhaddr, tbl_id, false)
}

#[no_mangle]
pub unsafe extern "C" fn input_action_end(skb: *mut sk_buff, slwt: *mut seg6_local_lwt) -> c_int {
    let srh = get_and_validate_srh(skb);
    if srh.is_null() {
        kfree_skb(skb);
        return EINVAL;
    }

    advance_nextseg(srh, &mut (*ipv6_hdr(skb)).daddr);
    seg6_lookup_nexthop(skb, ptr::null_mut(), 0);
    
    dst_input(skb)
}

#[no_mangle]
pub unsafe extern "C" fn input_action_end_x(skb: *mut sk_buff, slwt: *mut seg6_local_lwt) -> c_int {
    let srh = get_and_validate_srh(skb);
    if srh.is_null() {
        kfree_skb(skb);
        return EINVAL;
    }

    advance_nextseg(srh, &mut (*ipv6_hdr(skb)).daddr);
    seg6_lookup_nexthop(skb, &(*slwt).nh6, 0);
    
    dst_input(skb)
}

#[no_mangle]
pub unsafe extern "C" fn input_action_end_t(skb: *mut sk_buff, slwt: *mut seg6_local_lwt) -> c_int {
    let srh = get_and_validate_srh(skb);
    if srh.is_null() {
        kfree_skb(skb);
        return EINVAL;
    }

    advance_nextseg(srh, &mut (*ipv6_hdr(skb)).daddr);
    seg6_lookup_nexthop(skb, ptr::null_mut(), (*slwt).table as u32);
    
    dst_input(skb)
}

#[no_mangle]
pub unsafe extern "C" fn input_action_end_dx2(skb: *mut sk_buff, slwt: *mut seg6_local_lwt) -> c_int {
    let net = dev_net((*skb).dev);
    let mut eth = ptr::null_mut();
    
    if !decap_and_validate(skb, IPPROTO_ETHERNET) {
        kfree_skb(skb);
        return EINVAL;
    }

    if !pskb_may_pull(skb, ETH_HLEN) {
        kfree_skb(skb);
        return EINVAL;
    }

    skb_reset_mac_header(skb);
    eth = skb_data(skb) as *mut ethhdr;

    if !eth_proto_is_802_3((*eth).h_proto) {
        kfree_skb(skb);
        return EINVAL;
    }

    let odev = dev_get_by_index_rcu(net, (*slwt).oif);
    if odev.is_null() {
        kfree_skb(skb);
        return EINVAL;
    }

    if (*odev).type != ARPHRD_ETHER {
        kfree_skb(skb);
        return EINVAL;
    }

    if !((*odev).flags & IFF_UP) || !netif_carrier_ok(odev) {
        kfree_skb(skb);
        return EINVAL;
    }

    skb_orphan(skb);
    
    if skb_warn_if_lro(skb) {
        kfree_skb(skb);
        return EINVAL;
    }

    skb_forward_csum(skb);

    if (*skb).len - ETH_HLEN > (*odev).mtu {
        kfree_skb(skb);
        return EINVAL;
    }

    (*skb).dev = odev;
    (*skb).protocol = (*eth).h_proto;
    
    dev_queue_xmit(skb)
}

#[no_mangle]
pub unsafe extern "C" fn input_action_end_dx6(skb: *mut sk_buff, slwt: *mut seg6_local_lwt) -> c_int {
    let mut nhaddr: *mut in6_addr = ptr::null_mut();
    
    if !decap_and_validate(skb, IPPROTO_IPV6) {
        kfree_skb(skb);
        return EINVAL;
    }

    if !pskb_may_pull(skb, size_of::<ipv6hdr>()) {
        kfree_skb(skb);
        return EINVAL;
    }

    if !ipv6_addr_any(&(*slwt).nh6) {
        nhaddr = &(*slwt).nh6;
    }

    skb_set_transport_header(skb, size_of::<ipv6hdr>());
    
    seg6_lookup_nexthop(skb, nhaddr, 0);
    
    dst_input(skb)
}

// Helper functions (extern declarations)
extern "C" {
    fn ipv6_find_hdr(skb: *mut sk_buff, offset: *mut c_int, proto: c_int, 
                     csum: *mut u16, flags: *mut c_int) -> c_int;
    fn pskb_may_pull(skb: *mut sk_buff, len: size_t) -> bool;
    fn skb_data(skb: *mut sk_buff) -> *mut u8;
    fn seg6_validate_srh(srh: *mut ipv6_sr_hdr, len: size_t, strict: bool) -> bool;
    #[cfg(CONFIG_IPV6_SEG6_HMAC)]
    fn seg6_hmac_validate_skb(skb: *mut sk_buff) -> bool;
    fn dev_net(dev: *mut c_void) -> *mut c_void;
    fn ipv6_hdr(skb: *mut sk_buff) -> *mut ipv6hdr;
    fn ip6_flowinfo(hdr: *mut ipv6hdr) -> u32;
    fn ip6_route_input_lookup(net: *mut c_void, dev: *mut c_void, fl6: *mut c_void, 
                             skb: *mut sk_buff, flags: c_int) -> *mut dst_entry;
    fn fib6_get_table(net: *mut c_void, id: u32) -> *mut c_void;
    fn ip6_pol_route(net: *mut c_void, table: *mut c_void, flags: c_int, 
                    fl6: *mut c_void, skb: *mut sk_buff, flags2: c_int) -> *mut rt6_info;
    fn dst_input(skb: *mut sk_buff) -> c_int;
    fn kfree_skb(skb: *mut sk_buff);
    fn skb_reset_network_header(skb: *mut sk_buff);
    fn skb_reset_transport_header(skb: *mut sk_buff);
    fn iptunnel_pull_offloads(skb: *mut sk_buff) -> c_int;
    fn dev_get_by_index_rcu(net: *mut c_void, ifindex: c_int) -> *mut c_void;
    fn skb_orphan(skb: *mut sk_buff);
    fn skb_warn_if_lro(skb: *mut sk_buff) -> bool;
    fn skb_forward_csum(skb: *mut sk_buff);
    fn dev_queue_xmit(skb: *mut sk_buff) -> c_int;
    fn skb_dst_drop(skb: *mut sk_buff);
    fn skb_dst_set(skb: *mut sk_buff, dst: *mut dst_entry);
    fn dst_release(dst: *mut dst_entry);
    fn dst_hold(dst: *mut dst_entry);
    fn eth_proto_is_802_3(proto: u16) -> bool;
    fn netif_carrier_ok(dev: *mut c_void) -> bool;
    fn skb_set_transport_header(skb: *mut sk_buff, offset: size_t);
    fn skb_postpull_rcsum(skb: *mut sk_buff, data: *mut u8, len: size_t);
}

// Constants
const IPPROTO_ROUTING: c_int = 43;
const IP6_FH_F_SKIP_RH: c_int = 1;
const RT6_LOOKUP_F_HAS_SADDR: c_int = 1;
const FLOWI_FLAG_KNOWN_NH: c_int = 1;
const IFF_UP: c_int = 1 << 1;
const IFF_LOOPBACK: c_int = 1 << 1;
const IPPROTO_ETHERNET: c_int = 0x0608;
const ETH_HLEN: size_t = 14;
const ARPHRD_ETHER: c_int = 1;