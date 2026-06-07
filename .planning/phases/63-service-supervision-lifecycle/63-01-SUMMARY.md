---
phase: 63-service-supervision-lifecycle
plan: "01"
subsystem: operator-service-lifecycle
tags:
  - rust
  - cli
  - service-preview
  - launchd
  - systemd
  - open-bitcoind
requires:
  - .planning/phases/63-service-supervision-lifecycle/63-01-PLAN.md
  - docs/parity/index.json
provides:
  - first-class open-bitcoin service preview command
  - side-effect-free service definition rendering path
  - open-bitcoind service binary resolution for generated launchd/systemd definitions
affects:
  - packages/open-bitcoin-cli operator command surface
  - service manager dry-run rendering
  - dashboard service runtime binary selection
tech_stack:
  added: []
  patterns:
    - reuse ServiceManager install dry-run for service preview
    - resolve service daemon path as materialized sibling open-bitcoind, falling back to literal open-bitcoind
key_files:
  created:
    - .planning/phases/63-service-supervision-lifecycle/63-01-SUMMARY.md
  modified:
    - docs/metrics/lines-of-code.md
    - packages/open-bitcoin-cli/src/operator.rs
    - packages/open-bitcoin-cli/src/operator/runtime.rs
    - packages/open-bitcoin-cli/src/operator/service.rs
    - packages/open-bitcoin-cli/src/operator/service/fake.rs
    - packages/open-bitcoin-cli/src/operator/service/tests.rs
    - packages/open-bitcoin-cli/src/operator/tests.rs
key_decisions:
  - service preview rejects --apply before calling the service manager
  - install without --apply remains the legacy dry-run path
  - service and dashboard runtimes resolve open-bitcoind through one shared helper
  - TDD RED failures were observed but not committed because repo AGENTS requires passing Rust gates before commits
requirements_completed:
  - SVC-01
metrics:
  started_at: 2026-06-07T15:06:46Z
  completed_at: 2026-06-07T15:46:29Z
  duration_seconds: 2383
  tasks_completed: 3
  task_commits: 3
  files_changed: 7
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 63-2026-06-07T14-20-10
generated_at: 2026-06-07T15:46:29Z
---

# Phase 63 Plan 01: Service Supervision Lifecycle Preview Summary

Side-effect-free service preview with daemon-target launchd/systemd definitions and tested open-bitcoind resolution.

## What Changed

- Added `open-bitcoin service preview` as a first-class operator command.
- Routed preview through the existing install dry-run renderer with `apply: false`.
- Rejected `open-bitcoin service preview --apply` before any service manager call.
- Resolved generated service definitions to `open-bitcoind`, preferring a materialized sibling binary beside `open-bitcoin` and falling back to literal `open-bitcoind`.
- Reused the daemon resolver for both service commands and dashboard service actions.

## Task Results

| Task | Result | Commit |
| --- | --- | --- |
| 1. First-class service preview command | Added parser/runtime support and preview-focused tests | `c0f94b2` |
| 2. Resolve service definitions to open-bitcoind | Added daemon resolver and updated generator tests | `561ae63` |
| 3. Verify preview safety and generated definition content | Ran focused and workspace verification, plus content greps | `f165640` |

## Verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli service_preview --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli parsing_service_install_without_apply_flag_sets_apply_false --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli resolve_service_daemon_binary --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli service::tests::plist_content_contains_required_fields --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli service::tests::unit_content_contains_required_fields --all-features`
- `cargo fmt --manifest-path packages/Cargo.toml --all --check`
- `cargo clippy --manifest-path packages/Cargo.toml --workspace --all-targets --all-features -- -D warnings`
- `cargo build --manifest-path packages/Cargo.toml --workspace --all-targets --all-features`
- `cargo test --manifest-path packages/Cargo.toml --workspace --all-features`
- `git commit` hooks for all task commits, including repo `scripts/verify.sh`, parity breadcrumbs, architecture checks, benchmark smoke, Bazel smoke, and coverage gate.
- Content review greps confirmed preview routing, `open-bitcoind` resolver coverage, daemon binary assertions in service definition tests, and no positive generated-content assertion still targeting `/fake/bin/open-bitcoin`.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 2 - AGENTS Pre-Commit Compliance] Kept TDD RED failures uncommitted**
- **Found during:** Tasks 1 and 2
- **Issue:** The GSD TDD flow asks for RED test commits, but repo `AGENTS.md` requires passing Rust format, clippy, build, and tests before each commit.
- **Fix:** Wrote RED tests and observed the expected failures, then completed GREEN implementation and committed only passing task states after full Rust verification.
- **Files modified:** Task implementation and test files listed above.
- **Commit:** `c0f94b2`, `561ae63`

## Issues Encountered

- A broad content grep matched the new negative assertion that rejects `/fake/bin/open-bitcoin`; the check was narrowed to positive generated-content assertions and passed.
- Some early focused Cargo tests waited on the package lock while another Cargo command was running; verification continued sequentially afterward.

## Auth Gates

None.

## Known Stubs

None. Stub scan found no `TODO`, `FIXME`, placeholder text, empty hardcoded UI data, or similar stubs in the files modified by this plan.

## State Updates

Per executor instructions, `.planning/STATE.md` and `.planning/ROADMAP.md` were not updated. Existing orchestrator-owned working tree artifacts were left untouched.

## Self-Check: PASSED

- Found summary file: `.planning/phases/63-service-supervision-lifecycle/63-01-SUMMARY.md`
- Found task commit: `c0f94b2`
- Found task commit: `561ae63`
- Found task commit: `f165640`
