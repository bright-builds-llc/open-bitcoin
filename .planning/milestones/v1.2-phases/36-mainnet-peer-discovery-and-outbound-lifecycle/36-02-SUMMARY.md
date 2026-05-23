---
phase: 36
phase_name: "Mainnet Peer Discovery and Outbound Lifecycle"
plan_id: "36-02"
generated_by: gsd-execute-phase
lifecycle_mode: yolo
phase_lifecycle_id: "36-2026-05-01T22-57-33"
generated_at: "2026-05-01T23:56:28.365Z"
status: completed
---

# Summary 36-02: Bounded Outbound Peer Pool Runtime

## Completed

- Added a clean disconnect path to `ManagedPeerNetwork` and `PeerManager`, including negotiated remote capability tracking in peer state.
- Refactored `DurableSyncRuntime` to support injected resolver-driven sync rounds, bounded outbound target selection, retry backoff tracking, and rotation away from stalled peers when alternatives are available.
- Kept protocol semantics in the existing peer/message engine while moving resolver and lifecycle policy into the node shell.
- Added hermetic sync tests for resolver failure reporting, outbound-target limiting, alternative-peer rotation after stalls, and capability capture on successful peer negotiation.

## Tests Added

- Resolver failures project typed peer outcomes.
- Stalled peers rotate to alternative resolved endpoints.
- Successful peer-budget satisfaction stops further peer attempts in the round.
- Negotiated peer capabilities are captured in the sync outcome.

## Residual Risks

- Phase 36 still drives peer work sequentially within a sync round rather than holding multiple simultaneous sockets open; that is sufficient for bounded lifecycle truth today but later phases may raise the concurrency bar.
