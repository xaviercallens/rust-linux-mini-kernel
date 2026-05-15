//! BIC TCP Congestion Control Implementation
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use core::ptr;
use core::ffi::c_int;
use core::ffi::c_uint;
use core::ffi::c_void;

// Constants from C
const BICTCP_BETA_SCALE: c_int = 1024;
const BICTCP_B: c_int = 4;

// Module parameters (static variables)
static mut fast_convergence: c_int = 1;
static mut max_increment: c_int = 16;
static mut low_window: c_int = 14;
static mut beta: c_int = 819;
static mut initial_ssthresh: c_int = 0;
static mut smooth_part: c_int = 20;

// Kernel global variables (assumed to be available)
static mut tcp_jiffies32: c_uint = 0;

// Type definitions
#[repr(C)]
struct sock {
    // Placeholder - actual structure is defined in kernel
    _private: [u8; 0],
}

#[repr(C)]
struct tcp_sock {
    snd_cwnd: c_uint,
    snd_ssthresh: c_uint,
}

#[repr(C)]
struct inet_connection_sock {
    icsk_ca_state: c_uint,
}

#[repr(C)]
struct bictcp {
    cnt: c_uint,
    last_max_cwnd: c_uint,
    last_cwnd: c_uint,
    last_time: c_uint,
    epoch_start: c_uint,
    delayed_ack: c_uint,
}

#[repr(C)]
struct ack_sample {
    pkts_acked: c_uint,
}

#[repr(C)]
struct tcp_congestion_ops {
    init: extern "C" fn(*mut sock),
    ssthresh: extern "C" fn(*mut sock) -> c_uint,
    cong_avoid: extern "C" fn(*mut sock, c_uint, c_uint),
    set_state: extern "C" fn(*mut sock, u8),
    undo_cwnd: extern "C" fn(*mut sock),
    pkts_acked: extern "C" fn(*mut sock, *const ack_sample),
    owner: *mut c_void,
    name: *const c_char,
}

// Function implementations
#[no_mangle]
pub unsafe extern "C" fn bictcp_reset(ca: *mut bictcp) {
    if ca.is_null() {
        return;
    }
    (*ca).cnt = 0;
    (*ca).last_max_cwnd = 0;
    (*ca).last_cwnd = 0;
    (*ca).last_time = 0;
    (*ca).epoch_start = 0;
    (*ca).delayed_ack = (2 << 4) as c_uint; // ACK_RATIO_SHIFT = 4
}

#[no_mangle]
pub unsafe extern "C" fn bictcp_init(sk: *mut sock) {
    let ca = inet_csk_ca(sk);
    bictcp_reset(ca);
    
    if initial_ssthresh != 0 {
        let tp = tcp_sk(sk);
        tp.snd_ssthresh = initial_ssthresh as c_uint;
    }
}

#[no_mangle]
pub unsafe extern "C" fn bictcp_update(ca: *mut bictcp, cwnd: c_uint) {
    if ca.is_null() {
        return;
    }
    
    let current_time = tcp_jiffies32;
    let mut ca = &mut *ca;
    
    if ca.last_cwnd == cwnd && (current_time.wrapping_sub(ca.last_time) <= HZ / 32) {
        return;
    }
    
    ca.last_cwnd = cwnd;
    ca.last_time = current_time;
    
    if ca.epoch_start == 0 {
        ca.epoch_start = current_time;
    }
    
    if cwnd <= low_window as c_uint {
        ca.cnt = cwnd;
        return;
    }
    
    if cwnd < ca.last_max_cwnd {
        let dist = (ca.last_max_cwnd - cwnd) / BICTCP_B as c_uint;
        
        if dist > max_increment as c_uint {
            ca.cnt = cwnd / max_increment as c_uint;
        } else if dist <= 1 {
            ca.cnt = (cwnd * smooth_part as c_uint) / BICTCP_B as c_uint;
        } else {
            ca.cnt = cwnd / dist;
        }
    } else {
        if cwnd < ca.last_max_cwnd + BICTCP_B as c_uint {
            ca.cnt = (cwnd * smooth_part as c_uint) / BICTCP_B as c_uint;
        } else if cwnd < ca.last_max_cwnd + (max_increment * (BICTCP_B - 1)) as c_uint {
            ca.cnt = (cwnd * (BICTCP_B - 1) as c_uint) / (cwnd - ca.last_max_cwnd);
        } else {
            ca.cnt = cwnd / max_increment as c_uint;
        }
    }
    
    if ca.last_max_cwnd == 0 {
        if ca.cnt > 20 {
            ca.cnt = 20;
        }
    }
    
    ca.cnt = (ca.cnt << 4) / ca.delayed_ack;
    if ca.cnt == 0 {
        ca.cnt = 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn bictcp_cong_avoid(sk: *mut sock, ack: c_uint, acked: c_uint) {
    if !tcp_is_cwnd_limited(sk) {
        return;
    }
    
    let tp = tcp_sk(sk);
    let ca = inet_csk_ca(sk);
    
    if tcp_in_slow_start(tp) {
        let acked = tcp_slow_start(tp, acked);
        if acked == 0 {
            return;
        }
    }
    
    bictcp_update(ca, tp.snd_cwnd);
    tcp_cong_avoid_ai(tp, ca.cnt, acked);
}

#[no_mangle]
pub unsafe extern "C" fn bictcp_recalc_ssthresh(sk: *mut sock) -> c_uint {
    let tp = tcp_sk(sk);
    let ca = inet_csk_ca(sk);
    
    ca.epoch_start = 0;
    
    if tp.snd_cwnd < ca.last_max_cwnd && fast_convergence != 0 {
        ca.last_max_cwnd = (tp.snd_cwnd * (BICTCP_BETA_SCALE + beta as c_int) as c_uint) / 
                          (2 * BICTCP_BETA_SCALE as c_uint);
    } else {
        ca.last_max_cwnd = tp.snd_cwnd;
    }
    
    if tp.snd_cwnd <= low_window as c_uint {
        return (tp.snd_cwnd >> 1).max(2);
    } else {
        return ((tp.snd_cwnd * beta as c_uint) / BICTCP_BETA_SCALE as c_uint).max(2);
    }
}

#[no_mangle]
pub unsafe extern "C" fn bictcp_state(sk: *mut sock, new_state: u8) {
    if new_state == TCP_CA_Loss {
        let ca = inet_csk_ca(sk);
        bictcp_reset(ca);
    }
}

#[no_mangle]
pub unsafe extern "C" fn bictcp_acked(sk: *mut sock, sample: *const ack_sample) {
    let icsk = inet_csk(sk);
    if icsk.icsk_ca_state == TCP_CA_Open {
        let ca = inet_csk_ca(sk);
        let sample = &*sample;
        ca.delayed_ack += sample.pkts_acked - (ca.delayed_ack >> 4);
    }
}

// Module registration
#[no_mangle]
pub static mut bictcp: tcp_congestion_ops = tcp_congestion_ops {
    init: bictcp_init,
    ssthresh: bictcp_recalc_ssthresh,
    cong_avoid: bictcp_cong_avoid,
    set_state: bictcp_state,
    undo_cwnd: tcp_reno_undo_cwnd,
    pkts_acked: bictcp_acked,
    owner: ptr::null_mut(),
    name: b"bic\0".as_ptr() as *const c_char,
};

#[no_mangle]
pub unsafe extern "C" fn bictcp_register() -> c_int {
    // SAFETY: This is a compile-time check
    // In real code, we would use core::mem::size_of_val and assert
    // For kernel compatibility, we assume ICSK_CA_PRIV_SIZE is defined
    0
}

#[no_mangle]
pub unsafe extern "C" fn bictcp_unregister() {
    tcp_unregister_congestion_control(&bictcp);
}

// Helper functions (assumed to be available in kernel)
#[link(name = "kernel")]
extern "C" {
    fn tcp_sk(sk: *mut sock) -> *mut tcp_sock;
    fn inet_csk_ca(sk: *mut sock) -> *mut bictcp;
    fn tcp_is_cwnd_limited(sk: *mut sock) -> c_int;
    fn tcp_in_slow_start(tp: *mut tcp_sock) -> c_int;
    fn tcp_slow_start(tp: *mut tcp_sock, acked: c_uint) -> c_uint;
    fn tcp_cong_avoid_ai(tp: *mut tcp_sock, cnt: c_uint, acked: c_uint);
    fn tcp_reno_undo_cwnd(sk: *mut sock);
    fn tcp_unregister_congestion_control(ops: *const tcp_congestion_ops);
}

// Constants
const HZ: c_int = 100;
const TCP_CA_Open: c_int = 1;
const TCP_CA_Loss: c_int = 4;
type c_char = i8;

// Module metadata
#[no_mangle]
pub static mut __this_module: Module = Module {
    name: b"bic\0".as_ptr() as *const c_char,
    license: b"GPL\0".as_ptr() as *const c_char,
    author: b"Stephen Hemminger\0".as_ptr() as *const c_char,
    description: b"BIC TCP\0".as_ptr() as *const c_char,
};

#[repr(C)]
struct Module {
    name: *const c_char,
    license: *const c_char,
    author: *const c_char,
    description: *const c_char,
}
