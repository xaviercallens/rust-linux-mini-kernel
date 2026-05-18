#![no_std]
#![no_main]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::panic::PanicInfo;
use core::sync::atomic::AtomicUsize;
use kernel_types::*;

pub type size_t = usize;
pub type c_size_t = usize;
pub type socklen_t = u32;

// Constants from C headers
pub const IPPROTO_UDP: u8 = 17;
pub const IPPROTO_TCP: u8 = 6;
pub const IPPROTO_ICMP: u8 = 1;
pub const IPPROTO_RAW: u8 = 255;
pub const IPPROTO_ICMPV6: u8 = 58;
pub const IPPROTO_SCTP: u8 = 132;
pub const IPPROTO_DCCP: u8 = 33;
pub const IPPROTO_UDPLITE: u8 = 136;
pub const IPPROTO_GRE: u8 = 47;

pub const NF_ACCEPT: u32 = 0;
pub const NF_DROP: u32 = 1;
pub const NF_INET_PRE_ROUTING: u32 = 0;
pub const NF_INET_LOCAL_OUT: u32 = 1;
pub const NF_INET_POST_ROUTING: u32 = 2;
pub const NF_INET_LOCAL_IN: u32 = 3;
pub const NF_IP_PRI_CONNTRACK: i32 = -100;
pub const NF_IP_PRI_CONNTRACK_CONFIRM: i32 = 100;

// Opaque kernel types that may not be present in kernel_types.
#[repr(C)]
pub struct net {
    _private: [u8; 0],
}

// Opaque/FFI structs
#[repr(C)]
pub struct nf_conn {
    pub status: AtomicUsize,
}

#[repr(C)]
pub struct nf_conn_help {
    pub helper: *const nf_conntrack_helper,
}

#[repr(C)]
pub struct nf_conntrack_helper {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nf_conntrack_tuple_hash {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nf_conntrack_tuple {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nf_hook_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nf_hook_state {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nf_sockopt_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nf_ct_zone_dflt {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nf_conntrack_l4proto {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[allow(non_upper_case_globals)]
static nf_ct_proto_mutex: mutex = mutex { _private: [0; 0] };

// Function pointer typedefs
pub type nf_hook_fn =
    extern "C" fn(priv_: *mut c_void, skb: *mut sk_buff, state: *const nf_hook_state) -> c_ulong;
pub type nf_sockopt_get =
    extern "C" fn(sk: *mut c_void, optval: c_int, user: *mut c_void, len: *mut c_int) -> c_int;

// Exported l4proto symbols
#[no_mangle]
pub static nf_conntrack_l4proto_udp: nf_conntrack_l4proto = nf_conntrack_l4proto { _private: [0; 0] };
#[no_mangle]
pub static nf_conntrack_l4proto_tcp: nf_conntrack_l4proto = nf_conntrack_l4proto { _private: [0; 0] };
#[no_mangle]
pub static nf_conntrack_l4proto_icmp: nf_conntrack_l4proto = nf_conntrack_l4proto { _private: [0; 0] };
#[no_mangle]
pub static nf_conntrack_l4proto_icmpv6: nf_conntrack_l4proto = nf_conntrack_l4proto { _private: [0; 0] };
#[no_mangle]
pub static nf_conntrack_l4proto_sctp: nf_conntrack_l4proto = nf_conntrack_l4proto { _private: [0; 0] };
#[no_mangle]
pub static nf_conntrack_l4proto_dccp: nf_conntrack_l4proto = nf_conntrack_l4proto { _private: [0; 0] };
#[no_mangle]
pub static nf_conntrack_l4proto_udplite: nf_conntrack_l4proto = nf_conntrack_l4proto { _private: [0; 0] };
#[no_mangle]
pub static nf_conntrack_l4proto_gre: nf_conntrack_l4proto = nf_conntrack_l4proto { _private: [0; 0] };
#[no_mangle]
pub static nf_conntrack_l4proto_generic: nf_conntrack_l4proto = nf_conntrack_l4proto { _private: [0; 0] };

#[no_mangle]
pub unsafe extern "C" fn nf_ct_l4proto_find(l4proto: u8) -> *const nf_conntrack_l4proto {
    match l4proto {
        IPPROTO_UDP => &nf_conntrack_l4proto_udp,
        IPPROTO_TCP => &nf_conntrack_l4proto_tcp,
        IPPROTO_ICMP => &nf_conntrack_l4proto_icmp,
        IPPROTO_ICMPV6 => &nf_conntrack_l4proto_icmpv6,
        IPPROTO_SCTP => &nf_conntrack_l4proto_sctp,
        IPPROTO_DCCP => &nf_conntrack_l4proto_dccp,
        IPPROTO_UDPLITE => &nf_conntrack_l4proto_udplite,
        IPPROTO_GRE => &nf_conntrack_l4proto_gre,
        _ => &nf_conntrack_l4proto_generic,
    }
}

#[no_mangle]
pub unsafe extern "C" fn nf_l4proto_log_invalid(
    _skb: *const sk_buff,
    _net: *mut net,
    _pf: c_uint,
    _protonum: u8,
    _fmt: *const c_char,
) {
}

#[no_mangle]
pub unsafe extern "C" fn nf_ct_l4proto_log_invalid(
    _skb: *const sk_buff,
    _ct: *const nf_conn,
    _fmt: *const c_char,
) {
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}