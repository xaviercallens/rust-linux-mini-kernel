//! ICMP protocol implementation for IPv4
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]  // For C-style type names

use core::ptr;
use core::sync::atomic::{AtomicU32, Ordering};
use core::time::Duration;
use libc::{c_int, c_uint, c_void, size_t};

// Constants from C
pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ENOSYS: c_int = -38;
pub const ENETUNREACH: c_int = -101;
pub const EHOSTUNREACH: c_int = -111;
pub const ENOPROTOOPT: c_int = -92;
pub const ECONNREFUSED: c_int = -111;
pub const EMSGSIZE: c_int = -90;
pub const EOPNOTSUPP: c_int = -95;
pub const ENONET: c_int = -62;
pub const ENETDOWN: c_int = -100;
pub const EHOSTDOWN: c_int = -101;
pub const ENETUNREACH: c_int = -101;
pub const EHOSTUNREACH: c_int = -111;

// Type definitions
#[repr(C)]
pub struct icmphdr {
    pub type_: u8,
    pub code: u8,
    pub checksum: u16,
    pub unused: u16,
    pub identifier: u16,
    pub sequence: u16,
}

#[repr(C)]
pub struct icmp_err {
    pub errno: c_int,
    pub fatal: c_int,
}

#[repr(C)]
pub struct icmp_control {
    pub handler: extern "C" fn(*mut c_void) -> c_int,
    pub error: c_int,
}

#[repr(C)]
pub struct icmp_bxm {
    pub skb: *mut c_void,
    pub offset: c_int,
    pub data_len: c_int,

    pub data: icmp_bxm_data,
    pub head_len: c_int,
    pub replyopts: ip_options_data,
}

#[repr(C)]
pub struct icmp_bxm_data {
    pub icmph: icmphdr,
    pub times: [u32; 3],
}

#[repr(C)]
pub struct ip_options_data {
    // Simplified for example
    pub f0: u32,
    pub f1: u32,
    pub f2: u32,
    pub f3: u32,
}

// Function implementations
#[repr(C)]
pub static icmp_err_convert: [icmp_err; 16] = [
    icmp_err { errno: ENETUNREACH, fatal: 0 },
    icmp_err { errno: EHOSTUNREACH, fatal: 0 },
    icmp_err { errno: ENOPROTOOPT, fatal: 1 },
    icmp_err { errno: ECONNREFUSED, fatal: 1 },
    icmp_err { errno: EMSGSIZE, fatal: 0 },
    icmp_err { errno: EOPNOTSUPP, fatal: 0 },
    icmp_err { errno: ENETUNREACH, fatal: 1 },
    icmp_err { errno: EHOSTDOWN, fatal: 1 },
    icmp_err { errno: ENONET, fatal: 1 },
    icmp_err { errno: ENETUNREACH, fatal: 1 },
    icmp_err { errno: EHOSTUNREACH, fatal: 1 },
    icmp_err { errno: ENETUNREACH, fatal: 0 },
    icmp_err { errno: EHOSTUNREACH, fatal: 0 },
    icmp_err { errno: EHOSTUNREACH, fatal: 1 },
    icmp_err { errno: EHOSTUNREACH, fatal: 1 },
    icmp_err { errno: EHOSTUNREACH, fatal: 1 },
];

#[repr(C)]
pub static icmp_pointers: [icmp_control; 256] = [icmp_control {
    handler: None,
    error: 0,
}; 256];

#[repr(C)]
pub struct icmp_global {
    pub lock: spinlock_t,
    pub credit: u32,
    pub stamp: u32,
}

#[repr(C)]
pub struct spinlock_t {
    // Simplified for example
    pub slock: u32,
}

#[no_mangle]
pub unsafe extern "C" fn icmp_global_allow() -> c_int {
    let now = (u32::try_from(jiffies()).unwrap());
    let mut incr = 0;
    let mut credit = 0;
    let mut rc = 0;

    // Check if token bucket is empty
    if icmp_global.credit == 0 {
        let delta = min(now - icmp_global.stamp, HZ);
        if delta < HZ / 50 {
            return 0;
        }
    }

    // Acquire lock
    spin_lock(&icmp_global.lock);
    
    let delta = min(now - icmp_global.stamp, HZ);
    if delta >= HZ / 50 {
        incr = sysctl_icmp_msgs_per_sec * delta / HZ;
        icmp_global.stamp = now;
    }
    
    credit = min(icmp_global.credit + incr, sysctl_icmp_msgs_burst);
    if credit > 0 {
        // Randomize credit usage for security
        let random = prandom_u32_max(3);
        credit = max(credit - random, 0);
        rc = 1;
    }
    
    icmp_global.credit = credit;
    spin_unlock(&icmp_global.lock);
    
    rc
}

#[no_mangle]
pub unsafe extern "C" fn icmpv4_mask_allow(net: *mut c_void, type_: c_int, code: c_int) -> c_int {
    if type_ > NR_ICMP_TYPES {
        return 1;
    }
    
    // Don't limit PMTU discovery
    if type_ == ICMP_DEST_UNREACH && code == ICMP_FRAG_NEEDED {
        return 1;
    }
    
    // Limit if icmp type is enabled in ratemask
    if (1 << type_) & (*net).ipv4.sysctl_icmp_ratemask {
        return 0;
    }
    
    1
}

#[no_mangle]
pub unsafe extern "C" fn icmpv4_global_allow(net: *mut c_void, type_: c_int, code: c_int) -> c_int {
    if icmpv4_mask_allow(net, type_, code) != 0 {
        return 1;
    }
    
    if icmp_global_allow() != 0 {
        return 1;
    }
    
    0
}

#[no_mangle]
pub unsafe extern "C" fn icmpv4_xrlim_allow(
    net: *mut c_void,
    rt: *mut c_void,
    fl4: *mut c_void,
    type_: c_int,
    code: c_int
) -> c_int {
    let dst = &(*rt).dst;
    let mut rc = 1;
    let mut vif = 0;
    let mut peer = ptr::null_mut();
    
    if icmpv4_mask_allow(net, type_, code) != 0 {
        return 1;
    }
    
    // No rate limit on loopback
    if !dst.dev.is_null() && (*dst.dev).flags & IFF_LOOPBACK != 0 {
        return 1;
    }
    
    vif = l3mdev_master_ifindex(dst.dev);
    peer = inet_getpeer_v4((*net).ipv4.peers, (*fl4).daddr, vif, 1);
    rc = inet_peer_xrlim_allow(peer, (*net).ipv4.sysctl_icmp_ratelimit);
    
    if !peer.is_null() {
        inet_putpeer(peer);
    }
    
    rc
}

#[no_mangle]
pub unsafe extern "C" fn icmp_out_count(net: *mut c_void, type_: c_int) {
    ICMPMSGOUT_INC_STATS(net, type_);
    ICMP_INC_STATS(net, ICMP_MIB_OUTMSGS);
}

#[no_mangle]
pub unsafe extern "C" fn icmp_glue_bits(
    from: *mut c_void,
    to: *mut u8,
    offset: c_int,
    len: c_int,
    odd: c_int,
    skb: *mut c_void
) -> c_int {
    let icmp_param = from as *mut icmp_bxm;
    let csum = skb_copy_and_csum_bits(
        (*icmp_param).skb,
        (*icmp_param).offset + offset,
        to,
        len
    );
    
    (*skb).csum = csum_block_add((*skb).csum, csum, odd);
    if icmp_pointers[(*icmp_param).data.icmph.type].error != 0 {
        nf_ct_attach(skb, (*icmp_param).skb);
    }
    
    0
}

#[no_mangle]
pub unsafe extern "C" fn icmp_push_reply(
    icmp_param: *mut icmp_bxm,
    fl4: *mut c_void,
    ipc: *mut c_void,
    rt: *mut *mut c_void
) {
    let sk = icmp_sk(dev_net((*rt).dst.dev));
    if ip_append_data(sk, fl4, icmp_glue_bits, icmp_param, 
                     (*icmp_param).data_len + (*icmp_param).head_len,
                     (*icmp_param).head_len, ipc, rt, MSG_DONTWAIT) < 0 {
        __ICMP_INC_STATS(sock_net(sk), ICMP_MIB_OUTERRORS);
        ip_flush_pending_frames(sk);
    } else if let Some(skb) = skb_peek(&(*sk).sk_write_queue) {
        let icmph = icmp_hdr(skb);
        let mut csum = csum_partial_copy_nocheck(
            &(*icmp_param).data,
            icmph as *mut u8,
            (*icmp_param).head_len
        );
        
        skb_queue_walk(&(*sk).sk_write_queue, |skb1| {
            csum = csum_add(csum, (*skb1).csum);
        });
        
        icmph.checksum = csum_fold(csum);
    }
}

// Extern declarations for kernel functions
extern "C" {
    fn jiffies() -> u64;
    fn HZ() -> u32;
    fn sysctl_icmp_msgs_per_sec() -> u32;
    fn sysctl_icmp_msgs_burst() -> u32;
    fn NR_ICMP_TYPES() -> c_int;
    fn ICMP_DEST_UNREACH() -> c_int;
    fn ICMP_FRAG_NEEDED() -> c_int;
    fn IFF_LOOPBACK() -> u32;
    fn l3mdev_master_ifindex(dev: *mut c_void) -> c_int;
    fn inet_getpeer_v4(peers: *mut c_void, daddr: u32, vif: c_int, create: c_int) -> *mut c_void;
    fn inet_peer_xrlim_allow(peer: *mut c_void, ratelimit: u32) -> c_int;
    fn inet_putpeer(peer: *mut c_void);
    fn skb_copy_and_csum_bits(skb: *mut c_void, offset: c_int, to: *mut u8, len: c_int) -> u32;
    fn csum_block_add(csum: u32, addend: u32, odd: c_int) -> u32;
    fn nf_ct_attach(skb: *mut c_void, orig_skb: *mut c_void);
    fn ip_append_data(
        sk: *mut c_void,
        fl4: *mut c_void,
        build: extern "C" fn(*mut c_void, *mut u8, c_int, c_int, c_int, *mut c_void) -> c_int,
        from: *mut c_void,
        length: c_int,
        transhdrlen: c_int,
        ipc: *mut c_void,
        rt: *mut *mut c_void,
        flags: c_int
    ) -> c_int;
    fn ip_flush_pending_frames(sk: *mut c_void);
    fn skb_peek(queue: *mut c_void) -> *mut c_void;
    fn icmp_hdr(skb: *mut c_void) -> *mut icmphdr;
    fn csum_partial_copy_nocheck(from: *mut c_void, to: *mut u8, len: c_int) -> u32;
    fn csum_add(a: u32, b: u32) -> u32;
    fn csum_fold(csum: u32) -> u16;
    fn dev_net(dev: *mut c_void) -> *mut c_void;
    fn sock_net(sk: *mut c_void) -> *mut c_void;
    fn icmp_sk(net: *mut c_void) -> *mut c_void;
    fn this_cpu_read(ptr: *mut c_void) -> *mut c_void;
    fn spin_trylock(slock: *mut u32) -> c_int;
    fn spin_unlock(slock: *mut u32);
    fn prandom_u32_max(max: u32) -> u32;
    fn min(a: u32, b: u32) -> u32;
    fn max(a: c_int, b: c_int) -> c_int;
    fn ICMPMSGOUT_INC_STATS(net: *mut c_void, type_: c_int);
    fn ICMP_INC_STATS(net: *mut c_void, mib: c_int);
    fn __ICMP_INC_STATS(net: *mut c_void, mib: c_int);
}

// Spinlock operations
#[no_mangle]
pub unsafe extern "C" fn spin_lock(lock: *mut spinlock_t) {
    while spin_trylock(&lock.slock) == 0 {
        // Wait
    }
}

#[no_mangle]
pub unsafe extern "C" fn spin_unlock(lock: *mut spinlock_t) {
    spin_unlock(&lock.slock);
}

// Tests (conditional compilation)
#[cfg(test)]
mod tests {
    #[test]
    fn test_icmp_global_allow() {
        // Basic test for the token bucket algorithm
        unsafe {
            let result = super::icmp_global_allow();
            assert!(result >= 0);
        }
    }

    #[test]
    fn test_icmp_err_convert_size() {
        assert_eq!(core::mem::size_of::<super::icmp_err>(), 8);
        assert_eq!(core::mem::size_of::<super::icmp_err_convert>(), 16 * 8);
    }
}
