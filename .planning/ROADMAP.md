# Roadmap: Open Bitcoin

## Milestones

- **v1.0 Headless Parity** - Phases 1 through 12 shipped on 2026-04-26. Archive: [v1.0-ROADMAP.md](milestones/v1.0-ROADMAP.md)
- **v1.1 Operator Runtime and Real-Network Sync** - Phases 13 through 27 now define this milestone, including post-audit gap-closure follow-up work.

## v1.1 Operator Runtime and Real-Network Sync

This active milestone makes Open Bitcoin usable as an operator-facing, service-managed node that can begin true network sync work with durable storage, richer CLI/TUI surfaces, migration guidance, and expanded parity coverage.

Post-audit gap-closure phases now extend the milestone so service apply behavior, live status truthfulness, migration source selection, evidence reconciliation, and operator-runtime benchmark fidelity are resolved before archive.

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
- [x] **Phase 17: CLI Status and First-Run Onboarding** - Expand the clap command tree, implement rich status output, and add the idempotent wizard plus JSONC config layer. (completed 2026-04-27)
- [x] **Phase 18: Service Lifecycle Integration** - Add macOS launchd and Linux systemd install/uninstall/enable/disable/status support with dry-run safety. (completed 2026-04-27)
- [x] **Phase 19: Ratatui Node Dashboard** - Build the terminal dashboard on top of the shared status, metrics, logs, service, and sync models. (completed 2026-04-27)
- [x] **Phase 20: Wallet Runtime Expansion** - Expand practical wallet runtime behavior for send, wallet selection, descriptors, rescans, backups, and migration inspection. (completed 2026-04-27)
- [x] **Phase 21: Drop-In Parity Audit and Migration** - Audit Core/Knots replacement expectations and implement detection, education, and dry-run migration plans. (completed 2026-04-27)
- [x] **Phase 22: Real-Sync Benchmarks and Release Hardening** - Add real-sync benchmarks, docs, parity updates, and verification coverage for the v1.1 operator runtime. (completed 2026-04-27)
- [x] **Phase 23: Service Apply Completion and Status Truthfulness** - Complete launchd/systemd apply semantics, truthful enabled-state reporting, and dashboard service action closure after the v1.1 audit. (completed 2026-04-28)
- [x] **Phase 24: Wallet-Aware Live Status and Build Provenance** - Keep status and dashboard truthfully live when wallet selection is ambiguous and surface real build provenance. (completed 2026-04-28)
- [x] **Phase 25: Migration Source Selection Hardening** - Let `migrate plan --source-datadir` select valid custom-location installs without degrading to manual review. (completed 2026-04-28)
- [x] **Phase 26: Milestone Evidence and Audit Reconciliation** - Align verification reports, summary frontmatter, and requirements bookkeeping so the v1.1 audit can pass cleanly. (completed 2026-04-28)
- [x] **Phase 27: Operator Runtime Benchmark Fidelity** - Replace fixture-only operator-runtime benchmark cases with runtime-collected status/dashboard evidence. (completed 2026-04-28)
- [x] **Phase 28: Service Log-Path Truth and Operator Docs Alignment** - Preserve configured service log-path truth across launchd/systemd preview, apply, status, and operator docs. (completed 2026-04-29)
- [ ] **Phase 29: Closeout Hygiene and Build Provenance** - Address the remaining optional post-audit cleanup around build provenance truthfulness and milestone-closeout hygiene. (planned optional cleanup)

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
**Plans**: 3/3 plans complete

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
- [x] 17-01-PLAN.md — Operator module contracts and breadcrumb coverage
- [x] 17-02-PLAN.md — Config precedence and JSONC path reporting
- [x] 17-03-PLAN.md — Read-only Core/Knots detection
- [x] 17-04-PLAN.md — Shared status collection and rendering
- [x] 17-05-PLAN.md — First-run onboarding and open-bitcoin binary wiring

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
**Plans**: 3 plans

Plans:
- [x] 18-01-PLAN.md — ServiceManager trait, pure generators, FakeServiceManager, and platform factory
- [x] 18-02-PLAN.md — Runtime wiring: --apply flag, execute_service_command dispatch, detection_roots population
- [x] 18-03-PLAN.md — Status integration: service adapter injection into collect_status_snapshot and render verification

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
**Plans**: 3 plans

Plans:
- [x] 19-01-PLAN.md — Dashboard command dispatch and snapshot-first non-interactive fallback
- [x] 19-02-PLAN.md — Interactive Ratatui app loop with bounded charts and action confirmation model
- [x] 19-03-PLAN.md — Deterministic non-interactive and action-guard test coverage

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
**Plans**: 5/5 plans complete

Plans:
- [x] 20-01-PLAN.md — Pure wallet-core ranged descriptor, send-intent, and rescan-progress contracts
- [x] 20-02-PLAN.md — Durable named-wallet registry, Fjall snapshot upgrade, and rescan job persistence
- [x] 20-03-PLAN.md — Wallet-scoped RPC/CLI routing plus the practical Phase 20 wallet method subset
- [x] 20-04-PLAN.md — Shared wallet freshness status and format-aware read-only external wallet inspection
- [x] 20-05-PLAN.md — Operator send preview/confirm, backup export, and Phase 20 parity closeout

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
**Plans**: 3/3 plans complete

Plans:
- [x] 21-01-PLAN.md — `migrate plan` contract, dry-run planner, and focused unit coverage
- [x] 21-02-PLAN.md — Sandboxed migration binary proof and parity-notice sync guard
- [x] 21-03-PLAN.md — Drop-in audit matrix, parity ledger updates, and contributor docs

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
**Plans**: 3/3 plans complete

Plans:
- [x] 22-01-PLAN.md — Runtime-backed benchmark coverage and reproducible report metadata
- [x] 22-02-PLAN.md — Operator release guide and documentation refresh
- [x] 22-03-PLAN.md — Parity ledger, benchmark verification, and release-readiness closeout

### Phase 23: Service Apply Completion and Status Truthfulness

**Goal**: Finish real launchd/systemd apply semantics and make service state projections truthful for status and dashboard surfaces.
**Depends on**: Phase 22
**Requirements**: SVC-01, SVC-02, SVC-03, SVC-04, SVC-05, DASH-03
**Gap Closure**: Closes audit blocker `INT-01` and the service-action portion of the dashboard gap.
**Success Criteria** (what must be TRUE):
1. `open-bitcoin service install --apply` executes the required bootstrap/reload-enable steps on supported platforms after writing service artifacts.
2. Service status and dashboard summaries distinguish installed, enabled, running, failed, stopped, and unmanaged states using real manager evidence instead of inference.
3. Dashboard service actions reuse the corrected apply path and remain confirmation-gated.
4. Service verification evidence and requirements bookkeeping are refreshed around the corrected runtime behavior.
**Plans**: 3/3 plans complete

Plans:
- [x] 23-01-PLAN.md — launchd and systemd install apply sequencing plus truthful dry-run previews
- [x] 23-02-PLAN.md — explicit enabled-state evidence for service status and dashboard projections
- [x] 23-03-PLAN.md — Phase 23 verification, roadmap refresh, and requirements bookkeeping closeout

### Phase 24: Wallet-Aware Live Status and Build Provenance

**Goal**: Keep live status and dashboard snapshots truthful when wallet selection is missing or ambiguous, and surface real build provenance through the shared operator status model.
**Depends on**: Phase 22
**Requirements**: OBS-01, OBS-02, WAL-05, DASH-01
**Gap Closure**: Closes audit blocker `INT-02` and the build-provenance follow-up from the milestone audit.
**Success Criteria** (what must be TRUE):
1. Live status and dashboard snapshots no longer collapse to `NodeRuntimeState::Unreachable` when wallet selection is missing or ambiguous.
2. Wallet selection issues are surfaced as wallet-specific unavailable or diagnostic data while node reachability remains accurate.
3. Build provenance is populated in shared status snapshots when available and rendered through operator-facing status/dashboard surfaces.
4. Targeted verification covers zero-wallet, multiwallet, selected-wallet, and build-provenance runtime paths.
**Plans**: 3/3 plans complete

Plans:
- [x] 24-01-PLAN.md — wallet-aware status routing and wallet-only degradation for status and dashboard
- [x] 24-02-PLAN.md — compile-time build provenance population plus status and dashboard rendering
- [x] 24-03-PLAN.md — Phase 24 verification, roadmap refresh, and requirements traceability closeout

### Phase 25: Migration Source Selection Hardening

**Goal**: Let explicit migration source paths participate directly in source selection so custom-location Core/Knots installs can produce concrete dry-run plans.
**Depends on**: Phase 21
**Requirements**: MIG-02, MIG-04
**Gap Closure**: Closes warning `INT-W03` from the milestone audit.
**Success Criteria** (what must be TRUE):
1. `open-bitcoin migrate plan --source-datadir <custom-path>` can select a valid source install outside the default detection roots when the path is explicit and supported.
2. Ambiguous or missing source installs still fall back to manual review with clear operator guidance.
3. Migration verification covers explicit custom-path selection while preserving dry-run-first safety and source-data non-mutation.
**Plans**: 3/3 plans complete

Plans:
- [x] 25-01-PLAN.md — migration-scoped explicit source detection for custom datadirs
- [x] 25-02-PLAN.md — conservative explicit-path validation and migration regression coverage
- [x] 25-03-PLAN.md — Phase 25 verification, roadmap refresh, and migration traceability closeout

### Phase 26: Milestone Evidence and Audit Reconciliation

**Goal**: Reconcile v1.1 verification reports, summary frontmatter, and requirements bookkeeping so the milestone audit no longer reports orphaned or stale evidence gaps.
**Depends on**: Phase 23, Phase 24, Phase 25
**Requirements**: DB-01, DB-02, DB-03, DB-04, DB-05, SYNC-01, SYNC-02, SYNC-03, SYNC-04, DASH-02, DASH-04, MIG-01, MIG-03, MIG-05, VER-05, VER-07, VER-08
**Gap Closure**: Closes orphaned requirement evidence and ledger/frontmatter reconciliation gaps from the v1.1 milestone audit.
**Success Criteria** (what must be TRUE):
1. Phase 13 through Phase 15 verification artifacts explicitly cover `DB-01` through `DB-05` and `SYNC-01` through `SYNC-04` by requirement ID.
2. Later Phase 18, 19, 21, and 22 summaries and requirement bookkeeping reflect shipped requirement evidence consistently.
3. `REQUIREMENTS.md` traceability rows and checkbox state align with the milestone audit verdict and re-audit expectations.
4. A rerun milestone audit no longer reports orphaned or stale evidence-only gaps for the requirements assigned here.
**Plans**: 3/3 plans complete

Plans:
- [x] 26-01-PLAN.md — explicit DB/SYNC requirement coverage in legacy verification reports
- [x] 26-02-PLAN.md — summary frontmatter backfill and requirements ledger reconciliation
- [x] 26-03-PLAN.md — milestone audit rerun, roadmap refresh, and Phase 26 closeout

### Phase 27: Operator Runtime Benchmark Fidelity

**Goal**: Upgrade operator-runtime benchmark cases from snapshot-fixture projection checks to runtime-collected status/dashboard evidence without losing deterministic verification.
**Depends on**: Phase 24, Phase 26
**Requirements**: VER-06
**Gap Closure**: Optional benchmark fidelity follow-up accepted during v1.1 gap planning.
**Success Criteria** (what must be TRUE):
1. Operator-runtime benchmark cases collect status/dashboard data through the real runtime collection path or an equivalent shell entrypoint instead of only `sample_status_snapshot()` fixtures.
2. Report metadata and benchmark validation still describe the operator-runtime cases accurately after the fidelity upgrade.
3. Repo-native verification keeps the benchmark/report path deterministic and free of public-network requirements.
**Plans**: 3/3 plans complete

Plans:
- [x] 27-01-PLAN.md — runtime-collected status benchmark path
- [x] 27-02-PLAN.md — runtime-collected dashboard path and benchmark metadata refresh
- [x] 27-03-PLAN.md — verification, roadmap refresh, and final requirement closeout

### Phase 28: Service Log-Path Truth and Operator Docs Alignment

**Goal**: Preserve configured service log-path truth across launchd/systemd preview, apply, status, and operator docs.
**Depends on**: Phase 22, Phase 23
**Requirements**: SVC-03, SVC-04, VER-07
**Gap Closure**: Closes milestone audit blocker `INT-v1.1-01` and broken flow `FLOW-v1.1-01`.
**Success Criteria** (what must be TRUE):
1. Launchd and systemd service definitions preserve or truthfully surface the operator-selected log-path behavior in generated artifacts and dry-run previews.
2. `open-bitcoin service status` returns the effective service log path or an explicit platform-backed unavailable reason through `ServiceStateSnapshot`.
3. Operator docs and dashboard/shared service actions stay aligned with the repaired service log-path behavior.
**Plans**: 2/2 plans complete

Plans:
- [x] 28-01-PLAN.md — restore service log-path truth
- [x] 28-02-PLAN.md — verify, document, and reclose the blocker

### Phase 29: Closeout Hygiene and Build Provenance

**Goal**: Address the remaining optional post-audit cleanup around build provenance truthfulness and milestone-closeout hygiene before archive.
**Depends on**: Phase 24, Phase 28
**Requirements**: none (optional cleanup)
**Gap Closure**: Optional cleanup for post-Phase-27 audit tech debt.
**Success Criteria** (what must be TRUE):
1. Build provenance claims stay truthful for non-Cargo builds through either populated metadata or explicitly documented unavailable behavior.
2. Milestone closeout docs and roadmap surfaces remain internally consistent after the final gap-closure work.
3. Optional cleanup added here does not reopen the passing benchmark, migration, or service-flow gates.
**Plans**: 0 plans yet

## Progress

| Milestone | Phases | Plans | Status | Shipped |
| --- | ---: | ---: | --- | --- |
| v1.0 Headless Parity | 22/22 | 80/80 | Archived | 2026-04-26 |
| v1.1 Operator Runtime and Real-Network Sync | 16/17 | 55/55 current | Optional cleanup pending | - |
