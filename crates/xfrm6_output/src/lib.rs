//! Common IPsec encapsulation code for IPv6 in the Linux kernel.
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)] // For C-style type names

use core::ffi::c_int;
use core::ffi::c_uint;
use core::ffi::c_ulong;
use core::ffi::c_void;
use core::ptr;

// Constants from C
pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const EMSGSIZE: c_int = -90;

// Type definitions
#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xfrm_state {
    props: xfrm_state_props,
}

#[repr(C)]
pub struct xfrm_state_props {
    mode: c_int,
}

#[repr(C)]
pub struct dst_entry {
    xfrm: *mut xfrm_state,
}

#[repr(C)]
pub struct sock {
    sk_bound_dev_if: c_int,
    _private: [u8; 0],
}

#[repr(C)]
pub struct net {
    _private: [u8; 0],
}

#[repr(C)]
pub struct flowi6 {
    flowi6_oif: c_int,
    daddr: u32,
    fl6_dport: u16,
}

#[repr(C)]
pub struct ipv6hdr {
    daddr: u32,
}

// Function pointers for external C functions
extern "C" {
    fn ip6_find_1stfragopt(skb: *mut sk_buff, prevhdr: *mut *mut u8) -> c_int;
    fn ipv6_local_rxpmtu(sk: *mut sock, fl: *mut flowi6, mtu: c_uint);
    fn ipv6_local_error(sk: *mut sock, code: c_int, fl: *mut flowi6, mtu: c_uint);
    fn xfrm_output(sk: *mut sock, skb: *mut sk_buff) -> c_int;
    fn ip6_fragment(
        net: *mut net,
        sk: *mut sock,
        skb: *mut sk_buff,
        okfn: extern "C" fn(*mut net, *mut sock, *mut sk_buff) -> c_int,
    ) -> c_int;
    fn dst_output(net: *mut net, sk: *mut sock, skb: *mut sk_buff) -> c_int;
    fn xfrm_local_error(skb: *mut sk_buff, mtu: c_uint) -> c_int;
    fn kfree_skb(skb: *mut sk_buff);
    fn ip6_skb_dst_mtu(skb: *mut sk_buff) -> c_uint;
    fn dst_mtu(dst: *mut dst_entry) -> c_uint;
    fn skb_dst(skb: *mut sk_buff) -> *mut dst_entry;
    fn skb_is_gso(skb: *mut sk_buff) -> c_int;
    fn dst_allfrag(dst: *mut dst_entry) -> c_int;
    fn inner_ipv6_hdr(skb: *mut sk_buff) -> *mut ipv6hdr;
    fn ipv6_hdr(skb: *mut sk_buff) -> *mut ipv6hdr;
    fn xfrm6_local_dontfrag(sk: *mut sock) -> c_int;
    fn NF_HOOK_COND(
        proto: c_int,
        hook: c_int,
        net: *mut net,
        sk: *mut sock,
        skb: *mut sk_buff,
        indev: *mut c_void,
        outdev: *mut c_void,
        okfn: extern "C" fn(*mut net, *mut sock, *mut sk_buff) -> c_int,
        cond: c_int,
    ) -> c_int;
}

// Function implementations
/// Find the first fragment option in IPv6 packet
///
/// # Safety
/// - `x` must be a valid pointer to xfrm_state (unused in implementation)
/// - `skb` must be a valid pointer to sk_buff
/// - `prevhdr` must be a valid pointer to u8 pointer
///
/// # Returns
/// Result from ip6_find_1stfragopt
#[no_mangle]
pub unsafe extern "C" fn xfrm6_find_1stfragopt(
    x: *mut xfrm_state,
    skb: *mut sk_buff,
    prevhdr: *mut *mut u8,
) -> c_int {
    ip6_find_1stfragopt(skb, prevhdr)
}

/// Handle PMTU discovery for IPv6 local receive
///
/// # Safety
/// - `skb` must be a valid pointer to sk_buff
/// - `mtu` must be a valid MTU value
#[no_mangle]
pub unsafe extern "C" fn xfrm6_local_rxpmtu(skb: *mut sk_buff, mtu: c_uint) {
    let sk = (*skb).sk;
    let fl6 = flowi6 {
        flowi6_oif: (*sk).sk_bound_dev_if,
        daddr: (*ipv6_hdr(skb)).daddr,
        fl6_dport: 0, // Not used in this context
    };

    ipv6_local_rxpmtu(sk, &mut fl6);
}

/// Handle local error for IPv6 PMTU discovery
///
/// # Safety
/// - `skb` must be a valid pointer to sk_buff
/// - `mtu` must be a valid MTU value
#[no_mangle]
pub unsafe extern "C" fn xfrm6_local_error(skb: *mut sk_buff, mtu: c_uint) {
    let sk = (*skb).sk;
    let hdr = if (*skb).encapsulation != 0 {
        inner_ipv6_hdr(skb)
    } else {
        ipv6_hdr(skb)
    };

    let fl6 = flowi6 {
        fl6_dport: (*inet_sk(sk)).inet_dport,
        daddr: (*hdr).daddr,
        flowi6_oif: 0, // Not used in this context
    };

    ipv6_local_error(sk, EMSGSIZE, &mut fl6, mtu);
}

/// Final output handler for xfrm6_output
fn __xfrm6_output_finish(net: *mut net, sk: *mut sock, skb: *mut sk_buff) -> c_int {
    unsafe { xfrm_output(sk, skb) }
}

/// Main output handler for xfrm6_output
fn __xfrm6_output(net: *mut net, sk: *mut sock, skb: *mut sk_buff) -> c_int {
    let dst = unsafe { skb_dst(skb) };
    let x = unsafe { (*dst).xfrm };

    // CONFIG_NETFILTER handling
    #[cfg(CONFIG_NETFILTER)]
    {
        if x.is_null() {
            unsafe {
                (*IP6CB(skb)).flags |= IP6SKB_REROUTED;
                return dst_output(net, sk, skb);
            }
        }
    }

    if unsafe { (*x).props.mode } != XFRM_MODE_TUNNEL {
        return unsafe { xfrm_output(sk, skb) };
    }

    let protocol = unsafe { (*skb).protocol };
    let mtu = if protocol == htons(ETH_P_IPV6) as c_int {
        unsafe { ip6_skb_dst_mtu(skb) }
    } else {
        unsafe { dst_mtu(dst) }
    };

    let toobig = unsafe { (*skb).len > mtu && skb_is_gso(skb) == 0 };

    if toobig && unsafe { xfrm6_local_dontfrag((*skb).sk) } != 0 {
        unsafe {
            xfrm6_local_rxpmtu(skb, mtu);
            kfree_skb(skb);
            return -EMSGSIZE;
        }
    } else if !unsafe { (*skb).ignore_df } && toobig && !(*skb).sk.is_null() {
        unsafe {
            xfrm_local_error(skb, mtu);
            kfree_skb(skb);
            return -EMSGSIZE;
        }
    }

    if toobig || unsafe { dst_allfrag(dst) } != 0 {
        return unsafe { ip6_fragment(net, sk, skb, __xfrm6_output_finish) };
    }

    unsafe { xfrm_output(sk, skb) }
}

/// Main xfrm6 output function with netfilter hook
#[no_mangle]
pub unsafe extern "C" fn xfrm6_output(net: *mut net, sk: *mut sock, skb: *mut sk_buff) -> c_int {
    NF_HOOK_COND(
        NFPROTO_IPV6,
        NF_INET_POST_ROUTING,
        net,
        sk,
        skb,
        (*skb).dev as *mut c_void,
        (*skb_dst(skb)).dev as *mut c_void,
        __xfrm6_output,
        !((*IP6CB(skb)).flags & IP6SKB_REROUTED),
    )
}

// Helper functions (assumed to exist in C)
#[repr(C)]
struct inet_sock {
    inet_dport: u16,
}

#[repr(C)]
struct ip6_skb_cb {
    flags: c_ulong,
}

#[repr(C)]
struct ip6_skb {
    dev: *mut c_void,
    ignore_df: c_int,
    len: c_int,
    protocol: c_int,
    encapsulation: c_int,
    sk: *mut sock,
    _private: [u8; 0],
}

#[repr(C)]
struct IP6CB {
    flags: c_ulong,
}

const ETH_P_IPV6: c_int = 0x86DD;
const XFRM_MODE_TUNNEL: c_int = 2;
const IP6SKB_REROUTED: c_ulong = 1 << 0;
const NFPROTO_IPV6: c_int = 10;
const NF_INET_POST_ROUTING: c_int = 3;

#[inline(always)]
unsafe fn htons(x: c_int) -> c_int {
    ((x >> 8) & 0xff) | ((x & 0xff) << 8)
}

#[inline(always)]
unsafe fn inet_sk(sk: *mut sock) -> *mut inet_sock {
    (sk as *mut u8).offset(0) as *mut inet_sock
}

#[inline(always)]
unsafe fn IP6CB(skb: *mut sk_buff) -> *mut ip6_skb_cb {
    (skb as *mut u8).offset(0) as *mut ip6_skb_cb
}

#[inline(always)]
unsafe fn IP6CB(skb: *mut sk_buff) -> *mut IP6CB {
    (skb as *mut u8).offset(0) as *mut IP6CB
}
