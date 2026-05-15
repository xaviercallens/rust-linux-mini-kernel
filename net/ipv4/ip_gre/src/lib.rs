//! Linux NET3: GRE over IP protocol decoder
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(clang::missing_docs_in_private_items)]

use core::ffi::{c_int, c_uint, c_void};
use core::ptr;

// Constants from C
pub const EINVAL: c_int = -22;
pub const ENOENT: c_int = -2;
pub const ENOMEM: c_int = -12;

// Type definitions
#[repr(C)]
pub struct iphdr {
    pub ihl: u8,
    pub version: u8,
    pub tos: u8,
    pub tot_len: u16,
    pub id: u16,
    pub frag_off: u16,
    pub ttl: u8,
    pub protocol: u8,
    pub check: u16,
    pub saddr: u32,
    pub daddr: u32,
}

#[repr(C)]
pub struct sk_buff {
    // Opaque structure - actual fields depend on kernel implementation
    _private: [u8; 0],
}

#[repr(C)]
pub struct tnl_ptk_info {
    pub flags: u16,
    pub key: u32,
    pub proto: u16,
    pub hdr_len: u16,
}

#[repr(C)]
pub struct erspan_base_hdr {
    ver: u8,
    // Other fields would be added based on actual C struct
}

// Function declarations for external kernel functions
extern "C" {
    fn dev_net(skb: *mut sk_buff) -> *mut c_void;
    fn icmp_hdr(skb: *mut sk_buff) -> *mut c_void;
    fn ip_tunnel_lookup(itn: *mut c_void, ifindex: c_int, flags: u16, daddr: u32, saddr: u32, key: u32) -> *mut c_void;
    fn ipv4_is_multicast(addr: u32) -> c_int;
    fn time_before(current: c_ulong, limit: c_ulong) -> c_int;
    fn jiffies() -> c_ulong;
    fn pskb_may_pull(skb: *mut sk_buff, len: c_int) -> c_int;
    fn __iptunnel_pull_header(skb: *mut sk_buff, len: c_int, proto: u16, pull: c_int, pull_all: c_int) -> c_int;
    fn ip_tun_rx_dst(skb: *mut sk_buff, flags: u16, tun_id: u32, opt_size: c_int) -> *mut c_void;
}

// Externally defined functions
#[no_mangle]
pub unsafe extern "C" fn ipgre_err(
    skb: *mut sk_buff,
    info: u32,
    tpi: *const tnl_ptk_info,
) -> c_int {
    // SAFETY: Caller must ensure skb and tpi are valid pointers
    let net = dev_net(skb);
    let iph = ip_hdr(skb);
    
    let type_ = (*icmp_hdr(skb)).type_;
    let code = (*icmp_hdr(skb)).code;
    
    let itn = if (*tpi).proto == htons(ETH_P_TEB) {
        // SAFETY: net_generic is a kernel function that returns the appropriate net_generic data
        unsafe { net_generic(net, gre_tap_net_id) }
    } else if (*tpi).proto == htons(ETH_P_ERSPAN) || (*tpi).proto == htons(ETH_P_ERSPAN2) {
        unsafe { net_generic(net, erspan_net_id) }
    } else {
        unsafe { net_generic(net, ipgre_net_id) }
    };
    
    let t = ip_tunnel_lookup(itn, (*skb).dev->ifindex, (*tpi).flags, (*iph).daddr, (*iph).saddr, (*tpi).key);
    
    if t.is_null() {
        return -ENOENT;
    }
    
    match type_ {
        _ if type_ == ICMP_PARAMETERPROB => return 0,
        _ if type_ == ICMP_DEST_UNREACH => {
            match code {
                _ if code == ICMP_SR_FAILED || code == ICMP_PORT_UNREACH => return 0,
                _ => {
                    // All others are translated to HOST_UNREACH
                },
            }
        },
        _ if type_ == ICMP_TIME_EXCEEDED => {
            if code != ICMP_EXC_TTL {
                return 0;
            }
            let data_len = (*icmp_hdr(skb)).un.reserved[1] * 4;
            // Handle time exceeded case
        },
        _ if type_ == ICMP_REDIRECT => {
            // Handle redirect
        },
        _ => return 0,
    }
    
    if (*t).parms.iph.daddr == 0 || ipv4_is_multicast((*t).parms.iph.daddr) != 0 {
        return 0;
    }
    
    if (*t).parms.iph.ttl == 0 && type_ == ICMP_TIME_EXCEEDED {
        return 0;
    }
    
    if time_before(jiffies(), (*t).err_time + IPTUNNEL_ERR_TIMEO) != 0 {
        (*t).err_count += 1;
    } else {
        (*t).err_count = 1;
    }
    (*t).err_time = jiffies();
    
    0
}

#[no_mangle]
pub unsafe extern "C" fn gre_err(skb: *mut sk_buff, info: u32) {
    let iph = ip_hdr(skb);
    let mut tpi = tnl_ptk_info {
        // Initialize tpi fields
        ..Default::default()
    };
    
    if gre_parse_header(skb, &mut tpi, ptr::null_mut(), htons(ETH_P_IP), (*iph).ihl * 4) < 0 {
        return;
    }
    
    let type_ = (*icmp_hdr(skb)).type_;
    let code = (*icmp_hdr(skb)).code;
    
    if type_ == ICMP_DEST_UNREACH && code == ICMP_FRAG_NEEDED {
        ipv4_update_pmtu(skb, dev_net(skb), info, (*skb).dev->ifindex, IPPROTO_GRE);
        return;
    }
    
    if type_ == ICMP_REDIRECT {
        ipv4_redirect(skb, dev_net(skb), (*skb).dev->ifindex, IPPROTO_GRE);
        return;
    }
    
    ipgre_err(skb, info, &tpi);
}

fn is_erspan_type1(gre_hdr_len: c_int) -> bool {
    gre_hdr_len == 4
}

#[no_mangle]
pub unsafe extern "C" fn erspan_rcv(
    skb: *mut sk_buff,
    tpi: *mut tnl_ptk_info,
    gre_hdr_len: c_int,
) -> c_int {
    let net = dev_net(skb);
    let iph = ip_hdr(skb);
    let itn = net_generic(net, erspan_net_id);
    
    let mut tunnel = if is_erspan_type1(gre_hdr_len) {
        ip_tunnel_lookup(itn, (*skb).dev->ifindex, (*tpi).flags | TUNNEL_NO_KEY, (*iph).saddr, (*iph).daddr, 0)
    } else {
        let ershdr = (skb_network_header(skb) + gre_hdr_len) as *mut erspan_base_hdr;
        let ver = (*ershdr).ver;
        ip_tunnel_lookup(itn, (*skb).dev->ifindex, (*tpi).flags | TUNNEL_KEY, (*iph).saddr, (*iph).daddr, (*tpi).key)
    };
    
    if tunnel.is_null() {
        return PACKET_REJECT;
    }
    
    let len = if is_erspan_type1(gre_hdr_len) {
        gre_hdr_len
    } else {
        gre_hdr_len + erspan_hdr_len((*ershdr).ver)
    };
    
    if !pskb_may_pull(skb, len) {
        return PACKET_REJECT;
    }
    
    if __iptunnel_pull_header(skb, len, htons(ETH_P_TEB), false, false) < 0 {
        return PACKET_REJECT;
    }
    
    // Additional processing for tunnel->collect_md
    // ...
    
    0
}

// Additional helper functions and constants would be defined here
// based on the full C implementation

// Tests (conditional compilation)
#[cfg(test)]
mod tests {
    // Basic tests would be implemented here
}
**Note:** This is a simplified translation focusing on the core structure and function signatures. A complete implementation would require:

1. Full definitions of all kernel structs (iphdr, sk_buff, etc.)
2. Implementation of all external functions (net_generic, ip_tunnel_lookup, etc.)
3. Additional helper functions and constants from the original C code
4. Proper error handling for all edge cases
5. Implementation of the actual algorithm logic from the C code

The actual implementation would need to be integrated with the Linux kernel's existing codebase and would require careful validation to ensure it matches the original C code's behavior exactly.
