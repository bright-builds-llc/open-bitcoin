---
phase: 77-corruption-and-lock-recovery-hardening
plan: 06
subsystem: docs-parity
tags: [documentation, parity, recovery-evidence, operator-guidance]

requires:
  - phase: 77-corruption-and-lock-recovery-hardening
    provides: Plan 77-05 soak checkpoint and report recovery evidence
provides:
  - Phase 77 operator recovery evidence documentation
  - Phase 77 architecture recovery evidence contracts
  - Phase 77 parity and release-boundary roots for REC-05 through REC-08
affects: [operator-docs, parity-ledger, release-boundaries, recovery-evidence]

tech-stack:
  added: []
  patterns:
    - Document Phase 77 as diagnosis and evidence only.
    - Keep recovery labels stable across operator docs, architecture contracts, and parity roots.

key-files:
  created:
    - .planning/phases/77-corruption-and-lock-recovery-hardening/77-06-SUMMARY.md
  modified:
    - docs/architecture/status-snapshot.md
    - docs/architecture/storage-decision.md
    - docs/architecture/operator-observability.md
    - docs/operator/runtime-guide.md
    - docs/parity/index.json
    - docs/parity/README.md
    - docs/parity/checklist.md
    - docs/parity/release-readiness.md
    - docs/parity/catalog/operator-runtime-release-hardening.md
    - README.md

key-decisions:
  - "Keep Phase 77 recovery hardening framed as diagnosis and evidence only, not automatic repair or production-node readiness."
  - "Root REC-05 through REC-08 in one parity surface linking source, docs, checker, and planning artifacts."

patterns-established:
  - "Recovery evidence docs name the top-level `recovery_evidence: FieldAvailability<RecoveryEvidenceSnapshot>` contract beside legacy sync recovery summaries."
  - "Parity roots list destructive repair, lock cleanup, source datadir mutation, process scanning, public-network defaults, and production-node readiness as explicit non-goals."

requirements-completed: [REC-05, REC-06, REC-07, REC-08]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 77-2026-06-15T18-33-03
generated_at: 2026-06-16T00:07:35Z

duration: 9min
completed: 2026-06-16
***

# Phase 77 Plan 06: Recovery Documentation And Parity Summary

**Phase 77 recovery evidence now has operator guidance, architecture contracts, and parity roots that preserve diagnosis-only recovery boundaries.**

## Performance

- **Duration:** 9min
- **Started:** 2026-06-15T23:58:54Z
- **Completed:** 2026-06-16T00:07:35Z
- **Tasks:** 2
- **Files modified:** 10

## Accomplishments

- Documented `recovery_evidence: FieldAvailability<RecoveryEvidenceSnapshot>` as the shared status, support, dashboard, and soak recovery contract.
- Listed stable Phase 77 action classes, causes, and compatibility categories across operator and architecture docs.
- Added exact repo-local Cargo and Bazel status/support commands plus the required non-mutating safety boundary.
- Added the `phase77-corruption-and-lock-recovery-hardening` parity surface for REC-05 through REC-08 with source, docs, checker, and planning evidence roots.

## Task Commits

Each task was committed atomically:

1. **Task 1: Document status and storage recovery contracts** - `0e1ee7b` (docs)
2. **Task 2: Add Phase 77 parity and release-boundary roots** - `1e9459a` (docs)

## Files Created/Modified

- `docs/architecture/status-snapshot.md` - Added the Phase 77 top-level recovery evidence contract and stable labels.
- `docs/architecture/storage-decision.md` - Added non-mutating storage and lock recovery semantics plus the exact safety boundary.
- `docs/architecture/operator-observability.md` - Documented cross-surface recovery evidence rendering and non-goals.
- `docs/operator/runtime-guide.md` - Added Phase 77 operator guidance, exact commands, action class interpretation, and safety boundary.
- `README.md` - Scoped active v1.7 recovery wording to diagnosis and evidence only.
- `docs/parity/index.json` - Added the Phase 77 surface and audit root for REC-05 through REC-08.
- `docs/parity/README.md` - Added the Phase 77 parity root overview.
- `docs/parity/checklist.md` - Added the human-readable Phase 77 checklist surface.
- `docs/parity/release-readiness.md` - Added a scoped Phase 77 recovery-hardening claim and non-goals.
- `docs/parity/catalog/operator-runtime-release-hardening.md` - Added Phase 77 catalog evidence and known-gap wording.

## Decisions Made

- Kept Phase 77 wording scoped to diagnosis and evidence only.
- Preserved v1.6 release-readiness framing while adding a separate scoped v1.7 Phase 77 recovery-hardening claim.
- Used exact repo-local operator commands from the plan rather than installed alias-only forms.

## Verification

- `rg -n "Phase 77 corruption and lock recovery hardening|recovery_evidence|safe_retry|read_only_inspection|backup_then_rebuild|stop_and_escalate|stale_lock_evidence|concurrent_datadir_use" docs/architecture/status-snapshot.md docs/architecture/storage-decision.md docs/architecture/operator-observability.md docs/operator/runtime-guide.md README.md` passed.
- `rg -n "Phase 77 corruption and lock recovery hardening" docs/architecture/status-snapshot.md docs/architecture/storage-decision.md docs/operator/runtime-guide.md` passed.
- `rg -n "Phase 77 does not delete lock files, clear recovery markers, repair stores, compact stores, reindex stores, relocate datadirs, mutate source datadirs, scan OS process tables, or upload support bundles automatically." docs/operator/runtime-guide.md docs/architecture/storage-decision.md` passed.
- `rg -n "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir <path> status --format json|bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir <path> status --format json|support bundle --output-dir <path>/support --format json" docs/operator/runtime-guide.md` passed.
- `rg -n "phase77-corruption-and-lock-recovery-hardening|REC-05|REC-06|REC-07|REC-08|scripts/check-phase77-corruption-lock-recovery.ts" docs/parity/index.json docs/parity/README.md docs/parity/checklist.md docs/parity/release-readiness.md docs/parity/catalog/operator-runtime-release-hardening.md` passed.
- `rg -n '"phase77-corruption-and-lock-recovery-hardening"' docs/parity/index.json docs/parity/checklist.md` passed.
- `rg -n "packages/open-bitcoin-node/src/recovery.rs|packages/open-bitcoin-node/src/storage/lock_probe.rs|packages/open-bitcoin-cli/src/operator/status/recovery_evidence.rs|scripts/check-phase77-corruption-lock-recovery.ts" docs/parity/index.json docs/parity/catalog/operator-runtime-release-hardening.md` passed.
- `rg -n "automatic destructive repair|lock cleanup|source datadir mutation|process scanning|production-node readiness" docs/parity/release-readiness.md docs/parity/catalog/operator-runtime-release-hardening.md` passed.
- `node -e "const fs=require('fs'); JSON.parse(fs.readFileSync('docs/parity/index.json','utf8')); console.log('index json ok')"` passed.
- Manual scope review confirmed the changed docs do not claim inbound serving, relay, production-funds wallet safety, migration apply mode, automatic destructive repair, public-network default verification, or broad production-node readiness.

## Deviations from Plan

None - plan executed as written.

## Issues Encountered

- The GSD commit helper initially committed only three of the five Task 1 files. I amended my own just-created Task 1 commit with the two omitted architecture files before starting Task 2, preserving one atomic Task 1 commit.

## Known Stubs

None. The stub scan found only an existing phrase saying docs should not be "placeholders"; it did not identify unwired data, TODO/FIXME content, or placeholder UI/report data introduced by this plan.

## Threat Flags

None - this plan changed documentation and parity roots only. The trust-boundary changes were already covered by T-77-16 through T-77-18 in the plan.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Ready for Plan 77-07 to add the deterministic Phase 77 checker, verifier wiring, and verification evidence.

---
*Phase: 77-corruption-and-lock-recovery-hardening*
*Completed: 2026-06-16*

## Self-Check: PASSED

- Found summary file at `.planning/phases/77-corruption-and-lock-recovery-hardening/77-06-SUMMARY.md`.
- Verified task commits exist: `0e1ee7b`, `1e9459a`.
- Confirmed all documented plan `rg` verification and acceptance scans passed.
- Confirmed `docs/parity/index.json` parses successfully.
