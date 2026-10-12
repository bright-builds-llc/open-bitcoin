---
phase: 162-real-prune-retention-and-integrated-parity-proof
plan: "04"
subsystem: testing
tags: [authenticated-rpc, inbound-sockets, retained-basic-filters, cold-reopen, permissions]
requires:
  - phase: 162-01
    provides: Genuine normal-maturity recipe and independently decoded raw-block expectations
  - phase: 162-03
    provides: Actual paired manual/automatic pruning and same-datadir cold reopen
  - phase: 162-14
    provides: Ordinary selected trusted-header persistence for cold stale authorization
  - phase: 162-15
    provides: Verified full client run, historical guards and corrected pinned denial controls
provides:
  - Actual authenticated retained active/bodyless/stale RPC comparisons
  - Exact physical cfilter/cfheaders/positive-checkpoint replies under three configured grants
  - Pinned all-family no-grant disconnect and bounded session settlement
affects: [162-05, 162-06, 162-09, 162-10, 162-12]
tech-stack:
  added: []
  patterns: [Same-datadir configured client observations, raw-input oracle expectations, fresh denied connections]
key-files:
  created:
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/rpc.rs
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/inbound.rs
  modified:
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162.rs
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/fixtures.rs
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/coins_flush/tests/phase162.rs
key-decisions:
  - Close both client tasks from the actual accepted Plan15 full scenario, without inventing another Plan04 run.
  - Physical client totals and zero live reservations do not substitute for internal written-prefix receipt controls.
  - Preserve absent-capability disconnect and advertised All-only/no-backend silent refusal as distinct policies.
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 162-2026-10-11T16-25-37
generated_at: 2026-10-11T20:42:33Z
duration: evidence closure only; implementation timing not separately recorded
completed: 2026-10-11
---

# Phase 162 Plan 04: Authenticated Retained RPC and Inbound Client Summary

**The same genuinely pruned and cold-reopened daemon datadir serves exact independent active, bodyless and stale BASIC RPC/socket results, including positive h1000 checkpoints and configured permission controls.**

## Closure Scope and Attribution

Both tasks are complete from actual implementation and executed evidence. The earlier Plan04 executor implemented the client modules and configured factory, then stopped after two stale-stop failures. Checked prerequisite Plans14 and15 supplied the producer correction, historical guard compatibility, pinned negative expectation correction and successful complete-corpus run.

This handoff writes **only this summary**. It inspects frozen source, prior summaries/reviews/diagnoses and actual receipts; it runs no Cargo/Bazel command and makes no source, shared-state, registry or Git change. The key-files above describe the full Plan04 implementation, not files newly changed by this documentation closure.

The successful execution is explicitly **Plan15's** `phase162_integrated_lifecycle`, run from2026-10-11T20:23:45.755Z to20:27:43.971Z: **1 passed / 0 failed**,238.05-second libtest /238,216-ms wrapper. No separate Plan04 success run or timing is invented.

Material guidance includes repo AGENTS/sidecar/overrides, architecture/testing/verification/Rust guidance, phase CONTEXT/RESEARCH/UI, Plans04/15, Summaries01/13/02/03/14/15, supplemental FOUNDATION/RETENTION/CHECKPOINT reviews and tracked STALE-HEADER/NO-GRANT diagnoses. Both active lesson inputs were already fully loaded in this continuing execution:7,188bytes/2,397estimatedtokens, no archives. Phase161 presentation, redaction, schemas and defaults remain unchanged.

## Actual Same-Scenario Prerequisites

The scenario starts with1,001 genuinely accepted blocks at normal maturity100, actual durable body/undo/coins, raw-input oracle provenance and configured BASIC startuph7. The unchanged real idle worker completes125 bounded zero-peer turns toh1000. Actual extensionh1001 and shallow replacementh999..1002 disconnect three blocks and connect four from ancestor998; final active positions number1,003 and accepted immutable records1,006.

Normal authenticated manual pruning physically removes both matesh0..20. Configured automatic550 retention uses584 distinct bounded **codec-only nonactive pressure bodies**, then ordinarily removes694 more active pairsh21..714 and preserves the trailing288 pairsh715..1002. Every runtime/store/context/worker owner closes before the same-path configured cold reopen.

`SettledIdentity` preserves exact durable coins B, metadata with deliberately deleted undo excluded, empty transitional heads, safe BASIC checkpoint/protection and all1,006 immutable record identities/bytes through retention, reopen and clients. These1,006 preservation comparisons are not claimed as1,006 new independent oracle calculations.

Measured final automatic usage is579,152,814→578,426,929bytes; the550MiB target is576,716,800bytes and remains exceeded by1,710,129bytes of ineligible pressure. Auxiliary encoded values total578,091,088bytes. Allocated file observation is1,052,647,424bytes; pressure/apply/reopen takes198,312ms. These logical and allocated samples are separate; neither is a peak-memory/disk, immediate reclamation or physical-cap guarantee.

## Task 1: Authenticated RPC Acceptance

The configured factory creates actual `RpcHttpState` with parsed credentials and the same shared durable `ManagedRpcContext`. Requests go through `handle_http_request`, checking HTTP status and bounded JSON response bodies. This is actual HTTP-handler/server-state dispatch, not an additional claim of a TCP HTTP-client run.

| Required behavior                                | Actual source/assertion and executed evidence                                                                                                                                                                                          |
| ------------------------------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Retained filter results after paired loss/reopen | Nine named hashes cover manual-prunedh0/h20, automatic-prunedh21/h714, retainedh715, active replacementh1000/h1002 and displaced canonicalh999/h1000.                                                                                  |
| Independent exact bytes/header                   | Each result equals raw-oracle `encoded` and display-order header; both omitted type and explicit basic selectors run for every hash:18 exact successful comparisons.                                                                   |
| Authentication before dispatch                   | Missing and invalid credentials for each method produce HTTP401 and the exact challenge, with no result field, even when method parameters are invalid:four controls.                                                                  |
| Exact getindexinfo                               | Unselected, exact positional and named basic-index selectors return the exact baseline index name, synced flag and height; absent and case-mismatched selectors return empty objects. Initialh7 is unsynced; finalh1002 is synced.     |
| Error and mutation boundaries                    | Initial not-yet-indexed target gives the existing-1 error; unknown filter type gives exact-5. Before/after paired presence and BASIC summary remain unchanged; the outer settled-identity comparison preserves full durable authority. |

The successful full-run log records `authenticated_hashes=9 selectors=2 unauthorized=4 initial_sync=false7_then_true1002 independent_raw=true`. No request-time filter generation, repair, private persistence or body restoration is introduced.

## Task 2: Exact Physical Inbound Acceptance

The actual configured loopback/ephemeral listener is started through `start_inbound_listener_for_runtime_with_context_and_announcements`. Version/verack completes and a pre-request Ping/Pong fence drains handshake traffic without accepting a compact-filter frame as handshake success.

Each granted session sends the original ten requests and compares the complete physical frames against independently produced expectations:

| Requested range/ancestry                                   | Frames per granted session                                    |
| ---------------------------------------------------------- | ------------------------------------------------------------- |
| Activeh19..21, crossing manual/automatic bodyless deletion | Three cfilters and one cfheaders                              |
| Activeh713..715, crossing automatic loss/retained boundary | Three cfilters and one cfheaders                              |
| Displaced canonicalh997..1000                              | Four cfilters, one cfheaders and one positive-h1000 cfcheckpt |
| Active replacementh999..1002                               | Four cfilters, one cfheaders and one positive-h1000 cfcheckpt |

Each cfheaders expectation checks the independent predecessor and ordered hashes. Both checkpoint replies contain **exactly one** independently computed h1000 header; canonical displaced and active replacement commitments differ. Expected filter values originate in raw blocks/independently resolved spent outputs, never Rust query output. Bodyless cfheaders framing invokes the existing pinned serializer over those independent commitments.

Each granted case receives20exactframes/1,890bytes; three cases total **60 physical frames /5,670bytes**. This comprises42cfilter,12cfheaders and6cfcheckpt frames. Every request has its serial Pong fence and observes a default/zero live transport snapshot after completion.

### Configured Permission and Service Matrix

| Parsed configuration                            | Actual expected and observed behavior                                                                                  |
| ----------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------- |
| General serving on, listener off, in-only       | Listener Disabled; no bound endpoint. Serving permission alone never starts listening.                                 |
| General serving on, listener on, in-only        | Exact original ten-request granted session; general scope1, explicit0, All0.                                           |
| General off, listener on, explicit blockfilters | Exact granted session; explicit scope1, general0, All0.                                                                |
| General off, listener on, All                   | Exact granted session with enabled BASIC backend; All scope1 and explicit0.                                            |
| General off, listener on, in-only               | Own COMPACT_FILTERS absent; each of the original ten requests uses a fresh connection and terminates without response. |

All actual peer sessions assert NODE_NETWORK_LIMITED and absence of NODE_NETWORK; COMPACT_FILTERS equals the expected grant. Actual admitted peer count is one and live general/explicit/All/offered scope matches the independently requested policy. Reopens use the existing parsed factory, preserve automatic550 and recipe/network identity, and leave relay/body-serving defaults intact. Public bind permission remains false.

Absent-capability denial accepts only zero bytes followed by EOF/ConnectionReset/ConnectionAborted within five seconds. Any response byte, Pong, other error or timeout fails. There is no post-denial Pong expectation or reuse of a disconnected socket. All ten original denied requests passed; peer disappearance, zero charges, joined listener and unchanged settled identity are checked after each fresh session.

The distinct existing All-only/**backend-disabled** control was separately executed by Plan15: advertised capability permits all three valid families to be silently ignored while the later Pong succeeds; invalid type still disconnects. It passed1/1 in0.93seconds. This is separate constituent policy evidence, not another long-corpus history.

### Resource, Cleanup and Observation Boundary

The existing configured peer limit2/reserved slots1 and finite production resource policies remain. The test requests short fixed ranges, reads response frames only up to16KiB, bounds handshake drains to eight messages, and imposes a30-second client deadline. Peer disappearance and listener shutdown/join each have five-second bounds; disappearance is polled every10ms. Client timeout aborts and awaits the task; listener shutdown/join is attempted before propagating client failures.

Successful sessions report zero live transport reservations, actual peer removal and joined listener. The consuming fixture handoff drops HTTP/shared context/authority/store owners; the worker tail exercises normal and retained-typed-failure settlement, joins, cold reopens, a final raw Fjall open, directory deletion and path absence. The full scenario logs `joined=true failure_joined=true reopened=true cleanup=true`.

**Observation boundary:** The60frames/5,670bytes are physically received client totals. Public FilterTransportSnapshot exposes live charges, not cumulative completed-prefix receipts. No internal written-prefix count is inferred from these totals or from a zero snapshot. Plan09's actual private successful/partial-write controls remain mandatory; D-06 is not waived. Queue/resource ceilings are inherited production bounds, not new peak-queue measurements from this test.

## Failures, Prerequisite Corrections and Source Limits

1. Original Plan04 stopped twice at canonical staleh1000 getcfilters after real prune/reopen. Those failures are parent-reported and preserved in STALE-HEADER-DIAGNOSIS, not retroactively passing runs. RPC retained lookup succeeded while cold HeaderStore lacked the stale stop, despite surviving ScriptsValid.
1. The short diagnostic isolated missing ordinary trusted-header publication; its private publication counterfactual was diagnostic only. Plan14 corrected the actual selected durable producer, retaining lazy selection, admitted store-bound authority, body/undo→checkpoint→unlink→coins order, typed errors, no-op/legacy/header-only behavior and old missing-header fail-closed controls. No fixture injection or policy relaxation supplied final success.
1. Plan14's full flush suite initially reported70/71 because a raw contiguous source guard broke on rustfmt wrapping. Plan15 corrected the function-scoped IfNeeded/delegate/no-snapshot guard and preserved comment-aware lineage/order mutations;508checker tests and71whole-flush tests passed before the costly client run.
1. Plan15's first post-fix full run passed all three granted sessions, then failed the frozen no-grant Pong expectation:0passed/1failed,241.57-second test/254,697-ms wrapper. Independent diagnosis confirmed pinned Knots net_processing.cpp3162–3169 and p2p_blockfilters.py195–218 require absent-capability disconnect. Only the approved inbound negative expectation changed; production and every granted request/expectation/fence stayed frozen.
1. The corrected focused102-block denial test passed before the successful complete584-row rerun. Root's later focused-test-only RAII cleanup correction passed fresh focused denial1/1 in1.81seconds and strict node+RPC lint in1.95seconds. The full238-second corpus was not repeated because the scenario/client helpers did not change; final source differs only in that focused test's cleanup/setup from the successful corpus snapshot.

Plan15 compared twelve other frozen source hashes and eight original inbound helper/exchange functions plus three granted critical fragments byte for byte. This closure independently rechecked the five Plan04 source hashes below. The full-run inbound snapshot was c3cffcbd585f34a7d18c527f0f0747165a55ce9b2b72f392c915bce57f9b155c; final1d991... includes only the reviewed focused cleanup correction.

Supplemental FOUNDATION review resolved its subprocess/upper-bound warnings; RETENTION and CHECKPOINT reviews were clean within their exact earlier scopes. Those are constituent review results, not final approval of every Phase162 client/source/security surface. Root retains final independent reviews and native/formal gates.

## Executed Evidence Used for Closure

Plan15 directly executed the following, and this closure inspected the actual receipt/log files. No command below was rerun by this handoff.

```bash
export PATH=/tmp/open-bitcoin-bun-1.3.9-fresh:$PATH
bun run scripts/command-timings.ts run --key phase162-stale-after-normal-header-checkpoint -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --bin open-bitcoind --all-features phase162_integrated_lifecycle -- --test-threads=1 --nocapture
```

| Evidence                                          | Actual receipt/result                                                                                                                                                         |
| ------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Full successful same-datadir scenario             | .local/open-bitcoin-dev/command-timings/phase162-stale-after-normal-header-checkpoint/2026-10-11T20-23-45.755Z-e9f8b108-2431-4ae8-8564-ec92fe418da4.json; exit0,1/1,238,216ms |
| Full semantic no-grant RED                        | Same key /2026-10-11T20-07-12.418Z-0be048b2-5288-4f79-8a7a-171ce081cbc1.json; exit101,0/1,254,697ms                                                                           |
| Final focused denial after RAII                   | .local/open-bitcoin-dev/command-timings/phase162-no-grant-cleanup-focused/2026-10-11T20-34-02.929Z-73e9ae03-43a9-4a2a-a33f-890418bf2356.json; exit0,1/1,5,116ms               |
| Distinct All-only/no-backend                      | .local/open-bitcoin-dev/command-timings/phase162-no-grant-existing-all-only/2026-10-11T20-23-21.488Z-7a2c3f43-788e-46ec-8cb5-01dde2a32407.json; exit0,1/1,1,190ms             |
| Final strict node+RPC all-target/all-feature lint | .local/open-bitcoin-dev/command-timings/phase162-no-grant-cleanup-clippy/2026-10-11T20-34-08.068Z-a0dae962-5a9a-4c6d-8f76-e46ec689b1c4.json; exit0,2,182ms                    |

Actual full-run output remains `/tmp/phase162-plan15-client-final.log`; the semantic RED output is `/tmp/phase162-plan15-client.log`. These local logs are additional observations, not portable committed fixtures. Tracked summaries and source assertions retain the reviewed evidence limits.

Plan15 also passed affected source checkers158/159/160, registered breadcrumbs for1,133Rust files, managed all checks with zero findings, scoped rustfmt and diff checks. This handoff only checks Markdown/diff and artifact/receipt/source identity. The full default bash scripts/verify.sh, workspace/Bazel gates and formal activation are not claimed.

## Frozen Plan04 Sources and Existing Registration

| Source                                                                                  | Final SHA256                                                     |
| --------------------------------------------------------------------------------------- | ---------------------------------------------------------------- |
| packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162.rs          | 2f7c387ede5ae683e9e9b9737ad45deb40faa9add316b70c2cbe3a48b1c8c105 |
| packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/rpc.rs      | 3ef67bdf9dde29a7dabba37ec03c7325519d2a22f9c077d81869ffd8e18259a4 |
| packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/inbound.rs  | 1d991eeabfe62e1722828a9cdeb79fbf9ebc122b7a9e0fdb3b7d6ac0b85e7c60 |
| packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/fixtures.rs | affd6b1bbeaca218c658da05b735e98ff6cc0362cf941d20887a86266085036a |
| packages/open-bitcoin-rpc/src/bin/open_bitcoind/coins_flush/tests/phase162.rs           | 5475f93e828e6f092fa77ea5a0b123f7a3c76f01bbca460317c969c0a2b4b59c |

Existing registry groups are phase162-configured-daemon-fixture, phase162-actual-idle-worker, phase162-retained-authenticated-rpc and phase162-retained-inbound-filter-clients. RPC anchors pinned rpc/blockchain.cpp, rpc/node.cpp and index/base.cpp; inbound anchors net_processing.cpp, index/blockfilterindex.cpp and test/functional/p2p_blockfilters.py. Shared fixture/module anchors validation.cpp, index/blockfilterindex.cpp and node/blockstorage.cpp; worker anchors validation.cpp, bitcoind.cpp and node/chainstate.cpp, all under packages/bitcoin-knots.

No new source breadcrumb is needed for this summary-only closure. Root may retain these source paths, this summary and the Plan15 full receipt in the eventual parity evidence; requirement owners stay unchanged.

## Must-Have and Declared Mitigation Mapping

| Contract                                                       | Concrete evidence                                                                                                                                             |
| -------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Must-have authenticated retained active/bodyless/displaced RPC | Task1 exact18successful comparisons,4auth refusals, baseline index selection and immutable settled-state checks on the cold retained fixture.                 |
| Must-have all three physical families and positiveh1000        | Task2 original granted10request matrix, exact predecessor/order/branch commitments,20frames/session and distinct one-entry canonical/replacement checkpoints. |
| rpc.rs→actual HTTP state/dispatch                              | Existing factory builds authenticated shared state; RPC helper calls actual handle_http_request and checks HTTP/JSON outcomes.                                |
| inbound.rs→actual configured listener                          | Existing inbound_startup caller, real socket handshake, physical reads, permissions/services and cleanup.                                                     |
| Original lifecycle→same reopened clients                       | coins_flush/tests/phase162.rs calls manual→automatic cold reopen→RPC→inbound on the consuming same fixture; no new client datadir/authority.                  |
| T-162-04-1 authentication                                      | Actual configured auth, malformed-parameter-before-dispatch refusal controls and no credential output.                                                        |
| T-162-04-2 access/session scope                                | General/explicit/All matrix, own service assertions, no-grant physical disconnect and separate All-only/backend absence.                                      |
| T-162-04-3 ancestry integrity                                  | Independent raw inputs, exact physical active/stale predecessor chains and both nonemptyh1000checkpoints.                                                     |
| T-162-04-4 resource settlement                                 | Short bounded exchanges, deadlines, sentinel fences where permitted, peer removal, zero live charges and joined listeners/store closure.                      |

Declared mitigations are covered within Plan04's accepted observation boundary. Independent final security/High-finding disposition remains parent-owned; no ASVS certification is claimed.

## Commits, Residual Limits and Next Plan

Task1, Task2, their prerequisite RED/GREEN work and this metadata commit are **deferred by D-14**. No Git action occurred in this closure. Requirements-completed remains empty; root owns STATE/ROADMAP/REQUIREMENTS/todo, parity registries, generated LOC, formal activation and final Git.

No blocking source stub remains in the integrated client proof. The focused denial test's intentionally unused null oracle/empty response vectors are negative expectations only, not independent successful-response data. This documentation closure introduces no production endpoint, auth path, schema, dependency or trust authority.

The inherited Open Bitcoin subsidy interval210000 differs from pinned Knots Regtest150. Normal-maturity first-party runtime acceptance plus independently pinned BASIC/protocol evidence is not whole-chain Knots consensus differential or most-work-selection proof. Codec pressure is not accepted history. Raw pending-intent absence, private partial-write receipts, outbound transport, wallet/body restrictions and whole-phase fault products remain their later-plan obligations.

**Next05:** Extend the same recipe/datadir lifecycle with a genuinely h20-funded durable named wallet and normal pre-prune rescan checkpoint, then preserve that wallet/job on refusal after deletion. An explicitly eligible parsed body-serving/download-permission session must retrieve retainedh715 and receive exact NotFound for deletedh714 on the same connection, both inside the source-derived288+2limited window, while retained filter lookup still succeeds. Existing inbound client source stays frozen; Plan05 owns its listed factory/restrictions/lifecycle paths.

BIP37/V0, assumeutxo/dual-chainstate, archive scale, public-default/public-network CI, implicit repair/import/download, production-service/readiness/funds, milestone audit/archive/shipment and prior crash-window advisories remain excluded or unchanged. Phase161 presentation/support redaction is preserved. Plan04 completion does not complete CFPR-02/CFGR-02 or ship v2.5.

Cargo remains idle; no duplicate corpus or source mutation was performed for this closure.

## Self-Check: PASSED

This summary and all five Plan04 source paths exist. Fresh SHA256 checks match the frozen table. The actual full success and semantic RED receipts exist with exit0/101 respectively; the physical final log contains exactly three20-frame/1,890-byte granted successes, ten zero-byte denied disconnect successes and1/1 libtest success. Final focused-denial, separate All-only and strict lint receipts were inspected; prerequisite source/guard/native receipts remain attributed to their actual executors. The baseline commit a77edab276a9daa086055aa61e6143ce86bc1218 exists. Existing source breadcrumb groups and comments are retained. Lifecycle identity and empty requirements-completed are preserved; only top frontmatter uses standalone dash delimiters. Markdown uses the installed GFM/frontmatter-aware formatter and scoped diff checks. No task commits are claimed under D-14. Only this summary was written; shared artifacts, sources and Cargo/Git state remain untouched.
