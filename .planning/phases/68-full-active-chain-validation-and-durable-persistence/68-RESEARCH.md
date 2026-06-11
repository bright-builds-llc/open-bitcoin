# Phase 68: Full Active-Chain Validation and Durable Persistence - Research

## RESEARCH COMPLETE

### Scope Read

Phase 68 covers SYNC-01 through SYNC-04. The phase is a sync-runtime and
durable-state correctness phase:

- SYNC-01 needs explicit opt-in `open-bitcoind` sync to advance the active chain
  toward the best-known validated peer tip or return a typed blocker.
- SYNC-02 needs status evidence to distinguish header height, downloaded block
  height, connected block height, validated active-chain height, cumulative
  work, and tip freshness.
- SYNC-03 needs same-datadir restart to recover durable active-chain, UTXO,
  undo, block-index, and runtime metadata needed to continue validation.
- SYNC-04 needs block progress credited only after consensus validation and
  durable active-chain connection.

The context deliberately defers Phase 69 stay-current semantics, Phase 70
broader reorg/no-progress recovery, Phase 71 resource-bound proof, Phase 72
cross-surface evidence closeout, Phase 73 opt-in UAT breadth, and Phase 74
release-boundary closeout.

### Existing Assets

- `packages/open-bitcoin-chainstate/src/engine.rs` already validates contextual
  blocks, updates UTXOs, records undo data, supports disconnect/reorg, and
  exports `ChainstateSnapshot`.
- `packages/open-bitcoin-node/src/chainstate.rs` already has
  `ManagedChainstate`, which persists snapshots after connect, disconnect, and
  reorg through a `ChainstateStore`.
- `packages/open-bitcoin-node/src/storage/fjall_store.rs` already persists and
  reopens chainstate snapshots, header entries, block-index entries, downloaded
  block bodies, runtime metadata, metrics, and recovery markers.
- `packages/open-bitcoin-node/src/storage/snapshot_codec.rs` already encodes
  active-chain positions, UTXO records, undo records, header entries, block
  index entries, and runtime metadata as versioned storage DTOs.
- `packages/open-bitcoin-node/src/sync.rs` already reloads chainstate snapshots
  and header stores in `DurableSyncRuntime::open`, reconciles stored best-chain
  blocks before requesting more, and persists progress after receive/reconcile
  paths.
- `packages/open-bitcoin-node/src/sync/block_response.rs` saves requested
  best-chain connected block bodies, records accepted block credit, and keeps
  duplicate/disconnected/non-extending block responses as no-credit peer
  outcomes.
- `packages/open-bitcoin-node/src/sync/runtime_state.rs` already projects
  header, downloaded, and connected progress into durable status, metrics, and
  structured logs.
- Existing sync tests cover requested block connection, downloaded-only status,
  unrequested block no-credit, duplicate/disconnected/non-extending no-credit,
  invalid/malformed block no-credit, in-flight cleanup, and several status
  projections.
- Existing Fjall tests cover reopening chainstate snapshots, header store,
  block index, block bodies, runtime metadata, metrics, and wallet state.

### Planning-Critical Gaps

1. **Persisted-connected proof is implicit.** `persist_progress()` writes
   headers, chainstate snapshot, and runtime metadata after receive/reconcile
   paths, but tests should prove a connected block remains connected after
   reopening a new `DurableSyncRuntime` from the same Fjall store.
2. **Validated active-chain height is not an explicit status name.** Existing
   status uses `connected_block_height` and `block_height` as the compatibility
   alias. Phase 68 can either document that connected height is the validated
   active-chain height or add an additive stable field if the planner finds a
   low-risk path.
3. **Cumulative work evidence exists in `ChainPosition` and `HeaderEntry`, but
   operator-facing status may not expose it directly for the active connected
   tip.** Plan should decide whether Phase 68 needs a narrow additive projection
   or can leave richer tip evidence to Phase 69.
4. **Storage failure blockers need explicit tests at the credit boundary.**
   The code maps storage failures to `SyncRuntimeError::Storage` and storage
   recovery categories, but Phase 68 should prove storage errors prevent false
   connected-progress credit in the relevant runtime path.
5. **Docs already describe downloaded and connected progress.** Updates should
   be narrow and evidence-focused, avoiding Phase 72/74 broad observability or
   release-boundary work.

### Recommended Plan Shape

Use three focused plans.

1. **Durable connected-progress proof.** Add deterministic sync tests proving a
   requested best-chain block connects, persists block body and chainstate, and
   reopens with the same connected active-chain height/hash/chain work. Include
   downloaded-only and invalid/no-credit assertions where current coverage is
   missing.
2. **Credit-boundary and blocker hardening.** Tighten or codify the internal
   result path so downloaded-only, no-credit peer responses, validation errors,
   and storage errors cannot be reported as persisted connected progress. Add a
   small result enum/helper only if it removes ambiguity.
3. **Evidence and docs alignment.** Add or refine the minimal status/docs/checker
   evidence needed for SYNC-02 through SYNC-04, including active-chain/cumulative
   work wording, parity breadcrumbs if new files are created, and repo-local UAT
   command examples if operator docs change.

### Validation Architecture

Phase verification should include:

- Focused Rust tests for `DurableSyncRuntime` same-store reopen after connected
  block progress.
- Rust tests that downloaded-only stored block bodies do not advance connected
  active-chain height.
- Rust tests that invalid, disconnected, duplicate, non-extending, malformed,
  and `notfound` responses remain no-credit where not already covered.
- Rust tests or assertions that storage failures surface a typed storage
  blocker and do not produce false connected progress.
- If status fields change, tests that `SyncProgress` keeps header,
  downloaded, connected, active-chain, hash, and work evidence coherent.
- If docs/checkers change, deterministic Bun checks wired into `bash
  scripts/verify.sh`; no public-network or real service-manager commands in
  default verification.

### Risks And Constraints

- Do not replace the existing first-party chainstate and storage model with a
  new persistence layer.
- Do not add existing Rust Bitcoin libraries to the production path.
- Do not broaden the claim to inbound serving, relay, production funds,
  packaging, migration apply mode, hosted dashboards, GUI, or production-node
  readiness.
- Keep public-mainnet sync attempts opt-in UAT evidence.
- New first-party Rust source or test files under `packages/open-bitcoin-*`
  require parity breadcrumbs.
- Complete snapshot persistence is acceptable for this phase if it keeps the
  restart-safe proof simpler than incremental UTXO/undo persistence.

### Verification Commands

Use focused checks during implementation, then the aggregate repo gate:

```bash
cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node sync::
bash scripts/verify.sh
```

`bash scripts/verify.sh` is the final repo-native verification contract.
