//! net/core/dst_cache - Destination cache implementation
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use core::ptr;
use core::ffi::c_int;
use core::ffi::c_uint;
use core::ffi::c_void;

// Constants from C
pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;

// Type definitions
#[repr(C)]
struct in_addr {
    s_addr: u32,
}

#[repr(C)]
struct in6_addr {
    s6_addr: [u8; 16],
}

#[repr(C)]
union AddrUnion {
    in_saddr: in_addr,
    in6_saddr: in6_addr,
}

#[repr(C)]
struct dst_entry {
    obsolete: c_int,
    ops: *const c_void,
}

#[repr(C)]
struct rtable {
    dst: dst_entry,
}

#[repr(C)]
struct dst_cache_pcpu {
    refresh_ts: usize,
    dst: *mut dst_entry,
    cookie: u32,
    addr: AddrUnion,
}

#[repr(C)]
struct dst_cache {
    cache: *mut dst_cache_pcpu,
    reset_ts: usize,
}

// Function declarations for kernel APIs
extern "C" {
    fn this_cpu_ptr(ptr: *mut dst_cache_pcpu) -> *mut dst_cache_pcpu;
    fn per_cpu_ptr(ptr: *mut dst_cache_pcpu, cpu: c_int) -> *mut dst_cache_pcpu;
    fn alloc_percpu_gfp<T>(size: usize, gfp: c_int) -> *mut T;
    fn free_percpu<T>(ptr: *mut T);
    fn jiffies() -> usize;
    fn num_possible_cpus() -> c_int;
    fn dst_hold(dst: *mut dst_entry);
    fn dst_release(dst: *mut dst_entry);
    fn dst_cache_reset(cache: *mut dst_cache);
    fn rt6_get_cookie(dst: *mut dst_entry) -> u32;
}

// Helper functions
fn time_after(a: usize, b: usize) -> bool {
    (a.wrapping_sub(b) as isize) < (1 << 31)
}

// Internal functions
fn dst_cache_per_cpu_dst_set(dst_cache: *mut dst_cache_pcpu, dst: *mut dst_entry, cookie: u32) {
    unsafe {
        dst_release((*dst_cache).dst);
    }
    if !dst.is_null() {
        unsafe {
            dst_hold(dst);
        }
    }
    unsafe {
        (*dst_cache).cookie = cookie;
        (*dst_cache).dst = dst;
    }
}

fn dst_cache_per_cpu_get(dst_cache: *mut dst_cache, idst: *mut dst_cache_pcpu) -> *mut dst_entry {
    let dst = unsafe { (*idst).dst };
    if dst.is_null() {
        unsafe {
            (*idst).refresh_ts = jiffies();
        }
        return ptr::null_mut();
    }

    unsafe {
        dst_hold(dst);
    }

    let is_valid = unsafe {
        time_after((*idst).refresh_ts, (*dst_cache).reset_ts) &&
        (!(*dst).obsolete || {
            let check_fn = (*(*dst).ops).check;
            if check_fn.is_some() {
                check_fn.expect("check function")(dst, (*idst).cookie) != 0
            } else {
                true
            }
        })
    };

    if !is_valid {
        unsafe {
            dst_cache_per_cpu_dst_set(idst, ptr::null_mut(), 0);
            dst_release(dst);
            (*idst).refresh_ts = jiffies();
        }
        return ptr::null_mut();
    }

    dst
}

// Exported functions
#[no_mangle]
pub unsafe extern "C" fn dst_cache_get(dst_cache: *mut dst_cache) -> *mut dst_entry {
    if (*dst_cache).cache.is_null() {
        return ptr::null_mut();
    }

    let idst = this_cpu_ptr((*dst_cache).cache);
    dst_cache_per_cpu_get(dst_cache, idst)
}

#[no_mangle]
pub unsafe extern "C" fn dst_cache_get_ip4(dst_cache: *mut dst_cache, saddr: *mut u32) -> *mut rtable {
    if (*dst_cache).cache.is_null() {
        return ptr::null_mut();
    }

    let idst = this_cpu_ptr((*dst_cache).cache);
    let dst = dst_cache_per_cpu_get(dst_cache, idst);
    if dst.is_null() {
        return ptr::null_mut();
    }

    *saddr = unsafe { (*idst).addr.in_saddr.s_addr };
    
    // container_of implementation
    let rtable = (dst as *mut rtable).offset(-(core::mem::offset_of!(rtable, dst) as isize));
    rtable
}

#[no_mangle]
pub unsafe extern "C" fn dst_cache_set_ip4(dst_cache: *mut dst_cache, dst: *mut dst_entry, saddr: u32) {
    if (*dst_cache).cache.is_null() {
        return;
    }

    let idst = this_cpu_ptr((*dst_cache).cache);
    dst_cache_per_cpu_dst_set(idst, dst, 0);
    unsafe {
        (*idst).addr.in_saddr.s_addr = saddr;
    }
}

#[no_mangle]
pub unsafe extern "C" fn dst_cache_set_ip6(dst_cache: *mut dst_cache, dst: *mut dst_entry, saddr: *const in6_addr) {
    if (*dst_cache).cache.is_null() {
        return;
    }

    let idst = this_cpu_ptr((*dst_cache).cache);
    let cookie = if !dst.is_null() {
        rt6_get_cookie(dst)
    } else {
        0
    };
    
    dst_cache_per_cpu_dst_set(idst, dst, cookie);
    
    if !saddr.is_null() {
        unsafe {
            (*idst).addr.in6_saddr = *saddr;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn dst_cache_get_ip6(dst_cache: *mut dst_cache, saddr: *mut in6_addr) -> *mut dst_entry {
    if (*dst_cache).cache.is_null() {
        return ptr::null_mut();
    }

    let idst = this_cpu_ptr((*dst_cache).cache);
    let dst = dst_cache_per_cpu_get(dst_cache, idst);
    if dst.is_null() {
        return ptr::null_mut();
    }

    if !saddr.is_null() {
        unsafe {
            *saddr = (*idst).addr.in6_saddr;
        }
    }
    dst
}

#[no_mangle]
pub unsafe extern "C" fn dst_cache_init(dst_cache: *mut dst_cache, gfp: c_int) -> c_int {
    let gfp_zero = 0x20; // Assuming __GFP_ZERO is 0x20
    let gfp_with_zero = gfp | gfp_zero;
    
    let cache = alloc_percpu_gfp(core::mem::size_of::<dst_cache_pcpu>() as usize, gfp_with_zero);
    if cache.is_null() {
        return -ENOMEM;
    }
    
    (*dst_cache).cache = cache;
    dst_cache_reset(dst_cache);
    0
}

#[no_mangle]
pub unsafe extern "C" fn dst_cache_destroy(dst_cache: *mut dst_cache) {
    if (*dst_cache).cache.is_null() {
        return;
    }
    
    let num_cpus = num_possible_cpus();
    for i in 0..num_cpus {
        let per_cpu_dst = per_cpu_ptr((*dst_cache).cache, i);
        dst_release((*per_cpu_dst).dst);
    }
    
    free_percpu((*dst_cache).cache);
}
