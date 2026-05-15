# Contributing to Rust Linux Mini Kernel

Thank you for your interest in contributing! This project represents an automated C-to-Rust translation of Linux kernel networking subsystems.

## About This Project

This codebase was **automatically generated** by the [Socrate Agora](https://github.com/xaviercallens/socrateagora) hybrid C-to-Rust pipeline. The code is a direct translation of Linux kernel C code into Rust while maintaining FFI compatibility.

## How to Contribute

### Reporting Issues

If you find issues with the generated code, please report them with:

1. **Module name** - Which module has the issue (e.g., `net_core_skbuff`)
2. **Issue type** - Compilation error, incorrect translation, safety issue, etc.
3. **Original C code** - Reference to the Linux kernel source file
4. **Expected behavior** - What should happen
5. **Actual behavior** - What's happening now

### Code Improvements

Contributions are welcome for:

- **Fixing compilation errors** - Making modules compile successfully
- **Improving safety** - Better unsafe block documentation, removing unnecessary unsafe
- **Adding tests** - Unit tests, integration tests, FFI validation tests
- **Documentation** - Improving module documentation, adding examples
- **Performance** - Optimizations while maintaining correctness

### Guidelines

1. **Maintain FFI compatibility** - All structs must keep `#[repr(C)]`
2. **Preserve semantics** - Changes should maintain the same behavior as the original C code
3. **Document safety** - All unsafe blocks need SAFETY comments
4. **Test changes** - Ensure changes don't break existing functionality
5. **Follow Rust conventions** - Use `cargo fmt` and `cargo clippy`

### Development Setup

```bash
# Clone the repository
git clone https://github.com/xaviercallens/rust-linux-mini-kernel.git
cd rust-linux-mini-kernel

# Check compilation
make check

# Run tests
make test

# Run Clippy
make clippy

# Format code
cargo fmt --all
```

### Pull Request Process

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/improve-skbuff`)
3. Make your changes
4. Run tests and checks (`make check && make test && make clippy`)
5. Commit with a descriptive message
6. Push to your fork
7. Open a Pull Request with:
   - Clear description of changes
   - Why the change is needed
   - How you tested it
   - Any breaking changes

### Code Review

All PRs will be reviewed for:

- **Correctness** - Does it maintain original C semantics?
- **Safety** - Are unsafe blocks properly justified?
- **Testing** - Are there appropriate tests?
- **Documentation** - Is it well documented?
- **Style** - Does it follow Rust conventions?

## Questions?

- Open a GitHub issue for questions about the project
- For questions about the Socrate Agora pipeline, see [the main repo](https://github.com/xaviercallens/socrateagora)
- For questions about Rust for Linux, see [rust-for-linux.com](https://rust-for-linux.com)

## License

All contributions must be compatible with GPL-2.0 to match the Linux kernel licensing.

## Acknowledgments

This project builds upon the incredible work of:

- The Linux kernel community
- The Rust for Linux project
- The Rust language team
- The Socrate Agora pipeline contributors
