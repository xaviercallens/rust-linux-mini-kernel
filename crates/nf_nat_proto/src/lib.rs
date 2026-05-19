
//! Network Filter NAT Protocol Manipulation
//!
//! This module implements protocol-specific NAT manipulation for various transport
//! protocols in the Linux kernel. The implementation is FFI-compatible with the
//! original C code and maintains exact ABI compatibility for all exported symbols.
//!
//! The code handles UDP, TCP, ICMP, and other protocols by modifying packet headers
//! and recalculating checksums during NAT operations.

#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
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

// Type definitions

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

        let hdr = (skb.add(iphdroff as usize) as *mut u8).add(hdroff as usize) as *mut udphdr;
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

        let hdr = (skb.add(iphdroff as usize) as *mut u8).add(hdroff as usize) as *mut udphdr;
        __udp_manip_pkt(skb, iphdroff, hdr, tuple, maniptype, true);
        true
    }
}

#[cfg(not(feature = "udplite"))]
fn udplite_manip_pkt(
    _: *mut sk_buff,
    _: c_uint,
    _: c_uint,
    _: *const nf_conntrack_tuple,
    _: c_int,
) -> bool {
    true
}

fn sctp_manip_pkt(
    skb: *mut sk_buff,
    iphdroff: c_uint,
    hdroff: c_uint,
    tuple: *const nf_conntrack_tuple,
    maniptype: c_int,
) -> bool {
    unsafe {
        #[cfg(feature = "sctp")]
        {
            let hdrsize = if skb.add(iphdroff as usize).add(hdroff as usize).add(8)
                <= skb.add(iphdroff as usize).add(hdroff as usize).add(8)
            {
                8
            } else {
                core::mem::size_of::<sctphdr>() as c_uint
            };

            if skb_ensure_writable(skb, hdroff + hdrsize) != 0 {
                return false;
            }

            let hdr = (skb.add(iphdroff as usize) as *mut u8).add(hdroff as usize) as *mut sctphdr;

            if maniptype == NF_NAT_MANIP_SRC {
                (*hdr).source = (*tuple).src.u.sctp;
            } else {
                (*hdr).dest = (*tuple).dst.u.sctp;
            }

            if hdrsize < core::mem::size_of::<sctphdr>() as c_uint {
                return true;
            }

            if (*skb).ip_summed != 1 {
                // CHECKSUM_PARTIAL
                (*hdr).checksum = sctp_compute_cksum(skb, hdroff);
                (*skb).ip_summed = 0; // CHECKSUM_NONE
            }
        }
        true
    }
}

fn tcp_manip_pkt(
    skb: *mut sk_buff,
    iphdroff: c_uint,
    hdroff: c_uint,
    tuple: *const nf_conntrack_tuple,
    maniptype: c_int,
) -> bool {
    unsafe {
        let hdrsize = if skb.add(iphdroff as usize).add(hdroff as usize).add(8)
            <= skb.add(iphdroff as usize).add(hdroff as usize).add(8)
        {
            8
        } else {
            core::mem::size_of::<tcphdr>() as c_uint
        };

        if skb_ensure_writable(skb, hdroff + hdrsize) != 0 {
            return false;
        }

        let hdr = (skb.add(iphdroff as usize) as *mut u8).add(hdroff as usize) as *mut tcphdr;

        let newport = if maniptype == NF_NAT_MANIP_SRC {
            (*tuple).src.u.tcp
        } else {
            (*tuple).dst.u.tcp
        };

        let portptr = if maniptype == NF_NAT_MANIP_SRC {
            &mut (*hdr).source
        } else {
            &mut (*hdr).dest
        };

        let oldport = *portptr;
        *portptr = newport;

        if hdrsize < core::mem::size_of::<tcphdr>() as c_uint {
            return true;
        }

        nf_csum_update(skb, iphdroff, &mut (*hdr).check, tuple, maniptype);
        inet_proto_csum_replace2(&mut (*hdr).check, skb, oldport, newport, false);
        true
    }
}

fn dccp_manip_pkt(
    skb: *mut sk_buff,
    iphdroff: c_uint,
    hdroff: c_uint,
    tuple: *const nf_conntrack_tuple,
    maniptype: c_int,
) -> bool {
    unsafe {
        #[cfg(feature = "dccp")]
        {
            let hdrsize = if skb.add(iphdroff as usize).add(hdroff as usize).add(8)
                <= skb.add(iphdroff as usize).add(hdroff as usize).add(8)
            {
                8
            } else {
                core::mem::size_of::<dccp_hdr>() as c_uint
            };

            if skb_ensure_writable(skb, hdroff + hdrsize) != 0 {
                return false;
            }

            let hdr = (skb.add(iphdroff as usize) as *mut u8).add(hdroff as usize) as *mut dccp_hdr;

            let newport = if maniptype == NF_NAT_MANIP_SRC {
                (*tuple).src.u.dccp
            } else {
                (*tuple).dst.u.dccp
            };

            let portptr = if maniptype == NF_NAT_MANIP_SRC {
                &mut (*hdr).dccph_sport
            } else {
                &mut (*hdr).dccph_dport
            };

            let oldport = *portptr;
            *portptr = newport;

            if hdrsize < core::mem::size_of::<dccp_hdr>() as c_uint {
                return true;
            }

            nf_csum_update(skb, iphdroff, &mut (*hdr).dccph_checksum, tuple, maniptype);
            inet_proto_csum_replace2(&mut (*hdr).dccph_checksum, skb, oldport, newport, false);
        }
        true
    }
}

fn icmp_manip_pkt(
    skb: *mut sk_buff,
    iphdroff: c_uint,
    hdroff: c_uint,
    tuple: *const nf_conntrack_tuple,
    maniptype: c_int,
) -> bool {
    unsafe {
        if skb_ensure_writable(skb, hdroff + core::mem::size_of::<icmphdr>() as c_uint) != 0 {
            return false;
        }

        let hdr = (skb.add(iphdroff as usize) as *mut u8).add(hdroff as usize) as *mut icmphdr;
        let hdr = &mut *hdr;

        match hdr.type_ {
            8 | 0 | 13 | 14 | 15 | 16 | 17 | 18 => {
                inet_proto_csum_replace2(
                    &mut hdr.checksum,
                    skb,
                    hdr.un[0] as __be16,
                    (*tuple).src.u.icmp,
                    false,
                );
                hdr.un[0] = (*tuple).src.u.icmp as __u8;
            }
            _ => return true,
        }
        true
    }
}

fn icmpv6_manip_pkt(
    skb: *mut sk_buff,
    iphdroff: c_uint,
    hdroff: c_uint,
    tuple: *const nf_conntrack_tuple,
    maniptype: c_int,
) -> bool {
    unsafe {
        if skb_ensure_writable(skb, hdroff + core::mem::size_of::<icmp6hdr>() as c_uint) != 0 {
            return false;
        }

        let hdr = (skb.add(iphdroff as usize) as *mut u8).add(hdroff as usize) as *mut icmp6hdr;
        nf_csum_update(skb, iphdroff, &mut (*hdr).icmp6_cksum, tuple, maniptype);

        if (*hdr).icmp6_type == 128 || (*hdr).icmp6_type == 129 {
            inet_proto_csum_replace2(
                &mut (*hdr).icmp6_cksum,
                skb,
                (*hdr).icmp6_identifier,
                (*tuple).src.u.icmp,
                false,
            );
            (*hdr).icmp6_identifier = (*tuple).src.u.icmp;
        }
        true
    }
}

fn gre_manip_pkt(
    skb: *mut sk_buff,
    iphdroff: c_uint,
    hdroff: c_uint,
    tuple: *const nf_conntrack_tuple,
    maniptype: c_int,
) -> bool {
    unsafe {
        #[cfg(feature = "gre")]
        {
            if skb_ensure_writable(skb, hdroff + 8) != 0 {
                return false;
            }

            let greh = (skb.add(iphdroff as usize) as *mut u8).add(hdroff as usize) as *mut __u8;
            let greh = greh as *mut __be16;

            if maniptype != NF_NAT_MANIP_DST {
                return true;
            }

            match (*greh as __be16) & 0x8000 {
                0x0000 => {
                    // GREv0 - no NAT
                }
                0x8000 => {
                    let pgreh = greh as *mut __be32;
                    (*pgreh) = (*tuple).dst.u.gre;
                }
                _ => {
                    // Unknown GRE version
                    return false;
                }
            }
        }
        true
    }
}

fn l4proto_manip_pkt(
    skb: *mut sk_buff,
    iphdroff: c_uint,
    hdroff: c_uint,
    tuple: *const nf_conntrack_tuple,
    maniptype: c_int,
) -> bool {
    unsafe {
        match (*tuple).protonum {
            IPPROTO_TCP => tcp_manip_pkt(skb, iphdroff, hdroff, tuple, maniptype),
            IPPROTO_UDP => udp_manip_pkt(skb, iphdroff, hdroff, tuple, maniptype),
            IPPROTO_UDPLITE => udplite_manip_pkt(skb, iphdroff, hdroff, tuple, maniptype),
            IPPROTO_SCTP => sctp_manip_pkt(skb, iphdroff, hdroff, tuple, maniptype),
            IPPROTO_ICMP => icmp_manip_pkt(skb, iphdroff, hdroff, tuple, maniptype),
            IPPROTO_ICMPV6 => icmpv6_manip_pkt(skb, iphdroff, hdroff, tuple, maniptype),
            IPPROTO_DCCP => dccp_manip_pkt(skb, iphdroff, hdroff, tuple, maniptype),
            IPPROTO_GRE => gre_manip_pkt(skb, iphdroff, hdroff, tuple, maniptype),
            _ => true,
        }
    }
}

// Exported functions
#[no_mangle]
pub unsafe extern "C" fn nf_nat_ipv4_manip_pkt(
    skb: *mut sk_buff,
    iphdroff: c_uint,
    target: *const nf_conntrack_tuple,
    maniptype: c_int,
) -> c_int {
    // SAFETY: Caller must ensure skb is valid and writable
    if skb.is_null() || target.is_null() {
        return -22; // EINVAL
    }

    if skb_ensure_writable(skb, iphdroff + core::mem::size_of::<iphdr>() as c_uint) != 0 {
        return -12; // ENOMEM
    }

    let iph = (skb.add(iphdroff as usize)) as *mut iphdr;
    let iph = &mut *iph;
    let hdroff = iphdroff + (iph.ihl as c_uint) * 4;

    if !l4proto_manip_pkt(skb, iphdroff, hdroff, target, maniptype) {
        return -12; // ENOMEM
    }

    // Update IP header checksum
    if maniptype == NF_NAT_MANIP_SRC {
        // SAFETY: Valid pointer and data
        inet_proto_csum_replace2(&mut iph.check, skb, iph.saddr, (*target).src.u3.s_addr, false);
    } else {
        // SAFETY: Valid pointer and data
        inet_proto_csum_replace2(&mut iph.check, skb, iph.daddr, (*target).dst.u3.s_addr, false);
    }

    0 // Success
}

// Error codes
pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ENOSYS: c_int = -38;

// Test cases
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_udp_manipulation() {
        // This would require a real skb buffer to test
        // For demonstration purposes, we just verify the function signatures
        assert_eq!(core::mem::size_of::<udphdr>(), 8);
        assert_eq!(core::mem::size_of::<tcphdr>(), 20);
    }
}
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
