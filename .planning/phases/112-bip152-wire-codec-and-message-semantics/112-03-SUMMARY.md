---
phase: 112-bip152-wire-codec-and-message-semantics
plan: 03
subsystem: codec-network
tags: [rust, bip152, getblocktxn, blocktxn, p2p-wire, parity]

requires:
  - phase: 112-bip152-wire-codec-and-message-semantics
    provides: BIP152 sendcmpct and cmpctblock payload support from Plans 112-01 and 112-02
  - phase: 111-full-block-serving-request-path
    provides: Existing block inventory and peer message handling boundaries
provides:
  - BIP152 getblocktxn payload codec with checked differential transaction-index expansion
  - BIP152 blocktxn payload codec using witness transaction serialization
  - Explicit WireNetworkMessage branches for getblocktxn and blocktxn
  - Malformed BIP152 payload regression matrix at codec and network-message boundaries
affects: [phase-113-compact-relay-negotiation, phase-114-compact-reconstruction, phase-115-compact-missing-transactions]

tech-stack:
  added: []
  patterns:
    - Pure BIP152 request/response payload parsing remains in open-bitcoin-codec
    - Network message decoding delegates BIP152 payload bytes to typed codec helpers
    - Peer policy remains no-op for decoded compact relay payload messages until later phases

key-files:
  created:
    - .planning/phases/112-bip152-wire-codec-and-message-semantics/112-03-SUMMARY.md
    - packages/open-bitcoin-codec/src/compact_block/tests.rs
    - packages/open-bitcoin-network/src/message/cursor.rs
  modified:
    - docs/metrics/lines-of-code.md
    - docs/parity/source-breadcrumbs.json
    - packages/open-bitcoin-codec/src/compact_block.rs
    - packages/open-bitcoin-codec/src/lib.rs
    - packages/open-bitcoin-network/src/message.rs
    - packages/open-bitcoin-network/src/message/tests.rs
    - packages/open-bitcoin-network/src/peer.rs
    - packages/open-bitcoin-network/src/peer/tests.rs

key-decisions:
  - "getblocktxn preserves wire differential indexes as Vec<u64> and exposes checked u16 absolute-index expansion."
  - "blocktxn serializes transactions with TransactionEncoding::WithWitness to match BIP152 witness-aware block transaction responses."
  - "Malformed BIP152 payloads are rejected at codec/message decode boundaries before peer policy, reconstruction, fallback, or mempool state."
  - "No git commits were created because the wrapper reserves final commit ownership for the orchestrator."

patterns-established:
  - "BIP152 request/response codecs validate byte-level structure before exposing typed messages."
  - "Large inline protocol test modules should live in sibling tests.rs files to satisfy production file-length gates."

requirements-completed: [CMP-03, RCN-01]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 112-2026-07-04T19-37-55
generated_at: 2026-07-04T20:52:06Z

duration: 46m
completed: 2026-07-04
---

# Phase 112 Plan 03: Getblocktxn and Blocktxn Wire Semantics Summary

**BIP152 getblocktxn and blocktxn now round-trip typed payloads through codec and network message layers, with malformed byte-level regressions pinned before compact reconstruction or peer policy can run.**

## Performance

- **Duration:** 46m
- **Started:** 2026-07-04T20:06:00Z
- **Completed:** 2026-07-04T20:52:06Z
- **Tasks:** 3 completed
- **Files modified:** 12 including this summary

## Accomplishments

- Added `BlockTransactionsRequest` and `BlockTransactions` payload contracts plus encode/decode helpers for `getblocktxn` and `blocktxn`.
- Added checked `expand_block_transaction_indexes` behavior so differential request indexes can round-trip as wire deltas while rejecting values above `u16`.
- Wired `WireNetworkMessage::GetBlockTxn` and `WireNetworkMessage::BlockTxn` to `getblocktxn` and `blocktxn` commands with explicit encode/decode delegation.
- Added malformed BIP152 regression tests for `cmpctblock`, `getblocktxn`, and `blocktxn`, covering EOF, trailing data, non-canonical counts, overflow, null transactions, and superfluous witness records.
- Preserved plan scope: decoded BIP152 messages are accepted as no-op peer messages only, with reconstruction, fallback, mempool lookup, misbehavior, disconnect, and relay policy left to later phases.

## Task Changes

No commits were created. The wrapper instructed this executor to leave all changes uncommitted for orchestrator finalization.

1. **Task 1: Implement `getblocktxn` and `blocktxn` codecs**
   - Added RED tests for multi-index `getblocktxn` round trips, empty index vectors, `u16` overflow rejection, witness `blocktxn` round trips, and empty transaction vectors.
   - Implemented the request/response structs, typed payload encode/decode helpers, witness transaction serialization, and checked index expansion.
   - Re-exported the new codec types and helpers from `open-bitcoin-codec`.

2. **Task 2: Wire `getblocktxn` and `blocktxn` into `WireNetworkMessage`**
   - Added RED network tests for payload/wire round trips and an explicit BIP152 command surface guard.
   - Added `WireNetworkMessage::GetBlockTxn(BlockTransactionsRequest)` and `WireNetworkMessage::BlockTxn(BlockTransactions)` with command, encode, and decode mappings.
   - Updated peer handling so decoded `sendcmpct`, `cmpctblock`, `getblocktxn`, and `blocktxn` stay policy-free no-ops.

3. **Task 3: Add malformed BIP152 payload regression matrix**
   - Added codec matrix coverage for malformed `cmpctblock`, `getblocktxn`, and `blocktxn` payloads.
   - Added a network-message regression test proving representative malformed BIP152 payloads surface stable errors through `WireNetworkMessage::decode_payload`.
   - Added focused coverage for compact-block count overflow and the new peer no-op arms so repo coverage gates remain satisfied.

## Validation Evidence

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-codec compact_block -- --nocapture` passed with 16 codec tests before the test-module split and the affected focused tests passed after the split.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network message -- --nocapture` passed with 33 message-filtered tests plus generated wire-message property tests.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network message::tests::phase112_wire_getblocktxn -- --nocapture` passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network message::tests::phase112_wire_blocktxn -- --nocapture` passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-codec phase112_malformed -- --nocapture` passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network phase112_message_decode_surfaces_malformed_bip152_errors -- --nocapture` passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-codec phase112_cmpctblock_structure_rejects_implied_count_overflow -- --nocapture` passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network peer::tests::phase112_bip152_wire_messages_are_peer_noops -- --nocapture` passed.
- `cargo fmt --manifest-path packages/Cargo.toml --all -- --check` passed after preserving the literal `getblocktxn` decode acceptance anchor with a local rustfmt skip.
- `cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-codec -p open-bitcoin-network --all-targets --all-features -- -D warnings` passed.
- `bun run scripts/check-parity-breadcrumbs.ts --check` passed for 350 Rust files.
- `bash scripts/verify.sh` passed after regenerating `docs/metrics/lines-of-code.md`; final output: `verify.sh completed in 9m 27.685s (567685ms)`.
- Plan acceptance probes passed for required structs, helper names, witness serialization, command mappings, malformed test names, error-string fragments, and forbidden reconstruction/runtime-scope terms.

## Files Created/Modified

- `packages/open-bitcoin-codec/src/compact_block.rs` - Holds the BIP152 production payload contracts and delegates tests to a sibling module.
- `packages/open-bitcoin-codec/src/compact_block/tests.rs` - Contains sendcmpct, cmpctblock, getblocktxn, blocktxn, malformed-payload, and coverage regression tests.
- `packages/open-bitcoin-codec/src/lib.rs` - Re-exports the new BIP152 request/response types and helpers.
- `packages/open-bitcoin-network/src/message.rs` - Adds explicit `getblocktxn` and `blocktxn` wire message variants and delegates cursor helpers to a child module.
- `packages/open-bitcoin-network/src/message/cursor.rs` - Extracts private message cursor parsing helpers to keep the root message module below the line-count gate.
- `packages/open-bitcoin-network/src/message/tests.rs` - Adds BIP152 request/response wire round-trip and malformed-payload network tests.
- `packages/open-bitcoin-network/src/peer.rs` - Adds no-op handling for decoded BIP152 compact relay payload messages.
- `packages/open-bitcoin-network/src/peer/tests.rs` - Covers BIP152 peer no-op handling and updates deferred command expectations.
- `docs/parity/source-breadcrumbs.json` - Adds parity breadcrumb entries for the new sibling Rust files.
- `docs/metrics/lines-of-code.md` - Regenerated tracked LOC report required by `scripts/verify.sh`.
- `.planning/phases/112-bip152-wire-codec-and-message-semantics/112-03-SUMMARY.md` - Records this plan's uncommitted execution outcome.

## Decisions Made

- `getblocktxn` stores differential indexes exactly as wire deltas and uses `expand_block_transaction_indexes` for checked absolute `u16` positions.
- `blocktxn` uses witness transaction serialization for every transaction payload, preserving BIP152 response semantics.
- Network decoding now recognizes `getblocktxn` and `blocktxn`, but peer policy remains intentionally inert until compact relay negotiation and reconstruction phases define behavior.
- The codec and message root modules were split only to satisfy verifier-enforced production file-length boundaries; public behavior remains unchanged.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Added exhaustive peer handling for new BIP152 message variants**

- **Found during:** Task 2
- **Issue:** Adding `GetBlockTxn` and `BlockTxn` enum variants required `PeerManager::handle_message` to remain exhaustive.
- **Fix:** Added policy-free no-op handling for `GetBlockTxn` and `BlockTxn`, matching the existing `sendcmpct` and `cmpctblock` scope boundary.
- **Files modified:** `packages/open-bitcoin-network/src/peer.rs`, `packages/open-bitcoin-network/src/peer/tests.rs`
- **Verification:** Focused peer no-op test and full `bash scripts/verify.sh`.
- **Committed in:** Not committed per wrapper no-commit override.

**2. [Rule 1 - Bug] Updated stale deferred-command guard**

- **Found during:** Task 2 broader peer/message verification
- **Issue:** The deferred-command guard still listed `getblocktxn` and `blocktxn` as absent from the peer message surface even though this plan makes them explicit messages.
- **Fix:** Replaced those entries with still-deferred commands while preserving the guard's purpose.
- **Files modified:** `packages/open-bitcoin-network/src/peer/tests.rs`
- **Verification:** `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network message -- --nocapture`.
- **Committed in:** Not committed per wrapper no-commit override.

**3. [Rule 3 - Blocking] Split oversized production Rust files**

- **Found during:** Full `bash scripts/verify.sh`
- **Issue:** Inline tests pushed `compact_block.rs` over the production Rust line-count gate, and `message.rs` remained slightly over the same gate.
- **Fix:** Moved compact-block tests into `compact_block/tests.rs` and moved private message cursor helpers into `message/cursor.rs`.
- **Files modified:** `packages/open-bitcoin-codec/src/compact_block.rs`, `packages/open-bitcoin-codec/src/compact_block/tests.rs`, `packages/open-bitcoin-network/src/message.rs`, `packages/open-bitcoin-network/src/message/cursor.rs`, `docs/parity/source-breadcrumbs.json`
- **Verification:** Production Rust file-length check passed inside final `bash scripts/verify.sh`.
- **Committed in:** Not committed per wrapper no-commit override.

**4. [Rule 3 - Blocking] Refreshed generated LOC report**

- **Found during:** Full `bash scripts/verify.sh`
- **Issue:** `docs/metrics/lines-of-code.md` was stale after Rust source/test changes.
- **Fix:** Ran `bun run scripts/generate-loc-report.ts --source=worktree --output=docs/metrics/lines-of-code.md`.
- **Files modified:** `docs/metrics/lines-of-code.md`
- **Verification:** Final `bash scripts/verify.sh` reported `LOC report is current`.
- **Committed in:** Not committed per wrapper no-commit override.

**5. [Rule 2 - Missing critical test coverage] Added coverage for newly exposed validation/no-op branches**

- **Found during:** Full `bash scripts/verify.sh` llvm-cov stage
- **Issue:** The compact-block implied count overflow path and peer no-op BIP152 arms were uncovered.
- **Fix:** Added focused tests for `validate_compact_block_structure` count overflow and `PeerManager::handle_message` no-op handling of BIP152 compact relay payload messages.
- **Files modified:** `packages/open-bitcoin-codec/src/compact_block/tests.rs`, `packages/open-bitcoin-network/src/peer/tests.rs`
- **Verification:** Focused coverage tests passed and final `bash scripts/verify.sh` passed.
- **Committed in:** Not committed per wrapper no-commit override.

**Total deviations:** 5 auto-fixed issues.
**Impact on plan:** All deviations were required to keep compilation, existing guard tests, generated artifacts, line-count policy, and coverage gates aligned with the planned BIP152 wire semantics. No compact relay negotiation, reconstruction state, fallback behavior, mempool lookup, validation/connect handoff, misbehavior scoring, disconnect policy, RPC, CLI, dashboard, support bundle, or public-default behavior was introduced.

## Issues Encountered

- The TDD RED checks failed as expected for missing codec structs/functions and missing network variants before implementation.
- `cargo fmt` wrapped the literal `getblocktxn` decode arm required by the plan's acceptance probe; a local `#[rustfmt::skip]` was applied to that single match arm so formatting still passes and the acceptance anchor remains in production code.
- The first full verifier run failed on stale LOC, the second on production file length, and the third on coverage gaps. Each verifier failure was fixed directly and the final verifier passed.

## Known Stubs

None found.

## Threat Flags

None. The new decoded P2P payload surfaces are the planned trust boundary for this phase, and peer/runtime behavior remains no-op until later compact relay phases.

## User Setup Required

None.

## Next Phase Readiness

- Later compact relay negotiation and reconstruction phases can now rely on typed `getblocktxn` and `blocktxn` payloads plus stable malformed-payload errors.
- The BIP152 codec tests live in a sibling module, and the message cursor helper is isolated, so future additions should keep production root files under verifier line-count limits.

## Self-Check: PASSED

- Summary file created at `.planning/phases/112-bip152-wire-codec-and-message-semantics/112-03-SUMMARY.md`.
- No git commits were created.
- `STATE.md` and `ROADMAP.md` were not updated by this executor.
- Final focused and full verification passed.

*Phase: 112-bip152-wire-codec-and-message-semantics*
*Completed: 2026-07-04*
