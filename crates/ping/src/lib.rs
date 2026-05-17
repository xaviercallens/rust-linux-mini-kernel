//! IPv6 Ping Sockets Implementation
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use core::ffi::c_void;
use core::mem;
use core::ptr;
use kernel_types::*;

// Constants from C
pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ENOSYS: c_int = -38;
pub const EAFNOSUPPORT: c_int = -125;
pub const EDESTADDRREQ: c_int = -39;

// Type definitions
#[repr(C)]
#[derive(Copy, Clone)]
pub struct in6_addr {
    pub s6_addr: [u8; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_in6 {
    pub sin6_family: u16,
    pub sin6_port: u16,
    pub sin6_flowinfo: u32,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pingfakehdr {
    pub icmph: icmp6hdr,
    pub msg: *mut msghdr,
    pub wcheck: u16,
    pub family: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icmp6hdr {
    pub icmp6_type: u8,
    pub icmp6_code: u8,
    pub checksum: u16,
    pub un: icmp6_echo,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icmp6_echo {
    pub id: u16,
    pub sequence: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct proto {
    pub name: *const u8,
    pub owner: *mut c_void,
    pub init: extern "C" fn(*mut sock) -> c_int,
    pub close: extern "C" fn(*mut sock, c_int),
    pub connect: extern "C" fn(*mut sock, *const sockaddr_in6, socklen_t, c_int) -> c_int,
    pub disconnect: extern "C" fn(*mut sock, c_int),
    pub setsockopt: extern "C" fn(*mut sock, c_int, c_int, *const c_void, socklen_t) -> c_int,
    pub getsockopt: extern "C" fn(*mut sock, c_int, c_int, *mut c_void, *mut socklen_t) -> c_int,
    pub sendmsg: extern "C" fn(*mut sock, *mut msghdr, size_t) -> c_int,
    pub recvmsg: extern "C" fn(*mut sock, *mut msghdr, size_t, c_int) -> c_int,
    pub bind: extern "C" fn(*mut sock, *const sockaddr_in6, socklen_t) -> c_int,
    pub backlog_rcv: extern "C" fn(*mut sock, *mut sk_buff) -> c_int,
    pub hash: extern "C" fn(*mut sock),
    pub unhash: extern "C" fn(*mut sock),
    pub get_port: extern "C" fn(*mut sock, u16) -> c_int,
    pub obj_size: size_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inet_protosw {
    pub type_: c_int,
    pub protocol: c_int,
    pub prot: *mut proto,
    pub ops: *mut c_void,
    pub flags: c_int,
}

// Function implementations

/// Dummy IPv6 receive error handler
///
/// # Safety
/// This function is a compatibility stub and should not be called directly.
#[no_mangle]
pub unsafe extern "C" fn dummy_ipv6_recv_error(
    sk: *mut sock,
    msg: *mut msghdr,
    len: c_int,
    addr_len: *mut c_int,
) -> c_int {
    -EAFNOSUPPORT
}

/// Dummy IPv6 datagram receive control
///
/// # Safety
/// This function is a compatibility stub and should not be called directly.
#[no_mangle]
pub unsafe extern "C" fn dummy_ip6_datagram_recv_ctl(
    sk: *mut sock,
    msg: *mut msghdr,
    skb: *mut sk_buff,
) {
    // No-op
}

/// Dummy ICMPv6 error conversion
///
/// # Safety
/// This function is a compatibility stub and should not be called directly.
#[no_mangle]
pub unsafe extern "C" fn dummy_icmpv6_err_convert(type_: u8, code: u8, err: *mut c_int) -> c_int {
    -EAFNOSUPPORT
}

/// Dummy IPv6 ICMP error handler
///
/// # Safety
/// This function is a compatibility stub and should not be called directly.
#[no_mangle]
pub unsafe extern "C" fn dummy_ipv6_icmp_error(
    sk: *mut sock,
    skb: *mut sk_buff,
    err: c_int,
    port: u16,
    info: u32,
    payload: *mut u8,
) {
    // No-op
}

/// Dummy IPv6 address check
///
/// # Safety
/// This function is a compatibility stub and should not be called directly.
#[no_mangle]
pub unsafe extern "C" fn dummy_ipv6_chk_addr(
    net: *mut net,
    addr: *const in6_addr,
    dev: *const net_device,
    strict: c_int,
) -> c_int {
    0
}

/// Send IPv6 ping message
///
/// # Safety
/// - `sk` must be a valid pointer to a socket
/// - `msg` must be a valid message header
/// - Caller must ensure proper synchronization
#[no_mangle]
pub unsafe extern "C" fn ping_v6_sendmsg(sk: *mut sock, msg: *mut msghdr, len: size_t) -> c_int {
    let inet = inet_sk(sk);
    let np = inet6_sk(sk);
    let mut user_icmph: icmp6hdr = mem::zeroed();
    let mut err: c_int = 0;
    let mut daddr: *const in6_addr = ptr::null();
    let mut oif: c_int = 0;
    let mut fl6: flowi6 = mem::zeroed();
    let mut dst: *mut dst_entry = ptr::null_mut();
    let mut rt: *mut rt6_info = ptr::null_mut();
    let mut pfh: pingfakehdr = mem::zeroed();
    let mut ipc6: ipcm6_cookie = mem::zeroed();

    // SAFETY: ping_common_sendmsg is a kernel function that fills user_icmph
    err = ping_common_sendmsg(
        10,
        msg,
        len,
        &mut user_icmph as *mut _,
        core::mem::size_of_val(&user_icmph),
    ) as c_int;
    if err < 0 {
        return err;
    }

    if !msg.is_null() && (*msg).msg_name.is_some() {
        let u = (*msg).msg_name.unwrap() as *mut sockaddr_in6;
        if (*msg).msg_namelen < core::mem::size_of::<sockaddr_in6>() as socklen_t {
            return -EINVAL;
        }
        if (*u).sin6_family != 10 {
            return -EAFNOSUPPORT;
        }
        daddr = &(*u).sin6_addr;
        if ipv6_addr_needs_scope_id(ipv6_addr_type(daddr)) != 0 {
            oif = (*u).sin6_scope_id;
        }
    } else {
        if (*sk).sk_state != 1 {
            return -EDESTADDRREQ;
        }
        daddr = &(*sk).sk_v6_daddr;
    }

    if oif == 0 {
        oif = (*sk).sk_bound_dev_if;
    }

    if oif == 0 {
        oif = (*np).sticky_pktinfo.ipi6_ifindex;
    }

    if oif == 0 && ipv6_addr_is_multicast(daddr) != 0 {
        oif = (*np).mcast_oif;
    } else if oif == 0 {
        oif = (*np).ucast_oif;
    }

    let addr_type = ipv6_addr_type(daddr);
    if (ipv6_addr_needs_scope_id(addr_type) != 0 && oif == 0)
        || (addr_type & (1 << 30) != 0)
        || (oif != 0 && (*sk).sk_bound_dev_if != 0 && oif != (*sk).sk_bound_dev_if)
    {
        return -EINVAL;
    }

    // Initialize flowi6
    fl6.flowi6_proto = 58;
    fl6.saddr = (*np).saddr;
    fl6.daddr = *daddr;
    fl6.flowi6_oif = oif;
    fl6.flowi6_mark = (*sk).sk_mark;
    fl6.flowi6_uid = (*sk).sk_uid;
    fl6.fl6_icmp_type = user_icmph.icmp6_type;
    fl6.fl6_icmp_code = user_icmph.icmp6_code;
    security_sk_classify_flow(sk, &mut fl6 as *mut _ as *mut _);

    ipcm6_init_sk(&mut ipc6, np);
    ipc6.sockc.mark = (*sk).sk_mark;
    fl6.flowlabel = ip6_make_flowinfo(ipc6.tclass, fl6.flowlabel);

    dst = ip6_sk_dst_lookup_flow(sk, &mut fl6 as *mut _, daddr, false as c_int);
    if dst.is_null() {
        return -ENOMEM;
    }
    rt = dst as *mut rt6_info;

    if fl6.flowi6_oif == 0 && ipv6_addr_is_multicast(&fl6.daddr) != 0 {
        fl6.flowi6_oif = (*np).mcast_oif;
    } else if fl6.flowi6_oif == 0 {
        fl6.flowi6_oif = (*np).ucast_oif;
    }

    pfh.icmph.icmp6_type = user_icmph.icmp6_type;
    pfh.icmph.icmp6_code = user_icmph.icmp6_code;
    pfh.icmph.checksum = 0;
    pfh.icmph.un.echo.id = (*inet).inet_sport;
    pfh.icmph.un.echo.sequence = user_icmph.icmp6_sequence;
    pfh.msg = msg;
    pfh.wcheck = 0;
    pfh.family = 10;

    ipc6.hlimit = ip6_sk_dst_hoplimit(np, &mut fl6 as *mut _ as *mut _, dst);

    lock_sock(sk);
    err = ip6_append_data(
        sk,
        ping_getfrag,
        &mut pfh as *mut _ as *mut c_void,
        len,
        0,
        &mut ipc6 as *mut _ as *mut c_void,
        &mut fl6 as *mut _ as *mut c_void,
        rt,
        1,
    );
    if err < 0 {
        ICMP6_INC_STATS(sock_net(sk), (*rt).rt6i_idev, ICMP6_MIB_OUTERRORS);
        ip6_flush_pending_frames(sk);
    } else {
        icmpv6_push_pending_frames(
            sk,
            &mut fl6 as *mut _ as *mut c_void,
            &mut pfh.icmph as *mut _ as *mut c_void,
            len,
        );
    }
    release_sock(sk);

    dst_release(dst);

    if err < 0 {
        return err;
    }

    len as c_int
}

#[repr(C)]
pub static mut pingv6_prot: proto = proto {
    name: b"PINGv6\0".as_ptr() as *const u8,
    owner: ptr::null_mut(),
    init: ping_init_sock,
    close: ping_close,
    connect: ip6_datagram_connect_v6_only,
    disconnect: __udp_disconnect,
    setsockopt: ipv6_setsockopt,
    getsockopt: ipv6_getsockopt,
    sendmsg: ping_v6_sendmsg,
    recvmsg: ping_recvmsg,
    bind: ping_bind,
    backlog_rcv: ping_queue_rcv_skb,
    hash: ping_hash,
    unhash: ping_unhash,
    get_port: ping_get_port,
    obj_size: core::mem::size_of::<raw6_sock>() as size_t,
};

#[no_mangle]
pub static mut pingv6_protosw: inet_protosw = inet_protosw {
    type_: 2,
    protocol: 58,
    prot: &mut pingv6_prot as *mut _,
    ops: &mut inet6_sockraw_ops as *mut _,
    flags: INET_PROTOSW_REUSE,
};

#[cfg(feature = "proc_fs")]
mod proc {
    use super::*;

    #[no_mangle]
    pub unsafe extern "C" fn ping_v6_seq_start(seq: *mut c_void, pos: *mut loff_t) -> *mut c_void {
        ping_seq_start(seq, pos, 10)
    }

    #[no_mangle]
    pub unsafe extern "C" fn ping_v6_seq_show(seq: *mut c_void, v: *mut c_void) -> c_int {
        if v.is_null() {
            seq_puts(seq, b"IPV6_SEQ_DGRAM_HEADER\0".as_ptr() as *const u8);
        } else {
            let state = (*seq).private as *mut ping_iter_state;
            let bucket = (*state).bucket;
            let inet = inet_sk(v);
            let srcp = ntohs((*inet).inet_sport);
            let destp = ntohs((*inet).inet_dport);
            ip6_dgram_sock_seq_show(seq, v, srcp, destp, bucket);
        }
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn pingv6_init() -> c_int {
    #[cfg(feature = "proc_fs")]
    {
        let ret = register_pernet_subsys(&mut ping_v6_net_ops as *mut _);
        if ret < 0 {
            return ret;
        }
    }

    pingv6_ops.ipv6_recv_error = ipv6_recv_error;
    pingv6_ops.ip6_datagram_recv_common_ctl = ip6_datagram_recv_common_ctl;
    pingv6_ops.ip6_datagram_recv_specific_ctl = ip6_datagram_recv_specific_ctl;
    pingv6_ops.icmpv6_err_convert = icmpv6_err_convert;
    pingv6_ops.ipv6_icmp_error = ipv6_icmp_error;
    pingv6_ops.ipv6_chk_addr = ipv6_chk_addr;

    inet6_register_protosw(&mut pingv6_protosw as *mut _)
}

#[no_mangle]
pub unsafe extern "C" fn pingv6_exit() {
    pingv6_ops.ipv6_recv_error = dummy_ipv6_recv_error;
    pingv6_ops.ip6_datagram_recv_common_ctl = dummy_ip6_datagram_recv_ctl;
    pingv6_ops.ip6_datagram_recv_specific_ctl = dummy_ip6_datagram_recv_ctl;
    pingv6_ops.icmpv6_err_convert = dummy_icmpv6_err_convert;
    pingv6_ops.ipv6_icmp_error = dummy_ipv6_icmp_error;
    pingv6_ops.ipv6_chk_addr = dummy_ipv6_chk_addr;

    #[cfg(feature = "proc_fs")]
    {
        unregister_pernet_subsys(&mut ping_v6_net_ops as *mut _);
    }

    inet6_unregister_protosw(&mut pingv6_protosw as *mut _);
}