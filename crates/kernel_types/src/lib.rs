// Linux kernel type definitions for Rust FFI
// Target: Linux kernel 5.10 LTS networking stack
// Manually curated based on kernel headers

#![no_std]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

// Re-export core FFI types
pub use core::ffi::{c_int, c_uint, c_char, c_uchar, c_short, c_ushort, c_long, c_ulong, c_void};

// Standard types
pub type size_t = usize;
pub type c_size_t = usize;
pub type socklen_t = u32;

// Network byte order types
pub type __be16 = u16;
pub type __be32 = u32;
pub type __be64 = u64;
pub type __u8 = u8;
pub type __u16 = u16;
pub type __u32 = u32;
pub type __u64 = u64;
pub type __s8 = i8;
pub type __s16 = i16;
pub type __s32 = i32;
pub type __s64 = i64;

// ============================================================================
// Network Address Structures
// ============================================================================

/// IPv4 address (32-bit)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct in_addr {
    pub s_addr: __be32,
}

/// IPv6 address (128-bit)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct in6_addr {
    pub in6_u: in6_addr_union,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union in6_addr_union {
    pub u6_addr8: [__u8; 16],
    pub u6_addr16: [__be16; 8],
    pub u6_addr32: [__be32; 4],
}

/// Netfilter address (union of IPv4 and IPv6)
#[repr(C)]
#[derive(Copy, Clone)]
pub union nf_inet_addr {
    pub all: [__u32; 4],
    pub ip: __be32,
    pub ip6: [__be32; 4],
    pub in_addr: in_addr,
    pub in6: in6_addr,
}

// ============================================================================
// Network Protocol Headers
// ============================================================================

/// Ethernet header
#[repr(C)]
pub struct ethhdr {
    pub h_dest: [c_uchar; 6],
    pub h_source: [c_uchar; 6],
    pub h_proto: __be16,
}

/// IPv4 header
#[repr(C)]
pub struct iphdr {
    pub ihl: __u8,
    pub version: __u8,
    pub tos: __u8,
    pub tot_len: __be16,
    pub id: __be16,
    pub frag_off: __be16,
    pub ttl: __u8,
    pub protocol: __u8,
    pub check: __be16,
    pub saddr: __be32,
    pub daddr: __be32,
}

/// IPv6 header
#[repr(C)]
pub struct ipv6hdr {
    pub priority: __u8,
    pub version: __u8,
    pub flow_lbl: [__u8; 3],
    pub payload_len: __be16,
    pub nexthdr: __u8,
    pub hop_limit: __u8,
    pub saddr: in6_addr,
    pub daddr: in6_addr,
}

/// UDP header
#[repr(C)]
pub struct udphdr {
    pub source: __be16,
    pub dest: __be16,
    pub len: __be16,
    pub check: __be16,
}

/// ESP header
#[repr(C)]
pub struct ip_esp_hdr {
    pub spi: __be32,
    pub seq_no: __be32,
}

// ============================================================================
// Socket Structures
// ============================================================================

/// Internet socket (base)
#[repr(C)]
pub struct inet_sock {
    pub sk: *mut c_void, // struct sock *
    pub pinet6: *mut c_void, // struct ipv6_pinfo *
    pub inet_saddr: __be32,
    pub uc_ttl: __s16,
    pub cmsg_flags: __u16,
    pub inet_sport: __be16,
    pub inet_id: __u16,
    pub tos: __u8,
    pub min_ttl: __u8,
    pub mc_ttl: __u8,
    pub pmtudisc: __u8,
    pub recverr: __u8,
    pub freebind: __u8,
    pub hdrincl: __u8,
    pub mc_loop: __u8,
    pub transparent: __u8,
    pub mc_all: __u8,
    pub nodefrag: __u8,
    pub bind_address_no_port: __u8,
    pub defer_connect: __u8,
    pub rcv_tos: __u8,
    pub convert_csum: __u8,
    pub uc_index: c_int,
    pub mc_index: c_int,
    pub mc_addr: __be32,
}

/// IPv6 socket info
#[repr(C)]
pub struct ipv6_pinfo {
    pub saddr: in6_addr,
    pub daddr: in6_addr,
    pub flow_label: __be32,
    pub frag_size: __u32,
    pub hop_limit: __s16,
    pub mcast_hops: __s16,
    pub mcast_oif: c_int,
    pub rxopt: ip6cb,
}

/// UDP socket
#[repr(C)]
pub struct udp_sock {
    pub inet: inet_sock,
    pub pending: c_int,
    pub corkflag: c_uint,
    pub encap_type: __u8,
    pub encap_enabled: __u8,
    pub gro_enabled: __u8,
    pub pcflag: __u16,
}

/// Raw IPv6 socket
#[repr(C)]
pub struct raw6_sock {
    pub inet: inet_sock,
    pub checksum: __u32,
    pub offset: __u32,
    pub ip6mr: *mut c_void,
}

// ============================================================================
// Flow and Routing Structures
// ============================================================================

/// Flow identifier (base type)
#[repr(C)]
pub struct flowi {
    pub oif: c_int,
    pub iif: c_int,
    pub mark: __u32,
    pub scope: __u8,
    pub proto: __u8,
    pub flags: __u8,
    pub secid: __u32,
    pub flowi_tos: __u8,
}

/// Destination entry (routing cache)
#[repr(C)]
pub struct dst_entry {
    pub dev: *mut c_void, // struct net_device *
    pub ops: *mut c_void, // struct dst_ops *
    pub _rcuhead: *mut c_void,
    pub _metrics: [c_int; 17],
    pub _mtu: c_ulong,
    pub flags: c_ushort,
    pub obsolete: c_short,
    pub header_len: c_ushort,
    pub trailer_len: c_ushort,
}

/// IPv6 routing table entry
#[repr(C)]
pub struct rt6_info {
    pub dst: dst_entry,
    pub rt6_next: *mut rt6_info,
    pub rt6i_idev: *mut c_void, // struct inet6_dev *
    pub rt6i_flags: c_uint,
}

/// Routing table link operations
#[repr(C)]
pub struct rtnl_link_ops {
    pub list: *mut c_void,
    pub kind: *const c_char,
    pub maxtype: c_uint,
    pub policy: *const c_void,
}

/// FIB rule
#[repr(C)]
pub struct fib_rule {
    pub list: *mut c_void,
    pub table: __u32,
    pub flags: __u32,
    pub action: __u8,
}

// ============================================================================
// Packet Buffer Structures
// ============================================================================

/// Socket buffer (packet buffer)
#[repr(C)]
pub struct skbuff {
    pub next: *mut skbuff,
    pub prev: *mut skbuff,
    pub tstamp: __u64,
    pub dev: *mut c_void, // struct net_device *
    pub len: c_uint,
    pub data_len: c_uint,
    pub mac_len: __u16,
    pub hdr_len: __u16,
    pub csum: __u32,
    pub priority: __u32,
    pub protocol: __be16,
}

/// IPv6 control block (in skbuff->cb)
#[repr(C)]
pub struct ip6cb {
    pub nhoff: __u16,
    pub flags: __u16,
    pub dsfield: __u8,
    pub tclass: __u8,
    pub frag_max_size: __u16,
}

/// IPv6 fragmentation state
#[repr(C)]
pub struct ip6_frag_state {
    pub prevhdr: *mut u8,
    pub nexthdr: __u8,
    pub hlen: c_uint,
    pub mtu: c_uint,
    pub left: c_uint,
    pub offset: c_int,
}

/// IPv6 fraglist iterator
#[repr(C)]
pub struct ip6_fraglist_iter {
    pub frag: *mut skbuff,
    pub offset: c_int,
    pub hlen: c_uint,
}

// ============================================================================
// Netfilter Connection Tracking
// ============================================================================

/// Netfilter connection tracking zone
#[repr(C)]
pub struct nf_conntrack_zone {
    pub id: __u16,
    pub flags: __u8,
    pub dir: __u8,
}

/// Netfilter connection tracking helper
#[repr(C)]
pub struct nf_conntrack_helper {
    pub list: *mut c_void,
    pub name: [c_char; 16],
    pub module: *mut c_void,
    pub max_expected: c_uint,
    pub timeout: c_uint,
    pub flags: c_uint,
}

/// Netfilter connection
#[repr(C)]
pub struct nf_conn {
    pub ct_general: *mut c_void,
    pub tuplehash: [*mut c_void; 2],
    pub timeout: c_ulong,
    pub status: c_ulong,
}

// ============================================================================
// Misc Kernel Structures
// ============================================================================

/// Kernel timer
#[repr(C)]
pub struct timer_list {
    pub entry: *mut c_void,
    pub expires: c_ulong,
    pub function: *mut c_void,
    pub flags: c_ulong,
}

/// Hash list node (nulls variant)
#[repr(C)]
pub struct hlist_nulls_node {
    pub next: *mut hlist_nulls_node,
    pub pprev: *mut *mut hlist_nulls_node,
}

/// XFRM (IPsec) mode skb callback
#[repr(C)]
pub struct xfrm_mode_skb_cb {
    pub ihl: __u8,
    pub id: __u8,
    pub frag_off: __be16,
    pub tos: __u8,
    pub ttl: __u8,
}

/// U64 statistics synchronization
#[repr(C)]
pub struct u64_stats_sync {
    pub seq: c_uint,
}
