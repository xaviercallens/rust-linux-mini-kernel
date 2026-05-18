#![no_std]

use core::ffi::c_void;
use core::panic::PanicInfo;
use kernel_types::*;

#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    loop {
        core::hint::spin_loop();
    }
}

#[repr(C)]
pub struct CacheKey {
    pub src: [u8; 16],
    pub dst: [u8; 16],
    pub ifindex: c_int,
    pub mark: u32,
    pub tos: u8,
    pub scope: u8,
    pub pad: u16,
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
pub struct CacheEntry {
    pub key: CacheKey,
    pub value: u32,
    pub expires_at_jiffies: u64,
    pub next: *mut CacheEntry,
}

#[repr(C)]
pub struct CacheManager {
    pub head: *mut CacheEntry,
    pub max_entries: u32,
    pub entry_count: u32,
    pub stats: CacheStatistics,
    pub priv_data: *mut c_void,
}

pub type MatchFn = Option<unsafe extern "C" fn(key: *const CacheKey, ctx: *mut c_void) -> c_int>;
pub type HashFn = Option<unsafe extern "C" fn(key: *const CacheKey, seed: u32) -> u32>;
pub type CleanupFn = Option<unsafe extern "C" fn(ctx: *mut c_void)>;

#[repr(C)]
pub struct Fib6RuleOps {
    pub ctx: *mut c_void,
    pub match_fn: MatchFn,
    pub hash_fn: HashFn,
    pub cleanup_fn: CleanupFn,
}

#[no_mangle]
pub unsafe extern "C" fn cache_manager_init(
    mgr: *mut CacheManager,
    max_entries: u32,
    priv_data: *mut c_void,
) -> c_int {
    if mgr.is_null() {
        return -22;
    }

    (*mgr).head = core::ptr::null_mut();
    (*mgr).max_entries = max_entries;
    (*mgr).entry_count = 0;
    (*mgr).stats = CacheStatistics {
        lookups: 0,
        hits: 0,
        misses: 0,
        inserts: 0,
        evictions: 0,
    };
    (*mgr).priv_data = priv_data;
    0
}

#[no_mangle]
pub unsafe extern "C" fn cache_manager_reset_stats(mgr: *mut CacheManager) {
    if mgr.is_null() {
        return;
    }

    (*mgr).stats.lookups = 0;
    (*mgr).stats.hits = 0;
    (*mgr).stats.misses = 0;
    (*mgr).stats.inserts = 0;
    (*mgr).stats.evictions = 0;
}

#[no_mangle]
pub unsafe extern "C" fn cache_manager_get_stats(
    mgr: *const CacheManager,
    out_stats: *mut CacheStatistics,
) -> c_int {
    if mgr.is_null() || out_stats.is_null() {
        return -22;
    }

    *out_stats = CacheStatistics {
        lookups: (*mgr).stats.lookups,
        hits: (*mgr).stats.hits,
        misses: (*mgr).stats.misses,
        inserts: (*mgr).stats.inserts,
        evictions: (*mgr).stats.evictions,
    };
    0
}

#[no_mangle]
pub unsafe extern "C" fn fib6_rule_match(ops: *const Fib6RuleOps, key: *const CacheKey) -> c_int {
    if ops.is_null() || key.is_null() {
        return 0;
    }

    match (*ops).match_fn {
        Some(cb) => cb(key, (*ops).ctx),
        None => 0,
    }
}

#[no_mangle]
pub unsafe extern "C" fn fib6_rule_hash(
    ops: *const Fib6RuleOps,
    key: *const CacheKey,
    seed: u32,
) -> u32 {
    if ops.is_null() || key.is_null() {
        return 0;
    }

    match (*ops).hash_fn {
        Some(cb) => cb(key, seed),
        None => 0,
    }
}

#[no_mangle]
pub unsafe extern "C" fn fib6_rule_cleanup(ops: *mut Fib6RuleOps) {
    if ops.is_null() {
        return;
    }

    if let Some(cb) = (*ops).cleanup_fn {
        cb((*ops).ctx);
    }
    (*ops).ctx = core::ptr::null_mut();
}

#[no_mangle]
pub extern "C" fn rust_eh_personality() {}