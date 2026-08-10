---
phase: 135-snapshot-schema-checkpointing-and-recovery
plan: "11"
subsystem: verification
tags: [typescript, structural-checker, mutation-testing, parity, loc]
requires:
  - phase: 135-10
    provides: policy-independent persisted-input limits and current-policy replay
provides:
  - responsibility-focused persisted-input production checker
  - exact persisted-input mutation matrix with two-boundary parity evidence
  - fresh tracked LOC evidence ready for independent phase verification
affects: [phase-135-verification, MPDUR-02, snapshot-recovery]
tech-stack:
  added: []
  patterns:
    - stable checker roots delegate format-boundary responsibilities to focused pure modules
    - mutation fixtures assert one exact low-cardinality diagnostic per persisted-input regression
key-files:
  created:
    - scripts/check-phase135-snapshot-recovery/persisted-input.ts
    - scripts/check-phase135-snapshot-recovery/persisted-input-mutations.ts
    - .planning/phases/135-snapshot-schema-checkpointing-and-recovery/135-11-SUMMARY.md
  modified:
    - scripts/check-phase135-snapshot-recovery.ts
    - scripts/check-phase135-snapshot-recovery.test.ts
    - docs/parity/catalog/mempool-policy.md
    - docs/parity/checklist.md
    - docs/parity/index.json
    - docs/metrics/lines-of-code.md
key-decisions:
  - "Treat the four globally coexisting encoded dimensions separately from the individual per-transaction and input-edge ceilings embedded within transaction-byte budgets."
  - "Keep format-owned bounds authoritative only for persisted load/topology while current policy remains authoritative for live capture, replay, trimming, and final membership."
  - "Leave Phase 135 in progress, MPDUR rows Pending, and WR-01 open/non-blocking until the independent post-summary verifier transaction."
patterns-established:
  - "Production checker split: stable root orchestration plus one persisted-input contract module."
  - "Mutation split: stable test root plus one typed persisted-input fixture collection."
requirements-completed: [MPDUR-02]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 135-2026-08-02T17-41-48
generated_at: 2026-08-10T03:27:36Z
duration: 9m
completed: 2026-08-09
---

# Phase 135 Plan 11: Persisted-Input Checker and Evidence Closure Summary

**Compact production and mutation modules now enforce the format-bounded load/topology boundary, current-policy replay/final-membership boundary, and truthful pending parity evidence.**

## Performance

- **Duration:** 9m
- **Started:** 2026-08-10T03:18:33Z
- **Completed:** 2026-08-10T03:27:36Z
- **Tasks:** 3
- **Files modified:** 9, comprising eight declared checker/parity/LOC paths and this summary

## Accomplishments

- Extracted the persisted-input production responsibility from the 625-line root checker into a 229-line pure module while preserving the stable CLI, exports, diagnostics, filesystem-only behavior, and unrelated Phase 135 checks.
- Extracted 23 persisted-input mutations from the 628-line root test into a 259-line typed fixture module; the full suite now passes 75 cases and 132 assertions.
- Guarded the exact global encoded tuple, checked 41-byte input-edge derivations, streaming transaction-byte counters, policy-independent load/topology construction, current-policy capture/replay, and final `DroppedEvicted` rewrite.
- Reconciled the catalog, checklist, and machine index around the two-boundary model without promoting Phase 135 or MPDUR status, closing WR-01, or broadening Phase 136-138/public/readiness scope.
- Regenerated tracked LOC evidence from the worktree and proved it fresh at 311,739 counted lines.

## Task Commits

Each task was committed atomically:

1. **Task 1: Extract the production persisted-input checker responsibility** - `0e2da912` (`refactor`)
2. **Task 2: Extract mutation fixtures and reconcile pending parity evidence** - `e7ace67f` (`test`)
3. **Task 3: Refresh LOC, run aggregate focused gates, and publish immutable evidence** - published with this summary in the final documentation commit

## Checker Responsibilities and File Shape

| TypeScript path | Lines | Responsibility |
| --- | ---: | --- |
| `scripts/check-phase135-snapshot-recovery.ts` | 585 | Stable executable/export owner and cross-surface orchestration |
| `scripts/check-phase135-snapshot-recovery.test.ts` | 521 | Stable fixture lifecycle, shared assertions, and non-persisted mutation/claim cases |
| `scripts/check-phase135-snapshot-recovery/source.ts` | 186 | Dependency-free masked-source and structural helpers |
| `scripts/check-phase135-snapshot-recovery/persisted-input.ts` | 229 | Persisted decode, topology, current-policy replay, and final-membership checks |
| `scripts/check-phase135-snapshot-recovery/persisted-input-mutations.ts` | 259 | Twenty-three one-concern persisted-input mutations |

Every listed TypeScript file is at or below the 628-physical-line gate. The stable roots each delegate exactly one persisted-input responsibility. No parser, dependency, Python helper, duplicated limit arithmetic, or alternate root entrypoint was added.

## Exact Mutation Evidence

`bun test scripts/check-phase135-snapshot-recovery.test.ts` passed **75 tests, 0 failures, and 132 assertions**. Every delegated persisted-input case expects exactly one diagnostic. The matrix covers:

- encoded-size and raw-key preflight deletion, reordering, and lexical decoys;
- store decode recoupling to current `max_live_entries` and restoration of the 50,000-record cap;
- capture decoupling from current live accounting;
- topology vertex/edge recoupling to current policy and restoration of the 1,600,000-edge cap;
- removal of aggregate and per-record checked edge division;
- bypass of aggregate and per-record streaming transaction-byte counters;
- loader read-before-size-check and canonical identity bypass;
- unchecked topology edge accumulation;
- live-state mutation, lost unbroadcast intersection, reused rolling state, and bypassed final `DroppedEvicted` classification.

All pre-existing schema, compatibility, topology outcome, install, affine effect, persistence, coordinator, evidence, startup, shutdown, parity-status, verifier-order, and negative-claim mutations remain green. The unmodified live corpus also passes.

## Parity and Pending Status

The three parity artifacts now agree that format-derived candidate bounds govern persisted loading and topology, while fresh current policy governs live capture, replay, capacity trimming, and final membership. They explicitly distinguish the four globally coexisting encoded dimensions from individual allocation ceilings rather than claiming seven simultaneous maxima.

Top-level and checklist status remain `in_progress`; `MPDUR-01` through `MPDUR-04` remain `Pending`. Phase 135 review warning WR-01 remains open and non-blocking because the production preflight is unconditional while outer-`cfg` lexical hardening is deferred. Phase 136 scheduling/fanout/receipts, Phase 137 operator surfaces, Phase 138 release proof, runtime import, Knots `mempool.dat` compatibility, general package wire, whole-mempool rebroadcast, public/default relay, guaranteed propagation, public-network CI, destructive repair, and production readiness remain excluded.

## Verification Evidence

The executor-owned focused gates passed after LOC generation:

- `bun test scripts/check-phase135-snapshot-recovery.test.ts` - 75 passed, 0 failed, 132 assertions.
- `bun run scripts/check-phase135-snapshot-recovery.ts` - live Phase 135 checker passed.
- `bun run scripts/check-parity-breadcrumbs.ts --check` - 762 Rust files verified.
- `jq empty docs/parity/index.json` - machine index parsed successfully.
- `bun run scripts/bright-builds-check.ts all` - 1,044 files scanned for length, no findings; lesson ledger passed with no findings.
- `bun run scripts/generate-loc-report.ts --source=worktree --output=docs/metrics/lines-of-code.md --check` - current at 311,739 lines.
- TypeScript physical-line audit - exact counts recorded above, all at most 628.
- `git diff --check` - passed after generation.

The following Plan 10 Rust regressions were rerun sequentially through `scripts/command-timings.ts`, with no overlapping Cargo work:

- zero-current-capacity replay - 1 passed;
- later capacity-trim final-membership classification - 1 passed;
- accounted-capacity Fjall round trip - 1 passed;
- persisted-input formula/cross-field group - 8 passed;
- former 50,000-record Sync/reopen regression - 1 passed with 50,001 records;
- RPC persisted-input contract across current capacities - 1 passed;
- hostile narrow-limit original-snapshot retention - 1 passed;
- open-bitcoind checkpoint group - 10 passed.

Plan 10 also provides the inherited focused compile, formatting, warning-denied clippy, and all-target/all-feature build evidence recorded in `135-10-SUMMARY.md`.

Per the accepted transaction boundary, this executor **did not** run `bash scripts/verify.sh`, did not run `phase135-11-full-verify`, and did not create, restore, edit, stage, or forge canonical `135-VERIFICATION.md`. No default/canonical Phase 135 pass is claimed. Only the independent verifier may create the lifecycle-valid candidate report and run the exact final default command on the unchanged post-summary tree.

## Scope and Protected-Artifact Audit

The executor commit range plus the pending generated LOC artifact contains exactly the eight Plan 11 checker/parity/LOC paths. Adding this summary yields the exact nine-path final executor inventory. No ROADMAP, STATE, config, requirement, plan, prior summary, review, or canonical verification artifact appears in either task commit.

The protected aggregate SHA-256 over ROADMAP, STATE, config, REQUIREMENTS, Phase 135 CONTEXT/RESEARCH/REVIEW, Plans 01-11, and Summaries 01-10 remained `a8ab20e9888487098c7ed8d367416609cd6cdec1107103c2f01247696949b17e`. Canonical `135-VERIFICATION.md` was absent at the baseline and remains absent. Existing shared orchestration changes on protected planning paths were neither staged nor committed by this executor.

## Decisions Made

- Kept one format-owned factory as the source of persisted decode and topology candidate ceilings; individual edge ceilings remain checked derivations within transaction-byte budgets.
- Kept `MempoolCapacityBounds::from_capacity` only on live capture and preserved fresh `PolicyConfig` authority for replay, trimming, and final membership.
- Kept stable root paths as orchestration shells and moved cohesive production/mutation responsibilities into same-named child modules.
- Kept all parity and requirement statuses pending for the independent verifier transaction.

## Simplification Review

- The root checker is 40 lines smaller than the 625-line baseline even after adding the child inventory and strengthened parity guards.
- The root test is 107 lines smaller than the 628-line baseline while retaining shared fixture creation, cleanup, and assertions.
- Production arithmetic is checked once in Rust and structurally required once in the focused checker; the TypeScript module does not reproduce the arithmetic as executable policy.
- The mutation module contains data-only fixture definitions and reuses root-owned temp-tree and assertion infrastructure.

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

- The Task 2 RED run reproduced three stale pre-Plan-10 mutations that still searched for removed policy-derived strings. Moving that responsibility into the planned child module replaced them with exact repaired-boundary mutations; the final 75-case suite is green.
- Empty arrays in checker/test roots are intentional failure/temp-root accumulators, not runtime or UI stubs.

## Known Stubs

None. The changed checker and evidence paths contain no placeholder runtime or UI data flow.

## Threat and Security Review

- High threats T-135-11-01 and T-135-11-02 are mitigated by exact production anchors and exact-diagnostic mutation cases for each persisted/live boundary.
- Checker sources remain deterministic, filesystem-only, low-cardinality, and free of raw snapshot, transaction, identity, or secret output.
- No network endpoint, authentication path, schema, dependency, runtime file-access path, or other unplanned trust boundary was introduced.
- T-135-11-04 remains reserved for the independent verifier; this summary does not spoof its attestation.

## User Setup Required

None - no external service configuration or credentials are required.

## Deferred Issues

- WR-01 remains non-blocking and deferred: live preflight is unconditional; outer-`cfg` lexical mutation hardening is not claimed here.
- Default repository verification and canonical Phase 135 adjudication remain exclusively owned by the independent post-summary verifier transaction.

## Next Phase Readiness

- The immutable executor tree is ready for independent Phase 135 source-truth review and the exact `phase135-11-full-verify` transaction.
- Roadmap/state/requirement promotion must wait for a lifecycle-valid canonical verifier result.

## Self-Check: PASSED

Both focused modules, all eight declared checker/parity/LOC paths, and this summary exist. Commits `0e2da912` and `e7ace67f` resolve in repository history. The executor source/evidence inventory is exact, protected planning hashes are unchanged, canonical `135-VERIFICATION.md` remains absent, and no goal-blocking stub or unplanned threat surface remains.

***

*Phase: 135-snapshot-schema-checkpointing-and-recovery*
*Completed: 2026-08-09*
