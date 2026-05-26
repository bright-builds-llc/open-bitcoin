---
generated_by: gsd-discuss-phase
phase: 45
phase_name: Runtime Resource Bounds and Store Coordination
lifecycle_mode: yolo
phase_lifecycle_id: "45-2026-05-26T16-41-34"
generated_at: "2026-05-26T16:41:34Z"
requirements:
  - NODE-01
  - NODE-04
status: accepted
---

# Phase 45 Context

## Objective

Public-network sync must stay within documented runtime bounds and preserve coherent durable-store coordination while operators inspect, pause, resume, stop, and query status.

## Decisions

### D-01: Runtime hardening, not a sync-semantics rewrite

Phase 45 tightens the existing durable sync runtime, configuration, status, and operator-control paths. It must not change Bitcoin protocol semantics, peer message handling, header validation, block validation, or public-network defaults beyond making already-existing limits visible and configurable.

### D-02: Existing runtime config is the source of truth

The resource-bound contract starts from `SyncRuntimeConfig`: outbound target, per-peer message cap, sync rounds, peer retry cap, per-peer in-flight block cap, and global in-flight block cap. Phase 45 should project those values into status/docs and allow documented config overrides rather than inventing a parallel limits model.

### D-03: Bounded persistence remains synchronous and retention-based

Metrics and structured logs already use bounded retention policies. Durable sync writes happen through the store adapter and should remain direct, synchronous writes with no unbounded queue. Phase 45 should document that behavior and test the status/config projections that operators rely on.

### D-04: Store coordination must diagnose mutating offline controls

Offline read-only status may inspect durable metadata. Mutating offline controls (`pause`, `resume`) must not write directly to the durable store when metadata indicates an active daemon may still own sync state and live RPC is unavailable. In that case, the CLI should fail with an explicit second-writer diagnostic.

### D-05: Prefer deterministic local tests

Verification should use local stores, scripted runtime state, config parsing, and renderer tests. Live public-network behavior is out of scope for this phase.

### D-06: Keep the operator surface repo-local

Documentation and UAT commands must use repo-local Cargo/Bazel commands. The installed `open-bitcoin` alias can be mentioned only as a convenience, not as the sole operator path.

## Relevant Existing Surfaces

- `packages/open-bitcoin-node/src/sync/types.rs` owns `SyncRuntimeConfig` and `SyncRunSummary`.
- `packages/open-bitcoin-node/src/sync/runtime_state.rs` projects durable sync summaries into `RuntimeMetadata` and `SyncResourcePressure`.
- `packages/open-bitcoin-node/src/status.rs` owns shared status contracts used by RPC, CLI status, and dashboard views.
- `packages/open-bitcoin-rpc/src/config/open_bitcoin.rs` and `packages/open-bitcoin-rpc/src/config/loader/open_bitcoin_runtime.rs` parse and apply JSONC daemon config.
- `packages/open-bitcoin-cli/src/operator/runtime/support.rs` owns live-RPC-first sync control and offline fallback behavior.
- `docs/operator/runtime-guide.md`, `docs/architecture/status-snapshot.md`, and `docs/architecture/operator-observability.md` are the contributor-facing documentation anchors.

## Acceptance Evidence

- Status exposes configured sync resource bounds alongside observed resource pressure.
- JSONC runtime config supports validated overrides for sync resource bounds.
- Offline mutating sync control refuses to write when durable metadata indicates a potentially live daemon owner.
- Docs describe bounds, retention, and the second-writer diagnostic with copy-pasteable repo-local commands.
- Targeted tests plus the repo verification contract pass.
