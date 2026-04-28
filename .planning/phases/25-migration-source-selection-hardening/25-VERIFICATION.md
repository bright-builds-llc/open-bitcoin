---
phase: 25-migration-source-selection-hardening
verified: 2026-04-28T20:46:00.000Z
status: passed
score: 3/3 truths verified
generated_by: gsd-execute-phase
lifecycle_mode: yolo
phase_lifecycle_id: 25-2026-04-28T20-22-00
generated_at: 2026-04-28T20:46:00.000Z
lifecycle_validated: true
overrides_applied: 0
---

# Phase 25: Migration Source Selection Hardening Verification Report

**Phase Goal:** Let explicit migration source paths participate directly in
source selection so custom-location Core or Knots installs can produce concrete
dry-run plans.
**Requirements:** MIG-02, MIG-04
**Verified:** 2026-04-28T20:46:00.000Z
**Status:** PASS

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
|---|-------|--------|----------|
| 1 | `open-bitcoin migrate plan --source-datadir <custom-path>` can now select a valid source install outside the default detection roots when the path is explicit and supported. | VERIFIED | `packages/open-bitcoin-cli/src/operator/runtime.rs` now augments migration-time detection roots with the explicit source datadir, and `packages/open-bitcoin-cli/tests/operator_binary.rs` covers a custom source datadir outside default roots. |
| 2 | Ambiguous or missing source installs still fall back to manual review with clear operator guidance. | VERIFIED | `packages/open-bitcoin-cli/src/operator/migration/planning.rs` now requires source config, cookie, or wallet evidence before auto-selecting an explicit path, and `packages/open-bitcoin-cli/src/operator/migration/tests.rs` covers the unsupported explicit-path manual-review fallback. |
| 3 | Phase 25 verification covers explicit custom-path selection while preserving dry-run-first safety and source-data non-mutation. | VERIFIED | The new binary regression proves the custom-path plan stays read-only and secret-safe, while the existing migration package tests plus repo-native verification cover the broader dry-run surface. |

**Score:** 3/3 truths verified

## Requirements Coverage

| Requirement | Status | Evidence |
|---|---|---|
| MIG-02 | SATISFIED | The migration flow can now select explicit custom-location source datadirs through the existing detector rather than degrading to manual review solely because the path was outside default roots. |
| MIG-04 | SATISFIED | `open-bitcoin migrate plan` continues to produce a dry-run-only action plan for the selected source install, with unsupported explicit paths still falling back to manual review instead of mutation or overclaiming certainty. |

## Verification Evidence

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli migration::tests -- --nocapture` passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --test operator_binary -- --nocapture open_bitcoin_migrate_plan_selects_explicit_custom_source_outside_default_roots` passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli -- --nocapture` passed.
- `bun run scripts/generate-loc-report.ts --source=worktree --output=docs/metrics/lines-of-code.md` refreshed the tracked LOC report required by the repo-native gate.
- `bash scripts/verify.sh` passed end-to-end, including:
  - LOC freshness
  - parity breadcrumb validation
  - pure-core dependency and import checks
  - production Rust file-length validation
  - panic-site validation
  - workspace format, lint, build, test, and coverage steps
  - benchmark smoke validation
  - Bazel smoke build

## Human Verification Required

None. The phase ships a narrow dry-run migration hardening change with hermetic
planner tests, an out-of-roots operator-binary regression, and the repo-native
verification gate.

## Residual Risks

- Migration apply mode, source-service cutover, and source-datadir mutation
  remain intentionally out of scope.
- Product-family classification for custom paths still depends on conservative
  path heuristics; a supported custom path may still surface `unknown` product
  family when the path name lacks Core or Knots hints.

---

_Verified: 2026-04-28T20:46:00.000Z_  
_Verifier: GPT-5.4 (GSD yolo wrapper)_
