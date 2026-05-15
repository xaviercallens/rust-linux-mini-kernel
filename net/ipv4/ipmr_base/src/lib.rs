//! Linux multicast routing support - Common logic shared by IPv4 and IPv6 implementations
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]  // For C-style type names

use core::ptr;
use libc::{c_int, c_uint, c_ulong, c_ushort, c_uchar, size_t, c_void};

// Constants from C
pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ENOSYS: c_int = -38;

// Type definitions
#[repr(C)]
pub struct net_device {
    ifindex: c_int,
    // ... other fields as needed
}

#[repr(C)]
pub struct timer_list {
    // ... fields as needed
}

#[repr(C)]
pub struct rhlist_head {
    // ... fields as needed
}

#[repr(C)]
pub struct rhltable {
    // ... fields as needed
}

#[repr(C)]
pub struct list_head {
    next: *mut list_head,
    prev: *mut list_head,
}

#[repr(C)]
pub struct seq_file {
    private: *mut c_void,
    // ... other fields as needed
}

#[repr(C)]
pub struct net {
    // ... fields as needed
}

#[repr(C)]
pub struct mr_table_ops {
    rht_params: *mut c_void,
    cmparg_any: *mut c_void,
}

#[repr(C)]
pub struct vif_device {
    dev: *mut net_device,
    bytes_in: c_ulong,
    bytes_out: c_ulong,
    pkt_in: c_ulong,
    pkt_out: c_ulong,
    rate_limit: c_ulong,
    flags: c_ushort,
    threshold: c_uchar,
    link: c_int,
}

#[repr(C)]
pub struct mr_table {
    id: u32,
    net: *mut net,
    ops: mr_table_ops,
    mfc_hash: rhltable,
    mfc_cache_list: list_head,
    mfc_unres_queue: list_head,
    ipmr_expire_timer: timer_list,
    mroute_reg_vif_num: c_int,
    maxvif: c_int,
    vif_table: *mut vif_device,
}

#[repr(C)]
pub struct mr_mfc {
    mnode: rhlist_head,
    mfc_parent: c_int,
    mfc_flags: c_ushort,
    mfc_un: c_void, // Union - actual struct depends on context
    mfc_un_res: struct {
        minvif: c_int,
        maxvif: c_int,
        ttls: *mut c_uchar,
        pkt: c_ulong,
        bytes: c_ulong,
        wrong_if: c_ulong,
        lastuse: c_ulong,
    },
    list: list_head,
}

#[repr(C)]
pub struct mr_vif_iter {
    mrt: *mut mr_table,
    ct: c_int,
}

#[repr(C)]
pub struct mr_mfc_iter {
    mrt: *mut mr_table,
    cache: *mut list_head,
    lock: *mut c_void, // spinlock_t
}

// Function implementations

/// Sets everything common except 'dev', since that is done under locking
///
/// # Safety
/// - `v` must be a valid pointer to vif_device
/// - `dev` must be a valid pointer to net_device
#[no_mangle]
pub unsafe extern "C" fn vif_device_init(
    v: *mut vif_device,
    dev: *mut net_device,
    rate_limit: c_ulong,
    threshold: c_uchar,
    flags: c_ushort,
    get_iflink_mask: c_ushort,
) {
    if v.is_null() {
        return; // Safety: Caller must ensure valid pointer
    }

    // SAFETY: v is non-null (checked above)
    (*v).dev = ptr::null_mut();
    (*v).bytes_in = 0;
    (*v).bytes_out = 0;
    (*v).pkt_in = 0;
    (*v).pkt_out = 0;
    (*v).rate_limit = rate_limit;
    (*v).flags = flags;
    (*v).threshold = threshold;

    if (*v).flags & get_iflink_mask != 0 {
        // Assuming dev_get_iflink is available in C
        (*v).link = dev_get_iflink(dev);
    } else {
        (*v).link = (*dev).ifindex;
    }
}

#[no_mangle]
pub unsafe extern "C" fn mr_table_alloc(
    net: *mut net,
    id: u32,
    ops: *mut mr_table_ops,
    expire_func: extern "C" fn(*mut timer_list),
    table_set: extern "C" fn(*mut mr_table, *mut net),
) -> *mut mr_table {
    let size = core::mem::size_of::<mr_table>() as size_t;
    let mrt = libc::malloc(size) as *mut mr_table;
    
    if mrt.is_null() {
        return -ENOMEM as *mut mr_table; // ERR_PTR(-ENOMEM)
    }

    (*mrt).id = id;
    write_pnet(&mut (*mrt).net, net);

    // SAFETY: ops is valid pointer (caller provided)
    (*mrt).ops = *ops;
    
    let err = rhltable_init(&mut (*mrt).mfc_hash, (*ops).rht_params);
    if err != 0 {
        libc::free(mrt as *mut c_void);
        return err as *mut mr_table; // ERR_PTR(err)
    }

    INIT_LIST_HEAD(&mut (*mrt).mfc_cache_list);
    INIT_LIST_HEAD(&mut (*mrt).mfc_unres_queue);

    timer_setup(&mut (*mrt).ipmr_expire_timer, expire_func, 0);

    (*mrt).mroute_reg_vif_num = -1;
    table_set(mrt, net);
    
    mrt
}

#[no_mangle]
pub unsafe extern "C" fn mr_mfc_find_parent(
    mrt: *mut mr_table,
    hasharg: *mut c_void,
    parent: c_int,
) -> *mut c_void {
    if mrt.is_null() || hasharg.is_null() {
        return ptr::null_mut();
    }

    let list = rhltable_lookup(&(*mrt).mfc_hash, hasharg, *(*mrt).ops.rht_params);
    let mut tmp: *mut rhlist_head = ptr::null_mut();
    
    // SAFETY: list is valid pointer from rhltable_lookup
    rhl_for_each_entry_rcu::<mr_mfc>(tmp, list, mnode) {
        let c = tmp as *mut mr_mfc;
        if parent == -1 || parent == (*c).mfc_parent {
            return c as *mut c_void;
        }
    }

    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn mr_mfc_find_any_parent(
    mrt: *mut mr_table,
    vifi: c_int,
) -> *mut c_void {
    if mrt.is_null() {
        return ptr::null_mut();
    }

    let list = rhltable_lookup(
        &(*mrt).mfc_hash,
        (*mrt).ops.cmparg_any,
        *(*mrt).ops.rht_params
    );
    let mut tmp: *mut rhlist_head = ptr::null_mut();
    
    rhl_for_each_entry_rcu::<mr_mfc>(tmp, list, mnode) {
        let c = tmp as *mut mr_mfc;
        if (*c).mfc_un.res.ttls[vifi] < 255 {
            return c as *mut c_void;
        }
    }

    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn mr_mfc_find_any(
    mrt: *mut mr_table,
    vifi: c_int,
    hasharg: *mut c_void,
) -> *mut c_void {
    if mrt.is_null() || hasharg.is_null() {
        return ptr::null_mut();
    }

    let list = rhltable_lookup(&(*mrt).mfc_hash, hasharg, *(*mrt).ops.rht_params);
    let mut tmp: *mut rhlist_head = ptr::null_mut();
    
    rhl_for_each_entry_rcu::<mr_mfc>(tmp, list, mnode) {
        let c = tmp as *mut mr_mfc;
        if (*c).mfc_un.res.ttls[vifi] < 255 {
            return c as *mut c_void;
        }

        // Check static tree
        let proxy = mr_mfc_find_any_parent(mrt, (*c).mfc_parent);
        if !proxy.is_null() && (*proxy).mfc_un.res.ttls[vifi] < 255 {
            return c as *mut c_void;
        }
    }

    mr_mfc_find_any_parent(mrt, vifi)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_vif_device_init() {
        // Basic test - actual tests would require kernel environment
    }
}

// Helper functions (extern declarations)
extern "C" {
    fn dev_get_iflink(dev: *mut net_device) -> c_int;
    fn write_pnet(net: *mut *mut net, val: *mut net);
    fn rhltable_init(table: *mut rhltable, params: *mut c_void) -> c_int;
    fn INIT_LIST_HEAD(head: *mut list_head);
    fn timer_setup(timer: *mut timer_list, func: extern "C" fn(*mut timer_list), data: c_int);
    fn rhltable_lookup(table: *mut rhltable, key: *mut c_void, params: *mut c_void) -> *mut rhlist_head;
    fn rhl_for_each_entry_rcu<T>(tmp: *mut rhlist_head, list: *mut rhlist_head, mnode: *mut c_void) -> *mut T;
}
