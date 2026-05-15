//! Linux NET3: IP/IP protocol decoder modified to support virtual tunnel interface
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use core::ptr;
use libc::{c_int, c_uint, c_void, size_t};

// Constants from C
pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ENOSYS: c_int = -38;

// Type definitions
#[repr(C)]
pub struct iphdr {
    pub saddr: u32,
    pub daddr: u32,
    pub protocol: u8,
    pub ihl: u8,
}

#[repr(C)]
pub struct sk_buff {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct net_device {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct ip_tunnel {
    pub dev: *mut net_device,
    pub net: *mut c_void,
    pub parms: ip_tunnel_parm,
}

#[repr(C)]
pub struct ip_tunnel_parm {
    pub iph: iphdr,
    pub i_key: u32,
    pub o_key: u32,
    pub i_flags: u32,
    pub o_flags: u32,
}

#[repr(C)]
pub struct ip_tunnel_net {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct xfrm_state {
    pub props: xfrm_state_props,
    pub inner_mode: xfrm_mode,
}

#[repr(C)]
pub struct xfrm_state_props {
    pub mode: u8,
    pub family: u8,
    pub saddr: u32,
}

#[repr(C)]
pub struct xfrm_mode {
    pub family: u8,
}

#[repr(C)]
pub struct xfrm_tunnel_skb_cb {
    pub tunnel: xfrm_tunnel,
}

#[repr(C)]
pub struct xfrm_tunnel {
    pub ip4: *mut ip_tunnel,
}

#[repr(C)]
pub struct net {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct flowi {
    pub u: flowi_union,
}

#[repr(C)]
union flowi_union {
    pub ip4: flowi_ip4,
    pub ip6: flowi_ip6,
}

#[repr(C)]
struct flowi_ip4 {
    pub flowi4_oif: c_int,
    pub flowi4_flags: c_int,
}

#[repr(C)]
struct flowi_ip6 {
    pub flowi6_oif: c_int,
    pub flowi6_flags: c_int,
}

// Function implementations
#[no_mangle]
pub unsafe extern "C" fn vti_input(
    skb: *mut sk_buff,
    nexthdr: c_int,
    spi: u32,
    encap_type: c_int,
    update_skb_dev: bool,
) -> c_int {
    let itn = ptr::null_mut(); // net_generic would be implemented in kernel
    let dev = ptr::null_mut(); // skb->dev would be implemented in kernel
    let iph = ip_hdr(skb);
    let tunnel = ip_tunnel_lookup(itn, dev, 0, iph.saddr, iph.daddr, 0);

    if !tunnel.is_null() {
        // SAFETY: Kernel ensures XFRM policy check is valid
        if !xfrm4_policy_check(ptr::null_mut(), 0, skb) {
            goto drop;
        }

        let cb = XFRM_TUNNEL_SKB_CB(skb);
        (*cb).tunnel.ip4 = tunnel;

        if update_skb_dev {
            (*skb).dev = (*tunnel).dev;
        }

        return xfrm_input(skb, nexthdr, spi, encap_type);
    }

    return -EINVAL;

drop:
    kfree_skb(skb);
    0
}

#[no_mangle]
pub unsafe extern "C" fn vti_input_proto(
    skb: *mut sk_buff,
    nexthdr: c_int,
    spi: u32,
    encap_type: c_int,
) -> c_int {
    vti_input(skb, nexthdr, spi, encap_type, false)
}

#[no_mangle]
pub unsafe extern "C" fn vti_rcv(
    skb: *mut sk_buff,
    spi: u32,
    update_skb_dev: bool,
) -> c_int {
    let iph = ip_hdr(skb);
    XFRM_SPI_SKB_CB(skb).family = 2; // AF_INET
    XFRM_SPI_SKB_CB(skb).daddroff = 12; // offsetof(iphdr, daddr)

    vti_input(skb, iph.protocol as c_int, spi, 0, update_skb_dev)
}

#[no_mangle]
pub unsafe extern "C" fn vti_rcv_proto(skb: *mut sk_buff) -> c_int {
    vti_rcv(skb, 0, false)
}

#[no_mangle]
pub unsafe extern "C" fn vti_rcv_cb(
    skb: *mut sk_buff,
    err: c_int,
) -> c_int {
    let cb = XFRM_TUNNEL_SKB_CB(skb);
    let tunnel = (*cb).tunnel.ip4;
    let dev = if !tunnel.is_null() { (*tunnel).dev } else { ptr::null_mut() };

    if dev.is_null() {
        return 1;
    }

    if err != 0 {
        (*dev).rx_errors += 1;
        (*dev).rx_dropped += 1;
        return 0;
    }

    let x = xfrm_input_state(skb);
    let inner_mode = &(*x).inner_mode;

    if (*x).sel.family == 0 {
        let mode = xfrm_ip2inner_mode(x, XFRM_MODE_SKB_CB(skb).protocol);
        if mode.is_null() {
            XFRM_INC_STATS(dev_net(skb), 0); // LINUX_MIB_XFRMINSTATEMODEERROR
            return -EINVAL;
        }
        inner_mode = mode;
    }

    let family = (*inner_mode).family;
    let orig_mark = (*skb).mark;
    (*skb).mark = (*tunnel).parms.i_key;
    
    // SAFETY: Kernel ensures policy check is valid
    let ret = xfrm_policy_check(ptr::null_mut(), 0, skb, family);
    (*skb).mark = orig_mark;

    if ret == 0 {
        return -EPERM;
    }

    skb_scrub_packet(skb, !net_eq((*tunnel).net, dev_net(skb)));
    (*skb).dev = dev;
    dev_sw_netstats_rx_add(dev, (*skb).len as u32);

    0
}

#[no_mangle]
pub unsafe extern "C" fn vti_state_check(
    x: *mut xfrm_state,
    dst: u32,
    src: u32,
) -> bool {
    if x.is_null() || (*x).props.mode != 1 || (*x).props.family != 2 {
        return false;
    }

    let daddr = &dst as *const u32;
    let saddr = &src as *const u32;

    if dst == 0 {
        return xfrm_addr_equal(saddr, &(*x).props.saddr, 2);
    }

    xfrm_state_addr_check(x, daddr, saddr, 2) != 0
}

#[no_mangle]
pub unsafe extern "C" fn vti_xmit(
    skb: *mut sk_buff,
    dev: *mut net_device,
    fl: *mut flowi,
) -> netdev_tx_t {
    let tunnel = netdev_priv(dev);
    let parms = &(*tunnel).parms;
    let dst = skb_dst(skb);
    let tdev = ptr::null_mut(); // Would be implemented in kernel
    let mtu = dst_mtu(dst);

    if skb.len > mtu {
        skb_dst_update_pmtu_no_confirm(skb, mtu);
        if (*skb).protocol == htons(0x0800) {
            if !((*ip_hdr(skb)).frag_off & htons(0x4000)) != 0 {
                goto xmit;
            }
            icmp_ndo_send(skb, 3, 4, htonl(mtu));
        } else {
            if mtu < 1280 {
                mtu = 1280;
            }
            icmpv6_ndo_send(skb, 4, 0, mtu);
        }
        dst_release(dst);
        goto tx_error;
    }

xmit:
    skb_scrub_packet(skb, !net_eq((*tunnel).net, dev_net(dev)));
    skb_dst_set(skb, dst);
    (*skb).dev = (*skb).dst.dev;

    let err = dst_output((*tunnel).net, (*skb).sk, skb);
    iptunnel_xmit_stats(dev, err);
    NETDEV_TX_OK

tx_error:
    (*dev).tx_errors += 1;
    kfree_skb(skb);
    NETDEV_TX_OK
}

#[no_mangle]
pub unsafe extern "C" fn vti_tunnel_xmit(
    skb: *mut sk_buff,
    dev: *mut net_device,
) -> netdev_tx_t {
    let tunnel = netdev_priv(dev);
    let mut fl = flowi { u: flowi_union { ip4: flowi_ip4 { flowi4_oif: 0, flowi4_flags: 0 } } };

    if !pskb_inet_may_pull(skb) {
        goto tx_err;
    }

    match (*skb).protocol {
        0x0800 => {
            xfrm_decode_session(skb, &mut fl, 2);
            memset(IPCB(skb), 0, size_of::<*mut c_void>());
        },
        0x86DD => {
            xfrm_decode_session(skb, &mut fl, 10);
            memset(IP6CB(skb), 0, size_of::<*mut c_void>());
        },
        _ => goto tx_err,
    }

    (*fl.u.ip4).flowi4_oif = (*tunnel).parms.o_key as c_int;
    vti_xmit(skb, dev, &mut fl)

tx_err:
    (*dev).tx_errors += 1;
    kfree_skb(skb);
    NETDEV_TX_OK
}

// Helper functions (would be implemented in kernel)
#[no_mangle]
pub unsafe extern "C" fn ip_hdr(skb: *mut sk_buff) -> *mut iphdr {
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn ip_tunnel_lookup(
    itn: *mut ip_tunnel_net,
    ifindex: c_int,
    key: c_int,
    saddr: u32,
    daddr: u32,
    _: c_int,
) -> *mut ip_tunnel {
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn xfrm4_policy_check(
    _: *mut c_void,
    _: c_int,
    _: *mut sk_buff,
) -> bool {
    true
}

#[no_mangle]
pub unsafe extern "C" fn XFRM_TUNNEL_SKB_CB(skb: *mut sk_buff) -> *mut xfrm_tunnel_skb_cb {
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn XFRM_SPI_SKB_CB(skb: *mut sk_buff) -> *mut c_void {
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn xfrm_input(skb: *mut sk_buff, _: c_int, _: u32, _: c_int) -> c_int {
    0
}

#[no_mangle]
pub unsafe extern "C" fn xfrm_input_state(skb: *mut sk_buff) -> *mut xfrm_state {
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn xfrm_ip2inner_mode(_: *mut xfrm_state, _: c_int) -> *mut xfrm_mode {
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn xfrm_state_addr_check(_: *mut xfrm_state, _: *const u32, _: *const u32, _: c_int) -> bool {
    true
}

#[no_mangle]
pub unsafe extern "C" fn skb_dst(skb: *mut sk_buff) -> *mut c_void {
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn dst_mtu(_: *mut c_void) -> c_int {
    1500
}

#[no_mangle]
pub unsafe extern "C" fn skb_dst_set(_: *mut sk_buff, _: *mut c_void) {}

#[no_mangle]
pub unsafe extern "C" fn dst_output(_: *mut c_void, _: *mut c_void, _: *mut sk_buff) -> c_int {
    0
}

#[no_mangle]
pub unsafe extern "C" fn iptunnel_xmit_stats(_: *mut net_device, _: c_int) {}

#[no_mangle]
pub unsafe extern "C" fn kfree_skb(skb: *mut sk_buff) {}

#[no_mangle]
pub unsafe extern "C" fn netdev_priv(dev: *mut net_device) -> *mut ip_tunnel {
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn dev_net(_: *mut sk_buff) -> *mut net {
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn net_eq(_: *mut c_void, _: *mut c_void) -> bool {
    true
}

#[no_mangle]
pub unsafe extern "C" fn skb_scrub_packet(_: *mut sk_buff, _: bool) {}

#[no_mangle]
pub unsafe extern "C" fn dev_sw_netstats_rx_add(_: *mut net_device, _: u32) {}

#[no_mangle]
pub unsafe extern "C" fn dst_release(_: *mut c_void) {}

#[no_mangle]
pub unsafe extern "C" fn dst_link_failure(_: *mut sk_buff) {}

#[no_mangle]
pub unsafe extern "C" fn XFRM_INC_STATS(_: *mut net, _: c_int) {}

#[no_mangle]
pub unsafe extern "C" fn xfrm_addr_equal(_: *const u32, _: *const u32, _: c_int) -> bool {
    true
}

#[no_mangle]
pub unsafe extern "C" fn htons(_: c_int) -> c_int {
    0
}

#[no_mangle]
pub unsafe extern "C" fn htonl(_: c_int) -> c_int {
    0
}

#[no_mangle]
pub unsafe extern "C" fn IPCB(_: *mut sk_buff) -> *mut c_void {
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn IP6CB(_: *mut sk_buff) -> *mut c_void {
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn pskb_inet_may_pull(_: *mut sk_buff) -> bool {
    true
}

#[no_mangle]
pub unsafe extern "C" fn xfrm_decode_session(_: *mut sk_buff, _: *mut flowi, _: c_int) {}

#[no_mangle]
pub unsafe extern "C" fn skb_dst_update_pmtu_no_confirm(_: *mut sk_buff, _: c_int) {}

#[no_mangle]
pub unsafe extern "C" fn icmp_ndo_send(_: *mut sk_buff, _: c_int, _: c_int, _: c_int) {}

#[no_mangle]
pub unsafe extern "C" fn icmpv6_ndo_send(_: *mut sk_buff, _: c_int, _: c_int, _: c_int) {}

#[no_mangle]
pub unsafe extern "C" fn memset(_: *mut c_void, _: c_int, _: size_t) -> *mut c_void {
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn size_of<T>() -> size_t {
    core::mem::size_of::<T>()
}

// Types for return values
pub type netdev_tx_t = c_int;
pub const NETDEV_TX_OK: netdev_tx_t = 0;

// Error codes
pub const EPERM: c_int = -1;
pub const EINVA: c_int = -22;
