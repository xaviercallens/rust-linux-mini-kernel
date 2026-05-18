#![no_std]
#![no_main]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::all)]

use core::ffi::{c_int, c_uchar};
use core::panic::PanicInfo;
use kernel_types::*;

pub const AF_INET6: c_int = 10;

pub const IPV6_ADDR_ANY: c_int = 0;
pub const IPV6_ADDR_MAPPED: c_int = 1;
pub const SK_CAN_REUSE: c_int = 2;
pub const TCP_LISTEN: c_int = 1;
pub const TCP_TIME_WAIT: c_int = 7;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inet_bind_bucket {
    pub port: u16,
    pub l3mdev: c_int,
    pub fastreuseport: i32,
    pub fastuid: u32,
    pub fast_rcv_saddr: u32,
    pub fast_ipv6_only: c_uchar,
    pub fast_sk_family: c_int,
    pub fast_v6_rcv_saddr: in6_addr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inet_hashinfo {
    pub bhash_size: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct net {
    pub ipv4: ipv4_net,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipv4_net {
    pub ip_local_ports: seqlock,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct seqlock {
    pub lock: spinlock,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct spinlock {
    pub _priv: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sock_common {
    pub skc_family: u16,
    pub skc_rcv_saddr: u32,
    pub skc_v6_rcv_saddr: in6_addr,
}

unsafe extern "C" {
    fn ipv6_addr_type(addr: *const in6_addr) -> c_int;
    fn ipv6_addr_equal(a: *const in6_addr, b: *const in6_addr) -> c_uchar;
    fn ipv6_addr_any(addr: *const in6_addr) -> c_uchar;
    fn inet_is_local_reserved_port(net: *mut net, port: c_int) -> c_uchar;
    fn read_seqbegin(seq: *mut seqlock) -> c_int;
    fn read_seqretry(seq: *mut seqlock, start: c_int) -> c_uchar;
    fn prandom_u32() -> u32;
    fn cond_resched();

    fn inet6_rcv_saddr(sk: *const sock) -> *const in6_addr;
    fn ipv6_only_sock(sk: *const sock) -> c_uchar;
}

#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    loop {}
}

#[inline]
fn cbool(v: bool) -> c_uchar {
    if v { 1 } else { 0 }
}

#[inline]
fn rbool(v: c_uchar) -> bool {
    v != 0
}

#[inline]
unsafe fn sk_common(sk: *const sock) -> *const sock_common {
    sk.cast::<sock_common>()
}

#[inline]
unsafe fn sk_family(sk: *const sock) -> c_int {
    (*sk_common(sk)).skc_family as c_int
}

#[inline]
unsafe fn sk_rcv_saddr(sk: *const sock) -> u32 {
    (*sk_common(sk)).skc_rcv_saddr
}

#[inline]
unsafe fn sk_v6_rcv_saddr(sk: *const sock) -> *const in6_addr {
    &(*sk_common(sk)).skc_v6_rcv_saddr
}

#[no_mangle]
pub unsafe extern "C" fn ipv6_rcv_saddr_equal(
    sk1_rcv_saddr6: *const in6_addr,
    sk2_rcv_saddr6: *const in6_addr,
    sk1_rcv_saddr: u32,
    sk2_rcv_saddr: u32,
    sk1_ipv6only: c_uchar,
    sk2_ipv6only: c_uchar,
    match_sk1_wildcard: c_uchar,
    match_sk2_wildcard: c_uchar,
) -> c_uchar {
    if sk1_rcv_saddr6.is_null() || sk2_rcv_saddr6.is_null() {
        return 0;
    }

    let addr_type = ipv6_addr_type(sk1_rcv_saddr6);
    let addr_type2 = ipv6_addr_type(sk2_rcv_saddr6);

    if addr_type == IPV6_ADDR_MAPPED && addr_type2 == IPV6_ADDR_MAPPED {
        if !rbool(sk2_ipv6only) {
            if sk1_rcv_saddr == sk2_rcv_saddr {
                return 1;
            }
            return cbool(
                (rbool(match_sk1_wildcard) && sk1_rcv_saddr == 0)
                    || (rbool(match_sk2_wildcard) && sk2_rcv_saddr == 0),
            );
        }
        return 0;
    }

    if addr_type == IPV6_ADDR_ANY && addr_type2 == IPV6_ADDR_ANY {
        return 1;
    }

    if addr_type2 == IPV6_ADDR_ANY
        && rbool(match_sk2_wildcard)
        && !(rbool(sk2_ipv6only) && addr_type == IPV6_ADDR_MAPPED)
    {
        return 1;
    }

    if addr_type == IPV6_ADDR_ANY
        && rbool(match_sk1_wildcard)
        && !(rbool(sk1_ipv6only) && addr_type2 == IPV6_ADDR_MAPPED)
    {
        return 1;
    }

    if rbool(ipv6_addr_equal(sk1_rcv_saddr6, sk2_rcv_saddr6)) {
        return 1;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn ipv4_rcv_saddr_equal(
    sk1_rcv_saddr: u32,
    sk2_rcv_saddr: u32,
    sk2_ipv6only: c_uchar,
    match_sk1_wildcard: c_uchar,
    match_sk2_wildcard: c_uchar,
) -> c_uchar {
    if !rbool(sk2_ipv6only) {
        if sk1_rcv_saddr == sk2_rcv_saddr {
            return 1;
        }
        return cbool(
            (rbool(match_sk1_wildcard) && sk1_rcv_saddr == 0)
                || (rbool(match_sk2_wildcard) && sk2_rcv_saddr == 0),
        );
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn inet_rcv_saddr_equal(
    sk: *const sock,
    sk2: *const sock,
    match_wildcard: c_uchar,
) -> c_uchar {
    if sk.is_null() || sk2.is_null() {
        return 0;
    }

    if sk_family(sk) == AF_INET6 {
        let sk2_rcv_saddr6 = inet6_rcv_saddr(sk2);
        return ipv6_rcv_saddr_equal(
            sk_v6_rcv_saddr(sk),
            sk2_rcv_saddr6,
            sk_rcv_saddr(sk),
            sk_rcv_saddr(sk2),
            ipv6_only_sock(sk),
            ipv6_only_sock(sk2),
            match_wildcard,
            match_wildcard,
        );
    }

    ipv4_rcv_saddr_equal(
        sk_rcv_saddr(sk),
        sk_rcv_saddr(sk2),
        ipv6_only_sock(sk2),
        match_wildcard,
        match_wildcard,
    )
}