#![no_std]
#![no_main]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unknown_lints)]
#![allow(clippy::too_many_arguments)]

use core::ffi::{c_int, c_void};
use core::ptr;
use core::panic::PanicInfo;
use kernel_types::*;

pub type size_t = usize;
pub type c_size_t = usize;
pub type socklen_t = u32;

// Constants from C
pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ESOCKTNOSUPPORT: c_int = -94;
pub const EPROTONOSUPPORT: c_int = -93;
pub const EPERM: c_int = -1;
pub const ENOBUFS: c_int = -55;

// Minimal FFI placeholders required for compilation.
// Real definitions are expected to come from kernel bindings.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct skb_queue_head_t {
    _priv: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sk_buff {
    _priv: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dst_entry {
    _priv: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atomic_t {
    pub counter: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct refcount_t {
    pub refs: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct proto {
    _priv: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct linger {
    pub l_onoff: c_int,
    pub l_linger: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wait_queue_head_t {
    _priv: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct timer_list {
    _priv: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct timeval {
    pub tv_sec: i64,
    pub tv_usec: i64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct socket_ops {
    _priv: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sock {
    pub sk_receive_queue: skb_queue_head_t,
    pub sk_rx_skb_cache: *mut sk_buff,
    pub sk_error_queue: skb_queue_head_t,
    pub sk_type: c_int,
    pub sk_state: c_int,
    pub sk_max_ack_backlog: c_int,
    pub sk_dst_cache: *mut dst_entry,
    pub sk_rx_dst: *mut dst_entry,
    pub sk_rmem_alloc: atomic_t,
    pub sk_wmem_alloc: refcount_t,
    pub sk_wmem_queued: size_t,
    pub sk_forward_alloc: size_t,
    pub sk_backlog_rcv: *mut c_void,
    pub sk_prot: *mut proto,
    pub sk_destruct: Option<unsafe extern "C" fn(*mut sock)>,
    pub sk_protocol: c_int,
    pub sk_users: atomic_t,
    pub sk_refcnt: atomic_t,
    pub sk_shutdown: c_int,
    pub sk_no_check: c_int,
    pub sk_lingertime: c_int,
    pub sk_reuse: c_int,
    pub sk_bound_dev_if: c_int,
    pub sk_bind_mark: c_int,
    pub sk_priority: c_int,
    pub sk_rcvlowat: size_t,
    pub sk_rcvtimeo: c_int,
    pub sk_sndtimeo: c_int,
    pub sk_linger: linger,
    pub sk_info_cache: *mut c_void,
    pub sk_prot_creator: *mut proto,
    pub sk_wq: *mut wait_queue_head_t,
    pub sk_user_data: *mut c_void,
    pub sk_clockid: c_int,
    pub sk_flags: c_int,
    pub sk_tsflags: c_int,
    pub sk_peek_off: size_t,
    pub sk_rxhash: c_int,
    pub sk_filter: *mut c_void,
    pub sk_timer: timer_list,
    pub sk_stamp: timeval,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inet_protosw {
    pub list: list_head,
    pub protocol: c_int,
    pub ops: *mut socket_ops,
    pub prot: *mut proto,
    pub flags: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct socket {
    pub state: c_int,
    pub type_field: c_int,
    pub sk: *mut sock,
}

// Common constants used in shown code
pub const SOCK_STREAM: c_int = 1;
pub const TCP_CLOSE: c_int = 7;
pub const SOCK_DEAD: c_int = 1;
pub const SS_UNCONNECTED: c_int = 1;
pub const TCPF_CLOSE: c_int = 1 << TCP_CLOSE;
pub const TCP_LISTEN: c_int = 10;
pub const TCPF_LISTEN: c_int = 1 << TCP_LISTEN;

unsafe extern "C" {
    fn __skb_queue_purge(list: *const skb_queue_head_t);
    fn __kfree_skb(skb: *mut sk_buff);
    fn sk_mem_reclaim(sk: *mut sock);
    fn sock_flag(sk: *const sock, flag: c_int) -> bool;
    fn atomic_read(v: *const atomic_t) -> c_int;
    fn refcount_read(r: *const refcount_t) -> c_int;
    fn dst_release(dst: *mut dst_entry);
    fn sk_refcnt_debug_dec(sk: *mut sock);
    fn lock_sock(sk: *mut sock);
    fn release_sock(sk: *mut sock);
}

#[no_mangle]
pub unsafe extern "C" fn inet_sock_destruct(sk: *mut sock) {
    if sk.is_null() {
        return;
    }

    unsafe {
        __skb_queue_purge(&(*sk).sk_receive_queue);

        if !(*sk).sk_rx_skb_cache.is_null() {
            __kfree_skb((*sk).sk_rx_skb_cache);
            (*sk).sk_rx_skb_cache = ptr::null_mut();
        }

        __skb_queue_purge(&(*sk).sk_error_queue);
        sk_mem_reclaim(sk);

        if (*sk).sk_type == SOCK_STREAM && (*sk).sk_state != TCP_CLOSE {
            return;
        }

        if !sock_flag(sk, SOCK_DEAD) {
            return;
        }

        if atomic_read(&(*sk).sk_rmem_alloc) != 0 {
            return;
        }
        if refcount_read(&(*sk).sk_wmem_alloc) != 0 {
            return;
        }
        if (*sk).sk_wmem_queued != 0 || (*sk).sk_forward_alloc != 0 {
            return;
        }

        if !(*sk).sk_dst_cache.is_null() {
            dst_release((*sk).sk_dst_cache);
            (*sk).sk_dst_cache = ptr::null_mut();
        }
        if !(*sk).sk_rx_dst.is_null() {
            dst_release((*sk).sk_rx_dst);
            (*sk).sk_rx_dst = ptr::null_mut();
        }

        sk_refcnt_debug_dec(sk);
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    loop {}
}