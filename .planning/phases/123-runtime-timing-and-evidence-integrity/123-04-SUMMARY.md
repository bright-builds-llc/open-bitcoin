---
phase: 123-runtime-timing-and-evidence-integrity
plan: 04
subsystem: rpc-inbound-transport
tags: [rust, inbound-listener, block-serving, achieved-effect-evidence, HARD-03]

requires:
  - phase: 123-runtime-timing-and-evidence-integrity
    plan: 02
    provides: private successful block-write counter and typed acknowledgement
provides:
  - crate-private typed carrier pairing each inbound response with its encoded bytes
  - Written-only inbound block acknowledgement at the socket effect boundary
  - deterministic success, rejection, error, non-block, partial-batch, and encoding guards
affects: [123-05, 123-07, inbound-block-serving, block-relay-metrics]

tech-stack:
  added: []
  patterns:
    - Encode a complete response batch before pairing typed messages with bytes
    - Borrow write outcomes so acknowledgement precedes owned rejection and termination handling

key-files:
  created: []
  modified:
    - packages/open-bitcoin-network/src/message.rs
    - packages/open-bitcoin-rpc/src/context.rs
    - packages/open-bitcoin-rpc/src/context/network.rs
    - packages/open-bitcoin-rpc/src/inbound_listener.rs
    - packages/open-bitcoin-rpc/src/inbound_listener/tests.rs

key-decisions:
  - "Keep EncodedWireResponse crate-private and non-serialized; typed identity is ephemeral transport state."
  - "Acknowledge only a borrowed Ok(WriteWireMessageOutcome::Written) before consuming the owned write result."
  - "Apply the existing MAX_INV_SIZE invariant to outbound inventory encoding so failed batches cannot reach the listener."

patterns-established:
  - "Achieved inbound effect: encode, write, then acknowledge the retained typed message."
  - "Partial batches preserve each successful Block prefix before a later write failure terminates the loop."

requirements-completed:
  - HARD-03
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 123-2026-07-15T18-12-00
generated_at: 2026-07-16T02:25:00Z

duration: 37 min
completed: 2026-07-16
---

# Phase 123 Plan 04: Inbound Written-Block Evidence Summary

**Inbound responses now retain typed message identity through encoding and count served blocks only after the listener completes the corresponding socket write.**

## Performance

- **Duration:** 37 min
- **Started:** 2026-07-16T01:48:00Z
- **Completed:** 2026-07-16T02:25:00Z
- **Tasks:** 1 TDD transport migration
- **Files modified:** 5

## Accomplishments

- Added a two-field, crate-private `EncodedWireResponse` that retains the original `WireNetworkMessage` beside fully encoded bytes without entering any public or serialized schema.
- Added one borrowed async bridge that advances Plan 02 evidence only for `Ok(WriteWireMessageOutcome::Written)` and leaves rejection/error ownership unchanged.
- Proved exact counts for written, rejected, failed, non-block, and partial-batch outcomes, including one and two successful Block prefixes before later failures.
- Rejected oversized outbound inventory before carrier creation, proving failed encoding never reaches the listener write loop or served evidence.

## Task Commits

1. **RED: Add failing inbound write evidence coverage** — `6ed59cc1`
2. **GREEN: Acknowledge written inbound blocks** — `a659b27c`

## Verification Results

```text
Phase 123 RPC focused tests: 7 passed, 0 failed
Inbound-listener regression filter: 38 library + 1 daemon passed, 0 failed
Network message regression filter: 35 library + 2 property tests passed, 0 failed
Affected network/RPC Clippy --all-targets --all-features -D warnings: passed
All acceptance searches and git diff --check: passed
```

The orchestrator owns the merged-wave `bash scripts/verify.sh` contract, including final breadcrumb and Bazel gates.

## Decisions Made

- Encoding completes for the whole response batch before messages and bytes are zipped, so a failed encode cannot produce a partially sendable carrier batch.
- The listener calls acknowledgement immediately after `write_all_for_state` and before matching the owned result, avoiding clones while preserving existing rejection evidence and termination behavior.
- The runtime counter remains test-visible only through a crate-private `ManagedRpcContext` accessor; no RPC, CLI, dashboard, or support surface changed.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Enforced the planned outbound inventory encoding limit**

- **Found during:** Mandatory read-first review before Task 1 RED
- **Issue:** The plan required `InventoryList` with `MAX_INV_SIZE + 1` to fail in `encode_wire_responses`, but `encode_inventory_payload` accepted unbounded counts while only decoding enforced `MAX_INV_SIZE`.
- **Fix:** Extracted one private `validate_inventory_count` helper and reused it for both encode and decode, with no public API, dependency, or schema change.
- **Files modified:** `packages/open-bitcoin-network/src/message.rs`
- **Verification:** The dedicated RPC failed-encoding guard passes; existing network message and property regressions pass; affected Clippy passes with warnings denied.
- **Committed in:** `a659b27c`

**Total deviations:** 1 auto-fixed (1 blocking). **Impact:** The minimal fix makes the plan's exact deterministic guard truthful and closes an outbound protocol-bound violation without expanding scope.

## Issues Encountered

- Package-wide filtered Cargo runs were quiet between test binaries on the loaded host. Process liveness was checked and the resumable sessions were polled without terminating healthy commands; all runs completed successfully.

## User Setup Required

None - no external service configuration required.

## Residual Risks

- Full repository verification and Phase 123 parity/checker integration remain owned by the orchestrator and Plan 123-07.
- The carrier is intentionally ephemeral and crate-private; any future transport must explicitly preserve typed identity to its own successful write boundary.

## Next Phase Readiness

- The inbound half of HARD-03 is complete and ready for Plan 123-05's authoritative metric/log projection.
- Plan 123-07 can statically verify the exact Written-only ordering and existing breadcrumb coverage.

## Self-Check: PASSED

- FOUND: `packages/open-bitcoin-rpc/src/context.rs` (`EncodedWireResponse`)
- FOUND: `packages/open-bitcoin-rpc/src/context/network.rs` (`encode_wire_responses` and acknowledgement delegation)
- FOUND: `packages/open-bitcoin-rpc/src/inbound_listener.rs` (Written-only borrowed bridge before owned match)
- FOUND: `packages/open-bitcoin-rpc/src/inbound_listener/tests.rs` (all seven exact Phase 123 tests)
- FOUND: `.planning/phases/123-runtime-timing-and-evidence-integrity/123-04-SUMMARY.md`
- FOUND COMMITS: `6ed59cc1`, `a659b27c`

***

*Phase: 123-runtime-timing-and-evidence-integrity*
*Completed: 2026-07-16*
