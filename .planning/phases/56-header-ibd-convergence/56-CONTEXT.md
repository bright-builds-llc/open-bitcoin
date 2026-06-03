---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 56-2026-06-03T12-44-57
generated_at: 2026-06-03T12:55:00.000Z
---

# Phase 56: Header IBD Convergence - Context

**Gathered:** 2026-06-03
**Status:** Ready for planning
**Mode:** Yolo

<domain>

## Phase Boundary

Phase 56 proves validated header sync can advance through multiple
public-mainnet-like batches, stop on an explicit convergence boundary, persist
progress durably, and surface the first observed header-height increase through
fresh daemon status evidence.

This phase does not claim unattended public-mainnet full sync, inbound serving,
relay behavior, or block connection progress. Public-network smoke remains
operator opt-in and outside `bash scripts/verify.sh`.

</domain>

<decisions>

## Implementation Decisions

### Header Convergence Runtime Contract

- **D-01:** Continue using accepted headers as useful header progress. Raw peer
  activity, handshakes, and failed validation do not advance progress.
- **D-02:** Add an optional runtime header target so bounded smoke runs can stop
  once `best_header_height` reaches a configured target.
- **D-03:** When repeated rounds produce no new header or block height, record a
  typed no-progress diagnosis in the final summary and durable status signal.
- **D-04:** Preserve existing `max_rounds` as the deterministic timeout-style
  convergence bound.

### Live Smoke Evidence

- **D-05:** The live smoke report should record the first observed header-height
  increase with endpoint/source/timestamp evidence when status snapshots show
  header progress.
- **D-06:** The before/after proof should use fresh `openbitcoinsyncstatus`
  snapshots captured during the daemon run, then correlate the peer endpoint
  from final durable peer telemetry.
- **D-07:** If no progress is observed, keep the existing typed no-progress
  cause path and next-action guidance.

### Scope Controls

- **D-08:** Do not add live public-network commands to default verification.
- **D-09:** Do not expand into block download/connect success; Phase 57 owns
  block progress evidence.

</decisions>

<canonical_refs>

## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase Scope

- `.planning/ROADMAP.md` - Phase 56 goal and success criteria.
- `.planning/REQUIREMENTS.md` - HDR-01 through HDR-04.
- `.planning/PROJECT.md` - v1.4 public-mainnet progress boundaries.
- `.planning/STATE.md` - Phase 55 completion context and next-phase readiness.

### Prior Phase Evidence

- `.planning/phases/55-outbound-handshake-compatibility-fixes/55-CONTEXT.md` -
  Handshake compatibility decisions Phase 56 builds on.
- `.planning/phases/55-outbound-handshake-compatibility-fixes/55-01-SUMMARY.md`
  - Completed connected-handshake and typed-failure behavior.

### Implementation Surfaces

- `packages/open-bitcoin-node/src/sync.rs` - Durable sync rounds, idle
  detection, peer outcomes, and durable state persistence.
- `packages/open-bitcoin-node/src/sync/types.rs` - Runtime config and summary
  telemetry.
- `packages/open-bitcoin-node/src/sync/types/summary.rs` - Status projection,
  progress signals, logs, and metrics.
- `packages/open-bitcoin-node/src/sync/tests.rs` - Deterministic scripted peer
  tests for headers, invalid data, persistence, and status projection.
- `scripts/run-live-mainnet-smoke.ts` - Opt-in live smoke command and report
  schema.
- `docs/operator/runtime-guide.md` - Operator-facing live-smoke workflow.
- `docs/parity/catalog/p2p.md` - P2P parity catalog.

### Baseline Anchors

- `packages/bitcoin-knots/src/net_processing.cpp` - Header processing and peer
  message flow anchor.
- `packages/bitcoin-knots/src/headerssync.cpp` - Header sync behavior anchor.
- `packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py` -
  Initial multi-batch header sync behavior anchor.

</canonical_refs>

<code_context>

## Existing Code Insights

### Reusable Assets

- `DurableSyncRuntime::sync_until_idle_with_resolver` already loops
  `sync_once_with_resolver` while the `(header_height, block_height)` marker
  advances.
- `sync_connected_peer` already handles multiple `headers` messages in one peer
  session and only credits header contribution after managed network validation
  succeeds.
- `FjallNodeStore::save_header_entries` and runtime open already seed the
  in-memory network state from durable headers.
- `ScriptedTransport`, `headers_script`, and existing header tests can model
  public-mainnet-like multi-batch progress without network access.
- The live smoke script already polls `openbitcoinsyncstatus` during daemon
  execution and reads final durable peer telemetry from `open-bitcoin sync
  status`.

### Established Patterns

- Pure validation remains in domain/network crates; durable sync orchestration,
  logs, and smoke reports live in shell layers.
- Status and report schema fields are additive when possible.
- Tests use Arrange, Act, Assert comments for non-trivial cases.

### Integration Points

- Add a small convergence diagnosis type to sync summary/config without changing
  peer protocol parsing.
- Update `sync_until_idle_with_resolver` to set diagnosis when it stops due to
  header target, no progress, or round limit.
- Add a smoke-report helper that derives first header progress from snapshots
  and final peer telemetry.

</code_context>

<specifics>

## Specific Ideas

- Add `maybe_target_header_height: Option<u64>` to `SyncRuntimeConfig`.
- Add `SyncStopReason` to `SyncRunSummary`, with target-reached, no-progress,
  and max-rounds variants.
- Add deterministic tests for target stop, no-progress diagnosis, invalid
  header no-credit behavior, and restart/status visibility.
- Add `firstHeaderProgress` to the live smoke result object with before/after
  status snapshots and peer endpoint/source details.

</specifics>

<deferred>

## Deferred Ideas

- Public block progress remains Phase 57.
- Explicit restart/resume operator proof remains Phase 58.
- Support bundle, release boundaries, and threat-model closeout remain Phase 59.

</deferred>

---

*Phase: 56-header-ibd-convergence*
*Context gathered: 2026-06-03*
