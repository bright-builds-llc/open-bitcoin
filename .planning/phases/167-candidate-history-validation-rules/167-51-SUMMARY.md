---
phase: 167-candidate-history-validation-rules
plan: "51"
baseline_commit: 1b472fb225a290df5cdb06eeea2dc67c624c648e
subsystem: stored-body-request-completion-and-public-success
tags: [rust, compact-block, durable-storage, canonical-submission, request-lifetime, evidence]
requires:
  - phase: 167-50
    provides: Accepted ordered compact failure and canonical compact-header admission
provides:
  - Sender-only release for unvalidated full-block receipt
  - Matching request completion only after actual stored body and fresh full-linkage eligibility
  - Public Accepted and Duplicate cleanup after final durable, BASIC, achieved and physical checks
  - Real same-store Duplicate error distinction and request-conservation fault controls
affects: [167-17, 167-18, 167-19, 167-25, 167-20, CVLD]
tech-stack:
  added: []
  patterns: [one bounded ordered request finish, full-linkage stored-body qualification, final public provider hook]
key-files:
  created:
    - packages/open-bitcoin-node/src/network/tests/compact_stored_body_cases.rs
    - packages/open-bitcoin-node/src/network/canonical_admission/tests/phase167/compact_stored_body.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/validation_history/tests/compact_download_cleanup.rs
  modified:
    - packages/open-bitcoin-node/src/network/mempool_lifecycle.rs
    - packages/open-bitcoin-node/src/network/action_translation.rs
    - packages/open-bitcoin-network/src/peer/inventory_state.rs
    - packages/open-bitcoin-network/src/peer/block_requests.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/validation_history.rs
key-decisions:
  - Keep unvalidated receipt sender-only; finish all matching requests only after a real retained and freshly eligible body.
  - Qualify public Duplicate from the same held store, preserving genuine StorageError source versus missing-row support error without invented source.
  - Admit an uncached accepted genesis body only by full selected genesis identity, active height-zero root and absent cache entry; retain nonroot cache and full-linkage guards.
  - Disclose E2 positive-control timing deviation and the supported-surface BASIC barrier reachability limit without retrospective credit.
requirements-completed: [CVLD-01, CVLD-02, CVLD-03, CVLD-04, CVLD-06]
requirements_scope: bounded-plan51-contribution-only
global_cvld_acceptance: open
generated_by: gsd-execute-plan
generated_at: "2026-10-30T22:08:04Z"
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
commits: false
status: canonical-summary-pending-independent-qa-and-root-final51
---

# Phase 167 Plan 51: Stored eligible body completion at final public success

Plan 51 closes the compact request lifecycle only when a real body has been retained and is currently eligible. An unvalidated full Block receipt releases its sender, leaving another peer’s same-hash request and unrelated requests intact. One bounded ordered registry operation removes matching requests, requested-block sets, compact partials and consumed markers for the exact hash after qualified success. E1 checks exact header/hash/work, current binding and epoch, retained body and full ancestor linkage. It does not promote a header-only, stale, failed or disconnected candidate into request-completion authority.

The public canonical provider performs Accepted cleanup only after its existing `confirm_durable → achieved inventory → physical inventory` order and actual same-store body load. Duplicate keeps intrinsic/domain/accepted-identity checks first, then loads the retained body from the same held Fjall store before any matching cleanup. A corrupt serialized BlockIndex row propagates a genuine operational `StorageError` source; a removed row returns support unavailable without a fabricated source. A different submitted witness does not replace actual stored bytes. This backend-read failure distinction from the pinned Knots warm Duplicate path is intentional and must be recorded in Plan 19 parity documentation. A post-absorption operational failure may advance accepted state, but it cannot claim new E request cleanup or fictitious rollback. Global CVLD acceptance remains open.

## Test-first evidence and corrective source decisions

E2’s compiled test-first RED showed that an invalid full Block cleared the other requester’s order (`[]` instead of `[167512]`) before E1 production changed. A separate unchanged positive control was **not** run on that same E2 test-only frame before E1 edits. The checked [timing disposition](167-51-E2-POSITIVE-CONTROL-TIMING-DISPOSITION-AMENDMENT.md) preserves that process deviation: two accepted Plan 50 old controls had identical function bodies and had passed, and both later passed separately 1/1 after E1, followed by Task 2 GREEN. Those later passes are not described as an on-time preimplementation positive. E4 had its own correctly ordered old positive 1/1 and genuine public Accepted RED: two matching requests remained after final success. A separate E1 failed-ancestor correction RED showed a NonExtending descendant’s request was cleared after its ancestor failed; the focused GREEN preserved both descendant and unrelated request order using the existing managed full-linkage eligibility projection.

The original E4 warm Duplicate tests failed during fixture setup because ordinary `request_missing_blocks` refuses an already known accepted hash. The checked [compact fixture refinement](167-51-E4-POST-ACCEPT-COMPACT-FIXTURE-AMENDMENT.md) instead used real post-accept Version/Verack/SendCompact and an incomplete CompactBlock through managed receive. The probe observed GetBlockTxn, a matching ordered request and a partial; no private request injection or known-block guard bypass was added. Generic E2 `connect_stored_block` rejects changed witness bytes at `unexpected-witness` before its Duplicate branch, preserving requests/cache/state. The E4 public `command.submit` path separately proves witness-altered Duplicate from the actual stored body; the [checked boundary](167-51-E2-WITNESS-GUARD-AND-LIVE-UNDO-AMENDMENT.md) keeps those interfaces distinct. E3 checks live typed undo after connect and durable undo only after a real flush.

The first final broad provider command exposed a real regression: four old root/reopen/witness Duplicate controls failed `retained body eligibility unavailable`. Fresh and reopened accepted genesis have a proved stored body but intentionally no volatile `blocks_by_hash` cache entry. E1 now permits only a **cache-absent, entire proved selected genesis body and hash at the active height-zero root**, with its header/binding/epoch/availability and full-linkage checks intact. Every nonroot body still requires exact cache equality. Independent review caught an intermediate predicate that would also permit a conflicting cached root; a second one-file correction added the explicit `cached.is_none()` conjunct. The four old controls then passed 1/1 each, while E4 cached-descendant/corrupt/missing controls passed 11/11 and the E3 failed-ancestor control passed 1/1. No private wrong-cache fixture was fabricated. The checked [root correction](167-51-ROOT-DUPLICATE-CACHE-QUALIFICATION-AMENDMENT.md) and [absent-cache tightening](167-51-UNCACHED-ROOT-CONJUNCT-CORRECTION-AMENDMENT.md) retain the failed intermediate runs as diagnostics.

The E2 in-memory store has no real storage/flush/Always/BASIC fault switch. The checked [fault reachability disposition](167-51-E2-FAULT-REACHABILITY-DISPOSITION-AMENDMENT.md) assigns reachable durable failure request-conservation controls to E4’s real Fjall provider, without adding a fake E2 hook. E4 named controls reach publication faults, BASIC BeforeBody, an actual `accepted full flush` failure through BeforeCoins, achieved inventory, header-advance capacity and physical inventory. They compare the appropriate post-effect accepted/native facts and matching/unrelated request snapshots; corrupt/missing Duplicate cases compare full post-tamper native, accepted and request baselines. The one BASIC backlog hypothesis failed earlier at `prepared BASIC resource projection`, before absorption. Source inspection showed this pre-effect progress gate requires the index caught up to the parent; the checked [barrier reachability amendment](167-51-BASIC-BARRIER-REACHABILITY-AMENDMENT.md) replaces an unreachable supported-surface terminal barrier fault test with a named backlog refusal. That control passed with exact stage/source, tip at nine, tenth body absent and full accepted/native/request state unchanged. The terminal Pending and Ready-Err barrier branches remain **unrun**; no claim is made about arbitrary future external faults.

## Final frozen verification

All four literal task selectors passed on the final source: **7/7, 20/20, 20/20 and 11/11**, each with no ignored tests. The first final format attempt failed only on an owned test-module order and was corrected under a one-file checked release. The second final matrix passed commands 1–10 but its broad provider selector found the uncached-root regression above; commands 12–20 were not run in that attempt. These are retained diagnostics, not final credit.

On the corrected v4 frame, **all 20 published final commands** then ran fresh and serially from format and exited zero. Format, strict Clippy and all-target build passed. Every behavioral selector discovered named tests; across overlapping final selectors, **3,771 test results passed** and five ignored appearances were recorded (one reorg measurement control and four node-wide controls), with no failures. The broad provider selector passed **49/49**, including all four previously failing Duplicate controls. The full node-wide command passed **2,439 tests** across its groups (2,408 + 15 + 3 + 13), with four documented ignores and zero failures; it took 11,748 seconds. The [CORE-v2](../../../.local/open-bitcoin-dev/phase167/plan51/execution-v1/CORE-v2.json), [test-discovery record](../../../.local/open-bitcoin-dev/phase167/plan51/execution-v1/final-test-discovery.json) and [post-proof](../../../.local/open-bitcoin-dev/phase167/plan51/execution-v1/post-proof.json) retain exact names, ignored reasons, raw streams, arguments, timings, exits, probes and post-run checks.

The final causal frame contains **502 first-party source** files (13 direct owners, 489 protected), **201 native** files, one separate Byteview input and five pinned Knots anchors. Its current and saved copies still match. Strict original-before BSD `patch -F0` inverse and forward restored ten complete owner bodies and three genuine absences, then reproduced all 13 final bodies without fuzz or offsets. Normal formatting keeps every direct owner at or below 628 physical lines. The nine-key environment was proved in a real asynchronous Bun child before each command, with physical TMPDIR, default Cargo target, natural host process guard and cooperative command-timings lock. Slow macOS directory reads and long CPU-heavy test work completed naturally; no target/cache/flag/toolchain override or unrelated process termination occurred. The [independent whole-source review](../../../.local/open-bitcoin-dev/phase167/plan51/independent-whole-source-review-v3.json), [independent final command review](../../../.local/open-bitcoin-dev/phase167/plan51/execution-v1/independent-final20-raw-review.json), [whole-plan review](../../../.local/open-bitcoin-dev/phase167/plan51/independent-whole-plan51-review-v2.json), and [Summary release](../../../.local/open-bitcoin-dev/phase167/plan51/root-summary-release-v1/summary-release.json) bind the final evidence. All owned jobs are closed and the [lease-return receipt](../../../.local/open-bitcoin-dev/phase167/plan51/execution-v1/closed-jobs-lease-return.json) returns the source to ROOT.

## Remaining boundary

Plan 17 must first bind a full final Phase 167 source manifest, including changed Rust modules outside Plan 51’s 502-file causal subset, then renew current guards against the actual product route. Plans 18, 19, 25 and 20 still own verifier registration, parity/breadcrumbs, root documentation, the full default native verifier, fresh coverage/Bazel and independent phase/lifecycle review. No Git staging, commit or push has occurred. Phase 168 has not started.
