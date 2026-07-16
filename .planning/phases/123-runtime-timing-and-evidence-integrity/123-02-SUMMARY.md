---
phase: 123-runtime-timing-and-evidence-integrity
plan: 02
subsystem: network-runtime-evidence
tags: [rust, block-serving, runtime-evidence, schema-integrity]

requires:
  - phase: 116-operator-evidence-metrics-logs-and-support-boundary
    provides: Public block-relay evidence status contract kept unchanged by this plan
provides:
  - Private successful block-wire-write counter owned by ManagedBlockRelayEvidenceState
  - Typed post-write acknowledgement on ManagedPeerNetwork
  - Crate-only non-serialized runtime evidence snapshot for later projection wiring
affects: [123-03, 123-04, 123-05, block-relay-metrics, inbound-block-serving]

tech-stack:
  added: []
  patterns:
    - Typed achieved-effect acknowledgement after successful wire writes
    - Separate runtime-only evidence from serialized operator status

key-files:
  created: []
  modified:
    - packages/open-bitcoin-node/src/network.rs
    - packages/open-bitcoin-node/src/network/block_relay_evidence.rs
    - packages/open-bitcoin-node/src/network/tests.rs

key-decisions:
  - "Keep successful block-write evidence private and aggregate; do not expand BlockRelayEvidenceStatus or operator schemas."
  - "Route existing status and count accessors through the runtime snapshot so the isolated plan remains warning-free before later runtime consumers are merged."

patterns-established:
  - "Achieved-effect evidence: only typed WireNetworkMessage::Block acknowledgement advances served_count."
  - "Schema isolation: the crate-only snapshot carries served_count alongside, not inside, serialized status."

requirements-completed:
  - HARD-03
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 123-2026-07-15T18-12-00
generated_at: 2026-07-16T01:50:26Z

duration: 6 min
completed: 2026-07-16
---

# Phase 123 Plan 02: Private Served-Write Evidence Summary

**A typed post-write acknowledgement now owns a private successful-block counter and non-serialized runtime snapshot while public RPC/CLI/dashboard/support schemas remain unchanged.**

## Performance

- **Duration:** 6 min
- **Started:** 2026-07-16T01:44:25Z
- **Completed:** 2026-07-16T01:50:26Z
- **Tasks:** 1
- **Files modified:** 3

## Accomplishments

- Added one authoritative `served_count` to private managed-network evidence.
- Added a typed `WireNetworkMessage` acknowledgement that records only successful block writes and ignores all non-block messages.
- Added a crate-only snapshot carrying unchanged public status plus the private count, with regression coverage proving `served_count` never appears in serialized status.

## Task Commits

TDD work was committed atomically:

1. **RED: Add failing served-write evidence tests** - `1e4d44b2` (test)
2. **GREEN: Add private served-write evidence** - `03af8a5d` (feat)

## Files Created/Modified

- `packages/open-bitcoin-node/src/network.rs` - Exposes the runtime snapshot only to sibling crate modules.
- `packages/open-bitcoin-node/src/network/block_relay_evidence.rs` - Owns the private counter, snapshot, acknowledgement, and accessors.
- `packages/open-bitcoin-node/src/network/tests.rs` - Proves block/non-block acknowledgement behavior and unchanged serialized status.

## Decisions Made

- Preserved all serialized block-serving and block-relay status structs exactly; the successful-write count exists only in runtime evidence.
- Used the existing counter arithmetic convention (`+= 1`) and an early return for non-block messages.
- Reused the runtime snapshot in the existing public-status and count accessors so no temporary dead-code or unused-import suppression is required.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Made the planned future-facing snapshot live on the isolated branch**

- **Found during:** Task 1 focused Clippy verification
- **Issue:** Plan 05 is the first later runtime consumer, so the new crate-only snapshot and re-export were dead code in this isolated Plan 02 branch under `-D warnings`.
- **Fix:** Routed the existing `block_relay_evidence_status()` and `block_served_write_count()` accessors through the same snapshot instead of adding lint suppressions.
- **Files modified:** `packages/open-bitcoin-node/src/network/block_relay_evidence.rs`
- **Verification:** Focused Clippy with all node targets/features and warnings denied passes; both required timing-wrapped test commands remain green.
- **Committed in:** `03af8a5d`

**Total deviations:** 1 auto-fixed (1 blocking). **Impact:** No scope expansion or public behavior change; the snapshot became the single internal carrier immediately instead of waiting for Plan 05.

## Issues Encountered

- The initial focused Clippy run identified the expected intermediate-wave dead-code condition; it was resolved through reuse rather than lint suppression.

## User Setup Required

None - no external service configuration required.

## Verification

- RED proof: timing-wrapped `phase123_` tests failed with missing acknowledgement/count APIs before implementation.
- `bun run scripts/command-timings.ts run --key phase123-private-served-counter-tests -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase123_ -- --nocapture` - 3 passed.
- `bun run scripts/command-timings.ts run --key phase123-public-status-schema-test -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase123_public_block_relay_status_omits_runtime_served_count -- --nocapture` - 1 passed.
- Timing-wrapped `cargo clippy -p open-bitcoin-node --all-targets --all-features -- -D warnings` - passed.
- All acceptance searches and `git diff --check` - passed.

## Next Phase Readiness

- Plans 123-03 and 123-04 can wire the single acknowledgement into their successful transport-write seams.
- Plan 123-05 can consume `BlockRelayRuntimeEvidenceSnapshot` without changing public status schemas.
- No blockers remain for dependent plans.

## Self-Check: PASSED

- All three modified key files exist.
- Both `123-02` task commits are present in git history.
- Required tests, acceptance searches, diff checks, and focused Clippy passed.
- `.planning/STATE.md` and `.planning/ROADMAP.md` were not modified.

***

*Phase: 123-runtime-timing-and-evidence-integrity*
*Completed: 2026-07-16*
