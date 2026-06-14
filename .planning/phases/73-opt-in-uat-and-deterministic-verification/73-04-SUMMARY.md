---
phase: 73-opt-in-uat-and-deterministic-verification
plan: 04
subsystem: verification
tags: [uat, parity, deterministic-verification, breadcrumbs, bun, verify-sh]

# Dependency graph
requires:
  - phase: 73-opt-in-uat-and-deterministic-verification
    plan: 03
    provides: Default verifier wiring for the Phase 73 deterministic checker
provides:
  - Phase 73 parity root documentation for UAT, fixture, compatibility-harness, support-bundle, live-smoke, checker, and breadcrumb evidence
  - Checker enforcement for Phase 73 parity roots, breadcrumb registry coverage, deferred-scope non-claims, and live-smoke evidence files
  - Auditable closeout that keeps production-node, relay, production-wallet, migration-apply, packaging, GUI, hosted-dashboard, public-network CI, and release-blocking live-sync claims out of scope
affects: [repo-verification, parity-docs, operator-uat, source-breadcrumbs, generated-metrics]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - Fixture-rooted Bun checker tests for parity closeout drift
    - Source-breadcrumb registry assertions paired with the repo breadcrumb checker
    - Exact deferred-scope wording checks that allow non-claims without accepting production-readiness claims

key-files:
  created:
    - .planning/phases/73-opt-in-uat-and-deterministic-verification/73-04-SUMMARY.md
  modified:
    - docs/parity/catalog/p2p.md
    - docs/parity/catalog/chainstate.md
    - docs/parity/catalog/operator-runtime-release-hardening.md
    - docs/parity/index.json
    - docs/parity/checklist.md
    - docs/parity/README.md
    - scripts/check-phase73-uat-verification.ts
    - scripts/check-phase73-uat-verification.test.ts
    - docs/metrics/lines-of-code.md

key-decisions:
  - "Phase 73 parity closeout is enforced locally by the deterministic checker, not by public-network UAT or release-blocking live sync."
  - "Referenced Rust evidence files are asserted through docs/parity/source-breadcrumbs.json and the existing breadcrumb checker instead of adding new breadcrumbs."
  - "Deferred-scope terms are required as non-claims so evidence roots remain auditable without implying production-node readiness."

patterns-established:
  - "Parity closeout checkers should validate both positive evidence roots and explicit non-claim boundaries."
  - "Checker fixture roots should model required evidence files without depending on live public-network reports."

requirements-completed: [VER-01, VER-04]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 73-2026-06-13T22-08-43
generated_at: 2026-06-14T05:45:28Z

# Metrics
duration: 1h 20m 21s
completed: 2026-06-14
---

# Phase 73 Plan 04: Parity Evidence Closeout Summary

**Phase 73 parity roots and checker enforcement now make UAT, breadcrumb, live-smoke, support-bundle, and deferred-scope evidence auditable without production-readiness claims.**

## Performance

- **Duration:** 1h 20m 21s
- **Started:** 2026-06-14T04:25:07Z
- **Completed:** 2026-06-14T05:45:28Z
- **Tasks:** 2
- **Files modified:** 10

## Accomplishments

- Added Phase 73 parity root entries across P2P, chainstate, operator-runtime, root index, checklist, and README documentation.
- Extended `scripts/check-phase73-uat-verification.ts` with `verifyParityAndEvidenceCloseout`, covering parity root strings, live-smoke/support/checker evidence files, source-breadcrumb registry coverage, and deferred-scope non-claims.
- Added TDD coverage proving the checker fails when parity roots are missing, source breadcrumbs omit referenced Rust files, or evidence roots claim default public-network UAT or broad production-node readiness.
- Kept public-network UAT out of default verification while enforcing local evidence roots through `bash scripts/verify.sh`.

## Task Commits

Each task was committed atomically:

1. **Task 1: Update parity roots for Phase 73 evidence boundaries** - `39ab978` (docs)
2. **Task 2 RED: Add failing closeout checker tests** - `1fa5ac5` (test)
3. **Task 2 GREEN: Enforce parity and breadcrumb closeout** - `43b03ca` (feat)

**Plan metadata:** recorded in the final docs commit after summary self-check.

_Note: Task 2 used the required TDD split, so it has separate RED and GREEN commits._

## Files Created/Modified

- `docs/parity/catalog/p2p.md` - Added the Phase 73 opt-in public-mainnet UAT and deterministic verification evidence boundary.
- `docs/parity/catalog/chainstate.md` - Added the VER-02 deterministic coverage map with stable exact coverage labels.
- `docs/parity/catalog/operator-runtime-release-hardening.md` - Added Phase 73 operator evidence closeout rooted in runtime docs, checker, verifier, breadcrumbs, and support bundles.
- `docs/parity/index.json` - Added the `phase73-opt-in-uat-deterministic-verification` root surface.
- `docs/parity/checklist.md` - Added the Phase 73 checklist row and deferred-scope non-claim.
- `docs/parity/README.md` - Added a Phase 73 pointer to the parity closeout evidence.
- `scripts/check-phase73-uat-verification.ts` - Added parity root, breadcrumb registry, evidence-file, and non-claim checks.
- `scripts/check-phase73-uat-verification.test.ts` - Added hermetic TDD fixtures for missing root text, breadcrumb drift, and bad production/default-UAT claims.
- `docs/metrics/lines-of-code.md` - Refreshed the tracked generated LOC report.
- `.planning/phases/73-opt-in-uat-and-deterministic-verification/73-04-SUMMARY.md` - Captures execution results, deviations, and verification evidence.

## Decisions Made

- Phase 73 closeout remains local and deterministic; public-network UAT, public-network CI, and release-blocking live sync stay outside default verification.
- The checker asserts existing breadcrumb registry coverage for the referenced Rust files rather than modifying `docs/parity/source-breadcrumbs.json`, because this plan did not create new Rust files.
- Deferred-scope strings are required in parity docs as non-claims, while exact bad-claim sentences fail the checker.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Stabilized exact chainstate coverage labels**
- **Found during:** Task 2 GREEN (checker implementation)
- **Issue:** The new exact-string checker could not see three chainstate coverage labels because Markdown line wrapping split the phrases.
- **Fix:** Rewrote the VER-02 coverage sentence as a short list with one required phrase per line.
- **Files modified:** `docs/parity/catalog/chainstate.md`
- **Verification:** `bun --check scripts/check-phase73-uat-verification.ts` and `bun run scripts/check-phase73-uat-verification.ts` passed.
- **Committed in:** `43b03ca`

**2. [Rule 3 - Blocking] Refreshed stale tracked LOC report**
- **Found during:** Task 2 verification
- **Issue:** `bash scripts/verify.sh` failed fast because `docs/metrics/lines-of-code.md` was stale after checker and test changes.
- **Fix:** Ran `bun run scripts/generate-loc-report.ts --source=worktree --output=docs/metrics/lines-of-code.md` and committed the refreshed tracked artifact.
- **Files modified:** `docs/metrics/lines-of-code.md`
- **Verification:** `bash scripts/verify.sh` passed, and commit hooks passed.
- **Committed in:** `1fa5ac5`, `43b03ca`

---

**Total deviations:** 2 auto-fixed (1 bug, 1 blocking)
**Impact on plan:** Both fixes were required for the planned checker and repo-native verification contract. No production-scope or public-network behavior was added.

## Issues Encountered

- `bun --check scripts/check-phase73-uat-verification.test.ts` is not valid for this Bun test file because `afterEach` is provided by the test runner; RED/GREEN verification used `bun test scripts/check-phase73-uat-verification.test.ts`.
- The first `bash scripts/verify.sh` run after GREEN failed on stale LOC only; regenerating the tracked report resolved it.

## User Setup Required

None - no external service configuration required. No public-network UAT command was run for this plan.

## Verification

- RED: `bun test scripts/check-phase73-uat-verification.test.ts` failed as expected before implementation, with the three new closeout tests failing.
- GREEN: `bun test scripts/check-phase73-uat-verification.test.ts` passed with 7 tests.
- `bun --check scripts/check-phase73-uat-verification.ts`
- `bun run scripts/check-phase73-uat-verification.ts`
- `bun run scripts/check-parity-breadcrumbs.ts --check`
- `rg -n "verifyParityAndEvidenceCloseout" scripts/check-phase73-uat-verification.ts`
- `rg -n "packages/open-bitcoin-node/src/sync/tests.rs|packages/open-bitcoin-chainstate/tests/parity.rs|packages/open-bitcoin-cli/src/operator/support/evidence.rs" scripts/check-phase73-uat-verification.ts`
- `cargo fmt --all`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo build --all-targets --all-features`
- `cargo test --all-features`
- `bash scripts/verify.sh` passed before the GREEN commit in 5m 58.854s.
- Task commit hooks reran full verification and passed for all three task commits.

## Next Phase Readiness

Phase 73 closeout is now auditable from parity roots and enforced by default local verification. Future work can depend on `scripts/check-phase73-uat-verification.ts` to fail closed if Phase 73 evidence roots drift or if documentation starts implying production-node/public-network readiness.

## Known Stubs

None - stub-pattern scan found no TODO/FIXME, placeholder, coming-soon text, or unwired empty data stubs introduced by this plan. The only match was an existing prose reference to placeholders in the operator-runtime catalog.

## Threat Flags

None - no new network endpoint, auth path, schema boundary, file trust boundary, or public-network side effect was introduced. The checker only reads local repository evidence and runs the existing local breadcrumb checker.

## Self-Check: PASSED

- Found summary file: `.planning/phases/73-opt-in-uat-and-deterministic-verification/73-04-SUMMARY.md`
- Found checker file: `scripts/check-phase73-uat-verification.ts`
- Found checker test file: `scripts/check-phase73-uat-verification.test.ts`
- Found Task 1 commit: `39ab978`
- Found Task 2 RED commit: `1fa5ac5`
- Found Task 2 GREEN commit: `43b03ca`
- Stub-pattern scan found no goal-blocking stubs introduced by this plan.

---
*Phase: 73-opt-in-uat-and-deterministic-verification*
*Completed: 2026-06-14*
