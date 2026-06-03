---
phase: 56-header-ibd-convergence
plan: 01
subsystem: p2p-sync
tags:
  - rust
  - p2p
  - sync
  - operators
requires:
  - phase: 55-outbound-handshake-compatibility-fixes
    provides: connected outbound handshakes and typed peer failure outcomes
provides:
  - explicit header convergence stop reasons
  - target header height JSONC configuration
  - first-header-progress live-smoke evidence
  - deterministic accepted/rejected/no-progress/restart header tests
affects:
  - block-ibd-progress-evidence
  - restart-resume-status-proof
  - operator-evidence-threat-model-and-release-boundaries
tech-stack:
  added: []
  patterns:
    - bounded sync runs persist typed stop diagnoses through durable status
key-files:
  created:
    - .planning/phases/56-header-ibd-convergence/56-CONTEXT.md
    - .planning/phases/56-header-ibd-convergence/56-DISCUSSION-LOG.md
    - .planning/phases/56-header-ibd-convergence/56-RESEARCH.md
    - .planning/phases/56-header-ibd-convergence/56-01-PLAN.md
    - .planning/phases/56-header-ibd-convergence/56-REVIEW.md
    - .planning/phases/56-header-ibd-convergence/56-01-SUMMARY.md
  modified:
    - docs/metrics/lines-of-code.md
    - docs/operator/runtime-guide.md
    - docs/parity/catalog/p2p.md
    - packages/open-bitcoin-node/src/lib.rs
    - packages/open-bitcoin-node/src/sync.rs
    - packages/open-bitcoin-node/src/sync/tests.rs
    - packages/open-bitcoin-node/src/sync/types.rs
    - packages/open-bitcoin-node/src/sync/types/projection.rs
    - packages/open-bitcoin-node/src/sync/types/summary.rs
    - packages/open-bitcoin-rpc/src/config/loader/open_bitcoin_runtime.rs
    - packages/open-bitcoin-rpc/src/config/open_bitcoin.rs
    - packages/open-bitcoin-rpc/src/config/tests.rs
    - scripts/run-live-mainnet-smoke.ts
key-decisions:
  - "Use accepted header height as the sole header progress signal."
  - "Persist bounded sync stop reason as target-header reached, no progress, or max rounds."
  - "Keep live-smoke first-header-progress attribution additive and tolerate missing final peer telemetry."
patterns-established:
  - "Sync runtime convergence can be bounded by optional target header height without changing default behavior."
  - "Live evidence reports derive first progress from fresh status snapshots before correlating final peer telemetry."
requirements-completed:
  - HDR-01
  - HDR-02
  - HDR-03
  - HDR-04
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 56-2026-06-03T12-44-57
generated_at: 2026-06-03T13:05:17.692Z
duration: 20m
completed: 2026-06-03
---

# Phase 56 Plan 01 Summary

**Header sync now has deterministic bounded convergence evidence and opt-in
live-smoke first-header-progress reporting.**

## Accomplishments

- Added `sync.target_header_height` JSONC support and
  `SyncRuntimeConfig::maybe_target_header_height`.
- Added `SyncStopReason` telemetry for target reached, no progress, and max
  rounds; bounded `sync_until_idle` runs persist the stop reason through durable
  sync status health signals and phase names.
- Added live-smoke `result.firstHeaderProgress` evidence with before/after
  `openbitcoinsyncstatus` snapshots, observed timestamp, header delta, and
  peer endpoint/source when final telemetry is available.
- Added deterministic sync tests for multi-round header target convergence,
  no-progress diagnosis, rejected-header no-credit behavior, durable restart
  status, and structured stop-reason logs.
- Updated operator and parity docs while preserving public-network opt-in and
  block-progress deferral boundaries.

## Deviations from Plan

- The structured summary log line could not include stop reason and remain
  within the existing 160-character cap, so stop reason is emitted as its own
  compact structured log record.
- `bash scripts/verify.sh` initially failed on stale
  `docs/metrics/lines-of-code.md`; the repo-prescribed generator refreshed the
  tracked artifact and the full verification contract then passed.

## Verification

Passed:

```bash
cargo fmt --all --manifest-path packages/Cargo.toml
cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node sync --all-features
cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc config --all-features
bun run scripts/check-parity-breadcrumbs.ts --check
bun run scripts/run-live-mainnet-smoke.ts --help
cargo clippy --manifest-path packages/Cargo.toml --workspace --all-targets --all-features -- -D warnings
cargo build --manifest-path packages/Cargo.toml --workspace --all-targets --all-features
cargo test --manifest-path packages/Cargo.toml --workspace --all-features
bun run scripts/generate-loc-report.ts --source=worktree --output=docs/metrics/lines-of-code.md
bash scripts/verify.sh
```

The full workspace test suite passed with the expected ignored opt-in
live-network smoke test.

## Next Phase Readiness

Phase 57 can build on accepted header convergence and durable header status to
focus specifically on block download/connect progress evidence.

## Self-Check: PASSED
