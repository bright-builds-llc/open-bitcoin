---
phase: 18-service-lifecycle-integration
plan: "03"
subsystem: operator-service-lifecycle
tags:
  - service
  - status
  - trait-injection
  - field-availability
dependency_graph:
  requires:
    - packages/open-bitcoin-cli/src/operator/service.rs (Plan 01 ServiceManager trait)
    - packages/open-bitcoin-cli/src/operator/service/fake.rs (Plan 01 FakeServiceManager)
    - packages/open-bitcoin-cli/src/operator/runtime.rs (Plan 02 execute_status wiring)
    - packages/open-bitcoin-node/src/status.rs (ServiceStatus, FieldAvailability)
  provides:
    - StatusCollectorInput.maybe_service_manager field
    - collect_service_status() live vs detection fallback logic
    - 4 new service manager injection tests
  affects:
    - packages/open-bitcoin-cli/src/operator/status.rs
    - packages/open-bitcoin-cli/src/operator/status/tests.rs
    - packages/open-bitcoin-cli/src/operator/runtime.rs (already wired)
tech_stack:
  added: []
  patterns:
    - trait-injection: Box<dyn ServiceManager> in StatusCollectorInput enables FakeServiceManager in tests
    - graceful-degradation: manager.status() error falls back to all-unavailable fields (T-18-09)
    - detection-fallback: None maybe_service_manager falls back to file-presence detection
    - compile-time platform name: #[cfg(target_os)] selects manager_name string in collect_service_status
key_files:
  created: []
  modified:
    - packages/open-bitcoin-cli/src/operator/status.rs
    - packages/open-bitcoin-cli/src/operator/status/tests.rs
    - packages/open-bitcoin-cli/src/operator/service.rs
    - packages/open-bitcoin-cli/src/operator/service/launchd.rs
    - packages/open-bitcoin-cli/src/operator/service/systemd.rs
    - packages/open-bitcoin-cli/src/operator/service/tests.rs
decisions:
  - "StatusCollectorInput loses Clone/PartialEq/Debug derives because Box<dyn ServiceManager> cannot derive them — manual impls not needed since no caller currently relies on those derives"
  - "collect_service_status falls back to detection_service_status when no manager injected, preserving existing detection-based behavior as default"
  - "manager_name determined by #[cfg(target_os)] in collect_service_status rather than adding a platform_name field to ServiceStateSnapshot"
metrics:
  duration_minutes: 30
  completed_date: "2026-04-27"
  tasks_completed: 2
  files_created: 0
  files_modified: 6
---

# Phase 18 Plan 03: Service Adapter Status Integration Summary

## One-Liner

Wire optional Box<dyn ServiceManager> into StatusCollectorInput with collect_service_status() mapping ServiceStateSnapshot to ServiceStatus fields, enabling live service state in `open-bitcoin status` with graceful fallback to unavailable fields on errors.

## What Was Built

### StatusCollectorInput Service Adapter Field (status.rs)

- Added `pub maybe_service_manager: Option<Box<dyn super::service::ServiceManager>>` field
- Removed `#[derive(Clone, PartialEq, Eq, Debug)]` from `StatusCollectorInput` since `Box<dyn ServiceManager>` cannot derive those traits
- Added `collect_service_status(input: &StatusCollectorInput) -> ServiceStatus` function:
  - When `maybe_service_manager` is `Some(manager)`: calls `manager.status()`, maps `ServiceStateSnapshot` to `ServiceStatus` fields
  - On `Ok(snapshot)`: sets `installed = !Unmanaged`, `enabled = Enabled|Running`, `running = Running`; manager name from `#[cfg(target_os)]`
  - On `Err(_)`: falls back to all-unavailable fields (T-18-09 graceful degradation)
  - When `None`: delegates to `detection_service_status()` (existing file-presence fallback)
- Renamed old `service_status()` function to `detection_service_status()` for clarity
- Updated both `collect_live_status_snapshot` and `stopped_status_snapshot` to call `collect_service_status(input)`
- Renamed `ServiceManager` import from detect module to `DetectServiceManager` to resolve naming conflict

### Runtime Wiring (runtime.rs)

The `execute_status()` function in `runtime.rs` already had `maybe_service_manager` wired from the worktree base state — no additional changes needed.

### Service Render Verification (render.rs)

Confirmed that `service_text()` in `render.rs` already renders all four `ServiceStatus` fields:
```
Service: manager={} installed={} enabled={} running={}
```
No changes to `render.rs` needed — the plan's acceptance criteria were already met.

### New Tests (status/tests.rs)

4 new tests added:
1. `collect_status_snapshot_with_no_service_manager_preserves_unavailable_service_fields`
2. `collect_status_snapshot_with_fake_running_manager_sets_service_fields_to_available_true`
3. `collect_status_snapshot_with_fake_installed_manager_sets_installed_true_enabled_false`
4. `collect_status_snapshot_with_error_manager_falls_back_to_unavailable`

Updated `status_input()` helper and existing test to include `maybe_service_manager: None`.

Total: 64 unit tests + 11 integration tests (all passing).

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Fixed duplicate function definitions in service.rs**
- **Found during:** Initial build attempt (pre-Task 1)
- **Issue:** `service.rs` had two definitions each of `render_service_outcome`, `render_service_state_snapshot`, and `execute_service_command` causing E0428 compilation errors. The older set used `"  File: ..."` and `crate::operator::runtime::OperatorCommandOutcome`; the Plan 02 set used `"  Would write: ..."` and `super::runtime::OperatorCommandOutcome`.
- **Fix:** Removed the older duplicate set, keeping only the Plan 02 versions
- **Files modified:** `packages/open-bitcoin-cli/src/operator/service.rs`
- **Commit:** aec7dae

**2. [Rule 1 - Bug] Removed #[derive(Debug)] from StatusCollectorInput**
- **Found during:** Task 1 compile pass
- **Issue:** `Box<dyn ServiceManager>` does not implement `std::fmt::Debug`, so `#[derive(Debug)]` fails
- **Fix:** Removed `#[derive(Debug)]` (along with `Clone`, `PartialEq`, `Eq` which would also fail). No callers rely on those derives.
- **Files modified:** `packages/open-bitcoin-cli/src/operator/status.rs`
- **Commit:** aec7dae

## Known Stubs

None. `collect_service_status()` wires live service state when a manager is injected. The detection-evidence fallback is intentional documented behavior.

## Threat Flags

None. The T-18-09 threat (DoS from `manager.status()` error propagating) is mitigated by the `Err(_)` fallback arm in `collect_service_status()`. No new network endpoints or trust boundaries introduced.

## Self-Check: PASSED

Files modified:
- packages/open-bitcoin-cli/src/operator/status.rs: FOUND (contains `maybe_service_manager`, `collect_service_status`, `service manager not inspected`)
- packages/open-bitcoin-cli/src/operator/status/tests.rs: FOUND (4 new service manager injection tests)

Commits:
- aec7dae: feat(18-03): add service adapter injection to StatusCollectorInput and wire collect_service_status
- 8529c85: feat(18-03): verify service render output and pass full pre-commit check suite

Verification:
- cargo fmt --all: PASSED
- cargo clippy --all-targets --all-features -- -D warnings: zero warnings PASSED
- cargo build --all-targets --all-features: PASSED
- cargo test --all-features: all tests PASSED (64 CLI unit + 11 integration)
