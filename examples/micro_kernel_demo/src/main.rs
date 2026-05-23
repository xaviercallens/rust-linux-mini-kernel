#![allow(clippy::all, clippy::pedantic)]
use kernel_types::*;

/// Helper to safely initialize an IPv6 address from a 16-byte array
#[inline]
pub fn mk_in6_addr(bytes: [u8; 16]) -> in6_addr {
    in6_addr {
        in6_u: in6_addr_union { u6_addr8: bytes },
    }
}

fn main() {
    // Minimal kernel demo entry point
    let _test_addr = mk_in6_addr([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]);
    println!("Micro kernel initialized with IPv6 address");
}
