---
phase: 02-core-domain-and-serialization-foundations
plan: 03
subsystem: testing
tags: [rust, coverage, fixtures, verification]
requires:
  - phase: 02-02
    provides: foundational domain and codec modules
provides:
  - fixture-backed codec tests
  - repo-native coverage gate for all pure-core crates
  - 100 percent pure-core line coverage for Phase 2 code
affects: [ci, contributor-workflow, future pure-core crates]
tech-stack:
  added: [repo-owned fixture hex files]
  patterns: [fixture provenance, allowlist-driven coverage]
key-files:
  created:
    - packages/open-bitcoin-codec/testdata/transaction_valid.hex
    - packages/open-bitcoin-codec/testdata/block_header.hex
    - packages/open-bitcoin-codec/testdata/message_header.hex
  modified:
    - scripts/verify.sh
    - packages/open-bitcoin-codec/src/transaction.rs
    - packages/open-bitcoin-codec/src/block.rs
    - packages/open-bitcoin-codec/src/network.rs
key-decisions:
  - "Extended the pure-core coverage gate from one scaffold crate to the allowlisted pure-core crate set."
  - "Used checked-in hex fixtures and unit tests instead of runtime file I/O or ad hoc examples."
patterns-established:
  - "Pure-core coverage is verified for every allowlisted crate through `scripts/pure-core-crates.txt`."
  - "Codec edge cases get deterministic unit tests before later fuzzing and black-box parity phases."
requirements-completed: [CONS-01]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 2-2026-04-11T14-58-23
generated_at: 2026-04-11T15:25:48.127Z
duration: 9 min
completed: 2026-04-11
---

# Phase 2 Plan 03: Core Domain and Serialization Foundations Summary

**Locked the Phase 2 codec layer to baseline-derived fixtures and expanded the repo-native verify contract to hold every pure-core crate at 100% line coverage.**

## Performance

- **Duration:** 9 min
- **Started:** 2026-04-11T10:16:12-05:00
- **Completed:** 2026-04-11T10:24:58-05:00
- **Tasks:** 2
- **Files modified:** 15

## Accomplishments
- Added checked-in fixture hex files for transaction, block-header, and message-header coverage.
- Added targeted unit tests for codec helpers, error paths, and primitive branches until pure-core line coverage reached 100%.
- Updated `scripts/verify.sh` to derive the coverage package list from `scripts/pure-core-crates.txt`.

## Task Commits

Each task was committed atomically:

1. **Task 1: Add baseline-derived fixture files and round-trip tests** - `003d5a4` (test)
2. **Task 2: Raise pure-core unit coverage and expand the repo-native verification gate** - `003d5a4` (test)

**Plan metadata:** pending in docs commit after state and roadmap updates

## Files Created/Modified
- `packages/open-bitcoin-codec/testdata/transaction_valid.hex` - checked-in genesis transaction fixture
- `packages/open-bitcoin-codec/testdata/block_header.hex` - checked-in genesis block-header fixture
- `packages/open-bitcoin-codec/testdata/message_header.hex` - checked-in message-header fixture
- `scripts/verify.sh` - allowlist-driven pure-core coverage gate
- `packages/open-bitcoin-codec/src/*` - fixture tests and branch-coverage additions

## Decisions Made
- Measured coverage against all allowlisted pure-core crates rather than hand-maintaining a second package list in the verify script.
- Added negative tests for malformed CompactSize, witness flags, message commands, and EOF/trailing-data handling so parity evidence includes rejection paths too.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Replaced `mapfile` with a Bash 3.2-safe allowlist loop in `scripts/verify.sh`**
- **Found during:** Task 2 (Raise pure-core unit coverage and expand the repo-native verification gate)
- **Issue:** macOS system Bash does not provide `mapfile`, so the updated verify script failed before the coverage step.
- **Fix:** Replaced `mapfile` with a portable `while read` loop while preserving the allowlist-driven coverage behavior.
- **Files modified:** `scripts/verify.sh`
- **Verification:** `bash scripts/verify.sh` completed successfully with the new loop and reported 100% pure-core line coverage.
- **Committed in:** `003d5a4` (Task 2 commit)

---

**Total deviations:** 1 auto-fixed (1 blocking)
**Impact on plan:** No scope creep. The portability fix was required to make the planned verification contract usable on the local contributor environment.

## Issues Encountered
- The first coverage pass exposed untested helper/error branches in the new codec modules, which were resolved by adding deterministic unit tests rather than weakening the gate.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness
- The shared domain/codec layer now has evidence-backed tests and a stable verification contract.
- Later phases can depend on these crates without weakening the repo's pure-core coverage guarantees.

---
*Phase: 02-core-domain-and-serialization-foundations*
*Completed: 2026-04-11*
