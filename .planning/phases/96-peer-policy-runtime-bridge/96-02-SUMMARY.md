---
phase: 96-peer-policy-runtime-bridge
plan: 02
subsystem: node-rpc-runtime
tags: [rust, managed-network, rpc, inbound-listener, peer-policy]
requires:
  - phase: 96-peer-policy-runtime-bridge
    provides: Plan 01 pure PeerPolicyRuntimeState and PeerManager ownership.
provides:
  - ManagedPeerNetwork projection of actual runtime peer-policy decisions.
  - Scoped RPC reconnect suppression using remote_addr.ip() and injected timestamps.
affects: [peer-policy-runtime-bridge, inbound-listener, operator-status]
tech-stack:
  added: []
  patterns: [managed-runtime-projection, scoped-reconnect-suppression, test-only-context-seeding]
key-files:
  created: []
  modified:
    - packages/open-bitcoin-node/src/network.rs
    - packages/open-bitcoin-node/src/network/tests.rs
    - packages/open-bitcoin-rpc/src/context/network.rs
    - packages/open-bitcoin-rpc/src/inbound_listener/tests.rs
key-decisions:
  - "ManagedPeerNetwork::peer_policy_info reads runtime decision slices instead of empty placeholders."
  - "ManagedRpcContext reconnect suppression queries remote_addr.ip() with the injected timestamp."
  - "RPC peer-policy record helpers are cfg(test) seed helpers, not a widened production context API."
patterns-established:
  - "Managed runtime adapters project pure policy state through narrow methods."
  - "Listener reconnect tests use loopback-safe deterministic policy state rather than public network dependencies."
requirements-completed: [EVICT-03, EVICT-04, DOS-03]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 96-2026-06-28T02-38-04
generated_at: 2026-06-28T03:56:21Z
duration: 8min
completed: 2026-06-28
---

# Phase 96 Plan 02: Managed Runtime Projection Summary

**Managed node and RPC runtime now project real peer-policy decisions and suppress reconnects only for matching remotes.**

## Performance

- **Duration:** 8min
- **Started:** 2026-06-28T03:48:45Z
- **Completed:** 2026-06-28T03:56:21Z
- **Tasks:** 2
- **Files modified:** 4

## Accomplishments

- Replaced empty peer-policy decision slices in `ManagedPeerNetwork::peer_policy_info()` with runtime state slices.
- Added managed record methods for ban, unban, discouragement, and misbehavior decisions.
- Replaced aggregate reconnect suppression with `remote_addr.ip()` and `now_unix_seconds` scoped runtime queries.
- Added deterministic node/RPC tests for active bans, unbans, protected misbehavior, matching reconnect suppression, non-matching reconnect allowance, and stable Phase 94 reconnect labels.

## Task Commits

Deferred until the wrapper-level clean verification gate. The user-invoked wrapper requires no commit or push before final verification is clean.

## Files Created/Modified

- `packages/open-bitcoin-node/src/network.rs` - Adds managed peer-policy record/query methods and live runtime projection.
- `packages/open-bitcoin-node/src/network/tests.rs` - Adds managed policy projection tests for ban, unban, and protected misbehavior.
- `packages/open-bitcoin-rpc/src/context/network.rs` - Uses scoped reconnect suppression and test-only peer-policy seed helpers.
- `packages/open-bitcoin-rpc/src/inbound_listener/tests.rs` - Adds scoped reconnect suppression and listener evidence tests.

## Decisions Made

- Keep `ManagedPeerPolicyInfo::from_policy_decisions` as the shared projection function, but feed it real runtime slices from the peer manager.
- Keep RPC seed helpers test-only so production context behavior is only the scoped reconnect lookup.
- Preserve existing Phase 94 labels `reconnect_suppressed_banned` and `reconnect_suppressed_discouraged`.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 2 - Missing Critical] Gate RPC peer-policy seed helpers to tests**
- **Found during:** Task 2 (Use scoped policy state for reconnect suppression)
- **Issue:** Test seeding helpers produced dead-code warnings when compiled into the library target.
- **Fix:** Added `#[cfg(test)]` to the RPC context seed helpers and imports.
- **Files modified:** `packages/open-bitcoin-rpc/src/context/network.rs`
- **Verification:** `cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --all-targets --all-features -- -D warnings`
- **Committed in:** Deferred until final wrapper gate.

**Total deviations:** 1 auto-fixed (Rule 2).
**Impact on plan:** The production runtime behavior stayed narrower while tests can still seed policy state deterministically.

## Issues Encountered

None.

## Verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node managed_peer_policy_info -- --nocapture`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc reconnect_suppression -- --nocapture`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc inbound_listener --no-fail-fast`
- `cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-targets --all-features -- -D warnings`
- `cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --all-targets --all-features -- -D warnings`

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Ready for Plan 96-03. Shared status, structured log, CLI, and support-bundle evidence can now consume actual managed peer-policy runtime state.

---
*Phase: 96-peer-policy-runtime-bridge*
*Completed: 2026-06-28*
