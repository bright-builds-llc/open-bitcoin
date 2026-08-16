---
phase: 136-receive-independent-maintenance-and-transport-receipts
plan: "03"
subsystem: network
tags: [peer-emission, transaction-inventory, transaction-response, compact-evidence, affine-receipt]

# Dependency graph
requires:
  - phase: 134-authoritative-cross-cache-lifecycle-integration
    provides: Affine PeerEmission write capability and CompletePeerEmission receipt path
provides:
  - TransactionInventory and TransactionResponse PeerEmission write kinds
  - try_new_tx_inventory and try_new_tx_response constructors with no compact evidence reason
  - compact-only record_peer_emission announcement counters
affects: [phase-136-04, IBR-03, TransportWritten, D-01]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - compact PeerEmission::new still requires compact_announce_evidence_reason
    - TX constructors set maybe_evidence_reason to None and never share CompactInventoryFallback
    - record_peer_emission records compact counters only when maybe_evidence_reason is Some

key-files:
  created: []
  modified:
    - packages/open-bitcoin-node/src/network/announcement_transport.rs
    - packages/open-bitcoin-node/src/network/block_relay_evidence.rs
    - packages/open-bitcoin-node/src/network/tests/announcement_transport_cases.rs
    - scripts/check-phase122-compact-relay-peer-completion.ts
    - scripts/check-phase126-compact-relay-residual-hardening.ts
    - scripts/check-phase128-production-compact-announcement-transport.ts
    - docs/metrics/lines-of-code.md

key-decisions:
  - "TX INV and TX response are distinct write kinds; INV is announcement, not acknowledgement."
  - "PeerEmission::new stays compact-only and still rejects WireNetworkMessage::Tx."
  - "record_peer_emission increments compact counters only when maybe_evidence_reason is Some."
  - "Keep IBR-03 Pending until lifecycle-valid phase verification."

patterns-established:
  - "Affine TX write capabilities reuse acknowledge_write without compact evidence pollution."
  - "Optional evidence fields keep compact CompactBlock/Headers/INV constructors unchanged."

requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 136-2026-08-15T21-24-17
generated_at: 2026-08-16T01:17:15Z

# Metrics
duration: 71min
completed: 2026-08-16
---

# Phase 136 Plan 03: TX Write Kinds Without Compact Evidence Pollution Summary

**Distinct `TransactionInventory` and `TransactionResponse` PeerEmission constructors that reuse the affine receipt path without incrementing `compact_inventory_fallback_count`.**

## Performance

- **Duration:** 71 min
- **Started:** 2026-08-16T00:06:05Z
- **Completed:** 2026-08-16T01:17:15Z
- **Tasks:** 2
- **Files modified:** 9

## Accomplishments

- `PeerEmissionWriteKind` now includes `TransactionInventory` and `TransactionResponse` beside the compact `CompactBlock`, `Headers`, and `Inventory` kinds.
- `PeerEmission::new` remains the compact constructor and still returns `None` for `WireNetworkMessage::Tx`.
- `try_new_tx_inventory` accepts only `Inv` plus a `MempoolMemberIdentity`; `try_new_tx_response` accepts only `Tx`. Both set `maybe_evidence_reason = None`.
- `record_peer_emission` records compact announcement counters only when `maybe_evidence_reason` is `Some`, so TX INV cannot look like compact fallback.

## Task Commits

Each task was committed atomically:

1. **Task 1: Add TransactionInventory and TransactionResponse write kinds** - `73a61419` (feat)
2. **Task 2: Prove INV writes never clear and never increment compact fallback** - `13e90e2e` (test)

**Plan metadata:** pending docs commit

_Note: TDD RED commits were not created because `.githooks/pre-commit` runs `bash scripts/verify.sh`, which requires a green tree. Tests were written first, then production constructors and evidence gating were implemented in the Task 1 commit._

## Files Created/Modified

- `packages/open-bitcoin-node/src/network/announcement_transport.rs` - TX constructors, optional evidence fields, and receipt accessors
- `packages/open-bitcoin-node/src/network/block_relay_evidence.rs` - Compact-only `record_peer_emission` recording
- `packages/open-bitcoin-node/src/network/tests/announcement_transport_cases.rs` - Constructor and compact-fallback isolation tests
- `scripts/check-phase122-compact-relay-peer-completion.ts` - Optional evidence-accessor needles
- `scripts/check-phase122-compact-relay-peer-completion.test.ts` - Matching fixture and mutation updates
- `scripts/check-phase126-compact-relay-residual-hardening.ts` - Optional evidence-accessor needles
- `scripts/check-phase126-compact-relay-residual-hardening.test.ts` - Matching mutation update
- `scripts/check-phase128-production-compact-announcement-transport.ts` - Optional capability and evidence needles
- `docs/metrics/lines-of-code.md` - Hook-regenerated LOC report

## Decisions Made

- INV writes are announcement, not acknowledgement. `TransactionInventory` must never be treated as a TX-response receipt.
- Compact `PeerEmission::new` still requires `compact_announce_evidence_reason` and still rejects `WireNetworkMessage::Tx`.
- `record_peer_emission` uses `maybe_block_hash()` for header provenance and `maybe_evidence_reason()` before `record_announcement`.
- Keep IBR-03 Pending until Phase 136 has lifecycle-valid VERIFICATION.md.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Phase 122/126/128 checkers required compact-only evidence accessors**
- **Found during:** Task 1 commit (`verify.sh`)
- **Issue:** Structural checkers still required `evidence.block_hash()` and `evidence.evidence_reason()`.
- **Fix:** Update needles to `maybe_block_hash()` / `maybe_evidence_reason()` while still requiring evidence-derived `record_compact_block_announcement` and `record_announcement`.
- **Files modified:** `scripts/check-phase122-compact-relay-peer-completion.ts`, `scripts/check-phase122-compact-relay-peer-completion.test.ts`, `scripts/check-phase126-compact-relay-residual-hardening.ts`, `scripts/check-phase126-compact-relay-residual-hardening.test.ts`, `scripts/check-phase128-production-compact-announcement-transport.ts`
- **Verification:** Phase 122/126/128 checker tests and live checkers passed
- **Committed in:** `73a61419`

**2. [Rule 3 - Blocking] File-length gate after constructor additions**
- **Found during:** Task 1
- **Issue:** Inline tests would have pushed `announcement_transport.rs` over the 628-line production gate.
- **Fix:** Move constructor and outbox unit tests into `announcement_transport_cases.rs`.
- **Files modified:** `packages/open-bitcoin-node/src/network/announcement_transport.rs`, `packages/open-bitcoin-node/src/network/tests/announcement_transport_cases.rs`
- **Verification:** Production file-length check passed at 587 lines
- **Committed in:** `73a61419`

**3. [Rule 3 - Blocking] Unused TX constructors fail `-D dead-code` before Plan 04 wiring**
- **Found during:** Task 1 commit (`verify.sh` clippy)
- **Issue:** `try_new_tx_inventory` and `try_new_tx_response` are crate-visible but unused in the production lib until later plans.
- **Fix:** `#[cfg_attr(not(test), allow(dead_code))]` on those two constructors. `expect(dead_code)` was rejected by the panic-site checker.
- **Files modified:** `packages/open-bitcoin-node/src/network/announcement_transport.rs`
- **Verification:** Clippy and panic-site checks passed
- **Committed in:** `73a61419`

### Other Deviations

**4. TDD RED commits omitted**
- Pre-commit runs full `bash scripts/verify.sh`. Failing tests cannot be committed. Tests were written first, then constructors and evidence gating were implemented in the same feat commit.

**5. [Rule 3 - Blocking] Left IBR-03 Pending**
- Marking it Complete fails `check-active-milestone-verification-traceability` because Phase 136 has no lifecycle-valid VERIFICATION.md yet. Same pattern as Phase 136 Plans 01 and 02.

---

**Total deviations:** 3 auto-fixed (checkers, file length, dead-code lint) plus 2 process notes
**Impact on plan:** D-01 and IBR-03 constructor/evidence behavior match the plan. No TransportWritten application or GETDATA serving was added.

## Issues Encountered

- First Task 1 commit failed Phase 122 because `record_peer_emission` no longer called `evidence.block_hash()` / `evidence.evidence_reason()`.
- A later Task 1 commit failed clippy on unused TX constructors, then the panic-site checker on `expect(dead_code)`.
- Plan verify filter `tx_inventory_emission` matches the TX INV counter test; the compact regression was run afterward with `compact_inventory_emission`.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Plan 04 can apply `TransportWritten` only when `is_transaction_response()` is true.
- TX INV receipts exist as a distinct write kind and do not increment compact fallback counters.
- No GETDATA serving, timer, or fanout wiring was added.
- No blockers.

---
*Phase: 136-receive-independent-maintenance-and-transport-receipts*
*Completed: 2026-08-16*

## Self-Check: PASSED
