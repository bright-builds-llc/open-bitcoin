---
phase: 105-operator-rpc-metrics-logs-and-support-evidence
plan: 105-01
subsystem: node-status-rpc
tags:
  - rust
  - relay-evidence
  - rpc
  - parity
requires: []
provides:
  - Shared sanitized relay and mempool evidence status contract.
  - Fixed relay outcome counter projection from fanout and serving state.
  - Open Bitcoin-specific RPC relay evidence output without baseline RPC shape changes.
affects:
  - operator-status
  - dashboard
  - metrics
  - logging
  - support-bundles
tech-stack:
  added: []
  patterns:
    - Typed relay evidence field states at the node status boundary.
    - Fixed low-cardinality counter projection before serialization.
key-files:
  created:
    - packages/open-bitcoin-node/src/status/relay_evidence.rs
  modified:
    - packages/open-bitcoin-node/src/status.rs
    - packages/open-bitcoin-node/src/network/relay_fanout.rs
    - packages/open-bitcoin-rpc/src/context/network.rs
    - packages/open-bitcoin-rpc/src/dispatch/node.rs
    - packages/open-bitcoin-rpc/src/dispatch/tests.rs
    - packages/open-bitcoin-rpc/src/method/node.rs
key-decisions:
  - "Relay evidence is represented as typed implemented, unavailable, deferred, or intentionally_different fields with stable reasons for non-implemented states."
  - "Relay fanout, serving, and local submission records collapse to fixed counters before reaching RPC or operator-facing status."
  - "Baseline-compatible RPC methods retain their existing response shapes; Open Bitcoin-specific network status is the truth surface for relay evidence."
patterns-established:
  - "Sanitized relay evidence contract: downstream surfaces consume RelayEvidenceStatus instead of reconstructing relay state."
  - "Fixed counter projection: txids, wtxids, peers, endpoints, permissions, and free-form reasons are counted or classified before serialization."
requirements-completed:
  - OBS-01
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 105-2026-07-01T20-32-29
generated_at: 2026-07-01T23:34:18Z
duration: 1h 16m
completed: 2026-07-01
---

# Phase 105 Plan 01: Shared Relay Status And RPC Projection Summary

**Shared relay evidence status now gives RPC and later operator surfaces one sanitized source of truth with fixed outcome counters.**

## Performance

- **Duration:** 1h 16m
- **Started:** 2026-07-01T22:18:07Z
- **Completed:** 2026-07-01T23:34:18Z
- **Tasks:** 3
- **Files modified:** 16

## Accomplishments

- Added `RelayEvidenceStatus` with explicit implemented, unavailable, deferred, and intentionally different field states.
- Projected local submission, fanout, serving, cleanup, and rebroadcast-deferred records into fixed sanitized counters.
- Exposed relay evidence through `open_bitcoin_network_status` while keeping baseline-compatible RPC shapes non-promissory.
- Added focused node, RPC, CLI compatibility, and bench coverage for default, populated, and sensitive-value absence paths.

## Task Commits

Each task was committed atomically:

1. **Task 105-01-01: Define a shared relay evidence status contract** - `ea25411f` (feat)
2. **Task 105-01-02: Project relay fanout and serving state into fixed counters** - `a0be816a` (feat)
3. **Task 105-01-03: Expose relay evidence through Open Bitcoin-specific RPC** - `b0e24b28` (feat)

## Files Created/Modified

- `packages/open-bitcoin-node/src/status/relay_evidence.rs` - Defines the sanitized relay evidence contract, field states, reasons, and fixed counters.
- `packages/open-bitcoin-node/src/status.rs` - Publishes the relay evidence module from the node status boundary.
- `packages/open-bitcoin-node/src/status/tests.rs` - Covers serialization, default unavailable/deferred states, and forbidden sensitive fields.
- `packages/open-bitcoin-node/src/network/relay_fanout.rs` - Projects local relay submission, fanout, serving, cleanup, and rebroadcast evidence into fixed counters.
- `packages/open-bitcoin-node/src/network/tests/relay_fanout_cases.rs` - Covers fanout counter projection.
- `packages/open-bitcoin-node/src/network/tests/relay_local_submission_cases.rs` - Covers accepted, rejected, orphaned, and deferred local submission projection.
- `packages/open-bitcoin-node/src/network/tests/relay_serving_cases.rs` - Covers requested, served, rejected, evicted, and expired serving projection.
- `packages/open-bitcoin-rpc/src/context/network.rs` - Exposes relay evidence status from the RPC network context.
- `packages/open-bitcoin-rpc/src/dispatch/node.rs` - Threads relay evidence into `open_bitcoin_network_status`.
- `packages/open-bitcoin-rpc/src/method/node.rs` - Adds the defaulted relay evidence field to the Open Bitcoin network status response.
- `packages/open-bitcoin-rpc/src/dispatch/tests.rs` - Proves Open Bitcoin status includes sanitized relay evidence and baseline-compatible methods omit Open Bitcoin-only relay details.
- `packages/open-bitcoin-cli/src/operator/status.rs` - Keeps offline/fallback status construction compatible with the new shared response field.
- `packages/open-bitcoin-cli/src/operator/status/tests.rs` - Updates status response fixtures for the shared relay field.
- `packages/open-bitcoin-bench/src/cases/operator_runtime.rs` - Keeps benchmark status fixtures compatible with the shared relay field.
- `docs/parity/source-breadcrumbs.json` - Records the new first-party relay evidence source file breadcrumb.
- `docs/metrics/lines-of-code.md` - Regenerated tracked LOC metrics after Rust source changes.

## Verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node relay_evidence_status -- --nocapture`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node relay -- --nocapture`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc open_bitcoin_network_status -- --nocapture`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc sendrawtransaction -- --nocapture`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc node_info_methods -- --nocapture`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli fake_live_rpc_maps_metrics_from_open_bitcoin_network_status -- --nocapture`
- `cargo fmt --manifest-path packages/Cargo.toml --all`
- `cargo clippy --manifest-path packages/Cargo.toml --all-targets --all-features -- -D warnings`
- `cargo build --manifest-path packages/Cargo.toml --all-targets --all-features`
- `cargo test --manifest-path packages/Cargo.toml --all-features`
- Pre-commit hook via each task commit, including `bash scripts/verify.sh`, passed.

## Decisions Made

- The shared contract lives in `packages/open-bitcoin-node/src/status/relay_evidence.rs` to keep `status.rs` below the file-length threshold and preserve the node status boundary.
- Non-implemented relay capabilities are explicit states with stable reasons instead of omitted fields.
- Dynamic relay records are collapsed into fixed counters before crossing into RPC, CLI fixtures, metrics, logs, or support surfaces.
- `open_bitcoin_network_status` is the Open Bitcoin truth surface for relay evidence; `sendrawtransaction`, `getmempoolinfo`, and `getnetworkinfo` keep baseline-compatible response shapes.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Updated dependent status fixtures outside the RPC task file list**
- **Found during:** Task 105-01-03 (Expose relay evidence through Open Bitcoin-specific RPC)
- **Issue:** Adding a defaulted `relay` response field still required Rust struct literals in CLI and bench tests to provide the field at compile time.
- **Fix:** Added `RelayEvidenceStatus::default()` to affected CLI status and operator-runtime benchmark fixtures.
- **Files modified:** `packages/open-bitcoin-cli/src/operator/status.rs`, `packages/open-bitcoin-cli/src/operator/status/tests.rs`, `packages/open-bitcoin-bench/src/cases/operator_runtime.rs`
- **Verification:** Focused CLI fixture test, full clippy, full build, full test suite, and pre-commit hook passed.
- **Committed in:** `b0e24b28`

**Total deviations:** 1 auto-fixed (1 blocking)
**Impact on plan:** The fixture updates were necessary fallout from the shared response contract and did not broaden the operator-facing behavior beyond the plan.

## Issues Encountered

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Plan 105-02 can consume `RelayEvidenceStatus` directly for CLI status, dashboard rows, metrics, and structured logs. The fixed counter names and typed state values are in place; downstream work should avoid reconstructing relay evidence from lower-level fanout or serving records.

*Phase: 105-operator-rpc-metrics-logs-and-support-evidence*
*Completed: 2026-07-01*
