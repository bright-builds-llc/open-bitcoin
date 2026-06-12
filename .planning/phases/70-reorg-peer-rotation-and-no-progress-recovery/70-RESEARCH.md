# Phase 70: Reorg, Peer Rotation, and No-Progress Recovery - Research

**Researched:** 2026-06-12  
**Domain:** Rust Bitcoin sync runtime, durable chainstate reorgs, peer rotation, and operator status diagnosis  
**Confidence:** HIGH

<user_constraints>
## User Constraints (from CONTEXT.md)

All constraints in this section are copied verbatim from `.planning/phases/70-reorg-peer-rotation-and-no-progress-recovery/70-CONTEXT.md`. [VERIFIED: .planning/phases/70-reorg-peer-rotation-and-no-progress-recovery/70-CONTEXT.md]

### Locked Decisions

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

### Deferred Ideas (OUT OF SCOPE)

No `## Deferred Ideas` section is present in the Phase 70 context; the phase boundary explicitly excludes broader resource-bound proof, cross-surface observability closeout, public-network/default verification, inbound serving, relay, production-wallet claims, migration apply mode, packaging, Windows service support, GUI work, hosted dashboards, and broad production-node claims. [VERIFIED: .planning/phases/70-reorg-peer-rotation-and-no-progress-recovery/70-CONTEXT.md]
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| REC-01 | Operator can survive competing header branches through cumulative-work selection with deterministic active-chain outcomes. [VERIFIED: .planning/REQUIREMENTS.md] | Use `HeaderStore::update_best_tip` ordering and `best_chain_entries`, then gate active-chain replacement on available, validated, persisted block bodies. [VERIFIED: packages/open-bitcoin-network/src/header_store.rs; packages/open-bitcoin-node/src/sync/block_reconcile.rs] |
| REC-02 | Operator can survive reorgs through durable disconnect and reconnect behavior with bounded undo evidence. [VERIFIED: .planning/REQUIREMENTS.md] | Reuse `Chainstate::reorg`, `ManagedChainstate::reorg`, and `ManagedPeerNetwork::reorg_to_branch`; add bounded `ReorgEvidence` projection instead of raw undo dumps. [VERIFIED: packages/open-bitcoin-chainstate/src/engine.rs; packages/open-bitcoin-node/src/chainstate.rs; packages/open-bitcoin-node/src/network.rs; .planning/phases/70-reorg-peer-rotation-and-no-progress-recovery/70-CONTEXT.md] |
| REC-03 | Operator can recover from stale, slow, incompatible, malformed, invalid, disconnecting, or `notfound` peers through typed attribution, retry/backoff, and peer rotation. [VERIFIED: .planning/REQUIREMENTS.md] | Extend existing `PeerFailureReason`, endpoint-keyed `PeerRetryState`, in-flight release, and peer outcome summaries. [VERIFIED: packages/open-bitcoin-node/src/sync/types.rs; packages/open-bitcoin-node/src/sync/progress.rs; packages/open-bitcoin-node/src/sync.rs; packages/open-bitcoin-node/src/sync/block_reconcile.rs] |
| REC-04 | Operator can see typed no-progress causes and next actions when sync is behind, stalled, at tip, or recovering from stale in-flight work. [VERIFIED: .planning/REQUIREMENTS.md] | Add a shared typed diagnosis derived from Phase 69 `StayCurrentStatus`, `BestKnownTipStatus`, `SyncProgressSignal`, peer outcomes, resource pressure, and recovery category. [VERIFIED: packages/open-bitcoin-node/src/status.rs; packages/open-bitcoin-node/src/sync/runtime_state.rs; packages/open-bitcoin-node/src/sync/tip.rs; .planning/phases/69-tip-tracking-and-stay-current-operation/69-VERIFICATION.md] |
</phase_requirements>

## Project Constraints (from AGENTS.md)

- Prefer root `AGENTS.md`; it exists and requires reading `AGENTS.bright-builds.md`, `standards-overrides.md`, and relevant Bright Builds standards before planning, review, implementation, or audit work. [VERIFIED: AGENTS.md; AGENTS.bright-builds.md]
- Use `rust-toolchain.toml` as the Rust source of truth; the pinned Rust version is `1.94.1`. [VERIFIED: AGENTS.md; rust-toolchain.toml]
- Use `bash scripts/verify.sh` as the repo-native verification contract for first-party code, including Bazel smoke build. [VERIFIED: AGENTS.md; scripts/verify.sh]
- Public-network checks and service-manager operations must stay out of default verification. [VERIFIED: AGENTS.md; .planning/REQUIREMENTS.md; scripts/verify.sh]
- Use Bun for repo-owned higher-level TypeScript automation scripts; this repo has no `package.json`, so there is no `bun install` bootstrap step. [VERIFIED: AGENTS.md; .planning/PROJECT.md]
- New first-party Rust source or test files under `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests` require parity breadcrumb coverage in `docs/parity/source-breadcrumbs.json` and `scripts/check-parity-breadcrumbs.ts`. [VERIFIED: AGENTS.md; docs/parity/source-breadcrumbs.json; scripts/check-parity-breadcrumbs.ts]
- Preserve Bitcoin Knots `29.3.knots20260210` behavior for in-scope external surfaces and keep parity evidence auditable. [VERIFIED: AGENTS.md; .planning/PROJECT.md]
- Keep pure Bitcoin domain behavior in functional-core crates and isolate filesystem, process, network, terminal, RPC, service-manager, and durable-storage effects in shell adapters. [VERIFIED: AGENTS.md; .planning/PROJECT.md; AGENTS.bright-builds.md]
- Do not use existing Rust Bitcoin libraries in the production path. [VERIFIED: AGENTS.md; .planning/PROJECT.md]
- For Rust work, run `cargo fmt --all`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo build --all-targets --all-features`, and `cargo test --all-features` before commits; the repo-native `scripts/verify.sh` covers the active workspace with stricter local checks. [VERIFIED: AGENTS.md; scripts/verify.sh]
- No project-local `.claude/skills` or `.agents/skills` directories exist. [VERIFIED: `find .claude/skills .agents/skills -maxdepth 2 -name SKILL.md -print`]

## Summary

Phase 70 should be planned as an incremental extension of the existing sync runtime, not as a new chain selection or peer lifecycle subsystem. `HeaderStore` already implements deterministic best-tip ordering by cumulative work, height, and hash; `block_reconcile::reconcile_best_chain` already waits for replacement branch block bodies before reorg and treats missing active-chain block bodies as storage corruption. [VERIFIED: packages/open-bitcoin-network/src/header_store.rs; packages/open-bitcoin-node/src/sync/block_reconcile.rs]

The core reorg engine already exists in `Chainstate::reorg`, and `ManagedChainstate` persists a complete chainstate snapshot after connect, disconnect, and reorg. Phase 70 should add bounded reorg evidence and stronger adapter outcomes around that engine, not duplicate disconnect/reconnect logic. [VERIFIED: packages/open-bitcoin-chainstate/src/engine.rs; packages/open-bitcoin-node/src/chainstate.rs; .planning/phases/70-reorg-peer-rotation-and-no-progress-recovery/70-CONTEXT.md]

Peer outcome attribution, in-flight cleanup, endpoint-keyed retry/backoff, and Phase 69 stay-current status already cover much of the foundation. The missing planning center is a shared typed no-progress diagnosis and next-action contract that derives from existing status evidence and feeds CLI/RPC/dashboard/log/docs later without renderer-local strings. [VERIFIED: packages/open-bitcoin-node/src/sync.rs; packages/open-bitcoin-node/src/sync/types.rs; packages/open-bitcoin-node/src/status.rs; .planning/phases/69-tip-tracking-and-stay-current-operation/69-VERIFICATION.md]

**Primary recommendation:** Use the existing stack and add small pure status/result types: `ReconcileOutcome`/`ReorgEvidence` around `reconcile_best_chain`, and `NoProgressDiagnosis`/`NoProgressNextAction` in shared status projection, with deterministic Rust tests and one Bun checker for docs/verify boundary drift. [VERIFIED: .planning/phases/70-reorg-peer-rotation-and-no-progress-recovery/70-CONTEXT.md; packages/open-bitcoin-node/src/sync/block_reconcile.rs; packages/open-bitcoin-node/src/status.rs; scripts/check-phase69-tip-stay-current.ts]

## Standard Stack

### Core

| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| Rust workspace crates under `packages/open-bitcoin-*` | `0.1.0` workspace package version | First-party Bitcoin primitives, consensus, chainstate, network, node runtime, RPC, and CLI surfaces. | The project explicitly owns its production Bitcoin domain model and avoids external Rust Bitcoin libraries. [VERIFIED: packages/Cargo.toml; .planning/PROJECT.md] |
| Rust toolchain | `1.94.1` | Compile, lint, format, and test Phase 70 Rust code. | The repo pins Rust through `rust-toolchain.toml` and `AGENTS.md` names it as the source of truth. [VERIFIED: rust-toolchain.toml; AGENTS.md] |
| `open-bitcoin-network::HeaderStore` | first-party `0.1.0` | Header storage, best-tip ordering, best-chain entries, ancestor lookup, and locators. | It already implements cumulative-work, height, and hash tie-break ordering required by REC-01. [VERIFIED: packages/open-bitcoin-network/src/header_store.rs] |
| `open-bitcoin-chainstate::Chainstate` | first-party `0.1.0` | Active-chain connect, disconnect, UTXO updates, undo data, and reorg transitions. | It already validates blocks, records undo data, disconnects tips, and reorgs through `ChainTransition`. [VERIFIED: packages/open-bitcoin-chainstate/src/engine.rs; packages/open-bitcoin-chainstate/src/types.rs] |
| `open-bitcoin-node::DurableSyncRuntime` | first-party `0.1.0` | Effectful sync shell around peer transport, durable storage, metrics/logs, and status persistence. | Phase 68 and Phase 69 wired durable active-chain progress and stay-current status through this runtime. [VERIFIED: packages/open-bitcoin-node/src/sync.rs; .planning/phases/68-full-active-chain-validation-and-durable-persistence/68-VERIFICATION.md; .planning/phases/69-tip-tracking-and-stay-current-operation/69-VERIFICATION.md] |
| Fjall | `3.1.4` | Durable local key-value storage for headers, block index, block bodies, chainstate snapshots, runtime metadata, metrics, and recovery markers. | The current `FjallNodeStore` owns the durable storage namespaces Phase 70 must preserve. [VERIFIED: packages/open-bitcoin-node/Cargo.toml; packages/open-bitcoin-node/src/storage/fjall_store.rs] |

### Supporting

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| `serde` | `1.0.228` | Stable serialization for shared status, runtime metadata, and DTOs. | Use for new shared status fields such as bounded reorg evidence or no-progress diagnosis. [VERIFIED: packages/open-bitcoin-node/Cargo.toml; packages/open-bitcoin-node/src/status.rs] |
| `serde_json` | `1.0.149` | JSON status, runtime metadata tests, and deterministic checker fixtures. | Use only where existing JSON status/checker patterns require it. [VERIFIED: packages/open-bitcoin-node/Cargo.toml; packages/open-bitcoin-node/src/sync/tests.rs] |
| Bun | `1.3.9` | Repo-owned TypeScript checkers. | Use for a focused Phase 70 checker if docs, status strings, or `scripts/verify.sh` boundaries change. [VERIFIED: .bun-version; scripts/verify.sh] |
| Bazel/Bazelisk command surface | local `bazel 8.6.0` available | Top-level smoke build in repo-native verification. | Do not replace or bypass the existing `bazel build //:core //:node //:rpc //:cli //:test_harness //:bench` verifier step. [VERIFIED: `bazel --version`; scripts/verify.sh] |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| `HeaderStore` best-tip policy | External tip oracle, checkpoint, assumevalid, assumeutxo, or trusted peer | Rejected by locked Phase 70 decisions and v1.6 out-of-scope boundaries. [VERIFIED: .planning/phases/70-reorg-peer-rotation-and-no-progress-recovery/70-CONTEXT.md; .planning/REQUIREMENTS.md] |
| `Chainstate::reorg` and `ManagedChainstate` | A second sync-runtime reorg engine | Rejected because the first-party chainstate already owns undo/connect/disconnect semantics and Phase 70 requires reuse. [VERIFIED: .planning/phases/70-reorg-peer-rotation-and-no-progress-recovery/70-CONTEXT.md; packages/open-bitcoin-chainstate/src/engine.rs] |
| Existing endpoint-keyed backoff | Production peer banning or reputation governance | Rejected because broad peer eviction, banning, inbound reputation, address-manager governance, compact-block fallback, and transaction relay policy are explicitly out of scope. [VERIFIED: .planning/phases/70-reorg-peer-rotation-and-no-progress-recovery/70-CONTEXT.md] |
| Existing first-party crates | `rust-bitcoin` or other Rust Bitcoin production-path crates | Rejected by the project dependency policy. [VERIFIED: AGENTS.md; .planning/PROJECT.md] |

**Installation:**

No new packages are recommended for Phase 70. [VERIFIED: packages/Cargo.toml; packages/open-bitcoin-node/Cargo.toml; .planning/phases/70-reorg-peer-rotation-and-no-progress-recovery/70-CONTEXT.md]

```bash
# No npm, pnpm, cargo add, or bun install step is required.
```

**Version verification:** The phase does not recommend npm packages, so `npm view` is not applicable. Versions above were verified from `Cargo.toml`, `rust-toolchain.toml`, `.bun-version`, and local CLI probes. [VERIFIED: packages/Cargo.toml; packages/open-bitcoin-node/Cargo.toml; rust-toolchain.toml; .bun-version; `rustc --version`; `cargo --version`; `bun --version`; `bazel --version`; `cargo llvm-cov --version`]

## Architecture Patterns

### Recommended Project Structure

Keep work in the existing module boundaries unless a new small helper module is justified. [VERIFIED: packages/open-bitcoin-node/src/sync.rs; packages/open-bitcoin-node/src/status.rs; AGENTS.bright-builds.md]

```text
packages/open-bitcoin-chainstate/src/
  engine.rs              # Pure connect/disconnect/reorg and undo semantics. [VERIFIED: packages/open-bitcoin-chainstate/src/engine.rs]
  types.rs               # ChainPosition, ChainTransition, BlockUndo, snapshots. [VERIFIED: packages/open-bitcoin-chainstate/src/types.rs]

packages/open-bitcoin-network/src/
  header_store.rs        # Deterministic best-tip and best-chain evidence. [VERIFIED: packages/open-bitcoin-network/src/header_store.rs]

packages/open-bitcoin-node/src/
  status.rs              # Shared operator status contracts. Add bounded reorg/no-progress status types here if externally surfaced. [VERIFIED: packages/open-bitcoin-node/src/status.rs]
  sync.rs                # DurableSyncRuntime orchestration and peer loop. [VERIFIED: packages/open-bitcoin-node/src/sync.rs]
  sync/block_reconcile.rs# Best-chain stored-block reconciliation and reorg adapter boundary. [VERIFIED: packages/open-bitcoin-node/src/sync/block_reconcile.rs]
  sync/block_response.rs # Requested/unrequested block attribution and no-credit behavior. [VERIFIED: packages/open-bitcoin-node/src/sync/block_response.rs]
  sync/progress.rs       # Peer progress, failure attribution helpers, backoff helpers. [VERIFIED: packages/open-bitcoin-node/src/sync/progress.rs]
  sync/runtime_state.rs  # Durable status projection and metadata persistence. [VERIFIED: packages/open-bitcoin-node/src/sync/runtime_state.rs]
  sync/tip.rs            # Existing pure best-tip/stay-current helper pattern. [VERIFIED: packages/open-bitcoin-node/src/sync/tip.rs]
  sync/tests.rs          # Deterministic sync, reorg, peer, in-flight, and status tests. [VERIFIED: packages/open-bitcoin-node/src/sync/tests.rs]

scripts/
  check-phase70-reorg-recovery.ts # Add only if docs/status boundary strings change. [VERIFIED: scripts/check-phase69-tip-stay-current.ts]
```

### Pattern 1: Header Evidence Can Lead, Active Chain Must Wait

**What:** Let the header store identify the best header branch, but only apply active-chain replacement after every replacement block body needed from the common ancestor forward is durable and validated. [VERIFIED: packages/open-bitcoin-node/src/sync/block_reconcile.rs; .planning/phases/70-reorg-peer-rotation-and-no-progress-recovery/70-CONTEXT.md]

**When to use:** Use this for REC-01 and REC-02 branch competition planning. [VERIFIED: .planning/REQUIREMENTS.md]

**Example:**

```rust
// Source: packages/open-bitcoin-node/src/sync/block_reconcile.rs
let active_chain = runtime.network.chainstate_snapshot().active_chain;
let best_chain = runtime.network.best_chain_entries();
let common_prefix_len = active_chain
    .iter()
    .zip(best_chain.iter())
    .take_while(|(active, best)| active.block_hash == best.block_hash)
    .count();

// Phase 70 should preserve this shape: collect replacement blocks first,
// then load active-chain disconnect bodies, then call reorg_to_branch.
```

### Pattern 2: Reuse Pure Chainstate Reorg, Add Adapter Evidence

**What:** Keep disconnect/reconnect inside `Chainstate::reorg`; let the node shell load durable block bodies, persist snapshots, and project bounded evidence. [VERIFIED: packages/open-bitcoin-chainstate/src/engine.rs; packages/open-bitcoin-node/src/chainstate.rs; packages/open-bitcoin-node/src/sync/block_reconcile.rs]

**When to use:** Use this for REC-02 and for storage-blocker mapping. [VERIFIED: .planning/REQUIREMENTS.md; .planning/phases/70-reorg-peer-rotation-and-no-progress-recovery/70-CONTEXT.md]

**Example:**

```rust
// Source: packages/open-bitcoin-node/src/network.rs and packages/open-bitcoin-node/src/chainstate.rs
let transition = self.chainstate.reorg(
    disconnect_blocks,
    replacement_branch,
    verify_flags,
    consensus_params,
)?;
```

### Pattern 3: Typed Peer Outcome First, Renderer Text Last

**What:** Convert peer behavior into `PeerSyncOutcome` with `PeerSyncState`, `PeerFailureReason`, contribution counters, optional tip evidence, and optional error detail; renderers consume those typed fields. [VERIFIED: packages/open-bitcoin-node/src/sync/types.rs; packages/open-bitcoin-node/src/sync/progress.rs; packages/open-bitcoin-node/src/sync/types/projection.rs]

**When to use:** Use this for REC-03 peer rotation and REC-04 no-progress diagnosis. [VERIFIED: .planning/REQUIREMENTS.md]

**Example:**

```rust
// Source: packages/open-bitcoin-node/src/sync/progress.rs
pub(super) fn record_block_notfound(&mut self) {
    self.record_no_credit_block_response(PeerFailureReason::BlockNotFound);
}
```

### Pattern 4: Shared Status Projection Owns Operator Truth

**What:** Derive operator-facing state in `runtime_state.rs` from `SyncRunSummary`, durable metadata, peer outcomes, resource pressure, best tip evidence, and recovery category. [VERIFIED: packages/open-bitcoin-node/src/sync/runtime_state.rs; packages/open-bitcoin-node/src/status.rs]

**When to use:** Use this for the new Phase 70 no-progress diagnosis and next-action fields so CLI, RPC, dashboard, logs, metrics, and support evidence do not diverge later. [VERIFIED: .planning/phases/70-reorg-peer-rotation-and-no-progress-recovery/70-CONTEXT.md; .planning/phases/72 not yet started in .planning/ROADMAP.md]

**Example:**

```rust
// Recommended Phase 70 shape, following packages/open-bitcoin-node/src/sync/tip.rs.
let diagnosis = no_progress::classify_no_progress(&NoProgressInput {
    stay_current,
    progress_signal,
    peer_outcomes,
    resource_pressure,
    maybe_recovery_category,
    maybe_reorg_evidence,
});
sync.no_progress_diagnosis = FieldAvailability::available(diagnosis);
```

### Anti-Patterns to Avoid

- **Reorging on headers alone:** Headers may identify a better branch, but active-chain progress cannot move until replacement bodies are available, validated, connected, and durably persisted. [VERIFIED: .planning/phases/70-reorg-peer-rotation-and-no-progress-recovery/70-CONTEXT.md; packages/open-bitcoin-node/src/sync/block_reconcile.rs]
- **Counting downloaded-only blocks as progress:** Phase 68 requires connected validated active-chain height as progress credit. [VERIFIED: .planning/phases/68-full-active-chain-validation-and-durable-persistence/68-CONTEXT.md; .planning/phases/68-full-active-chain-validation-and-durable-persistence/68-VERIFICATION.md]
- **Flattening peer failures into network errors:** Existing code distinguishes address resolution, compatibility, connect, stall, retry backoff, invalid data, invalid magic, `notfound`, malformed block, invalid block, duplicate block, disconnected block, non-extending block, network, resource, and storage reasons. [VERIFIED: packages/open-bitcoin-node/src/sync/types.rs]
- **Renderer-local no-progress strings:** Phase 70 requires typed no-progress diagnosis from shared evidence. [VERIFIED: .planning/phases/70-reorg-peer-rotation-and-no-progress-recovery/70-CONTEXT.md]
- **Raw undo dumps in operator status:** Phase 70 requires bounded reorg evidence, not raw undo payloads. [VERIFIED: .planning/phases/70-reorg-peer-rotation-and-no-progress-recovery/70-CONTEXT.md]
- **Adding public-network checks to default verification:** v1.6 requirements and scripts keep default verification deterministic and public-network-free. [VERIFIED: .planning/REQUIREMENTS.md; scripts/verify.sh]

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Best branch selection | External tip source, trusted peer, or ad hoc branch sorter | `HeaderStore::best_tip`, `HeaderStore::best_chain_entries`, and existing tie-break policy | It already implements the locked cumulative-work, height, hash ordering. [VERIFIED: packages/open-bitcoin-network/src/header_store.rs; .planning/phases/70-reorg-peer-rotation-and-no-progress-recovery/70-CONTEXT.md] |
| Disconnect/reconnect reorg engine | A second runtime-level reorg algorithm | `Chainstate::reorg` through `ManagedChainstate` and `ManagedPeerNetwork::reorg_to_branch` | Existing code owns UTXO/undo mutation, validation, and snapshot persistence. [VERIFIED: packages/open-bitcoin-chainstate/src/engine.rs; packages/open-bitcoin-node/src/chainstate.rs; packages/open-bitcoin-node/src/network.rs] |
| Durable storage health taxonomy | New storage error strings | `StorageError`, `StorageRecoveryAction`, `SyncRecoveryCategory` | Storage blockers must outrank peer retry advice. [VERIFIED: packages/open-bitcoin-node/src/storage.rs; packages/open-bitcoin-node/src/sync/runtime_state.rs; .planning/phases/61-resource-bounds-and-recovery-taxonomy/61-CONTEXT.md] |
| Peer retry and rotation | Peer banning, reputation scoring, production eviction | Endpoint-keyed `PeerRetryState`, `mark_backoff`, `maybe_peer_backoff`, and bounded peer iteration | The phase locks endpoint backoff as the default and excludes broad production peer governance. [VERIFIED: packages/open-bitcoin-node/src/sync.rs; .planning/phases/70-reorg-peer-rotation-and-no-progress-recovery/70-CONTEXT.md] |
| Block response attribution | Renderer-specific or transport-specific strings | `PeerFailureReason` and `PeerSyncOutcome` | Existing typed outcomes already cover no-credit block responses and operator recovery actions. [VERIFIED: packages/open-bitcoin-node/src/sync/types.rs; packages/open-bitcoin-node/src/sync/block_response.rs] |
| No-progress diagnosis | A new CLI-only model | Shared status types in `status.rs` plus pure helper under `sync/` | Phase 70 requires shared evidence so later CLI/RPC/dashboard/log/support surfaces stay aligned. [VERIFIED: packages/open-bitcoin-node/src/status.rs; .planning/phases/70-reorg-peer-rotation-and-no-progress-recovery/70-CONTEXT.md] |
| Verification | Public mainnet peer tests in `scripts/verify.sh` | Deterministic Rust tests plus optional Bun checker | Default verification must remain public-network-free and service-manager-free. [VERIFIED: .planning/REQUIREMENTS.md; scripts/verify.sh] |

**Key insight:** Phase 70 is mostly about making existing decisions observable and atomic, not inventing new consensus or peer management machinery. [VERIFIED: packages/open-bitcoin-node/src/sync/block_reconcile.rs; packages/open-bitcoin-chainstate/src/engine.rs; .planning/phases/70-reorg-peer-rotation-and-no-progress-recovery/70-CONTEXT.md]

## Common Pitfalls

### Pitfall 1: Applying a Better Header Branch Before Bodies Exist

**What goes wrong:** The active chain can be disconnected before the replacement branch is actually connectable. [VERIFIED: .planning/phases/70-reorg-peer-rotation-and-no-progress-recovery/70-CONTEXT.md]

**Why it happens:** Header-store best-chain evidence can outrank the active chain before all replacement block bodies have been downloaded. [VERIFIED: packages/open-bitcoin-node/src/sync/block_reconcile.rs]

**How to avoid:** Make `reconcile_best_chain` return an explicit awaiting-bodies outcome when the best header branch outranks but replacement bodies are incomplete. [VERIFIED: packages/open-bitcoin-node/src/sync/block_reconcile.rs; .planning/phases/70-reorg-peer-rotation-and-no-progress-recovery/70-CONTEXT.md]

**Warning signs:** `best_header_height` or best tip work is ahead while `connected_block_height` remains on the old branch and no typed `awaiting_block_bodies` or `branch_competition_awaiting_bodies` diagnosis is available. [VERIFIED: packages/open-bitcoin-node/src/status.rs; docs/architecture/status-snapshot.md]

### Pitfall 2: Treating Missing Active-Chain Blocks as Peer Retry

**What goes wrong:** The operator gets advice to retry peers even though the local store cannot safely disconnect the current active chain. [VERIFIED: .planning/phases/70-reorg-peer-rotation-and-no-progress-recovery/70-CONTEXT.md]

**Why it happens:** Disconnect needs durable active-chain block bodies and undo data; missing active-chain block bodies are local storage corruption in current code. [VERIFIED: packages/open-bitcoin-node/src/sync/block_reconcile.rs; packages/open-bitcoin-chainstate/src/engine.rs]

**How to avoid:** Map missing active-chain bodies, missing undo data, and malformed chainstate to storage recovery category/action before peer guidance. [VERIFIED: packages/open-bitcoin-node/src/storage.rs; packages/open-bitcoin-node/src/sync/runtime_state.rs; .planning/phases/61-resource-bounds-and-recovery-taxonomy/61-CONTEXT.md]

**Warning signs:** `PeerFailureReason::BlockNotFound` appears for a failure that actually came from loading an active-chain disconnect body. [VERIFIED: packages/open-bitcoin-node/src/sync/types.rs; packages/open-bitcoin-node/src/sync/block_reconcile.rs]

### Pitfall 3: Losing Bounded Reorg Evidence

**What goes wrong:** A successful reorg changes the active tip but status cannot explain common ancestor, disconnected count, connected count, final tip, or persistence status. [VERIFIED: .planning/phases/70-reorg-peer-rotation-and-no-progress-recovery/70-CONTEXT.md]

**Why it happens:** `ChainTransition` currently carries disconnected and connected positions, but `reconcile_best_chain` returns only `bool`. [VERIFIED: packages/open-bitcoin-chainstate/src/types.rs; packages/open-bitcoin-node/src/sync/block_reconcile.rs]

**How to avoid:** Replace the internal `bool` with a bounded `ReconcileOutcome` that can preserve `ReorgEvidence` until runtime status persistence. [VERIFIED: packages/open-bitcoin-node/src/sync/block_reconcile.rs; packages/open-bitcoin-node/src/sync/runtime_state.rs]

**Warning signs:** Tests only assert final height after reorg and do not assert transition evidence or persisted status. [VERIFIED: packages/open-bitcoin-node/src/sync/tests.rs]

### Pitfall 4: Stale In-Flight Requests Keep the Runtime Looking Busy

**What goes wrong:** A block remains in `runtime.inflight_blocks` after `notfound`, invalid/malformed response, peer disconnect, or restart, so the runtime stops requesting it from eligible peers. [VERIFIED: packages/open-bitcoin-node/src/sync/block_reconcile.rs; packages/open-bitcoin-node/src/sync.rs; packages/open-bitcoin-node/src/sync/tests.rs]

**Why it happens:** In-flight state exists both in runtime-level `inflight_blocks` and peer-manager requested-block bookkeeping. [VERIFIED: packages/open-bitcoin-node/src/sync.rs; packages/open-bitcoin-node/src/sync/block_reconcile.rs]

**How to avoid:** Keep `release_inflight_for_message` and disconnect cleanup as the only release paths, then add tests for each no-credit outcome and restart-open state. [VERIFIED: packages/open-bitcoin-node/src/sync/block_reconcile.rs; packages/open-bitcoin-node/src/sync.rs; packages/open-bitcoin-node/src/sync/tests.rs]

**Warning signs:** `blocks_in_flight` stays nonzero while no peer has an outstanding request or while all peers are in backoff. [VERIFIED: packages/open-bitcoin-node/src/sync/runtime_state.rs; docs/architecture/status-snapshot.md]

### Pitfall 5: No-Progress Means Too Many Things

**What goes wrong:** Operators cannot tell whether no progress means at tip, behind headers, awaiting blocks, backoff, stalled peer, exhausted peers, branch competition, recovering, storage blocked, or resource blocked. [VERIFIED: .planning/phases/70-reorg-peer-rotation-and-no-progress-recovery/70-CONTEXT.md]

**Why it happens:** Phase 69 added `StayCurrentStatus`, but current `SyncProgressSignal` and `StayCurrentStatus::NoProgress` are still coarse for REC-04. [VERIFIED: packages/open-bitcoin-node/src/status.rs; packages/open-bitcoin-node/src/sync/tip.rs; docs/operator/runtime-guide.md]

**How to avoid:** Add a shared `NoProgressDiagnosis` enum and derive it from `StayCurrentStatus`, `SyncProgressSignal`, peer outcomes, resource pressure, recovery category, and reorg outcome. [VERIFIED: packages/open-bitcoin-node/src/status.rs; packages/open-bitcoin-node/src/sync/runtime_state.rs; .planning/phases/70-reorg-peer-rotation-and-no-progress-recovery/70-CONTEXT.md]

**Warning signs:** CLI/dashboard/docs contain new strings that do not correspond to stable status enum variants. [VERIFIED: scripts/check-phase69-tip-stay-current.ts; packages/open-bitcoin-cli/src/operator/status/render.rs]

## Code Examples

Verified patterns from local sources:

### Deterministic Best-Tip Ordering

```rust
// Source: packages/open-bitcoin-network/src/header_store.rs
if candidate.chain_work != current.chain_work {
    return candidate.chain_work > current.chain_work;
}
if candidate.height != current.height {
    return candidate.height > current.height;
}
candidate.block_hash > current.block_hash
```

This is the REC-01 policy; Phase 70 should test it through runtime branch outcomes, not just `HeaderStore` unit tests. [VERIFIED: packages/open-bitcoin-network/src/header_store.rs; .planning/phases/70-reorg-peer-rotation-and-no-progress-recovery/70-CONTEXT.md]

### Storage-Blocked Missing Active-Chain Body

```rust
// Source: packages/open-bitcoin-node/src/sync/block_reconcile.rs
let Some(block) = runtime.store.load_block(position.block_hash)? else {
    return Err(SyncRuntimeError::Storage(crate::StorageError::Corruption {
        namespace: StorageNamespace::BlockIndex,
        detail: format!(
            "missing durable block body for active chain block {:?}",
            position.block_hash
        ),
        action: StorageRecoveryAction::Repair,
    }));
};
```

This is the right storage-first failure class for disconnect blockers. [VERIFIED: packages/open-bitcoin-node/src/sync/block_reconcile.rs; .planning/phases/70-reorg-peer-rotation-and-no-progress-recovery/70-CONTEXT.md]

### Peer Backoff and Rotation

```rust
// Source: packages/open-bitcoin-node/src/sync.rs
if let Some(backoff) = self.maybe_peer_backoff(&peer, timestamp) {
    self.record_waiting_outcome(&mut summary, &peer, backoff, timestamp);
    continue;
}
```

This supports REC-03 by preserving a waiting peer outcome while allowing other resolved peers to fill outbound slots. [VERIFIED: packages/open-bitcoin-node/src/sync.rs; packages/open-bitcoin-node/src/sync/tests.rs]

### Shared Status Projection

```rust
// Source: packages/open-bitcoin-node/src/sync/runtime_state.rs
sync.resource_pressure = FieldAvailability::available(SyncResourcePressure {
    blocks_in_flight: self.inflight_blocks.len() as u64,
    max_blocks_in_flight_per_peer: self.config.max_blocks_in_flight_per_peer as u64,
    max_blocks_in_flight_total: self.config.max_blocks_in_flight_total as u64,
    max_sync_rounds: self.config.max_rounds as u64,
    outbound_peers: summary.connected_peers as u32,
    target_outbound_peers: self.config.target_outbound_peers as u32,
    .. /* existing fields omitted */
});
```

Phase 70 no-progress diagnosis should be projected in this same shared status path. [VERIFIED: packages/open-bitcoin-node/src/sync/runtime_state.rs; .planning/phases/70-reorg-peer-rotation-and-no-progress-recovery/70-CONTEXT.md]

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Header or downloaded block progress could be mistaken for active-chain progress. | Connected validated active-chain height/hash/work is the progress credit. | Phase 68, verified 2026-06-11. [VERIFIED: .planning/phases/68-full-active-chain-validation-and-durable-persistence/68-VERIFICATION.md] | Phase 70 must not credit reorg progress until the new active tip is persisted. [VERIFIED: .planning/phases/70-reorg-peer-rotation-and-no-progress-recovery/70-CONTEXT.md] |
| Stay-current and no-progress status were coarse. | `BestKnownTipStatus`, `StayCurrentStatus`, peer agreement, and next-action fields exist in shared status. | Phase 69, verified 2026-06-12. [VERIFIED: .planning/phases/69-tip-tracking-and-stay-current-operation/69-VERIFICATION.md] | Phase 70 should extend shared diagnosis rather than add renderer-specific wording. [VERIFIED: .planning/phases/70-reorg-peer-rotation-and-no-progress-recovery/70-CONTEXT.md] |
| Reconcile result reports only progressed/not-progressed. | Phase 70 should make reconcile outcomes typed and evidence-carrying. | Not yet implemented. [VERIFIED: packages/open-bitcoin-node/src/sync/block_reconcile.rs] | Planner should include a task to replace `bool` outcome with bounded evidence without changing the core reorg engine. [VERIFIED: packages/open-bitcoin-node/src/sync/block_reconcile.rs; packages/open-bitcoin-chainstate/src/engine.rs] |
| Peer failures could be interpreted from logs. | Existing runtime emits typed `PeerFailureReason`, `PeerSyncState`, recovery actions, and structured log records. | Phases 61, 66, 68, and 69. [VERIFIED: .planning/phases/61-resource-bounds-and-recovery-taxonomy/61-CONTEXT.md; .planning/phases/66-compatibility-harness-operator-wrapper/66-CONTEXT.md; packages/open-bitcoin-node/src/sync/types.rs] | REC-03 should preserve and expand typed variants rather than flatten failures. [VERIFIED: .planning/phases/70-reorg-peer-rotation-and-no-progress-recovery/70-CONTEXT.md] |

**Deprecated/outdated:**

- `SyncStopReason::NoProgress` alone is too coarse for REC-04 planning; keep it as a stop reason but add a more specific shared diagnosis. [VERIFIED: packages/open-bitcoin-node/src/sync/types.rs; .planning/phases/70-reorg-peer-rotation-and-no-progress-recovery/70-CONTEXT.md]
- Reorg evidence as "final height changed" is insufficient; Phase 70 requires common ancestor, disconnected count, connected count, final active tip, and persistence status. [VERIFIED: .planning/phases/70-reorg-peer-rotation-and-no-progress-recovery/70-CONTEXT.md]

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|

All claims in this research were verified or cited in this session; no `[ASSUMED]` claims are intentionally present. [VERIFIED: this research file source tags]

## Open Questions (RESOLVED)

1. **Exact public field names for Phase 70 status additions**
   - What we know: Shared status already exposes `stay_current`, `stay_current_next_action`, `progress_signal`, `recovery_category`, and `resource_pressure`. [VERIFIED: packages/open-bitcoin-node/src/status.rs]
   - What's unclear: The Phase 70 context specifies required meanings but does not lock field names such as `no_progress_diagnosis`, `no_progress_next_action`, or `latest_reorg`. [VERIFIED: .planning/phases/70-reorg-peer-rotation-and-no-progress-recovery/70-CONTEXT.md]
   - Recommendation: Use explicit shared status fields named `no_progress_diagnosis`, `no_progress_next_action`, and `latest_reorg` if the planner chooses external JSON visibility; otherwise keep internal pure helper types and project only existing compatible fields. [VERIFIED: packages/open-bitcoin-node/src/status.rs; .planning/phases/70-reorg-peer-rotation-and-no-progress-recovery/70-CONTEXT.md]
   - RESOLVED: Phase 70 will use additive public shared status fields named `latest_reorg`, `reconcile_progress`, `no_progress_diagnosis`, and `no_progress_next_action`. The first two fields expose bounded branch/reorg evidence and reconcile state; the latter two expose shared typed no-progress diagnosis and quiet operator guidance. [VERIFIED: .planning/phases/70-reorg-peer-rotation-and-no-progress-recovery/70-CONTEXT.md; packages/open-bitcoin-node/src/status.rs]

2. **Whether bounded reorg evidence must persist across restart**
   - What we know: Runtime metadata persists `DurableSyncState`, and Phase 70 requires final persisted status truth after reorg. [VERIFIED: packages/open-bitcoin-node/src/storage.rs; packages/open-bitcoin-node/src/sync/runtime_state.rs; .planning/phases/70-reorg-peer-rotation-and-no-progress-recovery/70-CONTEXT.md]
   - What's unclear: The context requires operator-facing bounded evidence but does not state retention duration. [VERIFIED: .planning/phases/70-reorg-peer-rotation-and-no-progress-recovery/70-CONTEXT.md]
   - Recommendation: Persist only the latest bounded reorg evidence in `RuntimeMetadata.maybe_sync_state` through `SyncStatus`; leave event-history retention to Phase 72 support evidence. [VERIFIED: .planning/ROADMAP.md; packages/open-bitcoin-node/src/storage.rs]
   - RESOLVED: Bounded reorg evidence persists only as the latest durable status evidence in `RuntimeMetadata.maybe_sync_state` via `SyncStatus.latest_reorg`. Phase 70 must not create an unbounded reorg history or event stream; broader support-evidence retention remains deferred to Phase 72. [VERIFIED: .planning/ROADMAP.md; packages/open-bitcoin-node/src/storage.rs; packages/open-bitcoin-node/src/sync/runtime_state.rs]

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| Rust `rustc` | Compile Phase 70 Rust code | yes | `rustc 1.94.1 (e408947bf 2026-03-25)` | None needed. [VERIFIED: `rustc --version`; rust-toolchain.toml] |
| Cargo | Cargo tests/builds | yes | `cargo 1.94.1 (29ea6fb6a 2026-03-24)` | None needed. [VERIFIED: `cargo --version`] |
| Bun | TypeScript checkers | yes | `1.3.9` | None needed. [VERIFIED: `bun --version`; .bun-version] |
| Bazel/Bazelisk command | Repo smoke build | yes | `bazel 8.6.0` | None needed. [VERIFIED: `bazel --version`; scripts/verify.sh] |
| `cargo-llvm-cov` | Repo-native pure-core coverage gate | yes | `cargo-llvm-cov 0.8.5` | None needed. [VERIFIED: `cargo llvm-cov --version`; scripts/verify.sh] |
| Git | Repo state and optional GSD commit | yes | `git version 2.53.0` | None needed. [VERIFIED: `git --version`] |

**Missing dependencies with no fallback:** None found for Phase 70 planning and deterministic local verification. [VERIFIED: environment probes above]

**Missing dependencies with fallback:** None found. [VERIFIED: environment probes above]

## Security Domain

The project config does not set `security_enforcement` to `false`, so this research includes a security domain. [VERIFIED: .planning/config.json]

The current official OWASP ASVS project page states that ASVS provides a basis for testing web application technical security controls and that the latest stable version is 5.0.0. [CITED: https://owasp.org/www-project-application-security-verification-standard/] The OWASP Cheat Sheet ASVS index is based on ASVS 5.0.x and lists ASVS sections such as V1 Encoding and Sanitization, V2 Authentication, V3 Session Management, V4 Access Control, and V13 API and Web Service. [CITED: https://cheatsheetseries.owasp.org/IndexASVS.html]

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|-----------------|
| V1 Encoding and Sanitization / input handling | yes | Parse peer wire data into first-party typed messages and map malformed/invalid data to typed outcomes; do not trust raw payloads. [VERIFIED: packages/open-bitcoin-node/src/sync/types.rs; packages/open-bitcoin-node/src/sync/block_response.rs; CITED: https://cheatsheetseries.owasp.org/IndexASVS.html] |
| V2 Authentication | no | Phase 70 does not add an authentication surface; RPC/CLI auth is outside this phase. [VERIFIED: .planning/phases/70-reorg-peer-rotation-and-no-progress-recovery/70-CONTEXT.md; CITED: https://cheatsheetseries.owasp.org/IndexASVS.html] |
| V3 Session Management | no | Phase 70 does not add browser or authenticated web sessions. [VERIFIED: .planning/phases/70-reorg-peer-rotation-and-no-progress-recovery/70-CONTEXT.md; CITED: https://cheatsheetseries.owasp.org/IndexASVS.html] |
| V4 Access Control | no | Phase 70 changes daemon sync internals and status projection, not operator authorization policy. [VERIFIED: .planning/phases/70-reorg-peer-rotation-and-no-progress-recovery/70-CONTEXT.md; CITED: https://cheatsheetseries.owasp.org/IndexASVS.html] |
| V7 Error Handling and Logging | yes | Preserve typed errors, bounded structured logs, storage-first recovery precedence, and no raw undo dumps in operator status. [VERIFIED: packages/open-bitcoin-node/src/sync/types.rs; packages/open-bitcoin-node/src/sync/types/summary.rs; .planning/phases/70-reorg-peer-rotation-and-no-progress-recovery/70-CONTEXT.md; CITED: https://owasp.org/www-project-application-security-verification-standard/] |
| V8 Data Protection | yes | Keep durable storage namespaces separate and avoid exposing raw undo data or wallet material in status/support surfaces. [VERIFIED: packages/open-bitcoin-node/src/storage.rs; packages/open-bitcoin-node/src/storage/fjall_store.rs; .planning/phases/70-reorg-peer-rotation-and-no-progress-recovery/70-CONTEXT.md] |
| V9 Communications Security | yes | Peer failures such as invalid magic, malformed data, disconnects, and network errors must remain typed; this phase does not add TLS or authenticated peer transport. [VERIFIED: packages/open-bitcoin-node/src/sync/types.rs; .planning/phases/70-reorg-peer-rotation-and-no-progress-recovery/70-CONTEXT.md; CITED: https://cheatsheetseries.owasp.org/IndexASVS.html] |
| V13 API and Web Service | limited | Shared status types may later flow through RPC, but Phase 72 owns full cross-surface observability alignment. [VERIFIED: .planning/ROADMAP.md; packages/open-bitcoin-node/src/status.rs; CITED: https://cheatsheetseries.owasp.org/IndexASVS.html] |
| V14 Configuration | yes | Respect configured peer, retry, resource, freshness, and persistence limits; do not add default public-network or service-manager checks. [VERIFIED: packages/open-bitcoin-node/src/sync/types.rs; scripts/verify.sh; .planning/REQUIREMENTS.md; CITED: https://cheatsheetseries.owasp.org/IndexASVS.html] |

### Known Threat Patterns for Rust P2P Sync Runtime

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Malformed or invalid block data from peers | Tampering | Reject through consensus/chainstate validation, do not persist as progress, attribute `MalformedBlock` or `InvalidBlock`, and retry eligible peers. [VERIFIED: packages/open-bitcoin-node/src/sync.rs; packages/open-bitcoin-node/src/sync/block_response.rs; packages/open-bitcoin-node/src/sync/tests.rs] |
| Branch competition causing unsafe active-chain disconnect | Tampering | Require replacement branch bodies before disconnect and use `Chainstate::reorg` with durable persistence. [VERIFIED: packages/open-bitcoin-node/src/sync/block_reconcile.rs; packages/open-bitcoin-chainstate/src/engine.rs] |
| Local storage corruption during reorg | Tampering / Denial of Service | Surface `StorageError::Corruption` and storage recovery action before peer retry advice. [VERIFIED: packages/open-bitcoin-node/src/storage.rs; packages/open-bitcoin-node/src/sync/block_reconcile.rs] |
| Peer stalls or retry backoff exhaustion | Denial of Service | Endpoint-keyed backoff, bounded attempt/round limits, typed waiting/stalled outcomes, and no-progress next actions. [VERIFIED: packages/open-bitcoin-node/src/sync.rs; packages/open-bitcoin-node/src/sync/types.rs; packages/open-bitcoin-node/src/sync/tests.rs] |
| Resource pressure from in-flight block requests | Denial of Service | Enforce `max_blocks_in_flight_per_peer` and `max_blocks_in_flight_total`; expose resource pressure in shared status. [VERIFIED: packages/open-bitcoin-node/src/sync/block_reconcile.rs; packages/open-bitcoin-node/src/sync/runtime_state.rs] |
| Diagnostic data leakage | Information Disclosure | Expose bounded reorg/status facts and avoid raw undo dumps, raw peer logs, credentials, wallet material, and unbounded report arrays. [VERIFIED: .planning/phases/70-reorg-peer-rotation-and-no-progress-recovery/70-CONTEXT.md; .planning/phases/61-resource-bounds-and-recovery-taxonomy/61-CONTEXT.md] |

## Sources

### Primary (HIGH confidence)

- `.planning/phases/70-reorg-peer-rotation-and-no-progress-recovery/70-CONTEXT.md` - locked Phase 70 decisions, boundaries, and verification posture.
- `.planning/REQUIREMENTS.md` - REC-01 through REC-04 and v1.6 out-of-scope boundaries.
- `.planning/ROADMAP.md` - Phase 70 placement and Phase 71 through Phase 74 boundaries.
- `.planning/PROJECT.md` - pinned Knots baseline, dependency policy, functional-core boundary, and v1.6 scope.
- `.planning/STATE.md` - current milestone state and Phase 70 as next work.
- `AGENTS.md`, `AGENTS.bright-builds.md`, `standards-overrides.md`, and `bright-builds-rules.audit.md` - repo-local and Bright Builds workflow constraints.
- Bright Builds pinned standards at commit `05f8d7a6c9c2e157ec4f922a05273e72dab97676`: `standards/index.md`, `standards/core/architecture.md`, `standards/core/code-shape.md`, `standards/core/verification.md`, `standards/core/testing.md`, and `standards/languages/rust.md`. [CITED: https://raw.githubusercontent.com/bright-builds-llc/bright-builds-rules/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/index.md]
- `.planning/phases/61-resource-bounds-and-recovery-taxonomy/61-CONTEXT.md` - recovery taxonomy and storage-first precedence.
- `.planning/phases/64-service-restart-and-same-datadir-resume-evidence/64-CONTEXT.md` - stale in-flight cleanup posture.
- `.planning/phases/66-compatibility-harness-operator-wrapper/66-CONTEXT.md` - typed compatibility/no-credit peer outcome alignment.
- `.planning/phases/67-release-boundaries-and-deterministic-verification/67-CONTEXT.md` - deterministic verification and release-claim boundaries.
- `.planning/phases/68-full-active-chain-validation-and-durable-persistence/68-CONTEXT.md` and `68-VERIFICATION.md` - validated active-chain progress and durable persistence.
- `.planning/phases/69-tip-tracking-and-stay-current-operation/69-CONTEXT.md` and `69-VERIFICATION.md` - best-known tip, stay-current, peer agreement, and residual Phase 70 risks.
- `packages/open-bitcoin-network/src/header_store.rs` - header best-tip policy and best-chain entries.
- `packages/open-bitcoin-chainstate/src/engine.rs`, `types.rs`, and `error.rs` - connect/disconnect/reorg, undo payloads, transitions, and errors.
- `packages/open-bitcoin-node/src/chainstate.rs`, `network.rs`, `storage.rs`, `storage/fjall_store.rs`, `storage/snapshot_codec.rs`, `sync.rs`, `sync/block_reconcile.rs`, `sync/block_response.rs`, `sync/progress.rs`, `sync/runtime_state.rs`, `sync/tip.rs`, `sync/types.rs`, `sync/types/projection.rs`, `sync/types/recovery.rs`, `sync/types/summary.rs`, and `sync/tests.rs` - Phase 70 implementation surfaces.
- `scripts/verify.sh`, `scripts/check-phase69-tip-stay-current.ts`, `docs/operator/runtime-guide.md`, `docs/architecture/status-snapshot.md`, and `docs/parity/source-breadcrumbs.json` - verification, docs, and parity patterns.

### Secondary (MEDIUM confidence)

- OWASP ASVS project page - ASVS purpose and latest stable version. [CITED: https://owasp.org/www-project-application-security-verification-standard/]
- OWASP Cheat Sheet ASVS index - ASVS 5.0.x category references used for security-domain mapping. [CITED: https://cheatsheetseries.owasp.org/IndexASVS.html]

### Tertiary (LOW confidence)

- None.

## Metadata

**Confidence breakdown:**

- Standard stack: HIGH - existing repo stack and versions were verified from local manifests and CLI probes; no new dependency is recommended. [VERIFIED: packages/Cargo.toml; packages/open-bitcoin-node/Cargo.toml; rust-toolchain.toml; .bun-version; environment probes]
- Architecture: HIGH - module boundaries and reusable primitives were verified directly in source and prior phase verification artifacts. [VERIFIED: packages/open-bitcoin-node/src/sync/block_reconcile.rs; packages/open-bitcoin-chainstate/src/engine.rs; .planning/phases/68-full-active-chain-validation-and-durable-persistence/68-VERIFICATION.md; .planning/phases/69-tip-tracking-and-stay-current-operation/69-VERIFICATION.md]
- Pitfalls: HIGH - each listed pitfall maps to locked Phase 70 decisions or observed current code gaps such as `reconcile_best_chain` returning `bool`. [VERIFIED: .planning/phases/70-reorg-peer-rotation-and-no-progress-recovery/70-CONTEXT.md; packages/open-bitcoin-node/src/sync/block_reconcile.rs]
- Security: MEDIUM - ASVS category mapping is current from official OWASP pages, but the exact control-level mapping should be revisited if Phase 70 expands RPC/API behavior. [CITED: https://owasp.org/www-project-application-security-verification-standard/; CITED: https://cheatsheetseries.owasp.org/IndexASVS.html; VERIFIED: .planning/ROADMAP.md]

**Research date:** 2026-06-12  
**Valid until:** 2026-07-12 for repo-local architecture; 2026-06-19 for ASVS/current external security-standard references.
