#![no_std]
#![no_main]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(clippy::all)]

use core::ffi::{c_char, c_int, c_uint};
use core::panic::PanicInfo;
use kernel_types::*;

pub type size_t = usize;
pub type c_size_t = usize;
pub type socklen_t = u32;

pub const NFPROTO_IPV4: u16 = 2;
pub const NFPROTO_IPV6: u16 = 10;

pub const IPPROTO_ICMP: u8 = 1;
pub const IPPROTO_TCP: u8 = 6;
pub const IPPROTO_UDP: u8 = 17;
pub const IPPROTO_DCCP: u8 = 33;
pub const IPPROTO_GRE: u8 = 47;
pub const IPPROTO_ICMPV6: u8 = 58;
pub const IPPROTO_SCTP: u8 = 132;
pub const IPPROTO_UDPLITE: u8 = 136;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct seq_file {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union nf_inet_addr {
    pub all: [u32; 4],
    pub ip: u32,
    pub ip6: [u32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_tuple_src_icmp {
    pub id: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_tuple_src_port {
    pub port: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_tuple_src_gre {
    pub key: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union nf_conntrack_tuple_src_u {
    pub icmp: nf_conntrack_tuple_src_icmp,
    pub tcp: nf_conntrack_tuple_src_port,
    pub udp: nf_conntrack_tuple_src_port,
    pub dccp: nf_conntrack_tuple_src_port,
    pub sctp: nf_conntrack_tuple_src_port,
    pub gre: nf_conntrack_tuple_src_gre,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_tuple_src {
    pub u3: nf_inet_addr,
    pub u: nf_conntrack_tuple_src_u,
    pub l3num: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_tuple_dst_icmp {
    pub type_: u8,
    pub code: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_tuple_dst_port {
    pub port: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_tuple_dst_gre {
    pub key: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union nf_conntrack_tuple_dst_u {
    pub icmp: nf_conntrack_tuple_dst_icmp,
    pub tcp: nf_conntrack_tuple_dst_port,
    pub udp: nf_conntrack_tuple_dst_port,
    pub dccp: nf_conntrack_tuple_dst_port,
    pub sctp: nf_conntrack_tuple_dst_port,
    pub gre: nf_conntrack_tuple_dst_gre,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_tuple_dst {
    pub u3: nf_inet_addr,
    pub u: nf_conntrack_tuple_dst_u,
    pub protonum: u8,
    pub dir: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_tuple {
    pub src: nf_conntrack_tuple_src,
    pub dst: nf_conntrack_tuple_dst,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_l4proto {
    pub l4proto: u8,
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn seq_printf(s: *mut seq_file, fmt: *const c_char, ...) -> c_int;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn print_tuple(
    s: *mut seq_file,
    tuple: *const nf_conntrack_tuple,
    l4proto: *const nf_conntrack_l4proto,
) {
    if s.is_null() || tuple.is_null() || l4proto.is_null() {
        return;
    }

    let l3num = unsafe { (*tuple).src.l3num };

    match l3num {
        NFPROTO_IPV4 => {
            let src_ip = unsafe { (*tuple).src.u3.ip };
            let dst_ip = unsafe { (*tuple).dst.u3.ip };
            unsafe {
                seq_printf(
                    s,
                    c"src=%pI4 dst=%pI4 ".as_ptr(),
                    &src_ip as *const _,
                    &dst_ip as *const _,
                );
            }
        }
        NFPROTO_IPV6 => {
            let src_ip6 = unsafe { (*tuple).src.u3.ip6 };
            let dst_ip6 = unsafe { (*tuple).dst.u3.ip6 };
            unsafe {
                seq_printf(
                    s,
                    c"src=%pI6 dst=%pI6 ".as_ptr(),
                    &src_ip6 as *const _,
                    &dst_ip6 as *const _,
                );
            }
        }
        _ => {}
    }

    let proto = unsafe { (*l4proto).l4proto };

    match proto {
        IPPROTO_ICMP | IPPROTO_ICMPV6 => {
            let t = unsafe { (*tuple).dst.u.icmp.type_ } as c_uint;
            let c = unsafe { (*tuple).dst.u.icmp.code } as c_uint;
            let id = u16::from_be(unsafe { (*tuple).src.u.icmp.id }) as c_uint;
            unsafe {
                seq_printf(s, c"type=%u code=%u id=%u ".as_ptr(), t, c, id);
            }
        }
        IPPROTO_TCP | IPPROTO_DCCP | IPPROTO_SCTP => {
            let sport = u16::from_be(unsafe { (*tuple).src.u.tcp.port }) as c_uint;
            let dport = u16::from_be(unsafe { (*tuple).dst.u.tcp.port }) as c_uint;
            unsafe {
                seq_printf(s, c"sport=%hu dport=%hu ".as_ptr(), sport, dport);
            }
        }
        IPPROTO_UDP | IPPROTO_UDPLITE => {
            let sport = u16::from_be(unsafe { (*tuple).src.u.udp.port }) as c_uint;
            let dport = u16::from_be(unsafe { (*tuple).dst.u.udp.port }) as c_uint;
            unsafe {
                seq_printf(s, c"sport=%hu dport=%hu ".as_ptr(), sport, dport);
            }
        }
        IPPROTO_GRE => {
            let key = u16::from_be(unsafe { (*tuple).src.u.gre.key }) as c_uint;
            unsafe {
                seq_printf(s, c"key=%hu ".as_ptr(), key);
            }
        }
        _ => {}
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    loop {}
}