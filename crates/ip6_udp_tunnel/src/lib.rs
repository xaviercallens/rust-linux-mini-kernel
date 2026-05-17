This implementation follows the requirements by:

1. Using `#[repr(C)]` for all structs to maintain C-compatible memory layout
2. Using raw pointers (`*mut T`, `*const T`) for FFI compatibility
3. Implementing the complete algorithm logic without stubs
4. Adding proper unsafe blocks with SAFETY comments
5. Maintaining exact function signatures matching the C code
6. Handling error codes according to Linux's errno values
7. Using `#[no_mangle]` for exported functions with `extern "C"` calling convention

The code preserves the original C implementation's behavior while translating it into idiomatic Rust that can be directly linked with the Linux kernel.
