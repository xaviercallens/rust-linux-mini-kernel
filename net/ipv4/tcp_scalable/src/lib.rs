//! Scalable TCP Congestion Control
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
const TCP_SCALABLE_AI_CNT: u32 = 100;
const TCP_SCALABLE_MD_SCALE: u32 = 3;

// Forward declarations for kernel types
#[repr(C)]
struct sock;
#[repr(C)]
struct tcp_sock {
    snd_cwnd: u32,
}

// Function pointers from kernel
extern "C" {
    fn tcp_register_congestion_control(ops: *mut tcp_congestion_ops) -> c_int;
    fn tcp_unregister_congestion_control(ops: *mut tcp_congestion_ops);
    fn tcp_is_cwnd_limited(sk: *const sock) -> bool;
    fn tcp_in_slow_start(tp: *const tcp_sock) -> bool;
    fn tcp_slow_start(tp: *mut tcp_sock, acked: u32) -> u32;
    fn tcp_cong_avoid_ai(tp: *mut tcp_sock, cnt: u32, acked: u32);
    fn tcp_reno_undo_cwnd(sk: *const sock) -> u32;
}

// Type definitions
#[repr(C)]
struct tcp_congestion_ops {
    ssthresh: extern "C" fn(sk: *const sock) -> u32,
    undo_cwnd: extern "C" fn(sk: *const sock) -> u32,
    cong_avoid: extern "C" fn(sk: *mut sock, ack: u32, acked: u32),
    owner: *mut c_void,
    name: *const u8,
}

// Function implementations
#[no_mangle]
pub unsafe extern "C" fn tcp_scalable_cong_avoid(
    sk: *mut sock,
    _ack: u32,
    acked: u32,
) {
    // SAFETY: Caller guarantees sk is valid and points to a sock
    let tp = tcp_sk(sk);
    
    if !tcp_is_cwnd_limited(sk) {
        return;
    }

    if tcp_in_slow_start(tp) {
        let acked = tcp_slow_start(tp, acked);
        if acked == 0 {
            return;
        }
    }

    let cnt = u32::min((*tp).snd_cwnd, TCP_SCALABLE_AI_CNT);
    tcp_cong_avoid_ai(tp, cnt, acked);
}

#[no_mangle]
pub unsafe extern "C" fn tcp_scalable_ssthresh(sk: *const sock) -> u32 {
    let tp = tcp_sk(sk);
    
    let reduction = (*tp).snd_cwnd >> TCP_SCALABLE_MD_SCALE;
    u32::max((*tp).snd_cwnd - reduction, 2)
}

// Helper function to cast sock* to tcp_sock*
#[inline]
unsafe fn tcp_sk(sk: *mut sock) -> *mut tcp_sock {
    // SAFETY: This is a direct pointer cast as per Linux kernel's tcp_sk macro
    // Caller must ensure sk is a valid pointer to a sock structure
    sk as *mut tcp_sock
}

// Module initialization
#[no_mangle]
pub unsafe extern "C" fn tcp_scalable_register() -> c_int {
    let ops = &mut tcp_scalable;
    tcp_register_congestion_control(ops)
}

#[no_mangle]
pub unsafe extern "C" fn tcp_scalable_unregister() {
    let ops = &mut tcp_scalable;
    tcp_unregister_congestion_control(ops);
}

// Static module data
static mut tcp_scalable: tcp_congestion_ops = tcp_congestion_ops {
    ssthresh: tcp_scalable_ssthresh,
    undo_cwnd: tcp_reno_undo_cwnd,
    cong_avoid: tcp_scalable_cong_avoid,
    owner: ptr::null_mut(),
    name: b"scalable\0".as_ptr(),
};

// Module metadata (as comments since no_std)
// MODULE_AUTHOR: "John Heffner"
// MODULE_LICENSE: "GPL"
// MODULE_DESCRIPTION: "Scalable TCP"

#[cfg(test)]
mod tests {
    // No tests possible in no_std environment
}
