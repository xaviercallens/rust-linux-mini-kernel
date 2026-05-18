#![no_std]
#![no_main]
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
    if sock.is_null() {
        return EINVAL;
    }

    if protocol < 0 {
        return EINVAL;
    }

    let _ = &mut *sock;
    0
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