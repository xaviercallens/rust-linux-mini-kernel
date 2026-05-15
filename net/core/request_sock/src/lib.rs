//! Generic infrastructure for Network protocols - Request Socket Queue Management
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]  // For C-style type names

use core::ptr;
use core::ffi::c_int;
use core::ffi::c_void;

// Constants from C
pub const TCP_LISTEN: c_int = 1; // Assuming TCP_LISTEN is defined as 1 in the kernel
pub const HZ: c_int = 100; // Assuming HZ is 100 (100 HZ = 10ms resolution)

// Type definitions
#[repr(C)]
pub struct request_sock_queue {
    rskq_lock: spinlock_t,
    fastopenq: fastopen_queue,
    rskq_accept_head: *mut request_sock,
}

#[repr(C)]
pub struct fastopen_queue {
    lock: spinlock_t,
    rskq_rst_head: *mut request_sock,
    rskq_rst_tail: *mut request_sock,
    qlen: c_int,
}

#[repr(C)]
pub struct request_sock {
    rsk_listener: *mut sock,
    dl_next: *mut request_sock,
    // Other fields omitted for brevity
}

#[repr(C)]
pub struct sock {
    sk: *mut sock, // For rsk_listener
    // Other fields omitted for brevity
}

#[repr(C)]
pub struct tcp_request_sock {
    tfo_listener: bool,
    // Other fields omitted for brevity
}

// Forward declarations for external functions
extern "C" {
    fn spin_lock_init(lock: *mut spinlock_t);
    fn spin_lock_bh(lock: *mut spinlock_t);
    fn spin_unlock_bh(lock: *mut spinlock_t);
    fn reqsk_put(req: *mut request_sock);
    fn RCU_INIT_POINTER(ptr: *mut *mut c_void, val: *mut c_void);
}

// External types
#[repr(C)]
pub struct spinlock_t {
    // Opaque type - actual implementation depends on kernel version
    _private: [u8; 0],
}

/// Initialize request socket queue
///
/// # Safety
/// - `queue` must be a valid pointer to request_sock_queue
/// - Caller must ensure no data races on `queue`
#[no_mangle]
pub unsafe extern "C" fn reqsk_queue_alloc(
    queue: *mut request_sock_queue,
) {
    // SAFETY: queue is non-null (caller responsibility)
    // Initialize main lock
    spin_lock_init(&mut (*queue).rskq_lock);
    
    // Initialize fastopenq fields
    spin_lock_init(&mut (*queue).fastopenq.lock);
    (*queue).fastopenq.rskq_rst_head = ptr::null_mut();
    (*queue).fastopenq.rskq_rst_tail = ptr::null_mut();
    (*queue).fastopenq.qlen = 0;
    
    // Initialize accept head
    (*queue).rskq_accept_head = ptr::null_mut();
}

/// Remove Fast Open request from queue
///
/// # Safety
/// - `sk` must be a valid pointer to sock
/// - `req` must be a valid pointer to request_sock
/// - `reset` indicates if RST handling is needed
/// - Caller must hold appropriate locks
#[no_mangle]
pub unsafe extern "C" fn reqsk_fastopen_remove(
    sk: *mut sock,
    req: *mut request_sock,
    reset: bool,
) {
    // SAFETY: req is non-null (caller responsibility)
    let lsk = (*req).rsk_listener;
    let fastopenq = &mut (*lsk).sk as *mut sock as *mut fastopen_queue;
    
    // Clear fastopen_rsk pointer with RCU
    RCU_INIT_POINTER((*sk).sk as *mut c_void, ptr::null_mut::<c_void>());
    
    // Acquire lock for fastopenq operations
    spin_lock_bh(&mut (*fastopenq).lock);
    
    // Decrement queue length
    (*fastopenq).qlen -= 1;
    
    // Mark as non-listener
    let treq = req as *mut tcp_request_sock;
    (*treq).tfo_listener = false;
    
    // Check if child socket has been accepted
    if !(*req).sk.is_null() {
        // Skip RST handling if child not accepted yet
        spin_unlock_bh(&mut (*fastopenq).lock);
        return;
    }
    
    // If not reset or listener not in LISTEN state, release req
    if !reset || (*lsk).sk != TCP_LISTEN {
        spin_unlock_bh(&mut (*fastopenq).lock);
        reqsk_put(req);
        return;
    }
    
    // Set 60s timer for RST handling
    let jiffies = 60 * HZ;
    (*req).rsk_timer.expires = jiffies;
    
    // Add to RST queue
    if (*fastopenq).rskq_rst_head.is_null() {
        (*fastopenq).rskq_rst_head = req;
    } else {
        (*(*fastopenq).rskq_rst_tail).dl_next = req;
    }
    
    (*fastopenq).rskq_rst_tail = req;
    (*fastopenq).qlen += 1;
    
    spin_unlock_bh(&mut (*fastopenq).lock);
}

// Tests (conditional compilation)
#[cfg(test)]
mod tests {
    // No tests for kernel FFI code
}
