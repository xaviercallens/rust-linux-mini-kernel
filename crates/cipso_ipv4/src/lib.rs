#![no_std]
#![no_main]
#![allow(non_camel_case_types)]

use core::ptr;
use kernel_types::*;

pub const CIPSO_V4_OPT_LEN_MAX: c_int = 40;
pub const CIPSO_V4_HDR_LEN: c_int = 6;
pub const CIPSO_V4_TAG_RBM_BLEN: c_int = 4;
pub const CIPSO_V4_TAG_ENUM_BLEN: c_int = 4;
pub const CIPSO_V4_TAG_RNG_BLEN: c_int = 4;
pub const CIPSO_V4_TAG_RNG_CAT_MAX: c_int = 8;
pub const CIPSO_V4_TAG_LOC_BLEN: c_int = 6;
pub const CIPSO_V4_CACHE_BUCKETBITS: c_int = 7;
pub const CIPSO_V4_CACHE_BUCKETS: c_int = 1 << CIPSO_V4_CACHE_BUCKETBITS;
pub const CIPSO_V4_CACHE_REORDERLIMIT: c_int = 10;
pub const CIPSO_V4_CACHE_ENABLED_DEFAULT: c_int = 1;
pub const CIPSO_V4_CACHE_BUCKETS_SIZE_DEFAULT: c_int = 10;

pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ENOENT: c_int = -2;

type size_t = usize;

#[repr(C)]
pub struct cipso_v4_map_cache_bkt {
    lock: *mut c_void,
    size: c_uint,
    list: *mut c_void,
}

#[repr(C)]
pub struct cipso_v4_map_cache_entry {
    hash: u32,
    key: *mut u8,
    key_len: size_t,
    lsm_data: *mut c_void,
    activity: u32,
    list: *mut c_void,
}

#[repr(C)]
pub struct netlbl_lsm_secattr {
    cache: *mut c_void,
    flags: c_uint,
    type_: c_uint,
}

static mut cipso_v4_cache: *mut cipso_v4_map_cache_bkt = ptr::null_mut();
static mut cipso_v4_cache_enabled: c_int = CIPSO_V4_CACHE_ENABLED_DEFAULT;
static mut cipso_v4_cache_bucketsize: c_int = CIPSO_V4_CACHE_BUCKETS_SIZE_DEFAULT;

unsafe extern "C" {
    fn netlbl_secattr_cache_free(cache: *mut c_void);
    fn kfree(ptr: *mut c_void);
    fn kmalloc(size: size_t, flags: c_int) -> *mut c_void;
    fn spin_lock_init(lock: *mut c_void);
    fn spin_lock_bh(lock: *mut c_void);
    fn spin_unlock_bh(lock: *mut c_void);
    fn INIT_LIST_HEAD(head: *mut c_void);
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo<'_>) -> ! {
    loop {
        core::hint::spin_loop();
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_eh_personality() {}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cipso_v4_cache_entry_free(entry: *mut cipso_v4_map_cache_entry) {
    if entry.is_null() {
        return;
    }

    if !(*entry).lsm_data.is_null() {
        netlbl_secattr_cache_free((*entry).lsm_data);
    }

    if !(*entry).key.is_null() {
        kfree((*entry).key as *mut c_void);
    }

    kfree(entry as *mut c_void);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cipso_v4_map_cache_hash(key: *const u8, key_len: c_uint) -> u32 {
    let mut hash: u32 = 0;
    let mut i: c_uint = 0;

    while i < key_len {
        let byte = *key.add(i as usize) as u32;
        hash = hash.wrapping_add(byte);
        hash = hash.wrapping_add(hash << 10);
        hash ^= hash >> 6;
        i += 1;
    }

    hash = hash.wrapping_add(hash << 3);
    hash ^= hash >> 11;
    hash = hash.wrapping_add(hash << 15);

    hash
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cipso_v4_cache_init() -> c_int {
    let buckets = CIPSO_V4_CACHE_BUCKETS as usize;
    let size = buckets * core::mem::size_of::<cipso_v4_map_cache_bkt>();

    let cache_ptr = kmalloc(size as size_t, 0);
    if cache_ptr.is_null() {
        return ENOMEM;
    }

    let cache = cache_ptr as *mut cipso_v4_map_cache_bkt;
    for i in 0..buckets {
        let bucket = &mut *cache.add(i);
        spin_lock_init(&mut bucket.lock as *mut _ as *mut c_void);
        bucket.size = 0;
        INIT_LIST_HEAD(&mut bucket.list as *mut _ as *mut c_void);
    }

    cipso_v4_cache = cache;
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cipso_v4_cache_invalidate() {
    if cipso_v4_cache.is_null() {
        return;
    }

    let buckets = CIPSO_V4_CACHE_BUCKETS as usize;
    let cache = cipso_v4_cache;

    for i in 0..buckets {
        let bucket = &mut *cache.add(i);
        spin_lock_bh(&mut bucket.lock as *mut _ as *mut c_void);

        bucket.size = 0;
        INIT_LIST_HEAD(&mut bucket.list as *mut _ as *mut c_void);

        spin_unlock_bh(&mut bucket.lock as *mut _ as *mut c_void);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cipso_v4_cache_free() {
    if !cipso_v4_cache.is_null() {
        kfree(cipso_v4_cache as *mut c_void);
        cipso_v4_cache = ptr::null_mut();
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cipso_v4_cache_set_enabled(enabled: c_int) {
    cipso_v4_cache_enabled = enabled;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cipso_v4_cache_get_enabled() -> c_int {
    cipso_v4_cache_enabled
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cipso_v4_cache_set_bucketsize(size: c_int) {
    cipso_v4_cache_bucketsize = size;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cipso_v4_cache_get_bucketsize() -> c_int {
    cipso_v4_cache_bucketsize
}