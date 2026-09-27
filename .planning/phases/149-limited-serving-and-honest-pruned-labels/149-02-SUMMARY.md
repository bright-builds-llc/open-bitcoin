---
phase: 149-limited-serving-and-honest-pruned-labels
plan: "02"
subsystem: networking
tags: [network-limited, prune-mode, version-message, bip159, parity-breadcrumbs]

requires:
  - phase: 149-01
    provides: advertised_service_flags from PruneMode and ServiceFlags::NETWORK_LIMITED
provides:
  - prune_mode and serving_have_pruned on ManagedPeerNetwork
  - set_prune_mode writes version-message services from advertised_service_flags
  - ManagedNetworkHandle::set_serving_have_pruned via mutate
  - node-limited-serve parity breadcrumb group
affects:
  - 149-03 out-of-window block-body refusal and Pruned labels

tech-stack:
  added: []
  patterns:
    - "Version-message services follow PruneMode, not the have-pruned flag"

key-files:
  created:
    - packages/open-bitcoin-node/src/network/limited_serve.rs
    - packages/open-bitcoin-node/src/network/tests/limited_serve_advertisement.rs
  modified:
    - packages/open-bitcoin-node/src/network.rs
    - packages/open-bitcoin-node/src/network/runtime_authority.rs
    - packages/open-bitcoin-network/src/peer.rs
    - docs/parity/source-breadcrumbs.json

key-decisions:
  - "Clone copies prune_mode and serving_have_pruned with the rest of the network so advertisement stays paired with the copied local config"
  - "Leave SERV-01 Pending until lifecycle-valid Phase 149 verification"
  - "Combined 149-02 tasks into one hook-passing feat commit because pre-commit verify.sh requires breadcrumbs and covered lines"

patterns-established:
  - "set_prune_mode assigns advertised_service_flags onto ManagedPeerNetwork and PeerManager and does not read serving_have_pruned"
  - "Disabled production constructors call advertised_service_flags(PruneMode::Disabled), which is still NETWORK | WITNESS"
  - "node-limited-serve cites init.cpp, protocol.h, and net_processing.cpp"

requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 149-2026-09-27T19-48-11
generated_at: 2026-09-27T23:36:00Z

duration: 58min
completed: 2026-09-27
---

# Phase 149 Plan 02: Advertise Limited Service from Prune Mode Summary

**Manual and automatic prune advertise NETWORK_LIMITED | WITNESS on the version message even when nothing has been deleted; disabled mode stays NETWORK | WITNESS**

## Performance

- **Duration:** 58 min
- **Started:** 2026-09-27T22:38:10Z
- **Completed:** 2026-09-27T23:36:00Z
- **Tasks:** 2
- **Files modified:** 12

## Accomplishments

- Stored `prune_mode` and `serving_have_pruned` on `ManagedPeerNetwork`, both starting disabled and false.
- `set_prune_mode` writes `advertised_service_flags` onto the local peer config and `PeerManager`, so the next outbound version message follows the mode.
- Manual and automatic mode advertise `NETWORK_LIMITED | WITNESS` while `serving_have_pruned` is false. Disabled mode, `transient_runtime`, and sync `local_peer_config` stay `NETWORK | WITNESS`.
- `ManagedNetworkHandle::set_serving_have_pruned` locks through `mutate` and stores the bool without changing services.

## Task Commits

Each task was committed atomically:

1. **Task 1 + Task 2: prune-mode advertisement, version-message tests, and breadcrumbs** - `c0d3a5fb` (feat)

**Plan metadata:** docs commit for this summary

_Note: Task 1 action 7 lands the tests and implementation in the same commit as the Task 2 breadcrumbs so pre-commit `verify.sh` can pass._

## Files Created/Modified

- `packages/open-bitcoin-node/src/network/limited_serve.rs` — `set_prune_mode`, `set_serving_have_pruned`, and `local_services`
- `packages/open-bitcoin-node/src/network/tests/limited_serve_advertisement.rs` — manual, automatic, disabled, and version-message tests
- `packages/open-bitcoin-node/src/network.rs` — `prune_mode` and `serving_have_pruned` fields
- `packages/open-bitcoin-node/src/network/relay_serving.rs` — constructors start at `PruneMode::Disabled` and `false`
- `packages/open-bitcoin-node/src/network/peer_network_clone.rs` — clone copies both fields
- `packages/open-bitcoin-node/src/network/runtime_authority.rs` — handle setter and disabled `transient_runtime` services
- `packages/open-bitcoin-node/src/sync/progress.rs` — disabled `local_peer_config` services
- `packages/open-bitcoin-network/src/peer.rs` — `PeerManager::set_local_services`
- `packages/open-bitcoin-network/src/peer/tests/handshake_policy_cases.rs` — coverage for the setter on the version message
- `docs/parity/source-breadcrumbs.json` — `node-limited-serve` group
- `docs/metrics/lines-of-code.md` — verifier freshness update from the pre-commit hook

## Decisions Made

- Clone copies `prune_mode` and `serving_have_pruned`. Resetting them to disabled while still cloning `local_config` and `PeerManager` would split the advertisement from the stored mode.
- SERV-01 stays Pending. This plan does not flip requirement rows before lifecycle-valid phase verification.
- Production advertisement changes are the two constructors the plan named: `transient_runtime` and `progress::local_peer_config`. Both call `advertised_service_flags(PruneMode::Disabled)`.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Clone keeps prune advertisement state**
- **Found during:** Task 1 (Prune mode selects version-message services)
- **Issue:** The plan told the `Clone` impl to hardcode `PruneMode::Disabled` and `serving_have_pruned: false`. Every other field is copied, including `local_config` and `peer_manager`, which already hold the services chosen by `set_prune_mode`. A hardcoded reset would leave mode and version-message bits disagreeing after clone.
- **Fix:** Clone copies `prune_mode` and `serving_have_pruned`. New networks still start at `PruneMode::Disabled` and `false` in both `relay_serving` constructors.
- **Files modified:** `packages/open-bitcoin-node/src/network/peer_network_clone.rs`
- **Verification:** The crate compiles and the advertisement tests pass. Clone sites continue to compile because the struct literal is exhaustive.
- **Committed in:** `c0d3a5fb` (Task 1 commit)

**2. [Rule 3 - Blocking] Covered `PeerManager::set_local_services` in the network crate**
- **Found during:** Task 1 commit hook (`bash scripts/verify.sh`)
- **Issue:** `llvm-cov` reported uncovered lines 471–473 in `packages/open-bitcoin-network/src/peer.rs`. Node tests call the setter through `set_prune_mode`, but coverage is measured per crate, so the network crate never executed the new method.
- **Fix:** Added `set_local_services_changes_the_outbound_version_message` in `handshake_policy_cases.rs`. It sets `NETWORK_LIMITED | WITNESS` and asserts the outbound version message.
- **Files modified:** `packages/open-bitcoin-network/src/peer/tests/handshake_policy_cases.rs`
- **Verification:** `cargo test -p open-bitcoin-network --lib set_local_services_changes_the_outbound_version_message` passed, then the pre-commit `verify.sh` completed with no uncovered lines.
- **Committed in:** `c0d3a5fb` (Task 1 commit)

---

**Total deviations:** 2 auto-fixed (1 bug, 1 blocking)
**Impact on plan:** Both fixes keep advertisement state consistent and let the verification hook pass. No scope change to serving or prune labels.

## Issues Encountered

The first commit hook failed after 20 minutes because `set_local_services` was uncovered in `open-bitcoin-network`. The coverage test above fixed it, and the retry completed `verify.sh` in 26 minutes.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Ready for 149-03. `set_prune_mode` chooses the version-message bits from `PruneMode` alone. `ManagedNetworkHandle::set_serving_have_pruned` stores the bool for a later `Pruned` label and does not assign `BlockServingDataAvailability::Pruned`. This plan does not refuse block bodies or add a remote limited-peer download policy.

## Self-Check: PASSED

- FOUND: packages/open-bitcoin-node/src/network/limited_serve.rs
- FOUND: packages/open-bitcoin-node/src/network/tests/limited_serve_advertisement.rs
- FOUND: c0d3a5fb
