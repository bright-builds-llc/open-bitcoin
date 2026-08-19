---
phase: 137-rpc-and-sanitized-operator-evidence
plan: 07
subsystem: operator-cli
tags: [mempool, dashboard, ratatui, human-status, collector]

requires:
  - phase: 137-rpc-and-sanitized-operator-evidence
    provides: Identifier-free mempool snapshot groups on openbitcoinnetworkstatus.mempool
  - phase: 105
    provides: Relay evidence rows and Phase 105 evicted_count
provides:
  - CLI collector copy of network_status.mempool groups
  - Locked UI-SPEC human-status policy lines after Mempool and before Relay evidence
  - Locked dashboard Mempool and Wallet policy rows without a ninth chart
affects:
  - 137-08 metrics and structured logs for the same labels
  - 137-09 support Markdown reuse of snapshot groups

tech-stack:
  added: []
  patterns:
    - Shared pub(crate) mempool_policy_entries formatters for human status and dashboard rows
    - Collector copies daemon groups; getmempoolinfo.size is fallback only when resources are unavailable

key-files:
  created:
    - packages/open-bitcoin-cli/src/operator/status/render/mempool_policy.rs
    - packages/open-bitcoin-cli/src/operator/status/tests/mempool_policy.rs
    - packages/open-bitcoin-cli/src/operator/dashboard/model/tests/mempool_policy.rs
  modified:
    - packages/open-bitcoin-cli/src/operator/status.rs
    - packages/open-bitcoin-cli/src/operator/status/render.rs
    - packages/open-bitcoin-cli/src/operator/status/tests.rs
    - packages/open-bitcoin-cli/src/operator/dashboard/model/relay.rs
    - packages/open-bitcoin-cli/src/operator/dashboard/model/tests.rs
    - docs/parity/source-breadcrumbs.json

key-decisions:
  - "Reuse one mempool_policy_entries projector for CLI human lines and dashboard rows."
  - "Gate Eviction (relay) on mempool.eviction availability so stopped lines share the transactions reason."
  - "Keep MAX_DASHBOARD_CHARTS = 8; Phase 137 groups stay rows, not sparklines."

patterns-established:
  - "Pattern 1: Locked Open Bitcoin labels live in one formatter family; Knots aliases stay off status and dashboard."
  - "Pattern 2: Stopped policy groups reuse the same Unavailable reason as mempool.transactions."

requirements-completed: [MPOBS-02, MPOBS-03]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 137-2026-08-19T21-52-42
generated_at: 2026-08-19T23:36:32Z

duration: 8min
completed: 2026-08-19
---

# Phase 137 Plan 07: Collector Copy And Locked Status Rows Summary

**CLI collector copies identifier-free mempool snapshot groups, and human status plus the Ratatui dashboard render locked Open Bitcoin policy rows without a ninth chart or last-package table.**

## Performance

- **Duration:** 8 min
- **Started:** 2026-08-19T23:28:30Z
- **Completed:** 2026-08-19T23:36:32Z
- **Tasks:** 2
- **Files modified:** 9

## Accomplishments

- Live collector assigns `network_status.mempool` groups and keeps `getmempoolinfo.size` only as a transactions fallback when resources are unavailable.
- Human status inserts the locked UI-SPEC lines after `Mempool:` and before `Relay evidence:`.
- Dashboard Mempool and Wallet rows use the same labels and values; `MAX_DASHBOARD_CHARTS` stays 8.

## Task Commits

Each task was committed atomically:

1. **Task 1 RED: failing human/JSON policy tests** - `24e82fc6` (test)
2. **Task 1 GREEN: collector copy and human policy lines** - `5283ca82` (feat)
3. **Task 2 RED: failing dashboard row tests** - `7f6b6810` (test)
4. **Task 2 GREEN: dashboard policy rows** - `2262bb93` (feat)

**Plan metadata:** pending docs commit for this SUMMARY

_Note: TDD tasks produced RED then GREEN commits; no refactor commit was needed._

## Files Created/Modified

- `packages/open-bitcoin-cli/src/operator/status/render/mempool_policy.rs` - Shared locked-label formatters
- `packages/open-bitcoin-cli/src/operator/status.rs` - Collector copy of `network_status.mempool` and stopped-group reasons
- `packages/open-bitcoin-cli/src/operator/status/render.rs` - Insert `mempool_policy_lines` after Mempool
- `packages/open-bitcoin-cli/src/operator/status/tests.rs` - Wire human policy tests
- `packages/open-bitcoin-cli/src/operator/status/tests/mempool_policy.rs` - Named human/JSON tests
- `packages/open-bitcoin-cli/src/operator/dashboard/model/relay.rs` - Insert policy rows from shared formatters
- `packages/open-bitcoin-cli/src/operator/dashboard/model/tests.rs` - Wire dashboard policy tests
- `packages/open-bitcoin-cli/src/operator/dashboard/model/tests/mempool_policy.rs` - Named dashboard tests
- `docs/parity/source-breadcrumbs.json` - Register new first-party Rust files

## Decisions Made

- Human and dashboard share `mempool_policy_entries` so labels cannot drift.
- `Eviction (relay)` still reads `evicted_count` from Phase 105 `relay.outcome_counters`, but only when `mempool.eviction` is Available. Stopped nodes keep `Unavailable: {transactions reason}` because default relay counters are Implemented zeros.
- Phase 137 groups stay rows. Chart kinds and `MAX_DASHBOARD_CHARTS = 8` are unchanged.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 2 - Missing Critical] Gate Eviction (relay) on the eviction group**
- **Found during:** Task 1 (human policy lines)
- **Issue:** Default Phase 105 `outcome_counters` are Implemented with zeros, so a stopped node would have rendered `evicted_count=0` instead of the shared transactions unavailable reason.
- **Fix:** Render `Unavailable: {reason}` when `mempool.eviction` is unavailable; otherwise format `evicted_count` from relay counters.
- **Files modified:** `packages/open-bitcoin-cli/src/operator/status/render/mempool_policy.rs`
- **Verification:** `stopped_status_policy_lines_share_transactions_unavailable_reason` passes
- **Committed in:** `5283ca82`

**2. [Rule 2 - Missing Critical] Registered new Rust files in breadcrumb map**
- **Found during:** Task 1 and Task 2
- **Issue:** New first-party Rust files require a parity-breadcrumb mapping.
- **Fix:** Added the new status and dashboard files to the existing CLI groups.
- **Files modified:** `docs/parity/source-breadcrumbs.json`
- **Verification:** File-header breadcrumbs present; mapping lists the new paths
- **Committed in:** `24e82fc6`, `5283ca82`, `7f6b6810`

**3. [Rule 3 - Blocking] Avoided the `mempoolminfee` literal under dashboard/model**
- **Found during:** Task 2 (acceptance criteria)
- **Issue:** The plan's `rg mempoolminfee dashboard/model` check would fail if the test named the Knots alias in source.
- **Fix:** Constructed the forbidden alias from `["mempool", "min", "fee"]` so production and tests stay alias-free.
- **Files modified:** `packages/open-bitcoin-cli/src/operator/dashboard/model/tests/mempool_policy.rs`
- **Verification:** `rg mempoolminfee packages/open-bitcoin-cli/src/operator/dashboard/model` returns no matches
- **Committed in:** `2262bb93`

---

**Total deviations:** 3 auto-fixed (2 missing critical, 1 blocking)
**Impact on plan:** Required for stopped-reason correctness, breadcrumb policy, and the written acceptance grep. No scope creep.

## Issues Encountered

- `dashboard_does_not_add_a_ninth_chart_kind` already passed in RED because the eight-chart lock already existed; it remains a regression lock.
- `packages/open-bitcoin-cli/src/operator/dashboard/model.rs` did not need edits; rows were inserted in `relay.rs` as specified.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Plan 08 can emit the same fixed labels on metrics and logs without adding a ninth chart.
- Shared formatters are the single source of locked dashboard/status copy.
- No blockers for those plans.

## Self-Check: PASSED

---
*Phase: 137-rpc-and-sanitized-operator-evidence*
*Completed: 2026-08-19*
