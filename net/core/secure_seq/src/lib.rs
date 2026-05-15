//! This module provides FFI-compatible Rust bindings for secure sequence generation
//! in the Linux kernel. It implements TCPv6, IPv6, TCP, and DCCP sequence number
//! generation using SIPHASH with secret keys.
//!
//! The implementation maintains ABI compatibility with the original C code and
//! follows strict FFI conventions for kernel module integration.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(clang::missing_docs_in_private_items)]

use core::ptr;
use core::ffi::c_void;

// Constants from Linux kernel
pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;

// Type definitions
#[repr(C)]
pub struct in6_addr {
    s6_addr: [u8; 16],
}

#[repr(C)]
pub struct siphash_key_t {
    key0: u64,
    key1: u64,
}

// Extern declarations for kernel functions
extern "C" {
    fn net_get_random_once(ptr: *mut c_void, len: u64);
    fn ktime_get_real_ns() -> u64;
    fn net_ipv4_tcp_timestamps(net: *const c_void) -> c_int;
    fn siphash(data: *const c_void, len: usize, key: *const siphash_key_t) -> u32;
    fn siphash_2u32(a: u32, b: u32, key: *const siphash_key_t) -> u32;
    fn siphash_3u32(a: u32, b: u32, c: u32, key: *const siphash_key_t) -> u32;
}

// Global secrets
static mut net_secret: siphash_key_t = siphash_key_t { key0: 0, key1: 0 };
static mut ts_secret: siphash_key_t = siphash_key_t { key0: 0, key1: 0 };

// Helper functions
fn is_tcp_timestamps_enabled(net: *const c_void) -> bool {
    unsafe { net_ipv4_tcp_timestamps(net) == 1 }
}

fn net_secret_init() {
    unsafe {
        net_get_random_once(&mut net_secret as *mut _ as *mut c_void, core::mem::size_of::<siphash_key_t>() as u64);
    }
}

fn ts_secret_init() {
    unsafe {
        net_get_random_once(&mut ts_secret as *mut _ as *mut c_void, core::mem::size_of::<siphash_key_t>() as u64);
    }
}

fn seq_scale(seq: u32) -> u32 {
    seq + (unsafe { ktime_get_real_ns() } >> 6) as u32
}

// Structs for SIPHASH input alignment
#[repr(C, align(16))]
struct CombinedTcpv6 {
    saddr: in6_addr,
    daddr: in6_addr,
}

#[repr(C, align(16))]
struct CombinedTcpv6Seq {
    saddr: in6_addr,
    daddr: in6_addr,
    sport: u16,
    dport: u16,
}

#[repr(C, align(16))]
struct CombinedIpv6Ephemeral {
    saddr: in6_addr,
    daddr: in6_addr,
    dport: u16,
}

#[repr(C, align(16))]
struct CombinedDccp {
    saddr: in6_addr,
    daddr: in6_addr,
    sport: u16,
    dport: u16,
}

// Exported functions
#[no_mangle]
pub unsafe extern "C" fn secure_tcpv6_ts_off(
    net: *const c_void,
    saddr: *const u32,
    daddr: *const u32,
) -> u32 {
    if !is_tcp_timestamps_enabled(net) {
        return 0;
    }

    ts_secret_init();

    // SAFETY: Caller guarantees saddr and daddr are valid pointers to in6_addr data
    let saddr_in6 = &*(saddr as *const in6_addr);
    let daddr_in6 = &*(daddr as *const in6_addr);

    let combined = CombinedTcpv6 {
        saddr: *saddr_in6,
        daddr: *daddr_in6,
    };

    let len = core::mem::size_of_val(&combined);
    siphash(&combined as *const _ as *const c_void, len, &ts_secret as *const _)
}

#[no_mangle]
pub unsafe extern "C" fn secure_tcpv6_seq(
    saddr: *const u32,
    daddr: *const u32,
    sport: u16,
    dport: u16,
) -> u32 {
    // SAFETY: Caller guarantees saddr and daddr are valid pointers to in6_addr data
    let saddr_in6 = &*(saddr as *const in6_addr);
    let daddr_in6 = &*(daddr as *const in6_addr);

    let combined = CombinedTcpv6Seq {
        saddr: *saddr_in6,
        daddr: *daddr_in6,
        sport,
        dport,
    };

    net_secret_init();
    let len = core::mem::size_of_val(&combined);
    let hash = siphash(&combined as *const _ as *const c_void, len, &net_secret as *const _);
    seq_scale(hash)
}

#[no_mangle]
pub unsafe extern "C" fn secure_ipv6_port_ephemeral(
    saddr: *const u32,
    daddr: *const u32,
    dport: u16,
) -> u32 {
    // SAFETY: Caller guarantees saddr and daddr are valid pointers to in6_addr data
    let saddr_in6 = &*(saddr as *const in6_addr);
    let daddr_in6 = &*(daddr as *const in6_addr);

    let combined = CombinedIpv6Ephemeral {
        saddr: *saddr_in6,
        daddr: *daddr_in6,
        dport,
    };

    net_secret_init();
    let len = core::mem::size_of_val(&combined);
    siphash(&combined as *const _ as *const c_void, len, &net_secret as *const _)
}

#[no_mangle]
pub unsafe extern "C" fn secure_tcp_ts_off(
    net: *const c_void,
    saddr: u32,
    daddr: u32,
) -> u32 {
    if !is_tcp_timestamps_enabled(net) {
        return 0;
    }

    ts_secret_init();
    siphash_2u32(saddr, daddr, &ts_secret as *const _)
}

#[no_mangle]
pub unsafe extern "C" fn secure_tcp_seq(
    saddr: u32,
    daddr: u32,
    sport: u16,
    dport: u16,
) -> u32 {
    let combined = (sport as u32) << 16 | (dport as u32);
    net_secret_init();
    let hash = siphash_3u32(saddr, daddr, combined, &net_secret as *const _);
    seq_scale(hash)
}

#[no_mangle]
pub unsafe extern "C" fn secure_ipv4_port_ephemeral(
    saddr: u32,
    daddr: u32,
    dport: u16,
) -> u32 {
    net_secret_init();
    siphash_3u32(saddr, daddr, dport as u32, &net_secret as *const _)
}

#[no_mangle]
pub unsafe extern "C" fn secure_dccp_sequence_number(
    saddr: u32,
    daddr: u32,
    sport: u16,
    dport: u16,
) -> u64 {
    let combined = (sport as u32) << 16 | (dport as u32);
    net_secret_init();
    let seq = siphash_3u32(saddr, daddr, combined, &net_secret as *const _);
    (seq as u64) + unsafe { ktime_get_real_ns() } & ((1u64 << 48) - 1)
}

#[no_mangle]
pub unsafe extern "C" fn secure_dccpv6_sequence_number(
    saddr: *const u32,
    daddr: *const u32,
    sport: u16,
    dport: u16,
) -> u64 {
    // SAFETY: Caller guarantees saddr and daddr are valid pointers to in6_addr data
    let saddr_in6 = &*(saddr as *const in6_addr);
    let daddr_in6 = &*(daddr as *const in6_addr);

    let combined = CombinedDccp {
        saddr: *saddr_in6,
        daddr: *daddr_in6,
        sport,
        dport,
    };

    net_secret_init();
    let len = core::mem::size_of_val(&combined);
    let seq = siphash(&combined as *const _ as *const c_void, len, &net_secret as *const _);
    (seq as u64) + unsafe { ktime_get_real_ns() } & ((1u64 << 48) - 1)
}

// Tests (conditional compilation)
#[cfg(test)]
mod tests {
    #[test]
    fn test_alignment() {
        use super::*;
        assert_eq!(core::mem::align_of::<CombinedTcpv6>(), 16);
        assert_eq!(core::mem::align_of::<CombinedTcpv6Seq>(), 16);
        assert_eq!(core::mem::align_of::<CombinedIpv6Ephemeral>(), 16);
        assert_eq!(core::mem::align_of::<CombinedDccp>(), 16);
    }
}
