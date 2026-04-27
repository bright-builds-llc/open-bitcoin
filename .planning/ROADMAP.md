# Roadmap: Open Bitcoin

## Milestones

- **v1.0 Headless Parity** - Phases 1 through 12 shipped on 2026-04-26. Archive: [v1.0-ROADMAP.md](milestones/v1.0-ROADMAP.md)
- **v1.1 Operator Runtime and Real-Network Sync** - Phases 13 through 22 are planned for the next milestone.

## v1.1 Operator Runtime and Real-Network Sync

This active milestone makes Open Bitcoin usable as an operator-facing, service-managed node that can begin true network sync work with durable storage, richer CLI/TUI surfaces, migration guidance, and expanded parity coverage.

## Phases

<details>
<summary>v1.0 Headless Parity - shipped 2026-04-26 (22 phases, 80 plans)</summary>

- [x] Phase 1: Workspace, Baseline, and Guardrails (4/4 plans) - completed 2026-04-11
- [x] Phase 2: Core Domain and Serialization Foundations (4/4 plans) - completed 2026-04-11
- [x] Phase 3: Consensus Validation Foundation (7/7 plans) - completed 2026-04-11
- [x] Phase 3.1: Legacy Signature Execution (3/3 plans) - completed 2026-04-11
- [x] Phase 3.2: P2SH and Segwit-v0 Execution (3/3 plans) - completed 2026-04-12
- [x] Phase 3.3: Taproot and Tapscript Execution (3/3 plans) - completed 2026-04-12
- [x] Phase 3.4: Consensus Parity Closure (3/3 plans) - completed 2026-04-12
- [x] Phase 4: Chainstate and UTXO Engine (3/3 plans) - completed 2026-04-12
- [x] Phase 5: Mempool and Node Policy (3/3 plans) - completed 2026-04-13
- [x] Phase 6: P2P Networking and Sync (4/4 plans) - completed 2026-04-14
- [x] Phase 7: Wallet Core and Adapters (4/4 plans) - completed 2026-04-17
- [x] Phase 07.1: Codebase Maintainability Refactor Wave (3/3 plans) - completed 2026-04-18
- [x] Phase 07.2: Protocol Constant Clarity Cleanup (1/1 plan) - completed 2026-04-19
- [x] Phase 07.3: Reduce nesting with early returns (3/3 plans) - completed 2026-04-19
- [x] Phase 07.4: Sweep the codebase for let-else opportunities (1/1 plan) - completed 2026-04-20
- [x] Phase 07.5: Fix consensus parity gaps in contextual header validation and lax DER signature verification (4/4 plans) - completed 2026-04-22
- [x] Phase 07.6: Enforce coinbase subsidy-plus-fees limits on the consensus and active chainstate paths (3/3 plans) - completed 2026-04-22
- [x] Phase 8: RPC, CLI, and Config Parity (8/8 plans) - completed 2026-04-24
- [x] Phase 9: Parity Harnesses and Fuzzing (4/4 plans) - completed 2026-04-24
- [x] Phase 10: Benchmarks and Audit Readiness (5/5 plans) - completed 2026-04-24
- [x] Phase 11: Panic and Illegal-State Hardening (3/3 plans) - completed 2026-04-24
- [x] Phase 12: Milestone Audit Artifact Closure (4/4 plans) - completed 2026-04-26

</details>

- [x] **Phase 13: Operator Runtime Foundations** - Define the dependency, command, config, status, metrics, log, and storage contracts that later user-facing work consumes. (completed 2026-04-26)
- [x] **Phase 14: Durable Storage and Recovery** - Implement the selected durable storage layer and restart/recovery behavior for node and wallet state. (completed 2026-04-26)
- [x] **Phase 15: Real Network Sync Loop** - Connect to real peers, drive headers/block sync, persist progress, and keep default tests deterministic. (completed 2026-04-26)
- [x] **Phase 16: Metrics, Logs, and Sync Telemetry** - Record bounded metrics history, rotate logs, and expose sync/runtime telemetry to status consumers. (completed 2026-04-26)
- [ ] **Phase 17: CLI Status and First-Run Onboarding** - Expand the clap command tree, implement rich status output, and add the idempotent wizard plus JSONC config layer.
- [ ] **Phase 18: Service Lifecycle Integration** - Add macOS launchd and Linux systemd install/uninstall/enable/disable/status support with dry-run safety.
- [ ] **Phase 19: Ratatui Node Dashboard** - Build the terminal dashboard on top of the shared status, metrics, logs, service, and sync models.
- [ ] **Phase 20: Wallet Runtime Expansion** - Expand practical wallet runtime behavior for send, wallet selection, descriptors, rescans, backups, and migration inspection.
- [ ] **Phase 21: Drop-In Parity Audit and Migration** - Audit Core/Knots replacement expectations and implement detection, education, and dry-run migration plans.
- [ ] **Phase 22: Real-Sync Benchmarks and Release Hardening** - Add real-sync benchmarks, docs, parity updates, and verification coverage for the v1.1 operator runtime.

## Phase Details

### Phase 13: Operator Runtime Foundations

**Goal**: Establish the stable contracts and dependency decisions that v1.1 runtime, CLI, service, dashboard, storage, and sync work depend on.
**Depends on**: v1.0 archive
**Requirements**: OBS-01, OBS-03, OBS-04, CLI-03, CLI-05, CLI-06, DB-01
**Success Criteria** (what must be TRUE):
1. The repository has a documented database decision comparing Rust-native and RocksDB-style options against v1.1 storage and verification constraints.
2. A shared status snapshot model exists for node, config, service, sync, peers, mempool, wallet, logs, metrics, version, commit, and build provenance.
3. Metrics and log retention contracts exist before dashboard and status renderers depend on them.
4. The CLI command architecture defines how clap handles new subcommands while preserving baseline-compatible RPC invocation behavior.
5. Open Bitcoin JSONC config ownership and `bitcoin.conf` compatibility boundaries are documented and covered by initial tests.
**Plans**: 5 plans

Plans:
- [x] 13-01-PLAN.md — Storage decision ADR and adapter-facing storage contracts
- [x] 13-02-PLAN.md — Metrics and structured log retention contracts
- [x] 13-03-PLAN.md — Shared status snapshot and build provenance model
- [x] 13-04-PLAN.md — Clap operator CLI routing and compatibility boundary
- [x] 13-05-PLAN.md — Open Bitcoin JSONC config ownership and precedence contracts

### Phase 14: Durable Storage and Recovery

**Goal**: Replace in-memory runtime stores with durable adapter-backed storage that survives restart and has defined recovery behavior.
**Depends on**: Phase 13
**Requirements**: DB-02, DB-03, DB-04, DB-05
**Success Criteria** (what must be TRUE):
1. Headers, block index metadata, chainstate or UTXO state, undo/reorg metadata, wallet state, runtime metadata, and schema version information persist across process restart.
2. Storage schema mismatches and corruption conditions return typed errors and clear operator guidance instead of panics.
3. Interrupted write, replay, reindex, and repair scenarios are covered by tests using isolated temp stores.
4. Pure core crates remain free of filesystem and database dependencies.
**Plans**: 4/4 plans complete

Plans:
- [x] 14-01-PLAN.md — Fjall-backed durable storage adapter
- [x] 14-02-PLAN.md — Schema-versioned snapshot DTOs
- [x] 14-03-PLAN.md — Restart, schema, corruption, and recovery tests
- [x] 14-04-PLAN.md — Closeout, metrics, and verification artifacts

### Phase 15: Real Network Sync Loop

**Goal**: Turn the existing peer/message/header primitives into a long-running sync runtime that can talk to real network peers and resume progress.
**Depends on**: Phase 14
**Requirements**: SYNC-01, SYNC-02, SYNC-03, SYNC-04, SYNC-05
**Success Criteria** (what must be TRUE):
1. The node can establish long-running outbound peer connections using configured peers and DNS/manual seed sources for supported networks.
2. Headers sync persists progress, resumes after restart, and reports progress through the shared status model.
3. Block download, validation, persistence, and connect flow handles bounded in-flight work and observable retry behavior.
4. Peer disconnects, invalid data, timeouts, stalls, and competing branches produce typed errors, metrics, and logs.
5. Default verification remains hermetic; optional live-network smoke tests are explicitly opt-in.
**Plans**: 4/4 plans complete

Plans:
- [x] 15-01-PLAN.md — Bounded peer sync hooks and header-store runtime helpers
- [x] 15-02-PLAN.md — Durable downloaded block persistence
- [x] 15-03-PLAN.md — Real-network sync runtime with pluggable transport
- [x] 15-04-PLAN.md — Closeout, status, and verification artifacts

### Phase 16: Metrics, Logs, and Sync Telemetry

**Goal**: Give operators and later dashboard work durable, bounded, and explainable runtime evidence.
**Depends on**: Phase 15
**Requirements**: OBS-03, OBS-04, OBS-05, SYNC-06
**Success Criteria** (what must be TRUE):
1. The node records bounded historical metrics for sync height, header height, peer counts, mempool size, wallet summary, disk usage, RPC health, and service restarts.
2. Structured logs rotate with documented retention and status-visible paths.
3. Recent warnings/errors can be queried through status-facing APIs without opening raw log files manually.
4. Sync bottlenecks and health signals are visible through metrics and logs without changing consensus or network behavior.
**Plans**: 3 plans

Plans:
- [x] 16-01-PLAN.md — Bounded Fjall-backed metrics history
- [x] 16-02-PLAN.md — Structured log writing, retention, and recent signal queries
- [x] 16-03-PLAN.md — Sync telemetry through shared metrics, logs, and status contracts

### Phase 17: CLI Status and First-Run Onboarding

**Goal**: Make the command-line operator surface coherent, friendly, idempotent, and automatable.
**Depends on**: Phase 16
**Requirements**: OBS-01, OBS-02, CLI-03, CLI-04, CLI-05, CLI-06, CLI-07, MIG-02
**Success Criteria** (what must be TRUE):
1. `open-bitcoin status` works against running and stopped nodes and can emit both rich human output and stable JSON.
2. The first-run wizard asks only practical questions, can be rerun idempotently, supports non-interactive mode, and never overwrites without explicit approval.
3. Open Bitcoin JSONC config stores wizard/dashboard/service/migration state without breaking `bitcoin.conf` compatibility.
4. CLI startup detects existing Core/Knots datadirs and config files and reports them without mutating user data.
5. Config precedence is documented and tested across CLI flags, environment, JSONC config, `bitcoin.conf`, cookies, and defaults.
**Plans**: 5 plans

Plans:
- [ ] 17-01-PLAN.md — Operator module contracts and breadcrumb coverage
- [ ] 17-02-PLAN.md — Config precedence and JSONC path reporting
- [ ] 17-03-PLAN.md — Read-only Core/Knots detection
- [ ] 17-04-PLAN.md — Shared status collection and rendering
- [ ] 17-05-PLAN.md — First-run onboarding and open-bitcoin binary wiring

### Phase 18: Service Lifecycle Integration

**Goal**: Let operators manage Open Bitcoin as a supervised daemon on macOS and Linux without losing inspectability or safety.
**Depends on**: Phase 17
**Requirements**: SVC-01, SVC-02, SVC-03, SVC-04, SVC-05
**Success Criteria** (what must be TRUE):
1. macOS launchd service install/uninstall/enable/disable/status works with dry-run output for generated plist contents.
2. Linux systemd service install/uninstall/enable/disable/status works with dry-run output for generated unit contents.
3. Commands surface privilege requirements, scope, generated paths, daemon command, config path, log path, and recovery behavior before applying changes.
4. Service status identifies installed, enabled, running, failed, stopped, and unmanaged states.
5. Tests run against isolated temp paths or fake managers and never modify real developer launchd/systemd state.
**Plans**: TBD

### Phase 19: Ratatui Node Dashboard

**Goal**: Provide a useful local terminal dashboard for live node operation, sync progress, metrics, logs, wallet summary, and safe actions.
**Depends on**: Phase 18
**Requirements**: DASH-01, DASH-02, DASH-03, DASH-04, SYNC-06
**Success Criteria** (what must be TRUE):
1. Dashboard consumes the shared status, metrics, logs, service, and sync models rather than inventing separate runtime state.
2. Terminal graphs show bounded history for sync progress, peers, mempool size, disk usage, and RPC health.
3. Keyboard menu supports safe queries and gated actions with clear confirmations for destructive or service-affecting commands.
4. Palette remains restrained, readable, and usable on common light/dark terminals and with color disabled.
5. Dashboard tests verify model-to-view behavior and non-interactive rendering without depending on a real terminal session.
**Plans**: TBD

### Phase 20: Wallet Runtime Expansion

**Goal**: Expand wallet behavior into practical runtime workflows that work with durable sync state and operator-facing tools.
**Depends on**: Phase 19
**Requirements**: WAL-04, WAL-05, WAL-06, WAL-07, WAL-08
**Success Criteria** (what must be TRUE):
1. Wallet send flow supports a safe `sendtoaddress`-style path with fee limits, change handling, confirmation prompts, and deterministic errors.
2. Wallet-scoped RPC/CLI selection supports the expected `-rpcwallet` style operator surface where in scope.
3. HD or ranged descriptor behavior supports practical receive/change address management.
4. Wallet rescan, recovery, and balance tracking integrate with durable sync state and survive restart.
5. Existing Core/Knots wallet candidates can be inspected for migration planning without mutation.
**Plans**: TBD

### Phase 21: Drop-In Parity Audit and Migration

**Goal**: Make Open Bitcoin's replacement story evidence-based, explicit, and safe for users with existing Core or Knots installs.
**Depends on**: Phase 20
**Requirements**: CLI-07, WAL-08, MIG-01, MIG-02, MIG-03, MIG-04, MIG-05
**Success Criteria** (what must be TRUE):
1. The project audits Core/Knots drop-in expectations for CLI, RPC, config, datadir layout, service behavior, wallet behavior, sync, logs, and operator docs.
2. Onboarding detects existing installations, datadirs, configs, cookie files, service definitions, and wallet candidates on macOS and Linux.
3. Migration wizard explains benefits, tradeoffs, unsupported surfaces, rollback expectations, and backup requirements before asking to proceed.
4. Dry-run migration plans list every proposed file, config, service, and wallet action before any write occurs.
5. Intentional behavior differences are recorded in parity docs and surfaced in migration output when relevant.
**Plans**: TBD

### Phase 22: Real-Sync Benchmarks and Release Hardening

**Goal**: Close v1.1 with reproducible performance evidence, documentation, parity updates, and verification coverage.
**Depends on**: Phase 21
**Requirements**: SYNC-05, MIG-05, VER-05, VER-06, VER-07, VER-08
**Success Criteria** (what must be TRUE):
1. Repo-native verification covers new CLI, config, service, storage, sync, metrics, logging, dashboard, migration, and parity-breadcrumb rules without requiring public network access.
2. Real-sync benchmarks measure headers sync, block download/connect, storage read/write, restart recovery, dashboard/status overhead, and wallet rescan cost with reproducible local reports.
3. Documentation explains install, onboarding, service lifecycle, status, dashboard, config layering, migration, real-sync testing, and known limitations.
4. Parity docs and machine-readable indexes distinguish v1.1 shipped claims from deferred or out-of-scope surfaces.
5. The milestone is ready for `/gsd-verify-work`, `/gsd-secure-phase`, and `/gsd-audit-milestone`.
**Plans**: TBD

## Progress

| Milestone | Phases | Plans | Status | Shipped |
| --- | ---: | ---: | --- | --- |
| v1.0 Headless Parity | 22/22 | 80/80 | Archived | 2026-04-26 |
| v1.1 Operator Runtime and Real-Network Sync | 4/10 | 16/16 | In Progress | - |
