//! IPv4 XFRM State Management
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use core::ptr;
use libc::{c_int, AF_INET, IPPROTO_IPIP};

#[repr(C)]
struct xfrm_state_afinfo {
    family: c_int,
    proto: c_int,
    output: extern "C" fn() -> c_int,
    transport_finish: extern "C" fn() -> c_int,
    local_error: extern "C" fn() -> c_int,
}

// SAFETY: This static is initialized once at compile time with valid function pointers
// and will be passed to the kernel registration function as a const pointer
static XFRM4_STATE_AFINFO: xfrm_state_afinfo = xfrm_state_afinfo {
    family: AF_INET,
    proto: IPPROTO_IPIP,
    output: xfrm4_output,
    transport_finish: xfrm4_transport_finish,
    local_error: xfrm4_local_error,
};

// External functions defined elsewhere in the kernel
extern "C" {
    fn xfrm4_output() -> c_int;
    fn xfrm4_transport_finish() -> c_int;
    fn xfrm4_local_error() -> c_int;
    fn xfrm_state_register_afinfo(afinfo: *const xfrm_state_afinfo);
}

/// Initialize IPv4 XFRM state
///
/// # Safety
/// - Must be called during kernel initialization
/// - Function pointers in XFRM4_STATE_AFINFO must be valid
#[no_mangle]
pub unsafe extern "C" fn xfrm4_state_init() {
    // SAFETY: XFRM4_STATE_AFINFO is a static const and valid for the entire runtime
    xfrm_state_register_afinfo(&XFRM4_STATE_AFINFO);
}
### Key Implementation Notes:

1. **Struct Representation**:
   - `#[repr(C)]` ensures the struct layout matches C's memory layout
   - Function pointers use `extern "C"` calling convention

2. **Static Initialization**:
   - `XFRM4_STATE_AFINFO` is a compile-time constant
   - Function pointers are initialized directly with the corresponding C functions

3. **FFI Compatibility**:
   - All exported functions use `#[no_mangle]` and `extern "C"`
   - Structs and function signatures match the C implementation

4. **Safety Justifications**:
   - Static initialization is safe as it's only written once at compile time
   - Function pointer assignments are valid as they reference actual C functions
   - The registration function is called with a valid pointer to the static struct

5. **Kernel Integration**:
   - The implementation preserves the original C behavior exactly
   - Maintains the same initialization sequence and registration pattern

This implementation provides a direct FFI-compatible Rust translation that can be linked with the Linux kernel while maintaining all the original semantics and ABI compatibility.
