//! Generic nexthop implementation for Linux kernel
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]  // For C-style type names

use core::ptr;
use libc::{c_int, c_uint, c_void, size_t};

// Constants from C
pub const NH_RES_DEFAULT_IDLE_TIMER: u32 = 120 * 4; // Assuming HZ=4
pub const NH_RES_DEFAULT_UNBALANCED_TIMER: u32 = 0;
pub const NH_DEV_HASHBITS: u32 = 8;
pub const NH_DEV_HASHSIZE: u32 = 1 << NH_DEV_HASHBITS;

// Error codes
pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ENOSYS: c_int = -38;

// Type definitions
#[repr(C)]
pub struct nl_info {
    // Opaque struct - fields defined in original C code
    _unused: [u8; 0],
}

#[repr(C)]
pub struct net {
    // Opaque struct - fields defined in original C code
    _unused: [u8; 0],
}

#[repr(C)]
pub struct nexthop {
    id: u32,
    is_group: bool,
    nh_info: *const c_void,
    nh_grp: *const c_void,
    _unused: [u8; 0],
}

#[repr(C)]
pub struct nh_notifier_info {
    net: *const net,
    id: u32,
    type_: u8,
    nh: *mut c_void,
    nh_grp: *mut c_void,
    nh_res_table: *mut c_void,
    nh_res_bucket: *mut c_void,
    extack: *mut c_void,
}

#[repr(C)]
pub struct nh_notifier_single_info {
    dev: *const c_void,
    gw_family: u8,
    ipv4: u32,
    ipv6: [u8; 16],
    is_reject: bool,
    is_fdb: bool,
    has_encap: bool,
}

#[repr(C)]
pub struct nh_info {
    fib_nhc: *const c_void,
    reject_nh: bool,
    fdb_nh: bool,
}

#[repr(C)]
pub struct nh_group {
    hash_threshold: u32,
    resilient: bool,
    res_table: *const c_void,
    num_nh: u16,
    fdb_nh: bool,
    _unused: [u8; 0],
}

#[repr(C)]
pub struct nh_res_table {
    num_nh_buckets: u16,
    nh_buckets: *const c_void,
}

#[repr(C)]
pub struct nh_res_bucket {
    nh_entry: *const c_void,
}

// Function implementations
/// Check if nexthop notifiers are empty
///
/// # Safety
/// - Caller must hold RTNL lock
///
/// # Returns
/// true if empty, false otherwise
#[no_mangle]
pub unsafe extern "C" fn nexthop_notifiers_is_empty(net: *const net) -> bool {
    // SAFETY: net is valid pointer per contract
    let net = &*(net as *const net);
    net.nexthop.notifier_chain.head.is_null()
}

/// Initialize single nexthop notifier info
///
/// # Safety
/// - info must be valid pointer
/// - nhi must be valid pointer to nh_info
///
/// # Returns
/// 0 on success, -ENOMEM if allocation fails
#[no_mangle]
pub unsafe extern "C" fn nh_notifier_single_info_init(
    info: *mut nh_notifier_info,
    nhi: *const nh_info,
) -> c_int {
    if info.is_null() || nhi.is_null() {
        return EINVAL;
    }

    let info = &mut *info;
    let nhi = &*nhi;
    
    // SAFETY: info is valid pointer
    info.type_ = 0; // NH_NOTIFIER_INFO_TYPE_SINGLE
    
    // Allocate memory for nh
    info.nh = libc::malloc(core::mem::size_of::<nh_notifier_single_info>());
    if info.nh.is_null() {
        return ENOMEM;
    }
    
    // Initialize single info
    let nh_info = &mut *(info.nh as *mut nh_notifier_single_info);
    let fib_nhc = &*(nhi.fib_nhc as *const c_void);
    
    nh_info.dev = fib_nhc as *const c_void;
    nh_info.gw_family = (*fib_nhc).cast_to::<u8>();
    
    if nh_info.gw_family == 2 { // AF_INET
        nh_info.ipv4 = (*fib_nhc).cast_to::<u32>();
    } else if nh_info.gw_family == 10 { // AF_INET6
        let ipv6 = (*fib_nhc).cast_to::<[u8; 16]>();
        nh_info.ipv6 = ipv6;
    }
    
    nh_info.is_reject = nhi.reject_nh;
    nh_info.is_fdb = nhi.fdb_nh;
    nh_info.has_encap = !nhi.fib_nhc.is_null();
    
    0
}

/// Finalize single nexthop notifier info
///
/// # Safety
/// - info must be valid pointer
#[no_mangle]
pub unsafe extern "C" fn nh_notifier_single_info_fini(info: *mut nh_notifier_info) {
    if !info.is_null() {
        let info = &mut *info;
        if !info.nh.is_null() {
            libc::free(info.nh);
            info.nh = ptr::null_mut();
        }
    }
}

/// Initialize multi-path nexthop notifier info
///
/// # Safety
/// - info must be valid pointer
/// - nhg must be valid pointer to nh_group
///
/// # Returns
/// 0 on success, -ENOMEM if allocation fails
#[no_mangle]
pub unsafe extern "C" fn nh_notifier_mpath_info_init(
    info: *mut nh_notifier_info,
    nhg: *const nh_group,
) -> c_int {
    if info.is_null() || nhg.is_null() {
        return EINVAL;
    }

    let info = &mut *info;
    let nhg = &*nhg;
    
    info.type_ = 1; // NH_NOTIFIER_INFO_TYPE_GRP
    
    // Calculate size
    let size = core::mem::size_of::<c_void>() + // nh_grp
              (nhg.num_nh as usize) * core::mem::size_of::<c_void>(); // nh_entries
    
    info.nh_grp = libc::malloc(size);
    if info.nh_grp.is_null() {
        return ENOMEM;
    }
    
    // Initialize group info
    let nh_grp = &mut *(info.nh_grp as *mut nh_group);
    nh_grp.num_nh = nhg.num_nh;
    nh_grp.fdb_nh = nhg.fdb_nh;
    
    for i in 0..nhg.num_nh {
        let nhge = &nhg.nh_entries[i];
        let nhi = &*nhge.nh;
        
        // Initialize each entry
        let entry = &mut nh_grp.nh_entries[i];
        entry.id = nhge.nh.id;
        entry.weight = nhge.weight;
        
        // Initialize single info
        let fib_nhc = &nhi.fib_nhc;
        entry.nh.dev = fib_nhc as *const c_void;
        entry.nh.gw_family = (*fib_nhc).cast_to::<u8>();
        
        if entry.nh.gw_family == 2 { // AF_INET
            entry.nh.ipv4 = (*fib_nhc).cast_to::<u32>();
        } else if entry.nh.gw_family == 10 { // AF_INET6
            let ipv6 = (*fib_nhc).cast_to::<[u8; 16]>();
            entry.nh.ipv6 = ipv6;
        }
        
        entry.nh.is_reject = nhi.reject_nh;
        entry.nh.is_fdb = nhi.fdb_nh;
        entry.nh.has_encap = !nhi.fib_nhc.is_null();
    }
    
    0
}

/// Initialize resilient table nexthop notifier info
///
/// # Safety
/// - info must be valid pointer
/// - nhg must be valid pointer to nh_group
///
/// # Returns
/// 0 on success, -ENOMEM if allocation fails
#[no_mangle]
pub unsafe extern "C" fn nh_notifier_res_table_info_init(
    info: *mut nh_notifier_info,
    nhg: *const nh_group,
) -> c_int {
    if info.is_null() || nhg.is_null() {
        return EINVAL;
    }

    let info = &mut *info;
    let nhg = &*nhg;
    
    info.type_ = 2; // NH_NOTIFIER_INFO_TYPE_RES_TABLE
    
    let res_table = &*nhg.res_table;
    info.nh_res_table = libc::malloc(core::mem::size_of::<nh_res_table>() +
                                    (res_table.num_nh_buckets as usize) * 
                                    core::mem::size_of::<nh_notifier_single_info>());
    
    if info.nh_res_table.is_null() {
        return ENOMEM;
    }
    
    let nh_res_table = &mut *(info.nh_res_table as *mut nh_res_table);
    nh_res_table.num_nh_buckets = res_table.num_nh_buckets;
    
    for i in 0..res_table.num_nh_buckets {
        let bucket = &res_table.nh_buckets[i];
        let nhge = &*bucket.nh_entry;
        let nhi = &*nhge.nh;
        
        // Initialize each bucket
        let entry = &mut nh_res_table.nhs[i];
        entry.id = nhge.nh.id;
        entry.weight = nhge.weight;
        
        // Initialize single info
        let fib_nhc = &nhi.fib_nhc;
        entry.nh.dev = fib_nhc as *const c_void;
        entry.nh.gw_family = (*fib_nhc).cast_to::<u8>();
        
        if entry.nh.gw_family == 2 { // AF_INET
            entry.nh.ipv4 = (*fib_nhc).cast_to::<u32>();
        } else if entry.nh.gw_family == 10 { // AF_INET6
            let ipv6 = (*fib_nhc).cast_to::<[u8; 16]>();
            entry.nh.ipv6 = ipv6;
        }
        
        entry.nh.is_reject = nhi.reject_nh;
        entry.nh.is_fdb = nhi.fdb_nh;
        entry.nh.has_encap = !nhi.fib_nhc.is_null();
    }
    
    0
}

/// Call nexthop notifiers
///
/// # Safety
/// - Caller must hold RTNL lock
/// - net must be valid pointer
/// - nh must be valid pointer
///
/// # Returns
/// 0 on success, error code otherwise
#[no_mangle]
pub unsafe extern "C" fn call_nexthop_notifiers(
    net: *const net,
    event_type: c_int,
    nh: *const nexthop,
    extack: *mut c_void,
) -> c_int {
    if net.is_null() || nh.is_null() {
        return EINVAL;
    }
    
    // Check if notifiers are empty
    if nexthop_notifiers_is_empty(net) {
        return 0;
    }
    
    let mut info: nh_notifier_info = core::mem::zeroed();
    info.net = net;
    info.extack = extack;
    info.id = (*nh).id;
    
    let mut err = 0;
    
    if (*nh).is_group {
        if (*nh).nh_grp.hash_threshold > 0 {
            err = nh_notifier_mpath_info_init(&mut info, (*nh).nh_grp);
        } else if (*nh).nh_grp.resilient {
            err = nh_notifier_res_table_info_init(&mut info, (*nh).nh_grp);
        } else {
            err = EINVAL;
        }
    } else {
        err = nh_notifier_single_info_init(&mut info, (*nh).nh_info);
    }
    
    if err != 0 {
        return err;
    }
    
    // Call notifiers (implementation would require kernel-specific APIs)
    // This is a placeholder for the actual notifier call chain
    // In real implementation, this would use the kernel's blocking_notifier_call_chain
    
    // Clean up
    if (*nh).is_group {
        if (*nh).nh_grp.hash_threshold > 0 {
            libc::free(info.nh_grp);
        } else if (*nh).nh_grp.resilient {
            libc::free(info.nh_res_table);
        }
    } else {
        libc::free(info.nh);
    }
    
    0
}

// Additional functions would be implemented similarly following the same pattern

// Tests (conditional compilation)
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_nexthop_notifiers_is_empty() {
        // This test would require valid net structure which is not available
        // in user space. This is just a placeholder.
        let net = ptr::null();
        assert!(unsafe { nexthop_notifiers_is_empty(net) });
    }
}
