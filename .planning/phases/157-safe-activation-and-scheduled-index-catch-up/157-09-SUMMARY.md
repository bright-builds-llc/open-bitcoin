---
phase: 157-safe-activation-and-scheduled-index-catch-up
plan: "09"
subsystem: testing
tags: [rust, fjall, actual-paired-loss, accepted-faults, ordinary-retention]
requires:
  - phase: 157-02
    provides: Configured suffix preflight before lifecycle/reconciliation/prune effects
  - phase: 157-05
    provides: Budgeted sealed append and exact durable-tip checkpoint publication
  - phase: 157-06
    provides: Ordinary accepted-state complete facts and persistence-error handoff
  - phase: 157-07
    provides: Continuous validated history and ordinary bounded turn
provides:
  - Genuine keep-window paired-loss fresh/saved refusal with complete raw namespace equality
  - Ordinary accepted persistence failures, truthful reopen and immutable identical retries
  - Periodic/Always checkpoint linkage and actual 550 MiB automatic retention
affects: [157-10, phase-verification]
tech-stack:
  added: []
  patterns: [test-only-shared-history, exact-raw-refusal-snapshots, streamed-volume-commitments]
key-files:
  created:
    - packages/open-bitcoin-node/src/sync/tests/filter_index/evidence.rs
    - packages/open-bitcoin-node/src/sync/tests/filter_index/evidence/history_loss.rs
    - packages/open-bitcoin-node/src/sync/tests/filter_index/evidence/accepted_faults.rs
    - packages/open-bitcoin-node/src/sync/tests/filter_index/evidence/retention.rs
  modified:
    - packages/open-bitcoin-node/src/sync/tests/filter_index.rs
    - packages/open-bitcoin-node/src/sync/tests/filter_index/catch_up.rs
    - packages/open-bitcoin-node/src/sync/tests/filter_index/catch_up/fixtures.rs
key-decisions:
  - "Reuse the continuously staged/committed fixture through minimal test-only sibling visibility."
  - "Preserve default connect cadence; real later normal flush and requested-body failures prove accepted-state errors."
  - "Earn all-payload accounting through actual nonactive codec body volume while active deletion candidates remain continuously validated."
  - "Compare complete raw values for refusal; stream complete body-key/length/SHA256d commitments for the large-volume reopen control."
requirements-completed: []
requirements-addressed: [CFAC-02, CFIX-01]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 157-2026-10-05T14-50-48
generated_at: 2026-10-05T21:09:00Z
duration: 18min
completed: 2026-10-05
commits: []
git_finalization: pending root whole-phase verification and strict finalization
---

# Phase 157 Plan 09: Real Store Loss, Failure and Retention Evidence Summary

**Continuous validated histories now prove fresh/saved missing-history refusal after achieved paired deletion, ordinary accepted persistence errors, exact reopen preservation and normal checkpoint-owned retention.**

## Performance and Scope

- **2/2 tasks complete**, with **14 registered nonignored Rust tests**: seven history-loss, four failure and three retention controls. Parameter loops exercise additional cases; counts below are registered tests, not case totals.
- First source creation: **2026-10-05T20:51:13Z**. Final lint completed **21:06:27.096Z**; summary/self-check at **21:09:00Z**, approximately **18 minutes**, excluding read-only preflight and implementation preparation.
- **7 Rust paths**, including **4 new files**, plus this summary. The root-approved scope amendment permits only minimal sibling visibility in the existing catch-up registry/fixture.
- No production code, dependency, crate, storage schema, public endpoint, networking activation or policy change. No Git/staging/commit/push or shared STATE/ROADMAP/REQUIREMENTS/config write. Requirements remain unactivated until root verification.

## Task 1: Actual History Loss and Nonmutation

The shared `TurnHistory` continuously stages and commits all **400 active blocks**, including historical and same-block spends. The existing serialized `flush_applying_prune_plan` path performs a full checkpoint and real paired deletion outside the trailing 288-block keep window. Fresh controls delete genesis or spending height 20; the saved Disabled control deletes required suffix height 20 while preserving its height-15 immutable prefix and ordinary wallet retention lock. Achieved deletion hashes, absent body/undo and earned have-pruned are asserted before all handles close.

Configured Enabled reopen then refuses the missing required body. Complete raw key/value snapshots cover **every BlockIndex, Chainstate and Coins row**, including immutable records, active projections, saved checkpoint/fence, lifecycle generation, reserved/ordinary locks, live intent, surviving body/undo payloads and coins B/H. Refusal comparisons are byte-for-byte equality after genuine closed-store reopen.

Independent missing-body and missing-non-genesis-undo controls remove only one mate, preserving the other. They retain a legal old indexed prune intent and prove source diagnosis precedes resumed deletion. Ahead immutable suffix rows cannot substitute for the old saved checkpoint. A deliberately seeded, separately validated replacement branch recovers a common prefix at height 1; its missing height-2 body refuses before the saved original height-15 authority can reconcile. This replacement seed is **not runtime-reorg evidence**.

The positive indexed-loss case achieves a legal height-1 paired delete, seeds a stronger covering lock, and injects `AfterDisable` at the actual lifecycle publication boundary. Configured re-enable preflights only the still-required suffix, preserves the stronger lock through incomplete work, completes the safe suffix and reopens without rereading the deleted indexed body/undo. Surviving Chainstate/Coins rows remain identical. A separate genesis control explicitly removes the fixture's optional persisted empty genesis undo and earns exact filters through the tip using the genuine no-undo rule.

These manual owner tests exercise the existing concrete plan application, keep-window, protection and paired-delete path. They do not claim an authenticated manual-prune RPC or network prune-after option acceptance flow.

## Task 2: Accepted Errors, Retry and Normal Retention

The ordinary direct `connect_local_block` consumer accepts the historical/same-block spending block with the unchanged default IfNeeded cadence. Actual later **Always** requests inject **BeforeUndo, BeforeCoins and BeforeChainMeta** in the concrete writers. The error caller retains accepted target, complete spent scripts, conservative saved checkpoint and covering protection. A poisoned owner cannot publish later work first. BeforeChainMeta occurs after actual new coins B with old metadata; configured reopen refuses and preserves all three raw namespaces. BeforeUndo/BeforeCoins recover the valid old prefix without phantom rows.

The ordinary requested peer consumer genuinely requests and accepts the body, then **BeforeBody** fails its real post-accept save. Its ordinary peer error report credits no received block, while accepted target and complete facts survive. Old durable authority reopens truthfully without that failed future body. No zero-cache force-flush seam or invented pressure policy is used by these tests.

The bounded append matrix covers **BeforeRecords, BeforeCheckpoint, BeforeProtection and AfterCommit**. Pre-commit cases retain 24 immutable rows; AfterCommit retains 32 despite the error. Genuine reopen replays from safe height 15; identical retries preserve every existing immutable key/value, eventually produce exactly 40 rows and earn safe height 39. Repeated closed reopen/idle retry leaves these rows unchanged. A real budgeted prepared capability followed by trusted disable cannot publish or release through stale completion; Disabled reopen preserves complete raw bytes.

The accepted-unflushed control processes height 2 ahead of coins B at height 1, then closes every handle. Reopen loses only the uncheckpointed accepted tip, retains its immutable record, hides its active projection and preserves the conservative checkpoint. This explicitly preserves the Phase 142 software loss boundary instead of introducing forced connect flushes.

For normal **Periodic and Always** linkage, the test derives coins/metadata at height **19** by genuinely disconnecting the validated history using every corresponding undo. Ordinary default-policy acceptance then connects **20 through 400**. Replay processes height 400 while safe progress remains 19; required height 20 is legal under the current keep window but actual deletion refuses. A deterministic due timestamp drives the existing normal checkpoint to 400, followed by one zero-record bounded safe release and genuine paired height-20 deletion. Closed reopen proves new coins B, absent sources and retained filter. No cadence, cache size, threshold or accounting override occurs.

Automatic retention uses **1,002 continuously validated active blocks**, the shipped Regtest prune-after threshold **1,000**, actual **550 MiB** mode and **584 distinct nonactive codec-valid stored bodies** derived from the existing legal-large-singleton shape. The changed-nonce nonactive volume controls are storage/codec evidence, **not accepted-branch consensus evidence**. Their actual bytes count through shipped all-payload accounting; only validated active heights are candidates. The fresh Empty checkpoint blocks ordinary Periodic deletion; after exact-tip indexing, the existing automatic Always owner checkpoints and removes **714 actual active body/undo pairs** while preserving the final 288 active blocks and every nonactive volume key.

Measured logical payload bytes decrease from **579,107,574** to **578,379,302**. All nonactive bodies survive reopen; complete sorted body keys, lengths and first-party SHA256d commitments match. Hash commitments stream values without retaining a duplicate 550 MiB cache. This large-volume comparison is not the exact-value snapshot used by the missing-history refusals. Remaining usage stays above the soft target because nonactive bytes are accounted but not eligible candidates. These figures are logical payload bytes, **not physical filesystem size or a disk-capacity guarantee**.

## Verification

All Cargo commands used pinned Bun 1.3.9, Rust 1.94.1, the repository manifest and `scripts/command-timings.ts`; cooperative locking serialized this work with the disjoint daemon executor. Counts overlap and must not be summed as distinct coverage.

| Final check | Result |
| --- | --- |
| Node `test --lib phase157_store_history_` | **7 passed**, zero failures/ignored; **37.99s** harness; **43.725s** wrapper, 21:00:58.399–21:01:42.124 UTC |
| Node `test --lib phase157_store_ -- --test-threads=4 --nocapture` | **14 passed**, zero failures/ignored; **244.51s** harness; **251.442s** wrapper, 21:01:58.173–21:06:09.615 UTC |
| Node targeted corrected normal checkpoint link | **1 passed**, both Periodic/Always cases; **16.51s** harness |
| Node `clippy --all-targets --all-features -- -D warnings` | **Passed**, **4.89s** Cargo; **5.647s** wrapper, 21:06:21.449–21:06:27.096 UTC |
| Scoped `rustfmt --edition 2024 --config skip_children=true --check` | **Passed** for all four new Rust files |
| Owned registration diff whitespace and source review | **Passed**; all four children registered through `mod evidence`; every source under 628 lines |

Root owns the native verifier, coverage, full build/Bazel, source/security/lifecycle review, breadcrumb registry and final Git gate. No production edit was required, so this plan did not duplicate unaffected existing full-suite regression checks after Plan 07's passing gate.

## Test-First Discoveries and Issues

The initial history run was **5 passed / 2 failed**: the fixture actually persists empty genesis undo, while two assertions assumed it was absent. The generic pair helper now compares actual validated undo presence; the no-undo positive deliberately removes it. These are fixture expectation corrections, **not product-bug RED claims**.

Two compilation checks caught test helper mistakes: a nonexistent raw-read convenience method and an incorrect SHA256d export path. The tests now compare existing raw rows after closed reopen and use the actual first-party crypto module. The first full matrix was **12 passed / 2 failed**: stale preparation correctly rejected unbudgeted acquisition, and replay correctly earned old durable height 399 when the test overconservatively expected 15. Budgeted acquisition and the root-approved genuine height-19 scenario corrected those tests. The final matrix passed all 14. No product failure is claimed from these test assumptions and no production fix was made.

## Deviations and Simplification

The root-approved scope amendment exposes only the existing fixture module/type to its test sibling. Existing history data and production owners are reused; no second history cache or retention worker was added. Root explicitly approved earning automatic pressure using actual codec-valid nonactive body volume while preserving continuously validated active candidates and truthful evidence labels. Test-only scenario corrections follow the checked contract without weakening policy or refusal assertions.

The simplification pass centralizes configured reopen, complete raw snapshots, paired-source assertions and bounded fixture completion in the small evidence registry. Four concern-focused files keep test responsibilities local; the large-volume control streams commitments. No architectural change or out-of-scope production work was necessary.

AGENTS, its Bright Builds sidecar, placeholder-only overrides and architecture/code-shape/testing/verification/local-guidance/Rust standards informed this work. Both active lessons were loaded completely within **7,188 bytes / 2,397 estimated tokens**; no archives were loaded.

## Threat Review and Known Limits

T-157-26 is covered by achieved fresh/saved paired loss, missing-mate distinction, exact raw refusal snapshots and pre-reconciliation/pre-intent ordering. T-157-27 is covered by actual ordinary accepted facts, concrete body/undo/coins/metadata faults, truthful reopen, sealed stale completion and immutable identical retries. T-157-28 is covered by unchanged default connects, ordinary Periodic/Always checkpoint linkage, true all-payload measurement and actual automatic paired deletion. All added effects are private test harness operations on fixture datadirs; no new production trust surface was introduced.

There are no goal-blocking stubs or deferred production gaps. Easy-header histories use test-only maturity one; nonactive volume is codec/storage evidence. Replacement authority and missing mates are explicitly seeded software faults. Closed Fjall reopen and software writer boundaries do not prove hardware power-loss resilience, all-payload support, physical space reclamation, public-mainnet behavior, archive-scale serving or production/funds readiness.

## Task Commits and Next Readiness

Tasks 1–2 and metadata are **pending root strict finalization** after whole-phase verification. No hashes are claimed. `requirements-completed` remains empty. Plan 10 can register the four immediate pinned breadcrumbs, publish the measured scoped evidence and run the remaining native/lifecycle/security gates.

## Self-Check: PASSED

All seven owned source paths and this summary exist. The four new files carry immediate pinned breadcrumbs and are registered through the existing test root. Actual final nonzero history/store results, lint and scoped formatting appear above. Source lengths are 113–582 lines, within the 628-line managed threshold. Lifecycle metadata matches the originating plan; only the opening/closing frontmatter delimiters use standalone `---`; requirements activation remains empty. No goal-blocking source stub was found. Commit-existence checks are inapplicable under explicit root finalization deferral.
