# Phase 69: Tip Tracking and Stay-Current Operation - Research

**Researched:** 2026-06-11 [VERIFIED: environment current_date]
**Domain:** Rust sync runtime status contracts, durable tip evidence, and bounded daemon stay-current operation [VERIFIED: .planning/phases/69-tip-tracking-and-stay-current-operation/69-CONTEXT.md]
**Confidence:** HIGH for local architecture and verification surfaces; MEDIUM for exact freshness threshold policy because the threshold is intentionally left to implementation discretion [VERIFIED: .planning/phases/69-tip-tracking-and-stay-current-operation/69-CONTEXT.md]

<user_constraints>

## User Constraints (from CONTEXT.md)

All bullets in this section are copied from `.planning/phases/69-tip-tracking-and-stay-current-operation/69-CONTEXT.md`; spelling and scope are preserved for planner use. [VERIFIED: .planning/phases/69-tip-tracking-and-stay-current-operation/69-CONTEXT.md]

### Locked Decisions

## Implementation Decisions

### Best-Known Tip Evidence

- **D-01:** Best-known tip evidence must be a first-class typed status contract,
  not a renderer-specific string. It should include source, height, hash,
  cumulative work, timestamp, freshness, and peer agreement evidence.
- **D-02:** Prefer a deterministic peer-derived tip model: use the validated
  header store and current peer outcomes to derive best-known tip evidence.
  Do not introduce a trusted external tip oracle, checkpoint shortcut,
  assumevalid shortcut, assumeutxo shortcut, centralized peer, or public API
  dependency for the v1.6 sync-to-tip claim.
- **D-03:** Peer agreement evidence should be bounded and auditable. It should
  expose enough detail to tell whether peers agree with the best-known tip,
  lag behind it, disagree with it, or provided no useful tip evidence, without
  requiring operators to inspect raw wire transcripts.
- **D-04:** Persist enough tip evidence in runtime metadata to remain coherent
  across restart and peer rotation. Fresh peer observations may update evidence,
  but restart should not collapse prior durable tip state into "unknown" when
  the store has enough validated header and runtime metadata.

### Stay-Current State Model

- **D-05:** Add or refine a shared stay-current status classification that
  distinguishes `initial_catch_up`, `current_at_best_known_tip`, `stale_tip`,
  `recovering`, and `no_progress`. This classification should be computed in
  core sync/status code and reused by CLI, RPC, dashboard/status JSON, logs, and
  support evidence in later phases.
- **D-06:** Current-at-tip means the validated active-chain height/hash/work is
  at the best-known validated peer tip and the tip evidence is fresh enough for
  the configured deterministic policy. Downloaded-only or headers-only progress
  must not satisfy current-at-tip.
- **D-07:** Stale-tip and no-progress are different states. Stale-tip means the
  best-known tip evidence is old or lacks fresh peer agreement; no-progress
  means work remains or evidence is insufficient and the daemon is not making
  useful progress. Both should carry operator-facing next-action context.
- **D-08:** Recovering remains reserved for typed recovery contexts already
  present in the runtime, such as storage, unclean restart, or peer failure
  recovery. Phase 70 may expand recovery detail, but Phase 69 should not flatten
  recovery into stale-tip or no-progress.

### Runtime Loop Behavior

- **D-09:** After catch-up, `open-bitcoind` should continue bounded daemon wake
  cycles that request fresh headers and needed blocks, validate and connect new
  active-chain blocks, persist progress, and refresh tip evidence.
- **D-10:** Preserve the bounded opt-in daemon posture from v1.5 and Phase 68:
  no hot loops, no unbounded peer fanout, no default public-network verification,
  and no claim that the daemon is a production full node.
- **D-11:** When a daemon wake observes no new work but evidence remains fresh
  and connected progress equals the best-known tip, report stay-current success
  rather than a generic no-progress warning.
- **D-12:** When new headers are observed after catch-up, the runtime should
  transition back through catch-up behavior until the corresponding blocks are
  downloaded, validated, connected, persisted, and reflected in the shared
  progress fields added in Phase 68.

### Operator Surface Boundaries

- **D-13:** Status evidence must preserve the Phase 68 separation among header
  height, downloaded block height, connected block height, validated active-chain
  height, cumulative work, and tip freshness. Phase 69 should add tip/stay-current
  meaning without hiding those lower-level counters.
- **D-14:** Operator wording should explain whether evidence proves caught-up,
  stay-current, stale, recovering, or blocked behavior. Avoid production-node,
  inbound-serving, relay, production-wallet, migration-apply, packaging, GUI,
  hosted-dashboard, or broad readiness phrasing.
- **D-15:** Public-mainnet stay-current review remains opt-in UAT evidence.
  Default verification must remain deterministic, public-network-free,
  service-manager-free, timing-stable, and short-running.

### Verification Posture

- **D-16:** Deterministic tests should prove best-known tip projection,
  peer-agreement classification, stale-tip classification, current-at-tip
  classification, post-catch-up header/block progress, and restart persistence
  of coherent tip evidence.
- **D-17:** Add a focused deterministic checker when docs or status contracts
  need release-boundary guardrails. Keep `bash scripts/verify.sh` as the final
  repo-native verification contract.
- **D-18:** New first-party Rust source or test files under
  `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests` must receive
  parity breadcrumb coverage through `docs/parity/source-breadcrumbs.json` and
  `scripts/check-parity-breadcrumbs.ts`.

### the agent's Discretion

- The planner may split implementation by status-domain types, runtime summary
  projection, daemon-loop stay-current behavior, and deterministic docs/checker
  coverage if that keeps commits reviewable.
- The executor may use a small pure helper module for tip freshness and
  stay-current classification when it reduces duplicated logic across runtime
  summary, durable metadata, status JSON, and tests.
- The executor may choose conservative freshness thresholds as explicit config
  defaults, provided deterministic tests avoid wall-clock flakiness and operator
  docs describe the policy.

### Deferred Ideas (OUT OF SCOPE)

No separate `## Deferred Ideas` section is present in the Phase 69 context; out-of-scope items are embedded in the Phase Boundary and decisions above. [VERIFIED: .planning/phases/69-tip-tracking-and-stay-current-operation/69-CONTEXT.md]

</user_constraints>

<phase_requirements>

## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| TIP-01 | Operator can see the best-known mainnet tip source, height, hash, work, timestamp, freshness, and peer agreement evidence. [VERIFIED: .planning/REQUIREMENTS.md] | Add a typed best-known-tip contract to shared status and project it from validated header store plus bounded peer observations. [VERIFIED: packages/open-bitcoin-node/src/status.rs; packages/open-bitcoin-node/src/sync/runtime_state.rs; packages/open-bitcoin-network/src/header_store.rs] |
| TIP-02 | Operator can distinguish initial catch-up, current-at-best-known-tip, stale-tip, recovering, and no-progress states without renderer-specific interpretation. [VERIFIED: .planning/REQUIREMENTS.md] | Add a shared stay-current enum/status computed in sync/runtime code, not CLI/RPC renderer strings. [VERIFIED: packages/open-bitcoin-node/src/status.rs; packages/open-bitcoin-node/src/sync/types/summary.rs] |
| TIP-03 | Operator can keep `open-bitcoind` running after catch-up so new headers and blocks are detected, validated, connected, and reported as stay-current progress. [VERIFIED: .planning/REQUIREMENTS.md] | Reuse the bounded daemon loop that calls `sync_until_idle`, then classify idle cycles as stay-current when tip evidence is fresh and connected progress equals the best-known tip. [VERIFIED: packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs; packages/open-bitcoin-node/src/sync.rs] |

</phase_requirements>

## Summary

Phase 69 should be planned as an additive status/runtime phase, not a new sync stack. [VERIFIED: .planning/phases/69-tip-tracking-and-stay-current-operation/69-CONTEXT.md] The existing runtime already has validated header storage, durable active-chain progress fields from Phase 68, bounded peer outcomes, persistent runtime metadata, CLI/RPC/status projection, and a bounded daemon worker loop. [VERIFIED: packages/open-bitcoin-network/src/header_store.rs; packages/open-bitcoin-node/src/status.rs; packages/open-bitcoin-node/src/sync.rs; packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs; .planning/phases/68-full-active-chain-validation-and-durable-persistence/68-VERIFICATION.md]

The missing planning unit is a typed contract for "best-known tip" and "stay-current state". [VERIFIED: packages/open-bitcoin-node/src/status.rs; packages/open-bitcoin-node/src/sync/types/summary.rs] The planner should require source, height, hash, cumulative work, header timestamp, observation timestamp, freshness, and bounded peer agreement evidence, then compute `initial_catch_up`, `current_at_best_known_tip`, `stale_tip`, `recovering`, and `no_progress` in `open-bitcoin-node` before CLI, RPC, dashboard, logs, and docs consume it. [VERIFIED: .planning/phases/69-tip-tracking-and-stay-current-operation/69-CONTEXT.md]

**Primary recommendation:** Add typed `BestKnownTipStatus` and `StayCurrentStatus` fields to `SyncStatus`, derive them from `HeaderStore::best_tip`, connected active-chain progress, and per-peer tip observations, persist them through `RuntimeMetadata.maybe_sync_state`, and cover them with deterministic sync tests plus a focused Phase 69 checker. [VERIFIED: packages/open-bitcoin-node/src/status.rs; packages/open-bitcoin-node/src/storage.rs; packages/open-bitcoin-node/src/sync/tests.rs; scripts/check-phase68-active-chain-persistence.ts]

## Project Constraints (from AGENTS.md)

- Use `AGENTS.md` as the repo-local instruction entrypoint, then `AGENTS.bright-builds.md`, `standards-overrides.md`, and relevant canonical Bright Builds pages before plan, review, implementation, or audit work. [VERIFIED: AGENTS.md; AGENTS.bright-builds.md; standards-overrides.md; CITED: https://raw.githubusercontent.com/bright-builds-llc/bright-builds-rules/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/index.md]
- Use `rust-toolchain.toml` as the Rust source of truth; the pinned Rust version is `1.94.1`. [VERIFIED: AGENTS.md; rust-toolchain.toml]
- Use `bash scripts/verify.sh` as the repo-native verification contract for first-party code, including the Bazel smoke build. [VERIFIED: AGENTS.md; scripts/verify.sh]
- Use Bun for repo-owned higher-level automation scripts and TypeScript for substantial script logic. [VERIFIED: AGENTS.md; .bun-version; scripts/verify.sh]
- Treat `docs/metrics/lines-of-code.md` as tracked generated output when verification regenerates it. [VERIFIED: AGENTS.md]
- Add parity breadcrumb coverage for new first-party Rust source or test files under `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests`. [VERIFIED: AGENTS.md]
- Preserve Knots `29.3.knots20260210` externally observable behavior for in-scope surfaces and keep parity evidence auditable. [VERIFIED: AGENTS.md; git submodule status]
- Keep pure domain logic in functional-core style and effects in thin adapters. [VERIFIED: AGENTS.md; CITED: https://raw.githubusercontent.com/bright-builds-llc/bright-builds-rules/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/core/architecture.md]
- Prefer early returns, `let...else` for Rust guard extraction, `maybe_` names for internal `Option` values, and newtypes/enums for invariants that prevent illegal states. [VERIFIED: AGENTS.md; CITED: https://raw.githubusercontent.com/bright-builds-llc/bright-builds-rules/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/core/code-shape.md; CITED: https://raw.githubusercontent.com/bright-builds-llc/bright-builds-rules/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/languages/rust.md]
- Pure/business logic must have unit tests, and Rust tests should use Arrange, Act, Assert comments when setup is non-trivial. [VERIFIED: AGENTS.md; CITED: https://raw.githubusercontent.com/bright-builds-llc/bright-builds-rules/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/core/testing.md]
- Public-network and real service-manager checks remain opt-in UAT unless a future phase deliberately changes the deterministic verification contract. [VERIFIED: .planning/REQUIREMENTS.md; .planning/ROADMAP.md; .planning/STATE.md]

## Standard Stack

### Core

| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| Rust workspace crates under `packages/open-bitcoin-*` | workspace version `0.1.0`, Rust edition 2024 [VERIFIED: packages/Cargo.toml] | Own the first-party sync, status, storage, network, RPC, and CLI surfaces. [VERIFIED: AGENTS.md; packages/Cargo.toml] | Project policy forbids production-path Rust Bitcoin libraries and keeps the domain model first-party. [VERIFIED: AGENTS.md] |
| `open-bitcoin-node` status/sync/storage modules | workspace local [VERIFIED: packages/Cargo.toml] | Own `SyncStatus`, `DurableSyncRuntime`, `RuntimeMetadata`, and Fjall-backed persistence. [VERIFIED: packages/open-bitcoin-node/src/status.rs; packages/open-bitcoin-node/src/sync.rs; packages/open-bitcoin-node/src/storage.rs] | These are already the shared truth source for CLI/RPC/status/dashboard/support consumers. [VERIFIED: .planning/phases/62-long-run-sync-truth-surfaces/62-CONTEXT.md; packages/open-bitcoin-cli/src/operator/status.rs] |
| `open-bitcoin-network::HeaderStore` | workspace local [VERIFIED: packages/Cargo.toml] | Stores validated headers and selects a deterministic best tip. [VERIFIED: packages/open-bitcoin-network/src/header_store.rs] | It already ranks candidate tips by chain work, height, and hash; it is the correct source for best-known validated header evidence. [VERIFIED: packages/open-bitcoin-network/src/header_store.rs] |
| Fjall | `3.1.4` [VERIFIED: packages/open-bitcoin-node/Cargo.toml; packages/Cargo.lock] | Durable store for headers, chainstate snapshots, runtime metadata, metrics, recovery markers, and block bodies. [VERIFIED: packages/open-bitcoin-node/src/storage/fjall_store.rs] | Phase 69 should persist additive tip state through the existing runtime metadata path instead of adding another store. [VERIFIED: packages/open-bitcoin-node/src/storage.rs; packages/open-bitcoin-node/src/storage/fjall_store.rs] |

### Supporting

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| serde / serde_json | `1.0.228` / `1.0.149` [VERIFIED: packages/open-bitcoin-node/Cargo.toml; packages/open-bitcoin-cli/Cargo.toml; packages/open-bitcoin-rpc/Cargo.toml] | Stable status JSON, RPC payloads, and versioned runtime metadata DTOs. [VERIFIED: packages/open-bitcoin-node/src/status.rs; packages/open-bitcoin-node/src/storage/snapshot_codec.rs] | Use for additive status fields with serde defaults so older runtime metadata remains decodable. [VERIFIED: packages/open-bitcoin-node/src/status.rs; packages/open-bitcoin-node/src/storage/snapshot_codec.rs] |
| Tokio / Axum | `1.52.1` / `0.8.9` [VERIFIED: packages/open-bitcoin-rpc/Cargo.toml] | Existing `open-bitcoind` JSON-RPC server runtime. [VERIFIED: packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs; packages/open-bitcoin-rpc/src/context.rs] | Do not change for this phase unless RPC method response wiring needs new typed fields. [VERIFIED: packages/open-bitcoin-rpc/src/method/node.rs] |
| clap / ratatui / crossterm | `4.6.1` / `0.30` / `0.29` [VERIFIED: packages/open-bitcoin-cli/Cargo.toml] | Existing operator CLI, dashboard, and terminal rendering. [VERIFIED: packages/open-bitcoin-cli/src/operator/status/render.rs; packages/open-bitcoin-cli/src/operator/dashboard/model.rs] | Render only already-typed status fields; do not reclassify stay-current state in renderers. [VERIFIED: .planning/phases/69-tip-tracking-and-stay-current-operation/69-CONTEXT.md] |
| Bun | `1.3.9` [VERIFIED: .bun-version; environment audit] | Repo-owned deterministic checker runtime. [VERIFIED: AGENTS.md; scripts/check-phase68-active-chain-persistence.ts] | Use for a focused Phase 69 contract checker only if docs/status boundary text changes. [VERIFIED: .planning/phases/69-tip-tracking-and-stay-current-operation/69-CONTEXT.md; scripts/verify.sh] |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| First-party validated header store | Public tip API, checkpoint shortcut, assumevalid, assumeutxo, centralized peer | Explicitly forbidden for the v1.6 claim and would undermine the audited first-party validation story. [VERIFIED: .planning/phases/69-tip-tracking-and-stay-current-operation/69-CONTEXT.md; .planning/REQUIREMENTS.md] |
| Additive typed status fields | Renderer-local strings in CLI/RPC/dashboard | Renderer-local interpretation is explicitly forbidden and would break cross-surface consistency. [VERIFIED: .planning/phases/69-tip-tracking-and-stay-current-operation/69-CONTEXT.md; .planning/phases/62-long-run-sync-truth-surfaces/62-CONTEXT.md] |
| Existing runtime metadata JSON via serde | New sidecar tip database or ad hoc JSON blob | Existing `RuntimeMetadata.maybe_sync_state` is already persisted and consumed by CLI/RPC/status, while a sidecar would add a second truth source. [VERIFIED: packages/open-bitcoin-node/src/storage.rs; packages/open-bitcoin-cli/src/operator/status/sync_state.rs; packages/open-bitcoin-rpc/src/context.rs] |

**Installation:** No new package installation is recommended for Phase 69. [VERIFIED: packages/Cargo.toml; .planning/phases/69-tip-tracking-and-stay-current-operation/69-CONTEXT.md]

**Version verification:** Recommended versions were verified from repo manifests and installed tools rather than npm because Phase 69 should not add npm packages. [VERIFIED: packages/Cargo.toml; packages/Cargo.lock; rust-toolchain.toml; .bun-version; environment audit]

## Architecture Patterns

### Recommended Project Structure

```text
packages/open-bitcoin-node/src/
├── status.rs                  # Add public typed tip/stay-current status DTOs. [VERIFIED: packages/open-bitcoin-node/src/status.rs]
├── sync.rs                    # Keep daemon sync orchestration in DurableSyncRuntime. [VERIFIED: packages/open-bitcoin-node/src/sync.rs]
├── sync/runtime_state.rs      # Project and persist durable best-tip/stay-current evidence. [VERIFIED: packages/open-bitcoin-node/src/sync/runtime_state.rs]
├── sync/types.rs              # Add peer observation and runtime config fields if needed. [VERIFIED: packages/open-bitcoin-node/src/sync/types.rs]
├── sync/types/summary.rs      # Include typed status/log/metric projection from summaries. [VERIFIED: packages/open-bitcoin-node/src/sync/types/summary.rs]
├── sync/tip.rs                # Optional pure helper for freshness, agreement, and stay-current classification; add parity breadcrumb if created. [VERIFIED: .planning/phases/69-tip-tracking-and-stay-current-operation/69-CONTEXT.md; AGENTS.md]
└── sync/tests.rs              # Add deterministic scripted peer/restart/stay-current tests. [VERIFIED: packages/open-bitcoin-node/src/sync/tests.rs]
```

### Pattern 1: Additive Typed Status Contract

**What:** Add `FieldAvailability<BestKnownTipStatus>` and `FieldAvailability<StayCurrentStatus>` or equivalent additive fields to `SyncStatus`. [VERIFIED: packages/open-bitcoin-node/src/status.rs]

**When to use:** Use this when evidence must be shared by CLI, RPC, dashboard/status JSON, logs, and later support evidence without renderer-specific interpretation. [VERIFIED: .planning/phases/69-tip-tracking-and-stay-current-operation/69-CONTEXT.md; .planning/phases/62-long-run-sync-truth-surfaces/62-CONTEXT.md]

**Example:**

```rust
// Source: packages/open-bitcoin-node/src/status.rs and Phase 69 context.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StayCurrentPhase {
    InitialCatchUp,
    CurrentAtBestKnownTip,
    StaleTip,
    Recovering,
    NoProgress,
}
```

### Pattern 2: Pure Classification, Effectful Projection

**What:** Keep freshness, peer agreement, and stay-current classification as pure data-in/data-out helpers, then call them from `DurableSyncRuntime::durable_sync_state_from_summary`. [VERIFIED: packages/open-bitcoin-node/src/sync/runtime_state.rs; CITED: https://raw.githubusercontent.com/bright-builds-llc/bright-builds-rules/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/core/architecture.md]

**When to use:** Use this for logic that compares connected active-chain height/hash/work, best-known validated header tip, peer observations, timestamps, and lifecycle/recovery inputs. [VERIFIED: .planning/phases/69-tip-tracking-and-stay-current-operation/69-CONTEXT.md; packages/open-bitcoin-node/src/sync/runtime_state.rs]

**Example:**

```rust
// Source: local architecture pattern in packages/open-bitcoin-node/src/sync/runtime_state.rs.
fn classify_stay_current(input: StayCurrentInput) -> StayCurrentPhase {
    if input.lifecycle == SyncLifecycleState::Recovering {
        return StayCurrentPhase::Recovering;
    }
    if !input.tip_is_fresh {
        return StayCurrentPhase::StaleTip;
    }
    if input.connected_tip == input.best_known_tip {
        return StayCurrentPhase::CurrentAtBestKnownTip;
    }
    if input.made_useful_progress {
        return StayCurrentPhase::InitialCatchUp;
    }
    StayCurrentPhase::NoProgress
}
```

### Pattern 3: Persist Through Existing Runtime Metadata

**What:** Persist the new evidence by embedding it in `DurableSyncState.sync` inside `RuntimeMetadata.maybe_sync_state`; add serde defaults for new fields. [VERIFIED: packages/open-bitcoin-node/src/storage.rs; packages/open-bitcoin-node/src/status.rs; packages/open-bitcoin-node/src/storage/snapshot_codec.rs]

**When to use:** Use this for restart and peer-rotation coherence because CLI and RPC already fall back to durable runtime metadata when live RPC is unavailable. [VERIFIED: packages/open-bitcoin-cli/src/operator/status.rs; packages/open-bitcoin-rpc/src/context.rs]

**Example:**

```rust
// Source: packages/open-bitcoin-node/src/status.rs serde default pattern.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SyncStatus {
    #[serde(default = "best_known_tip_unavailable")]
    pub best_known_tip: FieldAvailability<BestKnownTipStatus>,
}
```

### Anti-Patterns to Avoid

- **Current-at-tip from headers only:** Current-at-tip must require validated active-chain height/hash/work at the best-known validated peer tip, not only header or downloaded-block progress. [VERIFIED: .planning/phases/69-tip-tracking-and-stay-current-operation/69-CONTEXT.md; .planning/phases/68-full-active-chain-validation-and-durable-persistence/68-VERIFICATION.md]
- **No-progress for fresh idle success:** A daemon wake with fresh tip evidence and connected progress equal to the best-known tip should report stay-current success, not generic no-progress. [VERIFIED: .planning/phases/69-tip-tracking-and-stay-current-operation/69-CONTEXT.md]
- **Unbounded peer transcripts:** Peer agreement should be bounded typed evidence, not raw wire transcripts or unbounded endpoint tables. [VERIFIED: .planning/phases/69-tip-tracking-and-stay-current-operation/69-CONTEXT.md; .planning/phases/65-support-bundle-and-operator-review-docs/65-CONTEXT.md]
- **Schema-breaking runtime metadata:** Adding non-defaulted fields inside persisted `RuntimeMetadata` can make old stores fail to decode. [VERIFIED: packages/open-bitcoin-node/src/storage/snapshot_codec.rs; packages/open-bitcoin-node/src/status.rs]

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Best-known tip source | External oracle, centralized peer, public API, checkpoint, assumevalid, or assumeutxo shortcut | `HeaderStore::best_tip` plus current peer observations | The phase explicitly requires deterministic peer-derived evidence and forbids shortcut trust. [VERIFIED: .planning/phases/69-tip-tracking-and-stay-current-operation/69-CONTEXT.md; packages/open-bitcoin-network/src/header_store.rs] |
| Stay-current classification | CLI/RPC/dashboard string parsing | Shared Rust enum/status field in `open-bitcoin-node` | Renderers already consume typed `SyncStatus`; classification belongs before rendering. [VERIFIED: packages/open-bitcoin-node/src/status.rs; packages/open-bitcoin-cli/src/operator/status/render.rs] |
| Runtime persistence | Sidecar status files or raw JSON patches | `RuntimeMetadata.maybe_sync_state` and `FjallNodeStore::save_runtime_metadata` | Existing CLI/RPC/status paths already load durable metadata and preserve unavailable reasons. [VERIFIED: packages/open-bitcoin-node/src/storage.rs; packages/open-bitcoin-node/src/storage/fjall_store.rs; packages/open-bitcoin-cli/src/operator/status/sync_state.rs] |
| Peer agreement evidence | Raw transcript dumps or unbounded peer arrays | Bounded `PeerSyncOutcome`/peer observation rows and aggregate counts | Prior support and truth-surface decisions require bounded evidence and redaction-safe summaries. [VERIFIED: .planning/phases/62-long-run-sync-truth-surfaces/62-CONTEXT.md; .planning/phases/65-support-bundle-and-operator-review-docs/65-CONTEXT.md] |
| Header/block validation | New custom consensus shortcut | Existing network/header sync and chainstate validation path | Phase 68 already proved progress credit from consensus-validated connected active-chain state. [VERIFIED: .planning/phases/68-full-active-chain-validation-and-durable-persistence/68-VERIFICATION.md; packages/open-bitcoin-node/src/sync/block_reconcile.rs] |

**Key insight:** The hard part is evidence semantics, not transport plumbing; the existing daemon already wakes, requests headers/blocks, persists state, and retries, but it lacks a typed way to say "freshly current at the best-known tip." [VERIFIED: packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs; packages/open-bitcoin-node/src/sync.rs; packages/open-bitcoin-node/src/status.rs]

## Common Pitfalls

### Pitfall 1: Treating Header Tip as Current Active Chain

**What goes wrong:** Status says current when only validated headers reached the best-known tip. [VERIFIED: .planning/phases/69-tip-tracking-and-stay-current-operation/69-CONTEXT.md]

**Why it happens:** `SyncRunSummary` already has `best_header_height` and `best_block_height`, and older code can confuse header progress with connected chainstate progress. [VERIFIED: packages/open-bitcoin-node/src/sync/types.rs; packages/open-bitcoin-node/src/sync/types/summary.rs]

**How to avoid:** Compare `SyncProgress.validated_active_chain_height/hash/work` to best-known tip height/hash/work before returning `current_at_best_known_tip`. [VERIFIED: packages/open-bitcoin-node/src/status.rs; .planning/phases/68-full-active-chain-validation-and-durable-persistence/68-VERIFICATION.md]

**Warning signs:** Tests pass when headers advance but block bodies are not downloaded, validated, connected, and persisted. [VERIFIED: packages/open-bitcoin-node/src/sync/tests.rs]

### Pitfall 2: Collapsing Stale-Tip and No-Progress

**What goes wrong:** Operators cannot tell old tip evidence from a lack of useful work. [VERIFIED: .planning/phases/69-tip-tracking-and-stay-current-operation/69-CONTEXT.md]

**Why it happens:** Current `SyncStopReason::NoProgress` and `SyncProgressSignal::Steady` do not encode freshness or peer agreement. [VERIFIED: packages/open-bitcoin-node/src/sync/types.rs; packages/open-bitcoin-node/src/status.rs]

**How to avoid:** Add separate typed fields for tip freshness and stay-current phase, and derive no-progress only when work remains or evidence is insufficient and useful progress is absent. [VERIFIED: .planning/phases/69-tip-tracking-and-stay-current-operation/69-CONTEXT.md]

**Warning signs:** A fresh idle cycle at connected tip produces `no_progress` warning text. [VERIFIED: packages/open-bitcoin-node/src/sync.rs; .planning/phases/69-tip-tracking-and-stay-current-operation/69-CONTEXT.md]

### Pitfall 3: Wall-Clock Flaky Tests

**What goes wrong:** Freshness tests depend on real current time or public-network timing. [VERIFIED: .planning/phases/69-tip-tracking-and-stay-current-operation/69-CONTEXT.md; .planning/REQUIREMENTS.md]

**Why it happens:** The daemon uses current time in the worker, but sync runtime tests already pass explicit timestamps to deterministic cycles. [VERIFIED: packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs; packages/open-bitcoin-node/src/sync/tests.rs]

**How to avoid:** Thread explicit timestamps through freshness helpers and use fixed test timestamps. [VERIFIED: packages/open-bitcoin-node/src/sync.rs; packages/open-bitcoin-node/src/sync/tests.rs]

**Warning signs:** Tests need sleeps, public peers, or "today's tip" timing. [VERIFIED: .planning/REQUIREMENTS.md; scripts/verify.sh]

### Pitfall 4: Breaking Old Runtime Metadata

**What goes wrong:** Stores written before Phase 69 fail to decode because new nested status fields lack defaults. [VERIFIED: packages/open-bitcoin-node/src/storage/snapshot_codec.rs; packages/open-bitcoin-node/src/status.rs]

**Why it happens:** Runtime metadata is serialized as a versioned JSON payload and includes `DurableSyncState`; missing serde defaults on new fields can make deserialization strict. [VERIFIED: packages/open-bitcoin-node/src/storage.rs; packages/open-bitcoin-node/src/storage/snapshot_codec.rs]

**How to avoid:** Add `#[serde(default = "...")]` for additive `SyncStatus` fields and include reopen/decode tests. [VERIFIED: packages/open-bitcoin-node/src/status.rs; packages/open-bitcoin-node/src/sync/tests.rs]

**Warning signs:** `FjallNodeStore::load_runtime_metadata` fails for a fixture without Phase 69 fields. [VERIFIED: packages/open-bitcoin-node/src/storage/fjall_store.rs]

## Code Examples

### Best-Tip Projection Input

```rust
// Source: packages/open-bitcoin-network/src/header_store.rs
let maybe_best_tip = runtime
    .network
    .peer_manager()
    .header_store()
    .best_tip();
```

`HeaderStore::best_tip` returns a `HeaderEntry` with `height`, `block_hash`, `header`, and `chain_work`, which are the core fields TIP-01 needs. [VERIFIED: packages/open-bitcoin-network/src/header_store.rs]

### Runtime Status Projection Hook

```rust
// Source: packages/open-bitcoin-node/src/sync/runtime_state.rs
let mut sync = summary.sync_status(self.config.network);
// Phase 69 should fill additive tip/stay-current fields here, before persisting DurableSyncState.
```

`durable_sync_state_from_summary` is the natural hook because it already merges summary progress, connected active-chain evidence, lifecycle, recovery, and resource pressure before returning `DurableSyncState`. [VERIFIED: packages/open-bitcoin-node/src/sync/runtime_state.rs]

### Peer Observation Source

```rust
// Source: packages/open-bitcoin-node/src/sync/progress.rs and sync.rs
progress.record_activity(timestamp);
progress.record_validated_headers(header_count);
progress.maybe_capabilities = self.peer_capabilities(peer_id);
```

Current peer outcomes already carry activity time, header/block contribution, capabilities, state, failure reason, and optional endpoint; Phase 69 should add bounded tip observation fields rather than raw transcripts. [VERIFIED: packages/open-bitcoin-node/src/sync/progress.rs; packages/open-bitcoin-node/src/sync/types.rs]

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Phase 68 status exposes header/downloaded/connected/validated active-chain progress but not a first-class best-known tip evidence contract. [VERIFIED: .planning/phases/68-full-active-chain-validation-and-durable-persistence/68-VERIFICATION.md; packages/open-bitcoin-node/src/status.rs] | Phase 69 should add typed best-known tip and stay-current evidence on top of those counters. [VERIFIED: .planning/phases/69-tip-tracking-and-stay-current-operation/69-CONTEXT.md] | v1.6 Phase 69, after Phase 68 completed on 2026-06-11. [VERIFIED: .planning/ROADMAP.md; .planning/phases/68-full-active-chain-validation-and-durable-persistence/68-VERIFICATION.md] | Operators get proof of current/stale/no-progress semantics without losing low-level counters. [VERIFIED: .planning/REQUIREMENTS.md] |
| Bitcoin Knots tracks best header by accumulated chain work and requests headers/blocks from peers when new work is announced. [VERIFIED: packages/bitcoin-knots/src/node/blockstorage.cpp; packages/bitcoin-knots/src/net_processing.cpp] | Open Bitcoin should keep using first-party validated header and active-chain paths rather than external tip APIs. [VERIFIED: packages/open-bitcoin-network/src/header_store.rs; .planning/phases/69-tip-tracking-and-stay-current-operation/69-CONTEXT.md] | Pinned Knots baseline is `v29.3.knots20260210`. [VERIFIED: git submodule status; AGENTS.md] | Tip evidence can remain parity-aligned at the observable behavior level while staying simpler internally. [VERIFIED: AGENTS.md; packages/bitcoin-knots/src/node/blockstorage.cpp] |
| Prior daemon loop retries every bounded wake and reports `NoProgress` when summary progress stops changing. [VERIFIED: packages/open-bitcoin-node/src/sync.rs; packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs] | Phase 69 should classify fresh idle-at-tip as `current_at_best_known_tip` instead of no-progress. [VERIFIED: .planning/phases/69-tip-tracking-and-stay-current-operation/69-CONTEXT.md] | v1.6 Phase 69. [VERIFIED: .planning/ROADMAP.md] | Stay-current operation becomes truthful after catch-up without adding hot loops. [VERIFIED: .planning/phases/60-unattended-sync-loop-control/60-CONTEXT.md; .planning/phases/69-tip-tracking-and-stay-current-operation/69-CONTEXT.md] |

**Deprecated/outdated:** Do not plan Phase 69 around `packages/open-bitcoin-cli/src/operator/sync.rs`; that file does not exist and current sync status code lives under `packages/open-bitcoin-cli/src/operator/status/`. [VERIFIED: rg --files packages/open-bitcoin-cli/src; packages/open-bitcoin-cli/src/operator/status/sync_state.rs]

## Open Questions (RESOLVED)

1. **RESOLVED: What exact freshness threshold should be the default?** [VERIFIED: .planning/phases/69-tip-tracking-and-stay-current-operation/69-CONTEXT.md]
   - What we know: The context allows conservative explicit config defaults and requires deterministic tests that avoid wall-clock flakiness. [VERIFIED: .planning/phases/69-tip-tracking-and-stay-current-operation/69-CONTEXT.md]
   - Resolution: Use an additive `tip_freshness_threshold_seconds` runtime config field with a default of `1_200` seconds, equal to two target-spacing windows. This is conservative enough for deterministic local status semantics without implying public-mainnet timing guarantees. [VERIFIED: packages/open-bitcoin-node/src/sync/types.rs; packages/open-bitcoin-node/src/sync/tests.rs]
   - Planning requirement: Tests must use fixed timestamps and must not sleep, query public peers, or depend on today's public tip. [VERIFIED: .planning/REQUIREMENTS.md; scripts/verify.sh]

2. **RESOLVED: How broad should Phase 69 cross-surface rollout be?** [VERIFIED: .planning/ROADMAP.md]
   - What we know: The shared status field should be reusable by CLI, RPC, dashboard/status JSON, logs, and later support evidence. [VERIFIED: .planning/phases/69-tip-tracking-and-stay-current-operation/69-CONTEXT.md]
   - Resolution: Phase 69 updates the shared `SyncStatus` contract, durable runtime projection, minimal existing status consumers that deserialize/render the shared contract, structured logs only where needed to preserve the current status truth, docs, and a focused checker. Broad support-bundle breadth, cross-surface comparison, and operator-evidence unification remain Phase 72. [VERIFIED: .planning/ROADMAP.md; .planning/phases/69-tip-tracking-and-stay-current-operation/69-CONTEXT.md]
   - Planning requirement: Docs and checker coverage must explain Phase 69 fields without claiming production-node, inbound-serving, relay, production-wallet, migration-apply, hosted-dashboard, GUI, or public-network default-verification scope. [VERIFIED: .planning/REQUIREMENTS.md; .planning/phases/69-tip-tracking-and-stay-current-operation/69-CONTEXT.md]

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|-------------|-----------|---------|----------|
| Rust toolchain | Compile/test first-party Rust status and sync changes. [VERIFIED: AGENTS.md; scripts/verify.sh] | yes [VERIFIED: environment audit] | `cargo 1.94.1`, `rustc 1.94.1` [VERIFIED: environment audit; rust-toolchain.toml] | None needed. [VERIFIED: environment audit] |
| cargo-llvm-cov | Repo-native coverage gate inside `scripts/verify.sh`. [VERIFIED: scripts/verify.sh] | yes [VERIFIED: environment audit] | `0.8.5` [VERIFIED: environment audit] | None needed. [VERIFIED: environment audit] |
| Bun | Deterministic TypeScript checkers. [VERIFIED: AGENTS.md; scripts/verify.sh] | yes [VERIFIED: environment audit] | `1.3.9` [VERIFIED: environment audit; .bun-version] | None needed. [VERIFIED: environment audit] |
| Bazel/Bazelisk | Repo-native Bazel smoke build. [VERIFIED: AGENTS.md; scripts/verify.sh] | yes [VERIFIED: environment audit] | Bazelisk `1.28.1`, Bazel `8.6.0` [VERIFIED: environment audit] | None needed. [VERIFIED: environment audit] |
| Bitcoin Knots submodule | Parity source anchors. [VERIFIED: AGENTS.md] | yes [VERIFIED: git submodule status] | `a9aee730466ac67d35a3c03ee24676be5e045878` at `v29.3.knots20260210` [VERIFIED: git submodule status] | Run `git submodule update --init --recursive` if absent. [VERIFIED: AGENTS.md] |

**Missing dependencies with no fallback:** None found for planning Phase 69. [VERIFIED: environment audit]

**Missing dependencies with fallback:** None found for planning Phase 69. [VERIFIED: environment audit]

## Security Domain

`security_enforcement` is not explicitly disabled in `.planning/config.json`, so include security considerations for the phase. [VERIFIED: .planning/config.json]

### Applicable ASVS Categories

OWASP ASVS is a web-application verification standard for technical security controls; current ASVS 5.0 chapter numbering differs from older templates, so this table uses the current chapter names where verified. [CITED: https://owasp.org/www-project-application-security-verification-standard/; CITED: https://github.com/OWASP/ASVS]

| ASVS Category | Applies | Standard Control |
|---------------|---------|------------------|
| V1 Encoding and Sanitization | yes, for JSON/status/log rendering of untrusted peer-derived text. [CITED: https://raw.githubusercontent.com/OWASP/ASVS/master/5.0/en/0x10-V1-Encoding-and-Sanitization.md; VERIFIED: packages/open-bitcoin-node/src/status.rs] | Keep structured serde output, avoid dynamic code execution, and keep human rendering as display-only text. [VERIFIED: packages/open-bitcoin-cli/src/operator/status/render.rs; packages/open-bitcoin-node/src/sync/types/summary.rs] |
| V2 Validation and Business Logic | yes, for P2P tip evidence and stay-current decisions. [CITED: https://raw.githubusercontent.com/OWASP/ASVS/master/5.0/en/0x11-V2-Validation-and-Business-Logic.md; VERIFIED: packages/open-bitcoin-network/src/header_store.rs] | Validate headers through existing header/chainstate paths and encode current/stale/no-progress as enums. [VERIFIED: packages/open-bitcoin-node/src/network/header_sync.rs; packages/open-bitcoin-node/src/status.rs] |
| V4 API and Web Service | limited, because RPC/status JSON may expose the new fields. [CITED: https://raw.githubusercontent.com/OWASP/ASVS/master/5.0/en/0x13-V4-API-and-Web-Service.md; VERIFIED: packages/open-bitcoin-rpc/src/method/node.rs] | Use existing typed serde RPC/status response shapes and do not add new unauthenticated control methods. [VERIFIED: packages/open-bitcoin-rpc/src/method/node.rs; packages/open-bitcoin-rpc/src/context.rs] |
| V5 File Handling | limited, because runtime metadata is durable file-backed state. [CITED: https://raw.githubusercontent.com/OWASP/ASVS/master/5.0/en/0x14-V5-File-Handling.md; VERIFIED: packages/open-bitcoin-node/src/storage/fjall_store.rs] | Keep versioned serde DTOs, storage namespaces, and recovery markers instead of ad hoc file writes. [VERIFIED: packages/open-bitcoin-node/src/storage.rs; packages/open-bitcoin-node/src/storage/snapshot_codec.rs] |
| V6 Authentication / V7 Session Management / V8 Authorization | no new behavior in Phase 69. [CITED: https://raw.githubusercontent.com/OWASP/ASVS/master/5.0/en/0x15-V6-Authentication.md; CITED: https://raw.githubusercontent.com/OWASP/ASVS/master/5.0/en/0x16-V7-Session-Management.md; CITED: https://raw.githubusercontent.com/OWASP/ASVS/master/5.0/en/0x17-V8-Authorization.md] | Preserve existing RPC auth/control surfaces and avoid scope creep into auth/session/access-control redesign. [VERIFIED: packages/open-bitcoin-rpc/src/context.rs; .planning/ROADMAP.md] |
| V11 Cryptography | no new cryptography expected. [CITED: https://raw.githubusercontent.com/OWASP/ASVS/master/5.0/en/0x20-V11-Cryptography.md; VERIFIED: .planning/phases/69-tip-tracking-and-stay-current-operation/69-CONTEXT.md] | Do not hand-roll crypto or trust external signed/oracle tip data; use consensus/header validation already present. [VERIFIED: packages/open-bitcoin-node/src/network/header_sync.rs; .planning/phases/69-tip-tracking-and-stay-current-operation/69-CONTEXT.md] |

### Known Threat Patterns for Phase 69

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Malicious peer advertises false or lagging tip evidence. [VERIFIED: packages/open-bitcoin-network/src/peer.rs; .planning/phases/69-tip-tracking-and-stay-current-operation/69-CONTEXT.md] | Spoofing/Tampering | Accept only validated headers into best-tip evidence and classify peer agreement separately from current-at-tip. [VERIFIED: packages/open-bitcoin-node/src/network/header_sync.rs; packages/open-bitcoin-network/src/header_store.rs] |
| Old runtime metadata decodes as unknown or fails after upgrade. [VERIFIED: packages/open-bitcoin-node/src/storage/snapshot_codec.rs] | Denial of service | Use serde defaults for additive fields and deterministic reopen/decode tests. [VERIFIED: packages/open-bitcoin-node/src/status.rs; packages/open-bitcoin-node/src/sync/tests.rs] |
| Renderer-specific interpretation hides stale or no-progress state. [VERIFIED: packages/open-bitcoin-cli/src/operator/status/render.rs; packages/open-bitcoin-node/src/status.rs] | Information disclosure/Repudiation | Compute state in shared Rust status and render labels only after typed classification. [VERIFIED: .planning/phases/69-tip-tracking-and-stay-current-operation/69-CONTEXT.md] |
| Public-network timing leaks into default verification. [VERIFIED: .planning/REQUIREMENTS.md; scripts/verify.sh] | Denial of service | Keep default tests deterministic and keep public-mainnet stay-current review as opt-in UAT. [VERIFIED: .planning/phases/69-tip-tracking-and-stay-current-operation/69-CONTEXT.md; scripts/verify.sh] |

## Assumptions Log

All claims in this research are verified or cited from local repo files, pinned Bright Builds standards, OWASP docs, or local source inspection; no `[ASSUMED]` claims are intentionally used. [VERIFIED: research source list below]

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| None | No assumed claims recorded. [VERIFIED: research source list below] | All sections | None from assumptions. [VERIFIED: research source list below] |

## Sources

### Primary (HIGH confidence)

- `.planning/phases/69-tip-tracking-and-stay-current-operation/69-CONTEXT.md` - Locked Phase 69 decisions, boundaries, and implementation surfaces. [VERIFIED: local file read]
- `.planning/REQUIREMENTS.md` - TIP-01, TIP-02, TIP-03 and v1.6 out-of-scope/default verification boundaries. [VERIFIED: local file read]
- `.planning/ROADMAP.md` - Phase 69 scope and Phase 70/72/73/74 boundaries. [VERIFIED: local file read]
- `.planning/STATE.md` - Current milestone state and deterministic verification decisions. [VERIFIED: local file read]
- `AGENTS.md`, `AGENTS.bright-builds.md`, `standards-overrides.md` - Repo workflow, Rust, verification, parity breadcrumb, and override guidance. [VERIFIED: local file read]
- `packages/open-bitcoin-node/src/status.rs` - Shared status DTOs and `FieldAvailability` pattern. [VERIFIED: local source read]
- `packages/open-bitcoin-node/src/sync.rs`, `sync/runtime_state.rs`, `sync/types.rs`, `sync/types/summary.rs`, `sync/progress.rs`, `sync/block_reconcile.rs`, `sync/block_response.rs`, `sync/tests.rs` - Runtime loop, summary/status projection, peer outcomes, block reconciliation, and deterministic fixtures. [VERIFIED: local source read]
- `packages/open-bitcoin-network/src/header_store.rs`, `peer.rs`, and `packages/open-bitcoin-node/src/network/header_sync.rs` - Header validation and deterministic best-tip selection. [VERIFIED: local source read]
- `packages/open-bitcoin-node/src/storage.rs`, `storage/fjall_store.rs`, `storage/snapshot_codec.rs` - Runtime metadata persistence and versioned DTO behavior. [VERIFIED: local source read]
- `packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs`, `context.rs`, `method/node.rs` - Daemon loop and RPC sync control/status shapes. [VERIFIED: local source read]
- `packages/open-bitcoin-cli/src/operator/status.rs`, `operator/status/sync_state.rs`, `operator/status/render.rs`, `operator/dashboard/model.rs` - CLI/status consumption of durable sync state. [VERIFIED: local source read]
- `.planning/phases/68-full-active-chain-validation-and-durable-persistence/68-VERIFICATION.md` - Phase 68 completion evidence and residual risks. [VERIFIED: local file read]
- `packages/bitcoin-knots/src/node/blockstorage.cpp`, `net_processing.cpp`, `headerssync.cpp` - Pinned Knots best-header/work and header/block request anchors. [VERIFIED: local source read]
- `scripts/verify.sh`, `scripts/check-phase68-active-chain-persistence.ts`, `scripts/check-phase62-sync-truth-surfaces.ts` - Repo verification and checker patterns. [VERIFIED: local source read]

### Primary (CITED official/current)

- Bright Builds standards pinned at commit `05f8d7a6c9c2e157ec4f922a05273e72dab97676`: architecture, code shape, testing, verification, and Rust pages. [CITED: https://raw.githubusercontent.com/bright-builds-llc/bright-builds-rules/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/index.md]
- OWASP ASVS project and ASVS 5.0 chapter pages for security-domain mapping. [CITED: https://owasp.org/www-project-application-security-verification-standard/; CITED: https://github.com/OWASP/ASVS]

### Secondary (MEDIUM confidence)

- None needed; local source and official/pinned standards were sufficient. [VERIFIED: research process]

### Tertiary (LOW confidence)

- None. [VERIFIED: research process]

## Metadata

**Confidence breakdown:**

- Standard stack: HIGH - Phase 69 should reuse existing Rust/Bun/Bazel/Fjall/serde stack and all versions were verified locally. [VERIFIED: packages/Cargo.toml; packages/Cargo.lock; rust-toolchain.toml; .bun-version; environment audit]
- Architecture: HIGH - Target integration points are explicit in current source and Phase 69 context. [VERIFIED: packages/open-bitcoin-node/src/status.rs; packages/open-bitcoin-node/src/sync/runtime_state.rs; .planning/phases/69-tip-tracking-and-stay-current-operation/69-CONTEXT.md]
- Pitfalls: HIGH - Pitfalls are directly derived from locked Phase 69 decisions, Phase 68 verification, and current source gaps. [VERIFIED: .planning/phases/69-tip-tracking-and-stay-current-operation/69-CONTEXT.md; .planning/phases/68-full-active-chain-validation-and-durable-persistence/68-VERIFICATION.md; packages/open-bitcoin-node/src/status.rs]
- Freshness threshold value: MEDIUM - The need for a threshold is verified, but the exact default is not locked. [VERIFIED: .planning/phases/69-tip-tracking-and-stay-current-operation/69-CONTEXT.md]

**Research date:** 2026-06-11 [VERIFIED: environment current_date]
**Valid until:** 2026-07-11 for local architecture unless Phase 69 source changes first; re-check before implementation if Phase 70 or later modifies sync/status contracts. [VERIFIED: .planning/ROADMAP.md]
