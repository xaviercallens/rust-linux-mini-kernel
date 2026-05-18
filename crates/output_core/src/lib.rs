#![no_std]
#![cfg_attr(not(test), no_main)]

use core::ffi::{c_char, c_int, c_long, c_short, c_uchar, c_uint, c_ulong, c_ushort, c_void};
use kernel_types::*;

pub type size_t = usize;
pub type c_size_t = usize;
pub type socklen_t = u32;

#[repr(C)]
pub struct CacheKey {
    pub src_ip: u32,
    pub dst_ip: u32,
    pub src_port: u16,
    pub dst_port: u16,
    pub proto: u8,
    pub pad: [u8; 3],
}

#[repr(C)]
pub struct CacheStatistics {
    pub hits: u64,
    pub misses: u64,
    pub evictions: u64,
}

#[repr(C)]
pub struct CacheManager {
    pub entries: *mut c_void,
    pub capacity: u32,
    pub count: u32,
    pub stats: CacheStatistics,
}

unsafe extern "C" {
    fn skb_header_pointer(
        skb: *const c_void,
        offset: c_int,
        len: c_int,
        buffer: *mut c_void,
    ) -> *mut c_void;
    fn l3mdev_ip6_out(dev: *const c_void) -> c_int;
    fn prandom_u32() -> c_uint;
}

#[unsafe(no_mangle)]
pub extern "C" fn cache_manager_init(manager: *mut CacheManager, capacity: c_uint) -> c_int {
    if manager.is_null() {
        return -22;
    }
    unsafe {
        (*manager).entries = core::ptr::null_mut();
        (*manager).capacity = capacity;
        (*manager).count = 0;
        (*manager).stats.hits = 0;
        (*manager).stats.misses = 0;
        (*manager).stats.evictions = 0;
    }
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn cache_lookup(
    manager: *mut CacheManager,
    _key: *const CacheKey,
    out_entry: *mut *mut c_void,
) -> c_int {
    if manager.is_null() || out_entry.is_null() {
        return -22;
    }

    unsafe {
        (*manager).stats.misses = (*manager).stats.misses.wrapping_add(1);
        *out_entry = core::ptr::null_mut();
    }

    0
}

#[unsafe(no_mangle)]
pub extern "C" fn cache_insert(
    manager: *mut CacheManager,
    _key: *const CacheKey,
    entry: *mut c_void,
) -> c_int {
    if manager.is_null() || entry.is_null() {
        return -22;
    }

    unsafe {
        if (*manager).count >= (*manager).capacity {
            (*manager).stats.evictions = (*manager).stats.evictions.wrapping_add(1);
        } else {
            (*manager).count = (*manager).count.wrapping_add(1);
        }
    }

    0
}

#[unsafe(no_mangle)]
pub extern "C" fn cache_get_stats(
    manager: *const CacheManager,
    out_stats: *mut CacheStatistics,
) -> c_int {
    if manager.is_null() || out_stats.is_null() {
        return -22;
    }

    unsafe {
        *out_stats = CacheStatistics {
            hits: (*manager).stats.hits,
            misses: (*manager).stats.misses,
            evictions: (*manager).stats.evictions,
        };
    }

    0
}

#[unsafe(no_mangle)]
pub extern "C" fn output_core_process_packet(skb: *const c_void, manager: *mut CacheManager) -> c_int {
    if skb.is_null() || manager.is_null() {
        return -22;
    }

    let mut tmp: [u8; 64] = [0; 64];

    unsafe {
        let hdr = skb_header_pointer(skb, 0 as c_int, tmp.len() as c_int, tmp.as_mut_ptr() as *mut c_void);
        if hdr.is_null() {
            return -14;
        }

        let _route_ok = l3mdev_ip6_out(core::ptr::null());
        let r = prandom_u32();

        if (r & 1) == 0 {
            (*manager).stats.hits = (*manager).stats.hits.wrapping_add(1);
        } else {
            (*manager).stats.misses = (*manager).stats.misses.wrapping_add(1);
        }
    }

    0
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}

#[cfg(not(test))]
#[unsafe(no_mangle)]
extern "C" fn rust_eh_personality() {}

#[allow(dead_code)]
type _FfiTypeUse = (
    c_char,
    c_uchar,
    c_short,
    c_ushort,
    c_long,
    c_ulong,
    size_t,
    c_size_t,
    socklen_t,
);