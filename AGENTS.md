# RunuX Project Guidelines & Multi-Agent Architecture

Welcome to **RunuX** — the first production-grade, formally verified, multi-architecture (x86_64 & RISC-V) Rust Linux Kernel reimplementation with 297 modules, zero compiler warnings, and integrated Edge AI / SymBrain v4 neuro-symbolic reasoning.

This document serves as the master instruction file for all Antigravity agents, subagents, and automated workflows interacting with this repository.

---

## 🏛️ Kernel Architecture & Invariants

All agents operating in this repository must unconditionally uphold the following system invariants:

### 1. Zero-Warning `#![no_std]` Mandate
* Every crate under [`crates/`](file:///home/xavkal/xdev/rust-linux-mini-kernel/crates) must compile cleanly with `#![no_std]`, `#![deny(clippy::all)]`, and `#![warn(clippy::pedantic)]`.
* Zero compiler warnings and zero errors are permitted.
* In production builds, `panic="abort"` is enforced.

### 2. Fallible Allocations Only (No Kernel Panics)
* Kernel memory exhaustion must **never** cause a panic.
* All allocations must return `Result<T, AllocError>` or propagate standard Linux errno values (e.g., `-ENOMEM`).
* Sockets, page frames, and hardware descriptors must be managed using the Newtype pattern and RAII `Drop` wrappers (e.g. [`SafeSkb`](file:///home/xavkal/xdev/rust-linux-mini-kernel/crates/kernel_types), [`SafeSock`](file:///home/xavkal/xdev/rust-linux-mini-kernel/crates/kernel_types), [`SafePageFrame`](file:///home/xavkal/xdev/rust-linux-mini-kernel/crates/kernel_types), [`SafeDmaQueue`](file:///home/xavkal/xdev/rust-linux-mini-kernel/crates/kernel_types)).

### 3. Bit-Exact C ABI Struct Compatibility
* All structures crossing the C FFI boundary must declare `#[repr(C)]`.
* C bitfields must be packed manually into explicit byte fields matching GCC's ABI layout (e.g. `version_ihl: u8`), exposing safe zero-cost accessors (`version()`, `ihl()`).
* For unaligned pointer operations and `static mut` references, use `core::ptr::addr_of_mut!`—never create `&mut` references to unaligned addresses.
* Every `unsafe` block must be accompanied by an explicit `// SAFETY:` rationale proving alignment, non-null guarantees, and validity boundaries.

### 4. Lean 4 Mathematical Proof Integrity
* All 12 formal verification phases in [`specs/lean4/`](file:///home/xavkal/xdev/rust-linux-mini-kernel/specs/lean4) must maintain **zero `sorry` tactics**.
* Lean 4 specifications must remain structurally congruent with Rust kernel contract invariants ([`requires!()`](file:///home/xavkal/xdev/rust-linux-mini-kernel/crates/core) / [`ensures!()`](file:///home/xavkal/xdev/rust-linux-mini-kernel/crates/core)).

### 5. SymBrain v4 AI Protection & Deductive Floor
* All AI query processing, routing, and edge inference must strictly adhere to the SymBrain v4 specification ([`docs/SYMBRAIN_V4.md`](file:///home/xavkal/xdev/rust-linux-mini-kernel/docs/SYMBRAIN_V4.md)).
* The Calibrated PFC Router must enforce the hard **Deductive Floor** ($\sigma_{ded} \ge 0.30$) to prevent cognitive lockup and the Routing-Stall anomaly.
* Distributed volunteer nodes must satisfy the multi-gate verifier ([`scripts/neuro_symbolic_federated_verifier.py`](file:///home/xavkal/xdev/rust-linux-mini-kernel/scripts/neuro_symbolic_federated_verifier.py)) before executing training or inference workloads.

### 6. RunuX Core Defenses Pre-Dispatch Interception
* All incoming user-space / VM system calls must pass through the Ring 0 pre-dispatch interception hook ([`crates/syscall_table`](file:///home/xavkal/xdev/rust-linux-mini-kernel/crates/syscall_table), [`crates/ebpf_firewall`](file:///home/xavkal/xdev/rust-linux-mini-kernel/crates/ebpf_firewall)).
* Suspicious polymorphic payloads and sliding-window anomalies are evaluated sub-15 µs by the bare-metal TinyML classifier ([`crates/ai_detector`](file:///home/xavkal/xdev/rust-linux-mini-kernel/crates/ai_detector)).
* Security violations and verdicts are immutably logged to the append-only Merkle tree ([`crates/immutable_logs`](file:///home/xavkal/xdev/rust-linux-mini-kernel/crates/immutable_logs)).
* Kernel isolation invariants are proven in Lean 4 ([`specs/lean4/MVK/RunuxDefenses.lean`](file:///home/xavkal/xdev/rust-linux-mini-kernel/specs/lean4/MVK/RunuxDefenses.lean)).

---

## 🤖 Specialized Agent Personas

When tackling tasks in this codebase, assume or consult the corresponding specialized agent persona:

| Agent Persona | Configuration File | Focus Domain | Primary Skill |
| :--- | :--- | :--- | :--- |
| **`runux-kernel-architect`** | [`.agents/agents/runux-kernel-architect.md`](file:///home/xavkal/xdev/rust-linux-mini-kernel/.agents/agents/runux-kernel-architect.md) | Systems programming, FFI layout, `#![no_std]`, multi-arch compilation (x86_64, RISC-V) | `rust-kernel-ffi-audit` |
| **`runux-security-auditor`** | [`.agents/agents/runux-security-auditor.md`](file:///home/xavkal/xdev/rust-linux-mini-kernel/.agents/agents/runux-security-auditor.md) | Memory safety, Miri UB analysis, KASAN, fuzzing, `// SAFETY:` audit, exploit mitigation | `kernel-security-hardening` |
| **`runux-formal-verifier`** | [`.agents/agents/runux-formal-verifier.md`](file:///home/xavkal/xdev/rust-linux-mini-kernel/.agents/agents/runux-formal-verifier.md) | Lean 4 mathematical proofs, 12 verification phases, zero-`sorry` completeness | `lean4-formal-proofs` |
| **`runux-ai-protector`** | [`.agents/agents/runux-ai-protector.md`](file:///home/xavkal/xdev/rust-linux-mini-kernel/.agents/agents/runux-ai-protector.md) | SymBrain v4 PFC routing, neuro-symbolic multi-gate verification, prompt defense | `ai-engine-protection` |
| **`runux-chaos-validator`** | [`.agents/agents/runux-chaos-validator.md`](file:///home/xavkal/xdev/rust-linux-mini-kernel/.agents/agents/runux-chaos-validator.md) | Headless QEMU boot, RISC-V emulation, GKE Chaos Mesh fault injection, bare metal | `qemu-gcp-chaos-verification` |
| **`runux-core-defenses-specialist`** | [`.agents/agents/runux-core-defenses-specialist.md`](file:///home/xavkal/xdev/rust-linux-mini-kernel/.agents/agents/runux-core-defenses-specialist.md) | Ring 0 pre-dispatch interception, LMS firewall, TinyML latency, Merkle audit, Lean 4 congruence | `runux-core-defenses` |

---

## 🧰 Workspace Skills Directory

The repository provides six specialized on-demand skills located under [`.agents/skills/`](file:///home/xavkal/xdev/rust-linux-mini-kernel/.agents/skills):

1. **[`rust-kernel-ffi-audit`](file:///home/xavkal/xdev/rust-linux-mini-kernel/.agents/skills/rust-kernel-ffi-audit/SKILL.md)**: Procedures for verifying C ABI layouts, bitfield packing, target-dir cache isolation, and 297-module compilation.
2. **[`kernel-security-hardening`](file:///home/xavkal/xdev/rust-linux-mini-kernel/.agents/skills/kernel-security-hardening/SKILL.md)**: Procedures for running Miri tests, KASAN sanitizers, `cargo-fuzz` libFuzzer harnesses, and `cargo audit`.
3. **[`lean4-formal-proofs`](file:///home/xavkal/xdev/rust-linux-mini-kernel/.agents/skills/lean4-formal-proofs/SKILL.md)**: Workflows for running `verify_specs.sh`, building Lake specifications, and resolving proof stubs.
4. **[`ai-engine-protection`](file:///home/xavkal/xdev/rust-linux-mini-kernel/.agents/skills/ai-engine-protection/SKILL.md)**: Workflows for SymBrain v4 PFC gating, neuro-symbolic federated verification, and TurboQuant memory bounds.
5. **[`qemu-gcp-chaos-verification`](file:///home/xavkal/xdev/rust-linux-mini-kernel/.agents/skills/qemu-gcp-chaos-verification/SKILL.md)**: Step-by-step guides for QEMU boot validation, Chaos Mesh experiments, and GCP bare-metal deployment.
6. **[`runux-core-defenses`](file:///home/xavkal/xdev/rust-linux-mini-kernel/.agents/skills/runux-core-defenses/SKILL.md)**: Procedures for auditing pre-dispatch interception, LMS policy transitions, TinyML inference, and requirement traceability.

---

## ⚡ Quick Validation Reference

```bash
# 1. Automated Core Defenses Quality Gate & Traceability Matrix
python3 scripts/workflow.py --all

# 2. Safe compilation check with isolated target directory
CARGO_TARGET_DIR="/tmp/runux_target" cargo check --workspace --quiet

# 3. Cross-compilation for RISC-V 64-bit target
CARGO_TARGET_DIR="/tmp/runux_target" cargo check --workspace --target riscv64gc-unknown-none-elf --quiet

# 4. Lean 4 formal specification verification
./specs/scripts/verify_specs.sh

# 5. Neuro-Symbolic AI & Hardware Verifier
python3 scripts/neuro_symbolic_federated_verifier.py

# 6. Headless QEMU Boot Test
python3 scripts/qemu_boot_test.py
```
