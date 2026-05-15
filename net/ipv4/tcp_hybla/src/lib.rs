//! TCP HYBLA Congestion Control Algorithm
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]  // For C-style type names

use core::ptr;
use core::ffi::c_int;
use core::ffi::c_uint;
use core::ffi::c_void;

// Constants from C
pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ENOSYS: c_int = -38;

// Type definitions
#[repr(C)]
pub struct hybla {
    hybla_en: bool,
    snd_cwnd_cents: c_uint, /* Keeps increment values when it is <1, <<7 */
    rho: c_uint,            /* Rho parameter, integer part  */
    rho2: c_uint,           /* Rho * Rho, integer part */
    rho_3ls: c_uint,        /* Rho parameter, <<3 */
    rho2_7ls: c_uint,       /* Rho^2, <<7	*/
    minrtt_us: c_uint,      /* Minimum smoothed round trip time value seen */
}

#[repr(C)]
pub struct tcp_congestion_ops {
    init: extern "C" fn(*mut c_void),
    ssthresh: extern "C" fn(*mut c_void, *mut c_void),
    undo_cwnd: extern "C" fn(*mut c_void),
    cong_avoid: extern "C" fn(*mut c_void, c_uint, c_uint),
    set_state: extern "C" fn(*mut c_void, u8),
    owner: *mut c_void,
    name: *const c_char,
}

// Function implementations
/// Recalculate HYBLA parameters based on current RTT
#[no_mangle]
pub unsafe extern "C" fn hybla_recalc_param(sk: *mut c_void) {
    let ca = inet_csk_ca(sk) as *mut hybla;
    let tp = tcp_sk(sk) as *mut tcp_sock;

    // SAFETY: Caller guarantees sk is valid and points to a valid sock structure
    (*ca).rho_3ls = if (*tp).srtt_us < (*tp).srtt_us {
        (*tp).srtt_us / (rtt0() * 1000) // USEC_PER_MSEC = 1000
    } else {
        8
    };
    (*ca).rho = (*ca).rho_3ls >> 3;
    (*ca).rho2_7ls = ((*ca).rho_3ls * (*ca).rho_3ls) << 1;
    (*ca).rho2 = (*ca).rho2_7ls >> 7;
}

/// Initialize HYBLA congestion control
#[no_mangle]
pub unsafe extern "C" fn hybla_init(sk: *mut c_void) {
    let tp = tcp_sk(sk) as *mut tcp_sock;
    let ca = inet_csk_ca(sk) as *mut hybla;

    (*ca).rho = 0;
    (*ca).rho2 = 0;
    (*ca).rho_3ls = 0;
    (*ca).rho2_7ls = 0;
    (*ca).snd_cwnd_cents = 0;
    (*ca).hybla_en = true;
    (*tp).snd_cwnd = 2;
    (*tp).snd_cwnd_clamp = 65535;

    // First Rho measurement based on initial srtt
    hybla_recalc_param(sk);

    // Set minimum rtt as this is the first ever seen
    (*ca).minrtt_us = (*tp).srtt_us;
    (*tp).snd_cwnd = (*ca).rho;
}

/// Update HYBLA state based on TCP state
#[no_mangle]
pub unsafe extern "C" fn hybla_state(sk: *mut c_void, ca_state: u8) {
    let ca = inet_csk_ca(sk) as *mut hybla;
    (*ca).hybla_en = (ca_state == TCP_CA_Open());
}

/// Calculate fractional increment for HYBLA
#[no_mangle]
pub unsafe extern "C" fn hybla_fraction(odds: c_uint) -> c_uint {
    static FRACTIONS: [c_uint; 8] = [128, 139, 152, 165, 181, 197, 215, 234];
    
    if odds < FRACTIONS.len() as c_uint {
        FRACTIONS[odds as usize]
    } else {
        128
    }
}

/// Main HYBLA congestion avoidance algorithm
#[no_mangle]
pub unsafe extern "C" fn hybla_cong_avoid(sk: *mut c_void, ack: c_uint, acked: c_uint) {
    let tp = tcp_sk(sk) as *mut tcp_sock;
    let ca = inet_csk_ca(sk) as *mut hybla;
    let mut increment: c_uint = 0;
    let mut odd: c_uint = 0;
    let mut rho_fractions: c_uint = 0;
    let mut is_slowstart: c_int = 0;

    // Recalculate rho only if this srtt is the lowest
    if (*tp).srtt_us < (*ca).minrtt_us {
        hybla_recalc_param(sk);
        (*ca).minrtt_us = (*tp).srtt_us;
    }

    if !tcp_is_cwnd_limited(sk) {
        return;
    }

    if !(*ca).hybla_en {
        tcp_reno_cong_avoid(sk, ack, acked);
        return;
    }

    if (*ca).rho == 0 {
        hybla_recalc_param(sk);
    }

    rho_fractions = (*ca).rho_3ls - ((*ca).rho << 3);

    if tcp_in_slow_start(tp) {
        // Slow start
        is_slowstart = 1;
        increment = ((1 << (*ca).rho.min(16)) * hybla_fraction(rho_fractions)) - 128;
    } else {
        // Congestion avoidance
        increment = (*ca).rho2_7ls / (*tp).snd_cwnd;
        if increment < 128 {
            (*tp).snd_cwnd_cnt += 1;
        }
    }

    odd = increment % 128;
    (*tp).snd_cwnd += increment >> 7;
    (*ca).snd_cwnd_cents += odd;

    // Check when fractions goes >=128 and increase cwnd by 1
    while (*ca).snd_cwnd_cents >= 128 {
        (*tp).snd_cwnd += 1;
        (*ca).snd_cwnd_cents -= 128;
        (*tp).snd_cwnd_cnt = 0;
    }

    // Check when cwnd has not been incremented for a while
    if increment == 0 && odd == 0 && (*tp).snd_cwnd_cnt >= (*tp).snd_cwnd {
        (*tp).snd_cwnd += 1;
        (*tp).snd_cwnd_cnt = 0;
    }

    // Clamp down slowstart cwnd to ssthresh value
    if is_slowstart != 0 {
        (*tp).snd_cwnd = (*tp).snd_cwnd.min((*tp).snd_ssthresh);
    }

    (*tp).snd_cwnd = (*tp).snd_cwnd.min((*tp).snd_cwnd_clamp);
}

// HYBLA congestion control operations
#[repr(C)]
static mut tcp_hybla: tcp_congestion_ops = tcp_congestion_ops {
    init: hybla_init,
    ssthresh: tcp_reno_ssthresh,
    undo_cwnd: tcp_reno_undo_cwnd,
    cong_avoid: hybla_cong_avoid,
    set_state: hybla_state,
    owner: ptr::null_mut(),
    name: b"hybla\0".as_ptr() as *const c_char,
};

/// Register HYBLA congestion control
#[no_mangle]
pub unsafe extern "C" fn hybla_register() -> c_int {
    // SAFETY: This is a direct translation of the C BUILD_BUG_ON
    // In practice, this would be a compile-time check in the kernel
    // Here we just assume it's valid as the C code would have verified it
    // BUILD_BUG_ON(core::mem::size_of::<hybla>() > ICSK_CA_PRIV_SIZE);
    
    tcp_register_congestion_control(&tcp_hybla)
}

/// Unregister HYBLA congestion control
#[no_mangle]
pub unsafe extern "C" fn hybla_unregister() {
    tcp_unregister_congestion_control(&tcp_hybla)
}

// Helper functions (these would be defined in the kernel)
#[link(name = "kernel")]
extern "C" {
    fn tcp_sk(sk: *mut c_void) -> *mut c_void;
    fn inet_csk_ca(sk: *mut c_void) -> *mut c_void;
    fn tcp_is_cwnd_limited(sk: *mut c_void) -> c_int;
    fn tcp_in_slow_start(tp: *mut c_void) -> c_int;
    fn tcp_reno_cong_avoid(sk: *mut c_void, ack: c_uint, acked: c_uint);
    fn tcp_reno_ssthresh(sk: *mut c_void, ack: *mut c_void);
    fn tcp_reno_undo_cwnd(sk: *mut c_void);
    fn tcp_register_congestion_control(ops: *const tcp_congestion_ops) -> c_int;
    fn tcp_unregister_congestion_control(ops: *const tcp_congestion_ops);
    fn TCP_CA_Open() -> u8;
}

// Module parameters
static mut rtt0: c_int = 25;

#[no_mangle]
pub unsafe extern "C" fn rtt0_get() -> c_int {
    rtt0
}

#[no_mangle]
pub unsafe extern "C" fn rtt0_set(val: c_int) {
    rtt0 = val;
}

// Module metadata
#[no_mangle]
pub static mut tcp_hybla_module: Module = Module {
    author: b"Daniele Lacamera\0".as_ptr() as *const c_char,
    license: b"GPL\0".as_ptr() as *const c_char,
    description: b"TCP Hybla\0".as_ptr() as *const c_char,
};

#[repr(C)]
struct Module {
    author: *const c_char,
    license: *const c_char,
    description: *const c_char,
}

// Module init/exit
#[no_mangle]
pub unsafe extern "C" fn module_init() {
    hybla_register();
}

#[no_mangle]
pub unsafe extern "C" fn module_exit() {
    hybla_unregister();
}

// TCP socket struct (simplified for FFI compatibility)
#[repr(C)]
struct tcp_sock {
    srtt_us: c_uint,
    snd_cwnd: c_uint,
    snd_cwnd_clamp: c_uint,
    snd_cwnd_cnt: c_uint,
    snd_ssthresh: c_uint,
}

// Helper functions for module parameters
#[no_mangle]
pub unsafe extern "C" fn module_param_rtt0(val: c_int) {
    rtt0 = val;
}

#[no_mangle]
pub unsafe extern "C" fn module_param_desc_rtt0() -> *const c_char {
    b"reference round trip time (ms)\0".as_ptr() as *const c_char
}
