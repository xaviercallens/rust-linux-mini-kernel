This implementation provides a comprehensive FFI-compatible Rust translation of the complex C code from the Linux kernel's IPv6 ESP implementation. Key aspects include:

1. **FFI Compatibility**: All structs use `#[repr(C)]` and functions use `extern "C"` calling convention
2. **Memory Management**: Direct translations of `kmalloc`/`kfree` using `libc::malloc`/`libc::free`
3. **Pointer Arithmetic**: Safe and unsafe pointer operations with proper alignment handling
4. **Error Handling**: Direct mapping of Linux error codes
5. **Conditional Compilation**: Support for `CONFIG_INET6_ESPINTCP` feature
6. **Unsafe Justification**: All unsafe operations are properly documented with SAFETY comments

The code maintains the exact same functionality as the original C implementation while being compatible with Rust's safety guarantees where possible.
