---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 97-2026-06-28T16-11-36
generated_at: 2026-06-28T16:14:18.242Z
---

# Phase 97: Inbound Metrics Sample Production - Context

**Gathered:** 2026-06-28
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Produce persisted low-cardinality metric samples from already-existing inbound admission, permission, peer-policy, and resource-governance counters. This phase closes the counter-to-retained-metrics flow so status, dashboard history, support evidence, and deterministic verification can prove real inbound runtime outcomes without adding public-network exposure, new relay behavior, dynamic labels, peer identifiers, endpoint labels, or new operator-facing product scope.

</domain>

<decisions>
## Implementation Decisions

### Metric Source
- **D-01:** Treat `OpenBitcoinStatusSnapshot.peers.inbound` and `InboundPeerServingStatus` as the canonical source for inbound metric sample production.
- **D-02:** Do not create duplicate counter state for metrics. Runtime metric samples should be derived from the shared inbound status projection that already receives admission, permission, peer-policy, and resource-governance updates.
- **D-03:** When inbound status is unavailable, emit no inbound metric samples rather than manufacturing zero-valued availability evidence.

### Metric Mapping
- **D-04:** Map each existing fixed inbound `MetricKind` variant to one numeric aggregate from `InboundPeerServingStatus`.
- **D-05:** Preserve bounded low-cardinality metrics only: sample kind, numeric value, and timestamp. Do not attach peer ids, endpoints, raw permission class strings, ban scopes, reasons, labels, addresses, or other runtime-created dimensions.
- **D-06:** Keep the pure mapping testable without storage, network, or runtime side effects.

### Runtime Persistence
- **D-07:** Extend the existing metrics append path instead of creating a new history store. Inbound samples should flow through `FjallNodeStore::append_metric_samples` and the existing retention policy.
- **D-08:** Persist inbound samples alongside the existing sync samples during runtime progress collection so dashboard/status history sees one retained metrics snapshot.
- **D-09:** The implementation should be public-network-free and exercise local synthetic or loopback evidence only.

### Operator Evidence
- **D-10:** The dashboard already has labels for registered inbound metric kinds; this phase should prove retained inbound samples can be rendered by that path instead of adding new dashboard UI.
- **D-11:** Status/support evidence should remain redacted and aggregate. Support-bundle and runtime-guide changes should document the closed flow only when needed to make `INB-05` and `DOS-04` auditable.

### Verification
- **D-12:** Add a deterministic checker for Phase 97 that proves inbound metric samples are produced from real runtime/status evidence, use existing fixed `MetricKind`s, are retained through the existing metrics history path, and remain public-network-free.
- **D-13:** Default verification must include the Phase 97 checker through `bash scripts/verify.sh`.

### Folded Todos
No pending todos matched this phase.

### the agent's Discretion
The agent may choose the exact helper/module placement, test fixture construction, and checker implementation details, provided the implementation keeps the mapping pure, uses existing runtime/status contracts, and preserves the public-network-free verification boundary.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase Scope And Gap Evidence
- `.planning/ROADMAP.md` - Phase 97 goal, planned plan split, dependencies, and success criteria.
- `.planning/REQUIREMENTS.md` - `INB-05` and `DOS-04` requirement wording and traceability.
- `.planning/v1.9-MILESTONE-AUDIT.md` - `INT-02-inbound-metric-sample-production` and `FLOW-02-inbound-counters-to-metrics` gap evidence.

### Metrics And Runtime Persistence
- `packages/open-bitcoin-node/src/metrics.rs` - `MetricKind`, `MetricSample`, retention policy, and append/prune helper.
- `packages/open-bitcoin-node/src/sync/runtime_state.rs` - Current runtime metrics persistence path.
- `packages/open-bitcoin-node/src/sync/types/summary.rs` - Existing sync `metric_samples` producer pattern.
- `packages/open-bitcoin-node/src/storage/fjall_store.rs` - Existing metrics history append and load path.

### Inbound Status Sources
- `packages/open-bitcoin-node/src/status/inbound.rs` - Shared inbound status counters and latest bounded event contracts.
- `packages/open-bitcoin-node/src/network/inbound.rs` - Managed inbound admission, permission, peer-policy, and resource-governance counter producers.
- `packages/open-bitcoin-node/src/network.rs` - Runtime network integration points for inbound and resource-governance state.

### Operator Surfaces And Verification
- `packages/open-bitcoin-cli/src/operator/dashboard/model.rs` - Dashboard metric label and retained sample rendering path.
- `docs/architecture/operator-observability.md` - Low-cardinality inbound, permission, peer-policy, and resource-governance observability constraints.
- `docs/operator/runtime-guide.md` - Operator UAT command forms and inbound/resource-governance evidence guidance.
- `scripts/verify.sh` - Repo-native verification contract.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `MetricKind::Inbound*` variants already define the complete fixed metric vocabulary for admission, permission, peer-policy, and resource-governance counters.
- `MetricSample::new` and `append_and_prune_metric_samples` already provide the retained numeric sample contract.
- `FjallNodeStore::append_metric_samples` already appends and prunes retained metrics by series.
- `OpenBitcoinStatusSnapshot.peers.inbound` already carries the aggregate inbound counters needed for sample production.

### Established Patterns
- Sync metrics are produced by a pure `metric_samples(timestamp)` helper and persisted through `DurableSyncRuntime::persist_metrics`.
- Inbound observability fields are aggregate status values with unavailable wrappers for absent evidence.
- Existing tests use Arrange, Act, Assert comments and deterministic local fixtures for network/runtime behavior.

### Integration Points
- Add the pure inbound mapping near the metrics/status boundary so it can be tested independently.
- Extend `DurableSyncRuntime::persist_metrics` or its nearby helpers so runtime persistence appends sync and inbound samples together.
- Add checker coverage under `scripts/` and wire it into `scripts/verify.sh` following the existing Phase 90-96 checker pattern.

</code_context>

<specifics>
## Specific Ideas

Use the existing fixed metric kinds rather than adding series. The implementation should make the closed flow explicit:

`InboundPeerServingStatus` counters -> bounded `MetricSample`s -> `FjallNodeStore::append_metric_samples` -> dashboard/status retained history.

</specifics>

<deferred>
## Deferred Ideas

Traceability reconciliation for stale requirement ownership remains Phase 98. Public-network listener exposure, relay behavior, production node readiness, new dashboard UX, and dynamic metric dimensions remain out of scope.

</deferred>

---

*Phase: 97-inbound-metrics-sample-production*
*Context gathered: 2026-06-28*
