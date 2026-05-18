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
pub struct nf_conn {
    pub tuplehash: [nf_conn_tuplehash; 2],
    pub nfct_net: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_expect {
    pub master: *mut nf_conn,
    pub saved_proto: nf_ct_port,
    pub tuple: nf_conntrack_tuple,
    pub dir: c_int,
    pub expectfn: Option<unsafe extern "C" fn(*mut nf_conntrack_expect)>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_helper {
    pub name: *const c_char,
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

            if !ok {
                return 0;
            }
            p as c_int
        }
        NF_CT_FTP_EPRT | NF_CT_FTP_EPSV => 0,
        _ => 0,
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    loop {}
}