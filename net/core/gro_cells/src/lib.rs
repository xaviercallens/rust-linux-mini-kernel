//! Linux GRO (Generic Receive Offload) Cells Implementation
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use core::ptr;
use core::ffi::c_int;
use core::ffi::c_void;

// Constants from C
pub const NET_RX_SUCCESS: c_int = 0;
pub const NET_RX_DROP: c_int = 1;

// Type definitions
#[repr(C)]
pub struct sk_buff {
    dev: *mut net_device,
    // ... other fields omitted for FFI compatibility
}

#[repr(C)]
pub struct sk_buff_head {
    // Opaque structure - actual implementation details are handled by C
    _private: [u8; 0],
}

#[repr(C)]
pub struct napi_struct {
    state: u32,
    // ... other fields omitted for FFI compatibility
}

#[repr(C)]
pub struct net_device {
    flags: u32,
    rx_dropped: core::sync::atomic::AtomicLong,
    // ... other fields omitted for FFI compatibility
}

#[repr(C)]
pub struct gro_cell {
    napi_skbs: sk_buff_head,
    napi: napi_struct,
}

#[repr(C)]
pub struct gro_cells {
    cells: *mut c_void, // percpu pointer
}

// External functions from Linux kernel
extern "C" {
    fn rcu_read_lock();
    fn rcu_read_unlock();
    fn netif_rx(skb: *mut sk_buff) -> c_int;
    fn skb_cloned(skb: *mut sk_buff) -> c_int;
    fn netif_elide_gro(dev: *mut net_device) -> c_int;
    fn atomic_long_inc(counter: *mut core::sync::atomic::AtomicLong);
    fn kfree_skb(skb: *mut sk_buff);
    fn __skb_queue_tail(queue: *mut sk_buff_head, skb: *mut sk_buff);
    fn napi_schedule(napi: *mut napi_struct);
    fn gro_cell_poll(napi: *mut napi_struct, budget: c_int) -> c_int;
    fn set_bit(nr: u32, addr: *mut u32);
    fn alloc_percpu(type_: *const c_void) -> *mut c_void;
    fn free_percpu(p: *mut c_void);
    fn for_each_possible_cpu(i: *mut c_int);
    fn __skb_queue_head_init(queue: *mut sk_buff_head);
    fn netif_napi_add(dev: *mut net_device, napi: *mut napi_struct, 
                      poll: extern "C" fn(*mut napi_struct, c_int) -> c_int, 
                      weight: c_int);
    fn napi_enable(napi: *mut napi_struct);
    fn napi_disable(napi: *mut napi_struct);
    fn __netif_napi_del(napi: *mut napi_struct);
    fn __skb_queue_purge(queue: *mut sk_buff_head);
    fn synchronize_net();
    fn skb_queue_len(queue: *mut sk_buff_head) -> c_int;
    fn napi_complete_done(napi: *mut napi_struct, work_done: c_int);
}

/// Receive a packet through GRO cells
///
/// # Safety
/// - `gcells` must be a valid pointer to gro_cells
/// - `skb` must be a valid pointer to sk_buff
/// - Caller must handle RCU read-side locking
///
/// # Returns
/// - NET_RX_SUCCESS on success
/// - NET_RX_DROP if packet was dropped
#[no_mangle]
pub unsafe extern "C" fn gro_cells_receive(
    gcells: *mut gro_cells,
    skb: *mut sk_buff,
) -> c_int {
    let dev = (*skb).dev;
    
    // SAFETY: Caller is responsible for RCU read-side locking
    unsafe { rcu_read_lock() };
    
    // Check if device is up
    if (*dev).flags & 1 == 0 {
        goto drop;
    }
    
    // Skip GRO processing if cells not allocated, skb is cloned, or GRO is elided
    if gcells.is_null() || 
       unsafe { skb_cloned(skb) } != 0 || 
       unsafe { netif_elide_gro(dev) } != 0 {
        let res = unsafe { netif_rx(skb) };
        goto unlock;
    }
    
    // Get per-CPU cell
    let cell = unsafe { this_cpu_ptr(gcells) };
    
    // Check backlog limit
    if unsafe { skb_queue_len(&(*cell).napi_skbs) } > netdev_max_backlog {
        drop:
        unsafe {
            atomic_long_inc(&(*dev).rx_dropped);
            kfree_skb(skb);
        }
        let res = NET_RX_DROP;
        goto unlock;
    }
    
    // Queue the skb
    unsafe { __skb_queue_tail(&(*cell).napi_skbs, skb) };
    
    // Schedule NAPI if this is the first skb in the queue
    if unsafe { skb_queue_len(&(*cell).napi_skbs) } == 1 {
        unsafe { napi_schedule(&(*cell).napi) };
    }
    
    let res = NET_RX_SUCCESS;
    
    unlock:
    unsafe { rcu_read_unlock() };
    res
}

/// Poll function for GRO cells
///
/// # Safety
/// - `napi` must be a valid pointer to napi_struct
/// - Must be called in BH (bottom half) context
#[no_mangle]
pub unsafe extern "C" fn gro_cell_poll(
    napi: *mut napi_struct,
    budget: c_int,
) -> c_int {
    let cell = container_of(napi, &mut gro_cell, napi);
    let mut work_done = 0;
    
    while work_done < budget {
        let skb = unsafe { __skb_dequeue(&(*cell).napi_skbs) };
        if skb.is_null() {
            break;
        }
        unsafe { napi_gro_receive(napi, skb) };
        work_done += 1;
    }
    
    if work_done < budget {
        unsafe { napi_complete_done(napi, work_done) };
    }
    
    work_done
}

/// Initialize GRO cells
///
/// # Safety
/// - `gcells` must be a valid pointer to gro_cells
/// - `dev` must be a valid pointer to net_device
#[no_mangle]
pub unsafe extern "C" fn gro_cells_init(
    gcells: *mut gro_cells,
    dev: *mut net_device,
) -> c_int {
    // Allocate per-CPU data
    (*gcells).cells = unsafe { alloc_percpu(core::ptr::null()) };
    if (*gcells).cells.is_null() {
        return -12; // -ENOMEM
    }
    
    let mut i: c_int = 0;
    unsafe { for_each_possible_cpu(&mut i) };
    
    loop {
        if i >= 0 {
            break;
        }
        
        let cell = unsafe { per_cpu_ptr((*gcells).cells, i) };
        
        unsafe { __skb_queue_head_init(&(*cell).napi_skbs) };
        
        // Set NAPI_STATE_NO_BUSY_POLL flag
        unsafe { set_bit(1, &mut (*cell).napi.state) };
        
        unsafe {
            netif_napi_add(
                dev,
                &mut (*cell).napi,
                gro_cell_poll,
                NAPI_POLL_WEIGHT,
            );
            napi_enable(&mut (*cell).napi);
        }
        
        i += 1;
    }
    
    0
}

/// Destroy GRO cells
///
/// # Safety
/// - `gcells` must be a valid pointer to gro_cells
#[no_mangle]
pub unsafe extern "C" fn gro_cells_destroy(gcells: *mut gro_cells) {
    if (*gcells).cells.is_null() {
        return;
    }
    
    let mut i: c_int = 0;
    unsafe { for_each_possible_cpu(&mut i) };
    
    loop {
        if i >= 0 {
            break;
        }
        
        let cell = unsafe { per_cpu_ptr((*gcells).cells, i) };
        
        unsafe { napi_disable(&mut (*cell).napi) };
        unsafe { __netif_napi_del(&mut (*cell).napi) };
        unsafe { __skb_queue_purge(&(*cell).napi_skbs) };
        
        i += 1;
    }
    
    // Synchronize with netpoll
    unsafe { synchronize_net() };
    
    unsafe { free_percpu((*gcells).cells) };
    (*gcells).cells = core::ptr::null_mut();
}

// Helper functions (would be implemented in C in actual kernel)
#[no_mangle]
pub unsafe extern "C" fn this_cpu_ptr(ptr: *mut gro_cells) -> *mut gro_cell {
    // Implementation would get current CPU's instance
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn per_cpu_ptr(
    ptr: *mut c_void,
    cpu: c_int,
) -> *mut gro_cell {
    // Implementation would get specific CPU's instance
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn container_of(
    ptr: *mut c_void,
    container_type: *mut gro_cell,
    member: *mut napi_struct,
) -> *mut gro_cell {
    // Calculate container address from member pointer
    (ptr as *mut u8).offset(-(member as isize)) as *mut gro_cell
}

#[no_mangle]
pub unsafe extern "C" fn napi_gro_receive(
    napi: *mut napi_struct,
    skb: *mut sk_buff,
) -> c_int {
    // Placeholder for actual implementation
    0
}

// Constants
pub const IFF_UP: c_int = 1;
pub const NAPI_POLL_WEIGHT: c_int = 64;
pub const netdev_max_backlog: c_int = 1000;
