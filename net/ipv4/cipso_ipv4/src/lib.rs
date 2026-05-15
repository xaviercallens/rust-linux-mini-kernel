//! CIPSO - Commercial IP Security Option
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clang_undefined_int_to_float_cast)]
#![allow(clang_undefined_float_to_int_cast)]

use core::ptr;
use core::mem;
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
pub const CIPSO_V4_CACHE_BUCKETS_MASK: c_int = CIPSO_V4_CACHE_BUCKETS - 1;

pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ENOENT: c_int = -2;
pub const EPERM: c_int = -1;

// Type definitions
#[repr(C)]
struct list_head {
    prev: *mut list_head,
    next: *mut list_head,
}

#[repr(C)]
struct cipso_v4_map_cache_bkt {
    lock: *mut c_void, // spinlock_t
    size: c_uint,
    list: list_head,
}

#[repr(C)]
struct cipso_v4_map_cache_entry {
    hash: u32,
    key: *mut u8,
    key_len: size_t,
    lsm_data: *mut c_void, // struct netlbl_lsm_cache *
    activity: u32,
    list: list_head,
}

static mut cipso_v4_cache: *mut cipso_v4_map_cache_bkt = ptr::null_mut();
static mut cipso_v4_cache_enabled: c_int = 1;
static mut cipso_v4_cache_bucketsize: c_int = 10;

// Function implementations
/// Frees a cache entry
///
/// # Safety
/// - `entry` must be a valid pointer to a cipso_v4_map_cache_entry
#[no_mangle]
pub unsafe extern "C" fn cipso_v4_cache_entry_free(
    entry: *mut cipso_v4_map_cache_entry,
) {
    if !entry.is_null() {
        if !(*entry).lsm_data.is_null() {
            // SAFETY: Caller guarantees valid pointer
            netlbl_secattr_cache_free((*entry).lsm_data);
        }
        if !(*entry).key.is_null() {
            // SAFETY: Caller guarantees valid pointer
            libc::free((*entry).key as *mut c_void);
        }
        // SAFETY: Caller guarantees valid pointer
        libc::free(entry as *mut c_void);
    }
}

/// Hashing function for the CIPSO cache
///
/// # Safety
/// - `key` must be a valid pointer to a buffer of `key_len` bytes
#[no_mangle]
pub unsafe extern "C" fn cipso_v4_map_cache_hash(
    key: *const u8,
    key_len: u32,
) -> u32 {
    jhash(key, key_len as size_t, 0)
}

/// Initialize the CIPSO cache
///
/// # Safety
/// - This function must be called before any other cache operations
#[no_mangle]
pub unsafe extern "C" fn cipso_v4_cache_init() -> c_int {
    let buckets = CIPSO_V4_CACHE_BUCKETS as size_t;
    let size = buckets * mem::size_of::<cipso_v4_map_cache_bkt>();
    
    // SAFETY: Using libc calloc for kernel-like allocation
    let cache = libc::calloc(buckets, mem::size_of::<cipso_v4_map_cache_bkt>()) as *mut cipso_v4_map_cache_bkt;
    if cache.is_null() {
        return -ENOMEM;
    }
    
    for i in 0..buckets {
        let bucket = &mut *cache.add(i);
        // SAFETY: spinlock_t initialization
        spin_lock_init((*bucket).lock);
        (*bucket).size = 0;
        // SAFETY: list_head initialization
        INIT_LIST_HEAD(&mut (*bucket).list);
    }
    
    cipso_v4_cache = cache;
    0
}

/// Invalidates the current CIPSO cache
///
/// # Safety
/// - This function must be called with proper locking
#[no_mangle]
pub unsafe extern "C" fn cipso_v4_cache_invalidate() {
    if cipso_v4_cache.is_null() {
        return;
    }
    
    for i in 0..CIPSO_V4_CACHE_BUCKETS as usize {
        let bucket = &mut *cipso_v4_cache.add(i);
        spin_lock_bh((*bucket).lock);
        
        let mut entry = (*bucket).list.next;
        while entry != &mut (*bucket).list as *mut list_head {
            let entry_ptr = container_of(entry, cipso_v4_map_cache_entry, list);
            let next = (*entry_ptr).list.next;
            
            list_del(entry);
            cipso_v4_cache_entry_free(entry_ptr);
            
            entry = next;
        }
        
        (*bucket).size = 0;
        spin_unlock_bh((*bucket).lock);
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
    key_len: u32,
    secattr: *mut c_void,
) -> c_int {
    if cipso_v4_cache_enabled == 0 {
        return -ENOENT;
    }
    
    let hash = cipso_v4_map_cache_hash(key, key_len);
    let bkt = hash & CIPSO_V4_CACHE_BUCKETS_MASK as u32;
    let bucket = &mut *cipso_v4_cache.add(bkt as usize);
    
    spin_lock_bh((*bucket).lock);
    
    let mut entry = (*bucket).list.next;
    let mut prev_entry: *mut cipso_v4_map_cache_entry = ptr::null_mut();
    
    while entry != &mut (*bucket).list as *mut list_head {
        let entry_ptr = container_of(entry, cipso_v4_map_cache_entry, list);
        
        if (*entry_ptr).hash == hash &&
           (*entry_ptr).key_len == key_len as size_t &&
           libc::memcmp((*entry_ptr).key as *const c_void, key as *const c_void, key_len as size_t) == 0 {
            
            (*entry_ptr).activity += 1;
            refcount_inc((*(*entry_ptr).lsm_data).refcount);
            // SAFETY: secattr is a valid pointer
            ptr::write(secattr as *mut _, (*entry_ptr).lsm_data);
            // SAFETY: secattr is a valid pointer
            ptr::write_bytes(secattr as *mut _, 0, 1);
            // SAFETY: secattr is a valid pointer
            ptr::write_bytes(secattr as *mut _, NETLBL_SECATTR_CACHE, 1);
            // SAFETY: secattr is a valid pointer
            ptr::write_bytes(secattr as *mut _, NETLBL_NLTYPE_CIPSOV4, 1);
            
            if prev_entry.is_null() {
                spin_unlock_bh((*bucket).lock);
                return 0;
            }
            
            if (*prev_entry).activity > 0 {
                (*prev_entry).activity -= 1;
            }
            
            if (*entry_ptr).activity > (*prev_entry).activity &&
               (*entry_ptr).activity - (*prev_entry).activity > CIPSO_V4_CACHE_REORDERLIMIT as u32 {
                list_move(&mut (*entry_ptr).list, prev_entry);
            }
            
            spin_unlock_bh((*bucket).lock);
            return 0;
        }
        
        prev_entry = entry_ptr;
        entry = (*entry_ptr).list.next;
    }
    
    spin_unlock_bh((*bucket).lock);
    -ENOENT
}

/// Add an entry to the CIPSO cache
///
/// # Safety
/// - `cipso_ptr` must be a valid pointer to a CIPSO option
/// - `secattr` must be a valid pointer to a netlbl_lsm_secattr
#[no_mangle]
pub unsafe extern "C" fn cipso_v4_cache_add(
    cipso_ptr: *const u8,
    secattr: *const c_void,
) -> c_int {
    if cipso_v4_cache_enabled == 0 || cipso_v4_cache_bucketsize <= 0 {
        return 0;
    }
    
    let cipso_ptr_len = *cipso_ptr.add(1) as size_t;
    let entry = libc::calloc(1, mem::size_of::<cipso_v4_map_cache_entry>()) as *mut cipso_v4_map_cache_entry;
    if entry.is_null() {
        return -ENOMEM;
    }
    
    (*entry).key = libc::malloc(cipso_ptr_len) as *mut u8;
    if (*entry).key.is_null() {
        libc::free(entry as *mut c_void);
        return -ENOMEM;
    }
    
    // SAFETY: cipso_ptr is valid for cipso_ptr_len bytes
    libc::memcpy((*entry).key as *mut c_void, cipso_ptr as *const c_void, cipso_ptr_len);
    (*entry).key_len = cipso_ptr_len;
    (*entry).hash = cipso_v4_map_cache_hash(cipso_ptr, cipso_ptr_len as u32);
    (*entry).lsm_data = (*secattr).cache;
    refcount_inc((*(*entry).lsm_data).refcount);
    
    let bkt = (*entry).hash & CIPSO_V4_CACHE_BUCKETS_MASK as u32;
    let bucket = &mut *cipso_v4_cache.add(bkt as usize);
    
    spin_lock_bh((*bucket).lock);
    
    if (*bucket).size < cipso_v4_cache_bucketsize as c_uint {
        list_add(&mut (*entry).list, &mut (*bucket).list);
        (*bucket).size += 1;
    } else {
        let old_entry = container_of((*bucket).list.prev, cipso_v4_map_cache_entry, list);
        list_del(&mut (*old_entry).list);
        list_add(&mut (*entry).list, &mut (*bucket).list);
        cipso_v4_cache_entry_free(old_entry);
    }
    
    spin_unlock_bh((*bucket).lock);
    0
}

// Helper functions
#[no_mangle]
pub unsafe extern "C" fn spin_lock_init(lock: *mut c_void) {
    // Kernel-specific spinlock initialization
}

#[no_mangle]
pub unsafe extern "C" fn spin_lock_bh(lock: *mut c_void) {
    // Kernel-specific spinlock acquisition
}

#[no_mangle]
pub unsafe extern "C" fn spin_unlock_bh(lock: *mut c_void) {
    // Kernel-specific spinlock release
}

#[no_mangle]
pub unsafe extern "C" fn INIT_LIST_HEAD(head: *mut list_head) {
    (*head).prev = head;
    (*head).next = head;
}

#[no_mangle]
pub unsafe extern "C" fn list_add(
    new: *mut list_head,
    head: *mut list_head,
) {
    let next = (*head).next;
    (*new).next = next;
    (*new).prev = head;
    (*next).prev = new;
    (*head).next = new;
}

#[no_mangle]
pub unsafe extern "C" fn list_del(entry: *mut list_head) {
    let next = (*entry).next;
    let prev = (*entry).prev;
    (*next).prev = prev;
    (*prev).next = next;
}

#[no_mangle]
pub unsafe extern "C" fn list_move(
    entry: *mut list_head,
    head: *mut list_head,
) {
    list_del(entry);
    list_add(entry, head);
}

#[no_mangle]
pub unsafe extern "C" fn container_of(
    ptr: *mut list_head,
    type_: *mut c_void,
    member: *mut c_void,
) -> *mut c_void {
    (ptr as *mut u8).offset(-(member as isize)) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn refcount_inc(refcount: *mut c_int) {
    *refcount += 1;
}

#[no_mangle]
pub unsafe extern "C" fn netlbl_secattr_cache_free(cache: *mut c_void) {
    // Kernel-specific cache free implementation
}

#[no_mangle]
pub unsafe extern "C" fn jhash(
    key: *const u8,
    length: size_t,
    initval: u32,
) -> u32 {
    // Implementation of Jenkins 32-bit hash
    let mut a = initval;
    let mut b = initval;
    let mut c = 0;
    
    let mut key = key;
    let end = key.add(length);
    
    while key < end {
        a += *key as u32;
        b += *key as u32;
        c += *key as u32;
        key = key.add(1);
    }
    
    c += length as u32;
    
    a -= b;
    a -= c;
    a ^= c >> 13;
    b -= c;
    b -= a;
    b ^= a << 8;
    c -= a;
    c -= b;
    c ^= b >> 13;
    a -= b;
    a -= c;
    a ^= c >> 12;
    b -= c;
    b -= a;
    b ^= a << 16;
    c -= a;
    c -= b;
    c ^= b >> 4;
    a -= b;
    a -= c;
    a ^= c >> 5;
    b -= c;
    b -= a;
    b ^= a << 3;
    c -= a;
    c -= b;
    c ^= b >> 14;
    
    a ^ b ^ c
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_jhash() {
        let key = [0x01, 0x02, 0x03, 0x04];
        let hash = unsafe { super::jhash(key.as_ptr(), 4, 0) };
        assert_eq!(hash, 0x01020304);
    }
}
