# Phase 13: Operator Runtime Foundations - Research

**Researched:** 2026-04-26  
**Domain:** Rust operator runtime contracts, CLI/config architecture, observability, and durable storage decision-making  
**Confidence:** MEDIUM - storage choice remains the main uncertainty because no project-specific storage benchmark exists yet. [VERIFIED: .planning/ROADMAP.md, .planning/research/PITFALLS.md]

## User Constraints

No Phase 13 `*-CONTEXT.md` exists, so there are no locked decisions, discretion notes, or deferred ideas to copy verbatim from discuss-phase context. [VERIFIED: `gsd-tools init phase-op 13`]

Binding scope comes from Phase 13 in `.planning/ROADMAP.md`: define dependency, command, config, status, metrics, log, and storage contracts before later v1.1 work consumes them. [VERIFIED: .planning/ROADMAP.md]

Phase 13 must address OBS-01, OBS-03, OBS-04, CLI-03, CLI-05, CLI-06, and DB-01. [VERIFIED: .planning/REQUIREMENTS.md, .planning/ROADMAP.md]

The project must preserve externally observable behavior from Bitcoin Knots `29.3.knots20260210` for in-scope surfaces and keep parity claims auditable. [VERIFIED: AGENTS.md, .planning/STATE.md]

Pure consensus, chainstate, mempool, wallet, protocol, and domain crates must remain free of direct I/O, runtime side effects, and new database dependencies; storage and runtime work belongs in shell/adapters. [VERIFIED: AGENTS.md, scripts/pure-core-crates.txt, scripts/check-pure-core-deps.sh]

Repo-native verification is `bash scripts/verify.sh`, and new Rust files under first-party `src` or `tests` paths require parity breadcrumb mappings. [VERIFIED: AGENTS.md, scripts/verify.sh, scripts/check-parity-breadcrumbs.ts]

## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| OBS-01 | Status must report daemon state, build provenance, config/datadir paths, sync, peers, mempool, wallet, service, logs, and health for running or stopped nodes. [VERIFIED: .planning/REQUIREMENTS.md] | Plan a shared `StatusSnapshot` model with explicit unavailable fields and separate live-RPC, datadir, service, log, and storage collectors. [VERIFIED: packages/open-bitcoin-rpc/src/dispatch.rs, packages/open-bitcoin-node/src/network.rs, packages/open-bitcoin-node/src/wallet.rs] |
| OBS-03 | Runtime must record bounded historical metrics for sync, peers, mempool, wallet, disk, RPC health, and restarts. [VERIFIED: .planning/REQUIREMENTS.md] | Plan typed metric samples and retention caps before Phase 16 writes data. [VERIFIED: .planning/ROADMAP.md] |
| OBS-04 | Runtime must write structured logs with rotation, retention, and status-visible paths. [VERIFIED: .planning/REQUIREMENTS.md] | Use `tracing`, `tracing-subscriber`, and `tracing-appender`; add an explicit retention contract because `tracing-appender` documents rolling but not pruning. [CITED: docs.rs/tracing-subscriber, docs.rs/tracing-appender] |
| CLI-03 | First-party CLI commands must use clap while preserving baseline-compatible `bitcoin-cli` RPC invocation behavior. [VERIFIED: .planning/REQUIREMENTS.md] | Add a first-party `open-bitcoin` clap command tree and keep the current `open-bitcoin-cli` compatibility path stable; if a single binary is required later, route external subcommands carefully. [VERIFIED: packages/open-bitcoin-cli/src/args.rs, packages/open-bitcoin-cli/Cargo.toml; CITED: docs.rs/clap] |
| CLI-05 | Open Bitcoin-only wizard/dashboard/service/migration answers belong in JSONC without breaking `bitcoin.conf`. [VERIFIED: .planning/REQUIREMENTS.md] | Add an `OpenBitcoinConfig` JSONC model separate from the existing baseline `bitcoin.conf` parser. [VERIFIED: packages/open-bitcoin-rpc/src/config/loader.rs; CITED: docs.rs/jsonc-parser] |
| CLI-06 | Config precedence across CLI, env, JSONC, `bitcoin.conf`, cookies, and defaults must be documented and tested. [VERIFIED: .planning/REQUIREMENTS.md] | Extend current CLI-over-`bitcoin.conf` precedence tests to cover env and JSONC layers. [VERIFIED: packages/open-bitcoin-rpc/src/config/tests.rs] |
| DB-01 | Contributors must be able to inspect a database decision comparing Rust-native and RocksDB-style options against storage and verification constraints. [VERIFIED: .planning/REQUIREMENTS.md] | Write an ADR selecting a Rust-native LSM default (`fjall`) unless Phase 13 evidence rejects it; compare `redb` and `rocksdb` explicitly. [CITED: docs.rs/fjall, docs.rs/redb, docs.rs/rocksdb; ASSUMED] |

## Summary

Phase 13 should be planned as a contract-and-decision phase, not as a dashboard or full storage implementation phase. [VERIFIED: .planning/ROADMAP.md] The planner should create durable interfaces and docs for status snapshots, metric/log retention, config precedence, clap command routing, build provenance, and the storage decision so Phases 14 through 19 can consume stable models instead of inventing local variants. [VERIFIED: .planning/ROADMAP.md, .planning/research/ARCHITECTURE.md]

The highest-risk decision is storage. [VERIFIED: .planning/STATE.md] The best default for the decision record is `fjall` as the Rust-native LSM option because it offers a RocksDB-like storage shape without the native C++ dependency, but this is MEDIUM confidence until a small repo-specific write/read/restart spike is run. [CITED: docs.rs/fjall, docs.rs/rocksdb; ASSUMED]

The CLI should not force the existing `open-bitcoin-cli` compatibility parser through a broad clap rewrite in one step. [VERIFIED: packages/open-bitcoin-cli/src/args.rs] The safer plan is a first-party `open-bitcoin` clap command tree for `status`, `config`, `service`, `dashboard`, and `onboard`, while keeping `open-bitcoin-cli` as the baseline-shaped RPC client. [VERIFIED: packages/open-bitcoin-cli/Cargo.toml, packages/open-bitcoin-cli/BUILD.bazel; ASSUMED]

**Primary recommendation:** Define shared typed contracts first: `OpenBitcoinStatusSnapshot`, `OpenBitcoinConfig`, metric/log retention policies, storage ADR, and CLI command routing docs/tests before any later phase renders status or persists sync state. [VERIFIED: .planning/ROADMAP.md, .planning/research/PITFALLS.md]

## Project Constraints (from AGENTS.md)

- Prefer root `AGENTS.md`; this repo has one and it points to `AGENTS.bright-builds.md` plus `standards-overrides.md`. [VERIFIED: AGENTS.md, AGENTS.bright-builds.md, standards-overrides.md]
- Use Bitcoin Knots `29.3.knots20260210` as the behavioral baseline for in-scope surfaces. [VERIFIED: AGENTS.md]
- Keep functional core / imperative shell boundaries; storage, filesystem, network, process, logging, service, and terminal effects stay out of pure core crates. [VERIFIED: AGENTS.md, scripts/check-pure-core-deps.sh; CITED: Bright Builds architecture standard]
- Do not use existing Rust Bitcoin libraries in the production path. [VERIFIED: AGENTS.md]
- Use Rust 1.94.1 from `rust-toolchain.toml`, Bazel/Bzlmod from `MODULE.bazel`, and Bun for repo-owned higher-level automation scripts. [VERIFIED: rust-toolchain.toml, MODULE.bazel, AGENTS.md, .bun-version]
- Use `bash scripts/verify.sh` as the repo-native verification contract. [VERIFIED: AGENTS.md, scripts/verify.sh, .github/workflows/ci.yml]
- Add parity breadcrumb mappings for new first-party Rust source/test files. [VERIFIED: AGENTS.md, docs/parity/source-breadcrumbs.json, scripts/check-parity-breadcrumbs.ts]
- Update parity docs for intentional behavior differences and check README freshness after substantial operator-surface changes. [VERIFIED: AGENTS.md, docs/parity/index.json, README.md]
- Follow Bright Builds code-shape rules: early returns, `maybe_` names for optional internals, newtypes/enums for invariants, focused tests with Arrange/Act/Assert. [CITED: Bright Builds code-shape, testing, and Rust standards]
- Project skills directories `.claude/skills/` and `.agents/skills/` were not present. [VERIFIED: `find .claude/skills .agents/skills ...`]

## Standard Stack

### Core

| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| `clap` | 4.6.1, updated 2026-04-15. [VERIFIED: crates.io API, packages/open-bitcoin-cli/Cargo.toml] | First-party command tree, help, global flags, typed args, and subcommands. [CITED: docs.rs/clap] | Existing CLI crate already depends on it, but current source does not use clap parser APIs yet. [VERIFIED: packages/open-bitcoin-cli/Cargo.toml, `rg clap::Parser`] |
| `serde` | 1.0.228, updated 2025-09-27. [VERIFIED: crates.io API] | Stable status/config/metrics/log JSON data contracts. [VERIFIED: packages/open-bitcoin-rpc/src/method.rs] | Existing RPC/CLI response types already use serde derives. [VERIFIED: packages/open-bitcoin-rpc/src/method.rs, packages/open-bitcoin-cli/src/getinfo.rs] |
| `serde_json` | 1.0.149, updated 2026-01-06. [VERIFIED: crates.io API] | JSON output and test fixtures for status/config contracts. [VERIFIED: packages/open-bitcoin-rpc/Cargo.toml] | Existing RPC and CLI result paths already use serde_json. [VERIFIED: packages/open-bitcoin-rpc/src/dispatch.rs, packages/open-bitcoin-cli/src/client.rs] |
| `tokio` | 1.52.1, updated 2026-04-16. [VERIFIED: crates.io API, packages/open-bitcoin-rpc/Cargo.toml] | Async RPC daemon and later long-running sync/service runtime shell. [VERIFIED: packages/open-bitcoin-rpc/Cargo.toml] | Existing RPC daemon already depends on Tokio. [VERIFIED: packages/open-bitcoin-rpc/Cargo.toml] |
| `tracing` | 0.1.44, updated 2025-12-18. [VERIFIED: crates.io API] | Structured event emission from runtime adapters. [CITED: docs.rs/tracing-subscriber] | Libraries should emit tracing events and binaries should install subscribers. [CITED: docs.rs/tracing-subscriber] |
| `tracing-subscriber` | 0.3.23, updated 2026-03-13. [VERIFIED: crates.io API] | Runtime subscriber, filters, and JSON line formatting. [CITED: docs.rs/tracing-subscriber] | The docs identify JSON formatting for production structured logs and EnvFilter support. [CITED: docs.rs/tracing-subscriber] |
| `tracing-appender` | 0.2.5, updated 2026-04-17. [VERIFIED: crates.io API] | Non-blocking rolling file writer. [CITED: docs.rs/tracing-appender] | The rolling module creates files at fixed rotation periods, while the non-blocking writer has explicit flush-guard behavior. [CITED: docs.rs/tracing-appender] |
| `jsonc-parser` | 0.32.3, updated 2026-04-06. [VERIFIED: crates.io API] | User-editable Open Bitcoin JSONC config parsing. [CITED: docs.rs/jsonc-parser] | It supports comments/extensions and a `serde` feature for deserializing JSONC into typed config. [CITED: docs.rs/jsonc-parser] |
| `fjall` | 3.1.4, updated 2026-04-14. [VERIFIED: crates.io API] | Preferred database decision target for durable key-value storage. [CITED: docs.rs/fjall; ASSUMED] | It is a Rust LSM store with keyspaces, range/prefix iteration, cross-keyspace atomic semantics, compression, and explicit persist modes. [CITED: docs.rs/fjall] |

### Supporting

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| `redb` | 4.1.0, updated 2026-04-19. [VERIFIED: crates.io API] | Strong Rust-native ACID alternative for storage ADR. [CITED: docs.rs/redb] | Use as the strongest fallback if B-tree/MVCC simplicity and crash safety are higher priority than LSM write profile. [CITED: docs.rs/redb; ASSUMED] |
| `rocksdb` | 0.24.0, updated 2025-08-10. [VERIFIED: crates.io API] | Mature RocksDB-style comparison point. [CITED: docs.rs/rocksdb] | Use only if Rust-native options fail v1.1 storage/recovery benchmarks and the project accepts native build cost. [CITED: docs.rs/rocksdb, docs.rs/librocksdb-sys; ASSUMED] |
| `service-manager` | 0.11.0, updated 2026-02-18. [VERIFIED: crates.io API] | Phase 18 service lifecycle abstraction. [CITED: docs.rs/service-manager] | Do not implement in Phase 13, but define service-state fields to match launchd/systemd concepts. [VERIFIED: .planning/ROADMAP.md; CITED: docs.rs/service-manager] |
| `ratatui` | 0.30.0, updated 2025-12-26. [VERIFIED: crates.io API] | Phase 19 terminal dashboard consumer. [VERIFIED: .planning/research/STACK.md] | Do not add in Phase 13 unless compiling dashboard-facing model tests requires it. [VERIFIED: .planning/ROADMAP.md] |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| `fjall` | `redb` | `redb` has simple pure-Rust ACID/MVCC and crash-safe docs, but its B-tree design is a different write profile than RocksDB-style LSM storage. [CITED: docs.rs/redb, docs.rs/fjall; ASSUMED] |
| `fjall` | `rocksdb` | `rocksdb` brings mature LSM features and transactions, but the Rust crate depends on `librocksdb-sys`, which is native binding territory and heavier for Bazel/Bzlmod. [CITED: docs.rs/rocksdb, docs.rs/librocksdb-sys; VERIFIED: MODULE.bazel] |
| `jsonc-parser` | Hand-strip comments then parse JSON | Hand-stripping comments risks edge cases around strings, comments, trailing commas, and diagnostics; `jsonc-parser` owns JSONC parsing and serde conversion. [CITED: docs.rs/jsonc-parser; ASSUMED] |
| New `open-bitcoin` binary | Fold all subcommands into `open-bitcoin-cli` | A single binary can work, but it risks changing baseline-shaped RPC parsing unless compatibility argv is isolated and regression-tested. [VERIFIED: packages/open-bitcoin-cli/src/args.rs; ASSUMED] |

**Installation notes:** Add only dependencies needed for Phase 13 contracts and tests; do not add dashboard/service/storage implementation crates before their decisions are documented. [VERIFIED: .planning/ROADMAP.md; ASSUMED]

```bash
cargo add --manifest-path packages/open-bitcoin-node/Cargo.toml serde@1.0.228 --features derive
cargo add --manifest-path packages/open-bitcoin-node/Cargo.toml tracing@0.1.44
cargo add --manifest-path packages/open-bitcoin-rpc/Cargo.toml tracing-subscriber@0.3.23 --features env-filter,json
cargo add --manifest-path packages/open-bitcoin-rpc/Cargo.toml tracing-appender@0.2.5
cargo add --manifest-path packages/open-bitcoin-rpc/Cargo.toml jsonc-parser@0.32.3 --features serde
```

These commands are illustrative; the planner should update Cargo and Bazel deps in the repo's existing style and run `bash scripts/verify.sh`. [VERIFIED: packages/*/Cargo.toml, packages/*/BUILD.bazel, scripts/verify.sh]

## Architecture Patterns

### Recommended Project Structure

```text
packages/open-bitcoin-node/src/
  status.rs             # pure status snapshot structs and availability markers
  metrics.rs            # metric sample and retention contracts
  logging.rs            # log event/path/retention contracts, no subscriber install
  storage.rs            # storage traits, schema/recovery errors, DB ADR anchors
  config.rs             # Open Bitcoin-only config model if kept below CLI/RPC shell

packages/open-bitcoin-cli/src/
  operator.rs           # clap first-party open-bitcoin command tree
  status.rs             # status command rendering over shared snapshot
  config.rs             # JSONC config command helpers
  compat.rs             # thin wrapper around existing args/client compatibility path

docs/architecture/
  storage-decision.md   # DB-01 ADR comparing fjall, redb, rocksdb
  operator-runtime.md   # status, config, metrics, logs, provenance contracts
```

This structure keeps pure domain crates unchanged and places operator runtime contracts in shell crates/docs. [VERIFIED: scripts/pure-core-crates.txt, packages/open-bitcoin-node/src/lib.rs, packages/open-bitcoin-cli/src/lib.rs]

### Pattern 1: Shared Status Snapshot Before Renderers

**What:** Define one serializable `OpenBitcoinStatusSnapshot` consumed by CLI status, JSON status, service diagnostics, dashboard panels, and later RPC status. [VERIFIED: .planning/ROADMAP.md]

**When to use:** Use for all operator-facing status paths, including stopped-node inspection where live RPC fields are unavailable. [VERIFIED: .planning/REQUIREMENTS.md]

**Example:**

```rust
// Source: existing node/RPC projections plus Bright Builds provenance rule.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct OpenBitcoinStatusSnapshot {
    pub node: NodeStatus,
    pub config: ConfigStatus,
    pub service: ServiceStatus,
    pub sync: SyncStatus,
    pub peers: PeerStatus,
    pub mempool: MempoolStatus,
    pub wallet: WalletStatus,
    pub logs: LogStatus,
    pub metrics: MetricsStatus,
    pub build: BuildProvenance,
}
```

The snapshot should use explicit `Unavailable`/`Unknown` variants instead of omitting fields when the daemon is stopped. [CITED: Bright Builds operability standard; VERIFIED: .planning/REQUIREMENTS.md]

### Pattern 2: Two CLI Surfaces With One Compatibility Path

**What:** Add a clap-driven `open-bitcoin` operator command tree and keep the current `open-bitcoin-cli` RPC client parser stable. [VERIFIED: packages/open-bitcoin-cli/src/args.rs, packages/open-bitcoin-cli/BUILD.bazel; ASSUMED]

**When to use:** Use `open-bitcoin status`, `open-bitcoin config`, `open-bitcoin service`, `open-bitcoin dashboard`, and `open-bitcoin onboard` for Open Bitcoin-only commands; use `open-bitcoin-cli` for baseline-compatible RPC invocation. [VERIFIED: .planning/REQUIREMENTS.md; ASSUMED]

**Fallback:** If one binary must dispatch both surfaces, use `clap::Command::allow_external_subcommands(true)` only as a raw argv capture and manually route unknown external subcommands to the existing compatibility parser because clap docs warn that unexpected args become potential subcommands. [CITED: docs.rs/clap; VERIFIED: packages/open-bitcoin-cli/src/args.rs]

### Pattern 3: Config Layering Without `bitcoin.conf` Pollution

**What:** Keep `bitcoin.conf` as the baseline-compatible source for Bitcoin/Knots-style options and add JSONC only for Open Bitcoin-owned wizard, dashboard, service, migration, metrics, logging, storage, and sync settings. [VERIFIED: .planning/REQUIREMENTS.md, packages/open-bitcoin-rpc/src/config/loader.rs]

**When to use:** Use JSONC for settings that Core/Knots would not understand or that would risk migration compatibility. [VERIFIED: .planning/REQUIREMENTS.md]

**Recommended precedence:** CLI flags, environment, Open Bitcoin JSONC, `bitcoin.conf`, cookie files, defaults. [VERIFIED: .planning/REQUIREMENTS.md; ASSUMED]

### Pattern 4: Storage Decision Before Storage Adapter

**What:** Write `docs/architecture/storage-decision.md` before adding durable chainstate/header/block-index/wallet implementations. [VERIFIED: .planning/ROADMAP.md]

**When to use:** Use the ADR to lock the storage engine, schema versioning approach, crash/restart behavior, reindex/repair flow, Bazel dependency cost, and benchmark gate. [VERIFIED: .planning/REQUIREMENTS.md, .planning/research/PITFALLS.md]

### Anti-Patterns to Avoid

- **Dashboard/status renderers owning state:** This duplicates runtime truth and violates Phase 13's contract-first purpose. [VERIFIED: .planning/research/PITFALLS.md]
- **Putting Open Bitcoin-only config into `bitcoin.conf`:** This risks breaking Core/Knots compatibility. [VERIFIED: .planning/REQUIREMENTS.md]
- **Choosing RocksDB because Bitcoin Core uses LevelDB:** The baseline uses LevelDB-style storage, but this project has a minimal-dependency Rust/Bazel constraint and must justify native dependencies explicitly. [VERIFIED: packages/bitcoin-knots/src/dbwrapper.h, AGENTS.md, MODULE.bazel; CITED: docs.rs/rocksdb]
- **Letting live RPC be the only status source:** OBS-01 requires status against running or stopped nodes. [VERIFIED: .planning/REQUIREMENTS.md]
- **Relying on log rotation as retention:** `tracing-appender` documents rolling files, not deletion/pruning policy. [CITED: docs.rs/tracing-appender]

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| First-party subcommand parsing | Custom string parser for `status`, `config`, `service`, and `dashboard`. [ASSUMED] | `clap` for first-party commands. [VERIFIED: packages/open-bitcoin-cli/Cargo.toml; CITED: docs.rs/clap] | CLI-03 requires clap for the main command tree. [VERIFIED: .planning/REQUIREMENTS.md] |
| Baseline RPC compatibility | A new clap-only parser that changes existing `bitcoin-cli` flag behavior. [ASSUMED] | Existing `parse_cli_args` compatibility parser behind a routed path. [VERIFIED: packages/open-bitcoin-cli/src/args.rs] | Existing tests cover baseline-shaped parsing and deferred surfaces. [VERIFIED: packages/open-bitcoin-cli/tests/operator_flows.rs] |
| JSONC parsing | Regex/comment stripping before JSON parse. [ASSUMED] | `jsonc-parser` with `serde` feature. [CITED: docs.rs/jsonc-parser] | JSONC comments, trailing commas, and diagnostics are parser concerns. [CITED: docs.rs/jsonc-parser] |
| Structured log serialization | Manual JSON string formatting. [ASSUMED] | `tracing` plus `tracing-subscriber` JSON formatter. [CITED: docs.rs/tracing-subscriber] | Structured fields and formatter behavior belong to the tracing stack. [CITED: docs.rs/tracing-subscriber] |
| Rolling file writer | Custom timestamped file writer. [ASSUMED] | `tracing-appender::rolling`. [CITED: docs.rs/tracing-appender] | It already implements fixed-frequency log rollover. [CITED: docs.rs/tracing-appender] |
| Embedded database engine | Flat files and ad hoc indexes for chainstate/wallet/metrics. [ASSUMED] | `fjall` decision target, with `redb`/`rocksdb` compared in ADR. [CITED: docs.rs/fjall, docs.rs/redb, docs.rs/rocksdb] | DB-01 requires a deliberate decision against storage, recovery, Bazel, and dependency constraints. [VERIFIED: .planning/REQUIREMENTS.md] |
| Service lifecycle manager | Shell snippets for launchd/systemd. [ASSUMED] | `service-manager` in Phase 18. [CITED: docs.rs/service-manager] | It models systemd and launchd and exposes status/install contexts. [CITED: docs.rs/service-manager] |

**Key insight:** Hand-roll only Open Bitcoin's domain contracts and policy decisions; use maintained crates for parsers, logging, terminal UI, service manager integration, and database engines. [VERIFIED: AGENTS.md; CITED: docs.rs/clap, docs.rs/jsonc-parser, docs.rs/tracing-subscriber]

## Common Pitfalls

### Pitfall 1: Storage Choice Without Recovery Semantics

**What goes wrong:** A database crate lands without schema-version, corruption, restart, interrupted-write, reindex, and repair behavior. [VERIFIED: .planning/research/PITFALLS.md]

**Why it happens:** Storage benchmarks are mistaken for recovery design. [VERIFIED: .planning/research/PITFALLS.md]

**How to avoid:** DB-01 ADR must include schema version records, typed recovery errors, flush/persist policy, backup/reindex guidance, and Phase 14 test obligations. [VERIFIED: .planning/REQUIREMENTS.md]

**Warning signs:** Happy-path round-trip tests are the only storage tests. [VERIFIED: .planning/research/PITFALLS.md]

### Pitfall 2: Clap Rewrite Breaks Baseline RPC Invocation

**What goes wrong:** New first-party subcommands accidentally reinterpret `-named`, `-stdin`, `-getinfo`, RPC method names, or positional JSON parameters. [VERIFIED: packages/open-bitcoin-cli/src/args.rs, packages/open-bitcoin-cli/tests/operator_flows.rs]

**Why it happens:** Baseline `bitcoin-cli` syntax is not a normal subcommand tree. [VERIFIED: packages/open-bitcoin-cli/src/args.rs]

**How to avoid:** Add first-party clap routing beside the compatibility parser and keep Phase 8 operator-flow tests. [VERIFIED: packages/open-bitcoin-cli/tests/operator_flows.rs]

**Warning signs:** A plan deletes `args.rs` or rewrites compatibility parsing before adding regression tests for existing CLI behavior. [VERIFIED: packages/open-bitcoin-cli/src/args.rs; ASSUMED]

### Pitfall 3: Status Only Works When RPC Is Alive

**What goes wrong:** `open-bitcoin status` cannot report stopped daemon state, config paths, service state, log paths, or build provenance. [VERIFIED: .planning/REQUIREMENTS.md]

**Why it happens:** The status model is built as a thin `getinfo` wrapper instead of an operator snapshot. [VERIFIED: packages/open-bitcoin-cli/src/getinfo.rs]

**How to avoid:** Model live and stopped-node data sources separately and surface `Unavailable` for missing live fields. [CITED: Bright Builds operability standard; VERIFIED: .planning/REQUIREMENTS.md]

**Warning signs:** Status tests require a running RPC server for every field. [VERIFIED: packages/open-bitcoin-cli/tests/operator_flows.rs; ASSUMED]

### Pitfall 4: Log Rotation Without Retention

**What goes wrong:** Logs rotate forever and status shows a path but not bounded storage behavior. [CITED: docs.rs/tracing-appender; VERIFIED: .planning/REQUIREMENTS.md]

**Why it happens:** Rolling file creation is confused with retention enforcement. [CITED: docs.rs/tracing-appender]

**How to avoid:** Define retention separately as max age, max files, and max bytes, then implement a tested adapter in Phase 16. [ASSUMED]

**Warning signs:** The log contract mentions daily/hourly files but has no deletion/pruning rule. [CITED: docs.rs/tracing-appender; ASSUMED]

### Pitfall 5: JSONC Becomes a Replacement for `bitcoin.conf`

**What goes wrong:** Operators cannot safely use existing Core/Knots config files because Open Bitcoin settings contaminate the baseline config surface. [VERIFIED: .planning/REQUIREMENTS.md]

**Why it happens:** Convenience pushes all settings into one file. [ASSUMED]

**How to avoid:** Document config ownership, keep JSONC Open Bitcoin-only, and test precedence across both file formats. [VERIFIED: .planning/REQUIREMENTS.md, packages/open-bitcoin-rpc/src/config/tests.rs]

**Warning signs:** Tests write `dashboard`, `service`, or `migration` keys into `bitcoin.conf`. [VERIFIED: .planning/REQUIREMENTS.md; ASSUMED]

## Code Examples

### Status Snapshot Contract

```rust
// Source: Phase 13 requirements plus existing node/RPC projections.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FieldAvailability<T> {
    Available(T),
    Unavailable { reason: String },
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct BuildProvenance {
    pub version: String,
    pub commit: FieldAvailability<String>,
    pub build_time: FieldAvailability<String>,
    pub target: FieldAvailability<String>,
}
```

Use explicit availability rather than omitted fields so human and JSON renderers stay stable. [VERIFIED: .planning/REQUIREMENTS.md; CITED: Bright Builds operability standard]

### Clap Operator Routing

```rust
// Source: clap docs for subcommands/external subcommands plus existing compat parser.
#[derive(clap::Parser, Debug)]
pub struct OperatorCli {
    #[command(subcommand)]
    pub command: OperatorCommand,
}

#[derive(clap::Subcommand, Debug)]
pub enum OperatorCommand {
    Status(StatusArgs),
    Config(ConfigArgs),
    Service(ServiceArgs),
    Dashboard(DashboardArgs),
    Onboard(OnboardArgs),
}

pub enum CliRoute {
    Operator(OperatorCli),
    BitcoinCliCompat(Vec<std::ffi::OsString>),
}
```

Route `open-bitcoin-cli` through the existing compatibility parser and route `open-bitcoin` through clap. [VERIFIED: packages/open-bitcoin-cli/src/args.rs, packages/open-bitcoin-cli/BUILD.bazel; ASSUMED]

### JSONC Config Parse Boundary

```rust
// Source: jsonc-parser serde docs.
#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OpenBitcoinConfig {
    pub metrics: MetricsConfig,
    pub logs: LogConfig,
}

pub fn parse_open_bitcoin_config(text: &str) -> Result<OpenBitcoinConfig, ConfigError> {
    let value: OpenBitcoinConfig =
        jsonc_parser::parse_to_serde_value(text, &Default::default())?;
    Ok(value)
}
```

Parse JSONC into typed config at the boundary and reject unknown fields unless a forward-compatibility rule is documented. [CITED: docs.rs/jsonc-parser; VERIFIED: Bright Builds architecture standard]

### Structured Logging Setup

```rust
// Source: tracing-subscriber fmt docs and tracing-appender rolling/non-blocking docs.
let appender = tracing_appender::rolling::daily(log_dir, "open-bitcoin.log");
let (writer, guard) = tracing_appender::non_blocking(appender);

let subscriber = tracing_subscriber::fmt()
    .json()
    .with_writer(writer)
    .finish();
```

Keep the `WorkerGuard` alive for process lifetime because the docs state it flushes remaining logs on drop. [CITED: docs.rs/tracing-appender]

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| `service-manager` 0.9.x from milestone synthesis. [VERIFIED: .planning/research/STACK.md] | `service-manager` 0.11.0 is current. [VERIFIED: crates.io API] | Updated by registry before 2026-04-26 research. [VERIFIED: crates.io API] | Phase 18 docs/plans should use 0.11.0 APIs. [CITED: docs.rs/service-manager] |
| `tracing-appender` 0.2.4 from milestone synthesis. [VERIFIED: .planning/research/STACK.md] | `tracing-appender` 0.2.5 is current. [VERIFIED: crates.io API] | Updated 2026-04-17. [VERIFIED: crates.io API] | Log contracts should cite 0.2.5 docs. [CITED: docs.rs/tracing-appender] |
| Single `-getinfo` snapshot over live RPC. [VERIFIED: packages/open-bitcoin-cli/src/getinfo.rs] | Full operator status model spanning live and stopped nodes. [VERIFIED: .planning/REQUIREMENTS.md] | Required by v1.1 Phase 13 and Phase 17. [VERIFIED: .planning/ROADMAP.md] | `GetInfoSnapshot` is not enough for OBS-01. [VERIFIED: packages/open-bitcoin-cli/src/getinfo.rs, .planning/REQUIREMENTS.md] |
| In-memory chainstate/wallet stores. [VERIFIED: packages/open-bitcoin-node/src/chainstate.rs, packages/open-bitcoin-node/src/wallet.rs] | Durable adapter-backed storage after DB decision. [VERIFIED: .planning/ROADMAP.md] | Phase 14 depends on Phase 13. [VERIFIED: .planning/ROADMAP.md] | DB-01 must precede DB-02/03/04/05 implementation. [VERIFIED: .planning/ROADMAP.md] |

**Deprecated/outdated:**
- Treat `sled` as non-standard for this phase because crates.io reports `1.0.0-alpha.124` and the project needs a conservative storage decision. [VERIFIED: crates.io API; ASSUMED]
- Treat RocksDB as a comparison candidate, not the default, because the Rust wrapper depends on `librocksdb-sys` and this repo emphasizes minimal, security-conscious dependencies plus Bazel/Bzlmod. [CITED: docs.rs/rocksdb, docs.rs/librocksdb-sys; VERIFIED: AGENTS.md, MODULE.bazel]

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | Add a new `open-bitcoin` operator binary while keeping `open-bitcoin-cli` for RPC compatibility. | Summary, Architecture Patterns | If the project wants one binary, the clap routing plan must use external-subcommand/raw argv capture instead. |
| A2 | Select `fjall` as the default storage decision target unless Phase 13 spike evidence rejects it. | Standard Stack, Architecture Patterns | If `redb` is better for recovery simplicity or `fjall` underperforms, Phase 14 would start on the wrong adapter. |
| A3 | Store Open Bitcoin-only JSONC in a separate file under the selected Open Bitcoin datadir, likely `open-bitcoin.jsonc`. | Architecture Patterns | Wrong path choice can create migration confusion or duplicate config discovery work. |
| A4 | Config precedence should be CLI, environment, Open Bitcoin JSONC, `bitcoin.conf`, cookie files, defaults. | Architecture Patterns | Wrong precedence can break operator expectations and tests. |
| A5 | Default retention should be documented as bounded by age, file count, and byte size, with concrete defaults chosen in Phase 13. | Common Pitfalls | Without user-approved defaults, later dashboard/status work may depend on unstable history windows. |
| A6 | Build provenance should be injected through compile-time or release-time environment variables when git metadata is unavailable. | Code Examples | Missing build metadata could show `Unavailable` more often than desired. |

## Open Questions

1. **Should Phase 13 lock `fjall` immediately or require a small DB spike first?** [ASSUMED]
   - What we know: `fjall` is current, Rust-native, LSM-based, and has keyspaces plus persist modes. [CITED: docs.rs/fjall; VERIFIED: crates.io API]
   - What's unclear: The repo has no Open Bitcoin-specific DB workload benchmark yet. [VERIFIED: docs/parity/benchmarks.md]
   - Recommendation: Plan an ADR task with a tiny write/read/restart spike before adding Phase 14 production storage. [ASSUMED]

2. **Should the operator command be a new binary or a renamed/expanded current binary?** [ASSUMED]
   - What we know: Current Cargo/Bazel surfaces expose `open-bitcoin-cli`; requirements name `open-bitcoin status`. [VERIFIED: packages/open-bitcoin-cli/Cargo.toml, packages/open-bitcoin-cli/BUILD.bazel, .planning/REQUIREMENTS.md]
   - What's unclear: The project has not locked the binary naming contract for v1.1. [VERIFIED: .planning/ROADMAP.md]
   - Recommendation: Add `open-bitcoin` for first-party commands and keep `open-bitcoin-cli` stable. [ASSUMED]

3. **What are the exact log and metric retention defaults?** [ASSUMED]
   - What we know: OBS-03 and OBS-04 require bounded metrics and log retention. [VERIFIED: .planning/REQUIREMENTS.md]
   - What's unclear: The requirements do not specify days, samples, files, or byte caps. [VERIFIED: .planning/REQUIREMENTS.md]
   - Recommendation: Phase 13 should document defaults and make them config-driven. [ASSUMED]

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|-------------|-----------|---------|----------|
| Rust/Cargo | Rust contracts and tests | yes | cargo 1.94.1, rustc 1.94.1. [VERIFIED: command output] | none |
| Bun | Repo automation scripts | yes | 1.3.9. [VERIFIED: command output, .bun-version] | none |
| Bazel/Bazelisk command | Repo Bazel smoke build | yes | bazel 8.6.0. [VERIFIED: command output] | none |
| cargo-llvm-cov | `scripts/verify.sh` coverage gate | yes | 0.8.5. [VERIFIED: command output] | none |
| Git | GSD commit and breadcrumb tooling | yes | 2.53.0. [VERIFIED: command output] | none |
| clang | Native dependency comparison, especially RocksDB fallback | yes | Apple clang 21.0.0. [VERIFIED: command output] | Prefer Rust-native DB if native build cost is unacceptable. [ASSUMED] |
| CMake | Native dependency comparison, especially RocksDB fallback | yes | 3.27.9. [VERIFIED: command output] | Prefer Rust-native DB if native build cost is unacceptable. [ASSUMED] |
| jq | Registry/API inspection during planning | yes | jq 1.7.1. [VERIFIED: command output] | Use Bun/serde_json scripts. [ASSUMED] |

**Missing dependencies with no fallback:** None found for Phase 13 research/planning. [VERIFIED: command output]

**Missing dependencies with fallback:** None found for Phase 13 research/planning. [VERIFIED: command output]

## Security Domain

OWASP ASVS latest stable release is v5.0.0 according to the official OWASP/ASVS release source found during research. [CITED: github.com/OWASP/ASVS/releases]

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|------------------|
| V2 Authentication | yes | Preserve RPC cookie/user-password parsing and avoid logging secrets. [VERIFIED: packages/open-bitcoin-rpc/src/config/loader.rs, packages/open-bitcoin-cli/src/client.rs] |
| V3 Session Management | no for browser sessions | No browser session layer exists in Phase 13; status/dashboard is local CLI/TUI scope. [VERIFIED: .planning/ROADMAP.md] |
| V4 Access Control | partial | Service and migration commands must gate destructive or privileged actions later; Phase 13 should model dry-run/status states. [VERIFIED: .planning/ROADMAP.md] |
| V5 Input Validation | yes | Parse CLI, JSONC, `bitcoin.conf`, env, and status JSON into typed domain structs with unknown-field policy. [VERIFIED: packages/open-bitcoin-rpc/src/config/loader.rs; CITED: Bright Builds architecture standard] |
| V6 Cryptography | yes | Do not hand-roll wallet/RPC cryptography; preserve existing first-party crypto boundaries and avoid new crypto in config/logging. [VERIFIED: AGENTS.md, packages/open-bitcoin-consensus/src/crypto.rs] |

### Known Threat Patterns for This Stack

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Secret leakage in logs/status | Information Disclosure | Redact RPC password/cookie contents and expose only credential source/path metadata. [VERIFIED: packages/open-bitcoin-rpc/src/config/loader.rs; ASSUMED] |
| Config precedence confusion | Tampering | Document and test deterministic precedence across CLI/env/JSONC/`bitcoin.conf`/cookies/defaults. [VERIFIED: .planning/REQUIREMENTS.md] |
| Malformed JSONC or `bitcoin.conf` accepted loosely | Tampering | Parse at boundaries into typed structs and reject unknown/invalid fields where compatibility allows. [VERIFIED: packages/open-bitcoin-rpc/src/config/loader.rs; CITED: docs.rs/jsonc-parser, Bright Builds architecture standard] |
| Log retention unbounded | Denial of Service | Enforce max age/files/bytes and surface log paths in status. [VERIFIED: .planning/REQUIREMENTS.md; ASSUMED] |
| Storage corruption or schema mismatch panic | Denial of Service | Use typed storage/recovery errors and explicit schema-version checks. [VERIFIED: .planning/REQUIREMENTS.md] |

## Sources

### Primary (HIGH confidence)

- `.planning/REQUIREMENTS.md` - Phase requirement scope and traceability. [VERIFIED]
- `.planning/ROADMAP.md` - Phase 13 goal, dependencies, success criteria, and downstream phase ordering. [VERIFIED]
- `.planning/STATE.md` - v1.1 decisions and known blockers. [VERIFIED]
- `AGENTS.md`, `AGENTS.bright-builds.md`, `standards-overrides.md` - repo workflow, Rust, Bazel, Bun, parity, and verification constraints. [VERIFIED]
- Bright Builds standards at commit `05f8d7a6c9c2e157ec4f922a05273e72dab97676` - architecture, code shape, operability, verification, testing, Rust, and TypeScript/JavaScript guidance. [CITED: github.com/bright-builds-llc/bright-builds-rules]
- `packages/open-bitcoin-cli/src/args.rs`, `client.rs`, `getinfo.rs`, `startup.rs`, and `tests/operator_flows.rs` - existing CLI compatibility parser and operator tests. [VERIFIED]
- `packages/open-bitcoin-rpc/src/config/loader.rs`, `config/tests.rs`, `dispatch.rs`, and `method.rs` - current config precedence, RPC projections, and response models. [VERIFIED]
- `packages/open-bitcoin-node/src/chainstate.rs`, `wallet.rs`, and `network.rs` - current in-memory stores and runtime projections. [VERIFIED]
- `scripts/verify.sh`, `scripts/check-pure-core-deps.sh`, `scripts/check-parity-breadcrumbs.ts`, and `docs/parity/source-breadcrumbs.json` - verification and architecture guards. [VERIFIED]
- `packages/bitcoin-knots/src/dbwrapper.h`, `txdb.h`, and `node/blockstorage.h` - baseline LevelDB/block/undo storage concepts. [VERIFIED]
- crates.io API - current package versions and update timestamps for recommended crates. [VERIFIED]
- docs.rs pages for `clap`, `jsonc-parser`, `tracing-subscriber`, `tracing-appender`, `service-manager`, `fjall`, `redb`, `rocksdb`, and `librocksdb-sys`. [CITED]

### Secondary (MEDIUM confidence)

- `.planning/research/SUMMARY.md`, `STACK.md`, `ARCHITECTURE.md`, and `PITFALLS.md` - milestone-level synthesis that this phase refines with current version checks. [VERIFIED]
- OWASP/ASVS official releases page - ASVS v5.0.0 stable release status. [CITED: github.com/OWASP/ASVS/releases]

### Tertiary (LOW confidence)

- None used as authoritative sources. [VERIFIED]

## Metadata

**Confidence breakdown:**
- Standard stack: MEDIUM - crate versions are verified, but the DB default remains an ADR/spike decision. [VERIFIED: crates.io API; ASSUMED]
- Architecture: HIGH - repo patterns and phase ordering strongly support contract-first models. [VERIFIED: .planning/ROADMAP.md, packages/open-bitcoin-node/src/lib.rs]
- Pitfalls: HIGH - milestone research, requirements, and current code all expose the same risks. [VERIFIED: .planning/research/PITFALLS.md, packages/open-bitcoin-cli/src/args.rs, packages/open-bitcoin-node/src/chainstate.rs]
- Security: MEDIUM - ASVS category mapping is verified at category level, but specific controls need implementation-phase review. [CITED: github.com/OWASP/ASVS/releases; ASSUMED]

**Research date:** 2026-04-26  
**Valid until:** 2026-05-26 for architecture and repo constraints; re-check crate versions before dependency edits. [ASSUMED]
