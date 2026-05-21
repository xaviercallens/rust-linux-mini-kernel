# C vs Rust in Kernel Development

## Memory Safety and Undefined Behavior
One of the key differences between C and Rust in kernel development is how they handle memory safety. C places the burden of memory safety entirely on the developer, which historically leads to use-after-free, double-free, buffer overflows, and null pointer dereferences. Rust enforces memory safety at compile-time through its borrow checker, ownership rules, and lifetime annotations.

## Concurrency and Data Races
C has no built-in mechanism to prevent data races. Developers must manually use locks, mutexes, and atomics correctly. Rust’s type system guarantees thread safety at compile-time through the `Send` and `Sync` traits, ensuring that data races cannot occur in safe code.

## FFI and Legacy Code
Integrating Rust into a C codebase (like the Linux kernel) requires Foreign Function Interface (FFI). This creates a boundary between safe Rust and unsafe C. Data crossing this boundary must be carefully managed, often using raw pointers (`*mut T`, `*const T`). The `#[repr(C)]` attribute ensures memory layout compatibility between Rust and C structures.

## Error Handling
C relies on return codes and `errno`, which are easy to ignore or misinterpret. Rust uses the `Result` enum, which forces the developer to handle both success and error cases explicitly. The `?` operator provides ergonomic error propagation.

## Tooling and Ecosystem
C relies on external tools like `make`, `gcc`/`clang`, `valgrind`, and various static analyzers. Rust has a unified ecosystem centered around `cargo`, which handles dependency management, building, testing, and linting (via `clippy`).

## Performance
Rust's performance is comparable to C. Both languages compile down to machine code using LLVM (for `rustc` and `clang`) or GCC. Rust's zero-cost abstractions, like iterators and generics, provide high-level ergonomics without runtime overhead. However, boundary checks and panic handlers can introduce slight overhead if not optimized out.

## Summary
The migration from C to Rust in the kernel represents a significant shift towards compile-time safety and developer ergonomics, albeit with a steeper learning curve and the challenge of managing FFI boundaries.
