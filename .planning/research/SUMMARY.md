# Project Research Summary

**Project:** Open Bitcoin
**Domain:** Bitcoin node operator runtime, service lifecycle, durable sync, and terminal dashboard
**Researched:** 2026-04-26
**Confidence:** MEDIUM

## Executive Summary

v1.1 should turn the v1.0 headless parity baseline into an operator-usable node runtime. The dependency research supports a terminal-first path: keep `clap` as the CLI foundation, add Ratatui for the dashboard, add structured rotating logs, add a deliberate JSONC config layer for Open Bitcoin-only wizard and service state, and decide the database adapter before real-network sync depends on it.

The critical order is foundations first. Status snapshots, metrics history, storage recovery, and config/migration plans must exist before a TUI, service install command, or migration wizard can be trustworthy. Real-network sync should start with deterministic and bounded test surfaces, then add optional live-network benchmarks that do not make default verification flaky.

## Key Findings

### Recommended Stack

- Ratatui: terminal dashboard and charts, using the default crossterm backend unless implementation proves otherwise.
- clap: already present in `open-bitcoin-cli`; expand from dependency to coherent command tree.
- tracing plus tracing-appender: structured logs and rolling local files.
- service-manager: candidate abstraction for launchd and systemd lifecycle commands.
- jsonc-parser: candidate parser for user-editable Open Bitcoin JSONC config.
- Embedded database: evaluate redb, fjall, and RocksDB against recovery, Bazel cost, write profile, and sync benchmarks before selecting.

### Expected Features

**Must have:**
- Rich `status` command with human and JSON output.
- Durable storage and restart recovery.
- Real peer connections and bounded sync smoke.
- Metrics and log retention.
- Idempotent first-run wizard.
- macOS and Linux service lifecycle commands.
- Core/Knots install detection and migration dry-run.

**Should have:**
- Ratatui dashboard with gentle color palette, sync graphs, metrics, logs, and menu actions.
- Drop-in audit report mapping Knots/Core expectations to Open Bitcoin behavior.
- Real-sync benchmark scenarios.

**Defer:**
- Qt/desktop GUI parity.
- Windows service integration.
- Hosted dashboards.
- Automatic destructive migration.

### Architecture Approach

Build stable pure status and plan models that are rendered by CLI, TUI, RPC, service diagnostics, and migration output. Keep node runtime effects in shell adapters. Keep database, service manager, filesystem, process, network, log, and terminal effects out of pure core modules.

### Critical Pitfalls

1. **Dashboard before truth:** Build status and metrics contracts before Ratatui rendering.
2. **Unsafe migration:** Split detection, dry-run planning, backup, and apply into separate steps.
3. **Weak storage recovery:** Define schema, recovery, reindex, and crash behavior before live sync.
4. **Flaky live sync verification:** Keep default verification hermetic; make live-network sync benchmarks opt-in.
5. **Opaque service install:** Require dry-run and status introspection for generated service files.

## Implications for Roadmap

### Phase 13: Operator Runtime Foundations

**Rationale:** Defines command, config, status, metrics, log, storage, and dependency contracts before user-facing surfaces depend on them.

### Phase 14: Durable Storage and Recovery

**Rationale:** Real sync and wallet runtime work need persistent state that survives restart and has defined recovery behavior.

### Phase 15: Real Network Sync Loop

**Rationale:** Peer discovery, long-running sockets, headers, and block download belong after storage contracts are in place.

### Phase 16: Metrics, Logs, and Sync Telemetry

**Rationale:** Status and dashboard surfaces need historical metrics and operator logs.

### Phase 17: CLI Status and First-Run Onboarding

**Rationale:** Operators need a friendly and idempotent path to configure and inspect the node before service install or migration.

### Phase 18: Service Lifecycle Integration

**Rationale:** macOS and Linux service commands depend on config, status, and logging paths.

### Phase 19: Ratatui Dashboard

**Rationale:** The dashboard should consume existing status, metrics, logs, and service data.

### Phase 20: Wallet Runtime Expansion

**Rationale:** Durable state and real sync make richer wallet flows more meaningful and testable.

### Phase 21: Drop-In Parity Audit and Migration

**Rationale:** Migration should happen after status, storage, service, and wallet behavior are concrete enough to audit honestly.

### Phase 22: Real-Sync Benchmarks and Release Hardening

**Rationale:** The milestone should close with benchmark evidence, docs, audit updates, and verification hardening.

## Confidence Assessment

| Area | Confidence | Notes |
|------|------------|-------|
| Stack | MEDIUM | Ratatui/clap/service/log choices are clear; database choice needs implementation research. |
| Features | HIGH | User scope and v1.0 deferrals align strongly. |
| Architecture | MEDIUM | Boundaries are clear; storage and sync details require phase-level design. |
| Pitfalls | HIGH | Risks are concrete and visible from v1.0 deferrals. |

**Overall confidence:** MEDIUM

## Gaps to Address

- Database adapter choice must be made with benchmarks and recovery tests, not by preference.
- Exact Core/Knots migration compatibility must be audited against real datadir/config/wallet layouts.
- Live-network sync tests must be scoped so local verification remains deterministic.

## Sources

- `.planning/research/STACK.md`
- `.planning/research/FEATURES.md`
- `.planning/research/ARCHITECTURE.md`
- `.planning/research/PITFALLS.md`

---
*Research completed: 2026-04-26*
*Ready for roadmap: yes*
