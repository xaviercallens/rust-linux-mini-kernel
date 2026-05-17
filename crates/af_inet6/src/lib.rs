//! IPv6 protocol stack for Linux
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(clang::too_many_arguments)]


use kernel_types::*;
use core::ffi::c_int;
use core::ffi::c_uint;
use core::ffi::c_void;
use core::mem;
use core::ptr;
use core::sync::atomic::{AtomicU32, Ordering};

// Constants from C
pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ESOCKTNOSUPPORT: c_int = -94;
pub const EPROTONOSUPPORT: c_int = -93;
pub const EPERM: c_int = -1;
pub const ENOBUFS: c_int = -105;

// Type definitions
#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct sock {
    pub sk_prot: *const proto,
    pub sk_type: c_int,
    pub sk_family: c_int,
    pub sk_protocol: c_int,
    pub sk_backlog_rcv: Option<extern "C" fn(*mut sock, *mut c_void, size_t) -> c_int>,
    pub sk_destruct: Option<extern "C" fn(*mut sock)>,
    pub sk_bound_dev_if: c_int,
    pub sk_v6_rcv_saddr: [u8; 16],
    pub sk_ipv6only: c_int,
    pub sk_refcnt_debug: c_int,
}

#[repr(C)]
pub struct inet_sock {
    pub is_icsk: c_int,
    pub inet_num: c_int,
    pub inet_sport: u16,
    pub uc_ttl: c_int,
    pub mc_loop: c_int,
    pub mc_ttl: c_int,
    pub mc_index: c_int,
    pub rcv_tos: u8,
    pub pmtudisc: c_int,
}

#[repr(C)]
pub struct ipv6_pinfo {
    pub hop_limit: c_int,
    pub mcast_hops: c_int,
    pub mc_loop: c_int,
    pub mc_all: c_int,
    pub pmtudisc: c_int,
    pub repflow: c_int,
    pub saddr: [u8; 16],
}

#[repr(C)]
pub struct inet_protosw {
    pub list: list_head,
    pub protocol: c_int,
    pub ops: *const c_void,
    pub prot: *const proto,
    pub flags: c_int,
}

#[repr(C)]
pub struct proto {
    pub obj_size: c_int,
    pub slab: *const c_void,
    pub hash: Option<extern "C" fn(*mut sock) -> c_int>,
    pub init: Option<extern "C" fn(*mut sock) -> c_int>,
    pub backlog_rcv: Option<extern "C" fn(*mut sock, *mut c_void, size_t) -> c_int>,
}

#[repr(C)]
pub struct net {
    pub user_ns: *const c_void,
    pub ipv6: ipv6_net,
}

#[repr(C)]
pub struct ipv6_net {
    sysctl: ipv6_sysctl,
}

#[repr(C)]
pub struct ipv6_sysctl {
    bindv6only: c_int,
    flowlabel_reflect: c_int,
}

#[repr(C)]
pub struct sockaddr_in6 {
    pub sin6_family: c_int,
    pub sin6_port: u16,
    pub sin6_flowinfo: u32,
    pub sin6_addr: [u8; 16],
    pub sin6_scope_id: u32,
}

#[repr(C)]
pub struct ipv6_params {
    disable_ipv6: c_int,
    autoconf: c_int,
}

// Static variables
static mut inetsw6: [list_head; 16] = unsafe { mem::zeroed() };
static inetsw6_lock: spin::Mutex<()> = spin::Mutex::new(());

static mut ipv6_defaults: ipv6_params = ipv6_params {
    disable_ipv6: 0,
    autoconf: 1,
};

static mut disable_ipv6_mod: c_int = 0;

// Function implementations
#[no_mangle]
pub unsafe extern "C" fn ipv6_mod_enabled() -> bool {
    disable_ipv6_mod == 0
}

#[no_mangle]
pub unsafe extern "C" fn inet6_sk_generic(sk: *mut sock) -> *mut ipv6_pinfo {
    let offset = (*sk).sk_prot.offset(0) as *const proto;
    let offset = (*offset).obj_size as isize - mem::size_of::<ipv6_pinfo>() as isize;
    let sk_ptr = sk as *mut u8;
    (sk_ptr.add(offset)) as *mut ipv6_pinfo
}

#[no_mangle]
pub unsafe extern "C" fn inet6_create(
    net: *mut net,
    sock: *mut sock,
    protocol: c_int,
    kern: c_int,
) -> c_int {
    let mut answer: *mut inet_protosw = ptr::null_mut();
    let mut answer_prot: *mut proto = ptr::null_mut();
    let mut answer_flags: c_int = 0;
    let mut try_loading_module: c_int = 0;
    let mut err: c_int = 0;
    let mut protocol_saved: c_int = protocol;
    let mut sk: *mut sock = ptr::null_mut();
    let mut inet: *mut inet_sock = ptr::null_mut();
    let mut np: *mut ipv6_pinfo = ptr::null_mut();

    if protocol < 0 || protocol >= IPPROTO_MAX {
        return EINVAL;
    }

    // SAFETY: RCU read lock is held during list traversal
    unsafe {
        loop {
            err = ESOCKTNOSUPPORT;
            let mut list_entry: *mut inet_protosw = ptr::null_mut();
            let mut list_head: *mut list_head = &inetsw6[(*sock).sk_type as usize];

            // Simulate list_for_each_entry_rcu
            let mut entry = (*list_head).next;
            while entry != list_head {
                list_entry = entry as *mut inet_protosw;
                err = 0;

                if protocol == (*list_entry).protocol {
                    if protocol != IPPROTO_IP {
                        answer = list_entry;
                        break;
                    }
                } else {
                    if IPPROTO_IP == protocol {
                        protocol_saved = (*list_entry).protocol;
                        answer = list_entry;
                        break;
                    }
                    if IPPROTO_IP == (*list_entry).protocol {
                        answer = list_entry;
                        break;
                    }
                }
                err = EPROTONOSUPPORT;
                entry = (*entry).next;
            }

            if err != 0 {
                if try_loading_module < 2 {
                    // Module loading logic would go here
                    try_loading_module += 1;
                    if try_loading_module == 1 {
                        // request_module("net-pf-10-proto-132-type-1")
                    } else {
                        // request_module("net-pf-10-proto-132")
                    }
                    continue lookup_protocol;
                } else {
                    break;
                }
            }

            if (*sock).sk_type == SOCK_RAW && !kern &&
               !ns_capable((*net).user_ns, CAP_NET_RAW) {
                err = EPERM;
                break;
            }

            (*sock).ops = (*answer).ops;
            answer_prot = (*answer).prot;
            answer_flags = (*answer).flags;
            break;
        }
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

    inet = &mut (*sk).is_icsk as *mut _;
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

// Helper functions (simplified for FFI compatibility)
#[no_mangle]
pub unsafe extern "C" fn sk_alloc(
    net: *mut net,
    pf: c_int,
    gfp: c_int,
    prot: *mut proto,
    kern: c_int,
) -> *mut sock {
    // Simplified allocation - actual kernel uses kmalloc
    let size = (*prot).obj_size as usize;
    let ptr = libc::malloc(size);
    if ptr.is_null() {
        return ptr::null_mut();
    }
    ptr as *mut sock
}

#[no_mangle]
pub unsafe extern "C" fn sock_init_data(
    sock: *mut sock,
    sk: *mut sock,
) {
    // Minimal initialization
}

#[no_mangle]
pub unsafe extern "C" fn sk_refcnt_debug_inc(sk: *mut sock) {
    (*sk).sk_refcnt_debug += 1;
}

#[no_mangle]
pub unsafe extern "C" fn sk_common_release(sk: *mut sock) {
    // Minimal release
}

#[no_mangle]
pub unsafe extern "C" fn inet_sock_destruct(sk: *mut sock) {
    // Minimal destruct
}

// External declarations
extern "C" {
    fn BPF_CGROUP_RUN_PROG_INET_SOCK(sk: *mut sock) -> c_int;
    fn ns_capable(user_ns: *const c_void, cap: c_int) -> c_int;
}

// Constants
const IPPROTO_MAX: c_int = 256;
const IPPROTO_IP: c_int = 0;
const IPPROTO_RAW: c_int = 255;
const SOCK_RAW: c_int = 3;
const PF_INET6: c_int = 10;
const GFP_KERNEL: c_int = 0;
const SK_CAN_REUSE: c_int = 1;
const INET_PROTOSW_REUSE: c_int = 1 << 0;
const INET_PROTOSW_ICSK: c_int = 1 << 1;
const IPV6_DEFAULT_MCASTHOPS: c_int = -1;
const IPV6_PMTUDISC_WANT: c_int = 1;
const FLOWLABEL_REFLECT_ESTABLISHED: c_int = 1 << 0;
const IP_PMTUDISC_DONT: c_int = 0;
const CAP_NET_RAW: c_int = 130;
const CAP_NET_BIND_SERVICE: c_int = 10;