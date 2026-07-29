---
phase: 134-authoritative-cross-cache-lifecycle-integration
plan: "16"
subsystem: lifecycle-effect-authority
tags: [rust, lifecycle-authority, affine-capabilities, peer-fanout, successful-prefix, capacity-recovery]

requires:
  - phase: 134-15
    provides: peer-local effect freshness and atomic evidence-bearing completion
provides:
  - exact pre-achievement peer-effect aborts with typed classifications
  - explicit complete-or-abort termination for node and RPC peer fanout
  - successful-prefix preservation with failed and unsent suffix cleanup
  - bounded pending-capacity recovery across write, enqueue, and unregister failures
affects: [phase-134-verification, phase-135, phase-136, peer-effect-ledger, announcement-transport]

tech-stack:
  added: []
  patterns:
    - affine peer capabilities terminate exactly once through completion or abort
    - exact abort validates immutable ownership but tolerates newer lifecycle and peer-session freshness
    - fanout executors retain completed prefixes and abort the failed item plus every unsent suffix item

key-files:
  created:
    - packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/effects/contracts/peer_abort.rs
    - packages/open-bitcoin-node/src/sync/session/emission_terminal.rs
    - .planning/phases/134-authoritative-cross-cache-lifecycle-integration/134-16-SUMMARY.md
  modified:
    - packages/open-bitcoin-node/src/network/lifecycle_effects.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/lifecycle.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/effects.rs
    - packages/open-bitcoin-node/src/network/announcement_transport.rs
    - packages/open-bitcoin-node/src/sync/session.rs
    - packages/open-bitcoin-rpc/src/inbound_listener/connection_runtime.rs
    - scripts/check-phase123-runtime-timing-evidence-integrity/checks.ts
    - scripts/check-phase128-production-compact-announcement-transport.ts

key-decisions:
  - "Treat an exact pre-achievement abort as ownership of its reservation, so lifecycle or target-peer freshness advancement cannot prevent capacity release."
  - "Require every node and RPC peer-emission exit to complete an achieved write or explicitly abort all unachieved capabilities."
  - "Attempt every suffix abort even when an earlier abort fails, while surfacing cleanup failure instead of silently dropping capabilities."
  - "Keep MPLIFE-01 and MPLIFE-04 pending until independent phase re-verification."

patterns-established:
  - "Exact abort: immutable binding and authority incarnation select the reservation; current freshness is irrelevant because no external achievement occurred."
  - "Successful-prefix fanout: completed prefix evidence remains final while the failed current item and unsent suffix are explicitly aborted."

requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 134-2026-07-28T01-41-12
generated_at: 2026-07-29T06:51:56Z

duration: 2h 51m
completed: 2026-07-29
---

# Phase 134 Plan 16: Exact Peer Abort and Fanout Terminal Paths Summary

**Exact peer-effect aborts and complete-or-abort fanout shells now preserve successful-prefix evidence while recovering every failed or unsent pending reservation.**

## Performance

- **Duration:** 2h 51m
- **Started:** 2026-07-29T04:01:18Z
- **Completed:** 2026-07-29T06:51:56Z
- **Tasks:** 2
- **Files modified:** 23

## Accomplishments

- Added a typed, authority-routed `EffectAbort` path that consumes only the exact pending peer-effect binding, never records completion/evidence, tolerates later lifecycle or peer-session freshness, and classifies replay or mismatch without mutating another reservation.
- Routed node sync and RPC inbound fanout through explicit terminal handling: successful writes complete in order, while target mismatch, encoding, rejection, disconnect, and write failure abort the current capability plus every reserved unsent suffix.
- Preserved successful-prefix evidence exactly and proved first, middle, and last failures restore bounded peer pending capacity.
- Closed preparation-to-enqueue and unregister cleanup races so emissions that never reach a transport are aborted instead of dropped.
- Updated Phase 123 and Phase 128 mutation guardrails to follow the extracted send-before-ack implementation without weakening their historical guarantees.

## Task Commits

Each task was committed atomically:

1. **Task 1: Add an exact pre-achievement peer abort command** - `0ab43d71` (fix)
2. **Task 2: Route peer fanout failures and unsent suffixes through abort** - `ecc2f624` (fix)

## Files Created/Modified

- `packages/open-bitcoin-node/src/network/lifecycle_effects.rs` - Defines exact peer abort classification and pending-ledger consumption semantics.
- `packages/open-bitcoin-node/src/network/lifecycle_projection.rs` - Adds the peer-effect abort lifecycle command and typed result.
- `packages/open-bitcoin-node/src/network/runtime_authority/lifecycle.rs` - Applies exact abort under the sole authority guard without freshness rejection or false completion.
- `packages/open-bitcoin-node/src/network/runtime_authority/effects.rs` - Exposes authoritative peer-effect and peer-emission abort facades.
- `packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/effects/contracts/peer_abort.rs` - Covers exact abort, freshness advance, foreign and immutable mismatch, replay, and capacity recovery.
- `packages/open-bitcoin-node/src/network/announcement_transport.rs` - Allows an unachieved emission capability to enter the abort facade.
- `packages/open-bitcoin-node/src/sync/session/emission_terminal.rs` - Centralizes node-side ordered completion, current/suffix abort, and visible cleanup failures.
- `packages/open-bitcoin-node/src/sync/session.rs` - Aborts discarded enqueue outcomes, queued unregister emissions, and failed session fanout suffixes.
- `packages/open-bitcoin-node/src/sync/tests/production_announcement_transport_cases.rs` - Proves first/middle/last failure, encode failure, enqueue-race, unregister, prefix evidence, and capacity recovery behavior.
- `packages/open-bitcoin-rpc/src/inbound_listener/connection_runtime.rs` - Gives RPC fanout a complete-or-abort executor contract and a visible `AbortFailed` outcome.
- `packages/open-bitcoin-rpc/src/inbound_listener/tests/announcement_successful_prefix.rs` - Injects first/middle/last, target-mismatch, and abort-failure paths while asserting terminal counts and capacity.
- `scripts/check-phase123-runtime-timing-evidence-integrity/` - Follows the extracted node send-before-ack boundary and retains its mutation proof.
- `scripts/check-phase128-production-compact-announcement-transport.ts` - Follows the extracted outbound write boundary and updated enqueue signature.
- `docs/parity/source-breadcrumbs.json` - Registers both new Rust regression/helper modules with canonical Knots anchors.
- `docs/metrics/lines-of-code.md` - Regenerated by normal verification hooks.

## Decisions Made

- Exact abort validates authority incarnation and the full immutable peer/effect/session binding but deliberately ignores current lifecycle and target-peer freshness; those values classify achieved completion, not ownership of an unachieved reservation.
- Node and RPC shells consume capabilities only through `acknowledge_write` plus completion after confirmed transport success, or through abort before achievement.
- Cleanup loops continue across the full suffix after an abort error and return the first cleanup failure, preventing one bad classification from hiding additional leaked reservations.
- Enqueue pressure, missing outboxes, peer queue pressure, poisoned registry access, and unregister cleanup are terminal abort boundaries because each can otherwise strand a prepared reservation.
- `MPLIFE-01` and `MPLIFE-04` remain unchecked for independent phase-level verification.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 2 - Missing Critical] Closed outbox enqueue and unregister reservation leaks**

- **Found during:** Task 2 (peer fanout executor audit)
- **Issue:** Prepared emissions skipped after snapshot/enqueue races, queue pressure, missing outboxes, or peer unregister were dropped without completion or abort.
- **Fix:** Passed the authoritative network handle into enqueue/unregister and explicitly aborted every ready emission that could not remain queued.
- **Files modified:** `sync.rs`, `sync/session.rs`, RPC admission and announcement tests
- **Verification:** Enqueue-race and unregister regressions recovered the full pending capacity; node and RPC suites passed.
- **Committed in:** `ecc2f624`

**2. [Rule 3 - Blocking] Split abort regressions and fanout terminal logic under managed file limits**

- **Found during:** Tasks 1 and 2
- **Issue:** Adding the required regressions and terminal handling directly to existing Rust roots reached the repository's strict below-628-line production limit.
- **Fix:** Added focused `peer_abort.rs` and `session/emission_terminal.rs` modules, registered canonical breadcrumbs, and kept the original roots below the limit.
- **Files modified:** lifecycle effect test modules, `sync/session.rs`, `sync/session/emission_terminal.rs`, `docs/parity/source-breadcrumbs.json`
- **Verification:** Bright Builds reported zero findings; the final hook reported 336 production Rust files below 628 lines and verified 733 breadcrumbed Rust files.
- **Committed in:** `0ab43d71`, `ecc2f624`

**3. [Rule 3 - Blocking] Reconciled Phase 123 integrity checks with the extracted send terminal**

- **Found during:** Task 2 normal commit hook
- **Issue:** The historical Phase 123 checker required the peer send and acknowledgement strings inside `sync/session.rs` even though the behavior moved intact to `sync/session/emission_terminal.rs`.
- **Fix:** Added the helper to the checker corpus and moved the ordered send-before-ack assertion and mutation fixture to that file.
- **Files modified:** Phase 123 checker constants, checks, and mutation tests
- **Verification:** Phase 123 passed 38 mutation tests and its live checker.
- **Committed in:** `ecc2f624`

**4. [Rule 3 - Blocking] Reconciled Phase 128 transport checks with extraction and abort-aware enqueue**

- **Found during:** Task 2 normal commit hook
- **Issue:** The Phase 128 checker hard-coded the old `sync/session.rs` write boundary and one-argument `enqueue_prepared` call.
- **Fix:** Pointed the outbound ordered-write proof and mutation at the helper and updated durable dispatch to require the authoritative network argument.
- **Files modified:** Phase 128 checker and mutation tests
- **Verification:** Phase 128 passed 20 mutation tests and its live checker.
- **Committed in:** `ecc2f624`

**5. [Rule 3 - Blocking] Corrected stale focused test and breadcrumb command assumptions**

- **Found during:** Task 1 focused verification and Task 2 first commit attempt
- **Issue:** The plan's `network::tests::effects` selector selected no decomposed tests, and the new helper's handwritten breadcrumb block lacked all canonical generated anchors.
- **Fix:** Used the decomposed lifecycle projection selector and ran the repository breadcrumb writer before retrying the normal hook.
- **Files modified:** `sync/session/emission_terminal.rs`
- **Verification:** Focused lifecycle effects passed 35 tests; the final hook verified all breadcrumbs.
- **Committed in:** `ecc2f624`

**Total deviations:** 5 auto-fixed (1 missing critical, 4 blocking)
**Impact on plan:** Every deviation was required to make the complete-or-abort guarantee true at real ownership-loss boundaries or keep existing deterministic guardrails compatible; no new dependency, durable outbox, endpoint, schema, or public-network behavior was introduced.

## Issues Encountered

- Task 1 TDD RED failed on unresolved abort vocabulary and missing authority facades.
- Task 2 TDD RED reproduced the leak as `pending peer lifecycle effects are at capacity` after a first-write failure.
- Failing Rust RED states were not committed separately because repository policy requires formatting, Clippy, build, and tests to pass before every Rust commit; each task retained one atomic green commit.
- Task 1's first normal hook correctly rejected a 628-line production file; simplification brought it to 627 before the successful retry.
- Task 2's first normal hook rejected a stale breadcrumb block, its second rejected the Phase 123 hard-coded location, and its third rejected the Phase 128 hard-coded location. After the compatible checker repairs, the fourth normal hook passed in 41m 40s.
- The full verifier exceeded its advisory runtime threshold during known macOS test-binary startup delays and captured liveness evidence under `.local/open-bitcoin-dev/stall-diagnostics/`; it continued without interruption and completed successfully.

## Verification

- TDD RED: missing exact abort API failed compilation; first RPC write failure exhausted pending peer lifecycle capacity.
- Focused GREEN: 35 lifecycle-effect tests passed; 9 RPC successful-prefix/abort tests passed; node production announcement failure and cleanup cases passed.
- Ordered `cargo fmt`, Clippy with `-D warnings`, all-target/all-feature build, and all-feature workspace tests passed.
- `bun scripts/bright-builds-check.ts all`: zero findings.
- Phase 123 compatibility suite: 38 passed, 0 failed.
- Phase 128 compatibility suite: 20 passed, 0 failed.
- Phase 134 lifecycle suite: 89 passed, 0 failed.
- Final normal hook: node 584 passed with 1 expected public-network test ignored; RPC 163 passed; Bazel smoke build, benchmark smoke, doctests, coverage, parity breadcrumbs, LOC freshness, and `git diff --check` passed.

## Known Stubs

None.

## Threat Flags

None. T-134-16-01 through T-134-16-03 cover the bounded pending ledger, successful-prefix evidence, and exact abort binding. No unplanned endpoint, authentication path, file-access boundary, schema boundary, dependency, or network I/O under the authority guard was introduced.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Plan 16 closes the peer reserved-effect leak and provides an explicit terminal path for every audited node/RPC emission owner.
- Later lifecycle and relay plans can rely on bounded capacity recovery without weakening successful-prefix evidence.
- `MPLIFE-01` and `MPLIFE-04` intentionally remain pending until independent phase-level re-verification.

## Self-Check: PASSED

The summary, both task commits, all key implementation and guardrail files, and canonical breadcrumbs exist; frontmatter uses exactly two delimiters; no goal-blocking stubs or unplanned threat surface were found; and MPLIFE-01/MPLIFE-04 remain unchecked and Pending.

*Phase: 134-authoritative-cross-cache-lifecycle-integration*
*Completed: 2026-07-29*
