---
phase: 86-service-operation-expectations
status: passed
requirements:
  - SVC-01
  - SVC-02
generated_by: gsd-execute-phase
lifecycle_mode: yolo
phase_lifecycle_id: 86-2026-06-22T19-33-52
generated_at: 2026-06-22T21:12:44Z
lifecycle_validated: true
---

# Phase 86 Verification

Phase 86 passed focused verification and the repo-native verifier.

## Requirement Coverage

| Requirement | Evidence |
| --- | --- |
| SVC-01 | `docs/parity/service-operation-expectations.md`, parity roots, pointer docs, Phase 86 checker, and default verifier wiring. |
| SVC-02 | Service field rules, restart/resume evidence rules, support evidence boundaries, parity roots, Phase 86 checker, and default verifier wiring. |

## Commands Run

- `bun run scripts/generate-loc-report.ts --source=worktree --output=docs/metrics/lines-of-code.md`
- `bun run scripts/generate-loc-report.ts --source=worktree --output=docs/metrics/lines-of-code.md --check`
- `bun test scripts/check-phase86-service-operation-expectations.test.ts`
- `bun --check scripts/check-phase86-service-operation-expectations.ts`
- `bun run scripts/check-phase86-service-operation-expectations.ts`
- `rg -n "check-phase86-service-operation-expectations" docs/metrics/lines-of-code.md`
- `git diff --check -- docs/metrics/lines-of-code.md scripts/check-phase86-service-operation-expectations.ts scripts/check-phase86-service-operation-expectations.test.ts scripts/verify.sh README.md docs/operator/runtime-guide.md docs/parity/service-operation-expectations.md docs/parity/README.md docs/parity/catalog/operator-runtime-release-hardening.md docs/parity/checklist.md docs/parity/deviations-and-unknowns.md docs/parity/index.json docs/parity/operator-runbooks.md docs/parity/production-claim-boundary.md docs/parity/release-readiness.md docs/parity/support-matrix.md docs/parity/upgrade-and-rollback-policy.md`
- `bash scripts/verify.sh`
- `git diff --name-only -- 'packages/open-bitcoin-*/src' 'packages/open-bitcoin-*/tests'`

## Results

- LOC report was regenerated and passed `--check`.
- Phase 86 fixture tests passed: 11 tests, 0 failures.
- Phase 86 checker passed against the real repository state.
- `bash scripts/verify.sh` passed in 45m 21.704s.
- No first-party Rust source or test files changed, so no source breadcrumb update was needed.

## Default Verification Boundary

Default verification remained deterministic, public-network-free, real-service-manager-free, package-manager-service-free, Windows-service-free, support-upload-free, and multi-day-free.

## Residual Risk

Phase 86 records service expectation boundaries and deterministic drift guards. It does not claim packaged service support, Windows service support, automatic update behavior, production service ownership, uptime guarantees, or broad production full-node readiness.
