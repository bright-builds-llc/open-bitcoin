---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 70-2026-06-12T14-56-46
generated_at: 2026-06-12T14:58:48.782Z
---

# Phase 70: Reorg, Peer Rotation, and No-Progress Recovery - Context

**Gathered:** 2026-06-12
**Status:** Ready for planning
**Mode:** Yolo

<domain>

## Phase Boundary

Phase 70 makes branch competition, durable reorg handling, peer rotation, stale
in-flight cleanup, and no-progress diagnosis deterministic and operator-visible
for the explicit opt-in `open-bitcoind` mainnet sync-to-tip path.

This phase owns cumulative-work active-chain outcomes, durable disconnect and
reconnect behavior, typed peer failure attribution, retry/backoff and rotation
behavior, stale in-flight recovery, and operator-facing no-progress next
actions. It does not own broader long-run resource-bound proof, complete
cross-surface observability closeout, opt-in UAT command breadth, release
boundary closeout, inbound serving, address relay, block serving, transaction
relay, compact block relay, production-wallet claims, migration apply mode,
signed packaging, Windows service support, GUI work, hosted dashboards, or broad
production-node claims.

</domain>

<decisions>

## Implementation Decisions

### Branch Competition and Active-Chain Selection

- **D-01:** Competing branches must resolve through the existing deterministic
  best-tip policy: cumulative work first, then height, then block hash as the
  final stable tie-breaker. Do not add a trusted external tip source,
  centralized peer, checkpoint shortcut, assumevalid shortcut, assumeutxo
  shortcut, or public API dependency.
- **D-02:** Header-store best-chain evidence may identify a better branch, but
  active-chain progress is credited only after the replacement branch's blocks
  are available, consensus-validated, connected, and durably persisted.
- **D-03:** If a better header branch is known but required block bodies are
  missing, report the state as behind or awaiting blocks with actionable peer
  work. Do not disconnect the current active chain until the replacement branch
  can be applied safely.
- **D-04:** Equal-work or lower-work side branches should stay visible as
  competing evidence only where useful for diagnosis. They must not trigger a
  reorg or override the active chain.

### Durable Reorg Execution

- **D-05:** Reorg execution should reuse `Chainstate::reorg`,
  `ManagedChainstate`, and the durable reconcile path instead of introducing a
  second reorg engine. Keep the effectful shell responsible for loading block
  bodies, persisting snapshots, and projecting runtime status.
- **D-06:** Disconnect work must use durable active-chain block bodies and
  recorded undo evidence. Missing active-chain block bodies, missing undo data,
  or malformed stored chainstate are storage recovery blockers, not peer retry
  advice.
- **D-07:** Reorg status should expose bounded undo/reorg evidence: common
  ancestor height/hash, disconnected count, connected count, final active tip,
  and whether the transition was fully persisted. Avoid raw undo dumps in
  operator-facing surfaces.
- **D-08:** A reorg must be atomic from the operator truth perspective. If
  disconnect/reconnect or persistence fails, report recovering or blocked state
  with a typed recovery category and avoid claiming the new active tip.

### Peer Failure Attribution and Rotation

- **D-09:** Preserve and expand typed peer outcomes for stale, slow, incompatible,
  malformed, invalid, disconnecting, `notfound`, duplicate, disconnected,
  non-extending, storage-blocked, resource-limited, address-resolution, and
  network failures. Do not flatten these into generic network errors.
- **D-10:** Endpoint-keyed retry/backoff remains the default rotation mechanism.
  A failing or no-credit peer should be backed off and the runtime should try
  other configured or resolved peers within existing bounded attempt and round
  limits.
- **D-11:** `notfound`, malformed, invalid, duplicate, disconnected, and
  non-extending block responses should release stale in-flight bookkeeping for
  the affected block and preserve no-credit peer attribution. Retry missing
  best-chain blocks with another eligible peer when one is available.
- **D-12:** Do not implement broad production peer eviction, banning, inbound
  reputation, address-manager governance, compact-block fallback, or transaction
  relay policy in this phase. Those are future production-node surfaces.

### No-Progress Diagnosis and Next Actions

- **D-13:** No-progress diagnosis must be typed and derived from shared sync
  evidence rather than renderer strings. The model should distinguish at least:
  current at best-known tip, behind awaiting headers, awaiting block bodies,
  stale in-flight cleanup, peer backoff, peer stalled, peer failures exhausted,
  branch competition awaiting bodies, recovering from reorg/storage state, and
  storage or resource blockers.
- **D-14:** Phase 69's `StayCurrentStatus`, best-known tip evidence,
  `SyncProgressSignal`, `SyncRecoveryCategory`, peer outcomes, and resource
  pressure should feed the diagnosis. Avoid creating a parallel status contract
  that CLI, RPC, dashboard, logs, metrics, and support evidence would later have
  to reconcile.
- **D-15:** Operator next actions should be specific and quiet: wait for backoff,
  try another peer, inspect storage health, increase bounded resource limits,
  wait for block bodies, or confirm current-at-tip evidence. Avoid vague "sync
  failed" and avoid production-readiness wording.
- **D-16:** Stale in-flight work after restart or peer loss should be cleared,
  reassigned, or diagnosed explicitly. Stale requests must not make the daemon
  appear busy while no peer can satisfy the work.

### Verification Posture

- **D-17:** Default verification must stay deterministic, public-network-free,
  service-manager-free, timing-stable, and short-running. Public-mainnet peer
  rotation and reorg evidence remains opt-in UAT until Phase 73 expands operator
  commands.
- **D-18:** Deterministic Rust tests should cover cumulative-work branch
  selection, side-branch non-selection, durable block disconnect/reconnect,
  missing active-chain block/undo storage blockers, stale in-flight release,
  `notfound` retry attribution, slow/stalled peer backoff, incompatible or
  invalid peer rotation, and typed no-progress next actions.
- **D-19:** Add focused docs/checker coverage when operator wording or parity
  roots change. Keep `bash scripts/verify.sh` as the final repo-native
  verification contract.
- **D-20:** New first-party Rust source or test files under
  `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests` must receive
  parity breadcrumb coverage through `docs/parity/source-breadcrumbs.json` and
  `scripts/check-parity-breadcrumbs.ts`.

### the agent's Discretion

- The planner may split work across branch/reorg domain types, peer rotation
  and stale in-flight handling, no-progress status projection, deterministic
  tests, and docs/checker closeout.
- The executor may add small pure helper types for reorg/no-progress diagnosis
  if they keep illegal states unrepresentable and avoid duplicating renderer
  logic.
- The executor may keep Phase 70 operator surfacing limited to the shared
  status/runtime evidence needed for REC-01 through REC-04, leaving broader
  support-bundle and cross-surface alignment to Phase 72.

</decisions>

<canonical_refs>

## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase Scope

- `.planning/ROADMAP.md` - Phase 70 goal, dependency on Phase 69, success
  criteria, and deferred Phase 71 through Phase 74 boundaries.
- `.planning/REQUIREMENTS.md` - REC-01 through REC-04, v1.6 out-of-scope table,
  and default-verification public-network exclusion.
- `.planning/PROJECT.md` - v1.6 milestone goal, explicit opt-in full-sync claim,
  pinned Knots baseline, functional-core boundary, and production-claim limits.
- `.planning/STATE.md` - Current milestone state and Phase 70 as next work.
- `AGENTS.md` - Repo-local workflow, Rust, parity breadcrumb, and verification
  requirements.
- `AGENTS.bright-builds.md` - Bright Builds workflow and standards routing.
- `standards-overrides.md` - Current local standards override registry.

### Prior Phase Decisions and Evidence

- `.planning/phases/61-resource-bounds-and-recovery-taxonomy/61-CONTEXT.md` -
  Recovery categories, peer failure attribution, retry/backoff, resource
  pressure, and public-network verification boundaries.
- `.planning/phases/64-service-restart-and-same-datadir-resume-evidence/64-CONTEXT.md`
  - Same-datadir restart/resume evidence, stale in-flight cleanup, and recovery
  next-action posture.
- `.planning/phases/66-compatibility-harness-operator-wrapper/66-CONTEXT.md` -
  Compatibility wrapper diagnosis and no-credit peer outcome alignment.
- `.planning/phases/67-release-boundaries-and-deterministic-verification/67-CONTEXT.md`
  - Release-boundary wording, deterministic checker posture, and deferred
  production scopes.
- `.planning/phases/68-full-active-chain-validation-and-durable-persistence/68-CONTEXT.md`
  - Validated active-chain progress credit, durable persistence contract,
  no-credit block outcomes, and Phase 70 deferral.
- `.planning/phases/68-full-active-chain-validation-and-durable-persistence/68-VERIFICATION.md`
  - Passed Phase 68 evidence and residual risks deferred to Phase 70.
- `.planning/phases/69-tip-tracking-and-stay-current-operation/69-CONTEXT.md`
  - Best-known tip evidence, stay-current states, stale-tip/no-progress split,
  and shared status integration points.
- `.planning/phases/69-tip-tracking-and-stay-current-operation/69-VERIFICATION.md`
  - Passed Phase 69 evidence and residual Phase 70 risks.

### Implementation Surfaces

- `packages/open-bitcoin-network/src/header_store.rs` - Header best-tip
  selection, best-chain entries, ancestor lookup, locator behavior, and
  cumulative-work tie-breaker policy.
- `packages/open-bitcoin-chainstate/src/engine.rs` - Active-chain connect,
  disconnect, reorg, UTXO/undo handling, and `prefer_candidate_tip`.
- `packages/open-bitcoin-chainstate/src/types.rs` - `ChainPosition`,
  `AnchoredBlock`, `ChainTransition`, `ChainstateSnapshot`, and undo payload
  types.
- `packages/open-bitcoin-chainstate/src/error.rs` - Chainstate connect,
  disconnect, undo, and reorg error taxonomy.
- `packages/open-bitcoin-node/src/chainstate.rs` - Managed chainstate store
  wrapper and persistence after connect/disconnect/reorg.
- `packages/open-bitcoin-node/src/network.rs` - `ManagedPeerNetwork`,
  `BlockConnectDisposition`, sync message handling, local block tracking, and
  reorg-to-branch adapter.
- `packages/open-bitcoin-node/src/storage.rs` - Storage namespaces, runtime
  metadata, recovery actions, and typed storage errors.
- `packages/open-bitcoin-node/src/storage/fjall_store.rs` - Fjall persistence
  for chainstate snapshots, header entries, block-index entries, block bodies,
  runtime metadata, metrics, and recovery markers.
- `packages/open-bitcoin-node/src/storage/snapshot_codec.rs` - Versioned DTOs
  for persisted chainstate, headers, runtime metadata, and compatibility
  updates.
- `packages/open-bitcoin-node/src/sync.rs` - `DurableSyncRuntime`, bounded sync
  cycles, peer iteration, summary projection, and opt-in daemon entrypoints.
- `packages/open-bitcoin-node/src/sync/block_reconcile.rs` - Stored block
  reconciliation, best-chain extension, branch replacement, active-chain block
  loading, and storage blockers.
- `packages/open-bitcoin-node/src/sync/block_response.rs` - Requested and
  unrequested block disposition handling, block persistence, and no-credit
  attribution.
- `packages/open-bitcoin-node/src/sync/progress.rs` - Peer progress, typed
  failure reasons, retry/backoff helpers, health signals, and recovery actions.
- `packages/open-bitcoin-node/src/sync/runtime_state.rs` - Durable status
  projection, best-height helpers, runtime metadata persistence, peer backoff,
  and summary-to-status mapping.
- `packages/open-bitcoin-node/src/sync/types.rs` - Sync runtime config,
  summaries, stop reasons, peer outcomes, failure reasons, and runtime errors.
- `packages/open-bitcoin-node/src/sync/types/summary.rs` - Shared sync status,
  progress signal, lag, metrics, logs, and peer projection.
- `packages/open-bitcoin-node/src/status.rs` - Shared operator status contracts,
  best-known tip evidence, stay-current status, recovery category, peer
  telemetry, and field availability wrappers.
- `packages/open-bitcoin-node/src/sync/tests.rs` - Deterministic scripted
  transport, resolver, storage, restart, metrics, log, and peer failure
  fixtures.
- `packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs` - Daemon activation,
  preflight, runtime store opening, and worker loop shell.
- `packages/open-bitcoin-cli/src/operator/sync.rs` - Operator CLI sync status,
  pause, and resume rendering.
- `scripts/run-live-mainnet-smoke.ts` - Opt-in live-smoke report fields for
  no-progress, peer outcomes, restart/resume, and block progress evidence.
- `scripts/verify.sh` - Repo-native deterministic verification contract.

### Docs and Parity Roots

- `docs/operator/runtime-guide.md` - Operator sync activation, status evidence,
  restart/resume, support bundle, and UAT command guidance.
- `docs/architecture/status-snapshot.md` - Shared status snapshot truth
  contract.
- `docs/architecture/operator-observability.md` - Metrics/log/support evidence
  and deterministic verification boundaries.
- `docs/parity/catalog/chainstate.md` - Chainstate, connect/disconnect, reorg,
  and persistence parity scope.
- `docs/parity/catalog/p2p.md` - P2P sync, no-credit peer attribution,
  restart/resume, support evidence, and production-node boundary scope.
- `docs/parity/threat-model-v1.5.md` - Public peer input, recovery, stale
  in-flight, and service restart/resume threat boundaries.
- `docs/parity/index.json` and `docs/parity/source-breadcrumbs.json` - Parity
  root registry and required source breadcrumb coverage.

### Baseline Anchors

- `packages/bitcoin-knots/src/validation.cpp` - Active-chain connection,
  disconnect/reconnect, validation-state, and best-chain behavior anchor.
- `packages/bitcoin-knots/src/node/chainstate.cpp` - Chainstate lifecycle and
  active-chain behavior anchor.
- `packages/bitcoin-knots/src/node/blockstorage.cpp` - Block index and block
  storage behavior anchor.
- `packages/bitcoin-knots/src/coins.h` and `packages/bitcoin-knots/src/coins.cpp`
  - UTXO view and undo persistence anchors.
- `packages/bitcoin-knots/src/net_processing.cpp` - Peer block/header
  attribution, peer failure handling, and no-credit behavior anchor.
- `packages/bitcoin-knots/src/headerssync.cpp` - Header sync and branch
  selection anchor.

</canonical_refs>

<code_context>

## Existing Code Insights

### Reusable Assets

- `HeaderStore::update_best_tip` already applies deterministic branch selection
  by chain work, height, and block hash.
- `Chainstate::disconnect_tip` and `Chainstate::reorg` already implement
  UTXO/undo-backed disconnect and reconnect behavior.
- `ManagedChainstate` already persists chainstate snapshots after connect,
  disconnect, and reorg through a `ChainstateStore` abstraction.
- `reconcile_best_chain` already finds the active/best-chain common prefix,
  waits for replacement block bodies, checks candidate rank, loads active-chain
  block bodies for disconnect, and calls `reorg_to_branch`.
- `PeerFailureReason`, `PeerProgress`, and `PeerSyncOutcome` already model
  many no-credit peer outcomes, including `block_notfound`, malformed,
  invalid, duplicate, disconnected, non-extending, compatibility, stall,
  retry-backoff, resource, storage, and network categories.
- Phase 69 added `BestKnownTipStatus`, `StayCurrentStatus`, and related
  peer-agreement status contracts in `status.rs`.

### Established Patterns

- Public-network behavior is opt-in UAT and excluded from `bash scripts/verify.sh`.
- Operator status fields use typed enums and `FieldAvailability<T>` instead of
  renderer-specific strings.
- Runtime metadata changes require backward-compatible DTO updates and
  deterministic reopen tests.
- Peer failures produce typed recovery categories/actions and bounded backoff
  instead of unbounded retries or hot loops.
- Progress credit is validated active-chain progress, not headers-only,
  downloaded-only, or peer-advertised progress.

### Integration Points

- Reorg and branch competition should build on the existing header store,
  reconcile, managed network, and managed chainstate path.
- No-progress diagnosis should be computed in shared sync/status code and then
  reused by CLI, RPC, dashboard, metrics, logs, and later support evidence.
- Peer rotation work should update runtime peer outcomes, backoff, in-flight
  tracking, and summary projection before touching renderer code.
- Docs/checker work should be focused on REC-01 through REC-04 and avoid
  pre-claiming Phase 71 through Phase 74 outcomes.

</code_context>

<specifics>

## Specific Ideas

- Reorg status should be understandable without raw undo dumps: common
  ancestor, disconnected count, connected count, final active tip, and
  persistence verdict are enough for Phase 70.
- Missing data needed to disconnect the current active chain is a storage
  recovery problem, not a peer problem.
- Stale in-flight requests should be visible as cleanup or reassignment work,
  never as invisible "busy" state.
- "No progress" should answer "why no progress?" and "what should I do next?"
  in the same shared status contract.

</specifics>

<deferred>

## Deferred Ideas

- Long-run resource-bound proof, storage-pressure behavior, and synthetic
  long-chain bounds are Phase 71.
- Cross-surface support evidence and full observability alignment are Phase 72.
- Copy-pasteable opt-in public-mainnet UAT command breadth and deterministic
  release verification expansion are Phase 73.
- v1.6 parity roots, release-readiness matrix, README closeout, and
  claim-boundary checks are Phase 74.
- Inbound serving, address relay, block serving, transaction relay, compact
  block relay, production-wallet claims, migration apply mode, signed
  packaging, GUI, hosted dashboards, and broad production-node readiness remain
  out of scope for v1.6.

</deferred>

---

*Phase: 70-reorg-peer-rotation-and-no-progress-recovery*
*Context gathered: 2026-06-12*
