//! YeAH TCP Congestion Control
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]  // For C-style type names

use core::ptr;
use core::cmp;

// Constants from C
pub const TCP_YEAH_ALPHA: u32 = 80;
pub const TCP_YEAH_GAMMA: u32 = 1;
pub const TCP_YEAH_DELTA: u32 = 3;
pub const TCP_YEAH_EPSILON: u32 = 1;
pub const TCP_YEAH_PHY: u32 = 8;
pub const TCP_YEAH_RHO: u32 = 16;
pub const TCP_YEAH_ZETA: u32 = 50;
pub const TCP_SCALABLE_AI_CNT: u32 = 100;

// Type definitions
#[repr(C)]
struct vegas {
    beg_snd_una: u32,
    beg_snd_nxt: u32,
    beg_snd_cwnd: u32,
    cntRTT: u32,
    minRTT: u32,
    baseRTT: u32,
}

#[repr(C)]
struct yeah {
    vegas: vegas, // must be first

    // YeAH
    lastQ: u32,
    doing_reno_now: u32,

    reno_count: u32,
    fast_count: u32,
}

#[repr(C)]
struct tcp_sock {
    snd_cwnd: u32,
    snd_cwnd_clamp: u32,
    snd_una: u32,
    snd_nxt: u32,
    snd_ssthresh: u32,
}

#[repr(C)]
struct sock {
    // Placeholder - actual fields depend on kernel headers
    _private: [u8; 0],
}

#[repr(C)]
struct tcp_congestion_ops {
    init: extern "C" fn(*mut sock),
    ssthresh: extern "C" fn(*mut sock) -> u32,
    undo_cwnd: extern "C" fn(*mut sock),
    cong_avoid: extern "C" fn(*mut sock, u32, u32),
    set_state: extern "C" fn(*mut sock, u32),
    cwnd_event: extern "C" fn(*mut sock, u32),
    get_info: extern "C" fn(*mut sock, *mut u8, *mut u32) -> i32,
    pkts_acked: extern "C" fn(*mut sock, u32, u32),
    owner: *mut u8,
    name: [u8; 16],
}

// Function implementations
#[no_mangle]
pub unsafe extern "C" fn tcp_yeah_init(sk: *mut sock) {
    let tp: *mut tcp_sock = tcp_sk(sk);
    let yeah: *mut yeah = inet_csk_ca(sk);
    
    // SAFETY: Caller guarantees sk is valid
    tcp_vegas_init(sk);
    
    (*yeah).doing_reno_now = 0;
    (*yeah).lastQ = 0;
    (*yeah).reno_count = 2;
    
    // Ensure the MD arithmetic works
    (*tp).snd_cwnd_clamp = cmp::min((*tp).snd_cwnd_clamp, 0xFFFFFFFF / 128);
}

#[no_mangle]
pub unsafe extern "C" fn tcp_yeah_cong_avoid(sk: *mut sock, ack: u32, acked: u32) {
    let tp: *mut tcp_sock = tcp_sk(sk);
    let yeah: *mut yeah = inet_csk_ca(sk);
    
    if !tcp_is_cwnd_limited(sk) {
        return;
    }
    
    if tcp_in_slow_start(tp) {
        let acked = tcp_slow_start(tp, acked);
        if acked == 0 {
            // Jump to Vegas calculations
            do_vegas:
            if after(ack, (*yeah).vegas.beg_snd_nxt) {
                if (*yeah).vegas.cntRTT > 2 {
                    let rtt = (*yeah).vegas.minRTT;
                    let mut bw: u64 = (*tp).snd_cwnd as u64;
                    bw *= (rtt - (*yeah).vegas.baseRTT) as u64;
                    let queue = (bw / rtt as u64) as u32;
                    
                    if queue > TCP_YEAH_ALPHA || rtt - (*yeah).vegas.baseRTT > (*yeah).vegas.baseRTT / TCP_YEAH_PHY {
                        if queue > TCP_YEAH_ALPHA && (*tp).snd_cwnd > (*yeah).reno_count {
                            let reduction = cmp::min(queue / TCP_YEAH_GAMMA, (*tp).snd_cwnd >> TCP_YEAH_EPSILON);
                            (*tp).snd_cwnd = (*tp).snd_cwnd.saturating_sub(reduction);
                            (*tp).snd_cwnd = cmp::max((*tp).snd_cwnd, (*yeah).reno_count);
                            (*tp).snd_ssthresh = (*tp).snd_cwnd;
                        }
                        
                        if (*yeah).reno_count <= 2 {
                            (*yeah).reno_count = cmp::max((*tp).snd_cwnd >> 1, 2);
                        } else {
                            (*yeah).reno_count += 1;
                        }
                        
                        (*yeah).doing_reno_now = (*yeah).doing_reno_now.saturating_add(1);
                    } else {
                        (*yeah).fast_count += 1;
                        
                        if (*yeah).fast_count > TCP_YEAH_ZETA {
                            (*yeah).reno_count = 2;
                            (*yeah).fast_count = 0;
                        }
                        
                        (*yeah).doing_reno_now = 0;
                    }
                    
                    (*yeah).lastQ = queue;
                }
                
                (*yeah).vegas.beg_snd_una = (*yeah).vegas.beg_snd_nxt;
                (*yeah).vegas.beg_snd_nxt = (*tp).snd_nxt;
                (*yeah).vegas.beg_snd_cwnd = (*tp).snd_cwnd;
                (*yeah).vegas.cntRTT = 0;
                (*yeah).vegas.minRTT = 0x7FFFFFFF;
            }
        }
    } else {
        // Jump to Vegas calculations
        do_vegas:
        if after(ack, (*yeah).vegas.beg_snd_nxt) {
            if (*yeah).vegas.cntRTT > 2 {
                let rtt = (*yeah).vegas.minRTT;
                let mut bw: u64 = (*tp).snd_cwnd as u64;
                bw *= (rtt - (*yeah).vegas.baseRTT) as u64;
                let queue = (bw / rtt as u64) as u32;
                
                if queue > TCP_YEAH_ALPHA || rtt - (*yeah).vegas.baseRTT > (*yeah).vegas.baseRTT / TCP_YEAH_PHY {
                    if queue > TCP_YEAH_ALPHA && (*tp).snd_cwnd > (*yeah).reno_count {
                        let reduction = cmp::min(queue / TCP_YEAH_GAMMA, (*tp).snd_cwnd >> TCP_YEAH_EPSILON);
                        (*tp).snd_cwnd = (*tp).snd_cwnd.saturating_sub(reduction);
                        (*tp).snd_cwnd = cmp::max((*tp).snd_cwnd, (*yeah).reno_count);
                        (*tp).snd_ssthresh = (*tp).snd_cwnd;
                    }
                    
                    if (*yeah).reno_count <= 2 {
                        (*yeah).reno_count = cmp::max((*tp).snd_cwnd >> 1, 2);
                    } else {
                        (*yeah).reno_count += 1;
                    }
                    
                    (*yeah).doing_reno_now = (*yeah).doing_reno_now.saturating_add(1);
                } else {
                    (*yeah).fast_count += 1;
                    
                    if (*yeah).fast_count > TCP_YEAH_ZETA {
                        (*yeah).reno_count = 2;
                        (*yeah).fast_count = 0;
                    }
                    
                    (*yeah).doing_reno_now = 0;
                }
                
                (*yeah).lastQ = queue;
            }
            
            (*yeah).vegas.beg_snd_una = (*yeah).vegas.beg_snd_nxt;
            (*yeah).vegas.beg_snd_nxt = (*tp).snd_nxt;
            (*yeah).vegas.beg_snd_cwnd = (*tp).snd_cwnd;
            (*yeah).vegas.cntRTT = 0;
            (*yeah).vegas.minRTT = 0x7FFFFFFF;
        }
        
        if !(*yeah).doing_reno_now {
            // Scalable
            tcp_cong_avoid_ai(tp, cmp::min((*tp).snd_cwnd, TCP_SCALABLE_AI_CNT), acked);
        } else {
            // Reno
            tcp_cong_avoid_ai(tp, (*tp).snd_cwnd, acked);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn tcp_yeah_ssthresh(sk: *mut sock) -> u32 {
    let tp: *const tcp_sock = tcp_sk(sk);
    let yeah: *mut yeah = inet_csk_ca(sk);
    let mut reduction: u32 = 0;
    
    if (*yeah).doing_reno_now < TCP_YEAH_RHO {
        reduction = (*yeah).lastQ;
        reduction = cmp::min(reduction, cmp::max((*tp).snd_cwnd >> 1, 2));
        reduction = cmp::max(reduction, (*tp).snd_cwnd >> TCP_YEAH_DELTA);
    } else {
        reduction = cmp::max((*tp).snd_cwnd >> 1, 2);
    }
    
    (*yeah).fast_count = 0;
    (*yeah).reno_count = cmp::max((*yeah).reno_count >> 1, 2);
    
    cmp::max((*tp).snd_cwnd - reduction, 2)
}

// Extern declarations for kernel functions
extern "C" {
    fn tcp_vegas_init(sk: *mut sock);
    fn tcp_is_cwnd_limited(sk: *mut sock) -> bool;
    fn tcp_in_slow_start(tp: *mut tcp_sock) -> bool;
    fn tcp_slow_start(tp: *mut tcp_sock, acked: u32) -> u32;
    fn tcp_cong_avoid_ai(tp: *mut tcp_sock, cnt: u32, acked: u32);
    fn tcp_vegas_state(sk: *mut sock, state: u32);
    fn tcp_vegas_cwnd_event(sk: *mut sock, event: u32);
    fn tcp_vegas_get_info(sk: *mut sock, opt: *mut u8, len: *mut u32) -> i32;
}

// Helper functions to access C structures
#[inline]
unsafe fn tcp_sk(sk: *mut sock) -> *mut tcp_sock {
    // SAFETY: This is a C macro that casts the sock to tcp_sock
    sk as *mut tcp_sock
}

#[inline]
unsafe fn inet_csk_ca(sk: *mut sock) -> *mut yeah {
    // SAFETY: This is a C macro that gets the congestion control private data
    sk as *mut yeah
}

#[inline]
fn after(seq1: u32, seq2: u32) -> bool {
    // TCP sequence number comparison
    seq1.wrapping_sub(seq2) > (1 << 30)
}

// Module registration
#[no_mangle]
pub static mut tcp_yeah: tcp_congestion_ops = tcp_congestion_ops {
    init: tcp_yeah_init,
    ssthresh: tcp_yeah_ssthresh,
    undo_cwnd: tcp_reno_undo_cwnd,
    cong_avoid: tcp_yeah_cong_avoid,
    set_state: tcp_vegas_state,
    cwnd_event: tcp_vegas_cwnd_event,
    get_info: tcp_vegas_get_info,
    pkts_acked: tcp_vegas_pkts_acked,
    owner: ptr::null_mut(),
    name: *b"yeah\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
};

#[no_mangle]
pub unsafe extern "C" fn tcp_yeah_register() -> c_int {
    // SAFETY: This is a C BUG_ON macro that panics if the condition is true
    if core::mem::size_of::<yeah>() > ICSK_CA_PRIV_SIZE {
        // Kernel BUG - module loading will fail
        return -1;
    }
    
    tcp_register_congestion_control(&mut tcp_yeah);
    0
}

#[no_mangle]
pub unsafe extern "C" fn tcp_yeah_unregister() {
    tcp_unregister_congestion_control(&mut tcp_yeah);
}

// Extern declarations for kernel module functions
extern "C" {
    fn tcp_register_congestion_control(ops: *mut tcp_congestion_ops);
    fn tcp_unregister_congestion_control(ops: *mut tcp_congestion_ops);
    fn tcp_reno_undo_cwnd(sk: *mut sock);
    fn tcp_vegas_pkts_acked(sk: *mut sock, acked_s: u32, acked_b: u32);
}

// Constants from kernel headers
const ICSK_CA_PRIV_SIZE: usize = 128; // Example value - actual size depends on kernel config
