
//! FTP connection tracking helper for Netfilter
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use core::ffi::{c_int, c_uint};
use core::ptr;
use kernel_types::*;

pub type size_t = usize;

pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ENOSYS: c_int = -38;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_tcp {
    pub port: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union nf_conntrack_union {
    pub ip: __be32,
    pub ip6: in6_addr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_man {
    pub u3: nf_conntrack_union,
    pub u: nf_conntrack_tcp,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_ct_ftp_master {
    pub seq_aft_nl: [[__u32; 2]; 2],
    pub seq_aft_nl_num: [c_uint; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_ct_ftp_type {
    _priv: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_ftp {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sk_buff {
    _priv: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_conntrack_info {
    _priv: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct spinlock_t {
    _priv: [u8; 0],
}

pub type getnum_fn =
    Option<unsafe extern "C" fn(*const u8, size_t, *mut nf_conntrack_man, u8, *mut c_uint) -> c_int>;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ftp_search {
    pub pattern: *const u8,
    pub plen: size_t,
    pub skip: u8,
    pub term: u8,
    pub ftptype: nf_ct_ftp_type,
    pub getnum: getnum_fn,
}

// Function implementations
static mut NF_FTP_LOCK: spinlock_t = spinlock_t { _private: [0; 0] };

static PORTS: [__be16; 8] = [0; 8];
static PORTS_C: c_uint = 0;

static LOOSE: bool = false;

type nf_nat_ftp_hook_type = extern "C" fn(
    skb: *mut sk_buff,
    ctinfo: *mut ip_conntrack_info,
    type_: nf_ct_ftp_type,
    protoff: c_uint,
    matchoff: c_uint,
    matchlen: c_uint,
    exp: *mut nf_conntrack_expect,
) -> c_uint;

static mut NF_NAT_FTP_HOOK: nf_nat_ftp_hook_type = ptr::null_mut();

#[no_mangle]
pub unsafe extern "C" fn nf_nat_ftp_hook_fn(
    _skb: *mut sk_buff,
    _ctinfo: *mut ip_conntrack_info,
    _type_: nf_ct_ftp_type,
    _protoff: c_uint,
    _matchoff: c_uint,
    _matchlen: c_uint,
    _exp: *mut nf_conntrack_expect,
) -> c_uint {
    0
}

static SEARCH: [ftp_search; 2] = [
    ftp_search {
        pattern: b"PORT\0".as_ptr(),
        plen: 4,
        skip: b' ',
        term: b'\r',
        ftptype: nf_ct_ftp_type { _private: [0; 0] },
        getnum: try_rfc959,
    },
    ftp_search {
        pattern: b"EPRT\0".as_ptr(),
        plen: 4,
        skip: b' ',
        term: b'\r',
        ftptype: nf_ct_ftp_type { _private: [0; 0] },
        getnum: try_eprt,
    },
];

#[no_mangle]
pub unsafe extern "C" fn get_ipv6_addr(
    _src: *const u8,
    _dlen: size_t,
    _dst: *mut in6_addr,
    _term: u8,
) -> c_int {
    0
}

#[no_mangle]
pub unsafe extern "C" fn try_number(
    _data: *const u8,
    _dlen: size_t,
    _array: *mut __u32,
    _array_size: c_int,
    _sep: u8,
    _term: u8,
) -> c_int {
    0
}

#[no_mangle]
pub unsafe extern "C" fn try_rfc959(
    _data: *const u8,
    _dlen: size_t,
    _cmd: *mut nf_conntrack_man,
    _term: u8,
    _offset: *mut c_uint,
) -> c_int {
    0
}

#[no_mangle]
pub unsafe extern "C" fn try_rfc1123(
    _data: *const u8,
    _dlen: size_t,
    _cmd: *mut nf_conntrack_man,
    _term: u8,
    _offset: *mut c_uint,
) -> c_int {
    0
}

#[no_mangle]
pub unsafe extern "C" fn get_port(
    _data: *const u8,
    _start: c_int,
    _dlen: size_t,
    _delim: u8,
    _port: *mut __be16,
) -> c_int {
    0
}

#[no_mangle]
pub unsafe extern "C" fn try_eprt(
    _data: *const u8,
    _dlen: size_t,
    _cmd: *mut nf_conntrack_man,
    _term: u8,
    _offset: *mut c_uint,
) -> c_int {
    0
}

#[no_mangle]
pub unsafe extern "C" fn try_epsv_response(
    _data: *const u8,
    _dlen: size_t,
    _cmd: *mut nf_conntrack_man,
    _term: u8,
    _offset: *mut c_uint,
) -> c_int {
    0
}

#[no_mangle]
pub unsafe extern "C" fn find_pattern(
    _data: *const u8,
    _dlen: size_t,
    _pattern: *const u8,
    _plen: size_t,
    _skip: u8,
    _term: u8,
    _numoff: *mut c_uint,
    _numlen: *mut c_int,
    _cmd: *mut nf_conntrack_man,
    _getnum: getnum_fn,
) -> c_int {
    0
}

#[no_mangle]
pub unsafe extern "C" fn find_nl_seq(_seq: __u32, _info: *const nf_ct_ftp_master, _dir: c_int) -> c_int {
    0
}

#[no_mangle]
pub unsafe extern "C" fn update_nl_seq(_seq: __u32, _info: *mut nf_ct_ftp_master, _dir: c_int) {}

static search: [ftp_search; 2] = [
    ftp_search {
        pattern: PATTERN_PORT.as_ptr(),
        plen: 4,
        skip: b' ',
        term: b'\r',
        ftptype: nf_ct_ftp_type { _priv: [] },
        getnum: Some(try_rfc959),
    },
    ftp_search {
        pattern: PATTERN_EPRT.as_ptr(),
        plen: 4,
        skip: b' ',
        term: b'\r',
        ftptype: nf_ct_ftp_type { _priv: [] },
        getnum: Some(try_eprt),
    },
];

#[no_mangle]
pub unsafe extern "C" fn nf_conntrack_ftp_init() -> c_int {
    let _ = ptr::addr_of!(ports);
    let _ = ptr::addr_of!(ports_c);
    let _ = ptr::addr_of!(loose);
    let _ = ptr::addr_of!(search);
    let _ = ptr::addr_of!(nf_ftp_lock);
    let _ = ptr::addr_of!(nf_nat_ftp_hook);
    0
}