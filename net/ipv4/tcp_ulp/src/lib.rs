//! Pluggable TCP upper layer protocol support.
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]  // For C-style type names

use core::ptr;
use core::ffi::c_int;
use core::ffi::c_void;
use core::ffi::CStr;
use core::ffi::CString;
use core::mem;
use core::ptr::NonNull;

// Constants from C
pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ENOSYS: c_int = -38;
pub const EEXIST: c_int = -17;
pub const ENOENT: c_int = -2;

// Type definitions
#[repr(C)]
pub struct tcp_ulp_ops {
    pub name: *const u8,
    pub owner: *const c_void,
    pub init: extern "C" fn(*mut c_void) -> c_int,
    pub release: extern "C" fn(*mut c_void),
    pub update: extern "C" fn(*mut c_void, *mut c_void, extern "C" fn(*mut c_void)),
}

#[repr(C)]
struct list_head {
    next: *mut list_head,
    prev: *mut list_head,
}

// Static variables
static mut tcp_ulp_list_lock: spinlock_t = spinlock_t::new();
static mut tcp_ulp_list: list_head = list_head {
    next: ptr::null_mut(),
    prev: ptr::null_mut(),
};

// Function pointers for kernel functions
extern "C" {
    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);
    fn list_add_tail_rcu(new: *mut list_head, head: *mut list_head);
    fn list_del_rcu(entry: *mut list_head);
    fn synchronize_rcu();
    fn rcu_read_lock();
    fn rcu_read_unlock();
    fn request_module(fmt: *const u8, name: *const u8);
    fn capable(cap: c_int) -> c_int;
    fn try_module_get(owner: *const c_void) -> c_int;
    fn module_put(owner: *const c_void) -> c_int;
    fn sock_owned_by_me(sk: *mut c_void) -> c_int;
    fn inet_csk(sk: *mut c_void) -> *mut inet_connection_sock;
    fn snprintf(buf: *mut u8, size: size_t, fmt: *const u8, ...) -> size_t;
    fn strcmp(s1: *const u8, s2: *const u8) -> c_int;
}

#[repr(C)]
struct spinlock_t {
    raw: u32,
}

impl spinlock_t {
    const fn new() -> Self {
        spinlock_t { raw: 0 }
    }
}

#[repr(C)]
struct inet_connection_sock {
    icsk_ulp_ops: *mut tcp_ulp_ops,
}

// Function implementations
/// Find ULP by name
fn tcp_ulp_find(name: *const u8) -> *mut tcp_ulp_ops {
    let mut head = unsafe { &mut tcp_ulp_list };
    let mut curr = unsafe { head.next };
    
    while !curr.is_null() && curr != head {
        let entry = (curr as *mut tcp_ulp_ops).offset(-mem::size_of::<list_head>() as isize / mem::size_of::<tcp_ulp_ops>() as isize);
        
        // SAFETY: We're in RCU read-side critical section and lock is held
        let entry_name = unsafe { CStr::from_ptr(name as *const i8) };
        let entry_name_cstr = unsafe { CStr::from_ptr((*entry).name as *const i8) };
        
        if unsafe { strcmp((*entry).name, name) } == 0 {
            return entry;
        }
        
        // SAFETY: List traversal is safe within RCU critical section
        curr = unsafe { (*curr).next };
    }
    
    ptr::null_mut()
}

/// Find ULP with auto-load
fn __tcp_ulp_find_autoload(name: *const u8) -> *mut tcp_ulp_ops {
    let mut ulp = ptr::null_mut();
    
    unsafe { rcu_read_lock() };
    ulp = tcp_ulp_find(name);
    
    if ulp.is_null() {
        // CONFIG_MODULES support
        if unsafe { capable(1) } != 0 { // CAP_NET_ADMIN
            unsafe { rcu_read_unlock() };
            unsafe { request_module(b"tcp-ulp-%s", name) };
            unsafe { rcu_read_lock() };
            ulp = tcp_ulp_find(name);
        }
    }
    
    if !ulp.is_null() && unsafe { try_module_get((*ulp).owner) } != 0 {
        ulp
    } else {
        ptr::null_mut()
    }
}

/// Register ULP implementation
#[no_mangle]
pub unsafe extern "C" fn tcp_register_ulp(ulp: *mut tcp_ulp_ops) -> c_int {
    if ulp.is_null() {
        return EINVAL;
    }
    
    spin_lock(&mut tcp_ulp_list_lock);
    
    if !tcp_ulp_find((*ulp).name).is_null() {
        spin_unlock(&mut tcp_ulp_list_lock);
        return EEXIST;
    }
    
    // SAFETY: Lock is held, list is valid
    list_add_tail_rcu(&mut (*ulp).list, &mut tcp_ulp_list);
    
    spin_unlock(&mut tcp_ulp_list_lock);
    0
}

/// Unregister ULP implementation
#[no_mangle]
pub unsafe extern "C" fn tcp_unregister_ulp(ulp: *mut tcp_ulp_ops) {
    if ulp.is_null() {
        return;
    }
    
    spin_lock(&mut tcp_ulp_list_lock);
    list_del_rcu(&mut (*ulp).list);
    spin_unlock(&mut tcp_ulp_list_lock);
    
    synchronize_rcu();
}

/// Build string with list of available ULPs
#[no_mangle]
pub unsafe extern "C" fn tcp_get_available_ulp(buf: *mut u8, maxlen: size_t) {
    if buf.is_null() {
        return;
    }
    
    *buf = 0;
    rcu_read_lock();
    
    let mut head = &mut tcp_ulp_list;
    let mut curr = head.next;
    
    while !curr.is_null() && curr != head {
        let entry = (curr as *mut tcp_ulp_ops).offset(-mem::size_of::<list_head>() as isize / mem::size_of::<tcp_ulp_ops>() as isize);
        let offs = snprintf(buf, maxlen, b"%s%s\0", if *buf == 0 { ptr::null() } else { b" " }, (*entry).name);
        
        if offs >= maxlen {
            break;
        }
        
        curr = (*curr).next;
    }
    
    rcu_read_unlock();
}

/// Update ULP with new socket parameters
#[no_mangle]
pub unsafe extern "C" fn tcp_update_ulp(sk: *mut c_void, proto: *mut c_void, write_space: extern "C" fn(*mut c_void)) {
    let icsk = inet_csk(sk);
    
    if !(*icsk).icsk_ulp_ops.is_null() && (*(*icsk).icsk_ulp_ops).update != ptr::null() {
        ((*(*icsk).icsk_ulp_ops).update)(sk, proto, write_space);
    }
}

/// Clean up ULP resources
#[no_mangle]
pub unsafe extern "C" fn tcp_cleanup_ulp(sk: *mut c_void) {
    let icsk = inet_csk(sk);
    
    if (*icsk).icsk_ulp_ops.is_null() {
        return;
    }
    
    if !(*(*icsk).icsk_ulp_ops).release.is_null() {
        ((*(*icsk).icsk_ulp_ops).release)(sk);
    }
    
    module_put((*(*icsk).icsk_ulp_ops).owner);
    (*icsk).icsk_ulp_ops = ptr::null_mut();
}

/// Internal ULP setup
fn __tcp_set_ulp(sk: *mut c_void, ulp_ops: *mut tcp_ulp_ops) -> c_int {
    let icsk = inet_csk(sk);
    
    if !(*icsk).icsk_ulp_ops.is_null() {
        return EEXIST;
    }
    
    let ret = (ulp_ops as *mut tcp_ulp_ops).init(sk);
    if ret != 0 {
        module_put((*ulp_ops).owner);
        return ret;
    }
    
    (*icsk).icsk_ulp_ops = ulp_ops;
    0
}

/// Set ULP for socket
#[no_mangle]
pub unsafe extern "C" fn tcp_set_ulp(sk: *mut c_void, name: *const u8) -> c_int {
    if sock_owned_by_me(sk) != 0 {
        let ulp_ops = __tcp_ulp_find_autoload(name);
        if ulp_ops.is_null() {
            return ENOENT;
        }
        return __tcp_set_ulp(sk, ulp_ops);
    }
    
    EINVAL
}

// Tests (conditional compilation)
#[cfg(test)]
mod tests {
    #[test]
    fn test_ulp_registration() {
        // Basic test would require kernel environment
        // This is a placeholder for actual test cases
    }
}
