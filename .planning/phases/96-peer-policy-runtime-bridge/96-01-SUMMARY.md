---
phase: 96-peer-policy-runtime-bridge
plan: 01
subsystem: network
tags: [rust, peer-policy, bans, misbehavior, resource-governance]
requires:
  - phase: 93-eviction-ban-and-misbehavior-policy
    provides: Pure eviction, ban, and misbehavior policy types.
  - phase: 94-dos-and-resource-governance
    provides: ReconnectSuppressionInput for resource-governance decisions.
provides:
  - Pure PeerPolicyRuntimeState with scoped ban, discourage, unban, and misbehavior evidence.
  - PeerManager ownership of peer-policy runtime state.
affects: [peer-policy-runtime-bridge, managed-network, rpc-inbound-listener]
tech-stack:
  added: []
  patterns: [pure-runtime-policy-state, bounded-decision-evidence, injected-time-expiry]
key-files:
  created: []
  modified:
    - packages/open-bitcoin-network/src/peer_policy.rs
    - packages/open-bitcoin-network/src/peer_policy/tests.rs
    - packages/open-bitcoin-network/src/peer.rs
    - packages/open-bitcoin-network/src/lib.rs
key-decisions:
  - "Keep reconnect suppression scoped by BanScope::matches_ip instead of deriving it from aggregate active-ban counts."
  - "Store discouraged reconnect state separately from active ban decisions."
  - "Expose PeerPolicyRuntimeState through PeerManager while keeping it pure open-bitcoin-network data."
patterns-established:
  - "Peer policy runtime evidence uses bounded slices capped by MAX_PEER_POLICY_RUNTIME_DECISIONS."
  - "Expiry-sensitive peer-policy queries accept injected now_unix_seconds values."
requirements-completed: [EVICT-03, EVICT-04, DOS-03]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 96-2026-06-28T02-38-04
generated_at: 2026-06-28T03:48:39Z
duration: 9min
completed: 2026-06-28
---

# Phase 96 Plan 01: Pure Peer-Policy Runtime State Summary

**Scoped pure peer-policy state now records bounded ban, unban, discourage, and misbehavior evidence for managed runtime consumers.**

## Performance

- **Duration:** 9min
- **Started:** 2026-06-28T03:39:30Z
- **Completed:** 2026-06-28T03:48:39Z
- **Tasks:** 2
- **Files modified:** 4

## Accomplishments

- Added `PeerPolicyRuntimeState` with bounded runtime evidence for ban, unban, and misbehavior decisions.
- Added exact-address, IPv4-subnet, and IPv6-subnet matching through `BanScope::matches_ip`.
- Added scoped reconnect suppression checks that distinguish active bans from discouragement.
- Added `PeerManager` ownership and accessors for the pure runtime policy state.

## Task Commits

Deferred until the wrapper-level clean verification gate. The user-invoked wrapper requires no commit or push before final verification is clean.

## Files Created/Modified

- `packages/open-bitcoin-network/src/peer_policy.rs` - Adds scoped matching and pure runtime policy state.
- `packages/open-bitcoin-network/src/peer_policy/tests.rs` - Adds runtime-state tests for scoped bans, subnet bans, expiry, discouragement, unban, and protected misbehavior evidence.
- `packages/open-bitcoin-network/src/peer.rs` - Adds `PeerManager` ownership and accessors for runtime policy state.
- `packages/open-bitcoin-network/src/lib.rs` - Exports runtime policy state for managed runtime consumers.

## Decisions Made

- Reconnect suppression is derived from scope-matching runtime state, not aggregate policy counters.
- Discouraged reconnects are stored separately so they do not inflate active ban evidence.
- Runtime policy state remains pure network-core data with no socket, RPC, storage, metric, or support-rendering dependencies.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Export runtime policy state from crate root**
- **Found during:** Task 2 (Expose runtime state through PeerManager)
- **Issue:** Downstream Plan 96-02 consumers need a public `open_bitcoin_network::PeerPolicyRuntimeState` surface, but the plan's file list did not include `packages/open-bitcoin-network/src/lib.rs`.
- **Fix:** Exported `PeerPolicyRuntimeState` and `MAX_PEER_POLICY_RUNTIME_DECISIONS` from the crate root.
- **Files modified:** `packages/open-bitcoin-network/src/lib.rs`
- **Verification:** `cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-network --all-targets --all-features -- -D warnings`
- **Committed in:** Deferred until final wrapper gate.

**Total deviations:** 1 auto-fixed (Rule 3).
**Impact on plan:** The additional crate-root export is required for the next managed-runtime plan and does not broaden behavior.

## Issues Encountered

None.

## Verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network peer_policy::tests::runtime_state -- --nocapture`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network peer_policy -- --nocapture`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network peer --no-fail-fast`
- `cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-network --all-targets --all-features -- -D warnings`

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Ready for Plan 96-02. Managed node and RPC runtime code can now project real scoped ban, unban, misbehavior, and reconnect suppression state from `PeerManager`.

---
*Phase: 96-peer-policy-runtime-bridge*
*Completed: 2026-06-28*
