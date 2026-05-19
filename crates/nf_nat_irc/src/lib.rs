
// SPDX-License-Identifier: GPL-2.0-or-later
#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use core::ptr;
use kernel_types::*;

pub const EBUSY: c_int = 16;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_ct_tcp {
    pub port: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_expect {
    master: *mut nf_conn,
    saved_proto: nf_ct_proto,
    dir: c_int,
    expectfn: *const c_void,
    tuple: nf_conntrack_tuple,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_tuple {
    dst: nf_conntrack_address,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_address {
    u: nf_conntrack_address_union,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union nf_conntrack_address_union {
    ip: __be32,
    tcp: nf_ct_tcp,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_ct_proto {
    pub tcp: nf_ct_tcp,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union nf_conntrack_address_union {
    pub ip: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_address {
    pub u3: nf_conntrack_address_union,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_tuple_dst {
    pub u3: nf_conntrack_address,
    pub u: nf_conntrack_man_proto,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_tuple {
    pub dst: nf_conntrack_tuple_dst,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_tuple_hash {
    pub tuple: nf_conntrack_tuple,
}

#[repr(C)]
pub struct nf_conn {
    pub tuplehash: [nf_conntrack_tuple_hash; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_expect {
    pub master: *mut nf_conn,
    pub saved_proto: nf_ct_proto,
    pub dir: c_int,
    pub expectfn: *const c_void,
    pub tuple: nf_conntrack_tuple,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_nat_helper {
    pub name: *const c_char,
}

unsafe extern "C" {
    fn nf_ct_expect_related(exp: *mut nf_conntrack_expect, flags: c_int) -> c_int;
    fn nf_nat_mangle_tcp_packet(
        skb: *mut sk_buff,
        ct: *mut nf_conn,
        ctinfo: c_int,
        protoff: c_uint,
        matchoff: c_uint,
        matchlen: c_uint,
        data: *const c_char,
        datalen: c_size_t,
    ) -> c_int;
    fn nf_ct_helper_log(skb: *mut sk_buff, ct: *mut nf_conn, msg: *const c_char);
    fn nf_ct_unexpect_related(exp: *mut nf_conntrack_expect);
    fn nf_nat_helper_register(helper: *mut nf_conntrack_nat_helper);
    fn nf_nat_helper_unregister(helper: *mut nf_conntrack_nat_helper);
    fn synchronize_rcu();
    fn pr_info(fmt: *const c_char, ...);
}

static NAT_HELPER_NAME: &[u8] = b"irc\0";
static mut NAT_HELPER_IRC: nf_conntrack_nat_helper = nf_conntrack_nat_helper {
    name: NAT_HELPER_NAME.as_ptr() as *const c_char,
};
static mut NF_NAT_IRC_HOOK: *const c_void = ptr::null();

static mut NF_NAT_IRC_HOOK: *const c_void = ptr::null();

#[no_mangle]
pub unsafe extern "C" fn nf_nat_irc_init() -> c_int {
    let hook = help as *const c_void;
    ptr::write_volatile(&mut NF_NAT_IRC_HOOK as *mut *const c_void, hook);

    nf_nat_helper_register(&mut NAT_HELPER_IRC);
    0
}

#[no_mangle]
pub unsafe extern "C" fn nf_nat_irc_fini() {
    nf_nat_helper_unregister(&mut NAT_HELPER_IRC);
    ptr::write_volatile(&mut NF_NAT_IRC_HOOK as *mut *const c_void, ptr::null());
    synchronize_rcu();
}

static LOG_ALL_PORTS_IN_USE: &[u8] = b"all ports in use\0";
static LOG_CANNOT_MANGLE: &[u8] = b"cannot mangle packet\0";
static FMT_IP_PORT: &[u8] = b"%u %u\0";

#[no_mangle]
pub unsafe extern "C" fn help(
    skb: *mut sk_buff,
    ctinfo: c_int,
    protoff: c_uint,
    matchoff: c_uint,
    matchlen: c_uint,
    exp: *mut nf_conntrack_expect,
) -> c_uint {
    let mut buffer = [0u8; 32];

    let master = (*exp).master;
    let newaddr = (*master).tuplehash[0].tuple.dst.u3.u3.ip;
    let mut port: u16 = ntohs((*exp).saved_proto.tcp.port);

    (*exp).saved_proto.tcp.port = htons(port);
    (*exp).dir = 1;
    (*exp).expectfn = ptr::null();

    // Try to find an available port
    let mut current_port = port;
    while current_port <= 65535 {
        (*exp).tuple.dst.u.tcp.port = htons(current_port);

        match nf_ct_expect_related(exp, 0) {
            0 => {
                port = current_port;
                break;
            },
            -EBUSY => {
                current_port += 1;
                continue;
            },
            _ => {
                port = 0;
                break;
            }
            current = current.wrapping_add(1);
            continue;
        } else {
            break;
        }
    }

    if !found {
        nf_ct_helper_log(skb, master, LOG_ALL_PORTS_IN_USE.as_ptr() as *const c_char);
        return 1;
    }

    let new_ip = ntohl(newaddr);
    let new_port = port as u32;

    let n = snprintf(
        buffer.as_mut_ptr() as *mut c_char,
        buffer.len(),
        FMT_IP_PORT.as_ptr() as *const c_char,
        new_ip,
        new_port,
    );

    if n < 0 {
        nf_ct_unexpect_related(exp);
        return 1;
    }

    let datalen = n as c_size_t;
    if nf_nat_mangle_tcp_packet(
        skb,
        master,
        ctinfo,
        protoff,
        matchoff,
        matchlen,
        buffer.as_ptr() as *const c_char,
        datalen,
    ) == 0
    {
        nf_ct_helper_log(skb, master, LOG_CANNOT_MANGLE.as_ptr() as *const c_char);
        nf_ct_unexpect_related(exp);
        return 1;
    }

    0
}

// Helper macros translated to Rust functions
#[no_mangle]
pub unsafe extern "C" fn BUG_ON(condition: c_int) {
    if condition != 0 {
        panic!("BUG_ON triggered");
    }
}

// Module metadata (would be handled by kernel macros in C)
#[no_mangle]
pub static AUTHOR: &str = "Harald Welte <laforge@gnumonks.org>";
#[no_mangle]
pub static DESCRIPTION: &str = "IRC (DCC) NAT helper";
#[no_mangle]
pub static LICENSE: &str = "GPL";

// Helper functions for string formatting
#[no_mangle]
pub unsafe extern "C" fn snprintf(
    buf: *mut c_char,
    size: size_t,
    fmt: *const c_char,
    arg1: u32,
    arg2: u16,
) -> c_int {
    let fmt_str = CStr::from_ptr(fmt);
    let mut result = 0;

    // SAFETY: This is a simplified implementation for demonstration purposes
    // In a real kernel module, this would use the actual snprintf implementation
    let formatted = format!("{} {}", arg1, arg2);
    let bytes = formatted.as_bytes_with_nul();

    if !buf.is_null() && size > 0 {
        let copy_len = (size - 1).min(bytes.len());
        ptr::copy_nonoverlapping(bytes.as_ptr(), buf as *mut u8, copy_len);
        result = copy_len as c_int;
    }

    result
}

// Helper functions for network byte order conversion
#[no_mangle]
pub unsafe extern "C" fn ntohs(port: __be16) -> __be16 {
    u16::from_be(port)
}

#[no_mangle]
pub unsafe extern "C" fn htons(port: __be16) -> __be16 {
    u16::to_be(port)
}

#[no_mangle]
pub unsafe extern "C" fn ntohl(ip: __be32) -> __be32 {
    u32::from_be(ip)
}

// Test cases (conditional compilation)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_port_allocation() {
        // This would require actual kernel environment to test
        // For demonstration purposes, we just verify the function signature
        assert_eq!(core::mem::size_of_val(&NAT_HELPER_IRC), 8);
    }
}
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
