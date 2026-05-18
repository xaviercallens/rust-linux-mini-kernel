#![no_std]

use core::ffi::{c_int, c_uchar, c_uint, c_ulong, c_ushort, c_void};
use core::ptr::null_mut;
use kernel_types::*;

pub type size_t = usize;
pub type c_size_t = usize;
pub type socklen_t = u32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CacheKey {
    pub spi: c_uint,
    pub daddr: [u8; 16],
    pub saddr: [u8; 16],
    pub proto: c_uchar,
    pub family: c_ushort,
    pub ifindex: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CacheStatistics {
    pub lookups: c_ulong,
    pub hits: c_ulong,
    pub misses: c_ulong,
    pub inserts: c_ulong,
    pub removes: c_ulong,
    pub errors: c_ulong,
}

#[repr(C)]
pub struct CacheEntry {
    pub key: CacheKey,
    pub value: *mut c_void,
    pub next: *mut CacheEntry,
}

pub struct CacheManager {
    head: *mut CacheEntry,
    count: usize,
    stats: CacheStatistics,
}

impl CacheManager {
    pub const fn new() -> Self {
        Self {
            head: null_mut(),
            count: 0,
            stats: CacheStatistics {
                lookups: 0,
                hits: 0,
                misses: 0,
                inserts: 0,
                removes: 0,
                errors: 0,
            },
        }
    }
}

unsafe fn key_eq(a: *const CacheKey, b: *const CacheKey) -> bool {
    if a.is_null() || b.is_null() {
        return false;
    }
    let ka = unsafe { &*a };
    let kb = unsafe { &*b };

    ka.spi == kb.spi
        && ka.daddr == kb.daddr
        && ka.saddr == kb.saddr
        && ka.proto == kb.proto
        && ka.family == kb.family
        && ka.ifindex == kb.ifindex
}

#[inline]
unsafe fn kmalloc(size: usize) -> *mut c_void {
    unsafe { kernel_malloc(size as size_t) }
}

#[inline]
unsafe fn kfree(ptr: *mut c_void) {
    unsafe { kernel_free(ptr) }
}

#[no_mangle]
pub extern "C" fn esp6_cache_manager_new() -> *mut CacheManager {
    let ptr = unsafe { kmalloc(core::mem::size_of::<CacheManager>()) } as *mut CacheManager;
    if ptr.is_null() {
        return null_mut();
    }

    unsafe {
        ptr.write(CacheManager::new());
    }
    ptr
}

#[no_mangle]
pub unsafe extern "C" fn esp6_cache_manager_free(mgr: *mut CacheManager) {
    if mgr.is_null() {
        return;
    }

    let mut current = unsafe { (*mgr).head };
    while !current.is_null() {
        let next = unsafe { (*current).next };
        unsafe { kfree(current as *mut c_void) };
        current = next;
    }

    unsafe { kfree(mgr as *mut c_void) };
}

#[no_mangle]
pub unsafe extern "C" fn esp6_cache_insert(
    mgr: *mut CacheManager,
    key: *const CacheKey,
    value: *mut c_void,
) -> c_int {
    if mgr.is_null() || key.is_null() {
        return -22;
    }

    let entry_ptr = unsafe { kmalloc(core::mem::size_of::<CacheEntry>()) } as *mut CacheEntry;
    if entry_ptr.is_null() {
        unsafe { (*mgr).stats.errors += 1 };
        return -12;
    }

    unsafe {
        entry_ptr.write(CacheEntry {
            key: *key,
            value,
            next: (*mgr).head,
        });

        (*mgr).head = entry_ptr;
        (*mgr).count += 1;
        (*mgr).stats.inserts += 1;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn esp6_cache_lookup(
    mgr: *mut CacheManager,
    key: *const CacheKey,
) -> *mut c_void {
    if mgr.is_null() || key.is_null() {
        return null_mut();
    }

    unsafe { (*mgr).stats.lookups += 1 };

    let mut current = unsafe { (*mgr).head };
    while !current.is_null() {
        if unsafe { key_eq(&(*current).key, key) } {
            unsafe { (*mgr).stats.hits += 1 };
            return unsafe { (*current).value };
        }
        current = unsafe { (*current).next };
    }

    unsafe { (*mgr).stats.misses += 1 };
    null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn esp6_cache_remove(mgr: *mut CacheManager, key: *const CacheKey) -> c_int {
    if mgr.is_null() || key.is_null() {
        return -22;
    }

    let mut prev: *mut CacheEntry = null_mut();
    let mut current = unsafe { (*mgr).head };

    while !current.is_null() {
        if unsafe { key_eq(&(*current).key, key) } {
            if prev.is_null() {
                unsafe { (*mgr).head = (*current).next };
            } else {
                unsafe { (*prev).next = (*current).next };
            }

            unsafe {
                (*mgr).count -= 1;
                (*mgr).stats.removes += 1;
                kfree(current as *mut c_void);
            }

            return 0;
        }

        prev = current;
        current = unsafe { (*current).next };
    }

    -2
}

#[no_mangle]
pub unsafe extern "C" fn esp6_cache_count(mgr: *const CacheManager) -> usize {
    if mgr.is_null() {
        return 0;
    }
    unsafe { (*mgr).count }
}

#[no_mangle]
pub unsafe extern "C" fn esp6_cache_get_stats(
    mgr: *const CacheManager,
    out_stats: *mut CacheStatistics,
) -> c_int {
    if mgr.is_null() || out_stats.is_null() {
        return -22;
    }

    unsafe {
        *out_stats = (*mgr).stats;
    }
    0
}