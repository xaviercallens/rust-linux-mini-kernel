// SPDX-License-Identifier: GPL-2.0-or-later
#![no_std]
#![no_main]
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
pub union nf_conntrack_man_proto {
    pub all: __be16,
    pub tcp: nf_ct_tcp,
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

    fn htons(v: u16) -> __be16;
    fn ntohs(v: __be16) -> u16;
    fn ntohl(v: __be32) -> u32;
    fn snprintf(dst: *mut c_char, size: c_size_t, fmt: *const c_char, ...) -> c_int;
}

static NAT_HELPER_NAME: &[u8] = b"irc\0";
static mut NAT_HELPER_IRC: nf_conntrack_nat_helper = nf_conntrack_nat_helper {
    name: NAT_HELPER_NAME.as_ptr() as *const c_char,
};

static mut NF_NAT_IRC_HOOK: *const c_void = ptr::null();

#[no_mangle]
pub unsafe extern "C" fn nf_nat_irc_init() -> c_int {
    let hook = help as *const c_void;
    ptr::write_volatile(&raw mut NF_NAT_IRC_HOOK, hook);
    nf_nat_helper_register(&raw mut NAT_HELPER_IRC);
    0
}

#[no_mangle]
pub unsafe extern "C" fn nf_nat_irc_fini() {
    nf_nat_helper_unregister(&raw mut NAT_HELPER_IRC);
    ptr::write_volatile(&raw mut NF_NAT_IRC_HOOK, ptr::null());
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

    let mut found = false;
    let mut current = port;
    loop {
        (*exp).tuple.dst.u.tcp.port = htons(current);

        let rc = nf_ct_expect_related(exp, 0);
        if rc == 0 {
            port = current;
            found = true;
            break;
        } else if rc == -EBUSY {
            if current == u16::MAX {
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

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}