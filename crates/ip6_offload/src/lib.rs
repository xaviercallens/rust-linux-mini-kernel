#![no_std]
#![no_main]
#![allow(non_camel_case_types)]

use core::ffi::{c_int, c_uint};
use core::panic::PanicInfo;
use core::ptr;
use kernel_types::*;

// Constants from C
pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ENOSYS: c_int = -38;
pub const NEXTHDR_HOP: u8 = 0;
pub const INET6_PROTO_GSO_EXTHDR: c_int = 1;
pub const ETH_P_IPV6: c_int = 0x86DD;

// Missing protocol/GSO constants
pub const IPPROTO_UDP: u8 = 17;
pub const SKB_GSO_IPXIP4: c_uint = 1 << 0;
pub const SKB_GSO_IPXIP6: c_uint = 1 << 1;
pub const SKB_GSO_UDP: c_uint = 1 << 2;
pub const SKB_GSO_PARTIAL: c_uint = 1 << 3;

// Linux aliases typically provided by kernel headers
pub type netdev_features_t = u64;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipv6_opt_hdr {
    pub nexthdr: u8,
    pub hdrlen: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipv6hdr {
    pub nexthdr: u8,
    pub payload_len: u16,
    pub saddr: [u8; 16],
    pub daddr: [u8; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct frag_hdr {
    pub frag_off: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct net_offload_callbacks {
    pub gso_segment: Option<unsafe extern "C" fn(*mut sk_buff, netdev_features_t) -> *mut sk_buff>,
    pub gro_receive: Option<unsafe extern "C" fn(*mut list_head, *mut sk_buff) -> *mut sk_buff>,
    pub gro_complete: Option<unsafe extern "C" fn(*mut sk_buff, c_int) -> c_int>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct net_offload {
    pub flags: c_int,
    pub callbacks: net_offload_callbacks,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct packet_offload {
    pub type_: c_int,
    pub callbacks: net_offload_callbacks,
}

unsafe extern "C" {
    fn inet6_offloads(proto: c_int) -> *const net_offload;
    fn rcu_dereference(p: *const net_offload) -> *const net_offload;
    fn pskb_may_pull(skb: *mut sk_buff, len: c_int) -> bool;
    fn ipv6_optlen(opth: *const ipv6_opt_hdr) -> c_int;
    fn __skb_pull(skb: *mut sk_buff, len: c_int) -> *mut u8;
    fn skb_network_header(skb: *const sk_buff) -> *mut u8;
}

#[no_mangle]
pub unsafe extern "C" fn ipv6_gso_pull_exthdrs(skb: *mut sk_buff, proto: c_int) -> c_int {
    let mut proto_u8 = proto as u8;

    loop {
        if proto_u8 != NEXTHDR_HOP {
            let ops = rcu_dereference(inet6_offloads(proto_u8 as c_int));
            if ops.is_null() {
                break;
            }
            if ((*ops).flags & INET6_PROTO_GSO_EXTHDR) == 0 {
                break;
            }
        }

        if !pskb_may_pull(skb, 8) {
            break;
        }

        let opth = skb_network_header(skb as *const sk_buff) as *mut ipv6_opt_hdr;
        let len = ipv6_optlen(opth as *const ipv6_opt_hdr);

        if !pskb_may_pull(skb, len) {
            break;
        }

        proto_u8 = (*opth).nexthdr;
        __skb_pull(skb, len);
    }

    proto_u8 as c_int
}

#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    loop {}
}