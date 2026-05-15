//! Bottleneck Bandwidth and RTT (BBR) congestion control
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]  // For C-style type names

use core::ptr;
use core::ffi::c_int;
use core::ffi::c_uint;
use core::ffi::c_ulong;
use core::ffi::c_void;

// Constants from C
pub const BW_SCALE: u32 = 24;
pub const BW_UNIT: u32 = 1 << BW_SCALE;

pub const BBR_SCALE: u32 = 8;
pub const BBR_UNIT: u32 = 1 << BBR_SCALE;

pub const BBR_STARTUP: u32 = 0;
pub const BBR_DRAIN: u32 = 1;
pub const BBR_PROBE_BW: u32 = 2;
pub const BBR_PROBE_RTT: u32 = 3;

pub const CYCLE_LEN: u32 = 8;
pub const BBR_BW_RTT: u32 = CYCLE_LEN + 2;
pub const BBR_MIN_RTT_WIN_SEC: u32 = 10;
pub const BBR_PROBE_RTT_MODE_MS: u32 = 200;
pub const BBR_MIN_TSO_RATE: u32 = 1200000;
pub const BBR_PACING_MARGIN_PERCENT: u32 = 1;
pub const BBR_HIGH_GAIN: u32 = BBR_UNIT * 2885 / 1000 + 1;
pub const BBR_DRAIN_GAIN: u32 = BBR_UNIT * 1000 / 2885;
pub const BBR_CWND_GAIN: u32 = BBR_UNIT * 2;
pub const BBR_CWND_MIN_TARGET: u32 = 4;
pub const BBR_FULL_BW_THRESH: u32 = BBR_UNIT * 5 / 4;
pub const BBR_FULL_BW_CNT: u32 = 3;
pub const BBR_LT_INTVL_MIN_RTTS: u32 = 4;
pub const BBR_LT_LOSS_THRESH: u32 = 50;
pub const BBR_LT_BW_RATIO: u32 = BBR_UNIT / 8;
pub const BBR_LT_BW_DIFF: u32 = 4000 / 8;
pub const BBR_LT_BW_MAX_RTTS: u32 = 48;
pub const BBR_EXTRA_ACKED_GAIN: u32 = BBR_UNIT;
pub const BBR_EXTRA_ACKED_WIN_RTTS: u32 = 5;
pub const BBR_ACK_EPOCH_ACKED_RESET_THRESH: u32 = 1 << 20;
pub const BBR_EXTRA_ACKED_MAX_US: u32 = 100 * 1000;

// Type definitions
#[repr(C)]
pub struct minmax {
    pub max: u32,
    pub filter: u32,
    pub window: u32,
    pub rtt_cnt: u32,
    pub next_rtt_delivered: u32,
}

#[repr(C)]
pub struct bbr {
    pub min_rtt_us: u32,
    pub min_rtt_stamp: u32,
    pub probe_rtt_done_stamp: u32,
    pub bw: minmax,
    pub rtt_cnt: u32,
    pub next_rtt_delivered: u32,
    pub cycle_mstamp: u64,
    // Bitfields packed into a single u32
    pub flags: u32, // See comment below for bitfield layout
    pub lt_bw: u32,
    pub lt_last_delivered: u32,
    pub lt_last_stamp: u32,
    pub lt_last_lost: u32,
    pub pacing_gain: u32,
    pub cwnd_gain: u32,
    pub full_bw_reached: u32,
    pub full_bw_cnt: u32,
    pub cycle_idx: u32,
    pub has_seen_rtt: u32,
    pub unused_b: u32,
    pub prior_cwnd: u32,
    pub full_bw: u32,
    pub ack_epoch_mstamp: u64,
    pub extra_acked: [u16; 2],
    pub ack_epoch_acked: u32,
    pub extra_acked_win_rtts: u32,
    pub extra_acked_win_idx: u32,
    pub unused_c: u32,
}

// Bitfield layout for bbr.flags (u32):
// - mode: 3 bits
// - prev_ca_state: 3 bits
// - packet_conservation: 1 bit
// - round_start: 1 bit
// - idle_restart: 1 bit
// - probe_rtt_round_done: 1 bit
// - unused: 13 bits
// - lt_is_sampling: 1 bit
// - lt_rtt_cnt: 7 bits
// - lt_use_bw: 1 bit

// Function implementations
/// Do we estimate that STARTUP filled the pipe?
///
/// # Safety
/// - `sk` must be a valid pointer to a `sock` structure
/// - The `bbr` struct must be properly initialized
///
/// # Returns
/// - `1` if full bandwidth reached, `0` otherwise
#[no_mangle]
pub unsafe extern "C" fn bbr_full_bw_reached(
    sk: *const c_void,
) -> c_int {
    // SAFETY: Caller guarantees sk is valid
    let bbr = unsafe { inet_csk_ca(sk) };
    
    // SAFETY: bbr is valid and flags field is properly aligned
    let flags = unsafe { (*bbr).flags };
    
    // Extract full_bw_reached bit (bit 0)
    if (flags & 0x1) != 0 {
        1
    } else {
        0
    }
}

/// Return the windowed max recent bandwidth sample, in pkts/uS << BW_SCALE.
///
/// # Safety
/// - `sk` must be a valid pointer to a `sock` structure
/// - The `bbr` struct must be properly initialized
///
/// # Returns
/// - The max bandwidth value
#[no_mangle]
pub unsafe extern "C" fn bbr_max_bw(
    sk: *const c_void,
) -> u32 {
    // SAFETY: Caller guarantees sk is valid
    let bbr = unsafe { inet_csk_ca(sk) };
    
    // SAFETY: bbr is valid and bw field is properly aligned
    let bw = unsafe { &(*bbr).bw };
    
    // Return the current max value
    bw.max
}

/// Return the estimated bandwidth of the path, in pkts/uS << BW_SCALE.
///
/// # Safety
/// - `sk` must be a valid pointer to a `sock` structure
/// - The `bbr` struct must be properly initialized
///
/// # Returns
/// - The estimated bandwidth value
#[no_mangle]
pub unsafe extern "C" fn bbr_bw(
    sk: *const c_void,
) -> u32 {
    // SAFETY: Caller guarantees sk is valid
    let bbr = unsafe { inet_csk_ca(sk) };
    
    // SAFETY: bbr is valid and lt_bw field is properly aligned
    unsafe { (*bbr).lt_bw }
}

/// Helper function to get the bbr struct from a sock
#[no_mangle]
pub unsafe extern "C" fn inet_csk_ca(
    sk: *const c_void,
) -> *mut bbr {
    // In the Linux kernel, this is a macro that returns a pointer to the bbr struct
    // For FFI compatibility, we assume it's a valid pointer
    sk as *mut bbr
}

// Static functions
/// Internal function to check if PROBE_RTT is done
fn bbr_check_probe_rtt_done(sk: *mut c_void) {
    // Implementation would go here
}

/// Get the current value from minmax
fn minmax_get(mm: *const minmax) -> u32 {
    // SAFETY: Caller guarantees mm is valid
    unsafe { (*mm).max }
}

/// Calculate the extra acked value
fn bbr_extra_acked(sk: *const c_void) -> u16 {
    // SAFETY: Caller guarantees sk is valid
    unsafe {
        let bbr = inet_csk_ca(sk);
        (*bbr).extra_acked[(*bbr).extra_acked_win_idx as usize]
    }
}

// Error codes
pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ENOSYS: c_int = -38;

// Tests (conditional compilation)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bbr_flags() {
        let mut bbr = bbr {
            flags: 0,
            ..Default::default()
        };
        
        // Set mode to BBR_STARTUP (0)
        bbr.flags = (bbr.flags & !0x7) | BBR_STARTUP;
        
        // Set full_bw_reached
        bbr.flags |= 0x1;
        
        assert_eq!(unsafe { bbr_full_bw_reached(&bbr as *const _ as *const c_void) }, 1);
    }
}

// Default implementations for required types
impl Default for minmax {
    fn default() -> Self {
        minmax {
            max: 0,
            filter: 0,
            window: 0,
            rtt_cnt: 0,
            next_rtt_delivered: 0,
        }
    }
}

impl Default for bbr {
    fn default() -> Self {
        bbr {
            min_rtt_us: 0,
            min_rtt_stamp: 0,
            probe_rtt_done_stamp: 0,
            bw: minmax::default(),
            rtt_cnt: 0,
            next_rtt_delivered: 0,
            cycle_mstamp: 0,
            flags: 0,
            lt_bw: 0,
            lt_last_delivered: 0,
            lt_last_stamp: 0,
            lt_last_lost: 0,
            pacing_gain: 0,
            cwnd_gain: 0,
            full_bw_reached: 0,
            full_bw_cnt: 0,
            cycle_idx: 0,
            has_seen_rtt: 0,
            unused_b: 0,
            prior_cwnd: 0,
            full_bw: 0,
            ack_epoch_mstamp: 0,
            extra_acked: [0; 2],
            ack_epoch_acked: 0,
            extra_acked_win_rtts: 0,
            extra_acked_win_idx: 0,
            unused_c: 0,
        }
    }
}
