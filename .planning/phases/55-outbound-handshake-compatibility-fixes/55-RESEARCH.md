---
generated_by: gsd-plan-phase
lifecycle_mode: yolo
phase_lifecycle_id: 55-2026-06-02T22-36-24
generated_at: 2026-06-02T22:38:08.006Z
---

# Phase 55: Outbound Handshake Compatibility Fixes - Research

## Research Complete

Phase 55 can be implemented with a small deterministic runtime change. The
current peer manager already records all four handshake facts needed to
distinguish a completed outbound handshake from a pre-handshake stall:

- `local_version_sent`
- `remote_version_received`
- `local_verack_sent`
- `remote_verack_received`

The durable sync loop currently overwrites the peer outcome to `Stalled`
whenever `SyncPeerSession::receive` returns `None`, even after those handshake
facts are all true. That makes a valid peer that completes handshake and then
waits for the local node's next request look like a failed handshake in daemon
status and live-smoke evidence.

## Implementation Findings

- `DurableSyncRuntime::sync_connected_peer` is the narrow integration point.
  It already owns the loop that decides `Connected`, `Stalled`, and `Failed`.
- `ManagedPeerNetwork::peer_manager()` exposes `PeerManager::peer_state`, so
  no new effectful API is required to inspect handshake completion.
- `ManagedPeerNetwork::process_actions` currently handles
  `PeerAction::Disconnect(_)` by removing the peer and returning success. For
  daemon sync this hides protocol rejection as a later stall. Returning a
  `NetworkError` after disconnect preserves the rejection and lets sync record a
  failed outcome.
- `NetworkError::DuplicateVersion` and `NetworkError::MissingHeaderAncestor`
  already exist and can represent peer-manager disconnect reasons.
- `SyncRuntimeError::InvalidMagic` already covers wrong-network TCP messages.
  Existing deterministic session tests can continue to use scripted errors for
  wrong-network behavior without public network access.

## Recommended Plan Shape

1. Add a narrow handshake-complete helper in the durable sync runtime.
2. Keep idles before handshake as `Stalled`, but leave completed-handshake idles
   as `Connected`.
3. Convert peer-manager disconnect actions into errors after removing the peer.
4. Add deterministic tests for:
   - manual peer completes handshake and idles as connected;
   - DNS peer completes handshake and idles as connected;
   - pre-handshake idle still stalls and warns;
   - duplicate-version peer fails with no useful progress and replacement peer
     connects;
   - malformed peer data and wrong-network errors remain failed and uncredited;
   - durable status remains coherent for mixed failed and compatible peers.
5. Update `docs/parity/catalog/p2p.md` to replace the Phase 54 known-gap note
   with the new daemon-integrated handshake behavior.

## Risks

- Treating any post-handshake idle as connected must not imply useful header or
  block progress. The summary should only advance `connected_peers`, peer state,
  and message counters.
- Disconnect propagation may affect non-sync managed-network tests. The
  disconnect action should still remove the peer before returning an error.

## Verification Strategy

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node sync --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network --all-features`
- `cargo fmt --all --manifest-path packages/Cargo.toml`
- `cargo clippy --manifest-path packages/Cargo.toml --workspace --all-targets --all-features -- -D warnings`
- `cargo build --manifest-path packages/Cargo.toml --workspace --all-targets --all-features`
- `cargo test --manifest-path packages/Cargo.toml --workspace --all-features`
- `bash scripts/verify.sh`

Public-mainnet live smoke is intentionally not part of default verification.
