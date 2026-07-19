---
phase: 127-authoritative-network-state-unification
plan: "04"
subsystem: production-composition-guardrails
tags:
  - rust
  - typescript
  - integration
  - parity
  - mutation-testing
requires:
  - phase: 127-authoritative-network-state-unification
    provides: shared network authority, durable block source, and owned operator snapshot
  - phase: 126-compact-relay-residual-hardening
    provides: deterministic mutation-checker precedent and verifier ordering boundary
provides:
  - production-shaped sync, restart serving, and HTTP RPC integration proof
  - deterministic four-mutation guard against split network authority regressions
  - lifecycle-aware Phase 124 reconciliation through active and completed Phase 127
  - bounded Knots parity evidence with explicit Phase 128 and Phase 129 exclusions
affects:
  - phase-128
  - phase-129
tech-stack:
  added: []
  patterns:
    - one-datadir loopback composition proof across sync, durable restart, inbound serving, and RPC
    - deterministic source-corpus checker backed by exact mutation failures
    - lifecycle-stage validation tied to shared frontmatter identity
key-files:
  created:
    - scripts/check-phase127-authoritative-network-state-unification.ts
    - scripts/check-phase127-authoritative-network-state-unification.test.ts
  modified:
    - packages/open-bitcoin-rpc/tests/black_box_parity.rs
    - scripts/check-phase124-post-audit-gap-planning.ts
    - scripts/check-phase124-milestone-gap-closure.test.ts
    - scripts/verify.sh
    - docs/parity/index.json
    - docs/parity/catalog/p2p.md
    - docs/parity/source-breadcrumbs.json
    - docs/metrics/lines-of-code.md
key-decisions:
  - Register Phase 127 as integration evidence without duplicating canonical v2.1 requirement ownership.
  - Treat the existing direct dashboard and support regressions as the presentation proof while the black-box fixture proves shared RPC and serving provenance.
  - Accept lifecycle-valid Phase 127 planning, execution, summary, verification, promotion, and completion states while keeping Phases 128 and 129 strictly pending.
patterns-established:
  - "Production composition proof: mutate through durable sync, reopen the same datadir, serve over a real loopback peer, then compare HTTP RPC with the authoritative snapshot."
  - "Mutation guard: every prohibited split-authority shape has a named temporary-corpus mutation that must fail independently."
requirements-completed:
  - BSRV-03
  - BSRV-04
  - OBS-02
  - OBS-04
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 127-2026-07-19T15-09-40
generated_at: 2026-07-19T21:52:20Z
duration: 1h 8m
completed: 2026-07-19
---

# Phase 127 Plan 04: Production Composition Guardrails Summary

The authoritative network-state repair now has production-shaped end-to-end proof, exact Knots parity evidence, four independent structural mutations, and a passing public-network-free repository contract.

## Performance

- **Duration:** 1h 8m
- **Started:** 2026-07-19T20:44:27Z
- **Completed:** 2026-07-19T21:52:20Z
- **Tasks:** 2
- **Files changed:** 10

## Accomplishments

- Added a one-datadir integration fixture that mutates through `DurableSyncRuntime`, observes the pre-existing shared RPC context, reopens after cache loss, serves the persisted block over a real loopback inbound peer, and verifies the same truth through HTTP RPC.
- Locked exact `getblockchaininfo`, `getnetworkinfo`, and `openbitcoinnetworkstatus` top-level schemas while proving the block-relay aggregate matches the authoritative owned snapshot and excludes permission, endpoint, credential, transaction, and dynamic-label material.
- Registered a bounded Phase 127 parity surface with the five required Bitcoin Knots authority, RPC manager, peer-processing, validation, and block-storage anchors.
- Added a deterministic Phase 127 checker and exactly four required mutation cases for duplicate production allocation, fresh RPC chainstate, cache-only serving, and non-authoritative projection.
- Evolved the Phase 124 post-audit guard to validate Phase 127 plans, summaries, verification, promotion, completion, and routing by lifecycle identity without weakening the pending Phase 128 or Phase 129 boundaries.
- Refreshed the tracked LOC report through the full verifier and installed the Phase 127 mutation/check pair immediately after Phase 126 and before the final Phase 117 gate.

## Task Commits

Each task was committed atomically:

1. **Task 1: Add production-path and restart-shaped parity integration proof** - `c8f42303`
2. **Task 2: Install mutation-tested Phase 127 guard and run full verification** - `5b7f59e2`

## Files Created and Modified

- `packages/open-bitcoin-rpc/tests/black_box_parity.rs` - Proves shared sync/RPC authority, restart-shaped durable serving, exact RPC schemas, and aggregate redaction over loopback-only transports.
- `scripts/check-phase127-authoritative-network-state-unification.ts` - Enforces production authority composition, durable lookup, owned operator projection, evidence, exclusions, and verifier order.
- `scripts/check-phase127-authoritative-network-state-unification.test.ts` - Rejects each of the four required structural regressions independently.
- `scripts/check-phase124-post-audit-gap-planning.ts` - Reconciles legal Phase 127 lifecycle stages and promotion/routing projections.
- `scripts/check-phase124-milestone-gap-closure.test.ts` - Covers active, completed, stale-lifecycle, and stale-route Phase 127 states.
- `scripts/verify.sh` - Runs the Phase 127 mutation test and live checker before Phase 117.
- `docs/parity/catalog/p2p.md` - Records the bounded claim, exact Knots anchors, and Phase 128/129 exclusions.
- `docs/parity/index.json` - Registers Phase 127 integration evidence without duplicating canonical requirement ownership.
- `docs/parity/source-breadcrumbs.json` - Maps the black-box parity fixture to its expanded upstream anchors.
- `docs/metrics/lines-of-code.md` - Records the verifier-generated current repository line counts.

## Decisions Made

- The black-box test uses real loopback TCP for inbound serving and HTTP RPC because the production response-resolution helper is intentionally crate-private; no public-network connection or test-only production seam was introduced.
- Dashboard and support presentation remain proven by their existing direct Phase 127 regressions. The new integration test validates the authoritative serialized aggregate rather than creating an invalid CLI-to-RPC test dependency cycle.
- The new parity surface carries no canonical requirement ownership. Its rationale and evidence link BSRV-03, BSRV-04, OBS-02, and OBS-04 while their established v2.1 surfaces remain the unique owners required by the Phase 117 release-boundary checker.
- Phase 127 lifecycle reconciliation accepts the summary-before-roadmap transient state and verification-before-promotion state, but promotion requires a lifecycle-valid passed verification and completion additionally requires four summaries.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Evolved the Phase 124 post-audit guard for the active Phase 127 lifecycle**

- **Found during:** Task 2 preflight
- **Issue:** The existing Phase 124 guard hard-coded `**Plans:** 0 plans` for every post-audit gap phase and would reject the real four-plan/three-summary Phase 127 tree.
- **Fix:** Added lifecycle-aware Phase 127 artifact, progress, promotion, completion, and routing validation while retaining exact pending checks for Phases 128 and 129.
- **Files modified:** `scripts/check-phase124-post-audit-gap-planning.ts`, `scripts/check-phase124-milestone-gap-closure.test.ts`
- **Verification:** 73 Phase 124 reconciliation tests and the live repository checker passed.
- **Committed in:** `5b7f59e2`

**2. [Rule 1 - Bug] Avoided duplicate canonical parity requirement ownership**

- **Found during:** Task 1 normal commit hook
- **Issue:** Initially assigning the four Phase 127 requirements to a new integration-evidence surface duplicated their established v2.1 owners, which the Phase 117 checker correctly rejected.
- **Fix:** Kept the new surface as non-owning integration evidence and named the four linked requirements in its rationale.
- **Files modified:** `docs/parity/index.json`
- **Verification:** The Phase 117 mutation suite and live checker passed.
- **Committed in:** `c8f42303`

**3. [Rule 3 - Blocking] Used existing presentation regressions instead of introducing a dependency cycle**

- **Found during:** Task 1 design
- **Issue:** The RPC integration crate cannot import CLI dashboard/support rendering because the CLI already depends on RPC.
- **Fix:** Kept presentation proof in the existing direct dashboard/support tests and made the integration fixture assert the shared serialized aggregate and forbidden material boundary.
- **Files modified:** `packages/open-bitcoin-rpc/tests/black_box_parity.rs`, `scripts/check-phase127-authoritative-network-state-unification.ts`
- **Verification:** Focused RPC integration, Phase 127 checker, dashboard/support evidence guards, and full verification passed.
- **Committed in:** `c8f42303`, `5b7f59e2`

**Total deviations:** 3 auto-fixed issues.

**Impact on plan:** The deviations strengthened lifecycle precision, preserved unique parity ownership, and avoided an invalid dependency direction. No Phase 128 compact transport, Phase 129 milestone reconciliation, public-network gate, production readiness claim, or requirement promotion was added.

## Issues Encountered

- `scripts/check-parity-drift.ts`, named by the plan, does not exist in this repository. The available parity JSON validation, breadcrumb checker, Phase 117 release-boundary suite, and full verifier all passed instead.
- The first Task 1 hook stopped on duplicate parity ownership as intended. After correcting the surface registration, the full hook passed.
- Restoring the narrowly isolated Task 1 planning state produced one conflict because `git stash --keep-index` also captured staged parity-index content. The conflict was resolved to the committed duplicate-owner-safe index, all six planning-artifact hashes matched their pre-isolation values, and the named stash was dropped with no residual stash or unmerged entry.

## Verification

- Focused Phase 127 black-box integration test - passed.
- `bun run scripts/check-parity-breadcrumbs.ts --check` - passed for 384 Rust files.
- `bun test scripts/check-phase127-authoritative-network-state-unification.test.ts` - passed, 5/5 including all four required mutations.
- `bun run scripts/check-phase127-authoritative-network-state-unification.ts` - passed against the real repository corpus.
- `bun test scripts/check-phase124-milestone-closeout-reconciliation.test.ts` - passed, 73/73.
- Active traceability 21/21, Phase 126 16/16, and Phase 117 26/26 checker suites - passed with their live checks.
- Mandatory ordered Rust checks passed: format, clippy with warnings denied, all-targets/all-features build, and all-features workspace tests.
- `git diff --check`, JSON parsing, parity breadcrumbs, and production Rust file-length checks passed.
- Both normal commit hooks ran the full unisolated `bash scripts/verify.sh` contract successfully. Task 1 completed in 20m39.274s; Task 2 completed in 2m51.846s with warm caches.
- The full contract included deterministic guards, complete Cargo tests, benchmark list/smoke and report validation, Bazel build/provenance smoke, pure-core coverage, and LOC freshness. The only ignored test remained the explicitly opt-in public-network sync smoke.

## Authentication Gates

None.

## Known Stubs

None.

## Deferred Issues

- Phase 128 retains production compact negotiation, announcement construction, transport emission, and post-write compact-announcement evidence.
- Phase 129 retains aggregate four-flow integration guards, requirement promotion, milestone-audit reconciliation, and archive routing.

## Next Phase Readiness

- Phase 127 implementation and deterministic regression evidence are complete and ready for lifecycle verification/promotion.
- Phase 128 can build compact announcement transport on the shared network authority without recreating a second RPC, inbound, or operator projection.

## Self-Check: PASSED

- Verified both task commits `c8f42303` and `5b7f59e2` exist.
- Verified the two new checker files and all modified evidence files exist.
- Verified the summary has exactly one top-of-file YAML frontmatter block and matches lifecycle `127-2026-07-19T15-09-40`.
- Verified no unmerged paths or stashes remain.
- Verified the original three tracked lifecycle edits and six untracked planning/research/UI artifacts remain preserved and outside both task commits.

*Phase: 127-authoritative-network-state-unification*
*Completed: 2026-07-19*
