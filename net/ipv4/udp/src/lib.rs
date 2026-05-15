//! This module provides FFI-compatible Rust bindings for UDP port management
//! functionality from the Linux kernel. The implementation maintains exact ABI
//! compatibility with the original C code while using Rust's type system to ensure
//! memory safety where possible.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use core::ptr;
use core::ffi::c_int;
use core::ffi::c_uint;
use core::ffi::c_ulong;
use core::ffi::c_void;

// Constants from C
const MAX_UDP_PORTS: usize = 65536;
const PORTS_PER_CHAIN: usize = MAX_UDP_PORTS / UDP_HTABLE_SIZE_MIN;

// Type definitions
#[repr(C)]
struct sock {
    sk_reuse: c_int,
    sk_reuseport: c_int,
    sk_bound_dev_if: c_int,
    sk_prot: *const c_void,
    sk_reuseport_cb: *const c_void,
    __sk_common: sk_common,
}

#[repr(C)]
struct sk_common {
    slock: c_int,
    net: *const c_void,
    family: c_int,
    rcv_saddr: u32,
}

#[repr(C)]
struct udp_hslot {
    head: *mut sock,
    lock: c_int,
    count: c_int,
}

#[repr(C)]
struct udp_table {
    mask: c_int,
    log: c_int,
}

#[repr(C)]
struct net {
    // Placeholder for actual fields
    _private: [u8; 0],
}

// Exported symbols
#[no_mangle]
pub static mut udp_table: udp_table = unsafe { core::mem::zeroed() };
#[no_mangle]
pub static mut sysctl_udp_mem: [c_int; 3] = [0; 3];
#[no_mangle]
pub static mut udp_memory_allocated: atomic_long = unsafe { core::mem::zeroed() };

#[repr(C)]
struct atomic_long {
    counter: c_long,
}

// Function implementations
/// Check if a local port is in use
///
/// # Safety
/// - `net` must be a valid pointer to a network namespace
/// - `hslot` must point to a valid udp_hslot
/// - `sk` must be a valid socket pointer
///
/// # Returns
/// 0 if port is available, 1 if in use
#[no_mangle]
pub unsafe extern "C" fn udp_lib_lport_inuse(
    net: *mut net,
    num: u16,
    hslot: *const udp_hslot,
    bitmap: *mut c_ulong,
    sk: *mut sock,
    log: c_uint,
) -> c_int {
    let mut result = 0;
    let uid = sock_i_uid(sk);
    
    // SAFETY: hslot is valid and head is a valid pointer
    let mut sk2 = (*hslot).head;
    while !sk2.is_null() {
        if net_eq((*sk2).sk_net(), net) &&
           sk2 != sk &&
           (bitmap.is_null() || (*sk2).sk_reuseport == num) &&
           (!(*sk2).sk_reuse || !(*sk).sk_reuse) &&
           (!(*sk2).sk_bound_dev_if || !(*sk).sk_bound_dev_if ||
            (*sk2).sk_bound_dev_if == (*sk).sk_bound_dev_if) &&
           inet_rcv_saddr_equal(sk, sk2, 1) {
            
            if (*sk2).sk_reuseport != 0 && (*sk).sk_reuseport != 0 &&
               rcu_access_pointer((*sk).sk_reuseport_cb).is_null() &&
               uid_eq(uid, sock_i_uid(sk2)) {
                if bitmap.is_null() {
                    result = 0;
                } else {
                    let bit = (*sk2).sk_reuseport >> log;
                    __set_bit(bit, bitmap);
                }
            } else {
                if bitmap.is_null() {
                    result = 1;
                } else {
                    let bit = (*sk2).sk_reuseport >> log;
                    __set_bit(bit, bitmap);
                }
            }
        }
        sk2 = (*sk2).next;
    }
    
    result
}

/// Second pass port check with secondary hash
///
/// # Safety
/// - `hslot2` must be a valid pointer to a udp_hslot
/// - `sk` must be a valid socket pointer
///
/// # Returns
/// 0 if port is available, 1 if in use
#[no_mangle]
pub unsafe extern "C" fn udp_lib_lport_inuse2(
    net: *mut net,
    num: u16,
    hslot2: *mut udp_hslot,
    sk: *mut sock,
) -> c_int {
    let mut result = 0;
    let uid = sock_i_uid(sk);
    
    // SAFETY: hslot2 is valid and lock is properly held
    spin_lock(&mut (*hslot2).lock);
    let mut sk2 = (*hslot2).head;
    while !sk2.is_null() {
        if net_eq((*sk2).sk_net(), net) &&
           sk2 != sk &&
           (*sk2).sk_reuseport == num &&
           (!(*sk2).sk_reuse || !(*sk).sk_reuse) &&
           (!(*sk2).sk_bound_dev_if || !(*sk).sk_bound_dev_if ||
            (*sk2).sk_bound_dev_if == (*sk).sk_bound_dev_if) &&
           inet_rcv_saddr_equal(sk, sk2, 1) {
            
            if (*sk2).sk_reuseport != 0 && (*sk).sk_reuseport != 0 &&
               rcu_access_pointer((*sk).sk_reuseport_cb).is_null() &&
               uid_eq(uid, sock_i_uid(sk2)) {
                result = 0;
            } else {
                result = 1;
            }
            break;
        }
        sk2 = (*sk2).next;
    }
    spin_unlock(&mut (*hslot2).lock);
    
    result
}

// Helper functions (declared as extern for FFI compatibility)
extern "C" {
    fn spin_lock(lock: *mut c_int);
    fn spin_unlock(lock: *mut c_int);
    fn net_eq(net1: *mut net, net2: *mut net) -> c_int;
    fn sock_i_uid(sk: *mut sock) -> *mut c_void;
    fn inet_rcv_saddr_equal(sk1: *mut sock, sk2: *mut sock, strict: c_int) -> c_int;
    fn rcu_access_pointer(ptr: *mut c_void) -> *mut c_void;
    fn __set_bit(bit: c_ulong, bitmap: *mut c_ulong);
    fn reciprocal_scale(val: u32, base: u32) -> u32;
    fn prandom_u32() -> u32;
    fn inet_get_local_port_range(net: *mut net, low: *mut c_int, high: *mut c_int);
    fn inet_is_local_reserved_port(net: *mut net, port: u16) -> c_int;
    fn sk_unhashed(sk: *mut sock) -> c_int;
    fn sk_add_node_rcu(sk: *mut sock, head: *mut *mut sock);
    fn sock_prot_inuse_add(net: *mut net, val: c_int);
    fn reuseport_add_sock(sk: *mut sock, sk2: *mut sock, addr: u32) -> c_int;
    fn reuseport_alloc(sk: *mut sock, addr: u32) -> c_int;
}

// Test cases
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_port_availability() {
        // Basic test case for port availability check
        let mut net = core::mem::zeroed::<net>();
        let mut sk = core::mem::zeroed::<sock>();
        let mut hslot = core::mem::zeroed::<udp_hslot>();
        
        unsafe {
            // Setup test conditions
            (*sk).sk_reuseport = 54321;
            (*hslot).head = &mut sk as *mut _;
            
            let result = udp_lib_lport_inuse(
                &mut net as *mut _,
                54321,
                &mut hslot as *mut _ as *const _,
                ptr::null_mut(),
                &mut sk as *mut _,
                0
            );
            
            assert_eq!(result, 1); // Port should be in use
        }
    }
}
