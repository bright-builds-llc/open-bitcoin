---
phase: 75-multi-day-soak-runner-and-evidence-ledger
plan: 01
subsystem: operator-soak-ledger
tags: [rust, operator-cli, soak, ledger, reports, parity-breadcrumbs]

requires:
  - phase: 72-operator-observability-and-support-evidence
    provides: Shared status and support evidence verdict contracts wrapped by soak outcomes
provides:
  - Pure soak bounds, peer-policy, run-id, stop-condition, and final outcome contracts
  - Datadir-owned soak run index and append-only JSONL event ledger
  - Ledger-derived JSON and Markdown report projections
affects: [phase-75, operator-cli, support-evidence, parity-breadcrumbs]

tech-stack:
  added: []
  patterns:
    - Versioned JSONL ledger envelopes with bounded full-line writes
    - Reports as reproducible projections from ledger events

key-files:
  created:
    - packages/open-bitcoin-cli/src/operator/soak.rs
    - packages/open-bitcoin-cli/src/operator/soak/ledger.rs
    - packages/open-bitcoin-cli/src/operator/soak/outcome.rs
    - packages/open-bitcoin-cli/src/operator/soak/report.rs
    - packages/open-bitcoin-cli/src/operator/soak/tests.rs
  modified:
    - packages/open-bitcoin-cli/src/operator.rs
    - packages/open-bitcoin-cli/src/operator/support.rs
    - docs/parity/source-breadcrumbs.json

key-decisions:
  - "Keep soak outcomes separate from sync recovery/stop enums and wrap shared status/support evidence instead."
  - "Treat reports as projections only; write report artifacts without updating run-index.json."
  - "Use datadir-owned JSONL ledger events with sequence numbers, size caps, sync_all, partial-line recovery, and atomic run-index rename."

patterns-established:
  - "Soak source of truth: <datadir>/soak/run-index.json plus <datadir>/soak/runs/<run_id>/events.jsonl."
  - "Soak reports carry is_projection, source_ledger_path, and latest_sequence."

requirements-completed: [SOAK-01, SOAK-02, SOAK-03]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 75-2026-06-14T22-59-23
generated_at: 2026-06-15T01:27:30Z

duration: 21 min
completed: 2026-06-15
---

# Phase 75 Plan 01: Soak Ledger and Report Projection Summary

**Datadir-owned soak run ledger with typed outcome contracts and reproducible JSON/Markdown projections**

## Performance

- **Duration:** 21 min
- **Started:** 2026-06-15T01:06:02Z
- **Completed:** 2026-06-15T01:27:30Z
- **Tasks:** 3
- **Files modified:** 8

## Accomplishments

- Added soak bounds, run-id validation, peer policy, stop condition, and final outcome vocabulary.
- Added a versioned JSONL ledger with started, checkpoint, resume, stop, and verdict events plus atomic run-index writes.
- Added JSON and Markdown report projections that name the source ledger and latest sequence without mutating the run index.

## Task Commits

Each task was committed atomically:

1. **Task 1: Add soak bounds and outcome contracts** - `141efbb` (`feat`)
2. **Task 2: Add datadir-owned run index and JSONL ledger** - `29f5e55` (`feat`)
3. **Task 3: Add ledger-derived JSON and Markdown report projections** - `ab2a751` (`feat`)

## Files Created/Modified

- `packages/open-bitcoin-cli/src/operator.rs` - Registers the `soak` module.
- `packages/open-bitcoin-cli/src/operator/support.rs` - Re-exports support verdict types for soak evidence wrapping.
- `packages/open-bitcoin-cli/src/operator/soak.rs` - Soak bounds, run-id, peer-policy, stop-condition, and module entry contracts.
- `packages/open-bitcoin-cli/src/operator/soak/outcome.rs` - Soak-owned outcome labels and evidence classifier.
- `packages/open-bitcoin-cli/src/operator/soak/ledger.rs` - Run layout, run index, event envelopes, JSONL append/read logic, and durability bounds.
- `packages/open-bitcoin-cli/src/operator/soak/report.rs` - Ledger-derived report projection, JSON/Markdown rendering, and report writes.
- `packages/open-bitcoin-cli/src/operator/soak/tests.rs` - Focused soak bounds, outcome, ledger, and report tests.
- `docs/parity/source-breadcrumbs.json` - Breadcrumb mapping for new Open Bitcoin-only soak files.

## Decisions Made

- Reports remain projections and do not expose any run-index writing function.
- The ledger uses explicit event envelopes with schema version, run id, sequence, timestamp, and tagged event payload.
- Outcome classification prioritizes operator stops, resource stops, recovery stops, diagnosed blockers, unexpected interruptions, then clean completion from proven support verdicts.

## Verification

- RED Task 1: `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --lib soak_outcome_ --all-features` failed on unresolved soak outcome/bounds imports before implementation.
- RED Task 2: `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --lib soak_ledger_ --all-features` failed on unresolved ledger imports before implementation.
- RED Task 3: `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --lib soak_report_ --all-features` failed on unresolved report imports before implementation.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --lib soak_outcome_ --all-features` passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --lib soak_bounds_ --all-features` passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --lib soak_ledger_ --all-features` passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --lib soak_report_ --all-features` passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --lib soak_ --all-features` passed: 14 tests.
- `bun run scripts/check-parity-breadcrumbs.ts --check` passed for 245 Rust files.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Re-exported support verdict types for soak outcome wrapping**
- **Found during:** Task 1 (Add soak bounds and outcome contracts)
- **Issue:** `SupportEvidenceVerdict`, `EvidenceVerdictSummary`, and `TipEvidence` were private to the support module, so soak tests and the outcome classifier could not construct or inspect full-sync support verdict evidence.
- **Fix:** Added narrow `pub(crate)` re-exports from `operator/support.rs` instead of reaching into a private sibling module or duplicating support verdict types.
- **Files modified:** `packages/open-bitcoin-cli/src/operator/support.rs`
- **Verification:** Task 1 focused outcome and bounds tests passed.
- **Committed in:** `141efbb`

---

**Total deviations:** 1 auto-fixed (1 blocking)
**Impact on plan:** The change preserves the planned evidence boundary and avoids duplicating shared support verdict contracts.

## Issues Encountered

- Cargo waited on artifact/package locks during focused test runs in the shared worktree; each test command completed successfully.
- The parity breadcrumb checker only scans tracked Rust files, so new soak files were explicitly staged before running the checker.

## Known Stubs

None.

## Threat Flags

None - new file-write and report-projection surfaces were covered by the plan threat model.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Ready for Plan 75-02 to wire these contracts into the operator-facing soak command and same-run lifecycle flow.

## Self-Check: PASSED

- Summary file exists: `.planning/phases/75-multi-day-soak-runner-and-evidence-ledger/75-01-SUMMARY.md`
- Task commit exists: `141efbb`
- Task commit exists: `29f5e55`
- Task commit exists: `ab2a751`

---
*Phase: 75-multi-day-soak-runner-and-evidence-ledger*
*Completed: 2026-06-15*
