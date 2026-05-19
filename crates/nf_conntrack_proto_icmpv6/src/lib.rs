
#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![allow(non_camel_case_types)]

use core::ffi::{c_void, c_int, c_uint, c_ulong};
use core::mem;
use core::ptr;
use kernel_types::{sk_buff, nf_conn};

pub const IPPROTO_ICMPV6: c_int = 58;
pub const NF_ACCEPT: c_int = 1;
pub const NFPROTO_IPV6: c_int = 10;
pub const HZ: c_ulong = 100;

// ICMPv6 type inversion map for reply tracking
static INVMAP: [u8; 256] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    129, 130, 0, 0, 0, 0, 0, 0, 0, 137, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

// ICMPv6 types that can start new connections
static VALID_NEW: [u8; 256] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

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
pub struct nf_hook_state {
    pub pf: c_int,
    pub net: *const c_void,
    pub hook: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_icmp_net {
    pub timeout: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_l4proto {
    pub l4proto: c_int,
    #[cfg(feature = "nf_ct_netlink")]
    pub tuple_to_nlattr: extern "C" fn(*mut c_void, *const nf_conntrack_tuple) -> c_int,
    #[cfg(feature = "nf_ct_netlink")]
    pub nlattr_tuple_size: extern "C" fn() -> c_int,
    #[cfg(feature = "nf_ct_netlink")]
    pub nlattr_to_tuple: extern "C" fn(*mut nf_conntrack_tuple, *mut c_void, c_int) -> c_int,
    #[cfg(feature = "nf_ct_netlink")]
    pub nla_policy: *const c_void,
    #[cfg(feature = "nf_conntrack_timeout")]
    pub ctnl_timeout: nf_conntrack_timeout,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_timeout {
    pub nlattr_to_obj: extern "C" fn(*mut c_void, *const c_void, *mut c_ulong) -> c_int,
    pub obj_to_nlattr: extern "C" fn(*mut c_void, *const c_ulong) -> c_int,
    pub nlattr_max: c_int,
    pub obj_size: c_int,
    pub nla_policy: *const c_void,
}

pub static mut __UDP_DISCONNECT: *mut c_void = ptr::null_mut();
pub static mut ICMPV6_ERR_CONVERT: *mut c_void = ptr::null_mut();
pub static mut INET6_SOCKRAW_OPS: *mut c_void = ptr::null_mut();
pub static mut IP6_DATAGRAM_CONNECT_V6_ONLY: *mut c_void = ptr::null_mut();

#[no_mangle]
pub unsafe extern "C" fn icmpv6_pkt_to_tuple(
    skb: *const sk_buff,
    dataoff: c_uint,
    _net: *mut c_void,
    tuple: *mut nf_conntrack_tuple,
) -> bool {
    let mut _hdr: [u8; 4] = [0; 4];
    let hp = skb_header_pointer(skb, dataoff, 4, _hdr.as_mut_ptr() as *mut c_void);

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
    if (*state).pf != NFPROTO_IPV6 as u8 as c_int {
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
        unsafe { *icmpv6_get_timeouts((*state).net as *mut _) }
    } else {
        unsafe { *timeout_ptr }
    };

    unsafe { nf_ct_refresh_acct(ct, ctinfo, skb, timeout) };
    NF_ACCEPT
}

#[unsafe(no_mangle)]
pub static nf_ct_icmpv6_timeout: c_ulong = 30 * HZ;

unsafe fn skb_header_pointer(
    skb: *const sk_buff,
    dataoff: c_uint,
    size: c_int,
    buffer: *mut c_void,
) -> *mut c_void {
    if (*skb).len < (dataoff as c_int + size) as c_uint {
        return ptr::null_mut();
    }

    let data = (*skb).data.offset(dataoff as isize);
    ptr::copy_nonoverlapping(data, buffer, size as usize);
    buffer
}

unsafe fn nf_ct_timeout_lookup(ct: *mut nf_conn) -> *mut c_ulong {
    ptr::null_mut()
}

unsafe fn nf_ct_is_confirmed(ct: *mut nf_conn) -> bool {
    false
}

unsafe fn nf_ct_refresh_acct(
    ct: *mut nf_conn,
    ctinfo: c_int,
    skb: *mut sk_buff,
    timeout: c_ulong,
) {
}

unsafe fn nf_icmpv6_pernet(net: *const c_void) -> *mut nf_icmp_net {
    static mut dummy: nf_icmp_net = nf_icmp_net { timeout: nf_ct_icmpv6_timeout };
    &mut dummy
}

static invmap: [u8; 8] = [
    ICMPV6_ECHO_REPLY + 1,
    ICMPV6_ECHO_REQUEST + 1,
    0, 0, 0, 0, 0, 0,
];

const ICMPV6_ECHO_REQUEST: u8 = 128;
const ICMPV6_ECHO_REPLY: u8 = 129;
const ICMPV6_NI_QUERY: u8 = 139;
const ICMPV6_NI_REPLY: u8 = 140;

#[cfg(feature = "nf_ct_netlink")]
#[no_mangle]
pub unsafe extern "C" fn icmpv6_tuple_to_nlattr(
    skb: *mut c_void,
    tuple: *const nf_conntrack_tuple,
) -> c_int {
    let id = (*tuple).src.u.icmp.id;
    let type_ = (*tuple).dst.u.icmp.type_;
    let code = (*tuple).dst.u.icmp.code;

    if nla_put_be16(skb, CTA_PROTO_ICMPV6_ID, id) != 0 ||
       nla_put_u8(skb, CTA_PROTO_ICMPV6_TYPE, type_) != 0 ||
       nla_put_u8(skb, CTA_PROTO_ICMPV6_CODE, code) != 0 {
        return -1;
    }

    0
}

const CTA_PROTO_ICMPV6_ID: c_int = 1;
const CTA_PROTO_ICMPV6_TYPE: c_int = 2;
const CTA_PROTO_ICMPV6_CODE: c_int = 3;

unsafe fn nla_put_be16(_skb: *mut c_void, _type: c_int, _data: u16) -> c_int {
    0
}

unsafe fn nla_put_u8(_skb: *mut c_void, _type: c_int, _data: u8) -> c_int {
    0
}

#[no_mangle]
pub static nf_conntrack_l4proto_icmpv6: nf_conntrack_l4proto = nf_conntrack_l4proto {
    l4proto: IPPROTO_ICMPV6,
    #[cfg(feature = "nf_ct_netlink")]
    tuple_to_nlattr: icmpv6_tuple_to_nlattr,
    #[cfg(feature = "nf_ct_netlink")]
    nlattr_tuple_size: icmpv6_nlattr_tuple_size,
    #[cfg(feature = "nf_ct_netlink")]
    nlattr_to_tuple: icmpv6_nlattr_to_tuple,
    #[cfg(feature = "nf_ct_netlink")]
    nla_policy: icmpv6_nla_policy,
    #[cfg(feature = "nf_conntrack_timeout")]
    ctnl_timeout: nf_conntrack_timeout {
        nlattr_to_obj: icmpv6_timeout_nlattr_to_obj,
        obj_to_nlattr: icmpv6_timeout_obj_to_nlattr,
        nlattr_max: CTA_TIMEOUT_ICMP_MAX,
        obj_size: mem::size_of::<c_ulong>() as c_int,
        nla_policy: icmpv6_timeout_nla_policy,
    },
};

#[cfg(feature = "nf_conntrack_timeout")]
#[no_mangle]
pub unsafe extern "C" fn icmpv6_timeout_nlattr_to_obj(
    tb: *mut c_void,
    net: *const c_void,
    data: *mut c_ulong,
) -> c_int {
    let timeout = data;
    let in_net = nf_icmpv6_pernet(net);

    if tb.is_null() {
        *timeout = (*in_net).timeout;
        return 0;
    }

    let val = nla_get_be32(tb);
    *timeout = ntohl(val) * HZ;

    0
}

#[cfg(feature = "nf_conntrack_timeout")]
#[no_mangle]
pub unsafe extern "C" fn icmpv6_timeout_obj_to_nlattr(
    skb: *mut c_void,
    data: *const c_ulong,
) -> c_int {
    let timeout = *data / HZ;
    if nla_put_be32(skb, CTA_TIMEOUT_ICMPV6_TIMEOUT, htonl(timeout)) != 0 {
        return -1;
    }
    0
}

const CTA_TIMEOUT_ICMPV6_TIMEOUT: c_int = 1;
const CTA_TIMEOUT_ICMP_MAX: c_int = 2;

unsafe fn nla_get_be32(_tb: *mut c_void) -> u32 {
    0
}

unsafe fn htonl(_val: c_ulong) -> u32 {
    0
}

#[no_mangle]
pub unsafe extern "C" fn nf_conntrack_icmpv6_init_net(
    net: *const c_void,
) {
    let in_net = nf_icmpv6_pernet(net);
    (*in_net).timeout = nf_ct_icmpv6_timeout;
}

#[cfg(feature = "nf_ct_netlink")]
unsafe extern "C" fn icmpv6_nlattr_tuple_size() -> c_int {
    0
}

#[cfg(feature = "nf_ct_netlink")]
unsafe extern "C" fn icmpv6_nlattr_to_tuple(
    _tuple: *mut nf_conntrack_tuple,
    _tb: *mut c_void,
    _size: c_int,
) -> c_int {
    0
}

#[cfg(feature = "nf_ct_netlink")]
static icmpv6_nla_policy: *const c_void = ptr::null();

#[cfg(feature = "nf_conntrack_timeout")]
static icmpv6_timeout_nla_policy: *const c_void = ptr::null();
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
