---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 107-2026-07-03T02-54-20
generated_at: 2026-07-03T02:54:26.453Z
---

# Phase 107: Runtime Relay Activation and Download Eligibility Integration - Context

**Gathered:** 2026-07-03
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Phase 107 closes the live relay wiring gap discovered after the initial v2.0
closeout work: explicit `RuntimeConfig.relay` activation must reach the managed
network runtime, and transaction announcement/download scheduling must require
the Phase 100 relay activation and peer eligibility policy before emitting
request actions.

The phase may change managed network construction, transaction download
scheduling inputs/actions, peer eligibility evidence, RPC/status projection,
UAT/docs, and deterministic guardrails needed to prove the wiring. It must not
enable public relay by default, change service bits accidentally, add compact
block relay, package relay, bloom/filter serving, public-network CI, production
service operation, production full-node readiness, production-funds wallet
claims, or durable mempool recovery behavior owned by Phase 108.

</domain>

<decisions>
## Implementation Decisions

### Runtime Activation Propagation

- **D-01:** `RuntimeConfig.relay` is the source of truth for daemon/runtime
  relay activation. Runtime and RPC context construction must pass the resolved
  relay config into `ManagedPeerNetwork`; default construction must remain
  default-off.
- **D-02:** Existing constructors that intentionally use default relay behavior
  may remain for tests or compatibility, but production daemon/runtime paths
  must not accidentally instantiate `ManagedPeerNetwork` with
  `RelayActivationConfig::default()` after config loading has resolved a
  different value.
- **D-03:** The activation value should be inspectable through existing shared
  status/RPC evidence without expanding baseline-compatible RPC response shapes
  with ad hoc Open Bitcoin-only fields.

### Download Eligibility Gate

- **D-04:** Transaction announcement/download scheduling must consult Phase 100
  relay eligibility before scheduling `getdata` requests. Disabled activation,
  ordinary inbound peers, protected-only peers, and peers without scoped relay
  permission should suppress downloads with stable typed evidence.
- **D-05:** Eligibility suppression should be represented as typed scheduler or
  action vocabulary, not as a swallowed branch. Downstream status, logs, metrics,
  and tests should be able to distinguish `relay_disabled`,
  `not_relay_eligible`, `inbound_serving_required`, `permission_required`, and
  `protected_not_relay` style outcomes without exposing peer ids, endpoints,
  permission strings, txids, wtxids, or raw transaction material.
- **D-06:** Suppression must not leave stale announcement, in-flight request, or
  received-transaction cleanup state. Existing duplicate, already-have,
  recent-reject, mempool-known, request-cap, timeout, `notfound`, disconnect,
  and received-transaction cleanup behavior must continue to work.

### Peer Class Matrix

- **D-07:** Tests must prove enabled and disabled relay behavior across outbound,
  inbound, manual, protected, and permissioned peer classes. Outbound and manual
  peers require explicit activation; ordinary inbound peers remain ineligible;
  permissioned inbound peers require scoped `relay`, `forcerelay`, or `mempool`
  policy inputs; protected admission alone is not relay eligibility.
- **D-08:** `forcerelay` may remain a distinct scoped policy input, but this
  phase must not turn it into unbounded broadcast, package relay, compact block
  relay, or public propagation.
- **D-09:** Service bits, public defaults, inbound listener defaults, and compact
  block/filter behavior must remain unchanged unless a future phase explicitly
  scopes that work.

### Operator Evidence

- **D-10:** RPC/status/UAT evidence should distinguish default-off relay,
  explicitly enabled relay, eligible peers, and ineligible peers using the
  shared Phase 105 sanitized status contract where practical.
- **D-11:** `sendrawtransaction` success may still mean local admission and
  queued relay evidence inside the bounded v2.0 claim. It must not imply public
  propagation, production service readiness, or production-funds wallet safety.
- **D-12:** Metrics, logs, support bundles, and operator output must continue to
  use fixed low-cardinality labels and existing redaction boundaries. No raw
  transaction hex, txids, wtxids, peer endpoints, peer ids, permission strings,
  class names, credentials, or dynamic labels should appear.

### Deterministic Guardrails

- **D-13:** Add or update deterministic checker coverage so dropped runtime
  activation config and missing download eligibility gates fail locally before
  milestone archive.
- **D-14:** If docs, parity roots, checker fixtures, or verifier wiring change,
  keep `bash scripts/verify.sh` deterministic and public-network-free. UAT may
  describe opt-in local loopback/regtest review using repo-local Cargo and Bazel
  commands.
- **D-15:** New or touched first-party Rust source/test files under
  `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests` need parity
  breadcrumbs through `docs/parity/source-breadcrumbs.json` unless an explicit
  `none` breadcrumb is defensible.

### the agent's Discretion

The planner may choose the exact type names, constructor names, scheduler action
labels, test helpers, and whether the eligibility gate lives directly in the
transaction download scheduler or immediately before scheduler entry. Prefer the
smallest pure API that preserves Phase 100 policy ownership, keeps managed
runtime adapters thin, and avoids duplicating status/redaction logic.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Project And Phase Scope

- `.planning/PROJECT.md` - Current v2.0 boundary, parity value, architecture
  constraints, dependency policy, and deferred production/public relay claims.
- `.planning/REQUIREMENTS.md` - ACT-01, ACT-02, INV-02, INV-03, DL-01, DL-02,
  and REL-03 are owned by Phase 107 after the gap-closure roadmap update.
- `.planning/ROADMAP.md` - Phase 107 purpose, scope, success criteria,
  verification, and dependency on Phase 106 closeout evidence.
- `.planning/STATE.md` - Current milestone state, recent v2.0 decisions, local
  verification caveats, and repo-local UAT command reminders.
- `AGENTS.md` - Repo-local verification, parity breadcrumb, GSD, Rust, and
  Bright Builds guidance.

### Prior Locked Decisions

- `.planning/phases/100-relay-activation-boundary-and-permission-semantics/100-CONTEXT.md`
  - Default-off relay activation, peer eligibility matrix, scoped permission
  effects, low-cardinality evidence, and no-claim guardrails.
- `.planning/phases/101-transaction-inventory-identity-and-download-scheduling/101-CONTEXT.md`
  - Txid/wtxid identity, scheduler state, typed request/suppression actions,
  duplicate/fallback cleanup, and deterministic fake-clock tests.
- `.planning/phases/104-relay-serving-fanout-and-rebroadcast-policy/104-CONTEXT.md`
  - Relay serving/fanout eligibility, local submission evidence, and
  rebroadcast-deferred boundary.
- `.planning/phases/105-operator-rpc-metrics-logs-and-support-evidence/105-CONTEXT.md`
  - Sanitized shared status projection, RPC/CLI/dashboard/metrics/log/support
  evidence, and redaction constraints.
- `.planning/phases/106-parity-traceability-uat-and-release-boundary-guardrails/106-CONTEXT.md`
  - v2.0 closeout guardrail pattern that Phase 107 must refresh after fixing
  the integration gap.

### Open Bitcoin Code And Tests

- `packages/open-bitcoin-rpc/src/config.rs` - `RuntimeConfig` and
  `RelayActivationConfig` ownership.
- `packages/open-bitcoin-rpc/src/config/loader.rs` - JSONC/CLI relay config
  resolution, including `relay.enabled` and `-openbitcoinrelay`.
- `packages/open-bitcoin-rpc/src/config/tests.rs` - Existing relay activation
  config tests to extend for runtime propagation coverage.
- `packages/open-bitcoin-rpc/src/context.rs` - `ManagedRpcContext` construction
  path that should preserve runtime relay config.
- `packages/open-bitcoin-node/src/network.rs` - `ManagedPeerNetwork`,
  relay activation storage, action processing, and managed network constructors.
- `packages/open-bitcoin-node/src/network/action_translation.rs` - Translation
  from pure transaction download actions into managed network messages.
- `packages/open-bitcoin-node/src/network/admission_bridge.rs` - Local and peer
  transaction submission bridge to preserve while adding eligibility gates.
- `packages/open-bitcoin-node/src/network/relay_fanout.rs` - Phase 104 local
  submission/fanout evidence that must keep using the same eligibility policy.
- `packages/open-bitcoin-node/src/network/relay_serving.rs` - Managed serving
  and relay eligibility projection.
- `packages/open-bitcoin-node/src/network/tests.rs` - Managed network runtime,
  transaction relay, and local submission tests.
- `packages/open-bitcoin-network/src/relay.rs` - Phase 100 pure relay activation
  and peer eligibility policy.
- `packages/open-bitcoin-network/src/peer.rs` - `PeerManager`, peer modes,
  transaction relay actions, and download scheduler entry points.
- `packages/open-bitcoin-network/src/peer/inventory_state.rs` - Peer inventory,
  `getdata`, `notfound`, and `tx` message handling.
- `packages/open-bitcoin-network/src/peer/transaction_relay.rs` - Tx relay
  exports and identity/action vocabulary.
- `packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs` -
  Download scheduling, duplicate suppression, caps, fallback, expiry,
  `notfound`, and cleanup behavior to guard with eligibility.
- `packages/open-bitcoin-network/src/peer/transaction_relay/tests/scheduler_cases.rs`
  - Existing scheduler cases to extend with relay-disabled and ineligible peer
  coverage.
- `packages/open-bitcoin-network/src/peer/transaction_relay/tests/fanout_cases.rs`
  - Existing fanout eligibility coverage to keep coherent with scheduler gating.
- `packages/open-bitcoin-network/src/peer/tests.rs` - Peer manager integration
  tests for transaction relay behavior.
- `docs/parity/source-breadcrumbs.json` - Required breadcrumb registry for new
  or touched first-party Rust source/test files.

### Docs, Parity, And Checkers

- `docs/architecture/config-precedence.md` - Relay config precedence and
  Open Bitcoin-owned config boundary.
- `docs/architecture/status-snapshot.md` - Shared status ownership and relay
  activation/status boundaries.
- `docs/architecture/operator-observability.md` - Low-cardinality relay
  evidence and redaction rules.
- `docs/operator/runtime-guide.md` - Runtime relay UAT wording and repo-local
  Cargo/Bazel command style.
- `docs/parity/catalog/p2p.md` - P2P parity catalog and transaction relay
  anchors.
- `docs/parity/index.json` - v2.0 parity surface ownership and evidence roots.
- `scripts/check-phase100-relay-activation-boundary.ts` - Existing activation
  boundary checker and no-claim vocabulary.
- `scripts/check-phase101-transaction-inventory-download-scheduling.ts` -
  Existing download scheduler checker and required evidence.
- `scripts/check-phase104-relay-serving-fanout.ts` - Serving/fanout checker and
  relay eligibility evidence.
- `scripts/check-phase105-operator-relay-evidence.ts` - Operator evidence and
  redaction checker patterns.
- `scripts/check-phase106-parity-uat-release-boundary.ts` - Current v2.0
  traceability/guardrail checker to refresh after Phase 107.
- `scripts/verify.sh` - Repo-native verification contract and checker order.

### Bitcoin Knots Anchors

- `packages/bitcoin-knots/src/net_permissions.h` - Permission flag vocabulary
  and implied relay/mempool permission anchors.
- `packages/bitcoin-knots/src/net_permissions.cpp` - Permission parsing,
  `all` expansion, and label behavior.
- `packages/bitcoin-knots/src/net.cpp` - Peer connection classes, protected
  peer behavior, service flags, and connection manager context.
- `packages/bitcoin-knots/src/net_processing.cpp` - Transaction relay,
  permission effects, `mempool`, `relay`, `forcerelay`, P2P processing, and
  request flushing.
- `packages/bitcoin-knots/src/node/txdownloadman.h` - Transaction download
  manager contract and peer connection info.
- `packages/bitcoin-knots/src/node/txdownloadman_impl.cpp` - Already-have
  checks, txid/wtxid handling, request scheduling, in-flight expiry,
  `notfound`, accepted/rejected cleanup, and disconnect cleanup.
- `packages/bitcoin-knots/test/functional/p2p_permissions.py` - Permission and
  protected peer behavior expectations.
- `packages/bitcoin-knots/test/functional/p2p_tx_download.py` - Transaction
  download behavior for in-flight caps, expiry fallback, disconnect fallback,
  `notfound`, txid delay, and wtxidrelay mismatch cases.
- `packages/bitcoin-knots/test/functional/p2p_getdata.py` - `getdata` behavior
  and continued processing.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets

- `RuntimeConfig { relay: RelayActivationConfig }` already exists in
  `open-bitcoin-rpc`, and config loader tests already prove JSONC/CLI parsing.
- `ManagedPeerNetwork` already stores a `relay_activation` field and exposes
  `new_with_relay_activation`; the risk is whether production construction paths
  consistently use it after loading `RuntimeConfig`.
- `open-bitcoin-network::relay` already owns the pure `RelayEligibilityPolicy`,
  `RelayEligibilityDecision`, and peer-class vocabulary that should be reused
  instead of inventing a second eligibility model.
- `TxDownloadScheduler`, `TxDownloadAction`, `TxDownloadSuppressionReason`,
  `TxDownloadLocalFacts`, and scheduler case tests already provide the pure
  download scheduling seam.
- Phase 104/105 managed relay serving, fanout, local submission, and sanitized
  status evidence already provide bounded labels and no-claim wording to reuse.

### Established Patterns

- Pure relay and scheduler decisions belong in `open-bitcoin-network`; runtime
  config propagation and managed message translation belong in
  `open-bitcoin-node`; config parsing and RPC context wiring belong in
  `open-bitcoin-rpc`.
- Deterministic checkers are Bun/TypeScript files with companion tests, wired
  through `scripts/verify.sh` in phase order.
- Docs and support evidence use fixed low-cardinality labels and avoid raw peer,
  permission, txid/wtxid, raw transaction, endpoint, credential, and dynamic
  string material.
- Default verification remains local and deterministic; public-network relay
  review is opt-in UAT only.

### Integration Points

- Audit all production-like `ManagedPeerNetwork::new` and
  `ManagedRpcContext::from_runtime_config` paths for dropped relay config.
- Thread relay eligibility into transaction announcement/download scheduling
  close to the existing `TxAnnouncementInput` or peer-manager scheduling seam.
- Extend scheduler/peer tests before managed-network tests so pure behavior is
  pinned before shell translation.
- Refresh docs/checkers only after the code path and evidence vocabulary are
  concrete enough to guard.

</code_context>

<specifics>
## Specific Ideas

- Prefer tests that first fail when `RuntimeConfig { relay: enabled }` still
  creates a managed network with disabled relay state.
- Prefer a typed suppression reason over silently skipping ineligible peer
  announcements, because Phase 105 evidence needs truthful counts and labels.
- Preserve existing Phase 101 fallback behavior when the first announcing peer
  is ineligible by allowing an eligible alternate peer to request the
  transaction without stale state.
- Keep UAT examples copy-pasteable with repo-local Cargo and Bazel command
  forms; do not document a bare installed `open-bitcoin` alias as the only path.

</specifics>

<deferred>
## Deferred Ideas

Durable mempool relay state recovery, restart replay into relay-serving indexes,
compact block relay, package relay, bloom/filter serving, broad address relay,
public relay by default, public-network relay CI, production service operation,
production full-node readiness, production-funds wallet safety, GUI, hosted
dashboards, packaging, installer, and migration apply mode remain outside Phase
107.

</deferred>

***

*Phase: 107-runtime-relay-activation-and-download-eligibility-integration*
*Context gathered: 2026-07-03*
