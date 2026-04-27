---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 17-2026-04-26T23-56-00
generated_at: 2026-04-26T23:56:00.952Z
---

# Phase 17: CLI Status and First-Run Onboarding - Context

**Gathered:** 2026-04-26
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Implement the operator-facing command surface promised by Phase 13 and enabled by Phase 16. This phase makes `open-bitcoin status` useful for running and stopped nodes, adds stable JSON plus quiet human rendering, implements an idempotent first-run onboarding path, layers Open Bitcoin-only JSONC settings without changing `bitcoin.conf` semantics, detects existing Core/Knots data read-only, and documents/tests config precedence. This phase does not install launchd/systemd services, build the Ratatui dashboard, mutate existing Core/Knots data, implement wallet migration, or claim full drop-in replacement status.

</domain>

<decisions>
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

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase scope
- `.planning/ROADMAP.md` - Phase 17 goal, requirements, success criteria, and phase boundary.
- `.planning/REQUIREMENTS.md` - OBS-01, OBS-02, CLI-03, CLI-04, CLI-05, CLI-06, CLI-07, and MIG-02.
- `.planning/PROJECT.md` - v1.1 operator runtime direction, parity constraints, and quiet operator-surface tone.
- `.planning/phases/13-operator-runtime-foundations/13-CONTEXT.md` - locked CLI command boundary, shared status model, JSONC ownership, and config precedence decisions.
- `.planning/phases/16-metrics-logs-and-sync-telemetry/16-CONTEXT.md` - metrics/log/status health-signal contracts that status output must consume.

### Architecture and existing contracts
- `docs/architecture/cli-command-architecture.md` - `open-bitcoin` operator path and `open-bitcoin-cli` compatibility boundary.
- `docs/architecture/status-snapshot.md` - shared status snapshot field ownership and stopped-node unavailable-field semantics.
- `docs/architecture/config-precedence.md` - JSONC ownership, environment variables, and config precedence order.
- `packages/open-bitcoin-cli/src/operator.rs` - existing clap command contract and route selection.
- `packages/open-bitcoin-cli/src/operator/tests.rs` - existing operator routing tests.
- `packages/open-bitcoin-cli/src/startup.rs` - existing `bitcoin-cli` startup config/datadir resolution behavior.
- `packages/open-bitcoin-node/src/status.rs` - shared status snapshot, build provenance, and unavailable field model.
- `packages/open-bitcoin-node/src/logging.rs` - log retention, recent signal, and log status contracts.
- `packages/open-bitcoin-node/src/metrics.rs` - metric kinds, retention, samples, and metrics status contracts.
- `packages/open-bitcoin-rpc/src/config/open_bitcoin.rs` - Open Bitcoin JSONC config model and precedence identifiers.
- `packages/open-bitcoin-rpc/src/config/loader.rs` - existing `bitcoin.conf` loader and compatibility boundary.
- `docs/parity/source-breadcrumbs.json` - required breadcrumb manifest for new first-party Rust source/test files.

### Standards
- `AGENTS.md` - Repo-local GSD, parity, verification, Rust, and workflow rules.
- `AGENTS.bright-builds.md` - Bright Builds standards routing and highest-signal rules.
- `standards-overrides.md` - Local standards exception ledger.
- `https://github.com/bright-builds-llc/bright-builds-rules/blob/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/core/architecture.md`
- `https://github.com/bright-builds-llc/bright-builds-rules/blob/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/core/code-shape.md`
- `https://github.com/bright-builds-llc/bright-builds-rules/blob/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/core/testing.md`
- `https://github.com/bright-builds-llc/bright-builds-rules/blob/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/core/verification.md`
- `https://github.com/bright-builds-llc/bright-builds-rules/blob/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/languages/rust.md`

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `OperatorCli`, `OperatorCommand`, `StatusArgs`, `ConfigArgs`, and `OnboardArgs` already define the clap-owned operator command surface.
- `OpenBitcoinStatusSnapshot`, `FieldAvailability`, `BuildProvenance`, `LogStatus`, `MetricsStatus`, `SyncStatus`, `PeerStatus`, `MempoolStatus`, and `WalletStatus` already provide the reusable status model.
- `OpenBitcoinConfig`, `OnboardingConfig`, `ConfigPrecedence`, and `parse_open_bitcoin_jsonc_config` already provide the JSONC config contract.
- `resolve_startup_config` and the existing RPC config loader provide prior art for datadir, config path, and auth precedence without changing `bitcoin.conf` behavior.

### Established Patterns
- First-party Rust source files under breadcrumb scope need top-of-file parity breadcrumb comments and `docs/parity/source-breadcrumbs.json` entries.
- Multi-file Rust modules use `foo.rs` plus `foo/tests.rs` with `#[cfg(test)] mod tests;`.
- Pure status formatting, onboarding planning, detection classification, and config precedence decisions should be tested without real user datadirs or services.
- Effectful CLI shells should use isolated temp directories and fake collectors in tests.

### Integration Points
- The CLI crate should add operator execution/rendering modules without re-routing `open-bitcoin-cli` compatibility parsing.
- Status collectors should map local/RPC/service/log/metric evidence into `OpenBitcoinStatusSnapshot`.
- Onboarding should write or plan `open-bitcoin.jsonc` updates through Open Bitcoin-owned config types, preserving the `bitcoin.conf` compatibility boundary.
- Core/Knots detection should be a read-only support surface reused by later migration phases.

</code_context>

<specifics>
## Specific Ideas

- Favor operator trust over cleverness: show what was read, what was unavailable, and what would be written before writes happen.
- Prefer stable JSON field names and explicit unavailable reasons over compact output that hides missing evidence.
- Keep onboarding short enough to rerun confidently; advanced migration details belong to later migration phases.

</specifics>

<deferred>
## Deferred Ideas

- macOS launchd and Linux systemd install/enable/disable behavior belongs to Phase 18.
- Ratatui dashboard rendering belongs to Phase 19.
- Wallet runtime expansion and multiwallet operator workflows belong to Phase 20.
- Dry-run migration plans and backup-aware mutation belong to Phase 21.
- Real-sync benchmark reports and release hardening belong to Phase 22.

</deferred>

---

*Phase: 17-cli-status-and-first-run-onboarding*
*Context gathered: 2026-04-26*
