# Changelog

All notable changes to the Rust Linux Minimum Viable Kernel (MVK) will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [11.2.0] - 2026-09-26

### Added
- **v12 truth-and-metrics gate** (`scripts/metrics.py`, `scripts/lean_tools.py`, `scripts/rust_tools.py`): static-analysis measurement of Lean proof debt and Rust code-quality metrics, with a `ratchet` mode that fails CI on regression against a frozen baseline (`docs/roadmap/metrics/metrics.baseline.json`). See `docs/roadmap/RUNUX_V12_VERIFIED_CORE_PLAN.md` and `docs/roadmap/WORKFLOW_INFRASTRUCTURE.md`.
- **Oracle-gated unit workflow** (`scripts/units/generate.py`, `scripts/check_unit.py`): generates per-theorem/per-issue work units and independently verifies a proposed fix — including a theorem-statement-hash check that rejects a proof whose underlying claim was silently weakened, even if the weakened version still type-checks.
- **Axiom register** (`specs/lean4/AXIOMS.md`) and **placeholder-crate triage** (`docs/roadmap/placeholder_triage.csv`), both generated from the actual tree rather than hand-maintained.
- `specs/scripts/verify_specs.sh` now auto-discovers all Lean modules (previously hardcoded to 20 of 36, silently undercounting proof debt) and hard-fails if a module already claimed complete regresses to containing `sorry`.

### Fixed
- **4 Lean theorems** now have real, independently re-verified proofs (previously `sorry`): `do_ipv6_setsockopt_contract` (`specs/lean4/MVK/Phase5/IPv6.lean`), and `init_idempotent`, `init_produces_valid_state`, `phase1_establishes_safety` (`specs/lean4/MVK/Phase1/ArchSetup.lean`). Total open proof obligations: 249 → 245.
- **2 theorems** (`arp_send_safety` in `ARP.lean`, `interrupts_disabled_after_init` in `ArchSetup.lean`) identified as unprovable-as-stated — genuine specification defects, not proof failures — and flagged for spec review rather than left silently unresolved.
- `verify_specs.sh`'s sorry-counter previously matched the word "sorry" inside `--` comments (e.g. a comment boasting "zero sorry" in `Phase13/GpuCompute.lean` was itself miscounted as one); now skips comment lines.
- `paper/runux_paper.tex`: corrected a blanket "zero `sorry` tactics" claim in the abstract and Formal Verification section that did not hold once measured across the full 36-module specification tree (432 theorems, 138 axioms, 245 open at time of writing outside the fully-closed 84-theorem `RunuxDefenses` Ring-0 model). Added a new subsection reporting the oracle-gated proof-completion pilot as a measured methodology contribution.

### Process notes
- A pilot run of the oracle-gated workflow surfaced a case where a fast-tier (Haiku) attempt silently introduced 6 forbidden axioms while exploring an approach it later abandoned, without disclosing this in its own structured self-report; only caught because the escalated attempt happened to inspect git history. Self-reported completion status is not sufficient on its own — independent, tool-based re-verification of the actual committed diff remains mandatory. See `docs/roadmap/RUNUX_V12_VERIFIED_CORE_PLAN.md` section 4 and the pilot writeup in `paper/runux_paper.tex` Section 4.3.
- CI checks unrelated to this change (Clippy, RISC-V cross-compilation, Build/Boot/Fuzz integration) were failing before this release on `main` itself (verified by reproducing a `printk` test failure directly against the unmodified `e65392f` baseline); they are not caused or worsened by this release.

---

## [9.3.1] - 2026-05-20

### Added
- Created dedicated `specs/lean4/MVK/Phase2/Compatibility.lean` spec module to isolate CI/CD formal verification FFI compatibility helpers (e.g., `_root_.IO.toIO'`), preventing runtime script tree mutations.
- Added comprehensive FFI shadow structures locally in the `datagram` subsystem, declaring correct structs for `sock`, `ipv6_pinfo`, `inet_sock`, `dst_entry`, and `dst_ops`.
- Introduced missing fields and layouts to local shadow structs to match standard C alignment (`sk_v6_rcv_saddr`, `sk_v6_daddr`, `sk_uid`, `sk_mark`, `sticky_pktinfo`, `sndflow`, `opt`, `dst_cookie`, `inet_dport`, `inet_rcv_saddr`, and `check` callback).
- Declared zero-overhead inline stubs for RCU lock/unlock operations (`rcu_read_lock`, `rcu_read_unlock`).

### Changed
- Refactored `crates/inet_connection_sock/src/lib.rs` to fix `sk_reuse` FFI pointer comparison errors by comparing integer states (`(*sk).sk_reuse != 0`) rather than checking for null pointers.
- Aligned `crates/fib_rules/src/lib.rs` signatures and templates with `kernel_types` definitions (casting `net.ipv4.rules_ops` cleanly and using `core::ptr::null_mut()` correctly).
- Bumped workspace packages and workspace package configuration to version `9.3.1`.
- Cleaned up duplicate/redundant local types (e.g., `net_ipv4`) across member crates, delegating directly to unified `kernel_types` definitions.
- Updated all verification and interactive simulation scripts (`simulate_demo_v9.py` and `verify_specs.sh`) to target release `v9.3.1`.

### Removed
- Cleaned up obsolete local C-to-Rust script compilation reports from the root workspace directory.

### Security & Correctness
- Achieved **100% compilation success rate (297/297 modules)** across the entire workspace with zero compilation errors and warnings.
- Fully validated 19 Lean 4 mathematical specifications files containing 440 verification obligations with zero type-checking errors.

---

## [9.3.0] - 2026-05-20

### Added
- First v9.3.0 release stabilizing core Netfilter NAT, DCCP, and SCTP conntrack protocol tracking.
- Formalized Lean 4 verification specs for the buddy page allocator and SLUB allocator.
