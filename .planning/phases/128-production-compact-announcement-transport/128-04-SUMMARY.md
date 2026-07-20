---
phase: 128-production-compact-announcement-transport
plan: "04"
subsystem: verification
tags: [rust, compact-blocks, parity, mutation-testing, lifecycle-guards]
requires:
  - phase: 128-03
    provides: Production durable-tip fanout through outbound and inbound session writes
provides:
  - Fail-closed aggregate guard for production compact announcement transport
  - Auditable Knots parity evidence for CMP-04, CMP-05, and OBS-03
  - Lifecycle-aware Phase 128 execution and completion reconciliation
  - Clean repository-wide Rust, Bazel, coverage, parity, and claim-boundary verification
affects: [129-soak-and-parity-closure, milestone-closeout, compact-relay, verification]
tech-stack:
  added: []
  patterns:
    - Mutation-tested aggregate guard after dependency guards and before the final claim boundary
    - Focused child modules for connection I/O and daemon runtime control
    - Fail-closed planning lifecycle states with exact counts and next routes
key-files:
  created:
    - packages/open-bitcoin-rpc/src/inbound_listener/connection_runtime.rs
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/runtime_control.rs
  modified:
    - scripts/check-phase128-production-compact-announcement-transport.ts
    - scripts/check-phase128-production-compact-announcement-transport.test.ts
    - scripts/check-phase124-post-audit-gap-planning.ts
    - docs/parity/index.json
    - docs/parity/catalog/p2p.md
    - scripts/verify.sh
key-decisions:
  - "Guard the complete production path from bilateral negotiation through durable trigger, live facts, real writes, consuming receipts, and fixed observability."
  - "Preserve the bounded default-off claim boundary: production composition evidence does not imply public defaults or production full-node readiness."
  - "Model Phase 128 Plan 04 executing and Phase 128 completed as distinct fail-closed lifecycle states, with completion routing only to Phase 129."
  - "Extract focused RPC connection and runtime-control child modules when Phase 128 growth crossed the repository's 628-line production source limit."
patterns-established:
  - "Aggregate guards migrate with intentional module seams so exact evidence remains executable instead of becoming stale text."
  - "Closeout reconciliation validates lifecycle identity, exact plan and requirement counts, and one canonical next action together."
requirements-completed: [CMP-04, CMP-05, OBS-03]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 128-2026-07-20T01-54-33
generated_at: 2026-07-20T06:45:32Z
duration: 57min
completed: 2026-07-20
---

# Phase 128 Plan 04: Production Compact Announcement Transport Summary

**Mutation-tested aggregate guardrails now prove that negotiated compact announcements travel from durable best-tip activation through live peer sessions and receive evidence only after successful production writes.**

## Performance

- **Duration:** 57 min
- **Started:** 2026-07-20T05:48:36Z
- **Completed:** 2026-07-20T06:45:32Z
- **Tasks:** 3
- **Files modified:** 25

## Accomplishments

- Added an 18-mutation Phase 128 aggregate checker covering bilateral negotiation, post-durable triggering, live peer facts, bounded owned emissions, outbound and inbound write boundaries, consuming receipts, fixed observability, production composition, and bounded claims.
- Registered exact Bitcoin Knots anchors and production evidence for CMP-04, CMP-05, and OBS-03 in the parity manifest, index, and P2P catalog.
- Migrated earlier Phase 122, Phase 123, Phase 124, and Phase 126 guards so their exact assertions remain valid across the final production composition and planning lifecycle.
- Preserved the production source-shape contract by extracting focused inbound connection and daemon runtime-control modules without changing behavior.
- Completed the repository-native verification contract, including Rust formatting, clippy, all-target build, all-feature tests, Bazel smoke build, coverage, parity breadcrumbs, claim guards, and LOC freshness.

## Task Commits

Each task was committed atomically:

1. **Task 1: Add the aggregate production transport guard** - `9108b95a` (test)
2. **Task 2: Register production compact parity evidence** - `a37f1015` (docs)
3. **Task 3: Run and close the deterministic verification contract** - `f12da454` (chore)

## Files Created/Modified

- `scripts/check-phase128-production-compact-announcement-transport.ts` - Validates the complete production compact-announcement path and bounded scope.
- `scripts/check-phase128-production-compact-announcement-transport.test.ts` - Supplies the complete corpus plus 17 independent fail-closed mutations.
- `scripts/check-phase123-runtime-timing-evidence-integrity.ts` - Follows the post-write acknowledgement seam into the focused connection module.
- `scripts/check-phase124-post-audit-gap-planning.ts` - Distinguishes Phase 128 Plan 04 execution from completed routing to Phase 129.
- `scripts/check-phase124-milestone-closeout-reconciliation.fixtures.ts` - Models valid and invalid Phase 128 lifecycle combinations.
- `scripts/verify.sh` - Runs Phase 128 after its dependencies and before the unchanged final claim-boundary guard.
- `docs/parity/index.json` and `docs/parity/catalog/p2p.md` - Record production-path evidence while retaining default-off and deferred-scope language.
- `docs/parity/source-breadcrumbs.json` - Registers new source modules and the Phase 128 production regression anchor.
- `packages/open-bitcoin-rpc/src/inbound_listener/connection_runtime.rs` - Owns inbound message-loop I/O, announcement draining, and post-write completion.
- `packages/open-bitcoin-rpc/src/bin/open_bitcoind/runtime_control.rs` - Owns daemon shutdown, wait, timestamp, and lifecycle persistence helpers.
- `README.md` - Reconciles contributor-facing milestone status with the completed Phase 128 production composition.
- `docs/metrics/lines-of-code.md` - Captures the verified worktree's current LOC report.

## Decisions Made

- The aggregate guard checks achieved effects, not preparation intent: receipt completion must follow a successful session write on both outbound and inbound paths.
- Production composition is evidenced by one authoritative outbox registry shared with session owners; this remains a bounded explicit-activation claim, not a public-network or production-readiness claim.
- Earlier phase guards were migrated rather than weakened. Their mutation suites still fail when timing, write-order, lifecycle, ownership, or route facts drift.
- RPC source growth was resolved with two cohesive child modules; no API, storage schema, dependency, network endpoint, or runtime activation behavior changed.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Added an exact parity breadcrumb to the production regression source**

- **Found during:** Task 2 parity breadcrumb verification
- **Issue:** The Phase 128 production regression file needed a first-party source breadcrumb registered both inline and in the manifest.
- **Fix:** Added the exact Knots source comment and registered the file in `docs/parity/source-breadcrumbs.json`.
- **Files modified:** `packages/open-bitcoin-node/src/sync/tests/production_announcement_transport_cases.rs`, `docs/parity/source-breadcrumbs.json`
- **Verification:** `bun run scripts/check-parity-breadcrumbs.ts` passed for 387 Rust files.
- **Committed in:** `a37f1015`

**2. [Rule 2 - Missing Critical] Corrected stale contributor-facing milestone status**

- **Found during:** Task 3 README freshness review
- **Issue:** The root README still described production compact transport as pending after Phase 128 established it.
- **Fix:** Updated only the milestone-status paragraph, preserving default-off, deferred public surfaces, and the remaining Phase 129 work.
- **Files modified:** `README.md`
- **Verification:** The Phase 117 and Phase 128 claim-boundary suites and full verifier passed.
- **Committed in:** `f12da454`

**3. [Rule 3 - Blocking] Migrated the Phase 123 timing and evidence guard**

- **Found during:** Task 3 first full-verifier attempt
- **Issue:** Phase 123 still asserted the pre-Phase-128 session and acknowledgement shape.
- **Fix:** Retargeted it to peer-scoped sends, live reconciliation, post-durable dispatch, post-write receipt completion, and the focused inbound connection module; preserved and expanded its mutation coverage.
- **Files modified:** `scripts/check-phase123-runtime-timing-evidence-integrity.ts`, `scripts/check-phase123-runtime-timing-evidence-integrity.test.ts`
- **Verification:** All 38 Phase 123 checker tests and the live checker passed.
- **Committed in:** `f12da454`

**4. [Rule 3 - Blocking] Added Phase 128 execution and completion lifecycle states**

- **Found during:** Task 3 second full-verifier attempt
- **Issue:** The Phase 124 reconciliation guard modeled only through Phase 127 and rejected the valid 3/4 Phase 128 execution state.
- **Fix:** Added exact executing and completed variants, mutations for lifecycle/count/route drift, and reconciled aggregate roadmap and requirement counts to 36 complete and 3 pending.
- **Files modified:** `scripts/check-phase124-post-audit-gap-planning.ts`, `scripts/check-phase124-milestone-closeout-reconciliation.fixtures.ts`, `scripts/check-phase124-milestone-closeout-reconciliation.test.ts`, `.planning/ROADMAP.md`, `.planning/REQUIREMENTS.md`
- **Verification:** All 77 Phase 124 tests and the real-repository checker passed.
- **Committed in:** `f12da454`

**5. [Rule 4 - Authorized Architecture] Extracted focused RPC runtime modules**

- **Found during:** Task 3 third full-verifier attempt
- **Issue:** Phase 128 production additions pushed `inbound_listener.rs` and `open-bitcoind.rs` over the repository's 628-line production source limit.
- **Fix:** Moved the inbound connection loop and daemon runtime-control helpers into cohesive child modules, migrated exact checker targets, and registered their parity breadcrumbs.
- **Files modified:** `packages/open-bitcoin-rpc/src/inbound_listener.rs`, `packages/open-bitcoin-rpc/src/inbound_listener/connection_runtime.rs`, `packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs`, `packages/open-bitcoin-rpc/src/bin/open_bitcoind/runtime_control.rs`, Phase 123 and Phase 128 checkers, `docs/parity/source-breadcrumbs.json`
- **Verification:** Production file-length check passed for 291 files; focused RPC tests, all Rust gates, and the full verifier passed.
- **Committed in:** `f12da454`

**6. [Rule 1 - Bug] Corrected final plan progress after state updater overcount**

- **Found during:** Plan metadata update
- **Issue:** The generic state progress updater counted historical plan artifacts outside the active milestone and reported 68 completed plans against 66 total.
- **Fix:** Reconciled the active milestone to the 62-plan Phase 110-127 baseline plus four Phase 128 summaries, then routed the completed Phase 128 lifecycle to Phase 129.
- **Files modified:** `.planning/STATE.md`, `.planning/ROADMAP.md`
- **Verification:** Phase 124 real-repository lifecycle reconciliation passes with Phase 128 at 4/4, 36/39 requirements complete, and the sole next route `/gsd-plan-phase 129`.
- **Committed in:** Plan metadata commit

***

**Total deviations:** 6 resolved (1 bug, 1 missing critical, 3 blocking, 1 authorized architecture)
**Impact on plan:** Each change was required to keep evidence, lifecycle state, contributor documentation, and source-shape enforcement accurate. No production capability or public claim was added.

## Issues Encountered

- Three early full-verifier attempts correctly exposed stale Phase 123 assertions, missing Phase 128 lifecycle modeling, and source-shape overflow. Each issue was repaired at its owning seam, after which the complete verifier passed.
- The plan named node and network crate READMEs for review, but those files do not exist. The root README was the only contributor-facing status surface requiring an update.

## Verification Evidence

- `cargo fmt --manifest-path packages/Cargo.toml --all`
- `cargo clippy --manifest-path packages/Cargo.toml --all-targets --all-features -- -D warnings`
- `cargo build --manifest-path packages/Cargo.toml --all-targets --all-features`
- `cargo test --manifest-path packages/Cargo.toml --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc announcement_transport`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc daemon_sync`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --test black_box_parity`
- `bun test scripts/check-phase123-runtime-timing-evidence-integrity.test.ts` — 38 passed.
- `bun test scripts/check-phase124-milestone-closeout-reconciliation.test.ts` — 77 passed.
- `bun test scripts/check-phase128-production-compact-announcement-transport.test.ts` — 18 passed.
- `bun run scripts/check-parity-breadcrumbs.ts` — 387 Rust files verified.
- `bash scripts/check-file-lengths.sh` — 291 production Rust files below the 628-line limit.
- `bash scripts/verify.sh` — completed successfully in 4m 30.572s.

## Known Stubs

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Phase 128's production transport and aggregate guardrails are complete; Phase 129 can add final integration/soak evidence and reconcile the v2.1 milestone.
- No implementation, verification, authentication, or environment blocker remains.

## Self-Check: PASSED

- Created source, checker, and summary files exist.
- Task commits `9108b95a`, `a37f1015`, and `f12da454` exist in repository history.
- No goal-blocking stubs or unregistered security surfaces were introduced.

*Phase: 128-production-compact-announcement-transport*
*Completed: 2026-07-20*
