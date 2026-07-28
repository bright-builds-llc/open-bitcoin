---
phase: 134-authoritative-cross-cache-lifecycle-integration
plan: "04"
subsystem: node-network-lifecycle
tags: [rust, prepared-operations, compact-block, relay-serving, peer-lifecycle]

requires:
  - phase: 134-authoritative-cross-cache-lifecycle-integration
    plan: "02"
    provides: exhaustive lifecycle command vocabulary and closed authority-bound projection plan
  - phase: 134-authoritative-cross-cache-lifecycle-integration
    plan: "03"
    provides: bounded identity-complete peer transaction lifecycle capability
provides:
  - authoritative final-membership preparation for compact, serving, fanout, and peer targets
  - consuming unit-returning apply operations for every Plan 04 target
  - focused atomicity coverage and brace-balanced structural enforcement
affects: [phase-134-authority-projector, transaction-relay, compact-download, lifecycle-verification]

tech-stack:
  added: []
  patterns:
    - finish identity derivation, membership filtering, and bounded cloning before mutation
    - retain replacement compact bodies before retiring accepted-serving aliases
    - inspect an exact target-function allowlist with balanced-body extraction

key-files:
  created:
    - packages/open-bitcoin-node/src/network/lifecycle_projection/tests.rs
    - packages/open-bitcoin-node/src/network/relay_fanout/lifecycle.rs
    - packages/open-bitcoin-node/src/network/tests/lifecycle_projection_target_cases.rs
    - scripts/check-phase134-apply-boundaries.ts
  modified:
    - packages/open-bitcoin-node/src/network/lifecycle_projection.rs
    - packages/open-bitcoin-node/src/network/compact_receive_candidates.rs
    - packages/open-bitcoin-node/src/network/inventory.rs
    - packages/open-bitcoin-node/src/network/relay_serving.rs
    - packages/open-bitcoin-node/src/network/relay_fanout.rs
    - packages/open-bitcoin-node/src/network/tests.rs
    - docs/parity/source-breadcrumbs.json

key-decisions:
  - "Prepare every compact, serving, fanout, and peer replacement or teardown from authoritative final membership before any target mutates."
  - "Retain replacement transaction bodies in bounded compact extras before accepted-serving aliases are retired."
  - "Restrict structural enforcement to exact brace-balanced target apply bodies, excluding fallible validation and aggregate dispatch."

patterns-established:
  - "Prepared target apply: consume bounded owned operations, mutate synchronously, return unit."
  - "Target ordering: compact retention precedes serving retirement, followed by fanout and peer-local projection."

requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 134-2026-07-28T01-41-12
generated_at: 2026-07-28T08:35:34Z

duration: 58min
completed: 2026-07-28
---

# Phase 134 Plan 04: Infallible Lifecycle Target Application Summary

**Authoritative final-membership facts now prepare bounded compact, serving, fanout, and peer operations whose consuming unit-returning apply bodies cannot fail after core commit.**

## Performance

- **Duration:** 58min
- **Started:** 2026-07-28T07:37:25Z
- **Completed:** 2026-07-28T08:35:34Z
- **Tasks:** 2
- **Files modified:** 12

## Accomplishments

- Added real compact retention, dual-alias serving replacement/removal, ordered relay fanout, and Plan 03 peer lifecycle preparations to the closed projection plan.
- Kept all identity/body derivation, final-membership selection, conflict detection, and bound enforcement in preparation; the four target applies consume owned work and return unit.
- Proved final-absent dual-alias removal, replacement ordering, and complete-snapshot preservation across alias, membership, fingerprint-member, and work-limit preparation failures.
- Added a Bun structural checker that discovers exactly the target allowlist, extracts balanced bodies, and rejects fallible, derivational, codec, asynchronous, and I/O behavior only inside those bodies.
- Passed focused target tests and the complete normal-hook repository verification contract, including formatting, warnings-denied clippy, all-target build, all-feature tests, Bright Builds checks, parity breadcrumbs, Bazel smoke, and coverage.

## Task Commits

Each task was committed atomically:

1. **Task 1: Prepare and apply compact, serving, inventory, fanout, and peer targets** - `3c5ad761`
2. **Task 2: Prove apply bodies are structurally infallible without false positives** - `c596ccde`

## Files Created/Modified

- `packages/open-bitcoin-node/src/network/lifecycle_projection.rs` - Builds the mandatory prepared target set from authoritative final membership and exposes target applies to the network adapter.
- `packages/open-bitcoin-node/src/network/lifecycle_projection/tests.rs` - Focused prepared-target ordering and consumption coverage split from the production module.
- `packages/open-bitcoin-node/src/network/compact_receive_candidates.rs` - Applies exact compact candidate replacements while preserving bounded replacement bodies.
- `packages/open-bitcoin-node/src/network/inventory.rs` - Supports exact inventory projection used by prepared fanout work.
- `packages/open-bitcoin-node/src/network/relay_serving.rs` - Applies exact txid/wtxid accepted-serving aliases.
- `packages/open-bitcoin-node/src/network/relay_fanout.rs` and `relay_fanout/lifecycle.rs` - Apply exact parent-first admissions and descendant-first removals without derivation.
- `packages/open-bitcoin-node/src/network/tests.rs` and `tests/lifecycle_projection_target_cases.rs` - Register and prove final-membership, replacement-order, and failure-atomicity behavior.
- `scripts/check-phase134-apply-boundaries.ts` - Performs exact allowlist discovery and balanced target-body inspection.
- `docs/parity/source-breadcrumbs.json` - Registers exact Knots anchors for the new Rust files.
- `docs/metrics/lines-of-code.md` - Refreshed by the mandatory repository hook.

## Decisions Made

- Compact replacement bodies are cloned into bounded prepared extras during preparation so compact application cannot decode, derive, or fail.
- Serving preparation carries exact old and replacement alias pairs. Compact applies first, ensuring a replacement body remains retrievable before serving retirement.
- Fanout and peer operations preserve already validated parent-first admission and descendant-first teardown order rather than recomputing order at application time.
- Structural enforcement uses exact function names and brace matching rather than file-level negative searches, so future fallible validation remains legal while target apply bodies remain infallible.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Split lifecycle tests and fanout helpers to satisfy enforced code shape**

- **Found during:** Task 1 commit verification
- **Issue:** Adding the planned target behavior made the primary lifecycle and fanout modules exceed the repository's strict production-file length boundary.
- **Fix:** Moved focused lifecycle tests into `lifecycle_projection/tests.rs` and exact fanout lifecycle helpers into `relay_fanout/lifecycle.rs`, leaving the production modules below the enforced limit.
- **Files modified:** `packages/open-bitcoin-node/src/network/lifecycle_projection.rs`, `packages/open-bitcoin-node/src/network/lifecycle_projection/tests.rs`, `packages/open-bitcoin-node/src/network/relay_fanout.rs`, `packages/open-bitcoin-node/src/network/relay_fanout/lifecycle.rs`
- **Verification:** Bright Builds code-shape checks and the full repository verifier passed.
- **Committed in:** `3c5ad761`

**2. [Rule 3 - Blocking] Registered exact parity breadcrumbs for split Rust sources**

- **Found during:** Task 1 commit verification
- **Issue:** The newly split first-party Rust files required exact breadcrumb header blocks and matching registry entries.
- **Fix:** Added the required Knots anchors to both new files and registered their exact source headers.
- **Files modified:** `packages/open-bitcoin-node/src/network/lifecycle_projection/tests.rs`, `packages/open-bitcoin-node/src/network/relay_fanout/lifecycle.rs`, `docs/parity/source-breadcrumbs.json`
- **Verification:** `scripts/check-parity-breadcrumbs.ts` and the full repository verifier passed.
- **Committed in:** `3c5ad761`

**3. [Rule 3 - Blocking] Deferred requirement activation to lifecycle verification**

- **Found during:** Plan metadata preparation
- **Issue:** Intermediate Phase 134 summaries cannot activate `MPLIFE-02` or `MPLIFE-03` before the phase has a lifecycle-valid verification artifact.
- **Fix:** Preserved both requirements as pending and left `requirements-completed` empty, matching the active Phase 134 traceability contract.
- **Files modified:** `.planning/phases/134-authoritative-cross-cache-lifecycle-integration/134-04-SUMMARY.md`
- **Verification:** The repository's active milestone traceability checker passed with requirements pending.
- **Committed in:** Plan metadata commit

**Total deviations:** 3 auto-fixed (3 blocking)
**Impact on plan:** The adjustments preserve the planned behavior, keep the new sources auditable and maintainable, and avoid prematurely claiming milestone requirements.

## Issues Encountered

- Task 1's first normal commit hook identified missing exact source breadcrumbs; after registration, the next hook exposed the stricter production-file boundary, leading to the focused module split above.
- After Task 2's long-running normal hook lost its client cell handle, a duplicate commit hook was mistakenly started. The duplicate was terminated while waiting on the cooperative build lock, before it launched Cargo; the original hook was preserved and completed successfully as `c596ccde`.
- The first final metadata hook attempt hit the pre-existing Phase 94 claim-guardrail test's 5-second timeout. An isolated rerun passed all four Phase 94 cases, including the same negative case in 4.66 seconds, before the normal metadata commit was retried.

## Known Stubs

None. Empty accumulators in the structural checker are parser state initialized before balanced-body discovery, not user-visible or runtime placeholders.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Plan 134-05 can validate authority epoch plus core revision, then consume the four prepared Plan 04 targets in compact, serving, fanout, and peer order.
- The checker is ready for Plan 05 to add the three explicit unbroadcast, persistence, and evidence target applies without classifying validation or aggregate dispatch.
- No new endpoint, authentication path, schema boundary, or runtime file-access surface was introduced.
- `MPLIFE-02` and `MPLIFE-03` remain pending until Phase 134 verification establishes the lifecycle-valid completion artifact.
- No blockers remain.

## Self-Check: PASSED

- Summary and all four key created files exist.
- Task commits `3c5ad761` and `c596ccde` are present in repository history.
- The exact apply-boundary checker passes with only the four Plan 04 targets discovered.
- Changed sources contain no goal-blocking stubs, `git diff --check` passes, and Markdown frontmatter uses exactly two delimiters.

***

*Phase: 134-authoritative-cross-cache-lifecycle-integration*
*Completed: 2026-07-28*
