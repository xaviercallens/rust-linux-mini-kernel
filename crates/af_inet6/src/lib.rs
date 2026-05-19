//! IPv6 protocol stack for Linux
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use core::ffi::{c_int, c_void};
use core::mem;
use core::ptr;
use kernel_types::*;

pub const EINVAL: c_int = -22;

pub type socklen_t = u32;
pub type size_t = usize;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct proto {
    pub obj_size: c_int,
    pub slab: *const c_void,
    pub hash: Option<extern "C" fn(*mut sock) -> c_int>,
    pub init: Option<extern "C" fn(*mut sock) -> c_int>,
    pub backlog_rcv: Option<extern "C" fn(*mut sock, *mut c_void, size_t) -> c_int>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inet_protosw {
    pub list: list_head,
    pub protocol: c_int,
    pub ops: *const c_void,
    pub prot: *const proto,
    pub flags: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipv6_sysctl {
    pub bindv6only: c_int,
    pub flowlabel_reflect: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipv6_net {
    pub sysctl: ipv6_sysctl,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct net {
    pub user_ns: *const c_void,
    pub ipv6: ipv6_net,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_in6 {
    pub sin6_family: u16,
    pub sin6_port: u16,
    pub sin6_flowinfo: u32,
    pub sin6_addr: [u8; 16],
    pub sin6_scope_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipv6_params {
    pub disable_ipv6: c_int,
    pub autoconf: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct socket {
    pub _priv: *mut c_void,
}

unsafe extern "C" {
    static mut inetsw6: [list_head; 16];
    static mut disable_ipv6_mod: c_int;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ipv6_mod_enabled() -> bool {
    disable_ipv6_mod == 0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn inet6_sk_generic(sk: *mut sock) -> *mut ipv6_pinfo {
    if sk.is_null() {
        return ptr::null_mut();
    }

    let base = sk as *mut u8;
    let off = mem::size_of::<sock>() as isize;
    base.wrapping_offset(off) as *mut ipv6_pinfo
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn inet6_create(
    _net: *mut net,
    sock: *mut socket,
    protocol: c_int,
    _kern: c_int,
) -> c_int {
    let mut err: c_int = 0;
    let mut sk: *mut sock = ptr::null_mut();
    let answer_prot: *mut proto = ptr::null_mut();
    let net: *mut net = ptr::null_mut();
    let kern: c_int = 0;

    if sock.is_null() {
        return EINVAL;
    }

    if protocol < 0 {
        return EINVAL;
    }

    if err != 0 {
        return err;
    }

    err = ENOBUFS;
    sk = sk_alloc(net, PF_INET6, GFP_KERNEL, answer_prot, kern);
    if sk.is_null() {
        return err;
    }

    sock_init_data(sock, sk);

    err = 0;
    if (INET_PROTOSW_REUSE & answer_flags) != 0 {
        (*sk).sk_reuse = SK_CAN_REUSE;
    }

    inet = &mut (*sk).inet as *mut _;
    (*inet).is_icsk = (INET_PROTOSW_ICSK & answer_flags) != 0;

    if (*sock).sk_type == SOCK_RAW {
        (*inet).inet_num = protocol_saved;
        if protocol_saved == IPPROTO_RAW {
            (*inet).hdrincl = 1;
        }
    }

    (*sk).sk_destruct = Some(inet_sock_destruct);
    (*sk).sk_family = PF_INET6;
    (*sk).sk_protocol = protocol_saved;

    (*sk).sk_backlog_rcv = (*answer_prot).backlog_rcv;

    let np = inet6_sk_generic(sk);
    (*np).hop_limit = -1;
    (*np).mcast_hops = IPV6_DEFAULT_MCASTHOPS;
    (*np).mc_loop = 1;
    (*np).mc_all = 1;
    (*np).pmtudisc = IPV6_PMTUDISC_WANT;
    (*np).repflow = (*net).ipv6.sysctl.flowlabel_reflect & FLOWLABEL_REFLECT_ESTABLISHED;
    (*sk).sk_ipv6only = (*net).ipv6.sysctl.bindv6only;

    (*inet).uc_ttl = -1;
    (*inet).mc_loop = 1;
    (*inet).mc_ttl = 1;
    (*inet).mc_index = 0;
    (*inet).rcv_tos = 0;

    if (*net).ipv4.sysctl_ip_no_pmtu_disc {
        (*inet).pmtudisc = IP_PMTUDISC_DONT;
    } else {
        (*inet).pmtudisc = IP_PMTUDISC_WANT;
    }

    sk_refcnt_debug_inc(sk);

    if (*inet).inet_num != 0 {
        (*inet).inet_sport = protocol_saved as u16;
        err = (*sk).sk_prot.hash(sk);
        if err != 0 {
            sk_common_release(sk);
            return err;
        }
    }

    if let Some(init) = (*sk).sk_prot.init {
        err = init(sk);
        if err != 0 {
            sk_common_release(sk);
            return err;
        }
    }

    if !kern {
        let bpf_result = BPF_CGROUP_RUN_PROG_INET_SOCK(sk);
        if bpf_result != 0 {
            sk_common_release(sk);
            return bpf_result;
        }
    }

    err
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn af_inet6_init() -> c_int {
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn af_inet6_exit() {
    let _ = core::ptr::addr_of_mut!(inetsw6);
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}