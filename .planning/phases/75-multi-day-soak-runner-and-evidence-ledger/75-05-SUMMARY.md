---
phase: 75-multi-day-soak-runner-and-evidence-ledger
plan: 05
subsystem: operator-soak-documentation
tags: [docs, operator-cli, soak, parity, release-boundaries]

requires:
  - phase: 75-multi-day-soak-runner-and-evidence-ledger
    provides: Plan 75-02 operator-facing soak commands and same-run lifecycle behavior
  - phase: 75-multi-day-soak-runner-and-evidence-ledger
    provides: Plan 75-03 deterministic synthetic soak and ledger replay coverage
  - phase: 75-multi-day-soak-runner-and-evidence-ledger
    provides: Plan 75-04 compact redacted support-bundle soak projection
provides:
  - Exact repo-local Cargo and Bazel soak command docs for start, resume, stop, and report
  - Shared architecture vocabulary for soak ledger event kinds and final outcomes
  - Phase 75 parity root and scoped README wording for bounded opt-in soak evidence
affects: [phase-75, operator-docs, parity-docs, README]

tech-stack:
  added: []
  patterns:
    - Repo-local operator commands for opt-in soak workflows
    - Datadir-owned source-of-truth wording for run index and JSONL ledger
    - Proof/non-proof boundary wording for scoped soak evidence

key-files:
  created:
    - .planning/phases/75-multi-day-soak-runner-and-evidence-ledger/75-05-SUMMARY.md
  modified:
    - docs/operator/runtime-guide.md
    - docs/architecture/status-snapshot.md
    - docs/architecture/operator-observability.md
    - docs/parity/catalog/p2p.md
    - docs/parity/catalog/chainstate.md
    - docs/parity/catalog/operator-runtime-release-hardening.md
    - docs/parity/index.json
    - docs/parity/checklist.md
    - docs/parity/README.md
    - README.md

key-decisions:
  - "Document Phase 75 as bounded opt-in soak evidence instead of a production-node readiness claim."
  - "Keep run-index.json plus events.jsonl as the documented durable source of truth; reports and support bundles are projections."
  - "Add the parity surface as `phase75-multi-day-soak-runner-evidence-ledger` across machine and human roots."

patterns-established:
  - "Soak docs should include both Cargo and Bazel repo-local command forms."
  - "Soak outcomes wrap shared sync/status evidence without redefining lower-level sync stop or recovery labels."

requirements-completed: [SOAK-01, SOAK-02, SOAK-03, SOAK-04]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 75-2026-06-14T22-59-23
generated_at: 2026-06-15T02:50:10Z

duration: 8 min
completed: 2026-06-15
---

# Phase 75 Plan 05: Operator Soak Documentation and Parity Roots Summary

**Scoped operator soak workflow docs with durable ledger semantics, parity roots, and proof-boundary wording**

## Performance

- **Duration:** 8 min
- **Started:** 2026-06-15T02:42:32Z
- **Completed:** 2026-06-15T02:50:10Z
- **Tasks:** 1
- **Files modified:** 11

## Accomplishments

- Added `### Phase 75 multi-day soak runner` to the operator guide with exact repo-local Cargo and Bazel `soak start`, `resume`, `stop`, and `report` command forms.
- Documented the soak ledger source of truth, event kinds, and final outcome vocabulary in operator and architecture docs.
- Added the `phase75-multi-day-soak-runner-evidence-ledger` parity surface to machine-readable and human-readable parity roots plus scoped README wording.

## Task Commits

Each task was committed atomically:

1. **Task 1: Document operator soak commands, evidence semantics, and parity roots** - `90a322a` (`docs`)

## Files Created/Modified

- `docs/operator/runtime-guide.md` - Adds Phase 75 operator soak commands, source-of-truth wording, event/outcome vocabulary, and proof boundary.
- `docs/architecture/status-snapshot.md` - Adds soak ledger event and outcome vocabulary without overloading sync status labels.
- `docs/architecture/operator-observability.md` - Adds datadir-owned source-of-truth and projection boundaries for soak observability.
- `docs/parity/catalog/p2p.md` - Adds the Phase 75 P2P-facing soak evidence boundary.
- `docs/parity/catalog/chainstate.md` - Adds the Phase 75 chainstate evidence interpretation boundary.
- `docs/parity/catalog/operator-runtime-release-hardening.md` - Adds the Phase 75 operator-runtime audit row and exact proof/non-proof wording.
- `docs/parity/index.json` - Adds the Phase 75 parity surface, checklist entry, and audit pointer.
- `docs/parity/checklist.md` - Adds the Phase 75 human-readable parity checklist row.
- `docs/parity/README.md` - Adds a pointer to the Phase 75 parity root.
- `README.md` - Adds scoped operator-surface wording for `open-bitcoin soak`.
- `.planning/phases/75-multi-day-soak-runner-and-evidence-ledger/75-05-SUMMARY.md` - Records execution results.

## Decisions Made

- Kept Phase 75 documentation scoped to bounded opt-in full-sync soak behavior, durable resume evidence, or diagnosed blocker evidence.
- Treated reports, support summaries, and operator output as projections from `<datadir>/soak/run-index.json` plus `<datadir>/soak/runs/<run_id>/events.jsonl`.
- Left `.planning/STATE.md` and `.planning/ROADMAP.md` untouched because the sequential orchestrator owns shared state updates.

## Verification

- `jq empty docs/parity/index.json` passed.
- `git diff --check` passed.
- `rg -n "Phase 75 multi-day soak runner|soak start --elapsed-time-seconds 259200|durable source of truth is|phase75-multi-day-soak-runner-evidence-ledger" docs/operator/runtime-guide.md docs/parity/index.json README.md` passed.
- Acceptance checks for exact Cargo start command, exact Bazel start command, resume/stop/report forms, durable source-of-truth wording, proof-boundary wording, and parity-root references all passed.
- Architecture vocabulary check found the `started`, `checkpoint`, `resume`, `stop`, `verdict`, `clean_completion`, `diagnosed_blocker`, `operator_stop`, `resource_stop`, `recovery_stop`, and `unexpected_termination` labels in the architecture docs.

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

None.

## Known Stubs

None. The required stub-pattern scan found only a pre-existing prose reference to placeholders in `docs/parity/catalog/operator-runtime-release-hardening.md`; no introduced stub or unwired data path was found.

## Threat Flags

None - this plan changed documentation and parity roots only. It introduced no new network endpoint, auth path, file-access implementation, or runtime schema boundary.

## User Setup Required

None - no external service configuration required. The documented public-network multi-day soak commands remain explicit opt-in operator workflows outside default verification.

## Next Phase Readiness

Ready for Plan 75-06 to add checker and default-verifier guards around the Phase 75 docs, parity roots, and generated LOC freshness.

## Self-Check: PASSED

- Summary file exists: `.planning/phases/75-multi-day-soak-runner-and-evidence-ledger/75-05-SUMMARY.md`
- Task commit exists: `90a322a`

---
*Phase: 75-multi-day-soak-runner-and-evidence-ledger*
*Completed: 2026-06-15*
