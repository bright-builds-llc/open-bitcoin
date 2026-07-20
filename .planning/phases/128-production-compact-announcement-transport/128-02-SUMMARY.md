---
phase: 128-production-compact-announcement-transport
plan: "02"
subsystem: network
tags: [rust, compact-blocks, announcement-transport, write-receipts, observability]
requires:
  - phase: 128-01
    provides: live authoritative compact-relay negotiation and peer-session state
provides:
  - bounded peer-targeted compact, header, and inventory emissions
  - consuming write receipts for exactly-once announcement evidence
  - post-write-only peer/block provenance and aggregate observability
affects: [128-03, 128-04, compact-relay, sync-transport, block-relay-evidence]
tech-stack:
  added: []
  patterns:
    - owned emission leaves the network authority before transport effects
    - consuming receipt commits achieved-effect evidence after successful writes
key-files:
  created:
    - packages/open-bitcoin-node/src/network/announcement_transport.rs
    - packages/open-bitcoin-node/src/network/tests/announcement_transport_cases.rs
  modified:
    - packages/open-bitcoin-node/src/network.rs
    - packages/open-bitcoin-node/src/network/runtime_authority.rs
    - packages/open-bitcoin-node/src/network/block_relay_evidence.rs
    - packages/open-bitcoin-node/src/network/tests.rs
    - docs/parity/source-breadcrumbs.json
key-decisions:
  - "Derive receipt evidence from the bound wire-message variant instead of accepting a caller-supplied outcome."
  - "Record compact/header peer provenance only when the consuming write receipt completes; inventory success records only the fixed inventory fallback aggregate."
  - "Represent unavailable outboxes, queue pressure, disconnects, ineligibility, suppression, and construction failure as typed preparation outcomes."
patterns-established:
  - "Prepare then effect: live facts are read under one short authority lock and returned as owned values."
  - "Complete by capability: only a non-cloneable receipt created with an emission can credit achieved effects."
requirements-completed: [CMP-05, OBS-03]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 128-2026-07-20T01-54-33
generated_at: 2026-07-20T04:48:00Z
duration: 1h 13m
completed: 2026-07-20
---

# Phase 128 Plan 02: Announcement Transport Core Summary

**Bounded peer-targeted announcements now leave the network authority as owned emissions, while non-replayable write receipts credit compact/header/inventory evidence only after successful transport.**

## Performance

- **Duration:** 1h 13m
- **Started:** 2026-07-20T03:35:00Z
- **Completed:** 2026-07-20T04:48:00Z
- **Tasks:** 2
- **Files modified:** 11

## Accomplishments

- Prepared compact-block, headers, and inventory emissions from live negotiated peer state, header ancestry, eligibility, block availability, and bounded outbox pressure without holding the authority lock across effects.
- Bound peer, message, block identity, evidence reason, and write kind into a non-cloneable emission/receipt capability.
- Removed pre-write announcement evidence and provenance mutation; compact/header provenance and fixed aggregate evidence now change only when a consuming receipt completes.
- Added seven focused success, failure, suppression, partial-prefix, exactly-once, and redaction regressions, then migrated four older Phase 122/123 tests to the new write-completion boundary.

## Task Commits

Each task was committed atomically:

1. **Task 1: Prepare owned peer emissions from live authoritative facts** - `fcecddab` (feat)
2. **Task 2: Commit exactly-once achieved-effect evidence from consuming receipts** - `1f52b148` (feat)

## Files Created/Modified

- `packages/open-bitcoin-node/src/network/announcement_transport.rs` - Defines typed preparation outcomes, bounded outbox snapshots, owned emissions, and consuming receipts.
- `packages/open-bitcoin-node/src/network.rs` - Exposes the transport contract and removes legacy pre-write compact provenance mutation.
- `packages/open-bitcoin-node/src/network/runtime_authority.rs` - Provides short-lock preparation and consuming completion commands.
- `packages/open-bitcoin-node/src/network/block_relay_evidence.rs` - Maps actual wire messages to fixed outcomes and records evidence only from completed receipts.
- `packages/open-bitcoin-node/src/network/tests/announcement_transport_cases.rs` - Covers compact/header/inventory success, no-credit failures, queue suppression, partial prefixes, and public redaction.
- `packages/open-bitcoin-node/src/network/tests.rs` - Registers the focused suite and updates earlier evidence assertions.
- `packages/open-bitcoin-node/src/network/tests/compact_cleanup_cases.rs` - Completes the write receipt before checking disconnect cleanup.
- `packages/open-bitcoin-node/src/network/tests/compact_misbehavior_cases.rs` - Completes the write receipt before exercising malformed `getblocktxn`.
- `packages/open-bitcoin-node/src/network/tests/relay_serving_cases.rs` - Completes the write receipt before checking compact transaction serving.
- `packages/open-bitcoin-node/src/sync/tests/runtime_projection_cases.rs` - Completes the write receipt before projecting metrics and logs.
- `docs/parity/source-breadcrumbs.json` - Registers the new production and focused test files against pinned Knots sources.

## Decisions Made

- Evidence intent is derived from the actual `WireNetworkMessage` held by the emission, making mismatched caller-selected acknowledgement unrepresentable.
- `PeerEmissionReceipt` is consumed by value and deliberately cannot be cloned; a compile-fail doctest protects that ownership contract.
- Header-sent ancestry is updated for successful compact-block and headers writes, while inventory fallback does not create header provenance.
- Queue admission is a typed, per-peer snapshot boundary with fail-closed outcomes; this plan did not add retries, actors, async locks, or general P2P emission machinery.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Extracted the announcement transport deep module to satisfy verifier file-size policy**

- **Found during:** Task 1 (Prepare owned peer emissions from live authoritative facts)
- **Issue:** The initial implementation pushed `network.rs` and `runtime_authority.rs` above the repository's 628-line production-file limit.
- **Fix:** Moved preparation policy, typed outcomes, emission ownership, and focused unit tests into `announcement_transport.rs`, leaving thin authority and network entrypoints.
- **Files modified:** `packages/open-bitcoin-node/src/network.rs`, `packages/open-bitcoin-node/src/network/runtime_authority.rs`, `packages/open-bitcoin-node/src/network/announcement_transport.rs`
- **Verification:** Final line counts are 611, 627, and 447 respectively; the Task 1 full verifier passed.
- **Committed in:** `fcecddab`

**2. [Rule 1 - Regression] Migrated four stale tests from construction-time provenance to successful-write completion**

- **Found during:** Task 2 (Commit exactly-once achieved-effect evidence from consuming receipts)
- **Issue:** Four Phase 122/123 tests assumed `announce_block` immediately recorded compact provenance and aggregate evidence before any transport write.
- **Fix:** Each test now converts the selected compact message into a `PeerEmission`, consumes its bound receipt, completes the successful write, and retains its original serving, cleanup, disconnect, or observability assertion.
- **Files modified:** `packages/open-bitcoin-node/src/network/tests/compact_cleanup_cases.rs`, `packages/open-bitcoin-node/src/network/tests/compact_misbehavior_cases.rs`, `packages/open-bitcoin-node/src/network/tests/relay_serving_cases.rs`, `packages/open-bitcoin-node/src/sync/tests/runtime_projection_cases.rs`
- **Verification:** All four focused regressions pass and the complete all-feature workspace suite passes.
- **Committed in:** `1f52b148`

**Total deviations:** 2 auto-fixed (1 blocking, 1 regression)

**Impact on plan:** Both changes were directly required to preserve repository policy and existing behavior at the new post-write boundary. No runtime scope was added.

## Verification

- `cargo fmt --manifest-path packages/Cargo.toml --all` - passed
- `cargo clippy --manifest-path packages/Cargo.toml --all-targets --all-features -- -D warnings` - passed
- `cargo build --manifest-path packages/Cargo.toml --all-targets --all-features` - passed
- `cargo test --manifest-path packages/Cargo.toml --all-features` - passed, including 456 node tests, 1 intentional live-network ignore, and the receipt non-cloneability compile-fail doctest
- Focused `announcement_transport_cases`, `block_relay_evidence`, and four migrated regression commands - passed
- `bun run scripts/check-parity-breadcrumbs.ts` - passed for 386 Rust files
- `bash scripts/verify.sh` - all checks through Phase 121 passed; the run then stopped at the Phase 122 checker because it still requires the removed pre-write `record_compact_block_announcement` mutation

## Issues Encountered

- The Phase 122 deterministic checker still encodes the pre-write provenance behavior that this plan intentionally removes. Plan 04 owns checker updates and must migrate the Phase 122 guard plus the analogous expected Phase 126 guard before the phase-wide verifier can pass.

## Known Stubs

None.

## Threat Flags

None. The announcement transport and receipt trust boundaries were declared in the plan threat model; no new endpoint, authentication path, file-access pattern, or schema boundary was introduced.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Plan 03 can wire socket-write success to `PeerEmissionReceipt` completion without holding the network authority lock.
- Plan 04 must update stale Phase 122/126 text guards to assert post-write receipt completion, then run the complete repository verifier.

## Self-Check: PASSED

- Both task commits exist: `fcecddab`, `1f52b148`.
- Both newly created files exist and are registered in parity breadcrumbs.
- Config and generated LOC changes remain unstaged and are not part of either task commit.

*Phase: 128-production-compact-announcement-transport*
*Completed: 2026-07-20*
