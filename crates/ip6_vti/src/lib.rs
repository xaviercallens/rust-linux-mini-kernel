
//! IPv6 virtual tunneling interface
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]

use core::panic::PanicInfo;
use core::ptr;
use kernel_types::*;

pub const IP6_VTI_HASH_SIZE_SHIFT: c_int = 5;
pub const IP6_VTI_HASH_SIZE: c_int = 1 << IP6_VTI_HASH_SIZE_SHIFT;
pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ENOSYS: c_int = -38;

// Constants from kernel headers
pub const IFNAMSIZ: usize = 16;
pub const IFF_UP: c_int = 1 << 0;

// Type definitions
#[repr(C)]
#[derive(Copy, Clone)]
pub struct net_device {
    pub name: [c_char; IFNAMSIZ],
    pub flags: c_int,
    pub tstats: *mut c_void,
    pub dev: *mut net_device,
    pub next: *mut net_device,
    pub priv_data: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip6_tnl_parm {
    pub laddr: in6_addr,
    pub raddr: in6_addr,
    pub name: [c_char; IFNAMSIZ],
    pub proto: c_int,
    pub i_key: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip6_tnl {
    pub parms: ip6_tnl_parm,
    pub dev: *mut net_device,
    pub net: *mut c_void,
    pub next: *mut ip6_tnl,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vti6_net {
    pub fb_tnl_dev: *mut net_device,
    pub tnls_r_l: [*mut ip6_tnl; IP6_VTI_HASH_SIZE as usize],
    pub tnls_wc: [*mut ip6_tnl; 1],
    pub tnls: [*mut ip6_tnl; 2],
}

unsafe fn hash(_remote: *const in6_addr, _local: *const in6_addr) -> c_int {
    0
}

unsafe fn get_vti6_net(_net: *mut c_void) -> *mut vti6_net {
    ptr::null_mut()
}

unsafe fn ipv6_addr_equal(_a: *const in6_addr, _b: *const in6_addr) -> bool {
    false
}

unsafe fn ipv6_addr_any(_a: *const in6_addr) -> bool {
    false
}

#[no_mangle]
pub unsafe extern "C" fn vti6_tnl_lookup(
    net: *mut c_void,
    remote: *const in6_addr,
    local: *const in6_addr,
) -> *mut ip6_tnl {
    let ip6n = get_vti6_net(net);
    if ip6n.is_null() {
        return ptr::null_mut();
    }

    let ip6n = &*ip6n;
    let mut t: *mut ip6_tnl;
    let mut hash = HASH(remote, local);
    let any: in6_addr = unsafe { core::mem::zeroed() };

    for i in 0..IP6_VTI_HASH_SIZE {
        t = ip6n.tnls_r_l[i as usize];
        while !t.is_null() {
            if ipv6_addr_equal(local, &(*t).parms.laddr as *const _)
                && ipv6_addr_equal(remote, &(*t).parms.raddr as *const _)
                && !(*t).dev.is_null()
                && ((*(*t).dev).flags & IFF_UP != 0)
            {
                return t;
            }
            t = (*t).next;
        }
    }

    let _hash = hash(&any as *const _, local);
    for i in 0..IP6_VTI_HASH_SIZE {
        t = ip6n.tnls_r_l[i as usize];
        while !t.is_null() {
            if ipv6_addr_equal(local, &(*t).parms.laddr as *const _)
                && !(*t).dev.is_null()
                && ((*(*t).dev).flags & IFF_UP != 0)
            {
                return t;
            }
            t = (*t).next;
        }
    }

    let _hash = hash(remote, &any as *const _);
    for i in 0..IP6_VTI_HASH_SIZE {
        t = ip6n.tnls_r_l[i as usize];
        while !t.is_null() {
            if ipv6_addr_equal(remote, &(*t).parms.raddr as *const _)
                && !(*t).dev.is_null()
                && ((*(*t).dev).flags & IFF_UP != 0)
            {
                return t;
            }
            t = (*t).next;
        }
    }

    let t_wc = ip6n.tnls_wc[0];
    if !t_wc.is_null() && !(*t_wc).dev.is_null() && ((*(*t_wc).dev).flags & IFF_UP != 0) {
        return t_wc;
    }

    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn vti6_tnl_bucket(
    ip6n: *mut vti6_net,
    p: *const ip6_tnl_parm,
) -> *mut *mut ip6_tnl {
    let remote = &(*p).raddr as *const in6_addr;
    let local = &(*p).laddr as *const in6_addr;
    let h: usize = if !ipv6_addr_any(remote) || !ipv6_addr_any(local) {
        1
    } else {
        0
    };

    if h == 0 {
        &mut (*ip6n).tnls_wc[0]
    } else {
        let hv = hash(remote, local) as usize;
        &mut (*ip6n).tnls_r_l[hv % (IP6_VTI_HASH_SIZE as usize)]
    }
}

#[no_mangle]
pub unsafe extern "C" fn vti6_tnl_link(ip6n: *mut vti6_net, t: *mut ip6_tnl) {
    let tp = vti6_tnl_bucket(ip6n, &(*t).parms as *const _);
    (*t).next = *tp;
    *tp = t;
}

#[no_mangle]
pub unsafe extern "C" fn vti6_tnl_unlink(ip6n: *mut vti6_net, t: *mut ip6_tnl) {
    let mut tp = vti6_tnl_bucket(ip6n, &(*t).parms as *const _);
    while !(*tp).is_null() {
        let iter = *tp;
        if iter == t {
            *tp = (*t).next;
            break;
        }
        tp = &mut (*iter).next;
    }
}

/// Free tunnel device
///
/// # Safety
/// - `dev` must be valid pointer to net_device
#[no_mangle]
pub unsafe extern "C" fn vti6_dev_free(
    dev: *mut net_device,
) {
    free_percpu((*dev).tstats);
}

/// Create tunnel device
///
/// # Safety
/// - `dev` must be valid pointer to net_device
#[no_mangle]
pub unsafe extern "C" fn vti6_tnl_create2(
    dev: *mut net_device,
) -> c_int {
    let t = netdev_priv(dev);
    let net = dev_net(dev);
    let ip6n = get_vti6_net(net);

    // (*dev).rtnl_link_ops = &vti6_link_ops;
    let err = register_netdevice(dev);
    if err < 0 {
        return err;
    }

    strcpy((*t).parms.name.as_mut_ptr(), (*dev).name.as_ptr());
    vti6_tnl_link(ip6n, t);

    0
}

/// Locate or create tunnel
///
/// # Safety
/// - `net` must be valid pointer to network namespace
/// - `p` must be valid ip6_tnl_parm pointer
#[no_mangle]
pub unsafe extern "C" fn vti6_locate(
    net: *mut c_void,
    p: *mut ip6_tnl_parm,
    create: c_int,
) -> *mut ip6_tnl {
    let remote = &(*p).raddr;
    let local = &(*p).laddr;
    let ip6n = get_vti6_net(net);
    let mut tp = vti6_tnl_bucket(ip6n, p);
    let mut t: *mut ip6_tnl = ptr::null_mut();

    while !(*tp).is_null() {
        t = *tp;
        if ipv6_addr_equal(local, &(*t).parms.laddr) &&
           ipv6_addr_equal(remote, &(*t).parms.raddr) {
            if create != 0 {
                return ptr::null_mut();
            }
            return t;
        }
        tp = &mut (*t).next;
    }

    if create == 0 {
        return ptr::null_mut();
    }

    vti6_tnl_create(net, p)
}

// Helper functions
#[inline]
fn ipv6_addr_equal(a: *const in6_addr, b: *const in6_addr) -> bool {
    unsafe { ptr::read(a) == ptr::read(b) }
}

#[inline]
fn ipv6_addr_any(a: *const in6_addr) -> bool {
    unsafe { ptr::read(a).in6_u.u6_addr32[0] == 0 && ptr::read(a).in6_u.u6_addr32[1] == 0 }
}

#[inline]
fn HASH(addr1: *const in6_addr, addr2: *const in6_addr) -> c_uint {
    let hash1 = ipv6_addr_hash(addr1);
    let hash2 = ipv6_addr_hash(addr2);
    hash_32(hash1 ^ hash2, IP6_VTI_HASH_SIZE_SHIFT as u32)
}

#[inline]
fn hash_32(mut val: u32, bits: u32) -> c_uint {
    val = val.wrapping_mul(0x9e3779b9);
    (val >> (32 - bits)) as c_uint
}

#[inline]
fn ipv6_addr_hash(addr: *const in6_addr) -> u32 {
    // Simplified hash implementation
    let bytes = unsafe { &(*addr).in6_u.u6_addr8 };
    let mut hash = 0;
    for &b in bytes.iter() {
        hash = hash.wrapping_mul(31).wrapping_add(b as u32);
    }
    hash
}

// FFI helpers (mocked for example)
#[inline]
unsafe fn get_vti6_net(net: *mut c_void) -> *mut vti6_net {
    // In real implementation, this would use net_generic
    ptr::null_mut()
}

#[inline]
unsafe fn netdev_priv(dev: *mut net_device) -> *mut ip6_tnl {
    (*dev).priv_data as *mut ip6_tnl
}

#[inline]
unsafe fn dev_net(dev: *mut net_device) -> *mut c_void {
    // Mock implementation
    ptr::null_mut()
}

#[inline]
unsafe fn register_netdevice(dev: *mut net_device) -> c_int {
    // Mock implementation
    0
}

#[inline]
unsafe fn strcpy(dest: *mut c_char, src: *const c_char) {
    // Simple strcpy implementation
    let mut i = 0;
    while *src.offset(i) != 0 {
        *dest.offset(i) = *src.offset(i);
        i += 1;
    }
    *dest.offset(i) = 0;
}

#[inline]
unsafe fn free_percpu(ptr: *mut c_void) {
    // Mock implementation
}

// Mocked global variables
static vti6_link_ops: rtnl_link_ops = rtnl_link_ops {
    // ... fields initialized ...
};

// Test module
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        let a = in6_addr { in6_u: in6_addr_union { u6_addr8: [0; 16] } };
        let b = in6_addr { in6_u: in6_addr_union { u6_addr8: [1; 16] } };
        let hash = HASH(&a as *const _, &b as *const_);
        assert!(hash < IP6_VTI_HASH_SIZE as u32);
    }
}