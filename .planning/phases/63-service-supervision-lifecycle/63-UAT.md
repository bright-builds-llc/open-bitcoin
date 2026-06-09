---
status: complete
phase: 63-service-supervision-lifecycle
source:
  - 63-01-SUMMARY.md
  - 63-02-SUMMARY.md
  - 63-03-SUMMARY.md
  - 63-04-SUMMARY.md
started: 2026-06-09T12:48:30.000Z
updated: 2026-06-09T12:48:30.000Z
---

## Current Test

[testing complete]

## Tests

### 1. Side-effect-free service preview and daemon targeting
expected: `open-bitcoin service preview` should render launchd/systemd service definitions without applying side effects, reject `--apply`, and target `open-bitcoind` rather than the `open-bitcoin` operator wrapper.
result: pass

### 2. User-scope service lifecycle actions
expected: Service start, stop, and restart should route through the shared `ServiceManager` dispatcher, use user-level launchd/systemd commands only, and be available from both CLI service commands and confirmed dashboard actions.
result: pass

### 3. Shared service lifecycle status contract
expected: Direct service status, human status, JSON status, and dashboard rows should render the same shared lifecycle labels: `unmanaged`, `installed-stopped`, `running`, `failed`, `disabled`, and `unavailable-manager`, with explicit unavailable reasons instead of false, zero, or empty placeholders.
result: pass

### 4. Operator runbook and repo-local commands
expected: `docs/operator/runtime-guide.md` should document service preview/install/apply/start/status/restart/stop/disable/uninstall flows, Cargo and Bazel command forms, log/config review notes, lifecycle labels, and opt-in UAT boundaries for real service-manager actions.
result: pass

### 5. Deterministic verification boundary
expected: `scripts/check-phase63-service-lifecycle.ts` should validate source, docs, generated LOC evidence, and `scripts/verify.sh`; default verification should run the checker and should not invoke real `systemctl --user`, `launchctl`, public-network live smoke, manual-peer, or restart-after-progress commands.
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

- Test 1 passed by repo inspection of `packages/open-bitcoin-cli/src/operator.rs`, `packages/open-bitcoin-cli/src/operator/runtime.rs`, `packages/open-bitcoin-cli/src/operator/service.rs`, `packages/open-bitcoin-cli/src/operator/service/tests.rs`, and Phase 63 Plan 01 verification.
- Test 2 passed by repo inspection of `packages/open-bitcoin-cli/src/operator/service.rs`, `packages/open-bitcoin-cli/src/operator/service/launchd.rs`, `packages/open-bitcoin-cli/src/operator/service/systemd.rs`, `packages/open-bitcoin-cli/src/operator/dashboard/action.rs`, `packages/open-bitcoin-cli/src/operator/dashboard/app.rs`, and Phase 63 Plan 02 verification.
- Test 3 passed by repo inspection of `packages/open-bitcoin-node/src/status.rs`, `packages/open-bitcoin-node/src/status/tests.rs`, `packages/open-bitcoin-cli/src/operator/status/service_status.rs`, `packages/open-bitcoin-cli/src/operator/status/render.rs`, `packages/open-bitcoin-cli/src/operator/dashboard/model.rs`, and Phase 63 Plan 03 verification.
- Test 4 passed by repo inspection of `docs/operator/runtime-guide.md`.
- Test 5 passed with `bun run scripts/check-phase63-service-lifecycle.ts`, which printed `validated Phase 63 service lifecycle`.
- `scripts/verify.sh` contains no `systemctl --user start`, `systemctl --user stop`, `systemctl --user restart`, `launchctl bootstrap`, `launchctl bootout`, `launchctl kickstart`, `run-live-mainnet-smoke`, `--manual-peer`, or `--restart-after-progress` default-verification commands.
