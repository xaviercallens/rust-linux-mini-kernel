# Rust Linux Mini Kernel Makefile

.PHONY: all build check test clean docs

all: check

# Check all modules compile
check:
	@echo "Checking all modules..."
	cargo check --workspace

# Build release
build:
	@echo "Building release..."
	cargo build --workspace --release

# Run tests
test:
	@echo "Running tests..."
	cargo test --workspace

# Generate documentation
docs:
	@echo "Generating documentation..."
	cargo doc --workspace --no-deps

# Clean build artifacts
clean:
	@echo "Cleaning..."
	cargo clean

# Check specific subsystem
check-core:
	cargo check --package 'net_core_*'

check-ipv4:
	cargo check --package 'net_ipv4_*'

# Static analysis
clippy:
	@echo "Running Clippy..."
	cargo clippy --workspace -- -D warnings

# Security audit
audit:
	@echo "Running security audit..."
	cargo audit

# Count lines of code
stats:
	@echo "Counting lines of code..."
	@find net -name "*.rs" | xargs wc -l | tail -1
