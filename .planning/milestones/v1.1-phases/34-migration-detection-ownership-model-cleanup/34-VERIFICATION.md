---
phase: 34-migration-detection-ownership-model-cleanup
verified: 2026-04-30T07:55:55Z
status: passed
score: 3/3 truths verified
generated_by: gsd-execute-phase
lifecycle_mode: yolo
phase_lifecycle_id: 34-2026-04-30T07-38-33
generated_at: 2026-04-30T07:55:55Z
lifecycle_validated: true
overrides_applied: 0
---

# Phase 34: Migration Detection Ownership Model Cleanup Verification Report

**Phase Goal:** Tighten the shared migration detection ownership model so
detected installations stop carrying scan-wide service candidates that future
consumers could misread as source-specific evidence.
**Requirements:** none (optional cleanup)
**Verified:** 2026-04-30T07:55:55Z
**Status:** PASS

## Guidance Applied

- `AGENTS.md` materially informed the use of `bash scripts/verify.sh` as the
  repo-native verification contract and the requirement to update
  `docs/parity/source-breadcrumbs.json` when Phase 34 added a new first-party
  Rust file.
- `AGENTS.bright-builds.md` plus `standards/core/architecture.md` materially
  informed the introduction of a clearer domain type (`DetectionScan`) instead
  of leaving scan-level service evidence hidden inside every
  `DetectedInstallation`.
- `standards/core/testing.md` materially informed the focused detector, status,
  migration, and operator-binary regressions, including the new one-concern
  status fallback test for detected service candidates.
- `standards/core/verification.md` and `standards/languages/rust.md` materially
  informed the sync-first workflow, repo-native reruns, and the extraction of
  `migration/planning/labels.rs` when the planner exceeded the 628-line
  production Rust limit.
- No `standards-overrides.md` exception changed the phase outcome.

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
|---|-------|--------|----------|
| 1 | Shared detection types no longer attach scan-wide service definitions to every `DetectedInstallation` as though they belonged to each installation. | VERIFIED | `packages/open-bitcoin-cli/src/operator/detect.rs` now defines `DetectionScan` with a separate `service_candidates` collection, `DetectedInstallation` no longer stores service definitions directly, and `packages/open-bitcoin-cli/src/operator/detect/tests.rs` proves service candidates remain scan-level evidence instead of reappearing in installation-local source paths. |
| 2 | `open-bitcoin migrate plan --source-datadir <custom-path>` keeps the current truthful selected-source behavior and manual-review fallback after the ownership-model cleanup. | VERIFIED | `packages/open-bitcoin-cli/src/operator/migration.rs`, `packages/open-bitcoin-cli/src/operator/migration/planning.rs`, and `packages/open-bitcoin-cli/src/operator/migration/service_evidence.rs` now thread scan-level service evidence through the planner explicitly; `packages/open-bitcoin-cli/src/operator/migration/tests.rs` still passes the selected-source and ambiguous-service regressions, and `packages/open-bitcoin-cli/tests/operator_binary.rs` still passes `open_bitcoin_migrate_plan_selects_explicit_custom_source_outside_default_roots`. |
| 3 | Status, onboarding, wallet, migration, and related verification remain truthful after consumers adopt the tightened ownership model. | VERIFIED | `packages/open-bitcoin-cli/src/operator/status.rs` now consumes explicit scan-level service evidence, `packages/open-bitcoin-cli/src/operator/status/tests.rs` adds a focused fallback-status regression, nearby onboarding/wallet/dashboard fixtures were updated to the tightened installation model, `packages/open-bitcoin-bench/src/cases/operator_runtime.rs` was brought into sync during repo-native verification, and the final `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli` plus `bash scripts/verify.sh` runs completed cleanly. |

**Score:** 3/3 truths verified

## Verification Evidence

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli
  detect::tests` passed after moving service definitions to scan-level
  detection evidence.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli
  status::tests` passed after adding the explicit detected-service fallback
  regression.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli
  migration::tests` passed after the planner and service-association helpers
  adopted `DetectionScan`.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --test
  operator_binary open_bitcoin_migrate_plan_selects_explicit_custom_source_outside_default_roots`
  passed after the shared-model cleanup.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli` passed,
  including the full `operator_binary` integration suite and the updated
  onboarding, wallet, dashboard, and status consumers.
- `bun run scripts/generate-loc-report.ts --source=worktree
  --output=docs/metrics/lines-of-code.md` refreshed the tracked LOC report after
  the final Rust module split and downstream fixture update.
- `bash scripts/verify.sh` passed end to end in `1m 51.749s` after two useful
  first-run failures that surfaced the file-length split and one downstream
  `StatusDetectionEvidence` initializer.

## Human Verification Required

None. Phase 34 closes the remaining migration detection ownership debt through
focused regressions, full package tests, parity-breadcrumb verification, and the
repo-native verification contract.

## Residual Risks

- The base detector still does not assign service ownership eagerly; ownership
  remains a conservative consumer-side association in migration planning. Phase
  34 removes the misleading shared field so future consumers must opt into that
  association explicitly.
- No v1.1 cleanup phases remain, but milestone archive-level acceptance is still
  a separate closeout step outside this phase.

---

_Verified: 2026-04-30T07:55:55Z_  
_Verifier: GPT-5.4 (GSD yolo wrapper)_
