---
phase: 94-dos-and-resource-governance
plan: 05
subsystem: network
tags: [dos, resource-governance, inbound-status, metrics, structured-logging]

requires:
  - phase: 94-03
    provides: peer-policy and inbound-status evidence preceding resource-governance projection
  - phase: 94-04
    provides: shared request resource-governance labels and bounded peer request caps
provides:
  - Shared inbound resource-governance status counters and latest decision evidence
  - Fixed low-cardinality MetricKind variants for Phase 94 resource governance
  - Bounded inbound_resource_governance structured JSONL log records from managed RPC resource events
affects: [open-bitcoin-node, open-bitcoin-rpc, open-bitcoin-cli, resource-governance, observability]

tech-stack:
  added: []
  patterns:
    - Shared inbound status is the source of bounded resource-governance evidence.
    - Structured logs derive from the same InboundResourceGovernanceEvent fields as status.
    - Metrics remain fixed MetricKind variants with static dashboard labels.

key-files:
  created:
    - .planning/phases/94-dos-and-resource-governance/94-05-SUMMARY.md
    - packages/open-bitcoin-node/src/metrics/tests.rs
    - packages/open-bitcoin-rpc/src/context/resource_governance.rs
  modified:
    - docs/metrics/lines-of-code.md
    - docs/parity/source-breadcrumbs.json
    - packages/open-bitcoin-cli/src/operator/dashboard/model.rs
    - packages/open-bitcoin-cli/src/operator/dashboard/model/tests.rs
    - packages/open-bitcoin-cli/src/operator/status/render/tests.rs
    - packages/open-bitcoin-cli/src/operator/status/tests.rs
    - packages/open-bitcoin-cli/src/operator/support/tests.rs
    - packages/open-bitcoin-node/src/logging.rs
    - packages/open-bitcoin-node/src/logging/tests.rs
    - packages/open-bitcoin-node/src/metrics.rs
    - packages/open-bitcoin-node/src/network.rs
    - packages/open-bitcoin-node/src/network/inbound.rs
    - packages/open-bitcoin-node/src/status/inbound.rs
    - packages/open-bitcoin-node/src/status/inbound/tests.rs
    - packages/open-bitcoin-node/src/status/tests.rs
    - packages/open-bitcoin-rpc/src/context.rs
    - packages/open-bitcoin-rpc/src/context/network.rs
    - packages/open-bitcoin-rpc/src/context/tests.rs

key-decisions:
  - "Project Phase 94 resource-governance events once through shared inbound status and reuse that contract for RPC/log evidence."
  - "Keep resource-governance metrics as fixed MetricKind variants with static CLI dashboard labels; do not add dynamic metric labels."
  - "Make managed RPC resource-event logging datadir-backed and bounded, with redaction for suspicious raw fields and a bounded write-failure count."

patterns-established:
  - "Inbound resource evidence uses counters plus a latest low-cardinality decision, not renderer-local summaries."
  - "Structured log records use allowlisted key/value fields from shared status events and redact raw-looking values."

requirements-completed: [DOS-04, DOS-05]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 94-2026-06-26T15-47-23
generated_at: 2026-06-26T21:47:53Z

duration: 65m 12s
completed: 2026-06-26
---

# Phase 94 Plan 05: Resource Governance Evidence Summary

**Resource-governance events now flow through shared inbound status, fixed metric variants, and bounded inbound_resource_governance structured logs.**

## Performance

- **Duration:** 65m 12s
- **Started:** 2026-06-26T20:42:41Z
- **Completed:** 2026-06-26T21:47:53Z
- **Tasks:** 3
- **Files modified:** 20

## Accomplishments

- Added shared resource-governance counters and `latest_resource_governance_decision` to inbound status with serde-safe unavailable defaults.
- Added managed network and RPC context projection for resource events without exposing peer IDs, endpoints, payload bytes, permission strings, credentials, or dynamic labels.
- Added eight fixed `MetricKind` variants for inbound resource pressure, request caps, payload rejection, timeout, churn, and reconnect suppression evidence.
- Added bounded `inbound_resource_governance` structured log records written through the existing JSONL log writer when a managed RPC context has a data directory.

## Task Commits

1. **Task 1: Add shared resource status and managed projection** - `e3d7bb3f` (feat)
2. **Task 2: Add fixed resource-governance metric variants** - `1df7450c` (feat)
3. **Task 3: Add bounded structured log projection for resource events** - `ca471642` (feat)

## Files Created/Modified

- `docs/metrics/lines-of-code.md` - Hook-refreshed tracked LOC artifact.
- `docs/parity/source-breadcrumbs.json` - Added the new metrics test file breadcrumb.
- `packages/open-bitcoin-cli/src/operator/dashboard/model.rs` - Added static dashboard labels for the new metric variants.
- `packages/open-bitcoin-cli/src/operator/dashboard/model/tests.rs` - Covered the dashboard metric label exhaustiveness update.
- `packages/open-bitcoin-cli/src/operator/status/render/tests.rs` - Updated inbound status fixtures for new shared resource fields.
- `packages/open-bitcoin-cli/src/operator/status/tests.rs` - Updated status snapshot fixtures for new shared resource fields.
- `packages/open-bitcoin-cli/src/operator/support/tests.rs` - Updated support-bundle inbound fixtures for new shared resource fields.
- `packages/open-bitcoin-node/src/logging.rs` - Added bounded resource-governance structured log projection and sanitization.
- `packages/open-bitcoin-node/src/logging/tests.rs` - Covered allowlisted log fields and redaction of suspicious raw values.
- `packages/open-bitcoin-node/src/metrics.rs` - Added fixed Phase 94 resource-governance metric variants.
- `packages/open-bitcoin-node/src/metrics/tests.rs` - Split metrics tests out of the production metrics module and covered new metric names.
- `packages/open-bitcoin-node/src/network.rs` - Exported managed resource-governance network helpers.
- `packages/open-bitcoin-node/src/network/inbound.rs` - Added managed resource-governance counters, event recording, and log-record projection.
- `packages/open-bitcoin-node/src/status/inbound.rs` - Added shared resource-governance status contract and defaults.
- `packages/open-bitcoin-node/src/status/inbound/tests.rs` - Covered defaults and next-action counter mapping.
- `packages/open-bitcoin-node/src/status/tests.rs` - Updated status serialization fixture coverage.
- `packages/open-bitcoin-rpc/src/context.rs` - Added RPC context resource-governance log sink state.
- `packages/open-bitcoin-rpc/src/context/network.rs` - Wired shared resource-governance evidence into current inbound status and runtime config initialization.
- `packages/open-bitcoin-rpc/src/context/resource_governance.rs` - Added shared RPC resource evidence projection and log append helper.
- `packages/open-bitcoin-rpc/src/context/tests.rs` - Covered RPC status projection and actual JSONL log append behavior.

## Decisions Made

- Resource-governance status, metrics, and logs all use bounded labels from `InboundResourceGovernanceEvent`; raw peer, endpoint, payload, permission, and credential material stays out of shared evidence.
- Metrics remain enum variants in `MetricKind::ALL` with exact snake_case names, and CLI dashboard labels are static match arms.
- The managed RPC context writes resource-governance logs only when configured with a data directory; optional sink write failures increment a bounded debug-visible counter instead of introducing dynamic error evidence.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Updated public inbound status fixtures outside the initial narrow write set**
- **Found during:** Task 1 (Add shared resource status and managed projection)
- **Issue:** Adding serde-defaulted fields to `InboundPeerServingStatus` required fixture updates in CLI/status/support tests that deserialize or render the shared public status shape.
- **Fix:** Updated affected status, CLI render, and support tests to include the new unavailable resource-governance evidence.
- **Files modified:** `packages/open-bitcoin-node/src/status/tests.rs`, `packages/open-bitcoin-cli/src/operator/status/render/tests.rs`, `packages/open-bitcoin-cli/src/operator/status/tests.rs`, `packages/open-bitcoin-cli/src/operator/support/tests.rs`
- **Verification:** Focused inbound/RPC tests and the Task 1 commit hook passed.
- **Committed in:** `e3d7bb3f`

**2. [Rule 3 - Blocking] Split RPC resource projection into a helper module to satisfy file-length policy**
- **Found during:** Task 1 (Add shared resource status and managed projection)
- **Issue:** Adding resource-governance projection directly to `context/network.rs` would push a production file against the repo file-length gate.
- **Fix:** Added `packages/open-bitcoin-rpc/src/context/resource_governance.rs` and registered it from `context.rs`.
- **Files modified:** `packages/open-bitcoin-rpc/src/context.rs`, `packages/open-bitcoin-rpc/src/context/resource_governance.rs`, `packages/open-bitcoin-rpc/src/context/network.rs`
- **Verification:** `bash scripts/check-file-lengths.sh` and the Task 1 commit hook passed.
- **Committed in:** `e3d7bb3f`

**3. [Rule 3 - Blocking] Updated CLI dashboard labels for new metric variants**
- **Found during:** Task 2 (Add fixed resource-governance metric variants)
- **Issue:** Expanding `MetricKind` made the CLI dashboard metric-label match non-exhaustive outside the initial Task 2 file list.
- **Fix:** Added static labels for all eight new resource-governance metric variants and updated label coverage.
- **Files modified:** `packages/open-bitcoin-cli/src/operator/dashboard/model.rs`, `packages/open-bitcoin-cli/src/operator/dashboard/model/tests.rs`
- **Verification:** `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli dashboard_metric_labels_cover_all_metric_kinds --no-fail-fast` and the Task 2 commit hook passed.
- **Committed in:** `1df7450c`

**4. [Rule 3 - Blocking] Split metrics tests and added source breadcrumb**
- **Found during:** Task 2 (Add fixed resource-governance metric variants)
- **Issue:** Keeping expanded tests inline in `metrics.rs` would violate the production file-length gate and the new test file required a parity breadcrumb.
- **Fix:** Moved metrics tests to `packages/open-bitcoin-node/src/metrics/tests.rs` and added `docs/parity/source-breadcrumbs.json` coverage.
- **Files modified:** `packages/open-bitcoin-node/src/metrics.rs`, `packages/open-bitcoin-node/src/metrics/tests.rs`, `docs/parity/source-breadcrumbs.json`
- **Verification:** `bun run scripts/check-parity-breadcrumbs.ts --check`, `bash scripts/check-file-lengths.sh`, and the Task 2 commit hook passed.
- **Committed in:** `1df7450c`

**5. [Rule 3 - Blocking] Routed structured log append through the resource-governance helper**
- **Found during:** Task 3 (Add bounded structured log projection for resource events)
- **Issue:** Adding timestamped logging and sink state directly to `context/network.rs` would again pressure the file-length gate, and discarding log writer errors would conflict with repo error-handling guidance.
- **Fix:** Moved timestamped resource-event recording into `context/resource_governance.rs`, returned `Result` from the deterministic test helper, and tracked optional production sink failures with a bounded counter.
- **Files modified:** `packages/open-bitcoin-rpc/src/context.rs`, `packages/open-bitcoin-rpc/src/context/network.rs`, `packages/open-bitcoin-rpc/src/context/resource_governance.rs`, `packages/open-bitcoin-rpc/src/context/tests.rs`
- **Verification:** Focused logging/inbound/RPC tests, clippy for node/RPC, file-length check, full Rust gate, and the Task 3 commit hook passed.
- **Committed in:** `ca471642`

**Total deviations:** 5 auto-fixed blocking issues.
**Impact on plan:** All deviations were required for compile, repo policy, or verifier success. They did not add dynamic metric labels, raw evidence, relay behavior, public-production claims, or unplanned trust boundaries.

## Issues Encountered

- TDD RED tests were run locally for each task, but failing commits were not created because the user explicitly required passing commits and repo hooks require passing commits.
- A stale import introduced while moving Task 3 code was caught by `cargo clippy -- -D warnings`; it was removed before committing.

## Known Stubs

None. Stub-pattern scans found no `TODO`, `FIXME`, placeholder text, "coming soon", "not available", or hardcoded empty UI data stubs in the files touched by this plan. A broad empty-literal scan only matched existing format strings.

## Threat Flags

None. The only new file-access surface is the planned datadir-backed structured log append for `inbound_resource_governance` records, covered by T-94-05-05 and constrained to bounded, sanitized fields.

## Verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node inbound --no-fail-fast` - passed
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node metrics --no-fail-fast` - passed
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node logging --no-fail-fast` - passed
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc context --no-fail-fast` - passed
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli dashboard_metric_labels_cover_all_metric_kinds --no-fail-fast` - passed
- `cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-targets --all-features -- -D warnings` - passed
- `cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --all-targets --all-features -- -D warnings` - passed
- `cargo fmt --manifest-path packages/Cargo.toml --all` - passed before each task commit
- `cargo clippy --manifest-path packages/Cargo.toml --all-targets --all-features -- -D warnings` - passed before each task commit
- `cargo build --manifest-path packages/Cargo.toml --all-targets --all-features` - passed before each task commit
- `cargo test --manifest-path packages/Cargo.toml --all-features` - passed before each task commit
- `bash scripts/check-file-lengths.sh` - passed after helper/test splits
- `bun run scripts/check-parity-breadcrumbs.ts --check` - passed after adding the metrics test breadcrumb
- `bash scripts/verify.sh` - passed through all three successful commit hooks

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Phase 94 resource-governance evidence is now available from shared status, fixed metrics, and structured logs. Later renderer/support plans can consume the shared fields without inventing local summaries or adding raw peer, endpoint, payload, permission, credential, dynamic-label, relay, or production-readiness claims.

## Self-Check: PASSED

- Found summary file: `.planning/phases/94-dos-and-resource-governance/94-05-SUMMARY.md`
- Found task commit: `e3d7bb3f` (`feat(94-05): add shared resource governance status`)
- Found task commit: `1df7450c` (`feat(94-05): add resource governance metrics`)
- Found task commit: `ca471642` (`feat(94-05): append resource governance structured logs`)

---
*Phase: 94-dos-and-resource-governance*
*Completed: 2026-06-26*
