---
phase: 18-service-lifecycle-integration
plan: "01"
subsystem: operator-service-lifecycle
tags:
  - service
  - launchd
  - systemd
  - trait
  - dry-run
dependency_graph:
  requires:
    - packages/open-bitcoin-cli/src/operator.rs
    - packages/open-bitcoin-node/src/status.rs
  provides:
    - packages/open-bitcoin-cli/src/operator/service.rs
    - packages/open-bitcoin-cli/src/operator/service/launchd.rs
    - packages/open-bitcoin-cli/src/operator/service/systemd.rs
    - packages/open-bitcoin-cli/src/operator/service/fake.rs
    - packages/open-bitcoin-cli/src/operator/service/tests.rs
  affects:
    - packages/open-bitcoin-cli/src/operator.rs
    - docs/parity/source-breadcrumbs.json
tech_stack:
  added:
    - thiserror 2.0.12 (CLI crate dependency for typed error derives)
  patterns:
    - functional-core/imperative-shell: pure generators in service modules, subprocess calls in adapters
    - trait-injection: ServiceManager trait enables FakeServiceManager substitution in tests
    - dry-run-by-default: apply=false returns preview with file content and commands, no I/O
    - compile-time platform selection: #[cfg(target_os)] in platform_service_manager() factory
key_files:
  created:
    - packages/open-bitcoin-cli/src/operator/service.rs
    - packages/open-bitcoin-cli/src/operator/service/launchd.rs
    - packages/open-bitcoin-cli/src/operator/service/systemd.rs
    - packages/open-bitcoin-cli/src/operator/service/fake.rs
    - packages/open-bitcoin-cli/src/operator/service/tests.rs
  modified:
    - packages/open-bitcoin-cli/src/operator.rs
    - packages/open-bitcoin-cli/Cargo.toml
    - docs/parity/source-breadcrumbs.json
    - packages/Cargo.lock
decisions:
  - "Use thiserror 2.x for ServiceError derive since no workspace-level thiserror existed; added to CLI crate Cargo.toml directly"
  - "UnsupportedPlatformAdapter is always compiled (not cfg-gated) with #[allow(dead_code)] so the fallback arm in platform_service_manager() remains valid on all platforms"
  - "Tests live directly in service/tests.rs without a wrapping mod tests {} to avoid clippy::module_inception lint"
  - "LaunchdAdapter::uid() uses id -u subprocess rather than libc to avoid unsafe code (project forbids unsafe_code)"
metrics:
  duration_minutes: 40
  completed_date: "2026-04-27"
  tasks_completed: 1
  files_created: 5
  files_modified: 4
---

# Phase 18 Plan 01: Service Lifecycle Contracts Summary

## One-Liner

ServiceManager trait with pure plist/unit-file generators, LaunchdAdapter, SystemdAdapter, FakeServiceManager, and platform factory — the functional-core layer for macOS launchd and Linux systemd service lifecycle.

## What Was Built

### ServiceManager Trait and Types (service.rs)

- `ServiceLifecycleState` enum: Unmanaged, Installed, Enabled, Running, Failed, Stopped
- `ServiceStateSnapshot`, `ServiceInstallRequest`, `ServiceUninstallRequest`, `ServiceEnableRequest`, `ServiceDisableRequest`, `ServiceCommandOutcome` structs
- `ServiceError` with 5 variants: UnsupportedPlatform, AlreadyInstalled, NotInstalled, WriteFailure, ManagerCommandFailed — all derive Clone (required by FakeServiceManager)
- `ServiceManager` trait with 5 methods: install, uninstall, enable, disable, status
- `UnsupportedPlatformAdapter` private struct (always compiled, never cfg-gated)
- `platform_service_manager(home_dir)` factory using `#[cfg(target_os = "macos")]` / `#[cfg(target_os = "linux")]` / fallback

### LaunchdAdapter (service/launchd.rs)

- `OPEN_BITCOIN_LAUNCHD_LABEL = "org.open-bitcoin.node"` constant
- `OPEN_BITCOIN_LAUNCHD_FILE_NAME = "org.open-bitcoin.node.plist"` constant
- `generate_plist_content()` pure function producing valid XML plist with Label, ProgramArguments (binary + --datadir + optional --config), KeepAlive, RunAtLoad, optional StandardOutPath/StandardErrorPath
- `LaunchdAdapter::uid()` retrieves UID via `id -u` subprocess (avoids unsafe code)
- `impl ServiceManager for LaunchdAdapter`: install (dry-run/apply with AlreadyInstalled guard), uninstall (bootout then file removal), enable/disable (launchctl invocations), status (exit-code + CF-plist-text string matching for PID/LastExitStatus)

### SystemdAdapter (service/systemd.rs)

- `OPEN_BITCOIN_SYSTEMD_FILE_NAME = "open-bitcoin-node.service"` constant
- `generate_unit_content()` pure function producing [Unit]/[Service]/[Install] sections with ExecStart, Restart=on-failure, WantedBy=default.target
- `impl ServiceManager for SystemdAdapter`: same dry-run/apply pattern; target `~/.config/systemd/user/`; systemctl --user invocations

### FakeServiceManager (service/fake.rs)

- `FakeServiceCall` enum recording Install{apply}, Uninstall{apply}, Enable, Disable, Status
- `FakeServiceManager` with `recorded_calls: RefCell<Vec<FakeServiceCall>>`, `status_to_return`, `maybe_install_error`
- `FakeServiceManager::unmanaged()` convenience constructor
- No subprocess invocations or filesystem writes in any method

### Tests (service/tests.rs)

11 tests, all passing:
1. `plist_content_contains_required_fields`
2. `plist_content_includes_config_path_when_provided`
3. `plist_content_includes_log_paths_when_provided`
4. `unit_content_contains_required_fields`
5. `unit_content_includes_config_path_when_provided`
6. `fake_manager_install_records_call`
7. `fake_manager_status_returns_configured_state`
8. `service_error_unsupported_platform_displays_reason`
9. `service_error_already_installed_displays_path`
10. `launchd_install_dry_run_does_not_write_file`
11. `systemd_install_dry_run_does_not_write_file`

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Removed unsafe block around safe function**
- **Found during:** Task 1 build
- **Issue:** Initial implementation had `unsafe { libc_uid() }` but `libc_uid()` is a safe function using `std::process::Command`; the project also forbids unsafe_code via `#![forbid(unsafe_code)]`
- **Fix:** Inlined `id -u` subprocess call directly into `LaunchdAdapter::uid()` method, eliminating both the unsafe block and the separate `libc_uid()` function
- **Files modified:** packages/open-bitcoin-cli/src/operator/service/launchd.rs

**2. [Rule 1 - Bug] Fixed clippy::module_inception lint**
- **Found during:** Task 1 clippy pass
- **Issue:** `tests.rs` file had a `mod tests {}` wrapper which clippy flags as module_inception (module with same name as containing module)
- **Fix:** Removed the `mod tests {}` wrapper — test functions live directly at the top level of `tests.rs`; the `#[cfg(test)]` guard comes from `mod tests;` declaration in `service.rs`
- **Files modified:** packages/open-bitcoin-cli/src/operator/service/tests.rs

**3. [Rule 2 - Missing] Added #[allow(dead_code)] to UnsupportedPlatformAdapter**
- **Found during:** Task 1 build (warning)
- **Issue:** `UnsupportedPlatformAdapter` is compiled on all platforms but only reachable on unsupported platforms; the compiler warns dead_code on macOS
- **Fix:** Added `#[allow(dead_code)]` with explanatory comment
- **Files modified:** packages/open-bitcoin-cli/src/operator/service.rs

## Known Stubs

None. All generators produce real content. Adapters implement all 5 ServiceManager methods. No placeholder data flows to consumers.

## Threat Flags

None. No new network endpoints, external auth paths, or trust boundary crossings introduced. The subprocess calls (launchctl, systemctl, id) are local system utilities invoked with separate `arg()` calls (no shell interpolation). The `AlreadyInstalled` guard prevents silent overwrites per T-18-03.

## Self-Check: PASSED

Files exist:
- packages/open-bitcoin-cli/src/operator/service.rs: FOUND
- packages/open-bitcoin-cli/src/operator/service/launchd.rs: FOUND
- packages/open-bitcoin-cli/src/operator/service/systemd.rs: FOUND
- packages/open-bitcoin-cli/src/operator/service/fake.rs: FOUND
- packages/open-bitcoin-cli/src/operator/service/tests.rs: FOUND

Commits exist:
- dfe3038: feat(18-01): implement ServiceManager trait, generators, and FakeServiceManager — FOUND

Verification:
- cargo clippy --package open-bitcoin-cli --all-targets --all-features -- -D warnings: PASSED
- cargo test --package open-bitcoin-cli --all-features: 53 tests passed (11 service-specific)
