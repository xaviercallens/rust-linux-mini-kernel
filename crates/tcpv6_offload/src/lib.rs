use kernel_types::*;
use core::ffi::c_int;
use core::ptr;

// Constants from C
pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;

// Type definitions
#[repr(C)]
struct NapiGroCb {
    flush: u8,
}

#[repr(C)]
struct NetOffload {
    callbacks: NetOffloadCallbacks,
}

#[repr(C)]
struct NetOffloadCallbacks {
    gso_segment: extern "C" fn(*mut sk_buff, netdev_features_t) -> *mut sk_buff,
    gro_receive: extern "C" fn(*mut core::ffi::c_void, *mut sk_buff) -> *mut sk_buff,
    gro_complete: extern "C" fn(*mut sk_buff, c_int) -> c_int,
}

type netdev_features_t = u32;

// Function implementations
#[no_mangle]
pub unsafe extern "C" fn tcp6_gro_receive(
    head: *mut core::ffi::c_void,
    skb: *mut sk_buff,
) -> *mut sk_buff {
    // SAFETY: Caller guarantees valid skb pointer
    let cb = NAPI_GRO_CB(skb);

    if (*cb).flush == 0 && skb_gro_checksum_validate(skb, IPPROTO_TCP, ip6_gro_compute_pseudo) != 0 {
        // SAFETY: Valid pointer access
        (*cb).flush = 1;
        return ptr::null_mut();
    }

    tcp_gro_receive(head, skb)
}

#[no_mangle]
pub unsafe extern "C" fn tcp6_gro_complete(skb: *mut sk_buff, thoff: c_int) -> c_int {
    let iph = ipv6_hdr(skb);
    let th = tcp_hdr(skb);

    // SAFETY: Valid pointer access
    (*th).check = !tcp_v6_check(
        (*skb).len,
        &(*iph).saddr,
        &(*iph).daddr,
        0,
    );

    // SAFETY: skb_shinfo is valid for write
    (*skb_shinfo(skb)).gso_type |= SKB_GSO_TCPV6;

    tcp_gro_complete(skb)
}

#[no_mangle]
pub unsafe extern "C" fn tcp6_gso_segment(
    skb: *mut sk_buff,
    features: netdev_features_t,
) -> *mut sk_buff {
    let shinfo = skb_shinfo(skb);

    if (*shinfo).gso_type & SKB_GSO_TCPV6 == 0 {
        return ERR_PTR(EINVAL);
    }

    if !pskb_may_pull(skb, core::mem::size_of::<udphdr>() as _) {
        return ERR_PTR(EINVAL);
    }

    if (*skb).csum != CHECKSUM_PARTIAL {
        let ipv6h = ipv6_hdr(skb);
        let th = tcp_hdr(skb);

        // Set up pseudo header
        (*th).check = 0;
        (*skb).csum = CHECKSUM_PARTIAL;
        __tcp_v6_send_check(skb, &(*ipv6h).saddr, &(*ipv6h).daddr);
    }

    tcp_gso_segment(skb, features)
}

#[no_mangle]
pub extern "C" fn tcpv6_offload_init() -> c_int {
    unsafe { inet6_add_offload(&TCPV6_OFFLOAD, IPPROTO_TCP) }
}

// Constants
const IPPROTO_TCP: c_int = 6;
const SKB_GSO_TCPV6: netdev_features_t = 0x00000800;
const CHECKSUM_PARTIAL: c_int = 2;

// Helper macros translated to functions
#[inline]
unsafe fn NAPI_GRO_CB(skb: *mut sk_buff) -> *mut NapiGroCb {
    // In real implementation, this would use offsetof from Linux's napi_gro_cb location
    // For demonstration, we'll assume it's at a fixed offset
    let offset = 128; // Example offset - actual value depends on sk_buff layout
    (skb as *mut u8).add(offset) as *mut NapiGroCb
}

#[inline]
unsafe fn skb_gro_checksum_validate(
    skb: *mut sk_buff,
    proto: c_int,
    pseudo: extern "C" fn(*mut sk_buff) -> c_int,
) -> c_int {
    // Stub implementation - actual implementation would be complex
    0
}

#[inline]
unsafe fn ipv6_hdr(skb: *mut sk_buff) -> *mut ipv6hdr {
    // In real implementation, this would access (*skb).head
    let head = (*skb).head;
    head as *mut ipv6hdr
}

#[inline]
unsafe fn tcp_hdr(skb: *mut sk_buff) -> *mut udphdr {
    // In real implementation, this would access (*skb).head + transport header offset
    let head = (*skb).head;
    head.add(40) as *mut udphdr // IPv6 header is 40 bytes
}

#[inline]
unsafe fn skb_shinfo(skb: *mut sk_buff) -> *mut SkbSharedInfo {
    // In real implementation, this would point to (*skb).shares_info
    let offset = 256; // Example offset - actual depends on sk_buff layout
    (skb as *mut u8).add(offset) as *mut SkbSharedInfo
}

#[repr(C)]
struct SkbSharedInfo {
    gso_type: netdev_features_t,
}

#[inline]
unsafe fn ERR_PTR(error: c_int) -> *mut sk_buff {
    (error as *mut sk_buff).wrapping_sub(1)
}

#[inline]
unsafe fn pskb_may_pull(skb: *mut sk_buff, len: usize) -> bool {
    // Stub implementation - actual implementation would check headroom
    true
}

#[inline]
unsafe fn __tcp_v6_send_check(skb: *mut sk_buff, saddr: *const in6_addr, daddr: *const in6_addr) {
    // Stub implementation - actual implementation would compute checksum
}

#[inline]
unsafe fn tcp_v6_check(
    len: u32,
    saddr: *const in6_addr,
    daddr: *const in6_addr,
    old_checksum: u32,
) -> u32 {
    // Stub implementation - actual implementation would compute TCP checksum
    0
}

#[inline]
unsafe fn ip6_gro_compute_pseudo(skb: *mut sk_buff) -> c_int {
    // Stub implementation - actual implementation would compute pseudo header
    0
}

// Global static
#[no_mangle]
static TCPV6_OFFLOAD: NetOffload = NetOffload {
    callbacks: NetOffloadCallbacks {
        gso_segment: tcp6_gso_segment,
        gro_receive: tcp6_gro_receive,
        gro_complete: tcp6_gro_complete,
    },
};

// External functions (would be implemented elsewhere)
extern "C" {
    fn tcp_gro_receive(head: *mut core::ffi::c_void, skb: *mut sk_buff) -> *mut sk_buff;
    fn tcp_gro_complete(skb: *mut sk_buff) -> c_int;
    fn tcp_gso_segment(skb: *mut sk_buff, features: netdev_features_t) -> *mut sk_buff;
    fn inet6_add_offload(offload: *const NetOffload, proto: c_int) -> c_int;
}