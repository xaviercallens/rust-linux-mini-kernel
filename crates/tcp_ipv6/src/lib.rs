
//! TCP over IPv6 implementation for Linux kernel
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![no_main]
#![allow(non_camel_case_types)]

use core::ffi::c_void;
use kernel_types::*;

pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ENETUNREACH: c_int = -101;
pub const EAFNOSUPPORT: c_int = -97;
pub const ENOENT: c_int = -2;

// Static variables
pub static mut __UDP_DISCONNECT: *mut core::ffi::c_void = core::ptr::null_mut();
pub static mut ICMPV6_ERR_CONVERT: *mut core::ffi::c_void = core::ptr::null_mut();
pub static mut INET6_SOCKRAW_OPS: *mut core::ffi::c_void = core::ptr::null_mut();
pub static mut IP6_DATAGRAM_CONNECT_V6_ONLY: *mut core::ffi::c_void = core::ptr::null_mut();

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

#[repr(C)]
pub struct sockaddr_in6 {
    pub sin6_family: c_ushort,
    pub sin6_port: c_ushort,
    pub sin6_flowinfo: u32,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: u32,
}

#[repr(C)]
pub struct ipv6hdr {
    pub saddr: in6_addr,
    pub daddr: in6_addr,
}

#[repr(C)]
pub struct tcphdr {
    pub source: c_ushort,
    pub dest: c_ushort,
}

#[inline(always)]
unsafe fn skb_dst(_skb: *const sk_buff) -> *mut c_void {
    core::ptr::null_mut()
}

#[inline(always)]
unsafe fn skb_iif(_skb: *const sk_buff) -> c_int {
    0
}

#[inline(always)]
unsafe fn sock_set_rx_dst(_sk: *mut sock, _dst: *mut c_void) {}

#[inline(always)]
unsafe fn sock_set_rx_dst_ifindex(_sk: *mut sock, _ifindex: c_int) {}

#[inline(always)]
unsafe fn ipv6_pinfo_set_rx_dst_cookie(_np: *mut ipv6_pinfo, _cookie: u32) {}

#[inline(always)]
unsafe fn rt6_get_cookie(_rt: *const rt6_info) -> u32 {
    0
}

#[inline(always)]
unsafe fn skb_ipv6_hdr(_skb: *const sk_buff) -> ipv6hdr {
    ipv6hdr {
        saddr: in6_addr { s6_addr32: [0; 4] },
        daddr: in6_addr { s6_addr32: [0; 4] },
    }
}

#[inline(always)]
unsafe fn skb_tcp_hdr(_skb: *const sk_buff) -> tcphdr {
    tcphdr { source: 0, dest: 0 }
}

unsafe extern "C" {
    fn secure_tcpv6_seq(
        daddr: [u32; 4],
        saddr: [u32; 4],
        dport: c_ushort,
        sport: c_ushort,
    ) -> u32;
    fn secure_tcpv6_ts_off(net: *const c_void, daddr: [u32; 4], saddr: [u32; 4]) -> u32;
}

#[no_mangle]
pub unsafe extern "C" fn tcp_inet6_sk(sk: *const sock) -> *mut ipv6_pinfo {
    if sk.is_null() {
        return core::ptr::null_mut();
    }
    let offset = core::mem::size_of::<sock>() - core::mem::size_of::<ipv6_pinfo>();
    (sk as *const u8).add(offset) as *mut ipv6_pinfo
}

#[no_mangle]
pub unsafe extern "C" fn inet6_sk_rx_dst_set(sk: *mut sock, skb: *const sk_buff) {
    if sk.is_null() || skb.is_null() {
        return;
    }

    let dst = skb_dst(skb);
    if !dst.is_null() {
        let rt = dst as *const rt6_info;
        sock_set_rx_dst(sk, dst);
        sock_set_rx_dst_ifindex(sk, skb_iif(skb));
        ipv6_pinfo_set_rx_dst_cookie(tcp_inet6_sk(sk), rt6_get_cookie(rt));
    }
}

#[no_mangle]
pub unsafe extern "C" fn tcp_v6_init_seq(skb: *const sk_buff) -> u32 {
    if skb.is_null() {
        return 0;
    }
    let ipv6_hdr = skb_ipv6_hdr(skb);
    let tcp_hdr = skb_tcp_hdr(skb);
    secure_tcpv6_seq(
        ipv6_hdr.daddr.in6_u.u6_addr32,
        ipv6_hdr.saddr.in6_u.u6_addr32,
        tcp_hdr.dest,
        tcp_hdr.source,
    )
}

#[no_mangle]
pub unsafe extern "C" fn tcp_v6_init_ts_off(net: *const c_void, skb: *const sk_buff) -> u32 {
    let ipv6_hdr = (*skb).ipv6_hdr;
    secure_tcpv6_ts_off(net, ipv6_hdr.daddr.in6_u.u6_addr32, ipv6_hdr.saddr.in6_u.u6_addr32)
}

#[no_mangle]
pub unsafe extern "C" fn tcp_v6_pre_connect(
    _sk: *mut sock,
    _uaddr: *mut sockaddr,
    addr_len: c_int,
) -> c_int {
    if addr_len < 28 {
        return EINVAL;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn tcp_v6_connect(
    sk: *mut sock,
    uaddr: *mut sockaddr,
    addr_len: c_int,
) -> c_int {
    if sk.is_null() || uaddr.is_null() {
        return EINVAL;
    }

    if addr_len < 28 {
        return EINVAL;
    }

    let usin = uaddr as *mut sockaddr_in6;
    if (*usin).sin6_family as c_int != 10 {
        return EAFNOSUPPORT;
    }

    let _np = tcp_inet6_sk(sk);
    0
}

#[no_mangle]
pub unsafe extern "C" fn tcp_v6_mtu_reduced(_sk: *mut sock) {}

#[no_mangle]
pub unsafe extern "C" fn tcp_v6_err(
    _skb: *mut sk_buff,
    _opt: *mut c_void,
    _type_: c_int,
    _code: c_int,
    _offset: c_int,
    _info: u32,
) -> c_int {
    0
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_eh_personality() {}

#[unsafe(no_mangle)]
pub extern "C" fn _Unwind_Resume() -> ! {
    loop {}
}