---
phase: 134-authoritative-cross-cache-lifecycle-integration
plan: "15"
subsystem: lifecycle-effect-authority
tags: [rust, lifecycle-authority, peer-sessions, affine-receipts, atomic-evidence, security]

requires:
  - phase: 134-14
    provides: exact family-bound pending and completed receipt accounting
provides:
  - bounded peer-local session generations for receipt freshness
  - unrelated-peer churn isolation and same-peer replacement staleness
  - one-command peer-emission completion, classification, accounting, and evidence
  - compatibility guardrails for the atomic completion facade
affects: [phase-134-verification, phase-135, lifecycle-effects, compact-announcement-evidence]

tech-stack:
  added: []
  patterns:
    - peer receipt freshness compares only the generation of the receipt target
    - session-generation history is retained only for connected or pending-effect peers
    - peer emission receipts and evidence are consumed in one lifecycle command under one authority guard

key-files:
  created:
    - packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/effects/contracts/peer_sessions.rs
    - .planning/phases/134-authoritative-cross-cache-lifecycle-integration/134-15-SUMMARY.md
  modified:
    - packages/open-bitcoin-node/src/network.rs
    - packages/open-bitcoin-node/src/network/lifecycle_effects.rs
    - packages/open-bitcoin-node/src/network/lifecycle_projection.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/effects.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/lifecycle.rs
    - scripts/check-phase122-compact-relay-peer-completion.ts
    - scripts/check-phase126-compact-relay-residual-hardening.ts
    - scripts/check-phase128-production-compact-announcement-transport.ts
    - scripts/check-phase134-authoritative-lifecycle.ts

key-decisions:
  - "Retain a peer session generation only while that peer is connected or has a pending effect, bounding history by live plus pending peers."
  - "Use a family-specific CompletePeerEmission command so immutable validation, freshness, exact accounting, and evidence share one authority guard."
  - "Keep generic peer-effect completion for non-emission callers while routing emission receipts through the atomic evidence-bearing command."
  - "Keep MPLIFE-01 and MPLIFE-04 pending until independent phase re-verification."

patterns-established:
  - "Peer-local freshness: unrelated peer churn cannot change another peer receipt from Applied to stale."
  - "Atomic emission completion: exact current receipts record evidence once; stale, foreign, mismatched, and replayed receipts cannot append evidence."

requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 134-2026-07-28T01-41-12
generated_at: 2026-07-29T03:45:58Z

duration: 2h 7m
completed: 2026-07-29
---

# Phase 134 Plan 15: Peer-Local Atomic Completion Summary

**Bounded peer-local session generations and a single evidence-bearing lifecycle command now prevent unrelated churn and second-lock races from changing achieved peer-emission outcomes.**

## Performance

- **Duration:** 2h 7m
- **Started:** 2026-07-29T01:39:04Z
- **Completed:** 2026-07-29T03:45:58Z
- **Tasks:** 2
- **Files modified:** 22

## Accomplishments

- Replaced the aggregate-wide peer-session freshness counter with per-peer generations that advance only for the target peer and remain bounded by connected plus pending-effect peers.
- Proved peer B churn leaves peer A current, while peer A replacement makes an exact achieved receipt stale, records it completed exactly once, and preserves replacement state and evidence.
- Added `CompletePeerEmission`, which owns the affine emission receipt and performs immutable validation, freshness classification, exact accounting, and permitted evidence recording under one authority guard.
- Reduced `complete_peer_emission` to one dispatcher call with no follow-up `try_mutate` interval.
- Strengthened Phase 122, 126, 128, and 134 mutation checkers to reject the obsolete two-lock facade.

## Task Commits

Each task was committed atomically:

1. **Task 1: Scope peer-session freshness to the receipt target** - `6aedd999` (fix)
2. **Task 2: Complete peer effect and evidence in one authority command** - `87e0b400` (fix)

## Files Created/Modified

- `packages/open-bitcoin-node/src/network.rs` - Owns bounded per-peer session generations and target-specific advancement/cleanup.
- `packages/open-bitcoin-node/src/network/action_translation.rs` - Advances only the disconnected or replaced action target.
- `packages/open-bitcoin-node/src/network/inbound.rs` - Advances the exact inbound peer session.
- `packages/open-bitcoin-node/src/network/relay_serving.rs` - Preserves target-specific session handling at serving boundaries.
- `packages/open-bitcoin-node/src/network/lifecycle_effects.rs` - Exposes exact pending-binding validation for atomic completion.
- `packages/open-bitcoin-node/src/network/lifecycle_projection.rs` - Adds the family-specific emission completion command and evidence failure propagation.
- `packages/open-bitcoin-node/src/network/runtime_authority/effects.rs` - Dispatches emission completion exactly once with no second mutation lock.
- `packages/open-bitcoin-node/src/network/runtime_authority/lifecycle.rs` - Performs exact validation, peer-local freshness, accounting, and evidence under the dispatcher guard.
- `packages/open-bitcoin-node/src/network/lifecycle_projection/tests.rs` - Registers the emission command in the sealed command vocabulary.
- `packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/effects/contracts/peer_sessions.rs` - Covers unrelated churn, replacement, foreign bindings, current evidence, stale evidence, and replay.
- `scripts/check-phase122-compact-relay-peer-completion.ts` - Guards the one-command facade while retaining post-write provenance requirements.
- `scripts/check-phase126-compact-relay-residual-hardening.ts` - Guards receipt-derived evidence without requiring the retired second lock.
- `scripts/check-phase128-production-compact-announcement-transport.ts` - Guards atomic production transport completion.
- `scripts/check-phase134-authoritative-lifecycle.ts` - Requires exactly one emission completion dispatch and rejects `try_mutate`.
- `docs/parity/source-breadcrumbs.json` - Registers the new peer-session regression module.
- `docs/metrics/lines-of-code.md` - Regenerated by normal verification hooks.

## Decisions Made

- Retained session history only while a peer is connected or has pending work, preserving delayed exact-receipt classification without unbounded disconnected-peer state.
- Kept the generic `CompletePeerEffect` command for callers without evidence, and added `CompletePeerEmission` as the affine family-specific path rather than introducing a generic effect bus.
- Validated an exact pending binding before recording current evidence, then consumed that exact binding while the same guard remained held; stale and invalid inputs never reach evidence mutation.
- Left `MPLIFE-01` and `MPLIFE-04` unchecked for the independent phase verifier.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 2 - Missing Critical] Routed every peer lifecycle edge through target-specific generations**

- **Found during:** Task 1
- **Issue:** Replacing the stored counter also required all outbound, inbound, disconnect, replacement, and serving paths to identify the exact target peer.
- **Fix:** Updated the affected adapters to advance only their target peer and added cleanup tied to connected/pending ownership.
- **Files modified:** `network.rs`, `action_translation.rs`, `inbound.rs`, `relay_serving.rs`, `runtime_authority/lifecycle.rs`
- **Verification:** Unrelated-peer churn remained `Applied`; same-peer replacement returned `AchievedButStale`.
- **Committed in:** `6aedd999`

**2. [Rule 3 - Blocking] Split peer-session regressions to satisfy the managed file-length contract**

- **Found during:** Task 1
- **Issue:** Adding the required regressions directly to `contracts.rs` exceeded the Bright Builds file-length limit.
- **Fix:** Added the focused `contracts/peer_sessions.rs` module and registered its parity breadcrumb.
- **Files modified:** `effects/contracts.rs`, `effects/contracts/peer_sessions.rs`, `docs/parity/source-breadcrumbs.json`
- **Verification:** Bright Builds reported zero findings and parity breadcrumbs verified 731 Rust files.
- **Committed in:** `6aedd999`

**3. [Rule 3 - Blocking] Reconciled historical post-write guardrails with atomic completion**

- **Found during:** Task 2 normal commit hooks
- **Issue:** Phase 122, 126, and 128 checkers encoded the retired `complete_peer_effect` followed by `try_mutate(record_peer_emission)` facade.
- **Fix:** Updated each checker and mutation corpus to require `CompletePeerEmission`, the typed completion result, and no second mutation interval.
- **Files modified:** `scripts/check-phase122-compact-relay-peer-completion.ts`, `scripts/check-phase126-compact-relay-residual-hardening.ts`, `scripts/check-phase128-production-compact-announcement-transport.ts`, and companion tests
- **Verification:** Phase 122 passed 16 tests, Phase 126 passed 17, Phase 128 passed 20, and the complete normal hook passed.
- **Committed in:** `87e0b400`

**4. [Rule 3 - Blocking] Corrected the focused test selector**

- **Found during:** Task 1 verification
- **Issue:** The plan's `network::tests::effects` filter selected zero tests after earlier test decomposition.
- **Fix:** Used `lifecycle_projection_cases::effects`, which selected the intended lifecycle-effect corpus.
- **Files modified:** None.
- **Verification:** The corrected focused suite passed 29 tests after Task 2.
- **Committed in:** Not applicable.

**Total deviations:** 4 auto-fixed (1 missing critical, 3 blocking)
**Impact on plan:** All changes were necessary to apply the planned authority guarantee at real peer lifecycle edges and keep existing deterministic guardrails current; no new architecture, I/O, dependency, or public surface was added.

## Issues Encountered

- Task 1 TDD RED proved the defect when unrelated peer churn returned `AchievedButStale` instead of expected `Applied`.
- Task 2 TDD RED made the Phase 134 structural checker fail until the one-command emission variant and facade dispatch existed.
- Failing Rust RED states were not committed separately because the repository requires format, Clippy, build, and tests to pass before every Rust commit; each task retained an atomic green commit.
- One Task 2 regression initially inspected general relay evidence instead of compact-announcement evidence. Pointing it at `block_relay_evidence_status` made the assertion cover the intended store.
- The standalone full workspace test gate took 57 minutes because macOS delayed several executable and doctest launches. The timing wrapper captured liveness evidence and the run completed without interruption.
- The first two Task 2 commit attempts were correctly rejected by stale Phase 122 and Phase 126 guardrails. After repairing those and proactively updating Phase 128, the third normal hook passed in 17m 32s.

## Verification

- TDD RED: unrelated peer churn incorrectly classified the target receipt stale.
- TDD RED: the Phase 134 checker rejected the missing `CompletePeerEmission` command/facade.
- Focused GREEN: 29 lifecycle-effect tests and 12 announcement-transport/production tests passed.
- `bun test scripts/check-phase134-authoritative-lifecycle.test.ts`: 89 passed, 0 failed.
- `bun run scripts/check-phase134-authoritative-lifecycle.ts`: passed.
- Phase 122/126/128 compatibility mutation suites: 16/17/20 passed.
- Ordered `cargo fmt`, Clippy with `-D warnings`, all-target/all-feature build, and all-feature tests passed.
- `bun scripts/bright-builds-check.ts all`: 0 findings.
- Both task commits used normal hooks; the final Task 2 hook passed `bash scripts/verify.sh`, including Cargo tests/doc-tests/coverage, Bazel builds, benchmark smoke, parity breadcrumbs, and LOC freshness.
- `git diff --check` passed.

## Known Stubs

None.

## Threat Flags

None. T-134-15-01 through T-134-15-04 cover the peer transport receipt and serialized aggregate boundaries; no unplanned endpoint, authentication path, file-access boundary, schema boundary, dependency, or network I/O under the authority guard was introduced.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Plan 15 closes peer-local freshness and second-lock evidence races for Phase 134.
- Later gap plans can rely on current, stale, foreign/mismatched, and replayed emission outcomes being deterministic under one authority guard.
- `MPLIFE-01` and `MPLIFE-04` intentionally remain pending until independent phase-level re-verification.

## Self-Check: PASSED

The summary, both task commits, and all key implementation/guardrail files exist; frontmatter uses exactly two delimiters; no goal-blocking stubs or unplanned threat surface were found; and MPLIFE-01/MPLIFE-04 remain unchecked and Pending.

*Phase: 134-authoritative-cross-cache-lifecycle-integration*
*Completed: 2026-07-29*
