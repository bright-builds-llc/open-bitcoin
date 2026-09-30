---
phase: 151-parity-roots-and-no-claim-guardrails
plan: "03"
subsystem: parity
tags: [parity, claims, uat, verify]

requires:
  - phase: 151-parity-roots-and-no-claim-guardrails
    provides: Fixture-tested Phase 151 checker that still expected Phase 145 then Phase 151 in scripts/verify.sh
provides:
  - Exact v2.4 prune sentence on README and the runtime guide, with the v2.3 D14 sentence kept
  - Phase 145 prune-mode denial limited to every clause except the exact v2.4 sentence and the exact Fjall difference sentence
  - Committed 151-UAT.md with public-network recorded as not run
  - scripts/verify.sh Phase 151 pair immediately after Phase 145 and before Phase 121
affects: [151-04, GRD-01]

tech-stack:
  added: []
  patterns:
    - "Phase 145 skips only the prune-mode topic when the original clause contains the exact allowed sentence"
    - "Phase 151 sits immediately after Phase 145 in both the heredoc and the run_step chain"

key-files:
  created:
    - .planning/phases/151-parity-roots-and-no-claim-guardrails/151-UAT.md
  modified:
    - scripts/check-phase145-parity-uat-release-boundary/claims.ts
    - scripts/check-phase145-parity-uat-release-boundary/constants.ts
    - scripts/check-phase145-parity-uat-release-boundary.test.ts
    - README.md
    - docs/operator/runtime-guide.md
    - docs/parity/release-readiness.md
    - docs/parity/production-claim-boundary.md
    - docs/parity/support-matrix.md
    - scripts/verify.sh
    - docs/metrics/lines-of-code.md

key-decisions:
  - "Skip Phase 145 prune-mode only when the original clause contains the exact v2.4 prune sentence or the exact Fjall difference sentence."
  - "Do not add a Fjall difference fixture, because the existing does not marker already skips that paragraph."
  - "Leave requirements-completed empty so GRD-01 stays unchecked until Plan 04."

patterns-established:
  - "The v2.4 sentence and the Fjall difference sentence stay separate from does not denial sentences."
  - "The Phase 151 checker runs immediately after Phase 145 and Phase 138 stays the file-final check-phase command."

requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 151-2026-09-29T21-06-33
generated_at: 2026-09-30T04:48:30Z

duration: 1h 23m
completed: 2026-09-30
---

# Phase 151 Plan 03: Claim Copy, UAT Package, and verify.sh Wiring Summary

**Published the verbatim v2.4 prune sentence, carved the two-sentence Phase 145 prune-mode exception, and wired the Phase 151 checker immediately after Phase 145**

## Performance

- **Duration:** 1h 23m
- **Started:** 2026-09-30T03:25:00Z
- **Completed:** 2026-09-30T04:47:46Z
- **Tasks:** 3
- **Files modified:** 11

## Accomplishments

- README and the runtime guide contain the exact v2.4 prune sentence and still contain the v2.3 D14 sentence. Archive serving, assumeutxo, BIP37, public defaults, and production readiness stay denied.
- Phase 145 still rejects `Open Bitcoin provides prune-mode product behavior.` It accepts `prune-mode` only inside a clause that contains the exact v2.4 sentence or the exact Fjall difference sentence, and it still rejects `archive-node` in that same clause.
- `151-UAT.md` records public-network review as `not run`. `scripts/verify.sh` runs the Phase 151 test and check immediately after Phase 145, before Phase 121. GRD-01 stays unchecked.

## Task Commits

Each task was committed atomically:

1. **Task 1: Exempt only the exact v2.4 sentences from the Phase 145 prune-mode denial** - `3ead2dd6` (feat)
2. **Task 2: Publish the scoped claim and the release handoff** - `e3013e8b` (docs)
3. **Task 3: Commit 151-UAT.md and wire the checker immediately after Phase 145** - `d02988bc` (feat)

**Plan metadata:** recorded in the docs commit for this summary.

## Files Created/Modified

- `scripts/check-phase145-parity-uat-release-boundary/constants.ts` - Exports `V24_PRUNE_SENTENCE` and `FJALL_DIFFERENCE_SENTENCE` and keeps `prune-mode` in `DENIED_OVERCLAIMS`.
- `scripts/check-phase145-parity-uat-release-boundary/claims.ts` - Skips only the `prune-mode` topic when the original clause contains one of those two sentences.
- `scripts/check-phase145-parity-uat-release-boundary.test.ts` - Keeps the unscoped prune-mode failure and adds the v2.4 allow and archive-node denial cases.
- `README.md` - Adds the v2.4 sentence, the Fjall difference sentence, and the parity-root links. The v2.3 blockquote sentence stays.
- `docs/operator/runtime-guide.md` - Adds the Phase 151 review with the same two sentences and the companion allowed wording.
- `docs/parity/release-readiness.md` - Adds the v2.4 release-review handoff and keeps `bash scripts/verify.sh` as the release contract.
- `docs/parity/production-claim-boundary.md` and `docs/parity/support-matrix.md` - Add the scoped v2.4 paragraph that does not add archive-node serving, assumeutxo, BIP37, public defaults, or production readiness.
- `.planning/phases/151-parity-roots-and-no-claim-guardrails/151-UAT.md` - Cargo and Bazel operator forms, `clearprunelock` labeled as an Open Bitcoin extension, and public-network `not run`.
- `scripts/verify.sh` - Phase 151 test and check immediately after Phase 145 in the ordering comment, heredoc, and `run_step` chain.
- `docs/metrics/lines-of-code.md` - Pre-commit refreshed the tracked line-count report.

## Decisions Made

- Phase 145 leaves `prune-mode` on the denial list. The skip applies only when `topic === "prune-mode"` and the original clause contains `V24_PRUNE_SENTENCE` or `FJALL_DIFFERENCE_SENTENCE`.
- The Fjall difference fixture was not added. A paragraph of `Open Bitcoin provides prune-mode product behavior via this exact wording: ` plus `FJALL_DIFFERENCE_SENTENCE` already passed before the code skip, because that sentence contains `does not`. The code skip remains so the topic stays exempt if the marker list changes.
- `requirements-completed` stays empty. `.planning/REQUIREMENTS.md` still has `- [ ] **GRD-01**`.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Kept the v2.4 sentence and the prune-mode suffix in one claim clause**
- **Found during:** Task 1 (Exempt only the exact v2.4 sentences from the Phase 145 prune-mode denial)
- **Issue:** `V24_PRUNE_SENTENCE` ends with a period. A following space splits it into a second clause that still contains `prune-mode` and no longer contains the exact sentence, so the exception cannot apply.
- **Fix:** The allow and archive-node fixtures append the following words with no space after that period. The exact sentence remains inside the same clause.
- **Files modified:** `scripts/check-phase145-parity-uat-release-boundary.test.ts`
- **Verification:** The unscoped prune-mode test failed the checker before the skip, then the one-clause v2.4 test passed after the skip. `bun test scripts/check-phase145-parity-uat-release-boundary.test.ts` exits 0.
- **Committed in:** `3ead2dd6` (Task 1 commit)

**2. [Rule 3 - Blocking] TDD red and green shipped in one hook-passing commit**
- **Found during:** Task 1 commit
- **Issue:** A RED-only commit of the failing v2.4 accept test cannot pass the pre-commit `bash scripts/verify.sh` hook, and `--no-verify` is not allowed.
- **Fix:** Proved the accept test failed, implemented the skip, proved the suite passed, then committed the tests and the exception together.
- **Files modified:** Phase 145 checker constants, claims, and tests
- **Verification:** Focused bun test failed once on `accepts_prune_mode_inside_the_exact_v2_4_prune_sentence`, then the full file passed 24 tests.
- **Committed in:** `3ead2dd6` (Task 1 commit)

**3. [Rule 1 - Bug] Restored the Phase 145 heading after the new Phase 151 section**
- **Found during:** Task 2 (Publish the scoped claim and the release handoff)
- **Issue:** Inserting `## Phase 151` immediately after the v2.3 sentence would place the existing Phase 145 UAT commands and the `v2.3 does not add prune-mode` paragraph under the Phase 151 heading.
- **Fix:** Restated `## Phase 145 Parity Roots And No-Claim Guardrails Review` after the short Phase 151 section so the v2.3 review stays labeled.
- **Files modified:** `docs/operator/runtime-guide.md`
- **Verification:** The v2.3 sentence remains above the Phase 151 heading, and the live Phase 145 checker printed `Phase 145 parity UAT release boundary validated.`
- **Committed in:** `e3013e8b` (Task 2 commit)

---

**Total deviations:** 3 auto-fixed (2 blocking, 1 bug)
**Impact on plan:** The exception, the claim copy, and the verifier insertion match the plan. The fixture spacing and the restored heading keep the clause classifier and the Phase 145 section accurate. No scope creep.

## Issues Encountered

None

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Plan 151-04 can pin the live checker and flip GRD-01. This plan left `- [ ] **GRD-01**` unchecked and did not archive the milestone. No `blk`/`rev` store, runtime prune change, or public-network release gate was added.

Ready for 151-04.

## Self-Check: PASSED

- FOUND: .planning/phases/151-parity-roots-and-no-claim-guardrails/151-UAT.md
- FOUND: scripts/verify.sh Phase 151 pair immediately after Phase 145 and before Phase 121
- FOUND: README.md and docs/operator/runtime-guide.md contain the verbatim v2.4 sentence and the v2.3 D14 sentence
- FOUND: 3ead2dd6b7acfc580f6bbb6fc29a95e0cc519514
- FOUND: e3013e8b4a8cf81db0555e04ab5b213fc765fdc9
- FOUND: d02988bc0d5c38b0981c38d19538cf44f2834391
- FOUND: GRD-01 still unchecked as `- [ ] **GRD-01**`
- FOUND: bun run scripts/check-phase151-parity-uat-release-boundary.ts printed `Phase 151 parity UAT release boundary validated.`

---
*Phase: 151-parity-roots-and-no-claim-guardrails*
*Completed: 2026-09-30*
