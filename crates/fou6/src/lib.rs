//! Foo over UDP (IPv6) Tunneling Implementation
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(clippy::all)]

use core::ffi::c_int;
use core::ffi::c_void;
use core::mem;
use core::ptr;

// Constants from C
pub const IPPROTO_UDP: u8 = 17;
pub const IPPROTO_IPV6: u8 = 41;
pub const IPPROTO_IPIP: u8 = 4;
pub const IPPROTO_UDPLITE: u8 = 136;

pub const EINVAL: c_int = -22;
pub const ENOENT: c_int = -2;
pub const EOPNOTSUPP: c_int = -95;

// Type definitions
#[repr(C)]
#[derive(Copy, Clone)]
pub struct in6_addr {
    pub s6_addr: [u8; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iphdr {
    pub version: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct udphdr {
    pub source: u16,
    pub dest: u16,
    pub len: u16,
    pub check: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sk_buff {
    // Placeholder - actual fields depend on kernel version
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_tunnel_encap {
    pub dport: u16, // __be16
    pub flags: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flowi6 {
    pub saddr: in6_addr,
    pub daddr: in6_addr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct guehdr {
    pub version: u8,
    pub control: u8,
    pub hlen: u8,
    pub proto_ctype: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inet6_skb_parm {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inet6_protocol {
    pub err_handler: extern "C" fn(
        *mut sk_buff,
        *mut inet6_skb_parm,
        u8,
        u8,
        c_int,
        u32,
    ) -> c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip6_tnl_encap_ops {
    pub encap_hlen: extern "C" fn(*const ip_tunnel_encap) -> c_int,
    pub build_header: extern "C" fn(
        *mut sk_buff,
        *const ip_tunnel_encap,
        *mut u8,
        *mut flowi6,
    ) -> c_int,
    pub err_handler: extern "C" fn(
        *mut sk_buff,
        *mut inet6_skb_parm,
        u8,
        u8,
        c_int,
        u32,
    ) -> c_int,
}

// Function pointers for kernel functions
extern "C" {
    fn skb_push(skb: *mut sk_buff, len: usize) -> *mut c_void;
    fn skb_reset_transport_header(skb: *mut sk_buff);
    fn udp_hdr(skb: *mut sk_buff) -> *mut udphdr;
    fn udp6_set_csum(
        flag: c_int,
        skb: *mut sk_buff,
        saddr: *const in6_addr,
        daddr: *const in6_addr,
        len: usize,
    );
    fn __fou_build_header(
        skb: *mut sk_buff,
        e: *const ip_tunnel_encap,
        protocol: *mut u8,
        sport: *mut u16,
        type_: c_int,
    ) -> c_int;
    fn __gue_build_header(
        skb: *mut sk_buff,
        e: *const ip_tunnel_encap,
        protocol: *mut u8,
        sport: *mut u16,
        type_: c_int,
    ) -> c_int;
    fn pskb_may_pull(skb: *mut sk_buff, len: usize) -> c_int;
    fn validate_gue_flags(guehdr: *const guehdr, optlen: usize) -> c_int;
    fn ip6_tnl_encap_add_ops(
        ops: *const ip6_tnl_encap_ops,
        encap_type: c_int,
    ) -> c_int;
    fn ip6_tnl_encap_del_ops(
        ops: *const ip6_tnl_encap_ops,
        encap_type: c_int,
    );
    fn pr_err(fmt: *const u8, ...);
}

// Internal functions
fn fou6_build_udp(
    skb: *mut sk_buff,
    e: *const ip_tunnel_encap,
    fl6: *const flowi6,
    protocol: *mut u8,
    sport: u16,
) {
    unsafe {
        // SAFETY: Caller guarantees valid skb pointer
        let _ = skb_push(skb, mem::size_of::<udphdr>()) as *mut udphdr;
        skb_reset_transport_header(skb);
        
        let uh = udp_hdr(skb);
        (*uh).dest = (*e).dport;
        (*uh).source = sport;
        (*uh).len = (skb.len() as u16).to_be();
        udp6_set_csum(
            !((*e).flags & 0x0001) as i32, // TUNNEL_ENCAP_FLAG_CSUM6
            skb,
            &(*fl6).saddr,
            &(*fl6).daddr,
            skb.len(),
        );
        
        *protocol = IPPROTO_UDP;
    }
}

fn fou6_build_header(
    skb: *mut sk_buff,
    e: *const ip_tunnel_encap,
    protocol: *mut u8,
    fl6: *mut flowi6,
) -> c_int {
    unsafe {
        let mut sport = 0u16;
        let type_ = if (*e).flags & 0x0001 != 0 {
            1 // SKB_GSO_UDP_TUNNEL_CSUM
        } else {
            0 // SKB_GSO_UDP_TUNNEL
        };
        
        let err = __fou_build_header(skb, e, protocol, &mut sport, type_);
        if err != 0 {
            return err;
        }
        
        fou6_build_udp(skb, e, fl6, protocol, sport);
        0
    }
}

fn gue6_build_header(
    skb: *mut sk_buff,
    e: *const ip_tunnel_encap,
    protocol: *mut u8,
    fl6: *mut flowi6,
) -> c_int {
    unsafe {
        let mut sport = 0u16;
        let type_ = if (*e).flags & 0x0001 != 0 {
            1 // SKB_GSO_UDP_TUNNEL_CSUM
        } else {
            0 // SKB_GSO_UDP_TUNNEL
        };
        
        let err = __gue_build_header(skb, e, protocol, &mut sport, type_);
        if err != 0 {
            return err;
        }
        
        fou6_build_udp(skb, e, fl6, protocol, sport);
        0
    }
}

fn gue6_err_proto_handler(
    proto: c_int,
    skb: *mut sk_buff,
    opt: *mut inet6_skb_parm,
    type_: u8,
    code: u8,
    offset: c_int,
    info: u32,
) -> c_int {
    unsafe {
        let ipprot = ptr::read_volatile(&(*(&inet6_protos[proto as usize] as *const *const inet6_protocol)));
        if !ipprot.is_null() && !(*ipprot).err_handler.is_null() {
            let result = (*(*ipprot).err_handler)(
                skb,
                opt,
                type_,
                code,
                offset,
                info,
            );
            if result == 0 {
                return 0;
            }
        }
        -ENOENT
    }
}

fn gue6_err(
    skb: *mut sk_buff,
    opt: *mut inet6_skb_parm,
    type_: u8,
    code: u8,
    offset: c_int,
    info: u32,
) -> c_int {
    unsafe {
        let transport_offset = 0; // skb_transport_offset(skb)
        let guehdr = &(*(&(*udp_hdr(skb) as *const udphdr).offset(1) as *const guehdr));
        
        let len = mem::size_of::<udphdr>() + mem::size_of::<guehdr>();
        if pskb_may_pull(skb, (transport_offset + len) as usize) == 0 {
            return -EINVAL;
        }
        
        match guehdr.version {
            0 => {}
            1 => {
                skb_set_transport_header(skb, -(mem::size_of::<icmp6hdr>() as isize));
                
                match (*(&(*guehdr as *const guehdr as *const iphdr)).version) {
                    4 => {
                        let ret = gue6_err_proto_handler(
                            IPPROTO_IPIP as c_int,
                            skb,
                            opt,
                            type_,
                            code,
                            offset,
                            info,
                        );
                        return ret;
                    }
                    6 => {
                        let ret = gue6_err_proto_handler(
                            IPPROTO_IPV6 as c_int,
                            skb,
                            opt,
                            type_,
                            code,
                            offset,
                            info,
                        );
                        return ret;
                    }
                    _ => return -EOPNOTSUPP,
                }
            }
            _ => return -EOPNOTSUPP,
        }
        
        if guehdr.control != 0 {
            return -ENOENT;
        }
        
        let optlen = (guehdr.hlen as usize) << 2;
        if pskb_may_pull(skb, (transport_offset + len + optlen) as usize) == 0 {
            return -EINVAL;
        }
        
        let guehdr = &(*(&(*udp_hdr(skb) as *const udphdr).offset(1) as *const guehdr));
        if validate_gue_flags(guehdr, optlen) != 0 {
            return -EINVAL;
        }
        
        if guehdr.proto_ctype == IPPROTO_UDP || guehdr.proto_ctype == IPPROTO_UDPLITE {
            return -EOPNOTSUPP;
        }
        
        skb_set_transport_header(skb, -(mem::size_of::<icmp6hdr>() as isize));
        let ret = gue6_err_proto_handler(
            guehdr.proto_ctype as c_int,
            skb,
            opt,
            type_,
            code,
            offset,
            info,
        );
        
        skb_set_transport_header(skb, transport_offset);
        ret
    }
}

// Static data
static mut fou_ip6tun_ops: ip6_tnl_encap_ops = ip6_tnl_encap_ops {
    encap_hlen: fou_encap_hlen,
    build_header: fou6_build_header,
    err_handler: gue6_err,
};

static mut gue_ip6tun_ops: ip6_tnl_encap_ops = ip6_tnl_encap_ops {
    encap_hlen: gue_encap_hlen,
    build_header: gue6_build_header,
    err_handler: gue6_err,
};

// Extern declarations for undefined symbols
extern "C" {
    fn fou_encap_hlen(e: *const ip_tunnel_encap) -> c_int;
    fn gue_encap_hlen(e: *const ip_tunnel_encap) -> c_int;
    fn inet6_protos: [*const inet6_protocol; 256];
    fn icmp6hdr: [u8; 0];
    fn skb_set_transport_header(skb: *mut sk_buff, offset: isize);
}

// Module functions
#[no_mangle]
pub unsafe extern "C" fn ip6_tnl_encap_add_fou_ops() -> c_int {
    let mut ret = ip6_tnl_encap_add_ops(&fou_ip6tun_ops, 1); // TUNNEL_ENCAP_FOU
    if ret < 0 {
        pr_err(b"can't add fou6 ops\0".as_ptr() as *const u8);
        return ret;
    }
    
    ret = ip6_tnl_encap_add_ops(&gue_ip6tun_ops, 2); // TUNNEL_ENCAP_GUE
    if ret < 0 {
        ip6_tnl_encap_del_ops(&fou_ip6tun_ops, 1);
        pr_err(b"can't add gue6 ops\0".as_ptr() as *const u8);
        return ret;
    }
    
    ret
}

#[no_mangle]
pub unsafe extern "C" fn ip6_tnl_encap_del_fou_ops() {
    ip6_tnl_encap_del_ops(&fou_ip6tun_ops, 1);
    ip6_tnl_encap_del_ops(&gue_ip6tun_ops, 2);
}

// Module init/exit
#[no_mangle]
pub unsafe extern "C" fn fou6_init() -> c_int {
    ip6_tnl_encap_add_fou_ops()
}

#[no_mangle]
pub unsafe extern "C" fn fou6_fini() {
    ip6_tnl_encap_del_fou_ops()
}

// Module metadata
#[link_section = ".modinfo"]
#[no_mangle]
pub static MOD_AUTHOR: [u8; 22] = *b"Tom Herbert <therbert@google.com>\0";

#[link_section = ".modinfo"]
#[no_mangle]
pub static MOD_LICENSE: [u8; 4] = *b"GPL\0";

#[link_section = ".modinfo"]
#[no_mangle]
pub static MOD_DESCRIPTION: [u8; 24] = *b"Foo over UDP (IPv6)\0";