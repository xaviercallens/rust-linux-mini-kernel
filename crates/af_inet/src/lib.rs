//! IPv4 Socket Handling for Linux Kernel
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(clang::too_many_arguments)]

use kernel_types::*;
use core::ptr;
use core::ffi::c_int;
use core::ffi::c_void;
use core::ffi::size_t;

// Constants from C
pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ESOCKTNOSUPPORT: c_int = -94;
pub const EPROTONOSUPPORT: c_int = -93;
pub const EPERM: c_int = -1;
pub const ENOBUFS: c_int = -55;

// Type definitions
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sock {
    sk_receive_queue: skb_queue_head_t,
    sk_rx_skb_cache: *mut sk_buff,
    sk_error_queue: skb_queue_head_t,
    sk_type: c_int,
    sk_state: c_int,
    sk_max_ack_backlog: c_int,
    sk_dst_cache: *mut dst_entry,
    sk_rx_dst: *mut dst_entry,
    sk_rmem_alloc: atomic_t,
    sk_wmem_alloc: refcount_t,
    sk_wmem_queued: size_t,
    sk_forward_alloc: size_t,
    sk_backlog_rcv: *mut c_void,
    sk_prot: *mut proto,
    sk_destruct: unsafe extern "C" fn(*mut sock),
    sk_protocol: c_int,
    sk_users: atomic_t,
    sk_refcnt: atomic_t,
    sk_shutdown: c_int,
    sk_no_check: c_int,
    sk_lingertime: c_int,
    sk_reuse: c_int,
    sk_bound_dev_if: c_int,
    sk_bind_mark: c_int,
    sk_priority: c_int,
    sk_rcvlowat: size_t,
    sk_rcvtimeo: c_int,
    sk_sndtimeo: c_int,
    sk_linger: linger,
    sk_info_cache: *mut c_void,
    sk_prot_creator: *mut proto,
    sk_wq: *mut wait_queue_head_t,
    sk_user_data: *mut c_void,
    sk_clockid: c_int,
    sk_flags: c_int,
    sk_tsflags: c_int,
    sk_peek_off: size_t,
    sk_rxhash: c_int,
    sk_filter: *mut c_void,
    sk_timer: timer_list,
    sk_stamp: timeval,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inet_protosw {
    list: list_head,
    protocol: c_int,
    ops: *mut socket_ops,
    prot: *mut proto,
    flags: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct list_head {
    next: *mut list_head,
    prev: *mut list_head,
}

// Function implementations

/// Socket destructor for IPv4 sockets
///
/// # Safety
/// - `sk` must be a valid pointer to a sock structure
/// - The socket must be in the correct state for destruction
#[no_mangle]
pub unsafe extern "C" fn inet_sock_destruct(sk: *mut sock) {
    let inet = inet_sk(sk);

    // Purge receive queue
    __skb_queue_purge(&(*sk).sk_receive_queue);

    // Free cached skb
    if !(*sk).sk_rx_skb_cache.is_null() {
        __kfree_skb((*sk).sk_rx_skb_cache);
        (*sk).sk_rx_skb_cache = ptr::null_mut();
    }

    // Purge error queue
    __skb_queue_purge(&(*sk).sk_error_queue);

    // Reclaim memory
    sk_mem_reclaim(sk);

    // Validate state for TCP sockets
    if (*sk).sk_type == SOCK_STREAM && (*sk).sk_state != TCP_CLOSE {
        pr_err("Attempt to release TCP socket in state %d %p\n", (*sk).sk_state, sk);
        return;
    }

    if !sock_flag(sk, SOCK_DEAD) {
        pr_err("Attempt to release alive inet socket %p\n", sk);
        return;
    }

    // Debug checks
    assert!(atomic_read(&(*sk).sk_rmem_alloc) == 0);
    assert!(refcount_read(&(*sk).sk_wmem_alloc) == 0);
    assert!((*sk).sk_wmem_queued == 0);
    assert!((*sk).sk_forward_alloc == 0);

    // Free options
    kfree(rcu_dereference_protected(inet.inet_opt, 1));

    // Release destination caches
    dst_release(rcu_dereference_protected((*sk).sk_dst_cache, 1));
    dst_release((*sk).sk_rx_dst);

    // Final cleanup
    sk_refcnt_debug_dec(sk);
}

/// Move a socket into listening state
///
/// # Safety
/// - `sock` must be a valid pointer to a socket structure
/// - `backlog` must be a valid backlog size
#[no_mangle]
pub unsafe extern "C" fn inet_listen(sock: *mut socket, backlog: c_int) -> c_int {
    let sk = (*sock).sk;
    let mut err: c_int = 0;
    let mut old_state: c_int = 0;
    let mut tcp_fastopen: c_int = 0;

    lock_sock(sk);

    err = -EINVAL;
    if (*sock).state != SS_UNCONNECTED || (*sock).type_field != SOCK_STREAM {
        release_sock(sk);
        return err;
    }

    old_state = (*sk).sk_state;
    if !((1 << old_state) & (TCPF_CLOSE | TCPF_LISTEN)) {
        release_sock(sk);
        return err;
    }

    (*sk).sk_max_ack_backlog = backlog;

    if old_state != TCP_LISTEN {
        // Enable TFO w/o requiring TCP_FASTOPEN socket option
        tcp_fastopen = sock_net(sk).ipv4.sysctl_tcp_fastopen;
        if (tcp_fastopen & TFO_SERVER_WO_SOCKOPT1) != 0 &&
           (tcp_fastopen & TFO_SERVER_ENABLE) != 0 &&
           inet_csk(sk).icsk_accept_queue.fastopenq.max_qlen == 0 {
            fastopen_queue_tune(sk, backlog);
            tcp_fastopen_init_key_once(sock_net(sk));
        }

        err = inet_csk_listen_start(sk, backlog);
        if err != 0 {
            release_sock(sk);
            return err;
        }
        tcp_call_bpf(sk, BPF_SOCK_OPS_TCP_LISTEN_CB, 0, ptr::null_mut());
    }
    err = 0;

    release_sock(sk);
    return err;
}

/// Create an inet socket
///
/// # Safety
/// - `net` must be a valid network namespace
/// - `sock` must be a valid socket pointer
/// - `protocol` must be a valid protocol number
/// - `kern` must be a valid boolean flag
#[no_mangle]
pub unsafe extern "C" fn inet_create(
    net: *mut net,
    sock: *mut socket,
    protocol: c_int,
    kern: c_int,
) -> c_int {
    let mut sk: *mut sock = ptr::null_mut();
    let mut answer: *mut inet_protosw = ptr::null_mut();
    let mut answer_prot: *mut proto = ptr::null_mut();
    let mut answer_flags: c_int = 0;
    let mut try_loading_module: c_int = 0;
    let mut err: c_int = 0;

    if protocol < 0 || protocol >= IPPROTO_MAX {
        return -EINVAL;
    }

    (*sock).state = SS_UNCONNECTED;

    err = -ESOCKTNOSUPPORT;
    rcu_read_lock();
    let mut found = false;
    let mut list = &inetsw[(*sock).type_field];
    while !found {
        if list.next == list {
            break;
        }
        answer = container_of(list.next, inet_protosw, list);
        list = list.next;

        err = 0;
        // Check the non-wild match
        if protocol == (*answer).protocol {
            if protocol != IPPROTO_IP {
                found = true;
                break;
            }
        } else {
            // Check for the two wild cases
            if IPPROTO_IP == protocol {
                protocol = (*answer).protocol;
                found = true;
                break;
            }
            if IPPROTO_IP == (*answer).protocol {
                found = true;
                break;
            }
            err = -EPROTONOSUPPORT;
        }
    }

    if unlikely(err != 0) {
        if try_loading_module < 2 {
            rcu_read_unlock();
            if try_loading_module == 1 {
                request_module("net-pf-%d-proto-%d-type-%d",
                               PF_INET, protocol, (*sock).type_field);
            } else {
                request_module("net-pf-%d-proto-%d",
                               PF_INET, protocol);
            }
            try_loading_module += 1;
            rcu_read_lock();
            continue;
        } else {
            rcu_read_unlock();
            return err;
        }
    }

    err = -EPERM;
    if (*sock).type_field == SOCK_RAW && kern == 0 &&
       !ns_capable((*net).user_ns, CAP_NET_RAW) {
        rcu_read_unlock();
        return err;
    }

    (*sock).ops = (*answer).ops;
    answer_prot = (*answer).prot;
    answer_flags = (*answer).flags;
    rcu_read_unlock();

    assert!(!(*answer_prot).slab.is_null());

    err = -ENOBUFS;
    sk = sk_alloc(net, PF_INET, GFP_KERNEL, answer_prot, kern);
    if sk.is_null() {
        return err;
    }

    err = 0;
    if INET_PROTOSW_REUSE & answer_flags != 0 {
        (*sk).sk_reuse = SK_CAN_REUSE;
    }

    let inet = inet_sk(sk);
    inet.is_icsk = (INET_PROTOSW_ICSK & answer_flags) != 0;
    inet.nodefrag = 0;

    if (*sock).type_field == SOCK_RAW {
        inet.inet_num = protocol;
        if protocol == IPPROTO_RAW {
            inet.hdrincl = 1;
        }
    }

    if (*net).ipv4.sysctl_ip_no_pmtu_disc != 0 {
        inet.pmtudisc = IP_PMTUDISC_DONT;
    } else {
        inet.pmtudisc = IP_PMTUDISC_WANT;
    }

    inet.inet_id = 0;
    sock_init_data(sock, sk);

    (*sk).sk_destruct = inet_sock_destruct;
    (*sk).sk_protocol = protocol;
    (*sk).sk_backlog_rcv = (*sk).sk_prot.backlog_rcv;

    inet.uc_ttl = -1;
    inet.mc_loop = 1;
    inet.mc_ttl = 1;
    inet.mc_all = 1;
    inet.mc_index = 0;
    inet.mc_list = ptr::null_mut();
    inet.rcv_tos = 0;

    sk_refcnt_debug_inc(sk);

    if inet.inet_num != 0 {
        inet.inet_sport = htons(inet.inet_num);
        err = (*sk).sk_prot.hash(sk);
        if err != 0 {
            sk_common_release(sk);
            return err;
        }
    }

    if (*sk).sk_prot.init != ptr::null_mut() {
        err = (*sk).sk_prot.init(sk);
        if err != 0 {
            sk_common_release(sk);
            return err;
        }
    }

    if kern == 0 {
        err = BPF_CGROUP_RUN_PROG_INET_SOCK(sk);
        if err != 0 {
            sk_common_release(sk);
            return err;
        }
    }

    return err;
}

// Helper functions (would be implemented in C in the kernel)
#[no_mangle]
unsafe extern "C" fn __skb_queue_purge(queue: *mut skb_queue_head_t) {}
#[no_mangle]
unsafe extern "C" fn __kfree_skb(skb: *mut sk_buff) {}
#[no_mangle]
unsafe extern "C" fn sk_mem_reclaim(sk: *mut sock) {}
#[no_mangle]
unsafe extern "C" fn pr_err(fmt: *const c_char, ...) {}
#[no_mangle]
unsafe extern "C" fn kfree(ptr: *mut c_void) {}
#[no_mangle]
unsafe extern "C" fn dst_release(dst: *mut dst_entry) {}
#[no_mangle]
unsafe extern "C" fn sk_refcnt_debug_dec(sk: *mut sock) {}
#[no_mangle]
unsafe extern "C" fn lock_sock(sk: *mut sock) {}
#[no_mangle]
unsafe extern "C" fn release_sock(sk: *mut sock) {}
#[no_mangle]
unsafe extern "C" fn inet_csk_listen_start(sk: *mut sock, backlog: c_int) -> c_int { 0 }
#[no_mangle]
unsafe extern "C" fn tcp_call_bpf(sk: *mut sock, cb: c_int, arg1: c_int, arg2: *mut c_void) {}
#[no_mangle]
unsafe extern "C" fn fastopen_queue_tune(sk: *mut sock, backlog: c_int) {}
#[no_mangle]
unsafe extern "C" fn tcp_fastopen_init_key_once(net: *mut net) {}
#[no_mangle]
unsafe extern "C" fn sk_alloc(net: *mut net, family: c_int, gfp: c_int, prot: *mut proto, kern: c_int) -> *mut sock { ptr::null_mut() }
#[no_mangle]
unsafe extern "C" fn sock_init_data(sock: *mut socket, sk: *mut sock) {}
#[no_mangle]
unsafe extern "C" fn sk_common_release(sk: *mut sock) {}
#[no_mangle]
unsafe extern "C" fn BPF_CGROUP_RUN_PROG_INET_SOCK(sk: *mut sock) -> c_int { 0 }

// Constants and macros
pub const IPPROTO_MAX: c_int = 256;
pub const SS_UNCONNECTED: c_int = 0;
pub const SOCK_STREAM: c_int = 1;
pub const TCPF_CLOSE: c_int = 1 << 1;
pub const TCPF_LISTEN: c_int = 1 << 2;
pub const TCP_LISTEN: c_int = 2;
pub const INET_PROTOSW_REUSE: c_int = 1;
pub const INET_PROTOSW_ICSK: c_int = 2;
pub const SK_CAN_REUSE: c_int = 1;
pub const IPPROTO_RAW: c_int = 255;
pub const PF_INET: c_int = 2;
pub const GFP_KERNEL: c_int = 0;
pub const CAP_NET_RAW: c_int = 1;
pub const BPF_CGROUP_RUN_PROG_INET_SOCK: c_int = 1;

// Tests (conditional compilation)
#[cfg(test)]
mod tests {
    #[test]
    fn test_inet_create() {
        // Basic test would go here
        // Note: Actual testing would require kernel environment
    }
}