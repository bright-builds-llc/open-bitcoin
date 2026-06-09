---
status: complete
phase: 65-support-bundle-and-operator-review-docs
source:
  - 65-01-SUMMARY.md
  - 65-02-SUMMARY.md
started: 2026-06-09T13:41:07.000Z
updated: 2026-06-09T13:41:07.000Z
---

## Current Test

[testing complete]

## Tests

### 1. Redacted support bundle service evidence
expected: `support-evidence.md` should include compact labels for service lifecycle, service restart/resume, log path, metrics availability, and metrics samples, while `support-evidence.json` keeps service restart/resume evidence in the shared status snapshot and omits raw secrets or raw live-smoke details.
result: pass

### 2. v1.5 operator review runbook
expected: `docs/operator/runtime-guide.md` should document the v1.5 operator review sequence with repo-local Cargo and Bazel commands for deterministic checks, sync status, status snapshot, service status/restart, support bundle collection, and optional live-smoke attachment.
result: pass

### 3. Support evidence interpretation and parity boundary
expected: Operator and parity docs should tell reviewers to interpret `support-evidence.json`, `support-evidence.md`, `live_smoke.summary.finalStatus`, `restartResumeEvidence`, `status.service.restart_resume`, `status.metrics`, and `status.logs` as local redacted review evidence, not proof of sync success, production-node readiness, or a service guarantee.
result: pass

### 4. Deterministic verification boundary
expected: `scripts/check-phase65-support-review.ts` should validate support source/tests, docs, parity wording, and default-verification exclusions; `scripts/verify.sh` should run that checker without invoking public-network live smoke, manual peers, restart-after-progress, `systemctl --user`, or `launchctl`.
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

- Test 1 passed by repo inspection of `packages/open-bitcoin-cli/src/operator/support/render.rs` and `packages/open-bitcoin-cli/tests/operator_binary.rs`.
- Test 1 also passed with `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli open_bitcoin_support_bundle --all-features`, which ran 5 matching support-bundle tests successfully.
- Test 2 passed by repo inspection of `docs/operator/runtime-guide.md`.
- Test 3 passed by repo inspection of `docs/operator/runtime-guide.md`, `docs/architecture/operator-observability.md`, and `docs/parity/catalog/p2p.md`.
- Test 4 passed with `bun run scripts/check-phase65-support-review.ts`, which printed `Phase 65 support review checks passed.`
- `scripts/verify.sh` contains `bun run scripts/check-phase65-support-review.ts` and contains no `run-live-mainnet-smoke`, `--manual-peer`, `--restart-after-progress`, `systemctl --user`, or `launchctl` default-verification commands.
