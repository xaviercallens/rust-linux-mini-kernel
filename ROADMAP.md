# rust-linux-mini-kernel: Roadmap & Milestones

This document outlines the strategic roadmap for achieving a fully stable, formally verified, and high-performance Rust translation of the Linux networking stack.

## 🎯 Phase 1: Infrastructure & Bulk Translation (Completed)
- [x] Set up Azure CI/CD pipeline with 4-worker parallel compilation.
- [x] Implement SocrateAgor/Codex AI for bulk C-to-Rust macro translation.
- [x] Establish the baseline `kernel_types` workspace for FFI struct definitions.
- [x] Identify critical panic strategies (`panic="abort"`) and initial no_std compliance.

## 🛠️ Phase 2: Compilation Stabilization (Current)
- [ ] **Milestone 2.1**: Achieve 85% compilation success across all 121 modules.
- [ ] **Milestone 2.2**: Manually fix all Tier 1 critical modules (`netfilter`, `af_inet`, `udp`, `fib_trie`).
- [ ] **Milestone 2.3**: Enforce `#![no_std]` and clean up all C-macro syntax artifacts (e.g., Markdown backticks).

## 🧪 Phase 3: Validation & Kernel Integration (Next)
- [ ] **Milestone 3.1**: Boot a custom Linux 5.10 LTS kernel with at least one Rust module (`tunnel6` or `udplite`) loaded via Kbuild.
- [ ] **Milestone 3.2**: Establish a functional e2e test environment using QEMU to validate FFI memory layouts.
- [ ] **Milestone 3.3**: Publish extensive real-world `perf` benchmarks comparing the Rust translations to native C.

## 🤝 Phase 4: Community & Upstreaming
- [ ] **Milestone 4.1**: Cultivate an active community via GitHub Discussions and resolving `good first issue` tickets.
- [ ] **Milestone 4.2**: Refine unsafe boundaries and memory safety invariants to align with the Rust-for-Linux project's standards.
- [ ] **Milestone 4.3**: Propose and upstream the most stable translated modules into the official Rust-for-Linux kernel tree.

---

## 🛡️ Phase 5: MVK v9.0 Security Hardening (Next Objective)
- [x] **Milestone 5.1**: Integrate `cargo-audit` and `Miri` (UB interpreter) into GitHub Actions for continuous vulnerability detection.
- [ ] **Milestone 5.2**: Establish hardware-boundary fuzzing on all `unsafe` blocks using `cargo-fuzz`.
- [ ] **Milestone 5.3**: Implement Kernel Address Space Layout Randomization (KASLR) and kCFI exploit mitigations.
- [ ] **Milestone 5.4**: Privilege De-escalation (Microkernel Design) - Move drivers into Ring-3 userspace to protect core scheduler.

---
*If you are interested in accelerating this roadmap, check out our [CONTRIBUTING.md](./CONTRIBUTING.md) and jump into the codebase!*
