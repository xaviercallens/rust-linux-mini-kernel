//! CIPSO - Commercial IP Security Option
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)] // For C-style type names

use core::ptr;
use libc::{c_int, c_uint, c_void, size_t};

// Constants from C
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

// Error codes
pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ENOENT: c_int = -2;

// Type definitions
#[repr(C)]
struct cipso_v4_map_cache_bkt {
    lock: *mut c_void, // spinlock_t
    size: c_uint,
    list: *mut c_void, // struct list_head
}

#[repr(C)]
struct cipso_v4_map_cache_entry {
    hash: u32,
    key: *mut u8,
    key_len: size_t,
    lsm_data: *mut c_void, // struct netlbl_lsm_cache
    activity: u32,
    list: *mut c_void, // struct list_head
}

#[repr(C)]
struct netlbl_lsm_secattr {
    cache: *mut c_void, // struct netlbl_lsm_cache
    flags: c_uint,
    type_: c_uint,
}

// Global variables
static mut cipso_v4_cache: *mut cipso_v4_map_cache_bkt = ptr::null_mut();
static mut cipso_v4_cache_enabled: c_int = CIPSO_V4_CACHE_ENABLED_DEFAULT;
static mut cipso_v4_cache_bucketsize: c_int = CIPSO_V4_CACHE_BUCKETS_SIZE_DEFAULT;

// Function implementations
/// Frees a cache entry
///
/// # Safety
/// - `entry` must be a valid pointer to a cipso_v4_map_cache_entry
#[no_mangle]
pub unsafe extern "C" fn cipso_v4_cache_entry_free(entry: *mut cipso_v4_map_cache_entry) {
    if entry.is_null() {
        return;
    }

    // SAFETY: entry is valid (checked above)
    if !(*entry).lsm_data.is_null() {
        // Call netlbl_secattr_cache_free
        extern "C" {
            fn netlbl_secattr_cache_free(cache: *mut c_void);
        }
        netlbl_secattr_cache_free((*entry).lsm_data);
    }

    // SAFETY: entry->key is valid if not null
    if !(*entry).key.is_null() {
        // Call kfree
        extern "C" {
            fn kfree(ptr: *mut c_void);
        }
        kfree((*entry).key as *mut c_void);
    }

    // SAFETY: Free the entry itself
    kfree(entry as *mut c_void);
}

/// Hashing function for the CIPSO cache
///
/// # Safety
/// - `key` must be a valid pointer to a buffer of `key_len` bytes
#[no_mangle]
pub unsafe extern "C" fn cipso_v4_map_cache_hash(key: *const u8, key_len: c_uint) -> u32 {
    // Implement Jenkins one-at-a-time hash
    let mut hash: u32 = 0;
    let mut i: c_uint = 0;

    while i < key_len {
        // SAFETY: key is valid for key_len bytes (caller guarantee)
        let byte = *key.add(i as usize) as u32;
        hash += byte;
        hash += hash << 10;
        hash ^= hash >> 6;
        i += 1;
    }

    hash += hash << 3;
    hash ^= hash >> 11;
    hash += hash << 15;

    hash
}

/// Initialize the CIPSO cache
///
/// # Safety
/// - Must be called before any other cache functions
#[no_mangle]
pub unsafe extern "C" fn cipso_v4_cache_init() -> c_int {
    let buckets = CIPSO_V4_CACHE_BUCKETS as usize;
    let size = buckets * core::mem::size_of::<cipso_v4_map_cache_bkt>();

    // SAFETY: Allocate memory for the cache
    extern "C" {
        fn kmalloc(size: size_t, flags: c_int) -> *mut c_void;
    }
    let cache_ptr = kmalloc(size as size_t, 0);
    if cache_ptr.is_null() {
        return ENOMEM;
    }

    // SAFETY: Initialize each bucket
    let cache = cache_ptr as *mut cipso_v4_map_cache_bkt;
    for i in 0..buckets {
        // Initialize spinlock
        let bucket = &mut *cache.add(i);
        extern "C" {
            fn spin_lock_init(lock: *mut c_void);
        }
        spin_lock_init(&mut bucket.lock as *mut _ as *mut c_void);

        bucket.size = 0;
        // Initialize list_head (empty list)
        extern "C" {
            fn INIT_LIST_HEAD(head: *mut c_void);
        }
        INIT_LIST_HEAD(&mut bucket.list as *mut _ as *mut c_void);
    }

    cipso_v4_cache = cache;
    0
}

/// Invalidates the current CIPSO cache
///
/// # Safety
/// - No concurrent access to the cache
#[no_mangle]
pub unsafe extern "C" fn cipso_v4_cache_invalidate() {
    if cipso_v4_cache.is_null() {
        return;
    }

    let buckets = CIPSO_V4_CACHE_BUCKETS as usize;
    let cache = cipso_v4_cache as *mut cipso_v4_map_cache_bkt;

    for i in 0..buckets {
        let bucket = &mut *cache.add(i);
        // Acquire spinlock
        extern "C" {
            fn spin_lock_bh(lock: *mut c_void);
        }
        spin_lock_bh(&mut bucket.lock as *mut _ as *mut c_void);

        // Iterate through entries
        let mut entry = bucket.list;
        while !entry.is_null() {
            // Get next entry before freeing
            let next = {
                // SAFETY: entry is valid
                let entry_ptr = entry as *mut cipso_v4_map_cache_entry;
                (*entry_ptr).list
            };

            // Remove from list
            extern "C" {
                fn list_del(entry: *mut c_void);
            }
            list_del(entry);

            // Free entry
            let entry_ptr = entry as *mut cipso_v4_map_cache_entry;
            cipso_v4_cache_entry_free(entry_ptr);

            entry = next;
        }

        bucket.size = 0;
        // Release spinlock
        extern "C" {
            fn spin_unlock_bh(lock: *mut c_void);
        }
        spin_unlock_bh(&mut bucket.lock as *mut _ as *mut c_void);
    }
}

/// Check the CIPSO cache for a label mapping
///
/// # Safety
/// - `key` must be a valid pointer to a buffer of `key_len` bytes
/// - `secattr` must be a valid pointer to a netlbl_lsm_secattr
#[no_mangle]
pub unsafe extern "C" fn cipso_v4_cache_check(
    key: *const u8,
    key_len: c_uint,
    secattr: *mut netlbl_lsm_secattr,
) -> c_int {
    if cipso_v4_cache.is_null() || key.is_null() || secattr.is_null() {
        return ENOENT;
    }

    if cipso_v4_cache_enabled == 0 {
        return ENOENT;
    }

    let hash = cipso_v4_map_cache_hash(key, key_len);
    let bkt = hash & (CIPSO_V4_CACHE_BUCKETS - 1) as u32;
    let cache = cipso_v4_cache as *mut cipso_v4_map_cache_bkt;
    let bucket = &mut *cache.add(bkt as usize);

    // Acquire spinlock
    extern "C" {
        fn spin_lock_bh(lock: *mut c_void);
    }
    spin_lock_bh(&mut bucket.lock as *mut _ as *mut c_void);

    let mut entry = bucket.list;
    let mut prev_entry: *mut cipso_v4_map_cache_entry = ptr::null_mut();

    while !entry.is_null() {
        // SAFETY: entry is valid
        let entry_ptr = entry as *mut cipso_v4_map_cache_entry;
        if (*entry_ptr).hash == hash && (*entry_ptr).key_len == key_len as usize {
            // Check key content
            let mut match_found = true;
            for i in 0..key_len {
                // SAFETY: key and entry->key are valid for key_len bytes
                if *key.add(i as usize) != *(*entry_ptr).key.add(i as usize) {
                    match_found = false;
                    break;
                }
            }

            if match_found {
                // Update activity
                (*entry_ptr).activity += 1;

                // Update secattr
                (*secattr).cache = (*entry_ptr).lsm_data;
                (*secattr).flags |= 1; // NETLBL_SECATTR_CACHE
                (*secattr).type_ = 2; // NETLBL_NLTYPE_CIPSOV4

                // Release spinlock
                extern "C" {
                    fn spin_unlock_bh(lock: *mut c_void);
                }
                spin_unlock_bh(&mut bucket.lock as *mut _ as *mut c_void);
                return 0;
            }
        }

        prev_entry = entry;
        entry = {
            // SAFETY: entry is valid
            let entry_ptr = entry as *mut cipso_v4_map_cache_entry;
            (*entry_ptr).list
        };
    }

    // Release spinlock
    extern "C" {
        fn spin_unlock_bh(lock: *mut c_void);
    }
    spin_unlock_bh(&mut bucket.lock as *mut _ as *mut c_void);

    ENOENT
}

/// Add an entry to the CIPSO cache
///
/// # Safety
/// - `cipso_ptr` must be a valid pointer to a CIPSO option
/// - `secattr` must be a valid pointer to a netlbl_lsm_secattr
#[no_mangle]
pub unsafe extern "C" fn cipso_v4_cache_add(
    cipso_ptr: *const u8,
    secattr: *const netlbl_lsm_secattr,
) -> c_int {
    if cipso_v4_cache.is_null() || cipso_ptr.is_null() || secattr.is_null() {
        return 0;
    }

    if cipso_v4_cache_enabled == 0 || cipso_v4_cache_bucketsize <= 0 {
        return 0;
    }

    let cipso_ptr_len = *cipso_ptr.add(1) as c_uint;
    if cipso_ptr_len < CIPSO_V4_HDR_LEN as c_uint {
        return EINVAL;
    }

    // Allocate new entry
    extern "C" {
        fn kmalloc(size: size_t, flags: c_int) -> *mut c_void;
    }
    let entry_ptr = kmalloc(
        core::mem::size_of::<cipso_v4_map_cache_entry>() as size_t,
        0,
    );
    if entry_ptr.is_null() {
        return ENOMEM;
    }

    let entry = entry_ptr as *mut cipso_v4_map_cache_entry;
    (*entry).key = kmalloc(cipso_ptr_len as size_t, 0) as *mut u8;
    if (*entry).key.is_null() {
        kfree(entry as *mut c_void);
        return ENOMEM;
    }

    // Copy key
    for i in 0..cipso_ptr_len {
        // SAFETY: cipso_ptr and entry->key are valid for cipso_ptr_len bytes
        *(*entry).key.add(i as usize) = *cipso_ptr.add(i as usize);
    }

    (*entry).key_len = cipso_ptr_len as size_t;
    (*entry).hash = cipso_v4_map_cache_hash(cipso_ptr, cipso_ptr_len);
    (*entry).lsm_data = (*secattr).cache;
    (*entry).activity = 0;

    let bkt = (*entry).hash & (CIPSO_V4_CACHE_BUCKETS - 1) as u32;
    let cache = cipso_v4_cache as *mut cipso_v4_map_cache_bkt;
    let bucket = &mut *cache.add(bkt as usize);

    // Acquire spinlock
    extern "C" {
        fn spin_lock_bh(lock: *mut c_void);
    }
    spin_lock_bh(&mut bucket.lock as *mut _ as *mut c_void);

    if bucket.size < cipso_v4_cache_bucketsize as c_uint {
        // Add to head
        extern "C" {
            fn list_add(new_entry: *mut c_void, head: *mut c_void);
        }
        list_add(entry as *mut c_void, bucket.list);
        bucket.size += 1;
    } else {
        // Remove last entry
        let old_entry = {
            // SAFETY: bucket->list is valid
            let list = bucket.list;
            let entry = list as *mut cipso_v4_map_cache_entry;
            (*entry).list
        };

        // Remove old entry
        extern "C" {
            fn list_del(entry: *mut c_void);
        }
        list_del(old_entry);

        // Add new entry
        extern "C" {
            fn list_add(new_entry: *mut c_void, head: *mut c_void);
        }
        list_add(entry as *mut c_void, bucket.list);
    }

    // Release spinlock
    extern "C" {
        fn spin_unlock_bh(lock: *mut c_void);
    }
    spin_unlock_bh(&mut bucket.lock as *mut _ as *mut c_void);

    0
}

// Tests (conditional compilation)
#[cfg(test)]
mod tests {
    use super::*;
    use core::ptr;

    #[test]
    fn test_cache_init() {
        unsafe {
            let result = cipso_v4_cache_init();
            assert_eq!(result, 0);
            cipso_v4_cache_invalidate();
        }
    }

    #[test]
    fn test_hash_function() {
        let key: [u8; 4] = [0x01, 0x02, 0x03, 0x04];
        let key_ptr = key.as_ptr();
        let hash = unsafe { cipso_v4_map_cache_hash(key_ptr, 4) };
        assert!(hash != 0);
    }
}
