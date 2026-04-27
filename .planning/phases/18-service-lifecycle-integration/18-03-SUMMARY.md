---
phase: 18-service-lifecycle-integration
plan: "03"
subsystem: operator-status
tags:
  - service-lifecycle
  - status-collection
  - trait-injection
  - tdd
dependency_graph:
  requires:
    - 18-01  # ServiceManager trait and FakeServiceManager
    - 18-02  # service dispatch wiring in runtime.rs
  provides:
    - live service state in open-bitcoin status output
    - collect_service_status() function mapping ServiceStateSnapshot to ServiceStatus
  affects:
    - open-bitcoin-cli/src/operator/status.rs
    - open-bitcoin-cli/src/operator/runtime.rs
tech_stack:
  added: []
  patterns:
    - trait-object injection via Option<Box<dyn ServiceManager>>
    - fallback chain: live adapter → file-presence detection → unavailable
    - manual Debug impl for non-Debug trait object field
key_files:
  created: []
  modified:
    - packages/open-bitcoin-cli/src/operator/status.rs
    - packages/open-bitcoin-cli/src/operator/status/tests.rs
    - packages/open-bitcoin-cli/src/operator/runtime.rs
decisions:
  - "StatusCollectorInput loses Clone/PartialEq/Eq derives; manual Debug impl added for Box<dyn ServiceManagerTrait> field"
  - "detect::ServiceManager (enum) aliased to DetectServiceManager to avoid name collision with service::ServiceManager (trait)"
  - "collect_service_status() falls back to detection_service_status() when maybe_service_manager is None, preserving existing file-presence behavior"
  - "platform_service_manager() wired unconditionally in execute_status() — launchd on macOS, systemd on Linux, UnsupportedPlatformAdapter elsewhere"
metrics:
  duration: "~25 minutes"
  completed: "2026-04-27T02:56:00Z"
  tasks_completed: 2
  files_modified: 3
---

# Phase 18 Plan 03: Service Status Adapter Integration Summary

Wire the service state adapter into `open-bitcoin status` so the `service.*` fields are populated from live service manager inspection, completing SVC-03, SVC-04, and SVC-05.

## What Was Built

`StatusCollectorInput` now accepts an optional `Box<dyn ServiceManager>`. When wired, `collect_service_status()` calls `manager.status()` and maps the `ServiceStateSnapshot` to the four `ServiceStatus` fields (`manager`, `installed`, `enabled`, `running`) using `FieldAvailability`. On `Err(_)` or when no adapter is injected, all four fields fall back to `unavailable("service manager not inspected")`. The file-presence-based detection path is preserved as a secondary fallback for the `None` case.

In `runtime.rs`, `execute_status()` now constructs `platform_service_manager(home_dir)` and passes it as `maybe_service_manager` into `StatusCollectorInput`.

## Commits

| Task | Description | Commit |
|------|-------------|--------|
| 1 | Wire service adapter into StatusCollectorInput + tests (TDD) | 8f86342 |
| 2 | Verify render.rs and confirm pre-commit checks pass | b127b85 |

## Tests Added

Four new tests in `packages/open-bitcoin-cli/src/operator/status/tests.rs`:

- `collect_status_with_no_service_manager_keeps_service_fields_unavailable` — None adapter preserves fallback behavior
- `collect_status_with_fake_running_manager_sets_service_fields_available` — Running state maps installed=true, enabled=true, running=true
- `collect_status_with_fake_installed_manager_sets_installed_true_enabled_false` — Installed state maps correctly
- `collect_status_with_error_manager_falls_back_to_unavailable` — ServiceError causes graceful unavailable fallback (T-18-09)

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Name collision between detect::ServiceManager and service::ServiceManager**
- **Found during:** Task 1 implementation
- **Issue:** `status.rs` already imported `detect::ServiceManager` (an enum). Adding `service::ServiceManager` (a trait) under the same name would conflict.
- **Fix:** Aliased `detect::ServiceManager` as `DetectServiceManager` and `service::ServiceManager` as `ServiceManagerTrait` in imports. Updated `service_manager_name()` signature accordingly. Also updated `tests.rs` import to use `ServiceManager as DetectServiceManager`.
- **Files modified:** `status.rs`, `status/tests.rs`

**2. [Rule 1 - Bug] Box<dyn Trait> cannot derive Debug**
- **Found during:** Task 1 compile — `#[derive(Debug)]` on `StatusCollectorInput` failed because `Box<dyn ServiceManagerTrait>` does not implement `Debug`.
- **Fix:** Removed `#[derive(Debug)]` and added manual `fmt::Debug` impl that prints `"<ServiceManager>"` for the trait object field. Also removed `Clone, PartialEq, Eq` derives (they were present but incompatible with the new field) — no callers required them.
- **Files modified:** `status.rs`

**3. [Rule 2 - Out of scope] Pre-existing open-bitcoin-consensus build error**
- `cargo build --all-targets` fails in `open-bitcoin-consensus` test crate due to a missing `bitcoin-knots/src/test/data/sighash.json` vendored file. This is pre-existing and unrelated to Plan 18-03. Logged as out-of-scope; all relevant packages (open-bitcoin-cli, open-bitcoin-node, open-bitcoin-rpc) build and test clean.

## Verification

- `cargo fmt --all` — clean (no changes)
- `cargo clippy --package open-bitcoin-cli --package open-bitcoin-node --package open-bitcoin-rpc --all-targets --all-features -- -D warnings` — zero warnings
- `cargo test --package open-bitcoin-cli --all-features` — 63+ tests pass (0 failed)
- `render.rs service_text()` renders all four ServiceStatus fields (manager, installed, enabled, running)
- No TODO or placeholder in render.rs service field rendering

## Known Stubs

None — all four ServiceStatus fields are populated from live service manager inspection when the adapter is wired.

## Threat Flags

None — no new network endpoints, auth paths, or trust boundaries introduced. T-18-09 (DoS via manager.status() error) mitigated by `Err(_)` fallback in `collect_service_status()`.

## Self-Check: PASSED

- `packages/open-bitcoin-cli/src/operator/status.rs` — FOUND, contains `maybe_service_manager` and `collect_service_status`
- `packages/open-bitcoin-cli/src/operator/runtime.rs` — FOUND, contains `maybe_service_manager` in StatusCollectorInput construction
- Commit `8f86342` — FOUND (Task 1)
- Commit `b127b85` — FOUND (Task 2)
