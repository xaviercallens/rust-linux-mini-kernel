//! DCCP connection tracking protocol helper
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)] // For C-style type names

use core::ptr;
use libc::{c_int, c_uint, c_void};

// Constants from C
pub const DCCP_MSL: c_int = 2 * 60; // HZ is ticks per second, but value is relative

// State constants
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

// Packet type constants
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

// Role constants
pub const CT_DCCP_ROLE_CLIENT: c_int = 0;
pub const CT_DCCP_ROLE_SERVER: c_int = 1;

// State names (as pointers to static strings)
#[repr(C)]
pub struct DccpStateNames {
    names: [*const c_char; CT_DCCP_INVALID as usize + 1],
}

// State transition table
#[repr(C)]
pub struct DccpStateTable {
    table: [[[c_int; CT_DCCP_INVALID as usize + 1]; DCCP_PKT_SYNCACK as usize + 1];
        CT_DCCP_ROLE_SERVER as usize + 1],
}

// Predefined state transition table (simplified for brevity)
// In practice, this would be fully initialized with all values from the C code
lazy_static::lazy_static! {
    static ref DCCP_STATE_TABLE: DccpStateTable = DccpStateTable {
        table: [
            // Client role
            [
                // DCCP_PKT_REQUEST
                [CT_DCCP_REQUEST, CT_DCCP_REQUEST, CT_DCCP_RESPOND, CT_DCCP_IGNORE, CT_DCCP_IGNORE, CT_DCCP_IGNORE, CT_DCCP_IGNORE, CT_DCCP_REQUEST],
                // DCCP_PKT_RESPONSE
                [CT_DCCP_INVALID, CT_DCCP_IGNORE, CT_DCCP_IGNORE, CT_DCCP_IGNORE, CT_DCCP_IGNORE, CT_DCCP_IGNORE, CT_DCCP_IGNORE, CT_DCCP_INVALID],
                // DCCP_PKT_ACK
                [CT_DCCP_INVALID, CT_DCCP_INVALID, CT_DCCP_PARTOPEN, CT_DCCP_PARTOPEN, CT_DCCP_OPEN, CT_DCCP_CLOSEREQ, CT_DCCP_CLOSING, CT_DCCP_INVALID],
                // DCCP_PKT_DATA
                [CT_DCCP_INVALID, CT_DCCP_INVALID, CT_DCCP_INVALID, CT_DCCP_INVALID, CT_DCCP_OPEN, CT_DCCP_CLOSEREQ, CT_DCCP_CLOSING, CT_DCCP_INVALID],
                // DCCP_PKT_DATAACK
                [CT_DCCP_INVALID, CT_DCCP_INVALID, CT_DCCP_PARTOPEN, CT_DCCP_PARTOPEN, CT_DCCP_OPEN, CT_DCCP_CLOSEREQ, CT_DCCP_CLOSING, CT_DCCP_INVALID],
                // DCCP_PKT_CLOSEREQ
                [CT_DCCP_INVALID, CT_DCCP_INVALID, CT_DCCP_INVALID, CT_DCCP_INVALID, CT_DCCP_INVALID, CT_DCCP_INVALID, CT_DCCP_INVALID, CT_DCCP_INVALID],
                // DCCP_PKT_CLOSE
                [CT_DCCP_INVALID, CT_DCCP_INVALID, CT_DCCP_INVALID, CT_DCCP_CLOSING, CT_DCCP_CLOSING, CT_DCCP_CLOSING, CT_DCCP_CLOSING, CT_DCCP_INVALID],
                // DCCP_PKT_RESET
                [CT_DCCP_INVALID, CT_DCCP_TIMEWAIT, CT_DCCP_TIMEWAIT, CT_DCCP_TIMEWAIT, CT_DCCP_TIMEWAIT, CT_DCCP_TIMEWAIT, CT_DCCP_TIMEWAIT, CT_DCCP_IGNORE],
                // DCCP_PKT_SYNC
                [CT_DCCP_INVALID, CT_DCCP_IGNORE, CT_DCCP_IGNORE, CT_DCCP_IGNORE, CT_DCCP_IGNORE, CT_DCCP_IGNORE, CT_DCCP_IGNORE, CT_DCCP_IGNORE],
                // DCCP_PKT_SYNCACK
                [CT_DCCP_INVALID, CT_DCCP_IGNORE, CT_DCCP_IGNORE, CT_DCCP_IGNORE, CT_DCCP_IGNORE, CT_DCCP_IGNORE, CT_DCCP_IGNORE, CT_DCCP_IGNORE],
            ],
            // Server role
            [
                // DCCP_PKT_REQUEST
                [CT_DCCP_INVALID, CT_DCCP_IGNORE, CT_DCCP_IGNORE, CT_DCCP_IGNORE, CT_DCCP_IGNORE, CT_DCCP_IGNORE, CT_DCCP_IGNORE, CT_DCCP_REQUEST],
                // DCCP_PKT_RESPONSE
                [CT_DCCP_INVALID, CT_DCCP_RESPOND, CT_DCCP_RESPOND, CT_DCCP_IGNORE, CT_DCCP_IGNORE, CT_DCCP_IGNORE, CT_DCCP_IGNORE, CT_DCCP_INVALID],
                // DCCP_PKT_ACK
                [CT_DCCP_INVALID, CT_DCCP_INVALID, CT_DCCP_INVALID, CT_DCCP_OPEN, CT_DCCP_OPEN, CT_DCCP_INVALID, CT_DCCP_CLOSING, CT_DCCP_INVALID],
                // DCCP_PKT_DATA
                [CT_DCCP_INVALID, CT_DCCP_INVALID, CT_DCCP_INVALID, CT_DCCP_OPEN, CT_DCCP_OPEN, CT_DCCP_INVALID, CT_DCCP_CLOSING, CT_DCCP_INVALID],
                // DCCP_PKT_DATAACK
                [CT_DCCP_INVALID, CT_DCCP_INVALID, CT_DCCP_INVALID, CT_DCCP_OPEN, CT_DCCP_OPEN, CT_DCCP_INVALID, CT_DCCP_CLOSING, CT_DCCP_INVALID],
                // DCCP_PKT_CLOSEREQ
                [CT_DCCP_INVALID, CT_DCCP_INVALID, CT_DCCP_INVALID, CT_DCCP_INVALID, CT_DCCP_INVALID, CT_DCCP_INVALID, CT_DCCP_INVALID, CT_DCCP_INVALID],
                // DCCP_PKT_CLOSE
                [CT_DCCP_INVALID, CT_DCCP_INVALID, CT_DCCP_INVALID, CT_DCCP_CLOSING, CT_DCCP_CLOSING, CT_DCCP_INVALID, CT_DCCP_CLOSING, CT_DCCP_INVALID],
                // DCCP_PKT_RESET
                [CT_DCCP_INVALID, CT_DCCP_TIMEWAIT, CT_DCCP_TIMEWAIT, CT_DCCP_TIMEWAIT, CT_DCCP_TIMEWAIT, CT_DCCP_TIMEWAIT, CT_DCCP_TIMEWAIT, CT_DCCP_IGNORE],
                // DCCP_PKT_SYNC
                [CT_DCCP_INVALID, CT_DCCP_IGNORE, CT_DCCP_IGNORE, CT_DCCP_IGNORE, CT_DCCP_IGNORE, CT_DCCP_IGNORE, CT_DCCP_IGNORE, CT_DCCP_IGNORE],
                // DCCP_PKT_SYNCACK
                [CT_DCCP_INVALID, CT_DCCP_IGNORE, CT_DCCP_IGNORE, CT_DCCP_IGNORE, CT_DCCP_IGNORE, CT_DCCP_IGNORE, CT_DCCP_IGNORE, CT_DCCP_IGNORE],
            ],
        ],
    };
}

/// Get next state based on current state, packet type, and role
///
/// # Safety
/// - `role` must be valid (CT_DCCP_ROLE_CLIENT or CT_DCCP_ROLE_SERVER)
/// - `pkt_type` must be valid (DCCP_PKT_* constants)
/// - `current_state` must be valid (CT_DCCP_* constants)
///
/// # Returns
/// Next state value or CT_DCCP_INVALID if transition is invalid
#[no_mangle]
pub unsafe extern "C" fn dccp_get_next_state(
    role: c_int,
    pkt_type: c_int,
    current_state: c_int,
) -> c_int {
    if role < 0 || role > CT_DCCP_ROLE_SERVER as c_int {
        return CT_DCCP_INVALID;
    }

    if pkt_type < 0 || pkt_type > DCCP_PKT_SYNCACK as c_int {
        return CT_DCCP_INVALID;
    }

    if current_state < 0 || current_state > CT_DCCP_INVALID as c_int {
        return CT_DCCP_INVALID;
    }

    // SAFETY: All bounds checked above
    *DCCP_STATE_TABLE.table[role as usize][pkt_type as usize][current_state as usize]
}

/// Initialize DCCP connection tracking
///
/// # Safety
/// - `ct` must be a valid pointer to nf_conn
/// - `skb` must be a valid pointer to sk_buff
/// - `dh` must be a valid pointer to dccp_hdr
///
/// # Returns
/// true (1) if connection tracking initialized, false (0) otherwise
#[no_mangle]
pub unsafe extern "C" fn dccp_new(ct: *mut c_void, skb: *const c_void, dh: *const c_void) -> c_int {
    if ct.is_null() || skb.is_null() || dh.is_null() {
        return 0;
    }

    // Example implementation - actual logic would be more complex
    // SAFETY: Caller guarantees pointers are valid
    let pkt_type = (*dh as *const u8).offset(0) as c_int; // Simplified packet type extraction

    // Get initial state based on packet type
    let initial_state = if pkt_type == DCCP_PKT_REQUEST {
        CT_DCCP_REQUEST
    } else {
        CT_DCCP_INVALID
    };

    // Set state in connection tracking struct
    // SAFETY: Caller guarantees ct is valid and properly aligned
    ptr::write(ct as *mut c_int, initial_state);

    1 // Success
}

// Error codes
pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ENOSYS: c_int = -38;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_transitions() {
        // Test client role, REQUEST packet, NONE state
        unsafe {
            let next_state =
                dccp_get_next_state(CT_DCCP_ROLE_CLIENT, DCCP_PKT_REQUEST, CT_DCCP_NONE);
            assert_eq!(next_state, CT_DCCP_REQUEST);
        }

        // Test server role, RESPONSE packet, REQUEST state
        unsafe {
            let next_state =
                dccp_get_next_state(CT_DCCP_ROLE_SERVER, DCCP_PKT_RESPONSE, CT_DCCP_REQUEST);
            assert_eq!(next_state, CT_DCCP_RESPOND);
        }

        // Test invalid state transition
        unsafe {
            let next_state =
                dccp_get_next_state(CT_DCCP_ROLE_CLIENT, DCCP_PKT_RESPONSE, CT_DCCP_NONE);
            assert_eq!(next_state, CT_DCCP_INVALID);
        }
    }
}
