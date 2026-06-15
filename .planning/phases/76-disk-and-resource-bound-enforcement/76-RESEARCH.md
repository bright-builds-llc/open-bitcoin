# Phase 76: Disk and Resource Bound Enforcement - Research

**Researched:** 2026-06-15
**Domain:** Open Bitcoin soak resource governance, shared status contracts, disk-footprint measurement, and deterministic Rust/Bun verification
**Confidence:** HIGH for codebase integration and verification shape; MEDIUM for exact threshold defaults because Phase 76 context leaves numeric defaults to planning.

<user_constraints>
## User Constraints (from CONTEXT.md)

Source for this section: [VERIFIED: .planning/phases/76-disk-and-resource-bound-enforcement/76-CONTEXT.md]

### Locked Decisions

### Resource Inventory And Bound Surfaces

- **D-01:** Extend the existing shared typed contracts instead of creating a
  separate renderer-local resource model. `SyncResourcePressure` remains the
  network/in-flight resource envelope, while Phase 76 may add typed adjacent
  bound evidence for disk usage, file counts, cache or queue bounds, metrics
  retention, log retention, soak ledger/report footprints, and support-bundle
  size pressure.
- **D-02:** Surface bounds before and during a soak through the same shared
  status path that CLI status, dashboard, RPC status, support evidence, and
  soak reports already consume. Every field should be either available with
  measured/configured values or explicitly unavailable with a reason.
- **D-03:** Treat the Phase 75 `SoakBounds.disk_budget_bytes` value as the
  operator's explicit soak budget. Compare it against measured datadir,
  metrics, log, soak-ledger/report, and support-evidence footprints where those
  footprints are available. Missing measurements should produce typed
  unavailable evidence, not silent success.
- **D-04:** Keep bound evidence compact and allowlisted. Do not copy raw daemon
  logs, raw metrics stores, raw support bundles, raw live-smoke reports, wallet
  material, credentials, or unbounded peer tables into soak reports or support
  evidence.

### Enforcement And Operator Stop Policy

- **D-05:** Resource enforcement should be dry-run-first in preflight and
  evidence-first at runtime. Preflight should refuse obviously unsafe starts
  such as zero or already-exceeded budgets, missing datadirs that cannot be
  assessed, and unavailable required resource paths when the operator requested
  enforcement.
- **D-06:** Runtime pressure should produce typed warning and stop decisions
  instead of vague "sync failed" text. Use explicit threshold states such as
  normal, warning, and stop-required; planner may choose exact numeric defaults,
  but the defaults must be documented, tested, and configurable or derived from
  the operator's explicit disk budget.
- **D-07:** When resource pressure requires stopping a soak, record a
  `resource_stop` soak outcome with source evidence from shared status,
  recovery category, no-progress/resource diagnosis when available, and the
  resource-bound snapshot that triggered the decision. Durable progress and the
  run id must remain resumable under the Phase 75 same-run resume rules.
- **D-08:** Operator pause and stop guidance should prefer existing
  `open-bitcoin sync pause`, `open-bitcoin sync resume`, and `open-bitcoin soak`
  resume semantics before adding new control surfaces. If a new flag or command
  is needed, it must be explicit, non-destructive, and documented with
  repo-local Cargo and Bazel forms.

### Retention, Compaction, And Support Evidence

- **D-09:** Metrics and structured logs already have bounded retention policies;
  Phase 76 should expose their configured policy, current footprint or
  unavailable reason, and pressure classification rather than duplicating the
  retention engines.
- **D-10:** Support-bundle pressure is a first-class bound. Support evidence
  should report projected bundle size, compact summary availability, omitted
  raw artifacts, and any size pressure that would make bundle generation unsafe
  or misleading.
- **D-11:** Compaction and cleanup guidance must be advice, not hidden mutation.
  The system may tell an operator to free disk, rotate or prune configured logs,
  reduce retention, move output paths, or retry after clearing space. It must
  not silently delete, compact, repair, prune, or relocate user data.
- **D-12:** Resource guidance should preserve storage-first precedence from
  Phase 71. Low disk, storage pressure, backend write failures, and resource
  exhaustion outrank peer retry advice and should map to
  `SyncRecoveryCategory::ResourceExhaustion` or a more precise typed category
  only when planning proves the existing taxonomy is insufficient.

### Deterministic Verification

- **D-13:** RES-08 must be proven with deterministic fixtures, not public peers,
  real service managers, large local disk allocations, or multi-day sleeps.
  Tests should use small temp directories, synthetic file metadata, scripted
  status collectors, fake resource probes, and explicit timestamps.
- **D-14:** Rust tests are the canonical behavior proof for pure resource-bound
  decisions, status projections, soak runtime stop behavior, and retention
  classification. Keep tests one concern per test and use Arrange/Act/Assert
  comments when setup is non-trivial.
- **D-15:** Add a focused Bun checker only for docs, phase artifact anchors,
  parity roots, default-verification boundaries, and generated LOC freshness
  that Rust tests do not prove. Follow the Phase 71 through Phase 75 checker
  style and keep the checker public-network-free and service-manager-free.
- **D-16:** If new first-party Rust source or test files are added under
  `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests`, update
  `docs/parity/source-breadcrumbs.json` and keep
  `scripts/check-parity-breadcrumbs.ts` green.

### the agent's Discretion

- The planner may split Phase 76 across resource-bound domain/status types,
  soak preflight/runtime enforcement, retention/support pressure projection,
  deterministic fixtures, operator docs, and checker/parity closeout.
- The executor may add small pure helper types for budget thresholds, measured
  footprints, pressure states, and next-action guidance when they make illegal
  states unrepresentable.
- The executor may keep existing enum variants and add precise evidence fields
  when that is simpler than adding a new taxonomy. Add new variants only when
  the current labels cannot express RES-05 through RES-08 clearly.

### Deferred Ideas (OUT OF SCOPE)

- Corruption markers, schema mismatch, stale locks, partial writes, and
  storage-open recovery detail belongs to Phase 77.
- False-progress prevention, stalled subsystem diagnosis, peer contribution
  windows, and no-progress thresholds belong to Phase 78.
- Full support-bundle forensic timeline, failure narrative, and cross-surface
  final verdict forensics belong to Phase 79.
- Opt-in multi-day soak UAT command closeout and v1.7 release-boundary wording
  belongs to Phase 80.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| RES-05 | Operator can see disk, file, cache, queue, peer, in-flight, log, metric, and support-bundle bounds before starting a long soak. [VERIFIED: .planning/REQUIREMENTS.md] | Add one shared resource-bound snapshot next to `SyncResourcePressure`, collect datadir/log/metrics/soak/support footprints with typed unavailable reasons, and render it through status/dashboard/support/soak reports. [VERIFIED: packages/open-bitcoin-node/src/status.rs; VERIFIED: packages/open-bitcoin-cli/src/operator/status.rs; VERIFIED: packages/open-bitcoin-cli/src/operator/soak/report.rs] |
| RES-06 | Operator can receive typed low-disk, disk-growth, compaction, log-retention, metrics-retention, and support-bundle size guidance during and after a soak. [VERIFIED: .planning/REQUIREMENTS.md] | Reuse `SyncRecoveryCategory::ResourceExhaustion`, `NoProgressDiagnosis::StorageOrResourceBlocked`, `MetricRetentionPolicy`, and `LogRetentionPolicy`; add pressure-state/guidance types instead of renderer-local prose. [VERIFIED: packages/open-bitcoin-node/src/status/recovery.rs; VERIFIED: packages/open-bitcoin-node/src/metrics.rs; VERIFIED: packages/open-bitcoin-node/src/logging.rs; VERIFIED: packages/open-bitcoin-node/src/sync/progress.rs] |
| RES-07 | Operator can stop or pause a soak before unsafe storage pressure while preserving durable progress and an actionable next step. [VERIFIED: .planning/REQUIREMENTS.md] | Feed stop-required resource evidence into the Phase 75 soak loop so it records `resource_stop`; keep same-run resume behavior for `resource_stop` and point guidance to existing `sync pause/resume` and `soak resume`. [VERIFIED: packages/open-bitcoin-cli/src/operator/soak/runtime.rs; VERIFIED: packages/open-bitcoin-cli/src/operator/soak/outcome.rs; VERIFIED: packages/open-bitcoin-cli/src/operator/runtime/support.rs] |
| RES-08 | Contributor can verify resource-bound behavior with deterministic fixtures that do not require a public peer, real service manager, or large local disk allocation. [VERIFIED: .planning/REQUIREMENTS.md] | Use pure classifier tests, small temp-directory footprint fixtures, scripted `SoakStatusCollector`, `SoakTestClock`, and a Bun checker wired into `scripts/verify.sh` after the Phase 75 checker. [VERIFIED: packages/open-bitcoin-cli/src/operator/soak/tests/runtime.rs; VERIFIED: packages/open-bitcoin-node/src/sync/tests/soak.rs; VERIFIED: scripts/verify.sh] |
</phase_requirements>

## Summary

Phase 76 should be planned as an additive shared-status extension, not as a new operator-only reporting layer. `OpenBitcoinStatusSnapshot` and `SyncStatus` already carry availability-aware fields, recovery categories, no-progress diagnosis, metrics/log status, and `SyncResourcePressure`; Phase 76 should add adjacent typed resource-bound evidence for disk/file/cache/queue/log/metric/soak/support footprint and pressure decisions. [VERIFIED: packages/open-bitcoin-node/src/status.rs; VERIFIED: packages/open-bitcoin-node/src/metrics.rs; VERIFIED: packages/open-bitcoin-node/src/logging.rs]

The largest missing implementation surface is filesystem resource evidence: current status collection loads durable sync state, log status, and metrics status, but it does not yet calculate datadir footprint, file counts, log/metrics byte footprint, soak ledger/report footprint, support-bundle projected size, or actual filesystem available bytes. [VERIFIED: packages/open-bitcoin-cli/src/operator/status.rs; VERIFIED: packages/open-bitcoin-cli/src/operator/support.rs; VERIFIED: packages/open-bitcoin-cli/src/operator/soak/ledger.rs]

**Primary recommendation:** Add `ResourceBoundSnapshot` as a top-level `OpenBitcoinStatusSnapshot` field with pure pressure classification and thin filesystem probes; use `fs4 = 1.1.0` only for portable free/available/total space, and use `std::fs` for recursive allowlisted footprint and file-count measurement. [VERIFIED: cargo info fs4; VERIFIED: fs4-1.1.0 source; VERIFIED: packages/open-bitcoin-node/src/status.rs]

## Project Constraints (from AGENTS.md)

- Root repo instructions require reading `AGENTS.md`, `AGENTS.bright-builds.md`, `standards-overrides.md`, and relevant `standards/` pages before planning or implementation. [VERIFIED: AGENTS.md; VERIFIED: AGENTS.bright-builds.md]
- `bash scripts/verify.sh` is the repo-native verification contract for first-party code, including the Bazel smoke build. [VERIFIED: AGENTS.md; VERIFIED: scripts/verify.sh]
- Rust toolchain source of truth is `rust-toolchain.toml`, pinned to Rust `1.94.1`. [VERIFIED: AGENTS.md; VERIFIED: rust-toolchain.toml; VERIFIED: rustc --version]
- Bun is the canonical runtime for repo-owned higher-level automation scripts, and this repo has no `package.json` bootstrap step. [VERIFIED: AGENTS.md; VERIFIED: .planning/STACK.md; VERIFIED: .bun-version]
- Operator UAT docs must provide repo-local Cargo and Bazel command forms, not only the installed `open-bitcoin` alias. [VERIFIED: AGENTS.md]
- New first-party Rust source or test files under `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests` require parity breadcrumbs in `docs/parity/source-breadcrumbs.json` and must keep `scripts/check-parity-breadcrumbs.ts` green. [VERIFIED: AGENTS.md; VERIFIED: scripts/verify.sh]
- `docs/metrics/lines-of-code.md` is an intentionally tracked generated artifact and may require freshness updates after verification. [VERIFIED: AGENTS.md; VERIFIED: scripts/verify.sh]
- In-scope behavior differences from Bitcoin Knots must be recorded in `docs/parity/index.json` and companion docs under `docs/parity/`. [VERIFIED: AGENTS.md; VERIFIED: .planning/CONVENTIONS.md]
- Functional-core / imperative-shell boundaries are mandatory project architecture: pure decisions should stay data-in/data-out, while filesystem, process, network, terminal, RPC, service-manager, and durable-storage effects stay in shell adapters. [VERIFIED: AGENTS.md; VERIFIED: standards/core/architecture.md; VERIFIED: .planning/ARCHITECTURE.md]
- Bright Builds standards require illegal states to be unrepresentable where practical, early returns over nesting, `maybe_` names for optional internal Rust values, unit tests for pure business logic, and Arrange/Act/Assert test structure when non-trivial. [VERIFIED: standards/core/architecture.md; VERIFIED: standards/core/code-shape.md; VERIFIED: standards/core/testing.md; VERIFIED: standards/languages/rust.md]
- Project skills directories `.claude/skills/` and `.agents/skills/` are absent, so no repo-local skills modify Phase 76 planning. [VERIFIED: find .claude/skills .agents/skills]

## Standard Stack

### Core

| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| Rust / Cargo | 1.94.1 | Implement resource-bound domain types, status contracts, soak runtime enforcement, and deterministic tests. | The repo pins Rust `1.94.1` and all first-party packages live in `packages/Cargo.toml`. [VERIFIED: rust-toolchain.toml; VERIFIED: packages/Cargo.toml] |
| `open-bitcoin-node` first-party status/types | 0.1.0 workspace | Own shared status contracts, resource-bound domain types, metrics/log retention contracts, and durable sync state projections. | `OpenBitcoinStatusSnapshot`, `SyncStatus`, `FieldAvailability`, `SyncResourcePressure`, metrics, logs, and recovery labels already live there. [VERIFIED: packages/open-bitcoin-node/src/status.rs; VERIFIED: packages/open-bitcoin-node/src/metrics.rs; VERIFIED: packages/open-bitcoin-node/src/logging.rs] |
| `open-bitcoin-cli` first-party operator code | 0.1.0 workspace | Own soak command preflight/runtime wiring, support bundle projection, status rendering, dashboard projection, and operator docs. | Phase 75 added `open-bitcoin soak`, ledger/report projection, support soak summary, and scripted tests in this crate. [VERIFIED: packages/open-bitcoin-cli/src/operator/soak.rs; VERIFIED: packages/open-bitcoin-cli/src/operator/support.rs] |
| `serde` / `serde_json` | `serde` 1.0.228, `serde_json` 1.0.149 locked | Stable JSON contracts for status, soak ledgers/reports, support evidence, and checker fixtures. | Existing status, soak, metrics, logs, and support structs already derive/emit serde JSON. [VERIFIED: packages/Cargo.lock; VERIFIED: packages/open-bitcoin-node/src/status.rs; VERIFIED: packages/open-bitcoin-cli/src/operator/soak/ledger.rs] |
| `fs4` | 1.1.0 latest, published 2026-04-28 | Portable filesystem free/available/total-space measurements when Phase 76 needs actual free-space evidence. | Rust `std` has no portable free-space API; `fs4` exposes `available_space`, `free_space`, and `total_space` for Unix/Windows and is narrower than `sysinfo`. [VERIFIED: cargo info fs4; VERIFIED: ~/.cargo/registry/src/.../fs4-1.1.0/src/lib.rs; VERIFIED: crates.io API] |
| `std::fs` | Rust 1.94.1 | Recursive allowlisted footprint and file-count measurement for datadir, log dir, metrics store, soak ledger/report dir, and support output dir. | The repo already uses `std::fs` for log metadata, soak ledger/report writes, and support bundle writes. [VERIFIED: packages/open-bitcoin-node/src/logging/writer.rs; VERIFIED: packages/open-bitcoin-cli/src/operator/soak/ledger.rs; VERIFIED: packages/open-bitcoin-cli/src/operator/support.rs] |

### Supporting

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| `fjall` | 3.1.4 locked | Durable metadata, metrics history, runtime sync state, and resource/recovery persistence. | Use existing store APIs for runtime metadata and metrics status; do not build a second durable state store. [VERIFIED: packages/open-bitcoin-node/Cargo.toml; VERIFIED: packages/open-bitcoin-node/src/storage/fjall_store.rs] |
| `thiserror` | 2.0.18 locked in CLI; 1.0.69 also locked transitively | Typed errors for resource preflight/probe/classification failures. | Use where new Rust error enums cross operator/runtime boundaries. [VERIFIED: packages/open-bitcoin-cli/Cargo.toml; VERIFIED: packages/Cargo.lock] |
| `tempfile` | 3.27.0 locked | Deterministic small temp-directory fixtures. | Use only in tests where existing patterns need safer temporary roots. [VERIFIED: packages/Cargo.lock] |
| Bun | 1.3.9 | Phase checker and checker tests. | Use for focused `scripts/check-phase76-resource-bounds.ts` and `bun test` fixture tests if docs/parity/default-verifier anchors need auditing. [VERIFIED: .bun-version; VERIFIED: bun --version; VERIFIED: scripts/check-phase75-soak-runner.ts] |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| `fs4` for free-space stats | `sysinfo` 0.39.3 latest | `sysinfo` is broader and feature-heavy for a phase that only needs disk totals; use it only if future phases need process/system telemetry beyond disk. [VERIFIED: cargo search sysinfo; VERIFIED: cargo info sysinfo] |
| `fs4` for free-space stats | Direct `rustix`/`libc` statvfs calls | Direct OS calls would be easy to get wrong across Unix/Windows and would hand-roll cross-platform behavior. [VERIFIED: packages/Cargo.lock; VERIFIED: fs4-1.1.0 source] |
| Shared `ResourceBoundSnapshot` | Renderer-local strings in CLI/support/dashboard | Renderer-local strings would violate the existing shared truth contract and make support/soak evidence diverge from status JSON. [VERIFIED: .planning/phases/72-operator-observability-and-support-evidence/72-CONTEXT.md; VERIFIED: docs/architecture/status-snapshot.md] |

**Installation:**

```bash
cargo add fs4@1.1.0 --manifest-path packages/open-bitcoin-node/Cargo.toml
```

Existing locked versions should remain unchanged unless implementation proves a direct upgrade is needed. [VERIFIED: packages/Cargo.lock]

**Version verification:**

| Package/tool | Recommended version | Verified current/latest | Publish/update date |
|--------------|---------------------|-------------------------|---------------------|
| Rust | 1.94.1 | 1.94.1 installed | rustc build date 2026-03-25 [VERIFIED: rustc --version] |
| Bun | 1.3.9 | 1.3.9 installed | pinned locally [VERIFIED: .bun-version; VERIFIED: bun --version] |
| `fs4` | 1.1.0 | 1.1.0 latest | 2026-04-28 [VERIFIED: cargo info fs4; VERIFIED: crates.io API] |
| `serde` | 1.0.228 locked | 1.0.228 latest | 2025-09-27 [VERIFIED: packages/Cargo.lock; VERIFIED: crates.io API] |
| `serde_json` | 1.0.149 locked | 1.0.150 latest | 2026-05-21 for latest [VERIFIED: packages/Cargo.lock; VERIFIED: crates.io API] |
| `fjall` | 3.1.4 locked | 3.1.5 latest | 2026-06-08 for latest [VERIFIED: packages/Cargo.lock; VERIFIED: crates.io API] |
| `clap` | 4.6.1 locked | 4.6.1 latest | 2026-04-15 [VERIFIED: packages/Cargo.lock; VERIFIED: crates.io API] |
| `tempfile` | 3.27.0 locked | 3.27.0 latest | 2026-03-11 [VERIFIED: packages/Cargo.lock; VERIFIED: crates.io API] |

## Architecture Patterns

### Recommended Project Structure

```text
packages/open-bitcoin-node/src/
├── status.rs                    # Re-export shared status/resource contracts.
├── status/
│   ├── recovery.rs              # Existing recovery categories.
│   └── resource_bounds.rs       # New pure resource-bound snapshot and classifier types.
└── resource_bounds.rs           # New thin filesystem probe helpers if node owns probes.

packages/open-bitcoin-cli/src/operator/
├── status.rs                    # Collect resource-bound snapshot into OpenBitcoinStatusSnapshot.
├── status/render.rs             # Render compact human/JSON resource-bound evidence.
├── dashboard/model.rs           # Project resource-bound rows from shared snapshot.
├── soak/
│   ├── runtime.rs               # Preflight and runtime stop wiring.
│   └── resource_bounds.rs       # Soak-specific preflight/enforcement helper if kept CLI-owned.
└── support.rs                   # Support-bundle projected/actual size pressure.

scripts/
├── check-phase76-resource-bounds.ts
└── check-phase76-resource-bounds.test.ts
```

This structure keeps pure pressure decisions in reusable Rust types and isolates filesystem probing/status collection in shell-owned adapters. [VERIFIED: standards/core/architecture.md; VERIFIED: packages/open-bitcoin-node/src/status.rs; VERIFIED: packages/open-bitcoin-cli/src/operator/status.rs]

### Pattern 1: Shared Status Snapshot Extension

**What:** Add a top-level `resource_bounds: FieldAvailability<ResourceBoundSnapshot>` to `OpenBitcoinStatusSnapshot`, with `#[serde(default = "...")]` for backward-compatible deserialization. [VERIFIED: packages/open-bitcoin-node/src/status.rs]

**When to use:** Use this for disk, file, log, metric, soak ledger/report, and support-bundle evidence because these are not strictly network/in-flight sync pressure fields. [VERIFIED: .planning/phases/76-disk-and-resource-bound-enforcement/76-CONTEXT.md]

**Example:**

```rust
// Pattern source: packages/open-bitcoin-node/src/status.rs
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceBoundSnapshot {
    pub disk: FieldAvailability<DiskBoundEvidence>,
    pub files: FieldAvailability<FileBoundEvidence>,
    pub logs: FieldAvailability<RetentionFootprintEvidence>,
    pub metrics: FieldAvailability<RetentionFootprintEvidence>,
    pub soak: FieldAvailability<ArtifactFootprintEvidence>,
    pub support_bundle: FieldAvailability<SupportBundleBoundEvidence>,
    pub guidance: Vec<ResourceBoundGuidance>,
}
```

### Pattern 2: Pure Classifier, Thin Probe

**What:** Keep threshold math and next-action selection as pure data-in/data-out functions; pass measured bytes/counts from a filesystem probe. [VERIFIED: standards/core/architecture.md; VERIFIED: packages/open-bitcoin-node/src/logging/prune.rs]

**When to use:** Use pure tests for budget thresholds, missing measurements, growth pressure, and stop-required classification; use small temp directories only for adapter tests. [VERIFIED: .planning/phases/76-disk-and-resource-bound-enforcement/76-CONTEXT.md]

**Example:**

```rust
// Pattern source: packages/open-bitcoin-node/src/logging/prune.rs
pub fn classify_budget_pressure(input: ResourceBudgetInput) -> ResourcePressureState {
    if input.measured_bytes >= input.stop_bytes {
        return ResourcePressureState::StopRequired;
    }
    if input.measured_bytes >= input.warning_bytes {
        return ResourcePressureState::Warning;
    }
    ResourcePressureState::Normal
}
```

The exact default `warning_bytes` and `stop_bytes` should be documented in the plan and tests; warning at 80% and stop at 95% of the operator disk budget is the recommended default but is still an assumption until the planner locks it. [ASSUMED]

### Pattern 3: Evidence-First Soak Stop

**What:** Evaluate the shared resource-bound snapshot after each soak checkpoint and record `resource_stop` only when the snapshot says stop is required. [VERIFIED: packages/open-bitcoin-cli/src/operator/soak/runtime.rs; VERIFIED: packages/open-bitcoin-cli/src/operator/soak/outcome.rs]

**When to use:** Use this in `run_bounded_soak_loop` after `collector.collect()` and before scheduling the next checkpoint. [VERIFIED: packages/open-bitcoin-cli/src/operator/soak/runtime.rs]

**Example:**

```rust
// Pattern source: packages/open-bitcoin-cli/src/operator/soak/runtime.rs
let snapshot = collector.collect();
let status = checkpoint_status_from_snapshot(&snapshot);
ledger.append_event(checkpoint_at, SoakLedgerEvent::Checkpoint { status })?;

if resource_stop_required(&snapshot) {
    final_outcome = Some(SoakOutcomeLabel::ResourceStop);
}
```

The planner should extend `SoakCheckpointStatus` or report projection with a compact resource-bound snapshot pointer/summary so `resource_stop` is auditable after the run. [VERIFIED: packages/open-bitcoin-cli/src/operator/soak/ledger.rs; VERIFIED: packages/open-bitcoin-cli/src/operator/soak/report.rs]

### Pattern 4: Support Evidence Remains a Projection

**What:** Compute projected and actual support-bundle size pressure from compact JSON/Markdown strings and omitted-artifact summaries, not from raw logs or raw stores. [VERIFIED: packages/open-bitcoin-cli/src/operator/support.rs; VERIFIED: packages/open-bitcoin-cli/src/operator/support/render.rs]

**When to use:** Use before writing a support bundle if the output path is known; expose unavailable reason in status when projection cannot be calculated without the support command context. [VERIFIED: .planning/phases/76-disk-and-resource-bound-enforcement/76-CONTEXT.md]

### Anti-Patterns to Avoid

- **Duplicating `SyncResourcePressure`:** Keep it focused on network/in-flight sync bounds; add adjacent resource-bound evidence for disk/log/metrics/support instead. [VERIFIED: .planning/phases/76-disk-and-resource-bound-enforcement/76-CONTEXT.md; VERIFIED: packages/open-bitcoin-node/src/status.rs]
- **Renderer-local classification:** Do not classify low disk differently in CLI, dashboard, support, and soak reports. [VERIFIED: .planning/phases/72-operator-observability-and-support-evidence/72-CONTEXT.md]
- **Hidden cleanup:** Do not delete logs, compact stores, prune reports, relocate outputs, repair datadirs, or mutate wallets as part of guidance. [VERIFIED: .planning/phases/76-disk-and-resource-bound-enforcement/76-CONTEXT.md]
- **Large disk fixtures:** Do not allocate near-budget files to prove pressure; feed synthetic byte counts into pure classifiers and use tiny temp files for adapter smoke. [VERIFIED: .planning/phases/76-disk-and-resource-bound-enforcement/76-CONTEXT.md]
- **Growing near-limit files:** `status.rs` is 625 lines and `operator/runtime/support.rs` is 627 lines, so new substantial code there should be moved into child modules. [VERIFIED: wc -l packages/open-bitcoin-node/src/status.rs packages/open-bitcoin-cli/src/operator/runtime/support.rs; VERIFIED: standards/core/code-shape.md]

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Portable filesystem free/available/total-space stats | Ad hoc `statvfs`, `libc`, shell `df`, or platform-specific command parsing | `fs4::available_space`, `fs4::free_space`, `fs4::total_space` | This avoids OS-specific parsing and keeps free-space evidence in Rust tests. [VERIFIED: fs4-1.1.0 source; VERIFIED: cargo info fs4] |
| Resource-status availability | `Option<T>` fields with missing context | `FieldAvailability<T>` | Existing status JSON preserves unavailable reasons with a stable tagged shape. [VERIFIED: packages/open-bitcoin-node/src/status.rs] |
| Metrics retention | A second metrics pruning engine | `MetricRetentionPolicy` and `append_and_prune_metric_samples` | Existing metrics policy already bounds interval, count, and age. [VERIFIED: packages/open-bitcoin-node/src/metrics.rs] |
| Structured-log retention | A second log retention engine | `LogRetentionPolicy` and `plan_log_retention` | Existing log planner already enforces file count, age, and total-byte caps. [VERIFIED: packages/open-bitcoin-node/src/logging.rs; VERIFIED: packages/open-bitcoin-node/src/logging/prune.rs] |
| Soak run identity/report storage | A second support-owned run store | `SoakLedger`, `SoakRunIndex`, and report projection | Phase 75 defines the datadir-owned source of truth and support summaries are projections. [VERIFIED: packages/open-bitcoin-cli/src/operator/soak/ledger.rs; VERIFIED: packages/open-bitcoin-cli/src/operator/support.rs] |
| Pause/resume control | New mutable control channel for resource pressure | Existing `open-bitcoin sync pause`, `sync resume`, and `soak resume` semantics | Existing controls persist durable pause state and guard offline mutation when a live daemon may own the store. [VERIFIED: packages/open-bitcoin-cli/src/operator/runtime/support.rs] |
| Public-network resource tests | Live peers, service managers, multi-day sleeps, or large files | Scripted status collectors, `SoakTestClock`, synthetic metadata, and temp dirs | Existing Phase 75 tests already use these patterns for deterministic soak behavior. [VERIFIED: packages/open-bitcoin-cli/src/operator/soak/tests/runtime.rs; VERIFIED: packages/open-bitcoin-node/src/sync/tests/soak.rs] |

**Key insight:** Phase 76 is resource governance and evidence, not storage maintenance; the implementation should classify, refuse, warn, or stop, while mutation stays explicit operator action. [VERIFIED: .planning/phases/76-disk-and-resource-bound-enforcement/76-CONTEXT.md]

## Common Pitfalls

### Pitfall 1: Treating Missing Measurements As Passing

**What goes wrong:** A soak starts even though the datadir, log dir, metrics store, or support output path could not be assessed. [VERIFIED: .planning/phases/76-disk-and-resource-bound-enforcement/76-CONTEXT.md]

**Why it happens:** Existing status fields often fall back to unavailable reasons, and a naive classifier might ignore unavailable evidence. [VERIFIED: packages/open-bitcoin-node/src/status.rs; VERIFIED: packages/open-bitcoin-cli/src/operator/status/sync_state.rs]

**How to avoid:** Resource preflight should distinguish unavailable, normal, warning, and stop-required states, and enforcement should reject unavailable required paths when resource enforcement is enabled. [VERIFIED: .planning/phases/76-disk-and-resource-bound-enforcement/76-CONTEXT.md]

**Warning signs:** Tests assert only successful starts and do not include missing datadir/log/metrics/support path fixtures. [VERIFIED: packages/open-bitcoin-cli/src/operator/soak/tests/runtime.rs]

### Pitfall 2: Recording `resource_stop` Without Trigger Evidence

**What goes wrong:** The soak report says `resource_stop`, but the report does not contain the resource-bound snapshot or source status evidence that caused the stop. [VERIFIED: .planning/phases/76-disk-and-resource-bound-enforcement/76-CONTEXT.md]

**Why it happens:** Phase 75 checkpoints currently store compact sync/status labels, but not a full resource-bound snapshot. [VERIFIED: packages/open-bitcoin-cli/src/operator/soak/ledger.rs]

**How to avoid:** Extend `SoakCheckpointStatus` and `SoakReportProjection` with compact allowlisted resource-bound labels/bytes/counts or a source status path plus latest sequence. [VERIFIED: packages/open-bitcoin-cli/src/operator/soak/ledger.rs; VERIFIED: packages/open-bitcoin-cli/src/operator/soak/report.rs]

**Warning signs:** Tests only assert final outcome and not the resource snapshot/guidance that triggered it. [VERIFIED: packages/open-bitcoin-cli/src/operator/soak/tests.rs]

### Pitfall 3: Mutating User Data During "Guidance"

**What goes wrong:** The system silently prunes logs, compacts stores, deletes reports, or moves output paths to make resource pressure disappear. [VERIFIED: .planning/phases/76-disk-and-resource-bound-enforcement/76-CONTEXT.md]

**Why it happens:** Existing log retention can prune managed logs during log writes, but Phase 76 guidance itself must not turn into hidden cleanup of user-selected artifacts. [VERIFIED: packages/open-bitcoin-node/src/logging/writer.rs; VERIFIED: .planning/phases/76-disk-and-resource-bound-enforcement/76-CONTEXT.md]

**How to avoid:** Keep compaction/cleanup as `next_action` guidance strings and require explicit future operator commands for mutation. [VERIFIED: docs/operator/runtime-guide.md; VERIFIED: .planning/phases/76-disk-and-resource-bound-enforcement/76-CONTEXT.md]

**Warning signs:** New code calls `remove_file`, `remove_dir_all`, compaction, repair, or pruning APIs from resource-bound classification paths. [VERIFIED: packages/open-bitcoin-node/src/logging/writer.rs]

### Pitfall 4: Threshold Flapping And TOCTOU

**What goes wrong:** A status snapshot reports enough available space, but a write fails immediately after because another process consumed disk. [ASSUMED]

**Why it happens:** Disk free-space measurement is inherently point-in-time, and only backend write errors are authoritative after the check. [ASSUMED]

**How to avoid:** Treat preflight as advisory/refusal evidence, continue mapping backend low-disk errors to `ResourceExhaustion`, and make runtime checkpoints re-evaluate pressure. [VERIFIED: packages/open-bitcoin-node/src/storage.rs; VERIFIED: packages/open-bitcoin-node/src/sync/types/recovery.rs]

**Warning signs:** Tests expect free-space checks to prevent all backend ENOSPC paths. [ASSUMED]

### Pitfall 5: Broadening v1.7 Scope

**What goes wrong:** Resource-bound enforcement starts implementing corruption recovery, stall diagnosis, support forensics, pruning, or release-boundary closeout. [VERIFIED: .planning/phases/76-disk-and-resource-bound-enforcement/76-CONTEXT.md]

**Why it happens:** Resource pressure touches adjacent recovery, progress, support, and release surfaces. [VERIFIED: .planning/ROADMAP.md]

**How to avoid:** Keep Phase 76 to RES-05 through RES-08 and defer Phase 77 through Phase 80 topics exactly as listed in the context. [VERIFIED: .planning/REQUIREMENTS.md; VERIFIED: .planning/phases/76-disk-and-resource-bound-enforcement/76-CONTEXT.md]

## Code Examples

### Availability-Aware Status Fields

```rust
// Source: packages/open-bitcoin-node/src/status.rs
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "state", content = "value")]
pub enum FieldAvailability<T> {
    Available(T),
    Unavailable { reason: String },
}
```

Use this same wrapper for every resource-bound field that can fail to collect. [VERIFIED: packages/open-bitcoin-node/src/status.rs]

### Existing Retention Contract To Expose

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

Expose this policy and current footprint/availability instead of duplicating metric retention logic. [VERIFIED: packages/open-bitcoin-node/src/metrics.rs]

### Existing Soak Runtime Injection Point

```rust
// Source: packages/open-bitcoin-cli/src/operator/soak/runtime.rs
pub(crate) trait SoakStatusCollector {
    fn collect(&mut self) -> OpenBitcoinStatusSnapshot;
}
```

Deterministic Phase 76 runtime tests can inject snapshots that contain normal, warning, and stop-required resource-bound states. [VERIFIED: packages/open-bitcoin-cli/src/operator/soak/runtime.rs; VERIFIED: packages/open-bitcoin-cli/src/operator/soak/tests/runtime.rs]

### Existing Resource Outcome Classifier

```rust
// Source: packages/open-bitcoin-cli/src/operator/soak/outcome.rs
fn has_resource_stop(evidence: &SoakOutcomeEvidence) -> bool {
    matches!(
        evidence.maybe_recovery_category,
        Some(SyncRecoveryCategory::ResourceExhaustion)
    ) || matches!(
        evidence.maybe_no_progress_diagnosis,
        Some(NoProgressDiagnosis::StorageOrResourceBlocked)
    )
}
```

Phase 76 should add concrete resource-bound evidence behind the existing `resource_stop` path rather than replacing this taxonomy. [VERIFIED: packages/open-bitcoin-cli/src/operator/soak/outcome.rs]

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Renderer-specific resource prose | Shared `OpenBitcoinStatusSnapshot` with `FieldAvailability` and machine labels | Established before Phase 72 and reinforced in Phase 72 | Add resource-bound evidence once and project it everywhere. [VERIFIED: .planning/phases/72-operator-observability-and-support-evidence/72-CONTEXT.md; VERIFIED: docs/architecture/status-snapshot.md] |
| Network-only pressure visibility | `SyncResourcePressure` reports in-flight/network bounds, while Phase 76 adds adjacent disk/artifact pressure | Phase 71 created resource pressure; Phase 76 extends long-soak disk/artifact bounds | Do not overload `SyncResourcePressure` with support-bundle/log/metric footprint. [VERIFIED: .planning/phases/71-resource-bounds-and-durable-restart-resume/71-CONTEXT.md; VERIFIED: .planning/phases/76-disk-and-resource-bound-enforcement/76-CONTEXT.md] |
| Soak evidence as ad hoc reports | Datadir-owned run index plus append-only JSONL ledger, with JSON/Markdown projections | Phase 75 | Runtime `resource_stop` must append source evidence to the ledger/report path, not only print CLI text. [VERIFIED: packages/open-bitcoin-cli/src/operator/soak/ledger.rs; VERIFIED: .planning/phases/75-multi-day-soak-runner-and-evidence-ledger/75-VERIFICATION.md] |
| Public-network or long-wall-clock proof | Deterministic Rust fixtures plus focused Bun checkers | Reaffirmed across Phases 71-75 and v1.7 requirements | RES-08 must be proven without public peers, service managers, large disk allocation, or multi-day sleeps. [VERIFIED: .planning/REQUIREMENTS.md; VERIFIED: scripts/verify.sh] |

**Deprecated/outdated:**

- Using support bundles as raw artifact archives is out of bounds; support evidence must stay compact, redacted, and allowlisted. [VERIFIED: packages/open-bitcoin-cli/src/operator/support.rs; VERIFIED: .planning/phases/76-disk-and-resource-bound-enforcement/76-CONTEXT.md]
- Treating storage/resource blockers as peer retry guidance is out of bounds; storage/resource pressure outranks peer advice. [VERIFIED: packages/open-bitcoin-node/src/sync/progress.rs; VERIFIED: docs/architecture/status-snapshot.md]

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | Warning at 80% and stop at 95% of the explicit disk budget is the Phase 76 default threshold pair. | Architecture Patterns / Pattern 2 | If implementation drifts from the chosen values, docs, tests, and operator guidance will disagree. |
| A2 | Disk free-space checks are point-in-time and cannot prevent every subsequent backend low-disk write failure. | Common Pitfalls / Threshold Flapping And TOCTOU | If the implementation treats preflight as authoritative, it may miss runtime ENOSPC recovery tests. |
| A3 | Pure resource-bound contracts/classifiers live in `open-bitcoin-node::status::resource_bounds`; filesystem probe orchestration lives in `open-bitcoin-cli` status collection for Phase 76. | Open Questions / Where filesystem probes should live | Future daemon-side collection would need an explicit adapter plan rather than moving filesystem effects into pure contracts. |

## Open Questions (RESOLVED)

1. **Exact threshold defaults**
   - What we know: Phase context requires `normal`, `warning`, and `stop-required` states, and says planner may choose numeric defaults. [VERIFIED: .planning/phases/76-disk-and-resource-bound-enforcement/76-CONTEXT.md]
   - Decision: Phase 76 locks `RESOURCE_BOUND_WARNING_PERCENT = 80` and `RESOURCE_BOUND_STOP_PERCENT = 95` as default classifier constants. Threshold byte values are derived from each explicit budget, including `SoakBounds.disk_budget_bytes`, and must be documented and tested. [RESOLVED]

2. **Where filesystem probes should live**
   - What we know: `open-bitcoin-node` owns status/contracts/storage/logging, while `open-bitcoin-cli` owns operator status/support/soak collection. [VERIFIED: .planning/ARCHITECTURE.md; VERIFIED: packages/open-bitcoin-node/src/status.rs; VERIFIED: packages/open-bitcoin-cli/src/operator/status.rs]
   - Decision: Put pure resource-bound types/classifiers in `open-bitcoin-node::status::resource_bounds`; put filesystem probe orchestration in `open-bitcoin-cli/src/operator/status/resource_bounds.rs` for Phase 76. Required evidence that cannot be probed must be represented as unavailable with a reason rather than guessed. [RESOLVED]

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|-------------|-----------|---------|----------|
| Rust / Cargo | Rust implementation and tests | Yes | rustc/cargo 1.94.1 | None needed. [VERIFIED: rustc --version; VERIFIED: cargo --version] |
| Bun | Phase checker and checker tests | Yes | 1.3.9 | None needed. [VERIFIED: bun --version] |
| Bazel/Bazelisk | Repo-native smoke build in `scripts/verify.sh` | Yes | Bazelisk 1.28.1, Bazel 8.6.0 | None needed. [VERIFIED: bazelisk version; VERIFIED: bazel version] |
| cargo-llvm-cov | Repo-native coverage in `scripts/verify.sh` | Yes | 0.8.5 | None needed. [VERIFIED: cargo llvm-cov --version; VERIFIED: scripts/verify.sh] |
| Node | GSD init/research tooling and crate-version checks | Yes | v24.13.0 | Bun can run repo-owned TS scripts. [VERIFIED: node --version; VERIFIED: .bun-version] |
| ripgrep | Codebase research and checker-style audits | Yes | 15.1.0 | `grep` exists but is slower. [VERIFIED: rg --version] |
| Git | Optional research commit and repo state inspection | Yes | 2.53.0 | None needed. [VERIFIED: git --version] |

**Missing dependencies with no fallback:** None found for planning and deterministic implementation. [VERIFIED: environment audit commands]

**Missing dependencies with fallback:** None found. [VERIFIED: environment audit commands]

**Worktree note:** `.planning/config.json` is already modified outside this research artifact, changing `_auto_chain_active` from `false` to `true`; do not revert it from Phase 76 work. [VERIFIED: git diff -- .planning/config.json]

## Security Domain

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|-----------------|
| V2 Authentication | No new auth surface | Reuse existing RPC auth source when live sync controls are called; do not add credential handling to resource probes. [VERIFIED: packages/open-bitcoin-cli/src/operator/runtime/support.rs] |
| V3 Session Management | No | Phase 76 is local CLI/status/support evidence and does not add sessions. [VERIFIED: .planning/REQUIREMENTS.md] |
| V4 Access Control | Yes | Keep datadir-owned soak ledger paths validated against selected datadir and refuse mismatched ledgers. [VERIFIED: packages/open-bitcoin-cli/src/operator/soak/runtime/helpers.rs] |
| V5 Input Validation | Yes | Parse budgets/paths into typed Rust contracts, reject zero budgets, unavailable required paths, and out-of-bounds measurements. [VERIFIED: packages/open-bitcoin-cli/src/operator/soak.rs; VERIFIED: .planning/phases/76-disk-and-resource-bound-enforcement/76-CONTEXT.md] |
| V6 Cryptography | No new crypto | Do not add cryptographic primitives; support evidence remains redacted local artifacts. [VERIFIED: packages/open-bitcoin-cli/src/operator/support.rs] |

### Known Threat Patterns for Resource-Bound Enforcement

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Disk/resource exhaustion during long soak | Denial of Service | Preflight refusal, warning/stop-required states, runtime re-checks, `resource_stop`, and `ResourceExhaustion` recovery category. [VERIFIED: .planning/phases/76-disk-and-resource-bound-enforcement/76-CONTEXT.md; VERIFIED: packages/open-bitcoin-node/src/status/recovery.rs] |
| Support bundle leaks raw sensitive artifacts | Information Disclosure | Keep support evidence allowlisted and omit raw logs, raw stores, raw reports, credentials, wallet material, and unbounded peer tables. [VERIFIED: packages/open-bitcoin-cli/src/operator/support.rs; VERIFIED: packages/open-bitcoin-cli/src/operator/soak/report.rs] |
| Path confusion between selected datadir and stale/moved report | Tampering / Repudiation | Treat ledger as datadir-owned source of truth and reports as projections with source path/latest sequence. [VERIFIED: packages/open-bitcoin-cli/src/operator/soak/ledger.rs; VERIFIED: packages/open-bitcoin-cli/src/operator/soak/report.rs] |
| Resource guidance hides mutation | Tampering | Guidance only; no silent delete/compact/repair/prune/relocate behavior. [VERIFIED: .planning/phases/76-disk-and-resource-bound-enforcement/76-CONTEXT.md] |
| Proof confusion: resource-bounded soak mistaken for production-node readiness | Spoofing / Repudiation | Preserve v1.7 release boundaries and docs wording that public-network soaks are explicit opt-in evidence only. [VERIFIED: .planning/ROADMAP.md; VERIFIED: docs/operator/runtime-guide.md] |

## Sources

### Primary (HIGH confidence)

- `.planning/phases/76-disk-and-resource-bound-enforcement/76-CONTEXT.md` - locked decisions, discretion, deferred scope, canonical refs. [VERIFIED: file read]
- `.planning/REQUIREMENTS.md` - RES-05 through RES-08 and v1.7 out-of-scope boundaries. [VERIFIED: file read]
- `.planning/STATE.md`, `.planning/ROADMAP.md`, `.planning/PROJECT.md`, `.planning/STACK.md`, `.planning/CONVENTIONS.md`, `.planning/ARCHITECTURE.md` - milestone state, project constraints, stack, and architecture. [VERIFIED: file reads]
- `AGENTS.md`, `AGENTS.bright-builds.md`, `standards/core/*.md`, `standards/languages/*.md` - repo and Bright Builds constraints. [VERIFIED: file reads]
- `packages/open-bitcoin-node/src/status.rs`, `metrics.rs`, `logging.rs`, `logging/prune.rs`, `logging/writer.rs`, `storage.rs`, `sync/runtime_state.rs`, `sync/progress.rs`, `sync/types/summary.rs` - existing status/resource/recovery/retention integration. [VERIFIED: file reads]
- `packages/open-bitcoin-cli/src/operator/status.rs`, `status/render.rs`, `dashboard/model.rs`, `runtime/support.rs`, `support.rs`, `support/evidence.rs`, `support/render.rs`, `soak.rs`, `soak/runtime.rs`, `soak/runtime/helpers.rs`, `soak/ledger.rs`, `soak/report.rs`, `soak/outcome.rs` - operator status, support, and soak surfaces. [VERIFIED: file reads]
- `scripts/verify.sh`, `scripts/check-phase75-soak-runner.ts`, `scripts/check-phase75-soak-runner.test.ts` - default verifier and checker style. [VERIFIED: file reads]
- `cargo info fs4`, `cargo search fs4`, `~/.cargo/registry/src/.../fs4-1.1.0/src/lib.rs` - filesystem stats dependency verification. [VERIFIED: cargo/crate source]

### Secondary (MEDIUM confidence)

- Crates.io API with explicit User-Agent for latest version and publish/update dates. [VERIFIED: crates.io API]
- `cargo info sysinfo` and `cargo search sysinfo` for alternative evaluation. [VERIFIED: cargo info/search]

### Tertiary (LOW confidence)

- No tertiary sources were used. [VERIFIED: source audit]

## Metadata

**Confidence breakdown:**

- Standard stack: HIGH for existing locked stack and fs4 version, because versions were verified from local manifests/lockfile, installed tools, cargo metadata, and crates.io. [VERIFIED: packages/Cargo.lock; VERIFIED: cargo info fs4; VERIFIED: crates.io API]
- Architecture: HIGH for codebase integration surfaces, because canonical Phase 76 refs and current code agree on shared status/soak/support patterns. [VERIFIED: 76-CONTEXT.md; VERIFIED: packages/open-bitcoin-node/src/status.rs; VERIFIED: packages/open-bitcoin-cli/src/operator/soak/runtime.rs]
- Pitfalls: HIGH for shared-status, redaction, no-hidden-mutation, and deterministic verification risks; MEDIUM for threshold flapping because it is a general filesystem behavior rather than a repo-specific policy. [VERIFIED: 76-CONTEXT.md; ASSUMED]

**Research date:** 2026-06-15
**Valid until:** 2026-07-15 for codebase architecture; 2026-06-22 for crate-version currency.
