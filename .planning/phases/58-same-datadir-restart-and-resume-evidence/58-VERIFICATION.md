---
phase: 58-same-datadir-restart-and-resume-evidence
verified: 2026-06-05T14:03:25Z
status: passed
score: "11/11 must-haves verified"
generated_by: gsd-verifier
lifecycle_mode: yolo
phase_lifecycle_id: 58-2026-06-05T12-58-05
generated_at: 2026-06-05T14:03:25Z
lifecycle_validated: true
overrides_applied: 0
requirements_verified: [RESUME-01, RESUME-02, RESUME-03]
re_verification:
  previous_status: failed
  previous_score: not_recorded
  gaps_closed:
    - "Regenerated docs/metrics/lines-of-code.md and confirmed the LOC freshness check passes."
    - "Accepted orchestrator evidence that bash scripts/verify.sh passed after LOC regeneration."
  gaps_remaining: []
  regressions: []
---

# Phase 58: Same-Datadir Restart and Resume Evidence Verification Report

**Phase Goal:** Prove that the same public-mainnet datadir resumes from durable header or block progress after interruption.
**Verified:** 2026-06-05T14:03:25Z
**Status:** passed
**Re-verification:** Yes - after stale LOC report gap closure

## Context And Provenance

- Previous `58-VERIFICATION.md` was checked. It failed only because `bash scripts/verify.sh` reported a stale `docs/metrics/lines-of-code.md`.
- `58-CONTEXT.md`, all three `58-*-PLAN.md` files, all three `58-*-SUMMARY.md` files, `58-REVIEW.md`, and this report share `lifecycle_mode: yolo` and `phase_lifecycle_id: 58-2026-06-05T12-58-05`.
- No `direct-fallback` provenance was found in the phase artifacts.
- Repo-local rules read: `AGENTS.md`, `AGENTS.bright-builds.md`, and `standards-overrides.md`. No project skills were present under `.claude/skills` or `.agents/skills`.
- Canonical standards files were not present in this checkout; verification applied the available repo-local and Bright Builds sidecar rules.

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
| --- | --- | --- | --- |
| 1 | Operator can interrupt and restart the same v1.4 public-mainnet datadir after observed progress and see sync resume without duplicate block connects. | VERIFIED | `scripts/run-live-mainnet-smoke.ts` parses `--restart-after-progress`, runs a first session until progress, launches a second session with the same datadir, and passes only when durable heights and unchanged-height hashes are preserved. Rust same-datadir tests assert no duplicate connected block request/connect after reopen. |
| 2 | Live-smoke reporting captures before/after restart evidence for header height, block height, runtime phase, peer outcomes, and latest progress timestamp. | VERIFIED | `RestartResumeEvidence` includes `beforeRestart`, `afterRestart`, `peerOutcomeSummary`, `restartStatus`, `sameDatadir`, `duplicateConnectVerdict`, and `maybePostRestartProgressDelta`; status parsing reads `maybeLastSuccessfulProgressUnixSeconds`. |
| 3 | Recovery guidance distinguishes peer incompatibility, public-network unreachability, invalid peer data, store corruption, store incompatibility, resource exhaustion, and cancellation. | VERIFIED | `RecoveryDiagnosisCategory` defines all seven categories, and `scripts/test-run-live-mainnet-smoke.sh` covers the six fixture-matrix categories plus the cancellation case. |
| 4 | Deterministic restart/resume tests cover durable state transitions without public-network access. | VERIFIED | `same_datadir_reopen_*` tests use temp Fjall stores, `DurableSyncRuntime::open`, and `ScriptedTransport`; public-network live smoke is not invoked by `scripts/verify.sh`. |
| 5 | DurableSyncRuntime can reopen the same Fjall datadir after header-only progress and report persisted header state. | VERIFIED | `same_datadir_reopen_seeds_headers_from_durable_store` saves headers, reopens `FjallNodeStore::open(&path)`, opens `DurableSyncRuntime`, and asserts `best_header_height` plus durable sync status. |
| 6 | DurableSyncRuntime can reopen after downloaded and connected block progress and preserve downloaded and connected height/hash evidence. | VERIFIED | `same_datadir_reopen_reports_downloaded_and_connected_block_hashes_after_partial_download` asserts `downloaded_block_height`, `connected_block_height`, `maybe_downloaded_block_hash`, and `maybe_connected_block_hash` in summary and durable metadata. |
| 7 | Already connected blocks are not requested or connected again after same-datadir reopen. | VERIFIED | `same_datadir_reopen_does_not_duplicate_connected_block_getdata` asserts `summary.blocks_received == 0`, empty `getdata_block_hashes(&transport.sent_messages())`, and stable connected block hash after reopen. |
| 8 | The live-smoke report contains compact `result.restartResumeEvidence` that proves the restart boundary and same-datadir resume. | VERIFIED | Fixture assertions require `"restartResumeEvidence": {`, completed status, same-datadir booleans, before/after summaries, duplicate verdict, and post-restart delta; a JSON check rejects forbidden raw fields inside the compact evidence object. |
| 9 | Fresh post-relaunch `openbitcoinsyncstatus` snapshots are collected without requiring fresh public-network progress after restart. | VERIFIED | The restart path runs a second `first_snapshot` session and reports pass on preserved durable state; `maybePostRestartProgressDelta` may be zero. |
| 10 | Operator docs provide copy-pasteable Cargo, Bazel, and opt-in live-smoke commands for same-datadir restart/resume review. | VERIFIED | `docs/operator/runtime-guide.md` includes the `bun run ... --restart-after-progress` command, repo-local Cargo and Bazel `sync status --format json` commands, pass/fail fields, and all seven recovery categories. |
| 11 | Parity docs describe explicit restart/resume evidence without broadening into unattended production operation or support-bundle work. | VERIFIED | `docs/parity/catalog/p2p.md` documents opt-in `result.restartResumeEvidence` and preserves known gaps for unattended public-network full sync and service-manager restart policy. |

**Score:** 11/11 truths verified

## Required Artifacts

| Artifact | Expected | Status | Details |
| --- | --- | --- | --- |
| `packages/open-bitcoin-node/src/sync/tests.rs` | Deterministic same-datadir resume tests using real Fjall reopen and ScriptedTransport | VERIFIED | `gsd-tools verify artifacts` passed. Manual checks found header, block hash, no-duplicate, and best-branch same-datadir tests. |
| `scripts/run-live-mainnet-smoke.ts` | Restart option, two-session daemon orchestration, restart evidence schema, recovery diagnosis, Markdown rendering | VERIFIED | `gsd-tools verify artifacts` passed. Manual checks found restart flag parsing, `openbitcoinsyncstatus`, restart evidence derivation, diagnosis categories, and report output wiring. |
| `scripts/test-run-live-mainnet-smoke.sh` | Deterministic mocked two-session restart fixture and diagnosis matrix | VERIFIED | `gsd-tools verify artifacts` passed. Fixture checks assert restart success, regression failures, forbidden compact evidence fields, all recovery categories, and cancellation. |
| `docs/operator/runtime-guide.md` | Same-datadir restart/resume commands and pass/fail guidance | VERIFIED | `gsd-tools verify artifacts` passed. Docs include live-smoke, Cargo, Bazel, pass/fail fields, zero-delta guidance, and recovery categories. |
| `docs/parity/catalog/p2p.md` | Scoped P2P parity wording for restart/resume evidence | VERIFIED | `gsd-tools verify artifacts` passed. Docs keep the opt-in restart/resume claim and explicit unattended/service-manager gaps. |
| `docs/metrics/lines-of-code.md` | Fresh tracked generated LOC report | VERIFIED | `bun run scripts/generate-loc-report.ts --source=worktree --output=docs/metrics/lines-of-code.md --check` passed with `LOC report is current`. |

## Key Link Verification

| From | To | Via | Status | Details |
| --- | --- | --- | --- | --- |
| `packages/open-bitcoin-node/src/sync/tests.rs` | `DurableSyncRuntime::open` | `FjallNodeStore::open(&path)` on the same temp path before and after reopen | VERIFIED | Manual line-level checks show same-path store open followed by `DurableSyncRuntime::open` in each same-datadir test. The automated regex missed this because calls are split across lines. |
| `packages/open-bitcoin-node/src/sync/tests.rs` | getdata duplicate guard | `ScriptedTransport::sent_messages` and `getdata_block_hashes` | VERIFIED | `same_datadir_reopen_does_not_duplicate_connected_block_getdata` and best-branch reopen tests inspect sent messages and require no connected-block `getdata`. |
| `scripts/run-live-mainnet-smoke.ts` | `openbitcoinsyncstatus` | Fresh status command for daemon sessions | VERIFIED | `statusCommandForRpcPort` emits `openbitcoinsyncstatus`; restart mode runs a second session and stores both session command specs in the report. |
| `scripts/test-run-live-mainnet-smoke.sh` | `scripts/run-live-mainnet-smoke.ts` | Mock daemon/status binaries and grep assertions | VERIFIED | Shell fixtures invoke the script with `--restart-after-progress` and verify JSON/Markdown restart evidence. |
| `scripts/run-live-mainnet-smoke.ts` | `docs/operator/runtime-guide.md` | Shared diagnosis category names and pass/fail fields | VERIFIED | Script and docs share the seven recovery categories and restart evidence field names. |
| `docs/parity/catalog/p2p.md` | `scripts/run-live-mainnet-smoke.ts` | `result.restartResumeEvidence` schema field | VERIFIED | Parity docs name the schema field and keep it scoped to opt-in operator evidence. |
| `scripts/verify.sh` | Public-network live-smoke exclusion | No `run-live-mainnet-smoke` or `--restart-after-progress` invocation | VERIFIED | `rg -n "run-live-mainnet-smoke|--restart-after-progress" scripts/verify.sh` returned no matches. |

## Data-Flow Trace (Level 4)

| Artifact | Data Variable | Source | Produces Real Data | Status |
| --- | --- | --- | --- | --- |
| `scripts/run-live-mainnet-smoke.ts` | `SyncStatusSnapshot` | `readSyncStatus` executes the status command and parses `metadata.maybe_sync_state` from `openbitcoinsyncstatus` JSON | Yes, from daemon status or deterministic fixture status JSON | FLOWING |
| `scripts/run-live-mainnet-smoke.ts` | `restartResumeEvidence` | `restartResumeEvidence(...)` consumes first-session and second-session snapshots, endpoint outcomes, and final status | Yes, derived from session results and checked by fixture assertions | FLOWING |
| `scripts/run-live-mainnet-smoke.ts` | `recoveryDiagnosis` | `recoveryDiagnosis(...)` consumes endpoint outcomes, final status peer failures, last errors, and restart status | Yes, fixture matrix proves category selection and storage-first precedence | FLOWING |
| `docs/operator/runtime-guide.md` | Operator pass/fail fields | Static documentation of script report fields and repo-local commands | N/A for static docs | VERIFIED |
| `docs/parity/catalog/p2p.md` | Scoped parity wording | Static documentation tied to `result.restartResumeEvidence` and known gaps | N/A for static docs | VERIFIED |

## Behavioral Spot-Checks

| Behavior | Command | Result | Status |
| --- | --- | --- | --- |
| LOC report freshness gap is closed | `bun run scripts/generate-loc-report.ts --source=worktree --output=docs/metrics/lines-of-code.md --check` | `LOC report is current: docs/metrics/lines-of-code.md` | PASS |
| Restart flag is exposed to operators | `bun run scripts/run-live-mainnet-smoke.ts --help \| rg -n -- "Usage:\|--restart-after-progress"` | Help output includes `--restart-after-progress` | PASS |
| Public-network live smoke remains outside default verification | `rg -n "run-live-mainnet-smoke\|--restart-after-progress" scripts/verify.sh` | No matches | PASS |
| Deterministic same-datadir Rust tests pass | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node same_datadir --all-features` | Passed per orchestrator evidence | PASS |
| Restart-focused Rust tests pass | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node restart --all-features` | Passed per orchestrator evidence | PASS |
| Live-smoke fixture regression suite passes | `bash scripts/test-run-live-mainnet-smoke.sh` | Passed per orchestrator evidence | PASS |
| Repo-native aggregate verification passes | `bash scripts/verify.sh` | Passed per orchestrator evidence after LOC regeneration | PASS |

## Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
| --- | --- | --- | --- | --- |
| RESUME-01 | 58-01, 58-02 | Operator can interrupt and restart the same v1.4 public-mainnet datadir after observed header or block progress and see sync resume from durable state without duplicating block connects. | SATISFIED | Deterministic same-datadir Rust tests prove durable header/block resume and no duplicate `getdata`/connect; smoke runner exposes the two-session same-datadir restart flow. |
| RESUME-02 | 58-02, 58-03 | Live-smoke reporting can capture same-datadir before/after restart evidence for header height, block height, runtime phase, peer outcomes, and latest progress timestamp. | SATISFIED | `result.restartResumeEvidence` contains before/after summaries, same-datadir booleans, peer outcome summary, duplicate verdict, restart status, and recovery diagnosis; fixture tests assert JSON/Markdown output. |
| RESUME-03 | 58-03 | Recovery guidance distinguishes transient peer incompatibility, public-network unreachability, invalid peer data, store corruption, store incompatibility, resource exhaustion, and intentional cancellation. | SATISFIED | `RecoveryDiagnosisCategory` defines all seven categories, fixture matrix covers them, and operator docs list them with storage-first guidance. |

No additional Phase 58 requirement IDs were found in `.planning/REQUIREMENTS.md` beyond RESUME-01, RESUME-02, and RESUME-03.

## Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
| --- | --- | --- | --- | --- |
| `scripts/run-live-mainnet-smoke.ts` | multiple | Nullable defaults, `return null`, `return []`, and CLI `console.log` | INFO | Normal TypeScript option parsing, nullable report fields, status absence handling, and CLI output. These values are populated by status/session flows where required and are not hollow user-visible stubs. |
| `scripts/test-run-live-mainnet-smoke.sh` | 971 | `console.log` inside `bun --eval` | INFO | Test helper prints compact restart evidence JSON for forbidden-field inspection. |

No blocker stubs, TODO placeholders, hollow prop/data flows, or console-log-only implementations were found in the Phase 58 delivery surfaces.

## Human Verification Required

None for the Phase 58 verification contract. A real public-mainnet same-datadir run remains optional operator UAT and intentionally stays outside `bash scripts/verify.sh`; deterministic fixtures and repo-native verification cover the phase deliverables.

## Gaps Summary

No open gaps. The previous stale LOC report blocker is closed, Phase 58 requirements RESUME-01 through RESUME-03 are satisfied, and lifecycle/report provenance is coherent.

---

_Verified: 2026-06-05T14:03:25Z_
_Verifier: the agent (gsd-verifier)_
