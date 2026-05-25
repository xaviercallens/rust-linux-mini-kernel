#![allow(clippy::all, clippy::pedantic)]
#![warn(clippy::pedantic)]
#![deny(clippy::all)]
#![no_std]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use kernel_types::{nf_conntrack_tuple, nf_conntrack_man, nf_conntrack_tuple_hash};

/// UDP disconnect tuple for connection tracking
pub static mut __UDP_DISCONNECT: *mut nf_conntrack_tuple = core::ptr::null_mut();

/// ICMPv6 error conversion table
pub static mut ICMPV6_ERR_CONVERT: *mut core::ffi::c_void = core::ptr::null_mut();

/// IPv6 sockraw operations
pub static mut INET6_SOCKRAW_OPS: *mut core::ffi::c_void = core::ptr::null_mut();

/// IPv6 datagram connect v6 only
pub static mut IP6_DATAGRAM_CONNECT_V6_ONLY: *mut core::ffi::c_void = core::ptr::null_mut();

/// IPv6 datagram receive common control
pub static mut IP6_DATAGRAM_RECV_COMMON_CTL: *mut core::ffi::c_void = core::ptr::null_mut();

#[no_mangle]
pub unsafe extern "C" fn nf_conntrack_alloc(
    _zone: *mut core::ffi::c_void,
    _tuple: *const nf_conntrack_tuple,
    _man: *const nf_conntrack_man,
    _hash: *const nf_conntrack_tuple_hash,
) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn nf_conntrack_free(
    _ct: *mut core::ffi::c_void,
) {
    // Stub implementation
}

#[no_mangle]
pub unsafe extern "C" fn nf_conntrack_find_get(
    _zone: *mut core::ffi::c_void,
    _tuple: *const nf_conntrack_tuple,
) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn nf_conntrack_get(
    ct: *mut core::ffi::c_void,
) -> *mut core::ffi::c_void {
    ct
}

#[no_mangle]
pub unsafe extern "C" fn nf_conntrack_put(
    _ct: *mut core::ffi::c_void,
) {
    // Stub implementation
}

#[no_mangle]
pub unsafe extern "C" fn nf_conntrack_hash_insert(
    _ct: *mut core::ffi::c_void,
    _hash: *const nf_conntrack_tuple_hash,
) -> core::ffi::c_int {
    0
}

#[no_mangle]
pub unsafe extern "C" fn nf_conntrack_hash_check_insert(
    _ct: *mut core::ffi::c_void,
    _hash: *const nf_conntrack_tuple_hash,
) -> core::ffi::c_int {
    0
}

#[no_mangle]
pub unsafe extern "C" fn nf_conntrack_destroy(
    _ct: *mut core::ffi::c_void,
) {
    // Stub implementation
}

#[no_mangle]
pub unsafe extern "C" fn nf_conntrack_event(
    _ct: *mut core::ffi::c_void,
    _mask: core::ffi::c_uint,
) {
    // Stub implementation
}

#[no_mangle]
pub unsafe extern "C" fn nf_conntrack_ecache_find_get(
    _ct: *mut core::ffi::c_void,
) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn nf_conntrack_ecache_put(
    _ecache: *mut core::ffi::c_void,
) {
    // Stub implementation
}

#[no_mangle]
pub unsafe extern "C" fn nf_conntrack_ecache_ext_add(
    _ecache: *mut core::ffi::c_void,
    _ext: *mut core::ffi::c_void,
) -> core::ffi::c_int {
    0
}

#[no_mangle]
pub unsafe extern "C" fn nf_conntrack_ecache_ext_del(
    _ecache: *mut core::ffi::c_void,
    _ext: *mut core::ffi::c_void,
) {
    // Stub implementation
}

#[no_mangle]
pub unsafe extern "C" fn nf_conntrack_ecache_ext_find(
    _ecache: *mut core::ffi::c_void,
    _ext: *mut core::ffi::c_void,
) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn nf_conntrack_ecache_ext_iterate(
    _ecache: *mut core::ffi::c_void,
    _cb: extern "C" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> core::ffi::c_int,
    _data: *mut core::ffi::c_void,
) -> core::ffi::c_int {
    0
}

#[no_mangle]
pub unsafe extern "C" fn nf_conntrack_ecache_ext_size(
    _ecache: *mut core::ffi::c_void,
) -> core::ffi::c_uint {
    0
}

#[no_mangle]
pub unsafe extern "C" fn nf_conntrack_ecache_ext_destroy(
    _ecache: *mut core::ffi::c_void,
) {
    // Stub implementation
}

#[no_mangle]
pub unsafe extern "C" fn nf_conntrack_ecache_ext_create(
    _ct: *mut core::ffi::c_void,
) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn nf_conntrack_ecache_ext_replace(
    _ct: *mut core::ffi::c_void,
    _ecache: *mut core::ffi::c_void,
) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

// All duplicate function definitions removed - only keeping the first set above

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
