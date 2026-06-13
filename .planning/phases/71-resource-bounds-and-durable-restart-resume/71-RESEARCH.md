# Phase 71: Resource Bounds and Durable Restart/Resume - Research

**Researched:** 2026-06-13
**Domain:** Rust sync runtime resource bounds, durable storage recovery, same-datadir restart/resume, deterministic long-chain fixtures
**Confidence:** HIGH

<user_constraints>
## User Constraints (from CONTEXT.md)

> Source for this entire section: [VERIFIED: .planning/phases/71-resource-bounds-and-durable-restart-resume/71-CONTEXT.md]

### Locked Decisions

### Resource Bound Contract

- **D-01:** Extend the existing bounded sync contract instead of inventing a
  parallel resource model. `SyncResourcePressure` remains the operator-facing
  resource envelope and should cover peers, in-flight blocks, header request
  limits, message and round caps, and configured outbound targets. If Phase 71
  needs additional proof fields for queues, caches, storage writes, logs,
  metrics, or support evidence, add them as typed bounded facts or compact
  summaries, not renderer-local strings.
- **D-02:** Keep queue and retention behavior explicitly bounded. The runtime
  should avoid unbounded in-memory queues, retained peer outcome arrays, log
  samples, metrics samples, support report material, or durable write backlogs.
  Where a bound is enforced by an existing retention policy or synchronous
  adapter call, document and test that fact.
- **D-03:** Resource-limit blockers should remain typed runtime outcomes.
  Zero or exhausted block budgets, storage pressure, and low disk conditions
  should surface through `SyncRecoveryCategory`, `SyncRuntimeError`, shared
  status, and next-action guidance rather than vague "sync failed" messages.

### Restart And Interruption Matrix

- **D-04:** Same-datadir resume safety must be proven deterministically for
  clean shutdown, unclean shutdown, mid-download interruption, mid-connect
  interruption, and stale in-flight work. Reuse `DurableSyncRuntime`, Fjall
  reopen, `ScriptedTransport`, and existing block reconcile fixtures where they
  already prove behavior.
- **D-05:** Resume evidence should preserve the Phase 58, Phase 64, Phase 68,
  and Phase 70 truth contract: durable headers, downloaded bodies, connected
  active-chain state, UTXO/undo snapshot, runtime metadata, best-known tip
  evidence, stale in-flight cleanup, and typed recovery category. Already
  connected blocks must not be requested or connected again after reopen.
- **D-06:** Stale in-flight work after restart must be cleared, reassigned, or
  diagnosed explicitly. It must not make the daemon look busy when no peer can
  satisfy the work, and it must not hide durable progress that is safe to resume.

### Storage Pressure And Recovery Guidance

- **D-07:** Storage recovery guidance keeps storage-first precedence. Schema
  mismatch, corruption markers, lock contention, low disk, and storage pressure
  outrank peer retry advice and must not trigger hidden data mutation.
- **D-08:** Add or refine typed recovery categories only where the existing
  taxonomy cannot express Phase 71 requirements. `incompatible_schema`,
  `store_corruption`, `storage_lock_contention`, `storage_backend_failure`, and
  `resource_exhaustion` already exist; low-disk and storage-pressure evidence
  may map to `resource_exhaustion` only if operator guidance remains precise.
- **D-09:** Recovery guidance should be actionable and quiet: inspect storage
  health, free disk, close the competing process holding the lock, run the
  explicit repair/reindex path where available, increase a configured bound, or
  retry after peer backoff. Do not imply automatic repair or mutation.

### Deterministic Long-Chain Verification

- **D-10:** RES-04 should be proven through deterministic synthetic long-chain
  tests, not public-mainnet timing. Tests should exercise bounded peer fanout,
  in-flight block caps, request queues, block reconciliation, durable reconnect,
  restart/resume, stale in-flight cleanup, metrics/log retention, and support
  evidence compactness.
- **D-11:** The synthetic long-chain path should use first-party fixtures and
  scripted transport rather than new production dependencies. Prefer pure
  helper functions and typed fixtures where possible so the test isolates the
  bound being proven.
- **D-12:** `bash scripts/verify.sh` remains the final deterministic verification
  contract. Public-network full-sync, long-run, service-manager, or
  `--restart-after-progress` commands may be documented as opt-in UAT only.

### Operator Evidence And Documentation

- **D-13:** Operator surfaces should describe what the evidence proves:
  bounded long sync, safe same-datadir resume, diagnosed storage or resource
  blocker, or deferred production-node scope. Keep status, docs, support
  evidence, metrics, logs, and live-smoke reports aligned on field names.
- **D-14:** Update operator docs, architecture docs, parity notes, and focused
  deterministic checkers only where Phase 71 changes the truth contract or
  resource/recovery guidance. Preserve copy-pasteable repo-local Cargo and Bazel
  command forms for opt-in operator workflows.
- **D-15:** If new first-party Rust source or test files are added under
  `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests`, update
  parity breadcrumbs through `docs/parity/source-breadcrumbs.json` and
  `scripts/check-parity-breadcrumbs.ts`.

### the agent's Discretion

- The planner may split work across resource contract/status fields,
  restart/interruption fixtures, storage-pressure recovery guidance,
  synthetic long-chain tests, and docs/checker closeout.
- The executor may add small pure helper types for storage pressure or
  restart/resume classification when they keep illegal states unrepresentable
  and avoid renderer duplication.
- The executor may keep new tests inside existing sync/status/storage test files
  when that is the smallest robust path. If new files are cleaner, parity
  breadcrumbs are mandatory.
- The executor may preserve existing recovery labels and add more precise
  next-action text instead of adding enum variants, provided RES-03 remains
  auditable and typed.

### Deferred Ideas (OUT OF SCOPE)

None - discussion stayed within Phase 71 scope.
</user_constraints>

## Summary

Phase 71 should be planned as a proof-hardening phase over existing typed contracts, not as a new sync architecture. `SyncResourcePressure`, `DurableSyncRuntime`, Fjall reopen, storage-first recovery mapping, `ScriptedTransport`, existing block reconcile fixtures, metrics retention, log retention, and Phase 70 no-progress diagnosis are already present and verified in code. [VERIFIED: packages/open-bitcoin-node/src/status.rs] [VERIFIED: packages/open-bitcoin-node/src/sync.rs] [VERIFIED: packages/open-bitcoin-node/src/storage.rs] [VERIFIED: packages/open-bitcoin-node/src/sync/tests.rs]

The largest implementation gap is not the existence of bounds, but making them auditable for RES-01 through RES-04: exact peer/in-flight/message/round/retention/write/support bounds, a deterministic restart/interruption matrix, precise low-disk/storage-pressure guidance, and synthetic long-chain tests that do not depend on public-network timing. [VERIFIED: .planning/REQUIREMENTS.md] [VERIFIED: .planning/ROADMAP.md] [VERIFIED: .planning/phases/70-reorg-peer-rotation-and-no-progress-recovery/70-VERIFICATION.md]

**Primary recommendation:** Extend the existing sync/status/storage surfaces with any missing typed bounded facts, add deterministic `sync::tests` long-chain and interruption fixtures, add a focused Phase 71 Bun checker only for docs/source contract drift, and keep public-network or service-manager long-run checks as opt-in UAT. [VERIFIED: .planning/phases/71-resource-bounds-and-durable-restart-resume/71-CONTEXT.md] [VERIFIED: scripts/verify.sh]

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| RES-01 | Operator can run long mainnet sync attempts with documented and tested bounds for peers, in-flight blocks, queues, caches, storage writes, logs, metrics, and support evidence. | Use `SyncResourcePressure`, `SyncRuntimeConfig`, metrics/log retention policies, synchronous storage adapter writes, and compact support evidence; add missing typed facts only where existing fields cannot prove the bound. [VERIFIED: .planning/REQUIREMENTS.md] [VERIFIED: packages/open-bitcoin-node/src/status.rs] [VERIFIED: packages/open-bitcoin-node/src/metrics.rs] [VERIFIED: packages/open-bitcoin-node/src/logging.rs] |
| RES-02 | Operator can resume safely after clean shutdown, unclean shutdown, mid-download interruption, mid-connect interruption, and stale in-flight work. | Reuse `DurableSyncRuntime::open`, Fjall reopen, `RuntimeMetadata.last_clean_shutdown`, existing same-datadir and no-duplicate tests, and add missing interruption matrix cases in `sync/tests.rs`. [VERIFIED: .planning/REQUIREMENTS.md] [VERIFIED: packages/open-bitcoin-node/src/sync.rs] [VERIFIED: packages/open-bitcoin-node/src/storage.rs] [VERIFIED: packages/open-bitcoin-node/src/sync/tests.rs] |
| RES-03 | Operator can receive typed recovery guidance for schema mismatch, corruption markers, lock contention, low disk, and storage pressure without hidden data mutation. | Keep schema/corruption/lock on existing storage categories, map low disk/storage pressure through typed runtime outcomes with precise guidance, and preserve storage-first precedence over peer guidance. [VERIFIED: .planning/REQUIREMENTS.md] [VERIFIED: packages/open-bitcoin-node/src/storage.rs] [VERIFIED: packages/open-bitcoin-node/src/sync/types/recovery.rs] [VERIFIED: packages/open-bitcoin-node/src/sync/runtime_state.rs] |
| RES-04 | Operator can run deterministic synthetic long-chain tests that exercise resource bounds without requiring public-network access. | Use first-party block builders, real Fjall stores, `ScriptedTransport`, `ScriptedResolver`, bounded config knobs, and `bash scripts/verify.sh`; do not introduce public-mainnet timing into default verification. [VERIFIED: .planning/REQUIREMENTS.md] [VERIFIED: packages/open-bitcoin-node/src/sync/tests.rs] [VERIFIED: scripts/verify.sh] |
</phase_requirements>

## Project Constraints (from AGENTS.md)

- Use `AGENTS.md` as the repo-local instruction entrypoint; this repo has `AGENTS.md`, and no `CLAUDE.md` fallback was needed. [VERIFIED: ./AGENTS.md]
- Read `AGENTS.bright-builds.md`, `standards-overrides.md`, and relevant pinned Bright Builds standards before planning or implementation. [VERIFIED: ./AGENTS.md] [VERIFIED: AGENTS.bright-builds.md] [VERIFIED: standards-overrides.md] [CITED: https://raw.githubusercontent.com/bright-builds-llc/bright-builds-rules/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/index.md]
- Local `standards/index.md` was not present, so the pinned canonical Bright Builds pages were read from the exact GitHub commit named by `AGENTS.bright-builds.md`. [VERIFIED: find standards] [CITED: https://raw.githubusercontent.com/bright-builds-llc/bright-builds-rules/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/core/architecture.md]
- Keep functional core / imperative shell boundaries; business decisions should be pure where practical, and I/O, storage, network, clocks, randomness, and framework calls should stay in adapters. [VERIFIED: ./AGENTS.md] [CITED: https://raw.githubusercontent.com/bright-builds-llc/bright-builds-rules/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/core/architecture.md]
- Make illegal states unrepresentable with Rust types, constructors, enums, and typed state machines where practical. [VERIFIED: AGENTS.bright-builds.md] [CITED: https://raw.githubusercontent.com/bright-builds-llc/bright-builds-rules/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/languages/rust.md]
- Use `maybe_` naming for internal `Option`/nullable values and prefer `let...else` for guard-style extraction when it improves clarity. [VERIFIED: AGENTS.bright-builds.md] [CITED: https://raw.githubusercontent.com/bright-builds-llc/bright-builds-rules/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/languages/rust.md]
- New or touched multi-file Rust modules should prefer `foo.rs` plus `foo/` over `foo/mod.rs`; do not require broad rename-only cleanup of stable existing trees. [CITED: https://raw.githubusercontent.com/bright-builds-llc/bright-builds-rules/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/languages/rust.md]
- Unit tests for pure/business logic must be focused on one concern and clearly delineate Arrange, Act, and Assert unless the test is trivially obvious. [VERIFIED: ./AGENTS.md] [CITED: https://raw.githubusercontent.com/bright-builds-llc/bright-builds-rules/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/core/testing.md]
- Use `rust-toolchain.toml` as the Rust source of truth; the pinned local Rust toolchain is 1.94.1. [VERIFIED: ./AGENTS.md] [VERIFIED: rust-toolchain.toml] [VERIFIED: rustc --version]
- Use `bash scripts/verify.sh` as the repo-native final verification contract for first-party code, including Bazel smoke build. [VERIFIED: ./AGENTS.md] [VERIFIED: scripts/verify.sh]
- Run repo-local Cargo and Bazel command forms in operator docs/UAT, not only the installed `open-bitcoin` alias. [VERIFIED: ./AGENTS.md]
- Use Bun for repo-owned higher-level automation scripts and TypeScript for substantial script logic; this repo has `.bun-version` and no `package.json`. [VERIFIED: ./AGENTS.md] [VERIFIED: .planning/STACK.md] [VERIFIED: .bun-version]
- Do not add existing Rust Bitcoin libraries to the production path; Open Bitcoin owns its domain model and implementation surface. [VERIFIED: ./AGENTS.md] [VERIFIED: .planning/PROJECT.md]
- If new first-party Rust source or test files are added under `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests`, update parity breadcrumbs through `docs/parity/source-breadcrumbs.json` and `scripts/check-parity-breadcrumbs.ts`. [VERIFIED: ./AGENTS.md] [VERIFIED: docs/parity/source-breadcrumbs.json]
- Treat `docs/metrics/lines-of-code.md` as a tracked generated artifact that may need freshness updates after verification/checker changes. [VERIFIED: ./AGENTS.md] [VERIFIED: scripts/verify.sh]
- No project skills were found under `.claude/skills/` or `.agents/skills/`. [VERIFIED: find .claude/skills .agents/skills]

## Standard Stack

### Core

| Library / Tool | Version | Purpose | Why Standard |
|---|---:|---|---|
| Rust toolchain | 1.94.1 | First-party runtime, sync, storage, status, and tests | Pinned by repo toolchain and Bazel toolchain, so Phase 71 must not plan around another Rust version. [VERIFIED: rust-toolchain.toml] [VERIFIED: MODULE.bazel] |
| Rust edition | 2024 | First-party workspace edition | Workspace package config and Bazel rules both declare edition 2024. [VERIFIED: packages/Cargo.toml] [VERIFIED: MODULE.bazel] |
| Cargo workspace | 0.1.0 packages | Crate-level build, lint, and test execution | `packages/Cargo.toml` owns workspace members and package version. [VERIFIED: packages/Cargo.toml] |
| Bazel / rules_rust | Bazel 8.6.0 / rules_rust 0.69.0 | Top-level smoke build and Bzlmod integration | `scripts/verify.sh` runs Bazel smoke builds, and `MODULE.bazel` pins `rules_rust`. [VERIFIED: bazelisk --version] [VERIFIED: scripts/verify.sh] [VERIFIED: MODULE.bazel] |
| Fjall | 3.1.4 | Durable store adapter for headers, block index, block bodies, chainstate, metrics, runtime metadata, schema, and recovery markers | Existing `FjallNodeStore` is the storage shell Phase 71 must exercise via reopen and pressure/error fixtures. [VERIFIED: cargo tree --locked -p open-bitcoin-node] [VERIFIED: packages/open-bitcoin-node/src/storage/fjall_store.rs] |
| open-bitcoin first-party crates | 0.1.0 | Domain, network, chainstate, node, RPC, CLI, harness, and benchmark packages | Repo policy forbids production-path Rust Bitcoin libraries and keeps parity in first-party modules. [VERIFIED: packages/Cargo.toml] [VERIFIED: ./AGENTS.md] |
| serde / serde_json | 1.0.228 / 1.0.149 | Stable status, storage snapshot, RPC, support, and checker data shapes | Existing status, metrics, logs, storage DTOs, and support surfaces are serialized typed contracts. [VERIFIED: cargo tree --locked -p open-bitcoin-node] [VERIFIED: packages/open-bitcoin-node/src/status.rs] |
| Bun | 1.3.9 | Repo-owned TypeScript automation and phase checkers | `scripts/verify.sh` invokes Bun checkers, and repo guidance says Bun is canonical for substantial automation scripts. [VERIFIED: bun --version] [VERIFIED: scripts/verify.sh] [VERIFIED: ./AGENTS.md] |

### Supporting

| Library / Tool | Version | Purpose | When to Use |
|---|---:|---|---|
| clap | 4.6.1 | Operator CLI argument surface | Use only if Phase 71 changes operator commands or documented flags. [VERIFIED: cargo tree --locked -p open-bitcoin-cli] |
| axum / tokio | 0.8.9 / 1.52.1 | Local RPC server runtime | Use only if Phase 71 changes daemon/RPC status projection; the default plan likely stays in node/storage/tests/docs. [VERIFIED: cargo tree --locked -p open-bitcoin-rpc] |
| ratatui / crossterm | 0.30.0 / 0.29.0 | Terminal dashboard/status surfaces | Use only if resource/recovery contract changes require dashboard rendering updates. [VERIFIED: cargo tree --locked -p open-bitcoin-cli] |
| jsonc-parser | 0.32.3 | Open Bitcoin JSONC config parsing | Use only if Phase 71 adds or documents config knobs for resource pressure. [VERIFIED: cargo tree --locked -p open-bitcoin-rpc] |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|---|---|---|
| Existing `SyncResourcePressure` | A new Phase 71 resource model | Do not use: locked decision D-01 requires extending the existing operator-facing envelope. [VERIFIED: .planning/phases/71-resource-bounds-and-durable-restart-resume/71-CONTEXT.md] |
| Existing `FjallNodeStore` reopen tests | A separate storage harness or database | Do not use: durable restart/resume proof needs the current adapter and typed storage contract. [VERIFIED: packages/open-bitcoin-node/src/storage/fjall_store.rs] [VERIFIED: .planning/phases/71-resource-bounds-and-durable-restart-resume/71-CONTEXT.md] |
| `ScriptedTransport` / `ScriptedResolver` | Public-mainnet long-run tests | Do not use for default verification: RES-04 requires deterministic synthetic long-chain tests without public-network access. [VERIFIED: .planning/REQUIREMENTS.md] [VERIFIED: packages/open-bitcoin-node/src/sync/tests.rs] |
| Existing recovery categories | Renderer-local strings | Do not use: recovery blockers must flow through `SyncRecoveryCategory`, `SyncRuntimeError`, shared status, and next-action guidance. [VERIFIED: .planning/phases/71-resource-bounds-and-durable-restart-resume/71-CONTEXT.md] [VERIFIED: packages/open-bitcoin-node/src/status/recovery.rs] |

**Installation:**

```bash
git submodule update --init --recursive
bash scripts/verify.sh
```

No new dependency installation is recommended for Phase 71; dependency versions above were verified from the pinned toolchain, `MODULE.bazel`, and `cargo tree --locked`. [VERIFIED: ./AGENTS.md] [VERIFIED: cargo tree --locked -p open-bitcoin-node] [VERIFIED: cargo tree --locked -p open-bitcoin-cli] [VERIFIED: cargo tree --locked -p open-bitcoin-rpc]

## Architecture Patterns

### Recommended Project Structure

Keep implementation inside existing modules unless a helper becomes clearly reusable. [VERIFIED: .planning/phases/71-resource-bounds-and-durable-restart-resume/71-CONTEXT.md] [VERIFIED: docs/parity/source-breadcrumbs.json]

```text
packages/open-bitcoin-node/src/
├── status.rs                  # Extend SyncResourcePressure or shared status only if a typed bound is missing.
├── status/recovery.rs          # Add recovery labels only if existing taxonomy cannot express RES-03.
├── storage.rs                  # Add typed low-disk/storage-pressure evidence or actions here, not in renderers.
├── storage/fjall_store.rs      # Exercise real reopen, schema, markers, and backend failures.
├── sync.rs                     # DurableSyncRuntime shell and bounded cycle orchestration.
├── sync/runtime_state.rs       # Project resource pressure and recovery precedence into durable status.
├── sync/block_reconcile.rs     # In-flight caps, missing bodies, stale release, and reconnect behavior.
├── sync/progress.rs            # Pure no-progress and next-action helpers.
└── sync/tests.rs               # Scripted deterministic long-chain, interruption, and resource-bound tests.

scripts/
├── check-phase71-resource-restart.ts  # Add only if docs/source contract drift needs a focused deterministic guard.
└── verify.sh                          # Wire the checker here after Phase 70 if added.
```

### Pattern 1: Extend Typed Status, Then Render

**What:** Add missing evidence to `SyncResourcePressure`, `SyncRecoveryCategory`, or typed status DTOs first; CLI, RPC, dashboard, support, logs, and docs should consume typed facts instead of reconstructing strings. [VERIFIED: packages/open-bitcoin-node/src/status.rs] [VERIFIED: packages/open-bitcoin-node/src/sync/runtime_state.rs] [VERIFIED: .planning/phases/62-long-run-sync-truth-surfaces/62-CONTEXT.md]

**When to use:** Use this when RES-01 needs a new inspectable bound for queues, caches, storage writes, logs, metrics, or support evidence that cannot already be proven from config/retention/docs. [VERIFIED: .planning/phases/71-resource-bounds-and-durable-restart-resume/71-CONTEXT.md]

**Example:**

```rust
// Source: packages/open-bitcoin-node/src/sync/runtime_state.rs
sync.resource_pressure = FieldAvailability::available(SyncResourcePressure {
    blocks_in_flight: self.inflight_blocks.len() as u64,
    max_header_requests_in_flight_per_peer: MAX_HEADER_REQUESTS_IN_FLIGHT_PER_PEER,
    max_headers_per_message: MAX_HEADERS_RESULTS as u64,
    max_blocks_in_flight_per_peer: self.config.max_blocks_in_flight_per_peer as u64,
    max_blocks_in_flight_total: self.config.max_blocks_in_flight_total as u64,
    max_messages_per_peer: self.config.max_messages_per_peer as u64,
    max_sync_rounds: self.config.max_rounds as u64,
    outbound_peers: summary.connected_peers as u32,
    target_outbound_peers: self.config.target_outbound_peers as u32,
});
```

### Pattern 2: Use DurableSyncRuntime as the Effectful Shell

**What:** Keep peer transport, Fjall storage, runtime metadata persistence, metrics, logs, and reconnect/reopen behavior in `DurableSyncRuntime`; keep pure decision helpers small and testable. [VERIFIED: packages/open-bitcoin-node/src/sync.rs] [CITED: https://raw.githubusercontent.com/bright-builds-llc/bright-builds-rules/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/core/architecture.md]

**When to use:** Use this for restart/resume, interruption, stale in-flight, and synthetic long-chain tests. [VERIFIED: packages/open-bitcoin-node/src/sync/tests.rs]

**Example:**

```rust
// Source: packages/open-bitcoin-node/src/sync.rs
pub fn open(
    store: FjallNodeStore,
    config: SyncRuntimeConfig,
) -> Result<Self, SyncRuntimeError> {
    let mut memory_store = MemoryChainstateStore::default();
    if let Some(snapshot) = store.load_chainstate_snapshot()? {
        memory_store.save_snapshot(snapshot);
    }
    // ...
    if let Some(header_store) = store.load_header_store()? {
        network.seed_header_store(header_store);
    }
    // ...
}
```

### Pattern 3: Storage-First Recovery Precedence

**What:** Storage metadata and storage-derived runtime errors must outrank peer/network guidance in durable status. [VERIFIED: packages/open-bitcoin-node/src/sync/runtime_state.rs] [VERIFIED: packages/open-bitcoin-node/src/storage.rs]

**When to use:** Use this for schema mismatch, corruption markers, lock contention, low disk, storage pressure, and backend failure guidance. [VERIFIED: .planning/phases/71-resource-bounds-and-durable-restart-resume/71-CONTEXT.md]

**Example:**

```rust
// Source: packages/open-bitcoin-node/src/sync/runtime_state.rs
if let Some(category) = metadata
    .maybe_last_recovery_action
    .map(|action| action.recovery_category())
{
    return Some(category);
}

if let Some(category) = maybe_last_error.and_then(recovery_category_from_error_detail) {
    return Some(category);
}
```

### Pattern 4: Deterministic Synthetic Long-Chain Fixtures

**What:** Build a local synthetic chain with first-party block/header helpers, drive it through `ScriptedTransport`, force small config bounds, and assert status/resource/retention invariants after each cycle and reopen. [VERIFIED: packages/open-bitcoin-node/src/sync/tests.rs] [VERIFIED: .planning/phases/71-resource-bounds-and-durable-restart-resume/71-CONTEXT.md]

**When to use:** Use this for RES-04 and for RES-01/RES-02 interactions that would otherwise require public-mainnet timing. [VERIFIED: .planning/REQUIREMENTS.md]

**Example:**

```rust
// Source: packages/open-bitcoin-node/src/sync/tests.rs
let mut transport = ScriptedTransport::with_connect_results(vec![
    Ok(Vec::new()),
    Ok(invalid_headers_script(100)),
    Ok(version_verack_script(0)),
]);
let mut resolver = ScriptedResolver::new(Vec::new());
let summary = runtime
    .sync_once_with_resolver(&mut transport, &mut resolver, 1_777_225_200)
    .expect("first sync");
```

### Anti-Patterns to Avoid

- **Parallel resource model:** It would split operator truth from `SyncResourcePressure` and violate D-01. [VERIFIED: .planning/phases/71-resource-bounds-and-durable-restart-resume/71-CONTEXT.md]
- **Renderer-local recovery strings:** They would bypass `SyncRecoveryCategory`, `SyncRuntimeError`, and shared status. [VERIFIED: packages/open-bitcoin-node/src/status/recovery.rs] [VERIFIED: packages/open-bitcoin-node/src/sync/types/recovery.rs]
- **Public-network default proof:** It would violate RES-04 and the repo verification boundary. [VERIFIED: .planning/REQUIREMENTS.md] [VERIFIED: scripts/verify.sh]
- **Raw support/log/report arrays:** They would violate support redaction and bounded evidence contracts. [VERIFIED: packages/open-bitcoin-cli/src/operator/support.rs] [VERIFIED: docs/operator/runtime-guide.md]
- **New first-party Rust files without breadcrumbs:** It would fail repo parity policy and `scripts/check-parity-breadcrumbs.ts`. [VERIFIED: ./AGENTS.md] [VERIFIED: docs/parity/source-breadcrumbs.json] [VERIFIED: scripts/verify.sh]

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---|---|---|---|
| Resource bound model | A new Phase 71 resource DTO | `SyncResourcePressure` plus compact typed additions | Existing status and docs already depend on this field set. [VERIFIED: packages/open-bitcoin-node/src/status.rs] |
| Durable restart shell | A new restart harness or alternate store abstraction | `DurableSyncRuntime::open` with `FjallNodeStore::open` | Existing reopen path reloads chainstate and headers, which RES-02 needs. [VERIFIED: packages/open-bitcoin-node/src/sync.rs] |
| Storage recovery taxonomy | String matching in CLI/docs | `StorageError`, `StorageRecoveryAction`, `SyncRuntimeError`, `SyncRecoveryCategory` | Storage precedence and stable labels already exist. [VERIFIED: packages/open-bitcoin-node/src/storage.rs] [VERIFIED: packages/open-bitcoin-node/src/status/recovery.rs] |
| Peer/block in-flight tracking | Ad hoc request queues | Existing `inflight_blocks`, network peer request tracking, and block reconcile release helpers | Existing code enforces per-peer/total caps and releases on message/disconnect. [VERIFIED: packages/open-bitcoin-node/src/sync/block_reconcile.rs] [VERIFIED: packages/open-bitcoin-node/src/sync.rs] |
| Metrics/log retention | Custom sampling or raw log aggregation | `MetricRetentionPolicy`, `LogRetentionPolicy`, and existing append/prune helpers | Defaults are tested and surfaced in status/docs. [VERIFIED: packages/open-bitcoin-node/src/metrics.rs] [VERIFIED: packages/open-bitcoin-node/src/logging.rs] |
| Long-chain verification | Mainnet timing or service-manager loops | `ScriptedTransport`, synthetic block helpers, real Fjall reopen, and bounded configs | Default verification must be deterministic and public-network-free. [VERIFIED: .planning/REQUIREMENTS.md] [VERIFIED: scripts/verify.sh] |
| Support evidence compactness | Raw daemon tails, raw peer tables, raw reports | Existing support allowlist/redaction projections | Support docs and code exclude unbounded/raw evidence. [VERIFIED: packages/open-bitcoin-cli/src/operator/support.rs] [VERIFIED: docs/operator/runtime-guide.md] |

**Key insight:** Phase 71 is about proving and surfacing existing bounds under harder deterministic scenarios; custom substitutes would make operator evidence less auditable and create cross-surface drift. [VERIFIED: .planning/phases/71-resource-bounds-and-durable-restart-resume/71-CONTEXT.md]

## Common Pitfalls

### Pitfall 1: Treating Existing Bound Fields as Complete Proof

**What goes wrong:** `SyncResourcePressure` already reports key sync caps, but RES-01 also asks for queues, caches, storage writes, logs, metrics, and support evidence. [VERIFIED: packages/open-bitcoin-node/src/status.rs] [VERIFIED: .planning/REQUIREMENTS.md]

**Why it happens:** Earlier phases documented several non-status bounds, including synchronous writes and retention windows, but not every bound is necessarily a typed field. [VERIFIED: docs/operator/runtime-guide.md] [VERIFIED: docs/architecture/operator-observability.md]

**How to avoid:** Plan an explicit bound inventory table and add typed compact fields only for facts operators/tests cannot otherwise audit. [VERIFIED: .planning/phases/71-resource-bounds-and-durable-restart-resume/71-CONTEXT.md]

**Warning signs:** Tests assert only `blocks_in_flight` and omit metrics/log/support/write evidence. [VERIFIED: packages/open-bitcoin-node/src/sync/tests.rs]

### Pitfall 2: Low Disk Becomes a Generic Backend Failure

**What goes wrong:** Current storage mapping covers schema mismatch, corruption, lock signals, backend failure, interrupted writes, and resource-limit details, but no dedicated `LowDisk` or `StoragePressure` type was found. [VERIFIED: rg "low|disk|pressure" packages/open-bitcoin-node/src/storage.rs packages/open-bitcoin-node/src/sync]

**Why it happens:** Existing `StorageError::BackendFailure` carries a message and action, and `recovery_category_from_error_detail` maps generic resource words to `ResourceExhaustion`. [VERIFIED: packages/open-bitcoin-node/src/storage.rs] [VERIFIED: packages/open-bitcoin-node/src/sync/types/recovery.rs]

**How to avoid:** Add or refine typed storage-pressure evidence at the storage/runtime layer and keep operator guidance precise: free disk, reduce pressure, inspect store health, or increase configured bound. [VERIFIED: .planning/phases/71-resource-bounds-and-durable-restart-resume/71-CONTEXT.md]

**Warning signs:** Human text says "storage backend failure" when the next action should say "free disk" or "storage pressure". [VERIFIED: docs/operator/runtime-guide.md]

### Pitfall 3: Restart Tests Miss Mid-Operation Interruption

**What goes wrong:** Existing tests cover header seeding, partial download reopen, connected block no-duplicate request, branch reconnect, clean/unclean shutdown categories, and stale in-flight diagnosis, but RES-02 specifically asks for mid-download and mid-connect interruption. [VERIFIED: packages/open-bitcoin-node/src/sync/tests.rs] [VERIFIED: .planning/REQUIREMENTS.md]

**Why it happens:** Prior phases proved durable resume after progress, not every interruption point in the Phase 71 matrix. [VERIFIED: .planning/phases/58-same-datadir-restart-and-resume-evidence/58-CONTEXT.md] [VERIFIED: .planning/phases/68-full-active-chain-validation-and-durable-persistence/68-CONTEXT.md]

**How to avoid:** Plan a matrix with one deterministic test per interruption class: clean shutdown, unclean shutdown, mid-download, mid-connect, and stale in-flight. [VERIFIED: .planning/phases/71-resource-bounds-and-durable-restart-resume/71-CONTEXT.md]

**Warning signs:** A test reopens only after a complete sync cycle and does not simulate a failed block body or connect persistence boundary. [VERIFIED: packages/open-bitcoin-node/src/sync/tests.rs]

### Pitfall 4: Preserving Stale In-Flight State Across Reopen

**What goes wrong:** `DurableSyncRuntime::open` initializes `inflight_blocks` empty, so tests must prove stale work is cleared, reassigned, or diagnosed without hiding durable progress. [VERIFIED: packages/open-bitcoin-node/src/sync.rs] [VERIFIED: .planning/phases/71-resource-bounds-and-durable-restart-resume/71-CONTEXT.md]

**Why it happens:** In-flight work is runtime state, while headers, block bodies, chainstate snapshots, metrics, and runtime metadata are durable state. [VERIFIED: packages/open-bitcoin-node/src/sync.rs] [VERIFIED: packages/open-bitcoin-node/src/storage/fjall_store.rs]

**How to avoid:** Assert reopened runtime does not appear busy due to stale in-memory work and still reports downloaded/connected durable progress. [VERIFIED: packages/open-bitcoin-node/src/sync/tests.rs]

**Warning signs:** `blocks_in_flight > 0` after a fresh reopen without a new request cycle. [VERIFIED: packages/open-bitcoin-node/src/sync.rs]

### Pitfall 5: Phase 71 Checker Drifts Into Release-Boundary Work

**What goes wrong:** A checker can accidentally expand into Phase 72/73/74 observability, UAT, or release-readiness closeout. [VERIFIED: .planning/ROADMAP.md]

**Why it happens:** Resource, restart, support, UAT, and release docs are adjacent surfaces. [VERIFIED: docs/operator/runtime-guide.md] [VERIFIED: docs/parity/catalog/p2p.md]

**How to avoid:** Keep any Phase 71 checker focused on RES-01 through RES-04 source/docs/test contracts and default-verification exclusions. [VERIFIED: .planning/REQUIREMENTS.md] [VERIFIED: scripts/verify.sh]

**Warning signs:** The checker requires public peers, service-manager commands, support bundle completeness across all surfaces, or v1.6 release claims. [VERIFIED: .planning/ROADMAP.md]

## Code Examples

Verified patterns from local source:

### Existing Resource Pressure Projection

```rust
// Source: packages/open-bitcoin-node/src/status.rs
pub struct SyncResourcePressure {
    pub blocks_in_flight: u64,
    pub max_header_requests_in_flight_per_peer: u64,
    pub max_headers_per_message: u64,
    pub max_blocks_in_flight_per_peer: u64,
    pub max_blocks_in_flight_total: u64,
    pub max_messages_per_peer: u64,
    pub max_sync_rounds: u64,
    pub outbound_peers: u32,
    pub target_outbound_peers: u32,
}
```

### Existing Resource-Limit Blocker

```rust
// Source: packages/open-bitcoin-node/src/sync/block_reconcile.rs
if runtime.config.max_blocks_in_flight_total == 0 {
    return Err(SyncRuntimeError::ResourceLimit {
        message:
            "max_blocks_in_flight_total is 0; increase the global block budget to continue sync"
                .to_string(),
    });
}
```

### Existing Metrics Retention Contract

```rust
// Source: packages/open-bitcoin-node/src/metrics.rs
impl Default for MetricRetentionPolicy {
    fn default() -> Self {
        Self {
            sample_interval_seconds: 30,
            max_samples_per_series: 2_880,
            max_age_seconds: 86_400,
        }
    }
}
```

### Existing Log Retention Contract

```rust
// Source: packages/open-bitcoin-node/src/logging.rs
impl Default for LogRetentionPolicy {
    fn default() -> Self {
        Self {
            rotation: LogRotation::Daily,
            max_files: 14,
            max_age_days: 14,
            max_total_bytes: 268_435_456,
        }
    }
}
```

### Existing Storage Error Recovery Mapping

```rust
// Source: packages/open-bitcoin-node/src/storage.rs
pub fn recovery_category(&self) -> SyncRecoveryCategory {
    match self {
        Self::InvalidSchemaVersion { .. } | Self::SchemaMismatch { .. } => {
            SyncRecoveryCategory::IncompatibleSchema
        }
        Self::Corruption { .. } => SyncRecoveryCategory::StoreCorruption,
        Self::UnavailableNamespace { .. } | Self::InterruptedWrite { .. } => {
            SyncRecoveryCategory::StorageBackendFailure
        }
        Self::BackendFailure { message, .. } => {
            if contains_storage_lock_signal(message) {
                return SyncRecoveryCategory::StorageLockContention;
            }

            SyncRecoveryCategory::StorageBackendFailure
        }
    }
}
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|---|---|---|---|
| Public-network long-run proof as default verification | Deterministic scripted fixtures plus opt-in public-network UAT | Active v1.6 planning continues v1.3-v1.5 boundary | Phase 71 plans must keep `bash scripts/verify.sh` public-network-free. [VERIFIED: .planning/REQUIREMENTS.md] [VERIFIED: scripts/verify.sh] |
| Renderer-specific recovery wording | Shared `SyncRecoveryCategory`, `SyncRuntimeError`, `StorageError`, and typed status fields | Phase 61 and later phases | Phase 71 should refine typed guidance, not add CLI-only strings. [VERIFIED: .planning/phases/61-resource-bounds-and-recovery-taxonomy/61-CONTEXT.md] [VERIFIED: packages/open-bitcoin-node/src/status/recovery.rs] |
| Header/download-only progress as enough restart evidence | Validated, connected, durably persisted active-chain progress with reopen tests | Phase 68 | RES-02 resume evidence must preserve connected active-chain, UTXO/undo, runtime metadata, and best-known tip facts. [VERIFIED: .planning/phases/68-full-active-chain-validation-and-durable-persistence/68-CONTEXT.md] |
| Stale in-flight hidden behind generic no-progress | Typed no-progress diagnosis including stale in-flight cleanup and storage/resource blockers | Phase 70 | RES-02 and RES-03 should reuse no-progress diagnosis and add missing resource/storage specificity. [VERIFIED: .planning/phases/70-reorg-peer-rotation-and-no-progress-recovery/70-VERIFICATION.md] |
| Raw support/log evidence | Bounded metrics/log retention and support allowlisted compact summaries | Phases 62 and 65 | RES-01 should test/document compactness rather than storing raw report/log arrays. [VERIFIED: docs/architecture/operator-observability.md] [VERIFIED: packages/open-bitcoin-cli/src/operator/support.rs] |

**Deprecated/outdated:**

- Do not rely on public-mainnet timing, service-manager restart, or live-smoke `--restart-after-progress` for default Phase 71 proof. [VERIFIED: .planning/phases/71-resource-bounds-and-durable-restart-resume/71-CONTEXT.md] [VERIFIED: scripts/verify.sh]
- Do not add raw daemon stdout/stderr tails, raw peer tables, or raw live-smoke reports to support evidence. [VERIFIED: docs/operator/runtime-guide.md] [VERIFIED: packages/open-bitcoin-cli/src/operator/support.rs]
- Do not add existing Rust Bitcoin libraries or trusted shortcut sync paths such as assumeutxo, assumevalid, snapshots, or centralized tip oracles. [VERIFIED: ./AGENTS.md] [VERIFIED: .planning/REQUIREMENTS.md]

## Assumptions Log

> List all claims tagged `[ASSUMED]` in this research. The planner and discuss-phase use this section to identify decisions that need user confirmation before execution.

| # | Claim | Section | Risk if Wrong |
|---|---|---|---|

All claims in this research were verified or cited - no user confirmation needed.

## Open Questions (RESOLVED)

1. **RESOLVED: Should low disk/storage pressure become new variants or typed evidence under `ResourceExhaustion`?**
   - What we know: Existing categories include `resource_exhaustion`, `storage_backend_failure`, `storage_lock_contention`, `store_corruption`, and `incompatible_schema`. [VERIFIED: packages/open-bitcoin-node/src/status/recovery.rs]
   - What's unclear: No dedicated low-disk/storage-pressure type was found in node sync/storage code. [VERIFIED: rg "low|disk|pressure" packages/open-bitcoin-node/src/storage.rs packages/open-bitcoin-node/src/sync]
   - Recommendation: Prefer a small typed storage-pressure fact or error helper if it keeps guidance precise without expanding stable labels unnecessarily. [VERIFIED: .planning/phases/71-resource-bounds-and-durable-restart-resume/71-CONTEXT.md]
   - Resolution selected for planning: use typed `StorageRecoveryAction::FreeDisk` evidence mapped to `SyncRecoveryCategory::ResourceExhaustion`, with the exact operator guidance `Free disk space for the selected datadir, then retry sync.`. [VERIFIED: .planning/phases/71-resource-bounds-and-durable-restart-resume/71-03-PLAN.md]

2. **RESOLVED: How large should the synthetic long-chain fixture be?**
   - What we know: Existing deterministic tests already mine small synthetic blocks and use scripted transport; `bash scripts/verify.sh` must remain short-running and public-network-free. [VERIFIED: packages/open-bitcoin-node/src/sync/tests.rs] [VERIFIED: scripts/verify.sh]
   - What's unclear: The exact chain length that best exercises bounds without slowing default verification should be selected during planning/execution after local test timing. [VERIFIED: .planning/phases/71-resource-bounds-and-durable-restart-resume/71-CONTEXT.md]
   - Recommendation: Start with config-stressed bounded tests that prove caps and retention invariants, then increase fixture length only if the first version fails to exercise request/reopen/retention paths. [VERIFIED: packages/open-bitcoin-node/src/sync/tests.rs]
   - Resolution selected for planning: use exactly `48` synthetic blocks for `phase71_synthetic_long_chain_exercises_resource_bounds_without_public_network`. [VERIFIED: .planning/phases/71-resource-bounds-and-durable-restart-resume/71-02-PLAN.md]

3. **RESOLVED: Should Phase 71 add a checker script?**
   - What we know: `scripts/verify.sh` already runs phase checkers through Phase 70, and Phase 71 likely changes docs/source contracts around resource/restart proof. [VERIFIED: scripts/verify.sh]
   - What's unclear: If implementation stays entirely inside Rust tests and docs, a checker is useful but not automatically mandatory. [VERIFIED: .planning/phases/71-resource-bounds-and-durable-restart-resume/71-CONTEXT.md]
   - Recommendation: Add `scripts/check-phase71-resource-restart.ts` only if there are docs/parity/checker invariants that Rust tests cannot enforce, then wire it after Phase 70. [VERIFIED: scripts/verify.sh]
   - Resolution selected for planning: add `scripts/check-phase71-resource-restart.ts` and wire it into `scripts/verify.sh` after the Phase 70 checker. [VERIFIED: .planning/phases/71-resource-bounds-and-durable-restart-resume/71-04-PLAN.md]

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|---|---|---:|---|---|
| Rust compiler | Rust code/tests | yes | rustc 1.94.1 | None needed. [VERIFIED: rustc --version] |
| Cargo | Rust lint/build/test | yes | cargo 1.94.1 | None needed. [VERIFIED: cargo --version] |
| Bun | TypeScript checkers and LOC/check scripts | yes | 1.3.9 | None needed. [VERIFIED: bun --version] |
| Bazelisk/Bazel | Repo smoke build | yes | bazel 8.6.0 | None needed. [VERIFIED: bazelisk --version] |
| Git submodule baseline | Knots anchors | present path referenced; materialize with repo command if missing | Bitcoin Knots `29.3.knots20260210` per project docs | `git submodule update --init --recursive`. [VERIFIED: ./AGENTS.md] [VERIFIED: .planning/PROJECT.md] |

**Missing dependencies with no fallback:**
- None found for planning Phase 71 deterministic work. [VERIFIED: rustc --version] [VERIFIED: cargo --version] [VERIFIED: bun --version] [VERIFIED: bazelisk --version]

**Missing dependencies with fallback:**
- None found. [VERIFIED: rustc --version] [VERIFIED: cargo --version] [VERIFIED: bun --version] [VERIFIED: bazelisk --version]

## Security Domain

OWASP ASVS is an open application security verification standard for web apps and web services, and the latest stable version verified during research is ASVS 5.0.0 dated May 2025; Phase 71 uses the GSD-requested category table below as a planning lens for the local sync/RPC/operator surfaces. [CITED: https://github.com/OWASP/ASVS] [CITED: https://owasp.org/www-project-application-security-verification-standard/]

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---|---:|---|
| V2 Authentication | no | Do not change RPC credential or operator auth surfaces in this phase; preserve existing local RPC/auth behavior. [VERIFIED: .planning/ROADMAP.md] |
| V3 Session Management | no | No browser or session lifecycle change is in Phase 71 scope. [VERIFIED: .planning/ROADMAP.md] |
| V4 Access Control | no | No new authorization boundary is in Phase 71 scope. [VERIFIED: .planning/ROADMAP.md] |
| V5 Input Validation | yes | Validate public peer inputs through existing codec/consensus/sync paths and validate script/report inputs through typed DTOs and allowlists. [VERIFIED: packages/open-bitcoin-node/src/sync.rs] [VERIFIED: packages/open-bitcoin-cli/src/operator/support/live_smoke.rs] |
| V6 Cryptography | limited | Do not hand-roll new cryptography; keep using existing first-party consensus hashing and pinned `secp256k1` dependency where already used. [VERIFIED: cargo tree --locked -p open-bitcoin-node] [VERIFIED: ./AGENTS.md] |

### Known Threat Patterns for Open Bitcoin Sync

| Pattern | STRIDE | Standard Mitigation |
|---|---|---|
| Peer-driven memory or work exhaustion through headers/blocks/messages | Denial of Service | Enforce header request, protocol header batch, message, round, peer, and block in-flight caps through config and `SyncResourcePressure`. [VERIFIED: packages/open-bitcoin-node/src/status.rs] [VERIFIED: packages/open-bitcoin-node/src/sync/types.rs] |
| Disk exhaustion or storage pressure during long sync | Denial of Service / Tampering | Surface typed storage/resource guidance, preserve storage-first precedence, and avoid hidden repair or mutation. [VERIFIED: .planning/phases/71-resource-bounds-and-durable-restart-resume/71-CONTEXT.md] |
| Corrupted or incompatible durable state after interruption | Tampering / Repudiation | Use `StorageError`, `RecoveryMarker`, schema checks, and explicit operator recovery actions. [VERIFIED: packages/open-bitcoin-node/src/storage.rs] [VERIFIED: packages/open-bitcoin-node/src/storage/fjall_store.rs] |
| Duplicate block connect or unsafe replay after restart | Tampering / Repudiation | Reopen from durable headers/block bodies/chainstate, avoid requesting connected blocks, and assert no duplicate progress credit. [VERIFIED: packages/open-bitcoin-node/src/sync/tests.rs] |
| Raw logs or support bundles leaking credentials/local evidence | Information Disclosure | Keep support evidence allowlisted/redacted and logs bounded; do not embed raw daemon tails or raw peer tables. [VERIFIED: packages/open-bitcoin-cli/src/operator/support.rs] [VERIFIED: docs/operator/runtime-guide.md] |
| Ambiguous "sync failed" guidance hiding storage blockers | Repudiation | Route blockers through `SyncRecoveryCategory`, `SyncRuntimeError`, shared status, and no-progress next actions. [VERIFIED: packages/open-bitcoin-node/src/sync/types/recovery.rs] [VERIFIED: packages/open-bitcoin-node/src/sync/progress.rs] |

## Sources

### Primary (HIGH confidence)

- `.planning/phases/71-resource-bounds-and-durable-restart-resume/71-CONTEXT.md` - locked Phase 71 decisions, scope, canonical refs, and implementation constraints. [VERIFIED: cat]
- `.planning/REQUIREMENTS.md` - RES-01 through RES-04 and default verification exclusions. [VERIFIED: cat]
- `.planning/ROADMAP.md` - Phase 71 success criteria and Phase 72-74 boundaries. [VERIFIED: cat]
- `.planning/STATE.md` - current milestone state and Phase 71 readiness. [VERIFIED: cat]
- `AGENTS.md`, `AGENTS.bright-builds.md`, `standards-overrides.md` - repo-local workflow, Rust, verification, parity breadcrumb, Bun, and standards routing. [VERIFIED: cat]
- `packages/open-bitcoin-node/src/status.rs` and `status/recovery.rs` - `SyncResourcePressure`, shared status fields, `FieldAvailability`, and recovery labels. [VERIFIED: rg/sed]
- `packages/open-bitcoin-node/src/storage.rs` and `storage/fjall_store.rs` - storage namespaces, schema, recovery markers, recovery actions, backend error mapping, clean shutdown, and Fjall persistence. [VERIFIED: rg/sed]
- `packages/open-bitcoin-node/src/sync.rs`, `sync/runtime_state.rs`, `sync/block_reconcile.rs`, `sync/progress.rs`, `sync/types/*.rs`, and `sync/tests.rs` - durable runtime, resource projection, in-flight caps, no-progress guidance, recovery mapping, and deterministic fixtures. [VERIFIED: rg/sed]
- `packages/open-bitcoin-node/src/metrics.rs` and `logging.rs` - metrics and structured log retention contracts. [VERIFIED: sed]
- `scripts/verify.sh` - repo-native deterministic verification contract and phase checker order. [VERIFIED: sed]
- `docs/operator/runtime-guide.md`, `docs/architecture/status-snapshot.md`, `docs/architecture/operator-observability.md`, `docs/architecture/storage-decision.md`, `docs/parity/catalog/p2p.md`, and `docs/parity/catalog/chainstate.md` - current resource, recovery, restart, support evidence, and parity wording. [VERIFIED: sed]

### Secondary (MEDIUM confidence)

- Bright Builds pinned canonical standards pages at commit `05f8d7a6c9c2e157ec4f922a05273e72dab97676` - architecture, code shape, verification, testing, Rust, TypeScript/JavaScript, and operability guidance. [CITED: https://raw.githubusercontent.com/bright-builds-llc/bright-builds-rules/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/index.md]
- OWASP ASVS official project and repository - ASVS purpose and latest stable version context for the security-domain section. [CITED: https://owasp.org/www-project-application-security-verification-standard/] [CITED: https://github.com/OWASP/ASVS]

### Tertiary (LOW confidence)

- None. All recommendations are grounded in local code/docs, repo instructions, or official standards sources. [VERIFIED: this research session]

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - versions verified from local toolchain, Bazel module, Cargo workspace, lock/tree output, and repo docs. [VERIFIED: rustc --version] [VERIFIED: cargo tree --locked] [VERIFIED: MODULE.bazel]
- Architecture: HIGH - implementation surfaces and prior phase constraints align around existing typed contracts and deterministic tests. [VERIFIED: packages/open-bitcoin-node/src/sync.rs] [VERIFIED: .planning/phases/71-resource-bounds-and-durable-restart-resume/71-CONTEXT.md]
- Pitfalls: MEDIUM-HIGH - most pitfalls are verified from current code and prior phase artifacts; low-disk/storage-pressure specifics remain an open planning decision because no dedicated type was found. [VERIFIED: rg "low|disk|pressure" packages/open-bitcoin-node/src/storage.rs packages/open-bitcoin-node/src/sync]

**Research date:** 2026-06-13
**Valid until:** 2026-07-13 for local code patterns; re-check dependency/tool versions before implementation if lockfiles or toolchains change. [VERIFIED: current_date] [VERIFIED: rust-toolchain.toml]
