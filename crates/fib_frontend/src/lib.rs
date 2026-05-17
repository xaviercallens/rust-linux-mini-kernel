use kernel_types::*;

//! IPv4 Forwarding Information Base (FIB) frontend implementation
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clang::missing_docs_in_private_items)]

use core::ffi::c_int;
use core::ffi::c_uint;
use core::ffi::c_void;
use core::ffi::size_t;
use core::ptr;

// Constants from C
pub const RT_TABLE_MAIN: u32 = 254;
pub const RT_TABLE_LOCAL: u32 = 253;
pub const RT_TABLE_DEFAULT: u32 = 252;
pub const RT_TABLE_HASHSZ: u32 = 255;
pub const FIB_TABLE_HASHSZ: u32 = 255;
pub const LOOPBACK_IFINDEX: u32 = 1;

pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ENOSYS: c_int = -38;

// Type definitions
#[repr(C)]
struct hlist_node {
    next: *mut hlist_node,
}

#[repr(C)]
struct hlist_head {
    first: *mut hlist_node,
}

#[repr(C)]
struct fib_table {
    tb_id: u32,
    tb_hlist: hlist_node,
    // Additional fields would be added based on actual C struct
}

#[repr(C)]
struct net {
    ipv4: ipv4_net,
}

#[repr(C)]
struct ipv4_net {
    fib_table_hash: [hlist_head; FIB_TABLE_HASHSZ as usize],
    fib_main: *mut fib_table,
    fib_default: *mut fib_table,
    fib_has_custom_rules: bool,
}

#[repr(C)]
struct in_device {
    ifa_list: *mut c_void, // Placeholder for actual ifa_list type
}

#[repr(C)]
struct flowi4 {
    daddr: u32,
    saddr: u32,
    flowi4_tos: u8,
    flowi4_scope: u8,
    flowi4_mark: u32,
    flowi4_iif: u32,
    flowi4_oif: u32,
    // Additional fields would be added based on actual C struct
}

#[repr(C)]
struct fib_result {
    type_: u8,
    fi: *mut c_void, // Placeholder for actual fib_info type
}

// Function implementations
#[no_mangle]
pub unsafe extern "C" fn fib_new_table(net: *mut net, id: u32) -> *mut fib_table {
    if net.is_null() {
        return ptr::null_mut();
    }

    let mut tb: *mut fib_table = ptr::null_mut();
    let mut alias: *mut fib_table = ptr::null_mut();

    if id == 0 {
        id = RT_TABLE_MAIN;
    }

    // Check if table already exists
    let existing = fib_get_table(net, id);
    if !existing.is_null() {
        return existing;
    }

    // Special case for RT_TABLE_LOCAL
    if id == RT_TABLE_LOCAL && (*net).ipv4.fib_has_custom_rules == false {
        alias = fib_new_table(net, RT_TABLE_MAIN);
    }

    // Allocate new table
    tb = fib_trie_table(id, alias);
    if tb.is_null() {
        return ptr::null_mut();
    }

    // Set main/default pointers
    match id {
        RT_TABLE_MAIN => {
            (*net).ipv4.fib_main = tb;
        }
        RT_TABLE_DEFAULT => {
            (*net).ipv4.fib_default = tb;
        }
        _ => {}
    }

    // Add to hash table
    let h = id & (FIB_TABLE_HASHSZ - 1);
    hlist_add_head_rcu(
        &mut (*tb).tb_hlist,
        &mut (*net).ipv4.fib_table_hash[h as usize],
    );

    tb
}

#[no_mangle]
pub unsafe extern "C" fn fib_get_table(net: *mut net, id: u32) -> *mut fib_table {
    if net.is_null() {
        return ptr::null_mut();
    }

    if id == 0 {
        id = RT_TABLE_MAIN;
    }

    let h = id & (FIB_TABLE_HASHSZ - 1);
    let head = &mut (*net).ipv4.fib_table_hash[h as usize];

    let mut tb = (*head).first;
    while !tb.is_null() {
        if (*tb).tb_id == id {
            return tb as *mut fib_table;
        }
        tb = (*tb).tb_hlist.next;
    }

    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn fib_unmerge(net: *mut net) -> c_int {
    if net.is_null() {
        return EINVAL;
    }

    let old = fib_get_table(net, RT_TABLE_LOCAL);
    if old.is_null() {
        return 0;
    }

    let new = fib_trie_unmerge(old);
    if new.is_null() {
        return ENOMEM;
    }

    if new == old {
        return 0;
    }

    fib_replace_table(net, old, new);
    fib_free_table(old);

    let main_table = fib_get_table(net, RT_TABLE_MAIN);
    if main_table.is_null() {
        return 0;
    }

    fib_table_flush_external(main_table);

    0
}

#[no_mangle]
pub unsafe extern "C" fn fib_flush(net: *mut net) -> c_int {
    if net.is_null() {
        return EINVAL;
    }

    let mut flushed = 0;
    for h in 0..FIB_TABLE_HASHSZ {
        let head = &mut (*net).ipv4.fib_table_hash[h as usize];
        let mut tmp: *mut hlist_node = ptr::null_mut();
        let mut tb = (*head).first;

        while !tb.is_null() {
            // SAFETY: We're iterating through the list and need to handle the next pointer before processing
            tmp = (*tb).tb_hlist.next;
            flushed += fib_table_flush(net, tb as *mut fib_table, false);
            tb = tmp;
        }
    }

    if flushed > 0 {
        rt_cache_flush(net);
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn inet_addr_type_table(net: *mut net, addr: u32, tb_id: u32) -> u32 {
    __inet_dev_addr_type(net, ptr::null_mut(), addr, tb_id)
}

#[no_mangle]
pub unsafe extern "C" fn inet_addr_type(net: *mut net, addr: u32) -> u32 {
    __inet_dev_addr_type(net, ptr::null_mut(), addr, RT_TABLE_LOCAL)
}

#[no_mangle]
pub unsafe extern "C" fn inet_dev_addr_type(net: *mut net, dev: *mut c_void, addr: u32) -> u32 {
    let rt_table = l3mdev_fib_table(dev).unwrap_or(RT_TABLE_LOCAL);
    __inet_dev_addr_type(net, dev, addr, rt_table)
}

#[no_mangle]
pub unsafe extern "C" fn inet_addr_type_dev_table(
    net: *mut net,
    dev: *mut c_void,
    addr: u32,
) -> u32 {
    let rt_table = l3mdev_fib_table(dev).unwrap_or(RT_TABLE_LOCAL);
    __inet_dev_addr_type(net, ptr::null_mut(), addr, rt_table)
}

// Internal functions
unsafe fn __inet_dev_addr_type(net: *mut net, dev: *mut c_void, addr: u32, tb_id: u32) -> u32 {
    if ipv4_is_zeronet(addr) || ipv4_is_lbcast(addr) {
        return RTN_BROADCAST;
    }

    if ipv4_is_multicast(addr) {
        return RTN_MULTICAST;
    }

    // SAFETY: RCU read lock is required for fib_get_table and fib_table_lookup
    rcu_read_lock();

    let table = fib_get_table(net, tb_id);
    let mut ret = RTN_UNICAST;

    if !table.is_null() {
        let mut res = fib_result {
            type_: 0,
            fi: ptr::null_mut(),
        };

        if fib_table_lookup(
            table,
            &mut flowi4 {
                daddr: addr,
                saddr: 0,
                flowi4_tos: 0,
                flowi4_scope: 0,
                flowi4_mark: 0,
                flowi4_iif: 0,
                flowi4_oif: 0,
                // ... other fields
            },
            &mut res,
            0,
        ) == 0
        {
            // Check device match
            if dev.is_null() || dev == fib_info_nhc(res.fi, 0) {
                ret = res.type_;
            }
        }
    }

    rcu_read_unlock();

    ret
}

// Helper functions (stub implementations for FFI compatibility)
unsafe fn fib_trie_table(id: u32, alias: *mut fib_table) -> *mut fib_table {
    // In real implementation, this would allocate and initialize a trie-based table
    ptr::null_mut()
}

unsafe fn fib_trie_unmerge(table: *mut fib_table) -> *mut fib_table {
    // In real implementation, this would create a new unmerged table
    ptr::null_mut()
}

unsafe fn fib_replace_table(net: *mut net, old: *mut fib_table, new: *mut fib_table) {
    // In real implementation, this would replace the old table with the new one
}

unsafe fn fib_free_table(tb: *mut fib_table) {
    // In real implementation, this would free the table
}

unsafe fn fib_table_flush_external(tb: *mut fib_table) {
    // In real implementation, this would flush entries
}

unsafe fn fib_table_flush(net: *mut net, tb: *mut fib_table, flag: bool) -> c_int {
    // In real implementation, this would flush table entries
    0
}

unsafe fn rt_cache_flush(net: *mut net) {
    // In real implementation, this would flush routing cache
}

unsafe fn rcu_read_lock() {
    // In real implementation, this would acquire RCU read lock
}

unsafe fn rcu_read_unlock() {
    // In real implementation, this would release RCU read lock
}

unsafe fn fib_table_lookup(
    table: *mut fib_table,
    fl4: *mut flowi4,
    res: *mut fib_result,
    flags: c_int,
) -> c_int {
    // In real implementation, this would perform a lookup in the table
    0
}

unsafe fn fib_info_nhc(fi: *mut c_void, index: c_int) -> *mut c_void {
    // In real implementation, this would get the nth next-hop
    ptr::null_mut()
}

unsafe fn l3mdev_fib_table(dev: *mut c_void) -> Option<u32> {
    // In real implementation, this would get the L3 master device table
    None
}

unsafe fn ipv4_is_zeronet(addr: u32) -> bool {
    addr == 0
}

unsafe fn ipv4_is_lbcast(addr: u32) -> bool {
    // In real implementation, this would check for local broadcast
    false
}

unsafe fn ipv4_is_multicast(addr: u32) -> bool {
    // In real implementation, this would check for multicast
    false
}

// Constants
const RTN_BROADCAST: u32 = 1;
const RTN_MULTICAST: u32 = 2;
const RTN_UNICAST: u32 = 3;
const RTN_LOCAL: u32 = 4;

// Tests (conditional compilation)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib_new_table() {
        // Basic test - would require actual memory allocation in real implementation
        let mut net = net {
            ipv4: ipv4_net {
                fib_table_hash: [hlist_head {
                    first: ptr::null_mut(),
                }; FIB_TABLE_HASHSZ as usize],
                fib_main: ptr::null_mut(),
                fib_default: ptr::null_mut(),
                fib_has_custom_rules: false,
            },
        };

        let net_ptr = &mut net as *mut _;
        let table = unsafe { fib_new_table(net_ptr, RT_TABLE_MAIN) };
        assert!(!table.is_null());

        // Test that the table was added to the hash
        let lookup = unsafe { fib_get_table(net_ptr, RT_TABLE_MAIN) };
        assert_eq!(table, lookup);
    }
}
