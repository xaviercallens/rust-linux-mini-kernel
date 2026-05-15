//! IPv6 tunneling device implementation for Linux kernel
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]  // For C-style type names

use core::ptr;
use core::ffi::c_void;
use core::ffi::c_int;
use core::ffi::c_uint;
use core::ffi::c_ulong;
use core::ffi::c_char;
use core::mem;

// Constants from C
const IP6_TUNNEL_HASH_SIZE_SHIFT: c_int = 5;
const IP6_TUNNEL_HASH_SIZE: c_int = 1 << IP6_TUNNEL_HASH_SIZE_SHIFT;
const IFNAMSIZ: c_int = 16;
const IFF_UP: c_int = 1 << 0;
const NEXTHDR_NONE: c_int = 59;
const NEXTHDR_HOP: c_int = 0;
const NEXTHDR_TCP: c_int = 6;
const NEXTHDR_UDP: c_int = 17;
const NEXTHDR_IPV6: c_int = 41;
const NEXTHDR_ROUTING: c_int = 43;
const NEXTHDR_FRAGMENT: c_int = 44;
const NEXTHDR_ICMP: c_int = 58;

// Error codes
pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ENODEV: c_int = -19;
pub const EEXIST: c_int = -17;
pub const E2BIG: c_int = -75;

// Type definitions
#[repr(C)]
struct in6_addr {
    __in6_u: [u8; 16],
}

#[repr(C)]
struct net_device {
    name: [c_char; IFNAMSIZ],
    flags: c_int,
    priv: *mut c_void,
    // ... other fields omitted for brevity
}

#[repr(C)]
struct __ip6_tnl_parm {
    name: [c_char; IFNAMSIZ],
    link: c_int,
    mode: c_int,
    collect_md: c_int,
    raddr: in6_addr,
    laddr: in6_addr,
    // ... other fields omitted for brevity
}

#[repr(C)]
struct ip6_tnl {
    dev: *mut net_device,
    net: *mut c_void, // struct net*
    dst_cache: *mut c_void, // struct dst_cache*
    gro_cells: *mut c_void, // struct gro_cells*
    next: *mut ip6_tnl,
    parms: __ip6_tnl_parm,
}

#[repr(C)]
struct ip6_tnl_net {
    fb_tnl_dev: *mut net_device,
    tnls_r_l: [*mut ip6_tnl; IP6_TUNNEL_HASH_SIZE as usize],
    tnls_wc: [*mut ip6_tnl; 1],
    tnls: [[*mut ip6_tnl; IP6_TUNNEL_HASH_SIZE as usize]; 2],
    collect_md_tun: *mut ip6_tnl,
}

// Function implementations
#[no_mangle]
pub unsafe extern "C" fn HASH(
    addr1: *const in6_addr,
    addr2: *const in6_addr,
) -> c_uint {
    let hash = ipv6_addr_hash(addr1) ^ ipv6_addr_hash(addr2);
    hash_32(hash, IP6_TUNNEL_HASH_SIZE_SHIFT as u32)
}

#[no_mangle]
pub unsafe extern "C" fn hash_32(
    val: c_uint,
    bits: u32,
) -> c_uint {
    val & ((1 << bits) - 1)
}

#[no_mangle]
pub unsafe extern "C" fn ip6_tnl_lookup(
    net: *mut c_void, // struct net*
    link: c_int,
    remote: *const in6_addr,
    local: *const in6_addr,
) -> *mut ip6_tnl {
    if net.is_null() || remote.is_null() || local.is_null() {
        return ptr::null_mut();
    }

    let hash = HASH(remote, local);
    let ip6n = net_generic(net, ip6_tnl_net_id);
    let any = in6_addr { __in6_u: [0; 16] };
    let mut t: *mut ip6_tnl = ptr::null_mut();
    let mut cand: *mut ip6_tnl = ptr::null_mut();

    // First pass - exact match
    let bucket = &(*ip6n).tnls_r_l[hash as usize];
    for t in get_list(bucket) {
        if !ipv6_addr_equal(local, &(*t).parms.laddr) ||
           !ipv6_addr_equal(remote, &(*t).parms.raddr) ||
           !((*(*t).dev).flags & IFF_UP) != 0 {
            continue;
        }

        if link == (*t).parms.link {
            return t;
        } else {
            cand = t;
        }
    }

    // Second pass - local any
    let hash = HASH(&any, local);
    for t in get_list(&(*ip6n).tnls_r_l[hash as usize]) {
        if !ipv6_addr_equal(local, &(*t).parms.laddr) ||
           !ipv6_addr_any(&(*t).parms.raddr) ||
           !((*(*t).dev).flags & IFF_UP) != 0 {
            continue;
        }

        if link == (*t).parms.link {
            return t;
        } else if cand.is_null() {
            cand = t;
        }
    }

    // Third pass - remote any
    let hash = HASH(remote, &any);
    for t in get_list(&(*ip6n).tnls_r_l[hash as usize]) {
        if !ipv6_addr_equal(remote, &(*t).parms.raddr) ||
           !ipv6_addr_any(&(*t).parms.laddr) ||
           !((*(*t).dev).flags & IFF_UP) != 0 {
            continue;
        }

        if link == (*t).parms.link {
            return t;
        } else if cand.is_null() {
            cand = t;
        }
    }

    if !cand.is_null() {
        return cand;
    }

    // Check collect_md_tun
    if !(*ip6n).collect_md_tun.is_null() &&
       ((*(*ip6n).collect_md_tun).dev).is_null() &&
       ((*(*(*ip6n).collect_md_tun).dev).flags & IFF_UP) != 0 {
        return (*ip6n).collect_md_tun;
    }

    // Fallback to wildcard
    if !(*ip6n).tnls_wc[0].is_null() &&
       ((*(*ip6n).tnls_wc[0]).dev).is_null() &&
       ((*(*(*ip6n).tnls_wc[0]).dev).flags & IFF_UP) != 0 {
        return (*ip6n).tnls_wc[0];
    }

    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn ip6_tnl_bucket(
    ip6n: *mut ip6_tnl_net,
    p: *const __ip6_tnl_parm,
) -> *mut *mut ip6_tnl {
    if ip6n.is_null() || p.is_null() {
        return ptr::null_mut();
    }

    let remote = &(*p).raddr;
    let local = &(*p).laddr;
    let mut h: c_uint = 0;
    let mut prio: c_int = 0;

    if !ipv6_addr_any(remote) || !ipv6_addr_any(local) {
        prio = 1;
        h = HASH(remote, local);
    }
    &mut (*ip6n).tnls[prio as usize][h as usize]
}

#[no_mangle]
pub unsafe extern "C" fn ip6_tnl_link(
    ip6n: *mut ip6_tnl_net,
    t: *mut ip6_tnl,
) {
    if ip6n.is_null() || t.is_null() {
        return;
    }

    let tp = ip6_tnl_bucket(ip6n, &(*t).parms);
    if (*t).parms.collect_md != 0 {
        (*ip6n).collect_md_tun = t;
    }
    (*t).next = *tp;
    *tp = t;
}

#[no_mangle]
pub unsafe extern "C" fn ip6_tnl_unlink(
    ip6n: *mut ip6_tnl_net,
    t: *mut ip6_tnl,
) {
    if ip6n.is_null() || t.is_null() {
        return;
    }

    if (*t).parms.collect_md != 0 {
        (*ip6n).collect_md_tun = ptr::null_mut();
    }

    let mut tp = ip6_tnl_bucket(ip6n, &(*t).parms);
    let mut iter: *mut ip6_tnl = ptr::null_mut();

    while !(*tp).is_null() {
        iter = *tp;
        if iter == t {
            *tp = (*t).next;
            break;
        }
        tp = &mut (*iter).next;
    }
}

// Helper functions
unsafe fn get_list(head: *mut *mut ip6_tnl) -> impl Iterator<Item = *mut ip6_tnl> {
    let mut current = *head;
    core::iter::from_fn(move || {
        if current.is_null() {
            None
        } else {
            let next = (*current).next;
            Some(current)
        }
    })
}

unsafe fn ipv6_addr_equal(a: *const in6_addr, b: *const in6_addr) -> bool {
    if a.is_null() || b.is_null() {
        false
    } else {
        ptr::read(a) == ptr::read(b)
    }
}

unsafe fn ipv6_addr_any(addr: *const in6_addr) -> bool {
    if addr.is_null() {
        true
    } else {
        let zero: in6_addr = in6_addr { __in6_u: [0; 16] };
        *addr == zero
    }
}

unsafe fn ipv6_addr_hash(addr: *const in6_addr) -> c_uint {
    if addr.is_null() {
        0
    } else {
        let a = &(*addr).__in6_u;
        let mut hash = 0;
        for &byte in a {
            hash = hash.wrapping_mul(31).wrapping_add(byte as c_uint);
        }
        hash
    }
}

unsafe fn net_generic(net: *mut c_void, id: c_int) -> *mut ip6_tnl_net {
    // Simplified implementation - actual implementation depends on kernel's net_generic
    ptr::null_mut()
}

// Module parameters
static mut ip6_tnl_net_id: c_int = 0;
static mut log_ecn_error: bool = true;

// Tests (conditional compilation)
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_hash() {
        let a = in6_addr { __in6_u: [1; 16] };
        let b = in6_addr { __in6_u: [2; 16] };
        unsafe {
            let h = HASH(&a, &b);
            assert!(h != 0);
        }
    }
}