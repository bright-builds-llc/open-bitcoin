---
phase: 63-service-supervision-lifecycle
plan: "02"
subsystem: operator-service-lifecycle
tags:
  - rust
  - cli
  - launchd
  - systemd
  - dashboard
  - service-lifecycle
dependency_graph:
  requires:
    - 63-01 service preview and daemon content
  provides:
    - SVC-01 user-scope service lifecycle start stop restart actions
  affects:
    - packages/open-bitcoin-cli/src/operator.rs
    - packages/open-bitcoin-cli/src/operator/service.rs
    - packages/open-bitcoin-cli/src/operator/service/fake.rs
    - packages/open-bitcoin-cli/src/operator/service/launchd.rs
    - packages/open-bitcoin-cli/src/operator/service/systemd.rs
    - packages/open-bitcoin-cli/src/operator/service/tests.rs
    - packages/open-bitcoin-cli/src/operator/dashboard/action.rs
    - packages/open-bitcoin-cli/src/operator/dashboard/app.rs
    - packages/open-bitcoin-cli/src/operator/dashboard/app/tests.rs
    - packages/open-bitcoin-cli/src/operator/dashboard/model.rs
    - packages/open-bitcoin-cli/src/operator/dashboard/model/tests.rs
tech_stack:
  added: []
  patterns:
    - Existing ServiceManager dispatcher and typed ServiceCommandOutcome
    - User-scope launchd and systemd command adapters
    - Dashboard confirmation state machine through injected manager
key_files:
  created:
    - .planning/phases/63-service-supervision-lifecycle/63-02-SUMMARY.md
  modified:
    - docs/metrics/lines-of-code.md
    - packages/open-bitcoin-cli/src/operator.rs
    - packages/open-bitcoin-cli/src/operator/service.rs
    - packages/open-bitcoin-cli/src/operator/service/fake.rs
    - packages/open-bitcoin-cli/src/operator/service/launchd.rs
    - packages/open-bitcoin-cli/src/operator/service/systemd.rs
    - packages/open-bitcoin-cli/src/operator/service/tests.rs
    - packages/open-bitcoin-cli/src/operator/dashboard/action.rs
    - packages/open-bitcoin-cli/src/operator/dashboard/app.rs
    - packages/open-bitcoin-cli/src/operator/dashboard/app/tests.rs
    - packages/open-bitcoin-cli/src/operator/dashboard/model.rs
    - packages/open-bitcoin-cli/src/operator/dashboard/model/tests.rs
decisions:
  - Service start, stop, and restart are effectful commands and do not require --apply.
  - launchd and systemd lifecycle actions remain user-scope only.
  - Dashboard lifecycle actions reuse execute_service_command instead of a dashboard-only workflow.
requirements_completed:
  - SVC-01
metrics:
  started_at: 2026-06-07T15:55:25Z
  completed_at: 2026-06-07T16:44:10Z
  duration: 48m45s
  tasks_completed: 3
  task_commits: 3
---

# Phase 63 Plan 02: Service Supervision Lifecycle Actions Summary

Typed CLI, launchd/systemd, and dashboard start/stop/restart service lifecycle actions now route through the shared ServiceManager dispatcher.

## Task Results

| Task | Result | Commit | Key Files |
| --- | --- | --- | --- |
| 1. CLI service command contract | Added `ServiceCommand::Start/Stop/Restart`, request marker types, dispatcher branches, fake manager calls, and focused parser/dispatcher tests. | `0bc77b0` | `operator.rs`, `service.rs`, `service/fake.rs`, `service/tests.rs` |
| 2. User-scope platform adapters | Implemented user-level launchd/systemd start, stop, and restart with exact command rendering and missing-service guards. | `f170517` | `service/launchd.rs`, `service/systemd.rs`, `service/tests.rs` |
| 3. Dashboard lifecycle actions | Added confirmed `t/o/x` dashboard actions that reuse `execute_service_command` and injected managers. | `af562b1` | `dashboard/action.rs`, `dashboard/app.rs`, `dashboard/model.rs`, dashboard tests |

## Verification

- RED tests were written and observed failing before implementation for the missing service and dashboard lifecycle contract.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli service_start --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli service_stop --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli service_restart --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli launchd_start_stop_restart --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli systemd_start_stop_restart --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli dashboard_service_start --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli dashboard_service_stop --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli dashboard_service_restart --all-features`
- Acceptance `rg` checks for service command contract, platform command strings, dashboard action mappings, and forbidden dashboard direct manager commands passed.
- Pre-commit Rust gates passed before task commits: `cargo fmt --manifest-path packages/Cargo.toml --all`, `cargo clippy --manifest-path packages/Cargo.toml --all-targets --all-features -- -D warnings`, `cargo build --manifest-path packages/Cargo.toml --all-targets --all-features`, and `cargo test --manifest-path packages/Cargo.toml --all-features`.
- Normal git hooks passed for each task commit, including `bash scripts/verify.sh`, parity breadcrumbs, production file-length checks, benchmark smoke validation, Bazel build/run smoke, and coverage-backed tests.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 2 - AGENTS Pre-Commit Compliance] Kept TDD RED failures uncommitted**
- **Found during:** Tasks 1, 2, and 3
- **Issue:** The GSD TDD flow asks for RED test commits, but repo instructions require full passing Rust gates before every commit.
- **Fix:** Wrote RED tests, confirmed failing signals, then committed only passing GREEN changes after full verification.
- **Files modified:** Rust service and dashboard files listed in task results.
- **Commit:** `0bc77b0`, `f170517`, `af562b1`

**2. [Rule 3 - Blocking] Added default unsupported lifecycle methods to ServiceManager**
- **Found during:** Task 1
- **Issue:** Adding required trait methods would have broken existing platform adapters until Task 2 supplied concrete implementations.
- **Fix:** Added default `UnsupportedPlatform` implementations with exact lifecycle-specific reasons, preserving compilation and explicit unsupported behavior.
- **Files modified:** `packages/open-bitcoin-cli/src/operator/service.rs`
- **Commit:** `0bc77b0`

**3. [Rule 3 - Blocking] Consolidated platform subprocess helpers**
- **Found during:** Task 2
- **Issue:** Adding lifecycle execution duplicated launchd/systemd subprocess error handling and risked tripping the repo production file-length hook.
- **Fix:** Reused small `run_launchctl` and `run_systemctl` helpers while keeping exact rendered command strings in outcomes.
- **Files modified:** `packages/open-bitcoin-cli/src/operator/service/launchd.rs`, `packages/open-bitcoin-cli/src/operator/service/systemd.rs`
- **Commit:** `f170517`

## Auth Gates

None.

## Known Stubs

None. Stub scan found only format strings and existing service-manager availability diagnostics, not placeholder UI data or unwired mock data.

## Threat Surface Scan

No unplanned threat flags. The only subprocess and service-file access added is the planned user-scope launchd/systemd lifecycle surface, guarded by service-file existence checks and covered by no-sudo/no-machine-scope acceptance checks. Dashboard code still contains no direct `launchctl` or `systemctl` invocation.

## State Updates

Skipped by instruction. `.planning/STATE.md` and `.planning/ROADMAP.md` were not updated or staged by this executor.

## Self-Check: PASSED

- Found summary file: `.planning/phases/63-service-supervision-lifecycle/63-02-SUMMARY.md`
- Found task commit: `0bc77b0`
- Found task commit: `f170517`
- Found task commit: `af562b1`
