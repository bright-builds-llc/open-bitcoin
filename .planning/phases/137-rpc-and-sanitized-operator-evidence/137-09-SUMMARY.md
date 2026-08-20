---
phase: 137-rpc-and-sanitized-operator-evidence
plan: 09
subsystem: support-bundles
tags: [mempool, support, redaction, markdown, shareable-bundle]

requires:
  - phase: 137-rpc-and-sanitized-operator-evidence
    provides: Identifier-free pressure, eviction, checkpoint, recovery, retry, and admission groups on MempoolStatus
provides:
  - redact_mempool_policy_groups on support_status_for_bundle
  - Count-only support Markdown policy bullets after Mempool and before Relay evidence
  - Locked local-admission next-action sentence on shareable bundles
  - package fingerprints in the omitted list
affects:
  - 137-10 and later operator evidence consumers of shareable support copy

tech-stack:
  added: []
  patterns:
    - Available count structs pass through; Unavailable free-text sanitizes to redacted_relay_mempool_evidence
    - Support Markdown reuses status mempool_policy_entries for UI-SPEC value formats

key-files:
  created:
    - packages/open-bitcoin-cli/src/operator/support/tests/mempool_policy.rs
  modified:
    - packages/open-bitcoin-cli/src/operator/support/redaction.rs
    - packages/open-bitcoin-cli/src/operator/support/render/relay.rs
    - packages/open-bitcoin-cli/src/operator/support/tests.rs
    - packages/open-bitcoin-cli/src/operator/support/tests/forensics_recovery_relay.rs
    - packages/open-bitcoin-cli/src/operator/dashboard/model.rs
    - docs/parity/source-breadcrumbs.json

key-decisions:
  - "Reuse sanitized_relay_evidence_text so group reasons become redacted_relay_mempool_evidence."
  - "Reuse status mempool_policy_entries so support bullets match dashboard/status formats."
  - "Place the locked policy next-action after policy bullets and before Relay evidence."
  - "Scope the 64-hex Markdown forbid check to the Relay and Mempool Evidence section so forensic hashes stay out of that assertion."

patterns-established:
  - "Pattern 1: Counts need no redaction; identifier-bearing Unavailable reasons do."
  - "Pattern 2: Recovery (mempool counts) stays distinct from Relay recovery."

requirements-completed: [MPOBS-02, MPOBS-03]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 137-2026-08-19T21-52-42
generated_at: 2026-08-19T23:52:10Z

duration: 6min
completed: 2026-08-19
---

# Phase 137 Plan 09: Support Bundle Policy Redaction Summary

**Shareable support bundles now redact new mempool-group free text and render count-only policy bullets with the locked sentence that local admission is not public or default relay.**

## Performance

- **Duration:** 6 min
- **Started:** 2026-08-19T23:45:29Z
- **Completed:** 2026-08-19T23:52:10Z
- **Tasks:** 2
- **Files modified:** 7

## Accomplishments

- `support_status_for_bundle` walks resources, fee floors, pressure, eviction, checkpoint, recovery, retry, and admission; Available counts stay intact.
- Unavailable reasons that carry 64-hex or peer endpoints become `redacted_relay_mempool_evidence`, and poisoned recovery txids are absent from serialized bundle JSON.
- Support Markdown inserts Virtual size through Relay states after `Mempool:` and before `Relay evidence:`, plus the locked local-admission next-action sentence.
- The omitted list now mentions `package fingerprints`.

## Task Commits

Each task was committed atomically:

1. **Task 1 compile fix: dashboard MetricKind labels** - `89b75d9b` (fix)
2. **Task 1 RED: failing support group redaction tests** - `54c796b2` (test)
3. **Task 1 GREEN: redact mempool policy groups** - `57530255` (feat)
4. **Task 2 RED: failing support Markdown policy tests** - `185dd68c` (test)
5. **Task 2 GREEN: render count-only policy bullets** - `d714d6ef` (feat)

**Plan metadata:** pending docs commit for this SUMMARY

_Note: TDD tasks produced RED then GREEN commits; no refactor commit was needed._

## Files Created/Modified

- `packages/open-bitcoin-cli/src/operator/support/redaction.rs` - `redact_mempool_policy_groups` and package-fingerprint omitted text
- `packages/open-bitcoin-cli/src/operator/support/render/relay.rs` - count-only policy bullets and locked next action
- `packages/open-bitcoin-cli/src/operator/support/tests/mempool_policy.rs` - named redaction and Markdown tests
- `packages/open-bitcoin-cli/src/operator/support/tests.rs` - module and group-type imports
- `packages/open-bitcoin-cli/src/operator/support/tests/forensics_recovery_relay.rs` - expected Virtual size and next-action copy
- `packages/open-bitcoin-cli/src/operator/dashboard/model.rs` - labels for the 14 plan-08 mempool MetricKind values
- `docs/parity/source-breadcrumbs.json` - register the new test file

## Decisions Made

- Group Unavailable reasons reuse the existing relay/mempool sanitizer and `redacted_relay_mempool_evidence` label.
- Support bullets reuse `mempool_policy_entries` so value formats stay aligned with status/dashboard.
- The locked policy next-action sits with the new bullets, before `Relay evidence:`.
- The 64-hex Markdown guard is limited to the Relay and Mempool Evidence section so existing forensic hashes are not treated as policy leaks.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Cover new mempool MetricKind labels on the dashboard**
- **Found during:** Task 1 (RED compile)
- **Issue:** Plan 08 added 14 `MetricKind` values; `dashboard/model.rs` `metric_label` was exhaustive and blocked `open-bitcoin-cli` tests.
- **Fix:** Added Open Bitcoin labels for those kinds without adding a ninth chart.
- **Files modified:** `packages/open-bitcoin-cli/src/operator/dashboard/model.rs`
- **Verification:** crate compiles; named redaction tests run
- **Committed in:** `89b75d9b`

**2. [Rule 2 - Missing Critical] Register the new support test file in breadcrumbs**
- **Found during:** Task 1 (RED)
- **Issue:** New first-party Rust test files require a parity-breadcrumb mapping.
- **Fix:** Added `support/tests/mempool_policy.rs` to the cli-operator-support-bundles group.
- **Files modified:** `docs/parity/source-breadcrumbs.json`
- **Verification:** mapping lists the new path
- **Committed in:** `54c796b2`

---

**Total deviations:** 2 auto-fixed (1 blocking, 1 missing critical)
**Impact on plan:** Required for compile and breadcrumb policy. No scope creep into dashboard charts or JSON-RPC bodies.

## Issues Encountered

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Support bundles stay shareable and identifier-free for the new groups.
- Later plans can keep the same count-only labels and locked next-action copy.

## Self-Check: PASSED

---
*Phase: 137-rpc-and-sanitized-operator-evidence*
*Completed: 2026-08-19*
---
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 137-2026-08-19T21-52-42
generated_at: 2026-08-20T00:10:00Z
---
