# Phase 75: Multi-Day Soak Runner and Evidence Ledger - Research

**Researched:** 2026-06-14
**Domain:** operator CLI soak workflow, durable evidence ledger, sync runtime evidence, deterministic synthetic soak testing
**Confidence:** HIGH

<user_constraints>
## User Constraints (from CONTEXT.md)

Source for all copied constraints in this section: `.planning/phases/75-multi-day-soak-runner-and-evidence-ledger/75-CONTEXT.md`. [VERIFIED: .planning/phases/75-multi-day-soak-runner-and-evidence-ledger/75-CONTEXT.md]

### Locked Decisions

### Operator Invocation And Bounds

- **D-01:** Make `open-bitcoin soak` the stable operator-facing entrypoint for
  Phase 75. Keep it explicit opt-in, surfaced through repo-local Cargo and
  Bazel command forms, and separate from default verification.
- **D-02:** Use a layered contract: `open-bitcoin soak` owns argument parsing,
  run identity, ledger/report paths, resume mode, and final operator output;
  `open-bitcoind` and Open Bitcoin runtime/config inputs remain authoritative
  for daemon-owned sync bounds such as network, target height, peer policy, and
  runtime stop behavior.
- **D-03:** Treat `scripts/run-live-mainnet-smoke.ts` as a compatibility,
  fixture, or opt-in evidence helper rather than the durable soak product
  surface. Reuse its report, preflight, and deterministic fixture lessons where
  useful, but do not grow it into the primary multi-day soak runner.
- **D-04:** The soak command should accept or derive bounded elapsed time,
  target height, datadir, network, peer policy, disk budget, and stop conditions
  without creating hidden public-network defaults or implicit source-datadir
  mutation.

### Durable Evidence Ledger

- **D-05:** Use a hybrid evidence model: a small datadir-owned run index or
  current-run pointer anchors durable identity and resume ownership, while a
  typed append-only JSONL event ledger records started, checkpoint, resume,
  stop, and verdict events.
- **D-06:** Derive shareable JSON and Markdown reports from the ledger. Reports
  are operator artifacts, not the source of truth; stale or moved reports must
  not be mistaken for current durable state.
- **D-07:** Support bundles may include a compact, redacted soak summary derived
  from the ledger, but support bundles are projections only. They must not
  become the primary ledger and must not embed raw daemon logs, raw reports,
  wallet material, credentials, unbounded peer tables, or automatic uploads.
- **D-08:** Ledger writes should be typed, versioned, bounded, and resilient to
  partial/interrupted runs. Planning should define atomic write behavior,
  retention or compaction boundaries, and how the run index detects the latest
  resumable run.

### Run Outcome And Resume Taxonomy

- **D-09:** Add a soak-owned run outcome vocabulary for Phase 75 rather than
  overloading `SyncStopReason` or `SyncRecoveryCategory`. Required final labels
  are clean completion, diagnosed blocker, operator stop, resource stop,
  recovery stop, and unexpected termination.
- **D-10:** Every soak outcome must carry source evidence from existing shared
  contracts where possible: `SyncStopReasonStatus` for bounded sync stops,
  `SyncRecoveryCategory` for recovery or resource classes,
  `NoProgressDiagnosis` for blocker detail, `EvidenceVerdictSummary` for proof
  versus diagnosed blocker, and process/cancellation facts for operator stop or
  unexpected termination.
- **D-11:** Resume rules should be explicit. Clean completion should close the
  run and not resume as the same run. Operator, resource, and recovery stops may
  resume only through an explicit same-run resume record with preserved datadir
  and run identity. Unexpected termination should resume as interrupted-run
  recovery evidence, never as a clean stop.
- **D-12:** Keep the soak vocabulary shallow in Phase 75. Later phases own
  deeper resource-bound classification, corruption/lock recovery detail,
  progress guarantees, and support-bundle forensics.

### Deterministic Synthetic Coverage

- **D-13:** Use mixed deterministic coverage, with Rust tests as the canonical
  behavioral proof. Reuse `DurableSyncRuntime`, scripted transport/resolver,
  explicit timestamps or scripted clocks, durable reopen fixtures, and
  synthetic long-chain patterns to prove long-run control flow without public
  peers or wall-clock multi-day waits.
- **D-14:** Add a thin operator-level harness only for user-facing command and
  report behavior: argument validation, run identity paths, interrupted/resumed
  report behavior, and final output.
- **D-15:** Add a focused Bun checker when docs, report fixtures, parity roots,
  or default-verification boundaries need auditing. The checker should follow
  Phase 68 through Phase 74 patterns and remain local, short-running,
  public-network-free, service-manager-free, and timing-stable.
- **D-16:** Avoid timer-virtualization complexity unless planning proves it is
  needed. The existing sync paths already accept explicit timestamps in many
  places, so scripted clocks and deterministic fixture inputs are the preferred
  first path.

### Operator Guidance And Scope Boundaries

- **D-17:** Operator docs must describe what the soak evidence proves and what
  it does not prove. A soak run can prove bounded opt-in full-sync soak
  behavior, durable resume evidence, or diagnosed blocker evidence; it does not
  prove inbound serving, relay, production-funds wallet safety, migration apply
  mode, signed packages, GUI readiness, hosted dashboards, or broad
  production-node readiness.
- **D-18:** UAT commands should use repo-local Cargo and Bazel forms for
  operator CLI workflows. Avoid bare installed-alias instructions unless the
  user explicitly asks for them.
- **D-19:** If new first-party Rust source or test files are added under
  `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests`, update
  `docs/parity/source-breadcrumbs.json` and keep the breadcrumb checker green.

### the agent's Discretion

- The planner may split Phase 75 into operator CLI contract, soak ledger/domain
  model, report/support summary projection, deterministic runtime tests,
  operator docs, and checker/parity closeout.
- The executor may add small pure domain types for soak bounds, run identity,
  event records, and final outcomes when they make illegal states
  unrepresentable.
- The executor may keep the first support-bundle integration minimal: expose a
  redacted soak summary only after the ledger/report source of truth is typed
  and tested.

### Deferred Ideas (OUT OF SCOPE)

- Scheduled public-network soak monitors remain future SOAK-05 scope.
- Signed externally comparable soak result artifacts remain future SOAK-06
  scope.
- Deep disk/resource bound enforcement belongs to Phase 76.
- Corruption and lock recovery hardening belongs to Phase 77.
- Progress guarantees and stall diagnosis belongs to Phase 78.
- Full support-bundle forensics and failure narratives belong to Phase 79.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| SOAK-01 | Operator can run an explicit opt-in full-sync soak for multiple days with durable run identity, start and end checkpoints, and resumable report state. | Add `open-bitcoin soak` under the existing operator CLI, write a datadir-owned run index plus append-only ledger, and make reports projections from that ledger. [VERIFIED: .planning/REQUIREMENTS.md; VERIFIED: packages/open-bitcoin-cli/src/operator.rs; VERIFIED: .planning/phases/75-multi-day-soak-runner-and-evidence-ledger/75-CONTEXT.md] |
| SOAK-02 | Operator can bound a soak by elapsed time, target height, datadir, network, peer policy, disk budget, and stop condition without changing default verification. | Keep daemon sync bounds authoritative in `open-bitcoind`/runtime config, accept/derive soak bounds in the operator command, and keep public-network/multi-day execution out of `scripts/verify.sh`. [VERIFIED: .planning/REQUIREMENTS.md; VERIFIED: packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs; VERIFIED: scripts/verify.sh] |
| SOAK-03 | Operator can distinguish clean completion, diagnosed blocker, operator stop, resource stop, recovery stop, and unexpected termination in soak evidence. | Add a soak-owned outcome enum that wraps `SyncStopReasonStatus`, `SyncRecoveryCategory`, `NoProgressDiagnosis`, support evidence verdicts, and process facts instead of extending sync enums directly. [VERIFIED: .planning/REQUIREMENTS.md; VERIFIED: packages/open-bitcoin-node/src/status.rs; VERIFIED: packages/open-bitcoin-cli/src/operator/support/evidence.rs] |
| SOAK-04 | Contributor can replay deterministic synthetic soak scenarios that exercise long-run control flow without public-network access or wall-clock multi-day tests. | Reuse `DurableSyncRuntime`, scripted transport/resolver tests, explicit timestamps, durable reopen fixtures, and optional local Bun checkers rather than real public peers or long sleeps. [VERIFIED: .planning/REQUIREMENTS.md; VERIFIED: packages/open-bitcoin-node/src/sync.rs; VERIFIED: packages/open-bitcoin-node/src/sync/tests.rs; VERIFIED: scripts/verify.sh] |
</phase_requirements>

## Summary

Phase 75 should be planned as an operator-orchestration and evidence phase, not as a new sync engine phase. [VERIFIED: .planning/phases/75-multi-day-soak-runner-and-evidence-ledger/75-CONTEXT.md] The existing `open-bitcoin` operator CLI already has the command parser, runtime dispatch, config resolution, status collection, sync pause/resume, and support-bundle surfaces where the new soak command can integrate. [VERIFIED: packages/open-bitcoin-cli/src/operator.rs; VERIFIED: packages/open-bitcoin-cli/src/operator/runtime.rs; VERIFIED: packages/open-bitcoin-cli/src/operator/runtime/support.rs; VERIFIED: packages/open-bitcoin-cli/src/operator/support.rs]

The durable source of truth should be a datadir-owned run index plus typed JSONL ledger, with JSON and Markdown reports derived from ledger events. [VERIFIED: .planning/phases/75-multi-day-soak-runner-and-evidence-ledger/75-CONTEXT.md] The ledger should record `started`, `checkpoint`, `resume`, `stop`, and `verdict` events, while compact support summaries remain redacted projections and must not copy raw daemon logs, raw reports, credentials, wallet material, or unbounded peer tables. [VERIFIED: .planning/phases/75-multi-day-soak-runner-and-evidence-ledger/75-CONTEXT.md; VERIFIED: docs/operator/runtime-guide.md; VERIFIED: packages/open-bitcoin-cli/src/operator/support/live_smoke.rs]

Deterministic Rust tests should be the canonical proof for long-run control flow. [VERIFIED: .planning/phases/75-multi-day-soak-runner-and-evidence-ledger/75-CONTEXT.md] The codebase already has `DurableSyncRuntime`, injectable transport/resolver paths, explicit timestamp parameters, durable sync state persistence, and synthetic sync tests; Phase 75 should compose those pieces into soak lifecycle tests without public peers or multi-day wall-clock waits. [VERIFIED: packages/open-bitcoin-node/src/sync.rs; VERIFIED: packages/open-bitcoin-node/src/sync/runtime_state.rs; VERIFIED: packages/open-bitcoin-node/src/sync/tests.rs; VERIFIED: scripts/verify.sh]

**Primary recommendation:** Plan Phase 75 as six workstreams: operator CLI contract, pure soak domain/ledger, report/support projection, deterministic Rust lifecycle tests, operator docs/UAT commands, and a small Bun boundary checker if docs or default-verification exclusions need machine enforcement. [VERIFIED: .planning/phases/75-multi-day-soak-runner-and-evidence-ledger/75-CONTEXT.md; VERIFIED: scripts/verify.sh]

## Project Constraints (from AGENTS.md)

- Follow Bright Builds routing before plan/review/implementation by reading repo-local `AGENTS.md`, `AGENTS.bright-builds.md`, `standards-overrides.md`, and relevant standards under `standards/`. [VERIFIED: AGENTS.md; VERIFIED: AGENTS.bright-builds.md; VERIFIED: standards/index.md]
- Use `bash scripts/verify.sh` as the repo-native verification contract for first-party code, including the Bazel smoke build. [VERIFIED: AGENTS.md; VERIFIED: scripts/verify.sh]
- Keep public-network and multi-day checks opt-in and outside default deterministic verification. [VERIFIED: AGENTS.md; VERIFIED: .planning/STATE.md; VERIFIED: scripts/verify.sh]
- Use repo-local Cargo and Bazel command forms for UAT and operator workflows, especially `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- ...` and `bazel run //packages/open-bitcoin-cli:open_bitcoin -- ...`. [VERIFIED: AGENTS.md; VERIFIED: docs/operator/runtime-guide.md]
- Keep functional-core and imperative-shell boundaries: pure business logic should stay free of direct I/O, while filesystem/process/network effects belong in adapters. [VERIFIED: AGENTS.md; VERIFIED: standards/core/architecture.md]
- Prefer minimal dependencies and do not use existing Rust Bitcoin libraries in the production path. [VERIFIED: AGENTS.md]
- When adding first-party Rust source or test files under `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests`, update `docs/parity/source-breadcrumbs.json` and keep `scripts/check-parity-breadcrumbs.ts --check` green. [VERIFIED: AGENTS.md; VERIFIED: docs/parity/source-breadcrumbs.json; VERIFIED: scripts/check-parity-breadcrumbs.ts]
- Use Bun as the canonical runtime for repo-owned TypeScript automation scripts and prefer Bash only for thin orchestration wrappers or simple shell checks. [VERIFIED: AGENTS.md; VERIFIED: standards/languages/typescript-javascript.md]
- For Rust code, use Rust 2024, `thiserror` for library errors, `anyhow` only for application errors when present, `tracing` rather than `println!`, no `unwrap()`, `let ... else` for early returns, and `maybe_` prefixes for optional values. [VERIFIED: AGENTS.md; VERIFIED: standards/languages/rust.md]
- Unit tests should test one concern, prefer Arrange/Act/Assert structure, and use explicit comments when the structure is not trivial. [VERIFIED: AGENTS.md; VERIFIED: standards/core/testing.md]
- After substantial feature, parity, operator-surface, or workflow changes, check whether relevant README files need updates. [VERIFIED: AGENTS.md]

## Standard Stack

### Core

| Library / Tool | Version | Purpose | Why Standard |
|----------------|---------|---------|--------------|
| Rust toolchain | 1.94.1 | First-party CLI, node, RPC, domain, ledger, and tests | The repo pins Rust 1.94.1 in `rust-toolchain.toml`, Cargo workspace metadata, and Bazel toolchain config. [VERIFIED: rust-toolchain.toml; VERIFIED: packages/Cargo.toml; VERIFIED: MODULE.bazel] |
| Cargo workspace | Rust 2024 edition | Builds `open-bitcoin-cli`, `open-bitcoin-node`, `open-bitcoin-rpc`, and tests | `packages/Cargo.toml` defines the workspace and Rust 2024 edition for all first-party crates. [VERIFIED: packages/Cargo.toml] |
| Bazel / rules_rust | Bazel 8.6.0, `rules_rust` 0.69.0 | Repo-root smoke build and operator UAT command form | `MODULE.bazel` configures `rules_rust`, Cargo lock import, and Rust 1.94.1; local environment has Bazel 8.6.0. [VERIFIED: MODULE.bazel; VERIFIED: environment audit] |
| `clap` | 4.6.1 | Operator CLI parsing for `open-bitcoin soak` | `open-bitcoin-cli` already uses `clap` derive for `OperatorCli`, subcommands, and value enums. [VERIFIED: packages/open-bitcoin-cli/Cargo.toml; VERIFIED: packages/open-bitcoin-cli/src/operator.rs] |
| `serde` / `serde_json` | `serde` 1.0.228, `serde_json` 1.0.149 | Typed JSONL ledger events, JSON report projection, support summary projection | Support evidence and status contracts already serialize typed Rust data with `serde`/`serde_json`. [VERIFIED: packages/open-bitcoin-cli/Cargo.toml; VERIFIED: packages/open-bitcoin-cli/src/operator/support.rs; VERIFIED: packages/open-bitcoin-cli/src/operator/support/evidence.rs] |
| `open-bitcoin-node` sync/status contracts | Workspace 0.1.0 | Durable sync state, `OpenBitcoinStatusSnapshot`, stop/recovery/progress source evidence | `DurableSyncRuntime`, `SyncRunSummary`, `SyncStopReason`, and `SyncStatus` already carry authoritative sync evidence. [VERIFIED: packages/open-bitcoin-node/src/sync.rs; VERIFIED: packages/open-bitcoin-node/src/sync/types.rs; VERIFIED: packages/open-bitcoin-node/src/status.rs] |
| `open-bitcoin-rpc` daemon/config contracts | Workspace 0.1.0 | `open-bitcoind` daemon sync activation and runtime config ownership | `open-bitcoind` opens the durable store, constructs `DurableSyncRuntime`, seeds sync state, and runs bounded daemon sync cycles when sync is enabled. [VERIFIED: packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs] |

### Supporting

| Library / Tool | Version | Purpose | When to Use |
|----------------|---------|---------|-------------|
| Fjall | 3.1.4 | Existing durable node store and runtime metadata | Use it as the source for sync/runtime metadata; do not store the soak event ledger inside runtime metadata unless planning proves that coupling is necessary. [VERIFIED: packages/open-bitcoin-node/Cargo.toml; VERIFIED: packages/open-bitcoin-node/src/storage/fjall_store.rs] |
| `thiserror` | 2.0.18 in lockfile, `2.0.12` semver in CLI manifest | Structured errors for new soak domain/ledger modules | Use for new library-style operator errors because repo Rust guidance prefers `thiserror`. [VERIFIED: packages/Cargo.lock; VERIFIED: packages/open-bitcoin-cli/Cargo.toml; VERIFIED: AGENTS.md] |
| Bun | 1.3.9 | Optional deterministic checker for docs/report/default-verification boundaries | Use only if Rust tests do not cover documentation, fixture, parity-root, or `scripts/verify.sh` boundary assertions. [VERIFIED: .bun-version; VERIFIED: environment audit; VERIFIED: scripts/verify.sh] |
| `scripts/verify.sh` | repo script | Aggregate deterministic verification | Keep Phase 75 default checks local, fast, public-network-free, and service-manager-free. [VERIFIED: scripts/verify.sh; VERIFIED: .planning/STATE.md] |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| `open-bitcoin soak` operator command | Grow `scripts/run-live-mainnet-smoke.ts` into the product surface | Rejected by locked decision D-03; the script remains a helper/fixture surface, not durable soak orchestration. [VERIFIED: .planning/phases/75-multi-day-soak-runner-and-evidence-ledger/75-CONTEXT.md] |
| Datadir index plus JSONL ledger | JSON/Markdown report as durable state | Rejected by locked decisions D-05 and D-06 because reports are projections and may become stale or moved. [VERIFIED: .planning/phases/75-multi-day-soak-runner-and-evidence-ledger/75-CONTEXT.md] |
| Soak-owned outcome enum | Extend or overload `SyncStopReason` / `SyncRecoveryCategory` | Rejected by locked decision D-09; the soak outcome must wrap existing evidence rather than mutate lower-level sync vocabulary. [VERIFIED: .planning/phases/75-multi-day-soak-runner-and-evidence-ledger/75-CONTEXT.md] |
| Deterministic Rust lifecycle tests | Public peers or multi-day sleeps in default verification | Rejected by SOAK-04 and repo verification boundaries. [VERIFIED: .planning/REQUIREMENTS.md; VERIFIED: scripts/verify.sh] |

**Installation:** No new external package is recommended by default for Phase 75; plan with the existing workspace dependencies first. [VERIFIED: AGENTS.md; VERIFIED: packages/open-bitcoin-cli/Cargo.toml; VERIFIED: packages/open-bitcoin-node/Cargo.toml]

**Version verification:** The local audit found Rust/Cargo 1.94.1, Bun 1.3.9, Bazel 8.6.0, Git 2.53.0, and cargo-llvm-cov 0.8.5 available on the machine. [VERIFIED: environment audit]

## Architecture Patterns

### Recommended Project Structure

Recommended new files use `soak.rs` plus a `soak/` child directory if the implementation would otherwise exceed responsibility or file-size guidance. [VERIFIED: standards/languages/rust.md; VERIFIED: standards/core/code-shape.md]

```text
packages/open-bitcoin-cli/src/
|-- operator.rs                 # add OperatorCommand::Soak and SoakArgs wiring
|-- operator/runtime.rs         # dispatch OperatorCommand::Soak to soak executor
|-- operator/soak.rs            # command entrypoint, pure validation boundary
|-- operator/soak/
|   |-- ledger.rs               # run index, JSONL event writer/reader, resume scan
|   |-- outcome.rs              # soak-owned final outcome taxonomy and evidence links
|   |-- report.rs               # JSON/Markdown projections from ledger events
|   `-- tests.rs                # pure soak domain/ledger unit tests
`-- tests/operator_flows.rs     # existing integration harness or a new soak flow file
```

The exact `operator/soak/` split is a planner recommendation, not an existing repo fact. [ASSUMED]

### Pattern 1: Operator Command Integration

**What:** Add a `Soak(SoakArgs)` variant to `OperatorCommand`, add `pub mod soak;`, and dispatch it from `execute_operator_command` after config resolution and detection setup. [VERIFIED: packages/open-bitcoin-cli/src/operator.rs; VERIFIED: packages/open-bitcoin-cli/src/operator/runtime.rs]

**When to use:** Use this path for all `open-bitcoin soak` subcommands because `open-bitcoin` is the operator binary and `open-bitcoin-cli` remains the Bitcoin CLI compatibility route. [VERIFIED: packages/open-bitcoin-cli/src/operator.rs]

**Example:**

```rust
// Recommended shape based on the existing SyncArgs and SupportArgs patterns.
// Source pattern: packages/open-bitcoin-cli/src/operator.rs
#[derive(Debug, Clone, PartialEq, Eq, Args)]
pub struct SoakArgs {
    #[command(subcommand)]
    pub command: SoakCommand,
}

#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
pub enum SoakCommand {
    Start(SoakStartArgs),
    Resume(SoakResumeArgs),
    Report(SoakReportArgs),
}
```

The `start`, `resume`, and `report` subcommands are a recommended CLI shape, not a locked decision. [ASSUMED]

### Pattern 2: Layered Authority For Bounds

**What:** `open-bitcoin soak` should validate and record operator-requested bounds, but `open-bitcoind` and runtime config should remain authoritative for daemon sync behavior such as network, target height, peer policy, and daemon stop behavior. [VERIFIED: .planning/phases/75-multi-day-soak-runner-and-evidence-ledger/75-CONTEXT.md; VERIFIED: packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs]

**When to use:** Use this boundary whenever the soak command starts, observes, stops, or resumes a daemon-backed sync run. [VERIFIED: .planning/phases/75-multi-day-soak-runner-and-evidence-ledger/75-CONTEXT.md]

**Implementation note:** The soak executor should record the parsed request, resolved datadir, network, peer policy, target height, elapsed-time cap, disk budget, and stop policy in the ledger, then poll or collect `OpenBitcoinStatusSnapshot` for evidence rather than independently deriving sync success. [VERIFIED: packages/open-bitcoin-node/src/status.rs; VERIFIED: docs/architecture/operator-observability.md]

### Pattern 3: Append-Only Event Ledger With Projection Reports

**What:** Store the current run pointer/index under the selected datadir and append versioned JSONL ledger events for `started`, `checkpoint`, `resume`, `stop`, and `verdict`. [VERIFIED: .planning/phases/75-multi-day-soak-runner-and-evidence-ledger/75-CONTEXT.md]

**Recommended paths:** Use `<datadir>/soak/run-index.json`, `<datadir>/soak/runs/<run_id>/events.jsonl`, `<datadir>/soak/runs/<run_id>/report.json`, and `<datadir>/soak/runs/<run_id>/report.md`. [ASSUMED]

**Atomicity pattern:** Append ledger events as full JSON lines with sequence numbers, flush the event file after source-of-truth writes, and update the run index through a temporary file plus rename so interrupted updates are detectable. [ASSUMED]

**Resume pattern:** On resume, read the run index, validate same datadir identity, scan the ledger to the last complete line, classify an unterminated prior run as interrupted evidence, append a `resume` event, and never convert unexpected termination into clean completion. [VERIFIED: .planning/phases/75-multi-day-soak-runner-and-evidence-ledger/75-CONTEXT.md]

### Pattern 4: Outcome Vocabulary Wraps Shared Evidence

**What:** Create a shallow soak outcome enum with final labels `clean_completion`, `diagnosed_blocker`, `operator_stop`, `resource_stop`, `recovery_stop`, and `unexpected_termination`. [VERIFIED: .planning/phases/75-multi-day-soak-runner-and-evidence-ledger/75-CONTEXT.md]

**When to use:** Use the soak outcome only at the soak run/report layer; keep `SyncStopReason`, `SyncRecoveryCategory`, `NoProgressDiagnosis`, and support evidence verdicts as source evidence fields. [VERIFIED: .planning/phases/75-multi-day-soak-runner-and-evidence-ledger/75-CONTEXT.md; VERIFIED: packages/open-bitcoin-node/src/sync/types.rs; VERIFIED: packages/open-bitcoin-node/src/status.rs; VERIFIED: packages/open-bitcoin-cli/src/operator/support/evidence.rs]

**Example:**

```rust
// Recommended shape; exact field names should follow serde snake_case output.
// Source evidence exists in packages/open-bitcoin-node/src/status.rs and
// packages/open-bitcoin-cli/src/operator/support/evidence.rs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SoakOutcomeLabel {
    CleanCompletion,
    DiagnosedBlocker,
    OperatorStop,
    ResourceStop,
    RecoveryStop,
    UnexpectedTermination,
}
```

This enum is a recommended implementation shape inferred from locked vocabulary decisions. [ASSUMED]

### Anti-Patterns to Avoid

- **Report-as-state:** Do not treat JSON/Markdown reports as durable source of truth; reports are projections from the ledger. [VERIFIED: .planning/phases/75-multi-day-soak-runner-and-evidence-ledger/75-CONTEXT.md]
- **Hidden public-network defaults:** Do not start mainnet/public peers from default verification or implicit CLI defaults. [VERIFIED: .planning/STATE.md; VERIFIED: scripts/verify.sh]
- **Sync enum overloading:** Do not add soak-only outcome semantics directly to `SyncStopReason` or `SyncRecoveryCategory`. [VERIFIED: .planning/phases/75-multi-day-soak-runner-and-evidence-ledger/75-CONTEXT.md]
- **Status reinterpretation:** Do not independently reinterpret header height, downloaded block height, connected block height, validated active-chain height, best-known tip, stay-current, no-progress diagnosis, latest stop reason, or recovery category outside `OpenBitcoinStatusSnapshot`. [VERIFIED: docs/architecture/operator-observability.md; VERIFIED: docs/architecture/status-snapshot.md]
- **Raw support leakage:** Do not embed raw daemon logs, raw reports, wallet material, credentials, or unbounded peer tables in support summaries. [VERIFIED: .planning/phases/75-multi-day-soak-runner-and-evidence-ledger/75-CONTEXT.md; VERIFIED: docs/operator/runtime-guide.md]

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Full-sync execution | A new soak-specific sync loop | `open-bitcoind` plus `DurableSyncRuntime` and runtime config | Existing daemon code already owns durable sync activation, store opening, lifecycle seeding, shutdown, pause, retry, and bounded cycles. [VERIFIED: packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs; VERIFIED: packages/open-bitcoin-node/src/sync.rs] |
| Sync success classification | Renderer-local progress heuristics | `OpenBitcoinStatusSnapshot` and `derive_full_sync_evidence` | Existing support evidence distinguishes validated active-chain progress, best-known tip, stay-current state, blocker evidence, and inconclusive evidence. [VERIFIED: packages/open-bitcoin-cli/src/operator/support/evidence.rs; VERIFIED: docs/architecture/operator-observability.md] |
| CLI parser | Manual argument parsing | `clap` derive on `OperatorCli`/subcommands | Existing operator commands use `clap` `Args`, `Subcommand`, and `ValueEnum`. [VERIFIED: packages/open-bitcoin-cli/src/operator.rs] |
| JSON/JSONL encoding | String-concatenated JSON | `serde` and `serde_json` | Existing support bundles serialize typed structs with `serde_json::to_string_pretty`. [VERIFIED: packages/open-bitcoin-cli/src/operator/support.rs] |
| Public-network test proof | Real peers, DNS, or multi-day sleeps in tests | Scripted Rust transports/resolvers and explicit timestamps | Existing sync runtime paths accept injected transport/resolver and timestamp inputs, and default verify remains public-network-free. [VERIFIED: packages/open-bitcoin-node/src/sync.rs; VERIFIED: scripts/verify.sh] |
| Support bundle redaction | Raw copy of ledger/report/log files | Compact allowlisted soak summary | Existing live-smoke support projection copies allowlisted summary keys instead of raw report payloads. [VERIFIED: packages/open-bitcoin-cli/src/operator/support/live_smoke.rs; VERIFIED: docs/operator/runtime-guide.md] |

**Key insight:** The hard part of Phase 75 is preserving authority boundaries and durable evidence semantics, not inventing another sync engine. [VERIFIED: .planning/phases/75-multi-day-soak-runner-and-evidence-ledger/75-CONTEXT.md; VERIFIED: packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs]

## Common Pitfalls

### Pitfall 1: Report-Only Durability

**What goes wrong:** A run can be interrupted after a report write or before a report write, leaving the operator with stale or missing evidence. [VERIFIED: .planning/phases/75-multi-day-soak-runner-and-evidence-ledger/75-CONTEXT.md]

**Why it happens:** JSON/Markdown reports are convenient operator artifacts but are not durable ownership records. [VERIFIED: .planning/phases/75-multi-day-soak-runner-and-evidence-ledger/75-CONTEXT.md]

**How to avoid:** Treat the run index and JSONL ledger as source of truth, then derive reports idempotently from ledger events. [VERIFIED: .planning/phases/75-multi-day-soak-runner-and-evidence-ledger/75-CONTEXT.md]

**Warning signs:** Planner tasks mention report files but omit run index, event sequencing, partial-line handling, or resume scanning. [ASSUMED]

### Pitfall 2: Hidden Mainnet Or Default Verification Drift

**What goes wrong:** A deterministic local check accidentally starts public-network work or requires multi-day timing. [VERIFIED: .planning/STATE.md; VERIFIED: scripts/verify.sh]

**Why it happens:** Soak and live-smoke workflows are adjacent to public-network UAT, but default verification must remain hermetic. [VERIFIED: .planning/STATE.md; VERIFIED: scripts/verify.sh]

**How to avoid:** Keep actual public-network soaks behind explicit operator commands and make `scripts/verify.sh` run only synthetic tests/checkers. [VERIFIED: .planning/REQUIREMENTS.md; VERIFIED: scripts/verify.sh]

**Warning signs:** `scripts/verify.sh` gains `manual-peer`, live-mainnet smoke invocation, real service-manager commands, or long sleeps. [VERIFIED: scripts/test-run-live-mainnet-smoke.sh; VERIFIED: scripts/verify.sh]

### Pitfall 3: Overloading Existing Sync Vocabulary

**What goes wrong:** A soak-specific `resource_stop` or `unexpected_termination` becomes indistinguishable from lower-level sync stop and recovery states. [VERIFIED: .planning/phases/75-multi-day-soak-runner-and-evidence-ledger/75-CONTEXT.md]

**Why it happens:** `SyncStopReason` and `SyncRecoveryCategory` already exist and may look like the natural place for labels. [VERIFIED: packages/open-bitcoin-node/src/sync/types.rs; VERIFIED: packages/open-bitcoin-node/src/status/recovery.rs]

**How to avoid:** Use a separate `SoakOutcomeLabel` with explicit source evidence fields referencing sync stop, recovery, no-progress, support verdict, and process facts. [VERIFIED: .planning/phases/75-multi-day-soak-runner-and-evidence-ledger/75-CONTEXT.md]

**Warning signs:** Plan tasks modify `SyncStopReason` just to add soak operator outcomes. [ASSUMED]

### Pitfall 4: False Progress Credit

**What goes wrong:** A soak can appear successful from header or download movement even when validated active-chain progress has not been proven. [VERIFIED: docs/architecture/status-snapshot.md; VERIFIED: docs/architecture/operator-observability.md]

**Why it happens:** Status surfaces carry multiple progress dimensions, and only some represent validated active-chain proof. [VERIFIED: docs/architecture/status-snapshot.md; VERIFIED: packages/open-bitcoin-node/src/status.rs]

**How to avoid:** Base final verdicts on shared status/support evidence and preserve unavailable reasons for validated active-chain fields. [VERIFIED: packages/open-bitcoin-cli/src/operator/support/evidence.rs; VERIFIED: docs/architecture/operator-observability.md]

**Warning signs:** Report prose says "synced" from `header_height` or `downloaded_block_height` alone. [VERIFIED: docs/architecture/status-snapshot.md]

### Pitfall 5: Unsafe Resume Ownership

**What goes wrong:** A resume command accidentally attaches to a different datadir or resumes a run that already ended cleanly. [VERIFIED: .planning/phases/75-multi-day-soak-runner-and-evidence-ledger/75-CONTEXT.md]

**Why it happens:** Run identity and datadir ownership are not naturally encoded by reports or daemon status alone. [VERIFIED: .planning/phases/75-multi-day-soak-runner-and-evidence-ledger/75-CONTEXT.md]

**How to avoid:** Store run identity in the datadir-owned index, validate the selected datadir on resume, refuse same-run resume after clean completion, and append a `resume` event for interrupted recovery. [VERIFIED: .planning/phases/75-multi-day-soak-runner-and-evidence-ledger/75-CONTEXT.md]

**Warning signs:** Resume mode accepts only a report path or latest report without checking datadir-owned run identity. [ASSUMED]

### Pitfall 6: New Rust Files Without Breadcrumbs

**What goes wrong:** `scripts/check-parity-breadcrumbs.ts --check` fails after new operator or test files are tracked. [VERIFIED: AGENTS.md; VERIFIED: scripts/check-parity-breadcrumbs.ts]

**Why it happens:** The breadcrumb checker includes `packages/open-bitcoin-*/src/**/*.rs` and `packages/open-bitcoin-*/tests/**/*.rs`. [VERIFIED: docs/parity/source-breadcrumbs.json; VERIFIED: scripts/check-parity-breadcrumbs.ts]

**How to avoid:** Add new soak files to `docs/parity/source-breadcrumbs.json`, using explicit Knots anchors where defensible or the existing `none` reason for Open Bitcoin-only support/infrastructure. [VERIFIED: AGENTS.md; VERIFIED: docs/parity/source-breadcrumbs.json]

**Warning signs:** New files under `packages/open-bitcoin-cli/src/operator/soak*.rs` are absent from the breadcrumb mapping. [ASSUMED]

## Code Examples

Verified patterns from existing source and recommended Phase 75 shapes:

### Dispatch A New Operator Subcommand

```rust
// Source pattern: packages/open-bitcoin-cli/src/operator/runtime.rs
match &cli.command {
    OperatorCommand::Status(_) => execute_status(&cli, config_resolution, detections),
    OperatorCommand::Sync(args) => execute_sync_command(args, cli.format, &config_resolution),
    OperatorCommand::Soak(args) => {
        execute_soak_command(args, &cli, config_resolution, detections)
    }
    // existing arms stay unchanged
}
```

This dispatch example is a recommended adaptation of the existing operator runtime match. [VERIFIED: packages/open-bitcoin-cli/src/operator/runtime.rs; ASSUMED]

### Ledger Event Shape

```rust
// Recommended JSONL event envelope. Source requirements: D-05, D-08, SOAK-01.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum SoakLedgerEvent {
    Started(SoakStartedEvent),
    Checkpoint(SoakCheckpointEvent),
    Resume(SoakResumeEvent),
    Stop(SoakStopEvent),
    Verdict(SoakVerdictEvent),
}
```

This event shape is a planner recommendation derived from the locked ledger event list. [VERIFIED: .planning/phases/75-multi-day-soak-runner-and-evidence-ledger/75-CONTEXT.md; ASSUMED]

### Outcome Classification Inputs

```rust
// Recommended source-evidence carrier, not an existing type.
pub struct SoakOutcomeEvidence {
    pub maybe_sync_stop_reason: Option<SyncStopReasonStatus>,
    pub maybe_recovery_category: Option<SyncRecoveryCategory>,
    pub maybe_no_progress_diagnosis: Option<NoProgressDiagnosis>,
    pub maybe_support_verdict: Option<EvidenceVerdictSummary>,
    pub maybe_process_exit: Option<SoakProcessExitEvidence>,
}
```

This example reflects the required evidence sources listed in D-10 and existing status/support evidence types. [VERIFIED: .planning/phases/75-multi-day-soak-runner-and-evidence-ledger/75-CONTEXT.md; VERIFIED: packages/open-bitcoin-node/src/status.rs; VERIFIED: packages/open-bitcoin-cli/src/operator/support/evidence.rs; ASSUMED]

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Opt-in live-mainnet smoke reports as local UAT evidence | Durable soak product surface under `open-bitcoin soak`, with live-smoke retained only as helper/fixture evidence | Phase 75 context, 2026-06-14 | Do not grow `scripts/run-live-mainnet-smoke.ts` into the product runner. [VERIFIED: .planning/phases/75-multi-day-soak-runner-and-evidence-ledger/75-CONTEXT.md; VERIFIED: scripts/run-live-mainnet-smoke.ts] |
| Report or support bundle as evidence carrier | Datadir-owned run index plus append-only JSONL event ledger as source of truth, with reports/support as projections | Phase 75 context, 2026-06-14 | Plan storage/resume semantics before report rendering. [VERIFIED: .planning/phases/75-multi-day-soak-runner-and-evidence-ledger/75-CONTEXT.md] |
| Sync stop/recovery enums as final operator verdict | Soak-owned outcome vocabulary wrapping shared evidence | Phase 75 context, 2026-06-14 | Keep lower-level sync semantics stable and preserve source evidence. [VERIFIED: .planning/phases/75-multi-day-soak-runner-and-evidence-ledger/75-CONTEXT.md] |
| Public peers or multi-day wall-clock waits for confidence | Deterministic Rust synthetic scenarios plus opt-in operator UAT outside default verification | v1.7 requirements, 2026-06-14 | Default checks remain hermetic and short-running. [VERIFIED: .planning/REQUIREMENTS.md; VERIFIED: scripts/verify.sh] |

**Deprecated/outdated:** Treating support bundle existence as proof of sync success is explicitly not valid; interpretation must come from evidence fields and verdicts. [VERIFIED: docs/operator/runtime-guide.md; VERIFIED: packages/open-bitcoin-cli/src/operator/support/evidence.rs]

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | Use `<datadir>/soak/run-index.json` and `<datadir>/soak/runs/<run_id>/...` as the concrete ledger/report path layout. | Architecture Patterns | Low to medium: changing paths affects docs, tests, and support summary fixtures, but not core sync behavior. |
| A2 | Use `start`, `resume`, and `report` as the first `open-bitcoin soak` subcommands. | Architecture Patterns | Medium: CLI UX and UAT copy may need adjustment if the planner chooses direct flags or different subcommands. |
| A3 | Append full JSON lines with sequence numbers and update run index via temp file plus rename. | Architecture Patterns | Medium: implementation details may change if the executor chooses to reuse a stronger existing persistence helper. |
| A4 | Add a new `SoakOutcomeLabel` enum in the operator soak module. | Architecture Patterns | Low: the vocabulary is locked, but exact Rust type placement and name can vary. |
| A5 | Use existing dependencies without adding a new direct dependency for run IDs. | Standard Stack | Low: if stronger globally unique IDs are required, the planner may add a direct dependency with explicit justification. |
| A6 | Add a focused Bun checker only if docs/report/parity/default-verification boundaries need machine enforcement. | Summary | Low: Rust tests may cover enough if docs and verify boundaries are simple. |

## Open Questions (RESOLVED)

1. **Exact CLI shape**
   - Resolved decision: Plan `open-bitcoin soak start`, `open-bitcoin soak resume --run-id <id>`, `open-bitcoin soak stop --run-id <id> --reason operator-stop`, and `open-bitcoin soak report --run-id <id>` as the Phase 75 operator contract. [VERIFIED: .planning/phases/75-multi-day-soak-runner-and-evidence-ledger/75-CONTEXT.md; VERIFIED: .planning/phases/75-multi-day-soak-runner-and-evidence-ledger/75-02-PLAN.md]
   - Traceability: This implements D-01 through D-04 while keeping daemon-owned sync bounds authoritative and retaining `scripts/run-live-mainnet-smoke.ts` as a helper/fixture surface only. [VERIFIED: .planning/phases/75-multi-day-soak-runner-and-evidence-ledger/75-CONTEXT.md]

2. **Run ID generation**
   - Resolved decision: Use a human-readable, collision-checked run ID generated from the datadir-owned index as `soak-<unix_seconds>-<four_digit_sequence>` when the operator does not provide `--run-id`; do not add a new direct dependency for run IDs in Phase 75. [VERIFIED: .planning/phases/75-multi-day-soak-runner-and-evidence-ledger/75-02-PLAN.md]
   - Traceability: This satisfies durable run identity while preserving the repo dependency-minimization constraint. [VERIFIED: .planning/REQUIREMENTS.md; VERIFIED: AGENTS.md]

3. **Support bundle integration depth**
   - Resolved decision: Add support-summary projection after the ledger/report contracts and operator runner exist, and keep it compact, redacted, local, and summary-only. [VERIFIED: .planning/phases/75-multi-day-soak-runner-and-evidence-ledger/75-04-PLAN.md]
   - Traceability: Support bundles remain projections and do not embed raw daemon logs, raw reports, wallet material, credentials, unbounded peer tables, or automatic uploads. [VERIFIED: .planning/phases/75-multi-day-soak-runner-and-evidence-ledger/75-CONTEXT.md; VERIFIED: packages/open-bitcoin-cli/src/operator/support/live_smoke.rs]

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|-------------|-----------|---------|----------|
| Rust `rustc` | Rust implementation and tests | yes | 1.94.1 | None needed. [VERIFIED: environment audit] |
| Cargo | Workspace build/test commands | yes | 1.94.1 | None needed. [VERIFIED: environment audit] |
| Bun | Optional TypeScript checker and existing verify scripts | yes | 1.3.9 | Avoid new checker if not needed. [VERIFIED: environment audit; VERIFIED: scripts/verify.sh] |
| Bazel | Repo-root smoke build and UAT command form | yes | 8.6.0 | None needed. [VERIFIED: environment audit; VERIFIED: MODULE.bazel] |
| Git | Breadcrumb checker and repo metadata | yes | 2.53.0 | None needed. [VERIFIED: environment audit; VERIFIED: scripts/check-parity-breadcrumbs.ts] |
| cargo-llvm-cov | Final repo verification coverage gate | yes | 0.8.5 | None needed. [VERIFIED: environment audit; VERIFIED: scripts/verify.sh] |

**Missing dependencies with no fallback:** None found for deterministic Phase 75 planning and verification. [VERIFIED: environment audit]

**Missing dependencies with fallback:** None found. [VERIFIED: environment audit]

## Security Domain

Security enforcement is not explicitly disabled in `.planning/config.json`, so Phase 75 should include security controls. [VERIFIED: .planning/config.json]

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|------------------|
| V2 Authentication | no | Phase 75 should not introduce a new authentication surface; if it invokes RPC, reuse existing operator config and credential metadata handling. [VERIFIED: packages/open-bitcoin-cli/src/operator/config.rs; ASSUMED] |
| V3 Session Management | no | Phase 75 is a local CLI workflow and should not introduce sessions. [VERIFIED: packages/open-bitcoin-cli/src/operator.rs; ASSUMED] |
| V4 Access Control | yes | Require explicit datadir/run identity on resume and avoid implicit source-datadir mutation. [VERIFIED: .planning/phases/75-multi-day-soak-runner-and-evidence-ledger/75-CONTEXT.md] |
| V5 Input Validation | yes | Parse flags with `clap`, validate soak bounds at the CLI boundary, and use pure domain types for run IDs, durations, disk budgets, and target heights. [VERIFIED: packages/open-bitcoin-cli/src/operator.rs; VERIFIED: standards/core/architecture.md] |
| V6 Cryptography | no | Run IDs are evidence identifiers, not secrets; do not add cryptography or token semantics in Phase 75. [ASSUMED] |
| V8 Data Protection | yes | Keep ledger/report/support summaries redacted and avoid raw logs, credentials, wallet material, and unbounded peer tables. [VERIFIED: .planning/phases/75-multi-day-soak-runner-and-evidence-ledger/75-CONTEXT.md; VERIFIED: docs/operator/runtime-guide.md] |
| V12 File and Resources | yes | Keep ledger writes bounded/resumable, validate output paths against the selected datadir, and preserve explicit operator opt-in for any public-network workflow. [VERIFIED: .planning/phases/75-multi-day-soak-runner-and-evidence-ledger/75-CONTEXT.md; VERIFIED: scripts/verify.sh] |

### Known Threat Patterns for Phase 75

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Credential or wallet material leaks into ledger/report/support summary | Information Disclosure | Store compact allowlisted evidence only; never copy raw cookie contents, RPC passwords, wallet private material, raw daemon logs, raw reports, or unbounded peer tables. [VERIFIED: docs/operator/runtime-guide.md; VERIFIED: packages/open-bitcoin-cli/src/operator/support/live_smoke.rs] |
| Resuming a run against the wrong datadir | Tampering | Anchor run identity in a datadir-owned index and validate same-run/same-datadir before appending `resume`. [VERIFIED: .planning/phases/75-multi-day-soak-runner-and-evidence-ledger/75-CONTEXT.md] |
| Operator believes a stale report is current durable state | Repudiation | Make reports projections and print/report the source ledger path and latest event sequence. [VERIFIED: .planning/phases/75-multi-day-soak-runner-and-evidence-ledger/75-CONTEXT.md; ASSUMED] |
| Default verification starts public-network or long-running work | Denial of Service | Keep actual soak execution opt-in and add a local checker only for deterministic boundaries. [VERIFIED: .planning/STATE.md; VERIFIED: scripts/verify.sh] |
| False success from incomplete progress evidence | Spoofing | Use shared status/support evidence and preserve validated-active-chain unavailable reasons. [VERIFIED: docs/architecture/status-snapshot.md; VERIFIED: packages/open-bitcoin-cli/src/operator/support/evidence.rs] |

## Sources

### Primary (HIGH confidence)

- `.planning/phases/75-multi-day-soak-runner-and-evidence-ledger/75-CONTEXT.md` - locked decisions, scope, deferred ideas, canonical references, code-context insights. [VERIFIED: .planning/phases/75-multi-day-soak-runner-and-evidence-ledger/75-CONTEXT.md]
- `.planning/REQUIREMENTS.md` - SOAK-01 through SOAK-04 and v1.7 out-of-scope constraints. [VERIFIED: .planning/REQUIREMENTS.md]
- `.planning/ROADMAP.md` - Phase 75 goal, dependency, success criteria, and v1.7 phase sequence. [VERIFIED: .planning/ROADMAP.md]
- `.planning/STATE.md` - current milestone state and deterministic/public-network opt-in decisions. [VERIFIED: .planning/STATE.md]
- `AGENTS.md`, `AGENTS.bright-builds.md`, `standards-overrides.md`, `standards/index.md`, and relevant standards pages - repo workflow, verification, Rust, TypeScript, architecture, and testing constraints. [VERIFIED: AGENTS.md; VERIFIED: AGENTS.bright-builds.md; VERIFIED: standards/index.md]
- `packages/open-bitcoin-cli/src/operator.rs` and `packages/open-bitcoin-cli/src/operator/runtime.rs` - operator CLI parser and runtime dispatch integration points. [VERIFIED: packages/open-bitcoin-cli/src/operator.rs; VERIFIED: packages/open-bitcoin-cli/src/operator/runtime.rs]
- `packages/open-bitcoin-cli/src/operator/config.rs` - operator credential metadata handling and config resolution sources. [VERIFIED: packages/open-bitcoin-cli/src/operator/config.rs]
- `packages/open-bitcoin-cli/src/operator/support.rs`, `support/evidence.rs`, and `support/live_smoke.rs` - support bundle projection, full-sync verdict derivation, and allowlisted live-smoke summary pattern. [VERIFIED: packages/open-bitcoin-cli/src/operator/support.rs; VERIFIED: packages/open-bitcoin-cli/src/operator/support/evidence.rs; VERIFIED: packages/open-bitcoin-cli/src/operator/support/live_smoke.rs]
- `packages/open-bitcoin-node/src/sync.rs`, `sync/runtime_state.rs`, `sync/types.rs`, `status.rs`, and `status/recovery.rs` - sync runtime, durable state, stop reasons, status snapshot, and recovery labels. [VERIFIED: packages/open-bitcoin-node/src/sync.rs; VERIFIED: packages/open-bitcoin-node/src/sync/runtime_state.rs; VERIFIED: packages/open-bitcoin-node/src/sync/types.rs; VERIFIED: packages/open-bitcoin-node/src/status.rs; VERIFIED: packages/open-bitcoin-node/src/status/recovery.rs]
- `packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs` and RPC config files - daemon sync activation and authoritative runtime config. [VERIFIED: packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs; VERIFIED: packages/open-bitcoin-rpc/src/config.rs; VERIFIED: packages/open-bitcoin-rpc/src/config/open_bitcoin.rs]
- `scripts/run-live-mainnet-smoke.ts`, `scripts/test-run-live-mainnet-smoke.sh`, and `scripts/verify.sh` - opt-in live-smoke helper, deterministic fixture guard, and default verification contract. [VERIFIED: scripts/run-live-mainnet-smoke.ts; VERIFIED: scripts/test-run-live-mainnet-smoke.sh; VERIFIED: scripts/verify.sh]
- `docs/operator/runtime-guide.md`, `docs/architecture/status-snapshot.md`, and `docs/architecture/operator-observability.md` - operator UAT command patterns, redaction boundaries, and shared status/evidence semantics. [VERIFIED: docs/operator/runtime-guide.md; VERIFIED: docs/architecture/status-snapshot.md; VERIFIED: docs/architecture/operator-observability.md]

### Secondary (MEDIUM confidence)

- Local environment command audit for installed tool versions. [VERIFIED: environment audit]

### Tertiary (LOW confidence)

- Assumptions listed in the Assumptions Log for exact path layout, CLI subcommand names, atomic write details, and direct dependency posture. [ASSUMED]

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - versions and dependencies were verified from pinned repo files, manifests, lockfile, and local tool commands. [VERIFIED: rust-toolchain.toml; VERIFIED: packages/Cargo.toml; VERIFIED: packages/Cargo.lock; VERIFIED: environment audit]
- Architecture: HIGH - integration points are existing operator CLI, daemon sync, status, support, and sync runtime files, with only exact soak file/path naming marked as assumptions. [VERIFIED: packages/open-bitcoin-cli/src/operator.rs; VERIFIED: packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs; VERIFIED: packages/open-bitcoin-node/src/sync.rs; ASSUMED]
- Pitfalls: HIGH - major risks are directly constrained by CONTEXT.md, requirements, docs, and default verification scripts. [VERIFIED: .planning/phases/75-multi-day-soak-runner-and-evidence-ledger/75-CONTEXT.md; VERIFIED: .planning/REQUIREMENTS.md; VERIFIED: scripts/verify.sh; VERIFIED: docs/operator/runtime-guide.md]
- Security: MEDIUM - redaction and datadir controls are repo-verified, while ASVS category mapping is a research-level application of the security-domain requirement. [VERIFIED: docs/operator/runtime-guide.md; VERIFIED: .planning/config.json; ASSUMED]

**Research date:** 2026-06-14
**Valid until:** 2026-07-14 for local code integration; re-check tool versions and manifests before planning if dependency files change. [ASSUMED]
