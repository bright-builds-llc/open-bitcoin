---
phase: 36
phase_name: "Mainnet Peer Discovery and Outbound Lifecycle"
plan_id: "36-01"
generated_by: gsd-execute-phase
lifecycle_mode: yolo
phase_lifecycle_id: "36-2026-05-01T22-57-33"
generated_at: "2026-05-01T23:56:28.365Z"
status: completed
---

# Summary 36-01: Sync Peer Config And Resolver Boundary

## Completed

- Added typed sync peer configuration for manual peers, DNS seed overrides, and bounded outbound target count in the Open Bitcoin JSONC surface.
- Split peer resolution out of the TCP transport into an injected `SyncPeerResolver` with a real system-backed resolver implementation.
- Added deterministic loader tests covering manual-peer parsing, DNS-seed override parsing, target-count parsing, and invalid peer config errors.
- Preserved the Phase 35 `bitcoin.conf` compatibility boundary while allowing Phase 36 peer configuration through `open-bitcoin.jsonc`.

## Tests Added

- JSONC accepts `manual_peers`, `dns_seeds`, and `target_outbound_peers`.
- Runtime config applies manual-peer normalization and DNS-seed overrides.
- Invalid peer config fails deterministically before runtime startup.

## Residual Risks

- This plan only establishes config and resolver boundaries. Peer rotation, runtime lifecycle, and telemetry projection are completed in later Phase 36 plans.
