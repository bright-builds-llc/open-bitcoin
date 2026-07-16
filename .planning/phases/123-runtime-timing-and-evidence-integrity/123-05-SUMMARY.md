---
phase: 123-runtime-timing-and-evidence-integrity
plan: 05
subsystem: sync-runtime-observability
tags: [rust, block-relay, metrics, structured-logging, achieved-effect-evidence, HARD-03, HARD-04]

requires:
  - phase: 123-runtime-timing-and-evidence-integrity
    plan: 02
    provides: private runtime block-relay snapshot and successful Block write count
  - phase: 123-runtime-timing-and-evidence-integrity
    plan: 03
    provides: sync transport acknowledgement at the successful write boundary
provides:
  - one availability-gated block-relay snapshot from the sync-owned managed network per tick
  - shared metric and structured-log projection from the same borrowed runtime snapshot
  - independent served-block projection from achieved Block writes rather than peer eligibility
  - deterministic omission, compact-activity, served-count, and inbound-provider regression coverage
affects: [123-06, 123-07, block-relay-metrics, block-relay-logs, daemon-sync]

tech-stack:
  added: []
  patterns:
    - Sample runtime evidence once after peer processing and borrow it into every effect
    - Gate unobserved evidence before projection so both metrics and logs omit together

key-files:
  created:
    - packages/open-bitcoin-node/src/sync/tests/runtime_projection_cases.rs
  modified:
    - packages/open-bitcoin-node/src/sync.rs
    - packages/open-bitcoin-node/src/sync/metrics.rs
    - packages/open-bitcoin-node/src/sync/runtime_state.rs
    - packages/open-bitcoin-node/src/sync/tests.rs
    - packages/open-bitcoin-node/src/metrics/block_relay.rs
    - packages/open-bitcoin-node/src/metrics/tests.rs
    - packages/open-bitcoin-node/src/logging.rs
    - packages/open-bitcoin-node/src/logging/tests.rs
    - packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs

key-decisions:
  - "Use the Plan 02 BlockRelayRuntimeEvidenceSnapshot directly; do not retain a provider bridge or add a second carrier."
  - "Treat block-serving activation availability as the single omission gate for both metric and log projection."
  - "Keep the inbound metric provider unchanged and independent from sync-owned block-relay evidence."

patterns-established:
  - "Authoritative projection: sample one owner, gate once, borrow twice."
  - "Achieved-effect counters are explicit projection inputs and never inferred from eligibility counters."

requirements-completed:
  - HARD-03
  - HARD-04
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 123-2026-07-15T18-12-00
generated_at: 2026-07-16T03:25:40Z

duration: 35 min
completed: 2026-07-16
---

# Phase 123 Plan 05: Authoritative Runtime Projection Summary

**Each sync tick now samples block-relay evidence once from its own managed network and projects the same achieved-write count and compact activity to metrics and structured logs.**

## Performance

- **Duration:** 35 min
- **Completed:** 2026-07-16T03:25:40Z
- **Tasks:** 1 atomic TDD projection migration
- **Files modified:** 10

## Accomplishments

- Replaced eligibility-derived served output with an explicit private `served_count` input for both fixed metric and structured-log helpers.
- Deleted the block-relay provider field, setter, runtime lookups, and daemon `ManagedRpcContext` closure while preserving the inbound metric provider.
- Added one availability-gated `maybe_authoritative_block_relay_snapshot` sample after peer processing and passed the same borrowed snapshot to metrics and logs.
- Proved unobserved omission, eligibility `2` versus served writes `9`, compact-announcement coherence across both effects, and unchanged inbound samples.
- Preserved fixed metric kinds, log source/labels, retention, health signals, sensitive-marker guards, and all public schemas.

## Task Commits

1. **RED: Require authoritative served counts** — `ec65d984`
2. **GREEN: Project authoritative block-relay evidence** — `dd124283`

## Verification Results

```text
Phase 123 node test-target compile: passed
Phase 123 focused tests: 22 passed, 0 failed
Block-relay regression filter: 22 passed, 0 failed
RPC daemon_sync test-target compile: passed
Final git diff --check and acceptance searches: passed
```

The orchestrator owns full repository verification and parity-breadcrumb manifest registration after wave merge.

## Decisions Made

- The sync-owned `ManagedPeerNetwork` is the sole block-relay projection authority; RPC runtime state can no longer spoof or diverge from sync activity.
- Activation availability gates the complete snapshot once, so an unobserved network emits neither zero-valued relay metrics nor a misleading relay log.
- The private served-write counter remains outside serialized status while being passed explicitly into the two fixed projection helpers.

## Deviations from Plan

### Process Deviation

- A dedicated RED no-run compile was executed immediately after changing the helper contracts and failed at the two intentionally unmigrated production callers. The required named compile ran only after every production/test caller, provider removal, daemon cleanup, and focused test was complete. No source-scope or delivery deviation resulted.

## Issues Encountered

- The first GREEN compile reported one obsolete `mut` in a rewritten fixture; it was removed before focused verification and no warning remained in subsequent builds.

## User Setup Required

None - no external service configuration required.

## Residual Risks

- The new Rust test file still needs registration in `docs/parity/source-breadcrumbs.json`; the orchestrator owns this merged-wave step as instructed.
- Full repository verification remains Plan 123-07's lifecycle gate.

## Next Phase Readiness

- Plan 123-06 can update checker/docs against the final authoritative one-snapshot architecture.
- Plan 123-07 can register the new breadcrumb, run repository verification, and validate the completed lifecycle.

## Self-Check: PASSED

- FOUND: `packages/open-bitcoin-node/src/sync.rs` (`maybe_authoritative_block_relay_snapshot` and one direct sample)
- FOUND: `packages/open-bitcoin-node/src/sync/metrics.rs` (explicit optional snapshot input)
- FOUND: `packages/open-bitcoin-node/src/sync/runtime_state.rs` (same optional snapshot input)
- FOUND: `packages/open-bitcoin-node/src/sync/tests/runtime_projection_cases.rs` (all three exact Phase 123 tests)
- FOUND: `packages/open-bitcoin-node/src/metrics/block_relay.rs` (explicit served count)
- FOUND: `packages/open-bitcoin-node/src/logging.rs` (explicit served count)
- FOUND: `.planning/phases/123-runtime-timing-and-evidence-integrity/123-05-SUMMARY.md`
- FOUND COMMITS: `ec65d984`, `dd124283`

***

*Phase: 123-runtime-timing-and-evidence-integrity*
*Completed: 2026-07-16*
