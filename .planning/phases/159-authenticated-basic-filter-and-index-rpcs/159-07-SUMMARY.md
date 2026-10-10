---
phase: 159-authenticated-basic-filter-and-index-rpcs
plan: "07"
subsystem: daemon-rpc-evidence
tags: [authenticated-http, genuine-acceptance, paired-prune, retained-stale, closed-reopen, private-faults]
requires:
  - phase: 159-03
    provides: Genuine managed acceptance and retained validation provenance
  - phase: 159-04
    provides: Same-authority bounded queries and recovered integrity
  - phase: 159-05
    provides: Captured-frontier completion and terminal owner settlement
  - phase: 159-06
    provides: Actual authenticated shared HTTP dispatch and error projection
provides:
  - Configured daemon HTTP evidence over genuinely accepted continuous history
  - Actual paired body/undo deletion and all-handle production reopen with exact active/stale/pruned results
  - Genuine displaced-before-indexing missing-row provenance before and after reopen
  - Node-private production owner/query fault evidence composed with actual RPC error projection
affects: [159-08, phase-verification]
tech-stack:
  added: []
  patterns: [genuine empty-store fixtures, explicit evidence boundaries, store-bound cfg-test faults]
key-files:
  created:
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/rpc.rs
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/rpc/retention.rs
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/rpc/failures.rs
    - packages/open-bitcoin-node/src/sync/tests/filter_index/catch_up/rpc_faults.rs
    - .planning/phases/159-authenticated-basic-filter-and-index-rpcs/159-07-DAEMON-PROOF.md
  modified:
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index.rs
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/fixtures.rs
    - packages/open-bitcoin-node/src/sync/tests/filter_index/catch_up.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/query.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/publication.rs
key-decisions:
  - Accept from an empty store and flush genuine coins before actual daemon configuration/open; preserve raw-seeded History as legacy only.
  - Use a genuinely accepted successor displaced before indexing to prove ready missing-row provenance without raw mutation or clean-epoch fabrication.
  - Keep inaccessible private fault hooks inside node cfg(test), composing their actual authority outcomes with the real RPC mapper and explicitly distinguishing that evidence from configured HTTP.
  - Reuse the existing per-store publication fault control for injected point-read failure; add no global flag or public node test feature/API.
requirements-addressed: [CFRP-01, CFRP-02]
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 159-2026-10-09T16-11-49
generated_at: "2026-10-09T18:59:00Z"
duration: approximately 22min including focused Task3 replan and complete daemon regression
completed: 2026-10-09
---

# Phase 159 Plan 07: Configured Daemon Retention and Fault Proof Summary

**Authenticated configured daemon BASIC results now have genuine continuous acceptance, actual paired-prune/all-handle-reopen and exact retained active/stale/missing-row evidence, with private node faults linked explicitly to the production RPC projection.**

## Accomplishments

- All three tasks are implemented and scoped verification passes under the independently approved Task3 evidence-placement amendment. Tasks remain pending root's consolidated commit and formal phase verification. Nine source paths comprise four created Rust files and five modified files; the proof report and this summary are the two planning artifacts. The initial executor timestamp was not separately captured; duration is approximate.
- The actual configuration parser selects regtest/BASIC/manual pruning/password auth, `open_runtime_store` opens durable storage, `open_authoritative_network_runtime` initializes the owner, and its cloned network authority enters the actual shared `ManagedRpcContext`/HTTP state. No detached index or unrelated metrics-store authority is used. Unflushed acceptance observed by HTTP, same-handle scheduled progress, and actual daemon maintenance-worker completion prove shared authority behavior.
- Genuine fixtures open an empty store directly, accept every block through the managed consensus stage/absorb path, persist ordinary bodies and flush the actual coins fence before configured reopen. Raw snapshot seeding remains only the explicit legacy control. Rotating historical scripts plus a real same-block spend support independent pure historical filter/header generation from accepted positions and genuine undo; output-only omission is demonstrably different.
- Initial incomplete indexing serves available rows promptly and classifies genuinely accepted unavailable rows with exact indexing -1. Later accepted lag preserves the initial-sync latch, releases the HTTP context while Pending, permits independent summary requests, and completes through ordinary owner progress. Processed height2 can lead durable coins height1. Exact result keys, type/default/name selection, disabled/unknown/V0 precedence, genuine header-only absence, legacy unavailable, password/cookie authentication, duplicate names and root scope are exercised through configured authenticated HTTP.
- A continuous 401-block fixture fully indexes heights0–400, then the existing serialized explicit manual prune-plan owner actually deletes the height20 body/undo pair. An additional accepted but unindexed height401 is genuinely displaced by eleven validated replacement blocks. Indexed stale400 remains queryable, missing accepted stale401 retains its precise ready missing-row diagnostic, and replacement active401 is served. Real coins settlement and consuming every handle precede a fresh production configured reopen; all three exact stored responses and the missing-row diagnostic remain unchanged.
- Ten new private-node tests begin from genuine empty-store acceptance and earn configured recovery before accepting pending work. The actual production handle/query/completion paths exercise BeforeRecords/BeforeCheckpoint/BeforeProtection/AfterCommit, corrupt target/parent, missing predecessor/accepted target, injected backend read failure and raw-clone invalidation. Authentic validation history remains independent of these failures. Inconsistent corrupted/missing projections correctly refuse real reopen; no test forces recovery or grants a clean epoch.

Material guidance: repository AGENTS.md, Bright Builds sidecar, placeholder-only overrides and architecture/code-shape/testing/verification/Rust standards; both active lesson files were fully loaded, 7,188 bytes and 2,397 conservative estimated tokens, with no audit trigger. Approved CONTEXT/RESEARCH/nonvisual UI contract, Plan03–06 summaries, Plan04 measurements and relevant Phase157/158 evidence/fixtures informed execution. Root supplied synchronized main and the materialized pinned Knots baseline. No frontend, dependency, public/default activation or funds operation was added.

## Verification Evidence

All Cargo work used pinned Bun PATH, Rust 1.94.1 and `scripts/command-timings.ts`, with serialized target access. Sessions were polled within 60 seconds. The full daemon regression's existing volume test remained CPU-active and completed normally; no loader termination, retry recovery or security/global setting change occurred.

| Check | Exact positive result |
| --- | --- |
| Configured daemon suite, `phase159-daemon-http`, selector `phase159_daemon_rpc` | **9 passed**, zero failed/ignored; final distinct-script run 12.99s. |
| Focused configured failure selector, `phase159-daemon-failures` | **5 passed**, zero failed/ignored; 6.16s. Subsequent nine-test suite and full daemon regression include the final fixture scripts. |
| Complete daemon binary regression, `phase159-daemon-regression` | **74 passed**, zero failed/ignored/filtered; 152.50s. Includes all nine new configured tests and existing 65 daemon/maintenance/shutdown/prune controls. This final run also executes the live-Fjall-lock refusal assertion. |
| New private authority fault suite, `phase159-node-rpc-faults-final` | **10 passed**, zero failed/ignored; 1,420 filtered; 4.60s after restoring the read-fault mutation. |
| Existing node bounded-query/readiness regression, `phase159-node-query-readiness-regression` | **35 passed**, zero failed/ignored; 1,395 filtered; 41.06s. Includes target/parent/read-bound/zero-progress controls and all 21 readiness controls. |
| Actual RPC dispatch/projection suite, `phase159-rpc-fault-projection` | **5 passed**, zero failed/ignored; 315 filtered; 0.01s. Exact missing matrix, redacted typed fault mapping, registry/scope and precedence. |
| RPC strict scoped lint, `phase159-daemon-clippy` | All-targets/all-features/no-deps Clippy with `-D warnings` passed; 5.71s. |
| Node strict scoped lint, `phase159-node-rpc-faults-clippy` | All-targets/all-features/no-deps Clippy with `-D warnings` passed; 7.53s. |
| Scoped Rust formatting, diff review and `git diff --check` | Passed for all nine source paths. |
| Bright Builds managed all checks | Zero findings. New files also individually measured below 628 lines; publication entry remains exactly 628. |

Counts overlap and must not be added as distinct coverage. The ten-test new selector does not claim execution of the reused 35 query/readiness tests or the actual five mapper tests. Full native/Bazel verification, independent source/security/lifecycle review, breadcrumb registration, requirement completion and Git operations remain root/Plan08 gates.

## Real Deletion, Reopen and Scope

See [DAEMON-PROOF](./159-07-DAEMON-PROOF.md) for the complete nineteen-new-test case map and executing boundary of every fault category.

- Initial accepted chain: **401 blocks, heights 0–400**. Additional unindexed original successor: **height 401**, bringing original accepts to **402**. Genuine equal-height replacement: **11 blocks, heights 391–401**.
- Actual paired target: **height20**, one returned deleted hash, body=None and undo=None immediately and after reopen, with durable have_pruned=true.
- Actual logical payload values: **451,352 bytes before**, **450,226 after**, **1,126-byte loss**. These are logical stored payload lengths, not physical disk use or hardware/power-loss proof.
- Before close, a second Fjall open refuses while the configured handles are live. `close(self)` consumes HTTP state, shared context, authoritative runtime and its retained DurableSyncRuntime, store and config. All temporary snapshots/clones are dropped; no maintenance worker runs in this retention test. Only path/block recipes survive. Fresh production configuration/open succeeds and exact active/stale/pruned responses compare byte-for-byte.
- The naturally missing accepted stale row was accepted then displaced before its index turn. It is not injected disk corruption. It yields exact -32603 `Filter not found. This error is unexpected and indicates index corruption.` before/after reopen, demonstrating retained genuine ScriptsValid provenance independently of filter rows and active membership.
- Fixture consensus uses synthetic genesis, valid easy PoW, canonical BIP34 heights, P2SH and explicit maturity **1**. It does not prove hard-coded network genesis, standard maturity **100** or real-network synchronization. Explicit manual owner-plan deletion respects the unchanged trailing **288** window; default regtest automatic prune-after **1000** remains unchanged and unexercised. No ordinary prune-RPC eligibility/automatic-threshold/end-to-end-default-network claim follows.
- The real worker-success test lets HTTP complete before normal worker Always/shutdown settlement. Direct `stop_basic_filter_readiness` failure controls prove immediate owner stop, not graceful daemon shutdown order. Existing complete daemon regressions retain the graceful drain/producer settlement/clean-marker ordering evidence.

## Composed Fault Boundary

Node cfg(test) hooks do not exist in the RPC crate's normal node dependency. The parent-approved Task3 amendment therefore preserves actual configured HTTP for accessible cases and adds private genuine-owner/query fault tests, then exercises the real RPC mapper. This is explicitly composed evidence, not a claim that private disk/publication injections ran inside daemon HTTP.

The new node tests individually name all four publication boundaries and the target/parent/missing-parent/missing-target/backend/raw-clone cases. The read-failure seam reuses the existing per-store private publication control; no global injection flag, persistent test key, public node API or feature is introduced. Direct record tampering models external corruption under an already genuinely earned epoch; the helper neither invalidates nor readmits integrity, grants acceptance, installs progress or performs recovery. Actual raw APIs still invalidate every clone before mutation.

Production `prepare_filter` calls the actual context handle's `basic_filter_query`, then `query_failure`/`project_query`. HTTP `finish` awaits the captured barrier outside the context and calls the same handle's `complete_basic_filter_read`, then the same production mapping functions. The five actual dispatch tests assert the precise Missing matrix and fixed redacted errors for Storage/Readiness/UnknownLegacy. This connects the real node outcome categories to the shipped projection without a second classifier. A deterministic injected backend error is software fault evidence, not an observed operating-system or disk failure.

## RED, Issues and Simplification

- **Retention mutation RED:** an empty prune plan discovered one test and failed the actual deleted-hash assertion. Restoring `[20]` returned GREEN and achieved real payload loss. No no-op plan can satisfy the test.
- **Read-fault mutation RED:** disabling the cfg(test) BeforeQueryRead branch discovered one test and failed the typed BackendFailure assertion on the actual authority query. Restoring it returned the final ten-test GREEN.
- Initial import, sibling-field visibility and unsupported `-prune` CLI assumptions were fixture setup errors, not product-bug RED. The final fixture retains its shared context clone explicitly and obtains manual prune mode through the actual `open-bitcoin.jsonc` parser. Existing raw-seeded Phase157 fixture semantics are unchanged.
- The explicit simplification pass reused the real configured owner and current maintenance worker, existing generation/projection functions, existing private per-store fault control and existing mapper/readiness regressions. It separated only the retention/failure/node-fault concerns into children. No generalized integration framework, alternate storage, detached index, extra worker, cache weakening, test dependency or production regeneration path was added.

## Deviations from Plan

1. **Rule3: Parent-approved Task3 evidence-placement amendment.** Cross-crate cfg(test) visibility made the original all-faults-through-daemon request incompatible with the ban on public node test APIs/dependencies. The planner revised Task3 and the independent checker passed it with zero blockers/warnings. New node catch_up/rpc_faults.rs and thin declaration/query fault helpers stay private; every evidence boundary is named in this summary/report. Tasks1/2 remain actual configured daemon proof.
2. **Rule3: Parent-approved private publication glue.** The read-failure seam requires a new cfg(test) `BeforeQueryRead` variant and sibling visibility for the existing cfg(test) fault field in `storage/fjall_store/filters/publication.rs`. No new field, global flag, production branch/API or dependency is added. One blank line removal keeps the already628-line entry within the managed limit. This exact extra path was authorized before editing.

## Task Commits and Plan08 Handoff

1. Task1 — actual configured authenticated shared owner, initial/later semantics and real maintenance completion: pending root consolidated commit.
2. Task2 — genuine paired prune, validated replacement, retained active/stale/missing history and all-handle reopen: pending root consolidated commit.
3. Task3 — accessible configured failures plus approved private-owner/actual-mapper fault composition: pending root consolidated commit.

Plan08 must register exact breadcrumb comments for the four new Rust paths in key-files.created, incorporate the approved publication.rs extra path and evidence-placement distinction, update parity/docs/READMEs/UAT within its scope, and run full native/Bazel/source/security/lifecycle verification. No hardware/manual UAT or requirement completion is claimed. Production block-serving and wallet pruned-body restrictions were not changed; the existing daemon regression remains green. CFPR-02/CFGR/CFNET completion, public networking, funds safety and broad milestone client proof remain outside this plan.

No staging, commit, push, hook bypass or STATE/ROADMAP/REQUIREMENTS/todo/config/lesson mutation was performed. No authentication gate occurred.

## Known Stubs and Threat Review

None. Empty objects implement exact disabled/unmatched summary semantics; temporary empty batches/initial oracle values are test setup, not unimplemented production data. No TODO/FIXME, mock accepted flag, successful-empty missing filter or forced recovery authority remains.

T-159-23 is covered by empty-store genuine stage/absorb and explicit legacy controls; T-159-24 by actual paired loss, live-lock refusal and closed configured reopen with exact retained responses/provenance; T-159-25 by actual HTTP auth/context/fence/lifecycle controls plus privately injected production query/owner failures and exact real redacted RPC projection. The amendment introduces no new production trust surface. Independent full-phase source/security approval remains root-owned.

## Self-Check: PASSED

- All four created Rust files, five modified source paths, the proof report and this summary exist.
- Both mutations were restored; 19 new tests pass at their documented boundaries, with 74 full daemon regressions, 35 reused node regressions, five actual mapper tests, strict scoped lint and format/diff checks recorded separately.
- No commit hash, completed requirement, full native/Bazel pass, hardware/manual validation or independent phase/security acceptance is claimed. Those remain the consolidated root gates.
