---
phase: 162-real-prune-retention-and-integrated-parity-proof
plan: "02"
subsystem: testing
tags: [regtest, daemon, basic-filters, idle-worker, durability]
requires:
  - phase: 162-01
    provides: Shared normal-maturity recipe and independently decoded raw oracle
  - phase: 162-13
    provides: Disabled configured runtime preserves the selected chain and DNS defaults
provides:
  - Configured authenticated daemon fixture with genuine normal-maturity receipts and handle-free provenance handoff
  - Actual private idle-worker lifecycle from partial startup through bounded zero-peer catch-up
  - Typed failure settlement, worker join and all-handle configured reopen evidence
affects: [162-03, 162-04, 162-05, 162-06, 162-12]
tech-stack:
  added: []
  patterns: [capacity-one elapsed/park handshakes, shared bounded outer subprocess supervisor]
key-files:
  created:
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162.rs
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/fixtures.rs
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/coins_flush/tests/phase162.rs
  modified:
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index.rs
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/coins_flush/tests.rs
key-decisions:
  - Seed accepted history with indexing disabled, close every handle, then use actual configured BASIC startup to install the partial owner.
  - Observe public BASIC summary/query and real durable checkpoint/coins facts without widening node-private APIs.
  - Retain the strong Regtest assertion and require checked Plan13 to fix the existing selected-network producer defect.
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 162-2026-10-11T16-25-37
generated_at: 2026-10-11T18:08:13Z
duration: 47min
completed: 2026-10-11
---

# Phase 162 Plan 02: Configured Daemon and Actual Idle Worker Summary

**A genuinely configured Regtest daemon advances its accepted h0..1000 history from startup h7 to h1000 in 125 completed zero-peer idle turns, then joins and reopens after both success and a retained typed failure.**

## Performance and Scope

- Tasks: 2/2. Owned source files: five; three new test-only Rust files and two module registrations. No production API, dependency, authority, endpoint or operator schema was added by this plan.
- First task timing receipt: 2026-10-11T17:21:22.634Z. Final verification resumed at the observed clock 18:04:24Z; summary-preparation clock was 18:08:13Z. The 47-minute source/verification window includes shared-target coordination and the checked prerequisite pause; earlier context-loading time was not separately recorded.
- Material guidance: repo AGENTS, Bright Builds sidecar, overrides, architecture/code-shape/testing/verification/Rust standards, execution workflow/template and phase context/research/UI contract. Both active lesson files were fully read: 7,188 bytes and 2,397 conservative estimated tokens. The existing audit baseline had no new trigger.
- Parent owns STATE/ROADMAP/REQUIREMENTS/todo, registries, generated LOC, independent reviews, full native verification, formal requirement activation and Git finalization. Requirements remain pending.

## Fixture and Provenance

`Phase162Fixture::seed` uses actual runtime network parameters, normal maturity 100, the shared recipe and ordinary `save_block`/`connect_local_block`/Always flush calls. It preserves all 1,001 real acceptance receipts. Every non-genesis durable undo is checked against historical and same-block spent facts. Durable coins B and publicly hydrated chain metadata agree with the accepted history; the genuine h20 descriptor output remains unspent with its actual script and value.

The original unindexed runtime is completely dropped before actual parser/store-selection/authoritative startup. Explicit Regtest, BASIC, manual prune, test credentials and loopback-only listener settings are parsed. Sync, inbound listening and general compact-filter serving remain off; relay/block-serving policies remain their defaults. Configured runtime parameters equal the independent recipe's Regtest parameters. Startup installs a real partial owner with processed h7, accepted h1000 and zero peers. A second Fjall open refuses while the fixture is live.

The fixed-argv Bun oracle receives raw blocks only: 1,001 canonical blocks and four alternate blocks. Recipe ID and the ordered-input SHA256 digest are checked and retained with receipts and raw-derived expectations. All canonical and alternate spent facts match the independent decoder. Actual initial h7 and completed h1000 immutable bytes/hash/header match that oracle at their respective observations. No queried Rust filter commitment enters oracle inputs.

The daemon includes Plan01's shared `packages/testdata/phase162/process.rs` supervisor. Its frozen limits are 8 MiB stdin, 16 MiB stdout, 64 KiB stderr and 150 seconds total, with owned Unix process-group termination, bounded signal/drain/join handling and reap evidence. The daemon's own nine subprocess controls pass, including inherited-pipe descendants. This helper is a test-only dependency owned and verified by Plan01.

`close(self)` consumes HTTP state, shared context, authoritative runtime including its optional sync runtime, store and config. Its handoff carries only path, recipe and provenance. Successful configured reopens and the final raw Fjall open prove the original handles are gone.

## Actual Worker and Measured Result

`phase162_integrated_lifecycle` starts the unchanged private `coins_flush_worker_loop` in a real thread. Capacity-one command and acknowledgment channels allow one outstanding tick. A tick completes only when the production loop reaches its next wait invocation; acknowledgment waits have a 30-second deadline. The parked receiver waits for the next explicit event or sender disconnect, so later plans can extend this same live scenario without unsolicited turns.

Every completed catch-up tick observes an increasing public processed height, a delta at most eight, fixed accepted tip, zero peers, unchanged genuine coins B and a durable checkpoint bounded by both processed height and that coins-backed tip. The final actual safe checkpoint and immutable record identify h1000. The scenario never invokes the driver directly to observe or manufacture progress.

| Final correctly configured observation                  | Result                               |
| ------------------------------------------------------- | ------------------------------------ |
| Accepted canonical history                              | 1,001 blocks; h0..1000; maturity 100 |
| Actual startup processed endpoint                       | h7; not initially synchronized       |
| Completed elapsed worker turns                          | 125; strictly below the 256-turn cap |
| Measured seed/undo/raw-oracle preparation               | 14,461 ms                            |
| Measured catch-up                                       | 5,575 ms                             |
| Final accepted/processed/safe durable endpoint          | h1000 with matching actual coins B   |
| Peer input                                              | zero peers throughout catch-up       |
| Complete lifecycle libtest time                         | 21.87 seconds                        |
| Success/failure join, configured reopen, final deletion | all passed                           |

These are local measurements, not runtime, memory, disk-capacity or archive-scale guarantees. Physical footprint was not measured by this plan. No pressure rows or pruning were performed here.

The normal success path sends shutdown, performs the existing Always settlement and joins. A subsequent configured reopen starts another real worker, explicitly disables the live owner and sends one elapsed event. The existing stale-owner boundary yields a retained `CoinsFlushError::BasicFilter`; the harness preserves that exact typed cause through Always and join. The disabled prefix/protection and coins B remain unchanged. All handles close again, configured Enabled reopen restores the valid completed prefix, a final raw Fjall open succeeds and the unique datadir is removed. RAII settlement also requests shutdown and joins when the harness leaves scope during failure/unwinding.

## Operation-Budget Composition

The long scenario measures completed scheduling turns and prefix deltas. It does not claim detailed runtime operation counters, which the unchanged production loop discards.

The unconditional production composition is `coins_flush_worker_loop` → `maybe_drive_elapsed` → the genuine Fjall `BasicIndexMaintenance` implementation → `drive_basic_filter_index_turn` → `production_budget`. `chainstate/filter_reorg/preflight.rs::production_budget` supplies eight normal blocks, 512 record operations, 256 projection operations and 1,000,000 checkpoint operations. The owner reserves and checks the remaining work before publication.

Two existing executable controls were rerun independently:

- `phase157_idle_ordered_turns_extend_target_then_genuine_periodic_fence_releases`: actual loop adapter outcomes enforce eight blocks/body reads, one persistence batch and the existing operation ceilings, then exercise the genuine Periodic coins fence.
- `phase157_turn_prefix_independent_complete_operation_counts`: prefixes 16/128/512 retain measured eight body reads/decodes/undo borrows, eight generations, 98 record operations, 44 projection operations, 7,621 checkpoint operations and one persistence batch. These measurements belong to that existing controlled test, not the long Phase162 scenario.

## Final Verification Receipts

All Cargo work was serial through the cooperative timing wrapper with Bun 1.3.9 and Rust 1.94.1:

```bash
export PATH=/tmp/open-bitcoin-bun-1.3.9-fresh:$PATH
bun run scripts/command-timings.ts run --key KEY -- cargo CARGO_ARGS
```

Test commands used `--manifest-path packages/Cargo.toml --all-features`, the indicated package/binary, and `-- --test-threads=1 --nocapture`. Exact selectors and timing receipts follow; durations are libtest time / total wrapper time.

| Key / package and selector                                                                                                                               | UTC start    | Result                                      | Time             |
| -------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------ | ------------------------------------------- | ---------------- |
| phase162-daemon-fixture / open-bitcoin-rpc --bin open-bitcoind phase162_daemon_fixture                                                                   | 18:04:34.261 | 1 passed, 0 failed                          | 15.57s / 15.977s |
| phase162-idle-worker / open-bitcoin-rpc --bin open-bitcoind phase162_integrated_lifecycle                                                                | 18:05:02.904 | 1 passed, 0 failed                          | 21.87s / 22.023s |
| phase162-daemon-process-controls / open-bitcoin-rpc --bin open-bitcoind phase162_process                                                                 | 18:05:40.917 | 9 passed, 0 failed, 1 ignored control entry | 2.33s / 2.484s   |
| phase162-daemon-budget-composition / open-bitcoin-rpc --bin open-bitcoind phase157_idle_ordered_turns_extend_target_then_genuine_periodic_fence_releases | 18:06:05.633 | 1 passed, 0 failed                          | 1.25s / 1.599s   |
| phase162-owner-budget-composition / open-bitcoin-node phase157_turn_prefix_independent_complete_operation_counts                                         | 18:06:34.710 | 1 passed, 0 failed                          | 15.65s / 22.318s |
| phase162-daemon-clippy / clippy -p open-bitcoin-rpc --all-targets --all-features -- -D warnings                                                          | 18:07:09.148 | passed                                      | 0.465s wrapper   |
| phase162-daemon-format / fmt --manifest-path packages/Cargo.toml --all --check                                                                           | 18:07:23.386 | passed                                      | 1.406s wrapper   |

The ignored process control entry is explicitly invoked by the ordinary subprocess tests. Every requested behavioral selector discovered and executed nonzero tests.

Additional passed checks: scoped Rust formatting, owned/global diff whitespace, parity breadcrumbs for 1,126 Rust files after parent registration, Phase154 provenance/evidence/claim guard, its 28-test negative-control suite and managed checks with zero findings. Installed mdformat GFM/frontmatter extensions are checked before summary formatting. Parent full default `bash scripts/verify.sh` and independent source/mitigation review remain required.

## Failures, Prerequisite and Deviations

1. Task1 RED was a missing fixture module; Task2 RED was the not-yet-created harness. These were compile/discovery failures, not runtime behavior passes.
1. Initial fixture compilation exposed node-private `maybe_basic_index_progress` and `load_basic_filter_record`; the daemon now uses existing public summary/query APIs. The later metadata assertion exposed another private reader and now uses the existing public hydration read. Public cross-crate APIs were inventoried before final compilation. No visibility was widened and no snapshot was seeded.
1. Interim tests passed before the stronger parsed-network equality existed. Those results are superseded as configured-Regtest evidence. The stronger fixture then failed 0/1 in 13.24 seconds: actual mainnet pow-limit 486604799 versus Regtest 545259519, and false versus true minimum-difficulty/no-retarget flags; maturity stayed 100. Diagnosis found the Disabled sync producer discarded the parsed address chain. Execution stopped; parent added and independently checked prerequisite Plan13. Its minimal producer fix and regression matrix are documented in `162-13-SUMMARY.md`. The strong fixture and recipe were not weakened. Fresh post-fix results above are the plan's actual configured evidence.
1. Parent's independent foundation review found missing bounds around the outer Bun child. The duplicate interim glue was replaced by Plan01's corrected shared process supervisor before final verification.
1. The explicit simplification pass retained public observations, capacity-one handshakes and the existing owner; a small failure/reopen helper keeps the long test readable. Detailed operation counters use existing executable composition evidence rather than a new observation API.

The selected-network defect was a directly blocking prerequisite handled outside this plan's source ownership under checked Plan13. No production bug was patched by this executor. The single closed datadir from this executor's failed probe was removed only after parent authorization; exact path absence was verified. A broader final temp inventory found a separate closed Plan13 RED-reproduction datadir, reported to the parent without deleting it under the exact-path restriction. The parent subsequently confirmed that exact probe was removed and no known failed-fixture directories remain. Each fresh successful owned test removed its own datadir.

## Parent Registry and Continuation Handoff

The parent already registered these exact new first-party Rust paths:

- `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162.rs`
- `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/fixtures.rs`
- `packages/open-bitcoin-rpc/src/bin/open_bitcoind/coins_flush/tests/phase162.rs`

The first two source comment blocks anchor `packages/bitcoin-knots/src/validation.cpp`, `src/index/blockfilterindex.cpp` and `src/node/blockstorage.cpp` under that same Knots root. The worker source block anchors `src/validation.cpp`, `src/bitcoind.cpp` and `src/node/chainstate.cpp`. Shared recipe/process includes remain under the binary's test-only module root and outside the first-party source registry. No extra child Rust path was introduced.

Parity evidence additions are these three registered sources, the two module registrations, their identifiable fixture/lifecycle selectors and the shared Plan01 raw recipe/oracle/process dependencies. No intentional production behavior difference is claimed by Plan02.

The single identifiable lifecycle has an explicit extension point with the worker parked and live before normal shutdown. Later plans extend it through connect/reorg/prune/clients. `Phase162Handoff` retains recipe ID, ordered raw digest, original receipts, oracle and recipe data while carrying no authority/store/context handle. `Phase162Fixture::open` currently selects BASIC/manual-prune/default-off serving; later plans own policy extensions. This executor did not start Plan03.

## Task Commits and Remaining Gates

Task1, Task2, RED/GREEN and metadata commits are **explicitly deferred by D-14**. No staging, commits, pushes, branch changes or hook bypass occurred. Shared planning/state/requirements/registry artifacts were not edited by this executor.

This plan proves configured startup and actual idle scheduling. It does not yet prove accepted reorg, manual/automatic paired pruning, post-prune authenticated RPC/socket replies, wallet refusal or final outbound handoff. Public defaults, public-network CI, BIP37/V0, assumeutxo/dual-chainstate, archive scale, repair/import/download, production service/readiness/funds and milestone shipment remain excluded. No operator presentation or support schema changed.

## Known Stubs

None. Owned source has no TODO/FIXME/placeholder, mocked commitments or snapshot seeding. Later lifecycle steps are explicit later-plan obligations, not substitute success fixtures.

## Threat Mitigations

- T-162-02-1: unchanged production loop and real receipts, each observed prefix delta bounded by eight, safe durable checkpoint bounded by actual coins B, finite 125-turn catch-up, and executed operation-budget composition controls.
- T-162-02-2: capacity-one handshakes, bounded acknowledgment, typed error preservation, explicit Always/join on success and actual stale-owner failure, scope-exit settlement, live-lock refusal and successful all-handle reopens.
- T-162-02-3: actual parsed Regtest parameters after checked prerequisite repair, loopback-only configured addresses, disabled listener/sync/general serving, unchanged relay/block defaults and zero peers throughout catch-up.

All declared mitigations have scoped executable evidence. Independent final source/security review remains parent-owned; this summary does not certify that review or broader phase completion. No new undeclared production trust surface was introduced.

## Self-Check: PASSED

All three created source files, both registrations and this summary exist. Final named selectors executed nonzero tests; typed failure, actual joins/reopens and owned cleanup passed. Baseline `a77edab276a9daa086055aa61e6143ce86bc1218` exists as a commit. Task commit checks are intentionally inapplicable under D-14. Parent-owned state, requirement, native verification and Git updates remain delegated.
