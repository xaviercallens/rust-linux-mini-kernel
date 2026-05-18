use kernel_types::*;
use core::ffi::{c_int, c_void};

type socklen_t = u32;

#[repr(C)]
pub struct sk_buff_compat {
    pub data: *mut u8,
    pub len: u32,
    pub cb: [u8; 48],
}

/// Calculate UDP checksum for IPv6 packets
///
/// # Safety
/// - `skb` must be a valid sk_buff pointer
/// - `proto` must be a valid protocol number
/// - `len` must be the correct length of the data
/// - `saddr` and `daddr` must be valid IPv6 addresses
/// - `sk` must be a valid inet_sock pointer
#[no_mangle]
pub unsafe extern "C" fn ip6_udp_checksum(
    skb: *mut sk_buff,
    proto: c_int,
    len: c_int,
    saddr: *const in6_addr,
    daddr: *const in6_addr,
    sk: *mut inet_sock,
) -> c_int {
    if skb.is_null() || saddr.is_null() || daddr.is_null() || sk.is_null() {
        return -EINVAL;
    }

    if proto < 0 || proto > u8::MAX as c_int || len < 0 {
        return -EINVAL;
    }

    let _sk = &mut *sk;
    let skb_compat = &mut *(skb as *mut sk_buff_compat);

    let udp_len: usize = len as usize;
    if udp_len > skb_compat.len as usize {
        return -EINVAL;
    }

    let udp_offset = UDP_SKB_CB(skb);
    if udp_offset < 0 {
        return udp_offset;
    }
    let udp_offset = udp_offset as usize;

    let saddr = &*saddr;
    let daddr = &*daddr;

    let mut csum: u32 = 0;

    for i in 0..4 {
        csum = csum.wrapping_add(u32::from_be(saddr.in6_u.u6_addr32[i]));
        csum = csum.wrapping_add(u32::from_be(daddr.in6_u.u6_addr32[i]));
    }

    csum = csum.wrapping_add((proto as u8) as u32);
    csum = csum.wrapping_add((udp_len as socklen_t) as u32);

    let data_ptr = skb_compat.data as *const u8;
    if data_ptr.is_null() {
        return -EINVAL;
    }

    let udp_data = core::slice::from_raw_parts(data_ptr, udp_len);

    let mut i = 0usize;
    while i < udp_len {
        let word = if i + 1 < udp_len {
            u16::from_be_bytes([udp_data[i], udp_data[i + 1]])
        } else {
            u16::from_be_bytes([udp_data[i], 0])
        };
        csum = csum.wrapping_add(word as u32);
        i += 2;
    }

    while (csum >> 16) != 0 {
        csum = (csum & 0xFFFF) + (csum >> 16);
    }

    let check = !(csum as u16);

    if udp_offset.checked_add(core::mem::size_of::<udphdr>()).is_none()
        || udp_offset + core::mem::size_of::<udphdr>() > skb_compat.len as usize
    {
        return -EINVAL;
    }

    let udp_header = (skb_compat.data as *mut udphdr).add(udp_offset);
    (*udp_header).check = check;

    0
}

/// Calculate offset for UDP control block in sk_buff
///
/// # Safety
/// - `skb` must be a valid sk_buff pointer
#[no_mangle]
pub unsafe extern "C" fn UDP_SKB_CB(skb: *mut sk_buff) -> c_int {
    if skb.is_null() {
        return -EINVAL;
    }

    let skb_compat = &*(skb as *const sk_buff_compat);
    let cb = skb_compat.cb.as_ptr() as *const c_void;
    let udp_cb = (cb as *const u8).add(4) as *const c_int;
    *udp_cb
}