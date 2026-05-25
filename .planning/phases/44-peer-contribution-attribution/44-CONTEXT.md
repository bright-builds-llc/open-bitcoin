---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 44-2026-05-25T16-03-34
generated_at: 2026-05-25T16:03:34.725Z
---

# Phase 44: Peer Contribution Attribution - Context

**Gathered:** 2026-05-25
**Status:** Ready for planning
**Mode:** YOLO recommended

<domain>
## Phase Boundary

Phase 44 makes sync progress attribution truthful at the peer level. The phase
owns per-peer header and block contribution semantics in durable sync telemetry,
status projection, live smoke evidence, and support-facing reports. A peer that
connects but contributes no validated headers or blocks must remain visible as
active, idle, stalled, waiting, or failed without being credited for useful
progress.

This phase does not own a new peer scoring or eviction policy, long-run resource
bounds, restart/recovery proof, inbound serving, transaction relay, or final
public-mainnet progress evidence.
</domain>

<decisions>
## Implementation Decisions

### Attribution Semantics

- **D-01:** Keep contribution attribution in `DurableSyncRuntime` and the
  existing sync/status projection path. Do not add a second peer telemetry
  channel or a standalone peer reputation system.
- **D-02:** Treat `messages_processed` and `maybe_last_activity_unix_seconds`
  as activity evidence, not useful sync contribution.
- **D-03:** Count header contribution only after the runtime accepts the headers
  through the sync validation path. Invalid headers may update last activity and
  failure reason, but they must not increase useful header progress.
- **D-04:** Count block contribution only after the runtime accepts the block
  through the block sync path and preserves it for connection/reconciliation.
  Invalid or failed block messages may update last activity and failure reason,
  but they must not increase useful block progress.

### Peer State And Failure Separation

- **D-05:** Idle or stalled peers keep zero header/block contribution while
  remaining visible in peer telemetry with their connection state, attempts,
  capabilities when known, and last activity when available.
- **D-06:** Failing peers must retain last activity and failure reason separately
  from contribution counters. Failure handling should not collapse a peer that
  sent invalid data into an empty, never-active outcome.
- **D-07:** Keep Phase 43 waiting/backoff behavior intact: `waiting` peers with
  `retry_backoff` stay non-attributed and continue to support replacement-peer
  diagnosis.

### Operator Evidence Surfaces

- **D-08:** Preserve existing status JSON compatibility where practical, but make
  the meaning of per-peer `headers_received` and `blocks_received` counters
  validation-gated in docs and tests.
- **D-09:** Extend live-smoke evidence so final reports include per-peer runtime
  contribution rows from durable peer telemetry, not only endpoint reachability.
- **D-10:** Keep default verification deterministic and public-network-free.
  Live-mainnet behavior remains opt-in evidence and must not be required by
  `bash scripts/verify.sh`.

### Agent Discretion

The planner may choose exact helper names and whether to add additive report
fields, as long as existing durable status consumers keep working, contribution
counters are validation-gated, and deterministic tests prove idle, failed, and
contributing peers are distinguishable.
</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase Scope

- `.planning/ROADMAP.md` - Phase 44 goal, dependency, requirement mapping, and
  success criteria.
- `.planning/REQUIREMENTS.md` - PEER-03 status and v1.3 requirement inventory.
- `.planning/STATE.md` - Current v1.3 state after Phase 43.
- `.planning/phases/43-outbound-peer-resilience/43-CONTEXT.md` - Locked
  resilience/backoff decisions and explicit deferral of contribution
  attribution to Phase 44.
- `.planning/phases/43-outbound-peer-resilience/43-01-SUMMARY.md` - Phase 43
  implementation summary and residual risks.

### Sync Runtime

- `packages/open-bitcoin-node/src/sync.rs` - Sync loop, per-peer progress
  handling, outcome recording, and durable summary persistence.
- `packages/open-bitcoin-node/src/sync/progress.rs` - `PeerProgress`,
  `PeerFailure`, message activity accounting, and outcome conversion.
- `packages/open-bitcoin-node/src/sync/types.rs` - `PeerContribution`,
  `PeerSyncOutcome`, `SyncRunSummary`, status/metric/log projection entrypoints,
  and public sync runtime types.
- `packages/open-bitcoin-node/src/sync/types/projection.rs` - Peer telemetry,
  sync phase naming, and structured log projection.
- `packages/open-bitcoin-node/src/sync/runtime_state.rs` - Backoff, peer
  resolution, peer capabilities, durable status projection, and health signals.
- `packages/open-bitcoin-node/src/sync/block_reconcile.rs` - Missing-block
  request, inflight release, and best-chain reconciliation behavior.
- `packages/open-bitcoin-node/src/network.rs` - Header/block sync message
  validation and managed-network actions.
- `packages/open-bitcoin-node/src/network/header_sync.rs` - Contextual header
  validation used by runtime header sync.
- `packages/open-bitcoin-node/src/sync/tests.rs` - Deterministic sync runtime
  regression coverage.

### Operator And Evidence Surfaces

- `packages/open-bitcoin-node/src/status.rs` - Serializable `PeerTelemetry` and
  durable status shapes.
- `scripts/run-live-mainnet-smoke.ts` - Live-smoke JSON and Markdown evidence
  generation from durable peer telemetry.
- `scripts/test-run-live-mainnet-smoke.sh` - Deterministic script coverage for
  live-smoke report generation.
- `docs/operator/runtime-guide.md` - Operator interpretation guidance for sync
  status, peer telemetry, and live-smoke reports.
</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets

- `PeerContribution` already carries `messages_processed`, `headers_received`,
  and `blocks_received` per peer, and `PeerTelemetry` already projects per-peer
  header/block counters into durable status.
- `PeerProgress` already tracks messages, header messages, block messages,
  capabilities, and last activity for a connected peer.
- `PeerSyncState::Waiting` and `PeerFailureReason::RetryBackoff` from Phase 43
  already make retry-backoff peers visible without counting them as connected
  progress.
- The live-smoke script already parses `recent_peers` from durable status, but
  currently uses that data only to derive endpoint outcomes.

### Established Patterns

- Sync runtime tests use `ScriptedTransport` and `ScriptedResolver` for
  deterministic peer, message, failure, and replacement scenarios.
- Operator-facing status uses `FieldAvailability` rather than inventing default
  values for unavailable peer data.
- Runtime docs prefer clear interpretation guidance over bare field lists.

### Current Gaps

- `PeerProgress::record_message` currently counts header and block messages
  before the managed sync path proves they were accepted.
- If a connected peer sends invalid data, the failure path can lose the peer's
  message count and last-activity evidence because `record_outcome` creates a
  fresh failed outcome with zero activity.
- Live-smoke reports preserve endpoint outcomes and snapshots but do not yet
  include per-peer contribution evidence from runtime telemetry.
</code_context>

<specifics>
## Specific Ideas

- Split peer accounting into activity accounting and contribution accounting:
  record `messages_processed` plus last activity as soon as a message is seen,
  but record useful header/block contribution only after the relevant sync
  handler succeeds.
- Carry a partial `PeerProgress` snapshot through connected-peer failures so
  invalid-data peers retain last activity and failure reason while keeping zero
  useful contribution.
- Add deterministic tests for: accepted header/block contribution, invalid
  headers not being credited, failed peers retaining last activity, and
  idle/stalled peers reporting zero contribution.
- Add a live-smoke final peer contribution table to JSON/Markdown evidence using
  durable `recent_peers`.
</specifics>

<deferred>
## Deferred Ideas

- Peer scoring, reputation, eviction policy, and long-run peer selection
  heuristics remain outside Phase 44.
- Runtime resource bounds, retention bounds, and single-writer store
  coordination belong to Phase 45.
- Restart/resume invalid-data recovery proof and final public-mainnet progress
  evidence belong to later v1.3 phases.
</deferred>

---

*Phase: 44-peer-contribution-attribution*
*Context gathered: 2026-05-25*
