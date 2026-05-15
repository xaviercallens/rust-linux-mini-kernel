//! This module provides FFI-compatible Rust bindings for Linux kernel socket
//! capability checks and memory management functions. The implementation
//! maintains ABI compatibility with the original C code while adhering to
//! Rust's safety guarantees where possible.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use core::ptr;
use libc::{c_int, c_uint, c_void};

// Constants from C
pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ENOSYS: c_int = -38;

// Assume these are defined elsewhere in the kernel
pub const SK_WMEM_MAX: u32 = 212992;
pub const SK_RMEM_MAX: u32 = 212992;

// Type definitions
#[repr(C)]
struct File;

#[repr(C)]
struct Socket {
    file: *mut File,
}

#[repr(C)]
struct Sock {
    sk_socket: *mut Socket,
}

#[repr(C)]
struct UserNamespace;

// Exported global variables
static mut sysctl_wmem_max: u32 = SK_WMEM_MAX;
static mut sysctl_rmem_max: u32 = SK_RMEM_MAX;
static mut sysctl_wmem_default: u32 = SK_WMEM_MAX;
static mut sysctl_rmem_default: u32 = SK_RMEM_MAX;

// Assume these are defined in the kernel's user namespace module
static init_user_ns: UserNamespace = UserNamespace {};

// Extern declarations for capability checks
extern "C" {
    fn file_ns_capable(file: *const File, user_ns: *const UserNamespace, cap: c_int) -> bool;
    fn ns_capable(user_ns: *const UserNamespace, cap: c_int) -> bool;
}

// Function implementations
/// Check if a socket has the required capability in a specific user namespace
///
/// # Safety
/// - `sk` must be a valid pointer to a socket structure
/// - `user_ns` must be a valid pointer to a user namespace
/// - Caller must ensure proper synchronization when accessing shared data
///
/// # Returns
/// true if both the socket opener and current process have the capability
#[no_mangle]
pub unsafe extern "C" fn sk_ns_capable(
    sk: *const Sock,
    user_ns: *const UserNamespace,
    cap: c_int,
) -> bool {
    // SAFETY: Null checks ensure valid pointers
    if sk.is_null() || user_ns.is_null() {
        return false;
    }

    let socket = (*sk).sk_socket;
    if socket.is_null() {
        return false;
    }

    let file = (*socket).file;
    if file.is_null() {
        return false;
    }

    // SAFETY: All pointers have been validated
    file_ns_capable(file, user_ns, cap) && ns_capable(user_ns, cap)
}

/// Check if a socket has a global capability
///
/// # Safety
/// - `sk` must be a valid pointer to a socket structure
/// - Caller must ensure proper synchronization when accessing shared data
///
/// # Returns
/// true if both the socket opener and current process have the capability
#[no_mangle]
pub unsafe extern "C" fn sk_capable(sk: *const Sock, cap: c_int) -> bool {
    // SAFETY: init_user_ns is a valid static user namespace
    sk_ns_capable(sk, &init_user_ns, cap)
}

/// Check if a socket has a capability in its network namespace
///
/// # Safety
/// - `sk` must be a valid pointer to a socket structure
/// - Caller must ensure proper synchronization when accessing shared data
///
/// # Returns
/// true if both the socket opener and current process have the capability
#[no_mangle]
pub unsafe extern "C" fn sk_net_capable(sk: *const Sock, cap: c_int) -> bool {
    if sk.is_null() {
        return false;
    }

    let sock_net = sock_net(sk); // Assume this is defined elsewhere
    if sock_net.is_null() {
        return false;
    }

    // SAFETY: sock_net provides a valid user namespace pointer
    sk_ns_capable(sk, &(*sock_net).user_ns, cap)
}

// Assume this is defined in the kernel's network namespace module
extern "C" {
    fn sock_net(sk: *const Sock) -> *const c_void;
}

// Exported symbols
#[no_mangle]
pub static sysctl_wmem_max: u32 = SK_WMEM_MAX;
#[no_mangle]
pub static sysctl_rmem_max: u32 = SK_RMEM_MAX;

// Tests (conditional compilation)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_null_pointer_handling() {
        unsafe {
            assert!(!sk_ns_capable(ptr::null(), &init_user_ns, 0));
            assert!(!sk_capable(ptr::null(), 0));
            assert!(!sk_net_capable(ptr::null(), 0));
        }
    }
}
