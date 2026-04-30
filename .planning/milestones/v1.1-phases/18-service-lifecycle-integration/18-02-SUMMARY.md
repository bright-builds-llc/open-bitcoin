---
phase: 18-service-lifecycle-integration
plan: "02"
subsystem: operator-service-lifecycle
requirements-completed: [SVC-01, SVC-02, SVC-03, SVC-05]
tags:
  - service
  - dry-run
  - apply-flag
  - runtime-dispatch
  - detection-roots
dependency_graph:
  requires:
    - packages/open-bitcoin-cli/src/operator/service.rs (Plan 01 contracts)
    - packages/open-bitcoin-cli/src/operator/service/fake.rs (Plan 01 FakeServiceManager)
    - packages/open-bitcoin-cli/src/operator/runtime.rs (execute_operator_cli_inner dispatch point)
  provides:
    - ServiceArgs.apply flag (--apply CLI arg)
    - execute_service_command() in service.rs
    - Real service dispatch in runtime.rs (replaces Phase 18 stub)
    - detection_roots() service_dirs populated with platform paths
  affects:
    - packages/open-bitcoin-cli/src/operator.rs
    - packages/open-bitcoin-cli/src/operator/service.rs
    - packages/open-bitcoin-cli/src/operator/runtime.rs
    - packages/open-bitcoin-cli/src/operator/service/tests.rs
tech_stack:
  added: []
  patterns:
    - dry-run-by-default: apply=false is default; --apply flag unlocks mutations per D-11
    - trait-injection: execute_service_command accepts &dyn ServiceManager for test isolation
    - functional-core/imperative-shell: render helpers are pure; dispatch delegates to adapter
    - compile-time platform selection: service_dirs uses #[cfg(target_os)] in detection_roots
key_files:
  created: []
  modified:
    - packages/open-bitcoin-cli/src/operator.rs
    - packages/open-bitcoin-cli/src/operator/service.rs
    - packages/open-bitcoin-cli/src/operator/runtime.rs
    - packages/open-bitcoin-cli/src/operator/service/tests.rs
decisions:
  - "ServiceArgs.apply uses global = true so --apply works at any position (e.g. 'service install --apply' or 'service --apply install')"
  - "execute_service_command returns OperatorCommandOutcome directly (not Result) matching the non-error-propagating pattern of the rest of the dispatch surface"
  - "render_service_outcome shows 'Would write: <path>' rather than just 'File:' to make the dry-run intent clear to operators"
  - "detection_roots service_dirs uses home_dir.join() instead of PathBuf::from literal to stay consistent with the home_dir already computed in the function"
metrics:
  duration_minutes: 20
  completed_date: "2026-04-27"
  tasks_completed: 2
  files_created: 0
  files_modified: 4
---

# Phase 18 Plan 02: Service Command Dispatch Summary

## One-Liner

Wire --apply flag to ServiceArgs, implement execute_service_command() with dry-run rendering and scope surfacing, replace the Phase 18 stub in runtime.rs with real dispatch via platform_service_manager(), and populate detection_roots() service_dirs with LaunchAgents/systemd paths.

## What Was Built

### --apply Flag on ServiceArgs (operator.rs)

- Added `pub apply: bool` to `ServiceArgs` with `#[arg(long = "apply", global = true)]`
- Default is `false` (dry-run safe per D-11)
- `global = true` allows `--apply` at any subcommand position
- Visible in `open-bitcoin service --help` output

### execute_service_command() (service.rs)

- New `pub fn execute_service_command(args, binary_path, data_dir, maybe_config_path, maybe_log_path, manager)` function
- Dispatches all 5 `ServiceCommand` variants: Install, Uninstall, Enable, Disable, Status
- Returns `OperatorCommandOutcome` (not Result) directly matching surrounding dispatch patterns
- `render_service_outcome()` helper produces human-readable output:
  - Dry-run header: "Dry run (pass --apply to make changes):"
  - Description from adapter
  - "Would write: <path>" when file path provided
  - "Commands:" section listing platform commands that would run
  - "Scope: user-level (no sudo required)." when dry-run (per D-09, D-13)
  - "Generated content:" with indented file body
- `render_service_state_snapshot()` helper for status command output

### Real Dispatch in runtime.rs (replaces stub)

- Removed the "service lifecycle commands are deferred to Phase 18" failure arm
- `OperatorCommand::Service(service)` now:
  1. Resolves `home_dir` from `HOME` env var with `.` fallback
  2. Creates `platform_service_manager(home_dir)` (compile-time platform selection)
  3. Resolves `binary_path` via `current_exe()` with `"open-bitcoin"` fallback per T-18-08
  4. Resolves `data_dir` from `config_resolution.maybe_data_dir` or `default_data_dir`
  5. Calls `execute_service_command()` passing config and log paths from resolution
- Imports `execute_service_command` and `platform_service_manager` from `super::service`
- Removed the now-unused `ServiceCommand` import from the use block

### detection_roots() Service Dirs (runtime.rs)

- `service_dirs` in `DetectionRoots` now populated:
  - macOS: `vec![home_dir.join("Library/LaunchAgents")]`
  - Linux: `vec![home_dir.join(".config/systemd/user")]`
  - Other: `Vec::new()`
- Uses `#[cfg(target_os)]` compile-time gates consistent with platform adapter selection

### New Tests (service/tests.rs)

6 new tests added:
1. `execute_service_command_install_dry_run_shows_dry_run_output` — verifies dry-run indicator in stdout
2. `execute_service_command_install_already_installed_returns_failure` — verifies `AlreadyInstalled` error surfaces as failure with "already installed" in stderr
3. `execute_service_command_enable_returns_success_with_output` — enable returns success with non-empty stdout
4. `execute_service_command_uninstall_dry_run_succeeds` — dry-run uninstall with FakeServiceManager succeeds
5. `parsing_service_install_with_apply_flag_sets_apply_true` — `--apply` flag parses correctly to `true`
6. `parsing_service_install_without_apply_flag_sets_apply_false` — default `apply` is `false`
7. `execute_service_command_install_dry_run_shows_scope` — "user-level" or "Scope" present in dry-run output

Total test count: 60 (all passing, up from 53 in Plan 01).

## Deviations from Plan

None — plan executed exactly as written. The `render_service_outcome()` helper was implemented exactly as specified in the plan's action block. The `detection_roots()` service_dirs pattern followed the plan's note about keeping `home_dir` accessible.

## Known Stubs

None. All 5 service subcommands dispatch to real adapter methods. No placeholder text flows to output.

## Threat Flags

None. The only new dispatch path (current_exe fallback to "open-bitcoin" string) is a safety net, not a new trust boundary. The --apply flag default of false is correctly mitigating T-18-06. The detection_roots service_dirs addition is read-only per T-18-07 (accept disposition).

## Self-Check: PASSED

Files exist:
- packages/open-bitcoin-cli/src/operator.rs: FOUND (pub apply: bool added to ServiceArgs)
- packages/open-bitcoin-cli/src/operator/service.rs: FOUND (execute_service_command present)
- packages/open-bitcoin-cli/src/operator/runtime.rs: FOUND (deferred stub removed, execute_service_command called)
- packages/open-bitcoin-cli/src/operator/service/tests.rs: FOUND (6 new tests added)

Commits exist:
- 12742ce: feat(18-02): add --apply flag to ServiceArgs and implement execute_service_command()
- 6481fb2: feat(18-02): wire runtime.rs service dispatch and populate detection_roots service_dirs

Verification:
- cargo fmt --all: PASSED
- cargo clippy --package open-bitcoin-cli --all-targets --all-features -- -D warnings: PASSED
- cargo build --package open-bitcoin-cli --all-features: PASSED
- cargo test --package open-bitcoin-cli --all-features: 60 tests PASSED
