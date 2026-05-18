#![no_std]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use core::ffi::{c_int, c_void};
use core::mem;
use core::ptr;
use kernel_types::*;

pub type size_t = usize;
pub type c_size_t = usize;
pub type socklen_t = u32;

// Constants from C
pub const EINVAL: c_int = 22;
pub const ENOMEM: c_int = 12;
pub const ENETUNREACH: c_int = 101;
pub const EAFNOSUPPORT: c_int = 97;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct in6_addr {
    pub s6_addr: [u8; 16],
    pub s6_addr32: [u32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flowi6 {
    pub flowi6_proto: u8,
    pub daddr: in6_addr,
    pub saddr: in6_addr,
    pub flowi6_oif: c_int,
    pub flowi6_mark: c_int,
    pub fl6_dport: u16,
    pub fl6_sport: u16,
    pub flowlabel: u32,
    pub flowi6_uid: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pktinfo {
    pub ipi6_ifindex: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxopt_bits {
    pub rxpmtu: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxopt {
    pub bits: rxopt_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipv6_pinfo {
    pub sndflow: c_int,
    pub flow_label: u32,
    pub saddr: in6_addr,
    pub sticky_pktinfo: pktinfo,
    pub mcast_oif: c_int,
    pub rxopt: rxopt,
    pub opt: *mut ipv6_txoptions,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inet_sock {
    pub inet_dport: u16,
    pub inet_sport: u16,
    pub inet_rcv_saddr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sk_prot {
    pub rehash: Option<unsafe extern "C" fn(*mut sock)>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sock {
    pub sk_protocol: u8,
    pub sk_v6_daddr: in6_addr,
    pub sk_bound_dev_if: c_int,
    pub sk_mark: c_int,
    pub sk_uid: u32,
    pub sk_v6_rcv_saddr: in6_addr,
    pub sk_prot: *const sk_prot,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip6_flowlabel {
    pub opt: *mut ipv6_txoptions,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipv6_txoptions {
    _unused: [u8; 0],
}

unsafe extern "C" {
    fn ipv6_addr_v4mapped(a: *const in6_addr) -> bool;
    fn ipv6_addr_is_multicast(a: *const in6_addr) -> bool;
    fn ipv6_addr_any(a: *const in6_addr) -> bool;

    fn security_sk_classify_flow(sk: *mut sock, fl6: *mut flowi6);

    fn fl6_sock_lookup(sk: *mut sock, label: u32) -> *mut ip6_flowlabel;
    fn rcu_dereference(p: *mut ipv6_txoptions) -> *mut ipv6_txoptions;

    fn sock_net(sk: *mut sock) -> *mut c_void;
    fn ip6_dst_lookup_flow(
        net: *mut c_void,
        sk: *mut sock,
        fl6: *mut flowi6,
        final_p: *mut in6_addr,
    ) -> *mut c_void;

    fn inet_sk(sk: *mut sock) -> *mut inet_sock;
    fn inet6_sk(sk: *mut sock) -> *mut ipv6_pinfo;
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}

#[no_mangle]
pub unsafe extern "C" fn rust_eh_personality() {}

#[no_mangle]
pub unsafe extern "C" fn ipv6_mapped_addr_any(a: *const in6_addr) -> bool {
    if a.is_null() {
        return false;
    }
    let a_ref = &*a;
    ipv6_addr_v4mapped(a) && (a_ref.s6_addr32[3] == 0)
}

#[no_mangle]
pub unsafe extern "C" fn ip6_datagram_flow_key_init(fl6: *mut flowi6, sk: *mut sock) {
    if fl6.is_null() || sk.is_null() {
        return;
    }

    let inet = inet_sk(sk);
    let np = inet6_sk(sk);
    if inet.is_null() || np.is_null() {
        return;
    }

    ptr::write_bytes(fl6, 0, 1);
    (*fl6).flowi6_proto = (*sk).sk_protocol;
    (*fl6).daddr = (*sk).sk_v6_daddr;
    (*fl6).saddr = (*np).saddr;
    (*fl6).flowi6_oif = (*sk).sk_bound_dev_if;
    (*fl6).flowi6_mark = (*sk).sk_mark;
    (*fl6).fl6_dport = (*inet).inet_dport;
    (*fl6).fl6_sport = (*inet).inet_sport;
    (*fl6).flowlabel = (*np).flow_label;
    (*fl6).flowi6_uid = (*sk).sk_uid;

    if (*fl6).flowi6_oif == 0 {
        (*fl6).flowi6_oif = (*np).sticky_pktinfo.ipi6_ifindex;
    }

    if (*fl6).flowi6_oif == 0 && ipv6_addr_is_multicast(&(*fl6).daddr) {
        (*fl6).flowi6_oif = (*np).mcast_oif;
    }

    security_sk_classify_flow(sk, fl6);
}

#[no_mangle]
pub unsafe extern "C" fn ip6_datagram_dst_update(sk: *mut sock, fix_sk_saddr: c_int) -> c_int {
    if sk.is_null() {
        return -EINVAL;
    }

    let np = inet6_sk(sk);
    let inet = inet_sk(sk);
    if np.is_null() || inet.is_null() {
        return -EINVAL;
    }

    let mut flowlabel: *mut ip6_flowlabel = ptr::null_mut();

    if (*np).sndflow != 0 && ((*np).flow_label & 0x0FFF_FFFF) != 0 {
        flowlabel = fl6_sock_lookup(sk, (*np).flow_label);
        if flowlabel.is_null() {
            return -EINVAL;
        }
    }

    let mut fl6: flowi6 = mem::zeroed();
    ip6_datagram_flow_key_init(&mut fl6, sk);

    let _opt: *mut ipv6_txoptions = if !flowlabel.is_null() {
        (*flowlabel).opt
    } else {
        rcu_dereference((*np).opt)
    };

    let mut final_addr: in6_addr = mem::zeroed();
    let dst = ip6_dst_lookup_flow(sock_net(sk), sk, &mut fl6, &mut final_addr);

    if dst.is_null() {
        return -ENETUNREACH;
    }

    if fix_sk_saddr != 0 {
        if ipv6_addr_any(&(*np).saddr) {
            (*np).saddr = fl6.saddr;
        }
    }

    0
}