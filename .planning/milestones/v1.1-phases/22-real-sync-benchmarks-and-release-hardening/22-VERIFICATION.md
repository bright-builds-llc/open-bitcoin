---
phase: 22-real-sync-benchmarks-and-release-hardening
verified: 2026-04-28T01:57:12Z
status: passed
score: 5/5 truths verified
generated_by: gsd-execute-phase
lifecycle_mode: yolo
phase_lifecycle_id: 22-2026-04-28T01-24-15
generated_at: 2026-04-28T01:57:12Z
lifecycle_validated: true
overrides_applied: 0
---

# Phase 22: Real-Sync Benchmarks and Release Hardening Verification Report

**Phase Goal:** Close v1.1 with reproducible runtime evidence, operator-facing
guidance, and parity-ledger proof that the shipped runtime surface is auditable
without overclaiming deferred work.
**Requirements:** SYNC-05, MIG-05, VER-05, VER-06, VER-07, VER-08
**Verified:** 2026-04-28T01:57:12Z
**Status:** PASS

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
|---|-------|--------|----------|
| 1 | The repo-native verification contract now proves the current operator-runtime surface without requiring public-network access by default. | VERIFIED | [`scripts/verify.sh`](/Users/peterryszkiewicz/Repos/open-bitcoin/scripts/verify.sh) runs LOC freshness, parity breadcrumbs, pure-core checks, file-length checks, panic-site validation, workspace format/clippy/build/test, smoke benchmarks, benchmark-report validation, and Bazel smoke builds from one repo-owned entrypoint. |
| 2 | `open-bitcoin-bench` now emits deterministic runtime-backed benchmark evidence for sync, storage, operator-runtime, and wallet-rescan behavior with explicit profile and measurement metadata. | VERIFIED | [`packages/open-bitcoin-bench/src/registry.rs`](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-bench/src/registry.rs), [`packages/open-bitcoin-bench/src/report.rs`](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-bench/src/report.rs), [`packages/open-bitcoin-bench/src/runtime_fixtures.rs`](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-bench/src/runtime_fixtures.rs), and the new case files under [`packages/open-bitcoin-bench/src/cases/`](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-bench/src/cases/) define and execute the required Phase 22 scenarios. |
| 3 | Operators now have a practical v1.1 guide, and supporting docs describe shipped service, status, dashboard, config, migration, and benchmark flows as current behavior rather than deferred placeholders. | VERIFIED | [`docs/operator/runtime-guide.md`](/Users/peterryszkiewicz/Repos/open-bitcoin/docs/operator/runtime-guide.md), [`README.md`](/Users/peterryszkiewicz/Repos/open-bitcoin/README.md), [`docs/architecture/cli-command-architecture.md`](/Users/peterryszkiewicz/Repos/open-bitcoin/docs/architecture/cli-command-architecture.md), and [`docs/architecture/config-precedence.md`](/Users/peterryszkiewicz/Repos/open-bitcoin/docs/architecture/config-precedence.md) now present the current runtime workflow and limitations. |
| 4 | The parity ledger now models real-sync benchmarks and operator-runtime release hardening as explicit audit surfaces with evidence and non-claims. | VERIFIED | [`docs/parity/index.json`](/Users/peterryszkiewicz/Repos/open-bitcoin/docs/parity/index.json), [`docs/parity/checklist.md`](/Users/peterryszkiewicz/Repos/open-bitcoin/docs/parity/checklist.md), and [`docs/parity/catalog/operator-runtime-release-hardening.md`](/Users/peterryszkiewicz/Repos/open-bitcoin/docs/parity/catalog/operator-runtime-release-hardening.md) separate shipped evidence from deferred packaging, Windows service, hosted dashboard, and migration-apply work. |
| 5 | Release-readiness review now points at current Phase 22 evidence instead of older Phase 10 handoff language, and the smoke benchmark artifact is structurally validated before the repo can claim a clean verify pass. | VERIFIED | [`docs/parity/release-readiness.md`](/Users/peterryszkiewicz/Repos/open-bitcoin/docs/parity/release-readiness.md), [`docs/parity/benchmarks.md`](/Users/peterryszkiewicz/Repos/open-bitcoin/docs/parity/benchmarks.md), and [`scripts/check-benchmark-report.ts`](/Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-benchmark-report.ts) keep the closeout evidence current and auditable. |

**Score:** 5/5 truths verified

## Requirements Coverage

| Requirement | Status | Evidence |
|---|---|---|
| SYNC-05 | SATISFIED | The benchmark harness now measures deterministic headers sync, block connect, storage recovery, status or dashboard projection, and wallet rescan scenarios with local runtime fixtures. |
| MIG-05 | SATISFIED | The operator guide and parity ledger continue to document migration as dry-run only and keep current non-claims visible in release-facing docs. |
| VER-05 | SATISFIED | `bash scripts/verify.sh` covers the current operator-runtime verification surface from one repo-owned command without requiring public-network access. |
| VER-06 | SATISFIED | Smoke benchmark output is generated locally and structurally validated through the new report checker before verify can pass. |
| VER-07 | SATISFIED | Operator-facing docs now explain install, onboarding, service lifecycle, status, dashboard, config layering, migration planning, benchmark workflow, and current limitations. |
| VER-08 | SATISFIED | Machine-readable and human-readable parity sources now separate shipped v1.1 claims from deferred or out-of-scope surfaces. |

## Verification Evidence

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-bench -- --nocapture` passed.
- `bash scripts/run-benchmarks.sh --smoke --output-dir packages/target/benchmark-reports` passed.
- `bun scripts/check-benchmark-report.ts --report=packages/target/benchmark-reports/open-bitcoin-bench-smoke.json` passed.
- `bash scripts/verify.sh` passed end-to-end, including:
  - deterministic LOC freshness
  - parity breadcrumb validation
  - pure-core dependency/import checks
  - production Rust file-length validation
  - panic-site validation
  - Rust workspace format, clippy, build, and test coverage
  - smoke benchmark generation and benchmark-report validation
  - Bazel smoke build

## Human Verification Required

None. Phase 22 ships repo-owned docs and verification evidence rather than a
manual-only operational step.

## Residual Risks

- Packaged or signed release installation remains outside the current source-built claim.
- Windows service support, hosted dashboards, and migration apply mode remain deferred.
- Benchmark timing thresholds remain intentionally disabled, so the reports are evidence for review rather than automated release pass or fail gates.

---

_Verified: 2026-04-28T01:57:12Z_
_Verifier: GPT-5.4 (GSD yolo wrapper)_
