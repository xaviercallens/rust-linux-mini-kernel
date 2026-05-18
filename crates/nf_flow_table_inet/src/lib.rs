#![no_std]

use core::ffi::{c_int, c_void};
use core::panic::PanicInfo;
use kernel_types::*;

/// Kernel-compatible aliases.
pub type size_t = usize;
pub type c_size_t = usize;
pub type socklen_t = u32;

/// Opaque C handle placeholder.
#[repr(C)]
pub struct Opaque {
    _priv: [u8; 0],
}

/// Example cache key mapped as C-compatible layout.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CacheKey {
    pub src_ip: u32,
    pub dst_ip: u32,
    pub src_port: u16,
    pub dst_port: u16,
    pub proto: u8,
    pub _pad: [u8; 3],
}

/// Example cache entry for FFI boundary.
#[repr(C)]
pub struct CacheEntry {
    pub key: CacheKey,
    pub value: u64,
    pub next: *mut CacheEntry,
}

/// Rust-owned manager with raw-pointer FFI surface.
#[repr(C)]
pub struct CacheManager {
    head: *mut CacheEntry,
}

impl CacheManager {
    pub const fn new() -> Self {
        Self {
            head: core::ptr::null_mut(),
        }
    }

    unsafe fn find_mut(&mut self, key: &CacheKey) -> *mut CacheEntry {
        let mut cur = self.head;
        while !cur.is_null() {
            let c = unsafe { &mut *cur };
            if keys_equal(&c.key, key) {
                return cur;
            }
            cur = c.next;
        }
        core::ptr::null_mut()
    }
}

#[inline]
fn keys_equal(a: &CacheKey, b: &CacheKey) -> bool {
    a.src_ip == b.src_ip
        && a.dst_ip == b.dst_ip
        && a.src_port == b.src_port
        && a.dst_port == b.dst_port
        && a.proto == b.proto
}

#[no_mangle]
pub extern "C" fn cache_manager_init(mgr: *mut CacheManager) -> c_int {
    if mgr.is_null() {
        return -1;
    }
    unsafe {
        core::ptr::write(mgr, CacheManager::new());
    }
    0
}

#[no_mangle]
pub extern "C" fn cache_manager_put(
    mgr: *mut CacheManager,
    key: *const CacheKey,
    value: u64,
    alloc: extern "C" fn(size_t) -> *mut c_void,
) -> c_int {
    if mgr.is_null() || key.is_null() {
        return -1;
    }

    let mgr = unsafe { &mut *mgr };
    let key_ref = unsafe { &*key };

    let found = unsafe { mgr.find_mut(key_ref) };
    if !found.is_null() {
        unsafe {
            (*found).value = value;
        }
        return 0;
    }

    let node_ptr = alloc(core::mem::size_of::<CacheEntry>()) as *mut CacheEntry;
    if node_ptr.is_null() {
        return -12;
    }

    unsafe {
        core::ptr::write(
            node_ptr,
            CacheEntry {
                key: *key_ref,
                value,
                next: mgr.head,
            },
        );
        mgr.head = node_ptr;
    }

    0
}

#[no_mangle]
pub extern "C" fn cache_manager_get(
    mgr: *const CacheManager,
    key: *const CacheKey,
    out_value: *mut u64,
) -> c_int {
    if mgr.is_null() || key.is_null() || out_value.is_null() {
        return -1;
    }

    let mgr_ref = unsafe { &*mgr };
    let key_ref = unsafe { &*key };

    let mut cur = mgr_ref.head;
    while !cur.is_null() {
        let node = unsafe { &*cur };
        if keys_equal(&node.key, key_ref) {
            unsafe {
                *out_value = node.value;
            }
            return 0;
        }
        cur = node.next;
    }

    -2
}

#[no_mangle]
pub extern "C" fn cache_manager_remove(
    mgr: *mut CacheManager,
    key: *const CacheKey,
    free_fn: extern "C" fn(*mut c_void),
) -> c_int {
    if mgr.is_null() || key.is_null() {
        return -1;
    }

    let mgr_ref = unsafe { &mut *mgr };
    let key_ref = unsafe { &*key };

    let mut prev: *mut CacheEntry = core::ptr::null_mut();
    let mut cur = mgr_ref.head;

    while !cur.is_null() {
        let node = unsafe { &mut *cur };
        if keys_equal(&node.key, key_ref) {
            let next = node.next;
            if prev.is_null() {
                mgr_ref.head = next;
            } else {
                unsafe {
                    (*prev).next = next;
                }
            }
            free_fn(cur as *mut c_void);
            return 0;
        }
        prev = cur;
        cur = node.next;
    }

    -2
}

#[no_mangle]
pub extern "C" fn cache_manager_clear(
    mgr: *mut CacheManager,
    free_fn: extern "C" fn(*mut c_void),
) -> c_int {
    if mgr.is_null() {
        return -1;
    }

    let mgr_ref = unsafe { &mut *mgr };
    let mut cur = mgr_ref.head;
    mgr_ref.head = core::ptr::null_mut();

    while !cur.is_null() {
        let next = unsafe { (*cur).next };
        free_fn(cur as *mut c_void);
        cur = next;
    }

    0
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn rust_eh_personality() {}