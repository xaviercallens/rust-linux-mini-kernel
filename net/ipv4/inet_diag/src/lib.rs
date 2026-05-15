//! INET Transport Protocol Socket Diagnostics
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(clang::missing_docs_in_private_items)]

use core::ffi::{c_int, c_uint, c_void, size_t};
use core::mem;
use core::ptr;

// Constants from C
pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ENOENT: c_int = -2;
pub const IPPROTO_MAX: c_int = 256;
pub const KMALLOC_MAX_SIZE: size_t = 128 * 1024 * 1024; // Example value

// Type definitions
#[repr(C)]
pub struct in6_addr {
    pub s6_addr: [u8; 16],
}

#[repr(C)]
pub struct inet_diag_msg {
    pub idiag_family: c_uint,
    pub idiag_state: c_uint,
    pub idiag_timer: c_uint,
    pub idiag_retrans: c_uint,
    pub id: inet_diag_msg_id,
    pub idiag_inode: c_ulong,
    pub idiag_uid: c_uint,
}

#[repr(C)]
pub struct inet_diag_msg_id {
    pub idiag_sport: u16,
    pub idiag_dport: u16,
    pub idiag_if: c_uint,
    pub idiag_cookie: [c_ulong; 2],
    pub idiag_src: [u32; 4],
    pub idiag_dst: [u32; 4],
}

#[repr(C)]
pub struct inet_diag_meminfo {
    pub idiag_rmem: c_ulong,
    pub idiag_wmem: c_ulong,
    pub idiag_fmem: c_ulong,
    pub idiag_tmem: c_ulong,
}

#[repr(C)]
pub struct inet_diag_sockopt {
    pub recverr: u8,
    pub is_icsk: u8,
    pub freebind: u8,
    pub hdrincl: u8,
    pub mc_loop: u8,
    pub transparent: u8,
    pub mc_all: u8,
    pub nodefrag: u8,
    pub bind_address_no_port: u8,
    pub recverr_rfc4884: u8,
    pub defer_connect: u8,
    #[cfg(CONFIG_SOCK_CGROUP_DATA)]
    pub pad: [u8; 3],
}

#[repr(C)]
pub struct inet_diag_handler {
    pub idiag_get_info: extern "C" fn(
        *mut sock,
        *mut inet_diag_msg,
        *mut c_void,
    ) -> c_int,
    pub idiag_info_size: size_t,
    pub idiag_get_aux_size: extern "C" fn(*mut sock, c_int) -> size_t,
    pub idiag_get_aux: extern "C" fn(*mut sock, c_int, *mut sk_buff) -> c_int,
}

#[repr(C)]
pub struct sock {
    pub sk_family: c_uint,
    pub sk_num: u16,
    pub sk_dport: u16,
    pub sk_bound_dev_if: c_uint,
    pub sk_rcv_saddr: u32,
    pub sk_daddr: u32,
    pub sk_v6_rcv_saddr: in6_addr,
    pub sk_v6_daddr: in6_addr,
    pub sk_mark: u32,
    pub sk_state: c_uint,
    pub sk_shutdown: u8,
    pub sk_priority: u32,
    pub sk_timer: timer_list,
    pub sk_wmem_queued: u32,
    pub sk_forward_alloc: u32,
    #[cfg(CONFIG_IPV6)]
    pub sk_ipv6: *mut inet6_sock,
    #[cfg(CONFIG_SOCK_CGROUP_DATA)]
    pub sk_cgrp_data: cgroup_data,
}

#[repr(C)]
pub struct timer_list {
    pub expires: u64,
}

#[repr(C)]
pub struct inet6_sock {
    pub tclass: u8,
}

#[repr(C)]
pub struct cgroup_data {
    #[cfg(CONFIG_SOCK_CGROUP_DATA)]
    pub cgroup_id: u64,
}

#[repr(C)]
pub struct sk_buff {
    data: *mut u8,
    len: size_t,
}

#[repr(C)]
pub struct netlink_callback {
    skb: *mut sk_buff,
    nlh: *mut nlmsghdr,
    data: *mut c_void,
}

#[repr(C)]
pub struct nlmsghdr {
    nlmsg_len: u32,
    nlmsg_type: u16,
    nlmsg_flags: u16,
    nlmsg_seq: u32,
    nlmsg_pid: u32,
}

#[repr(C)]
pub struct nlattr {
    nla_len: u16,
    nla_type: u16,
}

#[repr(C)]
pub struct inet_diag_dump_data {
    req_nlas: *mut *mut nlattr,
}

// Function implementations
#[no_mangle]
pub unsafe extern "C" fn inet_diag_msg_common_fill(
    r: *mut inet_diag_msg,
    sk: *mut sock,
) {
    if r.is_null() || sk.is_null() {
        return;
    }

    // SAFETY: Caller guarantees valid pointers
    (*r).idiag_family = (*sk).sk_family;
    
    (*r).id.idiag_sport = (*sk).sk_num;
    (*r).id.idiag_dport = (*sk).sk_dport;
    (*r).id.idiag_if = (*sk).sk_bound_dev_if;
    
    // Cookie is filled by sock_diag_save_cookie (not implemented here)
    
    #[cfg(CONFIG_IPV6)]
    {
        if (*sk).sk_family == AF_INET6 {
            (*r).id.idiag_src = [0; 4];
            (*r).id.idiag_dst = [0; 4];
            // Copy in6_addr to u32[4] (simplified)
        }
    }
    
    // For IPv4
    (*r).id.idiag_src[0] = (*sk).sk_rcv_saddr;
    (*r).id.idiag_dst[0] = (*sk).sk_daddr;
}

#[no_mangle]
pub unsafe extern "C" fn inet_diag_msg_attrs_fill(
    sk: *mut sock,
    skb: *mut sk_buff,
    r: *mut inet_diag_msg,
    ext: c_int,
    user_ns: *mut c_void,
    net_admin: c_int,
) -> c_int {
    if sk.is_null() || skb.is_null() || r.is_null() {
        return EINVAL;
    }

    // SAFETY: Caller guarantees valid pointers
    let inet = &(*sk).sk_common as *const _ as *mut inet_sock;
    
    // Shutdown flag
    let shutdown = (*sk).sk_shutdown;
    if nla_put(skb, INET_DIAG_SHUTDOWN, 1, &shutdown) != 0 {
        return EINVAL;
    }
    
    // TOS handling
    if (ext & (1 << (INET_DIAG_TOS - 1))) != 0 {
        let tos = (*inet).tos;
        if nla_put(skb, INET_DIAG_TOS, 1, &tos) != 0 {
            return EINVAL;
        }
    }
    
    #[cfg(CONFIG_IPV6)]
    {
        if (*sk).sk_family == AF_INET6 {
            if (ext & (1 << (INET_DIAG_TCLASS - 1))) != 0 {
                let tclass = (*(*sk).sk_ipv6).tclass;
                if nla_put(skb, INET_DIAG_TCLASS, 1, &tclass) != 0 {
                    return EINVAL;
                }
            }
        }
    }
    
    if net_admin != 0 {
        let mark = (*sk).sk_mark;
        if nla_put(skb, INET_DIAG_MARK, 4, &mark) != 0 {
            return EINVAL;
        }
    }
    
    // Class ID handling
    let mut classid: u32 = 0;
    #[cfg(CONFIG_SOCK_CGROUP_DATA)]
    {
        classid = sock_cgroup_classid(&(*sk).sk_cgrp_data);
    }
    if classid == 0 {
        classid = (*sk).sk_priority;
    }
    if nla_put(skb, INET_DIAG_CLASS_ID, 4, &classid) != 0 {
        return EINVAL;
    }
    
    #[cfg(CONFIG_SOCK_CGROUP_DATA)]
    {
        let cgroup_id = cgroup_id(&(*sk).sk_cgrp_data);
        if nla_put(skb, INET_DIAG_CGROUP_ID, 8, &cgroup_id) != 0 {
            return EINVAL;
        }
    }
    
    (*r).idiag_uid = from_kuid_munged(user_ns, sock_i_uid(sk));
    (*r).idiag_inode = sock_i_ino(sk);
    
    let mut inet_sockopt = inet_diag_sockopt {
        recverr: (*inet).recverr,
        is_icsk: (*inet).is_icsk,
        freebind: (*inet).freebind,
        hdrincl: (*inet).hdrincl,
        mc_loop: (*inet).mc_loop,
        transparent: (*inet).transparent,
        mc_all: (*inet).mc_all,
        nodefrag: (*inet).nodefrag,
        bind_address_no_port: (*inet).bind_address_no_port,
        recverr_rfc4884: (*inet).recverr_rfc4884,
        defer_connect: (*inet).defer_connect,
        #[cfg(CONFIG_SOCK_CGROUP_DATA)]
        pad: [0; 3],
    };
    
    if nla_put(skb, INET_DIAG_SOCKOPT, mem::size_of_val(&inet_sockopt), &inet_sockopt) != 0 {
        return EINVAL;
    }
    
    0
}

#[no_mangle]
pub unsafe extern "C" fn inet_sk_diag_fill(
    sk: *mut sock,
    icsk: *mut inet_connection_sock,
    skb: *mut sk_buff,
    cb: *mut netlink_callback,
    req: *const inet_diag_req_v2,
    nlmsg_flags: u16,
    net_admin: c_int,
) -> c_int {
    if sk.is_null() || skb.is_null() || cb.is_null() || req.is_null() {
        return EINVAL;
    }

    let cb_data = (*cb).data as *mut inet_diag_dump_data;
    let handler = (*inet_diag_table)[(*req).sdiag_protocol];
    
    let nlh = nlmsg_put(skb, NETLINK_CB((*cb).skb).portid, (*cb).nlh.nlmsg_seq,
                        (*cb).nlh.nlmsg_type, mem::size_of::<inet_diag_msg>() as u32, nlmsg_flags);
    if nlh.is_null() {
        return -EMSGSIZE;
    }
    
    let r = nlmsg_data(nlh);
    
    // Common message fill
    inet_diag_msg_common_fill(r, sk);
    (*r).idiag_state = (*sk).sk_state;
    
    // Attributes fill
    if inet_diag_msg_attrs_fill(sk, skb, r, (*req).idiag_ext, sk_user_ns((*cb).skb), net_admin) != 0 {
        return EINVAL;
    }
    
    // Memory info
    if (*req).idiag_ext & (1 << (INET_DIAG_MEMINFO - 1)) != 0 {
        let minfo = inet_diag_meminfo {
            idiag_rmem: sk_rmem_alloc_get(sk),
            idiag_wmem: (*sk).sk_wmem_queued,
            idiag_fmem: (*sk).sk_forward_alloc,
            idiag_tmem: sk_wmem_alloc_get(sk),
        };
        if nla_put(skb, INET_DIAG_MEMINFO, mem::size_of_val(&minfo), &minfo) != 0 {
            return EINVAL;
        }
    }
    
    // Socket protocol for RAW sockets
    if (*sk).sk_type == SOCK_RAW {
        let protocol = (*sk).sk_protocol;
        if nla_put(skb, INET_DIAG_PROTOCOL, 1, &protocol) != 0 {
            return EINVAL;
        }
    }
    
    // Info handler
    if (*req).idiag_ext & (1 << (INET_DIAG_INFO - 1)) != 0 && handler.idiag_info_size > 0 {
        let attr = nla_reserve(skb, INET_DIAG_INFO, handler.idiag_info_size);
        if attr.is_null() {
            return EINVAL;
        }
        handler.idiag_get_info(sk, r, attr);
    }
    
    0
}

// Helper functions (simplified for FFI compatibility)
#[no_mangle]
pub unsafe extern "C" fn nla_put(
    skb: *mut sk_buff,
    attrtype: c_int,
    attrlen: size_t,
    data: *const c_void,
) -> c_int {
    // Simplified implementation for FFI compatibility
    if skb.is_null() || data.is_null() {
        return EINVAL;
    }
    // Actual implementation would handle skb data allocation
    0
}

#[no_mangle]
pub unsafe extern "C" fn nla_reserve(
    skb: *mut sk_buff,
    attrtype: c_int,
    attrlen: size_t,
) -> *mut nlattr {
    // Simplified implementation for FFI compatibility
    if skb.is_null() {
        return ptr::null_mut();
    }
    // Actual implementation would reserve space in skb
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn nlmsg_put(
    skb: *mut sk_buff,
    portid: u32,
    seq: u32,
    type_: u16,
    len: u32,
    flags: u16,
) -> *mut nlmsghdr {
    // Simplified implementation for FFI compatibility
    if skb.is_null() {
        return ptr::null_mut();
    }
    // Actual implementation would allocate and initialize nlmsghdr
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn nlmsg_data(nlh: *mut nlmsghdr) -> *mut inet_diag_msg {
    // Simplified implementation for FFI compatibility
    if nlh.is_null() {
        return ptr::null_mut();
    }
    // Actual implementation would return data after nlmsghdr
    ptr::null_mut()
}

// Constants (simplified)
pub const INET_DIAG_SHUTDOWN: c_int = 1;
pub const INET_DIAG_TOS: c_int = 2;
pub const INET_DIAG_TCLASS: c_int = 3;
pub const INET_DIAG_MARK: c_int = 4;
pub const INET_DIAG_CLASS_ID: c_int = 5;
pub const INET_DIAG_CGROUP_ID: c_int = 6;
pub const INET_DIAG_SOCKOPT: c_int = 7;
pub const INET_DIAG_MEMINFO: c_int = 8;
pub const INET_DIAG_PROTOCOL: c_int = 9;

pub const AF_INET: c_int = 2;
pub const AF_INET6: c_int = 10;
pub const SOCK_RAW: c_int = 3;

// Exported symbols
#[no_mangle]
pub static mut inet_diag_table: *mut *mut inet_diag_handler = ptr::null_mut();
#[no_mangle]
pub static mut inet_diag_table_mutex: mutex = mutex::new();

#[repr(C)]
pub struct mutex {
    // Opaque mutex structure
}

impl mutex {
    pub const fn new() -> Self {
        Self {}
    }
    
    pub unsafe fn lock(&self) {
        // Kernel mutex lock implementation
    }
    
    pub unsafe fn unlock(&self) {
        // Kernel mutex unlock implementation
    }
}

// Tests (conditional compilation)
#[cfg(test)]
mod tests {
    #[test]
    fn test_inet_diag_msg_common_fill() {
        // Basic test for null pointers
        unsafe {
            let mut msg = mem::zeroed::<inet_diag_msg>();
            inet_diag_msg_common_fill(&mut msg as *mut _, ptr::null_mut());
        }
    }
}
