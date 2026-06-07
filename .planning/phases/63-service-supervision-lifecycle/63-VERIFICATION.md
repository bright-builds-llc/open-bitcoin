---
phase: 63-service-supervision-lifecycle
verified: 2026-06-07T19:28:11Z
status: passed
score: "4/4 must-haves verified"
generated_by: gsd-verifier
lifecycle_mode: yolo
phase_lifecycle_id: 63-2026-06-07T14-20-10
generated_at: 2026-06-07T19:28:11Z
lifecycle_validated: true
lifecycle_notes:
  - "Copied lifecycle_mode and phase_lifecycle_id from 63-CONTEXT.md and PLAN frontmatter."
  - "Lifecycle provenance validates after normalizing 63-02-SUMMARY.md frontmatter."
overrides_applied: 0
---

# Phase 63: Service Supervision Lifecycle Verification Report

**Phase Goal:** Operators can manage launchd or systemd supervision for the opt-in unattended workflow with truthful lifecycle state.
**Verified:** 2026-06-07T19:28:11Z
**Status:** passed
**Re-verification:** No - initial verification

## Context Loaded

Read the mandatory phase files before verification: all four `63-*-PLAN.md` files, all four `63-*-SUMMARY.md` files, `.planning/REQUIREMENTS.md`, `63-CONTEXT.md`, `63-RESEARCH.md`, and the Phase 62 verification report.

Also read verifier guidance (`verification-overrides.md`, `gates.md`, thinking models, verifier examples), repo guidance (`AGENTS.md`, `AGENTS.bright-builds.md`, `standards-overrides.md`), and relevant pinned Bright Builds standards: index, architecture, code shape, verification, testing, Rust, and TypeScript/JavaScript. No project-local skills were present under `.claude/skills` or `.agents/skills`. No previous Phase 63 verification artifact existed.

## Goal Achievement

### Observable Truths

| # | Roadmap Success Criterion | Status | Evidence |
|---|---|---|---|
| 1 | Operator can preview, install, start, stop, restart, and inspect launchd or systemd supervision for the opt-in unattended daemon workflow. | VERIFIED | `ServiceCommand` includes `Preview`, `Start`, `Stop`, and `Restart` in `packages/open-bitcoin-cli/src/operator.rs`. `execute_service_command` routes preview through install dry-run with `apply: false`, rejects preview `--apply`, and routes start/stop/restart through typed manager requests in `service.rs:423-497`. launchd/systemd adapters execute user-manager commands after checking the Open Bitcoin service file exists. Focused tests passed: `service_preview`, `service_restart`, `launchd_start_stop_restart`, `systemd_start_stop_restart`, and `resolve_service_daemon_binary`. |
| 2 | Service status distinguishes unmanaged, installed-stopped, running, failed, disabled, and unavailable-manager states while preserving shared sync truth fields. | VERIFIED | `ServiceLifecycleStatus` is a shared serde kebab-case enum with the six exact labels and `as_str()` in `packages/open-bitcoin-node/src/status.rs:60-82`. `status/service_status.rs:11-109` maps manager snapshots and manager errors into shared `ServiceStatus` fields with explicit unavailable reasons. Human status and dashboard render the same fields from `ServiceStatus`. Focused tests passed for node status contract, lifecycle projection/rendering, unavailable-manager fallback, and Phase 62 sync truth preservation. |
| 3 | Service runbooks explain log locations, config paths, safe shutdown, restart review, and recovery actions for launchd and systemd operators. | VERIFIED | `docs/operator/runtime-guide.md` contains repo-local Cargo and Bazel forms for preview, install/apply, start, status, restart, stop, disable, uninstall/apply; documents user LaunchAgent and systemd user unit paths; states generated files supervise `open-bitcoind`; and includes log inspection, config path review, safe shutdown, restart review, and recovery next-action guidance. |
| 4 | Service commands and docs keep the workflow framed as opt-in extended operator review, not a broad production-node claim. | VERIFIED | Runtime guide frames service workflows as opt-in extended operator review. `scripts/check-phase63-service-lifecycle.ts` rejects forbidden production-claim phrases in docs and asserts `scripts/verify.sh` excludes live service-manager and public-network commands. `scripts/verify.sh` runs the Phase 63 checker and does not contain `systemctl --user start|stop|restart`, `launchctl bootstrap|bootout|kickstart`, `run-live-mainnet-smoke`, `--manual-peer`, or `--restart-after-progress`. |

**Score:** 4/4 roadmap must-haves verified.

### Plan Must-Have Coverage

| Plan | Must-Have Group | Status | Evidence |
|---|---|---|---|
| 63-01 | Side-effect-free `service preview`, install dry-run preservation, and generated daemon target. | VERIFIED | Preview parser/dispatcher tests pass; `resolve_service_daemon_binary` prefers sibling `open-bitcoind` and falls back to literal command; generated plist/unit tests assert `/fake/bin/open-bitcoind` plus daemon flags `-datadir` and `-openbitcoinconf`, rejecting old operator-only flags. |
| 63-02 | Start, stop, restart through shared service manager and dashboard paths with user-scope launchd/systemd commands. | VERIFIED | Trait/request/fake-manager/platform/dashboard wiring exists and focused tests pass. launchd commands are `bootstrap`, `bootout`, and `kickstart -k` under `gui/<uid>/org.open-bitcoin.node`; systemd commands are `systemctl --user ... open-bitcoin-node.service`. |
| 63-03 | Shared lifecycle labels and status rendering across direct service status, human status, JSON status, and dashboard while preserving Phase 62 sync truth. | VERIFIED | Shared enum and richer `ServiceStatus` exist. Projection helper was split into `packages/open-bitcoin-cli/src/operator/status/service_status.rs` and wired from `status.rs`; this resolves the literal `gsd-tools` artifact-pattern miss against `operator/status.rs`. |
| 63-04 | Runbook and deterministic checker guard lifecycle labels, commands, daemon target, docs, and default verification boundaries. | VERIFIED | `scripts/check-phase63-service-lifecycle.ts` passes, `scripts/verify.sh` invokes it, and `docs/metrics/lines-of-code.md` names the checker. |

## Required Artifacts

| Artifact | Expected | Status | Details |
|---|---|---|---|
| `packages/open-bitcoin-cli/src/operator.rs` | CLI service command parser | VERIFIED | Defines preview/start/stop/restart command variants. |
| `packages/open-bitcoin-cli/src/operator/runtime.rs` | Daemon binary resolver and service/dashboard runtime wiring | VERIFIED | Service and dashboard paths call `resolve_service_daemon_binary`; helper returns sibling `open-bitcoind` when present or literal `open-bitcoind`. |
| `packages/open-bitcoin-cli/src/operator/service.rs` | Shared service dispatcher, action requests, direct status rendering | VERIFIED | Preview uses install dry-run; start/stop/restart use typed requests; direct status maps exact lifecycle labels. |
| `packages/open-bitcoin-cli/src/operator/service/fake.rs` | Deterministic fake manager action recording | VERIFIED | Records Start/Stop/Restart and returns typed command outcomes without subprocess calls. |
| `packages/open-bitcoin-cli/src/operator/service/launchd.rs` | User-scope launchd lifecycle actions | VERIFIED | Generates and executes `launchctl bootstrap`, `bootout`, `kickstart -k` for the Open Bitcoin user LaunchAgent target after plist existence checks. |
| `packages/open-bitcoin-cli/src/operator/service/systemd.rs` | User-scope systemd lifecycle actions | VERIFIED | Generates and executes `systemctl --user start|stop|restart open-bitcoin-node.service` after unit existence checks. |
| `packages/open-bitcoin-node/src/status.rs` | Shared service lifecycle status contract | VERIFIED | Defines `ServiceLifecycleStatus`, richer `ServiceStatus`, and serde defaults with explicit unavailable reasons. |
| `packages/open-bitcoin-cli/src/operator/status/service_status.rs` | Service manager snapshot projection | VERIFIED | Maps manager snapshots/errors into shared status fields; wired by `operator/status.rs`. |
| `packages/open-bitcoin-cli/src/operator/status/render.rs` | Human status service rendering | VERIFIED | Renders lifecycle, manager, installed, enabled, running, file, logs, and diagnostics in stable order. |
| `packages/open-bitcoin-cli/src/operator/dashboard/action.rs`, `app.rs`, `model.rs` | Dashboard lifecycle actions and rows | VERIFIED | Start/stop/restart require confirmation, map to shared `ServiceCommand`, and service rows render the same shared fields. |
| `docs/operator/runtime-guide.md` | Launchd/systemd lifecycle runbook | VERIFIED | Covers commands, labels, paths, logs, config review, shutdown, restart review, recovery, and opt-in boundaries. |
| `scripts/check-phase63-service-lifecycle.ts` and `scripts/verify.sh` | Deterministic default verification guard | VERIFIED | Checker passes and is invoked by default verification without live service-manager or public-network commands. |

GSD artifact checks passed for 18/19 literal artifacts. The only literal miss was `service_lifecycle_from_snapshot` expected in `operator/status.rs`; verification confirmed the helper exists in the split module `operator/status/service_status.rs` and is imported/used by `operator/status.rs`, so this is not a goal gap.

## Key Link Verification

| From | To | Via | Status | Details |
|---|---|---|---|---|
| `operator/runtime.rs` | `operator/service.rs` | Resolved daemon binary path passed to `execute_service_command` | WIRED | Service runtime calls `resolve_service_daemon_binary(&operator_binary_path)` before building service requests. |
| `operator/runtime.rs` | dashboard service runtime | Same daemon binary resolver | WIRED | Dashboard runtime receives the same resolved `open-bitcoind` path. |
| `service.rs` | launchd/systemd install generators | Preview reuses install dry-run | WIRED | No separate preview generator exists; preview dispatch calls `manager.install` with `apply: false`. |
| dashboard actions | `execute_service_command` | `ServiceCommand::Start/Stop/Restart` mapping | WIRED | `service_args_for_action` maps dashboard actions to shared service commands. |
| platform adapters | user service managers | Native user manager commands | WIRED | launchd/systemd helpers render exact user-scope commands and tests reject machine-scope/source-service strings. |
| service manager status | shared `ServiceStatus` | `collect_service_status` projection | WIRED | Snapshot data flows into lifecycle, manager, booleans, paths, and diagnostics with unavailable reasons. |
| `scripts/verify.sh` | Phase 63 checker | Bun deterministic checker | WIRED | `scripts/verify.sh` runs `bun run scripts/check-phase63-service-lifecycle.ts`. |

## Data-Flow Trace (Level 4)

| Artifact | Data Variable | Source | Produces Real Data | Status |
|---|---|---|---|---|
| Service definitions | `binary_path`, `data_dir`, `maybe_config_path`, `maybe_log_path` | Runtime config resolution plus `resolve_service_daemon_binary` | Yes - service and dashboard paths pass resolved daemon path and selected operator paths into install requests. | FLOWING |
| Preview/install output | `ServiceCommandOutcome` | Platform service manager install dry-run | Yes - outcome includes dry-run flag, service file path/content, and commands; preview reuses install path. | FLOWING |
| Start/stop/restart output | `commands_that_would_run` | launchd/systemd helpers or fake manager | Yes - platform adapters return exact user-manager command strings after effectful calls; fakes return injected command vectors for tests. | FLOWING |
| Shared service status | `ServiceStatus.lifecycle`, manager, booleans, paths, diagnostics | `ServiceStateSnapshot` or manager error | Yes - projection maps snapshots into typed lifecycle labels and converts manager failures to `UnavailableManager`. | FLOWING |
| Human/dashboard status | Service rows/text | Shared `OpenBitcoinStatusSnapshot.service` | Yes - human and dashboard renderers read the shared status fields, not platform-specific strings. | FLOWING |
| Docs/checker | lifecycle labels, command strings, opt-in boundaries | Source, docs, and `scripts/verify.sh` | Yes - Bun checker reads tracked files and exits 0 only when required strings and exclusions hold. | FLOWING |

## Behavioral Spot-Checks

| Behavior | Command | Result | Status |
|---|---|---|---|
| Deterministic Phase 63 checker passes | `bun run scripts/check-phase63-service-lifecycle.ts` | Printed `validated Phase 63 service lifecycle`; exit 0. | PASS |
| Preview command contract | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli service_preview --all-features` | 4 matching tests passed. | PASS |
| Restart command/dashboard contract | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli service_restart --all-features` | 4 matching tests passed. | PASS |
| Shared CLI lifecycle projection/rendering | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli phase63_service_lifecycle --all-features` | 4 matching tests passed. | PASS |
| Shared node lifecycle JSON/default contract | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase63_service_lifecycle_status_contract --all-features` | 2 matching tests passed. | PASS |
| Dashboard service lifecycle rows | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli dashboard_sections_surface_service_lifecycle --all-features` | 1 matching test passed. | PASS |
| launchd user-scope commands and missing plist guard | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli launchd_start_stop_restart --all-features` | 2 matching tests passed. | PASS |
| systemd user-scope commands and missing unit guard | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli systemd_start_stop_restart --all-features` | 2 matching tests passed. | PASS |
| Phase 62 sync truth preserved in human status | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli status_render_includes_sync_progress_and_peer_evidence --all-features` | 1 matching test passed. | PASS |
| Phase 62 sync truth preserved in dashboard | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli dashboard_sections_surface_sync_progress_and_peer_counts --all-features` | 2 matching tests passed. | PASS |
| Manager error becomes unavailable-manager | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli collect_status_snapshot_with_error_manager --all-features` | 1 matching test passed. | PASS |
| Daemon binary resolver | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli resolve_service_daemon_binary --all-features` | 3 matching tests passed. | PASS |
| Diff whitespace | `git diff --check` | Exit 0. | PASS |

The verifier did not rerun full `bash scripts/verify.sh`; the user reported it passed after the WR-01 fix, and the clean code review report also records the relevant focused checks plus checker and diff verification.

## Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
|---|---|---|---|---|
| SVC-01 | 63-01, 63-02, 63-04 | Operator can preview, install, start, stop, restart, and inspect launchd or systemd supervision for the opt-in unattended daemon workflow without implying a broad production-node claim. | SATISFIED | CLI/dashboard parser and dispatcher support preview/start/stop/restart/status; launchd/systemd adapters use user-scope commands; generated definitions target `open-bitcoind`; docs/checker guard opt-in wording and production-claim exclusions. |
| SVC-02 | 63-03, 63-04 | Service status distinguishes unmanaged, installed-stopped, running, failed, disabled, and unavailable-manager states while preserving shared sync truth fields. | SATISFIED | Shared enum, projection, direct service status, human status, JSON status contract, dashboard rows, unavailable reasons, and Phase 62 sync-preservation tests verified. |
| SVC-04 | 63-04 | Service runbooks explain log locations, config paths, safe shutdown, restart review, and recovery actions for launchd and systemd operators. | SATISFIED | Runtime guide contains launchd/systemd lifecycle runbook, repo-local Cargo/Bazel commands, log/config guidance, safe shutdown, restart review, recovery next actions, and exact lifecycle labels. |

No orphaned Phase 63 requirements were found in `.planning/REQUIREMENTS.md`; SVC-01, SVC-02, and SVC-04 are all claimed by Phase 63 plans and verified above. SVC-03 is explicitly mapped to Phase 64 and was not counted as a Phase 63 gap.

## Anti-Patterns Found

| File | Line/Pattern | Severity | Impact |
|---|---|---|---|
| `packages/open-bitcoin-cli/src/operator/service/launchd.rs`, `systemd.rs` | `not available` diagnostics | Info | Expected explicit unavailable-manager diagnostics, not placeholder text. |
| `scripts/generate-loc-report.ts`, `scripts/check-phase63-service-lifecycle.ts` | `console.log`, `return []`, `return null` | Info | Expected script CLI output and parser/optional-control behavior, not stubs. |
| `packages/open-bitcoin-cli/src/operator/service/tests.rs` | Forbidden strings such as `sudo` and machine-scope paths | Info | Negative assertions ensuring generated commands do not use those strings. |
| `scripts/check-phase63-service-lifecycle.ts` | Forbidden production-claim phrases | Info | Deny-list strings used by the checker, not claims in docs. |

No blocker or warning-level stub patterns were found.

## Disconfirmation Pass

- Possible failure: generated services still supervise `open-bitcoin` after the WR-01 fix. Checked generator source/tests and focused resolver tests; generated plist/unit content targets `open-bitcoind` and tests reject `--datadir`, `--config`, and `/fake/bin/open-bitcoin` generated targets.
- Possible failure: lifecycle projection exists but is not wired. `gsd-tools` found a literal artifact-pattern miss because the helper moved to `status/service_status.rs`; manual trace confirmed `operator/status.rs` imports `collect_service_status`, and tests import and exercise `service_lifecycle_from_snapshot`.
- Possible failure: status adds service state but erases Phase 62 sync truth. Focused status and dashboard sync-preservation tests passed.
- Possible failure: default verification starts live services or public-network checks. `scripts/verify.sh` contains the Phase 63 deterministic checker and no forbidden live service/public-network command strings.

## Human Verification Required

None required for the Phase 63 deterministic contract. Live launchd/systemd service-manager and public-network service review remain explicit optional UAT per the phase context and runbook; they are intentionally excluded from default verification.

## Gaps Summary

No gaps found. Phase 63 meets its roadmap goal and success criteria with deterministic source, test, docs, and checker evidence.

---

_Verified: 2026-06-07T19:28:11Z_
_Verifier: the agent (gsd-verifier)_
