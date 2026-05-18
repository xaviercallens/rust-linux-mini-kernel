#![no_std]
#![no_main]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use core::ffi::c_int;
use core::mem;
use core::panic::PanicInfo;
use kernel_types::*;

pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ENOSYS: c_int = -38;
pub const AF_INET6: c_int = 10;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct refcount_t {
    pub counter: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union in6_addr_kcompat {
    pub u6_addr32: [u32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct in6_addr {
    pub in6_u: in6_addr_kcompat,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct udp_hslot {
    pub head: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct udp_table {
    pub mask: c_int,
    pub hash2: *mut udp_hslot,
}

#[repr(C)]
pub struct net {
    _priv: [u8; 0],
}

#[repr(C)]
pub struct sk_buff {
    _priv: [u8; 0],
}

#[repr(C)]
pub struct ipv6hdr {
    _priv: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inet_sock {
    pub inet_dport: u16,
}

#[repr(C)]
pub struct sock {
    pub sk_v6_rcv_saddr: in6_addr,
    pub sk_v6_daddr: in6_addr,
    pub sk_family: c_int,
    pub udp_port_hash: u16,
    pub udp_portaddr_hash: u32,
    pub inet_num: u16,
    pub sk_bound_dev_if: c_int,
    pub sk_incoming_cpu: c_int,
    pub inet_sk: inet_sock,
}

unsafe extern "C" {
    fn net_get_random_once(buf: *mut u32, size: usize);
    fn ipv6_portaddr_hash(net: *const net, addr: *const in6_addr, port: u16) -> u32;
    fn __inet6_ehashfn(lhash: u32, lport: u16, fhash: u32, fport: u16, secret: u32) -> u32;
    fn net_hash_mix(net: *const net) -> u32;

    fn sock_net(sk: *const sock) -> *const net;
    fn udp_lib_get_port(sk: *mut sock, snum: u16, hash2_nulladdr: u32) -> c_int;
    fn udp_lib_rehash(sk: *mut sock, new_hash: u32);

    fn net_eq(a: *const net, b: *const net) -> bool;
    fn ipv6_addr_equal(a1: *const in6_addr, a2: *const in6_addr) -> bool;
    fn ipv6_addr_any(a: *const in6_addr) -> bool;
    fn udp_sk_bound_dev_eq(net: *const net, bound_dev_if: c_int, dif: c_int, sdif: c_int) -> bool;
    fn raw_smp_processor_id() -> c_int;
}

#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    loop {}
}

#[no_mangle]
pub static in6addr_any: in6_addr = in6_addr {
    in6_u: in6_addr_kcompat {
        u6_addr32: [0, 0, 0, 0],
    },
};

#[no_mangle]
pub unsafe extern "C" fn udp6_ehashfn(
    net: *const net,
    laddr: *const in6_addr,
    lport: u16,
    faddr: *const in6_addr,
    fport: u16,
) -> u32 {
    static mut UDP6_EHASH_SECRET: u32 = 0;
    static mut UDP_IPV6_HASH_SECRET: u32 = 0;

    net_get_random_once(&raw mut UDP6_EHASH_SECRET, mem::size_of::<u32>());
    net_get_random_once(&raw mut UDP_IPV6_HASH_SECRET, mem::size_of::<u32>());

    let lhash = (*laddr).in6_u.u6_addr32[3];
    let fhash = ipv6_portaddr_hash(net, faddr, 0);

    __inet6_ehashfn(
        lhash,
        lport,
        fhash,
        fport,
        UDP_IPV6_HASH_SECRET.wrapping_add(net_hash_mix(net)),
    )
}

#[no_mangle]
pub unsafe extern "C" fn udp_v6_get_port(sk: *mut sock, snum: u16) -> c_int {
    let hash2_nulladdr = ipv6_portaddr_hash(sock_net(sk), &in6addr_any, snum);
    let hash2_partial = ipv6_portaddr_hash(sock_net(sk), &(*sk).sk_v6_rcv_saddr, 0);

    (*sk).udp_portaddr_hash = hash2_partial;
    udp_lib_get_port(sk, snum, hash2_nulladdr)
}

#[no_mangle]
pub unsafe extern "C" fn udp_v6_rehash(sk: *mut sock) {
    let new_hash = ipv6_portaddr_hash(sock_net(sk), &(*sk).sk_v6_rcv_saddr, (*sk).inet_num);
    udp_lib_rehash(sk, new_hash);
}

#[no_mangle]
pub unsafe extern "C" fn compute_score(
    sk: *mut sock,
    net: *const net,
    saddr: *const in6_addr,
    sport: u16,
    daddr: *const in6_addr,
    hnum: u16,
    dif: c_int,
    sdif: c_int,
) -> c_int {
    if !net_eq(sock_net(sk), net) || (*sk).udp_port_hash != hnum || (*sk).sk_family != AF_INET6 {
        return -1;
    }

    if !ipv6_addr_equal(&(*sk).sk_v6_rcv_saddr, daddr) {
        return -1;
    }

    let mut score = 0;
    let inet = &(*sk).inet_sk;

    if inet.inet_dport != 0 {
        if inet.inet_dport != sport {
            return -1;
        }
        score += 1;
    }

    if !ipv6_addr_any(&(*sk).sk_v6_daddr) {
        if !ipv6_addr_equal(&(*sk).sk_v6_daddr, saddr) {
            return -1;
        }
        score += 1;
    }

    if !udp_sk_bound_dev_eq(net, (*sk).sk_bound_dev_if, dif, sdif) {
        return -1;
    }
    score += 1;

    if (*sk).sk_incoming_cpu == raw_smp_processor_id() {
        score += 1;
    }

    score
}

#[no_mangle]
pub unsafe extern "C" fn lookup_reuseport(
    _net: *const net,
    _sk: *mut sock,
    _skb: *mut sk_buff,
) -> *mut sock {
    core::ptr::null_mut()
}