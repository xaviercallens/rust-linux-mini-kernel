//! TCP Veno congestion control implementation
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(clippy::all)]

use core::ptr;
use libc::{c_int, c_uint, c_void};

// Constants from C
const V_PARAM_SHIFT: u32 = 1;
const BETA: u32 = 3 << V_PARAM_SHIFT;

// Type definitions
#[repr(C)]
struct sock;
#[repr(C)]
struct tcp_sock;
#[repr(C)]
struct ack_sample {
    rtt_us: c_int,
}
#[repr(C)]
struct tcp_congestion_ops {
    init: extern "C" fn(*mut sock),
    ssthresh: extern "C" fn(*mut sock) -> u32,
    undo_cwnd: extern "C" fn(*mut sock),
    cong_avoid: extern "C" fn(*mut sock, u32, u32),
    pkts_acked: extern "C" fn(*mut sock, *const ack_sample),
    set_state: extern "C" fn(*mut sock, u8),
    cwnd_event: extern "C" fn(*mut sock, c_int),
    owner: *mut c_void,
    name: *const u8,
}

#[repr(C)]
struct veno {
    doing_veno_now: u8,
    cntrtt: u16,
    minrtt: u32,
    basertt: u32,
    inc: u32,
    diff: u32,
}

// Function implementations
/// Initialize Veno congestion control
///
/// # Safety
/// - `sk` must be a valid pointer to sock
#[no_mangle]
pub unsafe extern "C" fn veno_enable(sk: *mut sock) {
    let veno = inet_csk_ca(sk);
    (*veno).doing_veno_now = 1;
    (*veno).minrtt = 0x7FFFFFFF;
}

/// Disable Veno congestion control
///
/// # Safety
/// - `sk` must be a valid pointer to sock
#[no_mangle]
pub unsafe extern "C" fn veno_disable(sk: *mut sock) {
    let veno = inet_csk_ca(sk);
    (*veno).doing_veno_now = 0;
}

/// Initialize Veno state
///
/// # Safety
/// - `sk` must be a valid pointer to sock
#[no_mangle]
pub unsafe extern "C" fn tcp_veno_init(sk: *mut sock) {
    let veno = inet_csk_ca(sk);
    (*veno).basertt = 0x7FFFFFFF;
    (*veno).inc = 1;
    veno_enable(sk);
}

/// Process acknowledged packets for Veno
///
/// # Safety
/// - `sk` must be a valid pointer to sock
/// - `sample` must be a valid pointer to ack_sample
#[no_mangle]
pub unsafe extern "C" fn tcp_veno_pkts_acked(sk: *mut sock, sample: *const ack_sample) {
    let veno = inet_csk_ca(sk);
    let vrtt = if (*sample).rtt_us < 0 {
        return;
    } else {
        (*sample).rtt_us + 1
    };

    // Filter to find propagation delay
    if vrtt < (*veno).basertt {
        (*veno).basertt = vrtt as u32;
    }

    // Find the min rtt during the last rtt
    (*veno).minrtt = (*veno).minrtt.min(vrtt as u32);
    (*veno).cntrtt += 1;
}

/// Update Veno state based on TCP state
///
/// # Safety
/// - `sk` must be a valid pointer to sock
#[no_mangle]
pub unsafe extern "C" fn tcp_veno_state(sk: *mut sock, ca_state: u8) {
    if ca_state == TCP_CA_Open {
        veno_enable(sk);
    } else {
        veno_disable(sk);
    }
}

/// Handle congestion avoidance for Veno
///
/// # Safety
/// - `sk` must be a valid pointer to sock
#[no_mangle]
pub unsafe extern "C" fn tcp_veno_cwnd_event(sk: *mut sock, event: c_int) {
    if event == CA_EVENT_CWND_RESTART || event == CA_EVENT_TX_START {
        tcp_veno_init(sk);
    }
}

/// Main congestion avoidance algorithm for Veno
///
/// # Safety
/// - `sk` must be a valid pointer to sock
#[no_mangle]
pub unsafe extern "C" fn tcp_veno_cong_avoid(sk: *mut sock, ack: u32, acked: u32) {
    let tp = tcp_sk(sk);
    let veno = inet_csk_ca(sk);

    if (*veno).doing_veno_now == 0 {
        tcp_reno_cong_avoid(sk, ack, acked);
        return;
    }

    // Limited by applications
    if !tcp_is_cwnd_limited(sk) {
        return;
    }

    // We do the Veno calculations only if we got enough rtt samples
    if (*veno).cntrtt <= 2 {
        // We don't have enough rtt samples to do the Veno calculation
        tcp_reno_cong_avoid(sk, ack, acked);
    } else {
        let mut target_cwnd: u64 = (tp.snd_cwnd as u64) * (*veno).basertt as u64;
        target_cwnd = target_cwnd << V_PARAM_SHIFT;
        target_cwnd = target_cwnd / (*veno).minrtt as u64;

        (*veno).diff = (tp.snd_cwnd << V_PARAM_SHIFT) as u64 - target_cwnd;

        if tcp_in_slow_start(tp) {
            // Slow start
            let acked = tcp_slow_start(tp, acked);
            if acked == 0 {
                return;
            }
        }

        // Congestion avoidance
        if (*veno).diff < BETA {
            // In the "non-congestive state", increase cwnd every rtt
            tcp_cong_avoid_ai(tp, tp.snd_cwnd, acked);
        } else {
            // In the "congestive state", increase cwnd every other rtt
            if tp.snd_cwnd_cnt >= tp.snd_cwnd {
                if (*veno).inc != 0 && tp.snd_cwnd < tp.snd_cwnd_clamp {
                    tp.snd_cwnd += 1;
                    (*veno).inc = 0;
                } else {
                    (*veno).inc = 1;
                }
                tp.snd_cwnd_cnt = 0;
            } else {
                tp.snd_cwnd_cnt += acked;
            }
        }

        // Clamp cwnd
        if tp.snd_cwnd < 2 {
            tp.snd_cwnd = 2;
        } else if tp.snd_cwnd > tp.snd_cwnd_clamp {
            tp.snd_cwnd = tp.snd_cwnd_clamp;
        }
    }

    // Wipe the slate clean for the next rtt
    (*veno).minrtt = 0x7FFFFFFF;
}

/// Calculate ssthresh for Veno
///
/// # Safety
/// - `sk` must be a valid pointer to sock
#[no_mangle]
pub unsafe extern "C" fn tcp_veno_ssthresh(sk: *mut sock) -> u32 {
    let tp = tcp_sk(sk);
    let veno = inet_csk_ca(sk);

    if (*veno).diff < BETA {
        // In "non-congestive state", cut cwnd by 1/5
        return (tp.snd_cwnd * 4 / 5).max(2);
    } else {
        // In "congestive state", cut cwnd by 1/2
        return (tp.snd_cwnd >> 1).max(2);
    }
}

// Veno congestion control implementation
#[no_mangle]
static mut tcp_veno: tcp_congestion_ops = tcp_congestion_ops {
    init: tcp_veno_init,
    ssthresh: tcp_veno_ssthresh,
    undo_cwnd: tcp_reno_undo_cwnd,
    cong_avoid: tcp_veno_cong_avoid,
    pkts_acked: tcp_veno_pkts_acked,
    set_state: tcp_veno_state,
    cwnd_event: tcp_veno_cwnd_event,
    owner: ptr::null_mut(),
    name: b"veno\0".as_ptr() as *const u8,
};

// Module registration functions
#[no_mangle]
pub unsafe extern "C" fn tcp_veno_register() -> c_int {
    // BUILD_BUG_ON(sizeof(struct veno) > ICSK_CA_PRIV_SIZE)
    0
}

#[no_mangle]
pub unsafe extern "C" fn tcp_veno_unregister() {
    // Unregister congestion control
}

// Helper functions (FFI-compatible)
#[no_mangle]
pub unsafe extern "C" fn inet_csk_ca(sk: *mut sock) -> *mut veno {
    // In real implementation, this would return the private data of the socket
    // For FFI compatibility, we assume it's a pointer to veno
    // This is a placeholder - actual implementation depends on Linux kernel internals
    let priv_data: *mut veno = ptr::null_mut();
    priv_data
}

#[no_mangle]
pub unsafe extern "C" fn tcp_sk(sk: *mut sock) -> *mut tcp_sock {
    // In real implementation, this would return the tcp_sock structure
    // For FFI compatibility, we assume it's a pointer to tcp_sock
    // This is a placeholder - actual implementation depends on Linux kernel internals
    let tcp_sk: *mut tcp_sock = ptr::null_mut();
    tcp_sk
}

#[no_mangle]
pub unsafe extern "C" fn tcp_reno_cong_avoid(sk: *mut sock, ack: u32, acked: u32) {
    // Placeholder for actual Reno congestion avoidance implementation
}

#[no_mangle]
pub unsafe extern "C" fn tcp_reno_undo_cwnd(sk: *mut sock) {
    // Placeholder for actual Reno undo_cwnd implementation
}

#[no_mangle]
pub unsafe extern "C" fn tcp_in_slow_start(tp: *mut tcp_sock) -> c_int {
    // Placeholder for actual slow start check
    0
}

#[no_mangle]
pub unsafe extern "C" fn tcp_slow_start(tp: *mut tcp_sock, acked: u32) -> u32 {
    // Placeholder for actual slow start implementation
    acked
}

#[no_mangle]
pub unsafe extern "C" fn tcp_cong_avoid_ai(tp: *mut tcp_sock, cwnd: u32, acked: u32) {
    // Placeholder for actual congestion avoidance AI implementation
}

#[no_mangle]
pub unsafe extern "C" fn tcp_is_cwnd_limited(sk: *mut sock) -> c_int {
    // Placeholder for actual cwnd limit check
    1
}

// Module exports
#[no_mangle]
pub static TCP_CA_Open: u8 = 1;

#[no_mangle]
pub static CA_EVENT_CWND_RESTART: c_int = 1;
#[no_mangle]
pub static CA_EVENT_TX_START: c_int = 2;
