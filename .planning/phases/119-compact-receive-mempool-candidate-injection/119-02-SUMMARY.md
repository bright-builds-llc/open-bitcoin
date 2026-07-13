---
phase: 119-compact-receive-mempool-candidate-injection
plan: 02
subsystem: network
tags: [compact-block, mempool, bip152, reconstruction, admission-bridge]

requires:
  - phase: 119-01
    provides: CompactExtraTxnBuffer, mempool_compact_candidate_owned, PeerManager::on_mempool_transaction_removed
provides:
  - ManagedPeerNetwork CompactBlock shell intercept with mempool+extra CompactBlockReceiveFacts
  - Admission-bridge feeds into CompactExtraTxnBuffer (orphan/reject/replaced victims)
affects:
  - 119-03 mempool lifecycle hook and runtime tests

tech-stack:
  added: []
  patterns:
    - Shell CompactBlock intercept before handle_message empty-facts path
    - Owned candidate/extra snapshot before PeerManager mutable borrow
    - Knots-aligned admission feeds into CompactExtraTxnBuffer

key-files:
  created:
    - packages/open-bitcoin-node/src/network/tests/compact_receive_cases.rs
  modified:
    - packages/open-bitcoin-node/src/network.rs
    - packages/open-bitcoin-node/src/network/compact_receive_candidates.rs
    - packages/open-bitcoin-node/src/network/relay_serving.rs
    - packages/open-bitcoin-node/src/network/admission_bridge.rs
    - packages/open-bitcoin-node/src/network/tests.rs
    - packages/open-bitcoin-node/src/network/tests/admission_bridge_cases.rs
    - docs/parity/source-breadcrumbs.json

key-decisions:
  - "Live CompactBlock intercepts in receive_message and receive_sync_message call handle_compact_block_download with shell-built facts (D-01..D-04)"
  - "Orphan push / reject push_gated / replaced-victim push happen in admission_bridge before demotion; admitted Replaced wtxid is not an extras-removal feed (D-05)"
  - "PeerManager stays free of open-bitcoin-mempool; empty-facts handle_message path remains for non-shell callers"

patterns-established:
  - "Pattern: collect_compact_receive_owned then slice-ref CompactBlockReceiveFacts before PeerManager call"
  - "Pattern: admission outcomes feed CompactExtraTxnBuffer without package/filter surfaces"

requirements-completed: [RCN-02]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 119-2026-07-13T16-08-52
generated_at: 2026-07-13T18:46:30Z

duration: 14min
completed: 2026-07-13
---

# Phase 119 Plan 02: Shell CompactBlock Intercept and Extra Feeds Summary

**Live ManagedPeerNetwork CompactBlock receive injects mempool + bounded extras into handle_compact_block_download, and admission orphan/reject/replaced-victim bodies feed CompactExtraTxnBuffer.**

## Performance

- **Duration:** 14 min
- **Started:** 2026-07-13T18:31:23Z
- **Completed:** 2026-07-13T18:45:57Z
- **Tasks:** 2
- **Files modified:** 7

## Accomplishments

- Intercepted `WireNetworkMessage::CompactBlock` in both `receive_message` and `receive_sync_message` so production receive no longer uses empty `CompactBlockReceiveFacts::default()`.
- Added `compact_extra_txn` on `ManagedPeerNetwork` and owned snapshot helpers that avoid simultaneous mempool/PeerManager borrows.
- Fed orphaned staged bodies, size-gated rejects, and replaced-victim bodies into `CompactExtraTxnBuffer` from admission outcomes.

## Task Commits

Each task was committed atomically:

1. **Task 1: Shell CompactBlock intercept with mempool+extra facts** - `8be8cd2d` (test) → `d9e85b0b` (feat)
2. **Task 2: Feed CompactExtraTxnBuffer from admission outcomes** - `721cad8e` (test) → `800ef65e` (feat) → `27682cc4` (style)

**Plan metadata:** `e5c0b038` (docs: complete plan)

_Note: TDD tasks used RED → GREEN commits_

## Files Created/Modified

- `packages/open-bitcoin-node/src/network.rs` — CompactBlock intercept in receive paths
- `packages/open-bitcoin-node/src/network/compact_receive_candidates.rs` — owned snapshot + `handle_compact_block_receive` helper; test accessor
- `packages/open-bitcoin-node/src/network/relay_serving.rs` — `compact_extra_txn` field init; `maybe_accepted_wtxid_and_transaction`
- `packages/open-bitcoin-node/src/network/admission_bridge.rs` — orphan/reject/replaced-victim extra feeds
- `packages/open-bitcoin-node/src/network/tests/compact_receive_cases.rs` — live receive runtime proofs
- `packages/open-bitcoin-node/src/network/tests/admission_bridge_cases.rs` — orphan/reject/replaced feed proofs
- `packages/open-bitcoin-node/src/network/tests.rs` — wire compact_receive_cases module
- `docs/parity/source-breadcrumbs.json` — register compact_receive_cases

## Decisions Made

- Shell intercept is the live seam; PeerManager empty-facts CompactBlock branch stays for non-production/test callers.
- Replaced victims are resolved from relay serving records / `transactions_by_txid` before demotion; admitted Replaced wtxid is never treated as an extras-removal feed.
- No package relay, bloom/filter, or Phase 120 timeout work.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Live receive timestamp too low for reconstructed block connect**
- **Found during:** Task 1 GREEN
- **Issue:** Injected facts completed reconstruction and `ReceivedBlock` connect failed with `time-too-new` when receive timestamp was `10`.
- **Fix:** Tests use `announced.header.time + 60` as receive time so completed reconstruction can connect.
- **Files modified:** `packages/open-bitcoin-node/src/network/tests/compact_receive_cases.rs`
- **Verification:** `live_compact_receive_*` tests pass
- **Committed in:** `d9e85b0b` (Task 1 feat; test time fix landed with GREEN)

---

**Total deviations:** 1 auto-fixed (Rule 1)
**Impact on plan:** Necessary for correct Completed-path assertion; no scope creep.

## Issues Encountered

None beyond the timestamp fix above.

## User Setup Required

None

## Known Stubs

None

## Threat Flags

None — CompactBlock validation paths unchanged; extra feeds use existing `push` / `push_gated` bounds (T-119-05..T-119-07 mitigated as planned).

## Next Phase Readiness

Ready for Plan 03: mempool lifecycle hook wiring (`on_mempool_transaction_removed`) and remaining runtime proofs.

## Verification Evidence

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib compact_receive` — pass (8 tests)
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib admission_bridge` — pass (17 tests)
- PeerManager remains free of `open-bitcoin-mempool` coupling (shell-only facts adaptation)

## Self-Check: PASSED

- Created/modified key files present on disk
- Commits `8be8cd2d`, `d9e85b0b`, `721cad8e`, `800ef65e`, `27682cc4` present in git log
- Acceptance must_haves verified (live CompactBlock intercept, admission extra feeds, PeerManager mempool-free)
