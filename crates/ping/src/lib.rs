#![no_std]
#![no_main]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use core::ffi::{c_int, c_void};
use core::panic::PanicInfo;
use kernel_types::*;

pub type size_t = usize;
pub type c_size_t = usize;
pub type socklen_t = u32;

pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ENOSYS: c_int = -38;
pub const EAFNOSUPPORT: c_int = -125;
pub const EDESTADDRREQ: c_int = -39;

// Type definitions

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_in6 {
    pub sin6_family: u16,
    pub sin6_port: u16,
    pub sin6_flowinfo: u32,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icmp6_echo {
    pub id: u16,
    pub sequence: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icmp6hdr {
    pub icmp6_type: u8,
    pub icmp6_code: u8,
    pub checksum: u16,
    pub un: icmp6_echo,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pingfakehdr {
    pub icmph: icmp6hdr,
    pub msg: *mut msghdr,
    pub wcheck: u16,
    pub family: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct proto {
    pub name: *const u8,
    pub owner: *mut c_void,
    pub init: extern "C" fn(*mut sock) -> c_int,
    pub close: extern "C" fn(*mut sock, c_int),
    pub connect: extern "C" fn(*mut sock, *const sockaddr_in6, socklen_t, c_int) -> c_int,
    pub disconnect: extern "C" fn(*mut sock, c_int),
    pub setsockopt: extern "C" fn(*mut sock, c_int, c_int, *const c_void, socklen_t) -> c_int,
    pub getsockopt: extern "C" fn(*mut sock, c_int, c_int, *mut c_void, *mut socklen_t) -> c_int,
    pub sendmsg: extern "C" fn(*mut sock, *mut msghdr, size_t) -> c_int,
    pub recvmsg: extern "C" fn(*mut sock, *mut msghdr, size_t, c_int) -> c_int,
    pub bind: extern "C" fn(*mut sock, *const sockaddr_in6, socklen_t) -> c_int,
    pub backlog_rcv: extern "C" fn(*mut sock, *mut sk_buff) -> c_int,
    pub hash: extern "C" fn(*mut sock),
    pub unhash: extern "C" fn(*mut sock),
    pub get_port: extern "C" fn(*mut sock, u16) -> c_int,
    pub obj_size: size_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inet_protosw {
    pub type_: c_int,
    pub protocol: c_int,
    pub prot: *mut proto,
    pub ops: *mut c_void,
    pub flags: c_int,
}

#[repr(C)]
pub struct msghdr {
    _priv: [u8; 0],
}

#[repr(C)]
pub struct net_device {
    _priv: [u8; 0],
}

#[repr(C)]
pub struct net {
    _priv: [u8; 0],
}

#[no_mangle]
pub unsafe extern "C" fn dummy_ipv6_recv_error(
    _sk: *mut sock,
    _msg: *mut msghdr,
    _len: c_int,
    _addr_len: *mut c_int,
) -> c_int {
    EAFNOSUPPORT
}

#[no_mangle]
pub unsafe extern "C" fn dummy_ip6_datagram_recv_ctl(
    _sk: *mut sock,
    _msg: *mut msghdr,
    _skb: *mut sk_buff,
) {
}

#[no_mangle]
pub unsafe extern "C" fn dummy_icmpv6_err_convert(
    _type_: u8,
    _code: u8,
    _err: *mut c_int,
) -> c_int {
    EAFNOSUPPORT
}

#[no_mangle]
pub unsafe extern "C" fn dummy_ipv6_icmp_error(
    _sk: *mut sock,
    _skb: *mut sk_buff,
    _err: c_int,
    _port: u16,
    _info: u32,
    _payload: *mut u8,
) {
}

#[no_mangle]
pub unsafe extern "C" fn dummy_ipv6_chk_addr(
    _net: *mut net,
    _addr: *const in6_addr,
    _dev: *const net_device,
    _strict: c_int,
) -> c_int {
    0
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}