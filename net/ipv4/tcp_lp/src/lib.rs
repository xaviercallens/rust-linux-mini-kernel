//! TCP Low Priority (TCP-LP) congestion control algorithm
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]  // For C-style type names

use core::ptr;
use core::ffi::c_int;
use core::ffi::c_uint;
use core::ffi::c_void;

// Constants from C
const LP_RESOL: c_uint = 0; // TCP_TS_HZ from Linux kernel headers

// Type definitions
#[repr(C)]
struct sock {
    // Opaque structure - actual fields are in the Linux kernel
} as *mut c_void;

#[repr(C)]
struct tcp_sock {
    rx_opt: tcp_rx_opt,
} as *mut c_void;

#[repr(C)]
struct tcp_rx_opt {
    rcv_tsval: u32,
    rcv_tsecr: u32,
} as *mut c_void;

#[repr(C)]
struct tcp_congestion_ops {
    init: extern "C" fn(*mut sock),
    ssthresh: extern "C" fn(*mut sock) -> u32,
    undo_cwnd: extern "C" fn(*mut sock),
    cong_avoid: extern "C" fn(*mut sock, u32, u32),
    pkts_acked: extern "C" fn(*mut sock, *const ack_sample),
    owner: *mut c_void,
    name: *const c_char,
} as *mut c_void;

#[repr(C)]
struct ack_sample {
    rtt_us: u32,
} as *mut c_void;

#[repr(C)]
struct lp {
    flag: u32,
    sowd: u32,
    owd_min: u32,
    owd_max: u32,
    owd_max_rsv: u32,
    remote_hz: u32,
    remote_ref_time: u32,
    local_ref_time: u32,
    last_drop: u32,
    inference: u32,
} as *mut c_void;

// Function implementations
/// Initialize TCP-LP congestion control
///
/// # Safety
/// - `sk` must be a valid pointer to a socket
#[no_mangle]
pub unsafe extern "C" fn tcp_lp_init(sk: *mut sock) {
    let lp = sk as *mut lp;
    
    (*lp).flag = 0;
    (*lp).sowd = 0;
    (*lp).owd_min = 0xFFFFFFFF;
    (*lp).owd_max = 0;
    (*lp).owd_max_rsv = 0;
    (*lp).remote_hz = 0;
    (*lp).remote_ref_time = 0;
    (*lp).local_ref_time = 0;
    (*lp).last_drop = 0;
    (*lp).inference = 0;
}

/// TCP-LP congestion avoidance
///
/// # Safety
/// - `sk` must be a valid pointer to a socket
#[no_mangle]
pub unsafe extern "C" fn tcp_lp_cong_avoid(sk: *mut sock, ack: u32, acked: u32) {
    let lp = sk as *mut lp;
    
    if !((*lp).flag & (1 << 4)) {
        // Call tcp_reno_cong_avoid - actual implementation would be in kernel
        extern "C" {
            fn tcp_reno_cong_avoid(sk: *mut sock, ack: u32, acked: u32);
        }
        tcp_reno_cong_avoid(sk, ack, acked);
    }
}

/// Estimate remote HZ
///
/// # Safety
/// - `sk` must be a valid pointer to a socket
#[no_mangle]
pub unsafe extern "C" fn tcp_lp_remote_hz_estimator(sk: *mut sock) -> u32 {
    let tp = sk as *mut tcp_sock;
    let lp = sk as *mut lp;
    
    let mut rhz: i64 = (*lp).remote_hz as i64 << 6;
    let mut m: i64 = 0;
    
    if (*lp).remote_ref_time == 0 || (*lp).local_ref_time == 0 {
        return rhz >> 6;
    }
    
    if (*tp).rx_opt.rcv_tsval == (*lp).remote_ref_time || 
       (*tp).rx_opt.rcv_tsecr == (*lp).local_ref_time {
        return rhz >> 6;
    }
    
    m = (LP_RESOL as i64 * 
         ((*tp).rx_opt.rcv_tsval as i64 - (*lp).remote_ref_time as i64)) / 
        ((*tp).rx_opt.rcv_tsecr as i64 - (*lp).local_ref_time as i64);
    
    if m < 0 {
        m = -m;
    }
    
    if rhz > 0 {
        m -= rhz >> 6;
        rhz += m;
    } else {
        rhz = m << 6;
    }
    
    // Update flags
    if (rhz >> 6) > 0 {
        (*lp).flag |= (1 << 0);
    } else {
        (*lp).flag &= !(1 << 0);
    }
    
    // Update reference timestamps
    (*lp).remote_ref_time = (*tp).rx_opt.rcv_tsval;
    (*lp).local_ref_time = (*tp).rx_opt.rcv_tsecr;
    
    rhz >> 6
}

/// Calculate one-way delay
///
/// # Safety
/// - `sk` must be a valid pointer to a socket
#[no_mangle]
pub unsafe extern "C" fn tcp_lp_owd_calculator(sk: *mut sock) -> u32 {
    let tp = sk as *mut tcp_sock;
    let lp = sk as *mut lp;
    
    (*lp).remote_hz = tcp_lp_remote_hz_estimator(sk);
    
    let mut owd: i64 = 0;
    
    if (*lp).flag & (1 << 0) != 0 {
        owd = (*tp).rx_opt.rcv_tsval as i64 * (LP_RESOL as i64 / (*lp).remote_hz as i64) -
              (*tp).rx_opt.rcv_tsecr as i64 * (LP_RESOL as i64 / LP_RESOL as i64);
        
        if owd < 0 {
            owd = -owd;
        }
    }
    
    if owd > 0 {
        (*lp).flag |= (1 << 1);
    } else {
        (*lp).flag &= !(1 << 1);
    }
    
    owd as u32
}

/// Process RTT sample
///
/// # Safety
/// - `sk` must be a valid pointer to a socket
#[no_mangle]
pub unsafe extern "C" fn tcp_lp_rtt_sample(sk: *mut sock, rtt: u32) {
    let lp = sk as *mut lp;
    let mowd = tcp_lp_owd_calculator(sk);
    
    // Skip if no valid data
    if !((*lp).flag & (1 << 0) != 0 && (*lp).flag & (1 << 1) != 0) {
        return;
    }
    
    // Update min OWD
    if mowd < (*lp).owd_min {
        (*lp).owd_min = mowd;
    }
    
    // Update max OWD
    if mowd > (*lp).owd_max {
        if mowd > (*lp).owd_max_rsv {
            if (*lp).owd_max_rsv == 0 {
                (*lp).owd_max = mowd;
            } else {
                (*lp).owd_max = (*lp).owd_max_rsv;
            }
            (*lp).owd_max_rsv = mowd;
        } else {
            (*lp).owd_max = mowd;
        }
    }
    
    // Calculate smoothed OWD
    if (*lp).sowd != 0 {
        let mut mowd_diff = mowd as i64 - ((*lp).sowd as i64 >> 3);
        (*lp).sowd = (*lp).sowd as i64 + mowd_diff;
    } else {
        (*lp).sowd = mowd as i64 << 3;
    }
}

/// Handle packets acknowledged
///
/// # Safety
/// - `sk` must be a valid pointer to a socket
/// - `sample` must be a valid pointer to ack_sample
#[no_mangle]
pub unsafe extern "C" fn tcp_lp_pkts_acked(sk: *mut sock, sample: *const ack_sample) {
    let tp = sk as *mut tcp_sock;
    let lp = sk as *mut lp;
    let now = tcp_time_stamp(tp); // Would be defined in kernel
    
    if (*sample).rtt_us > 0 {
        tcp_lp_rtt_sample(sk, (*sample).rtt_us);
    }
    
    // Calculate inference
    let delta = now - (*tp).rx_opt.rcv_tsecr;
    if delta > 0 {
        (*lp).inference = 3 * delta;
    }
    
    // Test if within inference
    if (*lp).last_drop != 0 && (now - (*lp).last_drop < (*lp).inference) {
        (*lp).flag |= (1 << 4);
    } else {
        (*lp).flag &= !(1 << 4);
    }
    
    // Test if within threshold
    if ((*lp).sowd >> 3) < 
       (*lp).owd_min + 15 * ((*lp).owd_max - (*lp).owd_min) / 100 {
        (*lp).flag |= (1 << 3);
    } else {
        (*lp).flag &= !(1 << 3);
    }
    
    // If within threshold, return
    if (*lp).flag & (1 << 3) != 0 {
        return;
    }
    
    // Reset min/max OWD
    (*lp).owd_min = (*lp).sowd >> 3;
    (*lp).owd_max = (*lp).sowd >> 2;
    (*lp).owd_max_rsv = (*lp).sowd >> 2;
    
    // Handle congestion
    if (*lp).flag & (1 << 4) != 0 {
        // Within inference - drop to 1
        (*tp).snd_cwnd = 1;
    } else {
        // After inference - cut in half
        (*tp).snd_cwnd = (*tp).snd_cwnd >> 1;
        if (*tp).snd_cwnd < 1 {
            (*tp).snd_cwnd = 1;
        }
    }
    
    // Record drop time
    (*lp).last_drop = now;
}

// TCP-LP congestion control operations
#[no_mangle]
pub static mut tcp_lp: tcp_congestion_ops = tcp_congestion_ops {
    init: tcp_lp_init,
    ssthresh: tcp_reno_ssthresh,
    undo_cwnd: tcp_reno_undo_cwnd,
    cong_avoid: tcp_lp_cong_avoid,
    pkts_acked: tcp_lp_pkts_acked,
    owner: ptr::null_mut(),
    name: b"lp\0".as_ptr() as *const c_char,
};

// Extern declarations for functions used but not defined here
extern "C" {
    fn tcp_reno_ssthresh(sk: *mut sock) -> u32;
    fn tcp_reno_undo_cwnd(sk: *mut sock);
    fn tcp_time_stamp(tp: *mut tcp_sock) -> u32;
}

// Module registration
#[no_mangle]
pub unsafe extern "C" fn tcp_lp_register() -> c_int {
    // Check size of struct lp
    if core::mem::size_of::<lp>() > ICSK_CA_PRIV_SIZE {
        return -1; // EINVAL
    }
    
    tcp_register_congestion_control(&mut tcp_lp)
}

#[no_mangle]
pub unsafe extern "C" fn tcp_lp_unregister() {
    tcp_unregister_congestion_control(&mut tcp_lp)
}

// Extern declarations for kernel functions
extern "C" {
    fn tcp_register_congestion_control(ops: *mut tcp_congestion_ops) -> c_int;
    fn tcp_unregister_congestion_control(ops: *mut tcp_congestion_ops);
    fn ICSK_CA_PRIV_SIZE: usize;
}
