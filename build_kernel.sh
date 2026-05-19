#!/bin/bash
# Rust Linux Mini Kernel - Build Script for Phase 1

set -e

echo "=== Rust Linux Mini Kernel Build System ==="
echo "Building Phase 1: Boot to Panic"
echo ""

# Configuration
ARCH="x86_64"
TARGET="x86_64-unknown-none"  # Built-in Rust target
BUILD_DIR="build"
KERNEL_NAME="mvk-kernel"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Clean previous build
echo -e "${YELLOW}Cleaning previous build...${NC}"
rm -rf $BUILD_DIR
mkdir -p $BUILD_DIR

# Build critical modules in dependency order
echo -e "${YELLOW}Building kernel modules...${NC}"

MODULES=(
    "kernel_types"
    "printk"
    "arch_setup"
    "init_main"
)

for module in "${MODULES[@]}"; do
    echo -e "  Building ${GREEN}$module${NC}..."
    cargo build --release -p $module --target=$TARGET 2>&1 | grep -E "error|warning|Compiling|Finished" || true
done

# Collect object files
echo -e "${YELLOW}Collecting object files...${NC}"
mkdir -p $BUILD_DIR/objects

# Find all .rlib files from the critical modules
for module in "${MODULES[@]}"; do
    RLIB=$(find target/$TARGET/release/deps -name "lib${module}-*.rlib" | head -1)
    if [ -f "$RLIB" ]; then
        cp "$RLIB" "$BUILD_DIR/objects/lib${module}.rlib"
        echo "  Found: $module"
    else
        echo -e "  ${RED}Warning: Missing $module${NC}"
    fi
done

# Assemble entry point
echo -e "${YELLOW}Assembling entry point...${NC}"
nasm -f elf64 arch/x86_64/boot/entry.asm -o $BUILD_DIR/entry.o

# Extract objects from .rlib archives
echo -e "${YELLOW}Extracting Rust object files...${NC}"
cd $BUILD_DIR/objects
for rlib in *.rlib; do
    ar x $rlib
done
cd ../..

# Link kernel
echo -e "${YELLOW}Linking kernel...${NC}"
ld -n \
    -T arch/x86_64/linker.ld \
    -o $BUILD_DIR/$KERNEL_NAME.elf \
    $BUILD_DIR/entry.o \
    $BUILD_DIR/objects/*.o

# Check if kernel was created
if [ -f "$BUILD_DIR/$KERNEL_NAME.elf" ]; then
    echo -e "${GREEN}✓ Kernel linked successfully${NC}"
    ls -lh $BUILD_DIR/$KERNEL_NAME.elf

    # Show kernel size
    SIZE=$(du -h $BUILD_DIR/$KERNEL_NAME.elf | cut -f1)
    echo -e "  Kernel size: ${GREEN}$SIZE${NC}"
else
    echo -e "${RED}✗ Kernel linking failed${NC}"
    exit 1
fi

echo ""
echo -e "${GREEN}=== Build Complete ===${NC}"
echo ""
echo "Kernel binary: $BUILD_DIR/$KERNEL_NAME.elf"
echo ""
echo "To test in QEMU:"
echo "  qemu-system-x86_64 -kernel $BUILD_DIR/$KERNEL_NAME.elf -serial stdio"
echo ""
