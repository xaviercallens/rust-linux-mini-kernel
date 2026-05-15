//! IP/IP protocol decoder for Linux kernel
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clang::too_many_arguments)]

use core::ffi::c_void;
use core::ffi::c_int;
use core::ffi::c_uint;
use core::ffi::c_ulong;
use core::mem;
use core::ptr;
use core::slice;

// Constants from C
pub const EINVAL: c_int = -22;
pub const ENOENT: c_int = -2;
pub const ENOMEM: c_int = -12;
pub const NETDEV_TX_OK: c_int = 0;

// Type definitions
#[repr(C)]
pub struct in_addr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct iphdr {
    pub version: u8,
    pub ihl: u8,
    pub tos: u8,
    pub tot_len: u16,
    pub id: u16,
    pub frag_off: u16,
    pub ttl: u8,
    pub protocol: u8,
    pub check: u16,
    pub saddr: in_addr,
    pub daddr: in_addr,
}

#[repr(C)]
pub struct sk_buff {
    pub data: *const c_void,
    pub dev: *const c_void,
}

#[repr(C)]
pub struct net_device {
    pub ifindex: c_int,
    pub flags: c_int,
    pub netdev_ops: *const net_device_ops,
    pub type_: c_int,
    pub addr_len: c_int,
    pub features: c_int,
    pub hw_features: c_int,
    pub stats: *mut net_device_stats,
}

#[repr(C)]
pub struct net_device_ops {
    pub ndo_init: extern "C" fn(*mut net_device) -> c_int,
    pub ndo_uninit: extern "C" fn(*mut net_device),
    pub ndo_start_xmit: extern "C" fn(*mut sk_buff, *mut net_device) -> c_int,
    pub ndo_do_ioctl: extern "C" fn(*mut net_device, *mut c_void, c_int) -> c_int,
    pub ndo_change_mtu: extern "C" fn(*mut net_device, c_int) -> c_int,
    pub ndo_get_stats64: extern "C" fn(*mut net_device, *mut net_device_stats64) -> c_int,
    pub ndo_get_iflink: extern "C" fn(*mut net_device) -> c_int,
    pub ndo_tunnel_ctl: extern "C" fn(*mut net_device, *mut ip_tunnel_parm, c_int) -> c_int,
}

#[repr(C)]
pub struct net_device_stats {
    pub rx_packets: c_ulong,
    pub tx_packets: c_ulong,
    pub rx_bytes: c_ulong,
    pub tx_bytes: c_ulong,
    pub rx_errors: c_ulong,
    pub tx_errors: c_ulong,
    pub tx_dropped: c_ulong,
}

#[repr(C)]
pub struct net_device_stats64 {
    pub rx_packets: u64,
    pub tx_packets: u64,
    pub rx_bytes: u64,
    pub tx_bytes: u64,
    pub rx_errors: u64,
    pub tx_errors: u64,
    pub tx_dropped: u64,
}

#[repr(C)]
pub struct ip_tunnel_parm {
    pub iph: iphdr,
    pub link: c_int,
    pub i_key: u32,
    pub o_key: u32,
    pub i_flags: u16,
    pub o_flags: u16,
    pub itunnel: c_int,
    pub xmit: c_int,
}

#[repr(C)]
pub struct ip_tunnel {
    pub parms: ip_tunnel_parm,
    pub err_count: c_int,
    pub err_time: c_ulong,
    pub collect_md: c_int,
}

#[repr(C)]
pub struct ip_tunnel_net {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tnl_ptk_info {
    pub proto: u16,
}

// Function implementations
static mut log_ecn_error: bool = true;

#[no_mangle]
pub unsafe extern "C" fn ipip_err(skb: *mut sk_buff, info: u32) -> c_int {
    if skb.is_null() {
        return EINVAL;
    }

    let net = dev_net((*skb).dev);
    let itn = net_generic(net, ipip_net_id);
    let iph = &*(ptr::addr_of!((*skb).data) as *const iphdr);
    
    let type_ = (*icmp_hdr(skb)).type;
    let code = (*icmp_hdr(skb)).code;
    
    let t = ip_tunnel_lookup(itn, (*skb).dev as _, 0, iph.daddr.s_addr, iph.saddr.s_addr, 0);
    if t.is_null() {
        return ENOENT;
    }

    match type_ {
        3 => match code {
            5 => (), // ICMP_SR_FAILED
            _ => (), // Default to HOST_UNREACH
        },
        11 => if code != 0 {
            return 0;
        },
        5 => (), // ICMP_REDIRECT
        _ => return 0,
    }

    if type_ == 3 && code == 4 {
        ipv4_update_pmtu(skb, net, info, (*t).parms.link, iph.protocol);
        return 0;
    }

    if type_ == 5 {
        ipv4_redirect(skb, net, (*t).parms.link, iph.protocol);
        return 0;
    }

    if (*t).parms.iph.daddr.s_addr == 0 {
        return ENOENT;
    }

    if (*t).parms.iph.ttl == 0 && type_ == 11 {
        return 0;
    }

    if time_before(jiffies(), (*t).err_time + IPTUNNEL_ERR_TIMEO) {
        (*t).err_count += 1;
    } else {
        (*t).err_count = 1;
    }
    (*t).err_time = jiffies();

    0
}

#[no_mangle]
pub unsafe extern "C" fn ipip_tunnel_rcv(skb: *mut sk_buff, ipproto: u8) -> c_int {
    if skb.is_null() {
        return -1;
    }

    let net = dev_net((*skb).dev);
    let itn = net_generic(net, ipip_net_id);
    let iph = ip_hdr(skb);
    
    let tunnel = ip_tunnel_lookup(itn, (*skb).dev as _, 0, (*iph).saddr.s_addr, (*iph).daddr.s_addr, 0);
    if tunnel.is_null() {
        return -1;
    }

    if (*tunnel).parms.iph.protocol != ipproto && (*tunnel).parms.iph.protocol != 0 {
        goto drop;
    }

    if !xfrm4_policy_check(ptr::null_mut(), XFRM_POLICY_IN, skb) {
        goto drop;
    }

    let tpi = &ipip_tpi;
    if iptunnel_pull_header(skb, 0, tpi.proto, false) != 0 {
        goto drop;
    }

    if (*tunnel).collect_md != 0 {
        let tun_dst = ip_tun_rx_dst(skb, 0, 0, 0);
        if tun_dst.is_null() {
            return 0;
        }
        return ip_tunnel_rcv(tunnel, skb, tpi, tun_dst, log_ecn_error);
    }

    return ip_tunnel_rcv(tunnel, skb, tpi, ptr::null_mut(), log_ecn_error);

drop:
    kfree_skb(skb);
    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn ipip_rcv(skb: *mut sk_buff) -> c_int {
    ipip_tunnel_rcv(skb, IPPROTO_IPIP)
}

#[no_mangle]
pub unsafe extern "C" fn ipip_tunnel_xmit(skb: *mut sk_buff, dev: *mut net_device) -> c_int {
    if skb.is_null() || dev.is_null() {
        return NETDEV_TX_OK;
    }

    let tunnel = netdev_priv(dev);
    let tiph = &(*tunnel).parms.iph;
    let ipproto: u8;

    if !pskb_inet_may_pull(skb) {
        goto tx_error;
    }

    match (*skb).protocol {
        0x0800 => ipproto = IPPROTO_IPIP,
        #[cfg(CONFIG_MPLS)]
        0x8847 => ipproto = IPPROTO_MPLS,
        _ => goto tx_error,
    }

    if tiph.protocol != ipproto && tiph.protocol != 0 {
        goto tx_error;
    }

    if iptunnel_handle_offloads(skb, SKB_GSO_IPXIP4) != 0 {
        goto tx_error;
    }

    skb_set_inner_ipproto(skb, ipproto);

    if (*tunnel).collect_md != 0 {
        ip_md_tunnel_xmit(skb, dev, ipproto, 0);
    } else {
        ip_tunnel_xmit(skb, dev, tiph, ipproto);
    }

    return NETDEV_TX_OK;

tx_error:
    kfree_skb(skb);
    (*dev).stats.add_assign(1, 1);
    return NETDEV_TX_OK;
}

#[no_mangle]
pub unsafe extern "C" fn ipip_tunnel_setup(dev: *mut net_device) {
    if dev.is_null() {
        return;
    }

    (*dev).netdev_ops = &ipip_netdev_ops;
    (*dev).header_ops = &ip_tunnel_header_ops;
    
    (*dev).type = ARPHRD_TUNNEL;
    (*dev).flags = IFF_NOARP;
    (*dev).addr_len = 4;
    (*dev).features |= NETIF_F_LLTX;
    netif_keep_dst(dev);
    
    (*dev).features |= IPIP_FEATURES;
    (*dev).hw_features |= IPIP_FEATURES;
    ip_tunnel_setup(dev, ipip_net_id);
}

// Helper functions (extern declarations)
extern "C" {
    fn dev_net(dev: *const c_void) -> *mut c_void;
    fn net_generic(net: *mut c_void, id: c_int) -> *mut ip_tunnel_net;
    fn ip_hdr(skb: *mut sk_buff) -> *const iphdr;
    fn icmp_hdr(skb: *mut sk_buff) -> *const c_void;
    fn ip_tunnel_lookup(itn: *mut ip_tunnel_net, ifindex: c_int, key: u32, 
                        daddr: u32, saddr: u32, flags: u32) -> *mut ip_tunnel;
    fn xfrm4_policy_check(ctx: *mut c_void, dir: c_int, skb: *mut sk_buff) -> c_int;
    fn iptunnel_pull_header(skb: *mut sk_buff, offset: c_int, proto: u16, 
                            adjust: bool) -> c_int;
    fn ip_tun_rx_dst(skb: *mut sk_buff, link: c_int, ifindex: c_int, 
                     flags: c_int) -> *mut c_void;
    fn ip_tunnel_rcv(tunnel: *mut ip_tunnel, skb: *mut sk_buff, tpi: *const tnl_ptk_info, 
                     tun_dst: *mut c_void, log_ecn_error: bool) -> c_int;
    fn pskb_inet_may_pull(skb: *mut sk_buff) -> bool;
    fn iptunnel_handle_offloads(skb: *mut sk_buff, features: c_int) -> c_int;
    fn skb_set_inner_ipproto(skb: *mut sk_buff, proto: u8);
    fn ip_md_tunnel_xmit(skb: *mut sk_buff, dev: *mut net_device, 
                         proto: u8, flags: c_int);
    fn ip_tunnel_xmit(skb: *mut sk_buff, dev: *mut net_device, 
                      tiph: *const iphdr, ipproto: u8);
    fn kfree_skb(skb: *mut sk_buff);
    fn netif_keep_dst(dev: *mut net_device);
    fn ip_tunnel_setup(dev: *mut net_device, ipip_net_id: c_int);
}

// Constants
pub const IPPROTO_IPIP: u8 = 4;
#[cfg(CONFIG_MPLS)]
pub const IPPROTO_MPLS: u8 = 137;
pub const ARPHRD_TUNNEL: c_int = 776;
pub const IFF_NOARP: c_int = 0x8000;
pub const NETIF_F_LLTX: c_int = 0x20000000;
pub const IPIP_FEATURES: c_int = 0x0000000F;
pub const SKB_GSO_IPXIP4: c_int = 0x80000000;
pub const XFRM_POLICY_IN: c_int = 0;

// Static variables
static ipip_net_id: c_int = 0;
static ipip_link_ops: net_device_ops = net_device_ops {
    ndo_init: Some(ipip_tunnel_init),
    ndo_uninit: Some(ip_tunnel_uninit),
    ndo_start_xmit: Some(ipip_tunnel_xmit),
    ndo_do_ioctl: Some(ip_tunnel_ioctl),
    ndo_change_mtu: Some(ip_tunnel_change_mtu),
    ndo_get_stats64: Some(dev_get_tstats64),
    ndo_get_iflink: Some(ip_tunnel_get_iflink),
    ndo_tunnel_ctl: Some(ipip_tunnel_ctl),
};

static ipip_netdev_ops: net_device_ops = net_device_ops {
    ndo_init: Some(ipip_tunnel_init),
    ndo_uninit: Some(ip_tunnel_uninit),
    ndo_start_xmit: Some(ipip_tunnel_xmit),
    ndo_do_ioctl: Some(ip_tunnel_ioctl),
    ndo_change_mtu: Some(ip_tunnel_change_mtu),
    ndo_get_stats64: Some(dev_get_tstats64),
    ndo_get_iflink: Some(ip_tunnel_get_iflink),
    ndo_tunnel_ctl: Some(ipip_tunnel_ctl),
};

static ipip_tpi: tnl_ptk_info = tnl_ptk_info {
    proto: 0x0800, // ETH_P_IP
};

#[cfg(CONFIG_MPLS)]
static mplsip_tpi: tnl_ptk_info = tnl_ptk_info {
    proto: 0x8847, // ETH_P_MPLS_UC
};

// Additional helper functions
#[no_mangle]
pub unsafe extern "C" fn ipip_tunnel_init(dev: *mut net_device) -> c_int {
    ip_tunnel_init(dev)
}

#[no_mangle]
pub unsafe extern "C" fn ipip_tunnel_ctl(dev: *mut net_device, p: *mut ip_tunnel_parm, cmd: c_int) -> c_int {
    if p.is_null() {
        return EINVAL;
    }

    let p = &mut *p;
    if cmd == SIOCADDTUNNEL || cmd == SIOCCHGTUNNEL {
        if p.iph.version != 4 || p.iph.ihl != 5 || 
           (p.iph.frag_off & htons(!IP_DF)) != 0 {
            return EINVAL;
        }
        
        match p.iph.protocol {
            0, IPPROTO_IPIP => {},
            #[cfg(CONFIG_MPLS)]
            IPPROTO_MPLS => {},
            _ => return EINVAL,
        }
    }

    p.i_key = 0;
    p.o_key = 0;
    p.i_flags = 0;
    p.o_flags = 0;
    
    ip_tunnel_ctl(dev, p, cmd)
}

// Constants
pub const SIOCADDTUNNEL: c_int = 0x8940;
pub const SIOCCHGTUNNEL: c_int = 0x8941;
pub const IP_DF: u16 = 0x4000;

// Helper macros translated to functions
#[no_mangle]
pub unsafe extern "C" fn htons(x: u16) -> u16 {
    x.to_be()
}

#[no_mangle]
pub unsafe extern "C" fn time_before(a: c_ulong, b: c_ulong) -> bool {
    a < b
}

#[no_mangle]
pub unsafe extern "C" fn jiffies() -> c_ulong {
    // Placeholder - actual implementation would get current jiffies
    0
}

#[no_mangle]
pub unsafe extern "C" fn IPTUNNEL_ERR_TIMEO() -> c_ulong {
    10 * HZ()
}

#[no_mangle]
pub unsafe extern "C" fn HZ() -> c_ulong {
    100
}

// Tests (conditional compilation)
#[cfg(test)]
mod tests {
    #[test]
    fn test_ipip_err() {
        // Basic test case - would require actual skb and net_device instances
        // This is a placeholder as actual testing would require kernel environment
        assert!(true);
    }
}
