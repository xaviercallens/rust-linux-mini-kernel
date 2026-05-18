#![no_std]
#![no_main]
#![allow(non_camel_case_types)]

use core::ffi::{c_int, c_uint, c_void};
use kernel_types::*;

pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ENOSYS: c_int = -38;

pub const HZ: c_int = 100;
pub const IPPROTO_GRE: u8 = 47;

pub const PPTP_START_SESSION_REQUEST: u16 = 1;
pub const PPTP_START_SESSION_REPLY: u16 = 2;
pub const PPTP_STOP_SESSION_REQUEST: u16 = 5;
pub const PPTP_STOP_SESSION_REPLY: u16 = 6;
pub const PPTP_OUT_CALL_REQUEST: u16 = 10;
pub const PPTP_OUT_CALL_REPLY: u16 = 11;
pub const PPTP_IN_CALL_REQUEST: u16 = 12;
pub const PPTP_IN_CALL_REPLY: u16 = 13;
pub const PPTP_IN_CALL_CONNECT: u16 = 14;
pub const PPTP_CALL_CLEAR_REQUEST: u16 = 15;
pub const PPTP_CALL_DISCONNECT_NOTIFY: u16 = 16;
pub const PPTP_WAN_ERROR_NOTIFY: u16 = 17;
pub const PPTP_SET_LINK_INFO: u16 = 18;
pub const PPTP_MSG_MAX: u16 = 18;

pub const PPTP_GRE_TIMEOUT: c_int = 10 * 60 * HZ;
pub const PPTP_GRE_STREAM_TIMEOUT: c_int = 5 * 60 * 60 * HZ;

pub const PPTP_SESSION_NONE: c_int = 0;
pub const PPTP_SESSION_REQUESTED: c_int = 1;
pub const PPTP_SESSION_CONFIRMED: c_int = 2;
pub const PPTP_SESSION_ERROR: c_int = 3;
pub const PPTP_SESSION_STOPREQ: c_int = 4;

pub const PPTP_START_OK: u16 = 1;
pub const PPTP_STOP_OK: u16 = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PptpControlHeader {
    pub messageType: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PptpStartSessionReply {
    pub resultCode: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PptpStopSessionReply {
    pub resultCode: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PptpOutCallAck {
    pub callID: u16,
    pub peersCallID: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union pptp_ctrl_union {
    pub srep: PptpStartSessionReply,
    pub strep: PptpStopSessionReply,
    pub ocack: PptpOutCallAck,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_gre_address {
    pub key: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union nf_conntrack_address_u3 {
    pub gre: nf_conntrack_gre_address,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_address {
    pub u3: nf_conntrack_address_u3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_tuple {
    pub src: nf_conntrack_address,
    pub dst: nf_conntrack_address,
    pub protonum: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conn_proto_gre {
    pub timeout: c_int,
    pub stream_timeout: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conn_proto {
    pub gre: nf_conn_proto_gre,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conn {
    pub proto: nf_conn_proto,
    pub master: *mut nf_conn,
    pub status: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_expect {
    pub tuple: nf_conntrack_tuple,
    pub expectfn: Option<unsafe extern "C" fn(*mut nf_conn, *mut nf_conntrack_expect)>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_ct_pptp_master {
    pub sstate: c_int,
    pub cstate: c_int,
    pub pns_call_id: u16,
    pub pac_call_id: u16,
}

pub type nf_nat_pptp_hook_outbound_t = Option<
    unsafe extern "C" fn(
        *mut c_void,
        *mut nf_conn,
        c_int,
        c_uint,
        *mut PptpControlHeader,
        *mut pptp_ctrl_union,
    ) -> c_int,
>;

pub type nf_nat_pptp_hook_inbound_t = Option<
    unsafe extern "C" fn(
        *mut c_void,
        *mut nf_conn,
        c_int,
        c_uint,
        *mut PptpControlHeader,
        *mut pptp_ctrl_union,
    ) -> c_int,
>;

pub type nf_nat_pptp_hook_exp_gre_t =
    Option<unsafe extern "C" fn(*mut nf_conntrack_expect, *mut nf_conntrack_expect)>;

pub type nf_nat_pptp_hook_expectfn_t =
    Option<unsafe extern "C" fn(*mut nf_conn, *mut nf_conntrack_expect)>;

static mut nf_nat_pptp_hook_outbound: nf_nat_pptp_hook_outbound_t = None;
static mut nf_nat_pptp_hook_inbound: nf_nat_pptp_hook_inbound_t = None;
static mut nf_nat_pptp_hook_exp_gre: nf_nat_pptp_hook_exp_gre_t = None;
static mut nf_nat_pptp_hook_expectfn: nf_nat_pptp_hook_expectfn_t = None;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct spinlock_t {
    _private: [u8; 0],
}

static nf_pptp_lock: spinlock_t = spinlock_t { _private: [] };

#[no_mangle]
pub unsafe extern "C" fn pptp_expectfn(ct: *mut nf_conn, exp: *mut nf_conntrack_expect) {
    if ct.is_null() || exp.is_null() {
        return;
    }

    (*ct).proto.gre.timeout = PPTP_GRE_TIMEOUT;
    (*ct).proto.gre.stream_timeout = PPTP_GRE_STREAM_TIMEOUT;

    (*exp).tuple.protonum = IPPROTO_GRE;
}

#[no_mangle]
pub unsafe extern "C" fn nf_conntrack_pptp_init() -> c_int {
    let _ = &nf_pptp_lock;
    nf_nat_pptp_hook_expectfn = Some(pptp_expectfn);
    0
}

#[no_mangle]
pub unsafe extern "C" fn nf_conntrack_pptp_fini() {
    nf_nat_pptp_hook_expectfn = None;
    nf_nat_pptp_hook_outbound = None;
    nf_nat_pptp_hook_inbound = None;
    nf_nat_pptp_hook_exp_gre = None;
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}