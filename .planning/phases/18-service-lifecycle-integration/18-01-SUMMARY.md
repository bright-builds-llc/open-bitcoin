---
phase: 18-service-lifecycle-integration
plan: "01"
subsystem: operator/service
tags:
  - service-lifecycle
  - launchd
  - systemd
  - trait-pattern
  - dry-run
dependency_graph:
  requires:
    - open-bitcoin-cli/src/operator.rs (ServiceCommand enum, module wiring)
    - open-bitcoin-node/src/status.rs (ServiceStatus, FieldAvailability)
  provides:
    - ServiceManager trait (install/uninstall/enable/disable/status)
    - ServiceError enum (5 typed variants)
    - LaunchdAdapter with generate_plist_content pure generator
    - SystemdAdapter with generate_unit_content pure generator
    - FakeServiceManager for test isolation
    - platform_service_manager factory
  affects:
    - Plan 18-02 (adapter subprocess wiring)
    - Plan 18-03 (status wiring to collect_status_snapshot)
tech_stack:
  added:
    - thiserror 2.0 (typed error derivation in open-bitcoin-cli)
  patterns:
    - Functional core / imperative shell: pure generators (no I/O) + adapter impls (effectful)
    - Trait object injection for test isolation via FakeServiceManager
    - cfg(target_os) compile-time platform adapter selection
    - RefCell<Vec<FakeServiceCall>> for call recording without Mutex overhead
key_files:
  created:
    - packages/open-bitcoin-cli/src/operator/service.rs
    - packages/open-bitcoin-cli/src/operator/service/launchd.rs
    - packages/open-bitcoin-cli/src/operator/service/systemd.rs
    - packages/open-bitcoin-cli/src/operator/service/fake.rs
    - packages/open-bitcoin-cli/src/operator/service/tests.rs
  modified:
    - packages/open-bitcoin-cli/src/operator.rs (added pub mod service)
    - packages/open-bitcoin-cli/Cargo.toml (added thiserror 2.0)
    - docs/parity/source-breadcrumbs.json (operator-service-lifecycle group)
    - packages/Cargo.lock (thiserror locked)
decisions:
  - "Added thiserror 2.0 to open-bitcoin-cli as the first thiserror consumer in the workspace; matches CLAUDE.md guidance for library errors and the plan's explicit requirement"
  - "UnsupportedPlatformAdapter gated with cfg(not(any(target_os=macos,linux))) on both struct and impl to eliminate dead_code warning on macOS builds"
  - "FakeServiceManager uses RefCell (not Mutex) since tests are single-threaded; avoids unnecessary synchronization overhead"
metrics:
  duration: ~25 minutes
  completed: "2026-04-27"
  tasks_completed: 1
  tasks_total: 1
  files_created: 5
  files_modified: 4
---

# Phase 18 Plan 01: ServiceManager Trait, Generators, and FakeServiceManager Summary

**One-liner:** ServiceManager trait with launchd plist and systemd unit pure generators, typed ServiceError via thiserror, FakeServiceManager for call recording, and compile-time platform_service_manager factory.

## What Was Built

This plan establishes the functional core layer for Phase 18 service lifecycle integration:

1. **ServiceManager trait** (`service.rs`) — five methods: `install`, `uninstall`, `enable`, `disable`, `status`. Platform-agnostic; adapters implement it; test code uses `FakeServiceManager`.

2. **ServiceError enum** (`service.rs`) — five typed variants: `UnsupportedPlatform`, `AlreadyInstalled`, `NotInstalled`, `WriteFailure`, `ManagerCommandFailed`. Derived via `thiserror::Error`; also derives `Clone` so `FakeServiceManager::install_error: Option<ServiceError>` works.

3. **Request and outcome types** (`service.rs`) — `ServiceInstallRequest` (binary_path, data_dir, maybe_config_path, maybe_log_path, apply), `ServiceUninstallRequest` (apply), `ServiceEnableRequest`, `ServiceDisableRequest`, `ServiceCommandOutcome` (dry_run, description, maybe_file_path, maybe_file_content, commands_that_would_run), `ServiceStateSnapshot` (state, maybe_service_file_path, maybe_manager_diagnostics, maybe_log_path), `ServiceLifecycleState` (Unmanaged/Installed/Enabled/Running/Stopped/Failed).

4. **LaunchdAdapter** (`service/launchd.rs`) — pure `generate_plist_content()` function producing Apple plist XML with Label, ProgramArguments (binary + --datadir + optional --config), KeepAlive=true, RunAtLoad=true, optional StandardOutPath/StandardErrorPath. `LaunchdAdapter` implements `ServiceManager` with dry-run / apply distinction, `AlreadyInstalled` guard, parent directory creation, `launchctl list` state detection (exit code 113 = Unmanaged, `"PID" = ` = Running, `"LastExitStatus" = 0;` = Stopped, else Failed).

5. **SystemdAdapter** (`service/systemd.rs`) — pure `generate_unit_content()` function producing `[Unit]/[Service]/[Install]` unit file with `ExecStart`, `Restart=on-failure`, `StandardOutput=journal`, `WantedBy=default.target`. `SystemdAdapter` implements `ServiceManager` with same dry-run / apply pattern; `systemctl --user is-active` state detection (exit 0 = Running, exit 3 = Stopped, else = Failed/Unmanaged).

6. **FakeServiceManager** (`service/fake.rs`) — `RefCell<Vec<FakeServiceCall>>` for call recording, configurable `status_to_return`, optional `install_error` for error path testing. No filesystem or subprocess access.

7. **platform_service_manager factory** (`service.rs`) — `#[cfg(target_os = "macos")]` → `LaunchdAdapter`, `#[cfg(target_os = "linux")]` → `SystemdAdapter`, fallback `UnsupportedPlatformAdapter` (both struct and impl gated with `#[cfg(not(any(...)))]` to suppress dead_code on macOS builds).

8. **11 tests** (`service/tests.rs`) covering: plist generator required fields, config path, log paths; unit generator required fields, config path; FakeServiceManager install call recording, status return; ServiceError Display for UnsupportedPlatform and AlreadyInstalled; LaunchdAdapter dry-run no-write; SystemdAdapter dry-run no-write.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 2 - Missing Critical Functionality] Added thiserror to Cargo.toml**
- **Found during:** Task 1 — `thiserror::Error` derive in service.rs failed to compile
- **Issue:** `thiserror` was not in the workspace at all; the CLI crate had no thiserror dependency
- **Fix:** Added `thiserror = "2.0"` to `packages/open-bitcoin-cli/Cargo.toml`
- **Files modified:** `packages/open-bitcoin-cli/Cargo.toml`, `packages/Cargo.lock`
- **Commit:** bfb8f64

**2. [Rule 1 - Bug] cfg-gated UnsupportedPlatformAdapter struct and impl**
- **Found during:** Task 1 — dead_code warning for `UnsupportedPlatformAdapter` on macOS builds
- **Issue:** The struct was unconditionally defined but only used inside a `#[cfg(not(...))]` factory arm
- **Fix:** Applied `#[cfg(not(any(target_os = "macos", target_os = "linux")))]` to both the struct definition and the `impl ServiceManager` block
- **Files modified:** `packages/open-bitcoin-cli/src/operator/service.rs`
- **Commit:** bfb8f64

**3. [Rule 1 - Bug] Fixed then(|| x) → then_some(x) per clippy**
- **Found during:** Task 1 clippy run — `unnecessary_lazy_evaluations` lint
- **Issue:** `installed.then(|| plist_path)` uses unnecessary closure where `then_some` is cleaner
- **Fix:** Replaced 3 instances in launchd.rs and 2 in systemd.rs with `.then_some()`
- **Files modified:** `packages/open-bitcoin-cli/src/operator/service/launchd.rs`, `packages/open-bitcoin-cli/src/operator/service/systemd.rs`
- **Commit:** bfb8f64

## Known Stubs

None. This plan implements the functional core layer (pure generators + trait + fake); subprocess invocation for launchd/systemd enable/disable follows the live adapter pattern but gracefully handles missing binaries by returning a dry-run description. No placeholder text flows to UI rendering.

## Threat Flags

None beyond those in the plan's threat model. No new network endpoints, auth paths, or trust boundary crossings introduced. All subprocess calls use `std::process::Command::arg()` per-argument (no shell interpolation).

## Self-Check: PASSED

All created files exist at expected paths. Commit bfb8f64 verified in git log. `cargo test --package open-bitcoin-cli --all-features` exits 0 with 53/53 unit tests and 9/9 integration tests passing. `cargo clippy --package open-bitcoin-cli --all-targets --all-features -- -D warnings` exits 0.
