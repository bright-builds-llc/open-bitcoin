---
phase: 129-integration-guardrails-and-milestone-reconciliation
plan: "01"
subsystem: verification
tags: [integration-guard, deterministic-checker, flows, verify-sh]
requires:
  - phase: 127-authoritative-network-state-unification
    provides: Shared-authority seam guard and FLOW-01/FLOW-04 production anchors
  - phase: 128-production-compact-announcement-transport
    provides: sendcmpct/transport/post-write seam guard and FLOW-02/FLOW-03 anchors
provides:
  - Aggregate fail-closed guard composing the Phase 127/128 seam checkers under one surface
  - Named FLOW-01 through FLOW-04 production-path anchor assertions with distinct P129 failure strings
  - verify.sh wiring self-assertion and Phase 117 final-gate preservation check
  - Exported PHASE127_TARGET_FILES and PHASE127_DAEMON_HELPER_DIR for fixture composition
affects: [129-02, 129-03, 129-04, verify-contract]
tech-stack:
  added: []
  patterns:
    - Aggregate checker composes upstream exported check functions with a shared repoRoot instead of duplicating seam anchors
    - Live-clone mutation fixtures materialize the union of upstream target corpora plus the daemon helper directory
key-files:
  created:
    - scripts/check-phase129-integration-guardrails-and-milestone-reconciliation.ts
    - scripts/check-phase129-integration-guardrails-and-milestone-reconciliation.test.ts
  modified:
    - scripts/check-phase127-authoritative-network-state-unification.ts
    - scripts/verify.sh
    - docs/metrics/lines-of-code.md
key-decisions:
  - Compose checkPhase127/checkPhase128 by import (D-03) rather than re-asserting shared corpus anchors, so seam coverage stays in sync automatically.
  - Keep the Phase 129 checker stage-independent - no volatile planning-artifact assertions (Pitfall 12); artifact-state assertions stay with the Phase 124 stage machine.
  - Assert the FLOW-01/FLOW-04 shared production-composition anchor as two table rows so each flow keeps its own named failure string.
patterns-established:
  - "P129 FLOW-0N" failure vocabulary names each repaired end-to-end flow explicitly in the default verification contract.
  - Final-gate preservation is asserted per surface (heredoc and run_step lines) with a P129 final gate prefix.
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 129-2026-07-20T19-28-06
generated_at: 2026-07-20T21:45:00Z
duration: 27 min
completed: 2026-07-20
---

# Phase 129 Plan 01: Aggregate Integration Guard Summary

One deterministic Bun checker pair composes the Phase 127/128 seam guards, names FLOW-01 through FLOW-04 with distinct fail-closed strings, asserts its own verify.sh wiring, and preserves Phase 117 as the final gate — wired between Phase 128 and Phase 117 in default verification.

## Performance

- **Duration:** 27 min (excluding pre-commit hook verification)
- **Started:** 2026-07-20T21:17:18Z
- **Completed:** 2026-07-20T21:44:00Z
- **Tasks:** 3
- **Files modified:** 5

## Accomplishments

- Created `scripts/check-phase129-integration-guardrails-and-milestone-reconciliation.ts`: composes `checkPhase127AuthoritativeNetworkStateUnification` and `checkPhase128ProductionCompactAnnouncementTransport` with a shared repo root (six repaired seams, D-03), asserts the four named flow anchors (D-04), self-asserts its verify.sh heredoc/run_step wiring (D-02), preserves the Phase 117 final gate (D-15), and self-scans for forbidden effect tokens built by concatenation (D-16).
- Exported `PHASE127_TARGET_FILES` and `PHASE127_DAEMON_HELPER_DIR` from the Phase 127 checker so the Phase 129 fixture can materialize the composed corpus; the Phase 127 checker and its 15-test mutation suite pass unchanged.
- Added a 12-test live-clone mutation suite covering every Phase 129 assertion family, including composed `P127`- and `P128`-prefixed seam failures surfacing through the aggregate checker.
- Wired all three verify.sh surfaces together: appended one ordering-comment sentence, inserted the Phase 129 test+check pair into the `VERIFY_COMMAND_ORDER` heredoc and the live `run_step` block between Phase 128 and Phase 117.
- Full `bash scripts/verify.sh` passed end-to-end (5m 39s) with the Phase 129 steps visible, and the pre-commit hook verification passed again at commit time.

## Task Commits

The plan was committed as a single batch per the plan's explicit commit batching:

1. **Tasks 1-3: Phase 129 aggregate guard, mutation suite, verify.sh wiring** - `956f60f2`

## Files Created/Modified

- `scripts/check-phase129-integration-guardrails-and-milestone-reconciliation.ts` - Aggregate cross-phase seam and flow guard composing the Phase 127/128 exports.
- `scripts/check-phase129-integration-guardrails-and-milestone-reconciliation.test.ts` - Fixture-based mutation coverage for every Phase 129 assertion family.
- `scripts/check-phase127-authoritative-network-state-unification.ts` - Exported `PHASE127_TARGET_FILES` and `PHASE127_DAEMON_HELPER_DIR`; no assertion changes.
- `scripts/verify.sh` - Ordering comment, heredoc, and run_step wiring for the Phase 129 pair between Phase 128 and Phase 117.
- `docs/metrics/lines-of-code.md` - Required freshness regeneration for the new checker files.

## Deviations from Plan

None - plan executed exactly as written.

## Verification Evidence

- `bun run scripts/check-phase129-integration-guardrails-and-milestone-reconciliation.ts` exits 0 on the current repo.
- `bun test scripts/check-phase129-integration-guardrails-and-milestone-reconciliation.test.ts` passes 12 tests (1 complete-corpus pass + 11 mutation rows, including the FLOW-01/FLOW-04 shared-anchor pair).
- `bun test scripts/check-phase127-authoritative-network-state-unification.test.ts` still passes (15 tests) after the export rename.
- `rg -c "check-phase129-integration-guardrails-and-milestone-reconciliation" scripts/verify.sh` reports exactly 4 occurrences (2 heredoc + 2 run_step), positioned after Phase 128 and before Phase 117 in both surfaces.
- `rg -n "fetch\(|Bun\.spawn|node:child_process" scripts/check-phase129-...ts` produces no match; forbidden tokens appear only concatenated.
- `bash scripts/verify.sh` (via `command-timings.ts run --key verify-full`) exited 0 in 5m 39s with the Phase 129 test and check steps in the run.
- `git status --porcelain` on `.planning/ROADMAP.md`, `.planning/REQUIREMENTS.md`, `.planning/PROJECT.md`, `.planning/MILESTONES.md`, and `.planning/v2.1-MILESTONE-AUDIT.md` was empty — no reconciliation-guarded artifact moved early.

## README Review

Reviewed `README.md` status wording against the new guard: the Phase 129 sentence ("Phase 129 retains four-flow integration, soak and parity closure, requirement promotion, and the milestone audit/release handoff; the milestone is not yet archive-ready") remains true — this plan adds verification only, and reconciliation stays with plans 03/04. No README change needed.

## Known Stubs

None - the checker and test suite are fully wired into default verification with no placeholder logic.

## Next Steps

- Plan 02: flow verification and the D-06 fallback-counter decision procedure.
- Plan 03: Phase 124 stage-machine evolution for the archive-ready projection.
- Plan 04: verification-gated requirement promotion and milestone reconciliation.

## Self-Check: PASSED

- FOUND: scripts/check-phase129-integration-guardrails-and-milestone-reconciliation.ts
- FOUND: scripts/check-phase129-integration-guardrails-and-milestone-reconciliation.test.ts
- FOUND: .planning/phases/129-integration-guardrails-and-milestone-reconciliation/129-01-SUMMARY.md
- FOUND: commit 956f60f2
