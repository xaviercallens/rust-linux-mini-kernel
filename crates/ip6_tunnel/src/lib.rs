//!
//! IPv6 tunneling device implementation for Linux kernel
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![no_main]
#![allow(non_camel_case_types)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::panic::PanicInfo;
use core::ptr;
use kernel_types::*;

type Net = c_void;
type DstCache = c_void;
type GroCells = c_void;

const IP6_TUNNEL_HASH_SIZE_SHIFT: c_int = 5;
const IP6_TUNNEL_HASH_SIZE: usize = 1usize << (IP6_TUNNEL_HASH_SIZE_SHIFT as usize);
const IFNAMSIZ: usize = 16;
const IFF_UP: c_int = 1 << 0;

pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ENODEV: c_int = -19;
pub const EEXIST: c_int = -17;
pub const E2BIG: c_int = -75;

#[repr(C)]
pub struct net_device {
    pub name: [c_char; IFNAMSIZ],
    pub flags: c_int,
    pub priv_: *mut c_void,
}

#[repr(C)]
pub struct __ip6_tnl_parm {
    pub name: [c_char; IFNAMSIZ],
    pub link: c_int,
    pub mode: c_int,
    pub collect_md: c_int,
    pub raddr: in6_addr,
    pub laddr: in6_addr,
}

#[repr(C)]
pub struct ip6_tnl {
    pub dev: *mut net_device,
    pub net: *mut Net,
    pub dst_cache: *mut DstCache,
    pub gro_cells: *mut GroCells,
    pub next: *mut ip6_tnl,
    pub parms: __ip6_tnl_parm,
}

#[repr(C)]
pub struct ip6_tnl_net {
    pub fb_tnl_dev: *mut net_device,
    pub tnls_r_l: [*mut ip6_tnl; IP6_TUNNEL_HASH_SIZE],
    pub tnls_wc: [*mut ip6_tnl; 1],
    pub tnls: [[*mut ip6_tnl; IP6_TUNNEL_HASH_SIZE]; 2],
    pub collect_md_tun: *mut ip6_tnl,
}

unsafe extern "C" {
    static mut ip6_tnl_net_id: c_int;
    fn net_generic(net: *mut Net, id: c_int) -> *mut ip6_tnl_net;
    fn ipv6_addr_equal(a1: *const in6_addr, a2: *const in6_addr) -> bool;
    fn ipv6_addr_hash(a: *const in6_addr) -> c_uint;
}

#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    loop {}
}

unsafe fn in6_addr_any() -> in6_addr {
    core::mem::zeroed()
}

unsafe fn in6_addr_is_any(a: *const in6_addr) -> bool {
    let any = in6_addr_any();
    ipv6_addr_equal(a, &any as *const in6_addr)
}

unsafe fn get_list(mut head: *mut ip6_tnl, mut f: impl FnMut(*mut ip6_tnl)) {
    while !head.is_null() {
        f(head);
        head = (*head).next;
    }
}

#[no_mangle]
pub unsafe extern "C" fn hash_32(val: c_uint, bits: u32) -> c_uint {
    if bits == 0 {
        0
    } else {
        val & ((1u32 << bits) - 1u32)
    }
}

#[no_mangle]
pub unsafe extern "C" fn HASH(addr1: *const in6_addr, addr2: *const in6_addr) -> c_uint {
    let h1 = ipv6_addr_hash(addr1);
    let h2 = ipv6_addr_hash(addr2);
    hash_32(h1 ^ h2, IP6_TUNNEL_HASH_SIZE_SHIFT as u32)
}

#[no_mangle]
pub unsafe extern "C" fn ip6_tnl_lookup(
    net: *mut Net,
    link: c_int,
    remote: *const in6_addr,
    local: *const in6_addr,
) -> *mut ip6_tnl {
    if net.is_null() || remote.is_null() || local.is_null() {
        return ptr::null_mut();
    }

    let hash = HASH(remote, local);
    let ip6n = net_generic(net, IP6_TNL_NET_ID);
    let any = in6_addr { in6_u: in6_addr_union { u6_addr8: [0; 16] } };
    let mut cand: *mut ip6_tnl = ptr::null_mut();

    let h1 = HASH(remote, local) as usize;
    get_list((*ip6n).tnls_r_l[h1], |t| {
        if !ipv6_addr_equal(local, &(*t).parms.laddr as *const in6_addr)
            || !ipv6_addr_equal(remote, &(*t).parms.raddr as *const in6_addr)
            || (*(*t).dev).flags & IFF_UP == 0
        {
            return;
        }
        if link == (*t).parms.link {
            cand = t;
        } else if cand.is_null() {
            cand = t;
        }
    });
    if !cand.is_null() && (*cand).parms.link == link {
        return cand;
    }

    let h2 = HASH(&any as *const in6_addr, local) as usize;
    get_list((*ip6n).tnls_r_l[h2], |t| {
        if !ipv6_addr_equal(local, &(*t).parms.laddr as *const in6_addr)
            || !in6_addr_is_any(&(*t).parms.raddr as *const in6_addr)
            || (*(*t).dev).flags & IFF_UP == 0
        {
            return;
        }
        if link == (*t).parms.link {
            cand = t;
        } else if cand.is_null() {
            cand = t;
        }
    });
    if !cand.is_null() && (*cand).parms.link == link {
        return cand;
    }

    let h3 = HASH(remote, &any as *const in6_addr) as usize;
    get_list((*ip6n).tnls_r_l[h3], |t| {
        if !ipv6_addr_equal(remote, &(*t).parms.raddr as *const in6_addr)
            || !in6_addr_is_any(&(*t).parms.laddr as *const in6_addr)
            || (*(*t).dev).flags & IFF_UP == 0
        {
            return;
        }
        if link == (*t).parms.link {
            cand = t;
        } else if cand.is_null() {
            cand = t;
        }
    });

    if !cand.is_null() {
        return cand;
    }

    if !(*ip6n).collect_md_tun.is_null()
        && !(*(*ip6n).collect_md_tun).dev.is_null()
        && ((*(*(*ip6n).collect_md_tun).dev).flags & IFF_UP) != 0
    {
        return (*ip6n).collect_md_tun;
    }

    if !(*ip6n).tnls_wc[0].is_null()
        && !(*(*ip6n).tnls_wc[0]).dev.is_null()
        && ((*(*(*ip6n).tnls_wc[0]).dev).flags & IFF_UP) != 0
    {
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
        let zero = in6_addr { in6_u: in6_addr_union { u6_addr8: [0; 16] } };
        ptr::read(addr) == zero
    }
}

unsafe fn ipv6_addr_hash(addr: *const in6_addr) -> c_uint {
    if addr.is_null() {
        0
    } else {
        let a = &(*addr).in6_u.u6_addr8;
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
static mut IP6_TNL_NET_ID: c_int = 0;
static mut LOG_ECN_ERROR: bool = true;

// Tests (conditional compilation)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        let a = in6_addr { in6_u: in6_addr_union { u6_addr8: [1; 16] } };
        let b = in6_addr { in6_u: in6_addr_union { u6_addr8: [2; 16] } };
        unsafe {
            let h = HASH(&a, &b);
            assert!(h != 0);
        }
    }
}