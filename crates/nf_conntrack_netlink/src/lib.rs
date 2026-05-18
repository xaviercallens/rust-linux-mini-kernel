#![no_std]
#![no_main]
#![allow(non_camel_case_types)]

use core::ffi::c_int;
use core::panic::PanicInfo;
use kernel_types::*;

pub type size_t = usize;
pub type c_size_t = usize;
pub type socklen_t = u32;

pub const ENOMEM: c_int = 12;
pub const EINVAL: c_int = 22;
pub const EMSGSIZE: c_int = 90;

pub const CTA_TUPLE_PROTO: c_int = 1;
pub const CTA_PROTO_NUM: c_int = 1;
pub const CTA_IP_V4_SRC: c_int = 1;
pub const CTA_IP_V4_DST: c_int = 2;
pub const CTA_IP_V6_SRC: c_int = 3;
pub const CTA_IP_V6_DST: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nlattr {
    pub nla_len: u16,
    pub nla_type: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_inet_addr {
    pub all: [u32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_tuple_dst {
    pub protonum: u8,
    pub u3: nf_inet_addr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_tuple_src {
    pub u3: nf_inet_addr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_tuple {
    pub dst: nf_conntrack_tuple_dst,
    pub src: nf_conntrack_tuple_src,
    pub src_l3num: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_l4proto {
    pub tuple_to_nlattr:
        Option<extern "C" fn(skb: *mut sk_buff, tuple: *const nf_conntrack_tuple) -> c_int>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conn {
    pub status: u32,
    pub mark: u32,
    pub _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conn_counter {
    pub packets: [u64; 2],
    pub bytes: [u64; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conn_acct {
    pub counter: *mut nf_conn_counter,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conn_tstamp {
    pub start: u64,
    pub stop: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conn_labels {
    pub bits: [u64; 16],
}

unsafe extern "C" {
    fn nla_nest_start(skb: *mut sk_buff, attrtype: c_int) -> *mut nlattr;
    fn nla_nest_end(skb: *mut sk_buff, nest: *mut nlattr);
    fn nla_put_u8(skb: *mut sk_buff, attrtype: c_int, val: u8) -> c_int;
    fn nla_put_be16(skb: *mut sk_buff, attrtype: c_int, val: u16) -> c_int;
    fn nla_put_be32(skb: *mut sk_buff, attrtype: c_int, val: u32) -> c_int;
    fn nla_put_in_addr(skb: *mut sk_buff, attrtype: c_int, val: u32) -> c_int;
    fn nla_put_in6_addr(skb: *mut sk_buff, attrtype: c_int, val: *const [u8; 16]) -> c_int;
    fn nla_put_be64(skb: *mut sk_buff, attrtype: c_int, val: u64, pad: c_int) -> c_int;
    fn nla_put_string(skb: *mut sk_buff, attrtype: c_int, val: *const u8) -> c_int;
    fn nf_ct_l4proto_find(protonum: u8) -> *const nf_conntrack_l4proto;
    fn nf_ct_protonum(ct: *const nf_conn) -> u8;
    fn nf_ct_expires(ct: *const nf_conn) -> u32;
    fn nf_ct_acct_find(ct: *const nf_conn) -> *mut nf_conn_acct;
    fn nf_conn_tstamp_find(ct: *const nf_conn) -> *const nf_conn_tstamp;
    fn nf_ct_labels_find(ct: *const nf_conn) -> *const nf_conn_labels;
    fn rcu_read_lock();
    fn rcu_read_unlock();
    fn security_secid_to_secctx(secid: u32, secctx: *mut *mut u8, len: *mut size_t) -> c_int;
    fn security_release_secctx(secctx: *mut u8, len: size_t);
}

#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    loop {
        core::hint::spin_loop();
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_eh_personality() {}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ctnetlink_dump_tuples_proto(
    skb: *mut sk_buff,
    tuple: *const nf_conntrack_tuple,
    l4proto: *const nf_conntrack_l4proto,
) -> c_int {
    let mut ret: c_int = 0;
    let nest_parms = unsafe { nla_nest_start(skb, CTA_TUPLE_PROTO) };
    if nest_parms.is_null() {
        return -EMSGSIZE;
    }

    if unsafe { nla_put_u8(skb, CTA_PROTO_NUM, (*tuple).dst.protonum) } != 0 {
        return -EMSGSIZE;
    }

    if let Some(proto_to_nlattr) = unsafe { (*l4proto).tuple_to_nlattr } {
        ret = proto_to_nlattr(skb, tuple);
    }

    unsafe { nla_nest_end(skb, nest_parms) };
    ret
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ipv4_tuple_to_nlattr(
    skb: *mut sk_buff,
    tuple: *const nf_conntrack_tuple,
) -> c_int {
    if unsafe { nla_put_in_addr(skb, CTA_IP_V4_SRC, (*tuple).src.u3.all[0]) } != 0
        || unsafe { nla_put_in_addr(skb, CTA_IP_V4_DST, (*tuple).dst.u3.all[0]) } != 0
    {
        return -EMSGSIZE;
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ipv6_tuple_to_nlattr(
    skb: *mut sk_buff,
    tuple: *const nf_conntrack_tuple,
) -> c_int {
    let src: *const [u8; 16] = unsafe { (*tuple).src.u3.all.as_ptr() as *const [u8; 16] };
    let dst: *const [u8; 16] = unsafe { (*tuple).dst.u3.all.as_ptr() as *const [u8; 16] };

    if unsafe { nla_put_in6_addr(skb, CTA_IP_V6_SRC, src) } != 0
        || unsafe { nla_put_in6_addr(skb, CTA_IP_V6_DST, dst) } != 0
    {
        return -EMSGSIZE;
    }
    0
}