#![no_std]
#![no_main]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::all)]

use core::panic::PanicInfo;
use core::ptr;
use kernel_types::*;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Constants
pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const NF_ACCEPT: c_int = 0;
pub const NF_DROP: c_int = 1;
pub const NF_CT_EXPECT_MAX_CNT: c_uint = 100;

// Basic protocol/layout types
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
pub struct nf_conntrack_addr_u3 {
    pub ip: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union nf_conntrack_addr {
    pub u3: nf_conntrack_addr_u3,
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_helper {
    _priv: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Spinlock {
    _private: u32,
}

type nf_nat_irc_hook_t = Option<
    unsafe extern "C" fn(
        skb: *const sk_buff,
        ctinfo: c_int,
        protoff: c_uint,
        matchoff: c_uint,
        matchlen: c_uint,
        exp: *mut nf_conntrack_expect,
    ) -> c_int,
>;

static mut ports: [u16; 8] = [0; 8];
static mut ports_c: c_uint = 0;
static mut max_dcc_channels: c_uint = 8;
static mut dcc_timeout: c_uint = 300;
static mut irc_buffer: *mut c_void = ptr::null_mut();
static mut irc_buffer_lock: Spinlock = Spinlock { _private: 0 };

unsafe extern "C" {
    static mut nf_nat_irc_hook: nf_nat_irc_hook_t;
}

const DCC_PROTOS: [&[u8; 6]; 5] = [b"SEND \0", b"CHAT \0", b"MOVE \0", b"TSEND\0", b"SCHAT\0"];

static mut irc: [nf_conntrack_helper; 8] = [nf_conntrack_helper { _priv: [] }; 8];
static irc_exp_policy: nf_conntrack_expect_policy = nf_conntrack_expect_policy {
    max_expected: 1,
    timeout: 300,
};

unsafe extern "C" {
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

    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
}

unsafe extern "C" fn help(
    _skb: *const sk_buff,
    _protoff: c_uint,
    _ct: *mut nf_conn,
    _ctinfo: c_int,
) -> c_int {
    NF_ACCEPT
}

#[unsafe(no_mangle)]
pub extern "C" fn nf_conntrack_irc_init() -> c_int {
    unsafe {
        if max_dcc_channels < 1 {
            pr_debug(b"max_dcc_channels must not be zero\n\0".as_ptr());
            return EINVAL;
        }

        if max_dcc_channels > NF_CT_EXPECT_MAX_CNT {
            pr_debug(b"max_dcc_channels too large\n\0".as_ptr());
            return EINVAL;
        }

        ports_c = 1;
        ports[0] = 6667;

        if irc_buffer.is_null() {
            irc_buffer = malloc(4096);
            if irc_buffer.is_null() {
                return ENOMEM;
            }
        }

        let _ = DCC_PROTOS;
        let _ = &irc_buffer_lock as *const Spinlock;
        let _ = nf_nat_irc_hook.is_some();

        nf_conntrack_helpers_register(core::ptr::addr_of_mut!(irc[0]), ports_c)
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn nf_conntrack_irc_fini() {
    unsafe {
        nf_conntrack_helpers_unregister(core::ptr::addr_of_mut!(irc[0]), ports_c);
        if !irc_buffer.is_null() {
            free(irc_buffer);
            irc_buffer = ptr::null_mut();
        }
    }
}