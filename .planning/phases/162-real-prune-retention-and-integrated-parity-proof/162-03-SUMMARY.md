---
phase: 162-real-prune-retention-and-integrated-parity-proof
plan: "03"
subsystem: testing
tags: [regtest, daemon, reorg, basic-filters, paired-prune, fjall, configured-reopen]
requires:
  - phase: 162-01
    provides: Normal-maturity raw recipe and independent canonical/fork BASIC oracle
  - phase: 162-13
    provides: Actual parsed Regtest identity in disabled-sync daemon startup
  - phase: 162-02
    provides: Configured daemon fixture, consuming handoff and real private maintenance worker
provides:
  - Same-scenario accepted h1001 and shallow reorg across the positive h1000 checkpoint
  - Authenticated manual planner and actual configured 550 MiB automatic paired deletion
  - All-handle configured reopen with exact settled coins, metadata, checkpoint and 1006 immutable records
affects: [162-04, 162-05, 162-06, 162-07, 162-12]
tech-stack:
  added: []
  patterns: [Public cross-crate observations, streamed known-key pressure commitments, consuming configured reopen]
key-files:
  created:
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/lifecycle.rs
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/retention.rs
  modified:
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162.rs
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/fixtures.rs
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/coins_flush/tests/phase162.rs
key-decisions:
  - Preserve node privacy and use actual public summary/query, durable protection and metadata/coins readers.
  - Parse manual 1 or automatic 550 through one existing fixture factory; preserve the manual default and all strong selected-network assertions.
  - Auxiliary rows supply codec/storage pressure only; active accepted heights remain the only automatic candidates.
  - Raw intent rows are not observed by daemon tests; direct node-owned intent controls compose separately with integrated apply/reopen evidence.
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 162-2026-10-11T16-25-37
generated_at: 2026-10-11T18:37:31Z
duration: 20 min
completed: 2026-10-11
---

# Phase 162 Plan 03: Accepted Reorg, Real Paired Retention and Reopen Summary

**One configured normal-maturity history accepts h1001, replaces h999..1001 with h999..1002, then earns authenticated manual and ordinary automatic deletion while preserving settled coins and 1,006 exact active/stale BASIC records through real configured reopen.**

## Performance and Scope

- Tasks: 2/2. Source files: five, including two new test-only modules. No production source, public production API, dependency, authority or schema was added.
- First recorded execution clock: 2026-10-11T18:16:48Z. First RED command: 18:17:07.354Z. Final source behavioral verification ended at 18:33:31.212Z; final Cargo formatting verification ended at 18:34:59.495Z. Summary creation clock: 2026-10-11T18:37:31Z. Earlier context loading was not separately timed.
- Material guidance: repo AGENTS, Bright Builds sidecar, overrides, architecture/code-shape/testing/verification/Rust standards, checked Plan03 amendments, GSD workflow/template and phase context/research/UI contract. Prior 01/13/02 summary frontmatter and relevant handoffs were loaded.
- Both active lesson files were read fully: 7,188 bytes and 2,397 conservative estimated tokens. Existing audit baseline has no new trigger.
- Parent owns STATE/ROADMAP/REQUIREMENTS/todo, both parity registries, generated LOC, independent source/mitigation reviews, full native verification, formal requirement activation and Git finalization. Requirements remain pending.

## Task 1: Actual Connect, Worker Progress and Reorg

The same live configured owner accepts an ordinary saved h1001 extension with its real receipt and unchanged normal maturity 100. Before the first worker tick, its accepted target has advanced while the safe durable endpoint remains h1000. The actual private worker processes h1001, yet the safe endpoint stays h1000. An ordinary Always coins flush and another real elapsed worker turn advance safety to h1001.

The deterministic alternate branch starts at ancestor998. Its four actual bodies are saved normally; public `reorg_to_branch` disconnects three actual blocks h1001/h1000/h999 and returns four matching accepted positions h999..1002. Public processed summary and durable safe checkpoint both rewind to h998. One completed worker turn processes h1002 while safe progress stays h998; an ordinary coins flush and worker tick earn safe h1002. No direct driver shortcut, snapshot seed, maturity override or policy mutation occurs.

Actual extension and replacement undo are checked through historical inputs against raw-outpoint-derived spent scripts, including same-block spends. Canonical active positions through h998 match all prior real receipts; replacement positions match the actual transition receipts. Every replacement filter matches the independent raw oracle's exact bytes/hash/header. Active replacement h1000 and displaced canonical h1000 have distinct block identities and independently matched, distinct positive-checkpoint headers. Original h999/h1000 and the accepted h1001 immutable rows remain exact by hash.

Initial history has 1,001 accepted blocks; the extension and four replacement receipts bring the scenario to 1,006 distinct accepted records, with 1,003 positions on the final active branch.

## Task 2: Real Manual and Automatic Paired Loss

A pre-catch-up authenticated `pruneblockchain(20)` request uses the configured RPC context and real planner. It returns -1 on the observed fresh safe-Empty prefix. Required h8..20 body/undo pairs remain, and the complete saved protection snapshot is unchanged. The assertion permits the shipped caller to delete an eligible prefix in other safe-prefix states; it does not assume a whole-operation refusal.

After actual reorg/coins settlement, `SettledIdentity` captures all durable active positions, all durable UTXOs, confirmation-count metadata, real coins B with empty heads, the exact saved safe checkpoint and all 1,006 active/stale immutable filter identities/bytes. Undo is deliberately excluded from settled identity because it is one of the two payload mates being deleted.

Authenticated `pruneblockchain(1000)` returns the shipped -8 keep-window refusal and leaves usage unchanged. Authenticated `pruneblockchain(20)` returns 20 and physically removes both body and undo through h20. All later active pairs remain. Durable have-pruned is earned, usage falls, and settled identity plus all immutable rows stay exact.

The worker is already shut down and joined before consuming the complete fixture into its handle-free handoff. Actual JSONC parsing reopens that exact datadir with `prune=550`, explicit Regtest and BASIC, using the same factory; `open` retains its manual=1 default. Strong network, consensus, activation, relay/block-serving-default and context assertions remain intact.

Exactly 584 distinct codec-only nonactive bodies supply actual volume. Each has 989,882 encoded value bytes, including bounded repeated scripts. They are never connected, never accepted history and never deletion candidates. The public production accounting reader measures all actual body/undo keys; no accounting override is used.

An ordinary `flush_coins(Always)` on the configured automatic owner removes 694 newly selected active pairs h21..714. The full trailing 288-block window h715..1002 remains. Auxiliary bodies remain unchanged. All known auxiliary keys, encoded lengths and first-party SHA256d values are streamed before/after deletion and after reopen, keeping only small commitment tuples rather than duplicate payload caches.

Live second opens refuse before both configured close/reopen boundaries. Every HTTP state, context, authority/runtime, store and worker clone is consumed or joined. The exact same-path automatic reopen preserves usage without further deletion, both mates remain absent through h714, have-pruned remains earned, all trailing pairs remain, and settled coins/metadata/safe checkpoint plus all 1,006 immutable rows are unchanged.

The returned automatic fixture remains live at the downstream client insertion point in `phase162_integrated_lifecycle`. This plan then preserves Plan02's typed disable/failure/Always/join tail, another configured restore, final all-handle raw open and unique-datadir deletion. Later plans can extend that same insertion point; Plan04 was not started.

## Actual Final Measurements

Environment: macOS arm64, pinned Bun 1.3.9, Rust 1.94.1. These are local measurements, not archive-scale, memory-bound, timing or capacity guarantees.

| Observation                                                        | Final executed result                |
| ------------------------------------------------------------------ | ------------------------------------ |
| Initial configured processed height                                | h7                                   |
| Completed initial elapsed worker turns                             | 125; cap256; zero peers              |
| Seed/undo/independent-oracle preparation                           | 14,395 ms                            |
| Actual initial worker catch-up                                     | 5,624 ms                             |
| Final active positions / immutable accepted rows                   | 1,003 / 1,006                        |
| Settled manual usage before / after                                | 1,066,313 / 1,061,726 bytes          |
| Manual selected range                                              | h0..20, both mates absent            |
| Nonactive codec-only pressure bodies                               | 584; 989,882 encoded bytes each      |
| Auxiliary encoded value total                                      | 578,091,088 bytes                    |
| Actual automatic usage before / after                              | 579,152,814 / 578,426,929 bytes      |
| Automatic newly deleted active pairs                               | 694; h21..714; 725,885 bytes removed |
| Retained active trailing window                                    | h715..1002; 288 pairs                |
| Actual target / remaining excess                                   | 576,716,800 / 1,710,129 bytes        |
| Allocated temporary file footprint before consuming reopen         | 1,056,243,712 bytes                  |
| Pressure construction, streams, apply and configured reopen        | 172,162 ms                           |
| Full final scenario timer / libtest duration                       | 198,726 ms / 198.73 seconds          |
| Success/failure worker joins, configured reopens and final cleanup | passed                               |

The soft target remains exceeded by ineligible nonactive payloads. No target-attainment claim is made. Physical allocated file bytes use `MetadataExt::blocks * 512`, exclude directories and are a point observation rather than peak filesystem reservation. Logical accounting excludes engine allocation and is reported separately.

Only one pressure corpus existed at a time. The first successful full run cleaned its corpus before the final rerun. Three earlier pressure-free failed-run datadirs were removed by exact names only after their owning PIDs were confirmed gone: 7209, 7940 and 8811, with observed allocated footprints 9,580,544 / 15,183,872 / 6,397,952 bytes. All were absent after cleanup. The integrated test now declares its cleanup guard before the fixture so unwinding closes fixture/worker handles before attempting directory removal.

## Executed Verification

All Cargo commands were serial through the cooperative timing wrapper:

```bash
export PATH=/tmp/open-bitcoin-bun-1.3.9-fresh:$PATH
bun run scripts/command-timings.ts run --key KEY -- cargo CARGO_ARGS
```

Behavioral commands used `--manifest-path packages/Cargo.toml --all-features` and `-- --test-threads=1 --nocapture`. RPC commands also used `-p open-bitcoin-rpc --bin open-bitcoind`; node commands used `-p open-bitcoin-node`.

| Key / selector or command                                                                                       | UTC start    | Result                                              | Libtest / total wrapper |
| --------------------------------------------------------------------------------------------------------------- | ------------ | --------------------------------------------------- | ----------------------- |
| phase162-connect-reorg-red / phase162_integrated_lifecycle                                                      | 18:17:07.354 | intended 0/1 RED: actual1000 vs expected1002        | 20.51s / 23.561s        |
| phase162-connect-reorg / phase162_integrated_lifecycle                                                          | 18:19:49.583 | 1/1 passed                                          | 22.53s / 26.707s        |
| phase162-paired-retention-red / phase162_integrated_lifecycle                                                   | 18:20:34.701 | intended 0/1 RED: no earned paired deletion         | 21.52s / 24.508s        |
| phase162-paired-retention / phase162_integrated_lifecycle                                                       | 18:24:54.881 | 0/1: incorrect fresh safeh7 assertion               | 14.75s / 19.161s        |
| phase162-paired-retention / phase162_integrated_lifecycle                                                       | 18:25:46.107 | 1/1 passed                                          | 200.38s / 203.512s      |
| phase162-retention-clippy / clippy -p open-bitcoin-rpc --all-targets --all-features -- -D warnings              | 18:29:36.285 | passed                                              | 2.379s wrapper          |
| phase162-paired-retention-final / phase162_integrated_lifecycle                                                 | 18:30:09.021 | 1/1 passed, final source                            | 198.73s / 202.191s      |
| phase162-retention-fixture-regression / phase162_daemon_fixture                                                 | 18:33:51.247 | 1/1 passed, manual default/strong startup preserved | 14.97s / 15.302s        |
| phase162-retention-existing-rpc / phase159_daemon_rpc_retention_actual_paired_prune_stale_and_all_handle_reopen | 18:34:29.366 | 1/1 passed                                          | 6.81s / 6.964s          |
| phase162-retention-existing-intent / initialize_intent\_                                                        | 18:34:44.876 | 7/7 passed                                          | 4.77s / 4.921s          |
| phase162-retention-existing-window / automatic_prune_keeps_exact_trailing_window_above_target                   | 18:34:50.959 | 1/1 passed                                          | 0.01s / 0.145s          |
| phase162-retention-existing-threshold / automatic_prune_at_or_below_network_threshold_does_not_measure          | 18:34:57.913 | 1/1 passed                                          | 0.00s / 0.132s          |
| phase162-retention-format / fmt --manifest-path packages/Cargo.toml --all --check                               | 18:34:58.080 | passed                                              | 1.415s wrapper          |

Every behavioral selector discovered nonzero tests; no ignored integrated proof was added. While pressure runs were quiet, the wrapper reported 60/120/180-second liveness and process observations showed sustained CPU work. No process was terminated for exceeding an estimate.

Additional passed checks: scoped Rust formatting; global diff whitespace; managed checks with zero findings; parity breadcrumbs for 1,128 Rust files after parent registration; Phase154 BASIC provenance/evidence/claim guard and its 28 negative-control tests. Summary formatting uses installed mdformat 1.0.0 with GFM 1.0.0 and frontmatter 2.1.2.

The final rerun was justified by the simplification pass's additional real extension historical/same-block undo assertion and total-lifecycle timer. No broader repeated suite was run after those checks passed. Parent full default `bash scripts/verify.sh`, independent source/mitigation review and formal lifecycle verification remain required.

## Approved Amendments, Deviations and Observational Limits

1. The checked fixture-reopen amendment allows only selecting actual parsed manual1/automatic550 through the existing test factory. No parsed runtime config or network policy is mutated after loading.
1. Public cross-crate inventory found node-only `maybe_basic_index_progress`, immutable raw readers and `maybe_prune_intent` unavailable to the daemon crate. Public summary/query, durable protection and hydrated metadata/coins readers provide the integrated observations without widening visibility.
1. Parent approved a narrow observation amendment: raw namespace/intent rows are **not directly inspected by this daemon test**. Known auxiliary bodies are decoded/re-encoded and committed by key/length/SHA256d; public all-key logical accounting and exact settled state are compared across apply/reopen. Successful reopen alone is **not proof of raw intent absence**.
1. Direct raw-intent evidence composes separately with node-owned tests. Seven existing `initialize_intent_` tests passed: both mates finish; one mate removes its remaining key; have-pruned plus one mate finishes; both absent clear intent; hash mismatch refuses; keep-window intent refuses; empty-chain intent refuses. Plan07 owns the new concrete pending-intent fault matrix. Those controls are not claimed as the integrated history.
1. **[Rule 1 - Test bug]** The first retention attempt assumed processed startuph7 implied safeh7. Actual saved safety was Empty. The corrected assertion compares the complete before/after protection snapshot and required inputs, preserving the actual safety boundary. This was one failed assertion boundary followed by a passing fix; no production behavior changed.
1. The inherited runtime subsidy-halving interval 210,000 differs from pinned Knots Regtest's 150. Normal maturity100, first-party accepted-chain behavior, independent BASIC construction and raw protocol evidence remain valid within their stated scope; no whole-chain Knots consensus differential is claimed.

## Source Registration and Parity Handoff

Parent has applied these exact new breadcrumb groups, and the live checker passes:

| Group                           | New Rust path                                                                            | Exact ordered Knots anchors                                                                                                                 |
| ------------------------------- | ---------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------- |
| phase162-accepted-connect-reorg | packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/lifecycle.rs | packages/bitcoin-knots/src/validation.cpp; packages/bitcoin-knots/src/index/base.cpp; packages/bitcoin-knots/src/index/blockfilterindex.cpp |
| phase162-actual-prune-retention | packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/retention.rs | packages/bitcoin-knots/src/rpc/blockchain.cpp; packages/bitcoin-knots/src/node/blockstorage.cpp; packages/bitcoin-knots/src/validation.cpp  |

Existing phase162 configured-fixture and actual-idle-worker breadcrumbs remain intact. Proposed evidence additions to the existing owning parity surface, for parent publication after the final lifecycle gate: `phase162_integrated_lifecycle` in `packages/open-bitcoin-rpc/src/bin/open_bitcoind/coins_flush/tests/phase162.rs`, its new lifecycle/retention helper paths, and this summary. Scope the evidence to accepted connect/reorg, independent active/stale positiveh1000 commitments, authenticated planner, actual automatic550 paired loss and configured reopen. Do not claim post-prune RPC filter clients, socket transports, wallet/body restrictions or the Plan07 fault matrix from Plan03; downstream plans own those.

## Mitigation and Simplification Review

- T-162-03-1: actual Regtest threshold selection, pre-catch-up required protection, authenticated planner, shipped keep-window refusal, real active-only automatic selection, both-mate absence and exact settled/index retention are executed.
- T-162-03-2: worker joins, consuming fixture handoff, live lock refusals, actual same-path configured reopens and exact recipe/digest/settled-record identity are executed.
- T-162-03-3: 584 bounded codec-only rows, one corpus at a time, streamed commitment tuples, measured logical/allocated usage and finite cleanup are executed. No archive-scale or peak-memory guarantee is inferred.
- Simplification pass reused the single configured factory, factored shared actual undo/input checking, computed alternate raw spent facts once, retained short ownership boundaries and avoided a raw-store dependency or surrogate authority. Owned files remain below the managed size boundary.
- Stub and threat-surface scans found no TODO/placeholder/mock-data stub and no new production endpoint, authorization path, schema or trust-boundary file access.

## Task Commits and Readiness

1. Task 1: accepted extension/reorg/worker-fence evidence — **commit deferred by D-14**.
1. Task 2: legal manual/automatic retention and consuming reopen — **commit deferred by D-14**.
1. Plan summary metadata — **commit deferred by D-14**.

No stage, commit, branch, hook bypass or push occurred. Baseline remains `a77edab2`. Parent state/roadmap/requirements/registry finalization is intentionally separate.

Ready for Plan04 to extend the same reopened live fixture into client assertions. Cargo is idle after the recorded checks. Remaining phase verification and requirement activation belong to the parent.

## Self-Check: PASSED

- All five owned source paths and this summary exist.
- Both new source groups have the exact ordered parent-applied breadcrumb anchors.
- Lifecycle identity matches the checked plan; requirements-completed remains empty; only the two top frontmatter delimiters are standalone dashes.
- Baseline commit a77edab276a9daa086055aa61e6143ce86bc1218 exists. No task commit is claimed; all commits are explicitly deferred by D-14.
- Final focused test and all named existing regression selectors passed with nonzero discovery. The final pressure corpus and earlier exact failed-run datadirs were cleaned, and no Cargo session remains.
