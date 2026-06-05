---
phase: 59-operator-evidence-threat-model-and-release-boundaries
plan: 05
subsystem: release-boundary-verification
tags: [typescript, verification, parity, release-boundaries, rust]

requires:
  - phase: 59-operator-evidence-threat-model-and-release-boundaries
    provides: SEC-01/SEC-02 v1.4 threat model and release-boundary parity roots
provides:
  - SEC-03 deterministic v1.4 release-boundary verification in the repo-native gate
  - Public-network live-smoke exclusion assertions for default verification
  - Fresh tracked LOC evidence after Phase 59 source and script changes
affects: [verify-script, parity-checkers, loc-report, dashboard-model-tests]

tech-stack:
  added: []
  patterns:
    - Bun TypeScript checker mirrors the v1.3 release-boundary checker style
    - Default verification asserts opt-in public-network checks stay outside `scripts/verify.sh`

key-files:
  created:
    - scripts/check-v1.4-release-boundaries.ts
    - packages/open-bitcoin-cli/src/operator/dashboard/model/tests.rs
    - .planning/phases/59-operator-evidence-threat-model-and-release-boundaries/59-05-SUMMARY.md
  modified:
    - scripts/verify.sh
    - docs/metrics/lines-of-code.md
    - packages/open-bitcoin-cli/src/operator/dashboard/model.rs
    - docs/parity/source-breadcrumbs.json

key-decisions:
  - "Added a dedicated v1.4 checker instead of broadening the historical v1.3 checker."
  - "Kept `scripts/verify.sh` deterministic by asserting it contains no live-smoke, manual-peer, or restart-after-progress invocation."
  - "Moved dashboard model tests to a child test module when aggregate verification exposed a file-length regression from earlier Phase 59 work."
  - "Task commits were deferred to the final strict yolo push gate per wrapper instructions."

patterns-established:
  - "Milestone-specific release-boundary claims are checked through exact parity-root, operator-guide, threat-model, and deferred-surface strings."
  - "Tracked generated LOC is refreshed only when the aggregate verification contract reports it stale."

requirements-completed: [SEC-03]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 59-2026-06-05T15-10-59
generated_at: 2026-06-05T19:40:48Z

duration: 35min
completed: 2026-06-05
---

# Phase 59 Plan 05: Release Boundary Verification Summary

**Default repo verification now enforces the v1.4 release-boundary parity roots while keeping public-network live smoke opt-in only.**

## Accomplishments

- Added `scripts/check-v1.4-release-boundaries.ts` with deterministic checks for the `v1-4-operator-evidence-release-boundaries` surface, all OBS/SEC requirement IDs, required evidence paths, v1.4 audit roots, current/historical threat-model links, operator UAT commands, threat-model IDs, and deferred-surface wording.
- Wired `bun run scripts/check-v1.4-release-boundaries.ts` into `scripts/verify.sh` immediately after the v1.3 release-boundary checker.
- Refreshed `docs/metrics/lines-of-code.md` after the new script and Rust test module made the tracked generated report stale.
- Split inline dashboard model tests into `packages/open-bitcoin-cli/src/operator/dashboard/model/tests.rs` after aggregate verification caught `model.rs` above the production file-length limit; added the new test file to `docs/parity/source-breadcrumbs.json`.

## Deviations from Plan

- `bash scripts/verify.sh` initially failed because `packages/open-bitcoin-cli/src/operator/dashboard/model.rs` had grown to 712 lines during earlier Phase 59 work. The fix was a mechanical test-module split; dashboard behavior and assertions are unchanged.
- `bash scripts/verify.sh` then failed only for stale LOC. The tracked LOC report was regenerated once with the plan-approved command.

## Verification

Passed:

- `bun run scripts/check-v1.4-release-boundaries.ts`
- `bash scripts/test-run-live-mainnet-smoke.sh`
- `rg -n "SURFACE_ID|v1-4-operator-evidence-release-boundaries|REQUIRED_REQUIREMENTS|OBS-01|OBS-02|OBS-03|SEC-01|SEC-02|SEC-03|V14-TM-08" scripts/check-v1.4-release-boundaries.ts`
- `rg -n "requireNotContains\\(verifyScript, \"run-live-mainnet-smoke\"|requireNotContains\\(verifyScript, \"--restart-after-progress\"" scripts/check-v1.4-release-boundaries.ts`
- `rg -n "check-v1\\.3-release-boundaries\\.ts|check-v1\\.4-release-boundaries\\.ts" scripts/verify.sh`
- `! rg -n "run-live-mainnet-smoke|--manual-peer|--restart-after-progress" scripts/verify.sh`
- `cargo fmt --manifest-path packages/Cargo.toml --all`
- `bash scripts/check-file-lengths.sh`
- `bun run scripts/check-parity-breadcrumbs.ts --check`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli dashboard::model::tests`
- `bun run scripts/generate-loc-report.ts --source=worktree --output=docs/metrics/lines-of-code.md`
- `bash scripts/verify.sh`

## Known Stubs

None.

## Threat Flags

None. The checker only reads repo-owned docs/scripts and fails closed on missing or broadened release-boundary claims. `scripts/verify.sh` remains public-network-free.

## User Setup Required

None.

## Next Phase Readiness

SEC-03 is ready for Phase 59 code review and phase-level verification. The final phase gate has passing aggregate evidence from `bash scripts/verify.sh`.

## Self-Check: PASSED

- Found the v1.4 checker and `scripts/verify.sh` invocation.
- Confirmed `scripts/verify.sh` has no live-smoke/manual-peer/restart public-network command.
- Confirmed `model.rs` is below the production file-length cap after the test split.
- Confirmed `bash scripts/verify.sh` passed after the LOC refresh.
- Commit self-check intentionally skipped because the wrapper requires no staging, commits, or pushes until final strict verification passes.

---
*Phase: 59-operator-evidence-threat-model-and-release-boundaries*
*Completed: 2026-06-05*
