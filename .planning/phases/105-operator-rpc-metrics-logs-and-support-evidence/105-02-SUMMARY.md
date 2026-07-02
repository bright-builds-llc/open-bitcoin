---
phase: 105-operator-rpc-metrics-logs-and-support-evidence
plan: 105-02
subsystem: operator-telemetry
tags:
  - rust
  - relay-evidence
  - operator-cli
  - metrics
  - logging
requires:
  - 105-01
provides:
  - Operator CLI and dashboard relay evidence rendering from the shared status contract.
  - Fixed low-cardinality relay metric samples for accepted, rejected, orphaned, requested, served, announced, suppressed, evicted, expired, and rebroadcast-deferred outcomes.
  - Sanitized structured relay outcome log records with stable source and fixed fields.
affects:
  - operator-status
  - dashboard
  - metrics
  - logging
  - support-bundles
tech-stack:
  added: []
  patterns:
    - Shared relay evidence status is projected once into CLI, dashboard, metrics, and logs.
    - Telemetry uses fixed names and counter fields instead of dynamic labels.
key-files:
  created:
    - packages/open-bitcoin-cli/src/operator/dashboard/model/relay.rs
    - packages/open-bitcoin-cli/src/operator/status/render/relay.rs
  modified:
    - packages/open-bitcoin-cli/src/operator/status.rs
    - packages/open-bitcoin-cli/src/operator/status/render.rs
    - packages/open-bitcoin-cli/src/operator/status/render/tests.rs
    - packages/open-bitcoin-cli/src/operator/status/tests.rs
    - packages/open-bitcoin-cli/src/operator/dashboard/model.rs
    - packages/open-bitcoin-cli/src/operator/dashboard/model/metrics.rs
    - packages/open-bitcoin-cli/src/operator/dashboard/model/tests.rs
    - packages/open-bitcoin-cli/src/operator/support/tests.rs
    - packages/open-bitcoin-cli/src/operator/soak/tests/runtime.rs
    - packages/open-bitcoin-node/src/lib.rs
    - packages/open-bitcoin-node/src/status.rs
    - packages/open-bitcoin-node/src/status/tests.rs
    - packages/open-bitcoin-node/src/metrics.rs
    - packages/open-bitcoin-node/src/metrics/tests.rs
    - packages/open-bitcoin-node/src/logging.rs
    - packages/open-bitcoin-node/src/logging/tests.rs
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/inbound_metrics.rs
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests.rs
    - docs/parity/source-breadcrumbs.json
    - docs/metrics/lines-of-code.md
key-decisions:
  - "Operator status JSON exposes the shared sanitized relay shape under mempool.relay instead of inventing a CLI-only schema."
  - "Human status and dashboard rows render implemented, unavailable, deferred, and intentionally different relay fields from the shared status contract."
  - "Relay metrics and structured logs use fixed counter names and omit transaction identifiers, peer endpoints, permission strings, credentials, free-form reasons, and dynamic labels."
patterns-established:
  - "Operator relay rendering split into focused child modules to keep production files below the repo file-length guard."
  - "Daemon metric persistence samples inbound and relay metrics together after copying sanitized status under the context lock."
requirements-completed:
  - OBS-02
  - OBS-03
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 105-2026-07-01T20-32-29
generated_at: 2026-07-02T00:49:59Z
duration: 1h 7m
completed: 2026-07-01
---

# Phase 105 Plan 02: CLI, Dashboard, Metrics, And Logs Summary

**Operator telemetry now renders and records relay evidence from the shared sanitized status contract without adding dynamic metric labels or sensitive relay material.**

## Performance

- **Duration:** 1h 7m
- **Started:** 2026-07-01T23:35:49Z
- **Completed:** 2026-07-02T00:41:17Z
- **Tasks:** 4
- **Files modified:** 22

## Accomplishments

- Added operator status human rendering for relay evidence, mempool evidence, local submission, fanout, serving, deferred rebroadcast, and public-relay state.
- Exposed the shared sanitized relay evidence shape in operator status JSON under `mempool.relay`.
- Added compact dashboard rows for relay and mempool evidence, including implemented counters and explicit unavailable/deferred states.
- Added fixed relay metric kinds and samples for accepted, rejected, orphaned, requested, served, announced, suppressed, evicted, expired, and rebroadcast-deferred outcomes.
- Added sanitized structured relay log records with the stable `relay_mempool` source and fixed count-only messages.
- Wired daemon metric persistence to collect both inbound and relay samples from sanitized status.

## Task Commits

Plan 105-02 was committed as one verification-backed implementation commit:

1. **Tasks 105-02-01 through 105-02-04: Operator status, dashboard, metrics, and logs** - `29493d67` (feat)

## Files Created/Modified

- `packages/open-bitcoin-cli/src/operator/status.rs` - Carries relay evidence through the shared operator status snapshot.
- `packages/open-bitcoin-cli/src/operator/status/render.rs` - Delegates relay evidence rendering to a focused child module.
- `packages/open-bitcoin-cli/src/operator/status/render/relay.rs` - Renders human relay evidence lines from shared sanitized field states and counters.
- `packages/open-bitcoin-cli/src/operator/status/render/tests.rs` - Keeps render test exports aligned with the relay rendering split.
- `packages/open-bitcoin-cli/src/operator/status/tests.rs` - Covers human and JSON relay evidence output plus sensitive-value absence.
- `packages/open-bitcoin-cli/src/operator/dashboard/model.rs` - Adds relay and mempool evidence rows to the dashboard model.
- `packages/open-bitcoin-cli/src/operator/dashboard/model/metrics.rs` - Extends bounded dashboard metric candidates with fixed relay metric kinds.
- `packages/open-bitcoin-cli/src/operator/dashboard/model/relay.rs` - Builds compact dashboard relay evidence rows from the shared status contract.
- `packages/open-bitcoin-cli/src/operator/dashboard/model/tests.rs` - Covers implemented, unavailable, deferred, and sensitive-absence dashboard cases.
- `packages/open-bitcoin-cli/src/operator/support/tests.rs` - Keeps support test fixtures compatible with the shared relay field.
- `packages/open-bitcoin-cli/src/operator/soak/tests/runtime.rs` - Keeps soak fixture construction compatible with the shared relay field.
- `packages/open-bitcoin-node/src/lib.rs` - Exports relay metric sampling for daemon integration.
- `packages/open-bitcoin-node/src/status.rs` - Projects network relay evidence into the mempool status section.
- `packages/open-bitcoin-node/src/status/tests.rs` - Updates status fixture coverage for default relay evidence.
- `packages/open-bitcoin-node/src/metrics.rs` - Adds fixed relay metric kinds and sample projection.
- `packages/open-bitcoin-node/src/metrics/tests.rs` - Proves relay metrics use fixed names without dynamic labels or sensitive strings.
- `packages/open-bitcoin-node/src/logging.rs` - Adds sanitized structured relay outcome log records.
- `packages/open-bitcoin-node/src/logging/tests.rs` - Proves relay logs keep fixed outcome fields and omit sensitive material.
- `packages/open-bitcoin-rpc/src/bin/open_bitcoind/inbound_metrics.rs` - Persists relay metric samples alongside inbound samples from sanitized status.
- `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests.rs` - Covers daemon relay metric persistence.
- `docs/parity/source-breadcrumbs.json` - Records the new first-party relay renderer/model modules.
- `docs/metrics/lines-of-code.md` - Regenerated tracked LOC metrics after Rust source changes.

## Commands Run

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli operator_status -- --nocapture`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli dashboard_model -- --nocapture`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node metrics -- --nocapture`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node logging -- --nocapture`
- `cargo fmt --manifest-path packages/Cargo.toml --all`
- `cargo clippy --manifest-path packages/Cargo.toml --all-targets --all-features -- -D warnings`
- `cargo build --manifest-path packages/Cargo.toml --all-targets --all-features`
- `cargo test --manifest-path packages/Cargo.toml --all-features`
- `bun run scripts/check-parity-breadcrumbs.ts --check`
- `bash scripts/check-file-lengths.sh`
- `bun run scripts/check-phase97-inbound-metrics.ts`
- `git diff --check`
- Pre-commit hook via `git commit`, including `bash scripts/verify.sh`, passed.

## Decisions Made

- Relay evidence stays nested under `mempool.relay` in operator status because mempool participation is the operator-facing context for accepted, rejected, orphaned, serving, cleanup, and rebroadcast evidence.
- Dashboard relay rows share the CLI status vocabulary instead of adding dashboard-only names or states.
- Metrics and logs derive from `RelayEvidenceStatus` rather than lower-level fanout or serving internals, preserving the Phase 105 source-of-truth boundary.
- New renderer/model helper modules keep `status/render.rs`, `dashboard/model.rs`, and `status.rs` under the repo file-length guard.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Preserved Phase 97 dashboard checker compatibility while extending metric candidates**
- **Found during:** Task 105-02-02 (Render relay evidence in the terminal dashboard model)
- **Issue:** The Phase 97 checker expects the dashboard helper name `retained_inbound_metric_kinds`; renaming the helper while adding relay metrics broke that deterministic guard.
- **Fix:** Restored the existing helper name and chained fixed relay metric candidates through it.
- **Files modified:** `packages/open-bitcoin-cli/src/operator/dashboard/model/metrics.rs`
- **Verification:** `bun run scripts/check-phase97-inbound-metrics.ts`, full cargo verification, and the pre-commit hook passed.
- **Committed in:** `29493d67`

**2. [Rule 3 - Blocking] Split relay helpers to satisfy the production file-length guard**
- **Found during:** Task 105-02-01 and Task 105-02-02 (operator status and dashboard rendering)
- **Issue:** Adding relay rendering pushed `status/render.rs`, `dashboard/model.rs`, and `status.rs` over the repo production file-length limit.
- **Fix:** Moved relay status rendering and dashboard row construction into focused child modules and trimmed incidental blank lines.
- **Files modified:** `packages/open-bitcoin-cli/src/operator/status/render/relay.rs`, `packages/open-bitcoin-cli/src/operator/dashboard/model/relay.rs`, `packages/open-bitcoin-node/src/status.rs`
- **Verification:** `bash scripts/check-file-lengths.sh`, full cargo verification, and the pre-commit hook passed.
- **Committed in:** `29493d67`

**Total deviations:** 2 auto-fixed (2 blocking)
**Impact on plan:** Both fixes preserved the planned behavior while keeping existing deterministic guardrails green.

## Issues Encountered

- A full `cargo test --manifest-path packages/Cargo.toml --all-features` run had one transient stdin CLI test failure; the focused rerun passed and later full test runs passed.

## Self-Check

- Complete: OBS-02 and OBS-03 evidence is implemented, tested, summarized, and committed in `29493d67`.
- Passed: focused operator status, dashboard, metrics, logging, full cargo verification, parity breadcrumb, file-length, Phase 97 checker, diff whitespace, and hook-backed repository verification all passed.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Plan 105-03 can consume the same `mempool.relay` snapshot for support JSON and Markdown. The fixed metric/log outcome names are in place; support work should focus on stricter support-bundle sanitization and shared projection rather than reconstructing lower-level relay evidence.

*Phase: 105-operator-rpc-metrics-logs-and-support-evidence*
*Completed: 2026-07-01*
