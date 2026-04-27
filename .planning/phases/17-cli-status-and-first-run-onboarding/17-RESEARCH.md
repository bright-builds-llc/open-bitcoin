# Phase 17: CLI Status and First-Run Onboarding - Research

**Researched:** 2026-04-27 [VERIFIED: system date]
**Domain:** Rust CLI operator execution, status snapshots, JSONC config layering, read-only Core/Knots detection [VERIFIED: .planning/phases/17-cli-status-and-first-run-onboarding/17-CONTEXT.md]
**Confidence:** HIGH [VERIFIED: codebase audit plus pinned Knots docs]

<user_constraints>
## User Constraints (from CONTEXT.md)

All items in this section are copied from `.planning/phases/17-cli-status-and-first-run-onboarding/17-CONTEXT.md`. [VERIFIED: .planning/phases/17-cli-status-and-first-run-onboarding/17-CONTEXT.md]

### Locked Decisions

## Implementation Decisions

### Status Command
- **D-01:** `open-bitcoin status` must consume the shared `OpenBitcoinStatusSnapshot` and Phase 16 metrics/log contracts instead of inventing renderer-local status state.
- **D-02:** Human status output should be quiet, information-dense, and support-oriented: show daemon state, version/build provenance, datadir/config paths, network, chain/sync, peers, mempool, wallet, service, logs, metrics, and recent health signals with concise labels.
- **D-03:** JSON status output must be stable, serde-backed, and automation-friendly. Missing stopped-node or unavailable live data stays visible through explicit unavailable fields with reasons.
- **D-04:** Status must work against stopped nodes by collecting local evidence where available and marking live RPC/network/wallet values unavailable rather than failing the whole command.

### First-Run Onboarding
- **D-05:** The first-run wizard asks only practical operator questions: network, datadir, Open Bitcoin JSONC path, metrics/log defaults, whether to detect existing Core/Knots installs, and whether to write the proposed JSONC config.
- **D-06:** Rerunning onboarding must be idempotent. It should report existing answers, propose changes, and require explicit approval before overwriting any managed file.
- **D-07:** Non-interactive onboarding must be automatable with explicit flags and deterministic failures when required values are missing. It must not prompt, infer destructive actions, or overwrite without an explicit force/approve-style flag.
- **D-08:** Wizard decisions are stored only in `open-bitcoin.jsonc` under the Open Bitcoin-owned onboarding/config sections. No Open Bitcoin-only settings are written into `bitcoin.conf`.

### Config and Precedence
- **D-09:** Preserve the documented precedence order: CLI flags > environment > Open Bitcoin JSONC > `bitcoin.conf` > cookies > defaults.
- **D-10:** Keep `bitcoin.conf` compatibility strict. Existing Core/Knots keys remain in `bitcoin.conf`; Open Bitcoin-only keys remain in `open-bitcoin.jsonc` and unknown Open Bitcoin-only keys in `bitcoin.conf` should continue to be rejected.
- **D-11:** Config-path and datadir reporting belongs in both status and onboarding so operators can understand exactly which files were inspected or proposed.

### Core/Knots Detection
- **D-12:** Existing Bitcoin Core and Bitcoin Knots datadir, config, cookie, service, and wallet-candidate detection is read-only in this phase. Detection may inform status/onboarding output, but it must not copy, move, rewrite, or delete user data.
- **D-13:** Detection output should be explicit about uncertainty and source paths, because migration choices are deferred to later phases.

### Command Boundary
- **D-14:** Implement behavior behind the Phase 13 `open-bitcoin` clap operator path while preserving `open-bitcoin-cli` as the baseline-compatible RPC client path.
- **D-15:** Keep rendering and decision logic in pure helpers where practical; shell code owns filesystem, environment, stdin/stdout, and optional RPC collection.

### the agent's Discretion
- Exact formatter layout, section ordering, and helper names are discretionary if human output remains readable and JSON remains stable.
- The first implementation may use local/stopped-node status collectors before live daemon RPC collection, provided running-node support has a typed extension point and tests cover both available and unavailable states.

### Deferred Ideas (OUT OF SCOPE)

## Deferred Ideas

- macOS launchd and Linux systemd install/enable/disable behavior belongs to Phase 18.
- Ratatui dashboard rendering belongs to Phase 19.
- Wallet runtime expansion and multiwallet operator workflows belong to Phase 20.
- Dry-run migration plans and backup-aware mutation belong to Phase 21.
- Real-sync benchmark reports and release hardening belong to Phase 22.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| OBS-01 | Operator can run `open-bitcoin status` against running or stopped nodes and see daemon state, version, commit/build provenance, datadir, config paths, network, chain tip, sync progress, peer counts, mempool summary, wallet summary, service state, log paths, and recent health signals. [VERIFIED: .planning/REQUIREMENTS.md] | Use `OpenBitcoinStatusSnapshot`, `FieldAvailability`, `LogStatus`, `MetricsStatus`, RPC response DTOs, local path evidence, and unavailable reasons. [VERIFIED: packages/open-bitcoin-node/src/status.rs; packages/open-bitcoin-node/src/logging.rs; packages/open-bitcoin-node/src/metrics.rs; packages/open-bitcoin-rpc/src/method.rs] |
| OBS-02 | Operator can request machine-readable status output with stable JSON fields for automation and support. [VERIFIED: .planning/REQUIREMENTS.md] | Serialize the shared snapshot with `serde_json`, not a renderer-specific DTO. [VERIFIED: packages/open-bitcoin-node/src/status.rs; docs/architecture/status-snapshot.md; https://docs.rs/serde/1.0.228/serde/derive.Serialize.html] |
| CLI-03 | First-party CLI commands use `clap` for the main command tree while preserving baseline-compatible `bitcoin-cli` RPC invocation behavior. [VERIFIED: .planning/REQUIREMENTS.md] | Keep `OperatorCli` and `open-bitcoin-cli` compatibility parsing separate; add an actual `open-bitcoin` binary target. [VERIFIED: packages/open-bitcoin-cli/src/operator.rs; packages/open-bitcoin-cli/src/main.rs; https://doc.rust-lang.org/cargo/guide/project-layout.html] |
| CLI-04 | First-run onboarding is small, idempotent, non-interactive-capable, and never overwrites without approval. [VERIFIED: .planning/REQUIREMENTS.md] | Implement a pure onboarding planner that computes proposed writes and an effect shell that prompts or fails deterministically. [VERIFIED: .planning/phases/17-cli-status-and-first-run-onboarding/17-CONTEXT.md; Bright Builds architecture standard] |
| CLI-05 | Open Bitcoin-only answers live in JSONC without breaking `bitcoin.conf`. [VERIFIED: .planning/REQUIREMENTS.md] | Use `OpenBitcoinConfig`, `OnboardingConfig`, typed metrics/logs/service/dashboard/migration sections, and `jsonc-parser`. [VERIFIED: packages/open-bitcoin-rpc/src/config/open_bitcoin.rs; docs/architecture/config-precedence.md; https://docs.rs/jsonc-parser/latest/jsonc_parser/] |
| CLI-06 | Config precedence among CLI flags, environment, JSONC, `bitcoin.conf`, cookies, and defaults is documented and tested. [VERIFIED: .planning/REQUIREMENTS.md] | Preserve `ConfigPrecedence::ordered_sources()` and extend tests to operator status/onboarding resolution. [VERIFIED: packages/open-bitcoin-rpc/src/config/open_bitcoin.rs; packages/open-bitcoin-rpc/src/config/tests.rs] |
| CLI-07 | CLI startup detects existing Core/Knots datadirs/config files and reports them without mutating them. [VERIFIED: .planning/REQUIREMENTS.md] | Build read-only detection over default datadirs, `bitcoin.conf`, `.cookie`, service files, and wallet candidates. [VERIFIED: packages/bitcoin-knots/doc/files.md; packages/bitcoin-knots/doc/init.md; packages/bitcoin-knots/doc/managing-wallets.md] |
| MIG-02 | Onboarding detects existing Core/Knots installations, datadirs, config files, cookie files, service definitions, and wallet candidates on macOS and Linux. [VERIFIED: .planning/REQUIREMENTS.md] | Detect file/service candidates only; defer migration, copying, backup, and mutation. [VERIFIED: .planning/phases/17-cli-status-and-first-run-onboarding/17-CONTEXT.md; packages/bitcoin-knots/doc/init.md] |
</phase_requirements>

## Project Constraints (from AGENTS.md)

- Use repo-local `AGENTS.md` as the entrypoint, then `AGENTS.bright-builds.md`, `standards-overrides.md`, and pinned Bright Builds standards before planning or implementation. [VERIFIED: AGENTS.md; AGENTS.bright-builds.md; standards-overrides.md]
- Use `rust-toolchain.toml` as the Rust source of truth; the pinned toolchain is Rust `1.94.1`. [VERIFIED: AGENTS.md; rust-toolchain.toml; `cargo --version`]
- Use `bash scripts/verify.sh` as the repo-native verification contract, including Bazel smoke build, parity breadcrumbs, panic guard, file-length checks, fmt, clippy, build, tests, benchmarks, and coverage. [VERIFIED: AGENTS.md; scripts/verify.sh]
- Use Bun for repo-owned higher-level automation scripts and Bash only for thin orchestration wrappers. [VERIFIED: AGENTS.md]
- Keep Open Bitcoin behavior auditable against Bitcoin Knots `29.3.knots20260210`; record intentional in-scope differences in `docs/parity/index.json` and companion docs. [VERIFIED: AGENTS.md; .planning/PROJECT.md]
- Do not use existing Rust Bitcoin libraries in the production path; the project owns its domain model and implementation surface. [VERIFIED: AGENTS.md; .planning/PROJECT.md]
- Keep functional core / imperative shell boundaries: pure decision logic is data-in/data-out, and I/O stays in thin adapters. [VERIFIED: AGENTS.md; Bright Builds architecture standard]
- New first-party Rust source or test files under `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests` require parity breadcrumb comments and `docs/parity/source-breadcrumbs.json` entries. [VERIFIED: AGENTS.md; docs/parity/README.md; scripts/check-parity-breadcrumbs.ts]
- Before a commit in this Rust project, repo guidance requires Rust checks in order: `cargo fmt --all`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo build --all-targets --all-features`, and `cargo test --all-features`; repo-local `scripts/verify.sh` is the broader native contract. [VERIFIED: AGENTS.md; scripts/verify.sh]
- Use `foo.rs` plus `foo/` rather than `foo/mod.rs` for new or touched multi-file Rust modules. [CITED: https://raw.githubusercontent.com/bright-builds-llc/bright-builds-rules/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/languages/rust.md]
- Prefix internal optional Rust names with `maybe_` when the value normally represents `Option`. [CITED: https://raw.githubusercontent.com/bright-builds-llc/bright-builds-rules/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/languages/rust.md]
- Unit tests for pure/business logic are mandatory, and Rust tests should use Arrange, Act, Assert comments when setup is non-trivial. [CITED: https://raw.githubusercontent.com/bright-builds-llc/bright-builds-rules/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/core/testing.md]
- `standards-overrides.md` contains only a placeholder table, so no active local Bright Builds exception changes this phase. [VERIFIED: standards-overrides.md]
- No repo-local project skills were found under `.claude/skills/` or `.agents/skills/`. [VERIFIED: `find . -maxdepth 3 ... SKILL.md`]

## Summary

Phase 17 should implement execution around contracts that already exist: `OperatorCli` defines the clap surface, `OpenBitcoinStatusSnapshot` defines the status JSON shape, `LogStatus` and `MetricsStatus` define Phase 16 evidence, and `OpenBitcoinConfig` defines the JSONC ownership boundary. [VERIFIED: packages/open-bitcoin-cli/src/operator.rs; packages/open-bitcoin-node/src/status.rs; packages/open-bitcoin-node/src/logging.rs; packages/open-bitcoin-node/src/metrics.rs; packages/open-bitcoin-rpc/src/config/open_bitcoin.rs]

The most important planning gap is binary wiring: the package currently has a `src/main.rs` compatibility binary named by the package as `open-bitcoin-cli`, while the operator contract is named `open-bitcoin` and is not an executable target yet. [VERIFIED: packages/open-bitcoin-cli/Cargo.toml; packages/open-bitcoin-cli/src/main.rs; packages/open-bitcoin-cli/BUILD.bazel; https://doc.rust-lang.org/cargo/guide/project-layout.html] The plan should add a real operator binary and Bazel target, while preserving the existing `open-bitcoin-cli` path and tests. [VERIFIED: docs/architecture/cli-command-architecture.md; packages/open-bitcoin-cli/src/operator.rs]

The safest implementation shape is a pure planning/rendering core plus thin filesystem/RPC/stdin shells. [VERIFIED: Bright Builds architecture standard; .planning/phases/17-cli-status-and-first-run-onboarding/17-CONTEXT.md] Status should always produce the shared snapshot, JSON should be `serde_json` over that snapshot, onboarding should compute proposed JSONC writes before any mutation, and Core/Knots detection should report read-only candidates with source paths and uncertainty. [VERIFIED: docs/architecture/status-snapshot.md; docs/architecture/config-precedence.md; packages/bitcoin-knots/doc/files.md; packages/bitcoin-knots/doc/init.md]

**Primary recommendation:** Implement `open-bitcoin` as a separate operator binary in `open-bitcoin-cli`, backed by pure `operator/status`, `operator/onboarding`, and `operator/detect` helpers, with shared RPC HTTP code exposed from the library for live status collection. [VERIFIED: synthesis from packages/open-bitcoin-cli/src/operator.rs; packages/open-bitcoin-cli/src/client.rs; packages/open-bitcoin-rpc/src/method.rs]

## Standard Stack

### Core

| Library / Contract | Version | Purpose | Why Standard |
|--------------------|---------|---------|--------------|
| `clap` derive | 4.6.1 [VERIFIED: packages/Cargo.lock] | Parse the `open-bitcoin` operator command tree, global flags, subcommands, and value enums. [VERIFIED: packages/open-bitcoin-cli/src/operator.rs] | The repo already uses `Parser`, `Args`, `Subcommand`, and `ValueEnum`, and clap derive is the locked CLI decision. [VERIFIED: .planning/phases/17-cli-status-and-first-run-onboarding/17-CONTEXT.md; https://docs.rs/clap/latest/clap/_derive/index.html] |
| `serde` / `serde_json` | `serde` 1.0.228, `serde_json` 1.0.149 [VERIFIED: packages/Cargo.lock] | Serialize stable status JSON and JSONC-backed config structs. [VERIFIED: packages/open-bitcoin-node/src/status.rs; packages/open-bitcoin-rpc/src/config/open_bitcoin.rs] | Status/config structs already derive `Serialize` and `Deserialize`, and JSON output must be serde-backed. [VERIFIED: packages/open-bitcoin-node/src/status.rs; https://docs.rs/serde/1.0.228/serde/derive.Serialize.html] |
| `jsonc-parser` with `serde` feature | 0.32.3 [VERIFIED: packages/Cargo.lock] | Parse user-editable `open-bitcoin.jsonc` with comments and trailing commas. [VERIFIED: packages/open-bitcoin-rpc/src/config/open_bitcoin.rs; packages/open-bitcoin-rpc/src/config/tests.rs] | The crate supports JSONC parsing into serde values/types when the `serde` feature is enabled. [CITED: https://docs.rs/jsonc-parser/latest/jsonc_parser/] |
| `ureq` with `json` feature | 3.3.0 [VERIFIED: packages/Cargo.lock] | Perform blocking local HTTP JSON-RPC calls for running-node status. [VERIFIED: packages/open-bitcoin-cli/src/client.rs; packages/open-bitcoin-cli/Cargo.toml] | The existing CLI already uses `ureq::Agent`, `send_json`, and `read_json`; `ureq` documents JSON support through the `json` feature. [VERIFIED: packages/open-bitcoin-cli/src/client.rs; CITED: https://docs.rs/ureq/3.3.0/ureq/] |
| `OpenBitcoinStatusSnapshot` | First-party 0.1.0 workspace [VERIFIED: packages/Cargo.toml] | Shared status model for CLI status, service diagnostics, dashboard, and support output. [VERIFIED: packages/open-bitcoin-node/src/status.rs] | Phase 17 decisions require this model and explicit unavailable fields. [VERIFIED: .planning/phases/17-cli-status-and-first-run-onboarding/17-CONTEXT.md; docs/architecture/status-snapshot.md] |
| `OpenBitcoinConfig` | First-party 0.1.0 workspace [VERIFIED: packages/Cargo.toml] | JSONC config owner for onboarding, metrics, logs, service, dashboard, migration, storage, and sync settings. [VERIFIED: packages/open-bitcoin-rpc/src/config/open_bitcoin.rs] | It already encodes the Open Bitcoin-only config boundary and denies unknown fields. [VERIFIED: packages/open-bitcoin-rpc/src/config/open_bitcoin.rs] |

### Supporting

| Library / Tool | Version | Purpose | When to Use |
|----------------|---------|---------|-------------|
| `FjallNodeStore::load_metrics_status` | First-party wrapper over `fjall` 3.1.4 [VERIFIED: packages/open-bitcoin-node/src/storage/fjall_store.rs; packages/Cargo.lock] | Report metrics availability when a local Open Bitcoin store exists. [VERIFIED: packages/open-bitcoin-node/src/storage/fjall_store.rs] | Use for stopped-node metrics evidence; mark unavailable when no metrics snapshot exists. [VERIFIED: packages/open-bitcoin-node/src/storage/fjall_store.rs] |
| `logging::writer::load_log_status` | First-party [VERIFIED: packages/open-bitcoin-node/src/logging/writer.rs] | Load bounded recent warning/error signals from managed JSONL logs. [VERIFIED: packages/open-bitcoin-node/src/logging/writer.rs] | Use for stopped-node and local status evidence when a log directory is configured or discoverable. [VERIFIED: packages/open-bitcoin-node/src/logging/writer.rs] |
| `std::fs`, `std::path`, `std::io` | Rust std with toolchain 1.94.1 [VERIFIED: rust-toolchain.toml; `cargo --version`] | File detection, explicit write approval, stdin/stdout prompting. [VERIFIED: .planning/phases/17-cli-status-and-first-run-onboarding/17-CONTEXT.md] | Use for the thin imperative shell; keep decision logic pure and testable. [VERIFIED: Bright Builds architecture standard] |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Stdlib prompt shell | `dialoguer` or `inquire` [ASSUMED] | Do not add a prompt dependency in this phase because the wizard asks a small fixed question set and repo policy minimizes dependencies. [VERIFIED: AGENTS.md; .planning/phases/17-cli-status-and-first-run-onboarding/17-CONTEXT.md] |
| Full JSONC comment-preserving edits | `jsonc-parser` CST feature [CITED: https://docs.rs/jsonc-parser/latest/jsonc_parser/] | The current dependency enables `serde`, not `cst`; first implementation should write a full proposed config only after explicit approval rather than hand-roll a comment-preserving patcher. [VERIFIED: packages/open-bitcoin-rpc/Cargo.toml; docs/architecture/config-precedence.md] |
| Status-specific JSON DTO | Custom renderer model [VERIFIED: docs/architecture/status-snapshot.md] | Reject this; Phase 17 is locked to `OpenBitcoinStatusSnapshot` so dashboard/status/service consumers share one shape. [VERIFIED: .planning/phases/17-cli-status-and-first-run-onboarding/17-CONTEXT.md] |
| Writing Open Bitcoin settings to `bitcoin.conf` | Extend the existing loader [VERIFIED: packages/open-bitcoin-rpc/src/config/loader.rs] | Reject this; unknown Open Bitcoin-only keys in `bitcoin.conf` must continue to fail. [VERIFIED: docs/architecture/config-precedence.md; packages/open-bitcoin-rpc/src/config/tests.rs] |

**Installation:**

```bash
# No new crates are required for the recommended first implementation. [VERIFIED: packages/open-bitcoin-cli/Cargo.toml; packages/open-bitcoin-rpc/Cargo.toml; packages/open-bitcoin-node/Cargo.toml]
```

**Version verification:** Versions above were verified against `packages/Cargo.lock` and current workspace manifests rather than training data. [VERIFIED: packages/Cargo.lock; packages/open-bitcoin-cli/Cargo.toml; packages/open-bitcoin-rpc/Cargo.toml; packages/open-bitcoin-node/Cargo.toml]

## Architecture Patterns

### Recommended Project Structure

```text
packages/open-bitcoin-cli/
├── src/main.rs                  # Preserve open-bitcoin-cli compatibility binary. [VERIFIED: packages/open-bitcoin-cli/src/main.rs]
├── src/bin/open-bitcoin.rs      # Add operator binary target. [CITED: https://doc.rust-lang.org/cargo/guide/project-layout.html]
├── src/lib.rs                   # Export operator execution and shared RPC helpers. [VERIFIED: packages/open-bitcoin-cli/src/lib.rs]
├── src/operator.rs              # Existing clap contract; add submodule declarations here. [VERIFIED: packages/open-bitcoin-cli/src/operator.rs]
├── src/operator/status.rs       # Pure status collection planning and snapshot rendering. [VERIFIED: docs/architecture/status-snapshot.md]
├── src/operator/onboarding.rs   # Pure onboarding answers -> proposed JSONC write plan. [VERIFIED: .planning/phases/17-cli-status-and-first-run-onboarding/17-CONTEXT.md]
├── src/operator/detect.rs       # Read-only Core/Knots datadir/config/cookie/service/wallet detection. [VERIFIED: packages/bitcoin-knots/doc/files.md; packages/bitcoin-knots/doc/init.md]
├── src/operator/config_paths.rs # Shared config/datadir/JSONC path resolution and source reporting. [VERIFIED: docs/architecture/config-precedence.md]
└── src/rpc_client.rs            # Shared local JSON-RPC HTTP client extracted from binary-only client.rs. [VERIFIED: packages/open-bitcoin-cli/src/client.rs]
```

### Pattern 1: Operator Binary Boundary

**What:** Add a real `open-bitcoin` binary that parses `OperatorCli` and dispatches only operator commands; keep `open-bitcoin-cli` as the current compatibility binary. [VERIFIED: packages/open-bitcoin-cli/src/operator.rs; packages/open-bitcoin-cli/src/main.rs; docs/architecture/cli-command-architecture.md]

**When to use:** Use for `status`, `config`, `onboard`, and future `service`/`dashboard`; never route Bitcoin-style positional RPC calls through this path. [VERIFIED: docs/architecture/cli-command-architecture.md]

**Example:**

```rust
// Source: packages/open-bitcoin-cli/src/operator.rs + Cargo package layout docs.
let parsed = OperatorCli::try_parse_from(argv)?;
match parsed.command {
    OperatorCommand::Status(args) => run_status(parsed, args),
    OperatorCommand::Onboard(args) => run_onboarding(parsed, args),
    _ => todo!("future operator commands"),
}
```

### Pattern 2: Snapshot-First Status

**What:** Every status collector returns or contributes to `OpenBitcoinStatusSnapshot`, and unavailable live data uses `FieldAvailability::Unavailable { reason }`. [VERIFIED: packages/open-bitcoin-node/src/status.rs; docs/architecture/status-snapshot.md]

**When to use:** Use for both live RPC collection and stopped-node filesystem-only inspection. [VERIFIED: docs/architecture/status-snapshot.md]

**Example:**

```rust
// Source: packages/open-bitcoin-node/src/status.rs.
let peers = PeerStatus {
    peer_counts: FieldAvailability::unavailable("node stopped"),
};
let json = serde_json::to_string_pretty(&snapshot)?;
```

### Pattern 3: Pure Onboarding Plan, Effectful Apply

**What:** Represent onboarding as `existing_config + requested_answers + mode -> OnboardingPlan`, where the plan contains proposed paths, file contents, detected existing installs, prompts needed, and write actions. [VERIFIED: .planning/phases/17-cli-status-and-first-run-onboarding/17-CONTEXT.md; Bright Builds architecture standard]

**When to use:** Use before any prompt or write so non-interactive mode can fail with deterministic missing-value errors. [VERIFIED: .planning/phases/17-cli-status-and-first-run-onboarding/17-CONTEXT.md]

**Example:**

```rust
// Source: architecture recommendation from Phase 17 decisions.
let plan = plan_onboarding(existing_config, requested_answers, mode)?;
if plan.requires_prompt() && mode.non_interactive {
    return Err(CliError::new(plan.missing_inputs_message()));
}
apply_approved_plan(plan, shell)?;
```

### Pattern 4: Read-Only Existing Install Detection

**What:** Detect candidates by path/source/category, not by migration readiness. [VERIFIED: .planning/phases/17-cli-status-and-first-run-onboarding/17-CONTEXT.md]

**When to use:** Use in both `status` and `onboard` so operators can see what was inspected without any mutation. [VERIFIED: .planning/phases/17-cli-status-and-first-run-onboarding/17-CONTEXT.md]

**Detector inputs:** home directory, optional fake root for tests, optional explicit datadir/config paths, and OS family. [VERIFIED: packages/open-bitcoin-cli/src/startup/tests.rs; packages/open-bitcoin-rpc/src/config/tests.rs]

**Candidate paths to cover first:**
- Linux default datadir `$HOME/.bitcoin/`, config `$HOME/.bitcoin/bitcoin.conf`, cookie `$HOME/.bitcoin/.cookie`, wallets `$HOME/.bitcoin/wallets/` if present, and fallback default wallet in datadir if `wallets/` is absent. [VERIFIED: packages/bitcoin-knots/doc/files.md; packages/bitcoin-knots/doc/managing-wallets.md]
- macOS default datadir `$HOME/Library/Application Support/Bitcoin/`, config `.../bitcoin.conf`, cookie `.../.cookie`, wallets `.../wallets/`, and launch agent `~/Library/LaunchAgents/org.bitcoin.bitcoind.plist`. [VERIFIED: packages/bitcoin-knots/doc/files.md; packages/bitcoin-knots/doc/init.md; packages/bitcoin-knots/contrib/init/org.bitcoin.bitcoind.plist]
- Linux packaged service candidates such as `/usr/lib/systemd/system/bitcoind.service`, `/lib/systemd/system/bitcoind.service`, `/etc/systemd/system/bitcoind.service`, `/etc/init.d/bitcoind`, and `/etc/init/bitcoind.conf`. [VERIFIED: packages/bitcoin-knots/doc/init.md; packages/bitcoin-knots/contrib/init/README.md; packages/bitcoin-knots/contrib/init/bitcoind.service]

### Pattern 5: Config Precedence Trace

**What:** Return a resolved config plus a source trace for each displayed path/value. [VERIFIED: docs/architecture/config-precedence.md; packages/open-bitcoin-rpc/src/config/open_bitcoin.rs]

**When to use:** Use for status and onboarding path reporting so operators can see whether a value came from CLI flags, environment, JSONC, `bitcoin.conf`, cookies, or defaults. [VERIFIED: .planning/phases/17-cli-status-and-first-run-onboarding/17-CONTEXT.md]

**Required precedence:** CLI flags > environment > Open Bitcoin JSONC > `bitcoin.conf` > cookies > defaults. [VERIFIED: docs/architecture/config-precedence.md; packages/open-bitcoin-rpc/src/config/tests.rs]

### Anti-Patterns to Avoid

- **Renderer-local status fields:** This violates the shared status model and creates dashboard divergence. [VERIFIED: .planning/phases/17-cli-status-and-first-run-onboarding/17-CONTEXT.md; docs/architecture/status-snapshot.md]
- **Prompting in non-interactive mode:** This breaks automation and can hang CI/operator scripts. [VERIFIED: .planning/phases/17-cli-status-and-first-run-onboarding/17-CONTEXT.md]
- **Implicit overwrites:** Existing JSONC or detected Core/Knots data must not be overwritten without explicit approval. [VERIFIED: .planning/phases/17-cli-status-and-first-run-onboarding/17-CONTEXT.md]
- **`bitcoin.conf` mutation for Open Bitcoin settings:** This breaks the compatibility boundary and existing tests reject Open Bitcoin-only keys there. [VERIFIED: docs/architecture/config-precedence.md; packages/open-bitcoin-rpc/src/config/tests.rs]
- **Adding a broad interactive prompt dependency:** This conflicts with the small fixed wizard scope and minimal dependency policy. [VERIFIED: AGENTS.md; .planning/phases/17-cli-status-and-first-run-onboarding/17-CONTEXT.md]

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| CLI parsing | Manual argv parser for operator commands | `clap` derive in `OperatorCli` | The contract already uses clap, and docs support `Parser`, `Args`, `Subcommand`, and `ValueEnum`. [VERIFIED: packages/open-bitcoin-cli/src/operator.rs; CITED: https://docs.rs/clap/latest/clap/_derive/index.html] |
| Status JSON | Hand-written JSON strings | `serde_json::to_string_pretty(&OpenBitcoinStatusSnapshot)` | The snapshot is already serde-backed and unavailable fields serialize explicitly. [VERIFIED: packages/open-bitcoin-node/src/status.rs] |
| JSONC parsing | Regex/comment-stripping parser | `parse_open_bitcoin_jsonc_config` | The existing function uses `jsonc-parser` with serde support and tests comments/trailing commas. [VERIFIED: packages/open-bitcoin-rpc/src/config/open_bitcoin.rs; packages/open-bitcoin-rpc/src/config/tests.rs; CITED: https://docs.rs/jsonc-parser/latest/jsonc_parser/] |
| Bitcoin-style config semantics | A new `bitcoin.conf` parser in CLI | Existing `load_runtime_config_for_args` or a small exported wrapper around it | The loader already handles supported keys, sections, includes, relative paths, cookie auth fallback, and strict unknown-key errors. [VERIFIED: packages/open-bitcoin-rpc/src/config/loader.rs; packages/open-bitcoin-rpc/src/config/tests.rs] |
| Local RPC HTTP | A second ad-hoc HTTP client | Extract shared helper from `src/client.rs` | Existing code already handles JSON-RPC envelopes, auth headers, HTTP status handling, batch responses, and ureq JSON behavior. [VERIFIED: packages/open-bitcoin-cli/src/client.rs] |
| Metrics/log status | CLI parsing raw logs or metric storage directly | `load_log_status`, `FjallNodeStore::load_metrics_status`, and shared status contracts | Phase 16 already created status-facing APIs and recent signal mapping. [VERIFIED: packages/open-bitcoin-node/src/logging/writer.rs; packages/open-bitcoin-node/src/storage/fjall_store.rs] |
| Service lifecycle | `launchctl`/`systemctl` install or enable logic | Read-only service file detection only | Phase 18 owns service lifecycle mutation; Phase 17 only reports candidates. [VERIFIED: .planning/phases/17-cli-status-and-first-run-onboarding/17-CONTEXT.md; .planning/ROADMAP.md] |
| Wallet migration | Copying or opening wallet DBs | Path-only wallet candidate detection | Knots docs warn wallet files are high-value and backups should use wallet RPC; migration is deferred. [VERIFIED: packages/bitcoin-knots/doc/files.md; packages/bitcoin-knots/doc/managing-wallets.md; .planning/ROADMAP.md] |

**Key insight:** The hard part is not formatting text; it is preserving trust boundaries across status collection, config precedence, auth cookies, and existing user data. [VERIFIED: synthesis from Phase 17 context, config docs, and Knots docs]

## Runtime State Inventory

| Category | Items Found | Action Required |
|----------|-------------|-----------------|
| Stored data | No Open Bitcoin data migration is required by this phase; external Core/Knots datadirs may contain blocks, chainstate, indexes, wallets, `.cookie`, `.lock`, logs, and settings files. [VERIFIED: .planning/ROADMAP.md; packages/bitcoin-knots/doc/files.md] | Code must detect and report paths read-only; no data migration, copy, delete, or rewrite task belongs in Phase 17. [VERIFIED: .planning/phases/17-cli-status-and-first-run-onboarding/17-CONTEXT.md] |
| Live service config | Core/Knots sample service definitions include Linux systemd/OpenRC/Upstart/CentOS files and macOS `org.bitcoin.bitcoind.plist`. [VERIFIED: packages/bitcoin-knots/doc/init.md; packages/bitcoin-knots/contrib/init/README.md] | Detect candidate service definition files by known paths and report uncertainty; do not install, enable, disable, or query manager state beyond safe read-only inspection. [VERIFIED: .planning/ROADMAP.md] |
| OS-registered state | macOS launch agents can live under `~/Library/LaunchAgents/org.bitcoin.bitcoind.plist`; Linux services can live under systemd/init locations named for `bitcoind`. [VERIFIED: packages/bitcoin-knots/doc/init.md] | Report found service definition paths; leave actual registered/running/enabled status unavailable until Phase 18 unless a no-mutation, testable read-only probe is explicitly planned. [VERIFIED: .planning/ROADMAP.md] |
| Secrets/env vars | Existing RPC cookies live at `.cookie` by default and may be overridden by `-rpccookiefile`; Open Bitcoin env vars are `OPEN_BITCOIN_CONFIG`, `OPEN_BITCOIN_DATADIR`, and `OPEN_BITCOIN_NETWORK`. [VERIFIED: packages/bitcoin-knots/doc/files.md; packages/bitcoin-knots/doc/init.md; docs/architecture/config-precedence.md] | Report cookie path existence only; never read or print cookie contents, passwords, or authorization headers in status/onboarding output. [VERIFIED: packages/open-bitcoin-cli/src/client.rs; OWASP ASVS V2/V6 guidance] |
| Build artifacts | No rename/refactor build artifact migration is required; adding an `open-bitcoin` binary requires Cargo and Bazel target updates. [VERIFIED: packages/open-bitcoin-cli/Cargo.toml; packages/open-bitcoin-cli/BUILD.bazel; https://doc.rust-lang.org/cargo/guide/project-layout.html] | Add/verify Cargo bin target, Bazel `rust_binary`, root alias if needed, and integration tests using `CARGO_BIN_EXE_open-bitcoin`. [VERIFIED: packages/open-bitcoin-cli/tests/operator_flows.rs; packages/open-bitcoin-cli/BUILD.bazel] |

**Nothing found in category:** No runtime Open Bitcoin rename state, persistent service registration created by earlier phases, or installed Open Bitcoin operator binary state needs migration. [VERIFIED: .planning/STATE.md; rg audit of open-bitcoin service/operator surfaces]

## Common Pitfalls

### Pitfall 1: Clap Contract Without an Executable
**What goes wrong:** Tests pass for `OperatorCli`, but users still cannot run `open-bitcoin status`. [VERIFIED: packages/open-bitcoin-cli/src/operator/tests.rs; packages/open-bitcoin-cli/Cargo.toml]
**Why it happens:** The package currently has a compatibility `src/main.rs` binary and no `src/bin/open-bitcoin.rs` operator binary. [VERIFIED: packages/open-bitcoin-cli/src/main.rs; packages/open-bitcoin-cli/Cargo.toml]
**How to avoid:** Plan Cargo and Bazel binary targets as first-class work, then add integration tests invoking `CARGO_BIN_EXE_open-bitcoin`. [VERIFIED: https://doc.rust-lang.org/cargo/guide/project-layout.html; packages/open-bitcoin-cli/tests/operator_flows.rs]
**Warning signs:** `cargo test -p open-bitcoin-cli` has only `CARGO_BIN_EXE_open-bitcoin-cli` integration coverage. [VERIFIED: packages/open-bitcoin-cli/tests/operator_flows.rs]

### Pitfall 2: Treating Stopped Node Status as Failure
**What goes wrong:** `open-bitcoin status` exits non-zero when RPC is unavailable even though local datadir/config/log evidence exists. [VERIFIED: docs/architecture/status-snapshot.md]
**Why it happens:** Live RPC is mistaken for the only status source. [VERIFIED: docs/architecture/status-snapshot.md]
**How to avoid:** Build a snapshot with local fields available and live fields unavailable with explicit reasons. [VERIFIED: packages/open-bitcoin-node/src/status.rs]
**Warning signs:** JSON omits `sync`, `peers`, `mempool`, or `wallet` instead of serializing unavailable states. [VERIFIED: packages/open-bitcoin-node/src/status.rs]

### Pitfall 3: Breaking `bitcoin.conf` Compatibility
**What goes wrong:** Open Bitcoin-only keys such as `dashboard` or `service` are accepted in `bitcoin.conf`. [VERIFIED: packages/open-bitcoin-rpc/src/config/tests.rs]
**Why it happens:** Onboarding code writes or reads Open Bitcoin-only state through the baseline config loader. [VERIFIED: docs/architecture/config-precedence.md]
**How to avoid:** Keep Open Bitcoin-only settings in `open-bitcoin.jsonc`; use `bitcoin.conf` only for baseline-compatible keys. [VERIFIED: docs/architecture/config-precedence.md; packages/open-bitcoin-rpc/src/config/open_bitcoin.rs]
**Warning signs:** Tests add `dashboard=1`, `service=1`, or wizard keys to `bitcoin.conf`. [VERIFIED: packages/open-bitcoin-rpc/src/config/tests.rs]

### Pitfall 4: Prompting or Guessing in Non-Interactive Mode
**What goes wrong:** Automation hangs or writes an unintended config. [VERIFIED: .planning/phases/17-cli-status-and-first-run-onboarding/17-CONTEXT.md]
**Why it happens:** Interactive and non-interactive flows share an effectful implementation without a pure planning step. [VERIFIED: Bright Builds architecture standard]
**How to avoid:** Parse flags into `OnboardingAnswers`; return missing-value errors before any prompt or write when `--non-interactive` is set. [VERIFIED: .planning/phases/17-cli-status-and-first-run-onboarding/17-CONTEXT.md]
**Warning signs:** Tests need stdin for non-interactive mode, or non-interactive tests have to close stdin to terminate. [VERIFIED: packages/open-bitcoin-cli/tests/operator_flows.rs]

### Pitfall 5: Leaking RPC Credentials
**What goes wrong:** Status or onboarding prints cookie contents, `rpcpassword`, or Basic auth headers. [VERIFIED: packages/open-bitcoin-cli/src/client.rs]
**Why it happens:** Reusing HTTP auth code without redaction at the display boundary. [VERIFIED: packages/open-bitcoin-cli/src/client.rs]
**How to avoid:** Display only credential source and path availability; never include secret values in JSON or human output. [VERIFIED: OWASP ASVS V2/V6 guidance; packages/bitcoin-knots/doc/init.md]
**Warning signs:** Snapshot fields contain `password`, `authorization`, `cookie_value`, or raw `username:password`. [VERIFIED: rg audit target terms]

### Pitfall 6: Overwriting User JSONC Comments Silently
**What goes wrong:** Rerunning onboarding replaces a manually edited JSONC file and removes comments. [VERIFIED: docs/architecture/config-precedence.md]
**Why it happens:** `serde_json::to_string_pretty` writes valid JSON but does not preserve JSONC comments. [VERIFIED: packages/open-bitcoin-rpc/src/config/open_bitcoin.rs; https://docs.rs/jsonc-parser/latest/jsonc_parser/]
**How to avoid:** For existing files, show proposed full contents or a textual summary and require explicit approval/force; do not claim comment-preserving edits unless the CST path is deliberately implemented. [VERIFIED: .planning/phases/17-cli-status-and-first-run-onboarding/17-CONTEXT.md; https://docs.rs/jsonc-parser/latest/jsonc_parser/]
**Warning signs:** An existing `open-bitcoin.jsonc` is rewritten in tests without an approval flag. [VERIFIED: .planning/phases/17-cli-status-and-first-run-onboarding/17-CONTEXT.md]

### Pitfall 7: Treating Detection as Migration
**What goes wrong:** Onboarding copies wallets, moves datadirs, or edits Core/Knots service files. [VERIFIED: .planning/ROADMAP.md]
**Why it happens:** MIG-02 detection work is conflated with Phase 21 migration. [VERIFIED: .planning/ROADMAP.md]
**How to avoid:** Emit read-only `DetectedInstallation` records with confidence/source paths and defer all mutation. [VERIFIED: .planning/phases/17-cli-status-and-first-run-onboarding/17-CONTEXT.md]
**Warning signs:** Phase 17 code opens wallet databases for write, invokes migration RPCs, or runs `launchctl load` / `systemctl enable`. [VERIFIED: .planning/ROADMAP.md]

## Code Examples

Verified patterns from current code:

### Stable Unavailable Status Field

```rust
// Source: packages/open-bitcoin-node/src/status.rs
let value = FieldAvailability::<String>::unavailable("node stopped");
let encoded = serde_json::to_value(&value)?;
assert_eq!(encoded["state"], "unavailable");
assert_eq!(encoded["value"]["reason"], "node stopped");
```

### JSONC Parsing Through Existing Contract

```rust
// Source: packages/open-bitcoin-rpc/src/config/open_bitcoin.rs
pub fn parse_open_bitcoin_jsonc_config(text: &str) -> Result<OpenBitcoinConfig, ConfigError> {
    jsonc_parser::parse_to_serde_value(text, &Default::default()).map_err(|error| {
        ConfigError::new(format!("Error reading {OPEN_BITCOIN_CONFIG_FILE_NAME}: {error}"))
    })
}
```

### Precedence Source Order

```rust
// Source: packages/open-bitcoin-rpc/src/config/open_bitcoin.rs
ConfigPrecedence::ordered_sources()
// [CliFlags, Environment, OpenBitcoinJsonc, BitcoinConf, Cookies, Defaults]
```

### Status Mapping From Existing RPC DTOs

```rust
// Source: packages/open-bitcoin-rpc/src/method.rs and packages/open-bitcoin-node/src/status.rs
let sync = SyncStatus {
    network: FieldAvailability::available(blockchain.chain),
    chain_tip: maybe_best_hash.map_or_else(
        || FieldAvailability::unavailable("best block hash unavailable"),
        |hash| FieldAvailability::available(ChainTipStatus { height, block_hash: hash }),
    ),
    sync_progress: FieldAvailability::available(SyncProgress {
        header_height: blockchain.headers.into(),
        block_height: blockchain.blocks.into(),
        progress_ratio: blockchain.verificationprogress,
        messages_processed: 0,
        headers_received: 0,
        blocks_received: 0,
    }),
};
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Status renderers each infer their own fields. [VERIFIED: docs/architecture/status-snapshot.md] | Shared `OpenBitcoinStatusSnapshot` with explicit unavailable states. [VERIFIED: packages/open-bitcoin-node/src/status.rs] | Phase 13 [VERIFIED: .planning/ROADMAP.md] | CLI, service, dashboard, and support output can share one JSON shape. [VERIFIED: docs/architecture/status-snapshot.md] |
| Runtime evidence unavailable to status consumers. [VERIFIED: .planning/phases/16-metrics-logs-and-sync-telemetry/16-CONTEXT.md] | Metrics/log contracts and bounded recent warning/error access exist. [VERIFIED: packages/open-bitcoin-node/src/metrics.rs; packages/open-bitcoin-node/src/logging.rs] | Phase 16 [VERIFIED: .planning/ROADMAP.md] | Phase 17 should consume status-facing APIs instead of parsing raw files ad hoc. [VERIFIED: packages/open-bitcoin-node/src/logging/writer.rs; packages/open-bitcoin-node/src/storage/fjall_store.rs] |
| All config state in `bitcoin.conf`. [VERIFIED: docs/architecture/config-precedence.md] | Open Bitcoin-only settings in `open-bitcoin.jsonc`; baseline keys stay in `bitcoin.conf`. [VERIFIED: docs/architecture/config-precedence.md] | Phase 13 [VERIFIED: .planning/ROADMAP.md] | Onboarding can store wizard/dashboard/service/migration answers without breaking Core/Knots config semantics. [VERIFIED: packages/open-bitcoin-rpc/src/config/open_bitcoin.rs] |
| CLI crate only exposes `open-bitcoin-cli` runtime behavior. [VERIFIED: packages/open-bitcoin-cli/src/main.rs] | Phase 17 should add an operator binary `open-bitcoin`. [VERIFIED: docs/architecture/cli-command-architecture.md] | Phase 17 planned [VERIFIED: .planning/ROADMAP.md] | User-visible success criterion can be met literally. [VERIFIED: .planning/ROADMAP.md] |

**Deprecated/outdated:**
- Using `open-bitcoin-cli` for Open Bitcoin-specific operator workflows is outdated for Phase 17; the architecture document assigns those workflows to `open-bitcoin`. [VERIFIED: docs/architecture/cli-command-architecture.md]
- Treating a missing RPC endpoint as a total status failure is outdated; stopped-node unavailable-field semantics are already documented. [VERIFIED: docs/architecture/status-snapshot.md]

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | Adding `dialoguer` or `inquire` would be unnecessary for this small wizard. [ASSUMED] | Alternatives Considered | If the wizard becomes more complex than scoped, stdio prompting may require more bespoke validation and UX code. |

## Open Questions

1. **Should existing JSONC comments be preserved on approved overwrite?** [VERIFIED: docs/architecture/config-precedence.md]
   - What we know: JSONC is user-editable, and `jsonc-parser` can parse comments; the current dependency uses the `serde` feature, not the `cst` feature. [VERIFIED: packages/open-bitcoin-rpc/Cargo.toml; https://docs.rs/jsonc-parser/latest/jsonc_parser/]
   - What's unclear: Whether Phase 17 requires comment-preserving in-place edits or can write a full approved replacement. [VERIFIED: .planning/phases/17-cli-status-and-first-run-onboarding/17-CONTEXT.md]
   - Recommendation: Plan full-file replacement only after explicit approval; defer CST editing unless the user explicitly requires comment preservation. [VERIFIED: synthesis from D-06 and dependency policy]

2. **How much live status must be implemented in the first plan wave?** [VERIFIED: .planning/phases/17-cli-status-and-first-run-onboarding/17-CONTEXT.md]
   - What we know: Running-node status must work, and existing RPC DTOs cover network, blockchain, mempool, wallet, and balances. [VERIFIED: .planning/ROADMAP.md; packages/open-bitcoin-rpc/src/method.rs]
   - What's unclear: Whether service state should stay unavailable until Phase 18 or use shallow read-only service definition presence now. [VERIFIED: .planning/ROADMAP.md]
   - Recommendation: Implement live RPC collection for chain/network/mempool/wallet, local service definition detection for "managed/unmanaged candidate", and explicit unavailable state for installed/enabled/running until Phase 18. [VERIFIED: synthesis from Phase 17/18 boundaries]

3. **Should `route_cli_invocation` still be used after adding a separate operator binary?** [VERIFIED: packages/open-bitcoin-cli/src/operator.rs]
   - What we know: The routing helper exists and tests argv0 routing. [VERIFIED: packages/open-bitcoin-cli/src/operator/tests.rs]
   - What's unclear: Whether packaging will also install symlinks that rely on argv0 dispatch. [ASSUMED]
   - Recommendation: Keep the helper and tests, but make the new `open-bitcoin` binary call the operator path directly so success criteria do not depend on symlinks. [VERIFIED: synthesis from Cargo layout docs and CLI architecture]

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|-------------|-----------|---------|----------|
| Rust/Cargo | Build, tests, operator binary work | yes [VERIFIED: `cargo --version`] | cargo 1.94.1 [VERIFIED: `cargo --version`] | None needed |
| Bun | Parity breadcrumb and repo automation scripts | yes [VERIFIED: `bun --version`] | 1.3.9 [VERIFIED: `bun --version`] | None needed |
| Bazel/Bazelisk command | Repo-native verify and Bazel smoke build | yes [VERIFIED: `bazel --version`] | 8.6.0 [VERIFIED: `bazel --version`] | None needed |
| `cargo-llvm-cov` | `scripts/verify.sh` coverage gate | yes [VERIFIED: `cargo llvm-cov --version`] | 0.8.5 [VERIFIED: `cargo llvm-cov --version`] | None needed |
| `launchctl` | Optional macOS read-only service context | yes [VERIFIED: `command -v launchctl`] | `/bin/launchctl` [VERIFIED: `command -v launchctl`] | Prefer file detection for Phase 17 |
| `systemctl` | Optional Linux read-only service context | no [VERIFIED: `command -v systemctl`] | - | Use file detection; service lifecycle is Phase 18 |
| `bitcoind` / `bitcoin-cli` | Optional real Core/Knots runtime probing | no [VERIFIED: `command -v bitcoind`; `command -v bitcoin-cli`] | - | Use fake HTTP server and stopped-node fixtures in default tests |

**Missing dependencies with no fallback:**
- None for Phase 17 default implementation and verification. [VERIFIED: environment probes; scripts/verify.sh]

**Missing dependencies with fallback:**
- `systemctl`, `bitcoind`, and `bitcoin-cli` are missing locally; tests should use fake roots and fake RPC servers, and implementation should not require these commands for default behavior. [VERIFIED: environment probes; packages/open-bitcoin-cli/tests/operator_flows.rs]

## Security Domain

Security enforcement is enabled because `.planning/config.json` does not set `security_enforcement` to `false`. [VERIFIED: .planning/config.json]

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|------------------|
| V2 Authentication | yes [VERIFIED: phase uses RPC cookies/password auth paths] | Do not expose RPC cookie contents, passwords, or Basic auth headers; report only credential source/path availability. [VERIFIED: packages/open-bitcoin-cli/src/client.rs; packages/bitcoin-knots/doc/init.md; CITED: https://devguide.owasp.org/en/06-verification/01-guides/03-asvs/] |
| V3 Session Management | limited [VERIFIED: phase reads transient `.cookie` files but does not create web sessions] | Treat `.cookie` as a secret auth token and never persist it into JSONC/status output. [VERIFIED: packages/bitcoin-knots/doc/files.md; packages/bitcoin-knots/doc/init.md] |
| V4 Access Control | yes [VERIFIED: phase detects high-value user datadirs/wallets] | Read-only detection by default; no writes to existing Core/Knots data, service files, or wallet candidates. [VERIFIED: .planning/phases/17-cli-status-and-first-run-onboarding/17-CONTEXT.md] |
| V5 Validation, Sanitization and Encoding | yes [VERIFIED: phase accepts CLI flags, env vars, paths, JSONC, and RPC JSON] | Parse at boundaries with clap, typed path structs, serde/jsonc-parser, and existing config loader; escape/render paths as display strings only. [VERIFIED: packages/open-bitcoin-cli/src/operator.rs; packages/open-bitcoin-rpc/src/config/open_bitcoin.rs; Bright Builds architecture standard] |
| V6 Stored Cryptography | yes for secrets handling, no new crypto [VERIFIED: phase handles RPC credential sources but adds no cryptographic primitive] | Do not hand-roll crypto; do not store secrets in `open-bitcoin.jsonc`; use existing cookie/user-password auth handling. [VERIFIED: packages/open-bitcoin-cli/src/client.rs; packages/open-bitcoin-rpc/src/config.rs] |

### Known Threat Patterns for Rust CLI/Config Work

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Credential disclosure through status JSON or human output | Information Disclosure | Redact values; include only auth mode and path availability. [VERIFIED: packages/open-bitcoin-cli/src/client.rs; OWASP ASVS V2/V6 source] |
| Path traversal or unintended writes from CLI/env paths | Tampering | Resolve paths explicitly, reject writes without approval, and test with temp roots. [VERIFIED: packages/open-bitcoin-cli/src/startup.rs; packages/open-bitcoin-cli/src/startup/tests.rs] |
| Mutating Core/Knots datadir during detection | Tampering | Detection functions return candidate records and never open files for write. [VERIFIED: .planning/phases/17-cli-status-and-first-run-onboarding/17-CONTEXT.md] |
| Confusing source precedence | Spoofing/Tampering | Preserve source trace in status/onboarding output and test precedence order. [VERIFIED: docs/architecture/config-precedence.md; packages/open-bitcoin-rpc/src/config/tests.rs] |
| Invalid JSONC or unknown Open Bitcoin settings | Tampering/Denial of Service | Use `deny_unknown_fields` config structs and return deterministic parse errors. [VERIFIED: packages/open-bitcoin-rpc/src/config/open_bitcoin.rs; packages/open-bitcoin-rpc/src/config/tests.rs] |

## Sources

### Primary (HIGH confidence)
- `.planning/phases/17-cli-status-and-first-run-onboarding/17-CONTEXT.md` - locked Phase 17 decisions, boundaries, discretion, and deferred work. [VERIFIED: file read]
- `.planning/REQUIREMENTS.md` - OBS-01, OBS-02, CLI-03, CLI-04, CLI-05, CLI-06, CLI-07, MIG-02. [VERIFIED: file read]
- `.planning/ROADMAP.md` - Phase 17 goal, dependencies, and success criteria. [VERIFIED: file read]
- `.planning/PROJECT.md` and `.planning/STATE.md` - milestone state, baseline, and v1.1 decisions. [VERIFIED: file read]
- `AGENTS.md`, `AGENTS.bright-builds.md`, `standards-overrides.md` - repo-local workflow, Bright Builds routing, Rust verification, parity rules, and local overrides. [VERIFIED: file read]
- `docs/architecture/cli-command-architecture.md` - operator/compatibility CLI boundary. [VERIFIED: file read]
- `docs/architecture/status-snapshot.md` - shared snapshot and stopped-node unavailable semantics. [VERIFIED: file read]
- `docs/architecture/config-precedence.md` - JSONC ownership and precedence order. [VERIFIED: file read]
- `docs/architecture/operator-observability.md` - metrics/log retention defaults. [VERIFIED: file read]
- `packages/open-bitcoin-cli/src/operator.rs`, `packages/open-bitcoin-cli/src/main.rs`, `packages/open-bitcoin-cli/src/client.rs`, `packages/open-bitcoin-cli/src/startup.rs` - CLI parser, current binary path, HTTP RPC client, and startup config behavior. [VERIFIED: file read]
- `packages/open-bitcoin-node/src/status.rs`, `logging.rs`, `logging/writer.rs`, `metrics.rs`, `storage/fjall_store.rs`, `sync/types.rs` - status, log, metrics, storage, and sync projection contracts. [VERIFIED: file read]
- `packages/open-bitcoin-rpc/src/config/open_bitcoin.rs`, `config/loader.rs`, `config/tests.rs`, `method.rs` - JSONC config, `bitcoin.conf` loader, precedence tests, and RPC response DTOs. [VERIFIED: file read]
- `packages/bitcoin-knots/doc/files.md`, `doc/init.md`, `doc/managing-wallets.md`, `contrib/init/README.md`, `contrib/init/bitcoind.service`, `contrib/init/org.bitcoin.bitcoind.plist` - pinned baseline datadir, service, cookie, and wallet detection evidence. [VERIFIED: local pinned submodule docs]
- `packages/Cargo.lock`, `packages/open-bitcoin-cli/Cargo.toml`, `packages/open-bitcoin-rpc/Cargo.toml`, `packages/open-bitcoin-node/Cargo.toml`, `rust-toolchain.toml`, `scripts/verify.sh` - dependency/tool versions and verification contract. [VERIFIED: file read]

### Primary External (HIGH confidence)
- Bright Builds pinned standards:
  - `https://raw.githubusercontent.com/bright-builds-llc/bright-builds-rules/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/index.md`
  - `https://raw.githubusercontent.com/bright-builds-llc/bright-builds-rules/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/core/architecture.md`
  - `https://raw.githubusercontent.com/bright-builds-llc/bright-builds-rules/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/core/code-shape.md`
  - `https://raw.githubusercontent.com/bright-builds-llc/bright-builds-rules/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/core/testing.md`
  - `https://raw.githubusercontent.com/bright-builds-llc/bright-builds-rules/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/core/verification.md`
  - `https://raw.githubusercontent.com/bright-builds-llc/bright-builds-rules/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/languages/rust.md`
- Cargo package layout docs - `https://doc.rust-lang.org/cargo/guide/project-layout.html` for adding `src/bin/open-bitcoin.rs`. [CITED: official Cargo docs]
- clap derive docs - `https://docs.rs/clap/latest/clap/_derive/index.html` for derive parser/subcommand/value enum support. [CITED: docs.rs]
- jsonc-parser docs - `https://docs.rs/jsonc-parser/latest/jsonc_parser/` for parsing JSONC to serde values/types and CST caveat. [CITED: docs.rs]
- serde derive docs - `https://docs.rs/serde/1.0.228/serde/derive.Serialize.html` for `Serialize` derive feature. [CITED: docs.rs]
- ureq docs - `https://docs.rs/ureq/3.3.0/ureq/` for JSON request/response and `Agent` behavior. [CITED: docs.rs]
- OWASP ASVS Developer Guide - `https://devguide.owasp.org/en/06-verification/01-guides/03-asvs/` for ASVS category names. [CITED: OWASP]

### Secondary (MEDIUM confidence)
- None used for implementation recommendations. [VERIFIED: research log]

### Tertiary (LOW confidence)
- The prompt-library alternative note is based on dependency-policy judgment rather than a live ecosystem comparison. [ASSUMED]

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - dependencies and versions are pinned in workspace manifests/Cargo.lock and corroborated by docs.rs for external API usage. [VERIFIED: packages/Cargo.lock; docs.rs sources]
- Architecture: HIGH - Phase 13/16 contracts and Bright Builds standards already prescribe shared status models and functional-core/imperative-shell separation. [VERIFIED: docs/architecture/status-snapshot.md; packages/open-bitcoin-node/src/status.rs; Bright Builds architecture standard]
- Pitfalls: HIGH - most pitfalls map directly to locked phase decisions, existing tests, or pinned Knots docs. [VERIFIED: .planning/phases/17-cli-status-and-first-run-onboarding/17-CONTEXT.md; packages/open-bitcoin-rpc/src/config/tests.rs; packages/bitcoin-knots/doc/files.md]
- Core/Knots detection: HIGH for default paths and service files documented in pinned Knots docs; MEDIUM for distro-specific extra service locations beyond those docs. [VERIFIED: packages/bitcoin-knots/doc/init.md]
- JSONC comment preservation: MEDIUM - parser supports CST with a feature not currently enabled, but the phase does not explicitly require preserving comments on overwrite. [VERIFIED: packages/open-bitcoin-rpc/Cargo.toml; https://docs.rs/jsonc-parser/latest/jsonc_parser/]

**Research date:** 2026-04-27 [VERIFIED: system date]
**Valid until:** 2026-05-27 for codebase-local recommendations; re-check dependency docs before adding new crates. [VERIFIED: dependency policy and local lockfile]
