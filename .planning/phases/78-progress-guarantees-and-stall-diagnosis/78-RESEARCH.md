# Phase 78: Progress Guarantees and Stall Diagnosis - Research

**Researched:** 2026-06-16  
**Domain:** Durable sync status, soak checkpoint evidence, no-progress classification, operator status/report projections  
**Confidence:** HIGH

<user_constraints>

## User Constraints (from CONTEXT.md)

The following locked decisions, discretion areas, and deferred ideas are copied verbatim from `.planning/phases/78-progress-guarantees-and-stall-diagnosis/78-CONTEXT.md`. [CITED: .planning/phases/78-progress-guarantees-and-stall-diagnosis/78-CONTEXT.md]

### Locked Decisions

## Implementation Decisions

### Progress Credit Contract

- **D-01:** Treat validated, durably connected active-chain progress as the
  only normal source of soak progress credit. Header downloads, peer messages,
  queued block requests, in-flight work, report generation, and retries may be
  evidence, but they must not advance the credited progress watermark by
  themselves.
- **D-02:** Explicit stay-current evidence may also count as useful work when
  the node is already at the best-known validated tip. That evidence must be
  derived from the existing `StayCurrentStatus`, `BestKnownTipStatus`, peer-tip
  agreement, freshness threshold, and durable status projection rather than a
  renderer-local string.
- **D-03:** Preserve Phase 70's active-chain rule: a better header branch is
  not credited as active-chain progress until its blocks are available,
  consensus-validated, connected, and durably persisted. Branch competition
  should continue to report awaiting bodies or reorg progress without claiming
  the replacement active tip early.
- **D-04:** Store progress-credit evidence as typed shared status and soak
  checkpoint fields. A checkpoint should be able to show the credited
  validated height/hash/work, the evidence kind that justified credit, the
  source timestamp, and why non-credit activity was rejected when relevant.

### Stall Diagnosis Evidence

- **D-05:** Extend the existing shared status contracts instead of building a
  soak-only stall model. `SyncProgress`, `SyncProgressSignal`,
  `last_successful_progress_unix_seconds`, `StayCurrentStatus`,
  `NoProgressDiagnosis`, `SyncResourcePressure`, `RecoveryEvidenceSnapshot`,
  peer outcomes, and reconcile progress are the starting contract.
- **D-06:** Phase 78 should add or derive explicit fields for expected progress
  window, no-progress threshold, last useful work, last peer contribution,
  stalled subsystem, and diagnosis confidence/evidence basis. Missing evidence
  must remain an unavailable field with a reason, not an omitted value.
- **D-07:** Diagnosis should distinguish public-network reachability,
  incompatible peers, slow or stalled peers, peer failures exhausted, stale
  in-flight cleanup, branch competition awaiting bodies, stalled validation,
  storage/resource pressure, current-at-tip waiting, operator stop, and local
  shutdown. Reuse existing peer failure and recovery categories where they are
  precise enough; add narrowly scoped typed variants or evidence fields only
  when the current labels cannot express PROG-03 truthfully.
- **D-08:** Storage/resource pressure and recovery evidence outrank peer retry
  advice. If Phase 76/77 evidence says the selected datadir is blocked,
  diagnosis should point to storage/resource action rather than telling the
  operator to wait for or rotate peers.

### Soak Ledger And Operator Surfaces

- **D-09:** Carry progress-guarantee and stall fields through the Phase 75
  datadir-owned soak ledger checkpoint and report projection. Reports remain
  projections; the ledger and shared status are the durable source of truth.
- **D-10:** CLI status, dashboard status, RPC status, soak reports, live-smoke
  summaries, metrics/log summaries, and support evidence should consume the
  same typed progress/stall contract. Phase 78 may update the surfaces needed
  to prove PROG-01 through PROG-04, while Phase 79 owns the broader
  "what happened" support-bundle narrative.
- **D-11:** Operator wording should stay quiet and actionable: identify the
  stalled subsystem, the evidence basis, and the next action. Avoid vague
  "sync failed" text, false "making progress" language, and production-node
  readiness claims.
- **D-12:** Local shutdown and operator stop should be separate from network,
  peer, validation, and storage stalls. A clean local stop should not be
  reported as public-network failure or validation stall.

### Deterministic Verification

- **D-13:** PROG-04 must be proven with deterministic Rust tests for the core
  decision logic: false-progress prevention, stale in-flight cleanup, peer
  rotation/backoff, at-tip waiting, validation stalls, and storage/resource
  precedence. Use synthetic chain and scripted peer/status fixtures rather than
  public peers or wall-clock multi-day sleeps.
- **D-14:** Keep pure classifiers and progress-credit decisions easy to unit
  test. Tests should focus on one concern, use Arrange/Act/Assert comments when
  setup is non-trivial, and avoid driving behavior through renderer strings.
- **D-15:** Add a focused Bun checker only for docs, parity roots, phase
  artifact anchors, required field names, and default-verification exclusions
  that Rust tests cannot prove. Keep the checker public-network-free,
  service-manager-free, and short-running.
- **D-16:** If new first-party Rust source or test files are added under
  `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests`, update
  `docs/parity/source-breadcrumbs.json` and keep
  `scripts/check-parity-breadcrumbs.ts` green.

### the agent's Discretion

- The planner may split Phase 78 across progress-credit domain/status types,
  no-progress/stall classifier extensions, soak checkpoint/report projection,
  operator surface rendering, deterministic fixtures, docs, and checker/parity
  closeout.
- The executor may add compact typed structs/enums for progress-credit evidence,
  stalled subsystem, threshold/window evidence, and no-progress basis when they
  make illegal states unrepresentable.
- The executor may preserve existing `NoProgressDiagnosis` labels and add
  adjacent evidence fields when that is simpler and less disruptive than
  expanding enum labels.

### Deferred Ideas (OUT OF SCOPE)

- Phase 79 owns the richer redacted "what happened" support-bundle narrative,
  timeline reconstruction, and cross-surface forensic story. Phase 78 should
  expose the typed facts Phase 79 will later narrate.
- Phase 80 owns opt-in multi-day soak UAT command closeout, final v1.7 release
  boundary wording, and audit of public-network exclusions.
- Future production-node readiness, inbound serving, relay, production-wallet,
  migration-apply, packaging, GUI, hosted-dashboard, scheduled public soak
  monitors, and signed comparable soak artifacts remain outside Phase 78.

</user_constraints>

<phase_requirements>

## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| PROG-01 | Operator can trust that reported soak progress is credited only after validated, durably connected active-chain progress or explicit stay-current evidence. [CITED: .planning/REQUIREMENTS.md] | Use a typed `ProgressCreditEvidence` status/checkpoint contract sourced from `SyncProgress.validated_active_chain_*` or durable stay-current evidence, not from header/block message counters. [VERIFIED: packages/open-bitcoin-node/src/status.rs; packages/open-bitcoin-node/src/sync/types/summary.rs; packages/open-bitcoin-node/src/sync/runtime_state.rs] |
| PROG-02 | Operator can see expected progress windows, last useful work, last peer contribution, stalled subsystem, and no-progress threshold evidence. [CITED: .planning/REQUIREMENTS.md] | Add shared `FieldAvailability` fields for threshold/window, last useful work, last peer contribution, stalled subsystem, confidence, and evidence basis. [VERIFIED: packages/open-bitcoin-node/src/status.rs; packages/open-bitcoin-node/src/sync/progress.rs] |
| PROG-03 | Operator can distinguish public-network reachability issues, incompatible peers, slow peers, stalled validation, storage pressure, at-tip waiting, and local shutdown. [CITED: .planning/REQUIREMENTS.md] | Extend the pure no-progress path with adjacent typed evidence and precedence rules using `PeerFailureReason`, `SyncStopReason`, `SyncRecoveryCategory`, `StayCurrentStatus`, `SyncReconcileProgressStatus`, and recovery/resource evidence. [VERIFIED: packages/open-bitcoin-node/src/sync/progress.rs; packages/open-bitcoin-node/src/sync/types/recovery.rs; packages/open-bitcoin-node/src/sync/types.rs] |
| PROG-04 | Contributor can verify progress-guarantee logic with deterministic tests for false progress, stale in-flight work, peer rotation, at-tip waiting, and validation stalls. [CITED: .planning/REQUIREMENTS.md] | Existing scripted sync tests already cover Phase 69/70/71 fixtures and should be extended with Phase 78 cases plus a focused Bun checker. [VERIFIED: packages/open-bitcoin-node/src/sync/tests.rs; packages/open-bitcoin-node/src/sync/tests/soak.rs; scripts/check-phase75-soak-runner.ts; scripts/check-phase77-corruption-lock-recovery.ts] |

</phase_requirements>

## Summary

Phase 78 should be implemented as an additive shared-status contract, not as a new soak-only subsystem. [CITED: .planning/phases/78-progress-guarantees-and-stall-diagnosis/78-CONTEXT.md] The repository already has `FieldAvailability<T>`, `SyncStatus`, `SyncProgress`, `StayCurrentStatus`, `NoProgressDiagnosis`, `SyncResourcePressure`, top-level `RecoveryEvidenceSnapshot`, and soak checkpoint/report projection code that can carry the new evidence with minimal architectural churn. [VERIFIED: packages/open-bitcoin-node/src/status.rs; packages/open-bitcoin-cli/src/operator/soak/ledger.rs; packages/open-bitcoin-cli/src/operator/soak/runtime/helpers.rs]

The main implementation risk is false progress credit. `SyncRunSummary::last_successful_progress_unix_seconds()` currently treats accepted headers or accepted block responses as successful progress, and `tip::classify_stay_current()` currently receives `made_useful_progress` from `summary.headers_received > 0 || summary.blocks_received > 0`. [VERIFIED: packages/open-bitcoin-node/src/sync/types/summary.rs; packages/open-bitcoin-node/src/sync/runtime_state.rs] Phase 78 needs a stricter credited-progress source based on durably connected active-chain height/hash/work or explicit current-at-tip evidence. [CITED: .planning/phases/78-progress-guarantees-and-stall-diagnosis/78-CONTEXT.md]

The recommended plan is to add compact typed structs/enums in `status.rs` or a small `status/progress_guarantee.rs` child module, compute them in `sync/progress.rs` and `sync/runtime_state.rs`, pass them through `DurableSyncState`, add fields to `SoakCheckpointStatus`, and update CLI/dashboard/RPC/support/live-smoke projections to render those fields without reclassification. [VERIFIED: packages/open-bitcoin-node/src/status.rs; packages/open-bitcoin-node/src/sync/progress.rs; packages/open-bitcoin-node/src/sync/runtime_state.rs; packages/open-bitcoin-cli/src/operator/sync_truth_render.rs; packages/open-bitcoin-cli/src/operator/dashboard/model.rs]

**Primary recommendation:** Add typed shared `progress_credit` and `stall_diagnosis` evidence to `SyncStatus`, derive it from durable active-chain or stay-current facts, carry it into soak checkpoints/reports, and prove the classifier with deterministic Rust tests plus a short Phase 78 Bun checker. [VERIFIED: codebase inspection; CITED: .planning/phases/78-progress-guarantees-and-stall-diagnosis/78-CONTEXT.md]

## Project Constraints (from AGENTS.md)

- Read and follow `AGENTS.md`, `AGENTS.bright-builds.md`, `standards-overrides.md`, and relevant standards pages before planning or implementation. [CITED: AGENTS.md; AGENTS.bright-builds.md; standards/index.md]
- Preserve externally observable Bitcoin Knots `29.3.knots20260210` behavior for in-scope surfaces and keep parity evidence auditable through `docs/parity/`. [CITED: AGENTS.md; .planning/PROJECT.md]
- Keep functional core / imperative shell boundaries; pure Bitcoin/domain decisions stay out of direct I/O, network, storage, terminal, RPC, and service-manager adapters. [CITED: AGENTS.md; standards/core/architecture.md; .planning/ARCHITECTURE.md]
- Keep dependencies minimal and do not use existing Rust Bitcoin libraries in the production path. [CITED: AGENTS.md; .planning/PROJECT.md]
- Use Rust `1.94.1` from `rust-toolchain.toml`; use `packages/Cargo.toml` as the Cargo workspace root. [CITED: AGENTS.md; rust-toolchain.toml; packages/Cargo.toml]
- Use Bun as the runtime for substantial repo-owned TypeScript automation; this repo has no `package.json`, so do not add `bun install` setup. [CITED: AGENTS.md; .planning/STACK.md]
- Use `bash scripts/verify.sh` as the repo-native verification contract for first-party code. [CITED: AGENTS.md; scripts/verify.sh]
- Public-network sync, real service-manager checks, and multi-day wall-clock soak runs must stay opt-in and out of default `scripts/verify.sh`. [CITED: .planning/REQUIREMENTS.md; scripts/verify.sh; docs/operator/runtime-guide.md]
- New first-party Rust source or test files under `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests` require parity breadcrumbs in `docs/parity/source-breadcrumbs.json`. [CITED: AGENTS.md; scripts/check-parity-breadcrumbs.ts; docs/parity/source-breadcrumbs.json]
- Unit tests should test one concern and use Arrange/Act/Assert comments when setup is non-trivial. [CITED: standards/core/testing.md; standards/languages/rust.md]
- Internal nullable or optional values should use `maybe_` naming unless an external contract forces a stable name. [CITED: standards/core/code-shape.md; standards/languages/rust.md; standards/languages/typescript-javascript.md]

## Standard Stack

### Core

| Library / Component | Version | Purpose | Why Standard |
|---------------------|---------|---------|--------------|
| Rust / Cargo workspace | `1.94.1`, Rust 2024 edition [CITED: rust-toolchain.toml; packages/Cargo.toml] | First-party implementation and deterministic unit tests. [VERIFIED: packages/Cargo.toml] | Pinned by repo guidance and used by all `open-bitcoin-*` crates. [CITED: AGENTS.md] |
| `open-bitcoin-node` status/sync modules | workspace `0.1.0` [CITED: packages/Cargo.toml] | Shared status contracts, durable sync projection, progress/no-progress classifiers. [VERIFIED: packages/open-bitcoin-node/src/status.rs; packages/open-bitcoin-node/src/sync/progress.rs; packages/open-bitcoin-node/src/sync/runtime_state.rs] | Existing owner of `SyncStatus`, `DurableSyncState`, `SyncRunSummary`, and sync classifiers. [VERIFIED: packages/open-bitcoin-node/src/status.rs; packages/open-bitcoin-node/src/sync/types.rs] |
| `open-bitcoin-cli` soak/operator modules | workspace `0.1.0` [CITED: packages/Cargo.toml] | Soak ledger/checkpoint/report, status rendering, dashboard/support projections. [VERIFIED: packages/open-bitcoin-cli/src/operator/soak/ledger.rs; packages/open-bitcoin-cli/src/operator/soak/report.rs; packages/open-bitcoin-cli/src/operator/sync_truth_render.rs] | Existing owner of operator-facing projections; reports are projections over shared durable status. [CITED: docs/architecture/operator-observability.md] |
| Fjall | `3.1.4` [VERIFIED: packages/open-bitcoin-node/Cargo.toml] | Durable runtime metadata and node store. [VERIFIED: packages/open-bitcoin-node/src/storage.rs; packages/open-bitcoin-node/src/storage/fjall_store.rs] | Existing storage engine for `RuntimeMetadata` and `DurableSyncState`. [VERIFIED: packages/open-bitcoin-node/src/storage.rs] |
| serde / serde_json | `serde 1.0.228`, `serde_json 1.0.149` [VERIFIED: packages/open-bitcoin-node/Cargo.toml; packages/open-bitcoin-cli/Cargo.toml] | Stable status, ledger, report, and RPC JSON shapes. [VERIFIED: packages/open-bitcoin-node/src/status.rs; packages/open-bitcoin-cli/src/operator/soak/ledger.rs] | Existing serialization path for shared status and soak events. [VERIFIED: codebase rg] |

### Supporting

| Library / Tool | Version | Purpose | When to Use |
|----------------|---------|---------|-------------|
| Bun | `1.3.9` [VERIFIED: `.bun-version`; `bun --version`] | Deterministic TypeScript checker and checker test. [VERIFIED: scripts/check-phase75-soak-runner.ts; scripts/check-phase77-corruption-lock-recovery.ts] | Use for docs/parity/field-name/default-verification guardrails that Rust tests cannot prove. [CITED: .planning/phases/78-progress-guarantees-and-stall-diagnosis/78-CONTEXT.md] |
| Bazel / Bazelisk-compatible `bazel` | `8.6.0` available locally [VERIFIED: `bazel --version`] | Top-level smoke build and UAT command parity. [VERIFIED: scripts/verify.sh; AGENTS.md] | Do not add Phase 78-specific Bazel logic unless new Rust sources need build target visibility. [VERIFIED: scripts/verify.sh] |
| `cargo-llvm-cov` | `0.8.5` available locally [VERIFIED: `cargo llvm-cov --version`] | Existing pure-core coverage gate in `scripts/verify.sh`. [VERIFIED: scripts/verify.sh] | No direct Phase 78 API work expected; full verify depends on it. [VERIFIED: scripts/verify.sh] |
| `thiserror` | `2.0.12` in CLI crate [VERIFIED: packages/open-bitcoin-cli/Cargo.toml] | Existing CLI error modeling. [VERIFIED: packages/open-bitcoin-cli/Cargo.toml] | Use only if Phase 78 adds CLI-facing error types; status evidence should not need new errors. [VERIFIED: codebase inspection] |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Add shared status fields | Renderer-local strings in status/dashboard/report | Rejected by Phase 78 D-02, D-05, and D-10 because renderer strings would let surfaces disagree and would not be durable evidence. [CITED: .planning/phases/78-progress-guarantees-and-stall-diagnosis/78-CONTEXT.md] |
| Extend existing `NoProgressDiagnosis` with adjacent evidence fields | Replace all no-progress labels with a new enum | Adjacent fields minimize churn while still satisfying PROG-02/PROG-03 when existing labels are coarse. [CITED: .planning/phases/78-progress-guarantees-and-stall-diagnosis/78-CONTEXT.md; VERIFIED: packages/open-bitcoin-node/src/status.rs] |
| Derive progress credit from active-chain or stay-current evidence | Use `headers_received`, `blocks_received`, in-flight requests, or report generation | Header/block counters are useful diagnostics, but they are explicitly not progress credit under Phase 78. [CITED: .planning/phases/78-progress-guarantees-and-stall-diagnosis/78-CONTEXT.md; VERIFIED: packages/open-bitcoin-node/src/sync/types/summary.rs] |
| Deterministic synthetic Rust tests | Public peer or multi-day wall-clock verification | Public network and multi-day checks are out of default verification. [CITED: .planning/REQUIREMENTS.md; VERIFIED: scripts/verify.sh] |

**Installation:** No new dependencies are recommended. [VERIFIED: codebase inspection; CITED: AGENTS.md dependency policy]

**Version verification:** Existing versions were verified from `packages/Cargo.toml`, `rust-toolchain.toml`, `.bun-version`, and local command probes rather than external registries because Phase 78 should not add third-party packages. [VERIFIED: packages/Cargo.toml; rust-toolchain.toml; .bun-version; local command probes]

## Architecture Patterns

### Recommended Project Structure

```text
packages/open-bitcoin-node/src/
├── status.rs                         # add shared progress/stall status fields or re-export a child module
├── status/
│   ├── recovery.rs                   # existing recovery evidence contract
│   └── resource_bounds.rs            # existing resource-bound contract
├── sync/progress.rs                  # pure progress-credit and stall classifier logic
├── sync/runtime_state.rs             # durable projection into SyncStatus/DurableSyncState
├── sync/types.rs                     # summary/stop reason types if a compact summary field is needed
└── sync/tests.rs                     # deterministic PROG-04 cases using existing fixtures

packages/open-bitcoin-cli/src/operator/
├── sync_truth_render.rs              # shared human text for new typed fields
├── dashboard/model.rs                # dashboard rows consuming shared renderers
├── soak/ledger.rs                    # checkpoint schema fields
├── soak/runtime/helpers.rs           # checkpoint extraction from OpenBitcoinStatusSnapshot
├── soak/report.rs                    # markdown/json projection
└── support/evidence.rs               # compact support summary facts for Phase 79

scripts/
└── check-phase78-progress-stall.ts   # docs/parity/field/default-verification checker
```

This layout follows existing module ownership and keeps core decisions in node status/sync code while CLI/dashboard/report layers remain projections. [VERIFIED: packages/open-bitcoin-node/src/status.rs; packages/open-bitcoin-node/src/sync/progress.rs; packages/open-bitcoin-cli/src/operator/sync_truth_render.rs; standards/core/architecture.md]

### Pattern 1: Typed Progress Credit Evidence

**What:** Add a compact shared status type that states why a checkpoint gets progress credit, what validated height/hash/work was credited, when the source was observed, and which non-credit activity was rejected. [CITED: .planning/phases/78-progress-guarantees-and-stall-diagnosis/78-CONTEXT.md]

**When to use:** Use for every status snapshot and soak checkpoint that claims useful progress during Phase 78. [CITED: .planning/phases/78-progress-guarantees-and-stall-diagnosis/78-CONTEXT.md]

**Example:**

```rust
// Source pattern: packages/open-bitcoin-node/src/status.rs uses typed serde status contracts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProgressCreditEvidenceKind {
    ValidatedDurableActiveChain,
    CurrentAtBestKnownTip,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProgressCreditEvidence {
    pub kind: ProgressCreditEvidenceKind,
    pub credited_validated_active_chain_height: u64,
    pub credited_validated_active_chain_hash: String,
    pub credited_validated_active_chain_work: String,
    pub source_unix_seconds: u64,
    pub rejected_activity: Vec<RejectedProgressActivity>,
}
```

The exact field names should be finalized during planning, but the invariant is locked: credit requires `validated_active_chain_height/hash/work` or `current_at_best_known_tip` evidence. [CITED: .planning/phases/78-progress-guarantees-and-stall-diagnosis/78-CONTEXT.md; VERIFIED: packages/open-bitcoin-node/src/status.rs]

### Pattern 2: Pure Stall Diagnosis Classifier

**What:** Keep `classify_no_progress` as the central pure decision path or add a neighboring pure classifier that returns richer evidence while preserving existing `NoProgressDiagnosis` labels. [VERIFIED: packages/open-bitcoin-node/src/sync/progress.rs; CITED: .planning/phases/78-progress-guarantees-and-stall-diagnosis/78-CONTEXT.md]

**When to use:** Use when deriving PROG-02/PROG-03 evidence from status facts, peer outcomes, recovery evidence, resource bounds, stop reasons, reconcile progress, and threshold/window data. [VERIFIED: packages/open-bitcoin-node/src/sync/progress.rs; packages/open-bitcoin-node/src/sync/types/recovery.rs]

**Example:**

```rust
// Source pattern: packages/open-bitcoin-node/src/sync/progress.rs::classify_no_progress.
pub struct StallDiagnosisInput<'a> {
    pub maybe_progress_credit: Option<&'a ProgressCreditEvidence>,
    pub maybe_no_progress_diagnosis: Option<NoProgressDiagnosis>,
    pub maybe_recovery_category: Option<SyncRecoveryCategory>,
    pub maybe_stop_reason: Option<SyncStopReason>,
    pub maybe_reconcile_progress: Option<&'a SyncReconcileProgress>,
    pub resource_pressure: &'a FieldAvailability<SyncResourcePressure>,
    pub peer_outcomes: &'a [PeerSyncOutcome],
}
```

Precedence should stay storage/resource and recovery first, then local shutdown/operator stop, current-at-tip, branch/reorg, stale in-flight, validation, peer incompatibility, public-network reachability, slow/stalled peers, and generic awaiting headers. [CITED: .planning/phases/78-progress-guarantees-and-stall-diagnosis/78-CONTEXT.md; VERIFIED: packages/open-bitcoin-node/src/sync/progress.rs]

### Pattern 3: Ledger Projection From Shared Status

**What:** Extend `SoakCheckpointStatus` with optional scalar/string fields copied from shared status evidence, then render reports from the ledger. [VERIFIED: packages/open-bitcoin-cli/src/operator/soak/ledger.rs; packages/open-bitcoin-cli/src/operator/soak/runtime/helpers.rs; packages/open-bitcoin-cli/src/operator/soak/report.rs]

**When to use:** Use for durable soak checkpoint fields required by PROG-01 through PROG-03. [CITED: .planning/phases/78-progress-guarantees-and-stall-diagnosis/78-CONTEXT.md]

**Example:**

```rust
// Source pattern: packages/open-bitcoin-cli/src/operator/soak/ledger.rs::SoakCheckpointStatus.
pub(crate) struct SoakCheckpointStatus {
    pub(crate) maybe_progress_credit_kind_label: Option<String>,
    pub(crate) maybe_progress_credit_height: Option<u64>,
    pub(crate) maybe_progress_credit_hash: Option<String>,
    pub(crate) maybe_stalled_subsystem_label: Option<String>,
    pub(crate) maybe_no_progress_threshold_seconds: Option<u64>,
    pub(crate) maybe_last_useful_work_unix_seconds: Option<u64>,
}
```

Reports must remain projections over `<datadir>/soak/run-index.json` and `events.jsonl`; do not create a parallel report state store. [CITED: docs/architecture/operator-observability.md; docs/operator/runtime-guide.md]

### Anti-Patterns to Avoid

- **Crediting accepted headers:** Accepted headers can update best-known tip evidence but must not move the credited progress watermark by themselves. [CITED: .planning/phases/78-progress-guarantees-and-stall-diagnosis/78-CONTEXT.md; VERIFIED: packages/open-bitcoin-node/src/sync/types/summary.rs]
- **Crediting block responses before durable connect:** `blocks_received` and `PeerProgress::record_accepted_block()` are currently peer contribution signals, not proof that active-chain progress was durably connected. [VERIFIED: packages/open-bitcoin-node/src/sync/progress.rs; packages/open-bitcoin-node/src/sync/block_response.rs]
- **Renderer-local diagnosis:** CLI/dashboard/report/support layers must consume typed status fields instead of parsing prose or duplicating classifier logic. [CITED: docs/architecture/status-snapshot.md; VERIFIED: packages/open-bitcoin-cli/src/operator/sync_truth_render.rs]
- **Flattening local stop into network failure:** `SyncStopReason::OperatorPaused` and `ShutdownRequested` currently map to operator cancellation, and Phase 78 requires local shutdown to stay distinct. [VERIFIED: packages/open-bitcoin-node/src/sync/types/recovery.rs; CITED: .planning/phases/78-progress-guarantees-and-stall-diagnosis/78-CONTEXT.md]
- **Adding public-network gates to default verification:** `scripts/verify.sh` currently runs deterministic checkers/tests and does not invoke live-smoke public-network scripts. [VERIFIED: scripts/verify.sh]

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Shared availability and unavailable reasons | Ad hoc `Option` fields with missing-value semantics | `FieldAvailability<T>` [VERIFIED: packages/open-bitcoin-node/src/status.rs] | Existing JSON/human surfaces preserve unavailable reasons; Phase 78 D-06 requires unavailable evidence to remain explicit. [CITED: .planning/phases/78-progress-guarantees-and-stall-diagnosis/78-CONTEXT.md] |
| Progress/no-progress classification | A soak-only or renderer-only classifier | `sync/progress.rs` pure classifier path plus adjacent evidence fields [VERIFIED: packages/open-bitcoin-node/src/sync/progress.rs] | Existing `classify_no_progress` already encodes storage/resource precedence, at-tip, branch, stale in-flight, and peer cases. [VERIFIED: packages/open-bitcoin-node/src/sync/progress.rs] |
| Durable progress state | A new ledger or report-side state store | `RuntimeMetadata.maybe_sync_state` and datadir-owned soak ledger [VERIFIED: packages/open-bitcoin-node/src/storage.rs; packages/open-bitcoin-node/src/status.rs; packages/open-bitcoin-cli/src/operator/soak/ledger.rs] | Shared durable status and soak events already form the source of truth. [CITED: docs/architecture/operator-observability.md] |
| Resource/storage precedence | Peer retry advice from string matching | `SyncRecoveryCategory`, top-level `recovery_evidence`, `resource_bounds`, and `SyncResourcePressure` [VERIFIED: packages/open-bitcoin-node/src/status.rs; packages/open-bitcoin-node/src/recovery.rs; packages/open-bitcoin-node/src/status/resource_bounds.rs] | Phase 76/77 evidence must outrank peer retry advice. [CITED: .planning/phases/78-progress-guarantees-and-stall-diagnosis/78-CONTEXT.md] |
| Docs/parity guardrails | Manual checklist only | Bun checker following Phase 75/76/77 pattern [VERIFIED: scripts/check-phase75-soak-runner.ts; scripts/check-phase76-resource-bounds.ts; scripts/check-phase77-corruption-lock-recovery.ts] | Existing checkers guard field anchors, parity roots, and default verification exclusions. [VERIFIED: scripts/verify.sh] |

**Key insight:** The existing architecture already separates durable state, pure classification, and operator projection; Phase 78 should tighten the evidence contract rather than introduce new control flow or network behavior. [VERIFIED: packages/open-bitcoin-node/src/sync/progress.rs; packages/open-bitcoin-node/src/sync/runtime_state.rs; docs/architecture/status-snapshot.md]

## Common Pitfalls

### Pitfall 1: Header Progress Looks Like Useful Work

**What goes wrong:** A soak checkpoint advances because a peer delivered headers even though the active chain did not connect and persist new blocks. [CITED: .planning/phases/78-progress-guarantees-and-stall-diagnosis/78-CONTEXT.md]  
**Why it happens:** `SyncRunSummary::last_successful_progress_unix_seconds()` currently treats either `headers_received > 0` or `blocks_received > 0` as successful progress. [VERIFIED: packages/open-bitcoin-node/src/sync/types/summary.rs]  
**How to avoid:** Add a separate credited-progress field sourced from durable active-chain evidence or stay-current evidence; leave header activity as diagnostic peer contribution only. [CITED: .planning/phases/78-progress-guarantees-and-stall-diagnosis/78-CONTEXT.md]  
**Warning signs:** Tests that pass by asserting `headers_received`, `messages_processed`, or `latest_stop_reason` without checking `validated_active_chain_height/hash/work`. [VERIFIED: packages/open-bitcoin-node/src/status.rs; packages/open-bitcoin-node/src/sync/types/summary.rs]

### Pitfall 2: Block Download Is Not Durable Active-Chain Connect

**What goes wrong:** A requested block response is counted as soak progress even if it is downloaded-only, duplicate, disconnected, non-extending, invalid, or not part of the best chain. [VERIFIED: packages/open-bitcoin-node/src/sync/block_response.rs; packages/open-bitcoin-node/src/sync/block_reconcile.rs]  
**Why it happens:** Peer contribution counters and durable connect state are adjacent but distinct. [VERIFIED: packages/open-bitcoin-node/src/sync/progress.rs; packages/open-bitcoin-node/src/sync/types/summary.rs]  
**How to avoid:** Credit only after `connected_block_height` and `validated_active_chain_height` move with corresponding hash/work from runtime projection. [VERIFIED: packages/open-bitcoin-node/src/sync/runtime_state.rs]  
**Warning signs:** Soak report fields use `blocks_received` or `downloaded_block_height` as the progress watermark. [VERIFIED: packages/open-bitcoin-cli/src/operator/soak/runtime/helpers.rs]

### Pitfall 3: Storage Pressure Gets Hidden Behind Peer Advice

**What goes wrong:** An operator is told to rotate peers even though storage/recovery evidence says the selected datadir is blocked. [CITED: .planning/phases/78-progress-guarantees-and-stall-diagnosis/78-CONTEXT.md]  
**Why it happens:** Peer outcomes often provide the latest visible event, but recovery/resource evidence may be the actual blocker. [VERIFIED: packages/open-bitcoin-node/src/sync/progress.rs; packages/open-bitcoin-node/src/sync/types/recovery.rs]  
**How to avoid:** Preserve the existing `StorageOrResourceBlocked` first branch and add Phase 78 evidence fields that expose precedence basis. [VERIFIED: packages/open-bitcoin-node/src/sync/progress.rs]  
**Warning signs:** A classifier checks `PeerFailureReason::RetryBackoff`, `PeerSyncState::Stalled`, or `SyncProgressSignal::PeerFailures` before recovery/resource category checks. [VERIFIED: packages/open-bitcoin-node/src/sync/progress.rs]

### Pitfall 4: At-Tip Waiting Is Misreported As Stall

**What goes wrong:** A current node is diagnosed as stalled because no new messages arrived during a soak checkpoint. [CITED: .planning/phases/78-progress-guarantees-and-stall-diagnosis/78-CONTEXT.md]  
**Why it happens:** No message progress and no useful work are different when connected active-chain evidence already matches a fresh best-known peer tip. [VERIFIED: packages/open-bitcoin-node/src/sync/tip.rs]  
**How to avoid:** Credit explicit stay-current evidence only when `StayCurrentStatus::CurrentAtBestKnownTip`, `BestKnownTipStatus`, freshness, peer agreement, and connected tip match. [VERIFIED: packages/open-bitcoin-node/src/sync/tip.rs; docs/architecture/status-snapshot.md]  
**Warning signs:** Tests assert a "no progress" stop reason at tip without checking `stay_current` and best-known tip evidence. [VERIFIED: packages/open-bitcoin-node/src/sync/tests.rs]

### Pitfall 5: Expanding Network Behavior While Adding Diagnosis

**What goes wrong:** Phase 78 accidentally changes peer rotation, request scheduling, or public-network behavior while trying to diagnose stalls. [CITED: user task; .planning/phases/78-progress-guarantees-and-stall-diagnosis/78-CONTEXT.md]  
**Why it happens:** Stall diagnosis touches peer outcomes and in-flight state, which are close to sync control flow. [VERIFIED: packages/open-bitcoin-node/src/sync.rs; packages/open-bitcoin-node/src/sync/progress.rs]  
**How to avoid:** Keep changes in status/classifier/projection fields unless a deterministic test proves a false status cannot be fixed without a narrow summary fact. [CITED: standards/core/architecture.md; VERIFIED: packages/open-bitcoin-node/src/sync/progress.rs]  
**Warning signs:** Phase 78 plans alter outbound peer counts, request limits, timeouts, peer discovery, or block connect behavior. [CITED: .planning/phases/78-progress-guarantees-and-stall-diagnosis/78-CONTEXT.md]

## Code Examples

### Credited Progress Should Not Use Header/Block Message Counters

```rust
// Source: packages/open-bitcoin-node/src/sync/types/summary.rs
// Existing behavior to avoid using as the Phase 78 credit watermark.
pub(crate) fn last_successful_progress_unix_seconds(&self) -> Option<u64> {
    self.peer_outcomes
        .iter()
        .rev()
        .find(|outcome| {
            outcome.contribution.headers_received > 0
                || outcome.contribution.blocks_received > 0
        })
        .and_then(|outcome| outcome.maybe_last_activity_unix_seconds)
}
```

Use a new credited-work helper that requires durable active-chain fields or current-at-tip evidence. [CITED: .planning/phases/78-progress-guarantees-and-stall-diagnosis/78-CONTEXT.md]

```rust
// Source pattern: packages/open-bitcoin-node/src/sync/runtime_state.rs
fn maybe_credit_validated_active_chain(
    progress: &SyncProgress,
    source_unix_seconds: u64,
) -> Option<ProgressCreditEvidence> {
    let (Some(hash), Some(work)) = (
        progress.maybe_validated_active_chain_hash.clone(),
        progress.maybe_validated_active_chain_work.clone(),
    ) else {
        return None;
    };

    Some(ProgressCreditEvidence {
        kind: ProgressCreditEvidenceKind::ValidatedDurableActiveChain,
        credited_validated_active_chain_height: progress.validated_active_chain_height,
        credited_validated_active_chain_hash: hash,
        credited_validated_active_chain_work: work,
        source_unix_seconds,
        rejected_activity: Vec::new(),
    })
}
```

### Preserve Storage/Resource Precedence

```rust
// Source: packages/open-bitcoin-node/src/sync/progress.rs
pub(super) fn classify_no_progress(input: &NoProgressInput<'_>) -> NoProgressDiagnosis {
    if input
        .recovery_category
        .is_some_and(is_storage_or_resource_blocker)
    {
        return NoProgressDiagnosis::StorageOrResourceBlocked;
    }

    // Later peer and branch cases remain below storage/resource blockers.
}
```

Phase 78 stall evidence should add basis/confidence fields around this ordering, not invert it. [CITED: .planning/phases/78-progress-guarantees-and-stall-diagnosis/78-CONTEXT.md]

### Carry Evidence Into Soak Checkpoints

```rust
// Source: packages/open-bitcoin-cli/src/operator/soak/runtime/helpers.rs
pub(super) fn checkpoint_status_from_snapshot(
    snapshot: &OpenBitcoinStatusSnapshot,
) -> SoakCheckpointStatus {
    SoakCheckpointStatus {
        maybe_no_progress_diagnosis_label: maybe_available(&snapshot.sync.no_progress_diagnosis)
            .map(|value| serde_label(&value)),
        maybe_validated_active_chain_height: maybe_available(&snapshot.sync.sync_progress)
            .map(|value| value.validated_active_chain_height),
        maybe_best_known_tip_height: maybe_available(&snapshot.sync.best_known_tip)
            .map(|value| value.height),
        // Phase 78 should add progress_credit and stall_diagnosis fields here.
        // Existing fields omitted for brevity.
    }
}
```

This is the right projection point because soak checkpoints are already populated from `OpenBitcoinStatusSnapshot`. [VERIFIED: packages/open-bitcoin-cli/src/operator/soak/runtime/helpers.rs]

## State of the Art

| Old Approach | Current Approach | When Changed / Source | Impact |
|--------------|------------------|------------------------|--------|
| Header/download progress could be presented near active-chain progress. | `SyncProgress` explicitly separates header height, downloaded block height, connected block height, and validated active-chain height/hash/work. [VERIFIED: packages/open-bitcoin-node/src/status.rs] | Phase 68 and status docs. [CITED: .planning/phases/68-full-active-chain-validation-and-durable-persistence/68-CONTEXT.md; docs/architecture/status-snapshot.md] | Phase 78 should credit only `validated_active_chain_*` or stay-current evidence. [CITED: .planning/phases/78-progress-guarantees-and-stall-diagnosis/78-CONTEXT.md] |
| A no-progress event could be broad peer failure or generic no-progress text. | `NoProgressDiagnosis` distinguishes at-tip, awaiting headers, awaiting blocks, stale in-flight, peer backoff/stall/failures, branch bodies, reorg/storage recovery, and storage/resource blocked. [VERIFIED: packages/open-bitcoin-node/src/status.rs; packages/open-bitcoin-node/src/sync/progress.rs] | Phase 70. [CITED: .planning/phases/70-reorg-peer-rotation-and-no-progress-recovery/70-CONTEXT.md] | Add adjacent evidence for PROG-02/PROG-03 instead of replacing all labels. [CITED: .planning/phases/78-progress-guarantees-and-stall-diagnosis/78-CONTEXT.md] |
| Soak reports might become a state store. | Phase 75 defines reports/support summaries as projections from the datadir-owned run index and append-only event ledger. [CITED: docs/architecture/operator-observability.md; docs/operator/runtime-guide.md] | Phase 75. [CITED: .planning/phases/75-multi-day-soak-runner-and-evidence-ledger/75-CONTEXT.md] | Add Phase 78 fields to checkpoints and reports, but keep ledger/status as source of truth. [CITED: .planning/phases/78-progress-guarantees-and-stall-diagnosis/78-CONTEXT.md] |
| Recovery details were compatibility labels and prose. | Phase 77 adds top-level `recovery_evidence: FieldAvailability<RecoveryEvidenceSnapshot>` with action class, cause, basis, and next action. [VERIFIED: packages/open-bitcoin-node/src/status.rs; docs/architecture/operator-observability.md] | Phase 77. [CITED: .planning/phases/77-corruption-and-lock-recovery-hardening/77-CONTEXT.md] | Stall diagnosis should reuse recovery evidence and give it precedence over peer retry advice. [CITED: .planning/phases/78-progress-guarantees-and-stall-diagnosis/78-CONTEXT.md] |

**Deprecated/outdated for Phase 78:**

- Treating `last_successful_progress_unix_seconds` as the credited soak progress watermark is insufficient because it currently accepts header or block contribution timestamps. [VERIFIED: packages/open-bitcoin-node/src/sync/types/summary.rs; CITED: .planning/phases/78-progress-guarantees-and-stall-diagnosis/78-CONTEXT.md]
- Treating `downloaded_block_height` or `blocks_received` as active-chain progress is insufficient because Phase 68/78 require consensus validation, active-chain connection, and durable persistence. [CITED: .planning/phases/68-full-active-chain-validation-and-durable-persistence/68-CONTEXT.md; .planning/phases/78-progress-guarantees-and-stall-diagnosis/78-CONTEXT.md]

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|-------------|-----------|---------|----------|
| Rust `rustc` | Rust compile/test | yes [VERIFIED: `rustc --version`] | `1.94.1` [VERIFIED: `rustc --version`] | None needed |
| Cargo | Rust package commands | yes [VERIFIED: `cargo --version`] | `1.94.1` [VERIFIED: `cargo --version`] | None needed |
| Bun | Phase checker/tests | yes [VERIFIED: `bun --version`; `.bun-version`] | `1.3.9` [VERIFIED: `bun --version`; `.bun-version`] | None needed |
| Bazel | repo-native verify smoke build | yes [VERIFIED: `bazel --version`] | `8.6.0` [VERIFIED: `bazel --version`] | None needed |
| `cargo-llvm-cov` | full `scripts/verify.sh` coverage gate | yes [VERIFIED: `cargo llvm-cov --version`] | `0.8.5` [VERIFIED: `cargo llvm-cov --version`] | None needed |
| Git | repo workflow and verification | yes [VERIFIED: `git --version`] | `2.53.0` [VERIFIED: `git --version`] | None needed |
| Bash | `scripts/verify.sh` and shell wrappers | yes [VERIFIED: `bash --version`] | GNU bash `3.2.57` [VERIFIED: `bash --version`] | None needed |

**Missing dependencies with no fallback:** None found. [VERIFIED: local command probes]

**Missing dependencies with fallback:** None found. [VERIFIED: local command probes]

## Security Domain

Security enforcement is enabled by default because `.planning/config.json` does not set `security_enforcement` to `false`. [VERIFIED: .planning/config.json]

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|------------------|
| V2 Authentication | No new auth expected. [VERIFIED: Phase 78 scope; .planning/phases/78-progress-guarantees-and-stall-diagnosis/78-CONTEXT.md] | Do not touch RPC credential or auth handling for progress/stall reporting. [VERIFIED: codebase inspection] |
| V3 Session Management | No web/session surface expected. [VERIFIED: .planning/PROJECT.md; packages/open-bitcoin-rpc/src/method/node.rs] | Not applicable beyond existing local RPC/runtime metadata. [VERIFIED: packages/open-bitcoin-rpc/src/context.rs] |
| V4 Access Control | Limited to local operator command/RPC status exposure. [VERIFIED: packages/open-bitcoin-cli/src/operator; packages/open-bitcoin-rpc/src/dispatch.rs] | Reuse existing status/RPC surfaces; do not add privileged mutation or hidden datadir repair. [CITED: .planning/phases/78-progress-guarantees-and-stall-diagnosis/78-CONTEXT.md] |
| V5 Input Validation | Yes, for new JSON/status/ledger fields and checker anchors. [VERIFIED: packages/open-bitcoin-node/src/status.rs; packages/open-bitcoin-cli/src/operator/soak/ledger.rs] | Use typed Rust structs/enums with serde labels and deterministic tests. [CITED: standards/core/architecture.md; standards/languages/rust.md] |
| V6 Cryptography | No new cryptography expected. [VERIFIED: Phase 78 scope; packages/Cargo.toml] | Do not add cryptographic dependencies or shortcuts. [CITED: AGENTS.md dependency policy] |
| V7 Error Handling and Logging | Yes, operator-facing diagnosis and logs/metrics wording. [VERIFIED: packages/open-bitcoin-node/src/sync/types/summary.rs; packages/open-bitcoin-cli/src/operator/sync_truth_render.rs] | Preserve unavailable reasons and avoid false "making progress" or broad readiness claims. [CITED: .planning/phases/78-progress-guarantees-and-stall-diagnosis/78-CONTEXT.md; docs/architecture/status-snapshot.md] |

### Known Threat Patterns for This Phase

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| False progress reporting causes unsafe operator trust. [CITED: .planning/phases/78-progress-guarantees-and-stall-diagnosis/78-CONTEXT.md] | Tampering / Repudiation | Credit only typed durable active-chain or stay-current evidence; tests must prove headers, in-flight work, retries, and reports do not advance credit. [CITED: .planning/phases/78-progress-guarantees-and-stall-diagnosis/78-CONTEXT.md] |
| Misdiagnosing storage pressure as peer trouble delays safe operator action. [CITED: .planning/phases/78-progress-guarantees-and-stall-diagnosis/78-CONTEXT.md] | Denial of Service | Preserve storage/resource precedence in the classifier and expose evidence basis. [VERIFIED: packages/open-bitcoin-node/src/sync/progress.rs] |
| Omitting unavailable evidence hides diagnostic uncertainty. [CITED: docs/architecture/status-snapshot.md] | Repudiation | Use `FieldAvailability<T>` for new fields with explicit unavailable reasons. [VERIFIED: packages/open-bitcoin-node/src/status.rs] |
| Support/report projections leak raw or misleading status facts. [CITED: docs/architecture/operator-observability.md] | Information Disclosure / Repudiation | Keep reports compact projections; Phase 79 owns richer support-bundle forensics. [CITED: .planning/phases/78-progress-guarantees-and-stall-diagnosis/78-CONTEXT.md] |

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|

All claims in this research were verified against repo files, local command probes, or cited planning/docs artifacts. No `[ASSUMED]` claims are present. [VERIFIED: research session sources]

## Open Questions (RESOLVED)

1. **RESOLVED: None blocking planning.** [VERIFIED: .planning/phases/78-progress-guarantees-and-stall-diagnosis/78-CONTEXT.md]
   - What we know: The phase context explicitly allows compact typed structs/enums and adjacent evidence fields. [CITED: .planning/phases/78-progress-guarantees-and-stall-diagnosis/78-CONTEXT.md]
   - What's unclear: Exact final field names should be chosen by the planner for minimal churn and checker clarity. [VERIFIED: codebase inspection]
   - Recommendation: Plan field names around `progress_credit`, `stall_diagnosis`, `stalled_subsystem`, `expected_progress_window`, `no_progress_threshold`, `last_useful_work`, `last_peer_contribution`, `evidence_basis`, and `confidence`, then let implementation adjust only if compile-time conflicts appear. [VERIFIED: codebase inspection; CITED: .planning/phases/78-progress-guarantees-and-stall-diagnosis/78-CONTEXT.md]

## Sources

### Primary (HIGH confidence)

- `.planning/phases/78-progress-guarantees-and-stall-diagnosis/78-CONTEXT.md` - locked decisions, phase boundary, implementation surfaces, deferred scope.
- `.planning/REQUIREMENTS.md` - PROG-01 through PROG-04 and v1.7 out-of-scope boundaries.
- `.planning/ROADMAP.md` - Phase 78 goal, dependencies, and success criteria.
- `AGENTS.md`, `AGENTS.bright-builds.md`, `standards/core/architecture.md`, `standards/core/code-shape.md`, `standards/core/verification.md`, `standards/core/testing.md`, `standards/languages/rust.md`, `standards/languages/typescript-javascript.md` - repo workflow and code standards.
- `packages/open-bitcoin-node/src/status.rs` - `FieldAvailability`, `SyncStatus`, `SyncProgress`, `StayCurrentStatus`, `NoProgressDiagnosis`, `DurableSyncState`.
- `packages/open-bitcoin-node/src/sync/progress.rs` - `classify_no_progress`, peer contribution, no-credit response handling, next-action mapping.
- `packages/open-bitcoin-node/src/sync/runtime_state.rs` - durable status projection, carry-forward timestamps, stay-current/no-progress projection.
- `packages/open-bitcoin-node/src/sync/types.rs` and `packages/open-bitcoin-node/src/sync/types/summary.rs` - summary fields, stop reasons, progress signal, last successful progress timestamp.
- `packages/open-bitcoin-cli/src/operator/soak/ledger.rs`, `runtime/helpers.rs`, `report.rs`, `outcome.rs` - soak checkpoint/report/outcome projection.
- `scripts/verify.sh` and Phase 75/76/77 checkers - deterministic verification pattern and default exclusions.

### Secondary (MEDIUM confidence)

- `docs/architecture/status-snapshot.md` - shared status semantics and current field vocabulary.
- `docs/architecture/operator-observability.md` - soak ledger/report projection and resource/recovery evidence vocabulary.
- `docs/operator/runtime-guide.md` - operator command and boundary wording.
- `docs/parity/index.json`, `docs/parity/checklist.md`, `docs/parity/README.md`, `docs/parity/catalog/operator-runtime-release-hardening.md`, `docs/parity/catalog/p2p.md`, `docs/parity/catalog/chainstate.md` - parity roots and discoverability pattern.

### Tertiary (LOW confidence)

- None. [VERIFIED: all research claims sourced from repo files or local probes]

## Metadata

**Confidence breakdown:**

- Standard stack: HIGH - no new dependencies are recommended, and existing versions/tools were verified from manifests and local command probes. [VERIFIED: packages/Cargo.toml; rust-toolchain.toml; .bun-version; local command probes]
- Architecture: HIGH - integration points are explicitly named in context and confirmed in source. [CITED: .planning/phases/78-progress-guarantees-and-stall-diagnosis/78-CONTEXT.md; VERIFIED: packages/open-bitcoin-node/src/status.rs; packages/open-bitcoin-node/src/sync/progress.rs; packages/open-bitcoin-cli/src/operator/soak/ledger.rs]
- Pitfalls: HIGH - false-progress and precedence risks are directly visible in current code and locked decisions. [VERIFIED: packages/open-bitcoin-node/src/sync/types/summary.rs; packages/open-bitcoin-node/src/sync/progress.rs; CITED: .planning/phases/78-progress-guarantees-and-stall-diagnosis/78-CONTEXT.md]
- Verification: HIGH - test/checker patterns and required tools are present locally. [VERIFIED: packages/open-bitcoin-node/src/sync/tests.rs; scripts/check-phase77-corruption-lock-recovery.ts; scripts/verify.sh; local command probes]

**Research date:** 2026-06-16  
**Valid until:** 2026-07-16 for internal architecture and repo-local constraints; re-check manifests and `scripts/verify.sh` if the branch changes before planning. [VERIFIED: current repo inspection]
