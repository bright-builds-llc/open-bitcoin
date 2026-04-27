# Phase 18: Service Lifecycle Integration - Research

**Researched:** 2026-04-26
**Domain:** macOS launchd / Linux systemd service lifecycle in Rust
**Confidence:** HIGH

---

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

- **D-01:** `operator/service/` submodule with `ServiceManager` trait, `LaunchdAdapter`, `SystemdAdapter`, `FakeServiceManager`.
- **D-02:** Pure content generators (plist, unit file, state classification) in the submodule; effectful writes/subprocess calls in platform adapters.
- **D-03:** Wire `OperatorCommand::Service` dispatch in `operator/runtime.rs` — replace current "deferred to Phase 18" stub.
- **D-04:** No template engines. Pure Rust string/serialization. Each generator exposes `dry_run_content() -> String`.
- **D-05:** macOS → `~/Library/LaunchAgents/org.open-bitcoin.node.plist`. Linux → `~/.config/systemd/user/open-bitcoin-node.service`.
- **D-06:** Generated content includes daemon binary path, data directory, config path, log path, recovery behavior, and a comment header.
- **D-07:** `#[cfg(target_os)]` compile-time selection. `platform_service_manager()` factory returns box. Tests inject `FakeServiceManager`.
- **D-08:** Unsupported platform returns `ServiceError::UnsupportedPlatform` — no panics.
- **D-09:** User-level scope: LaunchAgents on macOS, `systemctl --user` on Linux.
- **D-10:** No system-scope flag in this phase. Interface must allow adding `--system` later without breaking user-level.
- **D-11:** Install and Uninstall are dry-run by default. `--apply` required to mutate filesystem.
- **D-12:** Enable and Disable print the invocations they would run; no `--apply` required since no files are written.
- **D-13:** Privilege requirements surfaced in dry-run/status output before any action.
- **D-14:** Status classifies: unmanaged, installed, enabled, running, failed, stopped — mapped to `ServiceStatus { manager, installed, enabled, running }` via `FieldAvailability<T>`.
- **D-15:** `collect_status_snapshot` wires service adapter as optional injection; falls back to `unavailable("service manager not inspected")`.
- **D-16:** `open-bitcoin service status` shows: manager, service file path, enabled, running, recent diagnostics, log path link.
- **D-17:** Tests use `FakeServiceManager`. No test invokes `launchctl`/`systemctl`.
- **D-18:** Integration tests write to isolated temp dirs; never touch `~/Library/LaunchAgents/` or `~/.config/systemd/user/`.
- **D-19:** At least one test per command covers dry-run vs apply, privilege-surfacing, and typed error paths.

### Claude's Discretion

- Adapter struct naming and internal helper names.
- `--apply` or `--execute` flag name (choose what reads best in help text).
- systemd `[Install]` section `WantedBy=` target.
- Double-install guard: return typed error rather than silently overwriting; `--force`/`--reinstall` is optional but must be typed and documented.

### Deferred Ideas (OUT OF SCOPE)

- System-level service scope (LaunchDaemons, system systemd).
- Ratatui dashboard service state panels (Phase 19).
- Service restart count metric collection wire-up beyond what Phase 16 already defined.
- Windows service integration.
- Socket-activation or launch-on-demand variants.
</user_constraints>

---

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| SVC-01 | macOS launchd install/uninstall/enable/disable/status with dry-run plist preview | Launchd plist format, `launchctl` commands, file paths, dry-run pattern |
| SVC-02 | Linux systemd install/uninstall/enable/disable/status with dry-run unit preview | systemd unit file format, `systemctl --user` commands, file paths, dry-run pattern |
| SVC-03 | Service commands surface privilege requirements, scope, paths, daemon command, config path, log path, recovery behavior before applying | Dry-run output structure, privilege detection |
| SVC-04 | Service status reports installed/enabled/running/failed/stopped/unmanaged with log/diagnostic links | `ServiceStatus` mapping, launchctl/systemctl query commands |
| SVC-05 | Tests use isolated temp paths or fake managers — never modify real developer state | `FakeServiceManager` + `DetectionRoots`-style temp dir injection pattern |
</phase_requirements>

---

## Summary

Phase 18 introduces a `ServiceManager` trait and two platform adapters that manage the Open Bitcoin daemon as a supervised process under macOS launchd (user-level `LaunchAgents`) and Linux systemd (user scope). All write-affecting commands are dry-run by default; `--apply` unlocks mutations. Plist and unit file bodies are generated from typed Rust structs using pure string assembly — no external template dependencies. The existing `ServiceStatus` model in `open-bitcoin-node/src/status.rs` already provides the shared contract; Phase 18 fills in the previously `unavailable` fields.

The codebase has several directly reusable assets: `ServiceCommand` enum (Status/Install/Uninstall/Enable/Disable) and `ServiceArgs` already exist in `operator.rs` and need no new clap definitions. The `DetectionRoots`-style injection pattern from `detect.rs` is the canonical model for test isolation. The `OperatorCommandOutcome::success / ::failure` contract is the return type for all service handlers.

The primary challenge is correct parity between what the dry-run output promises and what `--apply` actually executes. The planner should structure work as: (1) pure generators + trait definition, (2) launchd adapter, (3) systemd adapter, (4) runtime wiring, (5) `collect_status_snapshot` integration, with tests accompanying each layer.

**Primary recommendation:** Build the `ServiceManager` trait and pure generators first (no I/O), then layer adapters on top, then wire the runtime and status snapshot last. This keeps every layer testable before the next is started.

---

## Standard Stack

### Core

| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| `std::process::Command` | stdlib | Spawn `launchctl`/`systemctl` subprocess calls | No external dep needed for single-command invocations |
| `std::fs` | stdlib | Write plist/unit file to target path | Thin imperative shell; pure generator handles content |
| `#[cfg(target_os)]` attributes | Rust compiler | Compile-time platform selection | Project pattern per D-07 |

### Supporting

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| `std::env::current_exe()` | stdlib | Resolve daemon binary path for generated content | Inside `LaunchAgents` / unit `ExecStart` |
| `tempfile` crate | existing in workspace | Isolated temp dirs in integration tests | Already used elsewhere in the workspace [ASSUMED] |

[VERIFIED: codebase grep] `std::env::temp_dir()` is used in `detect/tests.rs` for test isolation — `tempfile` crate may not be a dep yet; fall back to `std::env::temp_dir()` + manual `fs::create_dir_all` as shown in detect/tests.rs.

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Pure string assembly for plist | `plist` crate | `plist` crate adds a dep; Apple plist for LaunchAgents is simple enough that hand-written XML avoids it |
| Pure string assembly for systemd unit | `ini` or `configparser` | systemd unit format is simple key=value with sections; no parser needed for generation |

**Installation:** No new Cargo dependencies required for this phase. All functionality uses `std`.

---

## Architecture Patterns

### Recommended Project Structure

```
packages/open-bitcoin-cli/src/operator/
├── service.rs               # ServiceManager trait, ServiceError, execute_service_command()
└── service/
    ├── launchd.rs           # LaunchdAdapter: plist generator + launchctl shell
    ├── systemd.rs           # SystemdAdapter: unit generator + systemctl shell
    ├── fake.rs              # FakeServiceManager for tests
    └── tests.rs             # Integration tests with temp dirs
```

This mirrors the existing `status.rs` + `status/` convention already in the codebase. [VERIFIED: codebase inspection]

### Pattern 1: ServiceManager Trait (Functional Core / Imperative Shell)

**What:** A trait with five methods matching `ServiceCommand`. Pure content generators (plist string, unit file string) live as free functions or associated methods in the same module with no I/O. The adapter implements the trait by calling the pure generator then writing or invoking.

**When to use:** All service command dispatch.

**Example (trait definition):**

```rust
// Source: project convention (functional core / imperative shell)
pub trait ServiceManager {
    fn install(&self, request: &ServiceInstallRequest) -> Result<ServiceCommandOutcome, ServiceError>;
    fn uninstall(&self, request: &ServiceUninstallRequest) -> Result<ServiceCommandOutcome, ServiceError>;
    fn enable(&self, request: &ServiceEnableRequest) -> Result<ServiceCommandOutcome, ServiceError>;
    fn disable(&self, request: &ServiceDisableRequest) -> Result<ServiceCommandOutcome, ServiceError>;
    fn status(&self) -> Result<ServiceStateSnapshot, ServiceError>;
}
```

### Pattern 2: Dry-Run by Default

**What:** Every mutating method (install, uninstall) accepts an `apply: bool` field in its request struct. When `apply == false`, the method generates content and target paths, then returns them in the outcome without touching the filesystem. The caller renders the outcome to stdout.

**When to use:** Install and Uninstall handlers. Enable/Disable differ: they print the command that would run but execute it without requiring `--apply`.

**Example (install request struct):**

```rust
pub struct ServiceInstallRequest {
    pub binary_path: PathBuf,
    pub data_dir: PathBuf,
    pub config_path: Option<PathBuf>,
    pub log_path: Option<PathBuf>,
    pub apply: bool,
}
```

### Pattern 3: Clap Flag Addition to ServiceArgs

**What:** `ServiceArgs` in `operator.rs` needs a `--apply` flag (or `--execute`) added so install and uninstall commands can distinguish dry-run from live mode.

**Current state (from codebase):**

```rust
// packages/open-bitcoin-cli/src/operator.rs — current
pub struct ServiceArgs {
    #[command(subcommand)]
    pub command: ServiceCommand,
}
```

**Required change:** Add `#[arg(long = "apply")]  pub apply: bool` to `ServiceArgs`. The flag is then passed down to the install/uninstall request. [VERIFIED: codebase inspection — ServiceArgs currently has only `command`]

### Pattern 4: Plist Content Generation (macOS)

**What:** A pure function that takes binary path, label, data dir, config path, log path, and returns a `String` of valid XML plist content. No I/O.

**Example plist template (pure output):**

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN"
    "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<!-- Generated by Open Bitcoin. Do not edit manually. -->
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>org.open-bitcoin.node</string>
    <key>ProgramArguments</key>
    <array>
        <string>/path/to/open-bitcoin</string>
        <string>--datadir</string>
        <string>/path/to/datadir</string>
    </array>
    <key>KeepAlive</key>
    <true/>
    <key>RunAtLoad</key>
    <true/>
    <key>StandardOutPath</key>
    <string>/path/to/logs/open-bitcoin.log</string>
    <key>StandardErrorPath</key>
    <string>/path/to/logs/open-bitcoin-error.log</string>
</dict>
</plist>
```

[CITED: https://developer.apple.com/library/archive/documentation/MacOSX/Conceptual/BPSystemStartup/Chapters/CreatingLaunchdJobs.html]

### Pattern 5: Systemd Unit File Generation (Linux)

**What:** A pure function returning a `String` of systemd user unit content.

**Example unit (pure output):**

```ini
# Generated by Open Bitcoin. Do not edit manually.
[Unit]
Description=Open Bitcoin Node
After=network.target

[Service]
ExecStart=/path/to/open-bitcoin --datadir /path/to/datadir
Restart=on-failure
StandardOutput=journal
StandardError=journal

[Install]
WantedBy=default.target
```

[CITED: https://www.freedesktop.org/software/systemd/man/systemd.service.html]

### Pattern 6: Platform Factory

**What:** A `platform_service_manager()` function that returns a `Box<dyn ServiceManager>`. Under `#[cfg(target_os = "macos")]` it returns `LaunchdAdapter`; under `#[cfg(target_os = "linux")]` it returns `SystemdAdapter`; otherwise it returns an error at call time (not at compile time, so unsupported platforms can still compile).

**Why compile-time selection but runtime error:** The adapters themselves compile only on their target platform (they reference platform-specific paths/invocations), but a graceful runtime error for unsupported platforms is still needed for the CLI entry path.

### Pattern 7: FakeServiceManager (Test Isolation)

**What:** A `FakeServiceManager` struct that records calls in a `Vec<FakeServiceCall>` and returns deterministic `ServiceStateSnapshot` values. No subprocess invocations.

**Example:**

```rust
pub struct FakeServiceManager {
    pub recorded_calls: RefCell<Vec<FakeServiceCall>>,
    pub status_to_return: ServiceStateSnapshot,
}
```

This mirrors the `FakeServiceManager` concept already described in CONTEXT.md and aligns with the `DetectionRoots` injection pattern used in `detect/tests.rs`. [VERIFIED: detect/tests.rs]

### Pattern 8: Runtime Dispatch Replacement

**What:** In `execute_operator_cli_inner` in `runtime.rs`, the current service match arm returns `OperatorCommandOutcome::failure("service lifecycle commands are deferred to Phase 18")`. Phase 18 replaces this with a call to `execute_service_command(service, &cli, config_resolution)`.

**Key wiring in `detection_roots()`:** Currently `service_dirs: Vec::new()`. Phase 18 should populate this with the platform-appropriate service directory (e.g. `home_dir.join("Library/LaunchAgents")` on macOS) so the detection pass can also find the Open Bitcoin service file.

### Anti-Patterns to Avoid

- **Invoking `launchctl`/`systemctl` without checking for dry-run mode first:** Every adapter method must check apply mode before any subprocess call.
- **Hardcoding home directory:** Use `std::env::var_os("HOME")` or `std::env::home_dir()` (deprecated but still functional; prefer `HOME` env var which the project already uses in `runtime.rs`).
- **Swallowing subprocess errors:** If `launchctl load` returns a non-zero exit, surface it as a typed `ServiceError::ManagerCommandFailed { exit_code, stderr }`.
- **Using a single `ServiceError` string variant:** Each error case (unsupported platform, already installed, write failed, manager command failed) needs its own variant so test assertions are exact.
- **Writing new Rust source files without parity breadcrumbs:** Every new `.rs` file under `packages/open-bitcoin-cli/src/` needs a breadcrumb block and a `docs/parity/source-breadcrumbs.json` entry. New service files have no direct Knots anchor — use the explicit `none` breadcrumb. [VERIFIED: AGENTS.md, source-breadcrumbs.json]

---

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| macOS plist format | Custom XML serializer | `format!()` with the fixed Apple plist schema | LaunchAgent plists have a small fixed schema; the format has not changed in a decade [CITED: Apple docs] |
| systemd unit format | Custom ini parser | `format!()` with the fixed [Unit]/[Service]/[Install] sections | Unit generation only writes; no parsing needed |
| Temp dir test cleanup | Custom RAII wrapper | Pattern from `detect/tests.rs` (struct with Drop impl) | Already implemented in the project — copy the `TestDirectory` struct |
| Subprocess timeout/retry | Custom async wrapper | Simple blocking `std::process::Command::output()` | `launchctl`/`systemctl` calls return immediately; no retry needed for happy path |

**Key insight:** Service file generation and subprocess invocation are deceptively simple once the dry-run / apply split is enforced. The complexity lives in state classification (running vs failed vs stopped vs unmanaged) where `launchctl print` and `systemctl --user is-active` return different output formats.

---

## Common Pitfalls

### Pitfall 1: launchctl Output Parsing Brittleness

**What goes wrong:** `launchctl print` output format differs between macOS versions and is not documented as stable. Parsing it for `state = running` breaks on older macOS.
**Why it happens:** launchctl does not have a machine-readable JSON output mode for `print` on all macOS versions.
**How to avoid:** For status, prefer `launchctl list org.open-bitcoin.node` (which returns exit code 0 if loaded, non-zero if not) plus checking whether the plist file exists — together these give installed + loaded. For running state, `launchctl list | grep org.open-bitcoin.node` and checking the PID column (non-zero means running) is more stable.
**Warning signs:** If status tests only pass on one macOS version, the parsing approach is too brittle.
[ASSUMED: based on general knowledge of launchctl behavior — verify against current macOS docs before implementation]

### Pitfall 2: systemctl --user Requires DBUS_SESSION_BUS_ADDRESS

**What goes wrong:** On headless Linux servers and CI environments, `systemctl --user` fails with "Failed to connect to bus: No such file or directory" because the user D-Bus session is not running.
**Why it happens:** User systemd requires an active login session's D-Bus socket.
**How to avoid:** In tests, never call real `systemctl`. In real adapter, surface the D-Bus error as `ServiceError::ManagerCommandFailed` with the raw stderr. Document in status output that the user session must be active.
**Warning signs:** CI test failures on Linux for any test that reaches real `systemctl`.
[CITED: https://www.freedesktop.org/software/systemd/man/systemctl.html — user instance requirements]

### Pitfall 3: `std::env::current_exe()` Path Under Cargo Test

**What goes wrong:** In tests, `current_exe()` returns the test binary path, not the production `open-bitcoin` binary path. Generated plist/unit content in tests will reference a test-runner path.
**How to avoid:** The `ServiceInstallRequest` struct should accept an explicit `binary_path: PathBuf` rather than calling `current_exe()` inside the generator. The caller (runtime.rs) resolves the path and passes it in. Test callers pass a fake path.
**Warning signs:** Tests showing `/cargo-test-runner` or similar in snapshot assertions.

### Pitfall 4: Double-Install Guard Missing

**What goes wrong:** Running `install --apply` twice corrupts or silently replaces a working service file.
**How to avoid:** Before writing, check whether the target plist/unit file already exists. If yes, return `ServiceError::AlreadyInstalled { path }`. Make the guard explicit in the dry-run output too: if the file exists, dry-run output should say "already installed at <path> — use --force to reinstall" rather than showing a clean install preview.
**Warning signs:** Integration tests that install twice and don't assert on the second call's error.

### Pitfall 5: ServiceArgs Missing `--apply` Flag

**What goes wrong:** The existing `ServiceArgs` struct in `operator.rs` only has `command: ServiceCommand`. Without adding `--apply`, there is no CLI path to actually write files.
**How to avoid:** Add `#[arg(long = "apply")]  pub apply: bool` to `ServiceArgs` before implementing any adapter. The clap derive will propagate it automatically.
**Warning signs:** All service command tests returning dry-run output even when `--apply` is passed.

### Pitfall 6: service_dirs Not Populated in detection_roots()

**What goes wrong:** `open-bitcoin status` does not show service state because `detection_roots()` leaves `service_dirs: Vec::new()`, so the existing Core/Knots detection (which uses those dirs) finds nothing for Open Bitcoin's own service file.
**How to avoid:** In `detection_roots()` in `runtime.rs`, add the platform-appropriate LaunchAgents or systemd user directory to `service_dirs`. The service status adapter injection (D-15) also feeds into `collect_status_snapshot`, so both paths need to be wired.

### Pitfall 7: Parity Breadcrumbs Missing on New Files

**What goes wrong:** `scripts/check-parity-breadcrumbs.ts` fails pre-commit for new `.rs` files under `packages/open-bitcoin-cli/src/operator/service*` that lack breadcrumb blocks and `source-breadcrumbs.json` entries.
**How to avoid:** Every new file gets the `// Parity breadcrumbs:\n// - none: Open Bitcoin-only support/infrastructure...` block at the top and an entry in `docs/parity/source-breadcrumbs.json`. The `noneReason` field in that file already provides the standard text. [VERIFIED: source-breadcrumbs.json, AGENTS.md]

---

## Code Examples

Verified patterns from existing codebase:

### TestDirectory RAII Pattern (from detect/tests.rs)

```rust
// Source: packages/open-bitcoin-cli/src/operator/detect/tests.rs
use std::{fs, path::PathBuf, sync::atomic::{AtomicU64, Ordering}};

static NEXT_TEST_DIRECTORY_ID: AtomicU64 = AtomicU64::new(0);

struct TestDirectory {
    path: PathBuf,
}

impl TestDirectory {
    fn new(label: &str) -> Self {
        let directory = std::env::temp_dir().join(format!(
            "open-bitcoin-service-tests-{label}-{}",
            NEXT_TEST_DIRECTORY_ID.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(&directory).expect("test directory");
        Self { path: directory }
    }
}

impl Drop for TestDirectory {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}
```

### OperatorCommandOutcome Return Pattern (from runtime.rs)

```rust
// Source: packages/open-bitcoin-cli/src/operator/runtime.rs
OperatorCommandOutcome::success(format!("{rendered}\n"))
OperatorCommandOutcome::failure("error message")
```

### ServiceStatus with FieldAvailability (from status.rs)

```rust
// Source: packages/open-bitcoin-node/src/status.rs
ServiceStatus {
    manager: FieldAvailability::available("launchd".to_string()),
    installed: FieldAvailability::available(true),
    enabled: FieldAvailability::unavailable("service manager not inspected"),
    running: FieldAvailability::unavailable("service manager not inspected"),
}
```

### Breadcrumb Block for New Service Files (from detect.rs)

```rust
// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.
```

### Service Status Render Pattern (from status/render.rs)

```rust
// Source: packages/open-bitcoin-cli/src/operator/status/render.rs
fn service_text(service: &ServiceStatus) -> String {
    format!(
        "manager={} installed={} enabled={} running={}",
        string_availability(&service.manager),
        bool_availability(&service.installed),
        bool_availability(&service.enabled),
        bool_availability(&service.running)
    )
}
```

### Platform-Conditional Compilation (Rust standard)

```rust
#[cfg(target_os = "macos")]
pub fn platform_service_manager(home_dir: PathBuf) -> Box<dyn ServiceManager> {
    Box::new(LaunchdAdapter::new(home_dir))
}

#[cfg(target_os = "linux")]
pub fn platform_service_manager(home_dir: PathBuf) -> Box<dyn ServiceManager> {
    Box::new(SystemdAdapter::new(home_dir))
}

#[cfg(not(any(target_os = "macos", target_os = "linux")))]
pub fn platform_service_manager(_home_dir: PathBuf) -> Box<dyn ServiceManager> {
    Box::new(UnsupportedPlatformAdapter)
}
```

---

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| launchctl `load`/`unload` | `launchctl bootstrap`/`bootout` (macOS 10.11+) | macOS Sierra (2016) | `load`/`unload` are deprecated; `bootstrap` is correct for user agents |
| systemd `--user enable` without `--now` | `systemctl --user enable --now` to start immediately | systemd v220+ | Combine enable + start into one command |
| Hardcoded plist `StartInterval` | `KeepAlive true` for daemon that must always run | N/A | KeepAlive is correct for a daemon; StartInterval is for periodic tasks |

**Deprecated/outdated:**
- `launchctl load ~/Library/LaunchAgents/foo.plist`: deprecated since macOS 10.11 (El Capitan). Use `launchctl bootstrap gui/$(id -u) ~/Library/LaunchAgents/foo.plist` for user agents. [ASSUMED: verify against current macOS launchctl man page before finalizing adapter]
- `launchctl unload`: replaced by `launchctl bootout gui/$(id -u) ~/Library/LaunchAgents/foo.plist`.

---

## Project Constraints (from CLAUDE.md)

- **Pre-commit:** `cargo fmt --all`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo build --all-targets --all-features`, `cargo test --all-features` — all must pass before commit.
- **No `unwrap()`:** Use `?` propagation; `expect()` only when panic is impossible.
- **`let...else` for early returns:** Prefer `let Some(x) = maybe else { return Err(...) }`.
- **Prefix `Option` types with `maybe_`:** e.g. `maybe_config_path: Option<PathBuf>`.
- **`thiserror` for library errors:** `ServiceError` in `open-bitcoin-cli` should derive `thiserror::Error`.
- **Test structure:** Arrange / Act / Assert with explicit comments.
- **File length:** Consider splitting files exceeding 300–500 lines into modules.
- **Comment the "why":** Code self-documents; comments explain purpose, not mechanics.
- **Parity breadcrumbs:** Required on every new first-party Rust source file. [VERIFIED: AGENTS.md]
- **Functional core / imperative shell:** Pure generators must have no filesystem or subprocess side effects. [VERIFIED: AGENTS.md]

---

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | `launchctl load/unload` is deprecated in favor of `bootstrap/bootout` on modern macOS | State of the Art, Pitfall 1 | Generated plist install/uninstall commands would use deprecated CLI verbs; would still work but emit deprecation warnings |
| A2 | `tempfile` crate may not already be a dependency in open-bitcoin-cli; the project uses manual `std::env::temp_dir()` for test isolation | Standard Stack | No impact if tempfile is not used; the detect/tests.rs pattern is already proven |
| A3 | `launchctl list <label>` exit code reliably distinguishes loaded vs unloaded for user agents | Common Pitfalls, Pitfall 1 | Status detection would need an alternative approach |

---

## Open Questions

1. **launchctl `bootstrap` vs `load`**
   - What we know: `launchctl load` is deprecated since macOS 10.11; `launchctl bootstrap gui/$(id -u)` is the current form.
   - What's unclear: Whether older macOS versions in the CI matrix require `load` fallback.
   - Recommendation: Use `bootstrap gui/$(id -u)` with a comment noting the macOS version requirement. No fallback needed for v1.1 (macOS-first, modern macOS assumed).

2. **`--apply` flag naming**
   - What we know: CONTEXT.md marks this as Claude's discretion.
   - What's unclear: Whether `--apply` or `--execute` reads more naturally in the help text alongside the dry-run description.
   - Recommendation: Use `--apply`. It pairs naturally with the phrase "by default, commands show what would happen; pass `--apply` to make changes."

3. **systemd `WantedBy=` target**
   - What we know: CONTEXT.md defers to Claude. `default.target` is conventional for user services.
   - What's unclear: Whether `graphical-session.target` would be more appropriate for daemon services not tied to a GUI session.
   - Recommendation: Use `default.target`. It is the standard choice for user daemon services on headless Linux. [CITED: freedesktop.org systemd docs]

4. **`service_dirs` in `detection_roots()` for Open Bitcoin own plist**
   - What we know: Currently `service_dirs: Vec::new()` means the existing detector does not look for Open Bitcoin's own service file.
   - What's unclear: Whether updating `detection_roots()` is the right place, or whether the service adapter injection (D-15) is sufficient on its own.
   - Recommendation: Both. Update `detection_roots()` to include the platform service directory for the Open Bitcoin plist/unit filename, AND wire the adapter injection. The detection pass handles the file presence; the adapter handles the running/enabled state.

---

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| `launchctl` | LaunchdAdapter apply mode | ✓ (macOS dev machine) | macOS built-in | Dry-run only path skips invocation |
| `systemctl` | SystemdAdapter apply mode | ✗ (macOS dev machine) | — | Tests use FakeServiceManager; CI Linux runner has systemctl |
| Rust 1.94.1 | All Rust compilation | ✓ | 1.94.1 (rust-toolchain.toml) | — |
| `cargo clippy`, `cargo fmt` | Pre-commit | ✓ | via rust-toolchain.toml | — |

**Missing dependencies with no fallback:**
- None that block development. systemctl being absent on macOS is expected; no real systemctl call occurs in tests.

**Missing dependencies with fallback:**
- `systemctl`: FakeServiceManager covers all test scenarios; real systemctl is only needed for manual smoke testing on Linux.

---

## Sources

### Primary (HIGH confidence)
- [VERIFIED: codebase inspection] `packages/open-bitcoin-cli/src/operator.rs` — ServiceArgs, ServiceCommand confirmed
- [VERIFIED: codebase inspection] `packages/open-bitcoin-cli/src/operator/runtime.rs` — dispatch stub, detection_roots, OperatorCommandOutcome
- [VERIFIED: codebase inspection] `packages/open-bitcoin-node/src/status.rs` — ServiceStatus, FieldAvailability
- [VERIFIED: codebase inspection] `packages/open-bitcoin-cli/src/operator/detect.rs` — DetectionRoots, ServiceManager enum, LAUNCHD_SERVICE_FILE_NAME, SYSTEMD_SERVICE_FILE_NAME
- [VERIFIED: codebase inspection] `packages/open-bitcoin-cli/src/operator/status.rs` — collect_status_snapshot, service_status(), StatusCollectorInput
- [VERIFIED: codebase inspection] `packages/open-bitcoin-cli/src/operator/detect/tests.rs` — TestDirectory RAII pattern
- [VERIFIED: codebase inspection] `docs/parity/source-breadcrumbs.json` — noneReason, scope, group format
- [VERIFIED: codebase inspection] `AGENTS.md` — parity breadcrumb rule, functional core / imperative shell constraint

### Secondary (MEDIUM confidence)
- [CITED: https://developer.apple.com/library/archive/documentation/MacOSX/Conceptual/BPSystemStartup/Chapters/CreatingLaunchdJobs.html] — plist format, KeepAlive key, LaunchAgents directory
- [CITED: https://www.freedesktop.org/software/systemd/man/systemd.service.html] — unit file format, Restart=on-failure, WantedBy=default.target
- [CITED: https://www.freedesktop.org/software/systemd/man/systemctl.html] — `--user` flag, D-Bus session requirement

### Tertiary (LOW confidence)
- [ASSUMED] launchctl `bootstrap`/`bootout` deprecation timeline — verify against current macOS man page

---

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH — all deps are stdlib; no new crates
- Architecture: HIGH — directly mirrors existing detect.rs and status.rs patterns verified in codebase
- Pitfalls: MEDIUM — launchctl parsing behavior marked ASSUMED; systemd D-Bus issue is CITED
- Plist/unit format: MEDIUM — cited from official Apple/freedesktop docs; format is stable but minor details may differ

**Research date:** 2026-04-26
**Valid until:** 2026-11-26 (stable platform APIs; launchctl/systemd behavior rarely changes at the user-agent level)
