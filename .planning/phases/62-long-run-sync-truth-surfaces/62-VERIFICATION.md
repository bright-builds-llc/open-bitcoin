---
phase: 62-long-run-sync-truth-surfaces
verified: 2026-06-07T00:27:11Z
status: passed
score: "4/4 roadmap success criteria verified"
generated_by: gsd-verifier
lifecycle_mode: yolo
phase_lifecycle_id: 62-2026-06-06T19-46-48
generated_at: 2026-06-07T00:27:11Z
lifecycle_validated: true
overrides_applied: 0
review_fixes:
  - finding: WR-01
    commit: 2d358be
    status: verified
  - finding: WR-02
    commit: 3d7e03c
    status: verified
residual_risks:
  - "Public-network live-smoke UAT was not run; Phase 62 explicitly keeps that opt-in and outside default verification."
  - "Full bash scripts/verify.sh was not rerun by this verifier; recent normal commit hooks reported it green and focused required checks were rerun."
---

# Phase 62: Long-Run Sync Truth Surfaces Verification Report

**Phase Goal:** Operators see consistent bounded sync truth across status, dashboard, RPC, metrics, logs, and live-smoke snapshots.
**Verified:** 2026-06-07T00:27:11Z
**Status:** passed
**Re-verification:** No - initial verification

## Context Loaded

Read repo guidance and phase artifacts before verification:

- `AGENTS.md`, `AGENTS.bright-builds.md`, `standards-overrides.md`
- Pinned Bright Builds standards via commit `05f8d7a6c9c2e157ec4f922a05273e72dab97676`: index, verification, testing, Rust, TypeScript/JavaScript
- `62-CONTEXT.md`, all four `62-*-PLAN.md` files, all four `62-*-SUMMARY.md` files
- `62-REVIEW.md` and `62-REVIEW-FIX.md`

No previous `62-VERIFICATION.md` existed. No project-local skills were present under `.claude/skills` or `.agents/skills`.

## Goal Achievement

### Observable Truths

| # | Roadmap Success Criterion | Status | Evidence |
|---|---|---|---|
| 1 | Status, dashboard, RPC sync status, metrics, structured logs, and live-smoke snapshots agree on loop phase, configured targets, attempt counters, latest progress, latest stop reason, peer health, and downloaded or connected block evidence. | VERIFIED | Shared fields exist in `status.rs`; summary projects metrics/log labels; CLI status/dashboard/sync-status and RPC warnings read typed fields; live-smoke maps from `metadata.maybe_sync_state.sync`. Focused Rust, Bun, and shell fixture checks passed. |
| 2 | Metrics and structured logs retain bounded long-run samples and cycle summaries without unbounded growth. | VERIFIED | `SyncRunSummary::metric_samples` returns the five numeric sync samples only. Structured logs emit compact cycle records with `latest_stop_reason=` and `progress_signal=` labels. |
| 3 | Operator can distinguish progress, waiting, retry, stop, and recovery states the same way across every truth surface. | VERIFIED | Status/dashboard/sync-status/RPC/live-smoke surfaces preserve `progress_signal`, `latest_stop_reason`, `recovery_category`, `recovery_action`, `resource_pressure`, unavailable reasons, and peer/progress evidence. |
| 4 | Repeated long-run snapshot output stays compact enough for operator review while preserving diagnosis evidence. | VERIFIED | Live-smoke report snapshots are table rows plus final compact status. Persisted report shape keeps daemon output summary counts/flags and fixture asserts no `stdoutTail`, `stderrTail`, or `Daemon Output Tail`. |

**Score:** 4/4 roadmap success criteria verified.

## Required Artifacts

| Artifact | Status | Evidence |
|---|---|---|
| `packages/open-bitcoin-node/src/status.rs` | VERIFIED | Defines `SyncConfiguredTargets`, `SyncAttemptCounters`, `SyncStopReasonStatus`, and serde-defaulted `SyncStatus` fields with explicit unavailable reasons. |
| `packages/open-bitcoin-node/src/sync/types/summary.rs` | VERIFIED | Emits bounded metric samples and compact structured logs with `latest_stop_reason=` and `progress_signal=` labels. |
| `packages/open-bitcoin-node/src/sync/runtime_state.rs` | VERIFIED | Projects runtime config targets, target header height, attempt counters, stop reason, resource pressure, and max rounds into durable sync state. |
| `packages/open-bitcoin-cli/src/operator/status/render.rs` | VERIFIED | Renders Phase 62 rows from `snapshot.sync.*` typed fields with `Unavailable: {reason}` preservation. |
| `packages/open-bitcoin-cli/src/operator/dashboard/model.rs` | VERIFIED | Dashboard rows include configured targets, attempt counters, latest stop reason, recovery, pressure, peer health, and progress in the contract order. |
| `packages/open-bitcoin-cli/src/operator/runtime/support.rs` | VERIFIED | `open-bitcoin sync status` prints lifecycle, phase, configured targets, attempts, signal, last progress, stop reason, error, recovery, pressure, peer health, heights, hashes, and counters from `RuntimeMetadata`. |
| `packages/open-bitcoin-rpc/src/dispatch/node.rs` | VERIFIED | Durable warnings include typed `progress_signal=`, `latest_stop_reason=`, and `recovery_category=` labels. |
| `scripts/run-live-mainnet-smoke.ts` | VERIFIED | Maps typed sync status into camelCase report fields, preserves unavailable/null reasons, renders compact Markdown snapshots/final status, and omits raw daemon tails from `SmokeReport`. |
| `scripts/test-run-live-mainnet-smoke.sh` | VERIFIED | Deterministic fixture asserts Phase 62 JSON/Markdown fields, raw-tail exclusion, and unavailable progress/peer fields as null rather than synthesized zeroes. |
| `scripts/check-phase62-sync-truth-surfaces.ts` | VERIFIED | Reads Rust, TypeScript, docs, fixture, and `scripts/verify.sh`; checks field labels and public-network exclusion. |
| `docs/operator/runtime-guide.md`, `docs/architecture/status-snapshot.md`, `docs/architecture/operator-observability.md` | VERIFIED | Document shared field order, `Unavailable: {reason}`, bounded metrics/logs, compact live-smoke reports, repo-local commands, and opt-in public-network UAT. |

GSD artifact checks passed for all plans: 14/14 artifacts passed. GSD key-link checks passed for all plans: 10/10 links verified.

## Key Link Verification

| From | To | Status | Evidence |
|---|---|---|---|
| Durable runtime | Shared `SyncStatus` contract | WIRED | `runtime_state.rs` sets configured targets and max rounds on durable sync state; `status.rs` owns typed fields. |
| `SyncRunSummary` | Metrics and structured logs | WIRED | Metrics stay numeric (`HeaderHeight`, `DownloadedBlockHeight`, `ConnectedBlockHeight`, `SyncHeight`, `PeerCount`); logs carry compact labels. |
| Operator status/dashboard/sync-status | Shared status metadata | WIRED | Renderers call `snapshot.sync.configured_targets`, `snapshot.sync.attempt_counters`, `snapshot.sync.latest_stop_reason`, or `metadata.maybe_sync_state`. |
| RPC warnings/status | Durable sync state | WIRED | `durable_warnings` uses typed field availability and tests assert metadata JSON fields. |
| Live-smoke reports | `metadata.maybe_sync_state.sync` | WIRED | `syncStatusSnapshotFromMetadata` and `finalStatusSummaryFromMetadata` consume typed `FieldAvailability` fields directly. |
| Default verification | Deterministic Phase 62 checker | WIRED | `scripts/verify.sh` runs `bun run scripts/check-phase62-sync-truth-surfaces.ts` after the Phase 61 checker and excludes public-network commands. |

## Data-Flow Trace

| Artifact | Data Variable | Source | Produces Real Data | Status |
|---|---|---|---|---|
| `SyncStatus` | `configured_targets`, `attempt_counters`, `latest_stop_reason` | `SyncRunSummary::sync_status` and `DurableSyncRuntime::durable_sync_state` | Yes - runtime config, peer attempts, and typed stop reason feed durable status. | FLOWING |
| Metrics/logs | `MetricSample`, `StructuredLogRecord.message` | `SyncRunSummary` counters/progress/stop/recovery fields | Yes - deterministic tests assert numeric samples and compact labels. | FLOWING |
| CLI status/dashboard/sync-status | Typed `OpenBitcoinStatusSnapshot` and `RuntimeMetadata` fields | Shared node status structs | Yes - renderers read typed fields, not prior human text. | FLOWING |
| RPC warnings/status | `durable_sync_state.sync` | Local durable runtime state | Yes - RPC tests assert warning labels and metadata JSON field names. | FLOWING |
| Live-smoke JSON/Markdown | `SyncStatusSnapshot`, `FinalStatusSummary` | Typed status JSON from `open-bitcoin sync status` or mocked fixture | Yes - fixture tests generated JSON/Markdown and unavailable/null behavior. | FLOWING |
| Docs/checker | Field arrays and exact source strings | Rust/TS/docs/verify files | Yes - checker exits 0 and prints `validated Phase 62 sync truth surfaces`. | FLOWING |

## Review Fix Confirmation

| Finding | Commit | Verification |
|---|---|---|
| WR-01 structured sync logs drift from Phase 62 truth labels | `2d358be fix(62): WR-01 align structured sync log labels` | `summary.rs` now emits `latest_stop_reason=` and `progress_signal=`; Rust tests assert those labels; checker validates them in `summary.rs` instead of excluding them. |
| WR-02 live-smoke reports hide unavailable progress and peer facts behind zeroes | `3d7e03c fix(62): preserve unavailable live smoke truth` | Live-smoke snapshots/final status carry `maybeSyncProgressUnavailableReason` and `maybePeerCountsUnavailableReason`; unavailable progress/peer numeric fields become `null`; fixture fails if they regress to zero. |

`git show --stat` confirmed both commits exist and modify the expected files. `62-REVIEW-FIX.md` reports normal hooks passed for both fix commits, including `bash scripts/verify.sh`.

## Behavioral Spot-Checks

| Behavior | Command | Result | Status |
|---|---|---|---|
| Cross-surface deterministic checker passes | `bun run scripts/check-phase62-sync-truth-surfaces.ts` | Printed `validated Phase 62 sync truth surfaces`; exit 0. | PASS |
| Deterministic live-smoke fixture validates JSON/Markdown | `bash scripts/test-run-live-mainnet-smoke.sh` | Quiet success; exit 0. | PASS |
| Node Phase 62 contract tests pass | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase62 --all-features` | 4 passed, 0 failed. | PASS |
| CLI sync-status Phase 62 test passes | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli sync_status --all-features` | 1 matching library test passed; filtered zero-test targets completed; exit 0. | PASS |
| Default verification excludes public-network live smoke | `bash -c 'if rg -n "run-live-mainnet-smoke|--manual-peer|--restart-after-progress" scripts/verify.sh; then exit 1; fi'` | No matches; exit 0. | PASS |
| Operator status render test passes | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli status_render_includes_sync_progress_and_peer_evidence --all-features` | 1 passed, 0 failed. | PASS |
| Dashboard truth rows pass | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli dashboard_sections_surface_sync_progress_and_peer_counts --all-features` | 2 passed, 0 failed. | PASS |
| RPC sync status metadata passes | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc open_bitcoin_sync_status --all-features` | 1 passed, 0 failed. | PASS |
| RPC blockchain-info warning labels pass | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc get_blockchain_info --all-features` | 1 passed, 0 failed. | PASS |
| Persisted live-smoke reports omit raw tails | `rg -n "stdoutTail|stderrTail|Daemon Output Tail" packages/target/live-mainnet-smoke-reports/open-bitcoin-live-mainnet-smoke.json packages/target/live-mainnet-smoke-reports/open-bitcoin-live-mainnet-smoke.md` | No matches after fixture run. | PASS |

## Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
|---|---|---|---|---|
| OBS-01 | 62-01, 62-02, 62-03, 62-04 | Operator-facing status, dashboard, RPC sync status, metrics, structured logs, and live-smoke snapshots agree on unattended loop phase, configured targets, attempt counters, latest progress, latest stop reason, peer health, and downloaded or connected block evidence. | SATISFIED | Shared typed contract, projections, renderers, RPC tests, live-smoke fixture, docs, and checker all verified. |
| OBS-02 | 62-01, 62-03, 62-04 | Metrics and structured logs retain bounded long-run samples and cycle summaries while preserving progress/waiting/retry/stop/recovery diagnosis evidence. | SATISFIED | Metrics are five numeric sync samples; structured logs are compact label records; live-smoke persists compact snapshots and daemon output summary counts, not raw tails. |

No orphaned Phase 62 requirements were found in `.planning/REQUIREMENTS.md`; both listed Phase 62 requirements are claimed by plans and verified above.

## Anti-Patterns Found

| File | Line/Pattern | Severity | Impact |
|---|---|---|---|
| `scripts/run-live-mainnet-smoke.ts` | Internal `return null`, `return []`, and CLI `console.log` paths | Info | Expected parser/optional-control and command-output behavior, not stubs. Unavailable fields now preserve reasons/nulls and are fixture-tested. |
| `scripts/run-live-mainnet-smoke.ts` | Internal `stdoutTail`/`stderrTail` session buffers | Info | Internal bounded buffers remain to compute observed flags and line counts; persisted `SmokeReport` omits tail fields and fixture checks prove raw tails are not written. |
| `packages/open-bitcoin-node/src/sync/types/summary.rs` | Separate `sync stop reason=` record still exists | Info | Not a blocker: Phase 62 compact summary records now use `latest_stop_reason=` and `progress_signal=`, and checker/tests guard those labels. |

No blocker or warning-level stub patterns were found.

## Disconfirmation Pass

- Possible failure mode: structured logs could keep old labels while checker passes. Checked `summary.rs`, tests, and checker after WR-01; `latest_stop_reason=` and `progress_signal=` are now guarded.
- Possible failure mode: live-smoke unavailable progress or peer counts could still become zeroes. Checked mapping and fixture assertions after WR-02; unavailable scenario requires `null` and explicit unavailable reasons, and exits nonzero on zero substitution.
- Possible failure mode: default verification could accidentally run public-network live smoke. Ran the exact exclusion command; `scripts/verify.sh` has no forbidden strings and includes only the deterministic Phase 62 checker.

## Residual Risks

- Public-network live-smoke was not run. This is not a Phase 62 blocker because the phase contract explicitly keeps public-network evidence opt-in UAT and outside `bash scripts/verify.sh`.
- The verifier did not rerun the full `bash scripts/verify.sh` aggregate gate. `62-REVIEW-FIX.md` reports normal hooks passed for the review-fix commits, including that gate, and this verifier reran the required focused checks. The orchestrator is expected to run the final full gate.
- Worktree dirtiness remains limited to orchestration-owned `.planning/ROADMAP.md`, `.planning/STATE.md`, and `.planning/config.json`; this verifier did not modify or revert them.

## Gaps Summary

No gaps found. Phase 62 meets its roadmap goal and success criteria with deterministic evidence.

---

_Verified: 2026-06-07T00:27:11Z_
_Verifier: the agent (gsd-verifier)_
