---
phase: 128-production-compact-announcement-transport
plan: "03"
subsystem: networking
tags: [rust, compact-blocks, peer-transport, durable-sync, observability]
requires:
  - phase: 128-01
    provides: Bilateral compact-block negotiation and live peer facts
  - phase: 128-02
    provides: Owned peer emissions, bounded preparation, and consuming write receipts
provides:
  - Post-persistence durable best-tip announcement trigger
  - Bounded per-peer outboxes shared by outbound and inbound sessions
  - Successful-prefix receipt completion on real transport write boundaries
  - Production-shaped fanout, failure, and redaction regressions
affects: [128-04, 129-soak-and-parity-closure, compact-relay, inbound-networking]
tech-stack:
  added: []
  patterns:
    - Typed post-durable event before announcement preparation
    - Session-owned bounded outbox drained outside authoritative network locks
    - Consuming receipt completed immediately after each successful write
key-files:
  created:
    - packages/open-bitcoin-node/src/sync/tests/production_announcement_transport_cases.rs
  modified:
    - packages/open-bitcoin-node/src/sync.rs
    - packages/open-bitcoin-node/src/sync/block_response.rs
    - packages/open-bitcoin-node/src/sync/block_reconcile.rs
    - packages/open-bitcoin-node/src/sync/session.rs
    - packages/open-bitcoin-rpc/src/inbound_listener.rs
    - packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs
key-decisions:
  - "Collapse multi-block reconciliation to one final DurableTipAdvanced event after persistence."
  - "Share bounded volatile outboxes across the durable runtime and inbound listener while keeping socket I/O outside authority guards."
  - "Drain an owned FIFO batch once and credit each successful prefix immediately; failed and unsent suffixes are dropped without credit or implicit retry."
patterns-established:
  - "Durable trigger: validate and activate, persist, then prepare announcements from live authority facts."
  - "Write evidence: preserve PeerEmission ownership through transport and consume its receipt only at the achieved write boundary."
requirements-completed: [CMP-04, CMP-05, OBS-03]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 128-2026-07-20T01-54-33
generated_at: 2026-07-20T05:41:20Z
duration: 51min
completed: 2026-07-20
---

# Phase 128 Plan 03: Production Compact Announcement Transport Summary

**Validated durable tips now fan out policy-selected compact, header, or inventory announcements through their owning live sessions, with evidence credited only after successful writes.**

## Performance

- **Duration:** 51 min
- **Started:** 2026-07-20T04:50:41Z
- **Completed:** 2026-07-20T05:41:20Z
- **Tasks:** 3
- **Files modified:** 9

## Accomplishments

- Added a typed `DurableTipAdvanced` boundary that fires once for the final newly active tip only after durable persistence succeeds.
- Wired a bounded process-wide outbox registry into both outbound sync sessions and inbound Tokio sessions without carrying authority guards across storage, encoding, socket I/O, awaits, logging, or shutdown.
- Preserved consuming `PeerEmissionReceipt` values through the write boundary so successful prefixes receive exactly-once credit and failed or unsent suffixes receive none.
- Added production-shaped multi-peer regressions for compact/header/inventory selection from live peer facts, partial failure, disconnect cleanup, fixed aggregates, and sensitive-field redaction.

## Task Commits

Each task was committed atomically:

1. **Task 1: Emit one post-durable best-tip announcement event** - `d784c197` (feat)
2. **Task 2: Drain peer emissions through real session transports and complete receipts** - `1187a669` (feat)
3. **Task 3: Prove end-to-end negotiation, fanout, fallback, and observability** - `7f4ea894` (test)

## Files Created/Modified

- `packages/open-bitcoin-node/src/sync.rs` - Owns the typed durable-tip sink and shared announcement outboxes.
- `packages/open-bitcoin-node/src/sync/block_response.rs` - Queues only newly durable active best-tip events.
- `packages/open-bitcoin-node/src/sync/block_reconcile.rs` - Collapses live reconciliation to its final newly activated tip.
- `packages/open-bitcoin-node/src/sync/session.rs` - Bounds peer outboxes and drains outbound emissions with successful-prefix receipt completion.
- `packages/open-bitcoin-node/src/sync/tests.rs` - Registers durable-tip and production transport regressions.
- `packages/open-bitcoin-node/src/sync/tests/production_announcement_transport_cases.rs` - Exercises live-fact fanout, failure semantics, observability, and redaction.
- `packages/open-bitcoin-rpc/src/inbound_listener.rs` - Drains owning inbound peer emissions and completes receipts after `Written`.
- `packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs` - Shares the durable runtime's outboxes and network authority with the production inbound listener.
- `docs/parity/source-breadcrumbs.json` - Registers the new pinned-Knots test breadcrumbs.

## Decisions Made

- Announcement preparation is installed as the durable runtime's default sink, so production callers cannot silently omit the Phase 128 transport path.
- Outbox snapshots are captured before authority preparation; prepared owned values are then enqueued under a separate bounded registry lock.
- A drain removes one peer's current FIFO batch before transport work. Each successful write consumes its receipt immediately, while a later failure drops the remaining owned values without replay or false evidence.
- Inbound sessions use the existing bounded read/write cadence as their smallest wakeup mechanism and always drain before the next read and after normal response writes.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Scoped the compatibility reconciliation helper to tests**

- **Found during:** Task 1 mandatory clippy gate
- **Issue:** The new live reconciliation entry point left the historical compatibility helper unused in production, and `-D warnings` rejected it as dead code.
- **Fix:** Added `#[cfg(test)]` to the compatibility helper while keeping startup and live production paths explicit.
- **Files modified:** `packages/open-bitcoin-node/src/sync/block_reconcile.rs`
- **Verification:** Format, clippy, all-target build, focused durable-tip tests, and all-feature tests pass.
- **Committed in:** `d784c197`

**2. [Rule 1 - Bug] Corrected stale plan progress after state updater overcount**

- **Found during:** Plan metadata update
- **Issue:** The state updater could not parse the pre-execution `Plan: Not started` position and then counted 67 completed plans against 66 total.
- **Fix:** Reconciled the milestone's explicit 62 completed baseline plus the three Phase 128 summaries to 65/66, and advanced the current position to Plan 04.
- **Files modified:** `.planning/STATE.md`
- **Verification:** Phase 128 roadmap progress is 3/4 and the state progress is 65/66 (98%).
- **Committed in:** Plan metadata commit

***

**Total deviations:** 2 auto-fixed (1 blocking, 1 bug)
**Impact on plan:** Both repairs preserve accurate production and planning state without scope expansion.

## Issues Encountered

- TDD failure signals were observed during implementation, but failing RED commits were not retained because the repository's hard Rust pre-commit contract requires format, clippy, build, and tests to pass before every commit. Each task was committed atomically after its focused and full gates passed.
- `bash scripts/verify.sh` stopped at the intentionally shared dirty `docs/metrics/lines-of-code.md` freshness check. This executor preserved that orchestrator-owned file unstaged and did not regenerate or commit it. All applicable Rust gates and all three plan-focused suites passed.

## Verification Evidence

- `cargo fmt --all --manifest-path packages/Cargo.toml`
- `cargo clippy --manifest-path packages/Cargo.toml --all-targets --all-features -- -D warnings`
- `cargo build --manifest-path packages/Cargo.toml --all-targets --all-features`
- `cargo test --manifest-path packages/Cargo.toml --all-features` — 460 node tests passed, 1 public-network smoke test ignored; all workspace and doc tests passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node durable_tip` — 2 passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node announcement_transport` — 11 passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc announcement_transport` — 1 passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node production_announcement_transport_cases` — 2 passed.

## Known Stubs

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Plan 04 can close aggregate verification and parity evidence using the real production transport established here.
- The shared LOC artifact must be refreshed by its current owner before the repo-native verifier can advance to its later aggregate checks.

## Self-Check: PASSED

- Created source and summary files exist.
- Task commits `d784c197`, `1187a669`, and `7f4ea894` exist in repository history.

*Phase: 128-production-compact-announcement-transport*
*Completed: 2026-07-20*
