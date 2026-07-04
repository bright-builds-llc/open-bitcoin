---
phase: 112-bip152-wire-codec-and-message-semantics
plan: 02
subsystem: codec-network
tags: [rust, bip152, cmpctblock, p2p-wire, parity]

requires:
  - phase: 112-bip152-wire-codec-and-message-semantics
    provides: BIP152 sendcmpct payload support and WireNetworkMessage mapping
  - phase: 111-full-block-serving-request-path
    provides: Compact-block inventory remains classified but not served from getdata
provides:
  - BIP152 cmpctblock payload codec with exact six-byte short IDs
  - Prefilled witness transaction serialization and checked differential index expansion
  - Stable compact-block structural CodecError variants
  - WireNetworkMessage CompactBlock branch mapped to command cmpctblock
affects: [phase-112-plan-03, phase-113-compact-relay-negotiation, phase-114-compact-reconstruction]

tech-stack:
  added: []
  patterns:
    - Pure BIP152 payload parsing and structural validation in open-bitcoin-codec
    - Network message enum delegates cmpctblock payload bytes to codec helpers

key-files:
  created:
    - .planning/phases/112-bip152-wire-codec-and-message-semantics/112-02-SUMMARY.md
  modified:
    - packages/open-bitcoin-codec/src/compact_block.rs
    - packages/open-bitcoin-codec/src/error.rs
    - packages/open-bitcoin-codec/src/lib.rs
    - packages/open-bitcoin-network/src/message.rs
    - packages/open-bitcoin-network/src/message/tests.rs
    - packages/open-bitcoin-network/src/peer.rs
    - packages/open-bitcoin-network/src/peer/tests.rs

key-decisions:
  - "Short IDs are represented as ShortId([u8; 6]) so eight-byte wire emission is unrepresentable."
  - "cmpctblock decode performs structural validation immediately after byte parsing and before any reconstruction state exists."
  - "Peer handling for decoded cmpctblock is intentionally a no-op until later compact relay policy phases."
  - "No git commits were created because the wrapper reserves final commit ownership for the orchestrator."

patterns-established:
  - "BIP152 payload structs preserve wire deltas while exposing checked absolute-position helpers."
  - "Compact-block inventory remains separate from the cmpctblock message command."

requirements-completed: [CMP-02, RCN-01]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 112-2026-07-04T19-37-55
generated_at: 2026-07-04T20:05:48Z

duration: 6m 54s
completed: 2026-07-04
---

# Phase 112 Plan 02: Cmpctblock Wire Codec Summary

**BIP152 cmpctblock byte-level support with exact six-byte short IDs, witness prefilled transactions, checked differential indexes, structural rejection, and explicit network command mapping.**

## Performance

- **Duration:** 6m 54s
- **Started:** 2026-07-04T19:58:54Z
- **Completed:** 2026-07-04T20:05:48Z
- **Tasks:** 2 completed
- **Files modified:** 8 including this summary

## Accomplishments

- Extended `open-bitcoin-codec::compact_block` with `ShortId`, `PrefilledTransaction`, `CompactBlockPayload`, `encode_compact_block_payload`, `decode_compact_block_payload`, `validate_compact_block_structure`, and checked differential index helpers.
- Added stable compact-block structural errors for differential overflow, empty compact blocks, transaction-count overflow, out-of-bounds prefilled positions, and structurally null prefilled transactions.
- Wired `WireNetworkMessage::CompactBlock(CompactBlockPayload)` to command `cmpctblock` while keeping compact-block inventory in `getdata`.
- Added focused Arrange/Act/Assert tests for byte-preserving witness round trips, exact six-byte short IDs, malformed compact-block structural errors, wire-message round trips, and malformed network payload rejection.

## Task Changes

No commits were created. The wrapper instructed this executor to leave all changes uncommitted for orchestrator finalization.

1. **Task 1: Implement `cmpctblock` payload codec and structural validation**
   - Added RED codec tests for short-ID width, witness prefilled transaction round trips, empty compact blocks, differential overflow, and out-of-bounds prefilled positions.
   - Implemented the typed payload codec, exact six-byte short IDs, witness transaction serialization, checked differential expansion, and compact-block structural validation.
   - Added stable `CodecError` variants and display-message coverage.

2. **Task 2: Wire `cmpctblock` into `WireNetworkMessage`**
   - Added RED network tests for `cmpctblock` payload/wire round trips, malformed payload rejection before message construction, and compact-block inventory staying under `getdata`.
   - Added `WireNetworkMessage::CompactBlock(CompactBlockPayload)` with command, encode, and decode mappings.
   - Preserved runtime scope by making peer handling for decoded `cmpctblock` a no-op and keeping `getblocktxn` and `blocktxn` deferred.

## Validation Evidence

- RED codec check failed as expected before implementation: missing `CompactBlockPayload`, `PrefilledTransaction`, `ShortId`, compact-block codec helpers, and new error variants.
- RED network check failed as expected before implementation: missing `WireNetworkMessage::CompactBlock`.
- `cargo fmt --manifest-path packages/Cargo.toml --all` passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-codec compact_block -- --nocapture` passed with 8 codec tests.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-codec error::tests::display_messages_are_human_readable -- --nocapture` passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network message::tests::phase112_wire_cmpctblock -- --nocapture` passed with 2 focused network tests.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network message -- --nocapture` passed with 29 message-filtered tests plus generated wire-message property tests.
- `cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-codec -p open-bitcoin-network --all-targets --all-features -- -D warnings` passed.
- `bun run scripts/check-parity-breadcrumbs.ts --check` passed for 350 Rust files.
- Plan acceptance probes passed for required structs, error variants, `TransactionEncoding::WithWitness`, `cmpctblock` command mapping, required test names, and forbidden reconstruction/runtime-scope terms.
- IDE lint check reported no linter errors on touched Rust files.

## Files Created/Modified

- `packages/open-bitcoin-codec/src/compact_block.rs` - Extended the existing BIP152 module with `cmpctblock` payload contracts, codec helpers, structural validation, and tests.
- `packages/open-bitcoin-codec/src/error.rs` - Added stable compact-block structural error variants and human-readable messages.
- `packages/open-bitcoin-codec/src/lib.rs` - Re-exported compact-block payload types and helpers.
- `packages/open-bitcoin-network/src/message.rs` - Added explicit `WireNetworkMessage::CompactBlock` command, encode, and decode mapping.
- `packages/open-bitcoin-network/src/message/tests.rs` - Added `cmpctblock` network round-trip, malformed payload, and inventory separation tests.
- `packages/open-bitcoin-network/src/peer.rs` - Added no-op handling for decoded `cmpctblock` to preserve exhaustiveness without adding peer policy.
- `packages/open-bitcoin-network/src/peer/tests.rs` - Updated the deferred-command guard so `getblocktxn` and `blocktxn` remain unknown while `cmpctblock` is no longer treated as deferred.
- `.planning/phases/112-bip152-wire-codec-and-message-semantics/112-02-SUMMARY.md` - Records this plan's uncommitted execution outcome.

## Decisions Made

- `ShortId([u8; 6])` is the only public short-ID representation for this payload, keeping the six-byte wire invariant at the type boundary.
- `CompactBlockPayload` preserves prefilled transaction wire deltas and exposes checked expansion helpers instead of normalizing away byte-level semantics.
- `validate_compact_block_structure` rejects malformed compact blocks before any later reconstruction, mempool lookup, fallback, or validation/connect handoff can exist.
- `cmpctblock` is now a decoded network command, but peer handling remains no-op and compact-block inventory still travels through `getdata`.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Added exhaustive peer handling for the new message variant**

- **Found during:** Task 2 (Wire `cmpctblock` into `WireNetworkMessage`)
- **Issue:** Adding `WireNetworkMessage::CompactBlock` required `PeerManager::handle_message` to handle the new enum variant.
- **Fix:** Added `WireNetworkMessage::CompactBlock(_) => Ok(Vec::new())` so decoded compact blocks have no peer-state consequences until Phase 113 and later policy phases.
- **Files modified:** `packages/open-bitcoin-network/src/peer.rs`
- **Verification:** `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network message -- --nocapture`
- **Committed in:** Not committed per wrapper no-commit override.

**2. [Rule 1 - Bug] Updated stale deferred-command guard**

- **Found during:** Task 2 broader message verification
- **Issue:** `deferred_relay_commands_remain_absent_from_peer_message_surface` still expected `cmpctblock` to decode as an unknown command.
- **Fix:** Replaced `cmpctblock` in the deferred-command list with `getblocktxn` and `blocktxn`, preserving the plan boundary for commands owned by Plan 112-03.
- **Files modified:** `packages/open-bitcoin-network/src/peer/tests.rs`
- **Verification:** `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network message -- --nocapture`
- **Committed in:** Not committed per wrapper no-commit override.

**Total deviations:** 2 auto-fixed issues.
**Impact on plan:** Both fixes were necessary to keep compilation and existing guard tests aligned with the new explicit `cmpctblock` command. No compact relay negotiation, serving, reconstruction, mempool lookup, fallback, validation/connect handoff, metrics, logs, RPC, CLI, dashboard, support bundle, or public-default behavior was introduced.

## Issues Encountered

- The broader `message` test filter initially failed because an existing deferred-command guard still listed `cmpctblock` as unknown. The guard now keeps `getblocktxn` and `blocktxn` deferred until Plan 112-03.

## Known Stubs

None found.

## Threat Flags

None. The new peer-bytes-to-`CompactBlockPayload` decode path is the planned trust boundary, and runtime policy remains deferred.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Ready for `112-03-PLAN.md` to add `getblocktxn` and `blocktxn` payload codecs plus the broader malformed-payload matrix. `sendcmpct` and `cmpctblock` are now typed wire messages, while negotiation, reconstruction, fallback, peer policy, and operator evidence remain intentionally unimplemented.

## Self-Check: PASSED

- Found `.planning/phases/112-bip152-wire-codec-and-message-semantics/112-02-SUMMARY.md` on disk.
- Confirmed the latest git commit remained `4b6e60cd fix(111): close block serving verifier gaps`; no commit was created for this plan.
- Confirmed the summary contains only the two YAML frontmatter delimiter lines and no body `---` separators.
