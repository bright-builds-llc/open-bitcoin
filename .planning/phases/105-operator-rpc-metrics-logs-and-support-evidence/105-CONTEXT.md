---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 105-2026-07-01T20-32-29
generated_at: 2026-07-01T20:32:29Z
---

# Phase 105: Operator, RPC, Metrics, Logs, and Support Evidence - Context

**Gathered:** 2026-07-01
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Phase 105 projects the Phase 104 relay serving, fanout, local submission, and
rebroadcast-deferred evidence through one sanitized operator evidence contract.
It owns the RPC, CLI status, dashboard, metrics, structured log, support bundle,
and documentation/checker presentation for OBS-01 through OBS-04.

This phase must keep baseline-shaped RPC methods compatible where possible,
while making Open Bitcoin-specific relay and mempool participation truth visible
through the shared Open Bitcoin status surface. It must not add compact block
relay, package relay, bloom/filter serving, public relay defaults, public-network
relay CI, production service operation, production full-node readiness, or
production-funds wallet claims.

</domain>

<decisions>
## Implementation Decisions

### Shared Status Contract

- **D-01:** Add a typed sanitized relay/mempool evidence projection to the
  shared `OpenBitcoinStatusSnapshot` contract, reusing the existing
  `FieldAvailability<T>` pattern for implemented, unavailable, deferred, and
  intentionally different fields instead of duplicating status logic in each
  renderer.
- **D-02:** Keep baseline RPC responses for `sendrawtransaction`,
  `getmempoolinfo`, and `getnetworkinfo` narrow and Knots-shaped where current
  parity requires that. Open Bitcoin-specific relay evidence should live in
  `openbitcoinnetworkstatus` and the shared operator snapshot rather than
  expanding baseline methods with ad hoc Open Bitcoin-only details.
- **D-03:** Classify every exposed relay/mempool field with explicit state:
  implemented, unavailable, deferred, or intentionally different. Do not rely on
  missing JSON fields or prose-only docs to communicate state.
- **D-04:** Preserve Phase 104's distinction between local mempool admission,
  queued relay evidence, served/requested transaction evidence, and public
  propagation. A successful `sendrawtransaction` response may expose the
  submitted txid through the existing RPC response, but operator evidence must
  not claim public propagation was guaranteed.

### CLI And Dashboard Projection

- **D-05:** CLI status and dashboard rows must render relay/mempool state from
  the shared status snapshot, not from separate renderer-local probes or
  duplicate allowlists.
- **D-06:** Human output should be compact and operational: show bounded labels,
  counts, and state classifications such as accepted, rejected, orphaned,
  requested, served, announced, suppressed, evicted, expired, and
  rebroadcast_deferred. Keep txids, wtxids, raw transaction hex, peer ids, peer
  endpoints, raw permission strings, and credential material out of default
  operator output.
- **D-07:** Dashboard charts should remain bounded. If relay metrics are added
  to dashboard candidates, use fixed `MetricKind` variants and existing chart
  replacement patterns instead of dynamic labels or per-peer/per-transaction
  series.
- **D-08:** Older-daemon or missing-method fallbacks should degrade to
  unavailable/deferred status with clear reasons. They should not silently drop
  relay fields or imply the daemon supports relay evidence when it does not.

### Metrics And Structured Logs

- **D-09:** Add fixed low-cardinality metric kinds for the Phase 105 relay
  outcome vocabulary. Required outcome families are accepted, rejected,
  orphaned, requested, served, announced, suppressed, evicted, and expired; add
  rebroadcast_deferred if it is needed to preserve the Phase 104 boundary in
  metrics.
- **D-10:** Metrics must be numeric fixed series such as `relay_accepted_count`
  rather than label-key/value combinations. Do not add metric labels containing
  txids, wtxids, peer endpoints, peer ids, permission strings, credentials,
  request ids, raw config names, or rejection text.
- **D-11:** Structured relay log records should use a dedicated fixed source and
  sanitized key/value messages, following the existing inbound resource
  governance and inbound peer policy log patterns. The log source and fields
  should be allowlisted, not free-form event dumps.
- **D-12:** Structured log sanitizers must reject suspicious hex material,
  socket-address shapes, peer identifiers, endpoint markers, permission strings,
  credentials, cookies, secrets, raw transaction hex, and raw txid/wtxid-like
  material unless a future explicit debug policy is deliberately designed and
  tested.

### Support Bundle Sanitization

- **D-13:** Support bundles should consume the sanitized status snapshot produced
  for bundle generation. Add relay/mempool-specific redaction in
  `support_status_for_bundle` or a closely related helper so support JSON and
  Markdown share the same safe projection.
- **D-14:** Support evidence should include bounded relay/mempool counts, fixed
  labels, classifications, unavailable/deferred reasons, and next-action
  guidance. It should not include raw transaction hex, disallowed txids/wtxids,
  peer endpoints, raw permission strings, credentials, dynamic metric labels, or
  raw structured-log bodies.
- **D-15:** Support Markdown should make the boundary explicit: evidence is for
  local troubleshooting and parity review only. It is not a release validator,
  public-network proof, compact-block proof, production-service proof, production
  full-node readiness proof, or production-funds wallet safety proof.

### Parity, Documentation, And Checkers

- **D-16:** Register a Phase 105 parity surface for OBS-01 through OBS-04 once
  implementation exists, rooted in the shared status contract, RPC dispatch
  tests, CLI/dashboard renderer tests, metrics/log tests, support redaction
  tests, docs, checker, and verifier wiring.
- **D-17:** Add a deterministic Phase 105 checker if docs, parity roots, or
  verifier ordering change. The checker should guard required evidence and
  reject positive claims for public relay defaults, compact block relay, package
  relay, bloom/filter serving, public-network relay CI, production service
  operation, production full-node readiness, and production-funds wallet use.
- **D-18:** Verification stays local and deterministic. Default verification is
  `bash scripts/verify.sh`; do not add public-network, service-manager,
  wall-clock soak, destructive repair, production deployment, or current-tip
  timing gates.

### the agent's Discretion

The planner may choose exact type names, module boundaries, metric-kind names,
and whether the sanitized relay/mempool projection lives directly in
`status.rs` or a child module. Prefer a small typed projection that maps from
existing Phase 104 managed evidence (`LocalRelaySubmissionEvidence`,
`ManagedRelayFanoutInfo`, `ManagedRelayServingInfo`) and can be consumed by
RPC, CLI, dashboard, support, metrics, and logs without duplicating sensitive
field filtering.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Project And v2.0 Scope

- `.planning/PROJECT.md` - Project value, architecture constraints, dependency
  policy, and v2.0 transaction relay boundary.
- `.planning/REQUIREMENTS.md` - OBS-01 through OBS-04 are owned by Phase 105;
  BOUND-* remains Phase 106.
- `.planning/ROADMAP.md` - Phase 105 purpose, scope, success criteria, and
  verification contract.
- `.planning/STATE.md` - Current milestone state, Phase 104 completion notes,
  deterministic verification caveats, and repo-local UAT command reminders.
- `.planning/phases/104-relay-serving-fanout-and-rebroadcast-policy/104-CONTEXT.md`
  - Locked Phase 104 serving, fanout, local submission, and rebroadcast-deferred
  decisions that Phase 105 projects.
- `.planning/phases/104-relay-serving-fanout-and-rebroadcast-policy/104-VERIFICATION.md`
  - Verified Phase 104 evidence and explicit deferral of Phase 105
  operator/RPC/metrics/log/support presentation.

### Shared Status, Metrics, Logs, And Runtime Evidence

- `packages/open-bitcoin-node/src/status.rs` - `FieldAvailability`,
  `MempoolStatus`, `OpenBitcoinStatusSnapshot`, and shared operator snapshot
  structure.
- `packages/open-bitcoin-node/src/metrics.rs` - Fixed `MetricKind` series,
  retention, metric samples, and current inbound metric projection pattern.
- `packages/open-bitcoin-node/src/logging.rs` - Structured log source,
  sanitization, retention, and inbound log record patterns to reuse for relay
  logs.
- `packages/open-bitcoin-node/src/logging/writer.rs` - JSONL append and bounded
  log-status loading pattern.
- `packages/open-bitcoin-node/src/network.rs` - `ManagedMempoolInfo`,
  `ManagedNetworkInfo`, and managed network relay integration points.
- `packages/open-bitcoin-node/src/network/relay_fanout.rs` - Phase 104
  `ManagedRelayFanoutInfo`, `LocalRelaySubmissionEvidence`, and fixed local
  submission labels.
- `packages/open-bitcoin-node/src/network/relay_serving.rs` - Phase 104
  `ManagedRelayServingInfo` and sanitized serve outcome labels.
- `packages/open-bitcoin-node/src/network/mempool_lifecycle.rs` - Mempool
  lifecycle cleanup hooks that affect served/evicted/expired status.
- `packages/open-bitcoin-mempool/src/outcome.rs` - `MempoolOutcome` vocabulary
  used to map local admission and lifecycle outcomes.
- `packages/open-bitcoin-network/src/peer/transaction_relay/serving.rs` - Pure
  serving outcome labels and status classification.
- `packages/open-bitcoin-network/src/peer/transaction_relay/fanout.rs` - Pure
  fanout actions, suppression reasons, queue bounds, and
  `rebroadcast_deferred` evidence.

### RPC, CLI, Dashboard, And Support Surfaces

- `packages/open-bitcoin-rpc/src/method/node.rs` - Baseline-shaped RPC response
  structs and `OpenBitcoinNetworkStatusResponse`.
- `packages/open-bitcoin-rpc/src/dispatch/node.rs` - `sendrawtransaction`,
  `getmempoolinfo`, `getnetworkinfo`, and `openbitcoinnetworkstatus`
  projections.
- `packages/open-bitcoin-rpc/src/context/network.rs` - Managed RPC context
  access to mempool info, network info, metrics, and latest local submission
  evidence.
- `packages/open-bitcoin-rpc/src/context/resource_governance.rs` and
  `packages/open-bitcoin-rpc/src/context/peer_policy.rs` - Existing structured
  log append patterns from managed RPC context.
- `packages/open-bitcoin-cli/src/operator/status.rs` - Live status collection
  from RPC into the shared snapshot and older-daemon fallback behavior.
- `packages/open-bitcoin-cli/src/operator/status/render.rs` - Human and JSON
  status renderer that should consume the shared relay/mempool projection.
- `packages/open-bitcoin-cli/src/operator/dashboard/model.rs` - Dashboard
  projection from `OpenBitcoinStatusSnapshot`.
- `packages/open-bitcoin-cli/src/operator/dashboard/model/metrics.rs` -
  Dashboard fixed metric candidate pattern.
- `packages/open-bitcoin-cli/src/operator/support.rs` - Support bundle assembly
  and sanitized snapshot write path.
- `packages/open-bitcoin-cli/src/operator/support/redaction.rs` - Support
  redaction pattern and inbound evidence sanitizer to extend for relay/mempool
  material.
- `packages/open-bitcoin-cli/src/operator/support/render.rs` - Support Markdown
  rendering that should include bounded relay/mempool evidence.

### Existing Tests And Deterministic Checkers

- `packages/open-bitcoin-rpc/src/dispatch/tests.rs` - Existing RPC dispatch,
  network status, mempool, and `sendrawtransaction` tests.
- `packages/open-bitcoin-node/src/metrics/tests.rs` - Fixed metric-kind and
  low-cardinality metric tests.
- `packages/open-bitcoin-node/src/logging/tests.rs` - Structured-log sanitizer
  and retention tests.
- `packages/open-bitcoin-cli/src/operator/status/tests.rs` - Status collection
  and renderer tests.
- `packages/open-bitcoin-cli/src/operator/dashboard/model/tests.rs` - Dashboard
  model tests.
- `packages/open-bitcoin-cli/src/operator/support/tests.rs` - Support bundle,
  redaction, and Markdown tests.
- `scripts/check-phase104-relay-serving-fanout.ts` - Phase 104 checker style,
  no-claim guardrails, and verifier-order checks to mirror for Phase 105 if
  docs/checker work is needed.
- `scripts/verify.sh` - Repo-native verification contract and checker wiring
  order.

### Parity And Operator Documentation

- `docs/architecture/status-snapshot.md` - Shared status snapshot contract and
  operator-surface expectations.
- `docs/architecture/operator-observability.md` - Operator status, metrics,
  logs, and support evidence architecture.
- `docs/operator/runtime-guide.md` - Operator commands, UAT guidance, and
  no-claim boundaries; use repo-local Cargo/Bazel command forms when adding UAT
  text.
- `docs/parity/catalog/p2p.md` - P2P and transaction relay parity roots,
  Phase 104 boundary, and relay non-claims.
- `docs/parity/catalog/mempool-policy.md` - Mempool policy and lifecycle parity
  roots.
- `docs/parity/catalog/rpc-cli-config.md` - RPC/CLI baseline behavior and
  Open Bitcoin operator extensions.
- `docs/parity/checklist.md` - Machine-readable surface checklist entries.
- `docs/parity/index.json` - Parity surface registry and evidence roots.
- `docs/parity/production-claim-boundary.md` - Production-readiness and
  deferred-surface claim boundaries.
- `docs/parity/source-breadcrumbs.json` - Required breadcrumbs for new
  first-party Rust source/test files.

### Bitcoin Knots Baseline Anchors

- `packages/bitcoin-knots/src/rpc/rawtransaction.cpp` - Baseline
  `sendrawtransaction` behavior.
- `packages/bitcoin-knots/src/rpc/mempool.cpp` - Baseline `getmempoolinfo`
  behavior.
- `packages/bitcoin-knots/src/rpc/net.cpp` - Baseline `getnetworkinfo`
  behavior.
- `packages/bitcoin-knots/src/net_processing.cpp` - Transaction request,
  serving, fanout, and peer processing baseline.
- `packages/bitcoin-knots/src/node/txdownloadman.h` and
  `packages/bitcoin-knots/src/node/txdownloadman_impl.cpp` - Transaction
  download/request management baseline.
- `packages/bitcoin-knots/src/protocol.h` - Inventory and transaction relay
  protocol identifiers.
- `packages/bitcoin-knots/src/txmempool.cpp` and
  `packages/bitcoin-knots/src/txmempool.h` - Mempool policy/status baseline.
- `packages/bitcoin-knots/src/validation.cpp` - Validation and mempool outcome
  baseline.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets

- `FieldAvailability<T>`: Existing status wrapper for unavailable reasons; use
  it to classify Phase 105 relay fields instead of inventing parallel state
  enums for each surface.
- `OpenBitcoinStatusSnapshot`: Shared source for CLI, dashboard, support, and
  JSON status; extend it so relay/mempool evidence flows through existing
  renderer and support paths.
- `MetricKind::ALL`: Existing fixed metric series registry; add relay outcome
  counters here and update tests so dynamic labels cannot appear.
- `StructuredLogRecord`: Existing low-cardinality JSONL record with sanitizers;
  add relay log record helpers rather than logging raw event structs.
- `LocalRelaySubmissionEvidence`: Phase 104's fixed label/count evidence for
  local `sendrawtransaction` outcomes.
- `ManagedRelayFanoutInfo` and `ManagedRelayServingInfo`: Phase 104 managed
  evidence for queue/serve outcomes that can be projected into sanitized
  operator status.
- `support_status_for_bundle`: Existing central place where support bundles
  receive sanitized status snapshots before JSON and Markdown write.

### Established Patterns

- Baseline RPC methods stay narrow and compatibility-oriented; richer Open
  Bitcoin evidence belongs in Open Bitcoin-prefixed status methods and operator
  surfaces.
- Status/dashboard/support renderers derive from `OpenBitcoinStatusSnapshot`,
  not from separate renderer-local state.
- Metrics are fixed enum variants with no dynamic labels.
- Structured logs are sanitized key/value messages with fixed sources and
  bounded recent-signal loading.
- Support bundle redaction happens before both JSON and Markdown rendering.
- Deterministic phase checkers guard parity roots and no-claim language when
  docs/verifier surfaces change.

### Integration Points

- RPC dispatch can extend `OpenBitcoinNetworkStatusResponse` and/or context
  accessors to include the sanitized relay/mempool status projection.
- Operator live status collection already calls `getnetworkinfo`,
  `getmempoolinfo`, and `openbitcoinnetworkstatus`; add relay status fallback
  handling there.
- CLI human status and dashboard `Mempool and Wallet`/`Logs and Health` sections
  can add compact relay rows without bypassing the shared snapshot.
- Metrics and logs should be projected from the same sanitized relay evidence
  and existing managed event hooks, not from raw peer or transaction material.
- Support bundle JSON/Markdown should include a compact relay/mempool evidence
  section generated from the sanitized snapshot.

</code_context>

<specifics>
## Specific Ideas

- Use the advisor consensus baseline: a shared sanitized presentation/status
  projection is the default; support-bundle-only redaction and renderer-only
  allowlists are fallback patterns, not the Phase 105 design.
- Keep an explicit future path for an opt-in sensitive debug policy, but do not
  implement one in Phase 105. Default operator evidence remains sanitized.
- Preserve existing `sendrawtransaction` success output while adding separate
  evidence that accepted or queued local submission does not guarantee public
  propagation.

</specifics>

<deferred>
## Deferred Ideas

- Periodic rebroadcast scheduling remains deferred beyond Phase 105 unless a
  later phase explicitly plans timer-driven rebroadcast with bounded evidence.
- Compact block relay, package relay, bloom/filter serving, public relay
  defaults, public-network relay CI, production service operation, production
  full-node readiness, and production-funds wallet use remain out of scope.
- Sensitive debug mode or pseudonymized transaction/peer correlation aliases are
  future design work and should not be added as a default support or status
  feature in this phase.

</deferred>

*Phase: 105-operator-rpc-metrics-logs-and-support-evidence*
*Context gathered: 2026-07-01*
