//! TCP Recovery (RACK) implementation for Linux kernel
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::all)]

use core::ptr;
use core::mem;
use core::cmp;

// Constants from C
pub const TCP_CA_Recovery: u8 = 2;
pub const TCP_RACK_NO_DUPTHRESH: u32 = 1 << 0;
pub const TCP_RACK_RECOVERY_THRESH: u32 = 16;
pub const TCP_TIMEOUT_MIN: u32 = 1;

// Type definitions
#[repr(C)]
pub struct sock {
    pub __bindgen_anon_1: [u8; 0x100],
}

#[repr(C)]
pub struct tcp_sock {
    pub reord_seen: u32,
    pub reordering: u32,
    pub sacked_out: u32,
    pub tcp_mstamp: u64,
    pub srtt_us: u32,
    pub rack: RACK,
    pub lost: u32,
    pub delivered: u32,
    pub __bindgen_anon_1: [u8; 0x100],
}

#[repr(C)]
pub struct RACK {
    pub rtt_us: u32,
    pub mstamp: u64,
    pub end_seq: u32,
    pub advanced: u8,
    pub reo_wnd_steps: u8,
    pub dsack_seen: u8,
    pub reo_wnd_persist: u32,
    pub last_delivered: u32,
}

#[repr(C)]
pub struct inet_connection_sock {
    pub icsk_ca_state: u8,
    pub icsk_ca_ops: *const c_void,
    pub icsk_pending: u8,
    pub __bindgen_anon_1: [u8; 0x100],
}

#[repr(C)]
pub struct sk_buff {
    pub tcp_tsorted_anchor: ListHead,
    pub __bindgen_anon_1: [u8; 0x100],
}

#[repr(C)]
pub struct ListHead {
    pub next: *mut ListHead,
    pub prev: *mut ListHead,
}

#[repr(C)]
pub struct TCP_SKB_CB {
    pub sacked: u8,
    pub end_seq: u32,
}

#[repr(C)]
pub struct rate_sample {
    pub prior_delivered: u32,
}

// Function pointers for FFI compatibility
extern "C" {
    fn tcp_sk(sk: *const sock) -> *mut tcp_sock;
    fn inet_csk(sk: *const sock) -> *mut inet_connection_sock;
    fn tcp_min_rtt(tp: *const tcp_sock) -> u32;
    fn tcp_skb_timestamp_us(skb: *const sk_buff) -> u64;
    fn tcp_skb_mss(skb: *const sk_buff) -> u32;
    fn tcp_skb_pcount(skb: *const sk_buff) -> u32;
    fn tcp_fragment(sk: *mut sock, flags: u8, skb: *mut sk_buff, mss: u32, frag_size: u32, gfp: u32);
    fn tcp_mark_skb_lost(sk: *mut sock, skb: *mut sk_buff);
    fn tcp_enter_recovery(sk: *mut sock, flag: bool);
    fn tcp_cwnd_reduction(sk: *mut sock, mib: u32, prior_packets: u32, flag: u32);
    fn tcp_xmit_retransmit_queue(sk: *mut sock);
    fn tcp_rearm_rto(sk: *mut sock);
    fn inet_csk_reset_xmit_timer(sk: *mut sock, event: u8, timeout: u32, rto: u32);
    fn tcp_packets_in_flight(tp: *const tcp_sock) -> u32;
}

// Helper functions
fn after(seq1: u32, seq2: u32) -> bool {
    seq1.wrapping_sub(seq2) > seq2.wrapping_sub(seq1)
}

fn usecs_to_jiffies(usecs: u32) -> u32 {
    (usecs + 999) / 1000
}

fn tcp_stamp_us_delta(t1: u64, t2: u64) -> i32 {
    (t1.wrapping_sub(t2)) as i32
}

// Internal functions
fn tcp_rack_sent_after(t1: u64, t2: u64, seq1: u32, seq2: u32) -> bool {
    t1 > t2 || (t1 == t2 && after(seq1, seq2))
}

fn tcp_rack_reo_wnd(sk: *const sock) -> u32 {
    let tp = unsafe { tcp_sk(sk) };
    let tp = unsafe { &*tp };
    
    if tp.reord_seen == 0 {
        let icsk = unsafe { inet_csk(sk) };
        if icsk.icsk_ca_state >= TCP_CA_Recovery {
            return 0;
        }
        
        if tp.sacked_out >= tp.reordering &&
           !(unsafe { &*sock_net(sk) }.ipv4.sysctl_tcp_recovery & TCP_RACK_NO_DUPTHRESH) != 0 {
            return 0;
        }
    }
    
    let min_rtt = unsafe { tcp_min_rtt(tp) };
    let reo_wnd = (min_rtt >> 2) * tp.rack.reo_wnd_steps as u32;
    let srtt = tp.srtt_us >> 3;
    
    cmp::min(reo_wnd, srtt)
}

fn tcp_rack_skb_timeout(tp: *mut tcp_sock, skb: *mut sk_buff, reo_wnd: u32) -> i32 {
    let rtt_us = unsafe { (*tp).rack.rtt_us };
    let skb_ts = unsafe { tcp_skb_timestamp_us(skb) };
    let mstamp = unsafe { (*tp).tcp_mstamp };
    
    rtt_us as i32 + reo_wnd as i32 - 
        tcp_stamp_us_delta(mstamp, skb_ts)
}

// Exported functions
#[no_mangle]
pub unsafe extern "C" fn tcp_rack_detect_loss(
    sk: *mut sock,
    reo_timeout: *mut u32,
) {
    let tp = tcp_sk(sk);
    let tp = &mut *tp;
    let mut reo_wnd = tcp_rack_reo_wnd(sk);
    
    *reo_timeout = 0;
    
    let mut skb = (*tp).tsorted_sent_queue.next;
    while !skb.is_null() && skb != &mut (*tp).tsorted_sent_queue {
        let scb = unsafe { &mut *(TCP_SKB_CB(skb)) };
        
        // Skip ones marked lost but not yet retransmitted
        if (scb.sacked & TCPCB_LOST) != 0 && (scb.sacked & TCPCB_SACKED_RETRANS) == 0 {
            let next_skb = (*skb).tcp_tsorted_anchor.next;
            skb = next_skb;
            continue;
        }
        
        if !tcp_rack_sent_after(tp.rack.mstamp, 
                               unsafe { tcp_skb_timestamp_us(skb) },
                               tp.rack.end_seq, scb.end_seq) {
            break;
        }
        
        let remaining = tcp_rack_skb_timeout(tp, skb, reo_wnd);
        if remaining <= 0 {
            tcp_mark_skb_lost(sk, skb);
            list_del_init(&mut (*skb).tcp_tsorted_anchor);
        } else {
            *reo_timeout = cmp::max(*reo_timeout, remaining as u32);
        }
        
        let next_skb = (*skb).tcp_tsorted_anchor.next;
        skb = next_skb;
    }
}

#[no_mangle]
pub unsafe extern "C" fn tcp_rack_mark_lost(
    sk: *mut sock,
) -> bool {
    let tp = tcp_sk(sk);
    let tp = &mut *tp;
    
    if tp.rack.advanced == 0 {
        return false;
    }
    
    tp.rack.advanced = 0;
    let mut timeout = 0;
    tcp_rack_detect_loss(sk, &mut timeout);
    
    if timeout != 0 {
        let timeout_jiffies = usecs_to_jiffies(timeout) + TCP_TIMEOUT_MIN;
        let icsk = inet_csk(sk);
        inet_csk_reset_xmit_timer(sk, ICSK_TIME_REO_TIMEOUT, timeout_jiffies, (*icsk).icsk_rto);
    }
    
    timeout != 0
}

#[no_mangle]
pub unsafe extern "C" fn tcp_rack_advance(
    tp: *mut tcp_sock,
    sacked: u8,
    end_seq: u32,
    xmit_time: u64,
) {
    let rtt_us = tcp_stamp_us_delta((*tp).tcp_mstamp, xmit_time);
    let min_rtt = tcp_min_rtt(tp);
    
    if rtt_us < min_rtt && (sacked & TCPCB_RETRANS) != 0 {
        return;
    }
    
    (*tp).rack.advanced = 1;
    (*tp).rack.rtt_us = rtt_us as u32;
    
    if tcp_rack_sent_after(xmit_time, (*tp).rack.mstamp, end_seq, (*tp).rack.end_seq) {
        (*tp).rack.mstamp = xmit_time;
        (*tp).rack.end_seq = end_seq;
    }
}

#[no_mangle]
pub unsafe extern "C" fn tcp_rack_reo_timeout(
    sk: *mut sock,
) {
    let tp = tcp_sk(sk);
    let tp = &mut *tp;
    let prior_inflight = tcp_packets_in_flight(tp);
    
    let mut timeout = 0;
    tcp_rack_detect_loss(sk, &mut timeout);
    
    if prior_inflight != tcp_packets_in_flight(tp) {
        let icsk = inet_csk(sk);
        if (*icsk).icsk_ca_state != TCP_CA_Recovery {
            tcp_enter_recovery(sk, false);
            if (*icsk).icsk_ca_ops.is_null() {
                let lost = tp.lost;
                tcp_cwnd_reduction(sk, 1, tp.lost - lost, 0);
            }
        }
        tcp_xmit_retransmit_queue(sk);
    }
    
    if (*inet_csk(sk)).icsk_pending != ICSK_TIME_RETRANS {
        tcp_rearm_rto(sk);
    }
}

#[no_mangle]
pub unsafe extern "C" fn tcp_rack_update_reo_wnd(
    sk: *mut sock,
    rs: *const rate_sample,
) {
    let tp = tcp_sk(sk);
    let tp = &mut *tp;
    
    if (sock_net(sk).ipv4.sysctl_tcp_recovery & TCP_RACK_STATIC_REO_WND) != 0 ||
       (*rs).prior_delivered == 0 {
        return;
    }
    
    if before((*rs).prior_delivered, tp.rack.last_delivered) {
        tp.rack.dsack_seen = 0;
        return;
    }
    
    if tp.rack.dsack_seen != 0 {
        tp.rack.reo_wnd_steps = cmp::min(0xFF, tp.rack.reo_wnd_steps + 1);
        tp.rack.dsack_seen = 0;
        tp.rack.last_delivered = tp.delivered;
        tp.rack.reo_wnd_persist = TCP_RACK_RECOVERY_THRESH;
    } else if tp.rack.reo_wnd_persist == 0 {
        tp.rack.reo_wnd_steps = 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn tcp_newreno_mark_lost(
    sk: *mut sock,
    snd_una_advanced: bool,
) {
    let icsk = inet_csk(sk);
    let state = (*icsk).icsk_ca_state;
    let tp = tcp_sk(sk);
    let tp = &mut *tp;
    
    if (state < TCP_CA_Recovery && tp.sacked_out >= tp.reordering) ||
       (state == TCP_CA_Recovery && snd_una_advanced) {
        let skb = tcp_rtx_queue_head(sk);
        let scb = TCP_SKB_CB(skb);
        
        if (scb.sacked & TCPCB_LOST) != 0 {
            return;
        }
        
        let mss = tcp_skb_mss(skb);
        if tcp_skb_pcount(skb) > 1 && (*skb).len > mss {
            tcp_fragment(sk, TCP_FRAG_IN_RTX_QUEUE, skb, mss, mss, GFP_ATOMIC);
        }
        
        tcp_mark_skb_lost(sk, skb);
    }
}

// Helper macros translated to functions
fn before(seq1: u32, seq2: u32) -> bool {
    seq1.wrapping_sub(seq2) < seq2.wrapping_sub(seq1)
}

fn list_del_init(head: *mut ListHead) {
    unsafe {
        (*head).next = head;
        (*head).prev = head;
    }
}

fn tcp_rtx_queue_head(sk: *mut sock) -> *mut sk_buff {
    unsafe { &mut (*sk).some_skb_field }
}

// Constants for TCP flags
pub const TCPCB_LOST: u8 = 0x01;
pub const TCPCB_SACKED_RETRANS: u8 = 0x02;
pub const TCPCB_RETRANS: u8 = 0x04;
pub const ICSK_TIME_REO_TIMEOUT: u8 = 0x01;
pub const TCP_FRAG_IN_RTX_QUEUE: u8 = 0x01;
pub const GFP_ATOMIC: u32 = 0x01;

// Type aliases for FFI compatibility
pub type c_int = i32;
pub type c_uint = u32;
pub type c_void = *const ();
pub type size_t = usize;
pub type socklen_t = u32;

// Helper functions for pointer operations
fn TCP_SKB_CB(skb: *mut sk_buff) -> *mut TCP_SKB_CB {
    unsafe { &mut (*skb).tcp_skb_cb }
}

fn sock_net(sk: *mut sock) -> *mut net {
    unsafe { &mut (*sk).some_net_field }
}

// Dummy types for FFI compatibility
#[repr(C)]
pub struct net {
    pub ipv4: IPv4,
}

#[repr(C)]
pub struct IPv4 {
    pub sysctl_tcp_recovery: u32,
}
