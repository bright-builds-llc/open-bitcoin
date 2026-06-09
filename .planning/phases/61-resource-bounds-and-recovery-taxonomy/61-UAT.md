---
status: complete
phase: 61-resource-bounds-and-recovery-taxonomy
source:
  - 61-01-SUMMARY.md
  - 61-02-SUMMARY.md
  - 61-03-SUMMARY.md
  - 61-04-SUMMARY.md
  - 61-05-SUMMARY.md
  - 61-06-SUMMARY.md
started: 2026-06-09T09:41:29.000Z
updated: 2026-06-09T09:41:29.000Z
---

## Current Test

[testing complete]

## Tests

### 1. Typed recovery category status contract
expected: Status snapshots should expose `sync.recovery_category` as a typed stable machine label beside human `sync.recovery_action`, preserve older metadata with an unavailable default, and register the new status recovery module in parity breadcrumbs.
result: pass

### 2. Storage and sync recovery taxonomy mapping
expected: Storage actions/errors, peer failures, stop reasons, runtime errors, and error details should map into shared `SyncRecoveryCategory` labels with storage-first precedence and boundary-aware lock-contention classification.
result: pass

### 3. Runtime projection and bounded resource evidence
expected: Durable sync state, sync summaries, structured logs, and repeated unattended-cycle tests should project recovery categories while proving resource pressure caps, endpoint-keyed retry state, synchronous durable writes, and bounded metric/log retention.
result: pass

### 4. Live-smoke and support evidence redaction
expected: Opt-in live-smoke reports should use Phase 61 recovery labels and compact `recoveryCategory`/`resourcePressure` facts, while support bundles only retain allowlisted recovery/resource evidence and do not add public-network live smoke to default verification.
result: pass

### 5. Operator status, dashboard, and RPC rendering
expected: Human status output, dashboard recovery rows, and `getblockchaininfo` durable warnings should surface the shared recovery category label while preserving separate human recovery guidance and bounded last-error detail.
result: pass

### 6. Documentation and deterministic verification boundary
expected: Operator and architecture docs should list the Phase 61 recovery labels, resource-pressure fields, RR-01 resource-bound statements, and repo-local commands; `scripts/verify.sh` should run the Phase 61 checker and exclude public-network live-smoke, manual-peer, and restart-after-progress commands.
result: pass

## Summary

total: 6
passed: 6
issues: 0
pending: 0
skipped: 0
blocked: 0

## Gaps

[]

## Evidence

- Test 1 passed by repo inspection of `packages/open-bitcoin-node/src/status.rs`, `packages/open-bitcoin-node/src/status/recovery.rs`, `packages/open-bitcoin-node/src/sync/types/summary.rs`, status/dashboard/RPC fixture updates, and `docs/parity/source-breadcrumbs.json`.
- Test 2 passed by repo inspection of `packages/open-bitcoin-node/src/storage.rs`, `packages/open-bitcoin-node/src/sync/types/recovery.rs`, `packages/open-bitcoin-node/src/sync/runtime_state.rs`, and the Phase 61 Plan 02 verification record.
- Test 3 passed by repo inspection of `packages/open-bitcoin-node/src/sync/types/summary.rs`, `packages/open-bitcoin-node/src/sync/runtime_state.rs`, `packages/open-bitcoin-node/src/sync/tests.rs`, and the `bounded_unattended_cycles_preserve_resource_pressure_and_retention` evidence.
- Test 4 passed by repo inspection of `scripts/run-live-mainnet-smoke.ts`, `scripts/test-run-live-mainnet-smoke.sh`, `packages/open-bitcoin-cli/src/operator/support/live_smoke.rs`, `packages/open-bitcoin-cli/src/operator/support/render.rs`, and `packages/open-bitcoin-cli/tests/operator_binary.rs`.
- Test 5 passed by repo inspection of `packages/open-bitcoin-cli/src/operator/status/render.rs`, `packages/open-bitcoin-cli/src/operator/dashboard/model.rs`, `packages/open-bitcoin-rpc/src/dispatch/node.rs`, and their focused tests.
- Test 6 passed with `bun run scripts/check-phase61-resource-recovery-boundaries.ts`, which printed `validated Phase 61 resource/recovery boundaries`.
- `scripts/verify.sh` contains no `run-live-mainnet-smoke`, `--manual-peer`, or `--restart-after-progress` default-verification commands.
