---
phase: 77-corruption-and-lock-recovery-hardening
verified: 2026-06-16T02:01:43Z
status: passed
generated_by: gsd-executor
lifecycle_mode: yolo
phase_lifecycle_id: 77-2026-06-15T18-33-03
generated_at: 2026-06-16T02:01:43Z
lifecycle_validated: true
---

# Phase 77 Verification Report

**Phase Goal:** Harden corruption and lock recovery evidence so operators get
deterministic, read-only diagnosis for lock contention, stale lock artifacts,
corruption marker records, schema mismatch, partial write evidence, and
storage-open failure paths without default public-network or service-manager
execution.

## Evidence Captured

- Recovery classifier: typed `RecoveryEvidenceSnapshot` output covers lock
  contention, stale lock evidence, schema mismatch, partial write markers,
  resource pressure, backend failures, and next-action classes.
- Lock probe: `probe_fjall_lock` distinguishes missing datadirs, no lock
  artifact, stale lock evidence, and held-lock active contention without
  opening Fjall stores in probe-only status/support paths.
- Fjall recovery evidence: storage-open failure, corruption marker, schema
  mismatch, partial write, and lock-contention cases map into typed recovery
  evidence.
- Status/support/dashboard: `recovery_evidence` remains top-level, renderable,
  and probe-only; support store-health evidence does not treat probe-only
  restart metadata as a durable store read.
- Live smoke and soak evidence: recovery action class, cause, next action, and
  unavailable reasons are preserved in compact summaries and reports.
- Deterministic checker: `scripts/check-phase77-corruption-lock-recovery.ts`
  and its fixture-root test are wired into `scripts/verify.sh` immediately
  after the Phase 76 checker.

## Focused Commands Passed

- Recovery classifier: `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib recovery_classifier --all-features`
- Lock probe: `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib lock_probe_ --all-features`
- Fjall recovery evidence: `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib fjall_recovery_evidence_ --all-features`
- Status recovery evidence: `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --lib status_recovery_evidence_ --all-features`
- Support recovery evidence: `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --lib support_recovery_evidence_ --all-features`
- Dashboard recovery evidence: `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --lib dashboard_recovery_evidence_ --all-features`
- Soak recovery evidence: `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --lib soak_recovery_evidence_ --all-features`
- Checker tests: `bun test scripts/check-phase77-corruption-lock-recovery.test.ts`
- Checker: `bun run scripts/check-phase77-corruption-lock-recovery.ts`

## Full Verification Passed

- `bash scripts/verify.sh` passed on 2026-06-16 in 46m 46.019s. The run
  covered hook installation, LOC freshness, parity breadcrumbs, Phase 61
  through Phase 77 checkers, panic-site and file-length checks, Rust workspace
  lint/build/test coverage, benchmark smoke reports, coverage test pass, and
  the Bazel smoke build/run.
- `node /Users/peterryszkiewicz/.codex/get-shit-done/bin/gsd-tools.cjs verify lifecycle 77 --require-plans --require-verification --raw`
  is run after this report is written and recorded in the plan summary.

## Residual Risks

- Public-network soak remains outside default verification and requires
  explicit opt-in UAT.
- Real service-manager behavior remains outside deterministic default
  verification.
- Destructive repair remains outside Phase 77 scope; Phase 77 records
  diagnosis-only evidence and next actions, not automatic mutation.
