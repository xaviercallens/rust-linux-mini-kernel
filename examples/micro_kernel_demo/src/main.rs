/// Helper to safely initialize an IPv6 address from a 16-byte array
     #[inline]
     pub fn mk_in6_addr(bytes: [u8; 16]) -> kernel_types::in6_addr {
         kernel_types::in6_addr { s6_addr: bytes }
     }