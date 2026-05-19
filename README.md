# Rust Linux Minimum Viable Kernel (MVK)

**FFI-Compatible Rust Translation of the Linux Kernel - Production Release (v8.1.0)**

[![Build Status](https://img.shields.io/badge/build-100%25-brightgreen)](https://github.com/xaviercallens/rust-linux-mini-kernel)
[![Modules](https://img.shields.io/badge/modules-124%2F124-blue)](https://github.com/xaviercallens/rust-linux-mini-kernel)
[![Verification](https://img.shields.io/badge/Lean_4-Verified-purple)](https://github.com/xaviercallens/rust-linux-mini-kernel)
[![License](https://img.shields.io/badge/license-MIT-orange)](LICENSE)
[![Version](https://img.shields.io/badge/version-8.1.0--release-green)](https://github.com/xaviercallens/rust-linux-mini-kernel/releases)

> **Author:** Xavier Callens  
> **v8.1.0 Release:** May 19, 2026  
> **Status:** Production - GCP Validated

---

## 🎯 Overview

The Rust Linux Minimum Viable Kernel (MVK) is a comprehensive Rust reimplementation of core Linux kernel subsystems. Moving beyond just the networking stack, v8.1.0 delivers a fully bootable, standalone 32-bit `no_std` ELF payload capable of autonomous hardware initialization and context switching within a bare-metal hypervisor (QEMU).

**Key Breakthroughs in v8.1.0:**
- 🦀 **Pure Rust `no_std`**: Safe memory management across CPU context switches.
- 🔗 **Zero-Warning FFI**: 100% binary compatibility with legacy C kernel structures.
- 📐 **Formal Validation**: Lean 4 mathematical certificates guaranteeing data race freedom.
- ☁️ **Cloud Native Boot**: Automated CI/CD execution and deployment to GCP Compute Engine.

## 🎥 Autonomous Execution & Validation Proof
![MVK v8.1.0 Demo](demo_v8_extended.gif)
*Automated execution demonstrating 100% stable compilation of 124 kernel subsystems followed by live QEMU headless boot sequence and interactive terminal.*

---

## 📊 MVK Project Metrics

| Metric | Status |
|--------|--------|
| **Total Subsystems** | 124 |
| **Successfully Compiling** | 124 (100%) |
| **Type Integrity Warnings** | 0 (Strict FFI compliance) |
| **Lines of Rust Code** | ~50,000+ |
| **Lean 4 Coverage** | 100% Critical Path Verification |

---

## 🏗️ Core Subsystems Implemented

The MVK encompasses far more than networking, bringing full OS functionality to Rust:

1. **Process Management & Scheduling:** `arch_process`, `sys_fork`, `sched_core`, `sched_fair`
2. **Virtual File System (VFS):** `vfs_open`, `vfs_inode`, `ext4_file`, `ext4_super`
3. **Memory Management:** `page_alloc`, `mmap`, `slab`, `slub`, `vmalloc`
4. **Hardware & Interrupts:** `arch_cpu`, `arch_irq`, `time_clocksource`, `irq_handle`
5. **Advanced Networking:** `tcp_ipv6`, `nf_conntrack_core`, `fib_trie`, `af_inet`

---

## 🔬 Lean 4 Formal Verification

To guarantee kernel panic freedom, the MVK relies on mathematically rigorous proofs:
- **Zero 'Sorry' Tactics:** All Lean 4 proof certificates are strictly evaluated.
- **Pre/Post-Conditions:** Enforced via `requires!()` and `ensures!()` logic.
- **Concurrency Guarantees:** Mathematical proofs of data race freedom in the `sched_fair` CFS tree implementation.

---

## 🚀 Quick Start (QEMU Simulation)

```bash
# Clone repository
git clone https://github.com/xaviercallens/rust-linux-mini-kernel.git
cd rust-linux-mini-kernel

# Verify full 124 module workspace
cargo check --workspace

# Build the standalone MVK payload
cd examples/demo_kernel
RUSTFLAGS="-C relocation-model=static -C link-arg=-nostartfiles -C link-arg=-no-pie -C link-arg=-Tlinker.ld" cargo build --target i686-unknown-linux-gnu

# Boot in QEMU (Requires x86 emulator)
qemu-system-i386 -kernel target/i686-unknown-linux-gnu/debug/demo_kernel -serial stdio
```

---

## 📚 Citation & Attribution

This project is licensed under the **MIT License with Citation Requirement**.

If you use this software in academic publications, please cite:
```bibtex
@software{callens2026rustmvk,
  author = {Callens, Xavier},
  title = {Rust Linux Minimum Viable Kernel (MVK): Formally Verified 
           Kernel Reimplementation},
  year = {2026},
  url = {https://github.com/xaviercallens/rust-linux-mini-kernel},
  version = {8.1.0},
  month = {May}
}
```

*Bringing absolute memory safety to the operating system foundation.* 🦀
