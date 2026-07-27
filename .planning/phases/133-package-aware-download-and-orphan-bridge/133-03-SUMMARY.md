---
phase: 133-package-aware-download-and-orphan-bridge
plan: "03"
subsystem: node
tags: [rust, package-admission, orphanage, reject-evidence, bitcoin-knots-parity]

requires:
  - phase: 133-01
    provides: Typed fixed-memory hard and reconsiderable reject evidence
  - phase: 133-02
    provides: Opaque newest-first same-peer 1P1C candidates with bounded announcer provenance
provides:
  - Exact remaining-parent evidence in authoritative package reports
  - One-call node-owned refinement and package admission preserving report, fingerprint, and delta
  - Exhaustive bounded package feedback without Phase 134 or Phase 136 projection
affects: [133-04, 134, 136, package-admission, transaction-relay]

tech-stack:
  added: []
  patterns:
    - Typed package reports as the sole hard-versus-reconsiderable classifier
    - Exact authoritative package truth retained at the node composition boundary
    - Exhaustive feedback limited to orphan and reject-evidence correctness

key-files:
  created:
    - packages/open-bitcoin-node/src/network/admission_bridge/package.rs
    - packages/open-bitcoin-node/src/network/tests/package_bridge_cases.rs
  modified:
    - packages/open-bitcoin-mempool/src/pool/candidate.rs
    - packages/open-bitcoin-mempool/src/package/report.rs
    - packages/open-bitcoin-mempool/src/pool/package_admission.rs
    - packages/open-bitcoin-network/src/peer/transaction_relay/orphanage.rs
    - packages/open-bitcoin-node/src/mempool.rs
    - packages/open-bitcoin-node/src/network/admission_bridge.rs
    - packages/open-bitcoin-node/src/network/action_translation.rs

key-decisions:
  - "Classify peer singletons through the authoritative package vocabulary; preserve ordinary singleton RBF only through an exact typed package-replacement fallback."
  - "Keep package reports and lifecycle deltas opaque at the node boundary until Phase 134, while applying only bounded candidate-state feedback."
  - "Carry retained child announcer provenance through the opaque network candidate so exact missing-input restaging never reconstructs ownership."

patterns-established:
  - "One-call boundary: refine an opaque candidate once and return SubmittedPackageResult unchanged."
  - "Feedback boundary: terminal members retire, true missing inputs restage exactly, and hard/transaction/package evidence remain distinct."

requirements-completed: []
requirements-addressed: [PPKG-02, PPKG-03]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 133-2026-07-26T16-12-51
generated_at: 2026-07-26T21:15:14Z

duration: 1h 4m
completed: 2026-07-26
---

# Phase 133 Plan 03: Authoritative Package Admission Bridge Summary

**Exact missing-parent reports now drive one-call same-peer 1P1C admission with unchanged package truth and exhaustive bounded feedback.**

## Performance

- **Duration:** 1h 4m
- **Started:** 2026-07-26T20:11:03Z
- **Completed:** 2026-07-26T21:15:14Z
- **Tasks:** 3
- **Files modified:** 20

## Accomplishments

- Added deterministic, sorted, deduplicated remaining-parent vectors to typed missing-input package results.
- Added the node-owned candidate refinement seam and thin managed mempool adapter, preserving exact report fingerprint, member order, and lifecycle delta.
- Applied exhaustive D-12 feedback for every member and package-status variant without serving, fanout, compact, persistence, retry, unbroadcast, or operator projection.
- Covered both arrival orders, different deliverer versus qualifying announcer, wrong peer, newest failed-fingerprint fallback, multi-parent/grandchild exclusion, exact call counts, and evidence-domain separation.

## Task Commits

1. **Task 1: Carry exact remaining parents in missing-input reports** - `3fa7c312`
2. **Task 2: Submit one refined candidate and preserve exact authoritative truth** - `3c50b770`
3. **Task 3: Apply exhaustive bounded feedback and integration proof** - `8d391dad`

## Decisions Made

- The typed package report is authoritative for hard, missing-input, fee, and replacement classification; no string or coarse outcome heuristic is used.
- Ordinary singleton RBF falls back to the existing transition only for the exact typed one-member package-replacement shape rejection.
- Package feedback mutates only orphan retention and typed reject evidence; all lifecycle-delta projection remains deferred.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 2 - Missing Critical] Split production modules to preserve the 628-line contract**
- **Found during:** Tasks 1 and 3
- **Issue:** Exact candidate calculation and provenance feedback exceeded production file limits.
- **Fix:** Extracted mempool candidate calculation and moved candidate proof types into existing child modules.
- **Committed in:** `3fa7c312`, `8d391dad`

**2. [Rule 3 - Blocking] Closed strict coverage and historical guard gaps**
- **Found during:** Tasks 1 and 2 normal hooks
- **Issue:** Strict coverage and the Phase 102 source guard required direct narrow-seam evidence.
- **Fix:** Added focused selector/adapter tests and retained the expected bridge anchor.
- **Committed in:** `3fa7c312`, `3c50b770`

**3. [Rule 2 - Missing Critical] Added exhaustive RPC mapping for the new typed network error**
- **Found during:** Task 2 full verification
- **Issue:** `ManagedNetworkError::PackageShape` made RPC error dispatch non-exhaustive.
- **Fix:** Mapped the typed shape error to the existing internal RPC failure contract with regression coverage.
- **Committed in:** `3c50b770`

**4. [Rule 2 - Missing Critical] Preserved retained announcers through candidate feedback**
- **Found during:** Task 3 API audit
- **Issue:** The opaque candidate carried aligned origins but dropped retained child announcers needed for exact MissingInputs restaging.
- **Fix:** Added a consume-only provenance-bearing candidate accessor while preserving the existing ordered-parts API.
- **Committed in:** `8d391dad`

**Total deviations:** 4 auto-fixed (3 missing-critical, 1 blocking)

## Verification

- Focused missing-parent package suite: 18 passed.
- Focused package bridge suite: 5 passed; retained-provenance network regression passed.
- Task 3 normal hook completed the full repository contract in 5m 13.486s, including formatting, Clippy with warnings denied, all-target build, all-feature tests, strict coverage, architecture/file-length checks, and Bazel smoke.
- Parity breadcrumbs verified for 444 Rust files; tracked LOC report refreshed to 276,904 lines.

## Authentication Gates

None.

## Known Stubs

None.

## Threat Flags

None. No endpoint, authentication path, file-access boundary, or schema trust surface was introduced.

## User Setup Required

None.

## Next Phase Readiness

- Plan 133-04 can close deterministic parity and source-guard evidence over the completed bridge.
- Phase 134 can consume the preserved package report and lifecycle delta for full cache projection.
- No Plan 03 blockers remain.

## Self-Check: PASSED

- Summary and all key implementation files exist.
- Task commits `3fa7c312`, `3c50b770`, and `8d391dad` exist in repository history.
- Stub and threat-surface scans found no goal-blocking placeholders or unplanned trust boundaries.
