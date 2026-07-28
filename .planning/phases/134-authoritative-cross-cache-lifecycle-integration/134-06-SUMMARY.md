---
phase: 134-authoritative-cross-cache-lifecycle-integration
plan: "06"
subsystem: node-network-lifecycle
tags: [rust, lifecycle-authority, admission, package-admission, transaction-relay]

requires:
  - phase: 134-authoritative-cross-cache-lifecycle-integration
    plan: "05"
    provides: sole validated lifecycle dispatcher and complete authoritative projection
provides:
  - singleton, local, peer, and package admission routed through the sole lifecycle authority
  - exact outward admission reports separated from authoritative projection facts
  - deterministic full, partial, post-trim, replacement, pressure, and failed package admission coverage
affects: [phase-134-production-routing, mempool-admission, package-relay, lifecycle-verification]

tech-stack:
  added: []
  patterns:
    - prepare an opaque admission transition once and dispatch it exactly once
    - preserve outward reports and feedback as non-authoritative caller data
    - derive every accepted projection from final authoritative membership

key-files:
  created:
    - packages/open-bitcoin-node/src/network/admission_bridge/singleton.rs
    - packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/admission.rs
    - packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/admission/partial_package.rs
  modified:
    - packages/open-bitcoin-node/src/network/admission_bridge.rs
    - packages/open-bitcoin-node/src/network/admission_bridge/package.rs
    - packages/open-bitcoin-node/src/network/action_translation.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/lifecycle.rs
    - scripts/check-phase133-package-aware-download-orphan-bridge.ts

key-decisions:
  - "Treat MempoolOutcome, SubmittedPackageResult, and Phase 133 feedback as outward compatibility data only; prepared lifecycle facts exclusively drive projection."
  - "Carry exact member-to-peer provenance into package preparation without exposing it as a second projection authority."
  - "Use one shared package dispatcher helper for production and exact scenario coverage so tests exercise the production authority path."

patterns-established:
  - "Admission facade: prepare once, preserve the outward report, construct one typed command, and invoke the sole dispatcher."
  - "Package finality: project survivors in authoritative admitted order and never infer accepted membership from the submitted package."

requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 134-2026-07-28T01-41-12
generated_at: 2026-07-28T11:52:00Z

duration: 1h 7m
completed: 2026-07-28
---

# Phase 134 Plan 06: Authoritative Admission Lifecycle Routing Summary

**Every singleton and package admission source now enters one typed lifecycle dispatcher while exact outward reports remain compatible and final authoritative membership alone controls all dependent caches.**

## Performance

- **Duration:** 1h 7m
- **Started:** 2026-07-28T10:45:17Z
- **Completed:** 2026-07-28T11:52:00Z
- **Tasks:** 2
- **Files modified:** 19

## Accomplishments

- Routed singleton, local, requested-peer, and package admission through `apply_lifecycle_command`, removing caller-side serving, fanout, peer, compact, unbroadcast, evidence, and generation projection authority.
- Preserved exact `MempoolOutcome`, `SubmittedPackageResult`, and bounded Phase 133 feedback semantics while preventing those outward shapes from determining accepted membership.
- Added complete deterministic admission scenarios for full and partial packages, post-trim absences, replacement, pressure, rejection, source-specific unbroadcast membership, generation behavior, and clean reconciliation.
- Proved package survivors project parent-first from exact final membership, rejected or trimmed members enter no accepted target, and replacement or pressure victims lose both identity aliases and fingerprints.
- Updated the Phase 133 structural mutation checker to enforce the new exactly-once package lifecycle dispatch contract.

## Task Commits

Each task was committed atomically:

1. **Task 1: Route singleton, local, and peer admission through the dispatcher** - `5f3c97b3`
2. **Task 2: Project complete package admission outcomes** - `9e5061d3`

## Files Created/Modified

- `packages/open-bitcoin-node/src/network/admission_bridge/singleton.rs` - Prepares singleton admission once, preserves its report, and dispatches the typed lifecycle command.
- `packages/open-bitcoin-node/src/network/admission_bridge/package.rs` - Shares one production package preparation/report/dispatch path with exact provenance and Phase 133 feedback.
- `packages/open-bitcoin-node/src/network/admission_bridge.rs` and `action_translation.rs` - Route local and peer callers through the authoritative admission facade.
- `packages/open-bitcoin-node/src/network/lifecycle_projection.rs` and `runtime_authority/lifecycle.rs` - Carry typed singleton/package commands through the sole validated authority path.
- `packages/open-bitcoin-node/src/network/relay_fanout.rs` and `relay_fanout/lifecycle.rs` - Apply authoritative admission source facts without report-driven projection.
- `packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/admission.rs` - Covers singleton compatibility, source-specific unbroadcast behavior, rejection, generations, and reconciliation.
- `packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/admission/partial_package.rs` - Covers full, partial, post-trim, replacement, pressure, and failed package outcomes across every projection.
- `packages/open-bitcoin-node/src/network/tests/package_bridge_cases/candidate_selection.rs` and `singleton_and_submission.rs` - Update legacy package expectations to the authoritative projection contract.
- `scripts/check-phase133-package-aware-download-orphan-bridge.ts` and its mutation tests - Require exactly one package lifecycle dispatch and reject restoration of the obsolete direct submission path.
- `docs/parity/source-breadcrumbs.json` - Registers the new first-party Rust sources against pinned Knots anchors.
- `docs/metrics/lines-of-code.md` - Refreshed by the mandatory repository hooks.

## Decisions Made

- Outward reports are compatibility values, not projection instructions; only the prepared transition consumed by lifecycle authority can change dependent state.
- Package provenance is assembled exactly once from member-to-peer input and supplied only to preparation, preserving untrusted-input validation at the admission boundary.
- The production and test package facades share one dispatcher helper so scenario coverage cannot drift into a parallel mutable path.
- Compact replacement bodies remain intentionally retained as bounded candidate material even after accepted victim identities and fingerprints are removed.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Extracted singleton routing to satisfy enforced code shape**

- **Found during:** Task 1
- **Issue:** Adding complete singleton preparation and dispatch to the existing bridge would exceed the repository's production-file length policy.
- **Fix:** Extracted a focused `admission_bridge/singleton.rs` module and registered its parity breadcrumb.
- **Files modified:** `packages/open-bitcoin-node/src/network/admission_bridge.rs`, `packages/open-bitcoin-node/src/network/admission_bridge/singleton.rs`, `docs/parity/source-breadcrumbs.json`
- **Verification:** Bright Builds code-shape and parity checks, all mandatory Rust gates, and the full normal hook passed.
- **Committed in:** `5f3c97b3`

**2. [Rule 3 - Blocking] Supplied the required ordered parent-source evidence anchor**

- **Found during:** Task 1 structural verification
- **Issue:** The inherited Phase 102 structural checker requires an accurate ordered-parent source anchor for package admission evidence.
- **Fix:** Added the precise comment at the real parent-order preservation boundary instead of weakening the checker.
- **Files modified:** `packages/open-bitcoin-node/src/network/admission_bridge/package.rs`
- **Verification:** The Phase 102 structural checker and full repository hook passed.
- **Committed in:** `5f3c97b3`

**3. [Rule 3 - Blocking] Updated the legacy Phase 133 structural contract**

- **Found during:** Task 2
- **Issue:** The Phase 133 checker still required direct package submission and no projection, contradicting this plan's exactly-once authoritative dispatch requirement.
- **Fix:** Replaced the obsolete assertions with exactly-one package lifecycle dispatch and authoritative projection assertions, and updated all mutation cases.
- **Files modified:** `scripts/check-phase133-package-aware-download-orphan-bridge.ts`, `scripts/check-phase133-package-aware-download-orphan-bridge.test.ts`
- **Verification:** All 30 Phase 133 mutation cases and the direct structural checker passed.
- **Committed in:** `9e5061d3`

**4. [Rule 2 - Missing Critical] Preserved exact package-dispatch probe semantics**

- **Found during:** Task 2 legacy package coverage
- **Issue:** Moving production submission behind lifecycle authority would otherwise remove the exact-call observation used to verify package candidates are dispatched once without changing runtime authority.
- **Fix:** Added a test-only managed-mempool dispatch recorder invoked after authoritative apply.
- **Files modified:** `packages/open-bitcoin-node/src/mempool.rs`, `packages/open-bitcoin-node/src/network/admission_bridge/package.rs`
- **Verification:** Legacy package bridge tests, the complete admission scenario matrix, and full Rust tests passed.
- **Committed in:** `9e5061d3`

**5. [Rule 3 - Blocking] Deferred requirement activation to lifecycle verification**

- **Found during:** Plan metadata preparation
- **Issue:** Intermediate Phase 134 summaries cannot activate `MPLIFE-01`, `MPLIFE-02`, or `MPLIFE-03` before the phase has a lifecycle-valid verification artifact.
- **Fix:** Preserved all three requirements as pending and left `requirements-completed` empty, matching the active Phase 134 traceability contract.
- **Files modified:** `.planning/phases/134-authoritative-cross-cache-lifecycle-integration/134-06-SUMMARY.md`
- **Verification:** The active milestone traceability checker remains green with requirements pending.
- **Committed in:** Plan metadata commit

**Total deviations:** 5 auto-fixed (1 missing critical, 4 blocking)
**Impact on plan:** The changes were required for repository policy compliance, exact regression coverage, and the authoritative dispatch contract; no new transport, scheduling, network endpoint, or architectural surface was added.

## Issues Encountered

- The Task 2 RED scenario failed at the expected accepted-package assertion before package dispatch was implemented, demonstrating the production projection gap. The shared dispatcher implementation made the scenario and all legacy package tests pass.
- TDD RED was not committed separately because this repository's mandatory normal commit hook requires a fully green verifier; each completed task was instead committed atomically after its RED/GREEN cycle and all gates passed.
- The resumable Task 2 hook session closed after completion while the conversation context compacted; repository history confirmed the successful normal-hook commit with a clean worktree.

## Known Stubs

None. Empty arrays found by the scan are local TypeScript checker accumulators, not runtime or user-visible placeholders.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Plans 07 and 08 can rely on admission using the sole epoch/revision-validated lifecycle authority.
- Complete admission reconciliation now provides deterministic fixtures for the remaining production routing and final phase verification.
- `MPLIFE-01`, `MPLIFE-02`, and `MPLIFE-03` remain pending until Phase 134 verification produces the lifecycle-valid completion artifact.
- No blockers remain.

## Self-Check: PASSED

- Summary and all three key created files exist.
- Task commits `5f3c97b3` and `9e5061d3` are present in repository history.
- Stub and threat-surface scans found no goal-blocking placeholder or unplanned trust-boundary surface.
- `git diff --check` passes and Markdown frontmatter uses exactly two delimiters.

***

*Phase: 134-authoritative-cross-cache-lifecycle-integration*
*Completed: 2026-07-28*
