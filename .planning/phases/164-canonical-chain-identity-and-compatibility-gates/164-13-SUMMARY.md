---
phase: 164-canonical-chain-identity-and-compatibility-gates
plan: "13"
subsystem: testing
tags: [rust, http, coupled-regression, explicit-fixtures]
requires:
  - phase: 164-05
    provides: Ordinary context guards and named fixture APIs
  - phase: 164-12
    provides: Earlier fixture caller migrations
provides:
  - Explicit HTTP fixture contexts
  - Actual coupled GREEN for the complete 39-path fixture migration and ten new canonical context tests
affects: [164-06, 164-07, 164-08, 165, 166, 167]
tech-stack:
  added: []
  patterns: [Serial timing-wrapped closure with nonzero test counts]
key-files:
  created: []
  modified:
    - packages/open-bitcoin-rpc/src/http/filter_index.rs
    - packages/open-bitcoin-rpc/src/http/tests.rs
    - packages/open-bitcoin-rpc/src/http/tests/filter_index.rs
    - packages/open-bitcoin-rpc/src/dispatch/tests/sync_status.rs
key-decisions:
  - Reuse the existing fixture store handle rather than reopen or restore swallowed errors.
  - Keep ordinary daemon and canonical tests guarded while historical regressions use explicit fixture factories.
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 164-2026-10-14T01-59-33Z
generated_at: "2026-10-14T04:50:47Z"
duration: 11min
completed: 2026-10-14
---

# Phase 164 Plan 13: HTTP Fixture and Coupled Closure Summary

**Five HTTP fixture calls complete the 39-path migration; all ten prescribed coupled commands pass with nonzero test discovery and benchmark compilation.**

## Accomplishments

Both tasks completed. The HTTP mixed-file change is confined to the existing `#[cfg(test)] initial_tests` module. Product HTTP code is unchanged. Across Plans 09–13, 65 runtime-config family calls plus 13 local operator alias calls migrated in 39 paths. All eight files containing legacy local operator aliases are covered. Original arguments, assertions, byte corpora, receipt digests, default flags and evidence labels remain intact, except the separately authorized existing-store argument addition documented below.

The final ordinary-call inventory contains only actual daemon construction, its unchanged `daemon_sync.rs` structural assertion and new canonical refusal tests. No ordinary constructor has a cfg-test bypass. Plan 04's already-applied daemon fixture opener adapters remain preserved. Synthetic fixture success does not authorize canonical production history.

## Actual Coupled Verification

Each command ran serially through `bun run scripts/command-timings.ts run --key phase164-fixtures-<key> --`. The Cargo arguments below are exact; all use `--manifest-path packages/Cargo.toml`. Logs are `/tmp/open-bitcoin-phase164-fixtures-<key>.log`, except the final dispatch rerun uses `dispatch-recheck.log`.

| Key | Cargo command after manifest path | Session | Actual result |
| --- | --- | --- | --- |
| canonical | `test -p open-bitcoin-rpc --lib canonical_chain` | 33652 | Exit 0; 11 passed, 0 failed, 0 ignored, 376 filtered; 9.99s. Ten new context tests plus one selector test. |
| context | `test -p open-bitcoin-rpc --lib context::` | 96524 | Exit 0; 49 passed, 0 failed, 0 ignored, 338 filtered; 22.44s. |
| config | `test -p open-bitcoin-rpc --lib config::tests` | Completed directly | Exit 0; 91 passed, 0 failed, 0 ignored, 296 filtered; 0.08s. |
| dispatch | `test -p open-bitcoin-rpc --lib dispatch::` | 8778, final | Exit 0; 86 passed, 0 failed, 0 ignored, 301 filtered; 11.18s. |
| inbound | `test -p open-bitcoin-rpc --lib inbound_listener::` | 61073 | Exit 0; 94 passed, 0 failed, 1 pre-existing ignored, 292 filtered; 42.55s. |
| http | `test -p open-bitcoin-rpc --lib http::` | 80094 | Exit 0; 19 passed, 0 failed, 0 ignored, 368 filtered; 8.33s. |
| daemon | `test -p open-bitcoin-rpc --bin open-bitcoind tests::` | 17712 | Exit 0; 112 passed, 0 failed, 1 pre-existing ignored, 0 filtered; 321.94s. |
| blackbox | `test -p open-bitcoin-rpc --test black_box_parity` | 37959 | Exit 0; 3 passed, 0 failed, 0 ignored, 0 filtered; 2.35s. No external Knots execution is inferred. |
| operator | `test -p open-bitcoin-cli --test operator_flows` | 52738 | Exit 0; 11 passed, 0 failed, 0 ignored, 0 filtered; 11.02s. |
| bench | `check -p open-bitcoin-bench --all-targets` | 95714 | Exit 0; benchmark all-target compilation passes. |

The new canonical tests establish actual ordinary refusal and preservation evidence: unbound/bound memory and durable owners, inconsistent selectors, ordinary aliases, actual daemon handle type, zero store-call spy, closed-source filesystem and durable record preservation, explicit fixture contracts and bounded diagnostics. This closes Plan 05's disclosed 86-error caller RED boundary; no remaining caller compile failures were found.

Actual daemon module discovery from session 17712:

| Module | Discovered | Passed | Ignored |
| --- | ---: | ---: | ---: |
| `tests::daemon_sync` | 14 | 14 | 0 |
| `tests::filter_index` | 46 | 45 | 1 |
| `tests::automatic_prune` | 7 | 7 | 0 |
| `tests::canonical_chain` | 8 | 8 | 0 |
| `coins_flush` | 15 | 15 | 0 |

The integrated Phase 162 lifecycle passed. Ignored child controls predate this work and remain exercised by their existing bounded subprocess drivers. No test was newly ignored. Long-running daemon work retained timing-wrapper heartbeats and active CPU evidence; sessions were polled within 60 seconds and none was terminated.

## Source Verification and Simplification

Scoped `rustfmt --check --edition 2024 --config skip_children=true` passed across all 39 caller paths. `git diff --check` and managed file-length checks passed with zero findings. Mechanical-equivalence comparison against HEAD passed for Plans 10–13 before the documented handle fix, normalizing only factory names, whitespace and formatter commas. Plan 09's exact diff was reviewed separately to distinguish preserved Plan 04 imports from this wave's factory substitutions. Final HTTP and store-helper diffs were reviewed.

The simplification pass retained direct named factory substitutions; the one shared-store fix avoids redundant backend open. No new abstraction, production dependency, scope boolean, config/env bypass, endpoint or auth surface was added. No goal-blocking stubs or authentication gates were introduced. Local AGENTS guidance, sidecar, overrides, architecture/code-shape/verification/testing/Rust standards and both complete active lesson inputs informed execution.

## Deviations from Plan

**[Rule 3 - Coupled fixture handle ownership] Reused the sync-status fixture's open store.** Initial dispatch session 34679 exited 101: 84 passed and 2 failed, both Phase 72 tests in `dispatch/tests/sync_status.rs`. `context_with_runtime_metadata` held a live Fjall store and attempted a second raw open through the new fixture wallet helper. The previous wallet fallback swallowed that lock error; the new helper correctly propagates it. Parent explicitly authorized the owned caller fix: `from_fixture_runtime_config_with_store(&same_config, Some(store.clone())).expect(...)`. Configuration, metadata, daemon control, core guards and every original assertion remain unchanged. No swallowed error was restored.

Focused repair command:
`bun run scripts/command-timings.ts run --key phase164-fixtures-sync-status-repair -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --lib dispatch::tests::sync_status::`
Session 28394 exited 0: 4 passed, 0 failed, including both original failures; 2.32s. Full dispatch then passed 86 tests in session 8778 before continuing the serial prescribed checks. Plan 11's summary records this same later closure correction; it is one deviation, not two fixes.

## Task Commits and Remaining Phase Work

Tasks 1 and 2: deferred parent-owned strict phase finalization. No staging, commits, pushes, hook changes, worktrees or shared STATE/ROADMAP/REQUIREMENTS/todo mutations were performed. Requirements remain incomplete here.

No fixture migration or prescribed coupled closure work remains. Parent owns full default native verification, independent review/security, source capability checks and normal-hook Git finalization. These focused regression passes do not imply public-network acceptance, canonical bootstrap/work validation, whole-chain parity or production/funds readiness.

## Self-Check: PASSED

All 39 caller paths and summaries 09–13 exist. All nine prescribed test logs contain nonzero successful results; all ten prescribed commands exited zero, including benchmark compilation. Actual canonical and daemon module counts were extracted from logs. Source formatting, whitespace, file lengths and ordinary-call inventory passed. Commit verification is intentionally inapplicable under parent-owned finalization.
