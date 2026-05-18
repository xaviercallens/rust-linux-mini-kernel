
//! TCP Sequence Adjustment for Netfilter Connection Tracking
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use core::ffi::{c_int, c_uint, c_void};
use kernel_types::*;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conn_seqadj {
    pub seq: [nf_ct_seqadj; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_ct_seqadj {
    pub offset_before: c_int,
    pub offset_after: c_int,
    pub correction_pos: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcphdr {
    pub seq: u32,
    pub ack_seq: u32,
    pub doff: u8,
    pub check: u16,
    pub ack: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp_sack_block_wire {
    pub start_seq: u32,
    pub end_seq: u32,
}

// Extern declarations for kernel functions
extern "C" {
    fn set_bit(bit: c_int, addr: *mut u32);
    fn nfct_seqadj(ct: *mut nf_conn) -> *mut nf_conn_seqadj;
    fn CTINFO2DIR(ctinfo: c_int) -> c_int;
    fn skb_network_header(skb: *mut sk_buff) -> *mut c_void;
    fn ip_hdrlen(skb: *mut sk_buff) -> c_int;
    fn skb_ensure_writable(skb: *mut sk_buff, len: c_int) -> c_int;
    fn inet_proto_csum_replace4(
        csum: *mut u16,
        skb: *mut sk_buff,
        old: *mut c_void,
        new: *mut c_void,
        pseudo: c_int,
    );
}

// Constants from C
pub const IPPROTO_TCP: u16 = 6;
pub const TCPOPT_SACK: u8 = 4;
pub const TCPOLEN_SACK_PERBLOCK: u8 = 8;
pub const TCPOPT_EOL: u8 = 0;
pub const TCPOPT_NOP: u8 = 1;

// Error codes
pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;

/// Initialize sequence adjustment for connection
///
/// # Safety
/// - `ct` must be a valid pointer to nf_conn
/// - `off` must be a valid s32 value
/// - Caller must ensure no data races on `ct`
///
/// # Returns
/// 0 on success, -EINVAL if parameters invalid
#[no_mangle]
pub unsafe extern "C" fn nf_ct_seqadj_init(ct: *mut nf_conn, ctinfo: c_int, off: c_int) -> c_int {
    if ct.is_null() {
        return EINVAL;
    }

    if off == 0 {
        return 0;
    }

    // Set the IPS_SEQ_ADJUST_BIT in (*ct).status
    unsafe { set_bit(0, &mut (*ct).status) };

    let seqadj = unsafe { nfct_seqadj(ct) };
    if seqadj.is_null() {
        return EINVAL;
    }

    let dir = unsafe { CTINFO2DIR(ctinfo) } as usize;
    let this_way = &mut (*seqadj).seq[dir];
    (*this_way).offset_before = off;
    (*this_way).offset_after = off;

    0
}

/// Set sequence adjustment parameters
///
/// # Safety
/// - `ct` must be a valid pointer to nf_conn
/// - `seq` must be a valid __be32 value
/// - `off` must be a valid s32 value
/// - Caller must ensure no data races on `ct`
///
/// # Returns
/// 0 on success, -EINVAL if parameters invalid
#[no_mangle]
pub unsafe extern "C" fn nf_ct_seqadj_set(
    ct: *mut nf_conn,
    ctinfo: c_int,
    seq: u32,
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

    unsafe { set_bit(0, &mut (*ct).status) };

    let dir = unsafe { CTINFO2DIR(ctinfo) } as usize;
    let this_way = &mut (*seqadj).seq[dir];

    // SAFETY: Lock is held by caller
    unsafe {
        if (*this_way).offset_before == (*this_way).offset_after
            || after((*this_way).correction_pos, seq)
        {
            (*this_way).correction_pos = seq;
            (*this_way).offset_before = (*this_way).offset_after;
            (*this_way).offset_after += off;
        }
    }

    0
}

/// Set TCP sequence adjustment from skb
///
/// # Safety
/// - `skb` must be a valid pointer to sk_buff
/// - `ct` must be a valid pointer to nf_conn
/// - `off` must be a valid s32 value
/// - Caller must ensure no data races on `ct` or `skb`
#[no_mangle]
pub unsafe extern "C" fn nf_ct_tcp_seqadj_set(
    skb: *mut sk_buff,
    ct: *mut nf_conn,
    ctinfo: c_int,
    off: c_int,
) {
    if ct.is_null() || skb.is_null() {
        return;
    }

    let network_header = unsafe { skb_network_header(skb) };
    let ip_header_len = unsafe { ip_hdrlen(skb) };
    let tcp_header = (network_header as *mut u8).add(ip_header_len as usize) as *mut tcphdr;
    let seq = (*tcp_header).seq;

    unsafe { nf_ct_seqadj_set(ct, ctinfo, seq, off) };
}

/// Adjust TCP SACK blocks
///
/// # Safety
/// - `skb` must be a valid pointer to sk_buff
/// - `tcph` must be a valid pointer to tcphdr
/// - `sackoff` and `sackend` must be valid offsets
/// - `seq` must be a valid pointer to nf_ct_seqadj
#[no_mangle]
pub unsafe extern "C" fn nf_ct_sack_block_adjust(
    skb: *mut sk_buff,
    tcph: *mut tcphdr,
    sackoff: c_int,
    sackend: c_int,
    seq: *mut nf_ct_seqadj,
) {
    if skb.is_null() || tcph.is_null() || seq.is_null() {
        return;
    }

    let mut current_off = sackoff;
    while current_off < sackend {
        let sack = (skb as *mut u8).add(current_off as usize) as *mut tcp_sack_block_wire;
        let new_start_seq = if after(
            ntohl((*sack).start_seq) - (*seq).offset_before as u32,
            (*seq).correction_pos,
        ) {
            htonl(ntohl((*sack).start_seq) - (*seq).offset_after as u32)
        } else {
            htonl(ntohl((*sack).start_seq) - (*seq).offset_before as u32)
        };

        let new_end_seq = if after(
            ntohl((*sack).end_seq) - (*seq).offset_before as u32,
            (*seq).correction_pos,
        ) {
            htonl(ntohl((*sack).end_seq) - (*seq).offset_after as u32)
        } else {
            htonl(ntohl((*sack).end_seq) - (*seq).offset_before as u32)
        };

        // Update checksum
        unsafe {
            inet_proto_csum_replace4(&mut (*tcph).check, skb, &(*sack).start_seq as *mut c_void, &new_start_seq as *mut c_void, 0);
            inet_proto_csum_replace4(&mut (*tcph).check, skb, &(*sack).end_seq as *mut c_void, &new_end_seq as *mut c_void, 0);
        }

        (*sack).start_seq = new_start_seq;
        (*sack).end_seq = new_end_seq;
        current_off += core::mem::size_of::<tcp_sack_block_wire>() as c_int;
    }
}

/// Adjust TCP SACK options
///
/// # Safety
/// - `skb` must be a valid pointer to sk_buff
/// - `protoff` must be a valid offset
/// - `ct` must be a valid pointer to nf_conn
/// - `ctinfo` must be a valid enum value
///
/// # Returns
/// 1 on success, 0 on failure
#[no_mangle]
pub unsafe extern "C" fn nf_ct_sack_adjust(
    skb: *mut sk_buff,
    protoff: c_int,
    ct: *mut nf_conn,
    ctinfo: c_int,
) -> c_int {
    if skb.is_null() || ct.is_null() {
        return 0;
    }

    let seqadj = unsafe { nfct_seqadj(ct) };
    if seqadj.is_null() {
        return 0;
    }

    let dir = unsafe { CTINFO2DIR(ctinfo) } as usize;
    let mut optoff = protoff + core::mem::size_of::<tcphdr>() as c_int;
    let optend = protoff + (*(*ct).sk).sk_protocol as c_int * 4;

    if unsafe { skb_ensure_writable(skb, optend) } != 0 {
        return 0;
    }

    while optoff < optend {
        let op = (skb as *mut u8).add(optoff as usize) as *mut u8;
        match (*op) {
            TCPOPT_EOL => return 1,
            TCPOPT_NOP => {
                optoff += 1;
                continue;
            }
            _ => {
                let len = *op.add(1) as c_int;
                if optoff + len > optend || len < 2 {
                    return 0;
                }

                if (*op) == TCPOPT_SACK
                    && len >= 2 + TCPOLEN_SACK_PERBLOCK
                    && (len - 2) % TCPOLEN_SACK_PERBLOCK == 0
                {
                    unsafe {
                        nf_ct_sack_block_adjust(
                            skb,
                            (*ct).sk as *mut tcphdr,
                            optoff + 2,
                            optoff + len,
                            &mut (*seqadj).seq[!dir],
                        );
                    }
                }
                optoff += len;
            }
        }
    }

    1
}

/// Adjust TCP sequence numbers
///
/// # Safety
/// - `skb` must be a valid pointer to sk_buff
/// - `ct` must be a valid pointer to nf_conn
/// - `ctinfo` must be a valid enum value
/// - `protoff` must be a valid offset
///
/// # Returns
/// 1 on success, 0 on failure
#[no_mangle]
pub unsafe extern "C" fn nf_ct_seq_adjust(
    skb: *mut sk_buff,
    ct: *mut nf_conn,
    ctinfo: c_int,
    protoff: c_int,
) -> c_int {
    if skb.is_null() || ct.is_null() {
        return 0;
    }

    let dir = unsafe { CTINFO2DIR(ctinfo) } as usize;
    let seqadj = unsafe { nfct_seqadj(ct) };
    if seqadj.is_null() {
        return 0;
    }

    let this_way = &(*seqadj).seq[dir];
    let other_way = &(*seqadj).seq[!dir];

    if unsafe { skb_ensure_writable(skb, protoff + core::mem::size_of::<tcphdr>() as c_int) } != 0 {
        return 0;
    }

    let tcph = (skb as *mut u8).add(protoff as usize) as *mut tcphdr;
    let mut res = 1;

    unsafe {
        let seqoff = if after(ntohl((*tcph).seq), (*this_way).correction_pos) {
            (*this_way).offset_after as u32
        } else {
            (*this_way).offset_before as u32
        };

        let newseq = htonl(ntohl((*tcph).seq) + seqoff);
        inet_proto_csum_replace4(&mut (*tcph).check, skb, &(*tcph).seq as *mut c_void, &newseq as *mut c_void, 0);
        (*tcph).seq = newseq;

        if (*tcph).ack != 0 {
            let ackoff = if after(
                ntohl((*tcph).ack_seq) - (*other_way).offset_before as u32,
                (*other_way).correction_pos,
            ) {
                (*other_way).offset_after as u32
            } else {
                (*other_way).offset_before as u32
            };

            let newack = htonl(ntohl((*tcph).ack_seq) - ackoff);
            inet_proto_csum_replace4(&mut (*tcph).check, skb, &(*tcph).ack_seq as *mut c_void, &newack as *mut c_void, 0);
            (*tcph).ack_seq = newack;
        }

        res = nf_ct_sack_adjust(skb, protoff, ct, ctinfo);
    }

    res
}

/// Get sequence offset
///
/// # Safety
/// - `ct` must be a valid pointer to nf_conn
/// - `dir` must be a valid direction
///
/// # Returns
/// s32 offset value
#[no_mangle]
pub unsafe extern "C" fn nf_ct_seq_offset(ct: *mut nf_conn, dir: c_int, seq: u32) -> c_int {
    if ct.is_null() {
        return 0;
    }

    let seqadj = unsafe { nfct_seqadj(ct) };
    if seqadj.is_null() {
        return 0;
    }

    let this_way = &(*seqadj).seq[dir as usize];
    if after(seq, (*this_way).correction_pos) {
        (*this_way).offset_after
    } else {
        (*this_way).offset_before
    }
}

/// Helper function to check if a sequence number is after a position
#[inline]
fn after(seq: u32, pos: u32) -> bool {
    seq.wrapping_sub(pos) < (1 << 31)
}

/// Helper function to convert network to host long
#[inline]
fn ntohl(n: u32) -> u32 {
    u32::from_be(n)
}

/// Helper function to convert host to network long
#[inline]
fn htonl(h: u32) -> u32 {
    u32::to_be(h)
}