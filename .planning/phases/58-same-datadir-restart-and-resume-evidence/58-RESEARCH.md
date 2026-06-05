# Phase 58: Same-Datadir Restart and Resume Evidence - Research

**Researched:** 2026-06-05 [VERIFIED: environment_context.current_date]
**Domain:** Open Bitcoin durable sync restart/resume evidence, live-smoke schema v2 reporting, deterministic Fjall reopen tests, and operator recovery diagnosis [VERIFIED: .planning/phases/58-same-datadir-restart-and-resume-evidence/58-CONTEXT.md]
**Confidence:** HIGH [VERIFIED: codebase grep and targeted file reads listed in Sources]

<user_constraints>
## User Constraints (from CONTEXT.md)

Source for all copied constraints in this section: [VERIFIED: .planning/phases/58-same-datadir-restart-and-resume-evidence/58-CONTEXT.md]

### Locked Decisions

## Implementation Decisions

### Same-Datadir Restart Flow

- **D-01:** Add a script-managed two-session live-smoke restart flow as the
  operator-facing public UAT proof. The flow should use the same selected
  datadir, capture progress before restart, terminate the first daemon
  intentionally, relaunch the daemon, and capture resume evidence from fresh
  `openbitcoinsyncstatus` snapshots.
- **D-02:** Treat deterministic store-reopen tests as the mandatory regression
  guard for RESUME-01. They must prove durable headers, block bodies,
  chainstate, downloaded height/hash, and connected height/hash survive reopen
  and that already connected blocks are not requested or connected again.
- **D-03:** Do not require fresh post-restart public-network progress when
  peers stall after the restart. The restart claim is durable resume from the
  same datadir plus a typed diagnosis of any post-restart blocker. Fresh
  progress after restart is stronger evidence when available, not the only
  acceptable evidence path.
- **D-04:** Keep service-manager and unattended restart-policy behavior out of
  scope. Phase 58 may relaunch a daemon process for explicit smoke evidence,
  but it must not imply launchd/systemd supervision or production-node
  operation.

### Restart Evidence Report Schema

- **D-05:** Add a compact schema v2 result object named
  `result.restartResumeEvidence` rather than raw top-level restart attempts.
  The object should prove the restart boundary and same-datadir resume using
  allowlist-friendly summary fields.
- **D-06:** `restartResumeEvidence` should include same-datadir confirmation,
  restart status, before-restart and after-restart durable heights and hashes,
  runtime phase/lifecycle, latest successful progress timestamp, peer outcome
  summary, duplicate-connect verdict, and optional post-restart progress delta.
- **D-07:** Preserve Phase 57 `firstHeaderProgress` and `firstBlockProgress`
  as local report evidence, but do not use them alone as restart proof because
  they cannot distinguish same-process progress from post-relaunch resume.
- **D-08:** Keep raw daemon stdout/stderr tails, raw status snapshots, raw
  options, raw endpoint tables, and high-volume peer rows out of the compact
  restart evidence object. Phase 59 can decide support-bundle allowlisting.

### Recovery Diagnosis Taxonomy

- **D-09:** Prefer a layered recovery diagnosis model for Phase 58 evidence.
  The user-facing category should be one of:
  `peer_incompatibility`, `public_network_unreachable`, `invalid_peer_data`,
  `store_corruption`, `store_incompatibility`, `resource_exhaustion`, or
  `intentional_cancellation`.
- **D-10:** Preserve underlying causes alongside the Phase 58 category when
  available, including existing live-smoke `NoProgressCause`, durable
  `PeerFailureReason`, storage recovery action, and last-error detail.
- **D-11:** Storage health outranks peer retry guidance. Store corruption or
  store incompatibility should classify before peer incompatibility,
  public-network unreachability, invalid peer data, or timeout-style guidance.
- **D-12:** Operator guidance should distinguish cancellation from failure:
  intentional interruption used for restart evidence is part of the flow, while
  cancellation before enough evidence is captured remains a typed
  `intentional_cancellation` diagnosis.

### Deterministic Test Strategy

- **D-13:** Prioritize `DurableSyncRuntime` two-pass same-datadir fixtures that
  use real Fjall reopen and `ScriptedTransport`. These tests should cover
  header-only resume, partial downloaded block resume, connected block resume,
  no duplicate `getdata` for already connected blocks, and best-chain block
  reconciliation after reopen.
- **D-14:** Add mocked live-smoke fixture tests for restart-report semantics:
  before/restart/after snapshots, same datadir, runtime phase, peer summaries,
  latest progress timestamp, duplicate-connect verdict, and recovery diagnosis.
- **D-15:** Add a narrow recovery diagnosis matrix for RESUME-03. Avoid a
  broad process-level local peer harness unless review evidence shows that the
  existing Rust and script fixtures cannot prove the phase claim.
- **D-16:** Public-network live-smoke commands remain opt-in UAT evidence and
  must not be added to `bash scripts/verify.sh`.

### Documentation and UAT Boundaries

- **D-17:** Update operator docs with copy-pasteable repo-local Cargo and Bazel
  commands for same-datadir restart/resume review, status checks, and pass/fail
  interpretation.
- **D-18:** Document that Phase 58 evidence proves explicit opt-in restart and
  resume review only. It does not claim unattended production operation,
  packaged-service restart policy, inbound serving, transaction relay, or
  broad production-node readiness.

### the agent's Discretion

- The planner may choose the smallest robust internal representation for
  `restartResumeEvidence` and recovery diagnosis as long as the externally
  observable schema remains additive, typed, and deterministic to test.
- The planner may split work across Rust runtime tests, live-smoke TypeScript
  changes, shell fixtures, docs, and parity evidence according to existing
  module boundaries.
- The executor may reuse existing restart tests where they already satisfy a
  requirement, but summaries and verification must make the evidence explicit
  for Phase 58.

### Deferred Ideas (OUT OF SCOPE)

## Deferred Ideas

- Support bundle allowlisting for `result.restartResumeEvidence` remains
  Phase 59 unless Phase 58 needs a minimal preparatory hook for schema
  compatibility.
- Threat-model updates, release-boundary copy, and final operator evidence
  closeout remain Phase 59.
- Service-manager restart policy, launchd/systemd supervision, unattended
  production-node operation, inbound serving, transaction relay, production
  wallet use, migration apply mode, packaging, hosted dashboard, and GUI work
  remain out of scope for v1.4.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| RESUME-01 | Operator can interrupt and restart the same v1.4 public-mainnet datadir after observed header or block progress and see sync resume from durable state without duplicating block connects. [VERIFIED: .planning/REQUIREMENTS.md] | Use the existing `DurableSyncRuntime::open` reopen path, add/tighten same-datadir two-pass Rust fixtures, and add a two-session live-smoke restart flow. [VERIFIED: packages/open-bitcoin-node/src/sync.rs; VERIFIED: packages/open-bitcoin-node/src/sync/tests.rs; VERIFIED: scripts/run-live-mainnet-smoke.ts] |
| RESUME-02 | Live-smoke reporting can capture same-datadir before/after restart evidence for header height, block height, runtime phase, peer outcomes, and latest progress timestamp. [VERIFIED: .planning/REQUIREMENTS.md] | Extend schema v2 under `result.restartResumeEvidence`, using current status snapshots, final peer telemetry, and `last_successful_progress_unix_seconds` projection. [VERIFIED: scripts/run-live-mainnet-smoke.ts; VERIFIED: packages/open-bitcoin-node/src/status.rs; VERIFIED: packages/open-bitcoin-node/src/sync/types/summary.rs] |
| RESUME-03 | Recovery guidance distinguishes transient peer incompatibility, public-network unreachability, invalid peer data, store corruption, store incompatibility, resource exhaustion, and intentional cancellation. [VERIFIED: .planning/REQUIREMENTS.md] | Add a storage-first recovery-diagnosis helper over current `NoProgressCause`, `PeerFailureReason`, storage health messages, last error, and cancellation state. [VERIFIED: scripts/run-live-mainnet-smoke.ts; VERIFIED: packages/open-bitcoin-node/src/sync/types.rs; VERIFIED: packages/open-bitcoin-node/src/storage.rs; VERIFIED: packages/open-bitcoin-node/src/sync/types/projection.rs] |
</phase_requirements>

## Summary

Phase 58 should be planned as a codebase-local evidence phase, not an ecosystem or dependency phase. The existing stack already persists headers, block bodies, active chainstate, runtime metadata, peer outcomes, downloaded/connected block heights, hashes, phase, lifecycle, and last-progress timestamps through Fjall-backed `DurableSyncRuntime` and shared status projections. [VERIFIED: packages/open-bitcoin-node/src/sync.rs; VERIFIED: packages/open-bitcoin-node/src/status.rs; VERIFIED: packages/open-bitcoin-node/src/sync/types/summary.rs]

The main implementation gap is to make the restart boundary explicit. Current live-smoke schema v2 reports `result.firstHeaderProgress` and `result.firstBlockProgress`, but those objects only prove same-process progress because the current runner starts one daemon process, polls `openbitcoinsyncstatus`, terminates that process, reads final status, and writes one JSON/Markdown report. [VERIFIED: scripts/run-live-mainnet-smoke.ts; VERIFIED: .planning/phases/57-block-download-and-connect-progress/57-04-SUMMARY.md]

**Primary recommendation:** split the phase into deterministic Rust resume evidence first, then TypeScript live-smoke two-session orchestration and compact `result.restartResumeEvidence`, then recovery-diagnosis/docs/parity wording with default verification kept deterministic. [VERIFIED: .planning/phases/58-same-datadir-restart-and-resume-evidence/58-CONTEXT.md; VERIFIED: scripts/verify.sh]

## Project Constraints (from AGENTS.md)

- Use `AGENTS.md` as the repo-local instruction source, then `AGENTS.bright-builds.md`, `standards-overrides.md`, and pinned Bright Builds standards pages when planning or implementing. [VERIFIED: AGENTS.md; VERIFIED: AGENTS.bright-builds.md; CITED: https://raw.githubusercontent.com/bright-builds-llc/bright-builds-rules/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/index.md]
- Materialize the pinned Knots baseline with `git submodule update --init --recursive` when needed. [VERIFIED: AGENTS.md]
- Treat `rust-toolchain.toml` as the Rust source of truth; the pinned local toolchain is Rust `1.94.1`. [VERIFIED: AGENTS.md; VERIFIED: rust-toolchain.toml; VERIFIED: `rustc --version`]
- Use `bash scripts/verify.sh` as the repo-native verification contract for first-party code. [VERIFIED: AGENTS.md; VERIFIED: scripts/verify.sh]
- Keep public-network checks opt-in and outside default verification. [VERIFIED: AGENTS.md; VERIFIED: .planning/REQUIREMENTS.md; VERIFIED: scripts/verify.sh]
- During UAT, provide repo-local Cargo and Bazel commands instead of only naming installed aliases. [VERIFIED: AGENTS.md]
- Use Bun as the canonical runtime for repo-owned higher-level automation scripts; this repo has no `package.json` bootstrap. [VERIFIED: AGENTS.md; VERIFIED: .bun-version]
- Treat `docs/metrics/lines-of-code.md` as a tracked generated artifact that may need freshness updates after verification. [VERIFIED: AGENTS.md; VERIFIED: scripts/verify.sh]
- Record in-scope behavior differences from Bitcoin Knots in `docs/parity/index.json` and companion docs under `docs/parity/`. [VERIFIED: AGENTS.md]
- When adding first-party Rust source or test files under `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests`, add parity breadcrumbs through `docs/parity/source-breadcrumbs.json` and `scripts/check-parity-breadcrumbs.ts`; use `none` only when no defensible Knots anchor exists. [VERIFIED: AGENTS.md]
- Follow functional-core / imperative-shell boundaries, early returns, `maybe` naming for nullable/optional internals, one-concern tests, and explicit Arrange/Act/Assert comments for non-trivial unit tests. [VERIFIED: AGENTS.bright-builds.md; CITED: https://raw.githubusercontent.com/bright-builds-llc/bright-builds-rules/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/core/architecture.md; CITED: https://raw.githubusercontent.com/bright-builds-llc/bright-builds-rules/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/core/testing.md]
- No project-local skills are installed under `.claude/skills` or `.agents/skills`. [VERIFIED: `find .claude/skills .agents/skills -maxdepth 2 -type f -name SKILL.md`]

## Implementation Surface Inventory

| Surface | Current Capability | Phase 58 Planning Implication |
|---------|--------------------|-------------------------------|
| `scripts/run-live-mainnet-smoke.ts` | Defines schema v2 `SmokeReport`, `SyncStatusSnapshot`, `FirstHeaderProgressEvidence`, `FirstBlockProgressEvidence`, `NoProgressCause`, daemon/status/final-status commands, one-process `spawn`, polling, termination, Markdown rendering, and fixture overrides. [VERIFIED: scripts/run-live-mainnet-smoke.ts] | Refactor the one-process body into a reusable daemon-session helper and add an explicit restart mode that writes compact `result.restartResumeEvidence`. [VERIFIED: scripts/run-live-mainnet-smoke.ts; VERIFIED: 58-CONTEXT.md] |
| `scripts/test-run-live-mainnet-smoke.sh` | Uses temp mock daemon/status/final-status scripts, JSON fixtures, env overrides, `grep` assertions, cancellation tests, and peer-failure diagnosis matrix. [VERIFIED: scripts/test-run-live-mainnet-smoke.sh] | Add restart fixtures by extending the same mock-script pattern; avoid a new public-network or local peer harness unless existing fixtures cannot prove the claim. [VERIFIED: scripts/test-run-live-mainnet-smoke.sh; VERIFIED: 58-CONTEXT.md] |
| `packages/open-bitcoin-node/src/sync.rs` | `DurableSyncRuntime::open` reloads chainstate snapshot and header store from `FjallNodeStore`; `snapshot_summary` reports downloaded and connected block hashes; `sync_once` persists durable sync state. [VERIFIED: packages/open-bitcoin-node/src/sync.rs] | Use this as the deterministic same-datadir reopen path; do not read Fjall files directly in tests or scripts. [VERIFIED: packages/open-bitcoin-node/src/sync.rs] |
| `packages/open-bitcoin-node/src/sync/block_reconcile.rs` | `request_missing_blocks` skips active-chain blocks, skips global in-flight blocks, notes locally stored block hashes, and requests missing best-chain blocks; `reconcile_best_chain` connects durable local blocks after reopen. [VERIFIED: packages/open-bitcoin-node/src/sync/block_reconcile.rs] | Duplicate-connect/request evidence should assert no `getdata` for already connected blocks and active-chain state remains stable after reopen. [VERIFIED: packages/open-bitcoin-node/src/sync/block_reconcile.rs; VERIFIED: packages/open-bitcoin-node/src/sync/tests.rs] |
| `packages/open-bitcoin-node/src/sync/tests.rs` | Existing tests cover header seeding on restart, persisted block reconnect before re-request, partial downloaded/connected status after reopen, invalid block no-credit paths, and best-branch reconciliation after restart. [VERIFIED: packages/open-bitcoin-node/src/sync/tests.rs] | Add focused Phase 58 tests instead of broad rewrites: one header-only resume test, one connected-block resume/no-duplicate-request test, one downloaded-only resume test, and one best-chain reconciliation test if current assertions are not explicit enough. [VERIFIED: packages/open-bitcoin-node/src/sync/tests.rs; VERIFIED: 58-CONTEXT.md] |
| `packages/open-bitcoin-node/src/status.rs` | `SyncProgress` includes header, block, downloaded block, connected block, optional downloaded/connected hashes, messages, headers, and blocks; `SyncStatus` includes lifecycle, phase, last successful progress, last error, recovery action, and resource pressure. [VERIFIED: packages/open-bitcoin-node/src/status.rs] | `restartResumeEvidence` should be derived from existing status fields rather than adding a new operator truth surface. [VERIFIED: packages/open-bitcoin-node/src/status.rs; VERIFIED: scripts/run-live-mainnet-smoke.ts] |
| `packages/open-bitcoin-node/src/storage.rs` and `sync/types/projection.rs` | Storage errors distinguish invalid schema version, schema mismatch, corruption, unavailable namespace, interrupted write, and backend failure; projection maps storage failures into operator health messages. [VERIFIED: packages/open-bitcoin-node/src/storage.rs; VERIFIED: packages/open-bitcoin-node/src/sync/types/projection.rs] | Recovery diagnosis can distinguish `store_incompatibility` from `store_corruption` by checking schema messages before peer-level causes. [VERIFIED: packages/open-bitcoin-node/src/storage.rs; VERIFIED: packages/open-bitcoin-node/src/sync/types/projection.rs] |
| `packages/open-bitcoin-rpc/src/context.rs` | `openbitcoinsyncstatus`, pause, and resume can read/write runtime metadata through daemon sync control or store-backed control. [VERIFIED: packages/open-bitcoin-rpc/src/context.rs] | The live-smoke restart flow should keep using fresh `openbitcoinsyncstatus` snapshots for daemon evidence and final `open-bitcoin sync status` for durable post-run evidence. [VERIFIED: packages/open-bitcoin-rpc/src/context.rs; VERIFIED: scripts/run-live-mainnet-smoke.ts] |
| `packages/open-bitcoin-cli/src/operator/status/sync_state.rs` | Operator status can read durable sync state directly from the selected datadir through `FjallNodeStore::open`. [VERIFIED: packages/open-bitcoin-cli/src/operator/status/sync_state.rs] | UAT docs should include Cargo and Bazel `open-bitcoin sync status --format json` commands against the same datadir. [VERIFIED: packages/open-bitcoin-cli/src/operator/status/sync_state.rs; VERIFIED: AGENTS.md] |
| `docs/operator/runtime-guide.md` | Documents opt-in live-mainnet smoke, first-block evidence, durable recovery fields, same-datadir status checks, recovery guidance, support-bundle redaction, and v1.4 limitations. [VERIFIED: docs/operator/runtime-guide.md] | Update the live-smoke and durable recovery sections with same-datadir restart/resume commands and pass/fail interpretation, without broadening production claims. [VERIFIED: docs/operator/runtime-guide.md; VERIFIED: 58-CONTEXT.md] |
| `docs/parity/catalog/p2p.md` | States the current P2P/sync coverage and known gaps, including opt-in live-smoke evidence and no unattended public-network full sync claim. [VERIFIED: docs/parity/catalog/p2p.md] | Add the explicit same-datadir restart/resume evidence claim while keeping unattended production sync in known gaps. [VERIFIED: docs/parity/catalog/p2p.md; VERIFIED: 58-CONTEXT.md] |

## Standard Stack

### Core

| Library / Tool | Version | Purpose | Why Standard |
|----------------|---------|---------|--------------|
| Rust toolchain | `1.94.1` | First-party node, RPC, CLI, network, and tests. [VERIFIED: rust-toolchain.toml; VERIFIED: packages/Cargo.toml] | Repo-local source of truth and Bazel toolchain both pin this version. [VERIFIED: rust-toolchain.toml; VERIFIED: MODULE.bazel] |
| Cargo workspace | Rust 2024 edition, resolver 3 | Workspace crates under `packages/open-bitcoin-*`. [VERIFIED: packages/Cargo.toml] | Existing first-party Rust surfaces and tests already use this workspace. [VERIFIED: packages/Cargo.toml] |
| Fjall | `3.1.4` | Durable node storage through `FjallNodeStore`. [VERIFIED: packages/Cargo.lock; VERIFIED: packages/open-bitcoin-node/src/storage.rs] | The restart claim depends on reopening the same durable store through existing adapters. [VERIFIED: packages/open-bitcoin-node/src/sync.rs] |
| Bun | `1.3.9` | Runs repo-owned TypeScript automation and live-smoke scripts. [VERIFIED: .bun-version; VERIFIED: `bun --version`] | Repo-local guidance makes Bun the canonical runtime for substantial scripts. [VERIFIED: AGENTS.md] |
| Bash | GNU bash `3.2.57` available locally | Thin orchestration wrappers and shell fixture tests. [VERIFIED: `bash --version`; VERIFIED: scripts/test-run-live-mainnet-smoke.sh] | Existing live-smoke regression suite is a Bash fixture harness. [VERIFIED: scripts/test-run-live-mainnet-smoke.sh] |
| Bazel / Bazelisk surface | Bazel `8.6.0`, `rules_rust` `0.69.0` | Top-level smoke builds and UAT command surface. [VERIFIED: `bazel --version`; VERIFIED: MODULE.bazel] | Repo-local guidance requires Bazel commands for operator workflows and `scripts/verify.sh` includes Bazel build checks. [VERIFIED: AGENTS.md; VERIFIED: scripts/verify.sh] |

### Supporting

| Library / Tool | Version | Purpose | When to Use |
|----------------|---------|---------|-------------|
| serde / serde_json | `1.0.228` / `1.0.149` | Stable status/report JSON shapes. [VERIFIED: packages/Cargo.lock] | Use existing JSON status/report contracts; avoid ad hoc text parsing of structured status. [VERIFIED: packages/open-bitcoin-node/src/status.rs; VERIFIED: scripts/run-live-mainnet-smoke.ts] |
| Tokio / Axum | `1.52.1` / `0.8.9` | Daemon/RPC runtime support. [VERIFIED: packages/Cargo.lock] | No new phase dependency, but relevant to `openbitcoinsyncstatus` daemon control behavior. [VERIFIED: packages/open-bitcoin-rpc/src/context.rs] |
| clap | `4.6.1` | CLI argument parsing in Rust operator binaries. [VERIFIED: packages/Cargo.lock] | Use existing CLI commands for UAT docs; do not add a new Rust CLI just for Phase 58. [VERIFIED: packages/open-bitcoin-cli/src/operator/status/sync_state.rs] |
| jsonc-parser | `0.32.3` | Open Bitcoin JSONC config parsing. [VERIFIED: packages/Cargo.lock] | Relevant only if docs or live-smoke examples include restart review configs. [VERIFIED: docs/operator/runtime-guide.md] |
| cargo-llvm-cov | `0.8.5` available locally | Full repo verification coverage check. [VERIFIED: `cargo llvm-cov --version`; VERIFIED: scripts/verify.sh] | Required by `bash scripts/verify.sh`; not needed for the narrow script fixture loop. [VERIFIED: scripts/verify.sh] |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Extending `scripts/run-live-mainnet-smoke.ts` | Separate restart-smoke script | A separate script would duplicate preflight, process orchestration, status parsing, report writing, and fixture plumbing already present in the existing live-smoke runner. [VERIFIED: scripts/run-live-mainnet-smoke.ts] |
| `DurableSyncRuntime` reopen tests | Direct Fjall key/value assertions | Direct store inspection would bypass the adapter-facing runtime contract that operators actually use after restart. [VERIFIED: packages/open-bitcoin-node/src/sync.rs; VERIFIED: packages/open-bitcoin-cli/src/operator/status/sync_state.rs] |
| Bash fixture tests | Local Bitcoin peer harness | Context locks the narrower mocked fixture strategy unless existing Rust/script fixtures cannot prove the phase claim. [VERIFIED: 58-CONTEXT.md; VERIFIED: scripts/test-run-live-mainnet-smoke.sh] |
| Service manager restart policy | launchd/systemd supervision tests | Explicitly out of scope for Phase 58 and v1.4 restart evidence. [VERIFIED: 58-CONTEXT.md; VERIFIED: .planning/REQUIREMENTS.md] |

**Installation:** No new dependencies are recommended for Phase 58. [VERIFIED: packages/Cargo.toml; VERIFIED: .planning/phases/58-same-datadir-restart-and-resume-evidence/58-CONTEXT.md]

**Version verification used during research:**

```bash
rustc --version
cargo --version
bun --version
bazel --version
cargo llvm-cov --version
git submodule status packages/bitcoin-knots
```

These commands verified Rust `1.94.1`, Cargo `1.94.1`, Bun `1.3.9`, Bazel `8.6.0`, cargo-llvm-cov `0.8.5`, and the Knots submodule at `v29.3.knots20260210`. [VERIFIED: command outputs]

## Architecture Patterns

### Recommended Project Structure

Use existing files; do not create new modules unless helpers push `scripts/run-live-mainnet-smoke.ts` past maintainability thresholds. [VERIFIED: scripts/run-live-mainnet-smoke.ts; CITED: Bright Builds code-shape standard]

```text
packages/open-bitcoin-node/src/
├── sync.rs                         # DurableSyncRuntime reopen/orchestration shell [VERIFIED]
├── sync/block_reconcile.rs         # Missing-block request and durable best-chain reconciliation [VERIFIED]
├── sync/tests.rs                   # Same-datadir Fjall reopen + ScriptedTransport tests [VERIFIED]
├── status.rs                       # Shared SyncStatus/SyncProgress contract [VERIFIED]
└── storage.rs                      # Storage recovery metadata and errors [VERIFIED]

scripts/
├── run-live-mainnet-smoke.ts       # Schema v2 report, process orchestration, restart evidence [VERIFIED]
└── test-run-live-mainnet-smoke.sh  # Deterministic mocked report fixtures [VERIFIED]

docs/
├── operator/runtime-guide.md       # Operator UAT and recovery guidance [VERIFIED]
└── parity/catalog/p2p.md           # Scoped parity claim wording [VERIFIED]
```

### Pattern 1: Two-Session Live-Smoke Restart Shell

**What:** Add an explicit opt-in flag such as `--restart-after-progress` that runs session 1 until durable header or block progress is observed or a typed blocker occurs, terminates that daemon intentionally, starts session 2 against the same resolved datadir, polls fresh `openbitcoinsyncstatus`, and writes compact evidence under `result.restartResumeEvidence`. [VERIFIED: 58-CONTEXT.md; VERIFIED: scripts/run-live-mainnet-smoke.ts]

**When to use:** Use this only for public-mainnet UAT review; keep the default runner behavior and deterministic fixture tests usable without public-network access. [VERIFIED: 58-CONTEXT.md; VERIFIED: scripts/verify.sh]

**Planning detail:** Refactor current `main` session logic into a helper returning `{ snapshots, maybeFirstHeaderProgress, maybeFirstBlockProgress, maybeFinalStatus, endpointOutcomes, daemonExit }`; then compose two calls for restart mode. [VERIFIED: scripts/run-live-mainnet-smoke.ts]

### Pattern 2: Compact Evidence Object, Raw Details Elsewhere

**What:** `result.restartResumeEvidence` should summarize same datadir, restart status, before/after heights/hashes, phase/lifecycle, latest progress timestamp, peer outcome counts, duplicate-connect verdict, optional post-restart deltas, and recovery diagnosis. [VERIFIED: 58-CONTEXT.md]

**When to use:** Use compact evidence for the additive schema v2 result field; keep raw snapshots, endpoint rows, command arrays, and daemon tails in existing report sections only. [VERIFIED: 58-CONTEXT.md; VERIFIED: scripts/run-live-mainnet-smoke.ts]

**Recommended shape:** Use tagged unions for restart status and duplicate-connect verdict so unavailable evidence is explicit rather than implied. [CITED: Bright Builds architecture standard; VERIFIED: scripts/run-live-mainnet-smoke.ts]

```typescript
type RestartStatus = "not_requested" | "completed" | "blocked_before_restart" | "cancelled";
type DuplicateConnectVerdict = "no_duplicate_connect_observed" | "unavailable" | "duplicate_connect_suspected";
```

### Pattern 3: Durable Reopen Through Runtime APIs

**What:** Use `FjallNodeStore::open` plus `DurableSyncRuntime::open` to prove reopen behavior because that path reloads chainstate snapshots and header stores before projecting summary/status. [VERIFIED: packages/open-bitcoin-node/src/sync.rs]

**When to use:** Use this for RESUME-01 deterministic regression tests. [VERIFIED: 58-CONTEXT.md]

**Test pattern:** Seed headers and block bodies with `PersistMode::Sync`, drop the first store/runtime scope, reopen the same path, run `sync_once` or inspect `snapshot_summary`, then assert durable status and outbound `getdata` behavior. [VERIFIED: packages/open-bitcoin-node/src/sync/tests.rs]

### Pattern 4: Storage-First Recovery Diagnosis

**What:** Build a pure TypeScript helper that accepts final status, endpoint outcomes, no-progress cause, cancellation state, and last probe error; it returns the Phase 58 category plus underlying causes. [VERIFIED: scripts/run-live-mainnet-smoke.ts; VERIFIED: 58-CONTEXT.md]

**When to use:** Use it for `restartResumeEvidence.recoveryDiagnosis` and Markdown guidance. [VERIFIED: 58-CONTEXT.md]

**Precedence:** Check storage schema/corruption/recovery strings first, then resource pressure, cancellation, invalid peer data, peer compatibility, public-network reachability, and timeout fallback. [VERIFIED: 58-CONTEXT.md; VERIFIED: packages/open-bitcoin-node/src/storage.rs; VERIFIED: packages/open-bitcoin-node/src/sync/types/projection.rs]

### Anti-Patterns to Avoid

- **Using first progress as restart proof:** `firstHeaderProgress` and `firstBlockProgress` are useful evidence, but they do not prove post-relaunch resume by themselves. [VERIFIED: 58-CONTEXT.md; VERIFIED: scripts/run-live-mainnet-smoke.ts]
- **Requiring fresh post-restart public progress:** Phase 58 success can be durable same-datadir resume plus typed post-restart diagnosis when peers stall. [VERIFIED: 58-CONTEXT.md]
- **Embedding raw snapshots in compact evidence:** Raw snapshots already live in `snapshots`; compact restart evidence should remain allowlist-friendly. [VERIFIED: 58-CONTEXT.md; VERIFIED: scripts/run-live-mainnet-smoke.ts]
- **Classifying peer failures before storage health:** Store corruption or incompatibility must outrank peer retry guidance. [VERIFIED: 58-CONTEXT.md; VERIFIED: docs/operator/runtime-guide.md]
- **Adding public-network smoke to `scripts/verify.sh`:** Default verification must remain deterministic. [VERIFIED: .planning/REQUIREMENTS.md; VERIFIED: scripts/verify.sh]

## Plan Split Recommendations

1. **Plan 58-01: Deterministic durable resume tests.** Tighten `packages/open-bitcoin-node/src/sync/tests.rs` around header-only reopen, downloaded-only reopen, connected-block reopen, no duplicate `getdata` for already connected blocks, durable hashes, and best-chain reconciliation. [VERIFIED: packages/open-bitcoin-node/src/sync/tests.rs; VERIFIED: 58-CONTEXT.md]
2. **Plan 58-02: Live-smoke restart orchestration and schema.** Refactor session orchestration in `scripts/run-live-mainnet-smoke.ts`, add an opt-in restart flag, add `result.restartResumeEvidence`, and extend Markdown rendering and `scripts/test-run-live-mainnet-smoke.sh`. [VERIFIED: scripts/run-live-mainnet-smoke.ts; VERIFIED: scripts/test-run-live-mainnet-smoke.sh; VERIFIED: 58-CONTEXT.md]
3. **Plan 58-03: Recovery diagnosis matrix and docs/parity.** Add the seven-category recovery diagnosis helper and fixtures, update `docs/operator/runtime-guide.md` with Cargo/Bazel UAT commands, and update `docs/parity/catalog/p2p.md` without broadening v1.4 scope. [VERIFIED: docs/operator/runtime-guide.md; VERIFIED: docs/parity/catalog/p2p.md; VERIFIED: .planning/REQUIREMENTS.md]

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Durable reopen proof | Custom Fjall key readers or report-only inference | `FjallNodeStore::open` + `DurableSyncRuntime::open` + `snapshot_summary` / durable status [VERIFIED: packages/open-bitcoin-node/src/sync.rs] | This is the runtime path the daemon and operator status rely on after restart. [VERIFIED: packages/open-bitcoin-node/src/sync.rs; VERIFIED: packages/open-bitcoin-cli/src/operator/status/sync_state.rs] |
| Block duplicate-request/connect evidence | A string search over logs | `ScriptedTransport::sent_messages`, `getdata_block_hashes`, active-chain status, and durable sync progress assertions [VERIFIED: packages/open-bitcoin-node/src/sync/tests.rs] | Existing tests already inspect outbound `getdata` and durable heights/hashes. [VERIFIED: packages/open-bitcoin-node/src/sync/tests.rs] |
| Restart live-smoke harness | New process runner with separate schema | Extend `scripts/run-live-mainnet-smoke.ts` and `scripts/test-run-live-mainnet-smoke.sh` [VERIFIED: scripts/run-live-mainnet-smoke.ts; VERIFIED: scripts/test-run-live-mainnet-smoke.sh] | Existing runner already owns preflight, process lifecycle, status polling, report writing, and fixture env overrides. [VERIFIED: scripts/run-live-mainnet-smoke.ts] |
| Peer and storage recovery categories | Scattered `if error.includes(...)` checks at call sites | One pure diagnosis helper with fixture matrix [VERIFIED: scripts/run-live-mainnet-smoke.ts; VERIFIED: 58-CONTEXT.md] | The seven user-facing categories must stay additive, typed, deterministic, and storage-first. [VERIFIED: 58-CONTEXT.md] |
| Public-network validation | Live network in default verification | Opt-in UAT command plus deterministic Rust/script tests [VERIFIED: .planning/REQUIREMENTS.md; VERIFIED: scripts/verify.sh] | Default verification must remain deterministic and public-network-free. [VERIFIED: .planning/REQUIREMENTS.md; VERIFIED: scripts/verify.sh] |
| Baseline behavior interpretation | Broad ecosystem research | Pinned Knots anchors in `feature_init.py`, `bitcoind.1`, `init.cpp`, `net_processing.cpp`, and `blockstorage.cpp` [VERIFIED: 58-CONTEXT.md; VERIFIED: packages/bitcoin-knots/test/functional/feature_init.py] | Roadmap says v1.4 planning should use targeted Knots/protocol comparison, not broad new-feature research. [VERIFIED: .planning/ROADMAP.md] |

**Key insight:** Phase 58 is mostly evidence composition over existing durable runtime primitives; custom storage, custom process harnesses, or new peer simulators would increase scope without improving the proof. [VERIFIED: packages/open-bitcoin-node/src/sync.rs; VERIFIED: scripts/run-live-mainnet-smoke.ts; VERIFIED: 58-CONTEXT.md]

## Common Pitfalls

### Pitfall 1: Treating Same-Process Progress as Restart Proof
**What goes wrong:** A report passes because `firstBlockProgress` exists, but no fresh post-relaunch status was collected. [VERIFIED: scripts/run-live-mainnet-smoke.ts; VERIFIED: 58-CONTEXT.md]
**Why it happens:** Current first-progress evidence is derived from snapshots inside one daemon session. [VERIFIED: scripts/run-live-mainnet-smoke.ts]
**How to avoid:** Require `restartResumeEvidence` to include pre-termination and post-relaunch status summaries plus same-datadir confirmation. [VERIFIED: 58-CONTEXT.md]
**Warning signs:** `result.restartResumeEvidence` is null while docs claim restart/resume proof. [VERIFIED: 58-CONTEXT.md]

### Pitfall 2: Overclaiming Fresh Public Progress After Restart
**What goes wrong:** A valid durable resume is marked failed only because public peers stall after relaunch. [VERIFIED: 58-CONTEXT.md]
**Why it happens:** Live network availability and peer behavior are nondeterministic. [VERIFIED: .planning/REQUIREMENTS.md; VERIFIED: scripts/run-live-mainnet-smoke.ts]
**How to avoid:** Accept durable same-datadir resume plus typed blocker diagnosis; report post-restart progress delta only when observed. [VERIFIED: 58-CONTEXT.md]
**Warning signs:** Test fixtures require after-restart height increase for every passing restart case. [VERIFIED: 58-CONTEXT.md]

### Pitfall 3: Duplicate Connects Hidden by Height-Only Assertions
**What goes wrong:** The active height remains correct while the runtime re-requests or reconnects already connected blocks. [VERIFIED: packages/open-bitcoin-node/src/sync/block_reconcile.rs; VERIFIED: packages/open-bitcoin-node/src/sync/tests.rs]
**Why it happens:** Height snapshots alone do not reveal outbound `getdata` attempts for already connected hashes. [VERIFIED: packages/open-bitcoin-node/src/sync/tests.rs]
**How to avoid:** Assert `getdata_block_hashes` excludes already connected hashes and durable progress still reports the expected connected hash after reopen. [VERIFIED: packages/open-bitcoin-node/src/sync/tests.rs]
**Warning signs:** Resume tests only compare `best_block_height` and never inspect sent messages or hashes. [VERIFIED: packages/open-bitcoin-node/src/sync/tests.rs]

### Pitfall 4: Peer Guidance Masking Store Health
**What goes wrong:** Operator guidance says "try another peer" when the datadir has schema mismatch or corruption. [VERIFIED: 58-CONTEXT.md; VERIFIED: docs/operator/runtime-guide.md]
**Why it happens:** Existing live-smoke `NoProgressCause` has one `storage_failure` category, while Phase 58 needs `store_corruption` and `store_incompatibility`. [VERIFIED: scripts/run-live-mainnet-smoke.ts; VERIFIED: 58-CONTEXT.md]
**How to avoid:** Check storage last-error/recovery strings before endpoint or peer outcomes. [VERIFIED: 58-CONTEXT.md; VERIFIED: packages/open-bitcoin-node/src/storage.rs]
**Warning signs:** Diagnosis fixtures put `storage_failure` after `handshake_failure` or `tcp_connection_failure`. [VERIFIED: scripts/test-run-live-mainnet-smoke.sh]

### Pitfall 5: Bloated Support-Bundle Scope
**What goes wrong:** Phase 58 starts allowlisting raw restart snapshots or daemon tails into support bundles. [VERIFIED: 58-CONTEXT.md]
**Why it happens:** `restartResumeEvidence` is support-friendly, but support-bundle allowlisting is Phase 59. [VERIFIED: 58-CONTEXT.md; VERIFIED: docs/operator/runtime-guide.md]
**How to avoid:** Keep Phase 58 report-local; only add a minimal compatibility hook if implementation cannot avoid it. [VERIFIED: 58-CONTEXT.md]
**Warning signs:** Changes touch support bundle redaction tables without a narrow schema-compatibility reason. [VERIFIED: docs/operator/runtime-guide.md]

### Pitfall 6: Default Verification Starts Hitting Public Mainnet
**What goes wrong:** `bash scripts/verify.sh` or CI begins requiring DNS/TCP public peers. [VERIFIED: .planning/REQUIREMENTS.md; VERIFIED: scripts/verify.sh]
**Why it happens:** Live-smoke UAT commands are easy to confuse with deterministic regression gates. [VERIFIED: docs/operator/runtime-guide.md]
**How to avoid:** Use Rust unit/integration tests and `scripts/test-run-live-mainnet-smoke.sh` for default verification; keep real live-smoke commands in docs/UAT. [VERIFIED: scripts/test-run-live-mainnet-smoke.sh; VERIFIED: docs/operator/runtime-guide.md]
**Warning signs:** `scripts/verify.sh` adds `run-live-mainnet-smoke.ts`. [VERIFIED: scripts/verify.sh]

## Code Examples

Verified patterns from existing repo sources:

### Current Status Snapshot Extraction

Source: [VERIFIED: scripts/run-live-mainnet-smoke.ts]

```typescript
const maybeProgress = availableValue(maybeSyncState?.sync?.sync_progress);
const connectedBlockHeight = Number(maybeProgress?.connected_block_height ?? blockHeight);
const downloadedBlockHeight = Number(
  maybeProgress?.downloaded_block_height ?? connectedBlockHeight,
);
```

Use this pattern for before-restart and after-restart summaries so `restartResumeEvidence` stays aligned with existing schema v2 status parsing. [VERIFIED: scripts/run-live-mainnet-smoke.ts]

### Current First-Block Evidence Derivation

Source: [VERIFIED: scripts/run-live-mainnet-smoke.ts]

```typescript
const maybeFirstConnectedBlockProgress = firstBlockProgressEvidence(
  maybeFirstConnectedBlockProgressSnapshots,
  maybeFinalStatus,
  "connected",
);
const maybeFirstDownloadedBlockProgress = firstBlockProgressEvidence(
  maybeFirstDownloadedBlockProgressSnapshots,
  maybeFinalStatus,
  "downloaded",
);
```

Add restart evidence beside this derivation; do not replace Phase 57 evidence. [VERIFIED: 58-CONTEXT.md; VERIFIED: scripts/run-live-mainnet-smoke.ts]

### Current Rust Reopen Pattern

Source: [VERIFIED: packages/open-bitcoin-node/src/sync/tests.rs]

```rust
let store = FjallNodeStore::open(&path).expect("reopen store");
let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
let summary = runtime
    .sync_once(&mut transport, i64::from(child_two.header.time))
    .expect("sync after restart");
```

Use this pattern for the Phase 58 two-pass same-datadir fixtures. [VERIFIED: packages/open-bitcoin-node/src/sync/tests.rs]

### Current Missing-Block Request Guard

Source: [VERIFIED: packages/open-bitcoin-node/src/sync/block_reconcile.rs]

```rust
if requested.len() >= available_global
    || active_chain_hashes.contains(&entry.block_hash)
    || runtime.inflight_blocks.contains(&entry.block_hash)
{
    continue;
}
if runtime.store.load_block(entry.block_hash)?.is_some() {
    runtime.network.note_local_block_hash(entry.block_hash);
    continue;
}
```

This is the duplicate-request prevention surface that RESUME-01 should assert through tests. [VERIFIED: packages/open-bitcoin-node/src/sync/block_reconcile.rs; VERIFIED: 58-CONTEXT.md]

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| v1.3 public-mainnet closeout through diagnosed blockers without successful live progress | v1.4 targets successful opt-in header, block, and restart/resume progress while preserving scoped claims | v1.4 roadmap started 2026-06-02 [VERIFIED: .planning/PROJECT.md; VERIFIED: .planning/ROADMAP.md] | Phase 58 should prove resume evidence, not just blocker diagnosis. [VERIFIED: .planning/ROADMAP.md] |
| Header progress only | `result.firstHeaderProgress` with fresh `openbitcoinsyncstatus` snapshots | Phase 56 [VERIFIED: .planning/phases/56-header-ibd-convergence/56-01-SUMMARY.md] | Phase 58 can reuse header progress as pre-restart evidence but not as restart proof. [VERIFIED: 58-CONTEXT.md] |
| Downloaded and connected block progress conflated | Downloaded and connected block heights/hashes are separate in status and reports | Phase 57 [VERIFIED: .planning/phases/57-block-download-and-connect-progress/57-04-SUMMARY.md; VERIFIED: packages/open-bitcoin-node/src/status.rs] | Restart evidence must report both downloaded and connected state before and after relaunch. [VERIFIED: 58-CONTEXT.md] |
| One-process live-smoke report | Two-session same-datadir restart/resume evidence under `result.restartResumeEvidence` | Phase 58 target [VERIFIED: 58-CONTEXT.md] | The planner should add a compact additive schema field and deterministic fixture coverage. [VERIFIED: 58-CONTEXT.md] |

**Deprecated/outdated for Phase 58:**
- Treating `result.firstHeaderProgress` or `result.firstBlockProgress` alone as restart proof is out of date for Phase 58. [VERIFIED: 58-CONTEXT.md]
- Treating `storage_failure` as a sufficient user-facing recovery category is too coarse for RESUME-03. [VERIFIED: scripts/run-live-mainnet-smoke.ts; VERIFIED: .planning/REQUIREMENTS.md]

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|

All claims in this research were verified against local files, command outputs, or cited official sources; no assumed claims are intentionally present. [VERIFIED: Sources section]

## Open Questions (RESOLVED)

1. **Exact restart flag name**
   - What we know: Context suggests `--restart-after-progress` or equivalent. [VERIFIED: 58-CONTEXT.md]
   - What's unclear: The final CLI spelling is not locked. [VERIFIED: 58-CONTEXT.md]
   - Recommendation: Use `--restart-after-progress` because it describes the trigger and avoids implying service-manager restart policy. [VERIFIED: 58-CONTEXT.md]
   - RESOLVED: Phase 58 plans lock the flag name to `--restart-after-progress`; implementation and docs should use that spelling. [VERIFIED: .planning/phases/58-same-datadir-restart-and-resume-evidence/58-02-PLAN.md; VERIFIED: .planning/phases/58-same-datadir-restart-and-resume-evidence/58-03-PLAN.md]
2. **Duplicate-connect verdict vocabulary**
   - What we know: The required evidence includes a duplicate-connect verdict, and current tests can inspect outbound `getdata` plus durable connected hashes. [VERIFIED: 58-CONTEXT.md; VERIFIED: packages/open-bitcoin-node/src/sync/tests.rs]
   - What's unclear: No existing report enum names this verdict. [VERIFIED: scripts/run-live-mainnet-smoke.ts]
   - Recommendation: Use a conservative enum: `no_duplicate_connect_observed`, `duplicate_connect_suspected`, `unavailable`. [VERIFIED: 58-CONTEXT.md]
   - RESOLVED: Phase 58 uses `no_duplicate_connect_observed`, `duplicate_connect_suspected`, and `unavailable` as the duplicate-connect verdict vocabulary. [VERIFIED: .planning/phases/58-same-datadir-restart-and-resume-evidence/58-02-PLAN.md]
3. **Minimal support-bundle compatibility**
   - What we know: Support-bundle allowlisting is deferred to Phase 59, and raw restart evidence should stay out of the compact object. [VERIFIED: 58-CONTEXT.md]
   - What's unclear: Whether existing support-bundle schema readers need a no-op tolerance change for the new nested result field. [VERIFIED: docs/operator/runtime-guide.md]
   - Recommendation: Avoid support bundle changes unless tests show the new field breaks existing schema v2 summary handling. [VERIFIED: 58-CONTEXT.md; VERIFIED: docs/operator/runtime-guide.md]
   - RESOLVED: Phase 58 does not change support-bundle allowlists unless implementation proves a schema compatibility break; otherwise support-bundle work stays deferred to Phase 59. [VERIFIED: .planning/phases/58-same-datadir-restart-and-resume-evidence/58-03-PLAN.md]

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|-------------|-----------|---------|----------|
| Rust / Cargo | Rust runtime tests and `scripts/verify.sh` | Yes | `rustc 1.94.1`, `cargo 1.94.1` [VERIFIED: command outputs] | None needed |
| Bun | Live-smoke script and fixture tests | Yes | `1.3.9` [VERIFIED: command output] | None needed |
| Bash | `scripts/test-run-live-mainnet-smoke.sh` and repo verification wrappers | Yes | GNU bash `3.2.57` [VERIFIED: command output] | None needed |
| Bazel | Repo-native smoke build and UAT command surface | Yes | `8.6.0` [VERIFIED: command output] | Cargo commands remain UAT fallback, but `scripts/verify.sh` expects Bazel. [VERIFIED: scripts/verify.sh] |
| cargo-llvm-cov | Full `bash scripts/verify.sh` coverage gate | Yes | `0.8.5` [VERIFIED: command output] | None for full verify; narrow tests can run without coverage. [VERIFIED: scripts/verify.sh] |
| Bitcoin Knots baseline submodule | Baseline anchors and parity breadcrumbs | Yes | `v29.3.knots20260210` at `a9aee730466ac67d35a3c03ee24676be5e045878` [VERIFIED: command output] | Run `git submodule update --init --recursive` if missing. [VERIFIED: AGENTS.md] |
| Public mainnet DNS/TCP peers | Optional UAT live smoke only | Not probed by research | N/A | Deterministic Rust and mocked script fixtures remain default verification. [VERIFIED: .planning/REQUIREMENTS.md; VERIFIED: scripts/test-run-live-mainnet-smoke.sh] |

**Missing dependencies with no fallback:** None for deterministic planning and implementation. [VERIFIED: command outputs]

**Missing dependencies with fallback:** Public mainnet reachability was intentionally not probed; use opt-in UAT only and deterministic fixtures for default verification. [VERIFIED: .planning/REQUIREMENTS.md; VERIFIED: scripts/verify.sh]

## Verification Commands

Use these narrow commands during implementation before the full repo gate:

```bash
cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node restart --all-features
bash scripts/test-run-live-mainnet-smoke.sh
bun run scripts/run-live-mainnet-smoke.ts --help
```

Use the repo-native full gate before marking Phase 58 implementation complete:

```bash
bash scripts/verify.sh
```

Use opt-in public UAT after implementation, with the final implemented flag name:

```bash
bun run scripts/run-live-mainnet-smoke.ts \
  --datadir=/tmp/open-bitcoin-mainnet \
  --manual-peer=HOST:8333 \
  --restart-after-progress \
  --timeout-seconds=180 \
  --poll-seconds=10
```

Use same-datadir status checks for operator review:

```bash
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- \
  --datadir=/tmp/open-bitcoin-mainnet \
  sync status --format json

bazel run //packages/open-bitcoin-cli:open_bitcoin -- \
  --datadir=/tmp/open-bitcoin-mainnet \
  sync status --format json
```

These commands align with repo-local verification, script fixtures, and UAT guidance. [VERIFIED: AGENTS.md; VERIFIED: scripts/verify.sh; VERIFIED: docs/operator/runtime-guide.md]

## Security Domain

OWASP states that ASVS provides a basis for testing web-application technical controls and that the latest stable ASVS version is `5.0.0`; unversioned requirement identifiers are assumed to refer to latest content, so any exact ASVS IDs in later security work should be versioned. [CITED: https://owasp.org/www-project-application-security-verification-standard/; CITED: https://github.com/OWASP/ASVS]

The table below keeps the GSD planner template's legacy V2-V6 labels for compatibility, while applying controls to this CLI/RPC/live-smoke phase. [VERIFIED: .planning/config.json; CITED: https://owasp.org/www-project-application-security-verification-standard/]

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|------------------|
| V2 Authentication | No new auth behavior | Do not change RPC auth handling in Phase 58; live-smoke continues using explicit local RPC credentials for the spawned daemon. [VERIFIED: scripts/run-live-mainnet-smoke.ts] |
| V3 Session Management | No web session behavior | No browser/session surface is in scope. [VERIFIED: .planning/REQUIREMENTS.md] |
| V4 Access Control | Limited local process/datadir scope | Keep explicit `--datadir` and opt-in live-smoke activation; do not imply service-manager or unattended production control. [VERIFIED: 58-CONTEXT.md; VERIFIED: scripts/run-live-mainnet-smoke.ts] |
| V5 Input Validation | Yes | Parse CLI flags and peer addresses with existing helpers, preserve command argument arrays, and add typed report unions for restart/recovery states. [VERIFIED: scripts/run-live-mainnet-smoke.ts; CITED: Bright Builds TypeScript standard] |
| V6 Cryptography | No new crypto | Do not add or alter Bitcoin cryptography; restart evidence uses existing status fields and hashes. [VERIFIED: packages/open-bitcoin-node/src/status.rs; VERIFIED: 58-CONTEXT.md] |

### Known Threat Patterns for This Stack

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| OS command injection through daemon/status override strings | Tampering / Elevation of privilege | Keep `spawn` and `execFileSync` argument-array patterns and retain the existing shell-metacharacter fixture guard. [VERIFIED: scripts/run-live-mainnet-smoke.ts; VERIFIED: scripts/test-run-live-mainnet-smoke.sh; CITED: https://owasp.org/www-project-application-security-verification-standard/] |
| Raw datadir/report leakage in compact restart evidence | Information disclosure | Keep raw status snapshots, options, endpoint rows, and daemon tails out of `result.restartResumeEvidence`. [VERIFIED: 58-CONTEXT.md; VERIFIED: docs/operator/runtime-guide.md] |
| Store corruption or schema mismatch misdiagnosed as peer failure | Tampering / Reliability | Apply storage-first recovery diagnosis and distinguish `store_corruption` from `store_incompatibility`. [VERIFIED: 58-CONTEXT.md; VERIFIED: packages/open-bitcoin-node/src/storage.rs] |
| Invalid or duplicate peer block data credited as progress | Tampering | Preserve peer-attributed no-credit paths and assert no duplicate request/connect behavior after reopen. [VERIFIED: packages/open-bitcoin-node/src/sync.rs; VERIFIED: packages/open-bitcoin-node/src/sync/tests.rs] |
| Resource exhaustion during public-network review | Denial of service | Preserve block in-flight caps, max messages, max rounds, disk-space preflight, and `resource_exhaustion` recovery guidance. [VERIFIED: packages/open-bitcoin-node/src/sync/types.rs; VERIFIED: scripts/run-live-mainnet-smoke.ts; VERIFIED: docs/operator/runtime-guide.md] |

## Baseline Anchors

- `packages/bitcoin-knots/test/functional/feature_init.py` explicitly tests interruption during initialization and clean subsequent starts. [VERIFIED: packages/bitcoin-knots/test/functional/feature_init.py]
- `packages/bitcoin-knots/doc/man/bitcoind.1` documents `-datadir`, restart-loaded mempool behavior, and reindex/reindex-chainstate recovery options. [VERIFIED: packages/bitcoin-knots/doc/man/bitcoind.1]
- `packages/bitcoin-knots/src/init.cpp` defines datadir/options, chainstate init/load retry/reindex behavior, and shutdown-request handling during initialization. [VERIFIED: packages/bitcoin-knots/src/init.cpp]
- `packages/bitcoin-knots/src/net_processing.cpp` requests blocks through bounded in-flight peer state and attributes invalid block data to peers without broadening Phase 58 into relay or serving behavior. [VERIFIED: packages/bitcoin-knots/src/net_processing.cpp]
- `packages/bitcoin-knots/src/node/blockstorage.cpp` loads block index DB state, checks block file presence, reads reindexing state, and exposes block data availability checks. [VERIFIED: packages/bitcoin-knots/src/node/blockstorage.cpp]

## Sources

### Primary (HIGH confidence)

- `.planning/phases/58-same-datadir-restart-and-resume-evidence/58-CONTEXT.md` - locked decisions, scope, deferred work, implementation surfaces.
- `.planning/REQUIREMENTS.md` - RESUME-01 through RESUME-03 and public-network/default-verification boundary.
- `.planning/ROADMAP.md` - Phase 58 goal, dependency, success criteria, and v1.4 boundaries.
- `.planning/STATE.md` and `.planning/PROJECT.md` - current milestone state and v1.4 decisions.
- `AGENTS.md`, `AGENTS.bright-builds.md`, `standards-overrides.md` - repo-local and Bright Builds constraints.
- `scripts/run-live-mainnet-smoke.ts` - schema v2 report/result flow, one-process orchestration, status parsing, diagnosis.
- `scripts/test-run-live-mainnet-smoke.sh` - deterministic live-smoke fixture patterns.
- `packages/open-bitcoin-node/src/sync.rs`, `sync/block_reconcile.rs`, `sync/tests.rs`, `status.rs`, `storage.rs`, `sync/types/projection.rs`, `sync/types/summary.rs` - durable reopen, status, recovery, and tests.
- `packages/open-bitcoin-rpc/src/context.rs` and `packages/open-bitcoin-cli/src/operator/status/sync_state.rs` - status/control integration.
- `docs/operator/runtime-guide.md` and `docs/parity/catalog/p2p.md` - operator and parity wording.
- `packages/bitcoin-knots/...` baseline anchors listed in the Baseline Anchors section.

### Secondary (HIGH-MEDIUM confidence)

- Bright Builds pinned standards pages fetched at commit `05f8d7a6c9c2e157ec4f922a05273e72dab97676`: architecture, code shape, verification, testing, Rust, and TypeScript/JavaScript. [CITED: https://raw.githubusercontent.com/bright-builds-llc/bright-builds-rules/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/index.md]
- OWASP ASVS project page and GitHub README for current ASVS version and general verification context. [CITED: https://owasp.org/www-project-application-security-verification-standard/; CITED: https://github.com/OWASP/ASVS]

### Tertiary (LOW confidence)

- None. [VERIFIED: research process]

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - versions and tools verified from pinned repo files and local command outputs. [VERIFIED: rust-toolchain.toml; VERIFIED: packages/Cargo.lock; VERIFIED: command outputs]
- Architecture: HIGH - implementation surfaces were verified in current source files and locked by phase context. [VERIFIED: scripts/run-live-mainnet-smoke.ts; VERIFIED: packages/open-bitcoin-node/src/sync.rs; VERIFIED: 58-CONTEXT.md]
- Pitfalls: HIGH - pitfalls map directly to locked decisions, current one-process runner behavior, deterministic test gaps, and repo verification constraints. [VERIFIED: 58-CONTEXT.md; VERIFIED: scripts/run-live-mainnet-smoke.ts; VERIFIED: scripts/verify.sh]
- Security: MEDIUM - project-specific threat patterns are verified in code; ASVS category mapping is planner-template compatible and should be refined during Phase 59 threat-model closeout. [VERIFIED: scripts/run-live-mainnet-smoke.ts; VERIFIED: 58-CONTEXT.md; CITED: https://owasp.org/www-project-application-security-verification-standard/]

**Research date:** 2026-06-05 [VERIFIED: environment_context.current_date]
**Valid until:** 2026-07-05 for local implementation surfaces; re-check external ASVS/Bright Builds links if security wording is reused after that date. [VERIFIED: current date; CITED: https://github.com/OWASP/ASVS]
