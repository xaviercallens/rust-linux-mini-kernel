//! This module provides FFI-compatible Rust bindings for the Linux kernel's
//! UDP tunnel NIC operations stub implementation. The implementation mirrors
//! the original C code's ABI and maintains compatibility with the exported
//! symbol `udp_tunnel_nic_ops`.
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]  // For C-style type names

use core::ptr;

// Opaque struct definition matching the C struct from net/udp_tunnel.h
#[repr(C)]
struct UdpTunnelNicOps;

/// Global pointer to UDP tunnel NIC operations
///
/// This matches the C declaration:
/// `const struct udp_tunnel_nic_ops *udp_tunnel_nic_ops;`
///
/// The pointer is exported with the exact symbol name required by the Linux kernel.
///
/// SAFETY:
/// - This symbol is exported for use by the Linux kernel
/// - The pointer is initialized to NULL by default
/// - The pointer type matches the C declaration exactly
#[no_mangle]
static mut udp_tunnel_nic_ops: *const UdpTunnelNicOps = ptr::null();
### Verification Checklist

- [x] All exported functions have `#[no_mangle]` and `extern "C"`
- [x] All structs used in FFI have `#[repr(C)]`
- [x] No `unimplemented!()`, `todo!()`, or `panic!()` in hot paths
- [x] Every `unsafe` block has a SAFETY comment (none required here)
- [x] Function signatures match C exactly (no functions in this case)
- [x] Error codes match C errno values (no errors in this case)
- [x] Algorithm logic is complete and correct (no logic needed here)

This implementation correctly mirrors the original C code's ABI while maintaining FFI compatibility. The opaque struct declaration ensures proper memory layout, and the exported symbol name matches the original C implementation exactly.
