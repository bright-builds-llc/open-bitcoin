---
phase: 65-support-bundle-and-operator-review-docs
plan: 01
subsystem: operator-support
tags: [support-bundle, redaction, service-evidence, markdown, rust]
requires:
  - phase: 64-service-restart-and-same-datadir-resume-evidence
    provides: service restart/resume status fields for support evidence
provides:
  - Support bundle Markdown service lifecycle, restart/resume, log path, and metrics availability labels
  - Regression coverage for service/log/metrics evidence in redacted support bundles
affects: [phase-65, support-bundles, operator-review, v1.5-uat]
tech-stack:
  added: []
  patterns: [allowlisted support summaries, metadata-only support evidence]
key-files:
  created: []
  modified:
    - packages/open-bitcoin-cli/src/operator/support/render.rs
    - packages/open-bitcoin-cli/tests/operator_binary.rs
key-decisions:
  - "Keep support bundle service/log/metrics details as compact Markdown labels backed by the existing shared status snapshot."
patterns-established:
  - "Support Markdown uses compact JSON rendering for nested FieldAvailability values instead of reinterpreting support evidence."
requirements-completed: [OBS-03]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 65-2026-06-08T14-45-59
generated_at: 2026-06-08T15:07:08.036Z
duration: 9min
completed: 2026-06-08
---

# Phase 65: Support Bundle and Operator Review Docs Summary

**Redacted support bundles now surface service lifecycle, restart/resume, log, and metrics availability evidence in Markdown.**

## Performance

- **Duration:** 9 min
- **Started:** 2026-06-08T14:58:00Z
- **Completed:** 2026-06-08T15:07:08Z
- **Tasks:** 2
- **Files modified:** 2

## Accomplishments

- Added compact Markdown labels for service lifecycle, service restart/resume, log path, and metrics availability.
- Extended the existing support bundle regression test to assert service restart/resume JSON state and Markdown labels.
- Re-ran focused support bundle tests successfully.

## Task Commits

1. **Tasks 1-2: Support bundle evidence and rendering** - `1ab284f` (feat)

**Plan metadata:** pending final metadata commit

## Files Created/Modified

- `packages/open-bitcoin-cli/src/operator/support/render.rs` - Renders service/log/metrics availability from the shared status snapshot in support Markdown.
- `packages/open-bitcoin-cli/tests/operator_binary.rs` - Asserts compact service evidence appears while forbidden raw support markers stay absent.

## Decisions Made

- Used a generic compact JSON renderer for nested availability values so support Markdown does not duplicate service/status domain interpretation.

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Plan 65-02 can document the new support evidence labels and enforce v1.5 review boundaries through a deterministic checker.

---
*Phase: 65-support-bundle-and-operator-review-docs*
*Completed: 2026-06-08*
