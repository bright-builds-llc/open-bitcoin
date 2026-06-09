---
status: complete
phase: 64-service-restart-and-same-datadir-resume-evidence
source:
  - 64-01-SUMMARY.md
  - 64-02-SUMMARY.md
  - 64-03-SUMMARY.md
started: 2026-06-09T13:29:58.000Z
updated: 2026-06-09T13:29:58.000Z
---

## Current Test

[testing complete]

## Tests

### 1. Shared restart/resume status contract
expected: Status JSON should expose `service.restart_resume` with the selected datadir, same-datadir verdict, prior shutdown status, durable progress, stale in-flight verdict, recovery category, and next action, while missing evidence remains explicitly unavailable.
result: pass

### 2. Status, dashboard, and restart guidance rendering
expected: Human status should include `restart_resume=`, the dashboard service section should show restart/resume rows for the same evidence, and successful `service restart` output should point operators to `open-bitcoin status --format json` using the same `--datadir`.
result: pass

### 3. Same-datadir operator runbook
expected: `docs/operator/runtime-guide.md` should document same-datadir service restart review with repo-local Cargo and Bazel commands for `service status`, `service restart`, `status --format json`, and `sync status --format json`, plus interpretation guidance for `service.restart_resume` fields.
result: pass

### 4. Deterministic verification and release boundary
expected: `scripts/check-phase64-service-restart-resume.ts` should validate Phase 64 source/docs/parity/default-verification boundaries, `scripts/verify.sh` should run that checker, and default verification should not invoke opt-in live smoke or real service-manager restart commands.
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

- Test 1 passed by repo inspection of `packages/open-bitcoin-node/src/status.rs`, `packages/open-bitcoin-cli/src/operator/status/service_status.rs`, and `packages/open-bitcoin-cli/src/operator/status/tests.rs`.
- Test 2 passed by repo inspection of `packages/open-bitcoin-cli/src/operator/status/render.rs`, `packages/open-bitcoin-cli/src/operator/dashboard/model.rs`, `packages/open-bitcoin-cli/src/operator/service.rs`, and `packages/open-bitcoin-cli/src/operator/service/tests.rs`.
- Test 3 passed by repo inspection of `docs/operator/runtime-guide.md`.
- Test 4 passed with `bun run scripts/check-phase64-service-restart-resume.ts`, which printed `Phase 64 service restart/resume checks passed.`
- `scripts/verify.sh` contains `bun run scripts/check-phase64-service-restart-resume.ts` and contains no `run-live-mainnet-smoke`, `--restart-after-progress`, `systemctl --user restart`, or `launchctl kickstart` default-verification commands.
- Parity wording in `docs/parity/catalog/p2p.md` keeps service-supervised restart/resume evidence scoped to opt-in operator review rather than a production-node service guarantee.
