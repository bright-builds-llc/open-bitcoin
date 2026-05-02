---
phase: 37
phase_name: "Header-First Mainnet Sync Integration"
plan_id: "37-01"
generated_by: gsd-execute-phase
lifecycle_mode: yolo
phase_lifecycle_id: "37-2026-05-02T00-08-13"
generated_at: "2026-05-02T01:02:05.851Z"
status: completed
---

# Summary 37-01: Durable Header Continuation And Validation

## Completed

- Split durable sync’s header path away from eager block download so Phase 37 stays header-first while still reusing the Phase 36 peer/runtime seams.
- Added contextual header validation on the sync-runtime path, including restart-safe ancestry lookup, median-time-past calculation, retarget-anchor recovery, and typed invalid-header errors.
- Added deterministic multi-batch header continuation, restart/fork regression coverage, and helper tests for the new header-store ancestry APIs and invalid-data display paths.
- Extracted the new header-sync validation helpers into a dedicated [`network/header_sync.rs`](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/network/header_sync.rs) child module to keep [`network.rs`](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/network.rs) under the repo’s production-file size limit.
- Hardened the CLI test server harness so the repo-native verification path no longer flakes on a nonblocking accepted socket.
- Refreshed README and repo-managed parity/LOC artifacts to match the new header-first Phase 37 behavior.

## Tests Added

- Header-first sync continues header batching without requesting blocks when peers advertise more work.
- Invalid contextual headers become typed `InvalidData` peer outcomes.
- Competing header branches can take over after restart when they extend farther.
- `HeaderStore::entry`, `ancestor_at_height`, and `median_time_past` cover present, missing, and corrupted-parent paths.
- CLI RPC error handling test is stable across repeated runs after forcing the accepted test socket back to blocking mode.

## Residual Risks

- Phase 37 still does not request, validate, persist, or connect blocks as part of normal daemon sync; Phase 38 remains the owner of block download/connect and partial-block restart recovery.
- Header-chain selection still relies on the current header-store work model; later phases may need deeper chainwork fidelity if mainnet evidence shows that to be necessary for parity.
