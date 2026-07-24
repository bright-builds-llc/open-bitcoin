---
phase: 130-resource-time-and-fee-primitives
plan: "12"
subsystem: parity-docs
tags: [parity, readme, breadcrumbs, fee-primitives, resource-accounting, documentation]
requires:
  - phase: 130-resource-time-and-fee-primitives
    provides: Implemented resource, fee, metadata, context, lifecycle, retry, and RPC contracts from Plans 130-01 through 130-11
provides:
  - Unique FEEP-01 through FEEP-05 parity surface ownership under v2-2-resource-time-fee-primitives
  - Mempool policy catalog documentation for version-1 accounting, fee roles, contexts, lifecycle, RPC mappings, and later-phase boundaries
  - Contributor README truth for active v2.2 Phase 130 versus shipped v2.1
affects: [phase-130-13, phase-131, phase-132, phase-134, phase-135, phase-136, FEEP-01, FEEP-02, FEEP-03, FEEP-04, FEEP-05]
tech-stack:
  added: []
  patterns:
    - Active-milestone README status with exact Phase 130 and mempool catalog anchors
    - Documentation reconciliation requires active v2.2 README anchors while historical v2.1 archive routes remain on release-readiness
key-files:
  created: []
  modified:
    - docs/parity/catalog/mempool-policy.md
    - docs/parity/checklist.md
    - docs/parity/index.json
    - docs/parity/source-breadcrumbs.json
    - README.md
    - packages/README.md
    - docs/parity/README.md
    - scripts/check-current-documentation-reconciliation.ts
    - scripts/check-current-documentation-reconciliation.test.ts
key-decisions:
  - "Register unique machine ownership of FEEP-01 through FEEP-05 under v2-2-resource-time-fee-primitives without claiming later-phase enforcement or package execution."
  - "Document the intentional Rust-owned accounting formula difference from C++ allocator estimates while preserving Knots RPC field meanings."
  - "Align documentation reconciliation with active v2.2 README truth instead of requiring /gsd-new-milestone in the root status block."
patterns-established:
  - "Phase 130 parity evidence cites exact deferred boundaries for Phases 131, 132, 134, 135, and 136."
  - "Contributor README surfaces distinguish active milestone work from the latest shipped release claim."
requirements-completed: []
requirements-addressed: [FEEP-01, FEEP-02, FEEP-03, FEEP-04, FEEP-05]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 130-2026-07-23T14-26-46
generated_at: 2026-07-24T06:42:35Z
duration: 40 min
completed: 2026-07-24
---

# Phase 130 Plan 12: Parity Evidence and README Truth Summary

**Auditable FEEP ownership is registered under `v2-2-resource-time-fee-primitives`, the mempool catalog records Phase 130 contracts and deferred boundaries, and contributor READMEs now state active v2.2 Phase 130 work without stale milestone boilerplate**

## Performance

- **Duration:** 40 min
- **Started:** 2026-07-24T06:02:15Z
- **Completed:** 2026-07-24T06:42:35Z
- **Tasks:** 2
- **Files modified:** 23

## Accomplishments

- Registered `v2-2-resource-time-fee-primitives` with unique FEEP-01 through FEEP-05 ownership in `docs/parity/index.json` and `docs/parity/checklist.md`.
- Expanded `docs/parity/catalog/mempool-policy.md` with the version-1 Rust-owned accounting formula, intentional allocator-estimate difference, four fee roles, metadata/contexts, lifecycle invariants, RPC mappings, retry inputs, and exact Phase 131/132/134/135/136 boundaries.
- Strengthened Knots breadcrumb anchors for fee/RPC and admission-context mappings and verified all Phase 130 production/test registrations.
- Refreshed root, packages, and parity README surfaces with required positive anchors and removed stale v2.1-active / new-milestone wording.
- Aligned the documentation reconciliation checker so root README active-v2.2 truth no longer conflicts with historical archive routing.

## Task Commits

1. **Task 1: Register complete parity evidence and deferred boundaries** - `0ce036c0` (docs)
2. **Task 2: Refresh relevant contributor README surfaces** - `6c8f81e9` (docs)

**Plan metadata:** `d867d9ae` (docs: complete plan)

## Files Created/Modified

- `docs/parity/catalog/mempool-policy.md` - Phase 130 resource/fee/metadata/lifecycle/RPC catalog and deferred boundaries.
- `docs/parity/checklist.md` - Human checklist row for `v2-2-resource-time-fee-primitives`.
- `docs/parity/index.json` - Machine-readable FEEP surface ownership.
- `docs/parity/source-breadcrumbs.json` - Strengthened Knots anchors for fee and context groups.
- Breadcrumb comment sync across mempool fee/context-related Rust files.
- `README.md` - Active milestone v2.2 / Phase 130 status with mempool catalog link and shipped v2.1 archive evidence.
- `packages/README.md` - Exact crate responsibility phrases for mempool, network, node, and RPC.
- `docs/parity/README.md` - Active v2.2 evidence root distinguished from shipped v2.1.
- `scripts/check-current-documentation-reconciliation.ts` - Active README contract for v2.2.
- `scripts/check-current-documentation-reconciliation.test.ts` - Mutation updated for active-milestone wording.

## Decisions Made

- FEEP requirements are uniquely owned by the new Phase 130 surface; later phases receive exact deferred boundaries rather than overlapping ownership.
- Root README must advertise active Phase 130 work and must not tell contributors to start a new milestone.
- Historical v2.1 archive routing (`/gsd-new-milestone`) remains required on release-readiness and planning archive headers, not on the active root status block.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Aligned documentation reconciliation with Plan 12 README contract**
- **Found during:** Task 2
- **Issue:** `scripts/check-current-documentation-reconciliation.ts` still required `/gsd-new-milestone` and treated the root README as an archived-milestone status block, which blocked the planned README refresh.
- **Fix:** Added `verifyActiveReadmeState` for active v2.2 / Phase 130 / shipped-v2.1 anchors, kept archive-route checks on release-readiness and planning headers, and updated the README mutation test.
- **Files modified:** `scripts/check-current-documentation-reconciliation.ts`, `scripts/check-current-documentation-reconciliation.test.ts`, `README.md`
- **Verification:** `bun test scripts/check-current-documentation-reconciliation.test.ts` and `bun run scripts/check-current-documentation-reconciliation.ts` passed; full hook verify passed on commit.
- **Committed in:** `6c8f81e9`

**2. [Rule 3 - Blocking] Kept FEEP requirements Pending until Phase 130 VERIFICATION.md exists**
- **Found during:** Final state updates
- **Issue:** `requirements mark-complete` activated FEEP-01..05 without lifecycle-valid verification coverage, failing `check-active-milestone-verification-traceability`.
- **Fix:** Reverted REQUIREMENTS.md checkboxes/traceability to Pending and kept SUMMARY `requirements-completed: []` with `requirements-addressed` only, matching Plans 130-01..11 practice.
- **Files modified:** `.planning/REQUIREMENTS.md`, `130-12-SUMMARY.md`
- **Verification:** FEEP rows are Pending; SUMMARY frontmatter does not claim completion.

**Total deviations:** 2 auto-fixed (2 blocking)
**Impact on plan:** Required for README truth and milestone-traceability verifier coherence; no Phase 130 scope expansion.

## Issues Encountered

- Host `rg -Eq` rejects combined flags on this environment; verification used `rg -q -e` for the public/default/production/package assertions while preserving exact Plan 12 positive and negative checks.

## Authentication Gates

None.

## Known Stubs

None.

## Threat Flags

None — no new network endpoints, auth paths, file-access patterns, or trust-boundary schema changes beyond planned documentation surfaces.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Plan 130-13 can freeze deterministic claim guardrails over the registered parity evidence and refreshed README anchors.
- No blockers remain for Phase 130 closeout planning.

## Self-Check: PASSED

- Created/modified files exist: `docs/parity/catalog/mempool-policy.md`, `docs/parity/index.json`, `README.md`, `packages/README.md`, `docs/parity/README.md`.
- Task commits `0ce036c0` and `6c8f81e9` exist.
- Surface `v2-2-resource-time-fee-primitives` owns exactly FEEP-01 through FEEP-05.
- Breadcrumb checker passes.
- README positive/negative assertions from Plan 12 verify independently.

---
*Phase: 130-resource-time-and-fee-primitives*
*Completed: 2026-07-24*
