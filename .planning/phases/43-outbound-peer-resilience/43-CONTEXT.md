---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 43-2026-05-24T20-38-15
generated_at: 2026-05-24T20:41:58Z
---

# Phase 43: Outbound Peer Resilience - Context

**Gathered:** 2026-05-24
**Status:** Ready for planning
**Mode:** YOLO recommended

<domain>
## Phase Boundary

Phase 43 hardens the already opt-in daemon sync path so peer failures stay
operator-visible and do not stop the runtime from trying useful replacement
peers. The phase owns bounded outbound target reporting, retry/backoff/stall
reason visibility, peer replacement after unhealthy outcomes, and mixed-failure
runtime survival.

This phase does not own per-peer contribution attribution, long-run resource
bound expansion, restart/resume proof, support bundles, final public-mainnet
progress evidence, transaction relay, inbound serving, packaging, or any
unattended production-node claim.
</domain>

<decisions>
## Implementation Decisions

### Runtime Resilience

- **D-01:** Keep Phase 43 inside `DurableSyncRuntime`; the live-smoke runner
  should benefit through durable peer telemetry rather than duplicating sync
  policy in TypeScript.
- **D-02:** Preserve the configured `target_outbound_peers` as the operator
  target in sync status. Current connected peers and target peers must remain
  distinct so a low peer count is diagnosable.
- **D-03:** Treat peers skipped due retry backoff as first-class peer outcomes
  with a stable state and failure reason instead of silently omitting them.
- **D-04:** Keep backoff bounded by the existing `retry_backoff_ms` and
  `consecutive_failures` policy. Do not introduce a new peer scoring system in
  this phase.

### Operator Visibility

- **D-05:** Expose retry-backoff skips through existing `PeerSyncOutcome`,
  `PeerTelemetry`, health signals, and structured logs. Do not add a second
  telemetry channel.
- **D-06:** Use stable enum/string values for new states and reasons so live
  smoke, status, and future support evidence can match them without parsing
  prose.
- **D-07:** When all resolved peers are waiting in backoff, the sync phase should
  say the runtime is waiting for peers instead of implying useful sync progress.

### Verification Posture

- **D-08:** Default verification stays deterministic and public-network-free.
  Prove peer replacement and mixed failures with scripted transports and
  resolvers.
- **D-09:** Mixed failure tests should include at least connection failure,
  invalid peer data, and a replacement peer that still succeeds, proving the
  runtime returns a summary and persists coherent state.

### Agent Discretion

The planner may decide exact enum names and helper boundaries as long as the
operator-facing values are stable, deterministic tests cover them, and the
implementation stays scoped to the sync runtime/status projection.
</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase Scope

- `.planning/ROADMAP.md` - Phase 43 goal, dependencies, requirements, and
  success criteria.
- `.planning/REQUIREMENTS.md` - PEER-02 and PEER-04 requirement text.
- `.planning/STATE.md` - Current v1.3 state after Phase 42.
- `.planning/phases/42-live-smoke-entry-and-network-preflight/42-CONTEXT.md`
  - Deferred runtime peer resilience boundary from Phase 42.

### Sync Runtime

- `packages/open-bitcoin-node/src/sync.rs` - Candidate peer loop, target
  outbound peer handling, retry loop, outcome recording, and durable state
  persistence.
- `packages/open-bitcoin-node/src/sync/runtime_state.rs` - Peer resolution,
  backoff helpers, durable status projection, metrics, and structured logging.
- `packages/open-bitcoin-node/src/sync/types.rs` - Runtime config, peer state,
  failure reasons, summary, sync status, and metric/log projection entrypoints.
- `packages/open-bitcoin-node/src/sync/types/projection.rs` - Peer telemetry,
  structured log records, and phase naming.
- `packages/open-bitcoin-node/src/sync/progress.rs` - Peer progress conversion
  and stalled-peer health signal.
- `packages/open-bitcoin-node/src/sync/tests.rs` - Scripted transport/resolver
  coverage for stalls, target budgets, retries, invalid data, and status
  projection.

### Operator Surfaces

- `docs/operator/runtime-guide.md` - Existing runtime sync and live-smoke
  operator guidance.
- `scripts/run-live-mainnet-smoke.ts` - Consumer of durable peer telemetry in
  opt-in live-mainnet reports.
</canonical_refs>

<code_context>
## Existing Code Insights

### Current Strengths

- `DurableSyncRuntime::sync_once_with_resolver` already continues past stalled
  or failed peers and stops only after successful connected slots reach
  `target_outbound_peers`.
- `sync_peer_with_retries` already preserves the final attempt count for a peer
  up to `max_peer_retries + 1`.
- `record_outcome` already marks failed and stalled peers in backoff, records
  health signals, and persists summaries, metrics, logs, and durable sync state.
- Existing tests prove stall rotation, configured outbound budget stop, connect
  retry counts, address resolution failures, and invalid header rejection.

### Current Gaps

- A peer skipped because its retry backoff has not expired is currently
  invisible to `SyncRunSummary`, peer telemetry, health signals, and structured
  logs.
- `SyncRunSummary::sync_status` reports `target_outbound_peers` as
  `connected_peers`, which hides a configured target greater than the current
  connected count outside the durable-state override path.
- If every resolved peer is waiting in backoff, the direct summary status phase
  still reads as `steady_state`; operators need a distinct waiting state.
- Mixed-failure behavior has individual tests but lacks one regression that
  proves connection failure, invalid data, and replacement success can occur in
  one run without corrupting durable progress or exiting unexpectedly.
</code_context>

<specifics>
## Specific Ideas

- Add a `Waiting` peer state and `RetryBackoff` failure reason, then record a
  waiting outcome when `peer_backoff.next_attempt_unix_seconds` is still in the
  future.
- Keep waiting peers out of `attempted_peers`, `connected_peers`, and
  `failed_peers` because no new outbound connection attempt occurs.
- Include `consecutive_failures` and `next_attempt_unix_seconds` in a short
  operator-visible error field without exposing private datadir or credential
  data.
- Store `target_outbound_peers` on `SyncRunSummary` so direct status projection
  and durable status projection agree about configured bounds.
</specifics>

<deferred>
## Deferred Ideas

- Per-peer header/block contribution and idle-peer usefulness attribution belong
  to Phase 44.
- Long-run in-flight resource bounds, retention bounds, and single-writer store
  coordination belong to Phase 45.
- Restart/resume recovery proof and invalid block/header attribution beyond this
  mixed-failure regression belong to Phase 46 and Phase 50.
</deferred>
