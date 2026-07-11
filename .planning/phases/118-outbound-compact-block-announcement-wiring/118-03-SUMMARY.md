---
phase: 118-outbound-compact-block-announcement-wiring
plan: 03
subsystem: network
tags: [bip152, compact-block, announce, evidence, knots-parity]

requires:
  - phase: 118-02
    provides: PeerManager::announce_block_with_action CompactBlock/Headers/Inv/None emission
  - phase: 118-01
    provides: Pure build_compact_block_payload coinbase-only announce shape
  - phase: 113-compact-relay-negotiation-and-announcement-policy
    provides: CompactAnnouncementAction and decide_compact_announcement policy
provides:
  - ManagedPeerNetwork::announce_block honors CompactAnnouncementAction via announce_block_with_action
  - Evidence-after-emit mapping (CompactAnnounced iff CompactBlock on the wire)
  - D-09 runtime proofs for HB CompactBlock emission, low-bandwidth non-increment, construction-fallback mapper
affects:
  - CMP-05 requirement satisfaction
  - Phase 119+ compact receive/timeout/metrics (deferred)

tech-stack:
  added: []
  patterns:
    - Evidence reason derived from decision + emitted WireNetworkMessage after announce
    - Deterministic hash-derived compact nonce stand-in (not Knots FastRandomContext)

key-files:
  created: []
  modified:
    - packages/open-bitcoin-node/src/network.rs
    - packages/open-bitcoin-node/src/network/block_relay_evidence.rs
    - packages/open-bitcoin-node/src/network/tests.rs

key-decisions:
  - "Evidence recorded after emission from actual message (D-05)"
  - "Hash-derived deterministic nonce: first 8 LE bytes of block hash"
  - "CMP-05 satisfied by this plan closing the runtime seam"

patterns-established:
  - "compact_announce_evidence_reason maps construction fallbacks without false CompactAnnounced"
  - "Node shell owns evidence timing; network crate owns action→wire emission"

requirements-completed: [CMP-05]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 118-2026-07-11T16-07-50
generated_at: 2026-07-11T19:13:42.000Z

duration: 16min
completed: 2026-07-11
---

# Phase 118 Plan 03: Evidence-After-Emit Announce Wiring Summary

**ManagedPeerNetwork::announce_block honors CompactAnnouncementAction and increments compact_announced_count only when CompactBlock is actually emitted**

## Performance

- **Duration:** 16 min
- **Started:** 2026-07-11T18:56:56Z
- **Completed:** 2026-07-11T19:13:42Z
- **Tasks:** 2/2
- **Files modified:** 3

## Accomplishments

- Rewired `ManagedPeerNetwork::announce_block` to call `announce_block_with_action` with a hash-derived nonce and record evidence only after emission.
- Added pure `compact_announce_evidence_reason` mapper proving CompactAnnounced vs Headers/Inv construction fallbacks vs policy-reason preservation.
- Fixed Phase 116 HB evidence test to require `WireNetworkMessage::CompactBlock` plus `compact_announced_count == 1`.
- Added low-bandwidth path proof that never increments `compact_announced_count`.

## Task Commits

Each task was committed atomically:

1. **Task 1 + Task 2: RED verified locally via prior false-positive seam, GREEN committed** - `c8745b06` (feat)
   - Mapper unit tests + runtime HB/LB proofs + announce rewire landed together
   - Separate RED commit blocked by pre-commit `verify.sh` (requires green tests)

**Plan metadata:** pending final docs commit

_Note: TDD RED was followed for behavior design; atomic RED commit could not land under repo hooks (same as Plans 01–02)._

## Files Created/Modified

- `packages/open-bitcoin-node/src/network.rs` - Action-honor announce + post-emission evidence
- `packages/open-bitcoin-node/src/network/block_relay_evidence.rs` - `compact_announce_evidence_reason` + mapper unit tests
- `packages/open-bitcoin-node/src/network/tests.rs` - CompactBlock message assert + low-bandwidth non-increment proof

## Decisions Made

- Evidence is recorded after emission from the message actually produced (D-05).
- Compact nonce is the first 8 LE bytes of the block hash — deterministic stand-in, not Knots FastRandomContext parity.
- No new first-party Rust source files → no `source-breadcrumbs.json` churn (existing files already registered).
- CMP-05 is satisfied by this plan closing the decide→emit→evidence seam.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Separate RED commit blocked by full verify**
- **Found during:** Task 1 commit
- **Issue:** Repo pre-commit runs full `verify.sh`, which requires passing tests; a failing CompactBlock assertion cannot land alone.
- **Fix:** Verified prior false-positive seam (decide CompactAnnounced then emit Headers/Inv), then shipped tests + implementation in one `feat(118-03)` commit after GREEN passed.
- **Files modified:** `network.rs`, `block_relay_evidence.rs`, `network/tests.rs`
- **Verification:** `cargo test -p open-bitcoin-node -- compact_announce` (7 passed); `network::tests` (86 passed); pre-commit verify completed
- **Committed in:** `c8745b06`

**2. [Rule 1 - Bug] MerkleRoot type in mapper fixture**
- **Found during:** Task 1 GREEN compile
- **Issue:** Test fixture used `Hash32` for `BlockHeader.merkle_root`; type is `MerkleRoot`.
- **Fix:** Import and construct `MerkleRoot::from_byte_array`.
- **Files modified:** `packages/open-bitcoin-node/src/network/block_relay_evidence.rs`
- **Verification:** Mapper suite compiled and passed
- **Committed in:** `c8745b06`

**Total deviations:** 2 auto-fixed (Rule 3 × 1, Rule 1 × 1)
**Impact on plan:** No scope creep; CMP-05 runtime seam closed as planned. TDD followed locally; commit granularity adapted to hooks.

## Issues Encountered

- Pre-commit verify takes ~14+ minutes; waited without `--no-verify`.
- Avoided `expect` on nonce slice conversion by indexing the hash byte array explicitly.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Phase 118 CMP-05 runtime seam is closed.
- Deferred Phases 119–121 (mempool inject, compact timeouts, DurableSyncRuntime metrics) remain untouched.
- Phase closeout may still run full `bash scripts/verify.sh` via orchestrator if desired; this plan's pre-commit already exercised the verify contract.

## Self-Check: PASSED

- FOUND: `packages/open-bitcoin-node/src/network.rs` (`announce_block_with_action` + evidence-after-emit)
- FOUND: `packages/open-bitcoin-node/src/network/block_relay_evidence.rs` (`compact_announce_evidence_reason`)
- FOUND: `packages/open-bitcoin-node/src/network/tests.rs` (HB CompactBlock + LB non-increment)
- FOUND: commit `c8745b06`
