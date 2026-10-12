---
phase: 162-real-prune-retention-and-integrated-parity-proof
plan: "06"
subsystem: testing
tags: [public-sync-once, outbound-tcp, retained-filters, same-datadir, independent-oracle]
requires:
  - phase: 162-05
    provides: Complete funded-wallet and body restrictions on the same pruned lifecycle
  - phase: 162-14
    provides: Ordinary trusted-header persistence for cold stale ancestry authorization
  - phase: 162-15
    provides: Frozen independent inbound clients and pinned denial controls
  - phase: 162-16
    provides: Correct bounded public descriptor storage with legacy compatibility
provides:
  - Consuming daemon-to-public-runtime handoff with actual lock refusal and cold reopen
  - Public sync_once and TcpPeerTransport exact physical responses for all three families
  - Full continuous lifecycle through outbound closure, final wallet/state reopen and removal
affects: [162-07, 162-09, 162-10, 162-12]
tech-stack:
  added: []
  patterns: [Explicit numeric-loopback discovery, bounded std socket remote, joined outcome capture]
key-files:
  created:
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/outbound.rs
  modified:
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162.rs
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/coins_flush/tests/phase162.rs
key-decisions:
  - Preserve actual same-datadir parsed prune and activation policies through existing public APIs.
  - Explicitly empty DNS discovery and configure exactly one ephemeral numeric loopback peer.
  - Join the remote and close the public runtime before inspecting captured failure outcomes.
  - Keep physical client totals distinct from private completed-prefix receipt evidence in Plan09.
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 162-2026-10-11T16-25-37
generated_at: 2026-10-11T21:50:13Z
duration: 18 min recorded execution window
completed: 2026-10-11
---

# Phase 162 Plan 06: Public Outbound Same-Datadir Proof Summary

**The complete genuinely pruned lifecycle cold-opens its exact datadir into public `sync_once`, serves 20 byte-exact independent outbound frames including both positive h1000 ancestries, then closes, reopens and removes the datadir.**

## Performance and Ownership

- Tasks: 2/2. Exactly three owned test-source paths changed, including one new module. This plan changes no production source, dependency, API, visibility, recipe, oracle, wallet behavior or public default.
- First separately recorded clock: 2026-10-11T21:31:56Z; initial mandatory context loading preceded it. Full acceptance ran 21:41:58.284Z–21:46:51.460Z. Final Cargo receipt ended 21:47:08.855Z; final summary clock is 21:50:13Z.
- Material guidance: repo AGENTS, Bright Builds sidecar/overrides, architecture/code-shape/testing/verification/Rust standards, active lessons, execute-plan workflow/template, checked Plan06, phase CONTEXT/RESEARCH/UI contract and Summaries03/04/05/14/15/16.
- Both active lesson inputs were read completely: 7,188 bytes and conservative 2,397 tokens. No archives were loaded and the parent-confirmed existing audit baseline has no new trigger. No project skills exist in the two prescribed directories.
- Parent supplied synchronized main baseline a77edab276a9daa086055aa61e6143ce86bc1218 and materialized pinned Knots a9aee730466ac67d35a3c03ee24676be5e045878. Every Cargo job used pinned Bun1.3.9 PATH, Rust1.94.1, the timing wrapper and exclusive serial checkout ownership.
- Root retains STATE/ROADMAP/REQUIREMENTS/todo, parity registries, generated LOC, independent source/security review, full native/formal gates and all Git. No staging, commit, branch change, push, hook bypass or requirement activation occurred.

## Task 1: Public Runtime and Consuming Handoff

The original integrated test still runs its real normal/failure maintenance settlement and cold restore before the new outbound insertion. It checks the exact settled identity and persisted funded wallet, and a real concurrent Fjall second open refuses while the daemon owners exist. Existing `Phase162Fixture::close` consumes HTTP state, shared context, network/runtime and store owners into the unchanged handle-free `Phase162Handoff`.

The public outbound helper consumes that handoff. The existing configuration parser reads the same datadir with explicit Regtest, BASIC, general serving enabled and inbound disabled. Existing relay/block-serving/inbound policies feed `DurableSyncRuntime::open_with_configured_runtime_activation`; `BasicFilterStartupMode::Enabled` is explicit. Existing public setters apply the parsed prune mode and general BASIC policy. No private sender, private session entry, snapshot reconstruction, new history or repair is called.

The final maintenance restore uses the frozen factory's manual-only setting, so outbound preserves that actual parsed mode rather than inventing a new automatic setting. The earlier continuous retention stage still parses and executes automatic550. Its soft target and the final outbound mode are separate facts. Actual Regtest identity/consensus parameters and prune mode are asserted, and the actual wire version advertises NODE_COMPACT_FILTERS plus NODE_NETWORK_LIMITED without NODE_NETWORK.

Before any `sync_once`, one ephemeral `127.0.0.1:0` remote is bound. `SyncRuntimeConfig` explicitly supplies exactly that numeric manual peer and an empty DNS list; candidate peers must equal the one-element manual list. It sets target peers1, rounds1, retries0, two-second connect/read budgets and exactly requests+3 incoming messages. No Mainnet seed list or fallback discovery reaches the dialer. The actual call is solely `runtime.sync_once(&mut TcpPeerTransport, timestamp)`.

The cold runtime verifies durable coins B, empty heads, the exact active fork1002 tip and earned prune bit. Every one of the1,003 active positions has both body/undo absent below715 or both present from715. The same checks repeat after serving. Complete durable snapshot and protection identity are compared before/after the public call. After remote join, zero transport charges, empty peer registry and empty announcement outboxes are asserted; runtime closure permits another physical cold store open.

The returned handoff has exactly the original path, recipe ID and ordered input digest. A final configured daemon reopen checks the original `SettledIdentity`, all1,006 immutable records and the existing funded-wallet snapshot/job assertions, preserving their permitted Failed/error normalization. It rechecks every active pair, closes every owner, proves a final raw store open, removes the unique datadir and asserts path absence.

## Task 2: Independent Complete Physical Families

The std loopback remote reads the actual outgoing version, performs physical version/verack and a Ping/Pong handshake fence, then sends these ten requests:

| Range/ancestry                                            | Physically accepted replies                    |
| --------------------------------------------------------- | ---------------------------------------------- |
| Active19..21, crossing manual/automatic deletion          | 3 cfilters and1 cfheaders                      |
| Active713..715, crossing automatic loss/retained boundary | 3 cfilters and1 cfheaders                      |
| Displaced canonical997..1000                              | 4 cfilters,1 cfheaders and1 nonempty cfcheckpt |
| Active replacement999..1002                               | 4 cfilters,1 cfheaders and1 nonempty cfcheckpt |

All20 complete physical frames, including network magic, command, payload length, checksum and payload, equal independent pinned expected bytes in exact order. Filter/header commitments come from the existing raw-input oracle, never Rust filter-query responses. Old-range cfheaders use only those independent predecessor/hashes through the existing pinned Python peer serializer and bounded process helper. Both checkpoint frames contain exactly one independently computed h1000 header, and canonical/replacement headers must differ. Final EOF rejects any additional frame.

The observed outbound totals are **10 requests,20 complete CF frames and1,890 bytes**:14 cfilters,4 cfheaders and2 checkpoints. The bounded remote session takes133ms, including accept/handshake/requests/closure but excluding public runtime opening and expected-frame preparation. Public sync summary independently records exactly one attempted/connected peer, no failed peer,13 received messages, one attempt and actual Regtest identity. Final live peers/outboxes/transport reservations are empty.

**Receipt boundary:** These are actual client-observed physical frame/byte totals. They are not invented `FilterWrittenPrefix` counters. The successful public route executes the existing production sender and its completion path, but Plan09 must still run the actual private successful/partial-write receipt controls. Zero live reservations alone cannot establish cumulative receipt counts; this plan does not waive D-06.

## Full Integrated Measurements

The final complete `phase162_integrated_lifecycle` passed **1/1**,292.37-second libtest runtime and293,176-ms wrapper. Both plan task commands target that same full scenario; one final joint run satisfies their shared acceptance without a redundant identical pressure run.

| Observation                                                                     | Actual final result                                                                                          |
| ------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------ |
| Initial accepted blocks / final active positions / distinct accepted records    | 1,001 /1,003 /1,006                                                                                          |
| Normal maturity / startup processed height / actual zero-peer maintenance turns | 100 /h7 /125                                                                                                 |
| Seed / catch-up / complete lifecycle                                            | 29,314 /10,189 /292,354 ms                                                                                   |
| Genuine extension / shallow replacement                                         | h1001 /h999..1002, ancestor998, three displaced rows                                                         |
| Manual logical usage before/after / absent pairs                                | 1,066,313→1,061,726 bytes /h0..20                                                                            |
| Codec-only nonactive pressure                                                   | 584 rows,578,091,088 encoded value bytes                                                                     |
| Automatic550 logical usage before/after                                         | 579,152,814→578,426,929 bytes                                                                                |
| Automatic newly deleted pairs / total absent / retained active pairs            | 694 /715 /288                                                                                                |
| Automatic deleted range / trailing retained range                               | h21..714 /h715..1002                                                                                         |
| Actual soft target / remaining excess                                           | 576,716,800 /1,710,129 bytes; target unattained                                                              |
| Pressure construction/streams/apply/reopen                                      | 211,612 ms                                                                                                   |
| Allocated files at pressure sample / later before outbound                      | 1,053,851,648 /605,433,856 bytes                                                                             |
| Authenticated retained RPC                                                      | 9 hashes×2 selectors,4 unauthorized controls                                                                 |
| Three unchanged granted inbound sessions                                        | 60 exact CF frames /5,670 bytes total                                                                        |
| Unchanged absent-grant negatives                                                | 10 fresh zero-byte terminal disconnects                                                                      |
| Existing body proof                                                             | Same eligible connection: h715 Block, deleted in-window h714 NotFound,2 Pongs                                |
| Existing wallet proof                                                           | Genuine h20/25,000-sat funding; saved checkpoint1002; later-range refusal requiring deleted creating history |
| New outbound physical observations                                              | 10 requests,20 frames,1,890 bytes,133 ms                                                                     |
| All task/store/peer closure and final removal                                   | passed; peers0, charges0, joined and datadir absent                                                          |

Allocated values are two point observations, neither peak space nor a cap/reclamation guarantee. Pressure rows are codec-only nonactive payloads, never validated history or deletion candidates. The unchanged expected `index unavailable` maintenance diagnostic is followed by asserted typed failure settlement and successful cold restoration.

The final aggregate evidence line exports the stable recipe ID, a verified-digest boolean, counts, booleans, heights and timings only. It includes startup/idle/connect/reorg/manual/automatic/reopen/RPC/inbound/body/wallet/outbound/cleanup handoffs and does not export raw commitments, identifiers, scripts, credentials or paths.

## Verification and Initial Failures

Every Cargo row uses pinned PATH and `bun run scripts/command-timings.ts run --key KEY -- cargo ...` with `--manifest-path packages/Cargo.toml`. Daemon tests use `-p open-bitcoin-rpc --bin open-bitcoind --all-features`, serial libtest threads and `--nocapture`.

| Key                                            | Actual result                                                                                    | Wrapper ms |
| ---------------------------------------------- | ------------------------------------------------------------------------------------------------ | ---------- |
| phase162-outbound-public-preflight,first       | Compile failure: startup enum import at wrong re-export                                          | 8,447      |
| same key,second                                | Compile failure: public prune setter requires mutable handle binding                             | 3,006      |
| phase162-outbound-compile                      | All-features daemon no-run compile passed,10.90s                                                 | 11,296     |
| phase162-outbound-public-preflight-red         | Deliberate activation-off configuration:0passed/1failed, exact missing BASIC advertisement,6.29s | 7,488      |
| phase162-outbound-public-preflight-green,first | 0passed/1failed, owned accepted remote socket yielded WouldBlock,4.18s                           | 10,621     |
| same key,corrected                             | 1passed,5.05s, explicit accepted blocking mode                                                   | 15,466     |
| phase162-outbound-clippy                       | Strict RPC all-targets/all-features passed,5.67s                                                 | 6,156      |
| phase162-outbound-public-preflight-final       | RAII-cleanup refinement:1passed,4.20s                                                            | 16,578     |
| phase162-outbound-public-policy-preflight      | Final same-datadir parsed policies:1passed,6.75s                                                 | 15,048     |
| phase162-complete-lifecycle                    | Final entire584-row same-datadir scenario:1passed,292.37s                                        | 293,176    |
| phase162-outbound-final-clippy                 | Final strict RPC all-targets/all-features passed,2.49s                                           | 2,802      |

Two compile failures triggered an explicit checked replan: inspect exact signatures/import, scoped format, no-run compile, bounded semantic control, then full corpus. Parent acknowledged that same-three-file replan. The activation-off RED is deliberately incomplete **test configuration**, not a production bug. The later WouldBlock was local remote-driver setup; explicit accepted blocking mode and stage-specific I/O error categories fixed it. Neither failed preflight consumed the pressure corpus, and both close/join/reopen/remove before propagating captured outcome assertions.

The first compile briefly reported an artifact-directory lock, then progressed normally. Process inspection found only this checkout's Cargo/compiler and an unrelated ESP-toolchain Cargo; no unrelated job was terminated. All commands completed normally, with resumable pressure polling and CPU liveness evidence; no yield or estimate was treated as timeout.

Exact full proof receipt: `.local/open-bitcoin-dev/command-timings/phase162-complete-lifecycle/2026-10-11T21-41-58.284Z-59de46b3-f7ff-45da-a3a1-05f544f4214a.json`.

Exact semantic RED receipt: `.local/open-bitcoin-dev/command-timings/phase162-outbound-public-preflight-red/2026-10-11T21-37-45.890Z-ff9fc80a-7251-4cb9-b328-63f33b555452.json`.

Exact final policy preflight: `.local/open-bitcoin-dev/command-timings/phase162-outbound-public-policy-preflight/2026-10-11T21-41-24.845Z-4baaab7b-5aac-490c-91ad-07bcf35f72db.json`. Exact final lint receipt: `.local/open-bitcoin-dev/command-timings/phase162-outbound-final-clippy/2026-10-11T21-47-06.053Z-92ca9289-a0c2-498e-8e37-4fc6f79ad9c5.json`.

Additional checks pass on final source: scoped rustfmt, parity breadcrumbs for1,135Rust files, managed all checks with0findings, Phase160 peer-filter and Phase161 operator claim guards and tracked diff whitespace. Explicit owned-source whitespace/stub scans supplement git diff because these planned modules are still untracked. No unaffected node/wallet suite or redundant full corpus was run. Root retains the full default native/Bazel/formal checks.

## Simplification and Cleanup Review

Small owned helpers separate expected frames, public configuration, bounded wire I/O, handshake, actual remote and runtime/store settlement. The std-only driver reuses the existing bounded child supervisor and pinned serializer without a dependency, new authority, private node API or extraction from frozen inbound code. The original intentionally linear integration test preserves continuous ownership and provenance across every consuming handoff.

All new helper functions remain below the approximate161-line trigger, and owned files are606/159/410lines. Absolute20-second remote deadlines are refreshed before each socket read/write; accept polling is finite, handshake drains are capped at8frames, payloads at16KiB and incoming messages at requests+3. The remote socket closes on success/error, and its task is joined before captured outcomes are inspected. The public runtime closes before a subsequent physical store open. Short preflight RAII is declared before its fixture owners, and the full scenario keeps the existing guard declared before owners. No separate exhaustive cleanup-fault matrix is claimed.

No blocking stub was found. Empty discovery lists and zero-frame handshake-only preflight expectations are intentional exclusions/controls. No new production endpoint, auth path, schema, dependency or trust-boundary file surface was added; the new loopback and temporary-datadir test surfaces are already covered by the plan's declared threats.

## Source Snapshot and Parent Registration Handoff

| Owned source suffix                     | Lines | SHA256                                                           |
| --------------------------------------- | ----- | ---------------------------------------------------------------- |
| tests/filter_index/phase162/outbound.rs | 606   | 06966508a66d211a3ee21e547464902cb1572780362031426778c3df76d29ab4 |
| tests/filter_index/phase162.rs          | 159   | f14b8f96862608730126bc2deb3ff60c4ec8004ee25f155f4666f3dbeb529e7b |
| coins_flush/tests/phase162.rs           | 410   | 550e8f4ad1045c2eb7c05abcd774856c0ed66bbb232f0c5555d542dfafabb7e8 |

These hashes stayed frozen through the final full proof and lint. Frozen fixtures/inbound/restrictions hashes still match Plan05/15: faa009380301d6e6acbf032fd814f5cf9e31ddcbdb629787477fe5fbc5d62615;1d991eeabfe62e1722828a9cdeb79fbf9ebc122b7a9e0fdb3b7d6ac0b85e7c60;e81ac2e1c4f7d459813b2e50c40dae9709571f765b94e7170e277642d9d232c6. Shared recipe remains82c73bc4dbd2990bf5b19947defa5b1b90aada230363b44978a691c7dac21497.

The actual new path is `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/outbound.rs`. Exact ordered breadcrumb anchors are packages/bitcoin-knots/src/net.cpp; packages/bitcoin-knots/src/net_processing.cpp; packages/bitcoin-knots/src/index/blockfilterindex.cpp; packages/bitcoin-knots/test/functional/p2p_blockfilters.py. Parent registered group `phase162-public-outbound-retained-filter-client`; the live checker passes.

For parent-owned parity publication, add this summary, the actual public outbound helper/preflight selector and final integrated selector/receipt beside retained-filter prune coordination and both-direction transport evidence. Preserve the distinct physical-client/private-receipt boundary and all inherited exclusions. No requirement owner or production behavior difference is created by this test-only plan.

## Declared Mitigation Evidence

| Threat     | Scoped executed evidence and remaining parent gate                                                                                                                                                     |
| ---------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| T-162-06-1 | Complete byte-exact physical frames through the existing production completion path, exact observed totals and zero final charges; Plan09 still owns direct private complete/partial receipt controls. |
| T-162-06-2 | Exactly one numeric ephemeral loopback peer, empty DNS/candidates assertion, actual Regtest handshake, explicit parsed BASIC/general activation and truthful LIMITED/noNETWORK service bits.           |
| T-162-06-3 | Finite accept/message/frame/socket work, actual joined remote, captured outcomes after closure, empty peers/outboxes/charges, physical cold open and final path absence.                               |

Independent final source/declared-mitigation review and unresolved-High disposition remain root gates; no ASVS certification is claimed.

## Deviations, Commits and Next Handoff

No implementation-scope expansion or outside-owned production defect was needed. Local import/mutable binding and socket setup errors were corrected after the checked replan. The final simplification pass preserves actual parsed handoff mode rather than hardcoding automatic550 after the frozen manual-only restore. All acceptance assertions remain; no seed, snapshot authority, policy relaxation or suppression was introduced.

Task1, Task2, RED/GREEN and metadata commits are explicitly **deferred by D-14**. Shared state, roadmap, requirements, parity/LOC and Git remain parent-owned and were untouched by this executor. No authentication gate occurred. Requirements-completed remains empty.

Ready for the remaining checked fault/receipt/parity/documentation plans. Plan09 must execute its actual private successful/partial-write accounting. Root must still perform independent reviews, full default `bash scripts/verify.sh`, lifecycle/current-document verification, evidence-gated requirements and strict Git finalization.

The synthetic root, caller-supplied branch work and inherited Open Bitcoin210000 versus Knots Regtest150 halving interval remain explicit limits. This proves scoped first-party accepted history and independent BASIC/protocol behavior, not whole-chain Knots consensus differential or most-work selection. Codec pressure is not validated history; physical samples are not caps/peaks. BIP37/V0, assumeutxo/dual-chainstate, archive scale, public defaults/public-network CI, implicit repair/import/download, wallet acceleration, production-service/readiness/funds and milestone audit/archive/shipment remain excluded. Phase161 presentation/schema/redaction and earlier v2.4 crash-window advisories remain unchanged.

## Self-Check: PASSED

All three owned source paths and this summary exist. Final source hashes match the full proof/lint freeze; four frozen consumer/recipe hashes match their initial values. Actual full1/1 and final preflight1/1 receipts record exit0, intentional semantic RED records exit101, and final strict lint records exit0. Breadcrumbs, managed/claim/rustfmt/whitespace checks pass. Baseline a77edab276a9daa086055aa61e6143ce86bc1218 is unchanged. Exactly two standalone top frontmatter delimiters, required lifecycle identity and empty requirements-completed are preserved. No task commits are falsely claimed. Final TMPDIR and/tmp inventories contain no owned daemon fixture directory, and every Cargo session exited; ownership is returned idle. Parent-owned shared files and Git were not changed by this executor.
