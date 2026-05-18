#![no_std]
#![no_main]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use core::ffi::c_void;
use core::ptr;
use kernel_types::*;

pub const IPPROTO_TCP: c_int = 6;
pub const IPPROTO_UDP: c_int = 17;
pub const NFPROTO_IPV4: c_int = 2;
pub const IPS_NAT_DONE_MASK: c_int = 0x0000_000F;

pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conn {
    pub status: c_int,
    pub master: *mut nf_conn,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_expect {
    pub dir: c_int,
    pub saved_proto: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_nat_range2 {
    pub flags: c_int,
    pub min_addr: u32,
    pub max_addr: u32,
    pub min_proto: c_int,
    pub max_proto: c_int,
}

unsafe extern "C" {
    fn skb_ensure_writable(skb: *mut sk_buff, len: usize) -> c_int;
    fn pskb_expand_head(skb: *mut sk_buff, headroom: usize, tailroom: usize, gfp: c_int) -> c_int;
    fn nf_nat_csum_recalc(
        skb: *mut sk_buff,
        l3num: c_int,
        protocol: c_int,
        old_hdr: *mut c_void,
        check: *mut u16,
        newlen: c_int,
        oldlen: c_int,
    ) -> c_int;
    fn nf_ct_seqadj_set(ct: *mut nf_conn, ctinfo: c_int, seq: *mut c_void, delta: c_int);
    fn nf_ct_l3num(ct: *mut nf_conn) -> c_int;
    fn skb_network_header(skb: *mut sk_buff) -> *mut u8;
    fn skb_tail_pointer(skb: *mut sk_buff) -> *mut u8;
    fn __skb_trim(skb: *mut sk_buff, len: usize);
    fn skb_put(skb: *mut sk_buff, len: usize) -> *mut u8;
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo<'_>) -> ! {
    loop {
        core::hint::spin_loop();
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_eh_personality() {}

#[inline]
unsafe fn enlarge_skb(skb: *mut sk_buff, extra: usize) -> c_int {
    if extra == 0 {
        return 1;
    }
    if unsafe { pskb_expand_head(skb, 0, extra, 0) } != 0 {
        return 0;
    }
    1
}

unsafe fn mangle_contents(
    skb: *mut sk_buff,
    dataoff: c_uint,
    match_offset: c_uint,
    match_len: c_uint,
    rep_buffer: *const c_void,
    rep_len: c_uint,
) {
    let data = unsafe { skb_network_header(skb).add(dataoff as usize) };
    let src = unsafe { data.add((match_offset + match_len) as usize) };
    let dst = unsafe { data.add((match_offset + rep_len) as usize) };
    let end = unsafe { skb_tail_pointer(skb) as usize };
    let from = src as usize;
    let len = end.saturating_sub(from);

    unsafe { ptr::copy(src, dst, len) };

    unsafe {
        ptr::copy_nonoverlapping(
            rep_buffer as *const u8,
            data.add(match_offset as usize),
            rep_len as usize,
        )
    };

    if rep_len > match_len {
        let _ = unsafe { skb_put(skb, (rep_len - match_len) as usize) };
    } else if match_len > rep_len {
        let new_len = unsafe { (*skb).len as usize }.saturating_sub((match_len - rep_len) as usize);
        unsafe { __skb_trim(skb, new_len) };
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __nf_nat_mangle_tcp_packet(
    skb: *mut sk_buff,
    ct: *mut nf_conn,
    ctinfo: c_int,
    protoff: c_uint,
    match_offset: c_uint,
    match_len: c_uint,
    rep_buffer: *const c_void,
    rep_len: c_uint,
    adjust: c_int,
) -> c_int {
    if skb.is_null() || ct.is_null() || rep_buffer.is_null() {
        return EINVAL;
    }

    let old_skb_len = unsafe { (*skb).len as usize };

    if unsafe { skb_ensure_writable(skb, old_skb_len) } != 0 {
        return EINVAL;
    }

    if rep_len > match_len {
        let grow = (rep_len - match_len) as usize;
        if unsafe { enlarge_skb(skb, grow) } != 1 {
            return ENOMEM;
        }
        if unsafe { skb_ensure_writable(skb, (*skb).len as usize) } != 0 {
            return EINVAL;
        }
    }

    let tcph = unsafe { skb_network_header(skb).add(protoff as usize) };
    let doff_words = unsafe { ((*tcph.add(12) >> 4) & 0x0f) as usize };
    let dataoff = protoff as usize + doff_words * 4;

    let oldlen = unsafe { ((*skb).len as usize).saturating_sub(protoff as usize) };

    unsafe {
        mangle_contents(
            skb,
            dataoff as c_uint,
            match_offset,
            match_len,
            rep_buffer,
            rep_len,
        )
    };

    let newlen = unsafe { ((*skb).len as usize).saturating_sub(protoff as usize) };

    let check = unsafe { tcph.add(16) as *mut u16 };
    let _ = unsafe {
        nf_nat_csum_recalc(
            skb,
            nf_ct_l3num(ct),
            IPPROTO_TCP,
            tcph as *mut c_void,
            check,
            newlen as c_int,
            oldlen as c_int,
        )
    };

    if adjust != 0 && rep_len != match_len {
        unsafe {
            nf_ct_seqadj_set(
                ct,
                ctinfo,
                tcph as *mut c_void,
                rep_len as c_int - match_len as c_int,
            )
        };
    }

    1
}