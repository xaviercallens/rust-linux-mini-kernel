//! TCP connection tracking module for Netfilter
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use core::ffi::c_int;
use core::ffi::c_uint;
use core::ffi::c_void;
use core::mem;
use core::ptr;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct TcpHdr {
    pub rst: u8,
    pub syn: u8,
    pub fin: u8,
    pub ack: u8,
    pub doff: u8,
}

// TCP state transition table indices
#[repr(u8)]
pub enum TcpBitSet {
    TCP_SYN_SET = 0,
    TCP_SYNACK_SET,
    TCP_FIN_SET,
    TCP_ACK_SET,
    TCP_RST_SET,
    TCP_NONE_SET,
}

// TCP connection states
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

// State transition table
static TCP_CONNTACKS: [[c_uint; 11]; 12] = {
    let mut table = [[0u32; 11]; 12];
    // Original direction transitions
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
    
    // Reply direction transitions
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

// Timeouts in jiffies
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
    timeouts[TcpConntrack::TCP_CONNTRACK_RETRANS as usize] = 5 * 60 * HZ;
    timeouts[TcpConntrack::TCP_CONNTRACK_UNACK as usize] = 5 * 60 * HZ;
    timeouts
};

// HZ is the number of jiffies per second
pub const HZ: c_uint = 100;

#[cfg(feature = "procfs")]
#[no_mangle]
pub unsafe extern "C" fn tcp_print_conntrack(s: *mut c_void, ct: *mut c_void) {
    if s.is_null() || ct.is_null() {
        return;
    }
    
    // SAFETY: Caller guarantees valid pointers
    let state = (*ct).cast::<TcpConntrack>().read();
    let name = tcp_conntrack_names[state as usize];
    
    // SAFETY: seq_printf is a valid kernel function
    extern "C" {
        fn seq_printf(s: *mut c_void, fmt: *const c_char, ...) -> c_int;
    }
    
    let fmt = "%s ".as_ptr() as *const c_char;
    seq_printf(s, fmt, name);
}

#[no_mangle]
pub unsafe extern "C" fn get_conntrack_index(tcph: *const TcpHdr) -> c_uint {
    if tcph.is_null() {
        return TcpBitSet::TCP_NONE_SET as c_uint;
    }
    
    // SAFETY: Caller guarantees valid pointer
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

// TCP state names
static TCP_CONNTRACK_NAMES: [*const c_char; 11] = {
    let names = [
        "NONE\0".as_ptr() as *const c_char,
        "SYN_SENT\0".as_ptr() as *const c_char,
        "SYN_RECV\0".as_ptr() as *const c_char,
        "ESTABLISHED\0".as_ptr() as *const c_char,
        "FIN_WAIT\0".as_ptr() as *const c_char,
        "CLOSE_WAIT\0".as_ptr() as *const c_char,
        "LAST_ACK\0".as_ptr() as *const c_char,
        "TIME_WAIT\0".as_ptr() as *const c_char,
        "CLOSE\0".as_ptr() as *const c_char,
        "SYN_SENT2\0".as_ptr() as *const c_char,
    ];
    names
};

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get_conntrack_index() {
        let mut tcph = TcpHdr {
            rst: 0,
            syn: 1,
            fin: 0,
            ack: 0,
            doff: 0,
        };
        assert_eq!(get_conntrack_index(&tcph), TcpBitSet::TCP_SYN_SET as c_uint);
        
        tcph.ack = 1;
        assert_eq!(get_conntrack_index(&tcph), TcpBitSet::TCP_SYNACK_SET as c_uint);
        
        tcph.syn = 0;
        tcph.ack = 0;
        tcph.fin = 1;
        assert_eq!(get_conntrack_index(&tcph), TcpBitSet::TCP_FIN_SET as c_uint);
        
        tcph.fin = 0;
        tcph.ack = 1;
        assert_eq!(get_conntrack_index(&tcph), TcpBitSet::TCP_ACK_SET as c_uint);
        
        tcph.ack = 0;
        tcph.rst = 1;
        assert_eq!(get_conntrack_index(&tcph), TcpBitSet::TCP_RST_SET as c_uint);
    }
}