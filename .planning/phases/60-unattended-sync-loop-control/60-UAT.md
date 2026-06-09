---
status: complete
phase: 60-unattended-sync-loop-control
source:
  - 60-01-SUMMARY.md
started: 2026-06-09T09:12:10.000Z
updated: 2026-06-09T09:34:08.000Z
---

## Current Test

[testing complete]

## Tests

### 1. Explicit opt-in daemon review loop
expected: Review `docs/operator/runtime-guide.md` and the Phase 60 summary. They should show that `open-bitcoind` only enters the bounded unattended mainnet review loop through explicit `mainnet-ibd` activation, and that the loop is documented as an operator-review workflow rather than production-node readiness.
result: pass

### 2. Graceful shutdown stop reason
expected: The Phase 60 evidence should show daemon graceful shutdown flows through a shutdown channel and persists lifecycle `stopped` with the `shutdown_requested` stop reason before the worker exits.
result: pass

### 3. Pause and shutdown status vocabulary
expected: The Phase 60 evidence should show additive `operator_paused` and `shutdown_requested` stop reasons with messages, health signals, and phase/status projections while preserving target, no-progress, max-round, retry/backoff, and no-credit behavior.
result: pass

### 4. Deterministic verification boundary
expected: Phase 60 verification and docs should show repo-local deterministic commands for loop behavior, file-length, parity breadcrumbs, and status wording, while `scripts/verify.sh` does not include public-network live smoke, manual-peer probing, or restart-after-progress commands.
result: pass

## Summary

total: 4
passed: 4
issues: 0
pending: 0
skipped: 0
blocked: 0

## Gaps

[]

## Evidence

- Test 1 passed by user confirmation.
- Tests 2-4 passed by repo inspection of `.planning/phases/60-unattended-sync-loop-control/60-01-SUMMARY.md`, `.planning/phases/60-unattended-sync-loop-control/60-VERIFICATION.md`, `docs/operator/runtime-guide.md`, `packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs`, `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests.rs`, `packages/open-bitcoin-node/src/sync/types.rs`, `packages/open-bitcoin-node/src/sync/types/projection.rs`, `packages/open-bitcoin-node/src/sync/types/summary.rs`, and `scripts/verify.sh`.
- `scripts/verify.sh` contains no `run-live-mainnet-smoke`, `--manual-peer`, or `--restart-after-progress` default-verification commands.
