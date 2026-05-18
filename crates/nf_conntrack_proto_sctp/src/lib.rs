#![no_std]
#![no_main]
#![allow(non_camel_case_types)]

use core::panic::PanicInfo;
use kernel_types::*;

pub const SCTP_CID_INIT: u8 = 1;
pub const SCTP_CID_INIT_ACK: u8 = 2;
pub const SCTP_CID_HEARTBEAT: u8 = 4;
pub const SCTP_CID_HEARTBEAT_ACK: u8 = 5;
pub const SCTP_CID_ABORT: u8 = 6;
pub const SCTP_CID_SHUTDOWN: u8 = 7;
pub const SCTP_CID_SHUTDOWN_ACK: u8 = 8;
pub const SCTP_CID_ERROR: u8 = 9;
pub const SCTP_CID_COOKIE_ECHO: u8 = 10;
pub const SCTP_CID_COOKIE_ACK: u8 = 11;
pub const SCTP_CID_SHUTDOWN_COMPLETE: u8 = 14;

pub const SCTP_CONNTRACK_NONE: u8 = 0;
pub const SCTP_CONNTRACK_CLOSED: u8 = 1;
pub const SCTP_CONNTRACK_COOKIE_WAIT: u8 = 2;
pub const SCTP_CONNTRACK_COOKIE_ECHOED: u8 = 3;
pub const SCTP_CONNTRACK_ESTABLISHED: u8 = 4;
pub const SCTP_CONNTRACK_SHUTDOWN_SENT: u8 = 5;
pub const SCTP_CONNTRACK_SHUTDOWN_RECD: u8 = 6;
pub const SCTP_CONNTRACK_SHUTDOWN_ACK_SENT: u8 = 7;
pub const SCTP_CONNTRACK_HEARTBEAT_SENT: u8 = 8;
pub const SCTP_CONNTRACK_HEARTBEAT_ACKED: u8 = 9;
pub const SCTP_CONNTRACK_MAX: u8 = 10;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctphdr {
    pub source: c_ushort,
    pub dest: c_ushort,
    pub vtag: c_uint,
    pub checksum: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_chunkhdr {
    pub type_: c_uchar,
    pub flags: c_uchar,
    pub length: c_ushort,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_conntrack {
    pub state: c_uchar,
    pub vtag: [c_uint; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_proto {
    pub sctp: sctp_conntrack,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conn {
    pub proto: nf_conntrack_proto,
}

#[repr(C)]
pub struct sk_buff {
    _priv: [u8; 0],
}

static SCTP_CONNTRACK_NAMES: [&str; (SCTP_CONNTRACK_MAX as usize) + 1] = [
    "NONE",
    "CLOSED",
    "COOKIE_WAIT",
    "COOKIE_ECHOED",
    "ESTABLISHED",
    "SHUTDOWN_SENT",
    "SHUTDOWN_RECD",
    "SHUTDOWN_ACK_SENT",
    "HEARTBEAT_SENT",
    "HEARTBEAT_ACKED",
    "MAX",
];

static SCTP_TIMEOUTS: [c_uint; SCTP_CONNTRACK_MAX as usize] = [10, 3, 3, 432000, 3, 3, 3, 30, 210, 210];

#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_eh_personality() {}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sctp_print_conntrack(_s: *mut c_void, ct: *const nf_conn) {
    if ct.is_null() {
        return;
    }
    let state = (*ct).proto.sctp.state as usize;
    let _name = if state < SCTP_CONNTRACK_NAMES.len() {
        SCTP_CONNTRACK_NAMES[state]
    } else {
        "UNKNOWN"
    };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_basic_checks(
    _ct: *mut nf_conn,
    _skb: *mut sk_buff,
    _dataoff: c_uint,
    _map: *mut c_void,
) -> c_int {
    1
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn new_state(ct: *mut nf_conn, dir: c_uint, chunk_type: c_uchar) -> c_uchar {
    if ct.is_null() {
        return SCTP_CONNTRACK_NONE;
    }

    let old = (*ct).proto.sctp.state;
    let mut new = old;

    if dir > 1 {
        return old;
    }

    match chunk_type {
        SCTP_CID_INIT => {
            new = if dir == 0 {
                SCTP_CONNTRACK_COOKIE_WAIT
            } else {
                SCTP_CONNTRACK_CLOSED
            };
        }
        SCTP_CID_INIT_ACK => {
            if old == SCTP_CONNTRACK_COOKIE_WAIT {
                new = SCTP_CONNTRACK_COOKIE_ECHOED;
            }
        }
        SCTP_CID_COOKIE_ECHO => {
            if old == SCTP_CONNTRACK_COOKIE_WAIT || old == SCTP_CONNTRACK_COOKIE_ECHOED {
                new = SCTP_CONNTRACK_COOKIE_ECHOED;
            }
        }
        SCTP_CID_COOKIE_ACK => {
            if old == SCTP_CONNTRACK_COOKIE_ECHOED {
                new = SCTP_CONNTRACK_ESTABLISHED;
            }
        }
        SCTP_CID_SHUTDOWN => {
            if old == SCTP_CONNTRACK_ESTABLISHED {
                new = SCTP_CONNTRACK_SHUTDOWN_SENT;
            }
        }
        SCTP_CID_SHUTDOWN_ACK => {
            if old == SCTP_CONNTRACK_SHUTDOWN_SENT || old == SCTP_CONNTRACK_SHUTDOWN_RECD {
                new = SCTP_CONNTRACK_SHUTDOWN_ACK_SENT;
            }
        }
        SCTP_CID_SHUTDOWN_COMPLETE => {
            new = SCTP_CONNTRACK_CLOSED;
        }
        SCTP_CID_ABORT => {
            new = SCTP_CONNTRACK_CLOSED;
        }
        SCTP_CID_HEARTBEAT => {
            if old == SCTP_CONNTRACK_ESTABLISHED {
                new = SCTP_CONNTRACK_HEARTBEAT_SENT;
            }
        }
        SCTP_CID_HEARTBEAT_ACK => {
            if old == SCTP_CONNTRACK_HEARTBEAT_SENT {
                new = SCTP_CONNTRACK_HEARTBEAT_ACKED;
            }
        }
        _ => {}
    }

    (*ct).proto.sctp.state = new;
    new
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sctp_packet(
    ct: *mut nf_conn,
    skb: *mut sk_buff,
    dataoff: c_uint,
    map: *mut c_void,
    dir: c_uint,
    chunk_type: c_uchar,
) -> c_int {
    if do_basic_checks(ct, skb, dataoff, map) == 0 {
        return 0;
    }
    let _ = new_state(ct, dir, chunk_type);
    1
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sctp_get_timeouts_array() -> *const c_uint {
    SCTP_TIMEOUTS.as_ptr()
}