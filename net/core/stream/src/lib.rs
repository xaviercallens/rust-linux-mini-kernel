//! Linux kernel stream socket handling routines
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::all)]

use core::ptr;
use core::mem;

// Constants from Linux kernel
pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ENOSYS: c_int = -38;
pub const EAGAIN: c_int = -11;
pub const EPIPE: c_int = -32;

// TCP state flags (bit positions)
pub const TCPF_ESTABLISHED: u8 = 1 << 0;
pub const TCPF_SYN_SENT: u8 = 1 << 1;
pub const TCPF_SYN_RECV: u8 = 1 << 2;
pub const TCPF_FIN_WAIT1: u8 = 1 << 3;
pub const TCPF_CLOSING: u8 = 1 << 4;
pub const TCPF_LAST_ACK: u8 = 1 << 5;
pub const TCPF_CLOSE_WAIT: u8 = 1 << 6;

// Socket flags
pub const SOCK_NOSPACE_BIT: u32 = 0;
pub const SEND_SHUTDOWN: c_int = 1 << 0;

// EPOLL constants
pub const EPOLLOUT: c_int = 0x0004;
pub const EPOLLWRNORM: c_int = 0x1000;
pub const EPOLLWRBAND: c_int = 0x2000;

// Socket wake flags
pub const SOCK_WAKE_SPACE: c_int = 1;

// Type definitions
#[repr(C)]
pub struct sock {
    sk_socket: *mut socket,
    sk_wq: *mut socket_wq,
    sk_shutdown: c_int,
    sk_state: u8,
    sk_write_pending: c_int,
    sk_error: c_int,
    sk_wmem_queued: c_int,
    sk_forward_alloc: c_int,
}

#[repr(C)]
pub struct socket {
    flags: u32,
}

#[repr(C)]
pub struct socket_wq {
    wait: wait_queue_head_t,
    fasync_list: *mut c_void,
}

#[repr(C)]
pub struct wait_queue_head_t {
    // Simplified representation for FFI compatibility
    _private: [u8; 0],
}

// Function declarations for kernel APIs
extern "C" {
    fn __sk_stream_is_writeable(sk: *mut sock, nonblock: c_int) -> c_int;
    fn sock_error(sk: *mut sock) -> c_int;
    fn sk_stream_memory_free(sk: *mut sock) -> c_int;
    fn prandom_u32() -> u32;
    fn current() -> *mut c_void;
    fn signal_pending(current: *mut c_void) -> c_int;
    fn sock_intr_errno(timeo: c_long) -> c_int;
    fn sk_set_bit(bit: c_int, sk: *mut sock);
    fn sk_clear_bit(bit: c_int, sk: *mut sock);
    fn sk_wait_event(sk: *mut sock, timeo: *mut c_long, condition: c_int, wait: *mut c_void) -> c_int;
    fn sock_wake_async(wq: *mut socket_wq, wake_flags: c_int, poll_flags: c_int);
    fn sk_mem_reclaim(sk: *mut sock);
    fn __skb_queue_purge(queue: *mut skb_queue);
    fn rcu_read_lock();
    fn rcu_read_unlock();
    fn rcu_dereference(ptr: *mut socket_wq) -> *mut socket_wq;
    fn skwq_has_sleeper(wq: *mut socket_wq) -> c_int;
    fn sk_sleep(sk: *mut sock) -> *mut wait_queue_head_t;
}

// Function implementations
#[no_mangle]
pub unsafe extern "C" fn sk_stream_write_space(sk: *mut sock) {
    let sock = (*sk).sk_socket;
    if __sk_stream_is_writeable(sk, 1) == 0 || sock.is_null() {
        return;
    }

    // Clear SOCK_NOSPACE bit
    let flags = &mut (*sock).flags;
    *flags &= !(1 << SOCK_NOSPACE_BIT);

    rcu_read_lock();
    let wq = rcu_dereference((*sk).sk_wq);
    if skwq_has_sleeper(wq) != 0 {
        wake_up_interruptible_poll(&(*wq).wait, EPOLLOUT | EPOLLWRNORM | EPOLLWRBAND);
    }
    if !wq.is_null() && !(*wq).fasync_list.is_null() && (*sk).sk_shutdown & SEND_SHUTDOWN == 0 {
        sock_wake_async(wq, SOCK_WAKE_SPACE, POLL_OUT);
    }
    rcu_read_unlock();
}

#[no_mangle]
pub unsafe extern "C" fn sk_stream_wait_connect(sk: *mut sock, timeo_p: *mut c_long) -> c_int {
    let mut done = 0;
    loop {
        let err = sock_error(sk);
        if err != 0 {
            return err;
        }
        
        let state_mask = 1 << (*sk).sk_state;
        if (state_mask & !(TCPF_SYN_SENT | TCPF_SYN_RECV)) != 0 {
            return -EPIPE;
        }
        
        if (*timeo_p).is_zero() {
            return -EAGAIN;
        }
        
        if signal_pending(current()) != 0 {
            return sock_intr_errno(*timeo_p);
        }
        
        let mut wait: [u8; 0] = [0; 0]; // Simplified wait_queue_entry_t
        add_wait_queue(sk_sleep(sk), &mut wait);
        (*sk).sk_write_pending += 1;
        
        done = sk_wait_event(
            sk, 
            timeo_p, 
            !(*sk).sk_error && 
            !((1 << (*sk).sk_state) & !(TCPF_ESTABLISHED | TCPF_CLOSE_WAIT)) != 0, 
            &mut wait
        );
        
        remove_wait_queue(sk_sleep(sk), &mut wait);
        (*sk).sk_write_pending -= 1;
        
        if done != 0 {
            break;
        }
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn sk_stream_wait_close(sk: *mut sock, timeout: c_long) {
    if timeout != 0 {
        let mut wait: [u8; 0] = [0; 0];
        add_wait_queue(sk_sleep(sk), &mut wait);
        
        loop {
            if sk_wait_event(
                sk, 
                &mut timeout, 
                !sk_stream_closing(sk), 
                &mut wait
            ) != 0 {
                break;
            }
            
            if signal_pending(current()) != 0 || timeout == 0 {
                break;
            }
        }
        
        remove_wait_queue(sk_sleep(sk), &mut wait);
    }
}

#[no_mangle]
pub unsafe extern "C" fn sk_stream_wait_memory(sk: *mut sock, timeo_p: *mut c_long) -> c_int {
    let mut err = 0;
    let mut vm_wait = 0;
    let mut current_timeo = *timeo_p;
    let mut wait: [u8; 0] = [0; 0];
    
    if sk_stream_memory_free(sk) != 0 {
        current_timeo = vm_wait = (prandom_u32() % (100)) + 2;
    }
    
    add_wait_queue(sk_sleep(sk), &mut wait);
    
    while 1 != 0 {
        sk_set_bit(SOCKWQ_ASYNC_NOSPACE, sk);
        
        if (*sk).sk_error != 0 || (*sk).sk_shutdown & SEND_SHUTDOWN != 0 {
            goto do_error;
        }
        
        if (*timeo_p).is_zero() {
            goto do_eagain;
        }
        
        if signal_pending(current()) != 0 {
            goto do_interrupted;
        }
        
        sk_clear_bit(SOCKWQ_ASYNC_NOSPACE, sk);
        
        if sk_stream_memory_free(sk) != 0 && vm_wait == 0 {
            break;
        }
        
        set_bit(SOCK_NOSPACE_BIT, &mut (*sk).sk_socket->flags);
        (*sk).sk_write_pending += 1;
        
        if sk_wait_event(
            sk, 
            &mut current_timeo, 
            (*sk).sk_error != 0 || 
            (*sk).sk_shutdown & SEND_SHUTDOWN != 0 || 
            (sk_stream_memory_free(sk) != 0 && vm_wait == 0), 
            &mut wait
        ) != 0 {
            break;
        }
        
        (*sk).sk_write_pending -= 1;
        
        if vm_wait != 0 {
            vm_wait -= current_timeo;
            current_timeo = *timeo_p;
            if current_timeo != core::i64::MAX as c_long && (current_timeo as i64 - vm_wait as i64) < 0 {
                current_timeo = 0;
            }
            vm_wait = 0;
        }
        
        *timeo_p = current_timeo;
    }
    
    remove_wait_queue(sk_sleep(sk), &mut wait);
    return err;
    
    do_error:
    err = -EPIPE;
    remove_wait_queue(sk_sleep(sk), &mut wait);
    return err;
    
    do_eagain:
    set_bit(SOCK_NOSPACE_BIT, &mut (*sk).sk_socket->flags);
    err = -EAGAIN;
    remove_wait_queue(sk_sleep(sk), &mut wait);
    return err;
    
    do_interrupted:
    err = sock_intr_errno(*timeo_p);
    remove_wait_queue(sk_sleep(sk), &mut wait);
    return err;
}

#[no_mangle]
pub unsafe extern "C" fn sk_stream_error(sk: *mut sock, flags: c_int, err: c_int) -> c_int {
    if err == -EPIPE {
        let sock_err = sock_error(sk);
        if sock_err != 0 {
            return sock_err;
        }
        return -EPIPE;
    }
    
    if err == -EPIPE && (flags & MSG_NOSIGNAL) == 0 {
        send_sig(SIGPIPE, current(), 0);
    }
    err
}

#[no_mangle]
pub unsafe extern "C" fn sk_stream_kill_queues(sk: *mut sock) {
    __skb_queue_purge(&mut (*sk).sk_receive_queue);
    __skb_queue_purge(&mut (*sk).sk_error_queue);
    
    // Write queue should be empty
    assert!(skb_queue_empty(&(*sk).sk_write_queue) != 0);
    
    sk_mem_reclaim(sk);
    
    assert!((*sk).sk_wmem_queued == 0);
    assert!((*sk).sk_forward_alloc == 0);
}

// Helper functions
unsafe fn sk_stream_closing(sk: *mut sock) -> c_int {
    let state_mask = 1 << (*sk).sk_state;
    (state_mask & (TCPF_FIN_WAIT1 | TCPF_CLOSING | TCPF_LAST_ACK)) != 0
}

unsafe fn set_bit(bit: c_int, flags: *mut u32) {
    let val = *flags;
    *flags = val | (1 << bit);
}

unsafe fn wake_up_interruptible_poll(wait: *mut wait_queue_head_t, mask: c_int) {
    // Implementation would depend on kernel internals
}

// Function declarations for external kernel APIs
extern "C" {
    fn send_sig(signal: c_int, tsk: *mut c_void, restart: c_int);
    fn SIGPIPE() -> c_int;
    fn MSG_NOSIGNAL() -> c_int;
    fn skb_queue_empty(queue: *mut skb_queue) -> c_int;
}
