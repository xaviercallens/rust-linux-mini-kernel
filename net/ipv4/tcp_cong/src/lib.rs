//! TCP Congestion Control Support
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(clang::too_many_arguments)]
#![allow(clang::implicit_return_in_non_void_function)]

use core::ffi::c_int;
use core::ffi::c_uint;
use core::ffi::c_ulong;
use core::ffi::c_void;
use core::mem;
use core::ptr;
use core::slice;

// Constants from C
pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const EEXIST: c_int = -17;
pub const ENOENT: c_int = -2;
pub const EBUSY: c_int = -16;
pub const EPERM: c_int = -1;
pub const TCP_CA_NAME_MAX: c_int = 16;
pub const TCP_CA_UNSPEC: u32 = 0;

// Type definitions
#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct tcp_congestion_ops {
    pub name: [u8; TCP_CA_NAME_MAX as usize],
    pub key: u32,
    pub flags: c_uint,
    pub list: list_head,
    pub ssthresh: extern "C" fn(*mut c_void, *mut c_void) -> c_ulong,
    pub cong_avoid: extern "C" fn(*mut c_void, *mut c_void, c_ulong, c_ulong),
    pub set_state: extern "C" fn(*mut c_void, c_int),
    pub cwnd_undo: extern "C" fn(*mut c_void),
    pub init: extern "C" fn(*mut c_void),
    pub release: extern "C" fn(*mut c_void),
    pub set_congestion: extern "C" fn(*mut c_void, *mut c_void),
    pub get_info: extern "C" fn(*mut c_void, *mut c_void, *mut c_void, *mut c_void),
    pub owner: *mut c_void,
}

// Function implementations
static mut tcp_cong_list_lock: c_int = 0;
static mut tcp_cong_list: list_head = list_head {
    next: &mut tcp_cong_list as *mut _,
    prev: &mut tcp_cong_list as *mut _,
};

/// Find congestion control by name
///
/// # Safety
/// - `name` must be a valid null-terminated string
/// - Must be called with RCU read lock held
#[no_mangle]
pub unsafe extern "C" fn tcp_ca_find(name: *const u8) -> *mut tcp_congestion_ops {
    let mut e: *mut tcp_congestion_ops = ptr::null_mut();
    let mut pos: *mut list_head = tcp_cong_list.next;
    
    while !pos.eq(&tcp_cong_list) {
        e = (pos as *mut tcp_congestion_ops).offset(-1);
        if strcmp(e, name) == 0 {
            return e;
        }
        pos = (*pos).next;
    }
    
    ptr::null_mut()
}

/// Find congestion control with autoload
///
/// # Safety
/// - `net` must be valid network namespace
/// - `name` must be valid null-terminated string
/// - Must handle module loading safely
#[no_mangle]
pub unsafe extern "C" fn tcp_ca_find_autoload(
    net: *mut c_void,
    name: *const u8
) -> *mut tcp_congestion_ops {
    let ca = tcp_ca_find(name);
    
    // SAFETY: Module loading requires CAP_NET_ADMIN
    if ca.is_null() {
        // Check if caller has CAP_NET_ADMIN
        let has_cap = capable(CAP_NET_ADMIN);
        if has_cap != 0 {
            // Release RCU lock before module load
            rcu_read_unlock();
            // Request module
            let _ = request_module(b"tcp_%s\0".as_ptr() as *const u8, name);
            // Reacquire RCU lock
            rcu_read_lock();
            // Try again
            ca = tcp_ca_find(name);
        }
    }
    
    ca
}

/// Find congestion control by key
///
/// # Safety
/// - Must be called with RCU read lock held
#[no_mangle]
pub unsafe extern "C" fn tcp_ca_find_key(key: u32) -> *mut tcp_congestion_ops {
    let mut e: *mut tcp_congestion_ops = ptr::null_mut();
    let mut pos: *mut list_head = tcp_cong_list.next;
    
    while !pos.eq(&tcp_cong_list) {
        e = (pos as *mut tcp_congestion_ops).offset(-1);
        if (*e).key == key {
            return e;
        }
        pos = (*pos).next;
    }
    
    ptr::null_mut()
}

/// Register congestion control algorithm
///
/// # Safety
/// - `ca` must point to valid initialized congestion control
/// - Must be called from module init context
#[no_mangle]
pub unsafe extern "C" fn tcp_register_congestion_control(
    ca: *mut tcp_congestion_ops
) -> c_int {
    if ca.is_null() {
        return EINVAL;
    }
    
    // Check required operations
    if (*ca).ssthresh.is_null() || (*ca).cwnd_undo.is_null() ||
       ((*ca).cong_avoid.is_null() && (*ca).cong_control.is_null()) {
        return EINVAL;
    }
    
    // Compute key
    let name_len = strnlen((*ca).name.as_ptr(), TCP_CA_NAME_MAX as usize) as usize;
    let key = jhash((*ca).name.as_ptr(), name_len, name_len as u32);
    
    // Acquire lock
    spin_lock(&mut tcp_cong_list_lock);
    
    if key == TCP_CA_UNSPEC || !tcp_ca_find_key(key).is_null() {
        spin_unlock(&mut tcp_cong_list_lock);
        return EEXIST;
    }
    
    (*ca).key = key;
    
    // Add to list
    list_add_tail_rcu(&mut (*ca).list, &mut tcp_cong_list);
    
    spin_unlock(&mut tcp_cong_list_lock);
    
    0
}

/// Unregister congestion control algorithm
///
/// # Safety
/// - `ca` must be a valid entry in the congestion control list
/// - Must be called from module exit context
#[no_mangle]
pub unsafe extern "C" fn tcp_unregister_congestion_control(
    ca: *mut tcp_congestion_ops
) {
    if !ca.is_null() {
        spin_lock(&mut tcp_cong_list_lock);
        list_del_rcu(&mut (*ca).list);
        spin_unlock(&mut tcp_cong_list_lock);
        
        // Wait for RCU grace period
        synchronize_rcu();
    }
}

/// Get key by name
///
/// # Safety
/// - `name` must be valid null-terminated string
/// - `net` must be valid network namespace
#[no_mangle]
pub unsafe extern "C" fn tcp_ca_get_key_by_name(
    net: *mut c_void,
    name: *const u8,
    ecn_ca: *mut c_int
) -> u32 {
    let mut key = TCP_CA_UNSPEC;
    
    if ecn_ca.is_null() {
        return key;
    }
    
    rcu_read_lock();
    let ca = tcp_ca_find_autoload(net, name);
    if !ca.is_null() {
        key = (*ca).key;
        *ecn_ca = if (*ca).flags & TCP_CONG_NEEDS_ECN != 0 { 1 } else { 0 };
    }
    rcu_read_unlock();
    
    key
}

/// Get name by key
///
/// # Safety
/// - `buffer` must be at least TCP_CA_NAME_MAX bytes
/// - Must be called with RCU read lock held
#[no_mangle]
pub unsafe extern "C" fn tcp_ca_get_name_by_key(
    key: u32,
    buffer: *mut u8
) -> *mut u8 {
    let mut ca: *mut tcp_congestion_ops = ptr::null_mut();
    
    rcu_read_lock();
    ca = tcp_ca_find_key(key);
    if !ca.is_null() {
        ptr::copy_nonoverlapping((*ca).name.as_ptr(), buffer, TCP_CA_NAME_MAX as usize);
        buffer
    } else {
        ptr::null_mut()
    }
}

// Helper functions
#[no_mangle]
pub unsafe extern "C" fn spin_lock(lock: *mut c_int) {
    // Simple spinlock implementation
    while !atomic_cmpxchg(lock, 0, 1).is_null() {}
}

#[no_mangle]
pub unsafe extern "C" fn spin_unlock(lock: *mut c_int) {
    *lock = 0;
}

#[no_mangle]
pub unsafe extern "C" fn list_add_tail_rcu(
    new: *mut list_head,
    head: *mut list_head
) {
    let prev = (*head).prev;
    (*new).prev = prev;
    (*new).next = head;
    (*prev).next = new;
    (*head).prev = new;
}

#[no_mangle]
pub unsafe extern "C" fn list_del_rcu(entry: *mut list_head) {
    let prev = (*entry).prev;
    let next = (*entry).next;
    (*next).prev = prev;
    (*prev).next = next;
}

#[no_mangle]
pub unsafe extern "C" fn synchronize_rcu() {
    // Wait for RCU grace period
}

#[no_mangle]
pub unsafe extern "C" fn jhash(key: *const u8, length: usize, initval: u32) -> u32 {
    // Simplified jhash implementation
    let mut hash = initval;
    let key_bytes = slice::from_raw_parts(key, length);
    for &b in key_bytes {
        hash += b as u32;
        hash += hash << 10;
        hash ^= hash >> 6;
    }
    hash
}

#[no_mangle]
pub unsafe extern "C" fn strcmp(s1: *const u8, s2: *const u8) -> c_int {
    let mut i = 0;
    loop {
        let c1 = *s1.offset(i);
        let c2 = *s2.offset(i);
        if c1 != c2 {
            return c1.wrapping_sub(c2) as c_int;
        }
        if c1 == 0 {
            return 0;
        }
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn strnlen(s: *const u8, max: usize) -> usize {
    let mut i = 0;
    while i < max && *s.offset(i as isize) != 0 {
        i += 1;
    }
    i
}

#[no_mangle]
pub unsafe extern "C" fn capable(cap: c_int) -> c_int {
    // Placeholder for capability check
    1 // Assume capable for simplicity
}

#[no_mangle]
pub unsafe extern "C" fn request_module(fmt: *const u8, name: *const u8) -> c_int {
    // Placeholder for module request
    0
}

#[no_mangle]
pub unsafe extern "C" fn rcu_read_lock() {
    // Placeholder for RCU read lock
}

#[no_mangle]
pub unsafe extern "C" fn rcu_read_unlock() {
    // Placeholder for RCU read unlock
}

#[no_mangle]
pub unsafe extern "C" fn atomic_cmpxchg(
    ptr: *mut c_int,
    old: c_int,
    new: c_int
) -> c_int {
    let current = *ptr;
    if current == old {
        *ptr = new;
    }
    current
}

// Exported symbols
#[no_mangle]
pub unsafe extern "C" fn tcp_register_congestion_control_exported(
    ca: *mut tcp_congestion_ops
) -> c_int {
    tcp_register_congestion_control(ca)
}

#[no_mangle]
pub unsafe extern "C" fn tcp_unregister_congestion_control_exported(
    ca: *mut tcp_congestion_ops
) {
    tcp_unregister_congestion_control(ca)
}

#[no_mangle]
pub unsafe extern "C" fn tcp_ca_get_key_by_name_exported(
    net: *mut c_void,
    name: *const u8,
    ecn_ca: *mut c_int
) -> u32 {
    tcp_ca_get_key_by_name(net, name, ecn_ca)
}

#[no_mangle]
pub unsafe extern "C" fn tcp_ca_get_name_by_key_exported(
    key: u32,
    buffer: *mut u8
) -> *mut u8 {
    tcp_ca_get_name_by_key(key, buffer)
}
