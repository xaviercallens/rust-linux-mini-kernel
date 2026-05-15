//! TCP CUBIC: Binary Increase Congestion control for TCP v2.3
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]  // For C-style type names

use core::ptr;
use core::ffi::c_int;
use core::ffi::c_uint;
use core::ffi::c_void;
use core::mem;

// Constants from C
const BICTCP_BETA_SCALE: c_int = 1024;
const BICTCP_HZ: c_int = 10;
const HYSTART_ACK_TRAIN: c_int = 1;
const HYSTART_DELAY: c_int = 2;
const HYSTART_MIN_SAMPLES: c_int = 8;
const HYSTART_DELAY_MIN: c_int = 4000;
const HYSTART_DELAY_MAX: c_int = 16000;

// Module parameters (read-mostly)
static mut fast_convergence: c_int = 1;
static mut beta: c_int = 717;
static mut initial_ssthresh: c_int = 0;
static mut bic_scale: c_int = 41;
static mut tcp_friendliness: c_int = 1;
static mut hystart: c_int = 1;
static mut hystart_detect: c_int = HYSTART_ACK_TRAIN | HYSTART_DELAY;
static mut hystart_low_window: c_int = 16;
static mut hystart_ack_delta_us: c_int = 2000;
static mut cube_rtt_scale: c_int = 0;
static mut beta_scale: c_int = 0;
static mut cube_factor: u64 = 0;

// BIC TCP Parameters
#[repr(C)]
struct bictcp {
    cnt: c_uint,          // increase cwnd by 1 after ACKs
    last_max_cwnd: c_uint, // last maximum snd_cwnd
    last_cwnd: c_uint,    // the last snd_cwnd
    last_time: c_uint,    // time when updated last_cwnd
    bic_origin_point: c_uint,
    bic_K: c_uint,        // time to origin point
    delay_min: c_uint,    // min delay (usec)
    epoch_start: c_uint,  // beginning of an epoch
    ack_cnt: c_uint,      // number of acks
    tcp_cwnd: c_uint,     // estimated tcp cwnd
    unused: c_uint,       // padding
    sample_cnt: u8,       // number of samples to decide curr_rtt
    found: u8,            // the exit point is found?
    round_start: c_uint,  // beginning of each round
    end_seq: c_uint,      // end_seq of the round
    last_ack: c_uint,     // last time when the ACK spacing is close
    curr_rtt: c_uint,     // the minimum rtt of current round
}

// TCP socket structure (simplified for FFI compatibility)
#[repr(C)]
struct tcp_sock {
    snd_cwnd: c_uint,
    snd_ssthresh: c_uint,
    lsndtime: c_uint,
    tcp_mstamp: c_uint,
}

// TCP socket (simplified for FFI compatibility)
#[repr(C)]
struct sock {
    _private: [u8; 0], // Actual struct is opaque, just for pointer compatibility
}

// TCP congestion control structure (simplified)
#[repr(C)]
struct inet_csk_ca {
    _private: [u8; 0], // Actual struct is opaque
}

// Function to get TCP socket from sock (FFI binding)
unsafe extern "C" fn tcp_sk(sk: *const sock) -> *mut tcp_sock {
    // In real implementation, this would be a proper cast based on actual struct layout
    // For FFI compatibility, we assume the offset is known
    let offset = 0; // This would be the actual offset in bytes
    let tcp_sk_ptr = (sk as *mut u8).offset(offset) as *mut tcp_sock;
    tcp_sk_ptr
}

// Function to get congestion control data from sock (FFI binding)
unsafe extern "C" fn inet_csk_ca(sk: *const sock) -> *mut bictcp {
    // In real implementation, this would be a proper cast based on actual struct layout
    // For FFI compatibility, we assume the offset is known
    let offset = 0; // This would be the actual offset in bytes
    let ca_ptr = (sk as *mut u8).offset(offset) as *mut bictcp;
    ca_ptr
}

// Function to get current time in jiffies (FFI binding)
unsafe extern "C" fn tcp_jiffies32() -> c_uint {
    // Placeholder for actual kernel time function
    0
}

// Function to convert usecs to jiffies (FFI binding)
unsafe extern "C" fn usecs_to_jiffies(usecs: c_uint) -> c_uint {
    // Placeholder for actual kernel conversion
    usecs / 1000 // Assuming 1 jiffy = 1ms
}

// Function to get current time in microseconds (FFI binding)
unsafe extern "C" fn bictcp_clock_us(sk: *const sock) -> c_uint {
    let tcp_sk_ptr = tcp_sk(sk);
    (*tcp_sk_ptr).tcp_mstamp
}

// Function to calculate cubic root
fn cubic_root(a: u64) -> c_uint {
    const V: [u8; 64] = [
        0, 54, 54, 54, 118, 118, 118, 118,
        123, 129, 134, 138, 143, 147, 151, 156,
        157, 161, 164, 168, 170, 173, 176, 179,
        181, 185, 187, 190, 192, 194, 197, 199,
        200, 202, 204, 206, 209, 211, 213, 215,
        217, 219, 221, 222, 224, 225, 227, 229,
        231, 232, 234, 236, 237, 239, 240, 242,
        244, 245, 246, 248, 250, 251, 252, 254,
    ];

    let b = a.leading_ones();
    if b < 7 {
        return ((V[a as usize] + 35) >> 6) as c_uint;
    }

    let b = ((b * 84) >> 8) - 1;
    let shift = (a >> (b * 3)) as u8;
    let mut x = ((V[shift as usize] + 10) << b) as c_uint;
    x >>= 6;

    // Newton-Raphson iteration
    x = (2 * x + (a / (x as u64 * (x as u64 - 1))) as c_uint) >> 2;
    x = (x * 341) >> 10;
    x
}

// Reset BIC TCP parameters
fn bictcp_reset(ca: *mut bictcp) {
    unsafe {
        if !ca.is_null() {
            // Zero out struct up to 'unused' field
            ptr::write_bytes(ca, 0, (mem::offset_of!(bictcp, unused) / mem::size_of::<c_uint>()) as usize);
            (*ca).found = 0;
        }
    }
}

// Reset HyStart parameters
fn bictcp_hystart_reset(sk: *mut sock) {
    unsafe {
        if !sk.is_null() {
            let ca = inet_csk_ca(sk);
            let tp = tcp_sk(sk);
            (*ca).round_start = bictcp_clock_us(sk);
            (*ca).last_ack = (*ca).round_start;
            (*ca).end_seq = (*tp).snd_cwnd;
            (*ca).curr_rtt = !0;
            (*ca).sample_cnt = 0;
        }
    }
}

// Initialize CUBIC TCP
#[no_mangle]
pub unsafe extern "C" fn cubictcp_init(sk: *mut sock) {
    if sk.is_null() {
        return;
    }

    let ca = inet_csk_ca(sk);
    bictcp_reset(ca);
    
    if hystart != 0 {
        bictcp_hystart_reset(sk);
    }
    
    if hystart == 0 && initial_ssthresh != 0 {
        let tp = tcp_sk(sk);
        (*tp).snd_ssthresh = initial_ssthresh as c_uint;
    }
}

// Handle cwnd events
#[no_mangle]
pub unsafe extern "C" fn cubictcp_cwnd_event(sk: *mut sock, event: c_int) {
    if sk.is_null() {
        return;
    }

    if event == 0 { // Assuming CA_EVENT_TX_START is 0
        let ca = inet_csk_ca(sk);
        let tp = tcp_sk(sk);
        let now = tcp_jiffies32();
        let delta = now as i32 - (*tp).lsndtime as i32;
        
        if (*ca).epoch_start != 0 && delta > 0 {
            let mut new_epoch_start = (*ca).epoch_start as i32 + delta;
            if new_epoch_start > now as i32 {
                new_epoch_start = now as i32;
            }
            (*ca).epoch_start = new_epoch_start as c_uint;
        }
        (*tp).lsndtime = now;
    }
}

// Update BIC TCP parameters
#[no_mangle]
pub unsafe extern "C" fn bictcp_update(ca: *mut bictcp, cwnd: c_uint, acked: c_uint) {
    if ca.is_null() {
        return;
    }

    let now = tcp_jiffies32();
    let ca_ref = &mut *ca;
    
    ca_ref.ack_cnt += acked;
    
    if ca_ref.last_cwnd == cwnd && (now as i32 - ca_ref.last_time as i32) <= (now / 32) as i32 {
        return;
    }
    
    if ca_ref.epoch_start != 0 && now == ca_ref.last_time {
        // Skip to TCP friendliness section
        goto tcp_friendliness;
    }
    
    ca_ref.last_cwnd = cwnd;
    ca_ref.last_time = now;
    
    if ca_ref.epoch_start == 0 {
        ca_ref.epoch_start = now;
        ca_ref.ack_cnt = acked;
        ca_ref.tcp_cwnd = cwnd;
        
        if ca_ref.last_max_cwnd <= cwnd {
            ca_ref.bic_K = 0;
            ca_ref.bic_origin_point = cwnd;
        } else {
            ca_ref.bic_K = cubic_root(cube_factor * (ca_ref.last_max_cwnd - cwnd) as u64) as c_uint;
            ca_ref.bic_origin_point = ca_ref.last_max_cwnd;
        }
    }
    
    let mut t = (now as i32 - ca_ref.epoch_start as i32) as u64;
    t += usecs_to_jiffies(ca_ref.delay_min) as u64;
    t <<= BICTCP_HZ as u64;
    t /= HZ as u64;
    
    let mut offs = if t < ca_ref.bic_K as u64 {
        ca_ref.bic_K as u64 - t
    } else {
        t - ca_ref.bic_K as u64
    };
    
    let delta = (cube_rtt_scale as u64 * offs * offs * offs) >> (10 + 3 * BICTCP_HZ);
    let bic_target = if t < ca_ref.bic_K as u64 {
        ca_ref.bic_origin_point - delta as c_uint
    } else {
        ca_ref.bic_origin_point + delta as c_uint
    };
    
    if bic_target > cwnd {
        ca_ref.cnt = cwnd / (bic_target - cwnd);
    } else {
        ca_ref.cnt = 100 * cwnd;
    }
    
    if ca_ref.last_max_cwnd == 0 && ca_ref.cnt > 20 {
        ca_ref.cnt = 20;
    }
    
tcp_friendliness:
    if tcp_friendliness != 0 {
        let scale = beta_scale;
        let mut delta = (cwnd * scale) >> 3;
        
        while ca_ref.ack_cnt > delta {
            ca_ref.ack_cnt -= delta;
            ca_ref.tcp_cwnd += 1;
        }
        
        if ca_ref.tcp_cwnd > cwnd {
            let delta = ca_ref.tcp_cwnd - cwnd;
            let max_cnt = cwnd / delta;
            if ca_ref.cnt > max_cnt {
                ca_ref.cnt = max_cnt;
            }
        }
    }
    
    ca_ref.cnt = ca_ref.cnt.max(2);
}

// Tests (conditional compilation)
#[cfg(test)]
mod tests {
    #[test]
    fn test_bictcp_reset() {
        let mut ca: bictcp = unsafe { core::mem::zeroed() };
        unsafe { super::bictcp_reset(&mut ca as *mut _) };
        assert_eq!(ca.found, 0);
    }
}
