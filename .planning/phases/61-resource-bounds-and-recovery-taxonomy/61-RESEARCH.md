# Phase 61: Resource Bounds and Recovery Taxonomy - Research

**Researched:** 2026-06-06
**Domain:** Open Bitcoin unattended sync resource bounds, recovery taxonomy, operator evidence
**Confidence:** HIGH

<user_constraints>

## User Constraints (from CONTEXT.md)

Copied verbatim from `.planning/phases/61-resource-bounds-and-recovery-taxonomy/61-CONTEXT.md`. [VERIFIED: .planning/phases/61-resource-bounds-and-recovery-taxonomy/61-CONTEXT.md]

### Locked Decisions

### Bounded Resource Envelope

- **D-01:** Treat `SyncResourcePressure` as the shared status contract for
  active sync bounds. It must report observed in-flight block pressure plus
  configured limits for header requests, protocol header batch size, per-peer
  and total block in-flight limits, messages per peer, sync rounds, outbound
  peers, and target outbound peers.
- **D-02:** Do not introduce unbounded queues or retained report arrays for
  Phase 61. Retry state, peer outcomes, metrics samples, structured logs, and
  support evidence summaries must stay bounded by existing config and retention
  policies or by explicit compact summaries.
- **D-03:** Make long-run bound preservation deterministic. Add tests around
  scripted sync outcomes and projection/rendering surfaces rather than relying
  on public-network long-run tests.

### Recovery Taxonomy

- **D-04:** Normalize recovery states into a single operator-facing taxonomy:
  clean shutdown, unclean shutdown, incompatible schema, store corruption,
  storage lock contention or backend failure, resource exhaustion, invalid peer
  data, public-network unreachability, and operator cancellation.
- **D-05:** Storage incompatibility and corruption continue to outrank peer or
  network guidance. If durable metadata exposes a storage recovery action, status
  and support surfaces should present that before recommending network retries.
- **D-06:** Map existing low-level signals into typed recovery categories rather
  than adding ad hoc strings at each renderer. `StorageError`,
  `StorageRecoveryAction`, `PeerFailureReason`, `SyncStopReason`,
  `SyncRuntimeError`, live-smoke `maybeNoProgressCause`, and durable
  `last_clean_shutdown` are the input facts.

### Operator Truth Surfaces

- **D-07:** Status, dashboard, RPC sync status, structured logs, metrics, support
  evidence, and docs should use the same names for recovery categories, progress
  signals, resource pressure, and next action guidance. A renderer may choose
  human wording, but the underlying category labels must remain stable.
- **D-08:** Phase 61 should add compact support-evidence fields only where needed
  to expose bounds and recovery taxonomy. It must preserve the allowlist and
  redaction posture from Phase 59 and avoid embedding raw live-smoke reports,
  daemon tails, peer endpoint tables, secrets, wallet material, or unbounded log
  samples.
- **D-09:** Operator docs should explain how to inspect the active bounds and how
  to interpret recovery categories with copy-pasteable repo-local Cargo and
  Bazel commands. Public-network review commands remain clearly opt-in UAT.

### Verification Posture

- **D-10:** Default verification stays deterministic. Phase verification should
  include targeted Rust tests, Bun fixture checks if live-smoke/support scripts
  change, documentation/release-boundary checks where relevant, and the
  repo-native `bash scripts/verify.sh`.
- **D-11:** Public-network long-run UAT may be documented as an optional operator
  review path, but it must not become part of `bash scripts/verify.sh` or phase
  completion proof.

### the agent's Discretion

- The planner may introduce a small domain enum or projection helper for
  recovery categories if it reduces string duplication across status, support,
  live-smoke, and docs.
- The planner may keep resource-bound proof in existing sync/status tests if no
  new module boundary is justified. If new first-party Rust files are added,
  parity breadcrumbs must be updated.
- The executor may defer Phase 62-only truth-surface expansion if Phase 61 can
  prove typed states and bounds without broad status/dashboard/RPC rewrites.

### Deferred Ideas (OUT OF SCOPE)

- Phase 62 owns broader long-run truth consistency across status, dashboard,
  RPC, metrics, logs, and live-smoke snapshots.
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
| RR-01 | Unattended sync enforces documented bounds for outbound peers, in-flight headers or blocks, retry queues, storage writes, metrics samples, structured logs, and support evidence size. | Use existing `SyncResourcePressure`, `SyncRuntimeConfig`, metric retention, log retention, and support allowlist tests as the implementation base. [VERIFIED: .planning/REQUIREMENTS.md; packages/open-bitcoin-node/src/status.rs:117; packages/open-bitcoin-node/src/sync/types.rs:169; packages/open-bitcoin-node/src/metrics.rs:55; packages/open-bitcoin-node/src/logging.rs:115; packages/open-bitcoin-cli/src/operator/support/live_smoke.rs:8] |
| RR-02 | Recovery handling distinguishes clean shutdown, unclean shutdown, incompatible schema, store corruption, storage lock contention, resource exhaustion, invalid peer data, public-network unreachability, and operator cancellation. | Add one typed recovery-category contract and map existing storage, sync, peer, stop-reason, live-smoke, and clean-shutdown signals into it with storage-first precedence. [VERIFIED: .planning/REQUIREMENTS.md; packages/open-bitcoin-node/src/storage.rs:81; packages/open-bitcoin-node/src/sync/types.rs:234; packages/open-bitcoin-node/src/sync/types.rs:367; packages/open-bitcoin-node/src/sync/types.rs:432; scripts/run-live-mainnet-smoke.ts:176] |
| RR-04 | Operator-visible errors and recovery guidance stay typed, actionable, and consistent across status, logs, support bundles, and docs. | Project the same category labels through status JSON/human rendering, dashboard rows, RPC warnings or sync status, structured logs, support summaries, live-smoke fixtures, and runtime docs without renderer-local string taxonomies. [VERIFIED: .planning/REQUIREMENTS.md; packages/open-bitcoin-cli/src/operator/status/render.rs:88; packages/open-bitcoin-cli/src/operator/dashboard/model.rs:126; packages/open-bitcoin-rpc/src/dispatch/node.rs:122; packages/open-bitcoin-cli/src/operator/support/render.rs:130; docs/architecture/status-snapshot.md:79] |

</phase_requirements>

## Summary

Phase 61 should be a first-party contract/projection phase, not a dependency phase. [VERIFIED: .planning/phases/61-resource-bounds-and-recovery-taxonomy/61-CONTEXT.md; packages/Cargo.toml] The codebase already has the resource-pressure shape, bounded metric/log retention primitives, storage recovery actions, peer failure reasons, stop reasons, and live-smoke recovery diagnosis inputs that this phase needs. [VERIFIED: packages/open-bitcoin-node/src/status.rs:117; packages/open-bitcoin-node/src/metrics.rs:55; packages/open-bitcoin-node/src/logging.rs:115; packages/open-bitcoin-node/src/storage.rs:81; packages/open-bitcoin-node/src/sync/types.rs:234; scripts/run-live-mainnet-smoke.ts:176]

The smallest robust implementation path is to add one stable typed recovery category to the shared sync/status contract, keep the existing human `recovery_action` wording, and centralize storage/peer/runtime/stop-reason mapping in one projection helper. [VERIFIED: .planning/phases/61-resource-bounds-and-recovery-taxonomy/61-CONTEXT.md D-04-D-07; packages/open-bitcoin-node/src/status.rs:131; packages/open-bitcoin-node/src/sync/runtime_state.rs:332] Resource-bound work should prove that repeated deterministic sync cycles preserve configured limits and retention policies rather than adding a public-network long-run gate. [VERIFIED: .planning/phases/61-resource-bounds-and-recovery-taxonomy/61-CONTEXT.md D-03,D-10,D-11; scripts/verify.sh]

**Primary recommendation:** Add `SyncRecoveryCategory` plus a storage-first projection helper in first-party Rust, expose it additively through existing status/support/doc surfaces, and verify with deterministic Rust and Bun fixture tests. [VERIFIED: .planning/phases/61-resource-bounds-and-recovery-taxonomy/61-CONTEXT.md; packages/open-bitcoin-node/src/sync/tests.rs; scripts/test-run-live-mainnet-smoke.sh]

## Project Constraints (from AGENTS.md)

- Use `git submodule update --init --recursive` when the pinned Bitcoin Knots baseline is needed. [VERIFIED: AGENTS.md]
- Use `rust-toolchain.toml` as the Rust source of truth; the repo pins Rust `1.94.1`. [VERIFIED: AGENTS.md; rust-toolchain.toml]
- Use `bash scripts/verify.sh` as the repo-native verification contract for first-party code, including Bazel smoke build coverage. [VERIFIED: AGENTS.md; scripts/verify.sh]
- Provide repo-local Cargo and Bazel commands for UAT and operator workflows, preferring `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- ...` and `bazel run //packages/open-bitcoin-cli:open_bitcoin -- ...`. [VERIFIED: AGENTS.md]
- Use Bun for repo-owned higher-level automation scripts and TypeScript for substantial script logic; keep Bash for thin orchestration wrappers and simple shell checks. [VERIFIED: AGENTS.md; .planning/STACK.md]
- Install or repair repo-managed hooks with `bash scripts/install-git-hooks.sh`; `bash scripts/verify.sh` self-heals missing hook installation outside CI. [VERIFIED: AGENTS.md; scripts/verify.sh]
- Treat `docs/metrics/lines-of-code.md` as tracked generated output that may need freshness updates after verification. [VERIFIED: AGENTS.md]
- Record intentional in-scope behavior differences from Bitcoin Knots in `docs/parity/index.json` and companion docs. [VERIFIED: AGENTS.md; docs/parity/index.json]
- When adding first-party Rust source or tests under `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests`, update `docs/parity/source-breadcrumbs.json`; use `none` only when no defensible Knots anchor exists. [VERIFIED: AGENTS.md]
- After substantial feature, parity, operator-surface, or workflow changes, check relevant README files for contributor-facing status updates. [VERIFIED: AGENTS.md]
- Preserve externally observable Bitcoin Knots `29.3.knots20260210` behavior for in-scope surfaces and keep parity evidence auditable. [VERIFIED: AGENTS.md; .planning/PROJECT.md]
- Keep functional-core/domain logic free of direct I/O and isolate filesystem, process, network, terminal, RPC, service-manager, and durable-storage effects in adapters. [VERIFIED: AGENTS.md; Bright Builds architecture standard]
- Do not use existing Rust Bitcoin libraries in the production path; Open Bitcoin owns its domain model and implementation surface. [VERIFIED: AGENTS.md; .planning/PROJECT.md]
- Before committing Rust work, run repo-appropriate formatting, clippy, build, and tests; this repo's aggregate contract is `bash scripts/verify.sh`. [VERIFIED: AGENTS.md; scripts/verify.sh]
- Rust style constraints include `foo.rs` plus `foo/` over new `mod.rs`, no `unwrap()`, `let...else` for guard extraction, `maybe_` prefixes for `Option`, `thiserror` for library errors, `anyhow` for application errors, and `tracing` instead of `println!`. [VERIFIED: AGENTS.md; Bright Builds Rust standard]
- Unit tests should test one concern, use Arrange/Act/Assert comments when non-trivial, and test behavior rather than implementation details. [VERIFIED: AGENTS.md; Bright Builds testing standard]
- Bright Builds canonical standards materially loaded for this research: architecture, code-shape, verification, testing, Rust, TypeScript/JavaScript, and operability. [CITED: https://raw.githubusercontent.com/bright-builds-llc/bright-builds-rules/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/index.md]

## Standard Stack

### Core

| Library / Surface | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| Rust toolchain | 1.94.1 | First-party implementation language for sync/runtime/status/domain code. | The repo pins Rust 1.94.1 in `rust-toolchain.toml` and the installed `cargo`/`rustc` match it. [VERIFIED: rust-toolchain.toml; command: `cargo --version`; command: `rustc --version`] |
| `open-bitcoin-node` | 0.1.0 workspace | Owns sync runtime, durable status, storage recovery, metrics, logs, and resource pressure. | Phase 61 inputs live in node status, storage, sync, metrics, and logging code. [VERIFIED: packages/Cargo.toml; packages/open-bitcoin-node/src/status.rs; packages/open-bitcoin-node/src/sync/types.rs] |
| `open-bitcoin-cli` | 0.1.0 workspace | Owns operator status, dashboard, support bundle, and live-smoke support summary rendering. | Phase 61 operator truth surfaces already project shared status and support summaries here. [VERIFIED: packages/Cargo.toml; packages/open-bitcoin-cli/src/operator/status/render.rs; packages/open-bitcoin-cli/src/operator/dashboard/model.rs; packages/open-bitcoin-cli/src/operator/support/live_smoke.rs] |
| `open-bitcoin-rpc` | 0.1.0 workspace | Owns `open-bitcoind`, RPC dispatch, and durable sync warnings. | RPC warnings already include durable `last_error` and `recovery_action`, so the category can be projected additively. [VERIFIED: packages/Cargo.toml; packages/open-bitcoin-rpc/src/dispatch/node.rs:122] |
| Bun | 1.3.9 installed, `.bun-version` pinned | Runs repo-owned TypeScript automation and live-smoke fixture checks. | Repo guidance makes Bun canonical for TS automation, and the installed version is available. [VERIFIED: AGENTS.md; command: `bun --version`] |
| Bazel/Bazelisk | Bazelisk 1.28.1, Bazel 8.6.0 | Top-level smoke build and operator command form. | Repo verification calls Bazel targets and UAT docs require Bazel command equivalents. [VERIFIED: AGENTS.md; command: `bazelisk version`; scripts/verify.sh] |

### Supporting

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| `fjall` | 3.1.4 | Durable store backend for runtime metadata, metrics snapshots, recovery markers, headers, blocks, and chainstate. | Use existing `FjallNodeStore` recovery and metrics APIs; do not add a new store abstraction for Phase 61. [VERIFIED: cargo metadata; packages/open-bitcoin-node/src/storage/fjall_store.rs] |
| `tokio` | 1.52.1 | Async runtime for RPC/daemon paths. | Only use where existing RPC/daemon tests require it; Phase 61 recovery mapping should stay pure where practical. [VERIFIED: cargo metadata; Bright Builds architecture standard] |
| `axum` | 0.8.9 | RPC HTTP server surface. | Avoid changing Axum routing unless RPC sync status needs an additive field; Phase 61 is mostly projection code. [VERIFIED: cargo metadata; packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs from Phase 60 summary] |
| `clap` | 4.6.1 | CLI parsing. | Use only if operator commands gain flags; current research recommends docs/status/support projection, not new command flags. [VERIFIED: cargo metadata; packages/open-bitcoin-cli/Cargo.toml] |
| `serde` / `serde_json` | 1.0.228 / 1.0.149 | Stable JSON contracts for status, support evidence, runtime metadata, and live-smoke summaries. | Use serde snake_case enums for typed recovery category labels. [VERIFIED: cargo metadata; packages/open-bitcoin-node/src/status.rs:88; packages/open-bitcoin-cli/src/operator/support/live_smoke.rs:6] |
| `jsonc-parser` | 0.32.3 | Open Bitcoin JSONC config parsing. | No Phase 61 config schema addition is recommended unless bounds docs need existing field references. [VERIFIED: cargo metadata; docs/architecture/config-precedence.md] |
| `ratatui` / `crossterm` | 0.30.0 / 0.29.0 | Terminal dashboard rendering. | Keep dashboard changes in pure `model.rs` projection tests unless Phase 62 expands dashboard behavior. [VERIFIED: cargo metadata; packages/open-bitcoin-cli/src/operator/dashboard/model.rs] |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Existing first-party Rust/TS projection code | New taxonomy/support/report dependency | Not recommended: Phase 61 already has typed input facts and repo constraints minimize dependencies. [VERIFIED: .planning/phases/61-resource-bounds-and-recovery-taxonomy/61-CONTEXT.md; AGENTS.md] |
| Deterministic scripted fixtures | Public-network long-run verification | Not allowed for default verification: public-network checks remain opt-in UAT and outside `bash scripts/verify.sh`. [VERIFIED: .planning/phases/61-resource-bounds-and-recovery-taxonomy/61-CONTEXT.md D-10,D-11; scripts/verify.sh] |
| One shared recovery category | Renderer-local strings | Not allowed by user decisions: category labels must remain stable across status, logs, support, and docs. [VERIFIED: .planning/phases/61-resource-bounds-and-recovery-taxonomy/61-CONTEXT.md D-06,D-07] |

**Installation:**

No new dependency installation is recommended. [VERIFIED: packages/Cargo.toml; cargo metadata]

```bash
cargo metadata --format-version 1 --manifest-path packages/Cargo.toml --locked
bash scripts/verify.sh
```

**Version verification:** Recommended package versions were verified from the local lockfile via Cargo metadata rather than `npm view`, because this phase uses the Rust workspace and Bun scripts without a `package.json`. [VERIFIED: cargo metadata; AGENTS.md]

## Architecture Patterns

### Recommended Project Structure

Keep implementation in existing files unless file length or cohesion forces a split. [VERIFIED: .planning/phases/61-resource-bounds-and-recovery-taxonomy/61-CONTEXT.md; Bright Builds code-shape standard]

```text
packages/open-bitcoin-node/src/
├── status.rs                 # Add shared SyncRecoveryCategory status contract if file length stays acceptable.
├── sync/types.rs             # Map PeerFailureReason, SyncStopReason, SyncRuntimeError into category inputs.
├── sync/runtime_state.rs     # Apply storage-first recovery projection into DurableSyncState.
├── sync/types/summary.rs     # Project category labels into logs/metrics-facing summary tests.
└── sync/tests.rs             # Deterministic long-run bounds and recovery matrix tests.

packages/open-bitcoin-cli/src/operator/
├── status/render.rs          # Render category plus existing human recovery action.
├── dashboard/model.rs        # Pure dashboard row projection.
└── support/live_smoke.rs     # Add compact allowlisted category/resource fields only if needed.

scripts/
├── run-live-mainnet-smoke.ts       # Align existing RecoveryDiagnosisCategory labels if script surface changes.
└── test-run-live-mainnet-smoke.sh  # Deterministic fixture checks only; no public-network default gate.

docs/
├── operator/runtime-guide.md
└── architecture/status-snapshot.md
```

### Pattern 1: Add One Typed Recovery Category

**What:** Add a serde snake_case enum for the Phase 61 recovery categories and expose it additively, likely as `sync.recovery_category: FieldAvailability<SyncRecoveryCategory>`, while keeping `sync.recovery_action` as human next-action text. [VERIFIED: packages/open-bitcoin-node/src/status.rs:131; .planning/phases/61-resource-bounds-and-recovery-taxonomy/61-CONTEXT.md D-04,D-07]

**When to use:** Use this when multiple renderers need the same stable labels but different human wording. [VERIFIED: packages/open-bitcoin-cli/src/operator/status/render.rs:88; packages/open-bitcoin-cli/src/operator/dashboard/model.rs:126; packages/open-bitcoin-cli/src/operator/support/render.rs:130]

**Example:**

```rust
// Source: local pattern from status.rs serde enums and Phase 61 context.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SyncRecoveryCategory {
    CleanShutdown,
    UncleanShutdown,
    IncompatibleSchema,
    StoreCorruption,
    StorageLockOrBackendFailure,
    ResourceExhaustion,
    InvalidPeerData,
    PublicNetworkUnreachable,
    OperatorCancellation,
}
```

### Pattern 2: Storage-First Projection Helper

**What:** Map existing facts into the category in one helper, checking storage schema/corruption/recovery metadata before peer or network reasons. [VERIFIED: .planning/phases/61-resource-bounds-and-recovery-taxonomy/61-CONTEXT.md D-05,D-06; packages/open-bitcoin-node/src/sync/runtime_state.rs:385]

**When to use:** Use the helper in durable status projection and tests; renderers should consume the already-projected category. [VERIFIED: Bright Builds architecture standard; packages/open-bitcoin-node/src/sync/runtime_state.rs:332]

**Example:**

```rust
// Source: local inputs in StorageError, PeerFailureReason, SyncStopReason, RuntimeMetadata.
fn recovery_category_from_inputs(
    metadata: &RuntimeMetadata,
    summary: &SyncRunSummary,
    maybe_error: Option<&SyncRuntimeError>,
) -> SyncRecoveryCategory {
    if let Some(category) = category_from_storage(metadata, maybe_error) {
        return category;
    }
    if matches!(summary.maybe_stop_reason, Some(SyncStopReason::OperatorPaused)) {
        return SyncRecoveryCategory::OperatorCancellation;
    }
    category_from_latest_peer(summary)
        .unwrap_or_else(|| {
            if metadata.last_clean_shutdown {
                SyncRecoveryCategory::CleanShutdown
            } else {
                SyncRecoveryCategory::UncleanShutdown
            }
        })
}
```

### Pattern 3: Deterministic Long-Run Bound Fixtures

**What:** Use scripted sync outcomes and repeated cycles to assert that peer outcomes, retry/backoff state, in-flight block counters, metric history, structured logs, and support summaries stay within documented caps. [VERIFIED: .planning/phases/61-resource-bounds-and-recovery-taxonomy/61-CONTEXT.md D-03; packages/open-bitcoin-node/src/sync/tests.rs; packages/open-bitcoin-node/src/metrics.rs:91; packages/open-bitcoin-node/src/logging/writer.rs:18]

**When to use:** Use this for RR-01 and RR-04 proof; do not use public mainnet for default verification. [VERIFIED: .planning/REQUIREMENTS.md; scripts/verify.sh]

**Example:**

```rust
// Source: existing sync tests use ScriptedTransport, ScriptedResolver, and AAA comments.
#[test]
fn repeated_sync_cycles_preserve_resource_bounds() {
    // Arrange
    // Build a bounded SyncRuntimeConfig and scripted peers that alternate waiting,
    // invalid data, resource-limit, and useful progress outcomes.

    // Act
    // Run enough deterministic cycles to exceed the retention windows if pruning
    // were absent.

    // Assert
    // Assert SyncResourcePressure fields, metric sample counts, log retention,
    // peer outcome count, and support summary shape stay bounded.
}
```

### Anti-Patterns to Avoid

- **Renderer-local recovery strings:** This would violate the locked decision to keep underlying category labels stable across surfaces. [VERIFIED: .planning/phases/61-resource-bounds-and-recovery-taxonomy/61-CONTEXT.md D-06,D-07]
- **Raw live-smoke/support report retention:** This would violate Phase 59's allowlist/redaction posture and Phase 61 D-08. [VERIFIED: .planning/phases/59-operator-evidence-threat-model-and-release-boundaries/59-CONTEXT.md D-05; .planning/phases/61-resource-bounds-and-recovery-taxonomy/61-CONTEXT.md D-08]
- **Public-network long-run tests in `scripts/verify.sh`:** This is explicitly out of scope for default verification. [VERIFIED: .planning/REQUIREMENTS.md; scripts/verify.sh]
- **Broad Phase 62/65 truth-surface expansion:** Phase 62 owns broader long-run consistency and Phase 65 owns v1.5 support-bundle expansion. [VERIFIED: .planning/ROADMAP.md; .planning/phases/61-resource-bounds-and-recovery-taxonomy/61-CONTEXT.md]
- **New Rust Bitcoin production dependency:** Repo constraints prohibit existing Rust Bitcoin libraries in the production path. [VERIFIED: AGENTS.md; .planning/PROJECT.md]

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Recovery state naming | Per-renderer string switch statements | Shared `SyncRecoveryCategory` enum plus one projection helper | The type system can make illegal category labels unrepresentable and keep RR-04 consistent. [VERIFIED: Bright Builds architecture/Rust standards; .planning/phases/61-resource-bounds-and-recovery-taxonomy/61-CONTEXT.md D-06,D-07] |
| Resource pressure fields | New ad hoc status DTO | Existing `SyncResourcePressure` | The existing status contract already contains in-flight blocks and configured header/block/message/round/peer bounds. [VERIFIED: packages/open-bitcoin-node/src/status.rs:117] |
| Metrics retention | Custom per-test pruning logic | `MetricRetentionPolicy` and `append_and_prune_metric_samples` | Existing metrics code caps by interval, per-series count, and age. [VERIFIED: packages/open-bitcoin-node/src/metrics.rs:55; packages/open-bitcoin-node/src/metrics.rs:91] |
| Structured log retention | Custom file cleanup | `LogRetentionPolicy`, `append_structured_log_record`, and `plan_log_retention` | Existing logging code rotates by Unix day and prunes by file count, age, and total bytes. [VERIFIED: packages/open-bitcoin-node/src/logging.rs:115; packages/open-bitcoin-node/src/logging/writer.rs:18; packages/open-bitcoin-node/src/logging/prune.rs:41] |
| Support evidence redaction | Raw report copying with filters after the fact | Existing allowlisted `support/live_smoke.rs` projection and redaction helpers | Existing tests assert raw daemon tails, endpoint tables, secrets, and wallet-like material are absent. [VERIFIED: packages/open-bitcoin-cli/src/operator/support/live_smoke.rs:8; packages/open-bitcoin-cli/tests/operator_binary.rs:1057] |
| Long-run proof | Public-mainnet sleep/timeout tests | Scripted Rust transports, Bun fixture tests, and docs checks | Default verification must stay deterministic and public-network free. [VERIFIED: .planning/phases/61-resource-bounds-and-recovery-taxonomy/61-CONTEXT.md D-10,D-11; scripts/verify.sh] |

**Key insight:** Phase 61's hard part is contract alignment, not algorithm invention; the existing repo already has the bounded runtime and evidence primitives, but recovery category labels are still split between Rust strings and TypeScript live-smoke categories. [VERIFIED: packages/open-bitcoin-node/src/sync/runtime_state.rs:385; scripts/run-live-mainnet-smoke.ts:176]

## Common Pitfalls

### Pitfall 1: Treating `recovery_action` as the Typed State

**What goes wrong:** Human guidance text becomes the machine category, causing status, logs, support, and docs to drift when wording changes. [VERIFIED: packages/open-bitcoin-node/src/sync/runtime_state.rs:385; packages/open-bitcoin-cli/src/operator/status/render.rs:92]

**Why it happens:** The current shared status has `recovery_action: FieldAvailability<String>` but no typed recovery category field. [VERIFIED: packages/open-bitcoin-node/src/status.rs:141]

**How to avoid:** Add an additive typed category field and keep `recovery_action` as wording only. [VERIFIED: .planning/phases/61-resource-bounds-and-recovery-taxonomy/61-CONTEXT.md D-04,D-07]

**Warning signs:** Tests assert long English strings instead of snake_case labels such as `store_corruption` or `resource_exhaustion`. [VERIFIED: packages/open-bitcoin-cli/src/operator/status/render.rs:476; scripts/test-run-live-mainnet-smoke.sh:1031]

### Pitfall 2: Letting Peer Guidance Outrank Storage Recovery

**What goes wrong:** Operators may be told to retry network peers while the durable store needs reindex, repair, restore, or schema handling. [VERIFIED: .planning/phases/61-resource-bounds-and-recovery-taxonomy/61-CONTEXT.md D-05; packages/open-bitcoin-node/src/storage.rs:91]

**Why it happens:** Peer failure reasons already carry operator recovery messages, and they are easy to project from summaries. [VERIFIED: packages/open-bitcoin-node/src/sync/types.rs:277; packages/open-bitcoin-node/src/sync/types/summary.rs:113]

**How to avoid:** In the recovery helper, classify `StorageError`, recovery marker, and `maybe_last_recovery_action` before peer outcomes or network diagnosis. [VERIFIED: packages/open-bitcoin-node/src/sync/runtime_state.rs:385; packages/open-bitcoin-node/src/storage/fjall_store.rs:373]

**Warning signs:** A schema mismatch or corruption fixture yields `public_network_unreachable` or peer retry text. [VERIFIED: packages/open-bitcoin-node/src/storage.rs:152; scripts/run-live-mainnet-smoke.ts:1934]

### Pitfall 3: Proving Bounds Only at the Status Renderer

**What goes wrong:** Human output shows configured limits, but repeated runtime cycles still allow unbounded metric/log/support or retry evidence growth. [VERIFIED: packages/open-bitcoin-cli/src/operator/status/render.rs:184; packages/open-bitcoin-node/src/metrics.rs:91; packages/open-bitcoin-node/src/logging/writer.rs:18]

**Why it happens:** Resource pressure, metrics, structured logs, and support summaries are separate code paths. [VERIFIED: packages/open-bitcoin-node/src/sync/runtime_state.rs:93; packages/open-bitcoin-node/src/sync/runtime_state.rs:109; packages/open-bitcoin-cli/src/operator/support/live_smoke.rs:8]

**How to avoid:** Add tests for runtime projection plus retention/support rendering surfaces, not just the human status line. [VERIFIED: .planning/phases/61-resource-bounds-and-recovery-taxonomy/61-CONTEXT.md D-03,D-07]

**Warning signs:** RR-01 tests only check `Sync pressure:` text and never inspect persisted metrics, log retention, or support summary JSON. [VERIFIED: packages/open-bitcoin-cli/src/operator/status/render.rs:88; packages/open-bitcoin-node/src/storage/fjall_store/tests.rs:407; packages/open-bitcoin-cli/tests/operator_binary.rs:771]

### Pitfall 4: Expanding Support Bundles into Phase 65

**What goes wrong:** Phase 61 starts collecting broad v1.5 support data, service state, daemon tails, endpoint tables, or raw reports. [VERIFIED: .planning/ROADMAP.md Phase 65; .planning/phases/61-resource-bounds-and-recovery-taxonomy/61-CONTEXT.md D-08]

**Why it happens:** RR-04 mentions support bundles, but Phase 65 owns the v1.5 support-bundle collection expansion. [VERIFIED: .planning/ROADMAP.md; .planning/REQUIREMENTS.md]

**How to avoid:** Add only compact fields needed to expose the recovery category/resource bounds and preserve the Phase 59 allowlist. [VERIFIED: .planning/phases/61-resource-bounds-and-recovery-taxonomy/61-CONTEXT.md D-08; packages/open-bitcoin-cli/src/operator/support/live_smoke.rs:75]

**Warning signs:** Tests start approving raw `endpoint_outcomes`, `daemonStderrTail`, `manualPeers`, or unbounded arrays in support JSON. [VERIFIED: packages/open-bitcoin-cli/tests/operator_binary.rs:1057]

### Pitfall 5: Treating Optional Public UAT as Completion Proof

**What goes wrong:** Phase completion becomes dependent on network reachability or long-run public peer behavior. [VERIFIED: .planning/REQUIREMENTS.md; .planning/phases/61-resource-bounds-and-recovery-taxonomy/61-CONTEXT.md D-11]

**Why it happens:** The phase goal mentions long runs, but the locked verification posture says deterministic default verification. [VERIFIED: .planning/ROADMAP.md Phase 61; .planning/phases/61-resource-bounds-and-recovery-taxonomy/61-CONTEXT.md D-10]

**How to avoid:** Document opt-in UAT commands, but verify with scripted Rust/Bun fixtures and `bash scripts/verify.sh`. [VERIFIED: docs/operator/runtime-guide.md:610; scripts/verify.sh]

**Warning signs:** `scripts/verify.sh` gains `run-live-mainnet-smoke`, `--manual-peer`, or `--restart-after-progress`. [VERIFIED: scripts/verify.sh; .planning/phases/60-unattended-sync-loop-control/60-01-SUMMARY.md]

## Code Examples

Verified patterns from local sources:

### Resource Pressure Contract

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

This contract is the required shared status surface for active sync bounds. [VERIFIED: packages/open-bitcoin-node/src/status.rs:117; .planning/phases/61-resource-bounds-and-recovery-taxonomy/61-CONTEXT.md D-01]

### Runtime Projection Pattern

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

Use the same projection point to add recovery category so status consumers do not reclassify facts independently. [VERIFIED: packages/open-bitcoin-node/src/sync/runtime_state.rs:392; Bright Builds architecture standard]

### Support Allowlist Pattern

```rust
// Source: packages/open-bitcoin-cli/src/operator/support/live_smoke.rs
const RECOVERY_DIAGNOSIS_KEYS: &[&str] = &[
    "category",
    "maybeLastError",
    "maybeNoProgressCause",
    "maybePeerFailureReason",
    "maybeStorageRecoveryAction",
];
```

If support evidence changes, extend allowlisted keys intentionally and keep raw reports, daemon tails, endpoint tables, secrets, wallet material, and unbounded samples out. [VERIFIED: packages/open-bitcoin-cli/src/operator/support/live_smoke.rs:75; .planning/phases/61-resource-bounds-and-recovery-taxonomy/61-CONTEXT.md D-08]

### Live-Smoke Category Gap to Align

```typescript
// Source: scripts/run-live-mainnet-smoke.ts
type RecoveryDiagnosisCategory =
  | "peer_incompatibility"
  | "public_network_unreachable"
  | "invalid_peer_data"
  | "store_corruption"
  | "store_incompatibility"
  | "resource_exhaustion"
  | "intentional_cancellation";
```

The TypeScript live-smoke category set already overlaps Phase 61 but does not yet exactly match the locked Phase 61 taxonomy because Phase 61 adds clean/unclean shutdown and storage lock/backend failure categories. [VERIFIED: scripts/run-live-mainnet-smoke.ts:176; .planning/phases/61-resource-bounds-and-recovery-taxonomy/61-CONTEXT.md D-04]

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Renderer-local recovery strings or status warnings | Shared typed category plus human guidance | Phase 61 should make this additive contract change. [VERIFIED: .planning/phases/61-resource-bounds-and-recovery-taxonomy/61-CONTEXT.md D-04-D-07] | RR-04 becomes testable by label equality instead of string wording. [VERIFIED: .planning/REQUIREMENTS.md] |
| Public-network long-run proof | Deterministic scripted sync outcomes plus optional UAT docs | v1.3-v1.5 decisions keep public checks opt-in. [VERIFIED: .planning/STATE.md; .planning/REQUIREMENTS.md] | Phase completion stays hermetic while docs still show operator review commands. [VERIFIED: scripts/verify.sh; docs/operator/runtime-guide.md:610] |
| Raw live-smoke/support report copying | Allowlisted compact summaries with recursive redaction | Phase 59 established this pattern. [VERIFIED: .planning/phases/59-operator-evidence-threat-model-and-release-boundaries/59-VERIFICATION.md; packages/open-bitcoin-cli/src/operator/support/live_smoke.rs] | Phase 61 can expose category/resource facts without growing support evidence unboundedly. [VERIFIED: .planning/phases/61-resource-bounds-and-recovery-taxonomy/61-CONTEXT.md D-08] |
| Metrics/logs as unbounded history | Retention policies cap samples and log files | Earlier observability code already implements defaults. [VERIFIED: packages/open-bitcoin-node/src/metrics.rs:55; packages/open-bitcoin-node/src/logging.rs:115] | RR-01 should assert the existing caps during repeated sync cycles. [VERIFIED: .planning/REQUIREMENTS.md RR-01] |

**Deprecated/outdated:**

- Treating `result.restartResumeEvidence.recoveryDiagnosis.category` as the only taxonomy is insufficient for Phase 61 because it is TypeScript-only live-smoke evidence and lacks clean/unclean shutdown categories. [VERIFIED: scripts/run-live-mainnet-smoke.ts:176; .planning/phases/61-resource-bounds-and-recovery-taxonomy/61-CONTEXT.md D-04]
- Treating `sync.recovery_action` as a stable machine label is insufficient because it is a human message string. [VERIFIED: packages/open-bitcoin-node/src/status.rs:141; packages/open-bitcoin-node/src/storage.rs:91]

## Assumptions Log

All claims in this research were verified or cited; no user-confirmation assumptions are required. [VERIFIED: local codebase, GSD context, official standards sources]

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|

## Open Questions

1. **Exact Rust file placement for the new enum**
   - What we know: `status.rs` owns shared serializable sync status fields and already defines serde snake_case enums. [VERIFIED: packages/open-bitcoin-node/src/status.rs:87]
   - What's unclear: File-length and cohesion may make `status.rs` or `sync/types.rs` the better final location after implementation. [VERIFIED: Bright Builds code-shape standard]
   - Recommendation: Start in `status.rs` for the shared wire/status contract; split only if file-length or ownership pressure justifies it, and update parity breadcrumbs if a new first-party Rust file is added. [VERIFIED: AGENTS.md; .planning/phases/61-resource-bounds-and-recovery-taxonomy/61-CONTEXT.md]

2. **Whether RPC gets a dedicated category field in Phase 61**
   - What we know: RPC `getblockchaininfo` warnings currently include durable `last_error` and `recovery_action`. [VERIFIED: packages/open-bitcoin-rpc/src/dispatch/node.rs:122]
   - What's unclear: Phase 62 owns broader long-run truth-surface expansion, so Phase 61 should avoid broad RPC schema rewrites unless RR-04 cannot be satisfied without a narrow additive field. [VERIFIED: .planning/ROADMAP.md Phase 62; .planning/phases/61-resource-bounds-and-recovery-taxonomy/61-CONTEXT.md]
   - Recommendation: Prefer shared status/RPC sync-status category exposure first; only add `getblockchaininfo` warning wording if existing tests already cover it. [VERIFIED: .planning/phases/61-resource-bounds-and-recovery-taxonomy/61-CONTEXT.md D-07]

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| Rust toolchain | Rust implementation and tests | yes | `cargo 1.94.1`, `rustc 1.94.1` | None needed. [VERIFIED: command output] |
| rustfmt | Formatting | yes | `rustfmt 1.8.0-stable` | None needed. [VERIFIED: command output] |
| clippy | Linting | yes | `clippy 0.1.94` | None needed. [VERIFIED: command output] |
| cargo-llvm-cov | `bash scripts/verify.sh` coverage gate | yes | `cargo-llvm-cov 0.8.5` | None needed. [VERIFIED: command output; scripts/verify.sh] |
| Bun | TypeScript fixture/check scripts | yes | `1.3.9` | None needed. [VERIFIED: command output] |
| Bazel/Bazelisk | Bazel smoke build and UAT command forms | yes | Bazelisk `1.28.1`, Bazel `8.6.0` | None needed. [VERIFIED: command output] |
| jq | JSON fixture and parity checks | yes | `jq-1.7.1-apple` | Bun/serde_json can parse JSON if needed, but no fallback is required. [VERIFIED: command output] |
| git | status and optional research commit | yes | `2.53.0` | None needed. [VERIFIED: command output] |
| Public network | Optional operator UAT only | not required | not probed | Do not include in `bash scripts/verify.sh`; document opt-in commands only. [VERIFIED: .planning/REQUIREMENTS.md; scripts/verify.sh] |

**Missing dependencies with no fallback:**

- None. [VERIFIED: environment audit commands]

**Missing dependencies with fallback:**

- None. [VERIFIED: environment audit commands]

## Security Domain

Security enforcement is enabled because `.planning/config.json` does not set `security_enforcement: false`. [VERIFIED: .planning/config.json] OWASP ASVS 5.0.0 is the current stable version as of the official OWASP/GitHub sources, and ASVS recommends version-qualified IDs because identifiers can change. [CITED: https://github.com/OWASP/ASVS; CITED: https://owasp.org/www-project-application-security-verification-standard/]

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|-----------------|
| V2 Validation and Business Logic | yes | Document and enforce bounds for resource-demanding sync behavior through typed limits and deterministic tests. [CITED: https://raw.githubusercontent.com/OWASP/ASVS/v5.0.0/5.0/en/0x11-V2-Validation-and-Business-Logic.md; VERIFIED: docs/architecture/status-snapshot.md:79] |
| V6 Authentication | no new work | Phase 61 adds no auth surface; keep credential values out of support evidence. [VERIFIED: .planning/phases/61-resource-bounds-and-recovery-taxonomy/61-CONTEXT.md D-08; docs/architecture/config-precedence.md:27] |
| V7 Session Management | no | Phase 61 adds no web/browser session state. [VERIFIED: .planning/ROADMAP.md; .planning/REQUIREMENTS.md] |
| V8 Authorization | no new work | No new protected operator command or role boundary is introduced. [VERIFIED: .planning/phases/61-resource-bounds-and-recovery-taxonomy/61-CONTEXT.md] |
| V11 Cryptography | no new work | Phase 61 does not add cryptographic algorithms or key handling. [VERIFIED: .planning/phases/61-resource-bounds-and-recovery-taxonomy/61-CONTEXT.md; .planning/REQUIREMENTS.md] |
| V14 Data Protection | yes | Preserve support allowlist/redaction for local evidence and avoid secrets, wallet material, raw logs, raw reports, or endpoint tables. [CITED: https://raw.githubusercontent.com/OWASP/ASVS/v5.0.0/5.0/en/0x23-V14-Data-Protection.md; VERIFIED: packages/open-bitcoin-cli/src/operator/support/live_smoke.rs:279] |
| V15 Secure Coding and Architecture | yes | Use typed enums/domain helpers, document resource-demanding functionality, and avoid unbounded loops/queues. [CITED: https://raw.githubusercontent.com/OWASP/ASVS/v5.0.0/5.0/en/0x24-V15-Secure-Coding-and-Architecture.md; VERIFIED: Bright Builds architecture standard] |
| V16 Security Logging and Error Handling | yes | Keep logs structured, bounded, and non-secret; keep recovery/error categories actionable without leaking internal sensitive data. [CITED: https://raw.githubusercontent.com/OWASP/ASVS/v5.0.0/5.0/en/0x25-V16-Security-Logging-and-Error-Handling.md; VERIFIED: packages/open-bitcoin-node/src/logging.rs:115] |

### Known Threat Patterns for This Stack

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Public peer sends malformed, invalid, duplicate, disconnected, or non-extending data that looks like progress | Tampering, Denial of Service | Map peer failure reasons into `invalid_peer_data` without useful-progress credit. [VERIFIED: packages/open-bitcoin-node/src/sync/types.rs:234; .planning/phases/57-block-download-and-connect-progress/57-CONTEXT.md] |
| Durable store schema mismatch or corruption is hidden behind peer retry guidance | Tampering, Repudiation, Denial of Service | Storage-first recovery category precedence. [VERIFIED: .planning/phases/61-resource-bounds-and-recovery-taxonomy/61-CONTEXT.md D-05; packages/open-bitcoin-node/src/storage.rs:152] |
| Long unattended runs grow metrics/logs/support evidence without limit | Denial of Service | Use metric retention, log retention, compact support summaries, and deterministic long-run bound tests. [VERIFIED: packages/open-bitcoin-node/src/metrics.rs:55; packages/open-bitcoin-node/src/logging.rs:115; packages/open-bitcoin-cli/src/operator/support/live_smoke.rs:8] |
| Support artifacts leak credentials, daemon tails, endpoint tables, or wallet material | Information Disclosure | Preserve allowlisted summary extraction and recursive redaction. [VERIFIED: packages/open-bitcoin-cli/src/operator/support/live_smoke.rs:279; packages/open-bitcoin-cli/tests/operator_binary.rs:1057] |
| Default verification accidentally depends on public network availability | Repudiation, Denial of Service | Keep live smoke and long-run public checks as opt-in UAT outside `bash scripts/verify.sh`. [VERIFIED: .planning/REQUIREMENTS.md; scripts/verify.sh] |

## Sources

### Primary (HIGH confidence)

- `.planning/phases/61-resource-bounds-and-recovery-taxonomy/61-CONTEXT.md` - locked Phase 61 decisions, boundaries, surfaces, deferred scope. [VERIFIED: file read]
- `.planning/REQUIREMENTS.md` - RR-01, RR-02, RR-04 and default verification/public-network exclusions. [VERIFIED: file read]
- `.planning/ROADMAP.md` - Phase 61 success criteria and Phase 62/65 boundaries. [VERIFIED: file read]
- `.planning/STATE.md` - prior milestone decisions and deterministic verification posture. [VERIFIED: file read]
- `AGENTS.md`, `AGENTS.bright-builds.md`, `standards-overrides.md` - repo-local, Bright Builds, and override constraints. [VERIFIED: file read]
- `rust-toolchain.toml`, `packages/Cargo.toml`, `cargo metadata` - workspace/toolchain/dependency versions. [VERIFIED: file read; command output]
- `packages/open-bitcoin-node/src/status.rs` - `SyncResourcePressure`, lifecycle, progress signals, status fields. [VERIFIED: code grep/read]
- `packages/open-bitcoin-node/src/storage.rs` and `storage/fjall_store.rs` - storage recovery action, runtime metadata, recovery marker, schema/corruption/backend signals. [VERIFIED: code grep/read]
- `packages/open-bitcoin-node/src/sync/types.rs`, `sync/runtime_state.rs`, `sync/types/summary.rs`, `sync/types/projection.rs`, `sync/tests.rs` - sync config, peer failure, stop reason, runtime errors, projection, metrics/logs, deterministic fixtures. [VERIFIED: code grep/read]
- `packages/open-bitcoin-cli/src/operator/status/render.rs`, `dashboard/model.rs`, `support/live_smoke.rs`, `support/render.rs`, `packages/open-bitcoin-rpc/src/dispatch/node.rs` - operator truth renderers and support/RPC integration. [VERIFIED: code grep/read]
- `scripts/run-live-mainnet-smoke.ts`, `scripts/test-run-live-mainnet-smoke.sh` - live-smoke no-progress and recovery diagnosis categories/fixtures. [VERIFIED: code grep/read]
- `docs/operator/runtime-guide.md`, `docs/architecture/status-snapshot.md`, `docs/architecture/operator-observability.md`, `docs/architecture/config-precedence.md`, `docs/parity/threat-model-v1.4.md`, `docs/parity/release-readiness.md`, `docs/parity/index.json` - operator docs, status contract, observability bounds, config/redaction, threat/release roots. [VERIFIED: docs grep/read]
- Bright Builds canonical standards at commit `05f8d7a6c9c2e157ec4f922a05273e72dab97676`: architecture, code-shape, verification, testing, Rust, TypeScript/JavaScript, operability. [CITED: https://raw.githubusercontent.com/bright-builds-llc/bright-builds-rules/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/index.md]
- OWASP ASVS 5.0.0 official repository and versioned chapters. [CITED: https://github.com/OWASP/ASVS; CITED: https://raw.githubusercontent.com/OWASP/ASVS/v5.0.0/5.0/en/0x11-V2-Validation-and-Business-Logic.md]

### Secondary (MEDIUM confidence)

- OWASP project page for ASVS version-qualified requirement reference guidance. [CITED: https://owasp.org/www-project-application-security-verification-standard/]

### Tertiary (LOW confidence)

- None. [VERIFIED: no LOW-confidence sources used]

## Metadata

**Confidence breakdown:**

- Standard stack: HIGH - verified from local manifests, lockfile metadata, installed tool versions, and repo instructions. [VERIFIED: packages/Cargo.toml; cargo metadata; command output; AGENTS.md]
- Architecture: HIGH - Phase 61 is constrained by explicit user decisions and existing first-party code surfaces. [VERIFIED: .planning/phases/61-resource-bounds-and-recovery-taxonomy/61-CONTEXT.md; packages/open-bitcoin-node/src/status.rs; packages/open-bitcoin-node/src/sync/runtime_state.rs]
- Pitfalls: HIGH - pitfalls map directly to prior phase decisions, existing tests, and observed string-vs-typed gaps. [VERIFIED: .planning/phases/59-operator-evidence-threat-model-and-release-boundaries/59-VERIFICATION.md; scripts/run-live-mainnet-smoke.ts; packages/open-bitcoin-node/src/status.rs]

**Research date:** 2026-06-06
**Valid until:** 2026-07-06 for local first-party implementation guidance; re-check dependency/tool versions before implementation if the lockfile or toolchain changes. [VERIFIED: cargo metadata; rust-toolchain.toml]
