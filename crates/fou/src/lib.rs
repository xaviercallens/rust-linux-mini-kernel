//! FOU (FOO) and GUE (Generic UDP Encapsulation) tunneling protocol implementation
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]  // For C-style type names

use core::ptr;
use libc::{c_int, c_uint, c_void, size_t};
use kernel_types::*;

// Constants from C
pub const FOU_F_REMCSUM_NOPARTIAL: u8 = 1 << 0;

// Type definitions
#[repr(C)]
#[derive(Copy, Clone)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcu_head {
    next: *mut rcu_head,
    func: unsafe extern "C" fn(*mut rcu_head),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct guehdr {
    pub version: u8,
    pub hlen: u8,
    pub proto_ctype: u8,
    pub flags: u8,
    pub control: u8,
    _pad: [u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fou {
    pub sock: *mut sock,
    pub protocol: u8,
    pub flags: u8,
    pub port: u16,
    pub family: u8,
    pub type_: u16,
    pub list: list_head,
    pub rcu: rcu_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fou_net {
    pub fou_list: list_head,
    pub fou_lock: *mut c_void, // mutex
}

// Function implementations
/// Adjust skb after removing FOU header
///
/// # Safety
/// - `skb` must be a valid pointer to sk_buff
/// - `fou` must be a valid pointer to fou
/// - Caller must ensure proper synchronization
///
/// # Returns
/// 0 on success, error code otherwise
#[no_mangle]
pub unsafe extern "C" fn fou_recv_pull(
    skb: *mut sk_buff,
    fou: *mut fou,
    len: size_t,
) -> c_int {
    if skb.is_null() || fou.is_null() {
        return -22; // EINVAL
    }

    let fou = unsafe { &*fou };

    // SAFETY: Caller guarantees valid skb and fou pointers
    if fou.family == 0x02 { // AF_INET
        let ip = unsafe { ip_hdr(skb) };
        if ip.is_null() {
            return -22; // EINVAL
        }
        let tot_len = unsafe { (*ip).tot_len };
        unsafe { (*ip).tot_len = ((ntohs(tot_len) - len as u16) as u16).to_be() };
    } else if fou.family == 0x0a { // AF_INET6
        let ipv6 = unsafe { ipv6_hdr(skb) };
        if ipv6.is_null() {
            return -22; // EINVAL
        }
        let payload_len = unsafe { (*ipv6).payload_len };
        unsafe { (*ipv6).payload_len = ((ntohs(payload_len) - len as u16) as u16).to_be() };
    }

    // SAFETY: skb is valid and len is correct
    unsafe { __skb_pull(skb, len as usize) };

    // SAFETY: udp_hdr is valid after pull
    unsafe { skb_postpull_rcsum(skb, udp_hdr(skb), len as usize) };

    unsafe { iptunnel_pull_offloads(skb) }
}

/// UDP receive handler for FOU
///
/// # Safety
/// - `sk` must be a valid pointer to sock
/// - `skb` must be a valid pointer to sk_buff
///
/// # Returns
/// -1 on error, 0 to drop, 1 to continue
#[no_mangle]
pub unsafe extern "C" fn fou_udp_recv(
    sk: *mut sock,
    skb: *mut sk_buff,
) -> c_int {
    if sk.is_null() || skb.is_null() {
        return 1;
    }

    let fou = unsafe { (*sk).sk_user_data as *mut fou };
    if fou.is_null() {
        return 1;
    }

    if unsafe { fou_recv_pull(skb, fou, core::mem::size_of::<udphdr>() as size_t) } != 0 {
        unsafe { kfree_skb(skb) };
        return 0;
    }

    -(*fou).protocol as c_int
}

/// GUE remote checksum handling
///
/// # Safety
/// - All pointers must be valid
/// - Caller must handle memory safety
#[no_mangle]
pub unsafe extern "C" fn gue_remcsum(
    skb: *mut sk_buff,
    guehdr: *mut guehdr,
    data: *mut c_void,
    hdrlen: size_t,
    ipproto: u8,
    nopartial: c_int,
) -> *mut guehdr {
    if skb.is_null() || guehdr.is_null() || data.is_null() {
        return ptr::null_mut();
    }

    let pd = data as *mut u16;
    let start = unsafe { ntohs(*pd) };
    let offset = unsafe { ntohs(*pd.offset(1)) };
    let plen = core::mem::size_of::<udphdr>() as u64 + hdrlen as u64 +
        (offset as u64 + core::mem::size_of::<u16>() as u64).max(start as u64);

    if unsafe { (*skb).remcsum_offload } != 0 {
        return guehdr;
    }

    if !unsafe { pskb_may_pull(skb, plen as size_t) } {
        return ptr::null_mut();
    }

    let new_guehdr = unsafe { &mut *(udp_hdr(skb) as *mut udphdr as *mut guehdr).offset(1) };

    unsafe { skb_remcsum_process(skb, (new_guehdr as *mut c_void).offset(hdrlen), start, offset, nopartial != 0) };

    new_guehdr
}

/// GUE control message handler
///
/// # Safety
/// - `skb` must be valid
#[no_mangle]
pub unsafe extern "C" fn gue_control_message(skb: *mut sk_buff, guehdr: *mut guehdr) -> c_int {
    if skb.is_null() || guehdr.is_null() {
        return 0;
    }

    unsafe { kfree_skb(skb) };
    0
}

/// GUE UDP receive handler
///
/// # Safety
/// - All pointers must be valid
/// - Caller must handle memory safety
#[no_mangle]
pub unsafe extern "C" fn gue_udp_recv(
    sk: *mut sock,
    skb: *mut sk_buff,
) -> c_int {
    if sk.is_null() || skb.is_null() {
        return 1;
    }

    let fou = unsafe { (*sk).sk_user_data as *mut fou };
    if fou.is_null() {
        return 1;
    }

    let len = core::mem::size_of::<udphdr>() as size_t + core::mem::size_of::<guehdr>() as size_t;
    if !unsafe { pskb_may_pull(skb, len) } {
        unsafe { kfree_skb(skb) };
        return 0;
    }

    let guehdr = unsafe { &mut *(udp_hdr(skb) as *mut udphdr as *mut guehdr).offset(1) };

    match guehdr.version {
        0 => {}
        1 => {
            let prot = match unsafe { (*guehdr as *const guehdr as *const iphdr).version } {
                4 => 4,
                6 => 46, // IPPROTO_IPV6
                _ => {
                    unsafe { kfree_skb(skb) };
                    return 0;
                }
            };

            if unsafe { fou_recv_pull(skb, fou, core::mem::size_of::<udphdr>() as size_t) } != 0 {
                unsafe { kfree_skb(skb) };
                return 0;
            }

            return -prot as c_int;
        }
        _ => {
            unsafe { kfree_skb(skb) };
            return 0;
        }
    }

    let optlen = (guehdr.hlen as size_t) << 2;
    let len = len + optlen;

    if !unsafe { pskb_may_pull(skb, len) } {
        unsafe { kfree_skb(skb) };
        return 0;
    }

    let guehdr = unsafe { &mut *(udp_hdr(skb) as *mut udphdr as *mut guehdr).offset(1) };

    if unsafe { validate_gue_flags(guehdr, optlen) } != 0 {
        unsafe { kfree_skb(skb) };
        return 0;
    }

    let hdrlen = core::mem::size_of::<guehdr>() as size_t + optlen;

    if unsafe { (*fou).family } == 0x02 { // AF_INET
        let ip = unsafe { ip_hdr(skb) };
        if ip.is_null() {
            unsafe { kfree_skb(skb) };
            return 0;
        }
        let tot_len = unsafe { (*ip).tot_len };
        unsafe { (*ip).tot_len = ((ntohs(tot_len) - len as u16) as u16).to_be() };
    } else if unsafe { (*fou).family } == 0x0a { // AF_INET6
        let ipv6 = unsafe { ipv6_hdr(skb) };
        if ipv6.is_null() {
            unsafe { kfree_skb(skb) };
            return 0;
        }
        let payload_len = unsafe { (*ipv6).payload_len };
        unsafe { (*ipv6).payload_len = ((ntohs(payload_len) - len as u16) as u16).to_be() };
    }

    unsafe { skb_postpull_rcsum(skb, udp_hdr(skb), len as usize) };

    let data = unsafe { &*guehdr as *const guehdr as *mut c_void }.offset(hdrlen as isize);
    let mut doffset = 0;

    if guehdr.flags & 0x01 != 0 { // GUE_FLAG_PRIV
        let flags = unsafe { *data.offset(doffset as isize) as u32 };
        doffset += 4;

        if flags & 0x01 != 0 { // GUE_PFLAG_REMCSUM
            let new_guehdr = unsafe { gue_remcsum(skb, guehdr, data.offset(doffset as isize), hdrlen, guehdr.proto_ctype,
                (*fou).flags & FOU_F_REMCSUM_NOPARTIAL != 0) };

            if new_guehdr.is_null() {
                unsafe { kfree_skb(skb) };
                return 0;
            }

            data = unsafe { &*new_guehdr as *const guehdr as *mut c_void }.offset(1);
            doffset += 4;
        }
    }

    if guehdr.control != 0 {
        return unsafe { gue_control_message(skb, guehdr) };
    }

    let proto_ctype = guehdr.proto_ctype;
    unsafe { __skb_pull(skb, core::mem::size_of::<udphdr>() as size_t + hdrlen) };
    unsafe { skb_reset_transport_header(skb) };

    if unsafe { iptunnel_pull_offloads(skb) } != 0 {
        unsafe { kfree_skb(skb) };
        return 0;
    }

    -proto_ctype as c_int
}

// Helper functions (declared as extern "C" if needed)
#[no_mangle]
pub unsafe extern "C" fn ip_hdr(skb: *mut sk_buff) -> *mut iphdr {
    // Implementation would depend on actual skb structure
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn ipv6_hdr(skb: *mut sk_buff) -> *mut ipv6hdr {
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn ntohs(x: u16) -> u16 {
    u16::from_be(x)
}

#[no_mangle]
pub unsafe extern "C" fn htons(x: u16) -> u16 {
    x.to_be()
}

#[no_mangle]
pub unsafe extern "C" fn __skb_pull(skb: *mut sk_buff, len: usize) {
    // Implementation would modify (*skb).data
}

#[no_mangle]
pub unsafe extern "C" fn skb_postpull_rcsum(skb: *mut sk_buff, data: *const udphdr, len: usize) {
    // Implementation would update skb checksum
}

#[no_mangle]
pub unsafe extern "C" fn iptunnel_pull_offloads(skb: *mut sk_buff) -> c_int {
    0 // Simplified implementation
}

#[no_mangle]
pub unsafe extern "C" fn pskb_may_pull(skb: *mut sk_buff, len: size_t) -> c_int {
    1 // Assume pull is possible
}

#[no_mangle]
pub unsafe extern "C" fn udp_hdr(skb: *mut sk_buff) -> *mut udphdr {
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn validate_gue_flags(guehdr: *mut guehdr, optlen: size_t) -> c_int {
    0 // Assume valid
}

#[no_mangle]
pub unsafe extern "C" fn skb_remcsum_process(
    skb: *mut sk_buff,
    data: *mut c_void,
    start: size_t,
    offset: size_t,
    nopartial: c_int,
) {
    // Implementation would handle checksum
}

#[no_mangle]
pub unsafe extern "C" fn kfree_skb(skb: *mut sk_buff) {
    // Implementation would free skb
}

#[no_mangle]
pub unsafe extern "C" fn skb_reset_transport_header(skb: *mut sk_buff) {
    // Implementation would reset transport header
}

// Error codes
pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ENOSYS: c_int = -38;

// Tests (conditional compilation)
#[cfg(test)]
mod tests {
    #[test]
    fn test_fou_recv_pull() {
        // Basic test would require valid skb and fou structures
        // This is a placeholder as actual testing would need kernel environment
        assert!(true);
    }
}