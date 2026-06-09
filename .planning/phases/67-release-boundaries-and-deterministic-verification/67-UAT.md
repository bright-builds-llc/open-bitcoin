---
status: complete
phase: 67-release-boundaries-and-deterministic-verification
source:
  - 67-01-SUMMARY.md
  - 67-VERIFICATION.md
started: 2026-06-09T14:33:21.000Z
updated: 2026-06-09T14:33:21.000Z
---

## Current Test

[testing complete]

## Tests

### 1. v1.5 threat model and release-boundary matrix
expected: `docs/parity/threat-model-v1.5.md` and `docs/parity/release-readiness.md` should document REL-01 through REL-04 with STRIDE, ASVS L1, evidence acceptance, residual risks, and a release-boundary matrix for source-built explicit opt-in unattended mainnet operator review readiness.
result: pass

### 2. Current parity roots and historical evidence boundary
expected: `docs/parity/index.json`, `docs/parity/checklist.md`, `docs/parity/README.md`, `docs/parity/catalog/p2p.md`, and `docs/parity/deviations-and-unknowns.md` should make `v1-5-unattended-operation-release-boundaries` the current v1.5 closeout surface while keeping v1.3 and v1.4 evidence historical.
result: pass

### 3. Explicit non-claims and operator-facing wording
expected: Parity docs and `docs/operator/runtime-guide.md` should distinguish v1.5 operator-review readiness from production-node readiness, production-funds wallet use, inbound serving, transaction relay, compact block relay, migration apply, packaging, hosted dashboard, GUI, Windows service support, public-network CI, and other deferred surfaces.
result: pass

### 4. Deterministic verification guard
expected: `scripts/check-v1.5-release-boundaries.ts` should validate required v1.5 roots, REL ids, evidence paths, release-boundary wording, runtime-guide wording, and forbidden default-verification commands; `scripts/verify.sh` should run the checker without invoking public-network live smoke, manual peers, restart-after-progress, `systemctl --user`, or `launchctl`.
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

- Test 1 passed by repo inspection of `docs/parity/threat-model-v1.5.md`, `docs/parity/release-readiness.md`, and `.planning/phases/67-release-boundaries-and-deterministic-verification/67-VERIFICATION.md`.
- Test 2 passed by repo inspection of `docs/parity/index.json`, `docs/parity/checklist.md`, `docs/parity/README.md`, `docs/parity/catalog/p2p.md`, and `docs/parity/deviations-and-unknowns.md`.
- Test 3 passed by repo inspection of `docs/parity/release-readiness.md`, `docs/parity/catalog/p2p.md`, `docs/parity/deviations-and-unknowns.md`, and `docs/operator/runtime-guide.md`.
- Test 4 passed with `bun run scripts/check-v1.5-release-boundaries.ts`, which printed `validated v1.5 release boundary parity roots`.
- `scripts/verify.sh` contains `bun run scripts/check-v1.5-release-boundaries.ts` and contains no `run-live-mainnet-smoke`, `--manual-peer`, `--restart-after-progress`, `systemctl --user`, or `launchctl` default-verification commands.
