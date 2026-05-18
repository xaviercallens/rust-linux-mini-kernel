#![no_std]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::panic::PanicInfo;
use core::ptr;
use core::sync::atomic::{AtomicU32, Ordering};
use kernel_types::*;

pub type size_t = usize;
pub type c_size_t = usize;
pub type socklen_t = u32;

#[repr(C)]
pub struct nf_conn {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct net {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hlist_nulls_head {
    pub first: *mut c_void,
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nf_conntrack_tuple {
    pub src_l3num: u16,
    pub src_protonum: u8,
    pub src_pad: u8,
    pub src_addr: [u8; 16],
    pub src_port: u16,
    pub dst_addr: [u8; 16],
    pub dst_port: u16,
    pub dir: u8,
    pub pad: [u8; 5],
}

#[repr(C)]
pub struct CacheKey {
    pub net: *mut net,
    pub hash: c_uint,
    pub zone: u16,
    pub protonum: u8,
    pub l3num: u8,
}

#[repr(C)]
pub struct CacheStatistics {
    pub searched: AtomicU32,
    pub found: AtomicU32,
    pub insert: AtomicU32,
    pub insert_failed: AtomicU32,
    pub drop: AtomicU32,
    pub early_drop: AtomicU32,
    pub error: AtomicU32,
    pub search_restart: AtomicU32,
}

#[repr(C)]
pub struct nf_conntrack_locks {
    pub locks: *mut spinlock_t,
    pub count: c_uint,
}

#[repr(C)]
pub struct nf_conntrack_hashtable {
    pub buckets: *mut hlist_nulls_head,
    pub size: c_uint,
}

unsafe extern "C" {
    fn spin_lock_bh(lock: *mut spinlock_t);
    fn spin_unlock_bh(lock: *mut spinlock_t);
    fn jhash2(k: *const u32, length: c_uint, initval: c_uint) -> c_uint;
}

#[unsafe(no_mangle)]
pub static mut nf_conntrack_htable: nf_conntrack_hashtable = nf_conntrack_hashtable {
    buckets: ptr::null_mut(),
    size: 0,
};

#[unsafe(no_mangle)]
pub static mut nf_conntrack_locks_all: nf_conntrack_locks = nf_conntrack_locks {
    locks: ptr::null_mut(),
    count: 0,
};

#[unsafe(no_mangle)]
pub static nf_conntrack_stat: CacheStatistics = CacheStatistics {
    searched: AtomicU32::new(0),
    found: AtomicU32::new(0),
    insert: AtomicU32::new(0),
    insert_failed: AtomicU32::new(0),
    drop: AtomicU32::new(0),
    early_drop: AtomicU32::new(0),
    error: AtomicU32::new(0),
    search_restart: AtomicU32::new(0),
};

#[inline]
unsafe fn lock_for_hash(hash: c_uint) -> *mut spinlock_t {
    let count = unsafe { nf_conntrack_locks_all.count };
    let locks = unsafe { nf_conntrack_locks_all.locks };
    if count == 0 || locks.is_null() {
        return ptr::null_mut();
    }
    unsafe { locks.add((hash % count) as usize) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nf_conntrack_core_hash(
    tuple: *const nf_conntrack_tuple,
    zone: u16,
    size: c_uint,
) -> c_uint {
    if tuple.is_null() || size == 0 {
        return 0;
    }

    let words = tuple as *const u32;
    let len_words =
        (core::mem::size_of::<nf_conntrack_tuple>() / core::mem::size_of::<u32>()) as c_uint;
    let init = (zone as c_uint) << 16;
    unsafe { jhash2(words, len_words, init) % size }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nf_conntrack_core_find_get(
    _net: *mut net,
    tuple: *const nf_conntrack_tuple,
    zone: u16,
) -> *mut nf_conn {
    if tuple.is_null() || unsafe { nf_conntrack_htable.size == 0 } {
        return ptr::null_mut();
    }

    let size = unsafe { nf_conntrack_htable.size };
    let hash = unsafe { nf_conntrack_core_hash(tuple, zone, size) };
    let lock = unsafe { lock_for_hash(hash) };

    if !lock.is_null() {
        unsafe { spin_lock_bh(lock) };
    }

    nf_conntrack_stat.searched.fetch_add(1, Ordering::Relaxed);

    if !lock.is_null() {
        unsafe { spin_unlock_bh(lock) };
    }

    ptr::null_mut()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nf_conntrack_core_insert(
    _net: *mut net,
    _ct: *mut nf_conn,
    tuple: *const nf_conntrack_tuple,
    zone: u16,
) -> c_int {
    if tuple.is_null() || unsafe { nf_conntrack_htable.size == 0 } {
        nf_conntrack_stat
            .insert_failed
            .fetch_add(1, Ordering::Relaxed);
        return -22;
    }

    let size = unsafe { nf_conntrack_htable.size };
    let hash = unsafe { nf_conntrack_core_hash(tuple, zone, size) };
    let lock = unsafe { lock_for_hash(hash) };

    if !lock.is_null() {
        unsafe { spin_lock_bh(lock) };
    }

    nf_conntrack_stat.insert.fetch_add(1, Ordering::Relaxed);

    if !lock.is_null() {
        unsafe { spin_unlock_bh(lock) };
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nf_conntrack_core_destroy() {
    unsafe {
        nf_conntrack_htable.buckets = ptr::null_mut();
        nf_conntrack_htable.size = 0;
        nf_conntrack_locks_all.locks = ptr::null_mut();
        nf_conntrack_locks_all.count = 0;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nf_conntrack_core_set_label(
    _name: *const c_char,
    _value: c_uint,
) -> c_int {
    0
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    loop {}
}