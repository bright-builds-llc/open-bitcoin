---
phase: 129-integration-guardrails-and-milestone-reconciliation
plan: "02"
subsystem: network-observability
tags: [obs-01, fallback-counters, compact-relay, truthfulness, d-06]
requires:
  - phase: 129-integration-guardrails-and-milestone-reconciliation/129-01
    provides: Aggregate Phase 129 guard composing the 127/128 seam checkers that must stay green
provides:
  - Truthful fallback facet - compact_timeout_count counts only real Timeout cleanups
  - Two focused regression tests pinning the fixed semantics on the live snapshot path
  - Recorded D-06 fix-path decision in docs/parity/catalog/p2p.md and the compact_timeout_count doc comment
affects: [129-04, 129-VERIFICATION, FLOW-04]
tech-stack:
  added: []
  patterns:
    - Live per-peer facts project through exactly one facet; durable counters are never re-derived at snapshot time
key-files:
  created: []
  modified:
    - packages/open-bitcoin-node/src/network/block_relay_evidence.rs
    - packages/open-bitcoin-node/src/status/block_relay_evidence.rs
    - packages/open-bitcoin-node/src/network/tests/compact_timeout_cases.rs
    - docs/parity/catalog/p2p.md
    - docs/metrics/lines-of-code.md
key-decisions:
  - "D-06 resolved on the fix path: the snapshot-time mixing of live getblocktxn_in_flight entries into fallback.compact_timeout_count is a truthfulness defect with no intentional-semantics evidence anywhere in packages or docs."
  - "Regression tests live in network/tests/compact_timeout_cases.rs (the existing module exercising snapshots with live in-flight state), not sync/tests.rs whose counters are hand-built fixtures."
patterns-established:
  - "Fallback facet counters are durable record_cleanup increments only; the in-flight facet is the single projection of live getblocktxn_in_flight facts."
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 129-2026-07-20T19-28-06
generated_at: 2026-07-20T22:21:00Z
duration: 28 min
completed: 2026-07-20
---

# Phase 129 Plan 02: D-06 Fallback Counter Truthfulness Summary

Fixed the OBS-01 fallback facet so `compact_timeout_count` reports only real Timeout cleanups: removed the snapshot-time addition of live `getblocktxn_in_flight` entries (already projected by the in-flight facet), pinned both semantics with regression tests, and recorded the decision in the parity catalog — no schema change, all guards green.

## Performance

- **Duration:** 28 min (excluding pre-commit hook verification)
- **Started:** 2026-07-20T21:52:00Z
- **Completed:** 2026-07-20T22:20:00Z
- **Tasks:** 2
- **Files modified:** 5

## D-06 Decision Record (evidence-driven procedure)

**Outcome: FIX PATH.** The inclusive snapshot semantics were a truthfulness defect, not an intentional contract.

Evidence gathered per the decision procedure:

1. `rg -n "compact_timeout_count" packages docs` — no test name, doc comment, or parity doc anywhere states that the fallback timeout counter includes live in-flight requests. Every match is either (a) the fixed metric-label vocabulary lists in `docs/architecture/status-snapshot.md`, `docs/architecture/operator-observability.md`, and `docs/operator/runtime-guide.md`, (b) hand-built fixture counter values in `sync/tests.rs`, `metrics/tests.rs`, `logging/tests.rs`, and the CLI status/dashboard/support test modules (none exercise `fallback_counters`), or (c) pass-through rendering/projection code with no semantics commentary.
2. The doc comment on `CompactRelayFallbackCounters` said only "Aggregate fallback counters safe for operator status surfaces" — silent on inclusivity.
3. The same live fact was already projected through `in_flight.getblocktxn_in_flight_count` (verified in `in_flight_counters`), so the mixing double-reported one fact across two facets and transiently inflated a counter named "timeout" with requests that had not timed out.

Per the plan's step 2 (expected outcome per 129-RESEARCH.md Pitfall 11), the minimal fix was applied:

- Deleted `fn fallback_counters` entirely; `observed_status` now passes `self.fallback` directly to `BlockRelayEvidenceStatus::with_components(...)`.
- Durable Timeout-cleanup increments in `record_cleanup` are untouched.
- Added the specified doc comment on `compact_timeout_count`: live in-flight getblocktxn requests are reported only by the in-flight facet.
- `CompactRelayFallbackCounters` keeps exactly its two fields — no schema change (D-05).

## Accomplishments

- **Task 1 (fix + tests):** Removed the snapshot-time mixing in `packages/open-bitcoin-node/src/network/block_relay_evidence.rs`; added regression Test A (`snapshot_with_live_getblocktxn_in_flight_reports_zero_fallback_timeouts`: live in-flight entry with zero Timeout cleanups reports `fallback.compact_timeout_count == 0` and `in_flight.getblocktxn_in_flight_count == 1`) and Test B (`timeout_cleanup_increments_fallback_timeout_and_fallback_counts`: one real Timeout cleanup reports `compact_timeout_count == 1` and `compact_fallback_count == 1`) in `network/tests/compact_timeout_cases.rs`.
- **Task 2 (parity + verification):** Recorded the fix-path sentence in the Phase 116 section of `docs/parity/catalog/p2p.md` in no-claim vocabulary; parity breadcrumbs verified for all touched Rust files (all pre-registered in `docs/parity/source-breadcrumbs.json`); full `bash scripts/verify.sh` passed (9m 1s) plus again at commit time via the pre-commit hook (5m 40s).

## Task Commits

The plan was committed as a single batch per the plan's explicit commit batching:

1. **Tasks 1-2: D-06 fallback counter fix, regression tests, parity record** - `3e91ff53`

## Files Created/Modified

- `packages/open-bitcoin-node/src/network/block_relay_evidence.rs` - Removed `fn fallback_counters` and the snapshot-time `compact_timeout_count` addition; fallback facet now projects `self.fallback` directly.
- `packages/open-bitcoin-node/src/status/block_relay_evidence.rs` - Doc comment pinning timeouts-only semantics on `compact_timeout_count`; struct fields unchanged.
- `packages/open-bitcoin-node/src/network/tests/compact_timeout_cases.rs` - Two Arrange/Act/Assert regression tests pinning the D-06 semantics on the live snapshot path.
- `docs/parity/catalog/p2p.md` - One D-06 semantics sentence added to the Phase 116 aggregate block-relay operator evidence section.
- `docs/metrics/lines-of-code.md` - Required freshness regeneration.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Regression tests placed in `network/tests/compact_timeout_cases.rs` instead of `sync/tests.rs`**
- **Found during:** Task 1
- **Issue:** The plan's `files_modified` listed `packages/open-bitcoin-node/src/sync/tests.rs`, but its `compact_timeout_count` usage is a hand-built fixture value for metrics/logging projection — it never exercises `fallback_counters` or live in-flight snapshots. The plan's action text itself directs the tests to "the existing test module that exercises snapshots with live in-flight state".
- **Fix:** Added both regression tests to `network/tests/compact_timeout_cases.rs`, which owns `start_in_flight_compact_download` and the live timeout-evidence proofs; `sync/tests.rs` needed no change (its fixture values remain valid under timeouts-only semantics).
- **Files modified:** packages/open-bitcoin-node/src/network/tests/compact_timeout_cases.rs
- **Commit:** 3e91ff53

No existing test asserted the inclusive snapshot value, so no assertions needed updating.

## Verification Evidence

- `cargo test -p open-bitcoin-node --all-features` (via `command-timings.ts run --key cargo-test-node`): 463 passed, 0 failed (unit) — both new regression tests pass.
- `rg -n "counters.compact_timeout_count \+= timed_out" packages/open-bitcoin-node/src/network/block_relay_evidence.rs` — no match (fix-path acceptance).
- `bun run scripts/check-phase128-production-compact-announcement-transport.ts`, `check-phase129-integration-guardrails-and-milestone-reconciliation.ts`, and `check-phase116-operator-block-relay-evidence.ts` all exit 0.
- `bun run scripts/check-parity-breadcrumbs.ts` — 389 Rust files verified, exit 0.
- `docs/parity/index.json` still contains the literal substrings "Phase 128 retains" and "Phase 129 retains".
- `bash scripts/verify.sh` (via `command-timings.ts run --key verify-full`) exited 0 in 9m 1s; the pre-commit hook verification passed again at commit time.
- `git status --porcelain` on `.planning/ROADMAP.md`, `.planning/REQUIREMENTS.md`, `.planning/PROJECT.md`, `.planning/MILESTONES.md`, and `.planning/v2.1-MILESTONE-AUDIT.md` was empty — no reconciliation-guarded artifact moved.

## README Review

Reviewed `README.md` wording: the compact-relay evidence sentences describe aggregate operator evidence without asserting fallback-counter internals, and the Phase 129 status sentence remains accurate (reconciliation still pending in Plans 03/04). No README change needed.

## Known Stubs

None - the fix is fully wired through the shared status contract and covered by regression tests.

## Next Steps

- Plan 03: Phase 124 stage-machine evolution for the archive-ready projection.
- Plan 04: verification-gated requirement promotion (OBS-01 promotion cites this fix) and milestone reconciliation.

## Self-Check: PASSED

- FOUND: .planning/phases/129-integration-guardrails-and-milestone-reconciliation/129-02-SUMMARY.md
- FOUND: commit 3e91ff53
- FOUND: regression tests in packages/open-bitcoin-node/src/network/tests/compact_timeout_cases.rs
