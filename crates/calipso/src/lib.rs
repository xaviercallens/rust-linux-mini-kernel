//! CALIPSO - Common Architecture Label IPv6 Security Option
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)] // For C-style type names

use core::ptr;
use libc::{c_int, c_uint, c_void, size_t};

// Constants from C
pub const CALIPSO_OPT_LEN_MAX: c_int = 2 + 252;
pub const CALIPSO_HDR_LEN: c_int = 2 + 8;
pub const CALIPSO_OPT_LEN_MAX_WITH_PAD: c_int = 3 + CALIPSO_OPT_LEN_MAX + 7;
pub const CALIPSO_MAX_BUFFER: c_int = 6 + CALIPSO_OPT_LEN_MAX;
pub const CALIPSO_CACHE_BUCKETBITS: c_int = 7;
pub const CALIPSO_CACHE_BUCKETS: c_int = 1 << CALIPSO_CACHE_BUCKETBITS;
pub const CALIPSO_CACHE_REORDERLIMIT: c_int = 10;

// Error codes
pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ENOENT: c_int = -2;

// Type definitions
#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct spinlock_t {
    raw: c_void,
}

#[repr(C)]
pub struct calipso_map_cache_bkt {
    lock: spinlock_t,
    size: c_uint,
    list: list_head,
}

#[repr(C)]
pub struct calipso_map_cache_entry {
    hash: c_uint,
    key: *mut u8,
    key_len: size_t,
    lsm_data: *mut c_void,
    activity: c_uint,
    list: list_head,
}

#[repr(C)]
pub struct netlbl_lsm_secattr {
    cache: *mut c_void,
    flags: c_uint,
    type_: c_uint,
}

// Function implementations
/// Frees a cache entry
///
/// # Safety
/// - `entry` must be a valid pointer to a calipso_map_cache_entry
#[no_mangle]
pub unsafe extern "C" fn calipso_cache_entry_free(entry: *mut calipso_map_cache_entry) {
    if !entry.is_null() {
        if !(*entry).lsm_data.is_null() {
            netlbl_secattr_cache_free((*entry).lsm_data);
        }
        if !(*entry).key.is_null() {
            libc::free((*entry).key as *mut c_void);
        }
        libc::free(entry as *mut c_void);
    }
}

/// Hashing function for the CALIPSO cache
///
/// # Safety
/// - `key` must be a valid pointer to a buffer of `key_len` bytes
#[no_mangle]
pub unsafe extern "C" fn calipso_map_cache_hash(key: *const u8, key_len: c_uint) -> c_uint {
    jhash(key, key_len, 0)
}

/// Initialize the CALIPSO cache
///
/// # Safety
/// - Must be called before any other cache operations
#[no_mangle]
pub unsafe extern "C" fn calipso_cache_init() -> c_int {
    let cache = libc::calloc(
        CALIPSO_CACHE_BUCKETS as size_t,
        core::mem::size_of::<calipso_map_cache_bkt>() as size_t,
    ) as *mut calipso_map_cache_bkt;

    if cache.is_null() {
        return ENOMEM;
    }

    for i in 0..CALIPSO_CACHE_BUCKETS {
        spin_lock_init(&(*cache).lock);
        (*cache).size = 0;
        (*cache).list.next = &mut (*cache).list;
        (*cache).list.prev = &mut (*cache).list;
        cache = cache.offset(1);
    }

    0
}

/// Invalidates the current CALIPSO cache
///
/// # Safety
/// - Must be called with proper locking context
#[no_mangle]
pub unsafe extern "C" fn calipso_cache_invalidate() {
    let mut cache = calipso_cache as *mut calipso_map_cache_bkt;

    for _ in 0..CALIPSO_CACHE_BUCKETS {
        spin_lock_bh(&(*cache).lock);
        let mut entry = (*cache).list.next;
        while entry != &mut (*cache).list {
            let entry_ptr = (entry as *mut list_head).offset_from(&(*cache).list)
                as *mut calipso_map_cache_entry;
            let next = (*entry).next;

            list_del(entry);
            calipso_cache_entry_free(entry_ptr);

            entry = next;
        }
        (*cache).size = 0;
        spin_unlock_bh(&(*cache).lock);
        cache = cache.offset(1);
    }
}

/// Check the CALIPSO cache for a label mapping
///
/// # Safety
/// - `key` must be a valid pointer to a buffer of `key_len` bytes
/// - `secattr` must be a valid pointer to a netlbl_lsm_secattr
#[no_mangle]
pub unsafe extern "C" fn calipso_cache_check(
    key: *const u8,
    key_len: c_uint,
    secattr: *mut netlbl_lsm_secattr,
) -> c_int {
    if !calipso_cache_enabled() {
        return ENOENT;
    }

    let hash = calipso_map_cache_hash(key, key_len);
    let bkt = hash & (CALIPSO_CACHE_BUCKETS - 1) as c_uint;
    let cache = calipso_cache.offset(bkt as isize);

    spin_lock_bh(&(*cache).lock);

    let mut entry = (*cache).list.next;
    let mut prev_entry: *mut calipso_map_cache_entry = ptr::null_mut();

    while entry != &mut (*cache).list {
        let entry_ptr =
            (entry as *mut list_head).offset_from(&(*cache).list) as *mut calipso_map_cache_entry;

        if (*entry_ptr).hash == hash
            && (*entry_ptr).key_len == key_len
            && ptr::read_unaligned(key).eq_slice((*entry_ptr).key, key_len as usize)
        {
            (*entry_ptr).activity += 1;
            refcount_inc(&(*(*entry_ptr).lsm_data).refcount);
            (*secattr).cache = (*entry_ptr).lsm_data;
            (*secattr).flags |= 1; // NETLBL_SECATTR_CACHE
            (*secattr).type_ = 2; // NETLBL_NLTYPE_CALIPSO

            if prev_entry.is_null() {
                spin_unlock_bh(&(*cache).lock);
                return 0;
            }

            if (*prev_entry).activity > 0 {
                (*prev_entry).activity -= 1;
            }

            if (*entry_ptr).activity > (*prev_entry).activity
                && (*entry_ptr).activity - (*prev_entry).activity
                    > CALIPSO_CACHE_REORDERLIMIT as c_uint
            {
                list_move(entry, prev_entry.offset(-1) as *mut list_head);
            }

            spin_unlock_bh(&(*cache).lock);
            return 0;
        }

        prev_entry = entry_ptr;
        entry = (*entry).next;
    }

    spin_unlock_bh(&(*cache).lock);
    -ENOENT
}

/// Add an entry to the CALIPSO cache
///
/// # Safety
/// - `calipso_ptr` must be a valid pointer to a CALIPSO option
/// - `secattr` must be a valid pointer to a netlbl_lsm_secattr
#[no_mangle]
pub unsafe extern "C" fn calipso_cache_add(
    calipso_ptr: *const u8,
    secattr: *const netlbl_lsm_secattr,
) -> c_int {
    if !calipso_cache_enabled() || calipso_cache_bucketsize() <= 0 {
        return 0;
    }

    let calipso_ptr_len = *calipso_ptr.add(1) as c_uint;
    let entry = libc::calloc(1, core::mem::size_of::<calipso_map_cache_entry>())
        as *mut calipso_map_cache_entry;

    if entry.is_null() {
        return ENOMEM;
    }

    (*entry).key = libc::malloc(calipso_ptr_len as size_t) as *mut u8;
    if (*entry).key.is_null() {
        libc::free(entry as *mut c_void);
        return ENOMEM;
    }

    ptr::copy_nonoverlapping(
        calipso_ptr.offset(2),
        (*entry).key,
        calipso_ptr_len as usize,
    );
    (*entry).key_len = calipso_ptr_len;
    (*entry).hash = calipso_map_cache_hash(calipso_ptr, calipso_ptr_len);
    refcount_inc(&(*(*secattr).cache).refcount);
    (*entry).lsm_data = (*secattr).cache;

    let bkt = (*entry).hash & (CALIPSO_CACHE_BUCKETS - 1) as c_uint;
    let cache = calipso_cache.offset(bkt as isize);

    spin_lock_bh(&(*cache).lock);

    if (*cache).size < calipso_cache_bucketsize() {
        list_add(&(*entry).list, &(*cache).list);
        (*cache).size += 1;
    } else {
        let old_entry = (list_last(&(*cache).list) as *mut list_head).offset_from(&(*cache).list)
            as *mut calipso_map_cache_entry;
        list_del(list_last(&(*cache).list));
        calipso_cache_entry_free(old_entry);
        list_add(&(*entry).list, &(*cache).list);
    }

    spin_unlock_bh(&(*cache).lock);

    0
}

// Helper functions (assumed to be available via FFI)
#[link(name = "kernel")]
extern "C" {
    fn spin_lock_init(lock: *mut spinlock_t);
    fn spin_lock_bh(lock: *mut spinlock_t);
    fn spin_unlock_bh(lock: *mut spinlock_t);
    fn list_add(new: *mut list_head, head: *mut list_head);
    fn list_del(entry: *mut list_head);
    fn list_last(head: *mut list_head) -> *mut list_head;
    fn refcount_inc(refcount: *mut c_int);
    fn netlbl_secattr_cache_free(data: *mut c_void);
    fn jhash(key: *const u8, key_len: c_uint, initval: c_uint) -> c_uint;
}

// Global variables (simulated with unsafe statics)
static mut calipso_cache: *mut calipso_map_cache_bkt = ptr::null_mut();
static mut calipso_cache_enabled: c_int = 1;
static mut calipso_cache_bucketsize: c_int = 10;

// Simulated global functions
unsafe fn calipso_cache_enabled() -> bool {
    calipso_cache_enabled != 0
}

unsafe fn calipso_cache_bucketsize() -> c_int {
    calipso_cache_bucketsize
}

// Tests (conditional compilation)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_init() {
        unsafe {
            let result = calipso_cache_init();
            assert_eq!(result, 0);
        }
    }
}
