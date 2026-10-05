---
phase: 157-safe-activation-and-scheduled-index-catch-up
plan: "08"
subsystem: rpc
tags: [rust, daemon, basic-filters, offline-maintenance, fjall]
requires:
  - phase: 157-01
    provides: Typed explicit BASIC option resolution
  - phase: 157-02
    provides: Configured pre-prune activation and history refusal
  - phase: 157-07
    provides: Configured first turn and public bounded ordinary turn
provides:
  - Actual daemon durable selection and explicit Enabled/Disabled startup policy
  - Typed index maintenance inside the existing Periodic/Always worker
  - Offline injected idle, disable, shutdown and genuine checkpoint/reopen controls
affects: [157-10, CFAC-01, CFIX-01]
tech-stack:
  added: []
  patterns: [typed durable/no-index adapter, single existing maintenance owner]
key-files:
  created:
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index.rs
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/fixtures.rs
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/coins_flush/tests/filter_index.rs
  modified:
    - packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/coins_flush.rs
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/coins_flush/tests.rs
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests.rs
key-decisions:
  - Use configured open's existing first turn without a duplicate daemon turn.
  - Keep transient workers typed as no-index rather than downcasting storage.
  - Stop index scheduling before Always while preserving saved Active protection.
  - Retain idle index errors through shutdown while still running Always cleanup.
requirements-completed: []
requirements-addressed: [CFAC-01, CFIX-01]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 157-2026-10-05T14-50-48
generated_at: 2026-10-05T21:15:35Z
duration: 25min
completed: 2026-10-05
commits: []
git_finalization: pending root whole-phase native verification and strict finalization
---

# Phase 157 Plan 08: Actual Offline Daemon Activation and Maintenance Summary

**Resolved BASIC configuration now reaches actual durable daemon startup before prune recovery, and the existing maintenance worker advances one bounded index turn per elapsed tick with preserved coins and shutdown ownership.**

## Performance and Scope

- First recorded check: **2026-10-05T20:50:03.937Z**. Final affected startup check: **21:12:33.582Z**; summary finalization: **21:15:35 UTC**. The approximately **25-minute** verification/documentation window includes cooperative lock waits and excludes earlier read-only preparation and uninstrumented initial test authoring.
- Tasks: **2/2 complete**. **Seven owned Rust files**, including **three new test modules**, plus this summary.
- All Cargo checks used pinned Bun 1.3.9 and the command-timings cooperative lock. No Cargo target overlap or enumeration of the quarantined target directory occurred.
- No staging, commit, push, shared STATE/ROADMAP/REQUIREMENTS/config mutation or requirement activation occurred. Root owns strict finalization.

## Task Outcomes

### Task 1: Configured actual daemon startup

`open_runtime_store` selects durable storage for explicit BASIC or explicit zero when the datadir exists, independently of sync, inbound and prune activation. BASIC without an existing directory refuses before opening or creating storage. Omission with no other durable trigger stays transient and leaves saved ownership untouched. Existing sync/inbound/prune triggers retain their selection behavior.

Every actual durable daemon open calls `open_with_configured_runtime_activation` with **Enabled** for BASIC and **Disabled** for omission/zero. The mode arrives before initialization and prune resume. Plan 07's configured constructor already performs the first bounded turn; this daemon adds no second startup turn.

Actual loader forms bare/1/basic select durable offline history. Tests prove absent/nonexistent datadir refusal, enabled empty-store refusal, missing required body versus undo at height 12, preserved source values and owner absence, explicit zero and omitted-with-prune-trigger saved disable, complete saved checkpoint preservation through zero/reopen, and omitted/no-trigger transient preservation. Actual sync/inbound worker selectors remain disabled with no listener endpoint; peer counts remain zero and compact-filter service bit 6 remains absent.

The root-approved startup observation is indirect: after actual startup, one public ordinary turn reports **eight generations**, processed endpoint **15**, and **initially_synchronized=false** on a 40-block history. Combined with the existing eight-block first-turn/default contract, this proves one strict eight-row startup prefix without a duplicate turn. Daemon tests do not directly read private immutable rows or private progress getters. Plan 07 owns its exact first-turn observation.

### Task 2: Existing idle and shutdown owner

`BasicIndexMaintenance` has concrete durable and transient/no-index implementations. The existing generic worker uses this typed bound; no downcast, additional service or deletion worker was added. Each elapsed event executes ordinary Periodic maintenance and then at most one parameter-free index turn, returning to the existing one-second injected wait.

The shared production elapsed body yields ordered prefixes **15, 23, 31, 39, 40** after a genuine ordinary accepted connect extends a behind index from target 39 to 40. Generation counts are **8, 8, 8, 8, 1**, with at most eight body probes, one persistence batch and the declared record/checkpoint/projection work caps. Authorities remain usable between turns with zero peers and no receives. Initial completion stays false until the expanded target is processed.

The genuine old durable B39 can earn safe height 39 even while accepted B40 remains unflushed; it cannot earn safe height 40. A later due **Periodic** checkpoint writes B40, and the next ordinary zero-generation/zero-body turn earns safe height 40 and advances reserved protection to **FromHeight(41)**. Advancing that frontier releases historical inputs while continuing to protect future inputs. Actual closed Fjall/configured reopen preserves the earned checkpoint and reports zero further generation.

Shutdown stops scheduling before **Always**, runs no shutdown index turn, preserves saved Active protection and joins the existing worker. A separate genuine accepted/unflushed control proves **Always advances actual B39 to B40 before closed reopen**. Explicit disable uses the existing generation-invalidating lifecycle. A stale elapsed turn after disable returns a typed underlying index error, yields to the next wait and still executes Always cleanup; the loop retains that error through shutdown rather than returning index success. An achieved later successful turn may clear a prior error. Existing checkpoint/clean-marker ordering remains intact.

## Verification and TDD Evidence

Commands use `PATH=/tmp/open-bitcoin-bun-1.3.9/bun-darwin-aarch64:$PATH bun run scripts/command-timings.ts run --key KEY -- cargo ...` with `--manifest-path packages/Cargo.toml -p open-bitcoin-rpc`. Counts overlap and are not a distinct-test sum.

| Boundary                                                            | Actual result                                                                                                                                                |
| ------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Task 1 behavioral RED, before daemon option wiring                  | **6 executed: 1 passed, 5 failed**, 43 filtered; **4.65s** harness; wrapper **36,352ms**, including a cooperative wait                                       |
| Task 1 initial GREEN                                                | **6 passed**, 0 failed; **7.96s** harness; wrapper **9,858ms**                                                                                               |
| Task 2 behavioral RED, before elapsed index wiring                  | **2 executed: 1 passed, 1 failed**, 49 filtered; **2.30s** harness; elapsed ticks left processed height 15 instead of 31                                     |
| Final expanded startup, after the positive-predicate simplification | `test --bin open-bitcoind phase157_daemon_`: **9 passed**, 0 failed, 50 filtered; **9.74s** harness; **13,391ms** wrapper, **21:12:20.191–21:12:33.582 UTC** |
| Final expanded idle                                                 | `test --bin open-bitcoind phase157_idle_`: **7 passed**, 0 failed, 52 filtered; **4.47s** harness; **7,345ms** wrapper                                       |
| Inherited daemon regression                                         | `test --bin open-bitcoind`: **59 passed**, 0 failed/ignored/filtered; **139.05s** harness; **139,674ms** wrapper                                             |
| Normal production/test diagnostics                                  | `clippy --all-targets --all-features -- -D warnings`: **passed**, **1,553ms** wrapper after fixing the introduced nonminimal boolean expression              |
| Owned formatting and diff                                           | Rust 2024 `rustfmt --check --config skip_children=true` on all seven owned paths and scoped `git diff --check`: passed                                       |

The inherited regression includes original Periodic/Always owner and jitter tests, checkpoint/clean-marker tests, existing sync/inbound/retry controls, and genuine ordinary automatic retention plus index protection and paired deletion after explicit disable. Its existing network controls use local loopback; no public-network test was run. The 59-test regression preceded only the logically equivalent named-positive datadir predicate cleanup; all nine affected startup controls and strict Clippy passed after that cleanup. No further broad rerun was needed. Root owns full native verification, coverage, Bazel and phase source/security/lifecycle review.

## Deviations and Issues Encountered

- Root explicitly approved public-outcome/protection observations instead of widening private node getters or adding a Fjall test dependency. The first two startup attempts were test compilation failures from private API assumptions and a progress method spelling, not behavioral RED evidence. Once corrected, the six registered behavioral controls produced the RED above.
- **[Rule 1 - Test fixture]** Two parallel fixtures initially received the same clock-derived temp path. A local atomic fixture-label counter fixes uniqueness without changing shared helpers or production. The failed run was 1/2 controls; subsequent runs pass.
- Two expanded idle expectations were corrected to the existing contract: safe B39 may advance before accepted B40 flush, and a caught-up checkpoint advances protection to the next height rather than removing future protection. These were test expectation errors, not claimed production bug REDs.
- The first expanded missing-undo control unintentionally restored the omitted undo through `seed_coins_from_snapshot`. The fixture now removes the intended undo from its test snapshot before seeding and relies on the existing seeder for remaining undo. This also removes redundant explicit undo writes. Final body/undo refusal controls pass.
- **[Rule 1 - Introduced lint]** Strict normal Clippy rejected the new negated compound datadir condition. A named positive `has_durable_trigger` predicate preserves behavior and simplifies the selection boundary; no warning suppression was added.

No production scope expansion, architectural change or authentication gate was required. Per-task/TDD/metadata Git finalization is intentionally deferred under the strict phase wrapper.

## Simplification, Threat Review and Limits

The simplification pass keeps two production files, one existing thread/loop and small typed adapters. It reuses configured open's first turn, existing accepted-connect API, genuine flush-cycle API and owned outcome. There is no index-owned history cache, runtime downcast, new dependency/crate, schema, network activation, RPC/service endpoint, status projection, import or repair path. All owned source files remain below 628 lines.

AGENTS, its Bright Builds sidecar, placeholder-only overrides, architecture/code-shape/testing/verification/local-guidance/Rust standards and both active lesson sources informed execution. Active lessons were completely loaded within **7,188 bytes / 2,397 estimated tokens**; no archives or new audit trigger were used. Project skill directories are absent.

- **T-157-23:** explicit startup mode precedes configured initialization; missing datadir/history/empty-store controls and actual network/worker/service-bit observations pass.
- **T-157-24:** the same production elapsed body and injected loop perform one bounded turn per tick and return to wait; authorities remain usable between turns without peer receives.
- **T-157-25:** genuine Periodic and Always fences, preserved Active shutdown protection, actual join, explicit disable/stale refusal and retained error ordering pass; inherited genuine automatic-retention controls remain green.

No new unmodeled trust boundary or goal-blocking stub was found. Fixture histories are continuously consensus-staged/committed with easy synthetic headers and test-only maturity one, including historical and same-block spends; they are not public-mainnet histories. Missing-history daemon fixtures intentionally omit storage inputs; actual paired-loss evidence belongs to Plans 02/09. Public protection snapshots do not claim raw key/value equality of all private index rows. The full required-history startup preflight, ordinary coins/prune checkpoints and storage latency remain outside a hard bounded-latency claim; Plan 07's admitted BASIC turn ledger supplies the bounded index work. New-path breadcrumb comments already cite the pinned index/base, index/blockfilterindex and node/blockstorage sources; Plan 10 owns consolidated manifest registration, docs/UAT and claim guards.

## Task Finalization and Next Plan Readiness

Both declared tasks are implemented and verified. No commits or hashes are claimed. CFAC-01/CFIX-01 remain pending whole-phase verification, with `requirements-completed: []`. No user setup is required. Plan 10 can close source/claim guardrails and documentation; root can perform native verification and strict Git finalization.

## Self-Check: PASSED

All seven owned source files and this summary exist. The final affected startup, idle, inherited daemon and strict normal Clippy checks passed with the exact nonzero counts above. Final owned Rust formatting and diff whitespace checks passed. Stub/threat scans found no blocking stub or unmodeled surface. The document retains exactly two standalone frontmatter delimiters and the originating lifecycle identity. Installed GFM/frontmatter extensions were checked before scoped Markdown formatting, and the final formatter check passed. Commit-existence checks are inapplicable because `commits: []` and strict root finalization remain explicit; shared state and requirement activation stay root-owned.
