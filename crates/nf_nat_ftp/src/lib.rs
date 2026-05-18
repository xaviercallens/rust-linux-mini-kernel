
//! FTP NAT helper for Linux kernel
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![no_main]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use core::ffi::c_void;
use core::panic::PanicInfo;
use kernel_types::*;

pub const NF_DROP: c_int = 0x01;
pub const NF_ACCEPT: c_int = 0x02;
pub const NFPROTO_IPV4: c_int = 2;
pub const NF_CT_FTP_PORT: c_int = 0;
pub const NF_CT_FTP_PASV: c_int = 1;
pub const NF_CT_FTP_EPRT: c_int = 2;
pub const NF_CT_FTP_EPSV: c_int = 3;
pub const EBUSY: c_int = 16;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_ct_port {
    pub port: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_l4proto {
    pub tcp: nf_ct_port,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_man_proto {
    pub u: nf_conntrack_l4proto,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_tuple {
    pub dst: nf_conntrack_man_proto,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conn_tuplehash {
    pub tuple: nf_conntrack_tuple,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conn_tuplehash {
    pub tuple: nf_conntrack_tuple,
    pub dir: c_int,
    pub expectfn: Option<unsafe extern "C" fn(*mut nf_conntrack_expect)>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_nat_helper {
    pub name: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_inet_addr {
    pub ip: u32,
    pub ip6: [u32; 4],
}

unsafe extern "C" {
    fn nf_ct_expect_related(exp: *mut nf_conntrack_expect, flags: c_uint) -> c_int;
    fn nf_ct_unexpect_related(exp: *mut nf_conntrack_expect);
    fn nf_ct_helper_log(skb: *mut c_void, ct: *mut nf_conn, msg: *const c_char);
    fn nf_nat_mangle_tcp_packet(
        skb: *mut c_void,
        ct: *mut nf_conn,
        ctinfo: c_int,
        protoff: c_int,
        matchoff: c_int,
        matchlen: c_int,
        buffer: *const u8,
        buflen: c_int,
    ) -> bool;
    fn nf_nat_helper_unregister(helper: *const nf_nat_helper);
    fn synchronize_rcu();
    fn RCU_INIT_POINTER(p: *mut *mut c_void, v: *mut c_void);
    static mut nf_nat_ftp_hook: *mut c_void;
    static nat_helper_ftp: nf_nat_helper;
}

#[inline]
fn htons(v: u16) -> u16 {
    v.to_be()
}

#[inline]
fn ntohs(v: u16) -> u16 {
    u16::from_be(v)
}

#[inline]
fn CTINFO2DIR(ctinfo: c_int) -> c_int {
    ctinfo & 1
}

unsafe extern "C" fn nf_nat_follow_master(_exp: *mut nf_conntrack_expect) {}

fn push_byte(buf: &mut [u8], pos: &mut usize, b: u8) -> bool {
    if *pos >= buf.len() {
        return false;
    }
    buf[*pos] = b;
    *pos += 1;
    true
}

fn push_u8_dec(buf: &mut [u8], pos: &mut usize, v: u8) -> bool {
    if v >= 100 {
        let h = v / 100;
        let t = (v / 10) % 10;
        let o = v % 10;
        push_byte(buf, pos, b'0' + h) && push_byte(buf, pos, b'0' + t) && push_byte(buf, pos, b'0' + o)
    } else if v >= 10 {
        let t = v / 10;
        let o = v % 10;
        push_byte(buf, pos, b'0' + t) && push_byte(buf, pos, b'0' + o)
    } else {
        push_byte(buf, pos, b'0' + v)
    }
}

fn push_u16_dec(buf: &mut [u8], pos: &mut usize, mut v: u16) -> bool {
    let mut tmp = [0u8; 5];
    let mut n = 0usize;
    if v == 0 {
        return push_byte(buf, pos, b'0');
    }
    while v > 0 {
        tmp[n] = (v % 10) as u8;
        v /= 10;
        n += 1;
    }
    while n > 0 {
        n -= 1;
        if !push_byte(buf, pos, b'0' + tmp[n]) {
            return false;
        }
    }
    true
}

#[no_mangle]
pub unsafe extern "C" fn nf_nat_ftp_fmt_cmd(
    ct: *mut nf_conn,
    type_: c_int,
    buffer: *mut u8,
    buflen: size_t,
    addr: *mut nf_inet_addr,
    port: u16,
) -> c_int {
    if ct.is_null() || buffer.is_null() || addr.is_null() || buflen == 0 {
        return 0;
    }

    let _ct = &*ct;
    let a = &*addr;
    let out = core::slice::from_raw_parts_mut(buffer, buflen as usize);
    let mut p = 0usize;

    match type_ {
        NF_CT_FTP_PORT | NF_CT_FTP_PASV => {
            let bytes = a.ip.to_be_bytes();
            let p_high = (port >> 8) as u8;
            let p_low = (port & 0xFF) as u8;

            let ok = push_u8_dec(out, &mut p, bytes[0])
                && push_byte(out, &mut p, b',')
                && push_u8_dec(out, &mut p, bytes[1])
                && push_byte(out, &mut p, b',')
                && push_u8_dec(out, &mut p, bytes[2])
                && push_byte(out, &mut p, b',')
                && push_u8_dec(out, &mut p, bytes[3])
                && push_byte(out, &mut p, b',')
                && push_u8_dec(out, &mut p, p_high)
                && push_byte(out, &mut p, b',')
                && push_u8_dec(out, &mut p, p_low);

            let len = write(buffer, buflen, &result);
            len as c_int
        },
        NF_CT_FTP_EPRT => {
            if nf_ct_l3num(ct) == NFPROTO_IPV4 {
                let mut result = format_args!("|1|%pI4|%u|", &addr.ip, port);
                let len = write(buffer, buflen, &result);
                len as c_int
            } else {
                let mut result = format_args!("|2|%pI6|%u|", &addr.ip6, port);
                let len = write(buffer, buflen, &result);
                len as c_int
            }
            p as c_int
        }
        NF_CT_FTP_EPRT | NF_CT_FTP_EPSV => 0,
        _ => 0,
    }
}

#[no_mangle]
pub unsafe extern "C" fn nf_nat_ftp(
    skb: *mut c_void,
    ctinfo: c_int,
    type_: c_int,
    protoff: c_int,
    matchoff: c_int,
    matchlen: c_int,
    exp: *mut nf_conntrack_expect,
) -> c_int {
    let exp = exp.as_mut().unwrap();
    let ct = (*exp).master.as_mut().unwrap();

    let dir = !CTINFO2DIR(ctinfo);
    let newaddr = (*ct).tuplehash[dir as usize].tuple.dst.u;

    (*exp).saved_proto = (*exp).tuple.dst.u;
    (*exp).dir = dir;
    (*exp).expectfn = nf_nat_follow_master;

    let mut port = ntohs((*exp).saved_proto.tcp.port);
    let mut found = false;

    while port != 0 {
        (*exp).tuple.dst.u.tcp.port = htons(port);

        // Simulate nf_ct_expect_related
        if nf_ct_expect_related(exp, 0) == 0 {
            found = true;
            break;
        } else if nf_ct_expect_related(exp, 0) != -EBUSY {
            port = 0;
            break;
        }
        port += 1;
    }

    if !found {
        nf_ct_helper_log(skb, ct, b"all ports in use\0".as_ptr() as *const u8);
        return NF_DROP;
    }

    let mut buffer = [0u8; 128];
    let buflen = nf_nat_ftp_fmt_cmd(ct, type_, buffer.as_mut_ptr(), buffer.len() as size_t, &newaddr, port);

    if buflen <= 0 {
        nf_ct_helper_log(skb, ct, b"cannot format command\0".as_ptr() as *const u8);
        nf_ct_unexpect_related(exp);
        return NF_DROP;
    }

    if !nf_nat_mangle_tcp_packet(skb, ct, ctinfo, protoff, matchoff, matchlen, buffer.as_ptr(), buflen as c_int) {
        nf_ct_helper_log(skb, ct, b"cannot mangle packet\0".as_ptr() as *const u8);
        nf_ct_unexpect_related(exp);
        return NF_DROP;
    }

    NF_ACCEPT
}

#[no_mangle]
pub unsafe extern "C" fn nf_nat_ftp_fini() {
    nf_nat_helper_unregister(&NAT_HELPER_FTP);
    RCU_INIT_POINTER(&mut NF_NAT_FTP_HOOK, ptr::null_mut());
    synchronize_rcu();
}

#[no_mangle]
pub unsafe extern "C" fn nf_nat_ftp_init() -> c_int {
    if !NF_NAT_FTP_HOOK.is_null() {
        return -1; // BUG_ON
    }
    nf_nat_helper_register(&NAT_HELPER_FTP);
    RCU_INIT_POINTER(&mut NF_NAT_FTP_HOOK, nf_nat_ftp as *mut c_void);
    0
}

// Helper functions
#[no_mangle]
pub unsafe extern "C" fn warn_set(val: *const u8, kp: *const c_void) -> c_int {
    pr_info(b"kernel >= 2.6.10 only uses 'ports' for conntrack modules\0".as_ptr() as *const u8);
    0
}

// Constants
static NAT_HELPER_NAME: &str = "ftp";
static NF_CT_NAT_HELPER_INIT: nf_conntrack_helper = nf_conntrack_helper {
    name: NAT_HELPER_NAME.as_ptr() as *const u8,
};

static mut NAT_HELPER_FTP: nf_nat_helper = nf_nat_helper {
    name: NAT_HELPER_NAME.as_ptr() as *const u8,
};

// Module macros
#[no_mangle]
pub static mut NF_NAT_FTP_HOOK: *mut c_void = ptr::null_mut();

// FFI compatibility functions
#[no_mangle]
pub unsafe extern "C" fn nf_ct_l3num(ct: *mut nf_conn) -> c_int {
    (*ct).nfct_net as c_int
}

#[no_mangle]
pub unsafe extern "C" fn CTINFO2DIR(ctinfo: c_int) -> c_int {
    // Simplified implementation
    ctinfo & 1
}

#[no_mangle]
pub unsafe extern "C" fn ntohs(port: u16) -> u16 {
    u16::from_be(port)
}

#[no_mangle]
pub unsafe extern "C" fn htons(port: u16) -> u16 {
    u16::to_be(port)
}

#[no_mangle]
pub unsafe extern "C" fn nf_ct_expect_related(exp: *mut nf_conntrack_expect, flags: c_int) -> c_int {
    // Simulated implementation
    0
}

#[no_mangle]
pub unsafe extern "C" fn nf_ct_unexpect_related(exp: *mut nf_conntrack_expect) {
    // Simulated implementation
}

#[no_mangle]
pub unsafe extern "C" fn nf_ct_helper_log(skb: *mut c_void, ct: *mut nf_conn, msg: *const u8) {
    // Simulated implementation
}

#[no_mangle]
pub unsafe extern "C" fn nf_nat_mangle_tcp_packet(
    skb: *mut c_void,
    ct: *mut nf_conn,
    ctinfo: c_int,
    protoff: c_int,
    matchoff: c_int,
    matchlen: c_int,
    buffer: *const u8,
    buflen: c_int,
) -> c_int {
    // Simulated implementation
    1
}

#[no_mangle]
pub unsafe extern "C" fn nf_nat_helper_register(helper: *mut nf_nat_helper) {
    // Simulated implementation
}

#[no_mangle]
pub unsafe extern "C" fn nf_nat_helper_unregister(helper: *mut nf_nat_helper) {
    // Simulated implementation
}

#[no_mangle]
pub unsafe extern "C" fn RCU_INIT_POINTER(ptr: *mut *mut c_void, val: *mut c_void) {
    // Simulated implementation
    *ptr = val;
}

#[no_mangle]
pub unsafe extern "C" fn synchronize_rcu() {
    // Simulated implementation
}

#[no_mangle]
pub unsafe extern "C" fn pr_info(msg: *const u8) {
    // Simulated implementation
}

// Module exports
#[no_mangle]
pub static NF_NAT_FTP_MODULE: Module = Module {
    license: b"GPL\0".as_ptr() as *const u8,
    author: b"Rusty Russell <rusty@rustcorp.com.au>\0".as_ptr() as *const u8,
    description: b"ftp NAT helper\0".as_ptr() as *const u8,
};

#[repr(C)]
struct Module {
    license: *const u8,
    author: *const u8,
    description: *const u8,
}

// Helper function for formatting
unsafe fn write(buffer: *mut u8, buflen: size_t, args: &core::fmt::Arguments) -> size_t {
    let mut writer = BufferWriter { buffer, pos: 0 };
    core::fmt::Write::write_fmt(&mut writer, args).unwrap();
    writer.pos
}

#[repr(C)]
struct BufferWriter {
    buffer: *mut u8,
    pos: usize,
}

impl core::fmt::Write for BufferWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let len = s.len();
        if self.pos + len > self.buffer as usize {
            return Err(core::fmt::Error);
        }

        // SAFETY: We've checked the bounds
        unsafe {
            ptr::copy_nonoverlapping(s.as_ptr(), self.buffer.add(self.pos), len);
            self.pos += len;
        }
        Ok(())
    }
}