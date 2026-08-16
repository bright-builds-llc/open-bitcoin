---
phase: 136-receive-independent-maintenance-and-transport-receipts
plan: "04"
subsystem: network
tags: [getdata, prepared-tx-serve, transport-written, unbroadcast, peer-emission]

# Dependency graph
requires:
  - phase: 136-receive-independent-maintenance-and-transport-receipts
    provides: TransactionResponse write kind, unbroadcast insert-on-admission, and affine PeerEmission receipts
provides:
  - Durable GETDATA TX serves as PreparedTxServe(PeerEmission)
  - Applied-only MempoolRetryClearCause::TransportWritten on fresh TX-response writes
  - Classify-time EligibleServe leaves unbroadcast membership intact
affects: [phase-136-05, phase-136-06, IBR-04, D-01, D-02, D-03]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - Durable GETDATA TX encode stores PeerEmissionWriteCapability; successful write calls complete_peer_emission
    - TransportWritten is recorded beside unbroadcast_members.remove only in the Applied TX-response branch
    - last_transport_written_clear exposes the receipt-path cause for tests

key-files:
  created:
    - packages/open-bitcoin-rpc/src/context/inbound_wire.rs
    - packages/open-bitcoin-node/src/network/tests/getdata_tx_receipt_cases.rs
  modified:
    - packages/open-bitcoin-node/src/network/inventory.rs
    - packages/open-bitcoin-node/src/network/types.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/lifecycle.rs
    - packages/open-bitcoin-rpc/src/inbound_listener/connection_runtime.rs
    - packages/open-bitcoin-node/src/network/tests/relay_serving_cases.rs
    - docs/parity/source-breadcrumbs.json

key-decisions:
  - "Clear unbroadcast only on Applied current-epoch TX-response writes as TransportWritten, or LifecycleRemoval."
  - "EligibleServe classify and TX INV writes must not clear membership."
  - "Keep IBR-04 Pending until lifecycle-valid phase verification."

patterns-established:
  - "PreparedTxServe is the durable GETDATA TX plan item; Immediate(Tx) is no longer the durable path."
  - "Stale, AlreadyApplied, aborted, and encode-failed TX writes leave retry eligibility intact."

requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 136-2026-08-15T21-24-17
generated_at: 2026-08-16T02:35:30Z

# Metrics
duration: 55min
completed: 2026-08-16
---

# Phase 136 Plan 04: GETDATA TX Receipts and TransportWritten Summary

**Eligible GETDATA TX serves become receipt-bearing `PreparedTxServe` writes, and unbroadcast clears only after a fresh Applied `TransactionResponse` as `MempoolRetryClearCause::TransportWritten`.**

## Performance

- **Duration:** 55 min
- **Started:** 2026-08-16T01:40:33Z
- **Completed:** 2026-08-16T02:35:30Z
- **Tasks:** 2
- **Files modified:** 22

## Accomplishments

- Durable GETDATA TX classify now prepares a `PeerEmission` via `try_new_tx_response` and stores `PreparedTxServe` instead of `Immediate(Tx)`.
- Encode failure aborts the reserved capability. A successful TX write calls `complete_peer_emission`; a failed write calls `abort_peer_emission`.
- `complete_peer_effect` applies `TransportWritten` only when returning `EffectCompletion::Applied` for a fresh TX-response receipt with a member identity.
- Classify-time EligibleServe, TX INV writes, stale receipts, AlreadyApplied, and aborted writes leave unbroadcast membership intact.
- After a TransportWritten clear, a later child-package admission does not re-insert the parent (Plan 02 insert gate).

## Task Commits

Each task was committed atomically:

1. **Task 1: Promote eligible GETDATA TX to PreparedTxServe** - `cde332d8` (feat)
2. **Task 2: Apply TransportWritten only on fresh TX-response completion** - `99482dfb` (feat)

**Plan metadata:** pending docs commit

_Note: TDD RED commits were not created because `.githooks/pre-commit` runs `bash scripts/verify.sh`, which requires a green tree. Tests were written first, then production code landed in the same feat commit._

## Files Created/Modified

- `packages/open-bitcoin-node/src/network/types.rs` - `PreparedTxServe(Box<PeerEmission>)` plan item
- `packages/open-bitcoin-node/src/network/inventory.rs` - Durable GETDATA TX prepare path without pre-write clear
- `packages/open-bitcoin-node/src/network/announcement_transport.rs` - Production use of `try_new_tx_response`
- `packages/open-bitcoin-rpc/src/context/inbound_wire.rs` - Encoded wire resolve plus TX capability abort/ack
- `packages/open-bitcoin-rpc/src/context.rs` - `complete_peer_emission` / `abort_peer_emission` wrappers
- `packages/open-bitcoin-rpc/src/inbound_listener/connection_runtime.rs` - TX write complete/abort path
- `packages/open-bitcoin-node/src/network/runtime_authority/lifecycle.rs` - Applied-only TransportWritten clear
- `packages/open-bitcoin-node/src/network.rs` - `maybe_last_transport_written_clear` field
- `packages/open-bitcoin-node/src/network/lifecycle_projection/authority.rs` - Last-applied clear accessor and recovery reset
- `packages/open-bitcoin-node/src/network/tests/relay_serving_cases.rs` - Durable GETDATA prepare-without-clear test
- `packages/open-bitcoin-node/src/network/tests/getdata_tx_receipt_cases.rs` - Five named receipt-path tests (module unregistered until Plan 06)
- `docs/parity/source-breadcrumbs.json` - `inbound_wire.rs` plus `getdata_tx_receipt_cases.rs` mappings
- `scripts/check-phase123-runtime-timing-evidence-integrity/*` - EncodedWireResponse needle updates
- `scripts/check-phase127-authoritative-network-state-unification.ts` - Durable serving needles in `inbound_wire.rs`
- `docs/metrics/lines-of-code.md` - Hook-regenerated LOC report

## Decisions Made

- Clear unbroadcast only on a successful TX write as `TransportWritten`, or `LifecycleRemoval`. INV is announcement, not acknowledgement (D-01).
- `EligibleServe` must not clear membership before the write (D-03). Do not write `EligibleServe` into `retry_clears`.
- Record `MempoolRetryClearCause::TransportWritten` beside `unbroadcast_members.remove`, and expose the last receipt-path clear for tests.
- A TX write is first-hop delivery evidence only. Do not claim network-wide propagation.
- Keep IBR-04 Pending until Phase 136 has lifecycle-valid VERIFICATION.md.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Extract inbound wire resolve after file-length gate**
- **Found during:** Task 1
- **Issue:** Adding TX capability fields to `context.rs` exceeded the 628-line production gate.
- **Fix:** Move `EncodedWireResponse` resolve/ack into `context/inbound_wire.rs`.
- **Files modified:** `packages/open-bitcoin-rpc/src/context.rs`, `packages/open-bitcoin-rpc/src/context/inbound_wire.rs`, `packages/open-bitcoin-rpc/src/context/network.rs`
- **Verification:** Production file-length check passed
- **Committed in:** `cde332d8`

**2. [Rule 3 - Blocking] Phase 123/127 checkers after inbound_wire extract**
- **Found during:** Task 1 commit (`verify.sh`)
- **Issue:** Structural needles still expected `EncodedWireResponse` and durable serving symbols in `context.rs`. Concatenating `context.rs` with `inbound_wire.rs` double-counted because `readSourceCorpus("context.rs")` already includes `context/*`.
- **Fix:** Point Written-only ack and durable serving needles at `inbound_wire.rs` only.
- **Files modified:** `scripts/check-phase123-runtime-timing-evidence-integrity/{constants.ts,evidence.ts,*.test.ts}`, `scripts/check-phase127-authoritative-network-state-unification.ts`, `scripts/check-phase127-authoritative-network-state-unification.test.ts`
- **Verification:** Phase 123 and 127 checkers passed
- **Committed in:** `cde332d8`

**3. [Rule 3 - Blocking] Breadcrumb mapping for inbound_wire.rs**
- **Found during:** Task 1
- **Issue:** New first-party Rust file requires a parity breadcrumb mapping.
- **Fix:** Register `inbound_wire.rs` in the same RPC breadcrumb group as `context.rs`.
- **Files modified:** `docs/parity/source-breadcrumbs.json`
- **Verification:** `bun run scripts/check-parity-breadcrumbs.ts --check`
- **Committed in:** `cde332d8`

**4. [Rule 3 - Blocking] Breadcrumb mapping for getdata_tx_receipt_cases.rs**
- **Found during:** Task 2 commit (`verify.sh`)
- **Issue:** The checker scans every in-scope Rust file on disk. The new test file cannot exist without a mapping, even though Plan 06 owns the shared `node-initial-broadcast-retry` group and `tests.rs` registration.
- **Fix:** Add the file to the existing `node-unbroadcast-projection` group so verify can pass. Leave `mod getdata_tx_receipt_cases` out of `tests.rs`.
- **Files modified:** `docs/parity/source-breadcrumbs.json`, `packages/open-bitcoin-node/src/network/tests/getdata_tx_receipt_cases.rs`
- **Verification:** Breadcrumb check passed; the five named tests passed when temporarily registered, then the `mod` line was removed before commit
- **Committed in:** `99482dfb`

### Other Deviations

**5. TDD RED commits omitted**
- Pre-commit runs full `bash scripts/verify.sh`. Failing tests cannot be committed. Tests were written first, then production code landed in the same feat commit.

**6. [Rule 3 - Blocking] Left IBR-04 Pending**
- Marking it Complete fails `check-active-milestone-verification-traceability` because Phase 136 has no lifecycle-valid VERIFICATION.md yet. Same pattern as Plans 01–03.

***

**Total deviations:** 4 auto-fixed (file length, checkers, two breadcrumb mappings) plus 2 process notes
**Impact on plan:** GETDATA receipt and TransportWritten behavior match the locked decisions. Plan 06 still owns `mod getdata_tx_receipt_cases` and may relocate the breadcrumb into `node-initial-broadcast-retry`.

## Issues Encountered

- Task 1 required extracting inbound wire resolve so `context.rs` stayed under the production file-length gate.
- Task 2 could not commit `getdata_tx_receipt_cases.rs` until a breadcrumb mapping existed, despite the plan deferring that JSON edit to Plan 06.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Plan 05 can add package fanout without treating INV as acknowledgement.
- Plan 06 can register `mod getdata_tx_receipt_cases` and re-run the `getdata_tx_receipt` filter; the five named tests already exist.
- IBR-04 remains Pending until phase verification.
- No blockers.

***
*Phase: 136-receive-independent-maintenance-and-transport-receipts*
*Completed: 2026-08-16*

## Self-Check: PASSED
