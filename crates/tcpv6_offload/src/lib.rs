use kernel_types::*;
use core::ffi::{c_int, c_void};
use core::ptr;

pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;

type netdev_features_t = u64;
type socklen_t = u32;
type c_size_t = usize;

#[repr(C)]
pub struct tcphdr {
    pub check: u16,
}

#[repr(C)]
struct napi_gro_cb {
    flush: u8,
}

#[repr(C)]
struct net_offload {
    callbacks: net_offload_callbacks,
}

#[repr(C)]
struct net_offload_callbacks {
    gso_segment: Option<extern "C" fn(*mut sk_buff, netdev_features_t) -> *mut sk_buff>,
    gro_receive: Option<extern "C" fn(*mut c_void, *mut sk_buff) -> *mut sk_buff>,
    gro_complete: Option<extern "C" fn(*mut sk_buff, c_int) -> c_int>,
}

#[repr(C)]
struct skb_shared_info {
    gso_type: netdev_features_t,
}

const IPPROTO_TCP: c_int = 6;
const SKB_GSO_TCPV6: netdev_features_t = 0x0000_0800;

#[no_mangle]
pub unsafe extern "C" fn tcp6_gro_receive(head: *mut c_void, skb: *mut sk_buff) -> *mut sk_buff {
    let cb = napi_gro_cb_ptr(skb);

    if (*cb).flush == 0 && skb_gro_checksum_validate(skb, IPPROTO_TCP, ip6_gro_compute_pseudo) != 0 {
        (*cb).flush = 1;
        return ptr::null_mut();
    }

    tcp_gro_receive(head, skb)
}

#[no_mangle]
pub unsafe extern "C" fn tcp6_gro_complete(skb: *mut sk_buff, _thoff: c_int) -> c_int {
    let iph = ipv6_hdr(skb);
    let th = tcp_hdr(skb);

    (*th).check = !tcp_v6_check(skb_len(skb), &(*iph).saddr, &(*iph).daddr, 0) as u16;
    (*skb_shinfo(skb)).gso_type |= SKB_GSO_TCPV6;

    tcp_gro_complete(skb)
}

#[no_mangle]
pub unsafe extern "C" fn tcp6_gso_segment(
    skb: *mut sk_buff,
    features: netdev_features_t,
) -> *mut sk_buff {
    let shinfo = skb_shinfo(skb);

    if ((*shinfo).gso_type & SKB_GSO_TCPV6) == 0 {
        return err_ptr(EINVAL);
    }

    if !pskb_may_pull(skb, core::mem::size_of::<tcphdr>()) {
        return err_ptr(EINVAL);
    }

    if !skb_is_checksum_partial(skb) {
        let ipv6h = ipv6_hdr(skb);
        let _th = tcp_hdr(skb);

        skb_set_checksum_partial(skb);
        __tcp_v6_send_check(skb, &(*ipv6h).saddr, &(*ipv6h).daddr);
    }

    tcp_gso_segment(skb, features)
}

#[no_mangle]
pub extern "C" fn tcpv6_offload_init() -> c_int {
    unsafe { inet6_add_offload(&TCPV6_OFFLOAD, IPPROTO_TCP) }
}

#[inline]
unsafe fn napi_gro_cb_ptr(skb: *mut sk_buff) -> *mut napi_gro_cb {
    let offset = 128usize;
    (skb as *mut u8).add(offset) as *mut napi_gro_cb
}

#[inline]
unsafe fn skb_gro_checksum_validate(
    _skb: *mut sk_buff,
    _proto: c_int,
    _pseudo: unsafe extern "C" fn(*mut sk_buff) -> c_int,
) -> c_int {
    0
}

#[inline]
unsafe fn ipv6_hdr(skb: *mut sk_buff) -> *mut ipv6hdr {
    skb_network_header(skb) as *mut ipv6hdr
}

#[inline]
unsafe fn tcp_hdr(skb: *mut sk_buff) -> *mut tcphdr {
    skb_transport_header(skb) as *mut tcphdr
}

#[inline]
unsafe fn skb_shinfo(skb: *mut sk_buff) -> *mut skb_shared_info {
    let offset = 256usize;
    (skb as *mut u8).add(offset) as *mut skb_shared_info
}

#[inline]
unsafe fn err_ptr(error: c_int) -> *mut sk_buff {
    error as isize as *mut sk_buff
}

#[inline]
unsafe fn pskb_may_pull(_skb: *mut sk_buff, _len: usize) -> bool {
    true
}

#[inline]
unsafe fn __tcp_v6_send_check(_skb: *mut sk_buff, _saddr: *const in6_addr, _daddr: *const in6_addr) {}

#[inline]
unsafe fn tcp_v6_check(
    _len: u32,
    _saddr: *const in6_addr,
    _daddr: *const in6_addr,
    _old_checksum: u32,
) -> u32 {
    0
}

#[inline]
unsafe extern "C" fn ip6_gro_compute_pseudo(_skb: *mut sk_buff) -> c_int {
    0
}

#[inline]
unsafe fn skb_network_header(_skb: *mut sk_buff) -> *mut c_void {
    ptr::null_mut()
}

#[inline]
unsafe fn skb_transport_header(_skb: *mut sk_buff) -> *mut c_void {
    ptr::null_mut()
}

#[inline]
unsafe fn skb_is_checksum_partial(_skb: *mut sk_buff) -> bool {
    true
}

#[inline]
unsafe fn skb_set_checksum_partial(_skb: *mut sk_buff) {}

#[inline]
unsafe fn skb_len(_skb: *mut sk_buff) -> u32 {
    0
}

#[inline]
unsafe extern "C" fn tcp_gro_receive(_head: *mut c_void, _skb: *mut sk_buff) -> *mut sk_buff {
    ptr::null_mut()
}

#[inline]
unsafe extern "C" fn tcp_gro_complete(_skb: *mut sk_buff) -> c_int {
    0
}

#[inline]
unsafe extern "C" fn tcp_gso_segment(_skb: *mut sk_buff, _features: netdev_features_t) -> *mut sk_buff {
    ptr::null_mut()
}

#[inline]
unsafe fn inet6_add_offload(_offload: *const net_offload, _protocol: c_int) -> c_int {
    0
}

#[no_mangle]
static TCPV6_OFFLOAD: net_offload = net_offload {
    callbacks: net_offload_callbacks {
        gso_segment: Some(tcp6_gso_segment),
        gro_receive: Some(tcp6_gro_receive),
        gro_complete: Some(tcp6_gro_complete),
    },
};