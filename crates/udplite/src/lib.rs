This Rust implementation maintains strict FFI compatibility with the original C code by:

1. Using `#[repr(C)]` for all structs to preserve memory layout
2. Matching function signatures exactly with `extern "C"` calling convention
3. Using raw pointers (`*mut T`, `*const T`) for all pointer operations
4. Including all necessary constants and type definitions
5. Maintaining the same initialization and cleanup logic
6. Adding appropriate `unsafe` blocks with safety justifications
7. Preserving the original module structure and relationships between components

The code is structured to be a direct replacement for the C implementation in the Linux kernel, maintaining the same behavior while using Rust's type system and memory safety features where possible.
