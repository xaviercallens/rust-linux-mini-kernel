#![no_std]

use core::ffi::c_void;
use core::panic::PanicInfo;
use core::ptr;
use kernel_types::*;

// Linux-style error codes
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;

#[repr(C)]
pub struct nf_conn {
    _priv: [u8; 0],
}

#[repr(C)]
pub struct nf_conntrack_tuple {
    _priv: [u8; 0],
}

#[repr(C)]
pub struct nf_nat_range2 {
    pub flags: c_uint,
    pub min_addr: c_uint,
    pub max_addr: c_uint,
    pub min_proto: c_ushort,
    pub max_proto: c_ushort,
}

#[repr(C)]
pub struct CacheKey {
    pub src_addr: c_uint,
    pub dst_addr: c_uint,
    pub src_port: c_ushort,
    pub dst_port: c_ushort,
    pub proto: c_uchar,
}

#[repr(C)]
pub struct CacheStatistics {
    pub entries: c_uint,
    pub hits: c_ulong,
    pub misses: c_ulong,
}

pub struct CacheManager {
    stats: CacheStatistics,
}

impl CacheManager {
    const fn new() -> Self {
        Self {
            stats: CacheStatistics {
                entries: 0,
                hits: 0,
                misses: 0,
            },
        }
    }
}

static mut CACHE_MANAGER: CacheManager = CacheManager::new();

#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn nf_nat_core_init() -> c_int {
    unsafe {
        CACHE_MANAGER.stats.entries = 0;
        CACHE_MANAGER.stats.hits = 0;
        CACHE_MANAGER.stats.misses = 0;
    }
    0
}

#[no_mangle]
pub extern "C" fn nf_nat_core_exit() {}

#[no_mangle]
pub extern "C" fn nf_nat_setup_info(
    ct: *mut nf_conn,
    range: *const nf_nat_range2,
    _maniptype: c_uint,
) -> c_int {
    if ct.is_null() || range.is_null() {
        return -EINVAL;
    }

    let r = unsafe { &*range };

    if r.min_proto > r.max_proto || r.min_addr > r.max_addr {
        return -EINVAL;
    }

    unsafe {
        CACHE_MANAGER.stats.hits = CACHE_MANAGER.stats.hits.wrapping_add(1);
    }

    0
}

#[no_mangle]
pub extern "C" fn nf_nat_packet(
    ct: *mut nf_conn,
    _ctinfo: c_uint,
    _hooknum: c_uint,
    _skb: *mut c_void,
) -> c_int {
    if ct.is_null() {
        return -EINVAL;
    }

    unsafe {
        CACHE_MANAGER.stats.entries = CACHE_MANAGER.stats.entries.wrapping_add(1);
    }

    1
}

#[no_mangle]
pub extern "C" fn nf_nat_alloc_null_binding(ct: *mut nf_conn, _hooknum: c_uint) -> c_int {
    if ct.is_null() {
        return -EINVAL;
    }
    0
}

#[no_mangle]
pub extern "C" fn nf_nat_inet_register_fn(_pf: c_uint, ops: *mut c_void) -> c_int {
    if ops.is_null() {
        return -EINVAL;
    }
    0
}

#[no_mangle]
pub extern "C" fn nf_nat_inet_unregister_fn(_pf: c_uint, _ops: *mut c_void) {}

#[no_mangle]
pub extern "C" fn nf_nat_get_statistics(out: *mut CacheStatistics) -> c_int {
    if out.is_null() {
        return -EINVAL;
    }

    unsafe {
        ptr::write(
            out,
            CacheStatistics {
                entries: CACHE_MANAGER.stats.entries,
                hits: CACHE_MANAGER.stats.hits,
                misses: CACHE_MANAGER.stats.misses,
            },
        );
    }

    0
}

#[no_mangle]
pub extern "C" fn nf_nat_cache_lookup(_key: *const CacheKey) -> c_int {
    unsafe {
        CACHE_MANAGER.stats.misses = CACHE_MANAGER.stats.misses.wrapping_add(1);
    }
    -ENOMEM
}