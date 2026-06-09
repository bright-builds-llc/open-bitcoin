---
status: complete
phase: 62-long-run-sync-truth-surfaces
source:
  - 62-01-SUMMARY.md
  - 62-02-SUMMARY.md
  - 62-03-SUMMARY.md
  - 62-04-SUMMARY.md
started: 2026-06-09T09:47:54.000Z
updated: 2026-06-09T09:47:54.000Z
---

## Current Test

[testing complete]

## Tests

### 1. Shared Phase 62 sync truth contract
expected: Status snapshots and durable sync state should expose configured targets, attempt counters, latest stop reason, progress signal, recovery category/action, resource pressure, peer/block evidence, and explicit unavailable reasons from typed fields rather than parsing human output.
result: pass

### 2. Terminal, dashboard, and RPC truth surfaces
expected: `open-bitcoin status`, `open-bitcoin dashboard`, `open-bitcoin sync status`, and RPC warnings should render the same Phase 62 truth fields in the documented order, preserve `Unavailable: {reason}`, and keep RPC warning labels compact and deterministic.
result: pass

### 3. Live-smoke truth reports
expected: Deterministic live-smoke JSON and Markdown reports should project configured targets, attempt counters, progress signal, latest stop reason, recovery fields, resource pressure, and block evidence from typed durable status while generated reports avoid persisted raw daemon output tails.
result: pass

### 4. Operator documentation and UI-spec contract
expected: Operator and architecture docs plus the Phase 62 UI design contract should document terminal/operator truth surfaces, field order, unavailable semantics, bounded numeric metrics, compact structured-log labels, and no hosted GUI/dashboard expansion.
result: pass

### 5. Deterministic verification boundary
expected: `scripts/check-phase62-sync-truth-surfaces.ts` should validate Rust, TypeScript, docs, fixture coverage, and `scripts/verify.sh`; default verification should run the checker and should not run public-network live smoke, manual-peer probing, or restart-after-progress commands.
result: pass

## Summary

total: 5
passed: 5
issues: 0
pending: 0
skipped: 0
blocked: 0

## Gaps

[]

## Evidence

- Test 1 passed by repo inspection of `packages/open-bitcoin-node/src/status.rs`, `packages/open-bitcoin-node/src/status/tests.rs`, `packages/open-bitcoin-node/src/sync/types/summary.rs`, `packages/open-bitcoin-node/src/sync/runtime_state.rs`, and Phase 62 Plan 01 verification.
- Test 2 passed by repo inspection of `packages/open-bitcoin-cli/src/operator/status/render.rs`, `packages/open-bitcoin-cli/src/operator/dashboard/model.rs`, `packages/open-bitcoin-cli/src/operator/runtime/support.rs`, `packages/open-bitcoin-rpc/src/dispatch/node.rs`, and focused Phase 62 CLI/RPC tests.
- Test 3 passed by repo inspection of `scripts/run-live-mainnet-smoke.ts`, `scripts/test-run-live-mainnet-smoke.sh`, and generated-report fixture assertions in Phase 62 Plan 03 verification.
- Test 4 passed by repo inspection of `docs/operator/runtime-guide.md`, `docs/architecture/status-snapshot.md`, `docs/architecture/operator-observability.md`, and `.planning/phases/62-long-run-sync-truth-surfaces/62-UI-SPEC.md`.
- Test 5 passed with `bun run scripts/check-phase62-sync-truth-surfaces.ts`, which printed `validated Phase 62 sync truth surfaces`.
- `scripts/verify.sh` contains no `run-live-mainnet-smoke`, `--manual-peer`, or `--restart-after-progress` default-verification commands.
