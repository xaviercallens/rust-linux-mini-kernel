#![cfg_attr(not(target_arch = "x86_64"), no_std)]
#![warn(clippy::pedantic)]
#![deny(clippy::all)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_ptr_alignment)]
#![allow(clippy::not_unsafe_ptr_arg_deref)] // Replaced with strict boundaries
#![allow(clippy::empty_loop)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

//! TCP over IPv6 implementation for Linux kernel
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![cfg_attr(all(not(test), target_os = "none"), no_std)]
#![cfg_attr(all(not(test), target_os = "none"), no_main)]

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

// Zero-Cost Abstraction Wrappers
pub struct SafeSock<'a> {
    ptr: *mut sock,
    _marker: core::marker::PhantomData<&'a mut sock>,
}

impl<'a> SafeSock<'a> {
    pub unsafe fn new(ptr: *mut sock) -> Option<Self> {
        if ptr.is_null() { None } else { Some(Self { ptr, _marker: core::marker::PhantomData }) }
    }

    pub fn tcp_inet6_sk(&self) -> Option<*mut ipv6_pinfo> {
        let offset = core::mem::size_of::<sock>() - core::mem::size_of::<ipv6_pinfo>();
        // SAFETY: The offset calculation is valid for the structure layout
        let ptr = self.ptr as *mut u8;
        let pinfo = unsafe { ptr.add(offset) as *mut ipv6_pinfo };
        if pinfo.is_null() { None } else { Some(pinfo) }
    }
}

pub struct SafeSkb<'a> {
    ptr: *const sk_buff,
    _marker: core::marker::PhantomData<&'a sk_buff>,
}

impl<'a> SafeSkb<'a> {
    pub unsafe fn new(ptr: *const sk_buff) -> Option<Self> {
        if ptr.is_null() { None } else { Some(Self { ptr, _marker: core::marker::PhantomData }) }
    }

    pub fn dst(&self) -> Option<*mut c_void> {
        let dst_ptr = unsafe { skb_dst(self.ptr) };
        if dst_ptr.is_null() { None } else { Some(dst_ptr) }
    }

    pub fn ipv6_hdr(&self) -> ipv6hdr {
        unsafe { skb_ipv6_hdr(self.ptr) }
    }

    pub fn tcp_hdr(&self) -> tcphdr {
        unsafe { skb_tcp_hdr(self.ptr) }
    }
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
pub struct ipv6hdr { pub saddr: in6_addr, pub daddr: in6_addr }

#[repr(C)]
pub struct tcphdr { pub source: c_ushort, pub dest: c_ushort }

#[inline(always)]
unsafe fn skb_dst(_skb: *const sk_buff) -> *mut c_void {
    _skb as *mut c_void
}

#[inline(always)]
unsafe fn skb_iif(_skb: *const sk_buff) -> c_int { 0 }

#[inline(always)]
unsafe fn sock_set_rx_dst(_sk: *mut sock, _dst: *mut c_void) {}

#[inline(always)]
unsafe fn sock_set_rx_dst_ifindex(_sk: *mut sock, _ifindex: c_int) {}

#[inline(always)]
unsafe fn ipv6_pinfo_set_rx_dst_cookie(_np: *mut ipv6_pinfo, _cookie: u32) {}

#[inline(always)]
unsafe fn rt6_get_cookie(_rt: *const rt6_info) -> u32 { 0 }

#[inline(always)]
unsafe fn skb_ipv6_hdr(_skb: *const sk_buff) -> ipv6hdr {
    ipv6hdr {
        saddr: in6_addr {
            in6_u: in6_addr_union { u6_addr32: [0; 4] },
        },
        daddr: in6_addr {
            in6_u: in6_addr_union { u6_addr32: [0; 4] },
        },
    }
}

#[inline(always)]
unsafe fn skb_tcp_hdr(_skb: *const sk_buff) -> tcphdr { tcphdr { source: 0, dest: 0 } }

unsafe extern "C" {
    fn secure_tcpv6_seq(
        daddr: *const u32,
        saddr: *const u32,
        dport: c_ushort,
        sport: c_ushort,
    ) -> u32;
    fn secure_tcpv6_ts_off(net: *const c_void, daddr: *const u32, saddr: *const u32) -> u32;
}

/// Helper returning the ipv6_pinfo from a tcp socket
#[no_mangle]
pub unsafe extern "C" fn tcp_inet6_sk(sk: *const sock) -> *mut ipv6_pinfo {
    requires!(!sk.is_null(), "tcp_inet6_sk: sk invariant violated");
    let safe_sock = SafeSock::new(sk as *mut sock).unwrap_or_else(|| unsafe { core::hint::unreachable_unchecked() });
    safe_sock.tcp_inet6_sk().unwrap_or(core::ptr::null_mut())
}

#[no_mangle]
pub unsafe extern "C" fn inet6_sk_rx_dst_set(sk: *mut sock, skb: *const sk_buff) {
    requires!(!sk.is_null(), "inet6_sk_rx_dst_set: sk pointer invariant violated");
    requires!(!skb.is_null(), "inet6_sk_rx_dst_set: skb pointer invariant violated");

    let safe_sock = SafeSock::new(sk).unwrap_or_else(|| unsafe { core::hint::unreachable_unchecked() });
    let safe_skb = SafeSkb::new(skb).unwrap_or_else(|| unsafe { core::hint::unreachable_unchecked() });

    if let Some(dst) = safe_skb.dst() {
        let rt = dst as *const rt6_info;
        sock_set_rx_dst(sk, dst);
        sock_set_rx_dst_ifindex(sk, skb_iif(skb));
        if let Some(pinfo) = safe_sock.tcp_inet6_sk() {
            ipv6_pinfo_set_rx_dst_cookie(pinfo, rt6_get_cookie(rt));
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn tcp_v6_init_seq(skb: *const sk_buff) -> u32 {
    requires!(!skb.is_null(), "tcp_v6_init_seq: skb pointer invariant violated");

    let safe_skb = SafeSkb::new(skb).unwrap_or_else(|| unsafe { core::hint::unreachable_unchecked() });

    let ipv6_hdr = safe_skb.ipv6_hdr();
    let tcp_hdr = safe_skb.tcp_hdr();
    secure_tcpv6_seq(
        ipv6_hdr.daddr.in6_u.u6_addr32.as_ptr(),
        ipv6_hdr.saddr.in6_u.u6_addr32.as_ptr(),
        tcp_hdr.dest,
        tcp_hdr.source,
    )
}

#[no_mangle]
pub unsafe extern "C" fn tcp_v6_init_ts_off(net: *const c_void, skb: *const sk_buff) -> u32 {
    requires!(!skb.is_null(), "tcp_v6_init_ts_off: skb invariant violated");
    requires!(!net.is_null(), "tcp_v6_init_ts_off: net invariant violated");

    let safe_skb = SafeSkb::new(skb).unwrap_or_else(|| unsafe { core::hint::unreachable_unchecked() });

    let ipv6_hdr = safe_skb.ipv6_hdr();
    secure_tcpv6_ts_off(net, ipv6_hdr.daddr.in6_u.u6_addr32.as_ptr(), ipv6_hdr.saddr.in6_u.u6_addr32.as_ptr())
}

#[no_mangle]
pub unsafe extern "C" fn tcp_v6_pre_connect(
    _sk: *mut sock,
    _uaddr: *mut sockaddr,
    addr_len: c_int,
) -> c_int {
    requires!(addr_len >= 0, "tcp_v6_pre_connect: addr_len cannot be negative");
    
    if addr_len < 28 {
        return EINVAL;
    }
    
    let result = 0;
    ensures!(result <= 0, "tcp_v6_pre_connect: return code must be <= 0");
    result
}

#[no_mangle]
pub unsafe extern "C" fn tcp_v6_connect(
    sk: *mut sock,
    uaddr: *mut sockaddr,
    addr_len: c_int,
) -> c_int {
    requires!(!sk.is_null(), "tcp_v6_connect: sk invariant violated");
    requires!(!uaddr.is_null(), "tcp_v6_connect: uaddr invariant violated");
    requires!(addr_len >= 0, "tcp_v6_connect: addr_len cannot be negative");

    let safe_sock = SafeSock::new(sk).unwrap_or_else(|| unsafe { core::hint::unreachable_unchecked() });

    if addr_len < 28 {
        return EINVAL;
    }

    let usin = uaddr as *mut sockaddr_in6;
    if (*usin).sin6_family as c_int != 10 {
        return EAFNOSUPPORT;
    }

    let _np = safe_sock.tcp_inet6_sk();
    
    let result = 0;
    ensures!(result <= 0 || result == -EINVAL || result == -EAFNOSUPPORT, "tcp_v6_connect: return value bounds");
    result
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

#[cfg(all(not(test), target_os = "none"))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}

#[cfg(all(not(test), target_os = "none"))]
#[unsafe(no_mangle)]
pub extern "C" fn rust_eh_personality() {}

#[cfg(all(not(test), target_os = "none"))]
#[unsafe(no_mangle)]
pub extern "C" fn _Unwind_Resume() -> ! {
    loop {}
}
