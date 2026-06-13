---
phase: 72-operator-observability-and-support-evidence
plan: 02
subsystem: operator-support
tags: [support-evidence, live-smoke, redaction, verdicts]

requires: [72-CONTEXT, 72-RESEARCH, 72-UI-SPEC, 72-01-SUMMARY]
provides:
  - Typed full-sync support evidence with connected and validated active-chain proof fields
  - Support evidence verdict derivation for sync-to-tip, stay-current, diagnosed-blocker, and inconclusive states
  - Markdown support output for full-sync evidence and compact verdict justifications
  - Live-smoke summary allowlist coverage for Phase 69-71 evidence fields
affects: [phase-72, operator-support, live-smoke, support-bundle]

tech-stack:
  added: []
  patterns:
    - Support verdicts are derived once from typed status and rendered read-only
    - Active-chain evidence preserves explicit unavailable reasons in JSON and Markdown
    - Live-smoke support summaries copy only named scalar/bounded fields

key-files:
  created:
    - .planning/phases/72-operator-observability-and-support-evidence/72-02-SUMMARY.md
  modified:
    - packages/open-bitcoin-cli/src/operator/support.rs
    - packages/open-bitcoin-cli/src/operator/support/live_smoke.rs
    - packages/open-bitcoin-cli/src/operator/support/render.rs
    - packages/open-bitcoin-cli/tests/operator_binary.rs

key-decisions:
  - "Treat normal bounded resource pressure alone as inconclusive; diagnosed_blocker requires explicit blocking evidence."
  - "Keep support Markdown as a projection of `full_sync_evidence` instead of rederiving verdicts in the renderer."
  - "Preserve Phase 69-71 live-smoke evidence through an allowlist while continuing to drop raw reports, logs, credentials, endpoint tables, and wallet material."

patterns-established:
  - "Support evidence tests compare exact connected/validated active-chain height, hash, work, and unavailable reasons."
  - "Binary support-bundle tests seed deterministic durable sync metadata for local JSON/Markdown proof without public-network dependencies."

requirements-completed: [OBS-02, OBS-04]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 72-2026-06-13T16-25-04
generated_at: 2026-06-13T18:35:51Z

duration: 14min
completed: 2026-06-13
---

# Phase 72 Plan 02: Support Evidence Verdict Summary

**Support bundles now carry compact full-sync evidence, typed verdicts, and redacted live-smoke summaries without copying raw operator material.**

## Performance

- **Duration:** 14 min
- **Started:** 2026-06-13T18:21:53Z
- **Completed:** 2026-06-13T18:35:51Z
- **Tasks:** 3
- **Files modified:** 4
- **Files created:** 1

## Accomplishments

- Added `FullSyncEvidence`, `SupportEvidenceVerdict`, and pure `derive_full_sync_evidence` verdict derivation.
- Added Markdown output under `## Full Sync Evidence` with verdict, connected/validated active-chain lines, stay-current, peer contribution, no-progress/reorg, resource pressure, recovery, and justifications.
- Expanded live-smoke support summaries for best-known tip, stay-current, no-progress, reorg, reconcile, resource pressure, peer contribution, and validated active-chain fields.
- Added deterministic unit and binary tests proving positive verdicts, blocker diagnosis, inconclusive states, unavailable reasons, and redaction boundaries.

## Task Commits

Task commits are pending the wrapper-owned final commit after full phase verification.

1. **Task 1: Support evidence and verdict derivation** - `pending final wrapper commit`
2. **Task 2: Support Markdown and binary output proof** - `pending final wrapper commit`
3. **Task 3: Live-smoke summary allowlist expansion** - `pending final wrapper commit`

## Files Created/Modified

- `packages/open-bitcoin-cli/src/operator/support.rs` - Adds full-sync evidence structs, verdict derivation, and verdict matrix tests.
- `packages/open-bitcoin-cli/src/operator/support/render.rs` - Adds full-sync evidence Markdown rendering from bundle evidence only.
- `packages/open-bitcoin-cli/src/operator/support/live_smoke.rs` - Adds Phase 72 allowlisted summary keys and redaction regression coverage.
- `packages/open-bitcoin-cli/tests/operator_binary.rs` - Adds local support-bundle JSON/Markdown proof with deterministic durable sync metadata.

## Decisions Made

- Kept verdict derivation in `support.rs` so JSON and Markdown share one typed result.
- Required explicit blocking evidence for `diagnosed_blocker`; normal resource pressure alone stays `inconclusive`.
- Used local durable sync fixtures rather than live public-network reports for default verification.

## Deviations from Plan

None.

## Issues Encountered

- Cargo tests serialized on package and artifact locks when run in parallel. The queued commands completed successfully.

## Verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --lib phase72_support_verdict_ --all-features -- --nocapture`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --lib phase72_live_smoke_summary_preserves_full_sync_evidence_without_raw_report --all-features -- --nocapture`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --test operator_binary open_bitcoin_support_bundle_includes_phase72_full_sync_evidence_and_typed_verdict --all-features -- --nocapture`
- Plan 72-02 `rg` acceptance checks for verdicts, evidence fields, unavailable reasons, Markdown labels, binary fixture strings, live-smoke allowlist strings, and forbidden raw material strings.
- `node "$HOME/.codex/get-shit-done/bin/gsd-tools.cjs" verify key-links .planning/phases/72-operator-observability-and-support-evidence/72-02-PLAN.md`

## User Setup Required

None - all checks are deterministic and local.

## Next Phase Readiness

Plan 72-03 can project the same validated active-chain, resource pressure, recovery, peer contribution, and stop-reason evidence into metrics/logs and live-smoke script summaries.

## Self-Check: PASSED

- Summary file exists.
- Focused Plan 02 tests pass.
- Support JSON and Markdown preserve compact evidence and exact unavailable reasons.
- Live-smoke summary tests reject raw report/log/credential/wallet material.

*Phase: 72-operator-observability-and-support-evidence*
*Completed: 2026-06-13*
