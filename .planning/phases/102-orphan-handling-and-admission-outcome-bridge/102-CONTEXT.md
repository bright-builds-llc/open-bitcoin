---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 102-2026-06-30T14-54-50
generated_at: 2026-06-30T14:54:50.926Z
---

# Phase 102: Orphan Handling and Admission Outcome Bridge - Context

**Gathered:** 2026-06-30
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Phase 102 connects the Phase 101 transaction download scheduler to mempool admission through a typed outcome boundary. It may add bounded missing-parent staging, eligible parent requests, reconsideration after parent acceptance, orphan expiry and eviction evidence, and stable admission outcomes for accepted, rejected, duplicate, replaced, orphaned, evicted, and expired transactions.

This phase must keep peer socket and transaction-download code from mutating mempool state directly. The peer/download layer should emit typed actions or candidate facts, the mempool layer should return stable outcomes, and the managed runtime should be the shell that bridges them. Durable mempool persistence, block connect/disconnect lifecycle, relay serving/fanout, rebroadcast, RPC/operator/support surfaces, release closeout, compact block relay, package relay, bloom/filter serving, public relay defaults, production service claims, and production-funds wallet support remain outside Phase 102 unless a narrow test seam is needed.

</domain>

<decisions>
## Implementation Decisions

### Missing-Parent Staging And Parent Requests

- **D-01:** Missing-input peer transactions should become a typed orphan or candidate outcome rather than a generic mempool error that callers interpret with string matching.
- **D-02:** Orphan staging must be bounded by deterministic caps for total entries, per-peer entries, and expiry time. The planner may choose exact constant names and values, but tests must cover cap eviction and expiry without sleeping or public-network behavior.
- **D-03:** Parent request behavior should reuse Phase 101 `TxRelayId`, request, and suppression vocabulary where possible. Parent requests must be eligible typed scheduler actions, not socket writes or mempool mutations inside peer state.
- **D-04:** Orphan evidence must use fixed low-cardinality labels such as `orphaned`, `parent_requested`, `orphan_evicted`, `orphan_expired`, and `orphan_reconsidered`. Do not expose raw transaction hex, txids, wtxids, peer endpoints, permission strings, class names, credentials, or dynamic labels in shared evidence.

### Reconsideration Flow

- **D-05:** Parent acceptance should trigger reconsideration of staged children through a pure or mostly pure coordinator that takes accepted parent identity plus bounded orphan state and returns candidate admission attempts.
- **D-06:** Reconsideration must be deterministic and bounded. It should avoid recursive unbounded walks, hidden wall-clock reads, and direct socket I/O.
- **D-07:** Reconsideration after parent acceptance should produce stable evidence for accepted child, still-missing-parent child, rejected child, expired child, and evicted child paths.
- **D-08:** Disconnect cleanup should remove or mark peer-owned orphan evidence without leaking stale request state from Phase 101.

### Admission Outcome Contract

- **D-09:** Introduce one stable mempool outcome contract consumed by both peer and local transaction submissions. It should represent at least `accepted`, `rejected`, `duplicate`, `replaced`, `orphaned`, `evicted`, and `expired`.
- **D-10:** Existing `MempoolError` variants should be mapped to the new outcome contract at the mempool boundary. Callers should not pattern-match display strings to decide whether a transaction is an orphan, duplicate, rejected, or eviction case.
- **D-11:** The outcome contract should preserve enough structured data for later RPC, metrics, logs, support bundles, and relay serving without forcing Phase 102 to implement those later surfaces.
- **D-12:** Outcome names and evidence labels must stay low-cardinality and stable so later Phase 105 observability can aggregate them safely.

### Admission Policy Scope

- **D-13:** Admission tests must cover standardness, fee, RBF, ancestor/descendant limits, duplicate handling, and no partial mutation on rejection. Reuse and deepen existing `open-bitcoin-mempool` pure tests before adding adapter-heavy tests.
- **D-14:** No-partial-mutation is a hard invariant. Rejected, non-standard, low-fee, failed replacement, and limit-exceeded candidates must leave accepted mempool entries, indexes, virtual-size totals, and replacement state coherent.
- **D-15:** Replacement outcomes should distinguish ordinary rejection from successful replacement and from replacement-caused eviction. Later phases may expose more operator detail, but Phase 102 must make the internal state transition explicit.
- **D-16:** Package relay and cluster mempool behavior are out of scope. Do not broaden single-transaction orphan handling into package-relay support.

### Managed Runtime Bridge

- **D-17:** Managed runtime tests should prove peer transactions pass through the Phase 101 relay/download boundary before mempool admission. `PeerManager` and socket-facing code should not call mempool APIs directly.
- **D-18:** `ManagedPeerNetwork::process_actions` or a small child bridge module is the expected shell integration point. Keep pure scheduler/admission decisions in `open-bitcoin-network` and `open-bitcoin-mempool`; keep storage, runtime, and managed mempool mutation in `open-bitcoin-node`.
- **D-19:** Local transaction submission should use the same stable outcome contract as peer submissions, even if local and peer callers map outcomes to different later surfaces.
- **D-20:** Phase 102 may add in-memory bridge tests, but durable recovery and restart behavior are Phase 103 scope.

### Resource Governance And Boundaries

- **D-21:** Preserve Phase 94 and Phase 101 resource-governance limits under adversarial transaction download and orphan bursts. Orphan staging must not silently bypass queue, request, timeout, churn, or per-peer caps.
- **D-22:** Default verification must remain deterministic and local. Do not add public-network relay checks, sleeps, service-manager gates, wall-clock soak, or production-deployment checks to `bash scripts/verify.sh`.
- **D-23:** If docs, parity roots, or checkers are updated, preserve the v2.0 no-claim boundary: compact block relay, package relay, bloom/filter serving, public relay defaults, public-network CI, production full-node readiness, and production-funds wallet use stay deferred.
- **D-24:** New first-party Rust source or test files under `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests` need parity breadcrumb entries unless the explicit `none` breadcrumb is the only defensible source anchor.

### the agent's Discretion

The planner may choose exact type names, module split, orphan caps, expiry constants, and whether orphan staging lives in a new network transaction-relay child module, a mempool admission child module, or a small bridge type, as long as functional-core boundaries remain clear and tests prove the observable outcomes above.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Project And v2.0 Scope

- `.planning/PROJECT.md` - Open Bitcoin parity, architecture, dependency, verification, and v2.0 transaction relay boundaries.
- `.planning/REQUIREMENTS.md` - DL-03 through DL-05 and MEM-01 through MEM-02 are owned by Phase 102; MEM-03+ and REL-* are later phases.
- `.planning/ROADMAP.md` - Phase 102 purpose, scope, success criteria, dependencies, and verification contract.
- `.planning/STATE.md` - Current milestone state, Phase 101 completion note, deterministic verification caveats, and repo-local UAT command reminders.
- `.planning/phases/100-relay-activation-boundary-and-permission-semantics/100-CONTEXT.md` - Locked relay activation, permission-effect, low-cardinality evidence, and no-claim decisions that Phase 102 must preserve.
- `.planning/phases/101-transaction-inventory-identity-and-download-scheduling/101-CONTEXT.md` - Locked transaction identity, scheduler, typed action, and managed bridge decisions that Phase 102 consumes.
- `.planning/phases/94-dos-and-resource-governance/94-CONTEXT.md` - Prior queue, request, timeout, churn, and resource-governance constraints under adversarial peer behavior.

### Open Bitcoin Code And Tests

- `packages/open-bitcoin-network/src/peer/transaction_relay.rs` - `TxRelayId`, peer relay mode, download action, suppression labels, and Phase 101 typed action vocabulary.
- `packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs` - Current bounded transaction download scheduler and local fact inputs.
- `packages/open-bitcoin-network/src/peer/transaction_relay/tests.rs` - Current transaction relay identity and scheduler coverage.
- `packages/open-bitcoin-network/src/peer/transaction_relay/tests/scheduler_cases.rs` - Deterministic scheduler cases for fallback, expiry, recent reject, and mempool-known suppression.
- `packages/open-bitcoin-network/src/peer.rs` - `PeerManager`, transaction message dispatch, and pure peer action boundary.
- `packages/open-bitcoin-network/src/peer/inventory_state.rs` - Existing inventory/request state patterns and cleanup helpers.
- `packages/open-bitcoin-network/src/resource.rs` - Phase 94 request-governance caps and low-cardinality pressure labels.
- `packages/open-bitcoin-mempool/src/lib.rs` - Pure-core mempool API exports and chainstate snapshot abstractions.
- `packages/open-bitcoin-mempool/src/error.rs` - Current `MempoolError` variants to map into stable outcomes.
- `packages/open-bitcoin-mempool/src/pool.rs` - Mempool admission, replacement, trimming, and no-partial-mutation core.
- `packages/open-bitcoin-mempool/src/pool/tests.rs` - Existing admission, missing input, duplicate, fee, RBF, ancestor/descendant, trimming, and invariant tests.
- `packages/open-bitcoin-mempool/src/policy.rs` - Standardness and policy checks used by admission.
- `packages/open-bitcoin-mempool/src/policy/output.rs` - Policy output parity anchors and structured policy result patterns.
- `packages/open-bitcoin-mempool/src/types.rs` - Mempool entry, transaction, and policy types.
- `packages/open-bitcoin-node/src/mempool.rs` - Managed mempool wrapper and shell-owned submission surface.
- `packages/open-bitcoin-node/src/network.rs` - `ManagedPeerNetwork`, local submission path, `process_actions`, and current peer-to-mempool bridge.
- `packages/open-bitcoin-node/src/network/action_translation.rs` - Managed action translation patterns.
- `packages/open-bitcoin-node/src/network/inventory.rs` - Managed transaction inventory helper patterns.
- `packages/open-bitcoin-node/src/network/tests.rs` - Existing managed network tests for transaction request and in-memory relay behavior.
- `docs/parity/source-breadcrumbs.json` - Required registry for new or touched first-party Rust source/test files.

### Bitcoin Knots Anchors

- `packages/bitcoin-knots/src/txorphanage.h` - Orphan transaction staging contract and orphanage data structures.
- `packages/bitcoin-knots/src/txorphanage.cpp` - Orphan add, erase, expiry, peer cleanup, and parent/child reconsideration behavior.
- `packages/bitcoin-knots/src/node/txdownloadman.h` - Transaction download manager contract and peer request state.
- `packages/bitcoin-knots/src/node/txdownloadman_impl.cpp` - Download scheduling, orphan/request interaction, `ReceivedTx`, accepted/rejected cleanup, and fallback behavior.
- `packages/bitcoin-knots/src/net_processing.cpp` - P2P transaction relay, orphan handling integration, parent requests, and mempool admission call boundaries.
- `packages/bitcoin-knots/src/validation.cpp` - Transaction admission, validation, and mempool interaction anchors.
- `packages/bitcoin-knots/src/txmempool.h` - Mempool state, conflict, ancestor/descendant, and policy data structures.
- `packages/bitcoin-knots/src/txmempool.cpp` - Mempool acceptance, trimming, replacement, and entry/index maintenance anchors.
- `packages/bitcoin-knots/src/policy/policy.h` - Standardness and relay policy declarations.
- `packages/bitcoin-knots/src/policy/policy.cpp` - Standardness and relay policy implementation.
- `packages/bitcoin-knots/src/policy/rbf.h` - Replacement policy declarations.
- `packages/bitcoin-knots/src/policy/rbf.cpp` - Replacement policy implementation.
- `packages/bitcoin-knots/test/functional/p2p_orphan_handling.py` - P2P orphan behavior and parent request expectations.
- `packages/bitcoin-knots/test/functional/p2p_tx_download.py` - Transaction download fallback and request behavior that Phase 102 builds on.
- `packages/bitcoin-knots/test/functional/mempool_accept.py` - Admission policy, validation, and rejection behavior.
- `packages/bitcoin-knots/test/functional/mempool_accept_wtxid.py` - Wtxid-aware admission behavior.
- `packages/bitcoin-knots/test/functional/feature_rbf.py` - Replacement policy behavior.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets

- `TxRelayId`, `TxDownloadAction`, `TxDownloadLocalFacts`, and `TxDownloadScheduler` already provide a typed pure download boundary for peer transaction announcements and requests.
- `Mempool::accept_transaction`, `MempoolError`, `AdmissionResult`, `PolicyConfig`, and the pool tests already cover much of the admission policy surface Phase 102 needs to stabilize into a caller-facing outcome contract.
- `ManagedMempool` and `ManagedPeerNetwork::process_actions` already form a shell seam where peer actions can be translated into mempool submissions without putting mempool mutation inside `PeerManager`.
- `ResourceGovernancePolicy` and Phase 94 tests already provide bounded request/queue/churn vocabulary to preserve under transaction burst pressure.

### Established Patterns

- Pure network decisions live in `open-bitcoin-network`; pure mempool policy and admission live in `open-bitcoin-mempool`; managed runtime mutation lives in `open-bitcoin-node`.
- Tests should use Arrange, Act, Assert when non-trivial and prefer deterministic fake inputs over sleeps, public-network behavior, service-manager behavior, or wall-clock gates.
- Evidence labels are fixed and low-cardinality. Raw transaction hex, txids, wtxids, peer endpoints, permission strings, class names, credentials, and dynamic labels do not belong in shared status/support/log planning.

### Integration Points

- Add the outcome contract close to `open-bitcoin-mempool` so local and peer submissions share one typed result.
- Add orphan staging either as a pure transaction-relay child module that returns admission candidates or as a mempool-facing orphan outcome helper, but keep shell mutation in `open-bitcoin-node`.
- Bridge Phase 101 `ReceivedTxCleanup` or received transaction actions to mempool admission through `ManagedPeerNetwork::process_actions` or a focused child module.
- Extend parity breadcrumbs and deterministic checker roots only for files actually touched by Phase 102 plans.

</code_context>

<specifics>
## Specific Ideas

- Favor a small `MempoolOutcome` or similarly named enum over expanding `MempoolError` as the only caller contract.
- Favor a bounded orphan state type with fake-clock expiry inputs and explicit eviction records.
- Treat missing parent and duplicate paths as first-class outcomes because later relay serving, RPC, metrics, and support surfaces will need to distinguish them.
- Preserve the repo-local UAT command lesson if operator guidance changes: use explicit Cargo and Bazel command forms, not a bare `open-bitcoin` alias.

</specifics>

<deferred>
## Deferred Ideas

Durable mempool persistence, block connect/disconnect lifecycle, long-lived mempool pressure and trimming evidence, relay serving, fanout, rebroadcast, RPC/operator/support evidence, support-bundle redaction for transaction material, release-boundary closeout, compact block relay, package relay, bloom/filter serving, public relay defaults, public-network relay CI, production full-node readiness, and production-funds wallet use remain outside Phase 102.

</deferred>

***

*Phase: 102-orphan-handling-and-admission-outcome-bridge*
*Context gathered: 2026-06-30*
