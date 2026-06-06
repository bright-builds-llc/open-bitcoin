# Phase 62: Long-Run Sync Truth Surfaces - Research

**Researched:** 2026-06-06  
**Domain:** Open Bitcoin unattended sync observability and operator truth surfaces  
**Confidence:** HIGH

<user_constraints>

## User Constraints (from CONTEXT.md)

Copied verbatim from `.planning/phases/62-long-run-sync-truth-surfaces/62-CONTEXT.md`. [VERIFIED: .planning/phases/62-long-run-sync-truth-surfaces/62-CONTEXT.md]

### Locked Decisions

### Shared Truth Contract

- **D-01:** Treat the shared status snapshot and durable sync state as the
  canonical truth source for operator surfaces. Renderers, RPC warnings,
  metrics projection, structured logs, and live-smoke parsing should consume
  already-typed fields instead of independently parsing text or re-inferring
  lifecycle, progress, recovery, resource pressure, or peer health.
- **D-02:** The consistent Phase 62 field set is lifecycle, phase, configured
  targets, attempt counters, latest progress signal, last successful progress
  timestamp, latest stop reason, latest error, recovery category, recovery
  action, resource pressure, peer health, header height, downloaded block height
  and hash, connected block height and hash, and bounded message/header/block
  counters.
- **D-03:** Unavailable data must stay explicit through `FieldAvailability` or
  equivalent report-level unavailable/null summaries with reasons. Do not hide
  missing fields behind zeroes, empty strings, or renderer-local "ok" summaries.

### Bounded Metrics And Structured Logs

- **D-04:** Metrics and structured logs should keep the same progress vocabulary
  as status: `header_height`, `downloaded_block_height`,
  `connected_block_height`, peer count, progress signal, recovery category,
  stop reason, and bounded cycle summary counters where those facts exist.
- **D-05:** Long-run evidence must remain bounded by existing retention policies
  or explicit compact cycle summaries. Phase 62 should not add unbounded arrays
  of snapshots, peer outcomes, log lines, metrics samples, or raw live-smoke
  report material.
- **D-06:** Structured log records should expose compact machine-stable cycle
  facts that can be compared with status and live-smoke snapshots. Human message
  text may remain, but deterministic checks should assert the stable labels or
  fields that downstream operators rely on.

### Live-Smoke Snapshot Compactness

- **D-07:** Opt-in live-smoke reports should use the same field names and
  semantics as status for final status and bounded snapshot tables. The final
  report should preserve enough diagnosis evidence to compare before/after
  progress without embedding raw daemon tails, full endpoint tables, or
  unbounded report history.
- **D-08:** Live-smoke markdown and JSON should let an operator distinguish
  progress, waiting, retry, stop, and recovery states the same way they appear
  in status/dashboard/RPC/logs. Where TypeScript report casing differs for JSON
  ergonomics, keep a single mapping layer and deterministic fixture coverage.
- **D-09:** Public-network live-smoke and long-run checks remain opt-in UAT
  evidence. Default verification may use deterministic fixtures and generated
  sample reports, but must not make public network access part of
  `bash scripts/verify.sh`.

### Verification And Documentation

- **D-10:** Add deterministic cross-surface checks that fail when a Phase 62
  truth field exists in one surface but is missing or renamed in another.
  Prefer focused Rust tests for shared projections/renderers and Bun fixture
  checks for scripts/docs when those surfaces change.
- **D-11:** Refresh operator and architecture docs only where Phase 62 changes
  the truth contract or review workflow. Docs should keep copy-pasteable
  repo-local Cargo and Bazel commands for operator workflows and continue to
  separate deterministic verification from opt-in public-network UAT.
- **D-12:** Preserve Phase 61 recovery labels and resource-pressure fields as
  stable inputs. Phase 62 should extend cross-surface agreement around them, not
  rename the taxonomy or broaden recovery semantics.

### the agent's Discretion

- The planner may introduce a small pure projection helper or checker data set
  if it removes duplication across status, dashboard, RPC, metrics, logs, and
  live-smoke fixtures.
- The executor may keep changes in existing modules when that is the least risky
  path. If new first-party Rust files are added under `packages/open-bitcoin-*`,
  update parity breadcrumbs before committing.
- The planner may split work into several small plans by surface cluster, but
  each plan should prove agreement against the same Phase 62 field contract.

### Deferred Ideas (OUT OF SCOPE)

- Phase 63 owns launchd/systemd service supervision lifecycle behavior.
- Phase 64 owns service-supervised restart and same-datadir resume evidence.
- Phase 65 owns v1.5 support-bundle collection and operator review docs.
- Phase 66 owns the compatibility harness operator wrapper.
- Phase 67 owns v1.5 release-boundary and deterministic verification closeout.
- Production-node, inbound-serving, relay, production-funds wallet, destructive
  migration apply, hosted dashboard, GUI, packaging/distribution, and Windows
  service claims remain future milestones.

</user_constraints>

<phase_requirements>

## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| OBS-01 | Operator-facing status, dashboard, RPC sync status, metrics, structured logs, and live-smoke snapshots agree on unattended loop phase, configured targets, attempt counters, latest progress, latest stop reason, peer health, and downloaded or connected block evidence. | Use `DurableSyncState` and `OpenBitcoinStatusSnapshot` as canonical sources, add any missing typed fields additively, and verify status/dashboard/RPC/metrics/logs/live-smoke from the same contract. [VERIFIED: .planning/REQUIREMENTS.md; packages/open-bitcoin-node/src/status.rs:260; packages/open-bitcoin-node/src/status.rs:275; packages/open-bitcoin-node/src/sync/types.rs:350; packages/open-bitcoin-cli/src/operator/status/render.rs:15; packages/open-bitcoin-cli/src/operator/dashboard/model.rs:92; packages/open-bitcoin-rpc/src/dispatch/node.rs:54; scripts/run-live-mainnet-smoke.ts:222] |
| OBS-02 | Metrics and structured logs retain bounded long-run samples and cycle summaries without unbounded growth, while preserving enough evidence to diagnose progress, waiting, retry, stop, and recovery states. | Keep existing metric/log retention primitives, extend compact cycle summary facts rather than arrays, and test bounded retention with deterministic sync fixtures. [VERIFIED: .planning/REQUIREMENTS.md; packages/open-bitcoin-node/src/metrics.rs:55; packages/open-bitcoin-node/src/metrics.rs:92; packages/open-bitcoin-node/src/logging.rs:115; packages/open-bitcoin-node/src/logging/prune.rs:41; packages/open-bitcoin-node/src/sync/types/summary.rs:195; packages/open-bitcoin-node/src/sync/types/summary.rs:225] |

</phase_requirements>

## Summary

Phase 62 is a first-party truth-contract phase, not an ecosystem or dependency phase. [VERIFIED: .planning/phases/62-long-run-sync-truth-surfaces/62-CONTEXT.md; packages/Cargo.toml] The codebase already has a shared `DurableSyncState`, a shared `OpenBitcoinStatusSnapshot`, `FieldAvailability`, typed progress/recovery/resource-pressure fields, bounded metrics retention, bounded log retention, CLI status/dashboard renderers, RPC durable warnings, support allowlists, and a live-smoke JSON/Markdown report path. [VERIFIED: packages/open-bitcoin-node/src/status.rs:17; packages/open-bitcoin-node/src/status.rs:78; packages/open-bitcoin-node/src/status.rs:123; packages/open-bitcoin-node/src/status.rs:137; packages/open-bitcoin-node/src/status.rs:260; packages/open-bitcoin-node/src/status.rs:275; packages/open-bitcoin-node/src/metrics.rs:55; packages/open-bitcoin-node/src/logging.rs:115; packages/open-bitcoin-cli/src/operator/support/live_smoke.rs:8; scripts/run-live-mainnet-smoke.ts:222]

The planning-critical gap is cross-surface agreement on the complete Phase 62 field set. [VERIFIED: .planning/phases/62-long-run-sync-truth-surfaces/62-CONTEXT.md D-02,D-10; packages/open-bitcoin-cli/src/operator/runtime/support.rs:309; scripts/run-live-mainnet-smoke.ts:58; scripts/run-live-mainnet-smoke.ts:2225] Existing surfaces already agree on many facts, but configured header targets and latest stop reason are not explicit shared status fields, `open-bitcoin sync status` human output currently renders only pause/lifecycle/phase/clean-shutdown/update, and live-smoke snapshot rows currently omit progress signal, recovery category/action, resource pressure, configured targets, and per-peer attempts. [VERIFIED: packages/open-bitcoin-node/src/status.rs:137; packages/open-bitcoin-node/src/sync/types.rs:170; packages/open-bitcoin-node/src/sync/types.rs:369; packages/open-bitcoin-cli/src/operator/runtime/support.rs:309; scripts/run-live-mainnet-smoke.ts:58; scripts/run-live-mainnet-smoke.ts:2225]

**Primary recommendation:** Add a small pure Phase 62 truth contract/projection in Rust near the status or sync summary types, extend `SyncStatus` additively for missing contract fields, update each renderer/report from that typed contract, and guard the field list with deterministic Rust and Bun checks. [VERIFIED: .planning/phases/62-long-run-sync-truth-surfaces/62-CONTEXT.md D-01,D-02,D-10; Bright Builds architecture standard; packages/open-bitcoin-node/src/status.rs:137; packages/open-bitcoin-node/src/sync/types/summary.rs:27]

## Project Constraints (from AGENTS.md)

- Prefer repo-local `AGENTS.md`; it is present and loaded. [VERIFIED: AGENTS.md]
- Read `AGENTS.bright-builds.md`, `standards-overrides.md`, and relevant canonical standards pages before planning or implementation. [VERIFIED: AGENTS.md; AGENTS.bright-builds.md; standards-overrides.md; /Users/peterryszkiewicz/Repos/coding-and-architecture-requirements/standards/index.md]
- `standards-overrides.md` contains only placeholder override rows and no active repo-specific exception. [VERIFIED: standards-overrides.md]
- Use `git submodule update --init --recursive` when the pinned Knots baseline needs materialization; the referenced Knots source files are present in this checkout. [VERIFIED: AGENTS.md; command: `test -f packages/bitcoin-knots/src/net_processing.cpp`]
- Use `rust-toolchain.toml` as the Rust source of truth; it pins Rust `1.94.1` with `clippy` and `rustfmt`. [VERIFIED: AGENTS.md; rust-toolchain.toml]
- Use `bash scripts/verify.sh` as the repo-native verification contract for first-party code, including Cargo checks, TypeScript checkers, Bazel smoke builds, benchmark smoke checks, parity breadcrumbs, panic-site checks, and coverage. [VERIFIED: AGENTS.md; scripts/verify.sh]
- Provide copy-pasteable repo-local Cargo and Bazel commands for UAT/operator workflows rather than only naming an installed `open-bitcoin` alias. [VERIFIED: AGENTS.md; .codex/tasks/lessons.md]
- Use Bun as the canonical runtime for repo-owned higher-level automation scripts; prefer TypeScript for substantial script logic and Bash for thin wrappers. [VERIFIED: AGENTS.md; .bun-version; scripts/verify.sh]
- Treat `docs/metrics/lines-of-code.md` as intentionally tracked generated output that may change when verification regenerates it. [VERIFIED: AGENTS.md; scripts/verify.sh]
- Record intentional in-scope behavior differences from Bitcoin Knots in `docs/parity/index.json` and companion docs. [VERIFIED: AGENTS.md; docs/parity/index.json]
- When adding first-party Rust source or tests under `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests`, update `docs/parity/source-breadcrumbs.json` through `scripts/check-parity-breadcrumbs.ts`; use `none` only when no defensible Knots anchor exists. [VERIFIED: AGENTS.md; scripts/verify.sh]
- Preserve externally observable Bitcoin Knots `29.3.knots20260210` behavior for in-scope surfaces and keep parity evidence auditable. [VERIFIED: AGENTS.md; .planning/PROJECT.md; docs/parity/index.json]
- Keep functional-core/domain logic free of direct I/O and keep filesystem, process, network, terminal, RPC, service-manager, and durable-storage effects in shell adapters. [VERIFIED: AGENTS.md; Bright Builds architecture standard]
- Do not use existing Rust Bitcoin libraries in the production path; Open Bitcoin owns its own domain model and implementation surface. [VERIFIED: AGENTS.md; .planning/PROJECT.md]
- Rust code style includes `foo.rs` plus `foo/` over new `mod.rs`, no `unwrap()`, `let...else` for guard extraction, `maybe_` prefixes for `Option`, `thiserror` for library errors, `anyhow` for application errors, and `tracing` instead of `println!`. [VERIFIED: AGENTS.md; Bright Builds Rust standard]
- Unit tests should cover one concern, use Arrange/Act/Assert comments when non-trivial, and verify behavior instead of implementation details. [VERIFIED: AGENTS.md; Bright Builds testing standard]
- No project skills were found under `.claude/skills` or `.agents/skills`. [VERIFIED: AGENTS.md; command: `ls -la .claude .agents`]

## Standard Stack

### Core

| Library / Surface | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| Rust toolchain | 1.94.1 | First-party implementation for sync runtime, status contracts, RPC dispatch, CLI renderers, metrics, and logs. | The repo pins Rust `1.94.1`, and installed `rustc`/`cargo` report `1.94.1`. [VERIFIED: rust-toolchain.toml; command: `rustc --version`; command: `cargo --version`] |
| `open-bitcoin-node` | 0.1.0 workspace | Owns `DurableSyncState`, `SyncStatus`, `SyncProgress`, metrics, logging, storage metadata, and sync summaries. | Phase 62 canonical facts live in node-owned status/sync/runtime projections. [VERIFIED: packages/Cargo.toml; packages/open-bitcoin-node/src/status.rs:137; packages/open-bitcoin-node/src/status.rs:260; packages/open-bitcoin-node/src/sync/types/summary.rs:27; packages/open-bitcoin-node/src/sync/runtime_state.rs:338] |
| `open-bitcoin-cli` | 0.1.0 workspace | Owns human/JSON status, dashboard model, focused sync status, support rendering, and support live-smoke summaries. | The operator surfaces named by OBS-01 already live in this crate. [VERIFIED: packages/Cargo.toml; packages/open-bitcoin-cli/src/operator/status/render.rs:15; packages/open-bitcoin-cli/src/operator/dashboard/model.rs:92; packages/open-bitcoin-cli/src/operator/runtime/support.rs:309; packages/open-bitcoin-cli/src/operator/support/live_smoke.rs:8] |
| `open-bitcoin-rpc` | 0.1.0 workspace | Owns `getblockchaininfo`, Open Bitcoin sync control RPC, config loading, and `open-bitcoind` integration. | RPC-facing compact and full sync metadata surfaces live here. [VERIFIED: packages/Cargo.toml; packages/open-bitcoin-rpc/src/dispatch/node.rs:54; packages/open-bitcoin-rpc/src/dispatch/node.rs:158; packages/open-bitcoin-rpc/src/method/node.rs:84] |
| Bun | 1.3.9 | Runs repo-owned TypeScript checkers and live-smoke fixture tests. | The repo pins Bun `1.3.9`, installed Bun reports `1.3.9`, and `scripts/verify.sh` requires `bun`. [VERIFIED: .bun-version; command: `bun --version`; scripts/verify.sh] |
| Bazel / Bzlmod | Bazel 8.6.0, `rules_rust` 0.69.0 | Top-level smoke build and repo-local UAT command equivalent. | `MODULE.bazel` pins `rules_rust` and Rust `1.94.1`, and `scripts/verify.sh` builds Bazel targets. [VERIFIED: MODULE.bazel; command: `bazel --version`; scripts/verify.sh] |

### Supporting

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| `serde` / `serde_json` | 1.0.228 / 1.0.149 | Stable JSON contracts for status, runtime metadata, RPC responses, metrics, logs, and support summaries. | Use serde-derived additive fields and snake_case enums for Rust surfaces. [VERIFIED: packages/open-bitcoin-node/Cargo.toml; packages/open-bitcoin-rpc/Cargo.toml; packages/open-bitcoin-cli/Cargo.toml; packages/open-bitcoin-node/src/status.rs:94] |
| `fjall` | 3.1.4 | Durable runtime metadata and metric snapshot storage. | Use existing store APIs for persisted sync status and metric retention; do not add a new datastore. [VERIFIED: packages/open-bitcoin-node/Cargo.toml; packages/open-bitcoin-node/src/storage.rs:114; packages/open-bitcoin-node/src/storage/fjall_store.rs:312] |
| `tokio` | 1.52.1 | RPC/daemon async runtime. | Phase 62 should avoid new async logic unless existing RPC/daemon tests require it. [VERIFIED: packages/open-bitcoin-rpc/Cargo.toml; Bright Builds architecture standard] |
| `axum` | 0.8.9 | HTTP JSON-RPC server. | No new Axum dependency or routing work is expected; use existing RPC response types. [VERIFIED: packages/open-bitcoin-rpc/Cargo.toml; packages/open-bitcoin-rpc/src/method/node.rs:84] |
| `clap` | 4.6.1 | CLI parsing. | Use existing command surfaces; this phase should not add flags unless docs/review needs force it. [VERIFIED: packages/open-bitcoin-cli/Cargo.toml; .planning/phases/62-long-run-sync-truth-surfaces/62-CONTEXT.md D-11] |
| `ratatui` / `crossterm` | 0.30.0 / 0.29.0 | Dashboard and terminal UI. | Keep dashboard changes in pure `dashboard/model.rs` and existing tests where possible. [VERIFIED: packages/open-bitcoin-cli/Cargo.toml; packages/open-bitcoin-cli/src/operator/dashboard/model.rs:92] |
| `jsonc-parser` | 0.32.3 | Open Bitcoin JSONC config parsing. | Use existing config fields for configured targets; do not add config schema unless the truth contract truly lacks a source field. [VERIFIED: packages/open-bitcoin-rpc/Cargo.toml; packages/open-bitcoin-rpc/src/config/open_bitcoin.rs:132; packages/open-bitcoin-rpc/src/config/loader/open_bitcoin_runtime.rs:122] |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Existing first-party status/sync projection code | A new observability or metrics framework | Not recommended because Phase 62 is about cross-surface agreement over existing first-party contracts, and repo guidance minimizes dependencies. [VERIFIED: .planning/phases/62-long-run-sync-truth-surfaces/62-CONTEXT.md D-01; AGENTS.md] |
| Additive typed status fields | Renderer-local string parsing or regex extraction from logs/status text | Not allowed by D-01 and D-03 because renderers should consume typed fields and unavailable data must remain explicit. [VERIFIED: .planning/phases/62-long-run-sync-truth-surfaces/62-CONTEXT.md D-01,D-03] |
| Deterministic fixtures and generated reports | Public-network long-run default verification | Not allowed because public-network live-smoke and long-run checks remain opt-in UAT and must not enter `bash scripts/verify.sh`. [VERIFIED: .planning/phases/62-long-run-sync-truth-surfaces/62-CONTEXT.md D-09; scripts/verify.sh] |

**Installation:**

No new dependency installation is recommended for Phase 62. [VERIFIED: packages/Cargo.toml; package manifests inspected]

```bash
bash scripts/verify.sh
```

**Version verification:** Recommended versions were verified from local pinned toolchain/manifests and installed commands rather than `npm view`, because this repo has no `package.json` and Phase 62 adds no npm package. [VERIFIED: rust-toolchain.toml; .bun-version; packages/Cargo.toml; MODULE.bazel; AGENTS.md]

## Architecture Patterns

### Recommended Project Structure

Keep the first implementation pass in existing files unless the new pure helper needs a small child module; add parity breadcrumbs if a new Rust file is created. [VERIFIED: .planning/phases/62-long-run-sync-truth-surfaces/62-CONTEXT.md; AGENTS.md]

```text
packages/open-bitcoin-node/src/
├── status.rs                    # Shared additive truth fields and availability wrappers.
├── sync/types.rs                # Existing config targets, attempt counters, stop reasons.
├── sync/types/projection.rs     # Existing phase-name and peer/status projection helpers.
├── sync/types/summary.rs        # Summary -> status, metrics, structured-log projections.
├── sync/runtime_state.rs        # Durable status projection from runtime config/store/summary.
└── sync/tests.rs                # Deterministic cross-surface and bounded-retention tests.

packages/open-bitcoin-cli/src/operator/
├── status/render.rs             # Human/JSON status from OpenBitcoinStatusSnapshot.
├── dashboard/model.rs           # Pure dashboard row and chart model.
├── runtime/support.rs           # Focused open-bitcoin sync status output.
├── support/live_smoke.rs        # Allowlisted compact live-smoke summary.
└── support/render.rs            # Support markdown projection.

packages/open-bitcoin-rpc/src/
├── dispatch/node.rs             # getblockchaininfo warnings and sync control response.
└── method/node.rs               # OpenBitcoinSyncControlResponse full metadata contract.

scripts/
├── run-live-mainnet-smoke.ts       # Opt-in report JSON/Markdown and mapping layer.
├── test-run-live-mainnet-smoke.sh  # Deterministic fixture checks.
└── check-phase62-sync-truth-surfaces.ts # Recommended deterministic field-contract checker.
```

### Pattern 1: Add a Pure Phase 62 Truth Field Contract

**What:** Define the Phase 62 field list once as typed Rust fields plus a deterministic checker data set so every surface can be compared against the same contract. [VERIFIED: .planning/phases/62-long-run-sync-truth-surfaces/62-CONTEXT.md D-02,D-10; packages/open-bitcoin-node/src/status.rs:137]

**When to use:** Use it when status, dashboard, RPC, metrics, logs, support, and live-smoke need identical labels/semantics but different render formats. [VERIFIED: .planning/phases/62-long-run-sync-truth-surfaces/62-CONTEXT.md D-01,D-07,D-08; docs/architecture/status-snapshot.md:1]

**Example:**

```rust
// Source: local pattern from FieldAvailability and additive status contracts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyncConfiguredTargets {
    pub target_outbound_peers: u32,
    pub maybe_target_header_height: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyncAttemptSummary {
    pub attempted_peers: u32,
    pub connected_peers: u32,
    pub failed_peers: u32,
}
```

### Pattern 2: Prefer Additive `FieldAvailability` Over Silent Defaults

**What:** Add missing report fields as `FieldAvailability<T>` or explicit `null`/reason summaries rather than using `0`, `""`, or renderer-local `"ok"`. [VERIFIED: .planning/phases/62-long-run-sync-truth-surfaces/62-CONTEXT.md D-03; packages/open-bitcoin-node/src/status.rs:17]

**When to use:** Use it for configured targets, latest stop reason, and other fields that may be unknown from older durable metadata or stopped-node snapshots. [VERIFIED: packages/open-bitcoin-node/src/storage.rs:114; packages/open-bitcoin-node/src/status.rs:137; packages/open-bitcoin-node/src/sync/types.rs:170; packages/open-bitcoin-node/src/sync/types.rs:369]

**Example:**

```rust
// Source: existing FieldAvailability pattern in status.rs.
pub struct SyncStatus {
    pub phase: FieldAvailability<String>,
    pub progress_signal: FieldAvailability<SyncProgressSignal>,
    pub last_successful_progress_unix_seconds: FieldAvailability<u64>,
    pub recovery_category: FieldAvailability<SyncRecoveryCategory>,
}
```

### Pattern 3: Project Once, Render Many

**What:** Keep status-field derivation in `open-bitcoin-node`; CLI, dashboard, RPC, support, and scripts should render already-typed values. [VERIFIED: .planning/phases/62-long-run-sync-truth-surfaces/62-CONTEXT.md D-01; AGENTS.md; Bright Builds architecture standard]

**When to use:** Use this for lifecycle, phase, progress signal, stop reason, recovery category/action, resource pressure, peer health, heights, hashes, and counters. [VERIFIED: .planning/phases/62-long-run-sync-truth-surfaces/62-CONTEXT.md D-02; packages/open-bitcoin-node/src/sync/runtime_state.rs:338]

**Example:**

```rust
// Source: existing durable projection pattern in runtime_state.rs.
sync.lifecycle = FieldAvailability::available(lifecycle);
sync.phase = FieldAvailability::available(match lifecycle {
    SyncLifecycleState::Paused => "paused".to_string(),
    SyncLifecycleState::Recovering => "recovering".to_string(),
    SyncLifecycleState::Failed => "failed".to_string(),
    SyncLifecycleState::Stopped => "stopped".to_string(),
    SyncLifecycleState::Active => match &sync.phase {
        FieldAvailability::Available(value) => value.clone(),
        FieldAvailability::Unavailable { .. } => "steady_state".to_string(),
    },
});
```

### Pattern 4: Keep Metrics and Logs Bounded by Existing Policies

**What:** Extend metric/log facts with compact labels/counters, not retained arrays. [VERIFIED: .planning/phases/62-long-run-sync-truth-surfaces/62-CONTEXT.md D-04,D-05,D-06; packages/open-bitcoin-node/src/metrics.rs:55; packages/open-bitcoin-node/src/logging.rs:115]

**When to use:** Use this for progress signal, stop reason, recovery category, peer count, and cycle counters; store numeric metrics through `MetricSample` and compact log facts through summary records. [VERIFIED: packages/open-bitcoin-node/src/sync/types/summary.rs:195; packages/open-bitcoin-node/src/sync/types/summary.rs:225]

**Example:**

```rust
// Source: existing SyncRunSummary metric/log projection.
MetricSample::new(MetricKind::DownloadedBlockHeight, self.downloaded_block_height as f64, timestamp_unix_seconds);
MetricSample::new(MetricKind::ConnectedBlockHeight, self.best_block_height as f64, timestamp_unix_seconds);
```

### Anti-Patterns to Avoid

- **Renderer-local inference:** Do not parse human status text, log messages, or live-smoke Markdown to infer lifecycle/progress/recovery fields. [VERIFIED: .planning/phases/62-long-run-sync-truth-surfaces/62-CONTEXT.md D-01]
- **Zero means unavailable:** Do not use `0` heights, empty hashes, or empty strings to hide missing durable state when `FieldAvailability` or report-level `null` can preserve the reason. [VERIFIED: .planning/phases/62-long-run-sync-truth-surfaces/62-CONTEXT.md D-03; packages/open-bitcoin-node/src/status.rs:17]
- **Downloaded equals connected:** Do not treat downloaded block height/hash as connected chainstate evidence; existing RPC tests assert `getblockchaininfo.blocks` uses connected height. [VERIFIED: packages/open-bitcoin-rpc/src/dispatch/tests.rs:552; packages/open-bitcoin-rpc/src/dispatch/tests.rs:640]
- **Public network in default verification:** Do not add live-mainnet long-run checks to `bash scripts/verify.sh`. [VERIFIED: .planning/phases/62-long-run-sync-truth-surfaces/62-CONTEXT.md D-09; scripts/verify.sh]
- **Unbounded evidence arrays:** Do not retain raw snapshot history, endpoint tables, daemon tails, or log lines as Phase 62 evidence. [VERIFIED: .planning/phases/62-long-run-sync-truth-surfaces/62-CONTEXT.md D-05,D-07; packages/open-bitcoin-cli/src/operator/support/live_smoke.rs:8]

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Missing-field semantics | Ad hoc `0`, empty-string, or `"ok"` sentinel handling in each renderer | `FieldAvailability<T>` for Rust status and explicit `null`/summary reasons in TypeScript reports | The status contract already serializes available/unavailable states and Phase 62 requires explicit absence. [VERIFIED: packages/open-bitcoin-node/src/status.rs:17; .planning/phases/62-long-run-sync-truth-surfaces/62-CONTEXT.md D-03] |
| Metrics retention | A new long-run sample store or vector of every cycle | `MetricRetentionPolicy` plus `append_and_prune_metric_samples` | Existing metrics retention caps by interval, age, and samples per series. [VERIFIED: packages/open-bitcoin-node/src/metrics.rs:55; packages/open-bitcoin-node/src/metrics.rs:92] |
| Structured-log retention | Manual file pruning in Phase 62 code | `LogRetentionPolicy`, `plan_log_retention`, and `append_structured_log_record` | Existing log retention caps by files, age, and bytes. [VERIFIED: packages/open-bitcoin-node/src/logging.rs:115; packages/open-bitcoin-node/src/logging/prune.rs:41; packages/open-bitcoin-node/src/logging/writer.rs:18] |
| Recovery taxonomy | New labels or human-message parsing | Phase 61 `SyncRecoveryCategory` and `SyncRecoveryCategory::as_str()` labels | Phase 62 must preserve Phase 61 labels and resource-pressure fields. [VERIFIED: .planning/phases/62-long-run-sync-truth-surfaces/62-CONTEXT.md D-12; packages/open-bitcoin-node/src/status/recovery.rs] |
| Live-smoke JSON casing | Multiple mapper functions per field | A single snake_case status to camelCase report mapping layer | Phase 62 explicitly allows casing differences only behind one deterministic mapping layer. [VERIFIED: .planning/phases/62-long-run-sync-truth-surfaces/62-CONTEXT.md D-08; scripts/run-live-mainnet-smoke.ts:1255] |
| Cross-surface verification | Manual reviewer eyeballing across output samples | Rust projection/render tests plus a Bun field-contract checker | Phase 62 requires deterministic failures when a truth field is missing or renamed in one surface. [VERIFIED: .planning/phases/62-long-run-sync-truth-surfaces/62-CONTEXT.md D-10; scripts/check-phase61-resource-recovery-boundaries.ts] |

**Key insight:** The hard part is not collecting new data; it is preventing existing typed data from diverging as it moves through status, dashboard, RPC, metrics, logs, support, and live-smoke output. [VERIFIED: .planning/phases/62-long-run-sync-truth-surfaces/62-CONTEXT.md; packages/open-bitcoin-node/src/status.rs; scripts/run-live-mainnet-smoke.ts]

## Common Pitfalls

### Pitfall 1: Mistaking the Existing Docs for an Executable Contract

**What goes wrong:** Docs say surfaces share the same truth, but a renderer can still omit or rename a Phase 62 field. [VERIFIED: docs/architecture/status-snapshot.md:1; packages/open-bitcoin-cli/src/operator/runtime/support.rs:309; scripts/run-live-mainnet-smoke.ts:2225]  
**Why it happens:** The field list currently lives in prose and scattered render tests, not in one deterministic Phase 62 field-contract checker. [VERIFIED: docs/architecture/status-snapshot.md:1; scripts/check-phase61-resource-recovery-boundaries.ts]  
**How to avoid:** Add a Phase 62 checker that asserts the exact Rust/TS/docs field vocabulary and that `scripts/verify.sh` runs only deterministic checks. [VERIFIED: .planning/phases/62-long-run-sync-truth-surfaces/62-CONTEXT.md D-10; scripts/verify.sh]  
**Warning signs:** A new truth field appears in `SyncStatus` but not in live-smoke final status, dashboard rows, RPC sync status, or docs. [VERIFIED: packages/open-bitcoin-node/src/status.rs:137; scripts/run-live-mainnet-smoke.ts:222]

### Pitfall 2: Losing Stop Reasons Outside Logs and Phase Names

**What goes wrong:** Operators can see `phase=no_progress` or a structured log stop reason, but status/live-smoke/RPC do not expose a typed `latest_stop_reason` field. [VERIFIED: packages/open-bitcoin-node/src/sync/types/projection.rs:127; packages/open-bitcoin-node/src/sync/types/summary.rs:248; packages/open-bitcoin-node/src/status.rs:137]  
**Why it happens:** `SyncStopReason` exists in `SyncRunSummary`, but `SyncStatus` does not currently have a distinct stop-reason field. [VERIFIED: packages/open-bitcoin-node/src/sync/types.rs:350; packages/open-bitcoin-node/src/sync/types.rs:369; packages/open-bitcoin-node/src/status.rs:137]  
**How to avoid:** Add an additive typed stop-reason summary to shared status or the Phase 62 truth projection, then render it consistently. [VERIFIED: .planning/phases/62-long-run-sync-truth-surfaces/62-CONTEXT.md D-02,D-10; packages/open-bitcoin-node/src/sync/types.rs:384]  
**Warning signs:** Tests only search log message text such as `sync stop reason=` instead of a typed field. [VERIFIED: packages/open-bitcoin-node/src/sync/types/summary.rs:248]

### Pitfall 3: Configured Targets Are Split Across Config and Resource Pressure

**What goes wrong:** `target_outbound_peers` is visible through resource pressure, but `maybe_target_header_height` is only in runtime config and stop-reason messages. [VERIFIED: packages/open-bitcoin-node/src/status.rs:123; packages/open-bitcoin-node/src/sync/types.rs:170; packages/open-bitcoin-node/src/sync/types.rs:398]  
**Why it happens:** `SyncRuntimeConfig` owns configured targets, while `SyncStatus` currently owns resource pressure and progress fields. [VERIFIED: packages/open-bitcoin-node/src/sync/types.rs:170; packages/open-bitcoin-node/src/status.rs:137]  
**How to avoid:** Add a compact `configured_targets` status/projection field or explicitly document/test equivalent typed fields. [VERIFIED: .planning/phases/62-long-run-sync-truth-surfaces/62-CONTEXT.md D-02,D-11]  
**Warning signs:** Live-smoke final status can show `targetOutboundPeers` but cannot show the configured header target. [VERIFIED: scripts/run-live-mainnet-smoke.ts:196; scripts/run-live-mainnet-smoke.ts:1320]

### Pitfall 4: Compact Live-Smoke Output Falls Behind Shared Status

**What goes wrong:** Live-smoke snapshots stay compact but omit fields needed to diagnose progress/waiting/retry/stop/recovery the same way as status/dashboard/RPC/logs. [VERIFIED: .planning/phases/62-long-run-sync-truth-surfaces/62-CONTEXT.md D-08; scripts/run-live-mainnet-smoke.ts:58; scripts/run-live-mainnet-smoke.ts:2225]  
**Why it happens:** The current snapshot table was designed around lifecycle, phase, heights, hashes, outbound peers, and last error. [VERIFIED: scripts/run-live-mainnet-smoke.ts:58; scripts/run-live-mainnet-smoke.ts:2232]  
**How to avoid:** Add only bounded Phase 62 columns/summary fields and fixture checks for both JSON and Markdown. [VERIFIED: .planning/phases/62-long-run-sync-truth-surfaces/62-CONTEXT.md D-07,D-08,D-10; scripts/test-run-live-mainnet-smoke.sh]  
**Warning signs:** JSON fixture checks pass while Markdown lacks the same recovery/progress field names. [VERIFIED: scripts/test-run-live-mainnet-smoke.sh; scripts/run-live-mainnet-smoke.ts:2394]

### Pitfall 5: Extending Metrics With Non-Numeric or Unbounded State

**What goes wrong:** Metrics become a history of object snapshots rather than bounded numeric series. [VERIFIED: packages/open-bitcoin-node/src/metrics.rs:76; .planning/phases/62-long-run-sync-truth-surfaces/62-CONTEXT.md D-05]  
**Why it happens:** OBS-02 asks for enough diagnosis evidence, but metrics are not the right surface for arbitrary structured state. [VERIFIED: .planning/REQUIREMENTS.md; packages/open-bitcoin-node/src/metrics.rs:12]  
**How to avoid:** Keep numeric series for heights/peer count and put stop/recovery/cycle labels in compact structured logs and status snapshots. [VERIFIED: packages/open-bitcoin-node/src/sync/types/summary.rs:195; packages/open-bitcoin-node/src/sync/types/summary.rs:225]  
**Warning signs:** A plan proposes `Vec<SyncStatus>` or `Vec<PeerTelemetry>` inside metrics status. [VERIFIED: packages/open-bitcoin-node/src/metrics.rs:150]

### Pitfall 6: Forgetting Repo-Local Parity and Verification Rules

**What goes wrong:** A new Rust module or test file lands without parity breadcrumbs, or docs give only the installed `open-bitcoin` alias. [VERIFIED: AGENTS.md; .codex/tasks/lessons.md]  
**Why it happens:** Phase 62 touches Rust, TypeScript, docs, and operator commands. [VERIFIED: .planning/phases/62-long-run-sync-truth-surfaces/62-CONTEXT.md canonical refs]  
**How to avoid:** Update breadcrumbs when adding Rust source/tests, use repo-local Cargo/Bazel commands in docs, and run the repo-native verification contract. [VERIFIED: AGENTS.md; scripts/verify.sh]  
**Warning signs:** `scripts/check-parity-breadcrumbs.ts --check` fails or UAT docs show only `open-bitcoin ...`. [VERIFIED: scripts/verify.sh; .codex/tasks/lessons.md]

## Code Examples

Verified local patterns from the current codebase:

### Explicit Field Availability

```rust
// Source: packages/open-bitcoin-node/src/status.rs
#[serde(rename_all = "snake_case", tag = "state", content = "value")]
pub enum FieldAvailability<T> {
    Available(T),
    Unavailable { reason: String },
}
```

Use this pattern for any additive status field whose value can be unknown in stopped-node or older-metadata paths. [VERIFIED: packages/open-bitcoin-node/src/status.rs:17; .planning/phases/62-long-run-sync-truth-surfaces/62-CONTEXT.md D-03]

### Summary to Status, Metrics, and Structured Logs

```rust
// Source: packages/open-bitcoin-node/src/sync/types/summary.rs
pub fn metric_samples(&self, timestamp_unix_seconds: u64) -> Vec<MetricSample> {
    vec![
        MetricSample::new(MetricKind::HeaderHeight, self.best_header_height as f64, timestamp_unix_seconds),
        MetricSample::new(MetricKind::DownloadedBlockHeight, self.downloaded_block_height as f64, timestamp_unix_seconds),
        MetricSample::new(MetricKind::ConnectedBlockHeight, self.best_block_height as f64, timestamp_unix_seconds),
        MetricSample::new(MetricKind::SyncHeight, self.best_block_height as f64, timestamp_unix_seconds),
        MetricSample::new(MetricKind::PeerCount, self.connected_peers as f64, timestamp_unix_seconds),
    ]
}
```

Use this projection point to keep metrics numeric and status/log vocabulary aligned. [VERIFIED: packages/open-bitcoin-node/src/sync/types/summary.rs:195; docs/architecture/operator-observability.md:26]

### Durable Runtime Projection

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

Use this point to add configured target and stop-reason projections because it has access to runtime config, durable storage, and `SyncRunSummary`. [VERIFIED: packages/open-bitcoin-node/src/sync/runtime_state.rs:338; packages/open-bitcoin-node/src/sync/runtime_state.rs:408]

### Live-Smoke Single Mapping Layer

```ts
// Source: scripts/run-live-mainnet-smoke.ts
function resourcePressureSummaryFromValue(
  value: ResourcePressureStatusJson | null,
): ResourcePressureSummary | null {
  if (value === null) {
    return null;
  }
  return {
    blocksInFlight: Number(value.blocks_in_flight ?? 0),
    maxHeaderRequestsInFlightPerPeer: Number(value.max_header_requests_in_flight_per_peer ?? 0),
    targetOutboundPeers: Number(value.target_outbound_peers ?? 0),
  };
}
```

Use this pattern for any new snake_case Rust status field that needs camelCase live-smoke JSON. [VERIFIED: scripts/run-live-mainnet-smoke.ts:1320; .planning/phases/62-long-run-sync-truth-surfaces/62-CONTEXT.md D-08]

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Renderer-local status summaries | Shared `OpenBitcoinStatusSnapshot` and `DurableSyncState` as the canonical truth source | Existing by v1.4 and locked for Phase 62 | Plans should add typed fields once and render them many times. [VERIFIED: docs/architecture/status-snapshot.md:1; packages/open-bitcoin-node/src/status.rs:260; .planning/phases/62-long-run-sync-truth-surfaces/62-CONTEXT.md D-01] |
| Single block height as sync progress | Separate header, downloaded block, connected block, and compatibility `block_height` | Existing before Phase 62 | Plans must not collapse downloaded and connected evidence. [VERIFIED: packages/open-bitcoin-node/src/status.rs:78; docs/architecture/status-snapshot.md:57] |
| Human recovery text only | Phase 61 stable `sync.recovery_category` plus human `sync.recovery_action` | Phase 61 completed 2026-06-06 | Phase 62 should preserve labels and expand consistency, not rename taxonomy. [VERIFIED: .planning/STATE.md; .planning/phases/61-resource-bounds-and-recovery-taxonomy/61-VERIFICATION.md; .planning/phases/62-long-run-sync-truth-surfaces/62-CONTEXT.md D-12] |
| Public-network evidence as possible proof source | Deterministic default verification plus opt-in live-smoke UAT | v1.5 decision and Phase 62 lock | Plans must use fixtures/checkers by default and keep public-network runs optional. [VERIFIED: .planning/STATE.md; .planning/phases/62-long-run-sync-truth-surfaces/62-CONTEXT.md D-09; scripts/verify.sh] |
| Unbounded long-run evidence risk | Bounded metric samples, bounded structured log files, and allowlisted support/live-smoke summaries | Existing metrics/log/support infrastructure | OBS-02 should extend compact facts without new retained arrays. [VERIFIED: packages/open-bitcoin-node/src/metrics.rs:55; packages/open-bitcoin-node/src/logging.rs:115; packages/open-bitcoin-cli/src/operator/support/live_smoke.rs:8] |

**Deprecated/outdated:**

- Treating `sync_progress.block_height` as the only block truth is outdated because it is now a compatibility alias for connected chainstate height. [VERIFIED: docs/architecture/status-snapshot.md:57; packages/open-bitcoin-node/src/status.rs:78]
- Treating live-smoke raw reports as support evidence is out of scope; support summaries are allowlisted and compact. [VERIFIED: .planning/phases/62-long-run-sync-truth-surfaces/62-CONTEXT.md D-07; packages/open-bitcoin-cli/src/operator/support/live_smoke.rs:8]
- Treating `bash scripts/verify.sh` as a public-network gate is forbidden for v1.5. [VERIFIED: .planning/REQUIREMENTS.md; scripts/verify.sh]

## Assumptions Log

All claims in this research were verified or cited; no user confirmation is needed for unverified assumed claims. [VERIFIED: this file review]

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| - | None | - | - |

## Open Questions (RESOLVED)

1. **Exact Rust module placement for the Phase 62 helper**
   - What we know: existing status/sync files already own the relevant domain types and projections. [VERIFIED: packages/open-bitcoin-node/src/status.rs; packages/open-bitcoin-node/src/sync/types/summary.rs; packages/open-bitcoin-node/src/sync/runtime_state.rs]
   - RESOLVED: Plan 62-01 keeps the Phase 62 status helper types in `packages/open-bitcoin-node/src/status.rs` and projection wiring in existing sync summary/runtime modules. Do not create a new Rust module for the initial implementation; if execution proves the helper too large, the executor may split it only with a same-plan `docs/parity/source-breadcrumbs.json` update and `scripts/check-parity-breadcrumbs.ts` verification. [VERIFIED: .planning/phases/62-long-run-sync-truth-surfaces/62-01-PLAN.md; AGENTS.md]

2. **Whether configured targets should be one struct or separate fields**
   - What we know: current config has `target_outbound_peers` and optional `target_header_height`, while status exposes outbound target through `SyncResourcePressure` and does not expose target header as a status field. [VERIFIED: packages/open-bitcoin-node/src/sync/types.rs:170; packages/open-bitcoin-node/src/status.rs:123; packages/open-bitcoin-node/src/status.rs:137]
   - RESOLVED: Use one additive `SyncConfiguredTargets` struct on `SyncStatus` with `target_outbound_peers` and `maybe_target_header_height`, surfaced as `FieldAvailability<SyncConfiguredTargets>`. Plan 62-01 also requires the same `maybe_target_header_height` value to reach structured-log projection so logs and durable status cannot diverge. [VERIFIED: .planning/phases/62-long-run-sync-truth-surfaces/62-01-PLAN.md]

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| Rust `rustc` | Rust implementation and tests | yes | 1.94.1 | None needed. [VERIFIED: command: `rustc --version`] |
| Cargo | Workspace build/test/check commands | yes | 1.94.1 | None needed. [VERIFIED: command: `cargo --version`] |
| rustfmt | Repo formatting | yes | 1.8.0-stable | None needed. [VERIFIED: command: `rustfmt --version`] |
| Clippy | Repo linting | yes | 0.1.94 | None needed. [VERIFIED: command: `cargo clippy --version`] |
| Bun | TypeScript checkers and live-smoke fixtures | yes | 1.3.9 | None needed. [VERIFIED: command: `bun --version`; .bun-version] |
| Bazel | Top-level smoke build | yes | 8.6.0 | None needed. [VERIFIED: command: `bazel --version`; MODULE.bazel] |
| cargo-llvm-cov | Repo coverage gate | yes | 0.8.5 | None needed. [VERIFIED: command: `cargo llvm-cov --version`; scripts/verify.sh] |
| Git | Commit and repo checks | yes | 2.53.0 | None needed. [VERIFIED: command: `git --version`] |
| Bash | Repo verification and fixture scripts | yes | GNU bash 3.2.57 | None needed. [VERIFIED: command: `bash --version`; scripts/verify.sh] |
| Bitcoin Knots source anchor | Parity breadcrumb context | yes | source files present | Run `git submodule update --init --recursive` if a missing baseline file is encountered. [VERIFIED: command: `test -f packages/bitcoin-knots/src/net_processing.cpp`; AGENTS.md] |

**Missing dependencies with no fallback:** None found for Phase 62 planning. [VERIFIED: environment probes above]

**Missing dependencies with fallback:** None found for Phase 62 planning. [VERIFIED: environment probes above]

## Security Domain

Security enforcement is enabled because `.planning/config.json` does not set `security_enforcement` to `false`. [VERIFIED: .planning/config.json]

### Applicable ASVS Categories

OWASP ASVS category names V2 Authentication, V3 Session Management, V4 Access Control, V5 Validation/Sanitization/Encoding, and V6 Stored Cryptography were checked against OWASP Developer Guide ASVS material. [CITED: https://devguide.owasp.org/en/03-requirements/05-asvs/]

| ASVS Category | Applies | Standard Control |
|---------------|---------|-----------------|
| V2 Authentication | no new auth in this phase | Preserve existing local RPC credential discovery and metadata-only credential reporting; do not expose cookie contents or passwords in status/support/live-smoke evidence. [VERIFIED: docs/architecture/config-precedence.md:1; packages/open-bitcoin-cli/src/operator/support/live_smoke.rs:8] |
| V3 Session Management | no browser/session surface in this phase | No session state is introduced by Phase 62. [VERIFIED: .planning/phases/62-long-run-sync-truth-surfaces/62-CONTEXT.md deferred scope; packages/open-bitcoin-rpc/src/method/node.rs:84] |
| V4 Access Control | no new remote administration model in this phase | Keep Phase 62 to local operator truth surfaces and existing RPC sync metadata; remote hosted/public dashboard and broad ACL work remain out of scope. [VERIFIED: .planning/phases/62-long-run-sync-truth-surfaces/62-CONTEXT.md deferred scope; docs/parity/release-readiness.md] |
| V5 Validation, Sanitization, and Encoding | yes | Use serde-typed Rust contracts, explicit availability states, JSON parsing with deterministic TypeScript mapping, and redacted/allowlisted support summaries. [VERIFIED: packages/open-bitcoin-node/src/status.rs:17; scripts/run-live-mainnet-smoke.ts:1255; packages/open-bitcoin-cli/src/operator/support/live_smoke.rs:8] |
| V6 Stored Cryptography | no new crypto in this phase | Do not add cryptography; preserve existing credential redaction boundaries. [VERIFIED: .planning/phases/62-long-run-sync-truth-surfaces/62-CONTEXT.md; docs/architecture/config-precedence.md:1] |

### Known Threat Patterns for Phase 62

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Truth-surface drift causes operators to trust stale or contradictory sync state | Tampering / Repudiation | One typed status/truth contract plus deterministic cross-surface tests. [VERIFIED: .planning/phases/62-long-run-sync-truth-surfaces/62-CONTEXT.md D-01,D-10] |
| Missing data is hidden as zero or empty text | Tampering / Information disclosure by omission | Preserve `FieldAvailability` and explicit `null` summaries with reasons. [VERIFIED: .planning/phases/62-long-run-sync-truth-surfaces/62-CONTEXT.md D-03; packages/open-bitcoin-node/src/status.rs:17] |
| Live-smoke/support artifacts leak raw daemon tails, endpoint tables, or credentials | Information Disclosure | Use allowlisted compact summaries and existing redaction boundaries. [VERIFIED: .planning/phases/62-long-run-sync-truth-surfaces/62-CONTEXT.md D-05,D-07; packages/open-bitcoin-cli/src/operator/support/live_smoke.rs:8; docs/architecture/config-precedence.md:1] |
| Long-run evidence grows without bound | Denial of Service | Use existing metric/log retention and compact cycle summaries. [VERIFIED: packages/open-bitcoin-node/src/metrics.rs:55; packages/open-bitcoin-node/src/logging.rs:115; .planning/phases/62-long-run-sync-truth-surfaces/62-CONTEXT.md D-05] |
| Default verification accidentally depends on public network | Denial of Service / Repudiation | Keep `scripts/verify.sh` deterministic and guard with negative checks. [VERIFIED: .planning/phases/62-long-run-sync-truth-surfaces/62-CONTEXT.md D-09; scripts/verify.sh; scripts/test-run-live-mainnet-smoke.sh] |

## Sources

### Primary (HIGH confidence)

- `.planning/phases/62-long-run-sync-truth-surfaces/62-CONTEXT.md` - locked decisions, scope boundaries, implementation surfaces, and deferred work. [VERIFIED: file read]
- `.planning/REQUIREMENTS.md` - OBS-01 and OBS-02 requirement text and public-network default-verification exclusion. [VERIFIED: file read]
- `.planning/ROADMAP.md` - Phase 62 goal, success criteria, dependencies, and downstream phase boundaries. [VERIFIED: file read]
- `.planning/STATE.md` - v1.5 decisions, Phase 61 outcomes, and deterministic verification posture. [VERIFIED: file read]
- `AGENTS.md`, `AGENTS.bright-builds.md`, `standards-overrides.md`, and canonical Bright Builds standards under `/Users/peterryszkiewicz/Repos/coding-and-architecture-requirements/standards/` - repo workflow, architecture, verification, testing, Rust, TypeScript, and operability constraints. [VERIFIED: file reads]
- `packages/open-bitcoin-node/src/status.rs` - `FieldAvailability`, `SyncStatus`, `SyncProgress`, `SyncResourcePressure`, `PeerTelemetry`, `DurableSyncState`, and `OpenBitcoinStatusSnapshot`. [VERIFIED: code read]
- `packages/open-bitcoin-node/src/sync/types.rs`, `sync/types/projection.rs`, `sync/types/summary.rs`, and `sync/runtime_state.rs` - runtime config targets, attempt counters, stop reasons, phase names, status projection, metrics, logs, and durable projection. [VERIFIED: code read]
- `packages/open-bitcoin-node/src/metrics.rs` and `packages/open-bitcoin-node/src/logging.rs` plus logging child modules - bounded metrics and structured log retention. [VERIFIED: code read]
- `packages/open-bitcoin-cli/src/operator/status/render.rs`, `dashboard/model.rs`, `runtime/support.rs`, `support/live_smoke.rs`, and `support/render.rs` - operator surfaces and compact support summaries. [VERIFIED: code read]
- `packages/open-bitcoin-rpc/src/dispatch/node.rs` and `packages/open-bitcoin-rpc/src/method/node.rs` - RPC durable blockchain info, sync warnings, and sync-control metadata response. [VERIFIED: code read]
- `scripts/run-live-mainnet-smoke.ts` and `scripts/test-run-live-mainnet-smoke.sh` - opt-in live-smoke report shape, mapping layer, Markdown output, and deterministic fixtures. [VERIFIED: code read]
- `docs/operator/runtime-guide.md`, `docs/architecture/status-snapshot.md`, `docs/architecture/operator-observability.md`, `docs/architecture/config-precedence.md`, `docs/parity/release-readiness.md`, and `docs/parity/index.json` - operator contract, observability boundaries, config/credential boundaries, release scope, and parity roots. [VERIFIED: docs read]

### Secondary (MEDIUM confidence)

- OWASP Developer Guide ASVS page - ASVS category names used for the security-domain table. [CITED: https://devguide.owasp.org/en/03-requirements/05-asvs/]

### Tertiary (LOW confidence)

- None. [VERIFIED: this research did not rely on unverified web/forum sources]

## Metadata

**Confidence breakdown:**

- Standard stack: HIGH - all recommended tools and package versions were verified from repo files or installed commands. [VERIFIED: rust-toolchain.toml; .bun-version; packages/Cargo.toml; MODULE.bazel; environment probes]
- Architecture: HIGH - the recommendation follows locked Phase 62 decisions and existing first-party projection boundaries. [VERIFIED: .planning/phases/62-long-run-sync-truth-surfaces/62-CONTEXT.md; packages/open-bitcoin-node/src/status.rs; packages/open-bitcoin-node/src/sync/types/summary.rs]
- Pitfalls: HIGH - gaps were observed directly in current code and docs. [VERIFIED: packages/open-bitcoin-cli/src/operator/runtime/support.rs:309; scripts/run-live-mainnet-smoke.ts:2225; packages/open-bitcoin-node/src/status.rs:137]
- Security: MEDIUM - phase-specific risks were mapped from local evidence and OWASP ASVS category names, but no full threat model update was performed during research. [VERIFIED: local sources above; CITED: https://devguide.owasp.org/en/03-requirements/05-asvs/]

**Research date:** 2026-06-06  
**Valid until:** 2026-07-06 for codebase-local planning, or sooner if Phase 62 implementation changes the status contract before planning is consumed. [VERIFIED: current date; .planning/config.json]
