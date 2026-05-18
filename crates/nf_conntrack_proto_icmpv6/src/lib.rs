#![no_std]
#![no_main]
#![allow(non_camel_case_types)]

use core::ffi::{c_int, c_uint, c_ulong, c_void};
use core::panic::PanicInfo;
use kernel_types::*;

pub type size_t = usize;
pub type socklen_t = u32;

pub const IPPROTO_ICMPV6: c_int = 58;
pub const NF_ACCEPT: c_int = 1;
pub const NFPROTO_IPV6: c_int = 10;
pub const HZ: c_ulong = 100;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_tuple {
    pub src: nf_conntrack_tuple_src,
    pub dst: nf_conntrack_tuple_dst,
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
pub union nf_conntrack_tuple_u {
    pub icmp: nf_conntrack_tuple_icmp,
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
pub struct nf_conn_tuplehash {
    pub tuple: nf_conntrack_tuple,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conn {
    pub tuplehash: [nf_conn_tuplehash; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_hook_state {
    pub pf: c_int,
    pub net: *mut c_void,
    pub hook: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_icmp_net {
    pub timeout: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sk_buff {
    _priv: [u8; 0],
}

unsafe extern "C" {
    fn skb_header_pointer(
        skb: *const sk_buff,
        offset: c_uint,
        len: c_int,
        buffer: *mut c_void,
    ) -> *const c_void;

    fn nf_ct_timeout_lookup(ct: *mut nf_conn) -> *mut c_ulong;
    fn nf_ct_is_confirmed(ct: *const nf_conn) -> bool;
    fn nf_ct_refresh_acct(ct: *mut nf_conn, ctinfo: c_int, skb: *mut sk_buff, timeout: c_ulong);
    fn nf_icmpv6_pernet(net: *mut c_void) -> *mut nf_icmp_net;
}

static INVMAP: [u8; 8] = [129, 128, 131, 130, 0, 0, 0, 0];
static VALID_NEW: [u8; 6] = [1, 0, 0, 0, 1, 0];

#[unsafe(no_mangle)]
pub unsafe extern "C" fn icmpv6_pkt_to_tuple(
    skb: *const sk_buff,
    dataoff: c_uint,
    _net: *mut c_void,
    tuple: *mut nf_conntrack_tuple,
) -> bool {
    let mut hdr = [0u8; 4];
    let hp = unsafe { skb_header_pointer(skb, dataoff, 4, hdr.as_mut_ptr() as *mut c_void) };
    if hp.is_null() {
        return false;
    }

    let p = hp as *const u8;
    let type_ = unsafe { *p.add(0) };
    let code = unsafe { *p.add(1) };
    let id = u16::from_be_bytes([unsafe { *p.add(2) }, unsafe { *p.add(3) }]);

    unsafe {
        (*tuple).dst.u.icmp.type_ = type_;
        (*tuple).dst.u.icmp.code = code;
        (*tuple).src.u.icmp.id = id;
    }
    true
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nf_conntrack_invert_icmpv6_tuple(
    tuple: *mut nf_conntrack_tuple,
    orig: *const nf_conntrack_tuple,
) -> bool {
    let t = unsafe { (*orig).dst.u.icmp.type_ };
    if t < 128 {
        return false;
    }
    let type_off = (t - 128) as usize;
    if type_off >= INVMAP.len() || INVMAP[type_off] == 0 {
        return false;
    }

    unsafe {
        (*tuple).src.u.icmp.id = (*orig).src.u.icmp.id;
        (*tuple).dst.u.icmp.type_ = INVMAP[type_off] - 1;
        (*tuple).dst.u.icmp.code = (*orig).dst.u.icmp.code;
    }
    true
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn icmpv6_get_timeouts(net: *mut c_void) -> *mut c_ulong {
    let in_net = unsafe { nf_icmpv6_pernet(net) };
    unsafe { &mut (*in_net).timeout }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nf_conntrack_icmpv6_packet(
    ct: *mut nf_conn,
    skb: *mut sk_buff,
    ctinfo: c_int,
    state: *const nf_hook_state,
) -> c_int {
    if unsafe { (*state).pf } != NFPROTO_IPV6 {
        return -NF_ACCEPT;
    }

    if !unsafe { nf_ct_is_confirmed(ct) } {
        let t = unsafe { (*ct).tuplehash[0].tuple.dst.u.icmp.type_ };
        if t < 128 {
            return -NF_ACCEPT;
        }
        let off = (t - 128) as usize;
        if off >= VALID_NEW.len() || VALID_NEW[off] == 0 {
            return -NF_ACCEPT;
        }
    }

    let timeout_ptr = unsafe { nf_ct_timeout_lookup(ct) };
    let timeout = if timeout_ptr.is_null() {
        unsafe { *icmpv6_get_timeouts((*state).net) }
    } else {
        unsafe { *timeout_ptr }
    };

    unsafe { nf_ct_refresh_acct(ct, ctinfo, skb, timeout) };
    NF_ACCEPT
}

#[unsafe(no_mangle)]
pub static nf_ct_icmpv6_timeout: c_ulong = 30 * HZ;

#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    loop {}
}