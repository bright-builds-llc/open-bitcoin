---
phase: 58-same-datadir-restart-and-resume-evidence
plan: 03
subsystem: live-smoke
tags: [typescript, recovery-diagnosis, operator-docs, parity]
requires:
  - phase: 58-same-datadir-restart-and-resume-evidence
    provides: compact restartResumeEvidence schema
provides:
  - typed recovery diagnosis for restart reports
  - operator same-datadir restart/resume UAT commands
  - scoped P2P parity wording for restart/resume evidence
affects: [operator-uat, parity-catalog, recovery-guidance]
tech-stack:
  added: []
  patterns: [storage-first recovery diagnosis, opt-in public-network docs]
key-files:
  created: []
  modified:
    - scripts/run-live-mainnet-smoke.ts
    - scripts/test-run-live-mainnet-smoke.sh
    - docs/operator/runtime-guide.md
    - docs/parity/catalog/p2p.md
key-decisions:
  - "Recovery diagnosis is stored under `result.restartResumeEvidence.recoveryDiagnosis`."
  - "Storage incompatibility/corruption takes precedence over peer or network guidance."
  - "Docs preserve the explicit no-unattended-production-node scope boundary."
patterns-established:
  - "Operator UAT docs include both Cargo and Bazel commands against the same datadir."
requirements-completed: [RESUME-02, RESUME-03]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 58-2026-06-05T12-58-05
generated_at: 2026-06-05T13:39:21Z
duration: 6min
completed: 2026-06-05
---

# Phase 58: Plan 03 Summary

**Storage-first restart recovery diagnosis plus operator and parity docs for same-datadir resume evidence.**

## Performance

- **Duration:** 6 min
- **Started:** 2026-06-05T13:33:22Z
- **Completed:** 2026-06-05T13:39:21Z
- **Tasks:** 2
- **Files modified:** 4

## Accomplishments

- Added the seven-category `RecoveryDiagnosisCategory` matrix to restart evidence.
- Added fixture coverage for `peer_incompatibility`, `public_network_unreachable`, `invalid_peer_data`, `store_corruption`, `store_incompatibility`, `resource_exhaustion`, and `intentional_cancellation`.
- Documented same-datadir restart/resume UAT with exact report fields and Cargo/Bazel status commands.
- Updated P2P parity docs to describe opt-in `restartResumeEvidence` without broadening the unattended production claim.

## Task Commits

Task commits are deferred to the final strict yolo push gate for this run. No code is committed until phase verification and repo verification pass.

## Files Created/Modified

- `scripts/run-live-mainnet-smoke.ts` - Adds typed recovery diagnosis and storage-first precedence.
- `scripts/test-run-live-mainnet-smoke.sh` - Adds deterministic recovery category matrix and cancellation restart fixture.
- `docs/operator/runtime-guide.md` - Adds same-datadir restart/resume commands and pass/fail interpretation.
- `docs/parity/catalog/p2p.md` - Adds scoped restart/resume parity wording and preserves known gaps.

## Decisions Made

- Kept support-bundle allowlisting unchanged; Phase 58 only documents compact report fields.
- Treated fresh post-restart progress as stronger evidence, not mandatory when durable resume and typed blocker evidence are present.

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

None.

## Verification

- `bash scripts/test-run-live-mainnet-smoke.sh` - passed.
- `bun run scripts/run-live-mainnet-smoke.ts --help` - passed.
- `rg -n "type RecoveryDiagnosisCategory|peer_incompatibility|public_network_unreachable|invalid_peer_data|store_corruption|store_incompatibility|resource_exhaustion|intentional_cancellation" scripts/run-live-mainnet-smoke.ts` - found all categories.
- `rg -n -- "--restart-after-progress|restartResumeEvidence|same-datadir restart|same datadir|duplicateConnectVerdict|store_incompatibility|intentional_cancellation|result.restartResumeEvidence.restartStatus|cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin --|bazel run //packages/open-bitcoin-cli:open_bitcoin --" docs/operator/runtime-guide.md docs/parity/catalog/p2p.md` - found the required docs strings.
- `rg -n "run-live-mainnet-smoke|--restart-after-progress" scripts/verify.sh` - returned no matches, preserving the opt-in boundary.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Phase 58 is ready for phase-level verification. Public-network restart UAT remains copy-pasteable and opt-in.

---
*Phase: 58-same-datadir-restart-and-resume-evidence*
*Completed: 2026-06-05*
