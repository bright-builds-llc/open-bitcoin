---
phase: 18-service-lifecycle-integration
plan: "02"
subsystem: operator/service
tags:
  - service-lifecycle
  - dry-run
  - apply-flag
  - dispatch
  - runtime-wiring
dependency_graph:
  requires:
    - Plan 18-01 (ServiceManager trait, ServiceCommandOutcome, FakeServiceManager, platform_service_manager)
    - open-bitcoin-cli/src/operator.rs (ServiceArgs, ServiceCommand enum)
    - open-bitcoin-cli/src/operator/runtime.rs (execute_operator_cli_inner, OperatorCommandOutcome)
  provides:
    - ServiceArgs.apply (--apply flag, global, default false)
    - execute_service_command() routing all five ServiceCommand variants
    - render_service_outcome() with dry-run indicator and user-level scope note
    - render_service_state_snapshot() for status display
    - FakeServiceManager.uninstall_error and enable_commands fields
    - runtime.rs service dispatch wired to platform_service_manager()
    - detection_roots() service_dirs populated with platform service directory
  affects:
    - Plan 18-03 (status wiring to collect_status_snapshot)
tech_stack:
  added: []
  patterns:
    - TDD RED→GREEN: failing tests written before implementation
    - Trait object injection: execute_service_command takes &dyn ServiceManager
    - cfg(target_os) compile-time service_dirs selection in detection_roots()
    - current_exe() with PathBuf fallback for resilience (T-18-08 mitigation)
key_files:
  created: []
  modified:
    - packages/open-bitcoin-cli/src/operator.rs (added apply: bool to ServiceArgs)
    - packages/open-bitcoin-cli/src/operator/service.rs (execute_service_command, render_service_outcome, render_service_state_snapshot)
    - packages/open-bitcoin-cli/src/operator/service/fake.rs (uninstall_error and enable_commands fields)
    - packages/open-bitcoin-cli/src/operator/service/tests.rs (6 new tests for execute_service_command and --apply parsing)
    - packages/open-bitcoin-cli/src/operator/runtime.rs (service dispatch wired, detection_roots populated)
decisions:
  - "--apply flag uses global = true so it works in any position after `service` subcommand, consistent with other global flags in OperatorCli"
  - "FakeServiceManager.enable_commands Vec<String> lets tests verify command strings surface in output without real launchctl/systemctl"
  - "detection_roots() uses cfg-gated let bindings (not cfg on struct field) for clearer cross-platform readability"
  - "render_service_outcome() renders dry-run scope note only when dry_run=true, keeping apply output concise"
metrics:
  duration: ~20 minutes
  completed: "2026-04-27"
  tasks_completed: 2
  tasks_total: 2
  files_created: 0
  files_modified: 5
---

# Phase 18 Plan 02: Service Dispatch Wiring and --apply Flag Summary

**One-liner:** Wired `--apply` flag to `ServiceArgs`, implemented `execute_service_command()` routing all five service subcommands to the injected `ServiceManager`, and replaced the Phase 18 deferred stub in `runtime.rs` with live dispatch via `platform_service_manager()`.

## What Was Built

This plan connects the Plan 18-01 functional core layer to the CLI command surface:

1. **`--apply` flag on `ServiceArgs`** (`operator.rs`) — `pub apply: bool` with `#[arg(long = "apply", global = true)]`. Defaults false (dry-run safe). Visible in `open-bitcoin service --help`.

2. **`execute_service_command()`** (`service.rs`) — Public function taking `&ServiceArgs`, `binary_path`, `data_dir`, optional config/log paths, and `&dyn ServiceManager`. Routes Install/Uninstall/Enable/Disable/Status to manager methods. Returns `OperatorCommandOutcome` directly (no `Result` wrapping).

3. **`render_service_outcome()`** (`service.rs`) — Private renderer producing human-readable dry-run preview with: "Dry run (pass --apply to make changes):" header, description, file path, commands, "Scope: user-level (no sudo required)." note, and generated file content block. Apply output omits the dry-run framing.

4. **`render_service_state_snapshot()`** (`service.rs`) — Status renderer producing service state, file path, log path, and manager diagnostics.

5. **`FakeServiceManager` extensions** (`service/fake.rs`) — Added `uninstall_error: Option<ServiceError>` for not-installed error path testing, and `enable_commands: Vec<String>` for verifying command strings surface in enable output.

6. **6 new tests** (`service/tests.rs`) — TDD RED→GREEN covering: dry-run install returns success mentioning dry run, apply install with `AlreadyInstalled` returns failure with "already installed", enable returns success with launchctl/systemctl command string, uninstall with `NotInstalled` returns failure with "not installed", `--apply` parses to `true`, default parses to `false`.

7. **runtime.rs service dispatch** (`operator/runtime.rs`) — Replaced the "service lifecycle commands are deferred to Phase 18" stub with: home_dir from `env::var_os("HOME")`, `platform_service_manager(home_dir)`, `current_exe()` with fallback, data_dir from config or default, `execute_service_command()` call. Removed `ServiceCommand` from the `use super::` block.

8. **`detection_roots()` service_dirs** (`operator/runtime.rs`) — Populated with `~/Library/LaunchAgents` on macOS and `~/.config/systemd/user` on Linux via `#[cfg(target_os)]` gated let bindings.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 2 - Missing Critical Functionality] Added uninstall_error and enable_commands to FakeServiceManager**
- **Found during:** Task 1 test writing — the plan requires testing not-installed uninstall error and enable command string surfacing, but FakeServiceManager only had `install_error`
- **Issue:** No mechanism to inject `ServiceError::NotInstalled` from `uninstall()`, and no way to configure what commands `enable()` surfaces in its outcome
- **Fix:** Added `uninstall_error: Option<ServiceError>` (checked in `uninstall()` before success) and `enable_commands: Vec<String>` (returned in `enable()` outcome's `commands_that_would_run`)
- **Files modified:** `packages/open-bitcoin-cli/src/operator/service/fake.rs`
- **Commit:** a3c7a25

## Known Stubs

None. All five service subcommands are now dispatched to real adapter implementations (or FakeServiceManager in tests). No placeholder text flows to UI rendering.

## Threat Flags

None. The changes are entirely within the existing trust boundary:
- `--apply` defaults to false (T-18-06 mitigation verified).
- `current_exe()` has `unwrap_or_else` fallback (T-18-08 mitigation verified).
- `detection_roots()` service_dirs addition is read-only detection scan (T-18-07 accepted).

## Self-Check: PASSED

- `packages/open-bitcoin-cli/src/operator/runtime.rs` does NOT contain "deferred to Phase 18"
- `packages/open-bitcoin-cli/src/operator/runtime.rs` contains `execute_service_command` and `platform_service_manager`
- `packages/open-bitcoin-cli/src/operator/runtime.rs` contains `LaunchAgents` and `systemd` in service_dirs (cfg-gated)
- `packages/open-bitcoin-cli/src/operator.rs` contains `pub apply: bool` and `long = "apply"`
- `packages/open-bitcoin-cli/src/operator/service.rs` contains `pub fn execute_service_command`, `fn render_service_outcome`, `Dry run (pass --apply`, `Scope: user-level`
- Commits a3c7a25 and 2d67d87 verified in git log
- `cargo test --package open-bitcoin-cli --all-features` exits 0 with 59 unit tests, 2 binary tests, 5 operator binary tests, 4 operator flow tests passing
- `cargo clippy --package open-bitcoin-cli --all-targets --all-features -- -D warnings` exits 0
