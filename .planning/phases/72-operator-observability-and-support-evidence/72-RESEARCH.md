# Phase 72: Operator Observability and Support Evidence - Research

**Researched:** 2026-06-13 [VERIFIED: environment current_date]
**Domain:** Rust operator observability, support evidence, deterministic verification [VERIFIED: .planning/phases/72-operator-observability-and-support-evidence/72-CONTEXT.md]
**Confidence:** HIGH [VERIFIED: local source audit]

<user_constraints>
## User Constraints (from CONTEXT.md)

Source: `.planning/phases/72-operator-observability-and-support-evidence/72-CONTEXT.md` [VERIFIED: local file]

### Locked Decisions

- D-01 Canonical truth contract is `OpenBitcoinStatusSnapshot` and nested `SyncStatus`; all CLI/dashboard/RPC Open-Bitcoin sync status/metrics/log projections/live-smoke/support bundles consume or summarize this, not reclassify.
- D-02 Preserve Phase 68 progress distinctions: header height, downloaded block height, connected block height, validated active-chain height/hash/work separate; downloaded-only/headers-only not proof.
- D-03 Preserve Phase 69 stay-current semantics: `current_at_best_known_tip` requires fresh best-known tip and connected active-chain height/hash/work matching validated tip; stale/recovering/no-progress distinct.
- D-04 Carry Phase 70/71 fields: reorg, reconcile, no-progress diagnosis/action, resource pressure, recovery category, restart/resume; fields as machine labels or `Unavailable`.
- D-05 Add missing Phase 69-71 fields to human CLI and dashboard, especially best-known tip, stay-current state/action, latest reorg, reconcile progress, no-progress diagnosis/action, resource pressure, validated active-chain work.
- D-06 RPC Open-Bitcoin sync status exposes durable sync state when available; baseline `getblockchaininfo` may stay scoped to Knots-compatible shape.
- D-07 Metrics bounded numeric samples, logs compact records, but enough shared labels/progress dimensions to correlate with status/support evidence.
- D-08 Add deterministic cross-surface comparison check for shared status JSON, CLI human, dashboard, RPC durable status, live-smoke compact summary, structured log summary, metrics projection, support evidence agreement on core fields/unavailable reasons.
- D-09 Support evidence allowlisted/redacted; may include shared snapshot and compact live-smoke summaries; must not embed raw logs, peer transcripts, cookies/passwords, config secrets, wallet material, raw reports, unbounded endpoint tables.
- D-10 Support evidence includes initial/final tip, connected active-chain height/hash/work, restart/resume checkpoints, stay-current window, peer contribution summary, no-progress/reorg events, resource pressure, recovery category/action, final verdict.
- D-11 Final verdict typed and evidence-derived. Suggested enum: `sync_to_tip_proven`, `stay_current_proven`, `diagnosed_blocker`, `inconclusive`; verdict must explain which fields justify it.
- D-12 Live-smoke ingestion summary-only; expand schema v2 allowlist where needed.
- D-13 Guidance explains what evidence proves; avoid broad production/inbound/relay/wallet/migration/packaging/Windows/GUI/hosted dashboard/drop-in readiness claims.
- D-14 Default verification remains deterministic, public-network-free, service-manager-free, timing-stable, short. Public mainnet full-sync/stay-current opt-in UAT only; not in `scripts/verify.sh`.
- D-15 Docs commands: use repo-local Cargo and Bazel UAT forms.
- D-16 Focused unit/fixture tests for changed renderer/evidence adapter; one concern/test; AAA comments when non-trivial.
- D-17 Phase 72 deterministic Bun checker, wired into `scripts/verify.sh` after Phase 71 checker, covering artifacts/source/test/docs/support allowlist/comparison/default verification boundaries.
- D-18 New first-party Rust source/test files under `packages/open-bitcoin-*/src|tests` require parity breadcrumbs in `docs/parity/source-breadcrumbs.json` and `scripts/check-parity-breadcrumbs.ts`.

### the agent's Discretion

- Split implementation across shared status/rendering alignment, support evidence schema/verdicts, live-smoke/log/metric projections, cross-surface comparison tests, docs/checker.
- Add small pure projection helpers to prevent divergence and make illegal verdict states unrepresentable.
- Keep support evidence compact/additive; old consumers decode via defaults/unavailable reasons.

### Deferred Ideas (OUT OF SCOPE)

- Phase 73 owns full UAT matrix.
- Phase 74 owns release readiness/threat model/final claim checks.
- Hosted dashboards, GUI, inbound, relay, production wallets, migration apply, signed packages, broad production-node claims are out of scope.
</user_constraints>

<phase_requirements>
## Phase Requirements

Source: `.planning/REQUIREMENTS.md` [VERIFIED: local file]

| ID | Description | Research Support |
|----|-------------|------------------|
| OBS-01 | Operator can inspect one shared full-sync truth contract through CLI status, dashboard, RPC, metrics, structured logs, live-smoke reports, and support bundles. | `OpenBitcoinStatusSnapshot` and nested `SyncStatus` already contain shared status, peer, log, metric, health, and build surfaces; Phase 72 should add projections and tests around this contract, not define a new truth model. [VERIFIED: packages/open-bitcoin-node/src/status.rs] |
| OBS-02 | Operator can generate redacted support evidence including initial/final tip, connected height/hash/work, restart/resume checkpoints, stay-current window, peer contribution, no-progress or reorg events, resource pressure, final verdict. | Support bundles already embed the shared snapshot and a summary-only live-smoke section; missing work is a compact allowlisted full-sync evidence object plus typed evidence-derived verdict. [VERIFIED: packages/open-bitcoin-cli/src/operator/support.rs] [VERIFIED: packages/open-bitcoin-cli/src/operator/support/live_smoke.rs] |
| OBS-03 | Operator can compare status surfaces and confirm they agree on connected chain progress, tip freshness, recovery category, peer health, next action. | Existing renderers and report summaries are pure enough to test from deterministic fixtures; add a cross-surface comparison fixture/checker that reads the same snapshot and verifies exact field agreement or matching unavailable reasons. [VERIFIED: packages/open-bitcoin-cli/src/operator/status/render.rs] [VERIFIED: packages/open-bitcoin-cli/src/operator/dashboard/model.rs] |
| OBS-04 | Operator can read concise guidance explaining whether evidence proves sync-to-tip, stay-current behavior, diagnosed blocker, or deferred production-node scope. | Runtime guide already documents support bundles and their limits; Phase 72 should extend that guidance with typed verdict meanings and bounded claims. [VERIFIED: docs/operator/runtime-guide.md] |
</phase_requirements>

## Summary

Phase 72 should be planned as an alignment and evidence phase, not a new sync/runtime phase. The canonical status model already exists in `open-bitcoin-node`: `OpenBitcoinStatusSnapshot` wraps `SyncStatus`, peer state, metrics, logs, health signals, and build provenance, while `DurableSyncRuntime` projects active-chain progress, best-known tip, stay-current state, reorg/reconcile, no-progress, recovery, and resource pressure into durable status. [VERIFIED: packages/open-bitcoin-node/src/status.rs] [VERIFIED: packages/open-bitcoin-node/src/sync/runtime_state.rs]

The main implementation gaps are downstream surfaces. Human CLI and dashboard output do not yet show all Phase 69-71 fields, live-smoke reports and support evidence do not yet preserve the full sync-to-tip/stay-current proof fields, and there is no deterministic cross-surface comparison that proves projections agree. [VERIFIED: packages/open-bitcoin-cli/src/operator/status/render.rs] [VERIFIED: packages/open-bitcoin-cli/src/operator/dashboard/model.rs] [VERIFIED: scripts/run-live-mainnet-smoke.ts] [VERIFIED: packages/open-bitcoin-cli/src/operator/support/render.rs]

**Primary recommendation:** Use `OpenBitcoinStatusSnapshot`/`SyncStatus` as the only truth source, add small pure projection helpers for comparable full-sync evidence and verdicts, extend renderers/report summaries additively, and lock agreement with deterministic fixtures plus a Phase 72 Bun checker wired after Phase 71. [VERIFIED: .planning/phases/72-operator-observability-and-support-evidence/72-CONTEXT.md]

## Project Constraints (from AGENTS.md)

- Use `bash scripts/verify.sh` as the repo-native verification contract before marking work complete; it includes the Bazel smoke build. [VERIFIED: AGENTS.md]
- Use Bun for repo-owned higher-level automation scripts; prefer TypeScript for substantial script logic and keep Bash as thin orchestration. [VERIFIED: AGENTS.md]
- Provide UAT docs with copy-pasteable repo-local Cargo and Bazel commands, such as `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- ...` and `bazel run //packages/open-bitcoin-cli:open_bitcoin -- ...`. [VERIFIED: AGENTS.md]
- Treat `docs/metrics/lines-of-code.md` as tracked generated output that may change when verification regenerates it. [VERIFIED: AGENTS.md]
- Record intentional in-scope behavior differences from Bitcoin Knots in `docs/parity/index.json` and companion docs under `docs/parity/`. [VERIFIED: AGENTS.md]
- New first-party Rust source or test files under `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests` require parity breadcrumbs via `docs/parity/source-breadcrumbs.json` and `scripts/check-parity-breadcrumbs.ts`. [VERIFIED: AGENTS.md]
- After substantial operator-surface or workflow changes, check relevant README files for needed updates. [VERIFIED: AGENTS.md]
- Follow Bright Builds functional core / imperative shell boundaries: parse boundary data into domain types, keep pure business logic out of effectful adapters, and make illegal states unrepresentable where feasible. [CITED: https://raw.githubusercontent.com/peterryszkiewicz/bright-builds/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/core/architecture.md]
- Keep scripts safe and rerunnable, fail fast on errors, and prefer repo-native verification over ad hoc checks. [CITED: https://raw.githubusercontent.com/peterryszkiewicz/bright-builds/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/core/verification.md]
- Unit tests should test one concern, use Arrange/Act/Assert comments when non-trivial, and prioritize pure logic over adapter-heavy tests. [CITED: https://raw.githubusercontent.com/peterryszkiewicz/bright-builds/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/core/testing.md]
- Rust code should prefer `foo.rs` plus `foo/`, `let...else`, `maybe_` names for optional values, and domain enums/newtypes over stringly values. [CITED: https://raw.githubusercontent.com/peterryszkiewicz/bright-builds/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/languages/rust.md]
- TypeScript automation should run under Bun, use explicit parsed/validated data boundaries, and favor simple data-in/data-out functions. [CITED: https://raw.githubusercontent.com/peterryszkiewicz/bright-builds/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/languages/typescript-javascript.md]
- No project-specific skills exist under `.claude/skills` or `.agents/skills`. [VERIFIED: filesystem scan]

## Standard Stack

### Core

| Library / Tool | Version | Purpose | Why Standard |
|----------------|---------|---------|--------------|
| Rust toolchain | 1.94.1 | First-party node, RPC, CLI, dashboard, support bundle code | Repo-pinned in `rust-toolchain.toml`; Cargo and Bazel both use the same Rust baseline. [VERIFIED: rust-toolchain.toml] [VERIFIED: rustc --version] |
| Rust 2024 edition | Workspace setting | Rust language edition for first-party packages | Workspace package metadata sets edition 2024. [VERIFIED: packages/Cargo.toml] |
| serde | 1.0.228 | Stable JSON/status/report data shapes | Already used by canonical status and support evidence types. [VERIFIED: packages/Cargo.lock] [VERIFIED: packages/open-bitcoin-node/src/status.rs] |
| serde_json | 1.0.149 | JSON serialization/parsing for status, support evidence, live-smoke summaries, RPC payloads | Already used across CLI support/status and scripts interop. [VERIFIED: packages/Cargo.lock] |
| Fjall | 3.1.4 | Durable runtime metadata/status storage | `FjallNodeStore` stores and loads runtime metadata used by CLI and RPC durable sync status. [VERIFIED: packages/Cargo.lock] [VERIFIED: packages/open-bitcoin-cli/src/operator/status/sync_state.rs] |
| Axum | 0.8.9 | RPC server HTTP stack | Existing Open Bitcoin RPC package depends on Axum; Phase 72 should use existing RPC methods rather than adding a second server path. [VERIFIED: packages/Cargo.lock] [VERIFIED: packages/open-bitcoin-rpc/src/context.rs] |
| Tokio | 1.52.1 | Async RPC/server runtime | Existing RPC runtime dependency. [VERIFIED: packages/Cargo.lock] |
| clap | 4.6.1 | Operator CLI parsing | Existing CLI dependency; no new CLI framework is needed. [VERIFIED: packages/Cargo.lock] |
| Ratatui | 0.30.0 | Terminal dashboard model/rendering | Existing dashboard dependency; Phase 72 should extend existing dashboard model rows. [VERIFIED: packages/Cargo.lock] [VERIFIED: packages/open-bitcoin-cli/src/operator/dashboard/model.rs] |
| Crossterm | 0.29.0 | Terminal IO for dashboard | Existing terminal dependency paired with Ratatui. [VERIFIED: packages/Cargo.lock] |
| Bun | 1.3.9 | Deterministic TypeScript checkers and live-smoke automation | Repo-pinned in `.bun-version` and used by existing Phase 68-71 checkers. [VERIFIED: .bun-version] [VERIFIED: bun --version] |
| Bazel / Bazelisk entrypoint | Bazel 8.6.0 in environment | Top-level smoke build | `scripts/verify.sh` requires `bazel`; `MODULE.bazel` pins `rules_rust` 0.69.0. [VERIFIED: bazel --version] [VERIFIED: MODULE.bazel] |

### Supporting

| Library / Tool | Version | Purpose | When to Use |
|----------------|---------|---------|-------------|
| jsonc-parser | 0.32.3 | Operator JSONC config parsing | Use only where existing config parsing needs support evidence metadata; do not parse config with ad hoc strings. [VERIFIED: packages/Cargo.lock] |
| cargo-llvm-cov | 0.8.5 | Repo coverage verification | Required by `scripts/verify.sh`; not a Phase 72 implementation dependency. [VERIFIED: cargo llvm-cov --version] [VERIFIED: scripts/verify.sh] |
| secp256k1 | 0.31.1 | Existing cryptographic primitive dependency | Do not add custom crypto for support evidence; use existing primitives only if a pre-existing domain path requires them. [VERIFIED: packages/Cargo.lock] |
| ureq | 3.3.0 | Existing blocking HTTP client used by CLI/RPC workflows | Use existing client patterns if status/support commands need HTTP calls; do not add another HTTP client. [VERIFIED: packages/Cargo.lock] |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Existing `OpenBitcoinStatusSnapshot`/`SyncStatus` | New observability DTO | Reject: locked decision D-01 requires consuming/summarizing canonical status, and a new DTO would create drift. [VERIFIED: .planning/phases/72-operator-observability-and-support-evidence/72-CONTEXT.md] |
| Existing support bundle JSON/Markdown | Separate support archive/report format | Reject: support command already writes redacted JSON and Markdown; extending it additively has lower compatibility risk. [VERIFIED: packages/open-bitcoin-cli/src/operator/support.rs] |
| Existing Bun checker style | New test runner/package setup | Reject: repo has no `package.json`, Bun is canonical for repo-owned automation, and existing phase checkers use direct Bun scripts. [VERIFIED: AGENTS.md] [VERIFIED: scripts/check-phase71-resource-restart.ts] |
| Baseline `getblockchaininfo` widening | Open Bitcoin extension RPC status | Reject for baseline shape: D-06 permits baseline-compatible `getblockchaininfo` to stay scoped while Open Bitcoin sync status exposes durable fields. [VERIFIED: .planning/phases/72-operator-observability-and-support-evidence/72-CONTEXT.md] [VERIFIED: packages/open-bitcoin-rpc/src/dispatch/node.rs] |

**Installation:** No new third-party packages should be installed for Phase 72; the repo already has the required Rust, Bun, Bazel, serde, CLI, RPC, terminal, and storage stack. [VERIFIED: packages/Cargo.toml] [VERIFIED: .bun-version]

**Version verification:** Versions above were verified from `rust-toolchain.toml`, `.bun-version`, `Cargo.lock`, `MODULE.bazel`, and local CLI probes on 2026-06-13. [VERIFIED: local commands]

## Architecture Patterns

### Recommended Project Structure

```text
packages/open-bitcoin-node/src/status.rs
  # Canonical OpenBitcoinStatusSnapshot, SyncStatus, availability enums, recovery labels. [VERIFIED: local file]
packages/open-bitcoin-node/src/sync/runtime_state.rs
  # Durable runtime projection into SyncStatus; source of active-chain/tip/recovery/resource truth. [VERIFIED: local file]
packages/open-bitcoin-cli/src/operator/status/render.rs
  # Human/JSON CLI status projection; add missing Phase 69-71 field rendering here. [VERIFIED: local file]
packages/open-bitcoin-cli/src/operator/dashboard/model.rs
  # Pure dashboard rows/metric projection; add matching rows here. [VERIFIED: local file]
packages/open-bitcoin-cli/src/operator/support.rs
packages/open-bitcoin-cli/src/operator/support/live_smoke.rs
packages/open-bitcoin-cli/src/operator/support/render.rs
  # Redacted support evidence, compact live-smoke summary, Markdown guidance/verdict output. [VERIFIED: local files]
scripts/run-live-mainnet-smoke.ts
  # Opt-in live-smoke report producer; expand schema v2 summary fields without default public-network verification. [VERIFIED: local file]
scripts/check-phase72-observability-evidence.ts
  # New deterministic Bun checker wired after Phase 71 in scripts/verify.sh. [VERIFIED: .planning/phases/72-operator-observability-and-support-evidence/72-CONTEXT.md]
docs/operator/runtime-guide.md
docs/architecture/status-snapshot.md
docs/architecture/operator-observability.md
  # Operator guidance and architecture contract updates. [VERIFIED: local files]
```

### Pattern 1: Canonical Snapshot In, Projections Out

**What:** Treat `OpenBitcoinStatusSnapshot` and nested `SyncStatus` as input to every status/support/rendering surface; renderers may summarize fields but must not recompute sync classification. [VERIFIED: .planning/phases/72-operator-observability-and-support-evidence/72-CONTEXT.md] [VERIFIED: packages/open-bitcoin-node/src/status.rs]

**When to use:** Use for CLI human output, JSON status, dashboard rows, support evidence, RPC Open Bitcoin sync status, live-smoke final summaries, metrics/log comparison, and deterministic fixtures. [VERIFIED: .planning/REQUIREMENTS.md]

**Implementation guidance:** If a field is unavailable, propagate `FieldAvailability::Unavailable { reason }` or the existing nullable/default field instead of inventing a substitute value. [VERIFIED: packages/open-bitcoin-node/src/status.rs]

### Pattern 2: Pure Evidence Projection and Typed Verdicts

**What:** Add a small data-in/data-out projection from shared status plus compact live-smoke summary into support evidence fields and a typed final verdict. [VERIFIED: .planning/phases/72-operator-observability-and-support-evidence/72-CONTEXT.md]

**When to use:** Use inside the support bundle path and tests; keep file IO, config discovery, store opening, and report reading in existing imperative support functions. [VERIFIED: packages/open-bitcoin-cli/src/operator/support.rs]

**Verdict shape:** The locked suggested labels are `sync_to_tip_proven`, `stay_current_proven`, `diagnosed_blocker`, and `inconclusive`; the implementation should encode required evidence per verdict so a verdict cannot be emitted without justification fields. [VERIFIED: .planning/phases/72-operator-observability-and-support-evidence/72-CONTEXT.md]

### Pattern 3: Additive Serde Fields With Defaults

**What:** Extend JSON evidence/report structs additively and default older/missing data to unavailable or absent optional fields. [VERIFIED: packages/open-bitcoin-node/src/status.rs] [VERIFIED: packages/open-bitcoin-cli/src/operator/support/live_smoke.rs]

**When to use:** Use for support evidence schema and live-smoke summary v2 additions so older reports remain parseable. [VERIFIED: .planning/phases/72-operator-observability-and-support-evidence/72-CONTEXT.md]

### Pattern 4: Deterministic Cross-Surface Fixture

**What:** Build one fixture status snapshot with best-known tip, stay-current, active-chain work, recovery, resource pressure, no-progress, reorg/reconcile, peers, metrics, logs, live-smoke, and support evidence; assert each surface agrees on the required core fields or unavailable reasons. [VERIFIED: .planning/phases/72-operator-observability-and-support-evidence/72-CONTEXT.md]

**When to use:** Use in Rust unit/fixture tests for renderers/evidence and in the Phase 72 Bun checker for source/docs/artifact coverage. [VERIFIED: packages/open-bitcoin-cli/src/operator/status/render/tests.rs] [VERIFIED: scripts/check-phase71-resource-restart.ts]

### Anti-Patterns to Avoid

- **Renderer-local classification:** Do not decide "current", "stalled", "blocked", or "proven" from a string renderer; use `SyncStatus` fields and typed evidence. [VERIFIED: .planning/phases/72-operator-observability-and-support-evidence/72-CONTEXT.md]
- **Downloaded height as proof:** Do not treat header height or downloaded block height as active-chain validation proof; connected and validated active-chain height/hash/work remain separate. [VERIFIED: docs/architecture/status-snapshot.md]
- **Raw support attachments:** Do not embed raw logs, raw live-smoke reports, peer transcripts, credentials, wallet material, or unbounded endpoint tables in support evidence. [VERIFIED: packages/open-bitcoin-cli/src/operator/support.rs] [VERIFIED: packages/open-bitcoin-cli/src/operator/support/live_smoke.rs]
- **Baseline RPC shape creep:** Do not widen baseline-compatible `getblockchaininfo` to carry Open Bitcoin support evidence; use Open Bitcoin sync status extensions. [VERIFIED: packages/open-bitcoin-rpc/src/dispatch/node.rs]
- **Public-network default verification:** Do not add mainnet live-smoke or service-manager checks to `scripts/verify.sh`; keep those as opt-in UAT. [VERIFIED: .planning/phases/72-operator-observability-and-support-evidence/72-CONTEXT.md] [VERIFIED: scripts/verify.sh]

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Sync truth model | A second observability/status schema that reclassifies progress | `OpenBitcoinStatusSnapshot` and `SyncStatus` | Locked D-01 requires one canonical contract; existing structs already carry Phase 68-71 fields. [VERIFIED: local context] [VERIFIED: packages/open-bitcoin-node/src/status.rs] |
| Availability/missing data semantics | Ad hoc strings like `n/a`, `unknown`, or inferred zeros | `FieldAvailability<T>` and existing nullable/default fields | The repo already serializes unavailable state with reasons, which lets comparison tests distinguish missing data from zero values. [VERIFIED: packages/open-bitcoin-node/src/status.rs] |
| Support redaction | Regex-only raw archive filtering after collection | Existing allowlisted support evidence and live-smoke summary extraction | Current support code omits raw logs, credentials, wallet material, and raw unbounded reports; extend the allowlist rather than collecting then scrubbing. [VERIFIED: packages/open-bitcoin-cli/src/operator/support.rs] [VERIFIED: packages/open-bitcoin-cli/src/operator/support/live_smoke.rs] |
| Final verdicts | Freeform Markdown conclusion based on bundle existence or elapsed time | Typed evidence-derived enum plus justification fields | D-11 requires a typed verdict and evidence justification; typed states reduce impossible or overbroad claims. [VERIFIED: .planning/phases/72-operator-observability-and-support-evidence/72-CONTEXT.md] |
| Cross-surface validation | Manual eyeballing of CLI/dashboard/RPC output | Deterministic fixture and Bun checker | D-08 and D-17 require deterministic comparison and checker coverage. [VERIFIED: .planning/phases/72-operator-observability-and-support-evidence/72-CONTEXT.md] |
| Metrics/log retention | Unbounded status history in support evidence | Existing bounded metric samples and compact structured log records | Metrics retention is bounded by policy, and structured logs retain compact records/signals. [VERIFIED: packages/open-bitcoin-node/src/metrics.rs] [VERIFIED: packages/open-bitcoin-node/src/logging.rs] |

**Key insight:** The hard part is not collecting more data; it is preventing surfaces from telling different stories about the same durable sync state. [VERIFIED: local source audit]

## Common Pitfalls

### Pitfall 1: Proving Sync With the Wrong Height

**What goes wrong:** A surface says sync is proven because headers or downloaded blocks advanced, while connected/validated active-chain height/hash/work do not prove the same result. [VERIFIED: docs/architecture/status-snapshot.md]

**Why it happens:** Existing renderers emphasize `headers`, `downloaded_blocks`, and `connected_blocks`; some surfaces do not render validated active-chain work yet. [VERIFIED: packages/open-bitcoin-cli/src/operator/status/render.rs] [VERIFIED: packages/open-bitcoin-cli/src/operator/dashboard/model.rs]

**How to avoid:** Include connected active-chain height/hash/work and validated active-chain work in comparison/evidence, and treat downloaded-only progress as insufficient proof. [VERIFIED: .planning/phases/72-operator-observability-and-support-evidence/72-CONTEXT.md]

**Warning signs:** Tests pass with only `downloaded_block_height` assertions, or verdict code ignores `maybe_validated_active_chain_work`. [VERIFIED: local source audit]

### Pitfall 2: Human CLI and Dashboard Drift

**What goes wrong:** JSON status contains Phase 69-71 fields but human CLI/dashboard omit or rename them, making operator support comparison ambiguous. [VERIFIED: packages/open-bitcoin-cli/src/operator/status/render.rs] [VERIFIED: packages/open-bitcoin-cli/src/operator/dashboard/model.rs]

**Why it happens:** CLI and dashboard have independent projection code and exact row tests. [VERIFIED: packages/open-bitcoin-cli/src/operator/status/render/tests.rs] [VERIFIED: packages/open-bitcoin-cli/src/operator/dashboard/model.rs]

**How to avoid:** Use shared helper formatting where practical, update exact tests, and make the cross-surface fixture assert row/line presence for best-known tip, stay-current state/action, no-progress diagnosis/action, latest reorg, reconcile progress, resource pressure, and validated work. [VERIFIED: .planning/phases/72-operator-observability-and-support-evidence/72-CONTEXT.md]

**Warning signs:** A new field appears in JSON/support evidence but not in human output or dashboard rows. [VERIFIED: local source audit]

### Pitfall 3: Support Bundle Overcollection

**What goes wrong:** A bundle embeds raw logs, raw live-smoke reports, peer transcripts, RPC cookies/passwords, wallet material, or unbounded tables. [VERIFIED: packages/open-bitcoin-cli/src/operator/support.rs] [VERIFIED: packages/open-bitcoin-cli/src/operator/support/live_smoke.rs]

**Why it happens:** Support evidence is tempting to implement as "attach everything" instead of allowlisting compact summaries. [VERIFIED: .planning/phases/72-operator-observability-and-support-evidence/72-CONTEXT.md]

**How to avoid:** Keep `support-evidence.json` additive, typed, and allowlisted; update redaction tests with forbidden field names and realistic secret-like strings. [VERIFIED: packages/open-bitcoin-cli/src/operator/support/live_smoke.rs]

**Warning signs:** New fields named `raw*`, `stdout`, `stderr`, `logTail`, `peerTable`, `cookie`, `password`, `rpcauth`, `xprv`, or `seed` enter support evidence. [VERIFIED: packages/open-bitcoin-cli/src/operator/support/live_smoke.rs]

### Pitfall 4: Accidentally Expanding Baseline RPC

**What goes wrong:** Baseline `getblockchaininfo` gains Open Bitcoin observability/support-specific fields that could break Knots-compatible expectations. [VERIFIED: packages/open-bitcoin-rpc/src/dispatch/node.rs]

**Why it happens:** Durable state is available in RPC context, so it is easy to expose too much through the baseline method. [VERIFIED: packages/open-bitcoin-rpc/src/context.rs]

**How to avoid:** Keep rich durable status under Open Bitcoin sync status/control RPC paths and test that baseline-compatible shape remains scoped. [VERIFIED: .planning/phases/72-operator-observability-and-support-evidence/72-CONTEXT.md]

**Warning signs:** New support-evidence fields appear in `BlockchainInfo`, or tests start depending on them through `getblockchaininfo`. [VERIFIED: packages/open-bitcoin-rpc/src/dispatch/node.rs]

### Pitfall 5: Checker Without Runtime Coverage

**What goes wrong:** The Phase 72 checker only looks for source strings and misses artifact agreement, support allowlist boundaries, or verification wiring. [VERIFIED: scripts/check-phase71-resource-restart.ts]

**Why it happens:** Existing phase checkers are lightweight and string-based, so planner must deliberately include fixture/artifact checks. [VERIFIED: scripts/check-phase68-active-chain-persistence.ts] [VERIFIED: scripts/check-phase71-resource-restart.ts]

**How to avoid:** Combine source/docs needle checks with deterministic fixture output checks where practical; wire the checker after Phase 71 in `scripts/verify.sh`. [VERIFIED: .planning/phases/72-operator-observability-and-support-evidence/72-CONTEXT.md]

**Warning signs:** `scripts/verify.sh` does not call `check-phase72-observability-evidence.ts`, or the checker cannot fail on raw support evidence fields. [VERIFIED: scripts/verify.sh]

### Pitfall 6: Missing Parity Breadcrumbs for New Rust Files

**What goes wrong:** New first-party Rust source/test files fail parity breadcrumb checks or omit explicit Open Bitcoin-only rationale. [VERIFIED: AGENTS.md]

**Why it happens:** Phase work may add new helper/test modules under `packages/open-bitcoin-*` paths. [VERIFIED: local source audit]

**How to avoid:** Either modify existing covered files or add/update entries in `docs/parity/source-breadcrumbs.json`; use explicit `none` breadcrumbs only when no defensible Knots anchor exists. [VERIFIED: docs/parity/source-breadcrumbs.json] [VERIFIED: AGENTS.md]

**Warning signs:** New `packages/open-bitcoin-*/src/*.rs` or `tests/*.rs` files are present but no breadcrumb diff exists. [VERIFIED: scripts/check-parity-breadcrumbs.ts]

## Code Examples

Verified local patterns to reuse.

### Availability-Carrying Fields

```rust
// Source: packages/open-bitcoin-node/src/status.rs [VERIFIED: local file]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "state", content = "value")]
pub enum FieldAvailability<T> {
    Available(T),
    Unavailable { reason: String },
}
```

Use this pattern for fields whose absence is meaningful, especially cross-surface comparison and support evidence unavailable reasons. [VERIFIED: packages/open-bitcoin-node/src/status.rs]

### Durable Status Projection Is the Current Truth Source

```rust
// Source: packages/open-bitcoin-node/src/sync/runtime_state.rs [VERIFIED: local file]
sync.sync_progress.downloaded_block_height = connected_block_height;
sync.sync_progress.connected_block_height = connected_block_height;
sync.sync_progress.validated_active_chain_height = connected_block_height;
sync.sync_progress.maybe_validated_active_chain_hash = maybe_connected_block_hash.clone();
sync.sync_progress.maybe_validated_active_chain_work =
    maybe_connected_block.as_ref().map(|block| block.chain_work.to_string());
```

Planner implication: compare/render these fields; do not recompute them downstream. [VERIFIED: packages/open-bitcoin-node/src/sync/runtime_state.rs]

### Existing CLI Human Renderer Style

```rust
// Source: packages/open-bitcoin-cli/src/operator/status/render.rs [VERIFIED: local file]
lines.push(format!(
    "progress: {:.2}% headers={} downloaded_blocks={} connected_blocks={}",
    snapshot.sync.sync_progress.progress_ratio * 100.0,
    snapshot.sync.sync_progress.header_height,
    snapshot.sync.sync_progress.downloaded_block_height,
    snapshot.sync.sync_progress.connected_block_height
));
```

Planner implication: extend this status block with validated active-chain height/hash/work and Phase 69-71 labels instead of adding a separate human status pathway. [VERIFIED: packages/open-bitcoin-cli/src/operator/status/render.rs]

### Existing Live-Smoke Summary Allowlist

```rust
// Source: packages/open-bitcoin-cli/src/operator/support/live_smoke.rs [VERIFIED: local file]
const FINAL_STATUS_KEYS: &[&str] = &[
    "headerHeight",
    "downloadedBlockHeight",
    "connectedBlockHeight",
    "blockHeight",
    "phase",
    "lifecycle",
    "outboundPeers",
    "messagesProcessed",
    "recoveryCategory",
    "maybeLastError",
    "maybeLastSuccessfulProgressUnixSeconds",
];
```

Planner implication: expand this allowlist for validated active-chain work, best-known tip, stay-current, no-progress, reorg/reconcile, and evidence verdict fields while keeping raw report keys excluded. [VERIFIED: packages/open-bitcoin-cli/src/operator/support/live_smoke.rs]

### Phase Checker Wiring Pattern

```bash
# Source: scripts/verify.sh [VERIFIED: local file]
run_step "Phase 70 reorg recovery checker" bun run scripts/check-phase70-reorg-recovery.ts
run_step "Phase 71 resource restart checker" bun run scripts/check-phase71-resource-restart.ts
```

Planner implication: add the Phase 72 checker immediately after the Phase 71 checker. [VERIFIED: .planning/phases/72-operator-observability-and-support-evidence/72-CONTEXT.md]

## State of the Art

| Old / Current Gap | Current Approach for Phase 72 | When Changed | Impact |
|-------------------|--------------------------------|--------------|--------|
| Renderer-local status summaries can omit shared fields | Shared snapshot is the canonical contract and every projection summarizes it | Locked for Phase 72 by D-01 [VERIFIED: 72-CONTEXT.md] | Prevents CLI/dashboard/RPC/support drift. |
| Header/downloaded progress could be confused with validated progress | Keep header, downloaded, connected, and validated active-chain height/hash/work distinct | Phase 68 requirement preserved by D-02 [VERIFIED: 72-CONTEXT.md] | Support evidence can prove only what active-chain fields prove. |
| Stay-current could be inferred from elapsed time or peer state | `current_at_best_known_tip` requires fresh best-known tip and connected active-chain height/hash/work matching validated tip | Phase 69 requirement preserved by D-03 [VERIFIED: 72-CONTEXT.md] | Verdicts can distinguish proven stay-current from stale/recovering/no-progress states. |
| Reorg/no-progress/recovery/resource fields could stay internal | These fields must be carried as labels or `Unavailable` across operator surfaces | Phase 70/71 requirement preserved by D-04 [VERIFIED: 72-CONTEXT.md] | Support evidence can diagnose blockers without raw logs. |
| Support evidence described state but did not type final proof | Typed `sync_to_tip_proven`, `stay_current_proven`, `diagnosed_blocker`, `inconclusive` verdict with justification | New Phase 72 decision D-11 [VERIFIED: 72-CONTEXT.md] | Operator guidance can make bounded claims. |
| Live-smoke support ingestion summarized a limited schema v2 field set | Expand schema v2 allowlist summary-only where needed | New Phase 72 decision D-12 [VERIFIED: 72-CONTEXT.md] | Existing report ingestion remains compact and redacted. |

**Deprecated/outdated for Phase 72:**

- Any implementation that derives final support verdicts from human output strings instead of shared status fields is outdated for this phase. [VERIFIED: 72-CONTEXT.md]
- Any default verification that requires public mainnet, service managers, timing-sensitive live smoke, or checked-in live reports is out of scope. [VERIFIED: 72-CONTEXT.md] [VERIFIED: scripts/verify.sh]

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| - | None. All research claims are verified against local project files or cited standards/docs. | - | - |

## Open Questions (RESOLVED)

1. **Where should the smallest shared projection helper live?**
   - What we know: Canonical status structs live in `open-bitcoin-node`, while CLI human output, dashboard, support bundles, and live-smoke ingestion live in `open-bitcoin-cli` and scripts. [VERIFIED: packages/open-bitcoin-node/src/status.rs] [VERIFIED: packages/open-bitcoin-cli/src/operator/support.rs]
   - RESOLVED: Keep canonical data types in `open-bitcoin-node`; put support/verdict-specific helpers in CLI support code unless RPC needs the exact same evidence object. Plans 72-01 and 72-02 implement this by extending status/dashboard/RPC projections from `SyncStatus` and deriving support verdicts inside the CLI support bundle path. [VERIFIED: architecture constraints] [VERIFIED: .planning/phases/72-operator-observability-and-support-evidence/72-01-PLAN.md] [VERIFIED: .planning/phases/72-operator-observability-and-support-evidence/72-02-PLAN.md]

2. **Should live-smoke report `schema_version` remain 2 or bump?**
   - What we know: Locked D-12 says "expand schema v2 allowlist where needed"; current report schema version is `2`. [VERIFIED: 72-CONTEXT.md] [VERIFIED: scripts/run-live-mainnet-smoke.ts]
   - RESOLVED: Keep support ingestion compatible with schema v2 and add tests for absent fields; bump producer schema only if the report meaning changes incompatibly. Plans 72-02 and 72-03 expand summary-only schema v2 allowlists and deterministic fixture checks without requiring an incompatible schema bump. [VERIFIED: D-12] [VERIFIED: .planning/phases/72-operator-observability-and-support-evidence/72-02-PLAN.md] [VERIFIED: .planning/phases/72-operator-observability-and-support-evidence/72-03-PLAN.md]

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|-------------|-----------|---------|----------|
| rustc | Rust source/tests/build | Yes | 1.94.1 | None needed. [VERIFIED: rustc --version] |
| cargo | Rust tests/build | Yes | 1.94.1 | None needed. [VERIFIED: cargo --version] |
| rustfmt | Formatting through Cargo toolchain | Yes | Toolchain component pinned by `rust-toolchain.toml` | None needed. [VERIFIED: rust-toolchain.toml] |
| clippy | Linting through Cargo toolchain | Yes | Toolchain component pinned by `rust-toolchain.toml` | None needed. [VERIFIED: rust-toolchain.toml] |
| bun | Phase checker and TS automation | Yes | 1.3.9 | None needed. [VERIFIED: .bun-version] [VERIFIED: bun --version] |
| bazel | Repo smoke build | Yes | 8.6.0 | None needed. [VERIFIED: bazel --version] |
| cargo-llvm-cov | Repo coverage gate | Yes | 0.8.5 | None needed. [VERIFIED: cargo llvm-cov --version] |
| git | Verification/hooks/source checks | Yes | Required by `scripts/verify.sh` | None needed. [VERIFIED: scripts/verify.sh] |
| grep | Verification/source checks | Yes | Required by `scripts/verify.sh` | None needed. [VERIFIED: scripts/verify.sh] |

**Missing dependencies with no fallback:** None found for deterministic Phase 72 planning. [VERIFIED: local environment probes]

**Missing dependencies with fallback:** None found. Public mainnet full-sync/stay-current UAT remains opt-in and must not be added to default verification. [VERIFIED: 72-CONTEXT.md]

## Security Domain

OWASP ASVS 5.0.0 is the current stable ASVS release according to OWASP project pages. [CITED: https://owasp.org/www-project-application-security-verification-standard/] [CITED: https://github.com/OWASP/ASVS]

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|------------------|
| V2 Authentication | Yes | Support evidence must preserve credential-source metadata only and must not serialize RPC cookies, RPC passwords, `rpcauth`, or auth headers. [VERIFIED: packages/open-bitcoin-cli/src/operator/support.rs] |
| V3 Session Management | Limited | No browser session is introduced; existing local RPC cookie/session material remains outside support bundles. [VERIFIED: packages/open-bitcoin-cli/src/operator/support.rs] |
| V4 Access Control | Limited | Do not expose support evidence through baseline RPC; preserve existing Open Bitcoin RPC control/status boundaries. [VERIFIED: packages/open-bitcoin-rpc/src/dispatch/node.rs] |
| V5 Input Validation | Yes | Parse live-smoke reports and support evidence as structured JSON, allowlist supported fields, and treat unavailable values explicitly. [VERIFIED: packages/open-bitcoin-cli/src/operator/support/live_smoke.rs] |
| V6 Cryptography | Limited | Do not add custom cryptography or cryptographic proof claims for support bundles; use existing domain crypto dependencies only where already required. [VERIFIED: packages/Cargo.lock] |

### Known Threat Patterns for Phase 72

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Credential leakage in support evidence | Information Disclosure | Allowlist compact evidence and test forbidden credential/log/wallet fields. [VERIFIED: packages/open-bitcoin-cli/src/operator/support/live_smoke.rs] |
| Raw logs or peer transcripts included in bundle | Information Disclosure | Keep structured log summary and compact live-smoke summary only; reject raw stdout/stderr/log tails. [VERIFIED: packages/open-bitcoin-cli/src/operator/support.rs] |
| Verdict overclaiming sync/stay-current proof | Spoofing / Repudiation | Use typed evidence-derived verdicts with justification fields tied to connected/validated active-chain and best-known-tip data. [VERIFIED: 72-CONTEXT.md] |
| Baseline RPC compatibility drift | Information Disclosure / Tampering | Keep rich durable evidence on Open Bitcoin sync status paths, not baseline `getblockchaininfo`. [VERIFIED: packages/open-bitcoin-rpc/src/dispatch/node.rs] |
| Flaky public-network default verification | Denial of Service | Keep public mainnet full-sync/stay-current UAT opt-in and out of `scripts/verify.sh`. [VERIFIED: 72-CONTEXT.md] [VERIFIED: scripts/verify.sh] |

## Sources

### Primary (HIGH confidence)

- `.planning/phases/72-operator-observability-and-support-evidence/72-CONTEXT.md` - locked decisions, discretion, deferred scope. [VERIFIED: local file]
- `.planning/REQUIREMENTS.md` - OBS-01 through OBS-04 requirement text. [VERIFIED: local file]
- `.planning/STATE.md` - Phase 72 pending state and default verification boundaries. [VERIFIED: local file]
- `AGENTS.md`, `AGENTS.bright-builds.md`, `standards-overrides.md` - repo and Bright Builds constraints. [VERIFIED: local files]
- `packages/open-bitcoin-node/src/status.rs` - canonical snapshot, sync status, availability/recovery/resource/tip/no-progress/reorg/reconcile types. [VERIFIED: local file]
- `packages/open-bitcoin-node/src/sync/runtime_state.rs` - durable sync-state projection. [VERIFIED: local file]
- `packages/open-bitcoin-node/src/sync/types/summary.rs` - sync-run summary, metric samples, structured log records. [VERIFIED: local file]
- `packages/open-bitcoin-node/src/metrics.rs` and `packages/open-bitcoin-node/src/logging.rs` - bounded metric/log behavior. [VERIFIED: local files]
- `packages/open-bitcoin-cli/src/operator/status/render.rs` and tests - CLI status renderer gaps and test patterns. [VERIFIED: local files]
- `packages/open-bitcoin-cli/src/operator/dashboard/model.rs` - dashboard projection gaps and exact row tests. [VERIFIED: local file]
- `packages/open-bitcoin-cli/src/operator/support.rs`, `support/live_smoke.rs`, `support/render.rs` - support bundle/redaction/live-smoke summary gaps. [VERIFIED: local files]
- `scripts/run-live-mainnet-smoke.ts` - opt-in live-smoke report schema and final status summary fields. [VERIFIED: local file]
- `scripts/verify.sh` and `scripts/check-phase71-resource-restart.ts` - checker/verification wiring pattern. [VERIFIED: local files]

### Secondary (MEDIUM confidence)

- Bright Builds canonical standards at commit `05f8d7a6c9c2e157ec4f922a05273e72dab97676` - architecture, code shape, verification, testing, Rust, TypeScript guidance. [CITED: https://raw.githubusercontent.com/peterryszkiewicz/bright-builds/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/index.md]
- OWASP ASVS official project and GitHub pages - ASVS 5.0.0 current stable source for security category framing. [CITED: https://owasp.org/www-project-application-security-verification-standard/] [CITED: https://github.com/OWASP/ASVS]

### Tertiary (LOW confidence)

- None. [VERIFIED: source review]

## Metadata

**Confidence breakdown:**

- Standard stack: HIGH - verified from pinned local files, lockfile, and installed command versions. [VERIFIED: rust-toolchain.toml] [VERIFIED: packages/Cargo.lock] [VERIFIED: local commands]
- Architecture: HIGH - verified from canonical status/runtime/support/render/RPC code and Phase 72 decisions. [VERIFIED: local source audit]
- Pitfalls: HIGH - derived from explicit locked decisions and observed renderer/support/live-smoke gaps. [VERIFIED: local source audit]
- Security: MEDIUM - local redaction threats are verified, ASVS category mapping is a planning classification over official ASVS sources. [VERIFIED: local source audit] [CITED: https://owasp.org/www-project-application-security-verification-standard/]

**Research date:** 2026-06-13 [VERIFIED: environment current_date]
**Valid until:** 2026-07-13 for repo-local architecture, or sooner if Phase 72 context/status APIs change. [VERIFIED: planning estimate]
