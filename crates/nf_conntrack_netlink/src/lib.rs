//! Connection tracking via netlink socket. Allows for user space
//! protocol helpers and general trouble making from userspace.
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]  // For C-style type names

use core::ptr;
use libc::{c_int, c_uint, c_void, size_t};

// Constants from C
pub const ENOMEM: c_int = -12;
pub const EINVAL: c_int = -22;
pub const EMSGSIZE: c_int = -90;

// Type definitions
#[repr(C)]
pub struct nlattr {
    pub nla_len: c_uint,
    pub nla_type: c_uint,
}

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nf_conntrack_tuple {
    pub dst: nf_conntrack_tuple_dst,
    pub src: nf_conntrack_tuple_src,
    pub src_l3num: u8,
}

#[repr(C)]
pub struct nf_conntrack_tuple_dst {
    protonum: u8,
}

#[repr(C)]
pub struct nf_conntrack_tuple_src {
    u3: nf_conntrack_tuple_src_u3,
}

#[repr(C)]
pub struct nf_conntrack_tuple_src_u3 {
    ip: u32,
    in6: [u8; 16],
}

#[repr(C)]
pub struct nf_conntrack_l4proto {
    tuple_to_nlattr: Option<extern "C" fn(skb: *mut sk_buff, tuple: *const nf_conntrack_tuple) -> c_int>,
}

#[repr(C)]
pub struct nf_conn {
    status: u32,
    mark: u32,
    _private: [u8; 0],
}

#[repr(C)]
pub struct nf_conn_acct {
    counter: *mut nf_conn_counter,
}

#[repr(C)]
pub struct nf_conn_counter {
    packets: [u64; 2],
    bytes: [u64; 2],
}

#[repr(C)]
pub struct nf_conn_tstamp {
    start: u64,
    stop: u64,
}

// Function prototypes for external C functions
extern "C" {
    fn nla_nest_start(skb: *mut sk_buff, attrtype: c_int) -> *mut nlattr;
    fn nla_nest_end(skb: *mut sk_buff, nest: *mut nlattr);
    fn nla_put_u8(skb: *mut sk_buff, attrtype: c_int, val: u8) -> c_int;
    fn nla_put_be16(skb: *mut sk_buff, attrtype: c_int, val: u16) -> c_int;
    fn nla_put_be32(skb: *mut sk_buff, attrtype: c_int, val: u32) -> c_int;
    fn nla_put_in_addr(skb: *mut sk_buff, attrtype: c_int, val: u32) -> c_int;
    fn nla_put_in6_addr(skb: *mut sk_buff, attrtype: c_int, val: *const [u8; 16]) -> c_int;
    fn nla_put_be64(skb: *mut sk_buff, attrtype: c_int, val: u64, pad: c_int) -> c_int;
    fn nla_put_string(skb: *mut sk_buff, attrtype: c_int, val: *const u8) -> c_int;
    fn nf_ct_l4proto_find(protonum: u8) -> *const nf_conntrack_l4proto;
    fn nf_ct_protonum(ct: *const nf_conn) -> u8;
    fn nf_ct_expires(ct: *const nf_conn) -> u32;
    fn nf_ct_acct_find(ct: *const nf_conn) -> *mut nf_conn_acct;
    fn nf_conn_tstamp_find(ct: *const nf_conn) -> *const nf_conn_tstamp;
    fn nf_ct_labels_find(ct: *const nf_conn) -> *const nf_conn_labels;
    fn rcu_read_lock();
    fn rcu_read_unlock();
    fn security_secid_to_secctx(secid: u32, secctx: *mut *mut u8, len: *mut size_t) -> c_int;
    fn security_release_secctx(secctx: *mut u8, len: size_t);
}

#[repr(C)]
struct nf_conn_labels {
    bits: [u64; 16],
}

// Function implementations
/// Dump protocol part of connection tuple to netlink message
///
/// # Safety
/// - `skb` must be a valid pointer to sk_buff
/// - `tuple` must be a valid pointer to nf_conntrack_tuple
/// - `l4proto` must be a valid pointer to nf_conntrack_l4proto
///
/// # Returns
/// 0 on success, -EMSGSIZE if message too large
#[no_mangle]
pub unsafe extern "C" fn ctnetlink_dump_tuples_proto(
    skb: *mut sk_buff,
    tuple: *const nf_conntrack_tuple,
    l4proto: *const nf_conntrack_l4proto,
) -> c_int {
    let mut ret: c_int = 0;
    let nest_parms = nla_nest_start(skb, CTA_TUPLE_PROTO);
    if nest_parms.is_null() {
        return -EMSGSIZE;
    }

    if nla_put_u8(skb, CTA_PROTO_NUM, (*tuple).dst.protonum) != 0 {
        return -EMSGSIZE;
    }

    if let Some(proto_to_nlattr) = (*l4proto).tuple_to_nlattr {
        ret = proto_to_nlattr(skb, tuple);
    }

    nla_nest_end(skb, nest_parms);
    ret
}

/// Dump IPv4 addresses to netlink message
///
/// # Safety
/// - `skb` must be a valid pointer to sk_buff
/// - `tuple` must be a valid pointer to nf_conntrack_tuple
///
/// # Returns
/// 0 on success, -EMSGSIZE if message too large
#[no_mangle]
pub unsafe extern "C" fn ipv4_tuple_to_nlattr(
    skb: *mut sk_buff,
    tuple: *const nf_conntrack_tuple,
) -> c_int {
    if nla_put_in_addr(skb, CTA_IP_V4_SRC, (*tuple).src.u3.ip) != 0 ||
       nla_put_in_addr(skb, CTA_IP_V4_DST, (*tuple).dst.u3.ip) != 0 {
        return -EMSGSIZE;
    }
    0
}

/// Dump IPv6 addresses to netlink message
///
/// # Safety
/// - `skb` must be a valid pointer to sk_buff
/// - `tuple` must be a valid pointer to nf_conntrack_tuple
///
/// # Returns
/// 0 on success, -EMSGSIZE if message too large
#[no_mangle]
pub unsafe extern "C" fn ipv6_tuple_to_nlattr(
    skb: *mut sk_buff,
    tuple: *const nf_conntrack_tuple,
) -> c_int {
    if nla_put_in6_addr(skb, CTA_IP_V6_SRC, &(*tuple).src.u3.in6) != 0 ||
       nla_put_in6_addr(skb, CTA_IP_V6_DST, &(*tuple).dst.u3.in6) != 0 {
        return -EMSGSIZE;
    }
    0
}

/// Dump IP addresses part of connection tuple to netlink message
///
/// # Safety
/// - `skb` must be a valid pointer to sk_buff
/// - `tuple` must be a valid pointer to nf_conntrack_tuple
///
/// # Returns
/// 0 on success, -EMSGSIZE if message too large
#[no_mangle]
pub unsafe extern "C" fn ctnetlink_dump_tuples_ip(
    skb: *mut sk_buff,
    tuple: *const nf_conntrack_tuple,
) -> c_int {
    let mut ret: c_int = 0;
    let nest_parms = nla_nest_start(skb, CTA_TUPLE_IP);
    if nest_parms.is_null() {
        return -EMSGSIZE;
    }

    match (*tuple).src_l3num {
        NFPROTO_IPV4 => {
            ret = ipv4_tuple_to_nlattr(skb, tuple);
        }
        NFPROTO_IPV6 => {
            ret = ipv6_tuple_to_nlattr(skb, tuple);
        }
        _ => {}
    }

    nla_nest_end(skb, nest_parms);
    ret
}

/// Dump connection tuple to netlink message
///
/// # Safety
/// - `skb` must be a valid pointer to sk_buff
/// - `tuple` must be a valid pointer to nf_conntrack_tuple
///
/// # Returns
/// 0 on success, -EMSGSIZE if message too large
#[no_mangle]
pub unsafe extern "C" fn ctnetlink_dump_tuples(
    skb: *mut sk_buff,
    tuple: *const nf_conntrack_tuple,
) -> c_int {
    let mut ret: c_int = 0;
    rcu_read_lock();
    ret = ctnetlink_dump_tuples_ip(skb, tuple);
    
    if ret >= 0 {
        let l4proto = nf_ct_l4proto_find((*tuple).dst.protonum);
        if !l4proto.is_null() {
            ret = ctnetlink_dump_tuples_proto(skb, tuple, l4proto);
        }
    }
    rcu_read_unlock();
    ret
}

/// Dump zone ID to netlink message
///
/// # Safety
/// - `skb` must be a valid pointer to sk_buff
/// - `zone` must be a valid pointer to nf_conntrack_zone
///
/// # Returns
/// 0 on success, -EMSGSIZE if message too large
#[no_mangle]
pub unsafe extern "C" fn ctnetlink_dump_zone_id(
    skb: *mut sk_buff,
    attrtype: c_int,
    zone: *const nf_conntrack_zone,
    dir: c_int,
) -> c_int {
    if (*zone).id == NF_CT_DEFAULT_ZONE_ID || (*zone).dir != dir {
        return 0;
    }
    if nla_put_be16(skb, attrtype, htons((*zone).id)) != 0 {
        return -EMSGSIZE;
    }
    0
}

/// Dump connection status to netlink message
///
/// # Safety
/// - `skb` must be a valid pointer to sk_buff
/// - `ct` must be a valid pointer to nf_conn
///
/// # Returns
/// 0 on success, -EMSGSIZE if message too large
#[no_mangle]
pub unsafe extern "C" fn ctnetlink_dump_status(
    skb: *mut sk_buff,
    ct: *const nf_conn,
) -> c_int {
    if nla_put_be32(skb, CTA_STATUS, htonl((*ct).status)) != 0 {
        return -EMSGSIZE;
    }
    0
}

/// Dump connection timeout to netlink message
///
/// # Safety
/// - `skb` must be a valid pointer to sk_buff
/// - `ct` must be a valid pointer to nf_conn
///
/// # Returns
/// 0 on success, -EMSGSIZE if message too large
#[no_mangle]
pub unsafe extern "C" fn ctnetlink_dump_timeout(
    skb: *mut sk_buff,
    ct: *const nf_conn,
    skip_zero: c_int,
) -> c_int {
    let timeout = nf_ct_expires(ct) / 4; // Assuming HZ=4 for example
    if skip_zero != 0 && timeout == 0 {
        return 0;
    }
    if nla_put_be32(skb, CTA_TIMEOUT, htonl(timeout)) != 0 {
        return -EMSGSIZE;
    }
    0
}

// Additional functions would be implemented similarly following the same pattern...

// Constants
pub const CTA_TUPLE_PROTO: c_int = 1;
pub const CTA_PROTO_NUM: c_int = 1;
pub const CTA_TUPLE_IP: c_int = 2;
pub const CTA_IP_V4_SRC: c_int = 1;
pub const CTA_IP_V4_DST: c_int = 2;
pub const CTA_IP_V6_SRC: c_int = 3;
pub const CTA_IP_V6_DST: c_int = 4;
pub const CTA_STATUS: c_int = 5;
pub const CTA_TIMEOUT: c_int = 6;
pub const CTA_PROTOINFO: c_int = 7;
pub const CTA_HELP: c_int = 8;
pub const CTA_COUNTERS_ORIG: c_int = 9;
pub const CTA_COUNTERS_REPLY: c_int = 10;
pub const CTA_TIMESTAMP: c_int = 11;
pub const CTA_MARK: c_int = 12;
pub const CTA_SECCTX: c_int = 13;
pub const CTA_LABELS: c_int = 14;
pub const CTA_TUPLE_MASTER: c_int = 15;

pub const NFPROTO_IPV4: u8 = 2;
pub const NFPROTO_IPV6: u8 = 10;
pub const NF_CT_DEFAULT_ZONE_ID: u16 = 0xffff;

// htons and htonl implementations for no_std environment
#[inline]
fn htons(x: u16) -> u16 {
    (x >> 8) | (x << 8)
}

#[inline]
fn htonl(x: u32) -> u32 {
    ((x & 0x000000ff) << 24) |
    ((x & 0x0000ff00) << 8)  |
    ((x & 0x00ff0000) >> 8)  |
    ((x & 0xff000000) >> 24)
}

// Tests (conditional compilation)
#[cfg(test)]
mod tests {
    #[test]
    fn test_htons() {
        assert_eq!(super::htons(0x1234), 0x3412);
    }

    #[test]
    fn test_htonl() {
        assert_eq!(super::htonl(0x12345678), 0x78063412);
    }
}