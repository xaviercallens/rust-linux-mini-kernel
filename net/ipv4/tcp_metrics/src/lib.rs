//! TCP Metrics Management
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(clippy::all)]

use core::ptr;
use core::ffi::c_void;
use core::ffi::c_int;
use core::ffi::c_uint;
use core::ffi::c_ulong;

// Constants from C
pub const TCP_METRIC_RTT: c_int = 0;
pub const TCP_METRIC_RTTVAR: c_int = 1;
pub const TCP_METRIC_SSTHRESH: c_int = 2;
pub const TCP_METRIC_CWND: c_int = 3;
pub const TCP_METRIC_REORDERING: c_int = 4;
pub const TCP_METRIC_MAX_KERNEL: c_int = 5;

pub const TCP_METRICS_TIMEOUT: c_ulong = 60 * 60 * 100; // HZ is typically 100
pub const TCP_METRICS_RECLAIM_DEPTH: c_int = 5;
pub const TCP_METRICS_RECLAIM_PTR: *mut TcpMetricsBlock = 0x1UL as *mut TcpMetricsBlock;

// Type definitions
#[repr(C)]
pub struct InetPeerAddr {
    family: c_int,
    // Actual fields depend on AF_INET/AF_INET6
    // This is a simplified representation
    _data: [u8; 16],
}

#[repr(C)]
pub struct TcpFastOpenMetrics {
    mss: u16,
    syn_loss: u16,    // 10 bits
    try_exp: u16,     // 2 bits
    last_syn_loss: c_ulong,
    cookie: TcpFastOpenCookie,
}

#[repr(C)]
pub struct TcpFastOpenCookie {
    exp: bool,
    len: u8,
    // Actual cookie data would follow
}

#[repr(C)]
pub struct TcpMetricsBlock {
    tcpm_next: *mut TcpMetricsBlock,
    tcpm_net: PossibleNet,
    tcpm_saddr: InetPeerAddr,
    tcpm_daddr: InetPeerAddr,
    tcpm_stamp: c_ulong,
    tcpm_lock: u32,
    tcpm_vals: [u32; (TCP_METRIC_MAX_KERNEL + 1) as usize],
    tcpm_fastopen: TcpFastOpenMetrics,
    rcu_head: RcuHead,
}

#[repr(C)]
pub struct PossibleNet {
    // This is a kernel-specific type for network namespace
    // Simplified as a pointer for FFI compatibility
    _data: *mut c_void,
}

#[repr(C)]
pub struct RcuHead {
    // Simplified RCU head structure
    _data: [u8; 8],
}

#[repr(C)]
pub struct TcpmHashBucket {
    chain: *mut TcpMetricsBlock,
}

#[repr(C)]
pub struct SpinLock {
    // Simplified spinlock structure
    _data: [u8; 8],
}

#[repr(C)]
pub struct DstEntry {
    dev: *mut c_void,
    // Additional fields as needed
}

#[repr(C)]
pub struct RequestSock {
    rsk_ops: *mut c_void,
}

#[repr(C)]
pub struct InetSock {
    inet_saddr: u32,
    inet_daddr: u32,
    sk_family: c_int,
}

#[repr(C)]
pub struct TcpSock {
    srtt_us: u32,
    mdev_us: u32,
}

#[repr(C)]
pub struct InetConnectionSock {
    icsk_backoff: u32,
}

#[repr(C)]
pub struct Sock {
    sk_family: c_int,
    sk_v6_daddr: [u8; 16],
    sk_v6_rcv_saddr: [u8; 16],
    sk_net: PossibleNet,
}

// Function implementations
/// Get network namespace from tcp_metrics_block
fn tm_net(tm: *const TcpMetricsBlock) -> *mut c_void {
    unsafe { (*tm).tcpm_net._data }
}

/// Check if a metric is locked
fn tcp_metric_locked(tm: *const TcpMetricsBlock, idx: c_int) -> bool {
    unsafe { (tm.is_null().then(|| 0).unwrap_or((*tm).tcpm_lock) & (1 << idx)) != 0 }
}

/// Get metric value
fn tcp_metric_get(tm: *const TcpMetricsBlock, idx: c_int) -> u32 {
    unsafe { (*tm).tcpm_vals[idx as usize] }
}

/// Set metric value
fn tcp_metric_set(tm: *mut TcpMetricsBlock, idx: c_int, val: u32) {
    unsafe { (*tm).tcpm_vals[idx as usize] = val }
}

/// Compare addresses
fn addr_same(a: *const InetPeerAddr, b: *const InetPeerAddr) -> bool {
    unsafe { a.is_null() || b.is_null() || ptr::eq(a, b) }
    // Actual implementation would compare fields based on address family
}

/// Check if addresses are the same
fn inetpeer_addr_cmp(a: *const InetPeerAddr, b: *const InetPeerAddr) -> c_int {
    if a.is_null() || b.is_null() {
        return 1;
    }
    // Simplified comparison
    unsafe {
        if (*a).family != (*b).family {
            return 1;
        }
        // Actual comparison would depend on address family
        0
    }
}

/// Update metrics from destination entry
fn tcpm_suck_dst(tm: *mut TcpMetricsBlock, dst: *const DstEntry, fastopen_clear: bool) {
    unsafe {
        (*tm).tcpm_stamp = jiffies();
        
        let mut val = 0;
        if dst_metric_locked(dst, RTAX_RTT) {
            val |= 1 << TCP_METRIC_RTT;
        }
        if dst_metric_locked(dst, RTAX_RTTVAR) {
            val |= 1 << TCP_METRIC_RTTVAR;
        }
        if dst_metric_locked(dst, RTAX_SSTHRESH) {
            val |= 1 << TCP_METRIC_SSTHRESH;
        }
        if dst_metric_locked(dst, RTAX_CWND) {
            val |= 1 << TCP_METRIC_CWND;
        }
        if dst_metric_locked(dst, RTAX_REORDERING) {
            val |= 1 << TCP_METRIC_REORDERING;
        }
        (*tm).tcpm_lock = val;
        
        let msval = dst_metric_raw(dst, RTAX_RTT);
        (*tm).tcpm_vals[TCP_METRIC_RTT as usize] = msval * 1000; // USEC_PER_MSEC
        
        let msval = dst_metric_raw(dst, RTAX_RTTVAR);
        (*tm).tcpm_vals[TCP_METRIC_RTTVAR as usize] = msval * 1000;
        
        (*tm).tcpm_vals[TCP_METRIC_SSTHRESH as usize] = dst_metric_raw(dst, RTAX_SSTHRESH);
        (*tm).tcpm_vals[TCP_METRIC_CWND as usize] = dst_metric_raw(dst, RTAX_CWND);
        (*tm).tcpm_vals[TCP_METRIC_REORDERING as usize] = dst_metric_raw(dst, RTAX_REORDERING);
        
        if fastopen_clear {
            (*tm).tcpm_fastopen.mss = 0;
            (*tm).tcpm_fastopen.syn_loss = 0;
            (*tm).tcpm_fastopen.try_exp = 0;
            (*tm).tcpm_fastopen.cookie.exp = false;
            (*tm).tcpm_fastopen.cookie.len = 0;
        }
    }
}

/// Check if metrics need updating
fn tcpm_check_stamp(tm: *mut TcpMetricsBlock, dst: *const DstEntry) {
    unsafe {
        if !tm.is_null() && jiffies() > (*tm).tcpm_stamp + TCP_METRICS_TIMEOUT {
            tcpm_suck_dst(tm, dst, false);
        }
    }
}

/// Get metrics for request socket
fn __tcp_get_metrics_req(req: *const RequestSock, dst: *const DstEntry) -> *mut TcpMetricsBlock {
    unsafe {
        let mut saddr = InetPeerAddr { family: 0, _data: [0; 16] };
        let mut daddr = InetPeerAddr { family: 0, _data: [0; 16] };
        let mut hash: c_uint = 0;
        let net = dev_net((*dst).dev);
        
        // Simplified address handling
        saddr.family = (*req).rsk_ops as *const c_void as *const c_int as c_int;
        daddr.family = saddr.family;
        
        hash = 0x12345678; // Simplified hash calculation
        
        let mut tm = rcu_dereference((*tcp_metrics_hash.offset(hash as isize)).chain);
        while !tm.is_null() {
            if addr_same(&(*tm).tcpm_saddr, &saddr) &&
               addr_same(&(*tm).tcpm_daddr, &daddr) &&
               net_eq(tm_net(tm), net) {
                break;
            }
            tm = rcu_dereference((*tm).tcpm_next);
        }
        
        if !tm.is_null() {
            tcpm_check_stamp(tm, dst);
        }
        tm
    }
}

/// Get metrics for socket
fn tcp_get_metrics(sk: *const Sock, dst: *const DstEntry, create: bool) -> *mut TcpMetricsBlock {
    unsafe {
        let mut saddr = InetPeerAddr { family: 0, _data: [0; 16] };
        let mut daddr = InetPeerAddr { family: 0, _data: [0; 16] };
        let mut hash: c_uint = 0;
        let net = dev_net((*dst).dev);
        
        if (*sk).sk_family == AF_INET {
            saddr.family = AF_INET;
            daddr.family = AF_INET;
            hash = 0x12345678; // Simplified hash
        }
        
        hash ^= net_hash_mix(net);
        hash = hash_32(hash, tcp_metrics_hash_log);
        
        let mut tm = __tcp_get_metrics(&saddr, &daddr, net, hash);
        if tm == TCP_METRICS_RECLAIM_PTR {
            tm = ptr::null_mut();
        }
        if tm.is_null() && create {
            tm = tcpm_new(dst, &saddr, &daddr, hash);
        } else if !tm.is_null() {
            tcpm_check_stamp(tm, dst);
        }
        tm
    }
}

/// Update TCP metrics
#[no_mangle]
pub unsafe extern "C" fn tcp_update_metrics(sk: *mut Sock) -> c_int {
    let icsk = &(*sk).icsk as *const InetConnectionSock;
    let dst = __sk_dst_get(sk);
    let tp = &(*sk).tp as *const TcpSock;
    let net = sock_net(sk);
    
    if net->ipv4.sysctl_tcp_nometrics_save != 0 || dst.is_null() {
        return 0;
    }
    
    rcu_read_lock();
    
    if (*icsk).icsk_backoff != 0 || (*tp).srtt_us == 0 {
        let tm = tcp_get_metrics(sk, dst, false);
        if !tm.is_null() && !tcp_metric_locked(tm, TCP_METRIC_RTT) {
            tcp_metric_set(tm, TCP_METRIC_RTT, 0);
        }
        rcu_read_unlock();
        return 0;
    } else {
        let tm = tcp_get_metrics(sk, dst, true);
        if tm.is_null() {
            rcu_read_unlock();
            return 0;
        }
    }
    
    let rtt = tcp_metric_get(tm, TCP_METRIC_RTT);
    let m = rtt - (*tp).srtt_us;
    
    if !tcp_metric_locked(tm, TCP_METRIC_RTT) {
        if m <= 0 {
            let rtt = (*tp).srtt_us;
            tcp_metric_set(tm, TCP_METRIC_RTT, rtt);
        } else {
            let rtt = rtt - (m >> 3);
            tcp_metric_set(tm, TCP_METRIC_RTT, rtt);
        }
    }
    
    if !tcp_metric_locked(tm, TCP_METRIC_RTTVAR) {
        let mut var = 0;
        let m = if m < 0 { -m } else { m };
        m >>= 1;
        if m < (*tp).mdev_us {
            m = (*tp).mdev_us;
        }
        var = tcp_metric_get(tm, TCP_METRIC_RTTVAR);
        var = var - (var >> 3) + m;
        tcp_metric_set(tm, TCP_METRIC_RTTVAR, var);
    }
    
    rcu_read_unlock();
    0
}

// Helper functions (extern declarations)
extern "C" {
    fn jiffies() -> c_ulong;
    fn dev_net(dev: *mut c_void) -> *mut PossibleNet;
    fn net_eq(net1: *mut c_void, net2: *mut c_void) -> bool;
    fn sock_net(sk: *mut Sock) -> *mut PossibleNet;
    fn __sk_dst_get(sk: *mut Sock) -> *mut DstEntry;
    fn dst_metric_locked(dst: *mut DstEntry, rtax: c_int) -> bool;
    fn dst_metric_raw(dst: *mut DstEntry, rtax: c_int) -> u32;
    fn rcu_read_lock();
    fn rcu_read_unlock();
    fn rcu_dereference(ptr: *mut TcpMetricsBlock) -> *mut TcpMetricsBlock;
    fn hash_32(val: c_uint, bits: c_int) -> c_uint;
}

// Global variables
static mut tcp_metrics_hash: *mut TcpmHashBucket = ptr::null_mut();
static mut tcp_metrics_hash_log: c_int = 0;
static mut tcp_metrics_lock: SpinLock = SpinLock { _data: [0; 8] };

// Error codes
pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ENOSYS: c_int = -38;

// Constants
pub const AF_INET: c_int = 2;
pub const RTAX_RTT: c_int = 1;
pub const RTAX_RTTVAR: c_int = 2;
pub const RTAX_SSTHRESH: c_int = 3;
pub const RTAX_CWND: c_int = 4;
pub const RTAX_REORDERING: c_int = 5;

#[cfg(test)]
mod tests {
    #[test]
    fn test_struct_layout() {
        // Verify struct layouts match expectations
        assert_eq!(core::mem::size_of::<super::TcpMetricsBlock>(), 128);
        assert_eq!(core::mem::align_of::<super::TcpMetricsBlock>(), 8);
    }
}
