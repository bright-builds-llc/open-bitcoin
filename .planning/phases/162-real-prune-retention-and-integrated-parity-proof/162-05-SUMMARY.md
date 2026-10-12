---
phase: 162-real-prune-retention-and-integrated-parity-proof
plan: "05"
subsystem: testing
tags: [funded-wallet, creating-history, body-serving, retained-filters, cold-reopen]
requires:
  - phase: 162-03
    provides: Genuine accepted history, real paired retention and consuming reopen
  - phase: 162-04
    provides: Verified retained-filter authenticated RPC and physical peer callers
  - phase: 162-16
    provides: Correct future descriptor metadata writes with unchanged legacy decoding
provides:
  - Actual h20-funded durable named wallet and public pre-prune scan/checkpoint
  - Same eligible socket retained h715 Block and deleted in-window h714 NotFound
  - Public later-range refusal preserving funded wallet and prior durable job progress
affects: [162-06, 162-09, 162-10, 162-12]
tech-stack:
  added: []
  patterns: [Handle-free wallet evidence, explicit body-positive control, bounded public descriptor import]
key-files:
  created:
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/restrictions.rs
  modified:
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/fixtures.rs
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162.rs
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/coins_flush/tests/phase162.rs
key-decisions:
  - Bound only the public test-wallet import to its genuine funded derivation zero, preserving the shared descriptor body and script.
  - Create and scan before protected_request, then refresh publicly after the accepted reorg and before destructive retention.
  - Require retained-body success before the actual in-window missing-body negative on the same eligible socket.
  - Route the discovered descriptor formatter defect through checked prerequisite16 without weakening the stored-metadata assertion.
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 162-2026-10-11T16-25-37
generated_at: 2026-10-11T21:27:41Z
duration: 41 min recorded window including checked prerequisite pause
completed: 2026-10-11
---

# Phase 162 Plan 05: Funded Wallet and Actual Body Restrictions Summary

**The real retained-filter datadir serves retained h715, refuses physically deleted in-window h714, and rejects a funded wallet rescan requiring deleted h20 history while preserving its saved wallet and checkpoint.**

## Performance and Ownership

- Tasks: 2/2. Exactly four owned test-source paths changed, including one new module. This plan changes no production source, dependency, API, visibility, shared recipe, oracle, schema or production default.
- First recorded clock: 2026-10-11T20:46:23Z; initial context loading preceded it. Full acceptance ran 21:17:32.109Z–21:21:54.076Z. Final Cargo receipt ended 21:23:16.519Z; final summary clock is 21:27:41Z. The recorded window includes the checked resource amendment and separate prerequisite 16 pause.
- Material guidance: repo AGENTS, Bright Builds sidecar/overrides and architecture/code-shape/testing/verification/Rust standards; execute-plan workflow/template; phase CONTEXT/RESEARCH/UI contract; checked Plan05 and prerequisite16; Summaries03/04/14/15/16 and retention/checkpoint reviews.
- Both active lessons were loaded fully: 7,188 bytes and conservative 2,397 tokens, with no archive loading or new audit trigger. No project skill directory exists in the two prescribed locations.
- Parent supplied synchronized baseline a77edab276a9daa086055aa61e6143ce86bc1218 and materialized pinned Knots. Every Cargo command used Bun 1.3.9 via pinned PATH, Rust 1.94.1, the timing wrapper and exclusive serial build ownership. All Cargo sessions exited; ownership was returned idle.
- Root retains STATE/ROADMAP/REQUIREMENTS/todo, parity registries, generated LOC, independent source/security review, full native/formal gates and Git. No staging, commit, branch change, push, hook bypass or requirement activation occurred.

## Task 1: Genuine Funding and Prior Checkpoint

The same accepted recipe supplies the h20 coinbase transaction's output 1 for 25,000 sats. Assertions compare the actual persisted creating block to the original recipe, its creating output to the durable unspent coin, creation height/value/script, and the eventual wallet-owned outpoint/output. No wallet UTXO or chainstate snapshot is seeded.

The test imports the unchanged shared descriptor body through the public wallet import API with explicit metadata `#ob:0:0:0`. The parser-backed descriptor's public range start/end/cursor are each Some(0); stored text retains that exact metadata, the body before # equals the shared constant, and derivation 0 and public default receive script equal the original recipe funding script. Only the test import is bounded. Shared recipe, funding, matcher, defaults, production wallet API and maturity 100 remain unchanged.

WalletRegistry creates the actual named durable wallet. The actual shared ManagedRpcContext selects it and performs a successful full public scan before `retention::protected_request`, which may legitimately delete an eligible earlier prefix. After the ordinary extension and shallow reorg, the public 8..1002 rescan refreshes the saved wallet/job before manual and automatic pruning. The captured job is Complete at 1002, scanned through 1002, next 1003 and targeted to the actual accepted tip 1002. The evidence contains only WalletSnapshot and WalletRescanJob values, never store, registry, context or network handles.

The actual registry is reloaded after successful scans, after automatic pruning/reopen, around refusal, after the restriction cold reopen and after final maintenance failure settlement/reopen. Full snapshot and job equality preserve descriptors, funding, progress, target hash/height, median time and freshness.

## Task 2: Body Positive, Actual Missing Body and Wallet Refusal

The same pruned datadir is reopened through the existing parsed fixture factory with explicit block serving enabled, compact relay disabled, inbound loopback listening and `in,download,blockfilters` permissions. Prior callers retain default-off block serving. Assertions check the requested activation exactly, the public eligibility classifier, effective DownloadServingPolicyInput, actual admitted PermissionedInbound class and complete handshake state.

At the actual accepted tip 1002, h715's distance 287 and deleted h714's distance 288 both pass the existing 288+2 limited-serve predicate. Physical paired storage checks prove h715 exists, h714 body/undo are absent and have_pruned was earned. The request therefore cannot succeed merely because serving was disabled, permission was absent or the body was outside the serving window.

One real handshaken socket first requests ordinary Block inventory for h715. Its physical frame equals the original accepted recipe body frame, decoded body serialization and block hash match, and a sentinel Pong fences completion. The same peer then requests actually deleted h714 and receives the exact NotFound inventory/frame followed by another sentinel Pong. Any extra Block frame would fail the strict Pong read. The peer remains live after each response; final disconnect removes it. One Block, one NotFound and two post-response Pongs are directly observed.

Authenticated getblockfilter for h714 independently matches the raw oracle's retained filter/header immediately before and after the body session, after wallet refusal and after cold reopen. Settled coins, active metadata, BASIC checkpoint and all 1,006 immutable records stay exact.

The selected real wallet's public 715..1002 rescan requests only retained body heights but still needs its selected unspent coin's deleted h20 creating history. It refuses with the established Creating/UnavailableNamespace/height 20 category. The wallet snapshot is exactly unchanged. Only existing job state/error may change to Failed; normalizing those two permitted fields yields exact equality with every captured prior job field, including target and freshness. Stored records reload with that same truth. Explicit h20/h714 paired absence checks after refusal and cold reopen rule out hidden body repair/download.

## Full Integrated Measurements

The final complete `phase162_integrated_lifecycle` passed 1/1 in 261.07 seconds, with 261,967 ms wrapper duration. Both task commands target this same complete scenario; one final joint run closes both obligations without a redundant identical pressure run. Parent's full native gate will execute the final source again.

- Initial accepted blocks 1,001; final active positions 1,003; accepted immutable records 1,006; ordinary maturity 100; genuine shallow replacement 999..1002 from ancestor 998.
- Seed 14,890 ms; catch-up 6,251 ms; 125 completed actual maintenance ticks; full lifecycle 261,059 ms.
- Manual target 20: logical usage 1,066,313→1,061,726 bytes, paired 0..20 absence and keep-window refusal verified.
- Automatic 550 MiB uses 584 codec-only nonactive pressure rows totaling 578,091,088 value bytes. Actual ordinary flush deletes 694 pairs at 21..714 and retains 715..1002. Logical usage 579,152,814→578,426,929 bytes; target is 576,716,800 bytes and remains unattained. Sampled allocated file bytes 1,052,659,712 are neither peak space nor immediate reclamation evidence.
- Prior RPC evidence remains unchanged: nine hashes with two selectors and four unauthorized calls. Three granted filter sessions still observe 60 exact physical CF frames/5,670 bytes total. Ten fresh absent-grant sessions each observe zero bytes then the pinned terminal disconnect, with peer/charge cleanup.
- New body/wallet assertions, failure-path Always settlement, joined maintenance/listener tasks, consuming reopens, raw all-handle store reopen, final datadir removal and path absence all pass. The expected injected `index unavailable` maintenance message is followed by the asserted typed failure settlement; the test succeeds.

## Verification and Initial Failures

All Cargo rows used `bun run scripts/command-timings.ts run --key KEY -- cargo ...`, pinned PATH and `--manifest-path packages/Cargo.toml`. Test selectors have nonzero discovery; RPC daemon selectors use `--bin open-bitcoind --all-features`, regression uses `--lib --all-features`, and libtest runs serially.

| Timing key                                | Actual result                                                                                                 | Wrapper ms |
| ----------------------------------------- | ------------------------------------------------------------------------------------------------------------- | ---------- |
| phase162-wallet-funded-preflight          | Original default wildcard genuine 102-block wallet/public scan/cold reload: 1 passed, 93.64s test runtime     | 97,854     |
| phase162-restrictions-compile             | First compile failed on owned imports and unavailable Tokio io-util; corrected without dependency/API changes | 3,580      |
| phase162-restrictions-compile             | Corrected daemon all-features no-run compile passed,4.93s                                                     | 5,200      |
| phase162-body-serving-preflight           | Genuine 102-block retained h101/unknown-hash socket control: 1 passed, 1.99s                                  | 6,181      |
| phase162-restrictions-bounded-preflight   | Body 1 passed/wallet 1 failed, 3.88s; exact stored metadata exposed extra-colon production defect             | 8,768      |
| phase162-wallet-bounded-after-storage-fix | Prerequisite 16 executed this unchanged bounded public caller: 1 passed, 2.23s                                | 10,640     |
| phase162-body-wallet-restrictions         | Final entire actual 584-pressure corpus: 1 passed, 261.07s                                                    | 261,967    |
| phase162-rpc-rescan-regression            | Existing public eligibility suite: 19 passed, 30.82s                                                          | 43,286     |
| phase162-body-wallet-clippy               | RPC all-targets/all-features strict Clippy passed,16.86s                                                      | 17,267     |

The 93.64s and 2.23s observations are scoped fixture test runtimes, including their actual work, rather than a benchmark, isolated CPU measurement or timeout. The unknown-hash socket diagnostic is setup evidence only; actual deleted h714 acceptance comes exclusively from the final full corpus.

Exact full proof receipt: `.local/open-bitcoin-dev/command-timings/phase162-body-wallet-restrictions/2026-10-11T21-17-32.109Z-3ed9843b-8a05-4576-b771-feb61ad0f76a.json`.

Exact bounded failure receipt: `.local/open-bitcoin-dev/command-timings/phase162-restrictions-bounded-preflight/2026-10-11T20-56-56.120Z-2846435a-57f7-42e7-80c7-bd27ec1f9e72.json`. Exact prerequisite caller GREEN receipt is recorded in162-16-SUMMARY.

Exact regression receipt: `.local/open-bitcoin-dev/command-timings/phase162-rpc-rescan-regression/2026-10-11T21-22-09.944Z-02a7b937-e229-4fac-8e52-b8e841c080f1.json`. Exact strict lint receipt: `.local/open-bitcoin-dev/command-timings/phase162-body-wallet-clippy/2026-10-11T21-22-59.252Z-b013e2f5-d820-4168-a2fa-acf579600f43.json`.

Additional checks pass: parity breadcrumbs for 1,134 Rust files; managed all checks with zero findings; Phase 160 peer-filter and Phase 161 operator evidence/claim guards; scoped rustfmt check; tracked diff check and explicit whitespace scan over all four untracked owned source files. Final TMPDIR and /tmp inventories contain no Phase162 daemon fixture directory. No unrelated wallet/node suite was repeated after prerequisite 16.

## Simplification, Cleanup and Source Snapshot

One existing fixture policy factory carries the explicit test-only serving switch; one restrictions module owns the small bounded driver and handle-free wallet evidence. Static permission checking is extracted to keep the body session below the approximate 161-line trigger. Shared short-test cleanup avoids duplicate guards. All new helpers stay within four owned paths, without extracting or widening the frozen 628-line inbound helper module.

Seed cleanup is declared before runtime owners and disarmed only after successful configured fixture construction. Lifecycle and short-test guards are declared before handle-owning fixtures. Client assertions run in a bounded spawned task; timeout abort is awaited, peer cleanup and listener shutdown/join occur before outcome assertions, and normal success proves all handles closed before datadir removal. Frame payloads are bounded to 16 KiB, handshake/drain loops to eight frames each, the whole socket client to 30 seconds, and peer cleanup/join to five seconds each. No separate fault-injection coverage of every cleanup branch is claimed.

| Owned source suffix                         | Lines | SHA256                                                           |
| ------------------------------------------- | ----- | ---------------------------------------------------------------- |
| tests/filter_index/phase162/restrictions.rs | 593   | e81ac2e1c4f7d459813b2e50c40dae9709571f765b94e7170e277642d9d232c6 |
| tests/filter_index/phase162/fixtures.rs     | 440   | faa009380301d6e6acbf032fd814f5cf9e31ddcbdb629787477fe5fbc5d62615 |
| tests/filter_index/phase162.rs              | 156   | e462b3b68d2b25dd32c256d33bbd2894d02c32022b8d295698053ca5ca423810 |
| coins_flush/tests/phase162.rs               | 367   | 6ec93f51f2f261a7161cb51050458014cae09b4080016c7ebec39e2420f3e49a |

These hashes remained unchanged from the frozen prerequisite caller through the full acceptance/regression/lint run. Source/stub scan found no blocking placeholder. Optional cleanup disarming and normal default states are deliberate lifecycle values; the 102-block preflight's raw digest is diagnostic provenance, not an independent oracle substitute. No new production endpoint, authentication route, file-access pattern or schema introduces an unregistered threat surface.

## Declared Mitigations

| Threat     | Executed scoped evidence                                                                                                                                 |
| ---------- | -------------------------------------------------------------------------------------------------------------------------------------------------------- |
| T-162-05-1 | Explicit body activation/effective download permission, actual retained positive and physical deleted negative; unchanged public creating-history guard. |
| T-162-05-4 | Same peer/session positive before exact in-window negative, source-owned 290-block bound, strict Pong fences and independent retained filter success.    |
| T-162-05-2 | Genuine successful prior scan/job and exact cold-reloaded snapshot/progress/target/freshness equality, permitting only Failed/error disposition.         |
| T-162-05-3 | Hermetic keys/scripts/outpoints/raw failures stay in test source/assertions; this summary exports aggregate facts and safe error categories.             |

Independent source/mitigation review and unresolved-High disposition remain root gates. This table supplies executed evidence, not ASVS certification.

## Deviations and Commits

The owned compile diagnostic was corrected using the existing available socket APIs. The checked resource amendment bounds only the public test descriptor import and fixes scan ordering around possible earlier-prefix deletion. The exact bounded stored-metadata assertion then exposed an outside-owned formatter defect; execution stopped, independent diagnosis/planning/checking created prerequisite 16, and its separately owned formatter fix plus compatibility tests restored the unchanged caller. No assertion, parser grammar, production matcher or legacy repair workaround was weakened here. No authentication gate occurred.

Task 1, Task 2 and metadata commits are explicitly deferred by D-14 to root's whole-phase clean finalization. Shared state/roadmap/requirements updates are parent-owned and intentionally not performed by this executor.

## Parent Registry Handoff and Limits

The new exact path is `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/restrictions.rs`. Its ordered Knots breadcrumb anchors are wallet/wallet.cpp, wallet/rpc/transactions.cpp, coins.cpp, node/blockstorage.cpp, net_processing.cpp, each under packages/bitcoin-knots/src. Root already registered this exact source block; the parity checker passes.

For parent-owned parity evidence, add this summary and final integrated selector to the scoped retained-filter/prune coordination evidence, body serving/limited-serving boundary and wallet creating-payload eligibility records. The actual restriction caller belongs beside retained-filter RPC/peer preservation evidence, without claiming a new production behavior difference. Existing retained-filter source, Knots source baseline and implementation semantics remain unchanged; prerequisite16 owns its distinct future descriptor write correction.

Root must still perform final source/security review, default `bash scripts/verify.sh`, lifecycle/current-doc checks, evidence-gated requirement activation, registry/LOC updates and strict Git finalization. Private completed-prefix transport receipts belong to Plan09; the physical client totals here do not prove those counters.

The inherited synthetic root, caller-supplied branch work and Open Bitcoin 210000 versus Knots Regtest 150 halving interval remain explicit limits. This proof does not establish whole-chain Knots consensus differential, most-work branch choice, archive-scale service, peak resource/reclamation performance, public-network readiness, production funds or automatic historical repair. Prerequisite 16 corrects future writes while retaining existing legacy decoder behavior; no migration/backfill/raw-text rewrite is implied. BIP37/V0, assumeutxo/dual-chainstate, archive/public defaults/network-CI, production-service/funds and milestone audit/shipment remain excluded; Phase161 presentation/schema/redaction and prior crash-window advisories remain unchanged. Requirements remain pending and v2.5 is unaudited/unshipped.

## Self-Check: PASSED

All four owned source paths and this summary exist. The final full proof, 19-test regression and strict lint receipts have actual successful exit status, with nonzero test discovery and recorded timings. Final source hashes match the frozen consumer evidence; baseline a77edab276a9daa086055aa61e6143ce86bc1218 is unchanged. There are exactly two top frontmatter delimiters, the required lifecycle identity and empty requirements-completed. No task commit is falsely claimed. Formatting, parity, managed and claim checks pass; temporary fixtures are absent and Cargo is idle. Parent-owned shared files and Git were not changed by this executor.
