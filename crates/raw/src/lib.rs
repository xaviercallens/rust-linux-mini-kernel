#![no_std]

pub mod kernel_types {
    pub use core::ffi::{
        c_char, c_int, c_long, c_short, c_uchar, c_uint, c_ulong, c_ushort, c_void,
    };

    pub type size_t = usize;
    pub type c_size_t = usize;
    pub type socklen_t = u32;
}

use kernel_types::*;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct in6_addr {
    pub s6_addr: [u8; 16],
}

#[repr(C)]
pub struct sock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct net {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CacheKey {
    pub src: in6_addr,
    pub dst: in6_addr,
    pub ifindex: c_int,
    pub proto: c_uchar,
    pub pad: [u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CacheStatistics {
    pub hits: c_ulong,
    pub misses: c_ulong,
    pub entries: c_ulong,
}

#[repr(C)]
pub struct CacheManager {
    _private: [u8; 0],
}

#[inline]
fn ipv6_addr_is_multicast(addr: *const in6_addr) -> bool {
    if addr.is_null() {
        return false;
    }
    unsafe { (*addr).s6_addr[0] == 0xff }
}

#[inline]
fn ipv6_addr_any(addr: *const in6_addr) -> bool {
    if addr.is_null() {
        return false;
    }
    unsafe { (*addr).s6_addr.iter().all(|&b| b == 0) }
}

#[inline]
fn ipv6_addr_equal(a: *const in6_addr, b: *const in6_addr) -> bool {
    if a.is_null() || b.is_null() {
        return false;
    }
    unsafe { (*a).s6_addr == (*b).s6_addr }
}

#[unsafe(no_mangle)]
pub extern "C" fn raw_v6_match(
    _net: *const net,
    _sk: *const sock,
    daddr: *const in6_addr,
    saddr: *const in6_addr,
    _iif: c_int,
    _sdif: c_int,
) -> c_int {
    if daddr.is_null() || saddr.is_null() {
        return 0;
    }

    if ipv6_addr_is_multicast(daddr) {
        return 1;
    }

    if ipv6_addr_any(saddr) || ipv6_addr_equal(saddr, daddr) {
        return 1;
    }

    0
}

#[unsafe(no_mangle)]
pub extern "C" fn raw_v6_bind(
    _net: *mut net,
    _sk: *mut sock,
    addr: *const in6_addr,
    _addr_len: socklen_t,
) -> c_int {
    if addr.is_null() {
        return -22;
    }

    if ipv6_addr_is_multicast(addr) {
        return -22;
    }

    0
}

#[unsafe(no_mangle)]
pub extern "C" fn raw_v6_icmp_filter(type_: c_uchar, filter_bits: c_uint) -> c_int {
    let bit = 1u32 << ((type_ as u32) & 31);
    if (filter_bits & bit) != 0 {
        1
    } else {
        0
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn __raw_v6_lookup(
    net_ns: *const net,
    sk: *const sock,
    daddr: *const in6_addr,
    saddr: *const in6_addr,
    iif: c_int,
    sdif: c_int,
) -> *const sock {
    let m = raw_v6_match(net_ns, sk, daddr, saddr, iif, sdif);
    if m != 0 { sk } else { core::ptr::null() }
}

#[unsafe(no_mangle)]
pub extern "C" fn cache_key_init(
    key: *mut CacheKey,
    src: *const in6_addr,
    dst: *const in6_addr,
    ifindex: c_int,
    proto: c_uchar,
) -> c_int {
    if key.is_null() || src.is_null() || dst.is_null() {
        return -22;
    }

    unsafe {
        (*key).src = *src;
        (*key).dst = *dst;
        (*key).ifindex = ifindex;
        (*key).proto = proto;
        (*key).pad = [0; 3];
    }

    0
}

#[unsafe(no_mangle)]
pub extern "C" fn cache_stats_reset(stats: *mut CacheStatistics) {
    if stats.is_null() {
        return;
    }

    unsafe {
        (*stats).hits = 0;
        (*stats).misses = 0;
        (*stats).entries = 0;
    }
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo<'_>) -> ! {
    loop {
        core::hint::spin_loop();
    }
}