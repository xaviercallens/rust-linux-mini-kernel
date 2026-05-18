#![no_std]
#![no_main]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use core::ffi::{c_int, c_void};
use core::ptr;

mod kernel_types {
    pub use core::ffi::{
        c_char, c_int, c_long, c_short, c_uchar, c_uint, c_ulong, c_ushort, c_void,
    };
    pub type size_t = usize;
    pub type c_size_t = usize;
    pub type socklen_t = u32;
}
use kernel_types::*;

pub const IPPROTO_UDP: c_int = 17;
pub const NEXTHDR_FRAGMENT: u8 = 44;
pub const CSUM_MANGLED_0: u16 = 0xbad0;
pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 256],
}

#[repr(C)]
pub struct skb_shared_info {
    pub gso_size: u16,
    _private: [u8; 128],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipv6hdr {
    pub saddr: [u8; 16],
    pub daddr: [u8; 16],
    _padding: [u8; 40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct udphdr {
    pub source: u16,
    pub dest: u16,
    pub len: u16,
    pub check: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct frag_hdr {
    pub nexthdr: u8,
    pub reserved: u8,
    pub frag_off: u16,
    pub identification: u32,
}

#[repr(C)]
pub struct net_offload {
    pub callbacks: net_offload_callbacks,
}

#[repr(C)]
pub struct net_offload_callbacks {
    pub gso_segment: extern "C" fn(skb: *mut sk_buff, features: u32) -> *mut sk_buff,
    pub gro_receive: extern "C" fn(head: *mut c_void, skb: *mut sk_buff) -> *mut sk_buff,
    pub gro_complete: extern "C" fn(skb: *mut sk_buff, nhoff: c_int) -> c_int,
}

#[repr(C)]
pub struct NapiGroCb {
    pub flush: c_int,
    pub is_ipv6: c_int,
    pub is_flist: c_int,
    pub encap_mark: c_int,
    pub mac_offset: isize,
    pub count: u16,
}

#[inline]
unsafe fn NAPI_GRO_CB(_skb: *mut sk_buff) -> &'static mut NapiGroCb {
    static mut CB: NapiGroCb = NapiGroCb {
        flush: 0,
        is_ipv6: 0,
        is_flist: 0,
        encap_mark: 0,
        mac_offset: 0,
        count: 0,
    };
    unsafe { &mut CB }
}

#[repr(C)]
pub struct udp_table_t {
    _private: [u8; 1],
}

static mut udp_table: udp_table_t = udp_table_t { _private: [0] };
static mut udpv6_encap_needed_key: c_int = 0;

unsafe fn skb_shinfo(_skb: *mut sk_buff) -> *mut skb_shared_info {
    static mut SHINFO: skb_shared_info = skb_shared_info {
        gso_size: 0,
        _private: [0; 128],
    };
    unsafe { &mut SHINFO }
}

unsafe fn skb_gro_network_header(_skb: *mut sk_buff) -> *mut ipv6hdr {
    ptr::null_mut()
}
unsafe fn dev_net(_dev: *mut c_void) -> *mut c_void {
    ptr::null_mut()
}
unsafe fn inet6_iif(_skb: *mut sk_buff) -> c_int {
    0
}
unsafe fn inet6_sdif(_skb: *mut sk_buff) -> c_int {
    0
}
unsafe fn __udp6_lib_lookup(
    _net: *mut c_void,
    _saddr: *const [u8; 16],
    _sport: u16,
    _daddr: *const [u8; 16],
    _dport: u16,
    _iif: c_int,
    _sdif: c_int,
    _table: *mut udp_table_t,
    _udm: *mut c_void,
) -> *mut c_void {
    ptr::null_mut()
}
unsafe fn udp_gro_udphdr(_skb: *mut sk_buff) -> *mut udphdr {
    ptr::null_mut()
}
unsafe fn skb_gro_checksum_validate_zero_check(
    _skb: *mut sk_buff,
    _proto: c_int,
    _check: u16,
    _pseudo: unsafe extern "C" fn(*mut sk_buff, *mut c_void) -> c_int,
) -> c_int {
    0
}
unsafe fn skb_gro_checksum_try_convert(
    _skb: *mut sk_buff,
    _proto: c_int,
    _pseudo: unsafe extern "C" fn(*mut sk_buff, *mut c_void) -> c_int,
) {
}
unsafe extern "C" fn ip6_gro_compute_pseudo(_skb: *mut sk_buff, _data: *mut c_void) -> c_int {
    0
}
unsafe fn rcu_read_lock() {}
unsafe fn rcu_read_unlock() {}
unsafe fn static_branch_unlikely(_key: *const c_int) -> c_int {
    0
}
unsafe fn udp_gro_receive(
    _head: *mut c_void,
    skb: *mut sk_buff,
    _uh: *mut udphdr,
    _offload: *mut c_void,
) -> *mut sk_buff {
    skb
}
unsafe fn ipv6_hdr(_skb: *mut sk_buff) -> *mut ipv6hdr {
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn udp6_ufo_fragment(skb: *mut sk_buff, _features: u32) -> *mut sk_buff {
    let _mss = unsafe { (*skb_shinfo(skb)).gso_size };
    skb
}

#[no_mangle]
pub unsafe extern "C" fn udp6_gro_lookup_skb(
    skb: *mut sk_buff,
    sport: u16,
    dport: u16,
) -> *mut c_void {
    let iph = unsafe { skb_gro_network_header(skb) };
    if iph.is_null() {
        return ptr::null_mut();
    }
    unsafe {
        __udp6_lib_lookup(
            dev_net(ptr::null_mut()),
            &(*iph).saddr,
            sport,
            &(*iph).daddr,
            dport,
            inet6_iif(skb),
            inet6_sdif(skb),
            &mut udp_table,
            ptr::null_mut(),
        )
    }
}

#[no_mangle]
pub unsafe extern "C" fn udp6_gro_receive(head: *mut c_void, skb: *mut sk_buff) -> *mut sk_buff {
    let uh = unsafe { udp_gro_udphdr(skb) };
    if uh.is_null() {
        unsafe { NAPI_GRO_CB(skb).flush = 1 };
        return ptr::null_mut();
    }

    if unsafe { NAPI_GRO_CB(skb).flush } != 0 {
        return ptr::null_mut();
    }

    if unsafe { skb_gro_checksum_validate_zero_check(skb, IPPROTO_UDP, (*uh).check, ip6_gro_compute_pseudo) } != 0 {
        unsafe { NAPI_GRO_CB(skb).flush = 1 };
        return ptr::null_mut();
    }

    if unsafe { (*uh).check } != 0 {
        unsafe { skb_gro_checksum_try_convert(skb, IPPROTO_UDP, ip6_gro_compute_pseudo) };
    }

    unsafe { udp_gro_receive(head, skb, uh, ptr::null_mut()) }
}

#[no_mangle]
pub unsafe extern "C" fn udp6_gro_complete(_skb: *mut sk_buff, _nhoff: c_int) -> c_int {
    0
}

#[no_mangle]
pub static udp6_offload: net_offload = net_offload {
    callbacks: net_offload_callbacks {
        gso_segment: udp6_ufo_fragment,
        gro_receive: udp6_gro_receive,
        gro_complete: udp6_gro_complete,
    },
};

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}