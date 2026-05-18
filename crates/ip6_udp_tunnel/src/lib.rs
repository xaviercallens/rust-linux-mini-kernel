#![no_std]
#![no_main]

use core::ffi::c_void;
use core::panic::PanicInfo;
use kernel_types::*;

/// C-compatible key used by the cache.
#[repr(C)]
pub struct CacheKey {
    pub src: [u8; 16],
    pub dst: [u8; 16],
    pub src_port: u16,
    pub dst_port: u16,
    pub proto: u8,
    pub pad: [u8; 3],
}

/// C-compatible statistics for cache operations.
#[repr(C)]
pub struct CacheStatistics {
    pub hits: u64,
    pub misses: u64,
    pub inserts: u64,
    pub evictions: u64,
    pub errors: u64,
}

/// Opaque cache manager owned externally (C/kernel side).
#[repr(C)]
pub struct CacheManager {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn kmalloc(size: size_t, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *const c_void);
}

#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_eh_personality() {}

/// Initialize statistics to zero.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cache_stats_init(stats: *mut CacheStatistics) -> c_int {
    if stats.is_null() {
        return -22;
    }
    (*stats).hits = 0;
    (*stats).misses = 0;
    (*stats).inserts = 0;
    (*stats).evictions = 0;
    (*stats).errors = 0;
    0
}

/// Compare two cache keys.
/// Returns 1 if equal, 0 otherwise.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cache_key_equal(a: *const CacheKey, b: *const CacheKey) -> c_int {
    if a.is_null() || b.is_null() {
        return 0;
    }

    let ka = &*a;
    let kb = &*b;

    if ka.src != kb.src
        || ka.dst != kb.dst
        || ka.src_port != kb.src_port
        || ka.dst_port != kb.dst_port
        || ka.proto != kb.proto
    {
        return 0;
    }

    1
}

/// Allocate a raw cache manager blob.
/// The actual layout is intentionally opaque.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cache_manager_alloc(bytes: size_t, gfp: c_uint) -> *mut CacheManager {
    if bytes == 0 {
        return core::ptr::null_mut();
    }

    let p = kmalloc(bytes, gfp);
    p.cast::<CacheManager>()
}

/// Free an allocated cache manager blob.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cache_manager_free(mgr: *mut CacheManager) {
    if mgr.is_null() {
        return;
    }
    kfree(mgr.cast::<c_void>());
}