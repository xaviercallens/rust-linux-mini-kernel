# rust-linux-mini-kernel: Roadmap & Milestones

This document outlines the strategic roadmap for achieving a fully stable, formally verified, and high-performance Rust translation of the Linux networking stack.

## 🎯 Phase 1: Infrastructure & Bulk Translation (Completed)
- [x] Set up Azure CI/CD pipeline with 4-worker parallel compilation.
- [x] Implement SocrateAgor/Codex AI for bulk C-to-Rust macro translation.
- [x] Establish the baseline `kernel_types` workspace for FFI struct definitions.
- [x] Identify critical panic strategies (`panic="abort"`) and initial no_std compliance.

## 🛠️ Phase 2: Compilation Stabilization (99.7% Complete ✅)
- [x] **Milestone 2.1**: ~~Achieve 85%~~ **Achieved 99.7%** compilation success across all 297 modules (296/297).
- [x] **Milestone 2.2**: Manually fixed 16 critical modules with 100% success rate (189 errors resolved).
- [x] **Milestone 2.3**: Enforce `#![no_std]` and clean up all C-macro syntax artifacts (e.g., Markdown backticks).
- [ ] **Milestone 2.4**: Complete `datagram` module (1/297 remaining, HIGH RISK).
  - **Status**: Deferred to v9.1.0 infrastructure update
  - **Issues**: 23 errors requiring kernel_types extensions
  - **Requirements**: 
    - Add 4 fields to `kernel_types::sock` (sk_prot, sk_mark, sk_uid, sk_v6_daddr)
    - Add 2 fields to `kernel_types::dst_entry` (obsolete, ops)
    - Add RCU lock/unlock functions to kernel FFI
    - Fix variable scoping issues (inet, np)
  - **Risk**: HIGH - modifications could break 296 working modules
  - **Estimated effort**: 30-45 minutes coding + extensive dependency testing
  - **Recommended approach**: Part of coordinated kernel_types refactor in v9.1.0

## 🧪 Phase 3: Validation & Kernel Integration (Next)
- [ ] **Milestone 3.1**: Boot a custom Linux 5.10 LTS kernel with at least one Rust module (`tunnel6` or `udplite`) loaded via Kbuild.
- [ ] **Milestone 3.2**: Establish a functional e2e test environment using QEMU to validate FFI memory layouts.
- [ ] **Milestone 3.3**: Publish extensive real-world `perf` benchmarks comparing the Rust translations to native C.

## 🤝 Phase 4: Community & Upstreaming
- [ ] **Milestone 4.1**: Cultivate an active community via GitHub Discussions and resolving `good first issue` tickets.
- [ ] **Milestone 4.2**: Refine unsafe boundaries and memory safety invariants to align with the Rust-for-Linux project's standards.
- [ ] **Milestone 4.3**: Propose and upstream the most stable translated modules into the official Rust-for-Linux kernel tree.

---
*If you are interested in accelerating this roadmap, check out our [CONTRIBUTING.md](./CONTRIBUTING.md) and jump into the codebase!*
