#![no_std]
#![no_main]
#![allow(non_camel_case_types)]

use core::ffi::c_char;
use core::panic::PanicInfo;
use kernel_types::*;

pub const DCCP_MSL: c_int = 2 * 60;

pub const CT_DCCP_NONE: c_int = 0;
pub const CT_DCCP_REQUEST: c_int = 1;
pub const CT_DCCP_RESPOND: c_int = 2;
pub const CT_DCCP_PARTOPEN: c_int = 3;
pub const CT_DCCP_OPEN: c_int = 4;
pub const CT_DCCP_CLOSEREQ: c_int = 5;
pub const CT_DCCP_CLOSING: c_int = 6;
pub const CT_DCCP_TIMEWAIT: c_int = 7;
pub const CT_DCCP_IGNORE: c_int = 8;
pub const CT_DCCP_INVALID: c_int = 9;

pub const DCCP_PKT_REQUEST: c_int = 0;
pub const DCCP_PKT_RESPONSE: c_int = 1;
pub const DCCP_PKT_ACK: c_int = 2;
pub const DCCP_PKT_DATA: c_int = 3;
pub const DCCP_PKT_DATAACK: c_int = 4;
pub const DCCP_PKT_CLOSEREQ: c_int = 5;
pub const DCCP_PKT_CLOSE: c_int = 6;
pub const DCCP_PKT_RESET: c_int = 7;
pub const DCCP_PKT_SYNC: c_int = 8;
pub const DCCP_PKT_SYNCACK: c_int = 9;

pub const CT_DCCP_ROLE_CLIENT: c_int = 0;
pub const CT_DCCP_ROLE_SERVER: c_int = 1;

const DCCP_ROLE_MAX: usize = (CT_DCCP_ROLE_SERVER as usize) + 1;
const DCCP_PKT_MAX: usize = (DCCP_PKT_SYNCACK as usize) + 1;
const DCCP_STATE_MAX: usize = (CT_DCCP_INVALID as usize) + 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct DccpStateNames {
    pub names: [*const c_char; DCCP_STATE_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct DccpStateTable {
    pub table: [[[c_int; DCCP_STATE_MAX]; DCCP_PKT_MAX]; DCCP_ROLE_MAX],
}

static DCCP_STATE_TABLE: DccpStateTable = DccpStateTable {
    table: [[[CT_DCCP_INVALID; DCCP_STATE_MAX]; DCCP_PKT_MAX]; DCCP_ROLE_MAX],
};

#[repr(C)]
pub struct sk_buff {
    _priv: [u8; 0],
}

#[repr(C)]
pub struct nf_conn {
    _priv: [u8; 0],
}

#[repr(C)]
pub struct nf_conntrack_tuple {
    _priv: [u8; 0],
}

#[repr(C)]
pub struct net {
    _priv: [u8; 0],
}

#[unsafe(no_mangle)]
pub extern "C" fn nf_conntrack_dccp_packet(
    _ct: *mut nf_conn,
    _skb: *const sk_buff,
    _dataoff: u32,
    _pf: u8,
    _hooknum: u8,
) -> c_int {
    let _ = &DCCP_STATE_TABLE;
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn nf_conntrack_dccp_new(
    _ct: *mut nf_conn,
    _skb: *const sk_buff,
    _dataoff: u32,
    _timeout: u32,
) -> bool {
    true
}

#[unsafe(no_mangle)]
pub extern "C" fn nf_conntrack_dccp_error(
    _net: *mut net,
    _tmpl: *mut nf_conn,
    _skb: *const sk_buff,
    _dataoff: u32,
    _pf: u8,
    _hooknum: u8,
) -> c_int {
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn nf_conntrack_dccp_loose() -> c_int {
    1
}

#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    loop {
        core::hint::spin_loop();
    }
}