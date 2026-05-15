// SPDX-License-Identifier: GPL-2.0-or-later
//!
//! This module provides FFI-compatible Rust bindings for TCP diagnostic functionality
//! in the Linux kernel. It maintains ABI compatibility with the original C implementation.
//!
//! The implementation handles TCP socket monitoring, MD5 signature support, and ULP
//! (User-Level Protocol) information retrieval while preserving the exact behavior
//! of the original C code.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use core::ffi::c_int;
use core::ffi::c_void;
use core::mem;
use core::ptr;
use core::sync::atomic::{AtomicU32, Ordering};

// Constants from C
pub const IPPROTO_TCP: c_int = 6;
pub const INET_DIAG_MD5SIG: c_int = 1;
pub const INET_ULP_INFO_NAME: c_int = 1;
pub const TCP_ULP_NAME_MAX: usize = 15;
pub const ECONNABORTED: c_int = 108;
pub const ENOMEM: c_int = -12;
pub const EINVAL: c_int = -22;
pub const EMSGSIZE: c_int = -90;

// Type definitions
#[repr(C)]
pub struct sock {
    // In real implementation, this would have all the fields from the C struct
    // For FFI compatibility, we keep it as an opaque type
    _private: [u8; 0],
}

#[repr(C)]
pub struct inet_diag_msg {
    idiag_family: u8,
    idiag_state: u8,
    idiag_timer: u8,
    idiag_retrans: u8,
    idiag_expires: u32,
    idiag_rqueue: u32,
    idiag_wqueue: u32,
    idiag_uid: u32,
    idiag_ino: u32,
    idiag_dbs: u16,
    idiag_cgroup: u32,
}

#[repr(C)]
pub struct tcp_info {
    tcpi_state: u8,
    tcpi_ca_state: u8,
    tcpi_retransmits: u8,
    tcpi_probes: u8,
    tcpi_backoff: u8,
    tcpi_options: u8,
    tcpi_rto: u32,
    tcpi_ato: u32,
    tcpi_snd_mss: u32,
    tcpi_rcv_mss: u32,
    tcpi_unacked: u32,
    tcpi_sacked: u32,
    tcpi_lost: u32,
    tcpi_retrans: u32,
    tcpi_fackets: u32,
    tcpi_last_data_sent: u32,
    tcpi_last_ack_sent: u32,
    tcpi_last_data_recv: u32,
    tcpi_last_ack_recv: u32,
    tcpi_pmtu: u32,
    tcpi_rcv_ssthresh: u32,
    tcpi_rtt: u32,
    tcpi_rttvar: u32,
    tcpi_snd_ssthresh: u32,
    tcpi_snd_cwnd: u32,
    tcpi_advmss: u32,
    tcpi_reordering: u32,
    tcpi_rcv_win: u32,
    tcpi_rcv_max: u32,
    tcpi_snd_win: u32,
    tcpi_delivery_rate: u64,
    tcpi_bytes_acked: u64,
    tcpi_bytes_received: u64,
    tcpi_dsack_bytes: u64,
    tcpi_app_limited: u32,
    tcpi_sacked_out: u32,
    tcpi_retrans_out: u32,
    tcpi_fackets_out: u32,
    tcpi_total_retrans: u32,
}

#[repr(C)]
pub struct tcp_diag_md5sig {
    tcpm_family: u8,
    tcpm_prefixlen: u8,
    tcpm_keylen: u16,
    tcpm_addr: [u8; 16],
    tcpm_key: [u8; 32],
}

#[repr(C)]
pub struct tcp_md5sig_key {
    family: u8,
    prefixlen: u8,
    keylen: u16,
    key: [u8; 32],
    addr: tcp_md5sig_key_addr,
}

#[repr(C)]
pub union tcp_md5sig_key_addr {
    a4: u32,
    a6: [u8; 16],
}

#[repr(C)]
pub struct tcp_md5sig_info {
    head: hlist_head,
}

#[repr(C)]
pub struct hlist_head {
    first: *mut hlist_node,
}

#[repr(C)]
pub struct hlist_node {
    next: *mut hlist_node,
    pprev: *mut *mut hlist_node,
}

#[repr(C)]
pub struct tcp_ulp_ops {
    name: *const u8,
    get_info: Option<unsafe extern "C" fn(*mut sock, *mut sk_buff) -> c_int>,
    get_info_size: Option<unsafe extern "C" fn(*mut sock) -> usize>,
}

#[repr(C)]
pub struct inet_diag_req_v2 {
    sdiag_family: u8,
    sdiag_protocol: u8,
    idiag_ext: u16,
    idiag_states: u32,
    idiag_if: u32,
    idiag_sport: u16,
    idiag_dport: u16,
    id: inet_diag_msg,
}

#[repr(C)]
pub struct inet_diag_handler {
    dump: Option<unsafe extern "C" fn(*mut sk_buff, *mut netlink_callback, *const inet_diag_req_v2)>,
    dump_one: Option<unsafe extern "C" fn(*mut netlink_callback, *const inet_diag_req_v2) -> c_int>,
    idiag_get_info: Option<unsafe extern "C" fn(*mut sock, *mut inet_diag_msg, *mut c_void)>,
    idiag_get_aux: Option<unsafe extern "C" fn(*mut sock, bool, *mut sk_buff) -> c_int>,
    idiag_get_aux_size: Option<unsafe extern "C" fn(*mut sock, bool) -> usize>,
    idiag_type: c_int,
    idiag_info_size: usize,
    destroy: Option<unsafe extern "C" fn(*mut sk_buff, *const inet_diag_req_v2) -> c_int>,
}

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct netlink_callback {
    _private: [u8; 0],
}

#[repr(C)]
pub struct net {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tcp_hashinfo {
    _private: [u8; 0],
}

#[repr(C)]
pub struct inet_connection_sock {
    icsk_ulp_ops: *mut tcp_ulp_ops,
}

// Function implementations
#[no_mangle]
pub unsafe extern "C" fn tcp_diag_get_info(
    sk: *mut sock,
    r: *mut inet_diag_msg,
    _info: *mut c_void,
) {
    // SAFETY: Caller guarantees sk and r are valid pointers
    let sk = sk as *mut sock;
    let r = r as *mut inet_diag_msg;

    // Read socket state
    let state = {
        // SAFETY: Safe as per kernel's inet_sk_state_load implementation
        let sk = sk as *mut sock;
        let inet_sk = (sk as *mut inet_connection_sock).offset(1) as *mut sock;
        (*inet_sk).idiag_state
    };

    if state == 1 { // TCP_LISTEN
        // SAFETY: Safe as per kernel's READ_ONCE implementation
        (*r).idiag_rqueue = (*sk).sk_ack_backlog;
        (*r).idiag_wqueue = (*sk).sk_max_ack_backlog;
    } else if (*sk).sk_type == 1 { // SOCK_STREAM
        let tp = tcp_sk(sk);
        let rcv_nxt = (*tp).rcv_nxt;
        let copied_seq = (*tp).copied_seq;
        let write_seq = (*tp).write_seq;
        let snd_una = (*tp).snd_una;

        (*r).idiag_rqueue = (rcv_nxt - copied_seq).max(0) as u32;
        (*r).idiag_wqueue = (write_seq - snd_una) as u32;
    }

    if !_info.is_null() {
        // SAFETY: tcp_get_info is a kernel function that should be available
        tcp_get_info(sk, _info as *mut tcp_info);
    }
}

#[no_mangle]
pub unsafe extern "C" fn tcp_diag_md5sig_fill(
    info: *mut tcp_diag_md5sig,
    key: *const tcp_md5sig_key,
) {
    // SAFETY: Caller guarantees info and key are valid
    let info = info as *mut tcp_diag_md5sig;
    let key = key as *const tcp_md5sig_key;

    (*info).tcpm_family = (*key).family;
    (*info).tcpm_prefixlen = (*key).prefixlen;
    (*info).tcpm_keylen = (*key).keylen;

    // Copy key data
    ptr::copy_nonoverlapping((*key).key.as_ptr(), (*info).tcpm_key.as_mut_ptr(), (*key).keylen as usize);

    if (*key).family == 2 { // AF_INET
        (*info).tcpm_addr[0] = (*key).addr.a4;
    } else if (*key).family == 10 { // AF_INET6
        ptr::copy_nonoverlapping((*key).addr.a6.as_ptr(), (*info).tcpm_addr.as_mut_ptr(), 16);
    }
}

#[no_mangle]
pub unsafe extern "C" fn tcp_diag_put_md5sig(
    skb: *mut sk_buff,
    md5sig: *const tcp_md5sig_info,
) -> c_int {
    // Count entries
    let mut md5sig_count = 0;
    let mut node = (*md5sig).head.first;
    
    while !node.is_null() {
        md5sig_count += 1;
        node = (*node).next;
    }

    if md5sig_count == 0 {
        return 0;
    }

    // Reserve space
    let size = md5sig_count * mem::size_of::<tcp_diag_md5sig>();
    let attr = nla_reserve(skb, INET_DIAG_MD5SIG, size as size_t);
    
    if attr.is_null() {
        return -ENOMEM;
    }

    let info = nla_data(attr) as *mut tcp_diag_md5sig;
    ptr::write_bytes(info, 0, md5sig_count);

    let mut node = (*md5sig).head.first;
    let mut i = 0;

    while !node.is_null() && i < md5sig_count {
        let key = node as *const tcp_md5sig_key;
        tcp_diag_md5sig_fill(info.add(i), key);
        i += 1;
        node = (*node).next;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn tcp_diag_put_ulp(
    skb: *mut sk_buff,
    sk: *mut sock,
    ulp_ops: *const tcp_ulp_ops,
) -> c_int {
    let nest = nla_nest_start(skb, INET_DIAG_ULP_INFO);
    
    if nest.is_null() {
        return -ENOMEM;
    }

    let name = (*ulp_ops).name;
    let err = nla_put_string(skb, INET_ULP_INFO_NAME, name);
    
    if err != 0 {
        nla_nest_cancel(skb, nest);
        return err;
    }

    if let Some(get_info) = (*ulp_ops).get_info {
        let err = get_info(sk, skb);
        if err != 0 {
            nla_nest_cancel(skb, nest);
            return err;
        }
    }

    nla_nest_end(skb, nest);
    0
}

#[no_mangle]
pub unsafe extern "C" fn tcp_diag_get_aux(
    sk: *mut sock,
    net_admin: bool,
    skb: *mut sk_buff,
) -> c_int {
    let icsk = (sk as *mut inet_connection_sock).offset(1) as *mut sock;
    
    if net_admin {
        let tcp_sk = (sk as *mut tcp_sock).offset(1) as *mut sock;
        let md5sig_info = (*tcp_sk).md5sig_info;
        
        if !md5sig_info.is_null() {
            let err = tcp_diag_put_md5sig(skb, md5sig_info);
            if err < 0 {
                return err;
            }
        }
    }

    if net_admin {
        let ulp_ops = (*icsk).icsk_ulp_ops;
        
        if !ulp_ops.is_null() {
            let err = tcp_diag_put_ulp(skb, sk, ulp_ops);
            if err < 0 {
                return err;
            }
        }
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn tcp_diag_get_aux_size(
    sk: *mut sock,
    net_admin: bool,
) -> usize {
    let mut size = 0;
    let icsk = (sk as *mut inet_connection_sock).offset(1) as *mut sock;
    
    if net_admin && sk_fullsock(sk) {
        let tcp_sk = (sk as *mut tcp_sock).offset(1) as *mut sock;
        let md5sig_info = (*tcp_sk).md5sig_info;
        
        if !md5sig_info.is_null() {
            let mut md5sig_count = 0;
            let mut node = (*md5sig_info).head.first;
            
            while !node.is_null() {
                md5sig_count += 1;
                node = (*node).next;
            }
            
            size += nla_total_size(md5sig_count * mem::size_of::<tcp_diag_md5sig>()) as usize;
        }
    }

    if net_admin && sk_fullsock(sk) {
        let ulp_ops = (*icsk).icsk_ulp_ops;
        
        if !ulp_ops.is_null() {
            size += nla_total_size(0) as usize;
            size += nla_total_size(TCP_ULP_NAME_MAX) as usize;
            
            if let Some(get_info_size) = (*ulp_ops).get_info_size {
                size += get_info_size(sk);
            }
        }
    }

    size
}

#[no_mangle]
pub unsafe extern "C" fn tcp_diag_dump(
    skb: *mut sk_buff,
    cb: *mut netlink_callback,
    r: *const inet_diag_req_v2,
) {
    inet_diag_dump_icsk(&tcp_hashinfo, skb, cb, r);
}

#[no_mangle]
pub unsafe extern "C" fn tcp_diag_dump_one(
    cb: *mut netlink_callback,
    req: *const inet_diag_req_v2,
) -> c_int {
    inet_diag_dump_one_icsk(&tcp_hashinfo, cb, req)
}

#[no_mangle]
pub unsafe extern "C" fn tcp_diag_destroy(
    in_skb: *mut sk_buff,
    req: *const inet_diag_req_v2,
) -> c_int {
    let net = sock_net(in_skb as *mut sock);
    let sk = inet_diag_find_one_icsk(net, &tcp_hashinfo, req);
    
    if sk.is_null() {
        return -EINVAL;
    }
    
    let err = sock_diag_destroy(sk, ECONNABORTED);
    sock_gen_put(sk);
    
    err
}

// Static handler definition
#[no_mangle]
pub static tcp_diag_handler: inet_diag_handler = inet_diag_handler {
    dump: Some(tcp_diag_dump),
    dump_one: Some(tcp_diag_dump_one),
    idiag_get_info: Some(tcp_diag_get_info),
    idiag_get_aux: Some(tcp_diag_get_aux),
    idiag_get_aux_size: Some(tcp_diag_get_aux_size),
    idiag_type: IPPROTO_TCP,
    idiag_info_size: mem::size_of::<tcp_info>(),
    destroy: Some(tcp_diag_destroy),
};

#[no_mangle]
pub unsafe extern "C" fn tcp_diag_init() -> c_int {
    inet_diag_register(&tcp_diag_handler)
}

#[no_mangle]
pub unsafe extern "C" fn tcp_diag_exit() {
    inet_diag_unregister(&tcp_diag_handler)
}

// Helper functions (assumed to be available in the kernel)
#[link(name = "kernel")]
extern "C" {
    fn tcp_sk(sk: *mut sock) -> *mut tcp_sock;
    fn sk_fullsock(sk: *mut sock) -> bool;
    fn nla_reserve(skb: *mut sk_buff, attrtype: c_int, attrlen: size_t) -> *mut c_void;
    fn nla_data(attr: *mut c_void) -> *mut c_void;
    fn nla_nest_start(skb: *mut sk_buff, attrtype: c_int) -> *mut c_void;
    fn nla_nest_end(skb: *mut sk_buff, attr: *mut c_void);
    fn nla_nest_cancel(skb: *mut sk_buff, attr: *mut c_void);
    fn nla_put_string(skb: *mut sk_buff, attrtype: c_int, name: *const u8) -> c_int;
    fn nla_total_size(len: size_t) -> size_t;
    fn inet_diag_dump_icsk(hashinfo: *const tcp_hashinfo, skb: *mut sk_buff, cb: *mut netlink_callback, req: *const inet_diag_req_v2);
    fn inet_diag_dump_one_icsk(hashinfo: *const tcp_hashinfo, cb: *mut netlink_callback, req: *const inet_diag_req_v2) -> c_int;
    fn sock_diag_destroy(sk: *mut sock, err: c_int) -> c_int;
    fn sock_gen_put(sk: *mut sock);
    fn sock_net(sk: *mut sock) -> *mut net;
    fn inet_diag_find_one_icsk(net: *mut net, hashinfo: *const tcp_hashinfo, req: *const inet_diag_req_v2) -> *mut sock;
    fn inet_diag_register(handler: *const inet_diag_handler) -> c_int;
    fn inet_diag_unregister(handler: *const inet_diag_handler);
    fn tcp_get_info(sk: *mut sock, info: *mut tcp_info);
}

// Module initialization
#[no_mangle]
pub unsafe extern "C" fn init_module() -> c_int {
    tcp_diag_init()
}

#[no_mangle]
pub unsafe extern "C" fn cleanup_module() {
    tcp_diag_exit()
}
