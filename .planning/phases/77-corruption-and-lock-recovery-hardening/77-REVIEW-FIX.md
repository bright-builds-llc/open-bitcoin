---
phase: 77-corruption-and-lock-recovery-hardening
review_path: .planning/phases/77-corruption-and-lock-recovery-hardening/77-REVIEW.md
fixed_at: 2026-06-16T03:03:26Z
status: all_fixed
findings_in_scope: 4
fixed: 4
skipped: 0
iteration: 1
fix_scope: all
commits:
  - f630842
  - aab8e1f
  - 2b0efb1
---

# Phase 77 Code Review Fix Report

## Summary

All code review findings from `77-REVIEW.md` and the follow-up re-reviews were
fixed in `f630842`, `aab8e1f`, and `2b0efb1`.

## Fixed Findings

### CR-01: Live-Smoke Support Evidence Can Leak Authorization Headers

- Added Authorization, Bearer, and Basic credential redaction to live-smoke support summary string sanitization.
- Added `live_smoke_recovery_evidence_redacts_authorization_from_allowlisted_fields` to prove allowlisted Phase 77 recovery fields do not preserve `Authorization`, `Bearer`, or the secret value.

### WR-01: Generic Store Corruption Is Reported As Corruption-Marker Evidence

- Added marker-specific `StorageError::RecoveryMarkerCorruption` for malformed persisted recovery marker records.
- Added `RecoveryCause::CorruptRecord` for generic corrupt stored records.
- Updated the recovery classifier so generic `StorageError::Corruption` maps to `corrupt_record`, while marker-specific corruption maps to `corruption_marker`.
- Updated stable-label docs and serialization coverage for `corrupt_record`.
- Updated sync projection and Fjall marker tests for the new storage error variant.

### Follow-Up CR-01: Live-Smoke Reports Persist RPC Password Arguments

- Added report-only command redaction for live-smoke `commands` and `daemon_sessions` arrays while keeping the original command specs for process execution.
- Replaced sensitive command arguments with `[redacted]` so report JSON/Markdown do not preserve `rpcpassword`, `rpcauth`, cookie-file, Authorization, Bearer, or Basic material.
- Added shell fixture assertions for both successful and preflight-failure reports.

### Follow-Up IN-01: Support Bundle Docs Understate Phase 77 Live-Smoke Summary Fields

- Updated the runtime guide support-bundle redaction boundary to describe the allowlisted Phase 77 recovery fields copied from schema v2 live-smoke reports.
- Kept the wording scoped to compact sanitized summaries rather than raw report embedding.

## Verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib recovery_classifier_corruption_sources_split_record_and_marker_causes --all-features` passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib recovery_evidence_contract_causes_serialize_stable_labels --all-features` passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib fjall_recovery_evidence_corruption_marker_maps_classifier_cause --all-features` passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib malformed_recovery_marker_maps_to_runtime_corruption --all-features` passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --lib live_smoke_recovery_evidence_redacts_authorization_from_allowlisted_fields --all-features` passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib recovery_classifier_ --all-features` passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --lib live_smoke_recovery_evidence_ --all-features` passed.
- `bun run scripts/check-phase77-corruption-lock-recovery.ts` passed.
- `bash scripts/test-run-live-mainnet-smoke.sh` passed.
- `bun test scripts/check-phase75-soak-runner.test.ts` passed.
- `bun run scripts/check-phase75-soak-runner.ts` passed.
- `bun run scripts/generate-loc-report.ts --source=worktree --output=docs/metrics/lines-of-code.md` refreshed the tracked LOC artifact.
- `rg -n "recoveryEvidence|recoveryActionClass|recoveryCause|recoveryNextAction|maybeRecoveryEvidenceUnavailableReason|allowlisted compact summary" docs/operator/runtime-guide.md` passed.

## Residual Risk

None identified for the reviewed findings. Full repo verification and the final review gate are still run by the orchestrator after this report.
