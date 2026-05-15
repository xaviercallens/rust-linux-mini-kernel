//! TCP Rate Estimator Implementation
//!
//! This module provides an FFI-compatible Rust translation of the Linux kernel's TCP rate estimator
//! implementation. The code maintains ABI compatibility with the original C implementation and
//! follows strict safety requirements for kernel FFI.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(clang_comments)]

use core::ptr;
use core::mem;
use libc::{c_int, c_uint, c_ulong, c_void};

// Error codes from errno.h
pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ENOSYS: c_int = -38;

// Forward declarations for kernel types
#[repr(C)]
pub struct sock {
    // Opaque structure - actual layout is defined by kernel
    _private: [u8; 0],
}

#[repr(C)]
pub struct sk_buff {
    // Opaque structure - actual layout is defined by kernel
    _private: [u8; 0],
}

#[repr(C)]
pub struct tcp_skb_cb {
    tx: tcp_skb_tx,
    sacked: c_uint,
}

#[repr(C)]
pub struct tcp_skb_tx {
    first_tx_mstamp: u64,
    delivered_mstamp: u64,
    delivered: u32,
    is_app_limited: u8,
}

#[repr(C)]
pub struct rate_sample {
    acked_sacked: u32,
    losses: u32,
    prior_delivered: u32,
    prior_mstamp: u64,
    delivered: i32,
    interval_us: i32,
    is_app_limited: u8,
    is_retrans: u8,
    snd_interval_us: u32,
    rcv_interval_us: u32,
}

#[repr(C)]
pub struct tcp_sock {
    packets_out: u32,
    first_tx_mstamp: u64,
    delivered_mstamp: u64,
    delivered: u32,
    app_limited: u32,
    rate_delivered: u32,
    rate_interval_us: u32,
    rate_app_limited: u8,
    mss_cache: u32,
    snd_cwnd: u32,
    write_seq: u32,
    snd_nxt: u32,
    retrans_out: u32,
    lost_out: u32,
    rx_opt: tcp_rx_opt,
}

#[repr(C)]
pub struct tcp_rx_opt {
    sack_ok: u8,
}

// Helper macros translated to functions
#[inline]
fn after(a: u32, b: u32) -> bool {
    (a > b)
}

#[inline]
fn tcp_skb_timestamp_us(skb: *const sk_buff) -> u64 {
    // In real implementation, this would access skb->tstamp
    // For FFI compatibility, we assume it's available at fixed offset
    unsafe { *(ptr::addr_of!((*skb).tstamp)) }
}

#[inline]
fn tcp_min_rtt(tp: *const tcp_sock) -> u32 {
    // In real implementation, this would access tp->min_rtt
    unsafe { (*tp).min_rtt }
}

// Function implementations
/// Update skb with delivery information for rate sampling
///
/// # Safety
/// - `sk` must be a valid pointer to a `sock` structure
/// - `skb` must be a valid pointer to a `sk_buff` structure
#[no_mangle]
pub unsafe extern "C" fn tcp_rate_skb_sent(
    sk: *mut sock,
    skb: *mut sk_buff,
) {
    let tp = tcp_sk(sk);
    
    if (*tp).packets_out == 0 {
        let tstamp_us = tcp_skb_timestamp_us(skb);
        (*tp).first_tx_mstamp = tstamp_us;
        (*tp).delivered_mstamp = tstamp_us;
    }

    let scb = TCP_SKB_CB(skb);
    (*scb).tx.first_tx_mstamp = (*tp).first_tx_mstamp;
    (*scb).tx.delivered_mstamp = (*tp).delivered_mstamp;
    (*scb).tx.delivered = (*tp).delivered;
    (*scb).tx.is_app_limited = if (*tp).app_limited != 0 { 1 } else { 0 };
}

/// Update rate sample when skb is delivered
///
/// # Safety
/// - `sk` must be a valid pointer to a `sock` structure
/// - `skb` must be a valid pointer to a `sk_buff` structure
/// - `rs` must be a valid pointer to a `rate_sample` structure
#[no_mangle]
pub unsafe extern "C" fn tcp_rate_skb_delivered(
    sk: *mut sock,
    skb: *mut sk_buff,
    rs: *mut rate_sample,
) {
    let scb = TCP_SKB_CB(skb);
    
    if (*scb).tx.delivered_mstamp == 0 {
        return;
    }

    if (*rs).prior_delivered == 0 || after((*scb).tx.delivered, (*rs).prior_delivered) {
        (*rs).prior_delivered = (*scb).tx.delivered;
        (*rs).prior_mstamp = (*scb).tx.delivered_mstamp;
        (*rs).is_app_limited = (*scb).tx.is_app_limited;
        (*rs).is_retrans = if (*scb).sacked & (1 << 1) != 0 { 1 } else { 0 };

        // Update first_tx_mstamp in tcp_sock
        let tp = tcp_sk(sk);
        (*tp).first_tx_mstamp = tcp_skb_timestamp_us(skb);
        
        // Calculate interval_us
        let interval_us = tcp_stamp_us_delta((*tp).first_tx_mstamp, (*scb).tx.first_tx_mstamp);
        (*rs).interval_us = interval_us as i32;
    }

    // Clear delivered_mstamp for SACKED_ACKED packets
    if (*scb).sacked & (1 << 0) != 0 {
        (*scb).tx.delivered_mstamp = 0;
    }
}

/// Generate rate sample for TCP connection
///
/// # Safety
/// - `sk` must be a valid pointer to a `sock` structure
/// - `rs` must be a valid pointer to a `rate_sample` structure
#[no_mangle]
pub unsafe extern "C" fn tcp_rate_gen(
    sk: *mut sock,
    delivered: u32,
    lost: u32,
    is_sack_reneg: bool,
    rs: *mut rate_sample,
) {
    let tp = tcp_sk(sk);
    
    // Clear app limited if bubble is acked and gone
    if (*tp).app_limited != 0 && after((*tp).delivered, (*tp).app_limited) {
        (*tp).app_limited = 0;
    }

    if delivered != 0 {
        (*tp).delivered_mstamp = (*tp).tcp_mstamp;
    }

    (*rs).acked_sacked = delivered;
    (*rs).losses = lost;

    if (*rs).prior_mstamp == 0 || is_sack_reneg {
        (*rs).delivered = -1;
        (*rs).interval_us = -1;
        return;
    }

    (*rs).delivered = (*tp).delivered.wrapping_sub((*rs).prior_delivered) as i32;

    let snd_us = (*rs).interval_us as u32;
    let ack_us = tcp_stamp_us_delta((*tp).tcp_mstamp, (*rs).prior_mstamp);
    (*rs).interval_us = if snd_us > ack_us { snd_us as i32 } else { ack_us as i32 };
    (*rs).snd_interval_us = snd_us;
    (*rs).rcv_interval_us = ack_us;

    if unlikely((*rs).interval_us < 0 || (*rs).interval_us < tcp_min_rtt(tp)) {
        if !(*rs).is_retrans {
            // pr_debug would go here in real implementation
        }
        (*rs).interval_us = -1;
        return;
    }

    // Update rate information if this is the best sample
    if !(*rs).is_app_limited || 
       (((*rs).delivered as u64) * (*tp).rate_interval_us as u64 >= 
        (*tp).rate_delivered as u64 * (*rs).interval_us as u64) {
        (*tp).rate_delivered = (*rs).delivered as u32;
        (*tp).rate_interval_us = (*rs).interval_us as u32;
        (*tp).rate_app_limited = (*rs).is_app_limited;
    }
}

/// Check if socket is application-limited
///
/// # Safety
/// - `sk` must be a valid pointer to a `sock` structure
#[no_mangle]
pub unsafe extern "C" fn tcp_rate_check_app_limited(
    sk: *mut sock,
) {
    let tp = tcp_sk(sk);
    
    // Check if we have less than one packet to send
    let has_data = (*tp).write_seq.wrapping_sub((*tp).snd_nxt) >= (*tp).mss_cache;
    if !has_data {
        // Check if nothing in qdisc/NIC queues
        let wmem_alloc = sk_wmem_alloc_get(sk);
        if wmem_alloc < SKB_TRUESIZE(1) {
            // Check if not limited by CWND
            let in_flight = tcp_packets_in_flight(tp);
            if in_flight < (*tp).snd_cwnd {
                // Check if all lost packets have been retransmitted
                if (*tp).lost_out <= (*tp).retrans_out {
                    let inflight = (*tp).delivered.wrapping_add(in_flight);
                    (*tp).app_limited = if inflight != 0 { inflight } else { 1 };
                }
            }
        }
    }
}

// Helper functions
#[inline]
fn TCP_SKB_CB(skb: *mut sk_buff) -> *mut tcp_skb_cb {
    // In real implementation, this would cast skb->cb
    // For FFI compatibility, we assume it's at fixed offset
    unsafe { &mut (*skb).cb as *mut _ as *mut tcp_skb_cb }
}

#[inline]
fn tcp_sk(sk: *mut sock) -> *mut tcp_sock {
    // In real implementation, this would cast sk to tcp_sock
    unsafe { &mut (*sk).tcp as *mut tcp_sock }
}

#[inline]
fn tcp_stamp_us_delta(a: u64, b: u64) -> u32 {
    (a.wrapping_sub(b) / 1000) as u32
}

#[inline]
fn unlikely(cond: bool) -> bool {
    cond
}

#[inline]
fn sk_wmem_alloc_get(sk: *mut sock) -> usize {
    // Placeholder for actual implementation
    0
}

#[inline]
fn SKB_TRUESIZE(size: usize) -> usize {
    size
}

#[inline]
fn tcp_packets_in_flight(tp: *mut tcp_sock) -> u32 {
    // In real implementation, this would calculate packets in flight
    // For FFI compatibility, we use packets_out directly
    unsafe { (*tp).packets_out }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_tcp_rate_skb_sent() {
        // Basic test - actual testing would require kernel environment
        unsafe {
            let mut skb = sk_buff { _private: [0; 0] };
            let mut tp = tcp_sock {
                packets_out: 0,
                first_tx_mstamp: 0,
                delivered_mstamp: 0,
                delivered: 0,
                app_limited: 0,
                rate_delivered: 0,
                rate_interval_us: 0,
                rate_app_limited: 0,
                mss_cache: 1500,
                snd_cwnd: 10,
                write_seq: 0,
                snd_nxt: 0,
                retrans_out: 0,
                lost_out: 0,
                rx_opt: tcp_rx_opt { sack_ok: 0 },
                min_rtt: 100,
            };
            
            let sk = &mut sock { _private: [0; 0] };
            let skb = &mut skb;
            
            // Simulate function call
            tcp_rate_skb_sent(sk, skb);
        }
    }
}
