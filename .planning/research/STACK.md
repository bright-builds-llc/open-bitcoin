# Stack Research

**Domain:** Bitcoin node operator runtime, terminal dashboard, service lifecycle, persistence, and real-network sync
**Researched:** 2026-04-26
**Confidence:** MEDIUM

## Recommended Stack

### Core Technologies

| Technology | Version | Purpose | Why Recommended |
|------------|---------|---------|-----------------|
| Ratatui | 0.30.0 | Terminal dashboard widgets, layout, charts, and input-driven UI | Official installation docs show Ratatui as the maintained terminal UI crate with `crossterm` enabled by default, which fits a portable local node dashboard. |
| clap | 4.6.1 in repo | CLI command model, help, parsing, subcommands, and structured flags | `open-bitcoin-cli` already depends on clap with `derive`; v1.1 should use it as the main command tree instead of keeping broad manual parsing. |
| tracing + tracing-appender | tracing-appender 0.2.4 documented | Structured logs and rolling file appenders | Rolling log files are a standard operator need before service integration and status reporting are credible. |
| tokio | 1.52.1 in repo | Async runtime for RPC server and long-running network sync | The RPC daemon already uses tokio; real peer sockets and service runtime should reuse the same runtime boundary. |
| service-manager | 0.9.0 documented | Cross-platform service install/enable/disable abstraction | Docs list service manager implementations for systemd and launchd, matching the v1.1 Mac/Linux service scope. |
| jsonc-parser | 0.32.x documented | JSONC config parsing for Open Bitcoin-only wizard answers | Supports comments, JSON extensions, and serde deserialization, which fits a user-editable local config layered on top of `bitcoin.conf`. |

### Supporting Libraries

| Library | Purpose | When to Use |
|---------|---------|-------------|
| redb | Embedded Rust key-value database candidate | Evaluate for simpler ACID local metadata, wallet snapshots, header indexes, and bounded metrics history. |
| fjall | Embedded Rust LSM key-value database candidate | Evaluate for write-heavy UTXO/header workloads where partitioning and ordered iteration matter. |
| rocksdb | Mature LSM database candidate with C++ dependency | Use only if Rust-native stores cannot meet sync and chainstate requirements; the dependency and Bazel costs are higher. |
| crossterm | Terminal backend for Ratatui | Use through Ratatui's default backend path unless platform behavior forces a narrower backend decision. |

### Development Tools

| Tool | Purpose | Notes |
|------|---------|-------|
| `bash scripts/verify.sh` | Repo-native verification | Must remain the aggregate local gate for v1.1 work. |
| `scripts/run-benchmarks.sh` | Benchmark smoke and report generation | Extend with real-sync benchmark scenarios instead of creating a separate benchmark entrypoint. |
| `docs/parity/index.json` | Machine-readable parity source | Add v1.1 operator, migration, sync, and storage audit surfaces here when implemented. |

## Alternatives Considered

| Recommended | Alternative | When to Use Alternative |
|-------------|-------------|-------------------------|
| Ratatui | Raw ANSI/crossterm UI | Only for small status output; not enough for graphs, panels, and keyboard navigation. |
| service-manager | Handwritten launchd/systemd files only | Use templates internally, but prefer a shared abstraction for install/enable/disable behavior and tests. |
| JSONC Open Bitcoin config | Extending `bitcoin.conf` for all wizard state | Do not store Open Bitcoin-only wizard/UI/service state in baseline config if it would break Knots/Core compatibility. |
| Rust-native embedded DB | RocksDB | Use RocksDB only after a storage decision confirms Rust-native options are insufficient. |

## What NOT to Use

| Avoid | Why | Use Instead |
|-------|-----|-------------|
| Existing Rust Bitcoin production libraries | Conflicts with the project's first-party domain ownership constraint | Continue using first-party `open-bitcoin-*` crates. |
| A decorative dashboard without status contracts | It would look useful while hiding missing runtime truth | Build status, metrics, and sync-state projections first. |
| Blind datadir migration | Existing node data and wallet files are high-value user data | Detect, explain, dry-run, backup, then require explicit approval. |

## Sources

- https://ratatui.rs/installation/ - Ratatui install path and default backend behavior.
- https://docs.rs/clap/latest/clap/_derive/ - clap derive documentation.
- https://docs.rs/crate/tracing-appender/latest/source/src/rolling.rs - rolling file appender behavior.
- https://docs.rs/service-manager - systemd and launchd service manager support.
- https://docs.rs/jsonc-parser - JSONC parsing and serde support.
- https://docs.rs/crate/rocksdb/latest - RocksDB Rust wrapper requirements and tradeoffs.
- https://fjall-rs.github.io/ - Fjall embedded key-value storage capabilities.
- https://bitcoincore.org/en/releases/0.11.0/ - Bitcoin Core block, undo, block-index, and UTXO storage model overview.

---
*Stack research for: Open Bitcoin v1.1 operator runtime*
*Researched: 2026-04-26*
