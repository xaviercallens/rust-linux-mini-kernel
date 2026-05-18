#![no_std]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;
use kernel_types::*;

pub const IPPROTO_DSTOPTS: c_int = 60;
pub const IPPROTO_ROUTING: c_int = 43;
pub const IPPROTO_MH: c_int = 135;
pub const IPPROTO_NONE: c_int = 59;
pub const NEXTHDR_HOP: c_int = 0;
pub const NEXTHDR_ROUTING: c_int = 43;
pub const NEXTHDR_DEST: c_int = 60;
pub const IP6_MH_TYPE_BRR: c_int = 0;
pub const IP6_MH_TYPE_HOTI: c_int = 1;
pub const IP6_MH_TYPE_COTI: c_int = 2;
pub const IP6_MH_TYPE_BU: c_int = 3;
pub const IP6_MH_TYPE_BACK: c_int = 4;
pub const IP6_MH_TYPE_HOT: c_int = 5;
pub const IP6_MH_TYPE_COT: c_int = 6;
pub const IP6_MH_TYPE_BERROR: c_int = 7;
pub const IP6_MH_TYPE_MAX: c_int = 15;
pub const XFRM_MODE_ROUTEOPTIMIZATION: c_int = 5;
pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ENOENT: c_int = -2;
pub const IPV6_TLV_PAD1: u8 = 0x00;
pub const IPV6_TLV_PADN: u8 = 0x01;
pub const IPV6_TLV_HAO: u8 = 0x08;

pub const XFRM_TYPE_NON_FRAGMENT: c_int = 1 << 0;
pub const XFRM_TYPE_LOCAL_COADDR: c_int = 1 << 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct spinlock_t {
    _priv: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct module {
    _priv: u32,
}

unsafe extern "C" {
    static THIS_MODULE: module;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip6_mh {
    pub ip6mh_type: u8,
    pub ip6mh_hdrlen: u8,
    pub ip6mh_proto: u8,
    pub ip6mh_reserved: [u8; 5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipv6_destopt_hdr {
    pub nexthdr: u8,
    pub hdrlen: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipv6_destopt_hao {
    pub type_: u8,
    pub length: u8,
    pub addr: in6_addr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt2_hdr {
    pub rt_hdr: ipv6_destopt_hdr,
    pub segments_left: u32,
    pub reserved: [u32; 3],
    pub addr: in6_addr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_id {
    pub spi: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_props {
    pub mode: c_int,
    pub header_len: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_state {
    pub id: xfrm_id,
    pub props: xfrm_props,
    pub coaddr: in6_addr,
    pub lock: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_type {
    pub description: *const c_char,
    pub owner: *const module,
    pub proto: c_int,
    pub flags: c_int,
    pub init_state: extern "C" fn(*mut xfrm_state) -> c_int,
    pub destructor: extern "C" fn(*mut xfrm_state),
    pub input: extern "C" fn(*mut xfrm_state, *mut c_void) -> c_int,
    pub output: extern "C" fn(*mut xfrm_state, *mut c_void) -> c_int,
    pub reject: extern "C" fn(*mut xfrm_state, *mut c_void, *const c_void) -> c_int,
    pub hdr_offset: extern "C" fn(*mut xfrm_state, *mut c_void, *mut *mut u8) -> c_int,
}

unsafe impl Sync for xfrm_type {}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mip6_report_rate_limiter {
    pub lock: spinlock_t,
    pub stamp: u64,
    pub iif: c_int,
    pub src: in6_addr,
    pub dst: in6_addr,
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn calc_padlen(len: c_uint, n: c_uint) -> c_uint {
    (n.wrapping_sub(len) + 16) & 0x7
}

#[no_mangle]
pub unsafe extern "C" fn mip6_padn(data: *mut u8, padlen: c_uint) -> *mut u8 {
    if data.is_null() {
        return ptr::null_mut();
    }

    if padlen == 1 {
        ptr::write(data, IPV6_TLV_PAD1);
    } else if padlen > 1 {
        ptr::write(data, IPV6_TLV_PADN);
        ptr::write(data.add(1), (padlen - 2) as u8);
        if padlen > 2 {
            ptr::write_bytes(data.add(2), 0, (padlen - 2) as usize);
        }
    }

    data.add(padlen as usize)
}

#[no_mangle]
pub unsafe extern "C" fn mip6_param_prob(_skb: *mut sk_buff, _code: u8, _pos: c_int) {}

#[no_mangle]
pub extern "C" fn mip6_mh_len(type_: c_int) -> c_int {
    match type_ {
        IP6_MH_TYPE_BRR => 0,
        IP6_MH_TYPE_HOTI | IP6_MH_TYPE_COTI | IP6_MH_TYPE_BU | IP6_MH_TYPE_BACK => 1,
        IP6_MH_TYPE_HOT | IP6_MH_TYPE_COT | IP6_MH_TYPE_BERROR => 2,
        _ => 0,
    }
}

#[no_mangle]
pub extern "C" fn mip6_destopt_init_state(_x: *mut xfrm_state) -> c_int {
    0
}

#[no_mangle]
pub extern "C" fn mip6_destopt_destroy(_x: *mut xfrm_state) {}

#[no_mangle]
pub extern "C" fn mip6_destopt_input(_x: *mut xfrm_state, _skb: *mut c_void) -> c_int {
    0
}

#[no_mangle]
pub extern "C" fn mip6_destopt_output(_x: *mut xfrm_state, _skb: *mut c_void) -> c_int {
    0
}

#[no_mangle]
pub extern "C" fn mip6_destopt_reject(
    _x: *mut xfrm_state,
    _skb: *mut c_void,
    _tmpl: *const c_void,
) -> c_int {
    0
}

#[no_mangle]
pub extern "C" fn mip6_destopt_hdr_offset(
    _x: *mut xfrm_state,
    _skb: *mut c_void,
    _nexthdr: *mut *mut u8,
) -> c_int {
    0
}

static MIP6_DESTOPT_DESC: &[u8] = b"mip6-destopt\0";

#[no_mangle]
pub static mip6_destopt_type: xfrm_type = xfrm_type {
    description: MIP6_DESTOPT_DESC.as_ptr() as *const c_char,
    owner: unsafe { &THIS_MODULE as *const module },
    proto: IPPROTO_DSTOPTS,
    flags: XFRM_TYPE_NON_FRAGMENT | XFRM_TYPE_LOCAL_COADDR,
    init_state: mip6_destopt_init_state,
    destructor: mip6_destopt_destroy,
    input: mip6_destopt_input,
    output: mip6_destopt_output,
    reject: mip6_destopt_reject,
    hdr_offset: mip6_destopt_hdr_offset,
};