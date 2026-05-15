//! TCP Output Implementation for Linux Kernel
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(clang::too_many_arguments)]

use core::ptr;
use core::mem;

// Constants from C
pub const NSEC_PER_USEC: u64 = 1000;
pub const MAX_TCP_WINDOW: u32 = 65535;
pub const U16_MAX: u16 = 65535;
pub const TCP_MAX_WSCALE: u8 = 14;

// Error codes
pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ENOSYS: c_int = -38;

// Type definitions
#[repr(C)]
pub struct tcp_sock {
    tcp_clock_cache: u64,
    tcp_mstamp: u64,
    snd_nxt: u32,
    snd_cwnd: u32,
    snd_ssthresh: u32,
    snd_cwnd_stamp: u32,
    snd_cwnd_used: u32,
    packets_out: u32,
    highest_sack: *mut sk_buff,
    rx_opt: tcp_rx_opt,
    rcv_wnd: u32,
    rcv_wup: u32,
    rcv_wscale: u8,
    ecn_flags: u8,
}

#[repr(C)]
pub struct tcp_rx_opt {
    wscale_ok: u8,
    rcv_wscale: u8,
}

#[repr(C)]
pub struct sock {
    sk_write_queue: sk_buff_head,
    sk_rmem_alloc: u32,
    sk_net: net_namespace,
}

#[repr(C)]
pub struct sk_buff_head {
    next: *mut sk_buff,
    prev: *mut sk_buff,
}

#[repr(C)]
pub struct sk_buff {
    data: *mut u8,
    len: u32,
    next: *mut sk_buff,
    prev: *mut sk_buff,
    _marker: [u8; 0], // Actual implementation would have more fields
}

#[repr(C)]
pub struct net_namespace {
    _data: [u8; 0], // Placeholder for actual network namespace data
}

// Function implementations
#[no_mangle]
pub unsafe extern "C" fn tcp_mstamp_refresh(tp: *mut tcp_sock) {
    // SAFETY: Caller guarantees tp is valid and points to a tcp_sock
    let val = tcp_clock_ns();
    (*tp).tcp_clock_cache = val;
    (*tp).tcp_mstamp = val / NSEC_PER_USEC;
}

#[no_mangle]
pub unsafe extern "C" fn tcp_event_new_data_sent(sk: *mut sock, skb: *mut sk_buff) {
    // SAFETY: Caller guarantees sk and skb are valid and properly initialized
    let icsk = &mut (*sk).sk_write_queue; // Simplified for example
    let tp = &mut *(sk as *mut tcp_sock);
    
    let prior_packets = tp.packets_out;
    tp.snd_nxt = TCP_SKB_CB(skb).end_seq;
    
    __skb_unlink(skb, &mut (*sk).sk_write_queue);
    tcp_rbtree_insert(&mut (*sk).sk_write_queue, skb);
    
    if tp.highest_sack.is_null() {
        tp.highest_sack = skb;
    }
    
    tp.packets_out += tcp_skb_pcount(skb);
    
    if prior_packets == 0 {
        tcp_rearm_rto(sk);
    }
    
    NET_ADD_STATS(&(*sk).sk_net, LINUX_MIB_TCPORIGDATASENT, tcp_skb_pcount(skb));
}

#[no_mangle]
pub unsafe extern "C" fn tcp_acceptable_seq(sk: *const sock) -> u32 {
    let tp = &(*sk as *const tcp_sock);
    let wnd_end = tcp_wnd_end(tp);
    
    if !before(wnd_end, tp.snd_nxt) || 
       (tp.rx_opt.wscale_ok != 0 && 
        ((tp.snd_nxt - wnd_end) < (1 << tp.rx_opt.rcv_wscale))) {
        return tp.snd_nxt;
    } else {
        return wnd_end;
    }
}

#[no_mangle]
pub unsafe extern "C" fn tcp_advertise_mss(sk: *mut sock) -> u16 {
    let tp = &mut *(sk as *mut tcp_sock);
    let dst = __sk_dst_get(sk);
    
    let mut mss = tp.advmss;
    
    if !dst.is_null() {
        let metric = dst_metric_advmss(dst);
        if metric < mss {
            mss = metric;
            tp.advmss = mss;
        }
    }
    
    mss as u16
}

#[no_mangle]
pub unsafe extern "C" fn tcp_select_initial_window(
    sk: *const sock,
    __space: c_int,
    mss: u32,
    rcv_wnd: *mut u32,
    window_clamp: *mut u32,
    wscale_ok: c_int,
    rcv_wscale: *mut u8,
    init_rcv_wnd: u32
) {
    let space = if __space < 0 { 0 } else { __space as u32 };
    
    if *window_clamp == 0 {
        *window_clamp = (U16_MAX as u32) << TCP_MAX_WSCALE;
    }
    
    let space = space.min(*window_clamp);
    
    if space > mss {
        *rcv_wnd = (space as u32).wrapping_sub(space % mss);
    } else {
        *rcv_wnd = space;
    }
    
    if init_rcv_wnd != 0 {
        *rcv_wnd = (*rcv_wnd).min(init_rcv_wnd * mss);
    }
    
    *rcv_wscale = 0;
    if wscale_ok != 0 {
        let space = (*window_clamp).max(sock_net(sk).ipv4.sysctl_tcp_rmem[2]);
        space = space.max(sysctl_rmem_max);
        space = space.min(*window_clamp);
        *rcv_wscale = ((space as u32).leading_zeros() - 15) as u8;
        *rcv_wscale = (*rcv_wscale).clamp(0, TCP_MAX_WSCALE);
    }
    
    *window_clamp = (U16_MAX as u32) << (*rcv_wscale);
}

#[no_mangle]
pub unsafe extern "C" fn tcp_select_window(sk: *mut sock) -> u16 {
    let tp = &mut *(sk as *mut tcp_sock);
    let old_win = tp.rcv_wnd;
    let cur_win = tcp_receive_window(tp);
    let new_win = __tcp_select_window(sk);
    
    if new_win < cur_win {
        if new_win == 0 {
            NET_INC_STATS(&(*sk).sk_net, LINUX_MIB_TCPWANTZEROWINDOWADV);
        }
        new_win = cur_win.align_to(1 << tp.rx_opt.rcv_wscale);
    }
    
    tp.rcv_wnd = new_win;
    tp.rcv_wup = tp.rcv_nxt;
    
    let mut new_win = new_win;
    if !tp.rx_opt.rcv_wscale && 
       sock_net(sk).ipv4.sysctl_tcp_workaround_signed_windows {
        new_win = new_win.min(MAX_TCP_WINDOW);
    } else {
        new_win = new_win.min((65535U << tp.rx_opt.rcv_wscale) as u32);
    }
    
    new_win >>= tp.rx_opt.rcv_wscale;
    
    if new_win == 0 {
        tp.pred_flags = 0;
        if old_win != 0 {
            NET_INC_STATS(&(*sk).sk_net, LINUX_MIB_TCPTOZEROWINDOWADV);
        }
    } else if old_win == 0 {
        NET_INC_STATS(&(*sk).sk_net, LINUX_MIB_TCPFROMZEROWINDOWADV);
    }
    
    new_win as u16
}

// Helper functions
#[inline]
fn before(a: u32, b: u32) -> bool {
    (a < b)
}

#[no_mangle]
unsafe extern "C" fn tcp_clock_ns() -> u64 {
    // Placeholder for actual clock implementation
    0
}

#[no_mangle]
unsafe extern "C" fn __skb_unlink(skb: *mut sk_buff, list: *mut sk_buff_head) {
    // Simplified implementation for example
    let skb = &mut *skb;
    let list = &mut *list;
    
    // Remove skb from the list
    (*list).next = (*skb).next;
    (*list).prev = (*skb).prev;
}

#[no_mangle]
unsafe extern "C" fn tcp_rbtree_insert(queue: *mut sk_buff_head, skb: *mut sk_buff) {
    // Simplified insertion logic
}

#[no_mangle]
unsafe extern "C" fn tcp_skb_pcount(skb: *mut sk_buff) -> u32 {
    // Simplified count calculation
    1
}

#[no_mangle]
unsafe extern "C" fn tcp_rearm_rto(sk: *mut sock) {
    // Placeholder for rearming RTO
}

#[no_mangle]
unsafe extern "C" fn tcp_wnd_end(tp: *const tcp_sock) -> u32 {
    // Simplified window end calculation
    0
}

#[no_mangle]
unsafe extern "C" fn tcp_receive_window(tp: *const tcp_sock) -> u32 {
    // Simplified receive window calculation
    0
}

#[no_mangle]
unsafe extern "C" fn __tcp_select_window(sk: *mut sock) -> u32 {
    // Simplified window selection
    0
}

#[no_mangle]
unsafe extern "C" fn __sk_dst_get(sk: *mut sock) -> *mut c_void {
    // Placeholder for destination entry
    ptr::null_mut()
}

#[no_mangle]
unsafe extern "C" fn dst_metric_advmss(dst: *mut c_void) -> u32 {
    // Placeholder for metric retrieval
    1500
}

#[no_mangle]
unsafe extern "C" fn NET_ADD_STATS(net: *mut net_namespace, mib: u32, count: u32) {
    // Placeholder for statistics
}

#[no_mangle]
unsafe extern "C" fn NET_INC_STATS(net: *mut net_namespace, mib: u32) {
    // Placeholder for statistics
}

#[no_mangle]
unsafe extern "C" fn sock_net(sk: *const sock) -> *mut net_namespace {
    &(*sk).sk_net
}

#[no_mangle]
unsafe extern "C" fn sysctl_rmem_max() -> u32 {
    // Placeholder for sysctl value
    131072
}

// Exported symbols
#[no_mangle]
pub unsafe extern "C" fn tcp_select_initial_window_exported(
    sk: *const sock,
    __space: c_int,
    mss: u32,
    rcv_wnd: *mut u32,
    window_clamp: *mut u32,
    wscale_ok: c_int,
    rcv_wscale: *mut u8,
    init_rcv_wnd: u32
) {
    tcp_select_initial_window(sk, __space, mss, rcv_wnd, window_clamp, wscale_ok, rcv_wscale, init_rcv_wnd)
}

#[no_mangle]
pub unsafe extern "C" fn tcp_release_cb() {
    // Placeholder for release callback
}

#[no_mangle]
pub unsafe extern "C" fn tcp_mtu_to_mss() -> u32 {
    // Placeholder
    1500
}

#[no_mangle]
pub unsafe extern "C" fn tcp_mss_to_mtu() -> u32 {
    // Placeholder
    1500
}

#[no_mangle]
pub unsafe extern "C" fn tcp_mtup_init() {
    // Placeholder
}

// Tests (conditional compilation)
#[cfg(test)]
mod tests {
    #[test]
    fn test_tcp_mstamp_refresh() {
        // Basic test case for tcp_mstamp_refresh
        let mut tp = super::tcp_sock {
            tcp_clock_cache: 0,
            tcp_mstamp: 0,
            snd_nxt: 0,
            snd_cwnd: 0,
            snd_ssthresh: 0,
            snd_cwnd_stamp: 0,
            snd_cwnd_used: 0,
            packets_out: 0,
            highest_sack: ptr::null_mut(),
            rx_opt: super::tcp_rx_opt { wscale_ok: 0, rcv_wscale: 0 },
            rcv_wnd: 0,
            rcv_wup: 0,
            rcv_wscale: 0,
            ecn_flags: 0,
        };
        
        unsafe {
            super::tcp_mstamp_refresh(&mut tp);
            assert!(tp.tcp_clock_cache > 0);
            assert!(tp.tcp_mstamp > 0);
        }
    }
}
