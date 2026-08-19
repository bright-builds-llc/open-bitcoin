---
phase: 137-rpc-and-sanitized-operator-evidence
plan: 02
subsystem: node
tags: [package-rpc, dry-run, submit-package, local-admission, lifecycle]

# Dependency graph
requires:
  - phase: 132-typed-package-vocabulary-and-staged-admission
    provides: DryRunPackageCommand, SubmitPackageCommand, PackageReport, and SubmissionPackage
  - phase: 134-authoritative-cross-cache-lifecycle-integration
    provides: LifecycleCommand::PackageAdmission and AdmissionProjectionSource::Local
provides:
  - ManagedNetworkHandle::dry_run_local_package as a non-mutating read
  - ManagedNetworkHandle::submit_local_package through PackageAdmission Local
affects: [137-03 package RPC dispatch, 137-06 Open Bitcoin package extension]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - Local package dry-run uses try_read plus DryRunPackageCommand only
    - Local package submit uses prepare_package then PackageAdmission Local

key-files:
  created:
    - packages/open-bitcoin-node/src/network/admission_bridge/local_package.rs
    - packages/open-bitcoin-node/src/network/tests/local_package_cases.rs
  modified:
    - packages/open-bitcoin-node/src/mempool.rs
    - packages/open-bitcoin-node/src/network/admission_bridge.rs
    - packages/open-bitcoin-node/src/network/runtime_authority.rs
    - packages/open-bitcoin-node/src/network/tests.rs

key-decisions:
  - "Dry-run is a handle try_read; it never takes try_mutate or applies LifecycleCommand"
  - "Submit commits through prepare_package plus PackageAdmission Local, not the peer 1P1C bridge"
  - "RelayIntent stays caller-supplied; NotRequested still admits locally when policy allows"

patterns-established:
  - "Pattern 1: Local RPC packages share Phase 132 commands and never reuse submit_same_peer_candidate"
  - "Pattern 2: Shell-sampled PolicyTime and RelayIntent are required handle arguments"

requirements-completed: [MPOBS-01]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 137-2026-08-19T21-52-42
generated_at: 2026-08-19T22:53:10Z

# Metrics
duration: 6min
completed: 2026-08-19
---

# Phase 137 Plan 02: Local Package Dry-Run and Submit Summary

**Local package dry-run is a non-mutating handle read, and submit commits through PackageAdmission Local without the peer 1P1C bridge**

## Performance

- **Duration:** 6 min
- **Started:** 2026-08-19T22:47:31Z
- **Completed:** 2026-08-19T22:53:10Z
- **Tasks:** 2
- **Files modified:** 6

## Accomplishments
- Locked D-03 on the node handle: dry-run leaves mempool count, rolling fee, relay counters, and checkpoint dirty generation unchanged
- Local submit returns authoritative `SubmittedPackageResult` membership through `AdmissionProjectionSource::Local`
- Shape errors fail closed as `ManagedNetworkError::PackageShape` before any lifecycle command

## Task Commits

Each task was committed atomically:

1. **Task 1: Add ManagedMempool::dry_run_package and handle dry_run_local_package** - `55fc7c2e` (test)
2. **Task 1: Add ManagedMempool::dry_run_package and handle dry_run_local_package** - `1d509823` (feat)
3. **Task 2: Add submit_local_package through PackageAdmission Local** - `5f231ac5` (test)
4. **Task 2: Add submit_local_package through PackageAdmission Local** - `51f9d3ad` (feat)

**Plan metadata:** docs commit follows this summary

_Note: TDD tasks may have multiple commits (test → feat → refactor)_

## Files Created/Modified
- `packages/open-bitcoin-node/src/network/admission_bridge/local_package.rs` - Local dry-run and submit on `ManagedPeerNetwork`
- `packages/open-bitcoin-node/src/network/tests/local_package_cases.rs` - Before/after dry-run snapshots and local submit cases
- `packages/open-bitcoin-node/src/mempool.rs` - `ManagedMempool::dry_run_package` forwards without the submit probe
- `packages/open-bitcoin-node/src/network/admission_bridge.rs` - Declares `mod local_package`
- `packages/open-bitcoin-node/src/network/runtime_authority.rs` - Handle facades using `try_read` / `try_mutate`
- `packages/open-bitcoin-node/src/network/tests.rs` - Registers `mod local_package_cases`

## Decisions Made
- Keep dry-run on `try_read` so the mutate lock cannot apply lifecycle
- Commit submit through `prepare_package` plus `LifecycleCommand::PackageAdmission` with `AdmissionProjectionSource::Local`, matching the peer package dispatcher without attaching peer ids
- Pass shell-sampled `RelayIntent` through unchanged, including `NotRequested`

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Commit membership only through PackageAdmission**
- **Found during:** Task 2 (local submit)
- **Issue:** The behavior line listed `ManagedMempool::submit_package` then `LifecycleCommand::PackageAdmission`. `submit_package` already commits mempool state, so a second lifecycle apply would double-commit or fail an invariant
- **Fix:** Follow the plan action and peer package file: `prepare_package` then `apply_lifecycle_command(..., LifecycleCommand::PackageAdmission(plan))` with `AdmissionProjectionSource::Local`
- **Files modified:** `packages/open-bitcoin-node/src/network/admission_bridge/local_package.rs`
- **Verification:** The four `submit_local_package_*` tests pass; empty shape errors leave dirty generation unchanged
- **Committed in:** `51f9d3ad` (Task 2 feat)

---

**Total deviations:** 1 auto-fixed (1 bug)
**Impact on plan:** Preserves D-03. Dry-run stays non-mutating. Submit still matches authoritative membership. No RPC, CLI, or snapshot group work was added.

## Issues Encountered
None

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Plan 03 can dispatch `testmempoolaccept` / `submitpackage` through these handle methods
- Plan 06 can expose the typed Open Bitcoin package report on the same dry-run/submit split
- Peer 1P1C helpers remain unused on the local RPC path

## Self-Check: PASSED

---
*Phase: 137-rpc-and-sanitized-operator-evidence*
*Completed: 2026-08-19*
