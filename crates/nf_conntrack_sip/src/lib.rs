
//! SIP connection tracking helper for Linux kernel
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(clippy::too_many_arguments)]

use core::ffi::{c_char, c_int, c_uchar};
use core::mem;
use core::ptr;
use kernel_types::*;

pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ENOSYS: c_int = -38;
pub const SIP_PORT: u16 = 5060;
pub const SIP_TIMEOUT: u32 = 1200;

// Type definitions

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_expect {
    pub _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_nat_sip_hooks {
    pub _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sip_header {
    pub name: *const c_char,
    pub short_name: *const c_char,
    pub uri_prefix: *const c_char,
    pub value_len:
        Option<unsafe extern "C" fn(*const nf_conn, *const c_uchar, *const c_uchar, *mut c_int) -> c_int>,
}

#[inline]
unsafe fn isalpha(c: c_uchar) -> c_int {
    if (c >= b'a' && c <= b'z') || (c >= b'A' && c <= b'Z') {
        1
    } else {
        0
    }
}

#[inline]
unsafe fn isdigit(c: c_uchar) -> c_int {
    if c >= b'0' && c <= b'9' {
        1
    } else {
        0
    }
}

#[inline]
unsafe fn isalnum(c: c_uchar) -> c_int {
    if isalpha(c) != 0 || isdigit(c) != 0 {
        1
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn string_len(
    _ct: *const nf_conn,
    dptr: *const c_uchar,
    limit: *const c_uchar,
    shift: *mut c_int,
) -> c_int {
    let mut len: c_int = 0;
    let mut current = dptr;

    while current < limit && isalpha(*current) != 0 {
        current = current.add(1);
        len += 1;
    }

    if !shift.is_null() {
        *shift = len;
    }
    len
}

#[no_mangle]
pub unsafe extern "C" fn digits_len(
    _ct: *const nf_conn,
    dptr: *const c_uchar,
    limit: *const c_uchar,
    shift: *mut c_int,
) -> c_int {
    let mut len: c_int = 0;
    let mut current = dptr;

    while current < limit && isdigit(*current) != 0 {
        current = current.add(1);
        len += 1;
    }

    if !shift.is_null() {
        *shift = len;
    }
    len
}

#[no_mangle]
pub unsafe extern "C" fn iswordc(c: c_uchar) -> c_int {
    if isalnum(c) != 0
        || c == b'!'
        || c == b'"'
        || c == b'%'
        || (c >= b'(' && c <= b'+')
        || c == b':'
        || c == b'<'
        || c == b'>'
        || c == b'?'
        || (c >= b'[' && c <= b']')
        || c == b'_'
        || c == b'`'
        || c == b'{'
        || c == b'}'
        || c == b'~'
        || (c >= b'-' && c <= b'/')
        || c == b'\''
    {
        1
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn word_len(dptr: *const c_uchar, limit: *const c_uchar) -> c_int {
    let mut len: c_int = 0;
    let mut current = dptr;

    while current < limit && iswordc(*current) != 0 {
        current = current.add(1);
        len += 1;
    }

    len
}

#[no_mangle]
pub unsafe extern "C" fn callid_len(
    _ct: *const nf_conn,
    dptr: *const c_uchar,
    limit: *const c_uchar,
    shift: *mut c_int,
) -> c_int {
    let mut len = word_len(dptr, limit);
    let mut current = dptr.add(len as usize);

    if len == 0 || current >= limit || *current != b'@' {
        if !shift.is_null() {
            *shift = len;
        }
        return len;
    }

    current = current.add(1);
    len += 1;

    let domain_len = word_len(current, limit);
    if domain_len == 0 {
        if !shift.is_null() {
            *shift = 0;
        }
        return 0;
    }

    len += domain_len;
    if !shift.is_null() {
        *shift = len;
    }
    len
}

#[no_mangle]
pub unsafe extern "C" fn media_len(
    ct: *const nf_conn,
    dptr: *const c_uchar,
    limit: *const c_uchar,
    shift: *mut c_int,
) -> c_int {
    let mut len = string_len(ct, dptr, limit, shift);
    let mut current = dptr.add(len as usize);

    if current >= limit || *current != b' ' {
        if !shift.is_null() {
            *shift = len;
        }
        return 0;
    }

    len += 1;
    current = current.add(1);

    len += digits_len(ct, current, limit, shift);
    if !shift.is_null() {
        *shift = len;
    }
    len
}

unsafe fn nf_ct_l3num(_ct: *const nf_conn) -> c_int {
    AF_INET
}

unsafe fn in4_pton(
    src: *const c_uchar,
    srclen: c_int,
    dst: *mut c_uchar,
    _delim: c_int,
    end: *mut *const c_uchar,
) -> c_int {
    if src.is_null() || dst.is_null() || srclen <= 0 {
        return 0;
    }
    if !end.is_null() {
        *end = src.add(srclen as usize);
    }
    1
}

unsafe fn in6_pton(
    src: *const c_uchar,
    srclen: c_int,
    dst: *mut c_uchar,
    _delim: c_int,
    end: *mut *const c_uchar,
) -> c_int {
    if src.is_null() || dst.is_null() || srclen <= 0 {
        return 0;
    }
    if !end.is_null() {
        *end = src.add(srclen as usize);
    }
    1
}

#[no_mangle]
pub unsafe extern "C" fn sip_parse_addr(
    ct: *const nf_conn,
    cp: *const c_uchar,
    endp: *mut *const c_uchar,
    addr: *mut nf_inet_addr,
    limit: *const c_uchar,
    _delim: c_int,
) -> c_int {
    if ct.is_null() || addr.is_null() || cp.is_null() || limit.is_null() {
        return 0;
    }

    ptr::write_bytes(addr as *mut u8, 0, mem::size_of::<nf_inet_addr>());

    match nf_ct_l3num(ct) {
        AF_INET => {
            let mut end: *const c_uchar = ptr::null();
            let ret = in4_pton(
                cp,
                (limit as usize).wrapping_sub(cp as usize) as c_int,
                addr as *mut c_uchar,
                -1,
                &mut end,
            );
            if ret == 0 {
                return 0;
            }
            if !endp.is_null() {
                *endp = end;
            }
            1
        }
        AF_INET6 => {
            let mut end: *const c_uchar = ptr::null();
            let ret = in6_pton(
                cp,
                (limit as usize).wrapping_sub(cp as usize) as c_int,
                addr as *mut c_uchar,
                -1,
                &mut end,
            );
            if ret == 0 {
                return 0;
            }
            if !endp.is_null() {
                *endp = end;
            }
            1
        }
        _ => 0,
    }
}

#[no_mangle]
pub unsafe extern "C" fn epaddr_len(ct: *const nf_conn, dptr: *const u8, limit: *const u8, shift: *mut c_int) -> c_int {
    let mut addr: nf_inet_addr = mem::zeroed();
    let mut end: *const u8 = ptr::null();
    let mut aux = dptr;

    if sip_parse_addr(ct, dptr, &mut end, &mut addr, limit, 1) == 0 {
        pr_debug(b"ip: %s parse failed.\n\0".as_ptr() as *const u8, dptr);
        return 0;
    }

    let mut length = end.offset_from(aux) as c_int;

    if end < limit && *end == b':' {
        let mut current = end.offset(1);
        let port_len = digits_len(ct, current, limit, shift);
        length += port_len as c_int;
    }

    if !shift.is_null() {
        *shift = length;
    }
    length
}

#[no_mangle]
pub unsafe extern "C" fn skp_epaddr_len(ct: *const nf_conn, dptr: *const u8, limit: *const u8, shift: *mut c_int) -> c_int {
    let mut start = dptr;
    let mut s = if !shift.is_null() { *shift } else { 0 };
    let mut current = dptr;

    while current < limit && *current != b'@' && *current != b'\r' && *current != b'\n' {
        if !shift.is_null() {
            *shift += 1;
        }
        current = current.offset(1);
    }

    if current < limit && *current == b'@' {
        current = current.offset(1);
        if !shift.is_null() {
            *shift += 1;
        }
    } else {
        current = start;
        if !shift.is_null() {
            *shift = s;
        }
    }

    epaddr_len(ct, current, limit, shift)
}

#[no_mangle]
pub unsafe extern "C" fn ct_sip_parse_request(ct: *const nf_conn, dptr: *const u8, datalen: c_uint, matchoff: *mut c_uint, matchlen: *mut c_uint, addr: *mut nf_inet_addr, port: *mut u16) -> c_int {
    let start = dptr;
    let limit = dptr.offset(datalen as isize);
    let mut current = dptr;
    let mut mlen: c_int = 0;
    let mut shift = 0;

    // Skip method and whitespace
    mlen = string_len(ct, current, limit, ptr::null_mut());
    if mlen == 0 {
        return 0;
    }

    current = current.offset(mlen as isize);
    if current < limit {
        current = current.offset(1);
    } else {
        return 0;
    }

    // Find SIP URI
    while current < limit.offset(-(4 as isize)) {
        if *current == b'\r' || *current == b'\n' {
            return -1;
        }
        if *current == b's' || *current == b'S' {
            if *(current.offset(1)) == b'i' &&
               *(current.offset(2)) == b'p' &&
               *(current.offset(3)) == b':' {
                current = current.offset(4);
                break;
            }
        }
        current = current.offset(1);
    }

    if skp_epaddr_len(ct, current, limit, &mut shift) == 0 {
        return 0;
    }

    current = current.offset(shift as isize);

    let mut end: *const u8 = ptr::null();
    if sip_parse_addr(ct, current, &mut end, addr, limit, 1) == 0 {
        return -1;
    }

    let mut p: u16 = SIP_PORT;
    if end < limit && *end == b':' {
        let mut current_port = end.offset(1);
        let mut port_str = [0u8; 6]; // Max 5 digits + null
        let mut i: c_int = 0;

        while current_port < limit && isdigit(*current_port) != 0 && i < 5 {
            port_str[i as usize] = *current_port;
            current_port = current_port.offset(1);
            i += 1;
        }

        port_str[i as usize] = 0;
        p = u16::from_str_radix(core::str::from_utf8_unchecked(&port_str[..i as usize]), 10).unwrap_or(SIP_PORT);

        if p < 1024 || p > 65535 {
            return -1;
        }
    }

    ptr::write(port, p);

    if end == current {
        return 0;
    }

    ptr::write(matchoff, current.offset_from(start) as c_uint);
    ptr::write(matchlen, end.offset_from(current) as c_uint);
    1
}

#[no_mangle]
pub static mut NF_NAT_SIP_HOOKS: *mut nf_nat_sip_hooks = ptr::null_mut();

// Helper functions (would be implemented in C headers)
#[no_mangle]
pub unsafe extern "C" fn nf_ct_l3num(ct: *const nf_conn) -> c_int {
    // Stub implementation - actual implementation would read from nf_conn
    2 // AF_INET
}

#[no_mangle]
pub unsafe extern "C" fn in4_pton(cp: *const u8, len: c_int, buf: *mut u8, _flags: c_int, end: *mut *const u8) -> c_int {
    // Stub implementation - actual implementation would parse IPv4
    1
}

#[no_mangle]
pub unsafe extern "C" fn in6_pton(cp: *const u8, len: c_int, buf: *mut u8, _flags: c_int, end: *mut *const u8) -> c_int {
    // Stub implementation - actual implementation would parse IPv6
    1
}

#[no_mangle]
pub unsafe extern "C" fn isalpha(c: u8) -> c_int {
    if (c >= b'A' && c <= b'Z') || (c >= b'a' && c <= b'z') {
        1
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn isdigit(c: u8) -> c_int {
    if c >= b'0' && c <= b'9' {
        1
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn isalnum(c: u8) -> c_int {
    isalpha(c) != 0 || isdigit(c) != 0
}

#[no_mangle]
pub unsafe extern "C" fn pr_debug(fmt: *const u8, args: ...) {
    // Stub implementation for debugging
}

// Module parameters (simplified)
static mut PORTS: [u16; 8] = [0; 8];
static mut PORTS_C: usize = 0;
static mut SIP_TIMEOUT: u32 = SIP_TIMEOUT;
static mut SIP_DIRECT_SIGNALLING: c_int = 1;
static mut SIP_DIRECT_MEDIA: c_int = 1;
static mut SIP_EXTERNAL_MEDIA: c_int = 0;

// These would be implemented with proper module_param macros in a real kernel module
#[no_mangle]
pub unsafe extern "C" fn module_param_ports() {
    // Stub
}

#[no_mangle]
pub unsafe extern "C" fn module_param_sip_timeout() {
    // Stub
}

// Exported symbols
#[no_mangle]
pub static HELPER_NAME: [u8; 4] = *b"SIP\0";

#[no_mangle]
pub static NF_CT_HELPER_SIP: nf_conntrack_helper = nf_conntrack_helper {
    _private: [0; 0],
};

#[no_mangle]
pub static CT_SIP_HDRS: [sip_header; 9] = unsafe {
    [
        sip_header {
            name: b"CSeq\0".as_ptr() as *const u8,
            short_name: ptr::null(),
            uri_prefix: ptr::null(),
            value_len: Some(string_len),
        },
        sip_header {
            name: b"From\0".as_ptr() as *const u8,
            short_name: b"f\0".as_ptr() as *const u8,
            uri_prefix: b"sip:\0".as_ptr() as *const u8,
            value_len: Some(skp_epaddr_len),
        },
        sip_header {
            name: b"To\0".as_ptr() as *const u8,
            short_name: b"t\0".as_ptr() as *const u8,
            uri_prefix: b"sip:\0".as_ptr() as *const u8,
            value_len: Some(skp_epaddr_len),
        },
        sip_header {
            name: b"Contact\0".as_ptr() as *const u8,
            short_name: b"m\0".as_ptr() as *const u8,
            uri_prefix: b"sip:\0".as_ptr() as *const u8,
            value_len: Some(skp_epaddr_len),
        },
        sip_header {
            name: b"Via\0".as_ptr() as *const u8,
            short_name: b"v\0".as_ptr() as *const u8,
            uri_prefix: b"UDP \0".as_ptr() as *const u8,
            value_len: Some(epaddr_len),
        },
        sip_header {
            name: b"Via\0".as_ptr() as *const u8,
            short_name: b"v\0".as_ptr() as *const u8,
            uri_prefix: b"TCP \0".as_ptr() as *const u8,
            value_len: Some(epaddr_len),
        },
        sip_header {
            name: b"Expires\0".as_ptr() as *const u8,
            short_name: ptr::null(),
            uri_prefix: ptr::null(),
            value_len: Some(digits_len),
        },
        sip_header {
            name: b"Content-Length\0".as_ptr() as *const u8,
            short_name: b"l\0".as_ptr() as *const u8,
            uri_prefix: ptr::null(),
            value_len: Some(digits_len),
        },
        sip_header {
            name: b"Call-Id\0".as_ptr() as *const u8,
            short_name: b"i\0".as_ptr() as *const u8,
            uri_prefix: ptr::null(),
            value_len: Some(callid_len),
        },
    ]
};

// Module metadata (would be implemented with proper macros in a real kernel module)
#[no_mangle]
pub static MODULE_LICENSE: [u8; 4] = *b"GPL\0";
#[no_mangle]
pub static MODULE_AUTHOR: [u8; 44] = *b"Christian Hentschel <chentschel@arnet.com.ar>\0";
#[no_mangle]
pub static MODULE_DESCRIPTION: [u8; 27] = *b"SIP connection tracking helper\0";
#[no_mangle]
pub static MODULE_ALIAS: [u8; 17] = *b"ip_conntrack_sip\0";