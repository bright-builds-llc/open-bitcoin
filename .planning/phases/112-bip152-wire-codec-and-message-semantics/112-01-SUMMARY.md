---
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 112-2026-07-04T19-37-55
generated_at: 2026-07-04T19:56:22Z
phase: 112-bip152-wire-codec-and-message-semantics
plan: 01
subsystem: codec-network
tags: [rust, bip152, sendcmpct, p2p-wire, parity]

requires:
  - phase: 111-full-block-serving-request-path
    provides: Full-block serving path with compact-block serving deferred
provides:
  - BIP152 sendcmpct payload type and fixed-width encode/decode helpers
  - WireNetworkMessage SendCompact branch mapped to command sendcmpct
  - Focused codec and network message tests for version 2 and unsupported versions
affects: [phase-113-compact-relay-negotiation, phase-112-plan-02, phase-112-plan-03]

tech-stack:
  added: []
  patterns:
    - Pure codec payload helpers in open-bitcoin-codec
    - Network message enum delegates BIP152 payload bytes to codec helpers

key-files:
  created:
    - packages/open-bitcoin-codec/src/compact_block.rs
  modified:
    - packages/open-bitcoin-codec/src/lib.rs
    - packages/open-bitcoin-network/src/message.rs
    - packages/open-bitcoin-network/src/message/tests.rs
    - packages/open-bitcoin-network/src/peer.rs
    - docs/parity/source-breadcrumbs.json

key-decisions:
  - "Decoded sendcmpct versions are preserved as data, including unsupported versions 1 and 3."
  - "Peer-level sendcmpct handling is intentionally a no-op until Phase 113 owns compact relay policy."
  - "No git commits were created because the wrapper reserves final commit ownership for the orchestrator."

patterns-established:
  - "BIP152 command payloads enter WireNetworkMessage through explicit variants, not overloaded inventory/block/transaction branches."
  - "New first-party codec source files carry both inline Knots breadcrumbs and registry entries."

requirements-completed: [CMP-01]

duration: 5m 12s
completed: 2026-07-04
---

# Phase 112 Plan 01: SendCompact Wire Codec Summary

**BIP152 sendcmpct payload support with exact 9-byte codec semantics, explicit network command mapping, and parity breadcrumbs.**

## Performance

- **Duration:** 5m 12s
- **Started:** 2026-07-04T19:51:10Z
- **Completed:** 2026-07-04T19:56:22Z
- **Tasks:** 2 completed
- **Files modified:** 7 including this summary

## Accomplishments

- Added `SendCompactMessage`, `BIP152_COMPACT_BLOCKS_VERSION`, `encode_send_compact_payload`, and `decode_send_compact_payload` in a new `open-bitcoin-codec` BIP152 module.
- Wired `WireNetworkMessage::SendCompact` to command `sendcmpct`, including payload encode/decode delegation and wire round-trip coverage.
- Preserved Phase 113 policy ownership by decoding unsupported `sendcmpct` versions as data and leaving peer-level handling as a no-op.
- Registered the new codec source in `docs/parity/source-breadcrumbs.json` with Knots protocol, block encoding, net processing, and functional-test anchors.

## Task Changes

No commits were created. The wrapper instructed this executor to leave all changes uncommitted for orchestrator finalization.

1. **Task 1: Create the `sendcmpct` codec contract**
   - Added failing RED tests for version 2 round-trip, unsupported versions 1/3, short payload EOF, and trailing payload rejection.
   - Implemented the pure codec payload type and helpers.
   - Added crate exports and parity breadcrumb registration.

2. **Task 2: Wire `sendcmpct` into `WireNetworkMessage`**
   - Added failing RED tests for network payload/wire round-trip, unsupported-version message decoding, and adjacent unknown commands.
   - Added `WireNetworkMessage::SendCompact(SendCompactMessage)` and the `sendcmpct` command mapping.
   - Added the minimal peer match arm required for exhaustive compilation without enabling compact relay policy.

## Files Created/Modified

- `packages/open-bitcoin-codec/src/compact_block.rs` - New parity-breadcrumbed BIP152 `sendcmpct` codec module.
- `packages/open-bitcoin-codec/src/lib.rs` - Exports the new BIP152 codec surface.
- `packages/open-bitcoin-network/src/message.rs` - Maps `sendcmpct` to and from `WireNetworkMessage::SendCompact`.
- `packages/open-bitcoin-network/src/message/tests.rs` - Covers `sendcmpct` payload/wire round trips and unknown adjacent commands.
- `packages/open-bitcoin-network/src/peer.rs` - Keeps decoded `sendcmpct` peer handling as a no-op until Phase 113.
- `docs/parity/source-breadcrumbs.json` - Registers the new codec source under `codec-bip152-compact-block`.

## Decisions Made

- Unsupported `sendcmpct` versions decode successfully because Knots ignores unsupported compact-block versions after parsing, and Phase 113 owns policy.
- The network message layer has the only new command branch in this plan; `cmpctblock`, `getblocktxn`, and `blocktxn` remain unknown until later Phase 112 plans.
- The peer handler returns no actions for `SendCompact` to satisfy Rust exhaustiveness without introducing compact relay negotiation or announcement behavior.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Added exhaustive peer handling for the new enum variant**
- **Found during:** Task 2 (Wire `sendcmpct` into `WireNetworkMessage`)
- **Issue:** Adding `WireNetworkMessage::SendCompact` made `PeerManager::handle_message` non-exhaustive.
- **Fix:** Added `WireNetworkMessage::SendCompact(_) => Ok(Vec::new())` so decoded messages have no peer-state consequences until Phase 113.
- **Files modified:** `packages/open-bitcoin-network/src/peer.rs`
- **Verification:** `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network message -- --nocapture`
- **Committed in:** Not committed per wrapper no-commit override.

***

**Total deviations:** 1 auto-fixed (1 blocking)
**Impact on plan:** The fix preserves the explicit no-policy boundary. No compact-block serving, negotiation, mempool, metrics, logs, RPC, CLI, dashboard, or support-bundle behavior was introduced.

## Issues Encountered

- `bun run scripts/check-parity-breadcrumbs.ts --check` uses `git ls-files`, so the new `compact_block.rs` file had to be marked with `git add -N` intent-to-add before the checker could see it. No commit was created.

## Known Stubs

None found.

## Threat Flags

None. The new trust boundary is the planned peer-bytes-to-`SendCompactMessage` decode path, and runtime policy remains deferred.

## Verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-codec compact_block -- --nocapture` - passed
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network message::tests::phase112_wire_sendcmpct -- --nocapture` - passed
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network message -- --nocapture` - passed
- `cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-codec -p open-bitcoin-network --all-targets --all-features -- -D warnings` - passed
- `bun run scripts/check-parity-breadcrumbs.ts --check` - passed
- `cargo fmt --manifest-path packages/Cargo.toml --all` - applied formatting

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Ready for `112-02-PLAN.md` to add the `cmpctblock` codec surface. `sendcmpct` is now decoded as data and available through `WireNetworkMessage`, but compact relay negotiation and announcement policy remain intentionally unimplemented.

## Self-Check: PASSED

- Found `.planning/phases/112-bip152-wire-codec-and-message-semantics/112-01-SUMMARY.md` on disk.
- Confirmed the latest git commit remained `4b6e60cd fix(111): close block serving verifier gaps`; no commit was created for this plan.

***
*Phase: 112-bip152-wire-codec-and-message-semantics*
*Completed: 2026-07-04*
