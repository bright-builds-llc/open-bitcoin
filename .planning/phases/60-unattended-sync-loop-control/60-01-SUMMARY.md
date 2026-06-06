---
phase: 60-unattended-sync-loop-control
plan: 01
subsystem: daemon-sync-loop-control
tags: [rust, daemon, sync, observability, docs]

requires:
  - phase: 59-operator-evidence-threat-model-and-release-boundaries
    provides: v1.4 scoped operator evidence and release-boundary posture
provides:
  - LOOP-01 explicit opt-in unattended review loop after RPC bind
  - LOOP-02 durable stop reasons for target, no-progress, max-rounds, pause, shutdown, and failure states
  - LOOP-03 bounded retry/backoff no-credit evidence for failed and waiting peers
  - LOOP-04 pause/resume/shutdown durable state preservation
affects: [open-bitcoind, sync-runtime, operator-docs, parity-breadcrumbs]

tech-stack:
  added: []
  patterns:
    - Finite daemon loop policy helper keeps sleep and shutdown behavior testable outside Axum
    - Additive sync stop reasons flow through durable status, phase projection, and structured logs

key-files:
  created:
    - packages/open-bitcoin-rpc/src/bin/tests.rs
    - .planning/phases/60-unattended-sync-loop-control/60-01-SUMMARY.md
  modified:
    - packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs
    - packages/open-bitcoin-rpc/Cargo.toml
    - packages/Cargo.lock
    - packages/open-bitcoin-node/src/sync/types.rs
    - packages/open-bitcoin-node/src/sync/types/projection.rs
    - packages/open-bitcoin-node/src/sync/types/summary.rs
    - packages/open-bitcoin-node/src/sync/tests.rs
    - docs/operator/runtime-guide.md
    - docs/parity/source-breadcrumbs.json

key-decisions:
  - "Kept `mainnet-ibd` as the explicit opt-in activation surface instead of adding a second unattended flag."
  - "Added `operator_paused` and `shutdown_requested` as additive stop reasons while preserving existing target/no-progress/max-round labels."
  - "Moved open-bitcoind tests into `src/bin/tests.rs` to keep the production binary below the repo file-length guard."
  - "Enabled Tokio's `signal` feature so the daemon can request worker shutdown on Ctrl+C graceful shutdown."

patterns-established:
  - "Daemon sync loop cycles return a typed decision and never sleep inside the testable helper."
  - "Pause/shutdown stop reasons are persisted through durable sync state and health signals."

requirements-completed: [LOOP-01, LOOP-02, LOOP-03, LOOP-04]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 60-2026-06-06T03-04-15
generated_at: 2026-06-06T03:13:31Z

duration: 45min
completed: 2026-06-06
---

# Phase 60 Plan 01: Unattended Sync Loop Control Summary

`open-bitcoind` now has an explicit bounded unattended review loop around the
existing opt-in mainnet sync worker.

## Accomplishments

- Added `DaemonSyncLoopPolicy` and `DaemonSyncLoopDecision` so each daemon wake
  runs one finite sync decision, returns a typed sleep/stop/failure outcome, and
  keeps `thread::sleep` outside the testable helper.
- Added a shutdown channel for the daemon worker and wired Axum graceful shutdown
  through `tokio::signal::ctrl_c()`. Shutdown persists lifecycle `stopped` and
  the `shutdown_requested` stop reason before the worker exits.
- Added additive `SyncStopReason::OperatorPaused` and
  `SyncStopReason::ShutdownRequested` labels, messages, health signals, and
  phase projections.
- Preserved existing target, no-progress, max-round, peer retry/backoff, and
  no-credit behavior while making the planned `retry_backoff` test filter
  exercise real assertions.
- Moved `open-bitcoind` tests into `packages/open-bitcoin-rpc/src/bin/tests.rs`
  and added the file to `docs/parity/source-breadcrumbs.json`.
- Documented the unattended review loop policy in
  `docs/operator/runtime-guide.md` with activation, stop reasons, backoff,
  pause/resume/shutdown, and non-production boundaries.

## Deviations from Plan

- The `open-bitcoind` test split was required after `bash scripts/check-file-lengths.sh`
  caught the binary over the 628-line production limit.
- The planned `cargo fmt --all` command is invalid from the repo root because
  the Cargo workspace is under `packages/Cargo.toml`; the repo-local equivalent
  is `cargo fmt --manifest-path packages/Cargo.toml --all`.

## Verification

Passed:

- `cargo fmt --manifest-path packages/Cargo.toml --all`
- `bash scripts/check-file-lengths.sh`
- `bun run scripts/check-parity-breadcrumbs.ts --check`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --bin open-bitcoind daemon_sync_loop --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node stop_reason --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node retry_backoff --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node sync_until_idle --all-features`
- `rg -n "Unattended review loop policy|unattended review loop|operator_paused|shutdown_requested|max\\(sync\\.retry_backoff_ms, 1000ms\\)|not a production-node" docs/operator/runtime-guide.md`
- `! rg -n "run-live-mainnet-smoke|--manual-peer|--restart-after-progress" scripts/verify.sh`

Full workspace pre-commit and repo verification are recorded in the phase-level
verification report.

## Known Stubs

None.

## Threat Flags

None. The loop helper persists pause, shutdown, failure, and bounded-cycle state
through existing durable metadata. Public-network long-run checks remain opt-in
UAT and are not part of default verification.

## User Setup Required

None for deterministic verification. Operators still need explicit
`mainnet-ibd` activation and a datadir for opt-in public-mainnet review.

## Next Phase Readiness

Phase 61 can build on explicit loop stop reasons and retry/backoff behavior to
expand resource-bound and recovery taxonomy.

## Self-Check: PASSED

- Confirmed lifecycle validation passed before execution.
- Confirmed targeted plan checks passed.
- Confirmed production file-length and parity-breadcrumb guards passed after the
  test split.
- Deferred commit and push to the final strict wrapper gate.

---
*Phase: 60-unattended-sync-loop-control*
*Completed: 2026-06-06*
