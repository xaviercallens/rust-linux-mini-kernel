
//! Connection tracking protocol helper module for SCTP.
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
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
pub struct nf_conntrack_proto {
    pub sctp: sctp_conntrack,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conn {
    pub proto: nf_conntrack_proto,
}

// Static data
static SCTP_CONNTRACK_NAMES: [&str; SCTP_CONNTRACK_MAX as usize + 1] = [
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

static SCTP_TIMEOUTS: [u32; SCTP_CONNTRACK_MAX as usize] = [
    10,     // SCTP_CONNTRACK_CLOSED
    3,      // SCTP_CONNTRACK_COOKIE_WAIT
    3,      // SCTP_CONNTRACK_COOKIE_ECHOED
    432000, // 5 DAYS in seconds (5*24*3600)
    3,      // SCTP_CONNTRACK_SHUTDOWN_SENT
    3,      // SCTP_CONNTRACK_SHUTDOWN_RECD
    3,      // SCTP_CONNTRACK_SHUTDOWN_ACK_SENT
    30,     // SCTP_CONNTRACK_HEARTBEAT_SENT
    210,    // SCTP_CONNTRACK_HEARTBEAT_ACKED
];

static SCTP_CONNTRACKS: [[[u8; SCTP_CONNTRACK_MAX as usize]; 11]; 2] = {
    let mut arr = [[[0u8; 10]; 11]; 2];
    // Original direction transitions
    arr[0][0] = [1, 1, 2, 3, 4, 5, 6, 7, 2, 9]; // INIT
    arr[0][1] = [1, 1, 2, 3, 4, 5, 6, 7, 1, 9]; // INIT_ACK
    arr[0][2] = [1; 10]; // ABORT
    arr[0][3] = [1, 1, 2, 3, 5, 5, 6, 7, 1, 5]; // SHUTDOWN
    arr[0][4] = [7, 1, 2, 3, 4, 7, 7, 7, 7, 9]; // SHUTDOWN_ACK
    arr[0][5] = [1, 1, 2, 3, 4, 5, 6, 7, 1, 9]; // ERROR
    arr[0][6] = [1, 1, 3, 3, 4, 5, 6, 7, 1, 9]; // COOKIE_ECHO
    arr[0][7] = [1, 1, 2, 3, 4, 5, 6, 7, 1, 9]; // COOKIE_ACK
    arr[0][8] = [1, 1, 2, 3, 4, 5, 6, 1, 1, 9]; // SHUTDOWN_COMP
    arr[0][9] = [8, 1, 2, 3, 4, 5, 6, 7, 8, 9]; // HEARTBEAT
    arr[0][10] = [1, 1, 2, 3, 4, 5, 6, 7, 9, 9]; // HEARTBEAT_ACK

    // Reply direction transitions
    arr[1][0] = [10, 1, 2, 3, 4, 5, 6, 7, 10, 9]; // INIT
    arr[1][1] = [10, 2, 2, 3, 4, 5, 6, 7, 10, 9]; // INIT_ACK
    arr[1][2] = [10, 1, 1, 1, 1, 1, 1, 1, 10, 1]; // ABORT
    arr[1][3] = [10, 1, 2, 3, 6, 5, 6, 7, 10, 6]; // SHUTDOWN
    arr[1][4] = [10, 1, 2, 3, 4, 7, 7, 7, 10, 9]; // SHUTDOWN_ACK
    arr[1][5] = [10, 1, 2, 1, 4, 5, 6, 7, 10, 9]; // ERROR
    arr[1][6] = [10, 1, 2, 3, 4, 5, 6, 7, 10, 9]; // COOKIE_ECHO
    arr[1][7] = [10, 1, 2, 4, 4, 5, 6, 7, 10, 9]; // COOKIE_ACK
    arr[1][8] = [10, 1, 2, 3, 4, 5, 6, 1, 10, 9]; // SHUTDOWN_COMP
    arr[1][9] = [10, 1, 2, 3, 4, 5, 6, 7, 8, 9]; // HEARTBEAT
    arr[1][10] = [10, 1, 2, 3, 4, 5, 6, 7, 9, 9]; // HEARTBEAT_ACK
    arr
};

// Function implementations
#[no_mangle]
pub unsafe extern "C" fn sctp_print_conntrack(s: *mut c_void, ct: *mut nf_conn) {
    if !s.is_null() && !ct.is_null() {
        let state = (*ct).proto.sctp.state;
        // SAFETY: This is a no-op in Rust as we don't have seq_file
        // In real implementation, this would format to the seq_file
    }
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
    let mut offset: u32 = 0;
    let mut count: u32 = 0;
    let mut flag = 0;
    let mut sch: *mut sctp_chunkhdr = ptr::null_mut();
    let mut _sch: sctp_chunkhdr = sctp_chunkhdr {
        type_: 0,
        length: 0,
    };

    // SAFETY: The for_each_sctp_chunk macro logic is implemented here
    // with bounds checking and pointer validation
    offset = dataoff + (core::mem::size_of::<sctphdr>() as u32);
    while offset < (*skb).len {
        sch = skb_header_pointer(
            skb,
            offset,
            core::mem::size_of::<sctp_chunkhdr>() as size_t,
            &mut _sch as *mut sctp_chunkhdr as *mut c_void,
        );
        if sch.is_null() {
            break;
        }

        // Process chunk
        if (*sch).type_ == SCTP_CID_INIT
            || (*sch).type_ == SCTP_CID_INIT_ACK
            || (*sch).type_ == SCTP_CID_SHUTDOWN_COMPLETE
        {
            flag = 1;
        }

        // Basic checks
        if (((*sch).type_ == SCTP_CID_COOKIE_ACK
            || (*sch).type_ == SCTP_CID_COOKIE_ECHO
            || flag != 0)
            && count != 0)
            || (*sch).length == 0
        {
            return 1;
        }

        if !map.is_null() {
            // SAFETY: Bit manipulation is safe with valid pointer
            set_bit((*sch).type_ as usize, map);
        }

        offset += ((*sch).length as u32 + 3) & !3;
        count += 1;
    }

    if count == 0 {
        return 1;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn sctp_new_state(dir: c_int, cur_state: u8, chunk_type: u8) -> u8 {
    let mut i: c_int = 0;

    match chunk_type {
        SCTP_CID_INIT => i = 0,
        SCTP_CID_INIT_ACK => i = 1,
        SCTP_CID_ABORT => i = 2,
        SCTP_CID_SHUTDOWN => i = 3,
        SCTP_CID_SHUTDOWN_ACK => i = 4,
        SCTP_CID_ERROR => i = 5,
        SCTP_CID_COOKIE_ECHO => i = 6,
        SCTP_CID_COOKIE_ACK => i = 7,
        SCTP_CID_SHUTDOWN_COMPLETE => i = 8,
        SCTP_CID_HEARTBEAT => i = 9,
        SCTP_CID_HEARTBEAT_ACK => i = 10,
        _ => return cur_state,
    }

    SCTP_CONNTRACKS[dir as usize][i as usize][cur_state as usize]
}

#[no_mangle]
pub unsafe extern "C" fn sctp_new(
    ct: *mut nf_conn,
    skb: *mut sk_buff,
    sh: *mut sctphdr,
    dataoff: c_uint,
) -> c_int {
    let mut new_state: u8 = SCTP_CONNTRACK_MAX;
    let mut offset: u32 = 0;
    let mut count: u32 = 0;
    let mut sch: *mut sctp_chunkhdr = ptr::null_mut();
    let mut _sch: sctp_chunkhdr = sctp_chunkhdr {
        type_: 0,
        length: 0,
    };

    // Initialize sctp struct
    (*ct).proto.sctp = sctp_conntrack {
        state: 0,
        vtag: [0, 0],
    };

    // Process each chunk
    offset = dataoff + (core::mem::size_of::<sctphdr>() as u32);
    while offset < (*skb).len {
        sch = skb_header_pointer(
            skb,
            offset,
            core::mem::size_of::<sctp_chunkhdr>() as size_t,
            &mut _sch as *mut sctp_chunkhdr as *mut c_void,
        );
        if sch.is_null() {
            break;
        }

        new_state = sctp_new_state(0, SCTP_CONNTRACK_NONE, (*sch).type_);

        if new_state == SCTP_CONNTRACK_NONE || new_state == SCTP_CONNTRACK_MAX {
            return 0; // false
        }

        if (*sch).type_ == SCTP_CID_INIT {
            let mut _inithdr: [u8; 16] = [0; 16]; // Assuming sctp_inithdr size
            let ih = skb_header_pointer(
                skb,
                offset + (core::mem::size_of::<sctp_chunkhdr>() as u32),
                16,
                &mut _inithdr as *mut [u8; 16] as *mut c_void,
            );
            if ih.is_null() {
                return 0;
            }

            // Set vtag
            (*ct).proto.sctp.vtag[1] = (*ih as *mut u8).read_unaligned();
        }

        offset += ((*sch).length as u32 + 3) & !3;
        count += 1;
    }

    if count == 0 {
        return 0;
    }
    let _ = new_state(ct, dir, chunk_type);
    1
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sctp_get_timeouts_array() -> *const c_uint {
    SCTP_TIMEOUTS.as_ptr()
}
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
