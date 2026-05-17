use kernel_types::*;

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

    let skb = &mut *skb;
    let sk = &mut *sk;

    let udp_len = len as u32;
    let udp_offset = UDP_SKB_CB(skb) as usize;

    // Calculate pseudo-header checksum
    let mut csum = 0u32;
    let saddr = &*saddr;
    let daddr = &*daddr;

    // Add source and destination addresses
    for i in 0..4 {
        csum += u32::from_be(saddr.in6_u.u6_addr32[i]);
        csum += u32::from_be(daddr.in6_u.u6_addr32[i]);
    }

    // Add protocol and length
    csum += proto as u32;
    csum += udp_len;

    // Add UDP header and data
    let udp_data = skb.head as *const u8;
    let udp_data = unsafe { core::slice::from_raw_parts(udp_data, udp_len as usize) };

    let mut i = 0;
    while i < udp_len {
        let word = if i + 1 < udp_len {
            u16::from_be_bytes([udp_data[i as usize], udp_data[(i + 1) as usize]])
        } else {
            u16::from_be_bytes([udp_data[i as usize], 0])
        };
        csum += word as u32;
        i += 2;
    }

    // Fold checksum
    while csum >> 16 != 0 {
        csum = (csum & 0xFFFF) + (csum >> 16);
    }

    let csum = !csum as u16;

    // Store checksum in UDP header
    let udp_header = (skb.head as *mut udphdr).add(udp_offset);
    unsafe {
        (*udp_header).check = csum;
    }

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

    let skb = &*skb;
    let cb = skb.cb.as_ptr() as *const c_void;
    let udp_cb = cb.add(4) as *const c_int;
    unsafe { *udp_cb }
}