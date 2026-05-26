---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 47-2026-05-26T21-36-05
generated_at: 2026-05-26T21:36:05.164Z
---

# Phase 47: Operator Sync Truth Surfaces - Context

**Gathered:** 2026-05-26
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Phase 47 aligns operator-facing sync truth across the shared JSON status model,
human status renderer, terminal dashboard, metrics samples, structured logs,
and RPC `getblockchaininfo`. It does not add hosted dashboards, public-network
verification to `bash scripts/verify.sh`, inbound serving, transaction relay,
or a new sync runtime loop.

</domain>

<decisions>
## Implementation Decisions

### Shared Status Contract

- **D-01:** Keep `OpenBitcoinStatusSnapshot` and nested `SyncStatus` as the
  single source of truth for operator status and dashboard consumers.
- **D-02:** Add an explicit progress signal to `SyncStatus` so JSON status can
  distinguish header progress, block progress, waiting/backoff, peer failures,
  awaiting blocks, and steady state without inferring from renderer text.
- **D-03:** Add `last_successful_progress_unix_seconds` to `SyncStatus`. Compute
  it from the latest peer activity that contributed accepted headers or blocks,
  and preserve the previous durable value when a later status refresh has no
  new successful progress.
- **D-04:** Treat existing `lag` as the estimated sync lag field. Document it as
  estimated lag rather than creating a duplicate field that would drift.

### Surface Alignment

- **D-05:** Human status and dashboard progress text should name the same three
  heights: `headers`, `downloaded_blocks`, and `connected_blocks`. No surface
  may imply full sync until connected chainstate height reaches the selected tip.
- **D-06:** Dashboard rows should expose progress signal, last successful
  progress, recovery guidance, and last error from the shared snapshot instead
  of hiding those fields behind metrics charts.
- **D-07:** RPC `getblockchaininfo.blocks` remains connected-chain height, while
  `headers`, `verificationprogress`, `initialblockdownload`, and warnings are
  derived from the durable sync state. Downloaded-but-unconnected blocks must
  not appear as validated chain height.

### Metrics And Logs

- **D-08:** Metrics should record header height, downloaded block height,
  connected block height, and the existing compatibility sync height so charts
  and support bundles can compare the same progress dimensions as JSON status.
- **D-09:** Structured sync summary logs should include header, downloaded, and
  connected heights plus progress signal and last successful progress timestamp.

### Verification

- **D-10:** Verification remains deterministic and local: status serialization,
  dashboard model/rendering, metrics/log projection, RPC dispatch, and durable
  sync runtime tests. Live mainnet smoke evidence stays out of this phase.
- **D-11:** Operator documentation and UAT examples must use repo-local Cargo
  and Bazel commands rather than relying on an installed `open-bitcoin` alias.

### the agent's Discretion

- The exact enum names and renderer labels may follow established Rust naming
  and existing operator-output style, provided the serialized JSON names are
  stable snake_case and the human/dashboard labels remain concise.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase Scope

- `.planning/ROADMAP.md` - Phase 47 goal, requirements, and success criteria.
- `.planning/REQUIREMENTS.md` - OBS-01 and OBS-02 acceptance requirements.
- `.planning/PROJECT.md` - v1.3 scope boundaries and public-network proof
  constraints.

### Operator Status Contracts

- `docs/architecture/status-snapshot.md` - Shared status snapshot ownership,
  sync progress semantics, and stopped-node behavior.
- `docs/architecture/operator-observability.md` - Bounded metrics and log
  retention contracts.
- `docs/operator/runtime-guide.md` - Operator-facing runtime status and
  repo-local command examples.

### Prior Phase Decisions

- `.planning/phases/45-runtime-resource-bounds-and-store-coordination/45-CONTEXT.md`
  - Resource-bound and store-coordination status decisions.
- `.planning/phases/46-durable-recovery-and-invalid-data-handling/46-CONTEXT.md`
  - Durable progress, invalid-data, and recovery guidance decisions.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets

- `packages/open-bitcoin-node/src/status.rs` owns `SyncStatus`,
  `SyncProgress`, `OpenBitcoinStatusSnapshot`, and serialization tests.
- `packages/open-bitcoin-node/src/sync/types.rs` owns `SyncRunSummary`,
  progress projection, metric sample generation, and structured log records.
- `packages/open-bitcoin-node/src/sync/runtime_state.rs` projects durable sync
  summaries into `DurableSyncState`.
- `packages/open-bitcoin-cli/src/operator/status/render.rs` renders human and
  JSON status from the shared snapshot.
- `packages/open-bitcoin-cli/src/operator/dashboard/model.rs` projects the same
  shared snapshot into dashboard rows and charts.
- `packages/open-bitcoin-rpc/src/dispatch/node.rs` maps durable sync state into
  `getblockchaininfo`.

### Established Patterns

- Optional operator fields use `FieldAvailability<T>` with explicit unavailable
  reasons.
- `SyncProgress.block_height` is a compatibility alias for connected height;
  explicit downloaded and connected fields were added in Phase 46.
- Metrics use stable `MetricKind` enum names and bounded retained samples.
- Structured logs use concise `StructuredLogRecord` messages sourced from sync
  summary projection.

### Integration Points

- Additive status fields require updates to every `SyncStatus` constructor in
  tests, RPC fallback status, durable sync projection, and CLI offline status.
- Renderer parity is checked by status and dashboard unit tests.
- Metric and log parity is checked through sync runtime/projection tests.

</code_context>

<specifics>
## Specific Ideas

- Prefer additive status fields and compatibility aliases over renaming existing
  JSON fields.
- Keep output quiet and operator-focused: compact labels, explicit unavailable
  reasons, no marketing language.

</specifics>

<deferred>
## Deferred Ideas

None - discussion stayed within phase scope.

</deferred>

---

*Phase: 47-operator-sync-truth-surfaces*
*Context gathered: 2026-05-26*
