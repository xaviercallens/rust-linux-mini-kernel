#![no_std]
#![no_main]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use core::ffi::{c_int, c_uint};
use core::panic::PanicInfo;
use kernel_types::*;

pub const IPPROTO_TCP: u8 = 6;
pub const IPPROTO_UDP: u8 = 17;
pub const IPPROTO_UDPLITE: u8 = 136;
pub const IPPROTO_SCTP: u8 = 132;
pub const IPPROTO_ICMP: u8 = 1;
pub const IPPROTO_ICMPV6: u8 = 58;
pub const IPPROTO_DCCP: u8 = 33;
pub const IPPROTO_GRE: u8 = 47;

pub const NF_NAT_MANIP_SRC: c_int = 0;
pub const NF_NAT_MANIP_DST: c_int = 1;

pub type __sum16 = __be16;
pub type __le32 = u32;

pub const CSUM_MANGLED_0: __be16 = 0xFFFFu16 as __be16;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct udphdr {
    pub source: __be16,
    pub dest: __be16,
    pub len: __be16,
    pub check: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcphdr {
    pub source: __be16,
    pub dest: __be16,
    pub seq: __be32,
    pub ack_seq: __be32,
    pub doff_res_flags: __be16,
    pub window: __be16,
    pub check: __be16,
    pub urg_ptr: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icmphdr {
    pub type_: __u8,
    pub code: __u8,
    pub checksum: __be16,
    pub un: [__u8; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icmp6hdr {
    pub icmp6_type: __u8,
    pub icmp6_code: __u8,
    pub icmp6_cksum: __be16,
    pub icmp6_identifier: __be16,
    pub icmp6_sequence: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctphdr {
    pub source: __be16,
    pub dest: __be16,
    pub verification_tag: __be32,
    pub checksum: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dccp_hdr {
    pub dccph_sport: __be16,
    pub dccph_dport: __be16,
    pub dccph_doff: __u8,
    pub dccph_cscov: __u8,
    pub dccph_checksum: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_tuple {
    pub src: nf_conntrack_man,
    pub dst: nf_conntrack_man,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_man {
    pub u3: nf_inet_addr,
    pub u: nf_conntrack_man_proto,
    pub l3num: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union nf_inet_addr {
    pub all: [__be32; 4],
    pub ip: __be32,
    pub ip6: [__be32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union nf_conntrack_man_proto {
    pub all: __be16,
    pub tcp: __be16,
    pub udp: __be16,
    pub icmp: __be16,
    pub dccp: __be16,
    pub sctp: __be16,
    pub gre: __be16,
}

extern "C" {
    fn skb_ensure_writable(skb: *mut sk_buff, len: c_uint) -> c_int;
    fn inet_proto_csum_replace2(
        sum: *mut __sum16,
        skb: *mut sk_buff,
        from: __be16,
        to: __be16,
        pseudohdr: c_int,
    );
    fn inet_proto_csum_replace4(
        sum: *mut __sum16,
        skb: *mut sk_buff,
        from: __be32,
        to: __be32,
        pseudohdr: c_int,
    );
    fn nf_csum_update(
        skb: *mut sk_buff,
        iphdroff: c_uint,
        check: *mut __sum16,
        tuple: *const nf_conntrack_tuple,
        maniptype: c_int,
    );
    fn sctp_compute_cksum(skb: *mut sk_buff, offset: c_uint) -> __le32;
}

#[inline(always)]
unsafe fn l4_ptr<T>(skb: *mut sk_buff, iphdroff: c_uint, hdroff: c_uint) -> *mut T {
    ((skb as *mut u8).add(iphdroff as usize).add(hdroff as usize)) as *mut T
}

fn __udp_manip_pkt(
    skb: *mut sk_buff,
    iphdroff: c_uint,
    hdr: *mut udphdr,
    tuple: *const nf_conntrack_tuple,
    maniptype: c_int,
    do_csum: bool,
) {
    unsafe {
        let h = &mut *hdr;
        let t = &*tuple;

        let newport = if maniptype == NF_NAT_MANIP_SRC {
            t.src.u.udp
        } else {
            t.dst.u.udp
        };

        let portptr: *mut __be16 = if maniptype == NF_NAT_MANIP_SRC {
            &mut h.source
        } else {
            &mut h.dest
        };

        if do_csum {
            nf_csum_update(
                skb,
                iphdroff,
                &mut h.check as *mut __be16 as *mut __sum16,
                tuple,
                maniptype,
            );
            inet_proto_csum_replace2(
                &mut h.check as *mut __be16 as *mut __sum16,
                skb,
                *portptr,
                newport,
                0,
            );
            if h.check == 0 {
                h.check = CSUM_MANGLED_0;
            }
        }

        *portptr = newport;
    }
}

pub fn udp_manip_pkt(
    skb: *mut sk_buff,
    iphdroff: c_uint,
    hdroff: c_uint,
    tuple: *const nf_conntrack_tuple,
    maniptype: c_int,
) -> bool {
    unsafe {
        if skb_ensure_writable(skb, hdroff + core::mem::size_of::<udphdr>() as c_uint) != 0 {
            return false;
        }
        let hdr = l4_ptr::<udphdr>(skb, iphdroff, hdroff);
        __udp_manip_pkt(skb, iphdroff, hdr, tuple, maniptype, (*hdr).check != 0);
        true
    }
}

pub fn udplite_manip_pkt(
    skb: *mut sk_buff,
    iphdroff: c_uint,
    hdroff: c_uint,
    tuple: *const nf_conntrack_tuple,
    maniptype: c_int,
) -> bool {
    unsafe {
        if skb_ensure_writable(skb, hdroff + core::mem::size_of::<udphdr>() as c_uint) != 0 {
            return false;
        }
        let hdr = l4_ptr::<udphdr>(skb, iphdroff, hdroff);
        __udp_manip_pkt(skb, iphdroff, hdr, tuple, maniptype, true);
        true
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}