---
phase: 71-resource-bounds-and-durable-restart-resume
plan: 01
subsystem: operator-support
tags: [support-evidence, live-smoke, redaction, resource-pressure]

requires: [71-CONTEXT, 71-RESEARCH]
provides:
  - Compact support evidence redaction regression coverage
  - Live-smoke summary allowlist regression coverage
  - Runtime support rendering coverage for all configured resource bounds
affects: [phase-71, operator-support, live-smoke, runtime-status]

tech-stack:
  added: []
  patterns:
    - Support tests assert exact redaction and safeguard labels
    - Live-smoke support summaries are verified from allowlisted JSON fields only
    - Runtime support rendering consumes `SyncResourcePressure` without local resource models

key-files:
  created:
    - .planning/phases/71-resource-bounds-and-durable-restart-resume/71-01-SUMMARY.md
  modified:
    - packages/open-bitcoin-cli/src/operator/runtime/support.rs
    - packages/open-bitcoin-cli/src/operator/support.rs
    - packages/open-bitcoin-cli/src/operator/support/live_smoke.rs

key-decisions:
  - "Keep support evidence compact and redacted; do not copy raw daemon, peer, log, cookie, or RPC password material."
  - "Use existing `SyncResourcePressure` rendering as the resource-bound truth source."

patterns-established:
  - "Phase 71 support tests use exact strings for redaction omissions, safeguards, and resource-pressure output."
  - "Live-smoke summaries can include bounded resource-pressure fields while excluding raw report material."

requirements-completed: [RES-01]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 71-2026-06-13T10-34-37
generated_at: 2026-06-13T11:23:00Z

duration: 25min
completed: 2026-06-13
---

# Phase 71 Plan 01: Support Evidence Resource-Bound Summary

**Operator support evidence now has deterministic tests proving Phase 71 resource evidence stays compact, redacted, and sourced from typed runtime pressure fields.**

## Performance

- **Duration:** 25 min
- **Started:** 2026-06-13T10:58:00Z
- **Completed:** 2026-06-13T11:23:00Z
- **Tasks:** 2
- **Files modified:** 3
- **Files created:** 1

## Accomplishments

- Added a support redaction regression test for exact omitted material and safeguard labels.
- Added a live-smoke summary regression test proving allowlisted `resourcePressure` fields survive while raw peer/log/stdout/stderr/RPC secret material is excluded.
- Added runtime support coverage for the complete `SyncResourcePressure` rendering line and bounded counter line.

## Task Commits

Task commits are pending the wrapper-owned final commit after full phase verification.

1. **Task 1: Support redaction and live-smoke compactness tests** - `pending final wrapper commit`
2. **Task 2: Runtime resource-pressure support rendering test** - `pending final wrapper commit`

## Files Created/Modified

- `packages/open-bitcoin-cli/src/operator/support.rs` - Added `phase71_support_redaction_names_compact_evidence_bounds`.
- `packages/open-bitcoin-cli/src/operator/support/live_smoke.rs` - Added `phase71_live_smoke_summary_is_allowlisted_and_bounded`.
- `packages/open-bitcoin-cli/src/operator/runtime/support.rs` - Added `phase71_runtime_support_resource_pressure_lists_all_configured_bounds`.

## Decisions Made

- Reused existing support redaction metadata and live-smoke summary allowlists instead of introducing a new resource evidence model.
- Kept the support rendering proof at the CLI boundary so future renderer changes cannot silently drop configured resource-bound fields.

## Deviations from Plan

None.

## Issues Encountered

- Cargo test commands were initially started in parallel and serialized on Cargo locks. The queued commands drained successfully and all targeted Plan 01 tests passed.

## Verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli phase71_support_redaction_names_compact_evidence_bounds --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli phase71_live_smoke_summary_is_allowlisted_and_bounded --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli phase71_runtime_support_resource_pressure_lists_all_configured_bounds --all-features`
- Acceptance `rg` checks for the three Phase 71 test names, exact redaction labels, allowlisted resource-pressure fields, and exact support output strings.

## User Setup Required

None - all checks are deterministic and local.

## Next Phase Readiness

Plan 71-04 can document and checker-gate support evidence compactness using the exact Phase 71 test names.

## Self-Check: PASSED

- Summary file exists.
- Required Plan 01 tests exist and pass.
- No public-network, service-manager, or raw support evidence scope was added.

*Phase: 71-resource-bounds-and-durable-restart-resume*
*Completed: 2026-06-13*
