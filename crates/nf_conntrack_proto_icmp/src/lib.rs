#![no_std]
#![no_main]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::transmutes_expressible_as_ptr_cast)]

use core::ffi::c_void;
use core::mem;
use core::panic::PanicInfo;
use kernel_types::*;

pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ENOSYS: c_int = -38;
pub const NF_ACCEPT: c_int = 1;
pub const NF_DROP: c_int = 0;
pub const HZ: c_int = 100;

pub const ICMP_ECHO: u8 = 8;
pub const ICMP_ECHOREPLY: u8 = 0;
pub const ICMP_TIMESTAMP: u8 = 13;
pub const ICMP_TIMESTAMPREPLY: u8 = 14;
pub const ICMP_INFO_REQUEST: u8 = 15;
pub const ICMP_INFO_REPLY: u8 = 16;
pub const ICMP_ADDRESS: u8 = 17;
pub const ICMP_ADDRESSREPLY: u8 = 18;
pub const NR_ICMP_TYPES: u8 = 18;

pub const NFPROTO_IPV4: u8 = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icmp_echo {
    pub id: u16,
    pub sequence: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icmp_ipv4 {
    pub gateway: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union icmp_un {
    pub echo: icmp_echo,
    pub ipv4: icmp_ipv4,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icmphdr {
    pub type_: u8,
    pub code: u8,
    pub checksum: u16,
    pub un: icmp_un,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_tuple_icmp {
    pub id: u16,
    pub type_: u8,
    pub code: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_tuple_u3 {
    pub ip: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union nf_conntrack_tuple_u {
    pub icmp: nf_conntrack_tuple_icmp,
    pub u3: nf_conntrack_tuple_u3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_tuple_src {
    pub u: nf_conntrack_tuple_u,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_tuple_dst {
    pub u: nf_conntrack_tuple_u,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_tuple {
    pub src: nf_conntrack_tuple_src,
    pub dst: nf_conntrack_tuple_dst,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_tuple_hash {
    pub tuple: nf_conntrack_tuple,
}

#[repr(C)]
pub struct nf_conn {
    pub tuplehash: [nf_conntrack_tuple_hash; 2],
}

#[repr(C)]
pub struct sk_buff {
    _priv: [u8; 0],
}

#[repr(C)]
pub struct nf_hook_state {
    pub pf: u8,
}

unsafe extern "C" {
    pub fn skb_header_pointer(
        skb: *const sk_buff,
        offset: c_uint,
        len: c_uint,
        buffer: *mut c_void,
    ) -> *mut c_void;
}

const fn build_invmap() -> [u8; 256] {
    let mut arr = [0u8; 256];
    arr[ICMP_ECHO as usize] = ICMP_ECHOREPLY + 1;
    arr[ICMP_ECHOREPLY as usize] = ICMP_ECHO + 1;
    arr[ICMP_TIMESTAMP as usize] = ICMP_TIMESTAMPREPLY + 1;
    arr[ICMP_TIMESTAMPREPLY as usize] = ICMP_TIMESTAMP + 1;
    arr[ICMP_INFO_REQUEST as usize] = ICMP_INFO_REPLY + 1;
    arr[ICMP_INFO_REPLY as usize] = ICMP_INFO_REQUEST + 1;
    arr[ICMP_ADDRESS as usize] = ICMP_ADDRESSREPLY + 1;
    arr[ICMP_ADDRESSREPLY as usize] = ICMP_ADDRESS + 1;
    arr
}

static INVMAP: [u8; 256] = build_invmap();
static VALID_NEW: [bool; 256] = [true; 256];

#[no_mangle]
pub unsafe extern "C" fn icmp_pkt_to_tuple(
    skb: *const sk_buff,
    dataoff: c_uint,
    _net: *mut c_void,
    tuple: *mut nf_conntrack_tuple,
) -> bool {
    let mut hdr: icmphdr = mem::zeroed();
    let hp = skb_header_pointer(
        skb,
        dataoff,
        mem::size_of::<icmphdr>() as c_uint,
        (&mut hdr as *mut icmphdr).cast::<c_void>(),
    );

    if hp.is_null() {
        return false;
    }

    let hp = hp.cast::<icmphdr>();

    (*tuple).dst.u.icmp.type_ = (*hp).type_;
    (*tuple).src.u.icmp.id = (*hp).un.echo.id;
    (*tuple).dst.u.icmp.code = (*hp).code;

    true
}

#[no_mangle]
pub unsafe extern "C" fn nf_conntrack_invert_icmp_tuple(
    tuple: *mut nf_conntrack_tuple,
    orig: *const nf_conntrack_tuple,
) -> bool {
    let orig_type = (*orig).dst.u.icmp.type_;

    if INVMAP[orig_type as usize] == 0 {
        return false;
    }

    (*tuple).src.u.icmp.id = (*orig).src.u.icmp.id;
    (*tuple).dst.u.icmp.type_ = INVMAP[orig_type as usize] - 1;
    (*tuple).dst.u.icmp.code = (*orig).dst.u.icmp.code;

    true
}

#[no_mangle]
pub unsafe extern "C" fn nf_conntrack_icmp_packet(
    ct: *mut nf_conn,
    _skb: *mut sk_buff,
    _ctinfo: c_int,
    state: *const nf_hook_state,
) -> c_int {
    if (*state).pf != NFPROTO_IPV4 {
        return -NF_ACCEPT;
    }

    let tuple = &(*ct).tuplehash[0].tuple;
    let type_ = tuple.dst.u.icmp.type_;

    if !VALID_NEW[type_ as usize] {
        return -NF_ACCEPT;
    }

    NF_ACCEPT
}

#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    loop {
        core::hint::spin_loop();
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_eh_personality() {}