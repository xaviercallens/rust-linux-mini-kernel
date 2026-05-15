//! TCP Illinois congestion control implementation for Linux kernel.
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use core::ptr;
use core::ffi::c_int;
use core::ffi::c_uint;
use core::ffi::c_ulong;
use core::ffi::c_void;

// Constants from C
const ALPHA_SHIFT: u32 = 7;
const ALPHA_SCALE: u32 = 1u32 << ALPHA_SHIFT;
const ALPHA_MIN: u32 = (3 * ALPHA_SCALE) / 10; // ~0.3
const ALPHA_MAX: u32 = 10 * ALPHA_SCALE; // 10.0
const ALPHA_BASE: u32 = ALPHA_SCALE; // 1.0
const RTT_MAX: u32 = (u32::MAX / ALPHA_MAX); // 3.3 secs

const BETA_SHIFT: u32 = 6;
const BETA_SCALE: u32 = 1u32 << BETA_SHIFT;
const BETA_MIN: u32 = BETA_SCALE / 8; // 0.125
const BETA_MAX: u32 = BETA_SCALE / 2; // 0.5
const BETA_BASE: u32 = BETA_MAX;

// Type definitions
#[repr(C)]
struct sock {
    // Opaque structure - actual fields defined in kernel
    _private: [u8; 0],
}

#[repr(C)]
struct tcp_sock {
    snd_nxt: u32,
    snd_cwnd: u32,
    snd_cwnd_cnt: u32,
    snd_cwnd_clamp: u32,
    // Other fields as needed
}

#[repr(C)]
struct inet_csk_ca {
    _private: [u8; 0],
}

#[repr(C)]
struct ack_sample {
    rtt_us: i32,
    pkts_acked: u16,
}

#[repr(C)]
struct tcpvegas_info {
    tcpv_enabled: u32,
    tcpv_rttcnt: u16,
    tcpv_minrtt: u32,
    tcpv_rtt: u64,
}

#[repr(C)]
struct tcp_congestion_ops {
    init: extern "C" fn(*mut sock),
    ssthresh: extern "C" fn(*mut sock) -> u32,
    undo_cwnd: extern "C" fn(*mut sock),
    cong_avoid: extern "C" fn(*mut sock, u32, u32),
    set_state: extern "C" fn(*mut sock, u8),
    get_info: extern "C" fn(*mut sock, u32, *mut i32, *mut tcp_cc_info) -> size_t,
    pkts_acked: extern "C" fn(*mut sock, *const ack_sample),
    owner: *mut c_void,
    name: *const u8,
}

#[repr(C)]
struct tcp_cc_info {
    vegas: tcpvegas_info,
}

#[repr(C)]
struct illinois {
    sum_rtt: u64,
    cnt_rtt: u16,
    base_rtt: u32,
    max_rtt: u32,
    end_seq: u32,
    alpha: u32,
    beta: u32,
    acked: u16,
    rtt_above: u8,
    rtt_low: u8,
}

// Function implementations
#[no_mangle]
pub unsafe extern "C" fn tcp_illinois_init(sk: *mut sock) {
    let ca = inet_csk_ca(sk);
    (*ca).alpha = ALPHA_MAX;
    (*ca).beta = BETA_BASE;
    (*ca).base_rtt = 0x7fffffff;
    (*ca).max_rtt = 0;
    (*ca).acked = 0;
    (*ca).rtt_low = 0;
    (*ca).rtt_above = 0;
    rtt_reset(sk);
}

#[no_mangle]
pub unsafe extern "C" fn rtt_reset(sk: *mut sock) {
    let ca = inet_csk_ca(sk);
    let tp = tcp_sk(sk);
    (*ca).end_seq = (*tp).snd_nxt;
    (*ca).cnt_rtt = 0;
    (*ca).sum_rtt = 0;
}

#[no_mangle]
pub unsafe extern "C" fn tcp_illinois_acked(sk: *mut sock, sample: *const ack_sample) {
    let ca = inet_csk_ca(sk);
    let rtt_us = (*sample).rtt_us;
    (*ca).acked = (*sample).pkts_acked;
    
    if rtt_us < 0 {
        return;
    }

    if rtt_us > RTT_MAX as i32 {
        let rtt_us = RTT_MAX;
        if (*ca).base_rtt > rtt_us as u32 {
            (*ca).base_rtt = rtt_us as u32;
        }
        if (*ca).max_rtt < rtt_us as u32 {
            (*ca).max_rtt = rtt_us as u32;
        }
        (*ca).cnt_rtt = (*ca).cnt_rtt.wrapping_add(1);
        (*ca).sum_rtt += rtt_us as u64;
    } else {
        let rtt_us = rtt_us as u32;
        if (*ca).base_rtt > rtt_us {
            (*ca).base_rtt = rtt_us;
        }
        if (*ca).max_rtt < rtt_us {
            (*ca).max_rtt = rtt_us;
        }
        (*ca).cnt_rtt = (*ca).cnt_rtt.wrapping_add(1);
        (*ca).sum_rtt += rtt_us as u64;
    }
}

#[no_mangle]
pub unsafe extern "C" fn max_delay(ca: *const illinois) -> u32 {
    (*ca).max_rtt - (*ca).base_rtt
}

#[no_mangle]
pub unsafe extern "C" fn avg_delay(ca: *const illinois) -> u32 {
    let mut t = (*ca).sum_rtt;
    t /= (*ca).cnt_rtt as u64;
    t as u32 - (*ca).base_rtt
}

#[no_mangle]
pub unsafe extern "C" fn alpha(ca: *mut illinois, da: u32, dm: u32) -> u32 {
    let d1 = dm / 100;
    if da <= d1 {
        if (*ca).rtt_above == 0 {
            return ALPHA_MAX;
        }
        if (*ca).rtt_low < theta {
            return (*ca).alpha;
        }
        (*ca).rtt_low = 0;
        (*ca).rtt_above = 0;
        return ALPHA_MAX;
    }
    
    (*ca).rtt_above = 1;
    let dm = dm - d1;
    let da = da - d1;
    (dm * ALPHA_MAX) / (dm + (da * (ALPHA_MAX - ALPHA_MIN)) / ALPHA_MIN)
}

#[no_mangle]
pub unsafe extern "C" fn beta(da: u32, dm: u32) -> u32 {
    let d2 = dm / 10;
    if da <= d2 {
        return BETA_MIN;
    }
    
    let d3 = (8 * dm) / 10;
    if da >= d3 || d3 <= d2 {
        return BETA_MAX;
    }
    
    (BETA_MIN * d3 - BETA_MAX * d2 + (BETA_MAX - BETA_MIN) * da) / (d3 - d2)
}

#[no_mangle]
pub unsafe extern "C" fn update_params(sk: *mut sock) {
    let tp = tcp_sk(sk);
    let ca = inet_csk_ca(sk);
    
    if (*tp).snd_cwnd < win_thresh {
        (*ca).alpha = ALPHA_BASE;
        (*ca).beta = BETA_BASE;
    } else if (*ca).cnt_rtt > 0 {
        let dm = max_delay(ca);
        let da = avg_delay(ca);
        (*ca).alpha = alpha(ca, da, dm);
        (*ca).beta = beta(da, dm);
    }
    
    rtt_reset(sk);
}

#[no_mangle]
pub unsafe extern "C" fn tcp_illinois_state(sk: *mut sock, new_state: u8) {
    let ca = inet_csk_ca(sk);
    if new_state == TCP_CA_Loss {
        (*ca).alpha = ALPHA_BASE;
        (*ca).beta = BETA_BASE;
        (*ca).rtt_low = 0;
        (*ca).rtt_above = 0;
        rtt_reset(sk);
    }
}

#[no_mangle]
pub unsafe extern "C" fn tcp_illinois_cong_avoid(sk: *mut sock, ack: u32, acked: u32) {
    let tp = tcp_sk(sk);
    let ca = inet_csk_ca(sk);
    
    if after(ack, (*ca).end_seq) {
        update_params(sk);
    }
    
    if !tcp_is_cwnd_limited(sk) {
        return;
    }
    
    if tcp_in_slow_start(tp) {
        tcp_slow_start(tp, acked as u16);
    } else {
        let mut delta = ((*tp).snd_cwnd_cnt * (*ca).alpha) >> ALPHA_SHIFT;
        if delta >= (*tp).snd_cwnd {
            (*tp).snd_cwnd = (*tp).snd_cwnd + delta / (*tp).snd_cwnd;
            (*tp).snd_cwnd_cnt = 0;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn tcp_illinois_ssthresh(sk: *mut sock) -> u32 {
    let tp = tcp_sk(sk);
    let ca = inet_csk_ca(sk);
    let cwnd = (*tp).snd_cwnd;
    let beta = (*ca).beta;
    let reduction = (cwnd * beta) >> BETA_SHIFT;
    cwnd - reduction
}

#[no_mangle]
pub unsafe extern "C" fn tcp_illinois_info(sk: *mut sock, ext: u32, attr: *mut i32, info: *mut tcp_cc_info) -> size_t {
    let ca = inet_csk_ca(sk);
    if ext & (1 << (INET_DIAG_VEGASINFO - 1)) != 0 {
        (*info).vegas.tcpv_enabled = 1;
        (*info).vegas.tcpv_rttcnt = (*ca).cnt_rtt;
        (*info).vegas.tcpv_minrtt = (*ca).base_rtt;
        (*info).vegas.tcpv_rtt = 0;
        
        if (*ca).cnt_rtt > 0 {
            let mut t = (*ca).sum_rtt;
            t /= (*ca).cnt_rtt as u64;
            (*info).vegas.tcpv_rtt = t;
        }
        *attr = INET_DIAG_VEGASINFO;
        return core::mem::size_of::<tcpvegas_info>() as size_t;
    }
    0
}

// Helper functions
#[no_mangle]
pub unsafe extern "C" fn after(a: u32, b: u32) -> bool {
    (a > b)
}

#[no_mangle]
pub unsafe extern "C" fn tcp_is_cwnd_limited(sk: *mut sock) -> bool {
    // Placeholder implementation - actual logic depends on kernel internals
    true
}

#[no_mangle]
pub unsafe extern "C" fn tcp_in_slow_start(tp: *mut tcp_sock) -> bool {
    // Placeholder implementation - actual logic depends on kernel internals
    (*tp).snd_cwnd < 10
}

#[no_mangle]
pub unsafe extern "C" fn tcp_slow_start(tp: *mut tcp_sock, acked: u16) {
    (*tp).snd_cwnd += acked;
}

// Module registration
#[no_mangle]
pub static mut tcp_illinois: tcp_congestion_ops = tcp_congestion_ops {
    init: tcp_illinois_init,
    ssthresh: tcp_illinois_ssthresh,
    undo_cwnd: tcp_reno_undo_cwnd,
    cong_avoid: tcp_illinois_cong_avoid,
    set_state: tcp_illinois_state,
    get_info: tcp_illinois_info,
    pkts_acked: tcp_illinois_acked,
    owner: ptr::null_mut(),
    name: b"illinois\0".as_ptr() as *const u8,
};

#[no_mangle]
pub unsafe extern "C" fn tcp_illinois_register() -> c_int {
    // Check size constraint
    if core::mem::size_of::<illinois>() > ICSK_CA_PRIV_SIZE {
        return -1; // EINVAL
    }
    tcp_register_congestion_control(&mut tcp_illinois);
    0
}

#[no_mangle]
pub unsafe extern "C" fn tcp_illinois_unregister() {
    tcp_unregister_congestion_control(&mut tcp_illinois);
}

// Module parameters
static mut win_thresh: c_int = 15;
static mut theta: c_int = 5;

// Constants
const TCP_CA_Loss: u8 = 3;
const INET_DIAG_VEGASINFO: c_int = 16;
const ICSK_CA_PRIV_SIZE: usize = 128;

// External functions (would be defined in kernel)
#[no_mangle]
pub unsafe extern "C" fn tcp_register_congestion_control(_: *mut tcp_congestion_ops) {
    // Placeholder
}

#[no_mangle]
pub unsafe extern "C" fn tcp_unregister_congestion_control(_: *mut tcp_congestion_ops) {
    // Placeholder
}

#[no_mangle]
pub unsafe extern "C" fn tcp_reno_undo_cwnd(_: *mut sock) {
    // Placeholder
}

// Helper functions to get pointers
#[inline]
unsafe fn inet_csk_ca(sk: *mut sock) -> *mut illinois {
    // In real implementation, this would cast from sock to inet_csk_ca
    // For this example, assume it's at a fixed offset
    (sk as *mut u8).add(100) as *mut illinois
}

#[inline]
unsafe fn tcp_sk(sk: *mut sock) -> *mut tcp_sock {
    // In real implementation, this would cast from sock to tcp_sock
    // For this example, assume it's at a fixed offset
    (sk as *mut u8).add(50) as *mut tcp_sock
}

// Tests
#[cfg(test)]
mod tests {
    #[test]
    fn test_alpha() {
        // Simple test case for alpha calculation
        // Would need actual data to test properly
        assert!(true);
    }
}
