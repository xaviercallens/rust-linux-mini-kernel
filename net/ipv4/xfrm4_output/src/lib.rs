//! IPv4 IPsec encapsulation code
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(clippy::all)]

use core::ffi::c_int;
use core::ffi::c_void;
use core::ptr;

// Constants from C
pub const EMSGSIZE: c_int = 100;
pub const IPSKB_REROUTED: c_int = 1 << 0;

// Type definitions
#[repr(C)]
pub struct net {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xfrm_state {
    _private: [u8; 0],
}

#[repr(C)]
pub struct iphdr {
    daddr: u32,
}

#[repr(C)]
pub struct dst_entry {
    xfrm: *mut xfrm_state,
    dev: *mut c_void,
}

// Function pointers
type HookFn = unsafe extern "C" fn(*mut net, *mut sock, *mut sk_buff) -> c_int;

// Assume these functions are available in the kernel
extern "C" {
    fn dst_output(net: *mut net, sk: *mut sock, skb: *mut sk_buff) -> c_int;
    fn xfrm_output(sk: *mut sock, skb: *mut sk_buff) -> c_int;
    fn ip_local_error(sk: *mut sock, code: c_int, daddr: u32, dport: u16, mtu: u32);
}

// Assume these functions are available
#[cfg(CONFIG_NETFILTER)]
unsafe fn __xfrm4_output(net: *mut net, sk: *mut sock, skb: *mut sk_buff) -> c_int {
    let dst = skb_dst(skb);
    let x = (*dst).xfrm;
    
    if x.is_null() {
        // SAFETY: IPCB(skb) returns a valid pointer, and we can write to flags
        let ipc = IPCB(skb);
        (*ipc).flags |= IPSKB_REROUTED;
        return dst_output(net, sk, skb);
    }
    
    xfrm_output(sk, skb)
}

#[no_mangle]
pub unsafe extern "C" fn xfrm4_output(net: *mut net, sk: *mut sock, skb: *mut sk_buff) -> c_int {
    // SAFETY: All parameters are valid pointers from the kernel context
    NF_HOOK_COND(
        NFPROTO_IPV4,
        NF_INET_POST_ROUTING,
        net,
        sk,
        skb,
        (*skb).dev,
        (*skb_dst(skb)).dev,
        Some(__xfrm4_output),
        !(IPCB(skb).flags & IPSKB_REROUTED),
    )
}

#[no_mangle]
pub unsafe extern "C" fn xfrm4_local_error(skb: *mut sk_buff, mtu: u32) {
    let hdr: *mut iphdr;
    
    if (*skb).encapsulation != 0 {
        // SAFETY: inner_ip_hdr returns a valid pointer when encapsulation is enabled
        hdr = inner_ip_hdr(skb);
    } else {
        // SAFETY: ip_hdr returns a valid pointer when encapsulation is disabled
        hdr = ip_hdr(skb);
    }
    
    let daddr = (*hdr).daddr;
    let dport = inet_sk(skb).inet_dport;
    
    // SAFETY: skb->sk is a valid pointer to sock
    ip_local_error((*skb).sk, EMSGSIZE, daddr, dport, mtu);
}

// Constants for NF_HOOK_COND
const NFPROTO_IPV4: c_int = 2;
const NF_INET_POST_ROUTING: c_int = 3;

// Assume these functions are available
unsafe fn skb_dst(skb: *mut sk_buff) -> *mut dst_entry {
    // Implementation would be provided by the kernel
    ptr::null_mut()
}

unsafe fn IPCB(skb: *mut sk_buff) -> *mut iphdr {
    // Implementation would be provided by the kernel
    ptr::null_mut()
}

unsafe fn inner_ip_hdr(skb: *mut sk_buff) -> *mut iphdr {
    // Implementation would be provided by the kernel
    ptr::null_mut()
}

unsafe fn ip_hdr(skb: *mut sk_buff) -> *mut iphdr {
    // Implementation would be provided by the kernel
    ptr::null_mut()
}

unsafe fn NF_HOOK_COND(
    pf: c_int,
    hook: c_int,
    net: *mut net,
    sk: *mut sock,
    skb: *mut sk_buff,
    indev: *mut c_void,
    outdev: *mut c_void,
    okfn: Option<HookFn>,
    cond: bool,
) -> c_int {
    // Implementation would be provided by the kernel
    0
}

unsafe fn inet_sk(sk: *mut sock) -> *mut c_void {
    // Implementation would be provided by the kernel
    ptr::null_mut()
}
