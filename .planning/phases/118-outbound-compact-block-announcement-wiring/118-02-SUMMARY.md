---
phase: 118-outbound-compact-block-announcement-wiring
plan: 02
subsystem: network
tags: [bip152, compact-block, announce, peer-manager, knots-parity]

requires:
  - phase: 118-01
    provides: Pure build_compact_block_payload(block, nonce) with Knots coinbase-only announce shape
  - phase: 113-compact-relay-negotiation-and-announcement-policy
    provides: CompactAnnouncementAction and decide_compact_announcement policy
provides:
  - PeerManager::announce_block_with_action emitting CompactBlock/Headers/Inv/None
  - Construction-failure fallback to Headers/Inv via remote_prefers_headers
  - Focused peer tests for action-aware announce emission
affects:
  - 118-03 ManagedPeerNetwork evidence-after-emit wiring

tech-stack:
  added: []
  patterns:
    - Action-aware announce API beside legacy Headers/Inv-only announce_block
    - Compact construction Err maps to Headers/Inv fallback without CompactBlock

key-files:
  created: []
  modified:
    - packages/open-bitcoin-network/src/peer.rs
    - packages/open-bitcoin-network/src/peer/tests.rs
    - docs/metrics/lines-of-code.md

key-decisions:
  - "New announce_block_with_action API; legacy announce_block signature unchanged (D-02)"
  - "announce_block delegates to Headers/Inv actions for DRY without compact path"
  - "CMP-05 left Pending until Plan 03 closes evidence-after-emit"

patterns-established:
  - "Network crate owns action→wire emission; node shell still owns evidence timing (Plan 03)"
  - "Construction failure never returns CompactBlock and never panics"

requirements-completed: []  # CMP-05 spans Plans 02–03; left Pending
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 118-2026-07-11T16-07-50
generated_at: 2026-07-11T18:45:42.000Z

duration: 53min
completed: 2026-07-11
---

# Phase 118 Plan 02: Action-Aware Announce Emission Summary

**PeerManager::announce_block_with_action emits CompactBlock/Headers/Inv/None per CompactAnnouncementAction, with Headers/Inv fallback on construction failure**

## Performance

- **Duration:** 53 min
- **Started:** 2026-07-11T17:52:35Z
- **Completed:** 2026-07-11T18:45:42Z
- **Tasks:** 2/2
- **Files modified:** 3

## Accomplishments

- Added `PeerManager::announce_block_with_action` that honors `CompactAnnouncementAction` on the wire.
- `AnnounceCompactBlock` calls `build_compact_block_payload` and returns `WireNetworkMessage::CompactBlock`; build `Err` falls back to Headers or Inv via `remote_prefers_headers`.
- Legacy `announce_block` signature preserved; it delegates to Headers/Inv action branches only.
- Seven focused peer tests cover CompactBlock / Headers / Inv / Suppress / unknown peer / construction fallbacks.

## Task Commits

Each task was committed atomically:

1. **Task 1 + Task 2: RED verified locally, GREEN committed** - `a3addc1e` (feat)
   - RED suite observed as compile errors for missing `announce_block_with_action` (`/tmp/118-02-red.log`)
   - Separate RED commit blocked by pre-commit `verify.sh` (requires green tests + rustfmt)
   - GREEN implementation + tests + LOC freshness landed together

**Plan metadata:** pending final docs commit

_Note: TDD RED was executed and verified; atomic RED commit could not land under repo hooks._

## Files Created/Modified

- `packages/open-bitcoin-network/src/peer.rs` - `announce_block_with_action` + legacy `announce_block` delegation
- `packages/open-bitcoin-network/src/peer/tests.rs` - Action-aware announce emission tests
- `docs/metrics/lines-of-code.md` - Hook-regenerated LOC freshness

## Decisions Made

- New `announce_block_with_action` API rather than mutating `announce_block` signature (D-02).
- Legacy `announce_block` delegates to Headers/Inv action branches for DRY; compact path stays explicit.
- No new first-party Rust source files → no `source-breadcrumbs.json` churn (`peer.rs` / `peer/tests.rs` already registered).
- CMP-05 remains Pending until Plan 03 wires ManagedPeerNetwork evidence-after-emit.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Separate RED commit blocked by full verify**
- **Found during:** Task 1 commit
- **Issue:** Repo pre-commit runs full `verify.sh`, which requires passing tests and rustfmt; a stub-only RED commit cannot land.
- **Fix:** Verified RED compile failures locally, then shipped tests + implementation in one `feat(118-02)` commit after GREEN passed.
- **Files modified:** `peer.rs`, `peer/tests.rs`
- **Verification:** `cargo test -p open-bitcoin-network -- announce_block_with_action` → 7 passed; pre-commit verify completed
- **Committed in:** `a3addc1e`

**2. [Rule 1 - Bug] Partial-move in construction-fallback tests**
- **Found during:** Task 2 GREEN compile
- **Issue:** `matches!(message, ... { inventory })` moved fields, then a second CompactBlock assert failed to compile.
- **Fix:** Bind with `ref inventory` / `ref headers` so `message` remains usable.
- **Files modified:** `packages/open-bitcoin-network/src/peer/tests.rs`
- **Verification:** GREEN suite compiled and passed
- **Committed in:** `a3addc1e`

**3. [Rule 2 - Correctness] Keep CMP-05 Pending until Plan 03**
- **Found during:** State/requirements update after Task 2
- **Issue:** Plan frontmatter lists `requirements: [CMP-05]`, but CMP-05 needs evidence-after-emit in Plan 03.
- **Fix:** Left CMP-05 unchecked/Pending in REQUIREMENTS.md after Plan 02 (emission only).
- **Files modified:** `.planning/REQUIREMENTS.md` (no mark-complete)
- **Verification:** Checklist and traceability still show Pending for Phase 118
- **Committed in:** docs metadata commit

**Total deviations:** 3 auto-fixed (Rule 3 × 1, Rule 1 × 1, Rule 2 × 1)
**Impact on plan:** No scope creep; CMP-05 emission deliverable unchanged. TDD followed locally; commit granularity adapted to hooks. Requirement completion deferred to Plan 03 / phase closeout.

## Issues Encountered

- Pre-commit verify takes ~30+ minutes; waited without `--no-verify`.
- rustfmt collapsed multi-line `announce_block_with_action` call sites; applied before commit.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Plan 118-03 can call `announce_block_with_action` from `ManagedPeerNetwork::announce_block` and record evidence from the message actually emitted.
- Nonce selection and evidence-after-emit remain Plan 03.

## Self-Check: PASSED

- FOUND: `packages/open-bitcoin-network/src/peer.rs` (`announce_block_with_action`)
- FOUND: `packages/open-bitcoin-network/src/peer/tests.rs` (7 action-aware tests)
- FOUND: commit `a3addc1e`

---
*Phase: 118-outbound-compact-block-announcement-wiring*
*Completed: 2026-07-11*
