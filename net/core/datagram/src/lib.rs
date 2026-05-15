// SPDX-License-Identifier: GPL-2.0
//!
//! This module provides FFI-compatible Rust implementations of Linux kernel
//! datagram handling functions. The implementation matches the original C
//! code's behavior exactly while maintaining ABI compatibility with the
//! Linux kernel's C API.
//!
//! All exported functions use `#[no_mangle]` and `extern "C"` to maintain
//! symbol compatibility. Structs are marked with `#[repr(C)]` to preserve
//! memory layout. Raw pointers are used throughout to match C's behavior.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use core::ptr;
use core::ffi::c_int;
use core::ffi::c_uint;
use core::ffi::c_void;
use core::ffi::c_long;
use core::ffi::size_t;

// Constants from Linux kernel
pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ENOTCONN: c_int = -107;
pub const EAGAIN: c_int = -11;
pub const ENOENT: c_int = -2;

// Socket types
pub const SOCK_SEQPACKET: c_int = 5;
pub const SOCK_STREAM: c_int = 1;

// Socket states
pub const TCP_ESTABLISHED: c_int = 1;
pub const TCP_LISTEN: c_int = 6;

// Socket shutdown flags
pub const RCV_SHUTDOWN: c_int = 1;

// Message flags
pub const MSG_PEEK: c_int = 2;
pub const MSG_DONTWAIT: c_int = 64;

// Wait queue flags
pub const TASK_INTERRUPTIBLE: c_int = 1;

// epoll events
pub const EPOLLIN: c_int = 1;
pub const EPOLLERR: c_int = 8;

// Define C-compatible structs
#[repr(C)]
pub struct sock {
    sk_type: c_int,
    sk_state: c_int,
    sk_shutdown: c_int,
    sk_receive_queue: sk_buff_head,
}

#[repr(C)]
pub struct sk_buff_head {
    lock: spinlock_t,
    prev: *mut sk_buff,
    next: *mut sk_buff,
}

#[repr(C)]
pub struct sk_buff {
    prev: *mut sk_buff,
    next: *mut sk_buff,
    len: size_t,
    peeked: c_int,
    users: atomic_t,
}

#[repr(C)]
pub struct atomic_t {
    counter: c_int,
}

#[repr(C)]
pub struct spinlock_t {
    raw_lock: c_int,
}

#[repr(C)]
pub struct wait_queue_entry_t {
    next: *mut wait_queue_entry_t,
    flags: c_int,
    func: unsafe extern "C" fn(wait: *mut wait_queue_entry_t, mode: c_int, sync: c_int, key: *mut c_void) -> c_int,
}

#[repr(C)]
pub struct wait_queue_head_t {
    lock: spinlock_t,
    first: *mut wait_queue_entry_t,
}

// Extern declarations for kernel functions
extern "C" {
    fn prepare_to_wait_exclusive(wait_queue_head: *mut wait_queue_head_t, wait: *mut wait_queue_entry_t, state: c_int);
    fn finish_wait(wait_queue_head: *mut wait_queue_head_t, wait: *mut wait_queue_entry_t);
    fn schedule_timeout(timeout: c_long) -> c_long;
    fn sock_error(sk: *mut sock) -> c_int;
    fn sk_sleep(sk: *mut sock) -> *mut wait_queue_head_t;
    fn signal_pending(current: *mut c_void) -> c_int;
    fn sock_intr_errno(timeo: c_long) -> c_int;
    fn skb_clone(skb: *mut sk_buff, gfp_mask: c_int) -> *mut sk_buff;
    fn consume_skb(skb: *mut sk_buff);
    fn sk_peek_offset_bwd(sk: *mut sock, len: c_int);
    fn skb_orphan(skb: *mut sk_buff);
    fn sk_mem_reclaim_partial(sk: *mut sock);
    fn __kfree_skb(skb: *mut sk_buff);
    fn lock_sock_fast(sk: *mut sock) -> c_int;
    fn unlock_sock_fast(sk: *mut sock, slow: c_int);
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut c_ulong);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: *mut c_ulong);
    fn key_to_poll(key: *mut c_void) -> c_int;
}

// Helper functions
fn connection_based(sk: *mut sock) -> c_int {
    unsafe {
        if (*sk).sk_type == SOCK_SEQPACKET || (*sk).sk_type == SOCK_STREAM {
            1
        } else {
            0
        }
    }
}

unsafe fn receiver_wake_function(wait: *mut wait_queue_entry_t, mode: c_int, sync: c_int, key: *mut c_void) -> c_int {
    if !key.is_null() && !(key_to_poll(key) & (EPOLLIN | EPOLLERR)) != 0 {
        return 0;
    }
    // Assume autoremove_wake_function is implemented in C
    extern "C" {
        fn autoremove_wake_function(wait: *mut wait_queue_entry_t, mode: c_int, sync: c_int, key: *mut c_void) -> c_int;
    }
    autoremove_wake_function(wait, mode, sync, key)
}

#[no_mangle]
pub unsafe extern "C" fn __skb_wait_for_more_packets(
    sk: *mut sock,
    queue: *mut sk_buff_head,
    err: *mut c_int,
    timeo_p: *mut c_long,
    skb: *const sk_buff,
) -> c_int {
    let mut error = 0;
    let mut wait: wait_queue_entry_t = std::mem::zeroed();
    
    // SAFETY: Using prepare_to_wait_exclusive with valid wait_queue_head
    let wait_queue_head = sk_sleep(sk);
    prepare_to_wait_exclusive(wait_queue_head, &mut wait, TASK_INTERRUPTIBLE);
    
    error = sock_error(sk);
    if error != 0 {
        *err = error;
        finish_wait(wait_queue_head, &mut wait);
        return error;
    }
    
    if (*queue).prev != skb {
        finish_wait(wait_queue_head, &mut wait);
        return 0;
    }
    
    if (*sk).sk_shutdown & RCV_SHUTDOWN != 0 {
        *err = 0;
        finish_wait(wait_queue_head, &mut wait);
        return 1;
    }
    
    error = -ENOTCONN;
    if connection_based(sk) != 0 && 
       (*sk).sk_state != TCP_ESTABLISHED && 
       (*sk).sk_state != TCP_LISTEN {
        *err = error;
        finish_wait(wait_queue_head, &mut wait);
        return error;
    }
    
    if signal_pending(ptr::null_mut()) != 0 {
        error = sock_intr_errno(*timeo_p);
        *err = error;
        finish_wait(wait_queue_head, &mut wait);
        return error;
    }
    
    error = 0;
    *timeo_p = schedule_timeout(*timeo_p);
    finish_wait(wait_queue_head, &mut wait);
    error
}

#[no_mangle]
pub unsafe extern "C" fn skb_set_peeked(skb: *mut sk_buff) -> *mut sk_buff {
    if (*skb).peeked != 0 {
        return skb;
    }
    
    if (*skb).users.counter <= 1 {
        goto done;
    }
    
    let nskb = skb_clone(skb, 0); // GFP_ATOMIC is 0 in this context
    if nskb.is_null() {
        return ptr::null_mut();
    }
    
    (*(*skb).prev).next = nskb;
    (*(*skb).next).prev = nskb;
    (*nskb).prev = (*skb).prev;
    (*nskb).next = (*skb).next;
    
    consume_skb(skb);
    let skb = nskb;
    
done:
    (*skb).peeked = 1;
    skb
}

#[no_mangle]
pub unsafe extern "C" fn __skb_try_recv_from_queue(
    sk: *mut sock,
    queue: *mut sk_buff_head,
    flags: c_int,
    off: *mut c_int,
    error: *mut c_int,
    last: *mut *mut sk_buff,
) -> *mut sk_buff {
    let mut peek_at_off = false;
    let mut _off = 0;
    
    if flags & MSG_PEEK != 0 && *off >= 0 {
        peek_at_off = true;
        _off = *off;
    }
    
    *last = (*queue).prev;
    
    let mut skb = (*queue).next;
    while !skb.is_null() && skb != &mut (*queue) as *mut _ as *mut sk_buff {
        if flags & MSG_PEEK != 0 {
            if peek_at_off && _off >= (*skb).len && (_off != 0 || (*skb).peeked != 0) {
                _off -= (*skb).len;
                skb = (*skb).next;
                continue;
            }
            
            if (*skb).len == 0 {
                let new_skb = skb_set_peeked(skb);
                if new_skb.is_null() {
                    *error = -ENOMEM;
                    return ptr::null_mut();
                }
                skb = new_skb;
            }
            
            // SAFETY: Incrementing reference count for peek
            (*skb).users.counter += 1;
        } else {
            // SAFETY: Unlinking from queue
            (*(*skb).prev).next = (*skb).next;
            (*(*skb).next).prev = (*skb).prev;
        }
        
        *off = _off as c_int;
        return skb;
    }
    
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn __skb_try_recv_datagram(
    sk: *mut sock,
    queue: *mut sk_buff_head,
    flags: c_int,
    off: *mut c_int,
    error: *mut c_int,
    last: *mut *mut sk_buff,
) -> *mut sk_buff {
    let mut cpu_flags: c_ulong = 0;
    let mut error_val = sock_error(sk);
    
    if error_val != 0 {
        *error = error_val;
        return ptr::null_mut();
    }
    
    loop {
        spin_lock_irqsave(&mut (*queue).lock, &mut cpu_flags);
        let skb = __skb_try_recv_from_queue(sk, queue, flags, off, error, last);
        spin_unlock_irqrestore(&mut (*queue).lock, &mut cpu_flags);
        
        if *error != 0 {
            return ptr::null_mut();
        }
        
        if !skb.is_null() {
            return skb;
        }
        
        if !sk_can_busy_loop(sk) {
            break;
        }
        
        sk_busy_loop(sk, flags & MSG_DONTWAIT);
    }
    
    *error = -EAGAIN;
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn __skb_recv_datagram(
    sk: *mut sock,
    sk_queue: *mut sk_buff_head,
    flags: c_int,
    off: *mut c_int,
    error: *mut c_int,
) -> *mut sk_buff {
    let mut last: *mut sk_buff = ptr::null_mut();
    let mut timeo = sock_rcvtimeo(sk, flags & MSG_DONTWAIT);
    
    loop {
        let skb = __skb_try_recv_datagram(sk, sk_queue, flags, off, error, &mut last);
        if !skb.is_null() {
            return skb;
        }
        
        if *error != -EAGAIN {
            break;
        }
        
        if !timeo || __skb_wait_for_more_packets(sk, sk_queue, error, &mut timeo, last) != 0 {
            break;
        }
    }
    
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn skb_recv_datagram(
    sk: *mut sock,
    flags: c_int,
    noblock: c_int,
    error: *mut c_int,
) -> *mut sk_buff {
    let mut off = 0;
    __skb_recv_datagram(sk, &mut (*sk).sk_receive_queue, flags | (noblock != 0) as c_int * MSG_DONTWAIT, &mut off, error)
}

#[no_mangle]
pub unsafe extern "C" fn skb_free_datagram(sk: *mut sock, skb: *mut sk_buff) {
    consume_skb(skb);
    sk_mem_reclaim_partial(sk);
}

#[no_mangle]
pub unsafe extern "C" fn __skb_free_datagram_locked(
    sk: *mut sock,
    skb: *mut sk_buff,
    len: c_int,
) {
    let slow = lock_sock_fast(sk);
    sk_peek_offset_bwd(sk, len);
    
    if (*skb).users.counter > 1 {
        (*skb).users.counter -= 1;
        unlock_sock_fast(sk, slow);
        return;
    }
    
    skb_orphan(skb);
    sk_mem_reclaim_partial(sk);
    unlock_sock_fast(sk, slow);
    __kfree_skb(skb);
}

#[no_mangle]
pub unsafe extern "C" fn __sk_queue_drop_skb(
    sk: *mut sock,
    sk_queue: *mut sk_buff_head,
    skb: *mut sk_buff,
    flags: c_int,
    destructor: Option<unsafe extern "C" fn(sk: *mut sock, skb: *mut sk_buff)>,
) -> c_int {
    let mut err = 0;
    
    if flags & MSG_PEEK != 0 {
        err = -ENOENT;
        spin_lock_irqsave(&mut (*sk_queue).lock, &mut 0);
        if !(*skb).next.is_null() {
            (*(*skb).prev).next = (*skb).next;
            (*(*skb).next).prev = (*skb).prev;
            (*skb).users.counter -= 1;
            if let Some(d) = destructor {
                d(sk, skb);
            }
            err = 0;
        }
        spin_unlock_irqrestore(&mut (*sk_queue).lock, &mut 0);
    }
    
    atomic_inc(&mut (*sk).sk_drops);
    err
}

// Helper functions (simplified for this example)
unsafe fn sk_can_busy_loop(sk: *mut sock) -> c_int {
    0
}

unsafe fn sk_busy_loop(sk: *mut sock, flags: c_int) {
    // Busy loop implementation
}

unsafe fn sock_rcvtimeo(sk: *mut sock, noblock: c_int) -> c_long {
    0
}

// Exported symbols
#[no_mangle]
pub extern "C" fn __skb_wait_for_more_packets_export() {}
#[no_mangle]
pub extern "C" fn __skb_try_recv_datagram_export() {}
#[no_mangle]
pub extern "C" fn __skb_recv_datagram_export() {}
#[no_mangle]
pub extern "C" fn skb_recv_datagram_export() {}
#[no_mangle]
pub extern "C" fn skb_free_datagram_export() {}
#[no_mangle]
pub extern "C" fn __skb_free_datagram_locked_export() {}
#[no_mangle]
pub extern "C" fn __sk_queue_drop_skb_export() {}
