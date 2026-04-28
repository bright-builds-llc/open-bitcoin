# Requirements: Open Bitcoin

**Defined:** 2026-04-26
**Core Value:** When a behavior is in scope, Open Bitcoin must behave like the pinned Knots baseline on the outside while staying simpler and safer on the inside.
**Milestone:** v1.1 Operator Runtime and Real-Network Sync

## v1.1 Requirements

### Operator Status and Observability

- [x] **OBS-01**: Operator can run `open-bitcoin status` against a running or stopped node and see daemon state, version, commit/build provenance, datadir, config paths, network, chain tip, sync progress, peer counts, mempool summary, wallet summary, service state, log paths, and recent health signals.
- [x] **OBS-02**: Operator can request machine-readable status output with stable JSON fields for automation and support.
- [x] **OBS-03**: The node records bounded historical metrics for sync height, header height, peer counts, mempool size, wallet balance summary, disk usage, RPC health, and service restarts.
- [x] **OBS-04**: The runtime writes structured logs with rotation, retention, and status-visible log locations.
- [x] **OBS-05**: Operators can inspect recent warnings or errors without opening raw log files manually.

### Terminal Dashboard

- [ ] **DASH-01**: Operator can launch a Ratatui dashboard that displays live sync state, peer state, mempool state, wallet summary, service state, logs, and health signals from the shared status model.
- [ ] **DASH-02**: Dashboard displays bounded historical metrics as terminal graphs for sync progress, peers, mempool size, disk usage, and RPC health.
- [x] **DASH-03**: Dashboard offers a keyboard menu for safe node queries and actions, with destructive or service-affecting operations gated by explicit confirmation.
- [ ] **DASH-04**: Dashboard uses a restrained, readable color palette that remains usable on common light and dark terminals and degrades cleanly without color.

### CLI, Config, and Onboarding

- [x] **CLI-03**: First-party CLI commands use the existing `clap` dependency for the main command tree, help text, global flags, subcommands, and typed arguments while preserving baseline-compatible `bitcoin-cli` RPC invocation behavior where in scope.
- [x] **CLI-04**: First-run onboarding wizard asks a small set of practical questions, is idempotent across repeated runs, supports non-interactive mode, and never overwrites user files without explicit approval.
- [x] **CLI-05**: Open Bitcoin-only wizard, dashboard, service, and migration answers are stored in a user-editable JSONC config file without breaking Bitcoin Core or Bitcoin Knots `bitcoin.conf` compatibility.
- [x] **CLI-06**: Config precedence among CLI flags, environment, Open Bitcoin JSONC config, `bitcoin.conf`, cookies, and defaults is documented and covered by tests.
- [x] **CLI-07**: CLI startup detects existing Bitcoin Core and Bitcoin Knots datadirs/config files and reports them in onboarding and status without mutating them.

### Service Lifecycle

- [x] **SVC-01**: Operator can `install`, `uninstall`, `enable`, `disable`, and inspect the Open Bitcoin daemon as a macOS launchd service, with dry-run output for generated plist contents.
- [x] **SVC-02**: Operator can `install`, `uninstall`, `enable`, `disable`, and inspect the Open Bitcoin daemon as a Linux systemd service, with dry-run output for generated unit contents.
- [x] **SVC-03**: Service commands surface privilege requirements, target scope, generated file paths, daemon command, config path, log path, and recovery behavior before applying changes.
- [x] **SVC-04**: Service status reports whether the service is installed, enabled, running, failed, stopped, or unmanaged, and links to relevant logs or manager diagnostics.
- [x] **SVC-05**: Service lifecycle tests run against isolated temp paths or fake managers and do not modify the developer machine's real launchd/systemd state.

### Durable Storage and Runtime Hardening

- [ ] **DB-01**: Contributors can inspect a documented database decision that compares Rust-native and RocksDB-style storage options against chainstate, header, block-index, wallet, metrics, recovery, Bazel, and dependency constraints.
- [ ] **DB-02**: Node persists headers, block index metadata, chainstate or UTXO state, undo/reorg metadata, wallet state, runtime metadata, and schema version information across restart.
- [ ] **DB-03**: Storage layer detects incompatible schema versions and corruption conditions and returns typed recovery errors instead of panicking.
- [ ] **DB-04**: Node can recover from interrupted writes through tested restart, replay, reindex, or repair flows.
- [ ] **DB-05**: Durable storage remains behind adapter traits so pure consensus, chainstate, mempool, wallet, and protocol code remain free of filesystem and database dependencies.

### Real Network Sync

- [ ] **SYNC-01**: Node can establish long-running outbound peer connections using configured peers and DNS/manual seed sources for supported networks.
- [ ] **SYNC-02**: Node can perform initial headers sync against real peers, persist progress, and resume after restart.
- [ ] **SYNC-03**: Node can request, download, validate, persist, and connect blocks from real peers with bounded in-flight work and observable progress.
- [ ] **SYNC-04**: Node handles peer disconnects, invalid data, timeouts, stalls, and competing branches with typed errors, metrics, and retry behavior.
- [x] **SYNC-05**: Sync behavior is covered by deterministic simulated-network tests plus opt-in live-network smoke tests that do not make default verification flaky.
- [x] **SYNC-06**: Sync progress and bottlenecks are visible through status, metrics history, logs, and dashboard panels.

### Wallet Runtime Expansion

- [x] **WAL-04**: Wallet supports richer send workflows equivalent to a safe `sendtoaddress`-style operator path, including fee limits, change handling, confirmation prompts, and deterministic error output.
- [x] **WAL-05**: Wallet supports multiwallet or wallet-scoped RPC/CLI selection compatible with the expected `-rpcwallet` style operator surface.
- [x] **WAL-06**: Wallet supports HD or ranged descriptor behavior needed for practical receive/change address management.
- [x] **WAL-07**: Wallet rescan, recovery, and balance tracking integrate with durable sync state and survive node restarts.
- [x] **WAL-08**: Wallet backup and migration planning can inspect existing Core/Knots wallet candidates without mutating them.

### Drop-In Parity and Migration

- [x] **MIG-01**: Open Bitcoin audits the Knots/Core drop-in replacement surface for CLI, RPC, config, datadir layout, service behavior, wallet behavior, network sync, logging, and operator documentation.
- [x] **MIG-02**: Onboarding can detect existing Bitcoin Core or Bitcoin Knots installations, datadirs, config files, cookie files, service definitions, and wallet candidates on macOS and Linux.
- [x] **MIG-03**: Migration wizard explains tradeoffs, benefits, unsupported surfaces, rollback expectations, and backup requirements before asking the operator to proceed.
- [x] **MIG-04**: Migration supports dry-run plans that show every proposed file, config, service, and wallet action before any write occurs.
- [x] **MIG-05**: Any intentional difference from Knots/Core behavior is recorded in the parity ledger and surfaced in migration output when relevant.

### Verification, Benchmarks, and Documentation

- [ ] **VER-05**: Repo-native verification covers new CLI, config, service, storage, sync, metrics, logging, dashboard, migration, and parity breadcrumb rules without requiring public network access by default.
- [ ] **VER-06**: Real-sync benchmarks measure headers sync, block download/connect, storage write/read, restart recovery, dashboard/status overhead, and wallet rescan costs with reproducible local reports.
- [ ] **VER-07**: Documentation explains v1.1 install, onboarding, service lifecycle, status, dashboard, config layering, migration, real-sync testing, and known limitations.
- [ ] **VER-08**: Parity docs and machine-readable indexes are updated so v1.1 claims are auditable and clearly separated from deferred surfaces.

## Future Requirements

### Future Product Surfaces

- **GUI-01**: Operators can use a graphical interface designed for Open Bitcoin rather than a faithful port of the reference Qt GUI.
- **WIN-01**: Operators can install and manage Open Bitcoin as a Windows service.
- **SITE-01**: Contributors and users can inspect a public progress-tracking site or dashboard that stays in sync with implementation status.
- **OBS-06**: Operators can export metrics to external observability systems.
- **PKG-01**: Operators can install signed release packages through OS-native package channels.

## Out of Scope

| Feature | Reason |
|---------|--------|
| Reference Qt GUI parity | v1.1 builds a local terminal dashboard, not a desktop GUI. |
| Windows service support | v1.1 prioritizes macOS and Linux lifecycle integration. |
| Hosted public dashboard | Local operator status and dashboard surfaces are more important for this milestone. |
| Automatic destructive migration | Existing Core/Knots data and wallets must be detected, explained, backed up, and explicitly approved before mutation. |
| Full drop-in replacement claim before audit closure | v1.1 must earn that claim through evidence and documented deviations. |
| Replacing `bitcoin.conf` semantics with JSONC | JSONC is for Open Bitcoin-only settings and must not break baseline config compatibility. |

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| OBS-01 | Phase 13, Phase 17, Phase 24 | Pending |
| OBS-02 | Phase 17, Phase 24 | Pending |
| OBS-03 | Phase 13, Phase 16 | Complete |
| OBS-04 | Phase 13, Phase 16 | Complete |
| OBS-05 | Phase 16, Phase 17 | Complete |
| DASH-01 | Phase 19, Phase 24 | Pending |
| DASH-02 | Phase 19, Phase 26 | Pending |
| DASH-03 | Phase 19, Phase 23 | Complete |
| DASH-04 | Phase 19, Phase 26 | Pending |
| CLI-03 | Phase 13, Phase 17 | Complete |
| CLI-04 | Phase 17 | Complete |
| CLI-05 | Phase 13, Phase 17 | Complete |
| CLI-06 | Phase 13, Phase 17 | Complete |
| CLI-07 | Phase 17, Phase 21 | Complete |
| SVC-01 | Phase 18, Phase 23 | Complete |
| SVC-02 | Phase 18, Phase 23 | Complete |
| SVC-03 | Phase 18, Phase 23 | Complete |
| SVC-04 | Phase 18, Phase 23 | Complete |
| SVC-05 | Phase 18, Phase 23 | Complete |
| DB-01 | Phase 13, Phase 26 | Pending |
| DB-02 | Phase 14, Phase 26 | Pending |
| DB-03 | Phase 14, Phase 26 | Pending |
| DB-04 | Phase 14, Phase 26 | Pending |
| DB-05 | Phase 14, Phase 26 | Pending |
| SYNC-01 | Phase 15, Phase 26 | Pending |
| SYNC-02 | Phase 15, Phase 26 | Pending |
| SYNC-03 | Phase 15, Phase 26 | Pending |
| SYNC-04 | Phase 15, Phase 26 | Pending |
| SYNC-05 | Phase 15, Phase 22 | Complete |
| SYNC-06 | Phase 16, Phase 19 | Complete |
| WAL-04 | Phase 20 | Complete |
| WAL-05 | Phase 20, Phase 24 | Pending |
| WAL-06 | Phase 20 | Complete |
| WAL-07 | Phase 20 | Complete |
| WAL-08 | Phase 20, Phase 21 | Complete |
| MIG-01 | Phase 21, Phase 26 | Pending |
| MIG-02 | Phase 17, Phase 21, Phase 25 | Pending |
| MIG-03 | Phase 21, Phase 26 | Pending |
| MIG-04 | Phase 21, Phase 25 | Pending |
| MIG-05 | Phase 21, Phase 22, Phase 26 | Pending |
| VER-05 | Phase 22, Phase 26 | Pending |
| VER-06 | Phase 22, Phase 27 | Pending |
| VER-07 | Phase 22, Phase 26 | Pending |
| VER-08 | Phase 22, Phase 26 | Pending |

**Coverage:**
- v1.1 requirements: 44 total
- Checked off: 28
- Mapped to phases: 44
- Unmapped: 0

---
*Requirements defined: 2026-04-26*
*Last updated: 2026-04-28 after Phase 23 closeout*
