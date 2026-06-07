# Phase 63: Service Supervision Lifecycle - Research

**Researched:** 2026-06-07
**Domain:** Rust operator CLI service lifecycle, macOS launchd, Linux systemd, shared status rendering
**Confidence:** HIGH

<user_constraints>

## User Constraints (from CONTEXT.md)

The following constraints are copied from `63-CONTEXT.md`. [VERIFIED: .planning/phases/63-service-supervision-lifecycle/63-CONTEXT.md]

### Locked Decisions

### Service Command Surface

- **D-01:** Keep `open-bitcoin service install` as a dry-run preview unless
  `--apply` is supplied, and add or document an explicit preview path so the
  operator can run a side-effect-free service preview without guessing the
  `install` dry-run convention. Preview output must show the exact service file
  path, generated content, and manager commands that would run.
- **D-02:** Add start, stop, and restart service actions to the existing
  `ServiceManager` abstraction, fake manager, CLI dispatcher, and dashboard
  service action path. These actions are effectful manager operations and must
  return typed `ServiceCommandOutcome` values with the exact launchd/systemd
  command strings surfaced in human output.
- **D-03:** Preserve existing install, uninstall, enable, disable, and status
  behavior while extending it. Existing dry-run safety for install/uninstall
  must stay intact, and no command may mutate a Bitcoin Core or Bitcoin Knots
  source service or source datadir.
- **D-04:** Service definitions must supervise `open-bitcoind`, not the
  `open-bitcoin` operator CLI wrapper. Resolve the daemon binary through a
  small testable helper that prefers a sibling `open-bitcoind` next to the
  operator binary and falls back to the literal `open-bitcoind` command name
  when a concrete sibling path cannot be proven.

### Lifecycle Status Contract

- **D-05:** Normalize service status into the Phase 63 contract:
  `unmanaged`, `installed-stopped`, `running`, `failed`, `disabled`, and
  `unavailable-manager`. Existing `Installed`, `Enabled`, `Stopped`, and
  manager-error evidence should map into those operator-facing labels rather
  than leaking inconsistent platform vocabulary.
- **D-06:** `open-bitcoin service status`, `open-bitcoin status`, dashboard
  service rows, and JSON status output should agree on service manager,
  installed, enabled, running, log path, service file path, diagnostics, and
  unavailable reasons. Missing manager evidence must stay explicit with
  `Unavailable` reasons instead of false success, empty strings, or zeros.
- **D-07:** Preserve shared sync truth fields from Phase 62 alongside service
  lifecycle state. Service status should complement the existing sync lifecycle,
  progress, stop reason, recovery category, configured targets, and
  downloaded/connected block evidence instead of creating a second independent
  sync interpretation.
- **D-08:** Failed or unavailable manager calls should become typed
  operator-visible states where status inspection can still succeed. Action
  commands may fail when the requested manager operation cannot run, but status
  inspection should distinguish unsupported platform, missing manager command,
  unmanaged service, disabled service, stopped service, running service, and
  failed service.

### Launchd And Systemd Behavior

- **D-09:** launchd support should stay user-level under `~/Library/LaunchAgents`
  and systemd support should stay user-level under `~/.config/systemd/user`.
  Do not introduce sudo, machine-wide unit installation, packaging hooks, or
  global daemon claims in Phase 63.
- **D-10:** Generated launchd plist and systemd unit files must include the
  selected datadir and optional Open Bitcoin JSONC config path, route stdout and
  stderr to the configured operator service log path when one exists, and keep
  explicit generated-by comments.
- **D-11:** Start/stop/restart implementation should use platform-native user
  manager commands: `systemctl --user start|stop|restart
  open-bitcoin-node.service` on Linux and launchd `bootstrap`, `bootout`, or
  `kickstart -k` operations against `gui/<uid>/org.open-bitcoin.node` on macOS
  where those operations are the least surprising fit for the existing user
  plist model.

### Operator Documentation And UAT

- **D-12:** Update the operator runbook to show launchd and systemd command
  flows for preview, install, start, stop, restart, status, disable, uninstall,
  log inspection, config path review, safe shutdown, and recovery next actions.
  Use copy-pasteable repo-local Cargo and Bazel command forms, not only the
  installed `open-bitcoin` alias.
- **D-13:** Keep service workflow language bounded to opt-in extended operator
  review. Generated files, CLI output, docs, and verification notes must not
  call Open Bitcoin a production service, production full node, packaged
  service guarantee, or unattended production-node replacement.
- **D-14:** Public-network service checks are optional UAT only. Default
  verification must remain deterministic through Rust tests, docs/checker
  assertions where useful, and `bash scripts/verify.sh`; it must not start a
  live public-mainnet service or require network access.

### the agent's Discretion

- The planner may split work by service contract, platform adapters, operator
  surfaces, and docs if that keeps each plan reviewable.
- The executor may add a small pure helper for service display-state mapping,
  daemon binary path resolution, or platform command rendering when it reduces
  duplication between CLI, dashboard, and status collectors.
- If new first-party Rust source or test files are added under
  `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests`, the executor
  must update parity breadcrumbs before committing.

### Deferred Ideas (OUT OF SCOPE)

- Service-supervised same-datadir restart/resume proof belongs to Phase 64.
- Redacted v1.5 support bundle expansion belongs to Phase 65.
- Compatibility harness operator wrapper belongs to Phase 66.
- v1.5 threat-model and release-boundary closeout belongs to Phase 67.
- Windows service integration, signed packages, machine-wide install flows, and
  broad production-node support remain out of scope for this milestone.

</user_constraints>

<phase_requirements>

## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| SVC-01 | Operator can preview, install, start, stop, restart, and inspect launchd or systemd supervision for the opt-in unattended daemon workflow without implying a broad production-node claim. [VERIFIED: .planning/REQUIREMENTS.md] | Extend the existing `ServiceManager` trait and CLI/dashboard dispatch path instead of creating a second service workflow. [VERIFIED: packages/open-bitcoin-cli/src/operator/service.rs; packages/open-bitcoin-cli/src/operator/dashboard/action.rs] Use launchd `bootstrap`/`bootout`/`kickstart -k` and systemd `systemctl --user start|stop|restart` semantics from platform docs. [VERIFIED: local man launchctl; CITED: https://www.freedesktop.org/software/systemd/man/latest/systemctl.html] |
| SVC-02 | Service status distinguishes unmanaged, installed-stopped, running, failed, disabled, and unavailable-manager states while preserving shared sync truth fields. [VERIFIED: .planning/REQUIREMENTS.md] | Add a typed shared service lifecycle field to `ServiceStatus`, project adapter snapshots into it with `FieldAvailability`, and keep Phase 62 sync fields untouched. [VERIFIED: packages/open-bitcoin-node/src/status.rs; packages/open-bitcoin-cli/src/operator/status.rs; .planning/phases/62-long-run-sync-truth-surfaces/62-CONTEXT.md] |
| SVC-04 | Service runbooks explain log locations, config paths, safe shutdown, restart review, and recovery actions for launchd and systemd operators. [VERIFIED: .planning/REQUIREMENTS.md] | Update `docs/operator/runtime-guide.md` and add deterministic checker coverage for repo-local Cargo/Bazel commands, service labels, log paths, config paths, and opt-in/public-network boundaries. [VERIFIED: docs/operator/runtime-guide.md; scripts/check-phase62-sync-truth-surfaces.ts; scripts/verify.sh] |

</phase_requirements>

## Summary

Phase 63 should be planned as an extension of the existing Open Bitcoin service adapter boundary, not as a new supervisor abstraction. [VERIFIED: packages/open-bitcoin-cli/src/operator/service.rs] The current code already has pure launchd plist and systemd unit generation, platform adapter structs, fake manager tests, CLI dispatch, dashboard dispatch, and status projection hooks. [VERIFIED: packages/open-bitcoin-cli/src/operator/service/launchd.rs; packages/open-bitcoin-cli/src/operator/service/systemd.rs; packages/open-bitcoin-cli/src/operator/service/fake.rs; packages/open-bitcoin-cli/src/operator/dashboard/action.rs; packages/open-bitcoin-cli/src/operator/status.rs]

The highest-risk implementation gap is that service install currently resolves `std::env::current_exe()` and therefore generates service definitions for the `open-bitcoin` operator binary rather than `open-bitcoind`. [VERIFIED: packages/open-bitcoin-cli/src/operator/runtime.rs:199] The second highest-risk gap is that shared `ServiceStatus` exposes only `manager`, `installed`, `enabled`, and `running`, while Phase 63 requires a stable lifecycle label, service file path, log path, diagnostics, and unavailable reasons across human status, JSON status, and dashboard rows. [VERIFIED: packages/open-bitcoin-node/src/status.rs:62; .planning/phases/63-service-supervision-lifecycle/63-CONTEXT.md D-05-D-06]

**Primary recommendation:** Add one shared typed service lifecycle contract, extend the existing `ServiceManager` trait with `start`, `stop`, and `restart`, add a first-class dry-run `preview` service subcommand, and keep launchd/systemd operations user-scope and deterministic-testable. [VERIFIED: .planning/phases/63-service-supervision-lifecycle/63-CONTEXT.md D-01-D-14; packages/open-bitcoin-cli/src/operator/service.rs]

## Project Constraints (from AGENTS.md)

- Read and apply `AGENTS.md`, `AGENTS.bright-builds.md`, `standards-overrides.md`, and relevant Bright Builds standards before planning. [VERIFIED: AGENTS.md; AGENTS.bright-builds.md; standards-overrides.md; CITED: https://raw.githubusercontent.com/bright-builds-llc/bright-builds-rules/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/index.md]
- Keep functional-core decisions pure and service-manager effects in thin adapters. [VERIFIED: AGENTS.md; CITED: https://raw.githubusercontent.com/bright-builds-llc/bright-builds-rules/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/core/architecture.md]
- Use Rust 1.94.1 from `rust-toolchain.toml` and Rust 2024 from `packages/Cargo.toml`. [VERIFIED: rust-toolchain.toml; packages/Cargo.toml]
- Use `bash scripts/verify.sh` as the repo-native verification contract, and keep public-network checks outside that default gate. [VERIFIED: AGENTS.md; scripts/verify.sh; .planning/REQUIREMENTS.md REL-03]
- Provide copy-pasteable repo-local Cargo and Bazel UAT commands instead of only the installed `open-bitcoin` alias. [VERIFIED: AGENTS.md; .planning/phases/63-service-supervision-lifecycle/63-CONTEXT.md D-12]
- Use Bun for repo-owned higher-level automation scripts and TypeScript for substantial checker logic. [VERIFIED: AGENTS.md; .bun-version; scripts/check-phase62-sync-truth-surfaces.ts]
- If new first-party Rust source or test files are added under `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests`, update parity breadcrumbs before committing. [VERIFIED: AGENTS.md; docs/parity/source-breadcrumbs.json]
- Treat `docs/metrics/lines-of-code.md` as tracked generated output that may change during verification. [VERIFIED: AGENTS.md; scripts/verify.sh]
- Do not recommend approaches that mutate Bitcoin Core or Bitcoin Knots source services or source datadirs. [VERIFIED: AGENTS.md; .planning/phases/63-service-supervision-lifecycle/63-CONTEXT.md D-03]
- The root `standards/` directory named by `AGENTS.md` is not present in this checkout; the pinned Bright Builds standards were read from the commit recorded in `AGENTS.bright-builds.md`. [VERIFIED: rg --files; AGENTS.bright-builds.md; CITED: https://raw.githubusercontent.com/bright-builds-llc/bright-builds-rules/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/core/testing.md]
- No project-local skills exist under `.claude/skills/` or `.agents/skills/`. [VERIFIED: find .claude/skills .agents/skills]

## Standard Stack

### Core

| Library / Tool | Version | Purpose | Why Standard |
|----------------|---------|---------|--------------|
| Rust toolchain | 1.94.1 | First-party implementation language for service contracts, adapters, renderers, and tests. | The repo pins this version and Rust 2024 for Cargo workspace code. [VERIFIED: rust-toolchain.toml; packages/Cargo.toml] |
| `std::process::Command` | Rust std | Execute `launchctl` and `systemctl` without shell interpolation. | Existing platform adapters already use `Command::new(...).args(...)`, which avoids shell parsing for manager invocations. [VERIFIED: packages/open-bitcoin-cli/src/operator/service/launchd.rs; packages/open-bitcoin-cli/src/operator/service/systemd.rs] |
| `clap` | 4.6.1 resolved | Parse `open-bitcoin service ...` subcommands. | `ServiceArgs` and `ServiceCommand` already use clap derive, so extending the enum is the local pattern. [VERIFIED: packages/open-bitcoin-cli/Cargo.toml; packages/Cargo.lock; packages/open-bitcoin-cli/src/operator.rs] |
| `serde` / `serde_json` | 1.0.228 / 1.0.149 resolved | Serialize shared status JSON, including additive service lifecycle fields. | `OpenBitcoinStatusSnapshot`, `ServiceStatus`, and `FieldAvailability` already derive serde traits. [VERIFIED: packages/open-bitcoin-node/src/status.rs; packages/Cargo.lock] |
| `thiserror` | 2.0.18 resolved | Typed service errors. | `ServiceError` already uses `thiserror::Error`; new action failures should reuse it. [VERIFIED: packages/open-bitcoin-cli/src/operator/service.rs; packages/Cargo.lock] |
| `ratatui` / `crossterm` | 0.30.0 / 0.29.0 resolved | Interactive dashboard service actions and rows. | Dashboard service actions already dispatch through the same service command path and terminal app stack. [VERIFIED: packages/open-bitcoin-cli/src/operator/dashboard/action.rs; packages/open-bitcoin-cli/Cargo.toml; packages/Cargo.lock] |

### Supporting

| Library / Tool | Version | Purpose | When to Use |
|----------------|---------|---------|-------------|
| Bun | 1.3.9 | Deterministic TypeScript checker for docs/default-verification boundaries. | Use if Phase 63 needs a checker like Phase 61 and Phase 62 did. [VERIFIED: .bun-version; command `bun --version`; scripts/check-phase62-sync-truth-surfaces.ts] |
| Bazel/Bazelisk | Bazelisk 1.28.1, Bazel 8.6.0 | Repo smoke build and UAT command form. | Docs must include `bazel run //packages/open-bitcoin-cli:open_bitcoin -- ...`; `scripts/verify.sh` builds Bazel targets. [VERIFIED: command `bazelisk version`; scripts/verify.sh; AGENTS.md] |
| macOS `launchctl` | Darwin Bootstrapper 7.0.0 locally | Optional local macOS service manager UAT. | Available on this machine for manual launchd review, but default verification must not install/start live services. [VERIFIED: command `launchctl version`; .planning/phases/63-service-supervision-lifecycle/63-CONTEXT.md D-14] |
| Linux `systemctl` | Not available locally | Linux user service manager. | Cover Linux behavior through pure unit generation and fake/adapter command tests on this macOS host. [VERIFIED: command `command -v systemctl`; .planning/phases/63-service-supervision-lifecycle/63-CONTEXT.md D-14] |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Existing Rust `ServiceManager` trait | A new service-controller module or external crate | Do not add this; the existing trait already isolates filesystem/subprocess effects and has a fake manager for deterministic tests. [VERIFIED: packages/open-bitcoin-cli/src/operator/service.rs; packages/open-bitcoin-cli/src/operator/service/fake.rs] |
| `std::process::Command` args | Shell command strings through `sh -c` | Do not use shell strings; manager commands include user paths and must avoid shell injection surfaces. [VERIFIED: packages/open-bitcoin-cli/src/operator/service/systemd.rs; packages/open-bitcoin-cli/src/operator/service/launchd.rs; CITED: https://www.freedesktop.org/software/systemd/man/latest/systemd.service.html] |
| Add service-supervised public-mainnet UAT to default verification | Deterministic Rust tests plus docs/checker assertions | Public-network and live service checks are explicitly optional UAT and excluded from `bash scripts/verify.sh`. [VERIFIED: .planning/REQUIREMENTS.md REL-03; scripts/verify.sh; .planning/phases/63-service-supervision-lifecycle/63-CONTEXT.md D-14] |

**Installation:**

```bash
# No new dependency installation is recommended for Phase 63.
```

**Version verification:** Recommended package versions are existing repo dependencies verified from `Cargo.toml`, `Cargo.lock`, `.bun-version`, `rust-toolchain.toml`, and local CLI probes; no new npm package version verification applies because Phase 63 should not add npm packages. [VERIFIED: packages/open-bitcoin-cli/Cargo.toml; packages/Cargo.lock; .bun-version; rust-toolchain.toml]

## Architecture Patterns

### Recommended Project Structure

```text
packages/open-bitcoin-node/src/
└── status.rs                         # Add shared ServiceLifecycleStatus and richer ServiceStatus fields. [VERIFIED: packages/open-bitcoin-node/src/status.rs]

packages/open-bitcoin-cli/src/operator/
├── operator.rs                       # Add service preview/start/stop/restart clap subcommands. [VERIFIED: packages/open-bitcoin-cli/src/operator.rs]
├── runtime.rs                        # Use daemon-binary resolver for service and dashboard runtime. [VERIFIED: packages/open-bitcoin-cli/src/operator/runtime.rs]
├── service.rs                        # Extend trait, command dispatcher, shared projection/render helpers. [VERIFIED: packages/open-bitcoin-cli/src/operator/service.rs]
├── service/
│   ├── fake.rs                       # Record start/stop/restart calls for deterministic tests. [VERIFIED: packages/open-bitcoin-cli/src/operator/service/fake.rs]
│   ├── launchd.rs                    # User LaunchAgent plist and launchctl commands. [VERIFIED: packages/open-bitcoin-cli/src/operator/service/launchd.rs]
│   ├── systemd.rs                    # User unit file and systemctl --user commands. [VERIFIED: packages/open-bitcoin-cli/src/operator/service/systemd.rs]
│   └── tests.rs                      # Parser, generator, dispatcher, state-mapping tests. [VERIFIED: packages/open-bitcoin-cli/src/operator/service/tests.rs]
├── status.rs                         # Project service snapshot into shared status fields. [VERIFIED: packages/open-bitcoin-cli/src/operator/status.rs]
├── status/render.rs                  # Human and JSON service status consistency. [VERIFIED: packages/open-bitcoin-cli/src/operator/status/render.rs]
└── dashboard/                        # Add service lifecycle rows/actions using same dispatcher. [VERIFIED: packages/open-bitcoin-cli/src/operator/dashboard]

docs/operator/runtime-guide.md        # Launchd/systemd runbook and UAT commands. [VERIFIED: docs/operator/runtime-guide.md]
scripts/check-phase63-service-lifecycle.ts # Optional deterministic docs/boundary checker if docs surface expands. [VERIFIED: scripts/check-phase62-sync-truth-surfaces.ts]
```

### Pattern 1: One Shared Service Lifecycle Contract

**What:** Add a shared serde enum for operator-facing service states and expose it through `ServiceStatus` as `FieldAvailability<ServiceLifecycleStatus>`. [VERIFIED: packages/open-bitcoin-node/src/status.rs; .planning/phases/63-service-supervision-lifecycle/63-CONTEXT.md D-05-D-06]

**When to use:** Use this for `open-bitcoin status`, dashboard rows, JSON output, and `open-bitcoin service status` display mapping so every surface uses the same labels. [VERIFIED: packages/open-bitcoin-cli/src/operator/status.rs; packages/open-bitcoin-cli/src/operator/status/render.rs; packages/open-bitcoin-cli/src/operator/dashboard/model.rs]

**Example:**

```rust
// Source: packages/open-bitcoin-node/src/status.rs and Phase 63 CONTEXT D-05.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ServiceLifecycleStatus {
    Unmanaged,
    InstalledStopped,
    Running,
    Failed,
    Disabled,
    UnavailableManager,
}
```

### Pattern 2: Pure Mapping From Adapter Snapshot To Operator Contract

**What:** Keep platform-specific status collection in launchd/systemd adapters, then map `ServiceStateSnapshot` into `ServiceStatus` with a pure helper. [VERIFIED: packages/open-bitcoin-cli/src/operator/service/launchd.rs; packages/open-bitcoin-cli/src/operator/service/systemd.rs; packages/open-bitcoin-cli/src/operator/status.rs]

**When to use:** Use after every manager `status()` result and also for manager errors so unavailable-manager remains inspectable instead of becoming a failed status command. [VERIFIED: .planning/phases/63-service-supervision-lifecycle/63-CONTEXT.md D-08; packages/open-bitcoin-cli/src/operator/status.rs:479]

**Example:**

```rust
// Source pattern: FieldAvailability in packages/open-bitcoin-node/src/status.rs.
fn service_lifecycle_from_snapshot(
    snapshot: &ServiceStateSnapshot,
) -> ServiceLifecycleStatus {
    match (snapshot.state, snapshot.maybe_enabled) {
        (ServiceLifecycleState::Unmanaged, _) => ServiceLifecycleStatus::Unmanaged,
        (ServiceLifecycleState::Running, _) => ServiceLifecycleStatus::Running,
        (ServiceLifecycleState::Failed, _) => ServiceLifecycleStatus::Failed,
        (_, Some(false)) => ServiceLifecycleStatus::Disabled,
        _ => ServiceLifecycleStatus::InstalledStopped,
    }
}
```

### Pattern 3: Extend The Existing Command Outcome Contract

**What:** Add request marker types and `ServiceManager::start`, `stop`, and `restart`, all returning `ServiceCommandOutcome`. [VERIFIED: packages/open-bitcoin-cli/src/operator/service.rs]

**When to use:** Use for both CLI and dashboard so action output includes the exact manager command string and no dashboard-only command path appears. [VERIFIED: packages/open-bitcoin-cli/src/operator/dashboard/action.rs; .planning/phases/63-service-supervision-lifecycle/63-CONTEXT.md D-02]

**Example:**

```rust
// Source pattern: ServiceEnableRequest/ServiceDisableRequest in service.rs.
pub struct ServiceStartRequest;
pub struct ServiceStopRequest;
pub struct ServiceRestartRequest;

pub trait ServiceManager {
    fn start(&self, request: &ServiceStartRequest)
        -> Result<ServiceCommandOutcome, ServiceError>;
    fn stop(&self, request: &ServiceStopRequest)
        -> Result<ServiceCommandOutcome, ServiceError>;
    fn restart(&self, request: &ServiceRestartRequest)
        -> Result<ServiceCommandOutcome, ServiceError>;
}
```

### Pattern 4: Resolve The Daemon Binary Once

**What:** Add a small pure helper that resolves `open-bitcoind` from the operator binary path, preferring a sibling binary and falling back to the literal command name. [VERIFIED: .planning/phases/63-service-supervision-lifecycle/63-CONTEXT.md D-04; packages/open-bitcoin-cli/src/operator/runtime.rs:199]

**When to use:** Use in both `OperatorCommand::Service` and `OperatorCommand::Dashboard`; do not call `current_exe()` directly at either call site for generated service definitions. [VERIFIED: packages/open-bitcoin-cli/src/operator/runtime.rs:199; packages/open-bitcoin-cli/src/operator/dashboard/mod.rs]

**Example:**

```rust
// Source decision: Phase 63 CONTEXT D-04.
fn resolve_service_daemon_binary(operator_binary_path: &Path) -> PathBuf {
    let Some(parent) = operator_binary_path.parent() else {
        return PathBuf::from("open-bitcoind");
    };
    let sibling = parent.join("open-bitcoind");
    if sibling.is_file() {
        return sibling;
    }
    PathBuf::from("open-bitcoind")
}
```

### Anti-Patterns to Avoid

- **Boolean-only service status:** Do not satisfy SVC-02 with only `installed/enabled/running`; Phase 63 requires one stable lifecycle label and richer evidence fields. [VERIFIED: .planning/REQUIREMENTS.md SVC-02; packages/open-bitcoin-node/src/status.rs:62]
- **Dashboard-only service actions:** Do not add dashboard shortcuts that bypass `execute_service_command`; the current dashboard already reuses that path. [VERIFIED: packages/open-bitcoin-cli/src/operator/dashboard/action.rs]
- **Live-manager default verification:** Do not make `scripts/verify.sh` call `launchctl`, `systemctl`, public peers, or live service starts. [VERIFIED: scripts/verify.sh; .planning/phases/63-service-supervision-lifecycle/63-CONTEXT.md D-14]
- **Source service mutation:** Do not disable, uninstall, or rewrite detected Bitcoin Core or Bitcoin Knots services. [VERIFIED: AGENTS.md; .planning/phases/63-service-supervision-lifecycle/63-CONTEXT.md D-03]
- **Production service wording:** Do not use "production service", "production full node", "packaged service guarantee", or "unattended production-node replacement" in generated output or docs. [VERIFIED: .planning/phases/63-service-supervision-lifecycle/63-CONTEXT.md D-13]

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Service lifecycle state | Ad hoc strings per renderer | Shared enum plus `FieldAvailability` | The status contract already models missing data explicitly and Phase 63 requires consistent labels. [VERIFIED: packages/open-bitcoin-node/src/status.rs; .planning/phases/63-service-supervision-lifecycle/63-CONTEXT.md D-05-D-06] |
| Platform command execution | Shell command strings | `std::process::Command::new(...).args(...)` | Existing adapters already avoid shell interpolation; paths and config values should not be shell-parsed. [VERIFIED: packages/open-bitcoin-cli/src/operator/service/launchd.rs; packages/open-bitcoin-cli/src/operator/service/systemd.rs] |
| Service manager fake | Real `launchctl`/`systemctl` in unit tests | `FakeServiceManager` and pure generation/parser tests | The fake manager records calls and avoids filesystem/subprocess side effects. [VERIFIED: packages/open-bitcoin-cli/src/operator/service/fake.rs; packages/open-bitcoin-cli/src/operator/service/tests.rs] |
| Launchd plist generation | Inline XML assembly in command handlers | Existing `generate_plist_content()` with XML escaping | The launchd adapter already has a pure generator and `xml_escape`. [VERIFIED: packages/open-bitcoin-cli/src/operator/service/launchd.rs] |
| Systemd unit generation | Inline unit text in command handlers | Existing `generate_unit_content()` with tests | The systemd adapter already has a pure generator and parser tests. [VERIFIED: packages/open-bitcoin-cli/src/operator/service/systemd.rs; packages/open-bitcoin-cli/src/operator/service/tests.rs] |
| Docs boundary enforcement | Manual review only | Bun checker integrated in `scripts/verify.sh` if service docs gain new exact claims | Phase 61 and Phase 62 use deterministic Bun checkers for docs/default-verification drift. [VERIFIED: scripts/check-phase61-resource-recovery-boundaries.ts; scripts/check-phase62-sync-truth-surfaces.ts; scripts/verify.sh] |

**Key insight:** The hard part is not starting a service; it is making status truthful when the manager is missing, disabled, failed, stopped, or platform-specific. [VERIFIED: .planning/phases/63-service-supervision-lifecycle/63-CONTEXT.md D-05-D-08; CITED: https://www.freedesktop.org/software/systemd/man/latest/systemctl.html]

## Common Pitfalls

### Pitfall 1: Confusing Enabled With Running

**What goes wrong:** A service is rendered as healthy because it is enabled, even though it is stopped, disabled, failed, or not loaded. [VERIFIED: packages/open-bitcoin-cli/src/operator/status.rs:493; CITED: https://www.freedesktop.org/software/systemd/man/latest/systemctl.html]

**Why it happens:** systemd explicitly treats enabling and starting as orthogonal, and the current Open Bitcoin shared service status only exposes booleans. [CITED: https://www.freedesktop.org/software/systemd/man/latest/systemctl.html; VERIFIED: packages/open-bitcoin-node/src/status.rs:62]

**How to avoid:** Add a lifecycle enum, keep `installed`, `enabled`, and `running` as separate evidence fields, and test every Phase 63 state. [VERIFIED: .planning/phases/63-service-supervision-lifecycle/63-CONTEXT.md D-05-D-06]

**Warning signs:** Tests assert only `running=true` or `installed=true` and never assert the rendered lifecycle label. [VERIFIED: packages/open-bitcoin-cli/src/operator/status/tests.rs]

### Pitfall 2: Treating Missing Manager As Command Failure Only

**What goes wrong:** `open-bitcoin status` cannot report service state on unsupported platforms or missing manager commands, so operators lose sync truth fields too. [VERIFIED: packages/open-bitcoin-cli/src/operator/status.rs:479; .planning/phases/63-service-supervision-lifecycle/63-CONTEXT.md D-08]

**Why it happens:** Action commands and inspection commands have different semantics; action failures can fail, but status inspection must still return a typed unavailable-manager state. [VERIFIED: .planning/phases/63-service-supervision-lifecycle/63-CONTEXT.md D-08]

**How to avoid:** Convert manager status errors into `ServiceLifecycleStatus::UnavailableManager` with unavailable reasons in shared status, while preserving nonzero outcomes for effectful start/stop/restart failures. [VERIFIED: packages/open-bitcoin-node/src/status.rs; packages/open-bitcoin-cli/src/operator/status.rs]

**Warning signs:** `collect_service_status()` returns all service fields unavailable with the same generic reason and no lifecycle label. [VERIFIED: packages/open-bitcoin-cli/src/operator/status.rs:507]

### Pitfall 3: Generating A Service For The Wrong Binary

**What goes wrong:** The installed service supervises `open-bitcoin` instead of `open-bitcoind`. [VERIFIED: packages/open-bitcoin-cli/src/operator/runtime.rs:199; .planning/phases/63-service-supervision-lifecycle/63-CONTEXT.md D-04]

**Why it happens:** `current_exe()` returns the running operator binary during `open-bitcoin service install`. [VERIFIED: packages/open-bitcoin-cli/src/operator/runtime.rs:199]

**How to avoid:** Add and test `resolve_service_daemon_binary()`, then use it for CLI service commands and dashboard service actions. [VERIFIED: .planning/phases/63-service-supervision-lifecycle/63-CONTEXT.md D-04; packages/open-bitcoin-cli/src/operator/dashboard/mod.rs]

**Warning signs:** Generated plist/unit content contains `/open-bitcoin` but not `/open-bitcoind`. [VERIFIED: packages/open-bitcoin-cli/src/operator/service/tests.rs]

### Pitfall 4: Parsing Platform Human Output As A Stable API

**What goes wrong:** Service status breaks after OS updates because parsers depend on unstable human output. [VERIFIED: local man launchctl; CITED: https://www.freedesktop.org/software/systemd/man/latest/systemctl.html]

**Why it happens:** local `launchctl` man page states `print` output is not API and legacy output is not intended for automation; systemd docs state unit state sets can change between releases. [VERIFIED: local man launchctl; CITED: https://www.freedesktop.org/software/systemd/man/latest/systemctl.html]

**How to avoid:** Keep parsers narrow, prefer specific predicates (`systemctl --user is-active`, `is-enabled`, `is-failed`) where available, and map unknown/unparseable output to diagnostics plus `Unavailable` instead of inventing success. [CITED: https://www.freedesktop.org/software/systemd/man/latest/systemctl.html; VERIFIED: packages/open-bitcoin-cli/src/operator/service/systemd.rs]

**Warning signs:** Parser tests assert large raw `launchctl print` fixtures or treat unknown stdout as running. [VERIFIED: packages/open-bitcoin-cli/src/operator/service/tests.rs]

### Pitfall 5: Broadening Phase 63 Into Restart Proof

**What goes wrong:** Plans add service-supervised restart/resume tests or public-network service checks as default verification. [VERIFIED: .planning/phases/63-service-supervision-lifecycle/63-CONTEXT.md; .planning/ROADMAP.md Phase 64]

**Why it happens:** Restart/recovery language overlaps with service lifecycle actions. [VERIFIED: .planning/ROADMAP.md Phase 63 and Phase 64]

**How to avoid:** Keep Phase 63 proof to command surfaces, generated definitions, lifecycle state mapping, docs, and deterministic checks; leave same-datadir service restart evidence to Phase 64. [VERIFIED: .planning/phases/63-service-supervision-lifecycle/63-CONTEXT.md D-14; .planning/ROADMAP.md Phase 64]

**Warning signs:** A Phase 63 plan invokes `run-live-mainnet-smoke`, `--restart-after-progress`, public peers, or live service start in `scripts/verify.sh`. [VERIFIED: scripts/verify.sh; scripts/check-phase62-sync-truth-surfaces.ts]

## Code Examples

Verified patterns from repo and platform sources:

### Service Command Dispatch

```rust
// Source: packages/open-bitcoin-cli/src/operator/service.rs.
match &args.command {
    ServiceCommand::Preview => {
        let request = ServiceInstallRequest {
            binary_path,
            data_dir,
            maybe_config_path,
            maybe_log_path,
            apply: false,
        };
        manager.install(&request)
    }
    ServiceCommand::Start => manager.start(&ServiceStartRequest),
    ServiceCommand::Stop => manager.stop(&ServiceStopRequest),
    ServiceCommand::Restart => manager.restart(&ServiceRestartRequest),
    _ => existing_dispatch(args),
}
```

### Systemd User Commands

```rust
// Source: freedesktop systemctl(1) and existing SystemdAdapter command style.
fn systemd_start_commands() -> Vec<String> {
    vec!["systemctl --user start open-bitcoin-node.service".to_string()]
}

fn systemd_restart_commands() -> Vec<String> {
    vec!["systemctl --user restart open-bitcoin-node.service".to_string()]
}
```

### Launchd User Commands

```rust
// Source: local man launchctl and existing LaunchdAdapter helpers.
fn launchd_start_command(uid: u32, plist_path: &Path) -> String {
    format!(
        "launchctl bootstrap gui/{uid} {}",
        plist_path.display()
    )
}

fn launchd_restart_command(uid: u32) -> String {
    format!("launchctl kickstart -k gui/{uid}/org.open-bitcoin.node")
}
```

### Status Projection

```rust
// Source: packages/open-bitcoin-node/src/status.rs FieldAvailability pattern.
ServiceStatus {
    manager: FieldAvailability::available(manager_name.to_string()),
    lifecycle: FieldAvailability::available(service_lifecycle_from_snapshot(&snapshot)),
    installed: FieldAvailability::available(installed),
    enabled: snapshot
        .maybe_enabled
        .map(FieldAvailability::available)
        .unwrap_or_else(|| FieldAvailability::unavailable("service manager did not report enablement")),
    running: FieldAvailability::available(matches!(snapshot.state, ServiceLifecycleState::Running)),
    service_file_path: path_availability(snapshot.maybe_service_file_path.as_ref()),
    log_path: path_or_unavailable(
        snapshot.maybe_log_path.as_ref(),
        snapshot.maybe_log_path_unavailable_reason.as_deref(),
    ),
    diagnostics: diagnostics_availability(snapshot.maybe_manager_diagnostics.as_deref()),
}
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| `launchctl load` / `unload` service lifecycle | `launchctl bootstrap`, `bootout`, `enable`, `disable`, and `kickstart -k` service targets | Current local macOS man page recommends `bootstrap`/`bootout`/`enable`/`disable` alternatives for legacy load/unload and documents `kickstart -k`. [VERIFIED: local man launchctl] | Plan launchd actions around `gui/<uid>/org.open-bitcoin.node` targets and keep legacy output parsing minimal. [VERIFIED: packages/open-bitcoin-cli/src/operator/service/launchd.rs] |
| Treating systemd enablement as process state | Separate `enable`/`disable` from `start`/`stop`/`restart` | systemd 260.1 docs state enabling and starting are orthogonal. [CITED: https://www.freedesktop.org/software/systemd/man/latest/systemctl.html] | Status must represent enabled/disabled and running/stopped independently. [VERIFIED: .planning/phases/63-service-supervision-lifecycle/63-CONTEXT.md D-05-D-06] |
| `systemctl status` parsing for automation | Specific commands such as `is-active`, `is-enabled`, and optional `show` properties | systemd docs mark `status` as human-readable and recommend `show` for computer-parsable output. [CITED: https://www.freedesktop.org/software/systemd/man/latest/systemctl.html] | Use narrow command outputs and treat unknown states as diagnostics. [VERIFIED: packages/open-bitcoin-cli/src/operator/service/systemd.rs] |
| Launchd/systemd live checks in default verification | Pure string generation, fake managers, parser fixtures, docs/checker assertions | v1.5 requirements keep public-network and service checks opt-in UAT. [VERIFIED: .planning/REQUIREMENTS.md REL-03; scripts/verify.sh] | Phase completion remains deterministic on macOS and Linux hosts. [VERIFIED: .planning/phases/63-service-supervision-lifecycle/63-CONTEXT.md D-14] |

**Deprecated/outdated:**
- `launchctl load`/`unload` should not be the Phase 63 primary implementation path because the local man page recommends `bootstrap`/`bootout`/`enable`/`disable` as alternatives. [VERIFIED: local man launchctl]
- Service status booleans without a lifecycle label are insufficient for Phase 63 because SVC-02 requires `unmanaged`, `installed-stopped`, `running`, `failed`, `disabled`, and `unavailable-manager`. [VERIFIED: .planning/REQUIREMENTS.md SVC-02]

## Assumptions Log

All claims in this research were verified or cited in this session. No `[ASSUMED]` claims are intentionally present. [VERIFIED: source list below]

## Open Questions (RESOLVED)

1. **Exact `service preview` CLI behavior**
   - What we know: Phase 63 requires a discoverable side-effect-free preview path and preserves `install` dry-run behavior. [VERIFIED: .planning/phases/63-service-supervision-lifecycle/63-CONTEXT.md D-01]
   - What's unclear: The context allows either adding or documenting the preview path; it does not mandate whether `service preview --apply` should be rejected or ignored. [VERIFIED: .planning/phases/63-service-supervision-lifecycle/63-CONTEXT.md D-01]
   - Recommendation: Add `open-bitcoin service preview` as a first-class command that always dry-runs and rejects `--apply` with a clear message. [VERIFIED: packages/open-bitcoin-cli/src/operator.rs]
   - RESOLVED: service preview --apply behavior: first-class preview rejects --apply.

2. **Shared `ServiceStatus` additive field names**
   - What we know: Required fields are service manager, installed, enabled, running, log path, service file path, diagnostics, and unavailable reasons. [VERIFIED: .planning/phases/63-service-supervision-lifecycle/63-CONTEXT.md D-06]
   - What's unclear: The context does not prescribe exact Rust field names for `service_file_path`, `log_path`, or `diagnostics`. [VERIFIED: .planning/phases/63-service-supervision-lifecycle/63-CONTEXT.md D-06]
   - Recommendation: Use explicit `FieldAvailability<String>` fields named `lifecycle`, `service_file_path`, `log_path`, and `diagnostics` to match existing status JSON style. [VERIFIED: packages/open-bitcoin-node/src/status.rs]
   - RESOLVED: ServiceStatus field names: lifecycle, service_file_path, log_path, diagnostics.

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|-------------|-----------|---------|----------|
| Rust `rustc` | Rust implementation and tests | yes | 1.94.1 | None needed. [VERIFIED: command `rustc --version`; rust-toolchain.toml] |
| Cargo | Rust build/test commands | yes | 1.94.1 | None needed. [VERIFIED: command `cargo --version`] |
| cargo-llvm-cov | `scripts/verify.sh` coverage gate | yes | 0.8.5 | None needed. [VERIFIED: command `cargo llvm-cov --version`; scripts/verify.sh] |
| Bun | TypeScript checkers | yes | 1.3.9 | None needed. [VERIFIED: command `bun --version`; .bun-version] |
| Bazel/Bazelisk | Bazel smoke build and UAT commands | yes | Bazelisk 1.28.1, Bazel 8.6.0 | None needed. [VERIFIED: commands `bazelisk version`, `bazel version`; scripts/verify.sh] |
| launchctl | Optional macOS launchd UAT and local man verification | yes | Darwin Bootstrapper 7.0.0 | Use deterministic tests for default verification. [VERIFIED: command `launchctl version`; local man launchctl] |
| systemctl | Optional Linux systemd UAT | no on this macOS host | unavailable | Use deterministic pure unit generation and fake adapter tests locally; run Linux manual UAT only on a Linux host. [VERIFIED: command `command -v systemctl`; packages/open-bitcoin-cli/src/operator/service/systemd.rs] |
| Public network | Optional service-based public-mainnet UAT | not required | not probed | Keep out of default verification. [VERIFIED: .planning/phases/63-service-supervision-lifecycle/63-CONTEXT.md D-14; scripts/verify.sh] |

**Missing dependencies with no fallback:**
- None for default Phase 63 planning and deterministic verification. [VERIFIED: scripts/verify.sh; environment probes]

**Missing dependencies with fallback:**
- `systemctl` is absent locally; Linux service behavior should be covered by deterministic systemd adapter tests and optional UAT on a Linux host. [VERIFIED: command `command -v systemctl`; packages/open-bitcoin-cli/src/operator/service/tests.rs]

## Security Domain

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|------------------|
| V2 Authentication | no | Phase 63 does not change RPC authentication or wallet auth flows. [VERIFIED: .planning/phases/63-service-supervision-lifecycle/63-CONTEXT.md; packages/open-bitcoin-cli/src/operator/service.rs] |
| V3 Session Management | no | Phase 63 does not add web sessions or cookies. [VERIFIED: .planning/phases/63-service-supervision-lifecycle/63-CONTEXT.md] |
| V4 Access Control | yes | Keep services user-level only and avoid sudo/global units. [VERIFIED: .planning/phases/63-service-supervision-lifecycle/63-CONTEXT.md D-09; CITED: https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html; CITED: https://developer.apple.com/library/archive/documentation/MacOSX/Conceptual/BPSystemStartup/Chapters/CreatingLaunchdJobs.html] |
| V5 Input Validation | yes | Use typed `PathBuf` inputs, XML escaping for launchd plist strings, systemd unit quoting tests, and no shell interpolation. [VERIFIED: packages/open-bitcoin-cli/src/operator/service/launchd.rs; packages/open-bitcoin-cli/src/operator/service/systemd.rs] |
| V6 Cryptography | no | Phase 63 does not add cryptography or key handling. [VERIFIED: .planning/phases/63-service-supervision-lifecycle/63-CONTEXT.md] |
| V7 Error Handling and Logging | yes | Preserve diagnostics and unavailable reasons; route service stdout/stderr to the configured service log path when available. [VERIFIED: .planning/phases/63-service-supervision-lifecycle/63-CONTEXT.md D-06,D-10; local man launchd.plist; CITED: https://www.freedesktop.org/software/systemd/man/latest/systemd.exec.html] |
| V10 Malicious Code | yes | Generated service definitions must target `open-bitcoind`, not arbitrary source services, and must not mutate Core/Knots services. [VERIFIED: .planning/phases/63-service-supervision-lifecycle/63-CONTEXT.md D-03,D-04] |

### Known Threat Patterns for Rust Service Adapters

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Operator-controlled paths interpreted by a shell | Tampering, Elevation of Privilege | Use `std::process::Command` with separate args for manager commands and pure string generation tests for service files. [VERIFIED: packages/open-bitcoin-cli/src/operator/service/launchd.rs; packages/open-bitcoin-cli/src/operator/service/systemd.rs] |
| Service definition points at the wrong executable | Tampering, Repudiation | Use a tested `open-bitcoind` resolver and assert generated plist/unit content contains the daemon binary. [VERIFIED: .planning/phases/63-service-supervision-lifecycle/63-CONTEXT.md D-04; packages/open-bitcoin-cli/src/operator/runtime.rs] |
| Missing manager reported as success | Repudiation | Model `unavailable-manager` and unavailable reasons explicitly in `ServiceStatus`. [VERIFIED: .planning/phases/63-service-supervision-lifecycle/63-CONTEXT.md D-08; packages/open-bitcoin-node/src/status.rs] |
| Service actions mutate source Core/Knots services | Tampering, Elevation of Privilege | Limit writes to Open Bitcoin user-level service file paths and preserve migration/source service read-only posture. [VERIFIED: .planning/phases/63-service-supervision-lifecycle/63-CONTEXT.md D-03,D-09; AGENTS.md] |
| Public-network service checks block default verification | Denial of Service | Keep live service/public-network checks optional UAT and enforce default `scripts/verify.sh` exclusion. [VERIFIED: .planning/REQUIREMENTS.md REL-03; scripts/verify.sh] |

## Sources

### Primary (HIGH confidence)

- `.planning/phases/63-service-supervision-lifecycle/63-CONTEXT.md` - User decisions D-01 through D-14, discretion, deferred scope, canonical refs. [VERIFIED: file read]
- `.planning/REQUIREMENTS.md` - SVC-01, SVC-02, SVC-04, REL-03, out-of-scope boundaries. [VERIFIED: file read]
- `.planning/ROADMAP.md` - Phase 63 and Phase 64 boundaries and success criteria. [VERIFIED: file read]
- `.planning/STATE.md` - v1.5 deterministic verification and production-claim decisions. [VERIFIED: file read]
- `AGENTS.md`, `AGENTS.bright-builds.md`, `standards-overrides.md` - repo guidance and Bright Builds routing. [VERIFIED: file read]
- Bright Builds pinned standards at commit `05f8d7a6c9c2e157ec4f922a05273e72dab97676` - architecture, code shape, verification, testing, Rust. [CITED: https://raw.githubusercontent.com/bright-builds-llc/bright-builds-rules/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/index.md]
- `packages/open-bitcoin-cli/src/operator/service.rs` and `packages/open-bitcoin-cli/src/operator/service/*` - existing service contract, adapters, fake manager, tests. [VERIFIED: codebase grep/read]
- `packages/open-bitcoin-node/src/status.rs` - shared status and `FieldAvailability` contract. [VERIFIED: codebase grep/read]
- `packages/open-bitcoin-cli/src/operator/runtime.rs`, `status.rs`, `status/render.rs`, and dashboard modules - service/status/dashboard integration points. [VERIFIED: codebase grep/read]
- Local macOS `man launchctl` and `man launchd.plist` - current `launchctl` target syntax, bootstrap/bootout/kickstart, plist keys, user LaunchAgents, and logging keys. [VERIFIED: local man pages]
- freedesktop systemd `systemctl(1)` 260.1 - start/stop/restart, is-active, is-enabled, `--user`, status caveats, exit statuses. [CITED: https://www.freedesktop.org/software/systemd/man/latest/systemctl.html]
- freedesktop systemd `systemd.unit(5)` 260.1 - user unit search path and `WantedBy`. [CITED: https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html]
- freedesktop systemd `systemd.exec(5)` 260.1 - `StandardOutput=append:` and `StandardError=` semantics. [CITED: https://www.freedesktop.org/software/systemd/man/latest/systemd.exec.html]
- freedesktop systemd `systemd.service(5)` 260.1 - `ExecStart=`, restart behavior, and `Restart=on-failure`. [CITED: https://www.freedesktop.org/software/systemd/man/latest/systemd.service.html]
- Apple archived "Creating Launch Daemons and Agents" - user LaunchAgents directory, `ProgramArguments`, `KeepAlive`, `StandardOutPath`, `StandardErrorPath`, SIGTERM behavior. [CITED: https://developer.apple.com/library/archive/documentation/MacOSX/Conceptual/BPSystemStartup/Chapters/CreatingLaunchdJobs.html]

### Secondary (MEDIUM confidence)

- Xcode man page mirror for `launchctl(1)` and `launchd.plist(5)` corroborated local man page content. [CITED: https://keith.github.io/xcode-man-pages/launchctl.1.html; CITED: https://keith.github.io/xcode-man-pages/launchd.plist.5.html]

### Tertiary (LOW confidence)

- None. [VERIFIED: source list above]

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - versions are pinned or resolved in repo files and local tool probes. [VERIFIED: rust-toolchain.toml; packages/Cargo.toml; packages/Cargo.lock; .bun-version; command probes]
- Architecture: HIGH - current service/status/dashboard code directly exposes the extension points. [VERIFIED: packages/open-bitcoin-cli/src/operator/service.rs; packages/open-bitcoin-cli/src/operator/status.rs; packages/open-bitcoin-cli/src/operator/dashboard/action.rs]
- Pitfalls: HIGH for repo-specific pitfalls and platform command semantics, MEDIUM for launchd output stability because local man pages warn that output is not API and platform versions can vary. [VERIFIED: local man launchctl; CITED: https://www.freedesktop.org/software/systemd/man/latest/systemctl.html]

**Research date:** 2026-06-07
**Valid until:** 2026-07-07 for repo/code findings; re-check systemd/launchd docs before implementation if platform commands or output parsing are changed. [VERIFIED: current_date; CITED: https://www.freedesktop.org/software/systemd/man/latest/systemctl.html]
