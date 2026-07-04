---
phase: 110-block-serving-activation-and-eligibility-boundary
plan: 04
subsystem: docs-parity-verification
tags: [block-serving, compact-relay, docs, parity, verifier, no-claim-guardrails]
requires:
  - phase: 110-block-serving-activation-and-eligibility-boundary
    provides: activation, eligibility, status, resource, and cleanup contracts from Plans 110-01 through 110-03
  - phase: 106-parity-traceability-uat-and-release-boundary-guardrails
    provides: v2.0 parity and no-claim guardrail pattern
  - phase: 108-durable-mempool-relay-state-recovery
    provides: last verifier guardrail before Phase 110
provides:
  - docs and parity evidence for the bounded Phase 110 block-serving boundary
  - deterministic Phase 110 no-claim checker and mutation tests
  - default verifier wiring for Phase 110 boundary checks
affects: [phase-111, phase-112, phase-113, phase-114, phase-115, phase-116, phase-117, block-serving, compact-relay, parity, verification]
tech-stack:
  added: []
  patterns: [deterministic Bun checker, parity surface evidence, scoped no-claim scanning]
key-files:
  created:
    - .planning/phases/110-block-serving-activation-and-eligibility-boundary/110-04-SUMMARY.md
    - scripts/check-phase110-block-serving-boundary.ts
    - scripts/check-phase110-block-serving-boundary.test.ts
  modified:
    - docs/architecture/config-precedence.md
    - docs/architecture/status-snapshot.md
    - docs/architecture/operator-observability.md
    - docs/operator/runtime-guide.md
    - docs/parity/catalog/p2p.md
    - docs/parity/checklist.md
    - docs/parity/index.json
    - docs/metrics/lines-of-code.md
    - scripts/verify.sh
key-decisions:
  - "Phase 110 evidence is documented as a default-off policy, status, resource, and cleanup boundary only."
  - "The checker scans Phase 110-specific units for positive overclaims while leaving historical no-claim rows alone."
  - "Runtime, checklist, and P2P docs point to status-snapshot and index for status labels to avoid support-maturity vocabulary conflicts."
patterns-established:
  - "New milestone-boundary docs are paired with deterministic checker tests and verifier wiring before phase closeout."
  - "Public-network block-serving or compact-relay review remains opt-in UAT outside the scripts/verify.sh verifier."
requirements-completed: [BSRV-01, BSRV-02, BSRV-03, BSRV-05, BSRV-06]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 110-2026-07-04T02-39-48
generated_at: 2026-07-04T08:27:40Z
duration: 45m
completed: 2026-07-04
---

# Phase 110 Plan 04: Docs, Parity Evidence, and Default-Off Guardrails Summary

**Phase 110 now has docs, parity roots, and deterministic verifier guardrails proving the bounded block-serving boundary without claiming public serving, BIP152, archive-node, package relay, filter serving, or production readiness behavior.**

## Performance

- **Duration:** 45m
- **Started:** 2026-07-04T07:42:19Z
- **Completed:** 2026-07-04T08:27:40Z
- **Tasks:** 2
- **Files modified:** 11

## Accomplishments

- Updated architecture, operator, and parity docs to describe Phase 110 as a default-off activation, eligibility, status, resource-governance, and cleanup boundary only.
- Added the parity surface `v2-1-block-serving-activation-eligibility-boundary` with BSRV-01, BSRV-02, BSRV-03, BSRV-05, BSRV-06, and Knots anchors.
- Added `scripts/check-phase110-block-serving-boundary.ts` and mutation tests for missing evidence, missing verifier wiring, and forbidden public/default/archive/BIP152/response/package claims.
- Wired Phase 110 checker tests and checker execution into the `scripts/verify.sh` Bash verifier after the Phase 108 guardrails and before broader pure-core/build verification.
- Kept public-network block-serving and compact-relay review as opt-in UAT guidance only, outside default deterministic verification.
- Regenerated the tracked LOC report after the new checker and tests landed.

## Task Commits

The two plan tasks were committed together because the checker validates the docs and parity roots it introduces:

1. **Tasks 1 and 2: Docs/parity evidence plus checker/verifier guardrails** - `843768ae`

## Validation Evidence

- `bun test scripts/check-phase110-block-serving-boundary.test.ts` passed with 6 mutation tests.
- `bun run scripts/check-phase110-block-serving-boundary.ts` passed.
- `bun run scripts/check-parity-breadcrumbs.ts --check` passed and verified 348 Rust files.
- Plan acceptance `rg` probes passed for config keys, CLI flags, status/resource/cleanup labels, BSRV requirement IDs, parity surface ID, and verifier wiring.
- `git diff --check -- scripts/check-phase110-block-serving-boundary.ts scripts/check-phase110-block-serving-boundary.test.ts scripts/verify.sh docs/architecture/config-precedence.md docs/architecture/status-snapshot.md docs/architecture/operator-observability.md docs/operator/runtime-guide.md docs/parity/catalog/p2p.md docs/parity/checklist.md docs/parity/index.json docs/metrics/lines-of-code.md` passed.
- `bash scripts/check-file-lengths.sh` passed.
- The `scripts/verify.sh` Bash verifier passed in 11m 17.325s.
- The implementation commit hook for `843768ae` reran the repo verifier and passed.

## Files Created/Modified

- `scripts/check-phase110-block-serving-boundary.ts` - Deterministic Phase 110 boundary checker.
- `scripts/check-phase110-block-serving-boundary.test.ts` - Mutation tests for required evidence, no-claim guardrails, and verifier wiring.
- `scripts/verify.sh` - Default local verifier wiring for Phase 110 checker tests and checker execution.
- `docs/architecture/config-precedence.md` - Phase 110 activation keys and default-off config evidence.
- `docs/architecture/status-snapshot.md` - Shared `BlockServingEvidenceStatus` labels and boundary wording.
- `docs/architecture/operator-observability.md` - Operator evidence and sanitized fixed-label reporting notes.
- `docs/operator/runtime-guide.md` - Repo-local Cargo/Bazel operator commands and opt-in UAT boundary wording.
- `docs/parity/catalog/p2p.md` - Phase 110 P2P parity surface with Knots anchors.
- `docs/parity/checklist.md` - Phase 110 checklist entry and no-claim pointers.
- `docs/parity/index.json` - Machine-readable parity surface entry.
- `docs/metrics/lines-of-code.md` - Regenerated tracked LOC report.

## Decisions Made

- Phase 110 docs intentionally claim only policy, status, resource, cleanup, and verifier guardrails; full block responses, compact relay, BIP152 codecs, reconstruction, and fallback stay deferred.
- The checker scans Phase 110-specific evidence units instead of every historical no-claim row so older release-boundary docs do not create false positives.
- Runtime/checklist/P2P docs avoid restating support-maturity status labels directly and point to status-snapshot/index evidence where those labels are owned.
- The default verifier remains local and deterministic; public-network review stays an opt-in UAT activity.

## Deviations from Plan

### Auto-fixed Issues

**1. LOC report needed worktree-source regeneration**

- **Found during:** Task 2 verifier wiring.
- **Issue:** The first LOC regeneration mode left the tracked report stale against the full verifier.
- **Fix:** Regenerated `docs/metrics/lines-of-code.md` from the worktree source before rerunning verification.
- **Verification:** The `scripts/verify.sh` Bash verifier accepted the refreshed LOC report.
- **Committed in:** `843768ae`

**2. Avoided legacy Phase 63 service wording conflict**

- **Found during:** Full `scripts/verify.sh` verification.
- **Issue:** A new runtime-guide sentence used the exact phrase rejected by the existing Phase 63 lifecycle checker.
- **Fix:** Changed the new Phase 110 wording to `production-service operation` while keeping the same no-claim meaning.
- **Verification:** The Phase 63 checker passed inside the full verifier.
- **Committed in:** `843768ae`

**3. Avoided legacy Phase 83 support-maturity label conflict**

- **Found during:** Full `scripts/verify.sh` verification.
- **Issue:** The older Phase 83 checker rejects support-maturity label wording in runtime, checklist, and P2P docs.
- **Fix:** Kept the required status labels in status-snapshot, operator-observability, and `docs/parity/index.json`, and changed runtime/checklist/P2P docs to point at those sources.
- **Verification:** The Phase 83 and Phase 110 checkers both passed inside the full verifier.
- **Committed in:** `843768ae`

**Total deviations:** 3 auto-fixed issues.
**Impact on plan:** No capability scope changed; the fixes tightened compatibility with existing no-claim and support-maturity guardrails.

## Issues Encountered

- Bazel emitted existing secp256k1-sys warnings during the smoke build, but the build and full verifier completed successfully.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Phase 111 can build the full block-serving request path on top of the guarded Phase 110 activation, eligibility, status, resource, cleanup, docs, parity, and verifier boundary. Phase 112+ compact-relay work remains explicitly deferred and guarded by the new no-claim checker.

## Self-Check: PASSED

- [x] Docs and parity roots describe Phase 110 as default-off boundary evidence only.
- [x] BSRV-01, BSRV-02, BSRV-03, BSRV-05, and BSRV-06 are represented in Phase 110 parity evidence.
- [x] Checker tests cover missing evidence, forbidden overclaims, allowed no-claim wording, and verifier omission.
- [x] `scripts/verify.sh` runs the Phase 110 checker locally and deterministically.
- [x] Public serving defaults, archive-node behavior, package relay, filter serving, BIP152, compact reconstruction, production readiness, production service operation, and production-funds wallet use remain unclaimed.
