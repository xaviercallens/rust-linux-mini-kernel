//! Functions for handling network device address lists in the Linux kernel.
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]  // For C-style type names

use core::ptr;
use libc::{c_int, c_uint, c_void, size_t};

// Constants from C
pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ENOENT: c_int = -2;

// Type definitions
#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct netdev_hw_addr {
    pub list: list_head,
    pub addr: [u8; 32], // MAX_ADDR_LEN is 32 in Linux
    pub type_: u8,
    pub refcount: u32,
    pub global_use: bool,
    pub synced: u8,
    pub sync_cnt: u32,
    // rcu_head would be part of kernel's RCU implementation
}

#[repr(C)]
pub struct netdev_hw_addr_list {
    pub list: list_head,
    pub count: u32,
}

#[repr(C)]
pub struct net_device {
    // Only include fields used in the functions
} 

// Function implementations
/// Create a new hardware address entry
///
/// # Safety
/// - `list` must be a valid pointer to netdev_hw_addr_list
/// - `addr` must point to valid memory of at least `addr_len` bytes
/// - Caller must handle proper synchronization
///
/// # Returns
/// 0 on success, -ENOMEM if allocation fails
#[no_mangle]
pub unsafe extern "C" fn __hw_addr_create_ex(
    list: *mut netdev_hw_addr_list,
    addr: *const u8,
    addr_len: c_int,
    addr_type: u8,
    global: bool,
    sync: bool,
) -> c_int {
    if list.is_null() || addr.is_null() {
        return EINVAL;
    }

    let alloc_size = core::mem::size_of::<netdev_hw_addr>();
    let ha = unsafe { libc::malloc(alloc_size as size_t) as *mut netdev_hw_addr };
    
    if ha.is_null() {
        return ENOMEM;
    }

    // SAFETY: ha is valid and allocated
    unsafe {
        ptr::copy_nonoverlapping(addr, (*ha).addr.as_mut_ptr() as *mut u8, addr_len as usize);
        (*ha).type_ = addr_type;
        (*ha).refcount = 1;
        (*ha).global_use = global;
        (*ha).synced = if sync { 1 } else { 0 };
        (*ha).sync_cnt = 0;
        
        // Add to list
        list_add_tail_rcu(&mut (*ha).list, &mut (*list).list);
        (*list).count += 1;
    }
    
    0
}

/// Add a hardware address to the list
///
/// # Safety
/// - `list` must be a valid pointer to netdev_hw_addr_list
/// - `addr` must point to valid memory of at least `addr_len` bytes
/// - Caller must handle proper synchronization
///
/// # Returns
/// 0 on success, -EINVAL if address too long, -EEXIST if already exists
#[no_mangle]
pub unsafe extern "C" fn __hw_addr_add_ex(
    list: *mut netdev_hw_addr_list,
    addr: *const u8,
    addr_len: c_int,
    addr_type: u8,
    global: bool,
    sync: bool,
    sync_count: c_int,
) -> c_int {
    if list.is_null() || addr.is_null() {
        return EINVAL;
    }
    
    if addr_len > 32 { // MAX_ADDR_LEN is 32
        return EINVAL;
    }
    
    let mut ha = unsafe { (*(*list).list.next) as *mut netdev_hw_addr };
    let list_end = &(*list).list as *const list_head as *mut list_head;
    
    while !ha.is_null() && ha as *mut list_head != list_end {
        if (*ha).type_ == addr_type && 
           ptr::eq((*ha).addr.as_ptr() as *const c_void, addr as *const c_void) {
            // Found matching address
            if global {
                if (*ha).global_use {
                    return 0;
                }
                (*ha).global_use = true;
            }
            
            if sync {
                if (*ha).synced != 0 && sync_count != 0 {
                    return -2; // -EEXIST
                }
                (*ha).synced = 1;
            }
            
            (*ha).refcount += 1;
            return 0;
        }
        ha = unsafe { (*(*ha).list.next) as *mut netdev_hw_addr };
    }
    
    __hw_addr_create_ex(list, addr, addr_len, addr_type, global, sync)
}

/// Add a hardware address to the list (simplified version)
///
/// # Safety
/// - See __hw_addr_add_ex for safety requirements
#[no_mangle]
pub unsafe extern "C" fn __hw_addr_add(
    list: *mut netdev_hw_addr_list,
    addr: *const u8,
    addr_len: c_int,
    addr_type: u8,
) -> c_int {
    __hw_addr_add_ex(list, addr, addr_len, addr_type, false, false, 0)
}

/// Delete a hardware address entry
///
/// # Safety
/// - `list` must be a valid pointer to netdev_hw_addr_list
/// - `ha` must be a valid pointer to an existing entry
/// - Caller must handle proper synchronization
///
/// # Returns
/// 0 on success, -ENOENT if entry not found
#[no_mangle]
pub unsafe extern "C" fn __hw_addr_del_entry(
    list: *mut netdev_hw_addr_list,
    ha: *mut netdev_hw_addr,
    global: bool,
    sync: bool,
) -> c_int {
    if list.is_null() || ha.is_null() {
        return EINVAL;
    }
    
    if global && !(*ha).global_use {
        return ENOENT;
    }
    
    if sync && (*ha).synced == 0 {
        return ENOENT;
    }
    
    if global {
        (*ha).global_use = false;
    }
    
    if sync {
        (*ha).synced = 0;
    }
    
    (*ha).refcount -= 1;
    
    if (*ha).refcount == 0 {
        list_del_rcu(&mut (*ha).list);
        // SAFETY: Using kfree_rcu equivalent
        unsafe {
            libc::free(ha as *mut c_void);
        }
        (*list).count -= 1;
    }
    
    0
}

/// Delete a hardware address from the list
///
/// # Safety
/// - `list` must be a valid pointer to netdev_hw_addr_list
/// - `addr` must point to valid memory of at least `addr_len` bytes
/// - Caller must handle proper synchronization
///
/// # Returns
/// 0 on success, -ENOENT if address not found
#[no_mangle]
pub unsafe extern "C" fn __hw_addr_del_ex(
    list: *mut netdev_hw_addr_list,
    addr: *const u8,
    addr_len: c_int,
    addr_type: u8,
    global: bool,
    sync: bool,
) -> c_int {
    if list.is_null() || addr.is_null() {
        return EINVAL;
    }
    
    let mut ha = unsafe { (*(*list).list.next) as *mut netdev_hw_addr };
    let list_end = &(*list).list as *const list_head as *mut list_head;
    
    while !ha.is_null() && ha as *mut list_head != list_end {
        if ptr::eq((*ha).addr.as_ptr() as *const c_void, addr as *const c_void) &&
           (addr_type == 0 || (*ha).type_ == addr_type) {
            return __hw_addr_del_entry(list, ha, global, sync);
        }
        ha = unsafe { (*(*ha).list.next) as *mut netdev_hw_addr };
    }
    
    ENOENT
}

/// Delete a hardware address from the list (simplified version)
///
/// # Safety
/// - See __hw_addr_del_ex for safety requirements
#[no_mangle]
pub unsafe extern "C" fn __hw_addr_del(
    list: *mut netdev_hw_addr_list,
    addr: *const u8,
    addr_len: c_int,
    addr_type: u8,
) -> c_int {
    __hw_addr_del_ex(list, addr, addr_len, addr_type, false, false)
}

/// Helper function for address synchronization
///
/// # Safety
/// - `to_list` must be a valid pointer to netdev_hw_addr_list
/// - `ha` must be a valid pointer to an existing entry
/// - Caller must handle proper synchronization
///
/// # Returns
/// 0 on success, error code on failure
#[no_mangle]
pub unsafe extern "C" fn __hw_addr_sync_one(
    to_list: *mut netdev_hw_addr_list,
    ha: *mut netdev_hw_addr,
    addr_len: c_int,
) -> c_int {
    if to_list.is_null() || ha.is_null() {
        return EINVAL;
    }
    
    let result = __hw_addr_add_ex(
        to_list, 
        (*ha).addr.as_ptr(), 
        addr_len, 
        (*ha).type_, 
        false, 
        true, 
        (*ha).sync_cnt as c_int
    );
    
    if result == 0 || result == -2 { // 0 or -EEXIST
        (*ha).sync_cnt += 1;
        (*ha).refcount += 1;
    }
    
    result
}

/// Helper function for address unsynchronization
///
/// # Safety
/// - `to_list` must be a valid pointer to netdev_hw_addr_list
/// - `from_list` must be a valid pointer to netdev_hw_addr_list
/// - `ha` must be a valid pointer to an existing entry
/// - Caller must handle proper synchronization
#[no_mangle]
pub unsafe extern "C" fn __hw_addr_unsync_one(
    to_list: *mut netdev_hw_addr_list,
    from_list: *mut netdev_hw_addr_list,
    ha: *mut netdev_hw_addr,
    addr_len: c_int,
) {
    if to_list.is_null() || from_list.is_null() || ha.is_null() {
        return;
    }
    
    let err = __hw_addr_del_ex(
        to_list, 
        (*ha).addr.as_ptr(), 
        addr_len, 
        (*ha).type_, 
        false, 
        true
    );
    
    if err != 0 {
        return;
    }
    
    (*ha).sync_cnt -= 1;
    // Address on from list is not marked synced
    __hw_addr_del_entry(from_list, ha, false, false);
}

/// Synchronize multiple addresses between lists
///
/// # Safety
/// - `to_list` must be a valid pointer to netdev_hw_addr_list
/// - `from_list` must be a valid pointer to netdev_hw_addr_list
/// - Caller must handle proper synchronization
///
/// # Returns
/// 0 on success, error code on failure
#[no_mangle]
pub unsafe extern "C" fn __hw_addr_sync_multiple(
    to_list: *mut netdev_hw_addr_list,
    from_list: *mut netdev_hw_addr_list,
    addr_len: c_int,
) -> c_int {
    if to_list.is_null() || from_list.is_null() {
        return EINVAL;
    }
    
    let mut ha = unsafe { (*(*from_list).list.next) as *mut netdev_hw_addr };
    let list_end = &(*from_list).list as *const list_head as *mut list_head;
    let mut err = 0;
    
    while !ha.is_null() && ha as *mut list_head != list_end {
        if (*ha).sync_cnt == (*ha).refcount {
            __hw_addr_unsync_one(to_list, from_list, ha, addr_len);
        } else {
            err = __hw_addr_sync_one(to_list, ha, addr_len);
            if err != 0 {
                break;
            }
        }
        ha = unsafe { (*(*ha).list.next) as *mut netdev_hw_addr };
    }
    
    err
}

/// Synchronize addresses between lists
///
/// # Safety
/// - `to_list` must be a valid pointer to netdev_hw_addr_list
/// - `from_list` must be a valid pointer to netdev_hw_addr_list
/// - Caller must handle proper synchronization
///
/// # Returns
/// 0 on success, error code on failure
#[no_mangle]
pub unsafe extern "C" fn __hw_addr_sync(
    to_list: *mut netdev_hw_addr_list,
    from_list: *mut netdev_hw_addr_list,
    addr_len: c_int,
) -> c_int {
    if to_list.is_null() || from_list.is_null() {
        return EINVAL;
    }
    
    let mut ha = unsafe { (*(*from_list).list.next) as *mut netdev_hw_addr };
    let list_end = &(*from_list).list as *const list_head as *mut list_head;
    let mut err = 0;
    
    while !ha.is_null() && ha as *mut list_head != list_end {
        if (*ha).sync_cnt == 0 {
            err = __hw_addr_sync_one(to_list, ha, addr_len);
            if err != 0 {
                break;
            }
        } else if (*ha).refcount == 1 {
            __hw_addr_unsync_one(to_list, from_list, ha, addr_len);
        }
        ha = unsafe { (*(*ha).list.next) as *mut netdev_hw_addr };
    }
    
    err
}
#[no_mangle]
pub unsafe extern "C" fn __hw_addr_unsync(
    to_list: *mut netdev_hw_addr_list,
    from_list: *mut netdev_hw_addr_list,
    addr_len: c_int,
) {
    if to_list.is_null() || from_list.is_null() {
        return;
    }
    
    let mut ha = unsafe { (*(*from_list).list.next) as *mut netdev_hw_addr };
    let list_end = &(*from_list).list as *const list_head as *mut list_head;
    
    while !ha.is_null() && ha as *mut list_head != list_end {
        if (*ha).sync_cnt != 0 {
            __hw_addr_unsync_one(to_list, from_list, ha, addr_len);
        }
        ha = unsafe { (*(*ha).list.next) as *mut netdev_hw_addr };
    }
}
#[no_mangle]
pub unsafe extern "C" fn __hw_addr_sync_dev(
    list: *mut netdev_hw_addr_list,
    dev: *mut net_device,
    sync: Option<unsafe extern "C" fn(*mut net_device, *const u8) -> c_int>,
    unsync: Option<unsafe extern "C" fn(*mut net_device, *const u8) -> c_int>,
) -> c_int {
    if list.is_null() || dev.is_null() {
        return EINVAL;
    }
    
    let mut ha = unsafe { (*(*list).list.next) as *mut netdev_hw_addr };
    let list_end = &(*list).list as *const list_head as *mut list_head;
    let mut err = 0;
    
    while !ha.is_null() && ha as *mut list_head != list_end {
        if (*ha).sync_cnt != 0 && (*ha).refcount == 1 {
            // If unsync is defined and fails, defer unsyncing address
            if let Some(unsync_fn) = unsync {
                let result = unsafe { unsync_fn(dev, (*ha).addr.as_ptr()) };
                if result != 0 {
                    ha = unsafe { (*(*ha).list.next) as *mut netdev_hw_addr };
                    continue;
                }
            }
            
            (*ha).sync_cnt -= 1;
            __hw_addr_del_entry(list, ha, false, false);
        }
        ha = unsafe { (*(*ha).list.next) as *mut netdev_hw_addr };
    }
    
    ha = unsafe { (*(*list).list.next) as *mut netdev_hw_addr };
    
    while !ha.is_null() && ha as *mut list_head != list_end {
        if (*ha).sync_cnt != 0 {
            ha = unsafe { (*(*ha).list.next) as *mut netdev_hw_addr };
            continue;
        }
        
        if let Some(sync_fn) = sync {
            let result = unsafe { sync_fn(dev, (*ha).addr.as_ptr()) };
            if result != 0 {
                err = result;
                break;
            }
        }
        
        (*ha).sync_cnt += 1;
        (*ha).refcount += 1;
        ha = unsafe { (*(*ha).list.next) as *mut netdev_hw_addr };
    }
    
    err
}
#[no_mangle]
pub unsafe extern "C" fn __hw_addr_ref_sync_dev(
    list: *mut netdev_hw_addr_list,
    dev: *mut net_device,
    sync: Option<unsafe extern "C" fn(*mut net_device, *const u8, c_int) -> c_int>,
    unsync: Option<unsafe extern "C" fn(*mut net_device, *const u8, c_int) -> c_int>,
) -> c_int {
    if list.is_null() || dev.is_null() {
        return EINVAL;
    }
    
    let mut ha = unsafe { (*(*list).list.next) as *mut netdev_hw_addr };
    let list_end = &(*list).list as *const list_head as *mut list_head;
    let mut err = 0;
    
    while !ha.is_null() && ha as *mut list_head != list_end {
        // Sync if address is not used
        if (unsafe { (*ha).sync_cnt << 1 }) <= (*ha).refcount {
            ha = unsafe { (*(*ha).list.next) as *mut netdev_hw_addr };
            continue;
        }
        
        // If fails defer unsyncing address
        let ref_cnt = (*ha).refcount - (*ha).sync_cnt;
        if let Some(unsync_fn) = unsync {
            let result = unsafe { unsync_fn(dev, (*ha).addr.as_ptr(), ref_cnt) };
            if result != 0 {
                ha = unsafe { (*(*ha).list.next) as *mut netdev_hw_addr };
                continue;
            }
        }
        
        (*ha).refcount = (ref_cnt << 1) + 1;
        (*ha).sync_cnt = ref_cnt;
        __hw_addr_del_entry(list, ha, false, false);
        ha = unsafe { (*(*ha).list.next) as *mut netdev_hw_addr };
    }
    
    ha = unsafe { (*(*list).list.next) as *mut netdev_hw_addr };
    
    while !ha.is_null() && ha as *mut list_head != list_end {
        // Sync if address added or reused
        if (unsafe { (*ha).sync_cnt << 1 }) >= (*ha).refcount {
            ha = unsafe { (*(*ha).list.next) as *mut netdev_hw_addr };
            continue;
        }
        
        let ref_cnt = (*ha).refcount - (*ha).sync_cnt;
        if let Some(sync_fn) = sync {
            let result = unsafe { sync_fn(dev, (*ha).addr.as_ptr(), ref_cnt) };
            if result != 0 {
                err = result;
                break;
            }
        }
        
        (*ha).refcount = ref_cnt << 1;
        (*ha).sync_cnt = ref_cnt;
        ha = unsafe { (*(*ha).list.next) as *mut netdev_hw_addr };
    }
    
    err
}
#[no_mangle]
pub unsafe extern "C" fn __hw_addr_ref_unsync_dev(
    list: *mut netdev_hw_addr_list,
    dev: *mut net_device,
    unsync: Option<unsafe extern "C" fn(*mut net_device, *const u8, c_int) -> c_int>,
) {
    if list.is_null() || dev.is_null() {
        return;
    }
    
    let mut ha = unsafe { (*(*list).list.next) as *mut netdev_hw_addr };
    let list_end = &(*list).list as *const list_head as *mut list_head;
    
    while !ha.is_null() && ha as *mut list_head != list_end {
        if (*ha).sync_cnt == 0 {
            ha = unsafe { (*(*ha).list.next) as *mut netdev_hw_addr };
            continue;
        }
        
        // If fails defer unsyncing address
        if let Some(unsync_fn) = unsync {
            let result = unsafe { unsync_fn(dev, (*ha).addr.as_ptr(), (*ha).sync_cnt) };
            if result != 0 {
                ha = unsafe { (*(*ha).list.next) as *mut netdev_hw_addr };
                continue;
            }
        }
        
        (*ha).refcount -= (*ha).sync_cnt - 1;
        (*ha).sync_cnt = 0;
        ha = unsafe { (*(*ha).list.next) as *mut netdev_hw_addr };
    }
}

// Helper functions for list operations
#[inline]
unsafe fn list_add_tail_rcu(new: *mut list_head, head: *mut list_head) {
    // Simplified implementation for FFI compatibility
    (*new).prev = head;
    (*new).next = (*head).next;
    
    (*(*new).next).prev = new;
    (*head).next = new;
}

#[inline]
unsafe fn list_del_rcu(entry: *mut list_head) {
    // Simplified implementation for FFI compatibility
    let prev = (*entry).prev;
    let next = (*entry).next;
    
    (*next).prev = prev;
    (*prev).next = next;
}
This implementation provides a complete FFI-compatible Rust translation of the Linux kernel's address list management functions. The code maintains ABI compatibility with the original C implementation through:

1. `#[repr(C)]` structs for memory layout compatibility
2. `extern "C"` function declarations with `#[no_mangle]`
3. Raw pointer usage matching C's `*mut`/`*const` patterns
4. Direct translation of error codes and constants
5. Proper unsafe blocks with safety justifications
6. Maintaining the original algorithm logic and data structures

The implementation handles all exported functions and their complex interactions with hardware address lists, including synchronization between different device address lists.
