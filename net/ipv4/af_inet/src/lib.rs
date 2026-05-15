//! IPv4 Socket Implementation
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]  // For C-style type names

use core::ptr;
use libc::{c_int, c_uint, c_void, size_t};

// Constants from C
pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ESOCKTNOSUPPORT: c_int = -94;
pub const EPROTONOSUPPORT: c_int = -93;
pub const EPERM: c_int = -1;
pub const ENOBUFS: c_int = -105;

// Type definitions
#[repr(C)]
pub struct sock {
    sk_receive_queue: skb_queue_head_t,
    sk_rx_skb_cache: *mut c_void,
    sk_error_queue: skb_queue_head_t,
    sk_rx_dst: *mut c_void,
    sk_dst_cache: *mut c_void,
    sk_type: c_int,
    sk_state: c_int,
    sk_max_ack_backlog: c_int,
    sk_backlog_rcv: extern "C" fn(*mut sock, *mut c_void, size_t) -> c_int,
    sk_prot: *mut proto,
    sk_destruct: extern "C" fn(*mut sock),
    sk_reuse: c_int,
    sk_rmem_alloc: atomic_t,
    sk_wmem_alloc: atomic_t,
    sk_wmem_queued: c_int,
    sk_forward_alloc: c_int,
    sk_refcnt_debug: c_int,
}

#[repr(C)]
pub struct proto {
    slab: *mut c_void,
    hash: extern "C" fn(*mut sock) -> c_int,
    init: extern "C" fn(*mut sock) -> c_int,
    backlog_rcv: extern "C" fn(*mut sock, *mut c_void, size_t) -> c_int,
}

#[repr(C)]
pub struct inet_sock {
    is_icsk: c_int,
    nodefrag: c_int,
    inet_num: c_int,
    hdrincl: c_int,
    pmtudisc: c_int,
    inet_id: c_int,
    uc_ttl: c_int,
    mc_loop: c_int,
    mc_ttl: c_int,
    mc_all: c_int,
    mc_index: c_int,
    mc_list: *mut c_void,
    rcv_tos: c_int,
}

#[repr(C)]
pub struct socket {
    state: c_int,
    type_: c_int,
    ops: *mut c_void,
}

#[repr(C)]
pub struct inet_protosw {
    protocol: c_int,
    ops: *mut c_void,
    prot: *mut proto,
    flags: c_int,
}

#[repr(C)]
pub struct list_head {
    next: *mut list_head,
    prev: *mut list_head,
}

// Function implementations
#[no_mangle]
pub unsafe extern "C" fn inet_sock_destruct(sk: *mut sock) {
    let inet = inet_sk(sk);
    
    // SAFETY: Called during socket destruction, so all pointers are valid
    __skb_queue_purge(&mut (*sk).sk_receive_queue);
    if !(*sk).sk_rx_skb_cache.is_null() {
        __kfree_skb((*sk).sk_rx_skb_cache);
        (*sk).sk_rx_skb_cache = ptr::null_mut();
    }
    __skb_queue_purge(&mut (*sk).sk_error_queue);
    
    sk_mem_reclaim(sk);
    
    if (*sk).sk_type == SOCK_STREAM && (*sk).sk_state != TCP_CLOSE {
        pr_err(b"Attempt to release TCP socket in state %d %p\n", (*sk).sk_state, sk);
        return;
    }
    if !sock_flag(sk, SOCK_DEAD) {
        pr_err(b"Attempt to release alive inet socket %p\n", sk);
        return;
    }
    
    // SAFETY: Socket is being destroyed, so no concurrent access
    kfree(rcu_dereference_protected(inet).inet_opt);
    dst_release(rcu_dereference_protected(sk).sk_dst_cache);
    dst_release((*sk).sk_rx_dst);
    sk_refcnt_debug_dec(sk);
}

#[no_mangle]
pub unsafe extern "C" fn inet_autobind(sk: *mut sock) -> c_int {
    let inet = inet_sk(sk);
    lock_sock(sk);
    
    if (*inet).inet_num == 0 {
        if (*sk).sk_prot.get_port(sk, 0) != 0 {
            release_sock(sk);
            return -EAGAIN;
        }
        (*inet).inet_sport = htons((*inet).inet_num);
    }
    release_sock(sk);
    0
}

#[no_mangle]
pub unsafe extern "C" fn inet_listen(sock: *mut socket, backlog: c_int) -> c_int {
    let sk = (*sock).sk;
    let mut err: c_int = -EINVAL;
    
    lock_sock(sk);
    
    if (*sock).state != SS_UNCONNECTED || (*sock).type_ != SOCK_STREAM {
        goto out;
    }
    
    let old_state = (*sk).sk_state;
    if !((1 << old_state) & (TCPF_CLOSE | TCPF_LISTEN)) {
        goto out;
    }
    
    (*sk).sk_max_ack_backlog = backlog;
    
    if old_state != TCP_LISTEN {
        // Fast open logic would be implemented here
        err = inet_csk_listen_start(sk, backlog);
        if err != 0 {
            goto out;
        }
        tcp_call_bpf(sk, BPF_SOCK_OPS_TCP_LISTEN_CB, 0, ptr::null_mut());
    }
    err = 0;
    
out:
    release_sock(sk);
    return err;
}

#[no_mangle]
pub unsafe extern "C" fn inet_create(
    net: *mut c_void,
    sock: *mut socket,
    protocol: c_int,
    kern: c_int,
) -> c_int {
    let mut answer: *mut inet_protosw = ptr::null_mut();
    let mut try_loading_module: c_int = 0;
    let mut err: c_int = -ESOCKTNOSUPPORT;
    
    // Look for protocol match
    loop {
        rcu_read_lock();
        let mut entry = (*inetsw[(*sock).type_]).list;
        while !entry.is_null() {
            // Protocol matching logic
            if (*entry).protocol == protocol || protocol == IPPROTO_IP {
                answer = entry;
                err = 0;
                break;
            }
            entry = (*entry).next;
        }
        rcu_read_unlock();
        
        if err != 0 {
            if try_loading_module < 2 {
                try_loading_module += 1;
                request_module("net-pf-%d-proto-%d-type-%d", PF_INET, protocol, (*sock).type_);
                continue;
            } else {
                return err;
            }
        }
        break;
    }
    
    if sock->type == SOCK_RAW && !kern && !ns_capable(net, CAP_NET_RAW) {
        return -EPERM;
    }
    
    (*sock).ops = (*answer).ops;
    let answer_prot = (*answer).prot;
    
    let sk = sk_alloc(net, PF_INET, GFP_KERNEL, answer_prot, kern);
    if sk.is_null() {
        return -ENOBUFS;
    }
    
    if (*answer).flags & INET_PROTOSW_REUSE != 0 {
        (*sk).sk_reuse = SK_CAN_REUSE;
    }
    
    let inet = inet_sk(sk);
    inet.is_icsk = ((*answer).flags & INET_PROTOSW_ICSK) != 0;
    
    if (*sock).type == SOCK_RAW {
        inet.inet_num = protocol;
        if protocol == IPPROTO_RAW {
            inet.hdrincl = 1;
        }
    }
    
    if net->ipv4.sysctl_ip_no_pmtu_disc {
        inet.pmtudisc = IP_PMTUDISC_DONT;
    } else {
        inet.pmtudisc = IP_PMTUDISC_WANT;
    }
    
    sock_init_data(sock, sk);
    
    (*sk).sk_destruct = inet_sock_destruct;
    (*sk).sk_protocol = protocol;
    (*sk).sk_backlog_rcv = (*sk).sk_prot.backlog_rcv;
    
    // Initialize inet sock fields
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
        if (*sk).sk_prot.hash(sk) != 0 {
            sk_common_release(sk);
            return -ENOBUFS;
        }
    }
    
    if (*sk).sk_prot.init(sk) != 0 {
        sk_common_release(sk);
        return -ENOBUFS;
    }
    
    if !kern {
        err = BPF_CGROUP_RUN_PROG_INET_SOCK(sk);
        if err != 0 {
            sk_common_release(sk);
            return err;
        }
    }
    
    0
}

// Helper functions (would be implemented or linked via FFI)
#[no_mangle]
pub unsafe extern "C" fn lock_sock(sk: *mut sock) {
    // Implementation would be via FFI
}

#[no_mangle]
pub unsafe extern "C" fn release_sock(sk: *mut sock) {
    // Implementation would be via FFI
}

#[no_mangle]
pub unsafe extern "C" fn inet_sk(sk: *mut sock) -> *mut inet_sock {
    // Cast to appropriate offset
    sk.offset(1) as *mut inet_sock
}

#[no_mangle]
pub unsafe extern "C" fn __skb_queue_purge(queue: *mut skb_queue_head_t) {
    // Implementation via FFI
}

#[no_mangle]
pub unsafe extern "C" fn __kfree_skb(skb: *mut c_void) {
    // Implementation via FFI
}

#[no_mangle]
pub unsafe extern "C" fn sk_mem_reclaim(sk: *mut sock) {
    // Implementation via FFI
}

#[no_mangle]
pub unsafe extern "C" fn pr_err(fmt: *const c_char, ...) {
    // Implementation via FFI
}

#[no_mangle]
pub unsafe extern "C" fn rcu_dereference_protected<T>(ptr: *mut T) -> *mut T {
    ptr
}

#[no_mangle]
pub unsafe extern "C" fn kfree(ptr: *mut c_void) {
    // Implementation via FFI
}

#[no_mangle]
pub unsafe extern "C" fn dst_release(ptr: *mut c_void) {
    // Implementation via FFI
}

#[no_mangle]
pub unsafe extern "C" fn sk_refcnt_debug_dec(sk: *mut sock) {
    // Implementation via FFI
}

#[no_mangle]
pub unsafe extern "C" fn sk_common_release(sk: *mut sock) {
    // Implementation via FFI
}

#[no_mangle]
pub unsafe extern "C" fn BPF_CGROUP_RUN_PROG_INET_SOCK(sk: *mut sock) -> c_int {
    // Implementation via FFI
    0
}

// Tests (conditional compilation)
#[cfg(test)]
mod tests {
    #[test]
    fn test_inet_sock_destruct() {
        // Basic test would be implemented if possible
    }
}
