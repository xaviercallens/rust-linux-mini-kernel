#![no_std]
#![no_main]

use core::ffi::c_void;
use kernel_types::*;

#[repr(C)]
pub struct nf_conn {
    _priv: [u8; 0],
}

#[repr(C)]
pub struct net_device {
    _priv: [u8; 0],
}

#[repr(C)]
pub struct notifier_block {
    _priv: [u8; 0],
}

#[repr(C)]
pub struct work_struct {
    _priv: [u8; 0],
}

#[repr(C)]
pub struct sk_buff {
    _priv: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct in_addr {
    pub s_addr: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct in6_addr {
    pub s6_addr: [u8; 16],
}

#[repr(C)]
pub union nf_inet_addr {
    pub ip: in_addr,
    pub ip6: in6_addr,
    pub all: [u32; 4],
}

pub type socklen_t = u32;
pub type c_size_t = usize;

const NOTIFY_DONE: c_int = 0;
const NF_ACCEPT: c_uint = 1;

#[no_mangle]
pub extern "C" fn rust_eh_personality() {}

#[no_mangle]
pub extern "C" fn nf_nat_masquerade_init() -> c_int {
    0
}

#[no_mangle]
pub extern "C" fn nf_nat_masquerade_fini() {}

#[no_mangle]
pub extern "C" fn nf_nat_masquerade_inet_register_notifier(_nb: *mut notifier_block) -> c_int {
    0
}

#[no_mangle]
pub extern "C" fn nf_nat_masquerade_inet_unregister_notifier(_nb: *mut notifier_block) -> c_int {
    0
}

#[no_mangle]
pub extern "C" fn nf_nat_masquerade_ipv4(
    _skb: *mut sk_buff,
    _ct: *mut nf_conn,
    _new_src: *const in_addr,
) -> c_uint {
    NF_ACCEPT
}

#[no_mangle]
pub extern "C" fn nf_nat_masquerade_ipv6(
    _skb: *mut sk_buff,
    _ct: *mut nf_conn,
    _new_src: *const in6_addr,
) -> c_uint {
    NF_ACCEPT
}

#[no_mangle]
pub extern "C" fn nf_nat_masquerade_device_event(
    _this: *mut notifier_block,
    _event: c_ulong,
    _ptr: *mut c_void,
) -> c_int {
    NOTIFY_DONE
}

#[no_mangle]
pub extern "C" fn nf_nat_masquerade_schedule(_ct: *mut nf_conn, _wq: *mut work_struct) -> c_int {
    0
}