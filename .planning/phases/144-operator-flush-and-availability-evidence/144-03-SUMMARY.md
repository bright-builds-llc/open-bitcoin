---
phase: 144-operator-flush-and-availability-evidence
plan: 03
subsystem: observability
tags: [chainstate-durability, csobs, metrics, structured-logs, support-redaction, rust]

requires:
  - phase: 144-operator-flush-and-availability-evidence
    provides: Dedicated chainstate_durability FieldAvailability field on OpenBitcoinStatusSnapshot
provides:
  - "Seven fixed MetricKind series and chainstate_durability_metric_samples with no dynamic labels"
  - "Allowlisted chainstate_durability structured log with cause/outcome/label and counts, never coins-best-block hash"
  - "Support Markdown ## Chainstate Durability bullets plus redact_chainstate_durability"
affects:
  - 144-04
  - docs-checker
  - metrics-logs-support

tech-stack:
  added: []
  patterns:
    - "MetricKind::ALL is [Self; 81]; cache-size sample is current None-mode class from the snapshot"
    - "Logs omit hash; optional height= only; fail_closed path invents no tip"
    - "Support Next action is locked; forbid tests scan JSON and value bullets only, not the negation sentence"

key-files:
  created:
    - packages/open-bitcoin-node/src/metrics/chainstate_durability.rs
    - packages/open-bitcoin-node/src/logging/chainstate_durability.rs
    - packages/open-bitcoin-node/src/sync/runtime_state/operator_logs.rs
    - packages/open-bitcoin-cli/src/operator/dashboard/model/metric_labels.rs
    - packages/open-bitcoin-cli/src/operator/support/render/chainstate_durability.rs
    - packages/open-bitcoin-cli/src/operator/support/render/text.rs
  modified:
    - packages/open-bitcoin-node/src/metrics.rs
    - packages/open-bitcoin-node/src/sync/metrics.rs
    - packages/open-bitcoin-node/src/logging.rs
    - packages/open-bitcoin-node/src/sync/runtime_state.rs
    - packages/open-bitcoin-cli/src/operator/support/redaction.rs
    - packages/open-bitcoin-cli/src/operator/support/render.rs
    - packages/open-bitcoin-cli/src/operator/support/tests/forensics_recovery_relay.rs
    - packages/open-bitcoin-cli/src/operator/support/tests/recovery_progress_inbound.rs
    - packages/open-bitcoin-cli/src/operator/support/tests/sync_fixtures.rs
    - docs/parity/source-breadcrumbs.json
    - docs/metrics/lines-of-code.md

key-decisions:
  - "Combined each task RED and GREEN into one hook-passing feat commit because pre-commit runs verify.sh"
  - "Duplicate OK/LARGE/CRITICAL tokens in support render rather than importing Plan 02 CLI/dashboard modules"
  - "Cache-size metric is current None-mode class from the snapshot, not last write"
  - "Leave CSOBS-01 and CSOBS-02 Pending until Plan 04 checker and lifecycle-valid phase verification"

patterns-established:
  - "Pattern 1: Closed MetricKind variants only; Unavailable fields emit no fabricated 0/0/0 samples"
  - "Pattern 2: Structured log source chainstate_durability never includes maybe_coins_best_block_hash"
  - "Pattern 3: Support tip-hash exception is exactly 64 lowercase hex; other hex becomes redacted_chainstate_durability_evidence"

requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 144-2026-09-17T11-23-17
generated_at: 2026-09-17T16:33:25Z

duration: 80min
completed: 2026-09-17
---

# Phase 144 Plan 03: Metrics, Logs, And Support Redaction Summary

**Seven fixed `MetricKind` series, an allowlisted `chainstate_durability` structured log, and support Markdown/JSON bullets now project the shared field with numeric cache-size and last-flush-reason classes and recursive redaction — no dynamic labels, coin dumps, or prune/getblock copy.**

## Performance

- **Duration:** 80 min
- **Started:** 2026-09-17T15:13:14Z
- **Completed:** 2026-09-17T16:33:25Z
- **Tasks:** 3
- **Files modified:** 25

## Accomplishments

- `MetricKind::ALL` is now `[Self; 81]`. Seven closed series project cache-size class 0/1/2, last-flush-reason 0..4, write-kind 0..3, recovery 0..3, and the three availability counters. Unavailable fields emit no samples. Persist appends samples from the operator snapshot after mempool_policy and does not bind them to dashboard charts.
- Structured log source `chainstate_durability` carries `outcome` / `cause` / `label` plus machine labels and counts. Optional `height=` only. Coins-best-block hash, peer ids, and outpoints stay out. Fail-closed mapping emits `recovery_outcome=fail_closed` without an invented tip.
- Support Markdown renders `## Chainstate Durability` with the six locked bullets and the exact Next action. `redact_chainstate_durability` sanitizes free-text reasons and non-tip hashes to `redacted_chainstate_durability_evidence`. The 64-lowercase-hex tip-hash exception remains.

## Task Commits

Each task was committed atomically:

1. **Task 1: Add fixed MetricKind series and samples** - `46070e39` (feat)
2. **Task 2: Allowlisted structured log record** - `31d1ef79` (feat)
3. **Task 3: Support bullets and redaction** - `7c5a047b` (feat)

**Plan metadata:** pending docs commit after this summary.

_Note: Combined RED+GREEN in one hook-passing feat commit per task because pre-commit runs `verify.sh`, matching Phases 140–143 and 144-01/02._

## Files Created/Modified

- `packages/open-bitcoin-node/src/metrics.rs` - Seven variants after `MempoolAdmissionStillPresent`; `ALL: [Self; 81]`
- `packages/open-bitcoin-node/src/metrics/chainstate_durability.rs` - `chainstate_durability_metric_samples`
- `packages/open-bitcoin-node/src/metrics/tests/relay_projection.rs` - Low-cardinality, mapping, unavailable, and omit-sensitive tests
- `packages/open-bitcoin-node/src/metrics/tests/contracts.rs` - `MetricKind::ALL.len() == 81`
- `packages/open-bitcoin-node/src/sync/metrics.rs` - Persist durability samples from the operator snapshot
- `packages/open-bitcoin-cli/src/operator/dashboard/model/metric_labels.rs` - Exhaustive labels extracted so the ninth-chart scan stays clean
- `packages/open-bitcoin-node/src/logging.rs` - Re-export source constant and log helper
- `packages/open-bitcoin-node/src/logging/chainstate_durability.rs` - Allowlisted record plus fail-closed constructor
- `packages/open-bitcoin-node/src/logging/tests/redaction.rs` - Source/label, omit-sensitive, and fail-closed tests
- `packages/open-bitcoin-node/src/sync/runtime_state.rs` - Delegates operator log emit
- `packages/open-bitcoin-node/src/sync/runtime_state/operator_logs.rs` - Emits durability beside block-relay
- `packages/open-bitcoin-cli/src/operator/support/render/chainstate_durability.rs` - Locked heading, six bullets, Next action, duplicated OK/LARGE/CRITICAL
- `packages/open-bitcoin-cli/src/operator/support/render/text.rs` - Shared Markdown helpers extracted under the 628-line gate
- `packages/open-bitcoin-cli/src/operator/support/render.rs` - `push_chainstate_durability` after block-relay
- `packages/open-bitcoin-cli/src/operator/support/redaction.rs` - `redact_chainstate_durability` plus safeguard
- `packages/open-bitcoin-cli/src/operator/support/tests/sync_fixtures.rs` - Phase 144 available and sensitive fixtures
- `packages/open-bitcoin-cli/src/operator/support/tests/forensics_recovery_relay.rs` - Shared-projection Markdown/JSON test
- `packages/open-bitcoin-cli/src/operator/support/tests/recovery_progress_inbound.rs` - Redaction, tip-hash, and forbid-prune tests
- `packages/open-bitcoin-cli/src/operator/support/tests/sync_soak_forensics.rs` - Safeguard list includes durability
- `docs/parity/source-breadcrumbs.json` - `none` breadcrumbs for new first-party files
- `docs/metrics/lines-of-code.md` - Hook-regenerated LOC

## Decisions Made

- Combined RED+GREEN because hooks run the workspace verifier; a RED-only commit cannot pass pre-commit.
- Duplicate `OK` / `LARGE` / `CRITICAL` in support render rather than importing Plan 02 CLI or dashboard modules.
- Cache-size metric samples the current None-mode class from the snapshot, not the last write.
- Leave CSOBS-01 and CSOBS-02 Pending. Plan 04 still owns docs/checker; phase verification is later.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Extract dashboard metric labels so ALL=81 stays exhaustive without a ninth chart**
- **Found during:** Task 1
- **Issue:** Adding seven variants forced dashboard `model.rs` over the 628-line gate. Moving `metric_label` into `metrics.rs` then failed the ninth-chart source scan because that file already mentioned `MempoolRetry`.
- **Fix:** Extracted labels into `dashboard/model/metric_labels.rs` and kept chart kinds in the existing module.
- **Files modified:** `packages/open-bitcoin-cli/src/operator/dashboard/model.rs`, `packages/open-bitcoin-cli/src/operator/dashboard/model/metric_labels.rs`
- **Verification:** pre-commit `verify.sh`
- **Committed in:** `46070e39` (Task 1)

**2. [Rule 3 - Blocking] Extract operator log emit to stay under 628 lines**
- **Found during:** Task 2
- **Issue:** Adding the durability emit in `runtime_state.rs` crossed the production file-length trigger.
- **Fix:** Moved emit into `runtime_state/operator_logs.rs`; the parent still names `chainstate_durability_log_record` for the plan `rg`.
- **Files modified:** `packages/open-bitcoin-node/src/sync/runtime_state.rs`, `packages/open-bitcoin-node/src/sync/runtime_state/operator_logs.rs`
- **Verification:** `cargo test -p open-bitcoin-node --lib chainstate_durability_log` and pre-commit `verify.sh`
- **Committed in:** `31d1ef79` (Task 2)

**3. [Rule 3 - Blocking] Extract support render text helpers after rustfmt wrap**
- **Found during:** Task 3
- **Issue:** Calling the new pusher plus rustfmt wrap pushed `render.rs` to 630 lines (over 628).
- **Fix:** Moved `csv_or_unavailable`, JSON helpers, and availability names into `render/text.rs`.
- **Files modified:** `packages/open-bitcoin-cli/src/operator/support/render.rs`, `packages/open-bitcoin-cli/src/operator/support/render/text.rs`
- **Verification:** `wc -l` = 600 after rustfmt; support tests; pre-commit `verify.sh`
- **Committed in:** `7c5a047b` (Task 3)

**4. [Rule 3 - Blocking] Update soak safeguard list for the new durability token**
- **Found during:** Task 3
- **Issue:** `phase71_support_redaction_names_compact_evidence_bounds` asserted the exact prior safeguard list.
- **Fix:** Added `chainstate durability free-text reasons bounded/redacted` next to the block-relay safeguard.
- **Files modified:** `packages/open-bitcoin-cli/src/operator/support/tests/sync_soak_forensics.rs`
- **Verification:** `cargo test -p open-bitcoin-cli --lib phase71_support_redaction_names_compact_evidence_bounds`
- **Committed in:** `7c5a047b` (Task 3)

**5. [Rule 1 - Bug] Collapse nested tip-hash `if` for clippy**
- **Found during:** Task 3
- **Issue:** First Task 3 commit failed clippy `collapsible_if` on the allowed-tip-hash check.
- **Fix:** Collapsed `if let Some(hash)` with `&& !is_allowed_tip_hash(hash)`.
- **Files modified:** `packages/open-bitcoin-cli/src/operator/support/redaction.rs`
- **Verification:** pre-commit `verify.sh`
- **Committed in:** `7c5a047b` (Task 3)

***

**Total deviations:** 5 auto-fixed (4 blocking, 1 bug)
**Impact on plan:** All auto-fixes kept file-length, clippy, and existing safeguard tests green. No RPC/CLI/dashboard rows, no docs/checker. Combined RED+GREEN was an allowed plan exception, not a deviation. Cargo cannot take two `--lib` filters; Task 3 verify used one `chainstate_durability` substring.

## Issues Encountered

- First Task 3 commit failed clippy; fixed the nested `if` and retried without `--no-verify` or amend.
- Plan verify listed two cargo test filters; cargo accepts one. Ran `chainstate_durability` (all four support tests) plus the soak safeguard test.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Plan 04 can document the seven series, log source, support heading, and wire the cross-surface checker after Phase 116.
- CSOBS-01 and CSOBS-02 stay Pending until that checker and lifecycle-valid phase verification exist.

***
*Phase: 144-operator-flush-and-availability-evidence*
*Completed: 2026-09-17*

## Self-Check: PASSED
