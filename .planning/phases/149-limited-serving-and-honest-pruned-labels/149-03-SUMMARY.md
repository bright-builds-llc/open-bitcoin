---
phase: 149-limited-serving-and-honest-pruned-labels
plan: "03"
subsystem: networking
tags: [network-limited, prune, block-serving, bip159, notfound]

requires:
  - phase: 149-02
    provides: prune_mode, serving_have_pruned, and ManagedNetworkHandle::set_serving_have_pruned
provides:
  - Out-of-window block bodies and getblocktxn return NotFound without payload bytes
  - Ordinary peers are disconnected; download peers are not; noban is not exempt
  - Historical block inventory is withheld and a recent Inv is still offered
  - Pruned availability only for a have-pruned active-chain gap
affects:
  - 149-04 remote limited-peer download policy

tech-stack:
  added: []
  patterns:
    - "Window refusal happens in the shared serve input before lookup"
    - "have_pruned loads once per inbound message and a failed load stores false"

key-files:
  created:
    - packages/open-bitcoin-node/src/network/tests/block_serving/limited_window.rs
  modified:
    - packages/open-bitcoin-node/src/network/inventory.rs
    - packages/open-bitcoin-node/src/network/block_serving.rs
    - packages/open-bitcoin-node/src/network/limited_serve.rs
    - packages/open-bitcoin-node/src/network/action_translation.rs
    - packages/open-bitcoin-node/src/network/announcement_transport.rs
    - packages/open-bitcoin-rpc/src/context.rs
    - packages/open-bitcoin-rpc/src/context/inbound_wire.rs
    - packages/open-bitcoin-rpc/src/inbound_listener/connection_runtime.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/prune.rs
    - docs/parity/source-breadcrumbs.json

key-decisions:
  - "Combined 149-03 tasks into one hook-passing feat commit because pre-commit runs verify.sh"
  - "SERV-02 and SERV-03 are listed in requirements-completed because 149-VERIFICATION.md is passed"
  - "noban expands to the download effect, so MisbehaviorPolicyProtected is not a window exemption"
  - "Inbound permission fixtures include the in direction because the parser requires it"

patterns-established:
  - "limited_window_refused denies the gate before lookup even when the payload is cached"
  - "Out-of-window announcements return Suppressed before the disabled-compact Inv fallback"

requirements-completed: [SERV-02, SERV-03]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 149-2026-09-27T19-48-11
generated_at: 2026-09-28T02:45:41Z

duration: 1h 14m
completed: 2026-09-28
---

# Phase 149 Plan 03: Refuse Out-of-Window and Pruned Block Bodies Summary

**Out-of-window block bodies and getblocktxn are NotFound, ordinary peers disconnect, historical inventory is withheld, and Pruned appears only after a real delete**

## Performance

- **Duration:** 1h 14m
- **Started:** 2026-09-28T01:31:01Z
- **Completed:** 2026-09-28T02:45:41Z
- **Tasks:** 3
- **Files modified:** 16

## Accomplishments

- A cached block past distance 290 is NotFound before lookup. Distance 290 is still served. Disabled prune still serves height 0.
- Ordinary and noban peers are removed. A download peer gets NotFound and stays. In-window misses do not disconnect.
- Historical preparation is Suppressed. The tip is still a Ready Inv. Out-of-window getblocktxn is NotFound with no BlockTxn.
- `have_pruned` loads once per inbound message. A load error stores false, so a failed read cannot become Pruned.

## Task Commits

Each task was committed atomically:

1. **Task 1–3: window flag, NotFound disconnect, have-pruned load, and tests** - `1b15bac0` (feat)

**Plan metadata:** docs commit for this summary

_Note: Tasks 1–3 landed in one hook-passing feat commit so pre-commit `verify.sh` could pass. A RED-only commit fails that hook._

## Files Created/Modified

- `packages/open-bitcoin-node/src/network/inventory.rs` — window flag, Pruned availability, request-path disconnect
- `packages/open-bitcoin-node/src/network/block_serving.rs` — gate deny before lookup
- `packages/open-bitcoin-node/src/network/limited_serve.rs` — ordinary-peer disconnect predicate
- `packages/open-bitcoin-node/src/network/action_translation.rs` — getblocktxn NotFound and disconnect
- `packages/open-bitcoin-node/src/network/announcement_transport.rs` — historical suppression before Inv
- `packages/open-bitcoin-rpc/src/context.rs` — load have_pruned and apply it on the handle
- `packages/open-bitcoin-rpc/src/context/inbound_wire.rs` — durable have-pruned load
- `packages/open-bitcoin-rpc/src/inbound_listener/connection_runtime.rs` — leave the loop after the peer is removed
- `packages/open-bitcoin-node/src/storage/fjall_store/prune.rs` — public `load_have_pruned`
- `packages/open-bitcoin-node/src/network/tests/block_serving/limited_window.rs` — window, disconnect, announcement, and getblocktxn tests
- `docs/parity/source-breadcrumbs.json` — limited-window tests in `node-limited-serve`

## Decisions Made

- `noban` expands to the download permission in the existing parser. The disconnect predicate still removes a peer whose effects include `MisbehaviorPolicyProtected`, so noban is not a window exemption. Explicit download without that effect stays connected.
- Inbound permission fixtures include `in` because a class without that direction fails to parse. The download and noban effects are unchanged.
- SERV-02 and SERV-03 are listed in `requirements-completed` because `149-VERIFICATION.md` is passed.
- The Phase 143 source scan that forbade any `Pruned` assignment now requires the have-pruned active-chain gate.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] noban is not treated as a download exemption**
- **Found during:** Task 3 (window tests)
- **Issue:** `PeerPermissionToken::NoBan` also inserts `Download`. The plan's predicate exempts any effect list that contains `DownloadServingPolicyInput`, so a noban peer stayed connected.
- **Fix:** A limited-window request still disconnects when the effects include `MisbehaviorPolicyProtected`. Explicit download without that effect does not disconnect.
- **Files modified:** `packages/open-bitcoin-node/src/network/limited_serve.rs`
- **Verification:** `noban_permission_does_not_keep_an_out_of_window_peer` and the download test both passed.
- **Committed in:** `1b15bac0` (task commit)

**2. [Rule 3 - Blocking] Inbound fixtures include the `in` direction**
- **Found during:** Task 3
- **Issue:** `permissioned_inbound_request` parses `&["download"]` and `&["noban"]` as `MissingInboundDirection`.
- **Fix:** The cases use `&["in", "download"]` and `&["in", "noban"]`.
- **Files modified:** `packages/open-bitcoin-node/src/network/tests/block_serving/limited_window.rs`
- **Verification:** Both permission tests passed.
- **Committed in:** `1b15bac0` (task commit)

**3. [Rule 1 - Bug] Coinbase height encoding covers the sign bit**
- **Found during:** Task 3
- **Issue:** `build_block` does not pad script numbers whose high bit is set, so connecting height 128 failed `bad-cb-height`.
- **Fix:** The window fixture writes the consensus height prefix, including the extra `0x00` when the last byte has the sign bit set.
- **Files modified:** `packages/open-bitcoin-node/src/network/tests/block_serving/limited_window.rs`, `packages/open-bitcoin-rpc/src/inbound_listener/tests/block_serving.rs`
- **Verification:** Chains through height 291 connect, and the window tests passed.
- **Committed in:** `1b15bac0` (task commit)

**4. [Rule 3 - Blocking] Socket-close coverage and the public prune-mode setter**
- **Found during:** Task 2
- **Issue:** The new message-loop break is uncovered unless an inbound socket receives an out-of-window NotFound. `set_prune_mode` existed only on `ManagedPeerNetwork`, and `runtime_authority.rs` is already 626 lines.
- **Fix:** Added `ManagedNetworkHandle::set_prune_mode` on the existing prune-flush impl. The RPC test setter is `cfg(test)`. The listener test connects a 0..=291 chain, requests height 0, and expects NotFound then a closed socket.
- **Files modified:** `packages/open-bitcoin-node/src/network/runtime_authority/prune_flush.rs`, `packages/open-bitcoin-rpc/src/context.rs`, `packages/open-bitcoin-rpc/src/inbound_listener/tests/block_serving.rs`
- **Verification:** `out_of_window_notfound_closes_the_inbound_socket` passed, and the pre-commit verifier completed.
- **Committed in:** `1b15bac0` (task commit)

**5. [Rule 1 - Bug] Phase 143 source guard still forbade any Pruned assignment**
- **Found during:** Task 1 commit hook
- **Issue:** `production_inventory_source_does_not_inject_pruned` failed once inventory assigned `BlockServingDataAvailability::Pruned`.
- **Fix:** The guard now requires that assignment to sit behind `serving_have_pruned && validated_on_active_chain`.
- **Files modified:** `packages/open-bitcoin-node/src/network/tests/block_serving.rs`
- **Verification:** `production_inventory_source_assigns_pruned_only_for_a_have_pruned_gap` passed.
- **Committed in:** `1b15bac0` (task commit)

**Total deviations:** 5 auto-fixed (3 bug, 2 blocking)
**Impact on plan:** The fixes keep the window, noban, and Pruned contracts true and let the verifier pass. No serving surface beyond this plan.

## Issues Encountered

The first commit hook failed because `set_prune_mode` on the RPC context was unused outside tests. `#[cfg(test)]` fixed it. The second hook failed the Phase 143 source guard above. The third pre-commit `verify.sh` completed.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Ready for 149-04. Out-of-window bodies are refused, ordinary peers disconnect, and Pruned is earned only when `have_pruned` is true. This plan does not add a remote limited-peer download policy.

## Self-Check: PASSED

- FOUND: packages/open-bitcoin-node/src/network/tests/block_serving/limited_window.rs
- FOUND: 1b15bac0

---
*Phase: 149-limited-serving-and-honest-pruned-labels*
*Completed: 2026-09-28*
