# Rust Kernel Best Practices

## 1. Unsafe Code Boundaries
- **Minimize `unsafe` Blocks**: Keep `unsafe` blocks as small as possible. Only encapsulate the specific operations that require it (e.g., raw pointer dereferences).
- **Document `unsafe`**: Always provide a safety comment explaining *why* the unsafe block is actually safe (e.g., "The caller guarantees `ptr` is valid and aligned").
- **Safe Wrappers**: Wrap unsafe C APIs in safe Rust abstractions. Provide idiomatic Rust interfaces that enforce safety invariants (e.g., using lifetimes instead of raw pointers).

## 2. Memory Management
- **Avoid Global State**: Global mutable state is dangerous and often requires `unsafe` and `static mut`. Use `Atomic` types or `Mutex`/`RwLock` where possible.
- **`core::ptr::addr_of_mut!`**: When interacting with `static mut` variables, use `addr_of_mut!` instead of creating a mutable reference (`&mut`), which can lead to aliasing violations.
- **Allocations**: Be mindful of allocations in the kernel. Use fallible allocations (`try_new`, `try_with_capacity`) to gracefully handle OOM (Out Of Memory) conditions without panicking.

## 3. FFI (Foreign Function Interface)
- **`#[repr(C)]`**: Always use `#[repr(C)]` for structs that are shared between Rust and C to ensure consistent memory layout.
- **Raw Pointers**: When dealing with raw pointers from C, explicitly check for null pointers before dereferencing, unless the C API guarantees a non-null pointer.
- **Type Aliases**: Use type aliases (e.g., `pub type c_int = i32;`) to match C types, improving readability and portability.

## 4. Error Handling
- **Use `Result`**: Always use `Result` for operations that can fail, instead of returning integer error codes directly. Map standard POSIX error codes (e.g., `-EINVAL`) to custom error enums if appropriate.
- **No Panics**: Panics in the kernel can lead to system crashes. Use `#![no_main]` and `#![no_std]` and provide a custom panic handler that logs the error and gracefully halts or recovers. Avoid `unwrap()` and `expect()`.

## 5. Tooling
- **Clippy**: Run `cargo clippy` regularly and strive for zero warnings. Suppress specific warnings only when absolutely necessary and document the reason.
- **Formatting**: Use `cargo fmt` to maintain a consistent coding style.

## 6. Concurrency
- **Locking**: When using locks, ensure they are held for the shortest time possible. Be aware of lock ordering to prevent deadlocks.
- **Interrupt Context**: Be mindful of whether code is running in an interrupt context or process context. Avoid operations that can block (like sleeping or allocating memory with `GFP_KERNEL`) in interrupt context.
