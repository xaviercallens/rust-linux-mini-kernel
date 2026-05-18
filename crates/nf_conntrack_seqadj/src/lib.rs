#![no_std]
#![no_main]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use core::ffi::{c_int, c_void};
use core::panic::PanicInfo;
use kernel_types::*;

type __be32 = u32;
type __sum16 = u16;

#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    loop {}
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conn {
    pub status: u32,
    pub lock: *mut c_void,
    pub proctnum: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_ct_seqadj {
    pub offset_before: c_int,
    pub offset_after: c_int,
    pub correction_pos: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conn_seqadj {
    pub seq: [nf_ct_seqadj; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcphdr {
    pub seq: __be32,
    pub ack_seq: __be32,
    pub doff_res_flags: u16,
    pub window: u16,
    pub check: __sum16,
    pub urg_ptr: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp_sack_block_wire {
    pub start_seq: __be32,
    pub end_seq: __be32,
}

unsafe extern "C" {
    fn set_bit(bit: c_int, addr: *mut u32);
    fn nfct_seqadj(ct: *mut nf_conn) -> *mut nf_conn_seqadj;
    fn CTINFO2DIR(ctinfo: c_int) -> c_int;
    fn skb_network_header(skb: *mut sk_buff) -> *mut c_void;
    fn ip_hdrlen(skb: *mut sk_buff) -> c_int;
}

pub const IPPROTO_TCP: u16 = 6;
pub const EINVAL: c_int = -22;
pub const IPS_SEQ_ADJUST_BIT: c_int = 0;

#[inline]
fn after(a: __be32, b: __be32) -> bool {
    (b.wrapping_sub(a) as i32) < 0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_eh_personality() {}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nf_ct_seqadj_init(ct: *mut nf_conn, ctinfo: c_int, off: c_int) -> c_int {
    if ct.is_null() {
        return EINVAL;
    }
    if off == 0 {
        return 0;
    }

    unsafe { set_bit(IPS_SEQ_ADJUST_BIT, &mut (*ct).status) };

    let seqadj = unsafe { nfct_seqadj(ct) };
    if seqadj.is_null() {
        return EINVAL;
    }

    let dir = unsafe { CTINFO2DIR(ctinfo) as usize };
    if dir >= 2 {
        return EINVAL;
    }

    let this_way = unsafe { &mut (*seqadj).seq[dir] };
    this_way.offset_before = off;
    this_way.offset_after = off;
    this_way.correction_pos = 0;

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nf_ct_seqadj_set(
    ct: *mut nf_conn,
    ctinfo: c_int,
    seq: __be32,
    off: c_int,
) -> c_int {
    if ct.is_null() {
        return EINVAL;
    }
    if off == 0 {
        return 0;
    }

    let seqadj = unsafe { nfct_seqadj(ct) };
    if seqadj.is_null() {
        return EINVAL;
    }

    unsafe { set_bit(IPS_SEQ_ADJUST_BIT, &mut (*ct).status) };

    let dir = unsafe { CTINFO2DIR(ctinfo) as usize };
    if dir >= 2 {
        return EINVAL;
    }

    let this_way = unsafe { &mut (*seqadj).seq[dir] };
    if this_way.offset_before == this_way.offset_after || after(this_way.correction_pos, seq) {
        this_way.correction_pos = seq;
        this_way.offset_before = this_way.offset_after;
        this_way.offset_after = this_way.offset_after.wrapping_add(off);
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nf_ct_tcp_seqadj_set(
    skb: *mut sk_buff,
    ct: *mut nf_conn,
    ctinfo: c_int,
    off: c_int,
) {
    if skb.is_null() || ct.is_null() {
        return;
    }
    if unsafe { (*ct).proctnum } != IPPROTO_TCP {
        return;
    }

    let nh = unsafe { skb_network_header(skb) as *mut u8 };
    if nh.is_null() {
        return;
    }

    let th = unsafe { nh.add(ip_hdrlen(skb) as usize) as *mut tcphdr };
    let seq = unsafe { (*th).seq };

    let _ = unsafe { nf_ct_seqadj_set(ct, ctinfo, seq, off) };
}