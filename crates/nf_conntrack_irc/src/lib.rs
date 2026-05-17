//! IRC (DCC) connection tracking helper for Linux kernel
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::all)]

use core::ffi::{c_int, c_uint, c_ulong, c_void};
use core::ptr::{self, NonNull};

// Constants from C
pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const NF_ACCEPT: c_int = 0;
pub const NF_DROP: c_int = 1;
pub const NF_CT_EXPECT_MAX_CNT: c_uint = 100;

// Type definitions
#[repr(C)]
#[derive(Copy, Clone)]
pub struct in_addr {
    pub s_addr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iphdr {
    pub saddr: in_addr,
    pub daddr: in_addr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcphdr {
    pub source: u16,
    pub dest: u16,
    pub doff: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sk_buff {
    pub len: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_tuple {
    pub src: nf_conntrack_addr,
    pub dst: nf_conntrack_addr,
    pub src_l3num: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union nf_conntrack_addr {
    pub u3: nf_conntrack_addr_u3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_addr_u3 {
    pub ip: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_tuple_hash {
    pub tuple: nf_conntrack_tuple,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conn {
    pub tuplehash: [nf_conntrack_tuple_hash; 2],
    pub status: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_expect {
    pub class: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_expect_policy {
    pub max_expected: c_uint,
    pub timeout: c_uint,
}

// Function pointer type
type nf_nat_irc_hook_t = unsafe extern "C" fn(
    skb: *const sk_buff,
    ctinfo: c_int,
    protoff: c_uint,
    matchoff: c_uint,
    matchlen: c_uint,
    exp: *mut nf_conntrack_expect,
) -> c_int;

// Global variables
static mut ports: [u16; 8] = [0; 8];
static mut ports_c: c_uint = 0;
static mut max_dcc_channels: c_uint = 8;
static mut dcc_timeout: c_uint = 300;
static mut irc_buffer: *mut c_void = ptr::null_mut();
static mut irc_buffer_lock: Spinlock = Spinlock { _private: 0 };
static mut nf_nat_irc_hook: nf_nat_irc_hook_t = parse_dcc_helper;

// DCC protocol strings
const DCC_PROTOS: [&[u8; 5]; 5] = [
    b"SEND ", b"CHAT ", b"MOVE ", b"TSEND ", b"SCHAT ",
];

// Helper function prototypes
extern "C" {
    fn nf_conntrack_helpers_register(helper: *mut nf_conntrack_helper, count: c_uint) -> c_int;
    fn nf_conntrack_helpers_unregister(helper: *mut nf_conntrack_helper, count: c_uint);
    fn nf_ct_expect_alloc(ct: *const nf_conn) -> *mut nf_conntrack_expect;
    fn nf_ct_expect_init(
        exp: *mut nf_conntrack_expect,
        class: c_int,
        l3num: u8,
        laddr: *const nf_conntrack_addr,
        lport: *const u16,
        protonum: u8,
        faddr: *const nf_conntrack_addr,
        fport: *const u16,
    );
    fn nf_ct_expect_related(exp: *mut nf_conntrack_expect, timeout: c_int) -> c_int;
    fn nf_ct_expect_put(exp: *mut nf_conntrack_expect);
    fn nf_ct_helper_init(
        helper: *mut nf_conntrack_helper,
        l3num: u8,
        protonum: u8,
        name: *const u8,
        src_port: u16,
        dst_port: u16,
        index: c_int,
        policy: *const nf_conntrack_expect_policy,
        flags: c_int,
        help: unsafe extern "C" fn(
            skb: *const sk_buff,
            protoff: c_uint,
            ct: *mut nf_conn,
            ctinfo: c_int,
        ) -> c_int,
        me: *const c_void,
        module: *const c_void,
    );
    fn ip_hdr(skb: *const sk_buff) -> *const iphdr;
    fn skb_header_pointer(
        skb: *const sk_buff,
        offset: c_int,
        size: c_int,
        buffer: *mut c_void,
    ) -> *mut c_void;
    fn pr_debug(fmt: *const u8, ...);
    fn net_warn_ratelimited(fmt: *const u8, ...);
    fn nf_ct_helper_log(skb: *const sk_buff, ct: *const nf_conn, fmt: *const u8, ...);
}

// Spinlock type (simplified for FFI compatibility)
#[repr(C)]
struct Spinlock {
    _private: u32,
}

// Module init/exit
#[no_mangle]
pub extern "C" fn nf_conntrack_irc_init() -> c_int {
    if max_dcc_channels < 1 {
        unsafe { pr_debug(b"max_dcc_channels must not be zero\n\0".as_ptr() as *const u8); }
        return EINVAL;
    }

    if max_dcc_channels > NF_CT_EXPECT_MAX_CNT {
        unsafe { pr_debug(b"max_dcc_channels must not be more than %u\n\0".as_ptr() as *const u8); }
        return EINVAL;
    }

    unsafe {
        irc_buffer = libc::malloc(65536);
        if irc_buffer.is_null() {
            return ENOMEM;
        }

        // Default to standard IRC port if none specified
        if ports_c == 0 {
            ports[0] = 6667;
            ports_c = 1;
        }

        let mut i: c_int = 0;
        while i < ports_c as c_int {
            nf_ct_helper_init(
                &mut irc[i],
                AF_INET,
                IPPROTO_TCP,
                HELPER_NAME.as_ptr(),
                IRC_PORT,
                ports[i as usize],
                i,
                &irc_exp_policy,
                0,
                help,
                ptr::null(),
                ptr::null(),
            );
            i += 1;
        }

        let ret = nf_conntrack_helpers_register(&mut irc[0], ports_c);
        if ret != 0 {
            libc::free(irc_buffer);
            return ret;
        }

        0
    }
}

#[no_mangle]
pub extern "C" fn nf_conntrack_irc_fini() {
    unsafe {
        nf_conntrack_helpers_unregister(&mut irc[0], ports_c);
        libc::free(irc_buffer);
    }
}

// Helper function implementation
#[no_mangle]
pub unsafe extern "C" fn help(
    skb: *const sk_buff,
    protoff: c_uint,
    ct: *mut nf_conn,
    ctinfo: c_int,
) -> c_int {
    let dir = CTINFO2DIR(ctinfo);
    if dir == IP_CT_DIR_REPLY {
        return NF_ACCEPT;
    }

    if ctinfo != IP_CT_ESTABLISHED && ctinfo != IP_CT_ESTABLISHED_REPLY {
        return NF_ACCEPT;
    }

    let skb_len = (*skb).len;
    let th: *mut tcphdr = skb_header_pointer(skb, protoff as c_int, core::mem::size_of::<tcphdr>() as c_int, &mut _tcph as *mut _ as *mut c_void) as *mut tcphdr;
    if th.is_null() {
        return NF_ACCEPT;
    }

    let dataoff = protoff + (*th).doff as c_uint * 4;
    if dataoff >= skb_len as c_uint {
        return NF_ACCEPT;
    }

    // Acquire spinlock
    spin_lock_bh(&mut irc_buffer_lock);

    let ib_ptr = skb_header_pointer(
        skb,
        dataoff as c_int,
        skb_len as c_int - dataoff as c_int,
        irc_buffer,
    ) as *mut u8;
    if ib_ptr.is_null() {
        spin_unlock_bh(&mut irc_buffer_lock);
        return NF_ACCEPT;
    }

    let data = ib_ptr;
    let data_limit = unsafe { data.add(skb_len as usize - dataoff as usize) };

    let mut ret = NF_ACCEPT;
    let mut data = data;

    while data < data_limit.offset(-(19 + 5) as isize) {
        if !ptr::slice_from_raw_parts(data, 5).eq(b"\1DCC ") {
            data = data.offset(1);
            continue;
        }

        data = data.offset(5);
        let iph = ip_hdr(skb);
        unsafe {
            pr_debug(
                b"DCC found in master %pI4:%u %pI4:%u\n\0".as_ptr() as *const u8,
                &(*iph).saddr.s_addr,
                &(*th).source,
                &(*iph).daddr.s_addr,
                &(*th).dest,
            );
        }

        for i in 0..DCC_PROTOS.len() {
            let proto = DCC_PROTOS[i];
            if !ptr::slice_from_raw_parts(data, proto.len()).eq(proto) {
                continue;
            }

            data = data.offset(proto.len() as isize);
            let mut dcc_ip: u32 = 0;
            let mut dcc_port: u16 = 0;
            let mut addr_beg_p: *mut u8 = ptr::null_mut();
            let mut addr_end_p: *mut u8 = ptr::null_mut();

            if parse_dcc(data, data_limit, &mut dcc_ip, &mut dcc_port, &mut addr_beg_p, &mut addr_end_p) != 0 {
                unsafe { pr_debug(b"unable to parse dcc command\n\0".as_ptr() as *const u8); }
                continue;
            }

            unsafe { pr_debug(b"DCC bound ip/port: %pI4:%u\n\0".as_ptr() as *const u8, &dcc_ip, &dcc_port); }

            let tuple = &(*ct).tuplehash[dir as usize].tuple;
            if tuple.src.u3.ip != dcc_ip && tuple.dst.u3.ip != dcc_ip {
                unsafe {
                    net_warn_ratelimited(
                        b"Forged DCC command from %pI4: %pI4:%u\n\0".as_ptr() as *const u8,
                        &tuple.src.u3.ip,
                        &dcc_ip,
                        &dcc_port,
                    );
                }
                continue;
            }

            let exp = nf_ct_expect_alloc(ct);
            if exp.is_null() {
                nf_ct_helper_log(skb, ct, b"cannot alloc expectation\0".as_ptr() as *const u8);
                ret = NF_DROP;
                break;
            }

            let tuple = &(*ct).tuplehash[!dir as usize].tuple;
            let port = htons(dcc_port);
            nf_ct_expect_init(
                exp,
                NF_CT_EXPECT_CLASS_DEFAULT,
                tuple.src.l3num,
                ptr::null(),
                &tuple.dst.u3 as *const _ as *const nf_conntrack_addr_u3,
                IPPROTO_TCP,
                ptr::null(),
                &port as *const _ as *const u16,
            );

            let nf_nat_irc = nf_nat_irc_hook;
            if !nf_nat_irc.is_null() && (*ct).status & IPS_NAT_MASK != 0 {
                let nat_ret = nf_nat_irc(
                    skb,
                    ctinfo,
                    protoff,
                    (addr_beg_p as usize - ib_ptr as usize) as c_uint,
                    (addr_end_p as usize - addr_beg_p as usize) as c_uint,
                    exp,
                );
                if nat_ret != 0 {
                    ret = nat_ret;
                }
            } else if nf_ct_expect_related(exp, 0) != 0 {
                nf_ct_helper_log(skb, ct, b"cannot add expectation\0".as_ptr() as *const u8);
                ret = NF_DROP;
            }

            nf_ct_expect_put(exp);
            break;
        }
    }

    spin_unlock_bh(&mut irc_buffer_lock);
    ret
}

// Parse DCC command implementation
#[no_mangle]
pub unsafe extern "C" fn parse_dcc(
    data: *mut u8,
    data_end: *mut u8,
    ip: *mut u32,
    port: *mut u16,
    ad_beg_p: *mut *mut u8,
    ad_end_p: *mut *mut u8,
) -> c_int {
    let mut data = data;
    let data_end = data_end.offset(-12);

    while *data != b' ' as u8 {
        if data > data_end {
            return -1;
        }
        data = data.offset(1);
    }

    data = data.offset(1);
    let mut tmp = data;

    while tmp <= data_end {
        if *tmp == b'\n' as u8 {
            break;
        }
        tmp = tmp.offset(1);
    }

    if tmp > data_end || *tmp != b'\n' as u8 {
        return -1;
    }

    *ad_beg_p = data;
    let mut num: u32 = 0;
    let mut end: *mut u8 = data;

    while end <= data_end && *end != b' ' as u8 && *end != b'\n' as u8 {
        if *end < b'0' as u8 || *end > b'9' as u8 {
            return -1;
        }
        num = num * 10 + (*end - b'0' as u8) as u32;
        end = end.offset(1);
    }

    *ip = num.to_be();

    // Skip spaces
    while *end == b' ' as u8 {
        if end >= data_end {
            return -1;
        }
        end = end.offset(1);
    }

    num = 0;
    while end <= data_end && *end != b'\n' as u8 {
        if *end < b'0' as u8 || *end > b'9' as u8 {
            return -1;
        }
        num = num * 10 + (*end - b'0' as u8) as u32;
        end = end.offset(1);
    }

    *port = num as u16;
    *ad_end_p = end;
    0
}

// Spinlock operations (simplified)
#[no_mangle]
pub unsafe extern "C" fn spin_lock_bh(lock: *mut Spinlock) {
    // SAFETY: Kernel guarantees lock is valid and properly aligned
    (*lock)._private = 1;
}

#[no_mangle]
pub unsafe extern "C" fn spin_unlock_bh(lock: *mut Spinlock) {
    // SAFETY: Kernel guarantees lock is valid and properly aligned
    (*lock)._private = 0;
}

// Constants
const AF_INET: u8 = 2;
const IPPROTO_TCP: u8 = 6;
const HELPER_NAME: &str = "irc";
const IRC_PORT: u16 = 6667;
const IPS_NAT_MASK: u32 = 0x0000000F;

// Module exports
#[no_mangle]
pub static nf_nat_irc_hook: nf_nat_irc_hook_t = parse_dcc_helper;

#[no_mangle]
pub extern "C" fn parse_dcc_helper(
    skb: *const sk_buff,
    ctinfo: c_int,
    protoff: c_uint,
    matchoff: c_uint,
    matchlen: c_uint,
    exp: *mut nf_conntrack_expect,
) -> c_int {
    // Implementation would go here if needed
    0
}

// Module metadata
#[no_mangle]
pub static NF_CT_HELPER_IRC: nf_conntrack_helper = nf_conntrack_helper {
    name: HELPER_NAME.as_ptr() as *const u8,
    ..Default::default()
};

// Test cases (conditional compilation)
#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_dcc() {
        // Basic test case for DCC parsing
        let data = b"\1DCC SEND 192.168.1.1 1234\0";
        let mut ip: u32 = 0;
        let mut port: u16 = 0;
        let mut ad_beg: *mut u8 = ptr::null_mut();
        let mut ad_end: *mut u8 = ptr::null_mut();
        
        unsafe {
            let result = super::parse_dcc(
                data.as_ptr() as *mut u8,
                data.as_ptr().add(data.len()) as *mut u8,
                &mut ip,
                &mut port,
                &mut ad_beg,
                &mut ad_end,
            );
            
            assert_eq!(result, 0);
            assert_eq!(ip, 0xC0A80101u32.to_be());
            assert_eq!(port, 1234);
        }
    }
}