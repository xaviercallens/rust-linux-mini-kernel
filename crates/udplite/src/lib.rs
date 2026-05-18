#![no_std]
#![cfg_attr(not(test), no_main)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::panic::PanicInfo;

pub mod kernel_types {
    pub type size_t = usize;
    pub type c_size_t = usize;
    pub type socklen_t = u32;
}

use kernel_types::*;

#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    loop {}
}

/// Required for `no_std` builds to avoid unwinding support.
#[cfg(not(test))]
#[unsafe(no_mangle)]
pub extern "C" fn rust_eh_personality() {}

/// Opaque C-visible handle type.
#[repr(C)]
pub struct CacheManager {
    _private: [u8; 0],
}

#[repr(C)]
pub struct CacheKey {
    pub src_ip: u32,
    pub dst_ip: u32,
    pub src_port: u16,
    pub dst_port: u16,
    pub proto: u8,
    pub _pad: [u8; 3],
}

#[repr(C)]
pub struct CacheStatistics {
    pub hits: u64,
    pub misses: u64,
    pub inserts: u64,
    pub evictions: u64,
}

unsafe extern "C" {
    fn kmalloc(size: size_t, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *const c_void);
}

const GFP_KERNEL: c_uint = 0x10;

#[unsafe(no_mangle)]
pub extern "C" fn udplite_cache_manager_create() -> *mut CacheManager {
    let ptr = unsafe { kmalloc(core::mem::size_of::<u8>(), GFP_KERNEL) };
    ptr.cast::<CacheManager>()
}

#[unsafe(no_mangle)]
pub extern "C" fn udplite_cache_manager_destroy(manager: *mut CacheManager) {
    if !manager.is_null() {
        unsafe { kfree(manager.cast::<c_void>()) };
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn udplite_cache_insert(
    _manager: *mut CacheManager,
    _key: *const CacheKey,
    _value: *const c_void,
    _value_len: c_size_t,
) -> c_int {
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn udplite_cache_lookup(
    _manager: *mut CacheManager,
    _key: *const CacheKey,
    _out_value: *mut *mut c_void,
    _out_value_len: *mut c_size_t,
) -> c_int {
    -2
}

#[unsafe(no_mangle)]
pub extern "C" fn udplite_cache_remove(
    _manager: *mut CacheManager,
    _key: *const CacheKey,
) -> c_int {
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn udplite_cache_get_stats(
    _manager: *const CacheManager,
    out_stats: *mut CacheStatistics,
) -> c_int {
    if out_stats.is_null() {
        return -22;
    }

    unsafe {
        (*out_stats).hits = 0;
        (*out_stats).misses = 0;
        (*out_stats).inserts = 0;
        (*out_stats).evictions = 0;
    }
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn udplite_cache_reset_stats(_manager: *mut CacheManager) -> c_int {
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn udplite_cache_set_label(
    _manager: *mut CacheManager,
    _label: *const c_char,
    _label_len: socklen_t,
) -> c_int {
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn udplite_cache_get_label(
    _manager: *const CacheManager,
    _buf: *mut c_char,
    _buf_len: socklen_t,
) -> c_int {
    0
}