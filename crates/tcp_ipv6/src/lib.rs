//! TCP over IPv6 implementation for Linux kernel
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)] // For C-style type names

use kernel_types::*;

// Constants from C
pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ENETUNREACH: c_int = -101;
pub const EAFNOSUPPORT: c_int = -97;
pub const ENOENT: c_int = -2;

// Function implementations

/// Helper returning the ipv6_pinfo from a tcp socket
///
/// # Safety
/// - `sk` must be a valid pointer to sock
/// - Caller must ensure the pointer is valid and properly aligned
#[no_mangle]
pub unsafe extern "C" fn tcp_inet6_sk(sk: *const sock) -> *mut ipv6_pinfo {
    let offset = core::mem::size_of::<sock>() - core::mem::size_of::<ipv6_pinfo>();
    // SAFETY: The offset calculation is valid for the structure layout
    // Caller guarantees sk is valid and properly aligned
    let ptr = sk as *const u8;
    ptr.add(offset) as *mut ipv6_pinfo
}

/// Set rx destination information
///
/// # Safety
/// - `sk` must be a valid pointer to sock
/// - `skb` must be a valid pointer to sk_buff
#[no_mangle]
pub unsafe extern "C" fn inet6_sk_rx_dst_set(sk: *mut sock, skb: *const sk_buff) {
    let dst = (*skb).dst;
    if !dst.is_null() {
        // SAFETY: dst is non-null and valid
        let rt = dst as *const rt6_info;
        (*sk).rx_dst = dst;
        (*sk).rx_dst_ifindex = (*skb).skb_iif;
        (*tcp_inet6_sk(sk)).rx_dst_cookie = (*rt).cookie;
    }
}

/// Initialize TCP sequence number for IPv6
///
/// # Safety
/// - `skb` must be a valid pointer to sk_buff
#[no_mangle]
pub unsafe extern "C" fn tcp_v6_init_seq(skb: *const sk_buff) -> u32 {
    let ipv6_hdr = (*skb).ipv6_hdr;
    let tcp_hdr = (*skb).tcp_hdr;
    secure_tcpv6_seq(
        ipv6_hdr.daddr.s6_addr32,
        ipv6_hdr.saddr.s6_addr32,
        tcp_hdr.dest,
        tcp_hdr.source,
    )
}

/// Initialize TCP timestamp offset for IPv6
///
/// # Safety
/// - `net` must be a valid pointer to net
/// - `skb` must be a valid pointer to sk_buff
#[no_mangle]
pub unsafe extern "C" fn tcp_v6_init_ts_off(net: *const c_void, skb: *const sk_buff) -> u32 {
    let ipv6_hdr = (*skb).ipv6_hdr;
    secure_tcpv6_ts_off(net, ipv6_hdr.daddr.s6_addr32, ipv6_hdr.saddr.s6_addr32)
}

/// Pre-connect processing for IPv6
///
/// # Safety
/// - `sk` must be a valid pointer to sock
/// - `uaddr` must be a valid pointer to sockaddr
#[no_mangle]
pub unsafe extern "C" fn tcp_v6_pre_connect(
    sk: *mut sock,
    uaddr: *mut sockaddr,
    addr_len: c_int,
) -> c_int {
    if addr_len < 28 {
        // SIN6_LEN_RFC2133
        return EINVAL;
    }
    // Implementation of sock_owned_by_me and BPF_CGROUP_RUN_PROG_INET6_CONNECT
    // would go here
    0
}

/// Connect function for TCP over IPv6
///
/// # Safety
/// - `sk` must be a valid pointer to sock
/// - `uaddr` must be a valid pointer to sockaddr
#[no_mangle]
pub unsafe extern "C" fn tcp_v6_connect(
    sk: *mut sock,
    uaddr: *mut sockaddr,
    addr_len: c_int,
) -> c_int {
    let usin = uaddr as *mut sockaddr_in6;
    let inet = (*sk).inet_sk;
    let icsk = (*sk).icsk;
    let np = tcp_inet6_sk(sk);
    let tp = (*sk).tcp_sock;
    let tcp_death_row = (*sk).tcp_death_row;

    if addr_len < 28 {
        return EINVAL;
    }

    if (*usin).sin6_family != 10 {
        // AF_INET6
        return EAFNOSUPPORT;
    }

    // ... rest of the implementation would follow
    // This is a simplified version showing the structure
    0
}

/// Handle MTU reduction for IPv6
///
/// # Safety
/// - `sk` must be a valid pointer to sock
#[no_mangle]
pub unsafe extern "C" fn tcp_v6_mtu_reduced(sk: *mut sock) {
    let tp = (*sk).tcp_sock;
    let mtu = (*tp).mtu_info;
    // ... rest of the implementation
}

/// Handle TCP error for IPv6
///
/// # Safety
/// - `skb` must be a valid pointer to sk_buff
#[no_mangle]
pub unsafe extern "C" fn tcp_v6_err(
    skb: *mut sk_buff,
    opt: *mut c_void,
    type_: c_int,
    code: c_int,
    offset: c_int,
    info: u32,
) -> c_int {
    // Implementation would go here
    0
}

// Tests (conditional compilation)
#[cfg(test)]
mod tests {
    use super::*;
    use core::ptr;

    #[test]
    fn test_tcp_inet6_sk() {
        // Basic test for pointer arithmetic
        let sk = ptr::null_mut();
        let result = unsafe { super::tcp_inet6_sk(sk) };
        assert!(result.is_null());
    }
}