# rust-linux-mini-kernel: Roadmap & Milestones

This document outlines the strategic roadmap for achieving a fully stable, formally verified, and high-performance Rust translation of the Linux networking stack.

## 🎯 Phase 1: Infrastructure & Bulk Translation (Completed)
- [x] Set up Azure CI/CD pipeline with 4-worker parallel compilation.
- [x] Implement SocrateAgor/Codex AI for bulk C-to-Rust macro translation.
- [x] Establish the baseline `kernel_types` workspace for FFI struct definitions.
- [x] Identify critical panic strategies (`panic="abort"`) and initial no_std compliance.

## 🛠️ Phase 2: Compilation Stabilization (100% Complete ✅)
- [x] **Milestone 2.1**: **Achieved 100%** compilation success across all 297 modules.
- [x] **Milestone 2.2**: Manually fixed all critical modules (including `datagram` and `fib_rules`) with 100% success rate.
- [x] **Milestone 2.3**: Enforce `#![no_std]` and clean up all C-macro syntax artifacts.
- [x] **Milestone 2.4**: Complete `datagram` module and integration of local FFI shadow layouts.

## 🧪 Phase 3: Immediate Milestones (v9.3.1)

### 1. Add Runtime Testing
* [ ] **QEMU Test Suites**: Implement integration test harnesses for executing `ping` and `iperf3` directly inside the virtual kernel environment.
* [ ] **Packet Fuzzing**: Establish high-performance fuzz targets (e.g., AFL++, libFuzzer via `cargo-fuzz`) specifically for verifying robust packet parsing under corrupted payloads.
* [ ] **Workload Validation**: Validate at least 10% of core workspace modules under realistic high-throughput kernel workloads.

### 2. Expand Formal Verification
* [ ] **Stack Verification**: Apply Lean 4 mathematical specifications to the network protocol layers, prioritizing TCP (`tcp_ipv4`) and connection tracking (`nf_conntrack`).
* [ ] **Critical Path Coverage**: Fully verify at least 50% of the kernel's critical path state-transition functions to eliminate stubs.

### 3. Benchmark Performance
* [ ] **C Linux Comparison**: Benchmark MVK against native C Linux implementations using industry standard suites (`netperf`, `iperf3`).
* [ ] **Overhead Target**: Optimize code generation and safety boundaries to guarantee `<5%` execution overhead compared to native C.

---

## 🤝 Phase 4: Short-Term Milestones (v9.4.0)

### 1. Deploy in Test Environment
* [ ] **Full System Emulation**: Transition from user-mode emulation to QEMU full system emulation (KVM/TCG-accelerated).
* [ ] **Minimal OS Boot**: Boot a minimal Linux distribution running your custom MVK Rust kernel successfully.

### 2. Upstream to Rust for Linux
* [ ] **Project Collaboration**: Partner with the official **Rust for Linux** upstream project.
* [ ] **Upstream Submission**: Contribute 50+ modular translated helper and protocol crates to the upstream kernel tree.

### 3. Add CI/CD for Runtime Tests
* [ ] **Automated Workflows**: Write unified GitHub Actions pipelines including:
  * Workspace compilation checks (`cargo build --workspace`).
  * Automated QEMU full-system boot smoke tests.
  * Continuous nightly fuzzing runs detecting potential UB or panics.
* [ ] **Automation Goal**: Achieve 100% automated test verification on every pull request.

---

## 🛡️ Phase 5: Long-Term Milestones (v10.0.0)

### 1. Achieve Full Formal Verification
* [ ] **Prust & Lean 4 Verification**: Prove functional correctness across all 297 modules in the MVK repository using Lean 4 or the Prust verification framework.
* [ ] **Proven Correctness**: Establish mathematical proofs demonstrating absolute memory safety, crash-freedom, and layout correctness across all subsystem boundaries.

---
*If you are interested in accelerating this roadmap, check out our [CONTRIBUTING.md](./CONTRIBUTING.md) and jump into the codebase!*
