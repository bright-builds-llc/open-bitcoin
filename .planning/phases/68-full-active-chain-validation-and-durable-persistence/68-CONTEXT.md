---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 68-2026-06-11T11-56-49
generated_at: 2026-06-11T11:56:49.713Z
---

# Phase 68: Full Active-Chain Validation and Durable Persistence - Context

**Gathered:** 2026-06-11
**Status:** Ready for planning
**Mode:** Yolo

<domain>

## Phase Boundary

Phase 68 makes the explicit opt-in `open-bitcoind` mainnet sync path credit
active-chain progress only after a block has passed consensus validation,
connected to the active chain, and been durably persisted with the chainstate,
UTXO set, undo data, block index, header metadata, downloaded block body, and
runtime metadata needed to resume safely.

This phase owns durable active-chain connect progress and same-datadir recovery
for the validation path. It does not own stay-current operation after catch-up,
full best-tip peer agreement policy, reorg/no-progress recovery expansion,
long-run resource-bound proof, cross-surface observability closeout, opt-in UAT
documentation breadth, release-boundary closeout, inbound serving, relay,
production-wallet claims, migration apply mode, packaging, Windows service
support, GUI work, hosted dashboards, or broad production-node claims.

</domain>

<decisions>

## Implementation Decisions

### Validated Progress Credit

- **D-01:** Treat connected active-chain height as the only block-progress
  credit that satisfies Phase 68. Header height and downloaded block height stay
  visible diagnostic counters, but they must not be described as validated
  active-chain progress.
- **D-02:** Connect progress must pass through the existing consensus and
  chainstate validation path before status, metrics, logs, support evidence, or
  live-smoke summaries report it as connected. Do not add a headers-only,
  downloaded-only, or externally trusted shortcut to the sync-to-tip claim.
- **D-03:** Preserve the distinction among header height, downloaded block
  height, connected block height, validated active-chain height, cumulative
  work, and tip freshness. Phase 68 should fill the durable active-chain pieces;
  Phase 69 may strengthen best-known-tip agreement and stay-current semantics.

### Durable Persistence Contract

- **D-04:** Use the existing first-party chainstate model as the source of truth
  for active-chain, UTXO, and undo state. `ChainstateSnapshot` persistence should
  include active-chain positions, UTXOs, and undo-by-block evidence needed to
  reopen without unsafe in-memory assumptions.
- **D-05:** Keep durable storage namespace boundaries explicit: headers,
  block-index entries, downloaded block bodies, chainstate snapshots, runtime
  metadata, metrics, and recovery markers must remain separate typed storage
  concerns instead of being collapsed into one ad hoc status blob.
- **D-06:** Persist connected active-chain progress before crediting the sync
  cycle as successful. If durable chainstate or runtime metadata persistence
  fails, the phase should return a typed storage blocker and avoid reporting a
  successful connected height that was not durably committed.
- **D-07:** Same-datadir restart must recover the persisted active-chain,
  UTXO/undo snapshot, header store, block-index projection, downloaded block
  bodies, and runtime metadata needed to continue validation. Missing or
  corrupt active-chain block bodies should surface as storage recovery blockers,
  not as peer retry advice.

### Runtime Integration

- **D-08:** Continue using `DurableSyncRuntime` as the effectful shell around
  peer transport, durable storage, and sync orchestration. Keep domain decisions
  in pure helpers or first-party core crates where practical, with the daemon
  binary remaining a thin activation/preflight shell.
- **D-09:** Reconcile stored best-chain blocks before requesting more block
  bodies. Stored block bodies should be connected only when they extend or
  validly replace the active chain according to cumulative work and existing
  chainstate checks.
- **D-10:** Keep invalid, malformed, duplicate, disconnected, non-extending, and
  `notfound` block responses as no-credit peer outcomes. They may update peer
  diagnostics and recovery guidance, but they must not advance connected
  active-chain counters.
- **D-11:** Phase 68 may add a small connection result or persistence result
  type when that makes illegal states unrepresentable, such as separating
  downloaded-only, validated-connected, persisted-connected, and blocked
  outcomes.

### Operator And Scope Boundaries

- **D-12:** Operator wording should describe explicit opt-in sync-to-tip
  validation progress only when evidence actually supports it. Avoid production
  full-node, inbound-serving, relay, production-wallet, migration-apply, signed
  packaging, hosted-dashboard, GUI, or broad production-readiness phrasing.
- **D-13:** Public-mainnet sync attempts are opt-in UAT evidence. Default
  verification must remain deterministic, public-network-free, service-manager
  free, and short-running.
- **D-14:** Preserve the v1.5 service, support, compatibility, redaction, and
  release-boundary posture. Phase 68 should update operator-facing text only
  where durable active-chain validation changes the truth contract.

### Verification Posture

- **D-15:** Deterministic Rust tests should prove the active-chain path:
  validated block connect, durable chainstate snapshot write, same-datadir
  reopen, duplicate connect prevention, downloaded-only no-credit behavior,
  invalid or disconnected block no-credit behavior, and storage failure blockers.
- **D-16:** If docs, live-smoke parsing, parity breadcrumbs, or deterministic
  checker scripts change, add focused local checks and keep `bash
  scripts/verify.sh` as the final repo-native verification contract.
- **D-17:** New first-party Rust source or test files under
  `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests` must receive
  parity breadcrumb coverage through the repo-owned parity breadcrumb files and
  checker.

### the agent's Discretion

- The planner may split work by durable chainstate integration, runtime
  reconcile/connect behavior, status/projection evidence, and deterministic
  verification if that keeps review focused.
- The executor may persist a complete chainstate snapshot per connected block
  for this phase if that is the simplest robust path; incremental storage can be
  deferred unless the existing architecture already supports it cleanly.
- The executor may add small pure helpers or result enums in existing sync or
  storage modules when they reduce duplication and make no-credit versus
  credited progress explicit.

</decisions>

<canonical_refs>

## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase Scope

- `.planning/ROADMAP.md` - Phase 68 goal, success criteria, dependency on Phase
  67, and deferred Phase 69 through Phase 74 boundaries.
- `.planning/REQUIREMENTS.md` - SYNC-01 through SYNC-04, v1.6 deferred scope,
  and default-verification public-network exclusion.
- `.planning/PROJECT.md` - v1.6 milestone goal, pinned Knots baseline,
  functional-core boundary, and production-claim limits.
- `.planning/STATE.md` - Current milestone state and decisions about
  deterministic verification, opt-in UAT, and Phase 68 as the next phase.
- `AGENTS.md` - Repo-local workflow, Rust, parity breadcrumb, and verification
  requirements.
- `AGENTS.bright-builds.md` - Bright Builds workflow and standards routing.
- `standards-overrides.md` - Current local standards override registry.

### Prior Phase Decisions And Evidence

- `.planning/phases/60-unattended-sync-loop-control/60-CONTEXT.md` - Explicit
  opt-in daemon sync loop activation, durable stop reasons, and deterministic
  verification posture.
- `.planning/phases/61-resource-bounds-and-recovery-taxonomy/61-CONTEXT.md` -
  Recovery taxonomy, resource pressure, storage-first recovery precedence, and
  support evidence boundaries.
- `.planning/phases/62-long-run-sync-truth-surfaces/62-CONTEXT.md` - Shared
  sync truth fields for header, downloaded, connected, recovery, metrics, logs,
  and live-smoke surfaces.
- `.planning/phases/63-service-supervision-lifecycle/63-CONTEXT.md` - Service
  lifecycle boundary and service-manager UAT exclusion.
- `.planning/phases/64-service-restart-and-same-datadir-resume-evidence/64-CONTEXT.md`
  - Same-datadir restart/resume safety, stale in-flight cleanup, and durable
  progress evidence.
- `.planning/phases/65-support-bundle-and-operator-review-docs/65-CONTEXT.md`
  - Redacted support evidence and repo-local operator command guidance.
- `.planning/phases/66-compatibility-harness-operator-wrapper/66-CONTEXT.md` -
  Compatibility wrapper no-credit peer outcome alignment.
- `.planning/phases/67-release-boundaries-and-deterministic-verification/67-CONTEXT.md`
  - Release-boundary wording, deterministic checker posture, and deferred
  production scopes.

### Implementation Surfaces

- `packages/open-bitcoin-chainstate/src/engine.rs` - Active-chain connect,
  disconnect, reorg, UTXO, undo, contextual block validation, and snapshot
  behavior.
- `packages/open-bitcoin-chainstate/src/types.rs` - `ChainstateSnapshot`,
  `ChainPosition`, `Coin`, `BlockUndo`, and transition types.
- `packages/open-bitcoin-chainstate/src/error.rs` - Chainstate validation and
  recovery error taxonomy.
- `packages/open-bitcoin-node/src/chainstate.rs` - Managed chainstate store
  wrapper and persistence after connect/disconnect/reorg.
- `packages/open-bitcoin-node/src/storage.rs` - Storage namespaces, recovery
  actions, runtime metadata, and typed storage errors.
- `packages/open-bitcoin-node/src/storage/fjall_store.rs` - Fjall persistence
  for chainstate snapshots, header entries, block-index entries, block bodies,
  runtime metadata, metrics, and recovery markers.
- `packages/open-bitcoin-node/src/storage/snapshot_codec.rs` - Versioned
  storage DTOs for chainstate, UTXO, undo, header entries, block index, and
  runtime metadata.
- `packages/open-bitcoin-node/src/sync.rs` - `DurableSyncRuntime`,
  `sync_once`, `sync_until_idle`, durable state projection, and progress
  persistence.
- `packages/open-bitcoin-node/src/sync/block_reconcile.rs` - Stored block
  reconciliation, active-chain connection, reorg candidate selection, and
  storage blockers.
- `packages/open-bitcoin-node/src/sync/block_response.rs` - Block response
  handling, accepted-block attribution, and no-credit peer outcomes.
- `packages/open-bitcoin-node/src/sync/progress.rs` - Peer progress counters
  and no-credit block response reasons.
- `packages/open-bitcoin-node/src/sync/runtime_state.rs` - Durable sync state,
  status projection, metrics persistence, structured logs, and recovery
  precedence.
- `packages/open-bitcoin-node/src/sync/types.rs` - Sync runtime config,
  summaries, stop reasons, peer outcomes, and runtime errors.
- `packages/open-bitcoin-node/src/sync/types/summary.rs` - Status, metrics,
  progress, stop reason, and structured-log projection.
- `packages/open-bitcoin-node/src/sync/tests.rs` - Scripted deterministic
  transport/resolver fixtures for sync behavior.
- `packages/open-bitcoin-node/src/status.rs` - Shared operator sync status,
  active progress, field availability, lifecycle, resource pressure, and
  recovery category contracts.
- `packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs` - Daemon activation,
  preflight, runtime store opening, and worker loop shell.
- `scripts/run-live-mainnet-smoke.ts` - Opt-in live-smoke report fields for
  header/downloaded/connected progress and restart/resume evidence.
- `scripts/verify.sh` - Repo-native deterministic verification contract.
- `docs/operator/runtime-guide.md` - Operator sync activation, status evidence,
  restart/resume, support bundle, and UAT command guidance.
- `docs/architecture/status-snapshot.md` - Shared status snapshot truth
  contract.
- `docs/architecture/operator-observability.md` - Metrics/log/support evidence
  and deterministic verification boundaries.
- `docs/parity/index.json` and `docs/parity/catalog/p2p.md` - Parity roots and
  P2P/sync evidence boundaries.

### Baseline Anchors

- `packages/bitcoin-knots/src/validation.cpp` - Block validation, active-chain
  connection, and validation-state behavior anchor.
- `packages/bitcoin-knots/src/node/chainstate.cpp` - Chainstate lifecycle and
  active-chain behavior anchor.
- `packages/bitcoin-knots/src/node/blockstorage.cpp` - Block index and block
  storage behavior anchor.
- `packages/bitcoin-knots/src/coins.h` and `packages/bitcoin-knots/src/coins.cpp`
  - UTXO view and undo persistence anchors.
- `packages/bitcoin-knots/src/net_processing.cpp` - Peer block/header progress
  attribution and no-credit behavior anchor.
- `packages/bitcoin-knots/src/headerssync.cpp` - Header sync progress anchor.

</canonical_refs>

<code_context>

## Existing Code Insights

### Reusable Assets

- `open-bitcoin-chainstate::Chainstate` already validates contextual blocks,
  updates UTXOs, records undo data, supports disconnect/reorg, and exports a
  complete `ChainstateSnapshot`.
- `open-bitcoin-node::ManagedChainstate` already persists snapshots after
  connect, disconnect, and reorg through a `ChainstateStore` abstraction.
- `FjallNodeStore` already persists chainstate snapshots, header entries,
  block-index entries, downloaded block bodies, runtime metadata, metrics, and
  recovery markers in separate namespaces.
- `DurableSyncRuntime::open` already reloads chainstate snapshots and header
  stores into the in-memory network/chainstate runtime.
- `block_reconcile::reconcile_best_chain` already connects stored best-chain
  blocks, records local block hashes, handles replacement branches, and returns
  storage corruption when an active-chain block body is missing.
- `SyncRunSummary`, `DurableSyncState`, `SyncStatus`, metrics, structured logs,
  and live-smoke parsing already distinguish header, downloaded, and connected
  progress.

### Established Patterns

- Pure consensus and chainstate behavior belongs in first-party core crates;
  `open-bitcoin-node` owns storage/runtime adapters and `open-bitcoind` owns
  process orchestration.
- Public-network checks, live-smoke runs, real service-manager operations, and
  long-running sync are opt-in UAT evidence, not default verification.
- Operator surfaces should render unavailable or blocked evidence explicitly
  instead of substituting zero-like success.
- Storage recovery categories outrank peer retry advice when durable state is
  missing, corrupt, incompatible, or locked.
- New first-party Rust files require parity breadcrumbs, with `none` used only
  when no defensible Knots source anchor exists.

### Integration Points

- Connect `DurableSyncRuntime` progress credit to the durable chainstate
  persistence result so connected height is not credited before persistence.
- Reuse `ManagedChainstate` or a similarly small store adapter for Fjall-backed
  chainstate persistence if the current in-memory network path needs a durable
  bridge.
- Extend deterministic sync tests around stored block reconciliation, same-store
  reopen, invalid block/no-credit outcomes, and storage failure blockers.
- Update status/projection and opt-in live-smoke parsing only where the
  validated active-chain height or cumulative-work evidence needs a clearer
  stable field.

</code_context>

<specifics>

## Specific Ideas

- Prefer an explicit internal state transition such as downloaded -> validated
  -> persisted-connected over comments that rely on callers to remember when a
  block is safe to credit.
- Keep initial implementation boring and auditable: complete snapshots after
  validated connect are acceptable for Phase 68 if they keep restart safety
  simple and deterministic.
- Use repo-local operator command examples in docs if Phase 68 changes UAT
  guidance:
  `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- ...`
  and `bazel run //packages/open-bitcoin-cli:open_bitcoin -- ...`.

</specifics>

<deferred>

## Deferred Ideas

- Phase 69 owns stay-current operation and stronger best-known-tip agreement
  semantics after initial catch-up.
- Phase 70 owns broader branch competition, reorg, peer rotation, and
  no-progress recovery expansion.
- Phase 71 owns long-sync resource-bound proof and durable restart/resume under
  storage pressure.
- Phase 72 owns cross-surface operator observability and support evidence
  closeout.
- Phase 73 owns opt-in public-mainnet UAT command breadth and deterministic
  verification expansion.
- Phase 74 owns v1.6 release-boundary docs, parity roots, and final claim
  closeout.

</deferred>

---

*Phase: 68-full-active-chain-validation-and-durable-persistence*
*Context gathered: 2026-06-11 via yolo discussion*
