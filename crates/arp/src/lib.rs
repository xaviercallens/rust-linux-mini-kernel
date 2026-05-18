#![no_std]
#![no_main]

use core::ffi::c_void;
use core::panic::PanicInfo;
use kernel_types::*;

/// Per spec: socklen_t is u32 in this module.
pub type socklen_t = u32;

#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    loop {}
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CacheKey {
    pub ip_be: u32,
    pub ifindex: c_int,
}

#[repr(C)]
pub struct CacheStatistics {
    pub lookups: u64,
    pub hits: u64,
    pub misses: u64,
    pub inserts: u64,
    pub evictions: u64,
}

#[repr(C)]
pub struct ArpEntry {
    pub key: CacheKey,
    pub mac: [u8; 6],
    pub state: u8,
    pub _pad: u8,
    pub updated_jiffies: c_ulong,
}

#[repr(C)]
pub struct CacheManager {
    pub entries: *mut ArpEntry,
    pub capacity: c_uint,
    pub len: c_uint,
    pub stats: CacheStatistics,
}

#[repr(C)]
pub struct ArpTable {
    pub cache: CacheManager,
    pub flags: c_uint,
}

/// Exported ARP table symbol (FFI-visible).
#[no_mangle]
pub static mut arp_tbl: ArpTable = ArpTable {
    cache: CacheManager {
        entries: core::ptr::null_mut(),
        capacity: 0,
        len: 0,
        stats: CacheStatistics {
            lookups: 0,
            hits: 0,
            misses: 0,
            inserts: 0,
            evictions: 0,
        },
    },
    flags: 0,
};

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;

/// Minimal ARP sender ABI-compatible stub.
#[no_mangle]
pub extern "C" fn arp_send(
    skb: *mut c_void,
    dev: *mut c_void,
    sip_be: u32,
    tip_be: u32,
    sha: *const u8,
    tha: *const u8,
) -> c_int {
    let _ = (skb, sip_be, tip_be, sha, tha);
    if dev.is_null() {
        return -EINVAL;
    }
    0
}

#[no_mangle]
pub extern "C" fn arp_cache_init(
    tbl: *mut ArpTable,
    entries: *mut ArpEntry,
    capacity: c_uint,
) -> c_int {
    if tbl.is_null() {
        return -EINVAL;
    }
    if entries.is_null() && capacity != 0 {
        return -ENOMEM;
    }

    unsafe {
        (*tbl).cache.entries = entries;
        (*tbl).cache.capacity = capacity;
        (*tbl).cache.len = 0;
        (*tbl).cache.stats.lookups = 0;
        (*tbl).cache.stats.hits = 0;
        (*tbl).cache.stats.misses = 0;
        (*tbl).cache.stats.inserts = 0;
        (*tbl).cache.stats.evictions = 0;
    }

    0
}

#[no_mangle]
pub extern "C" fn arp_cache_lookup(
    tbl: *mut ArpTable,
    key: *const CacheKey,
    out_mac: *mut u8,
) -> c_int {
    if tbl.is_null() || key.is_null() || out_mac.is_null() {
        return -EINVAL;
    }

    unsafe {
        let cache = &mut (*tbl).cache;
        cache.stats.lookups = cache.stats.lookups.wrapping_add(1);

        if cache.entries.is_null() || cache.len == 0 {
            cache.stats.misses = cache.stats.misses.wrapping_add(1);
            return -EINVAL;
        }

        let k = *key;
        let mut i: c_uint = 0;
        while i < cache.len && i < cache.capacity {
            let e = cache.entries.add(i as usize);
            if (*e).key.ip_be == k.ip_be && (*e).key.ifindex == k.ifindex {
                core::ptr::copy_nonoverlapping((*e).mac.as_ptr(), out_mac, 6);
                cache.stats.hits = cache.stats.hits.wrapping_add(1);
                return 0;
            }
            i += 1;
        }

        cache.stats.misses = cache.stats.misses.wrapping_add(1);
    }

    -EINVAL
}