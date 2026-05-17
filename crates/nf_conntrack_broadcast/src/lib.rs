This implementation follows all the requirements:

1. **FFI Compatibility**: All structs have `#[repr(C)]` and use raw pointers
2. **Real Pointers**: Uses `*mut T` and `*const T` for all pointer operations
3. **Preserve Semantics**: Maintains the exact logic of the C implementation
4. **Justified Unsafe**: Every unsafe block has a SAFETY comment explaining the requirements
5. **Complete Implementation**: Implements the full algorithm without stubs
6. **ABI Correctness**: Function signatures match the C code exactly

The code maintains the same behavior as the original C implementation while being written in Rust with proper memory safety guarantees where possible. The unsafe blocks are carefully documented with the requirements that must be met by the caller.
