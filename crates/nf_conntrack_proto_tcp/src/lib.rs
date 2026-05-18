#![no_std]
#![no_main]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use core::ffi::c_void;
use core::panic::PanicInfo;
use kernel_types::*;

pub type size_t = usize;
pub type c_size_t = usize;
pub type socklen_t = u32;

#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    loop {}
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TcpHdr {
    pub rst: u8,
    pub syn: u8,
    pub fin: u8,
    pub ack: u8,
    pub doff: u8,
}

#[repr(u8)]
pub enum TcpBitSet {
    TCP_SYN_SET = 0,
    TCP_SYNACK_SET,
    TCP_FIN_SET,
    TCP_ACK_SET,
    TCP_RST_SET,
    TCP_NONE_SET,
}

#[repr(u8)]
pub enum TcpConntrack {
    TCP_CONNTRACK_NONE = 0,
    TCP_CONNTRACK_SYN_SENT,
    TCP_CONNTRACK_SYN_RECV,
    TCP_CONNTRACK_ESTABLISHED,
    TCP_CONNTRACK_FIN_WAIT,
    TCP_CONNTRACK_CLOSE_WAIT,
    TCP_CONNTRACK_LAST_ACK,
    TCP_CONNTRACK_TIME_WAIT,
    TCP_CONNTRACK_CLOSE,
    TCP_CONNTRACK_SYN_SENT2,
    TCP_CONNTRACK_MAX,
    TCP_CONNTRACK_IGNORE,
}

pub const HZ: c_uint = 100;

static TCP_CONNTACKS: [[c_uint; 11]; 12] = {
    let mut table = [[0u32; 11]; 12];

    table[0][0] = TcpConntrack::TCP_CONNTRACK_SYN_SENT as c_uint;
    table[0][1] = TcpConntrack::TCP_CONNTRACK_SYN_SENT as c_uint;
    table[0][2] = TcpConntrack::TCP_CONNTRACK_IGNORE as c_uint;
    table[0][3] = TcpConntrack::TCP_CONNTRACK_IGNORE as c_uint;
    table[0][4] = TcpConntrack::TCP_CONNTRACK_IGNORE as c_uint;
    table[0][5] = TcpConntrack::TCP_CONNTRACK_IGNORE as c_uint;
    table[0][6] = TcpConntrack::TCP_CONNTRACK_IGNORE as c_uint;
    table[0][7] = TcpConntrack::TCP_CONNTRACK_SYN_SENT as c_uint;
    table[0][8] = TcpConntrack::TCP_CONNTRACK_SYN_SENT as c_uint;
    table[0][9] = TcpConntrack::TCP_CONNTRACK_SYN_SENT2 as c_uint;

    table[1][0] = TcpConntrack::TCP_CONNTRACK_IGNORE as c_uint;
    table[1][1] = TcpConntrack::TCP_CONNTRACK_SYN_SENT2 as c_uint;
    table[1][2] = TcpConntrack::TCP_CONNTRACK_IGNORE as c_uint;
    table[1][3] = TcpConntrack::TCP_CONNTRACK_IGNORE as c_uint;
    table[1][4] = TcpConntrack::TCP_CONNTRACK_IGNORE as c_uint;
    table[1][5] = TcpConntrack::TCP_CONNTRACK_IGNORE as c_uint;
    table[1][6] = TcpConntrack::TCP_CONNTRACK_IGNORE as c_uint;
    table[1][7] = TcpConntrack::TCP_CONNTRACK_SYN_SENT as c_uint;
    table[1][8] = TcpConntrack::TCP_CONNTRACK_IGNORE as c_uint;
    table[1][9] = TcpConntrack::TCP_CONNTRACK_SYN_SENT2 as c_uint;

    table
};

static TCP_TIMEOUTS: [c_uint; 12] = {
    let mut timeouts = [0u32; 12];
    timeouts[TcpConntrack::TCP_CONNTRACK_SYN_SENT as usize] = 2 * 60 * HZ;
    timeouts[TcpConntrack::TCP_CONNTRACK_SYN_RECV as usize] = 60 * HZ;
    timeouts[TcpConntrack::TCP_CONNTRACK_ESTABLISHED as usize] = 5 * 24 * 60 * HZ;
    timeouts[TcpConntrack::TCP_CONNTRACK_FIN_WAIT as usize] = 2 * 60 * HZ;
    timeouts[TcpConntrack::TCP_CONNTRACK_CLOSE_WAIT as usize] = 60 * HZ;
    timeouts[TcpConntrack::TCP_CONNTRACK_LAST_ACK as usize] = 30 * HZ;
    timeouts[TcpConntrack::TCP_CONNTRACK_TIME_WAIT as usize] = 2 * 60 * HZ;
    timeouts[TcpConntrack::TCP_CONNTRACK_CLOSE as usize] = 10 * HZ;
    timeouts[TcpConntrack::TCP_CONNTRACK_SYN_SENT2 as usize] = 2 * 60 * HZ;
    timeouts
};

#[cfg(feature = "procfs")]
#[no_mangle]
pub unsafe extern "C" fn tcp_print_conntrack(s: *mut c_void, ct: *mut c_void) {
    if s.is_null() || ct.is_null() {
        return;
    }

    let state = *(ct as *const u8) as usize;
    if state >= TCP_CONNTRACK_NAMES.len() {
        return;
    }
    let name = TCP_CONNTRACK_NAMES[state];

    unsafe extern "C" {
        fn seq_printf(s: *mut c_void, fmt: *const c_char, ...) -> c_int;
    }

    static FMT: &[u8] = b"%s \0";
    seq_printf(s, FMT.as_ptr() as *const c_char, name);
}

#[no_mangle]
pub unsafe extern "C" fn get_conntrack_index(tcph: *const TcpHdr) -> c_uint {
    if tcph.is_null() {
        return TcpBitSet::TCP_NONE_SET as c_uint;
    }

    let flags = *tcph;

    if flags.rst != 0 {
        TcpBitSet::TCP_RST_SET as c_uint
    } else if flags.syn != 0 {
        if flags.ack != 0 {
            TcpBitSet::TCP_SYNACK_SET as c_uint
        } else {
            TcpBitSet::TCP_SYN_SET as c_uint
        }
    } else if flags.fin != 0 {
        TcpBitSet::TCP_FIN_SET as c_uint
    } else if flags.ack != 0 {
        TcpBitSet::TCP_ACK_SET as c_uint
    } else {
        TcpBitSet::TCP_NONE_SET as c_uint
    }
}

static TCP_CONNTRACK_NAMES: [&[u8]; 11] = [
    b"NONE\0",
    b"SYN_SENT\0",
    b"SYN_RECV\0",
    b"ESTABLISHED\0",
    b"FIN_WAIT\0",
    b"CLOSE_WAIT\0",
    b"LAST_ACK\0",
    b"TIME_WAIT\0",
    b"CLOSE\0",
    b"SYN_SENT2\0",
    b"IGNORE\0",
];