---
phase: 165-shared-exact-work-authority
plan: "09"
subsystem: durable-sync-branch-authority
tags: [proof-work, body-eligibility, sync, basic-index, typed-errors]
requires:
  - phase: 165-08
    provides: Sealed source/preference authorization and exact replacement failure identity
provides:
  - All eligible retained branches reconciled through existing guarded managed owners
  - Checked cached-body linkage with immutable unknown arrival provenance
  - Typed consensus-only fallback and accepted-prefix progress with immediate turn termination
  - Selected-proof-only best and side header request projection
  - Genuine live BASIC and legacy sync fixture integration
affects: [165-10, 165-12, 165-13, 165-14]
tech-stack:
  added: []
  patterns:
    - Separate immutable checked linkage from known historical ordering
    - Requery current eligibility after exact consensus invalidation
    - Same-live fixture attachment under actual runtime and publication serialization
key-files:
  created:
    - packages/open-bitcoin-node/src/sync/tests/phase165_candidates.rs
    - packages/open-bitcoin-node/src/sync/tests/phase165_candidates/fixtures.rs
    - packages/open-bitcoin-network/src/header_store/eligibility/tests/phase165_cached_order.rs
    - packages/open-bitcoin-node/src/sync/tests/phase158_preflight.rs
    - packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/fixtures/live.rs
    - packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/fixtures/return_branch.rs
    - packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/branches/manual_lifecycle.rs
  modified:
    - packages/open-bitcoin-node/src/sync/block_reconcile.rs
    - packages/open-bitcoin-node/src/sync/block_response.rs
    - packages/open-bitcoin-node/src/network/candidate_eligibility.rs
    - packages/open-bitcoin-node/src/network/work_authority.rs
    - packages/open-bitcoin-network/src/header_store/eligibility.rs
    - packages/open-bitcoin-network/src/header_store.rs
    - packages/open-bitcoin-node/src/chainstate/live_fixture.rs
    - scripts/check-phase164-canonical-identity/contracts.ts
    - scripts/check-phase164-canonical-identity.test.ts
key-decisions:
  - "Storage presence never establishes body eligibility or historical receive order."
  - "Unknown cached arrival provenance permits strictly greater work, never a manufactured equal-work tie."
  - "Only typed consensus failures permit candidate fallback; local failures propagate."
  - "Actual accepted-prefix progress ends the reconciliation turn before ancestry is reacquired."
  - "Requestable headers require actual proof facts and exact full selected parameters."
requirements-completed: [CWOR-02, CWOR-03]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 165-2026-10-14T16-09-59Z
generated_at: 2026-10-15T06:42:14Z
duration: 79min
completed: 2026-10-15
---

# Phase 165 Plan 09: Retained Branch Reconciliation Summary

**Durable sync activates every eligible retained branch through the existing sealed work authority, preserving genuine body order, unknown cache provenance, exact failure identity and actual durable progress.**

## Scope and Execution

Both tasks are complete. The parent-approved inventory contains 39 source/test/checker/breadcrumb paths; all exist, and every owned Rust/TypeScript path is at most 628 lines. Seven Rust test files were added or relocated with immediate breadcrumb registration. `runtime_state.rs` and `tip.rs` required inspection but no additional Plan 09 edit. Shared files also contain predecessor work; this summary attributes the Plan 09 behavior rather than claiming the whole checkout diff.

The first recorded RED command began at 2026-10-15T05:23:23.197Z. Summary preparation began at 06:39Z, approximately 76 minutes later; this is workflow duration, not a performance result. AGENTS.md, Bright Builds sidecar, overrides, architecture/code-shape/verification/testing/Rust/TypeScript standards, execution contract and predecessor summary informed execution. Both active lesson inputs were read completely: 7,188 bytes and 2,397 conservative estimated tokens. Parent orchestration owns shared planning state and initial bootstrap.

## Delivered Behavior and APIs

- `reconcile_best_chain_inner` checks cached bodies against actual selected header facts before ranking all eligible retained candidates. It derives common ancestry and replacement suffixes from actual retained lineage. Header synchronization keeps its existing best-header selector. Body activation uses exact work and genuine body eligibility, without height/hash tie ordering.
- Cached bodies enter through `retain_cached_candidate` and `check_cached_body_eligibility_detailed`. Actual validation runs once at the existing owner. `BodyEligibilityError::{Validation, Network}` preserves the actual `BlockValidationError`; compatible public checks retain their original `NetworkError` mapping. Parameter, capacity and local failures cannot become peer invalidity.
- Actual checked transaction linkage and immutable linkage seals remain independent of public ordering evidence. Raw-cache provenance is established before seal creation, and unknown ancestry stays unknown through pending descendants, later descendants, duplicates, authorization copies and accepted bindings. Existing genuinely known order is preserved. Unknown stronger work can qualify; unknown equal work cannot. Known candidates precede unknown candidates at equal work so an unknown entry cannot hide a genuinely preferred known branch.
- Activation uses the Plan 08 route: `authorize_reorg` → `retain_authorized_reorg` → lower `prepare_authorized_reorg`, or ordinary guarded stored connect. Public lower unqualified reorg still refuses. Authorization copies use cached checking for newly encountered bodies; replacement-array order cannot mint history. Actual bindings establish linkage, while known sequences are used only for ties. Source, owner, epoch and pre-effect guards remain intact.
- Private `ActivationFailure` distinguishes recoverable typed consensus errors from fatal local failures. Only actual block/transaction validation, missing-coin or output-overwrite causes can permit fallback; exact `ReplacementBlock` identity and nested sync classification remain preserved. Storage, missing undo/body, stale ownership and BASIC failures surface immediately, including when another viable candidate exists. After a pre-effect consensus failure, the turn re-queries current eligibility with attempted-hash exclusion instead of reusing revoked grants.
- If an extension accepts a valid prefix before a later consensus-invalid body, sync reports the genuine connected count, queues only the accepted final body and immediately ends that turn. The next turn reacquires active ancestry. Valid prefixes and siblings remain retained; only the exact invalid body is marked. Fatal errors propagate regardless of connected count.
- `HeaderStore::requestable_header_entries(&ConsensusParams)` is one read-only projection using the existing best ordering followed by retained sides. Both best and side entries require actual accepted proof facts and identical full selected parameters. Raw seeded/from-entries/record-position entries, generic unbound proof facts and wrong-parameter stores cannot schedule requests. Raw/checkpoint getters retain their existing meaning. Request budgets and active/cached/inflight exclusions remain enforced.
- Requested valid side/pending bodies are persisted without false connected credit. Tip notification follows actual active progress and durable persistence, including an accepted side branch whose header remains outside the best-header tip.

## Genuine Fixture Integration

`ForkFixture::compact_early` creates a fresh genuine live owner, accepts the actual root/common prefix, actually checks side bodies before the original suffix, then performs a real flush and requested BASIC setup. It does not retrofit sequence numbers into reopened state. Configured BASIC equal-work replacement uses the real atomic authorized reorg path and the actual early body history.

The cfg-test `attach_fixture_basic_owner_after_flush` requires genuine non-preview origin, Fixture scope, the same physical coins parent, complete matching coins/B/heads/metadata/undo/counts, no pending history, no owner and no existing lineage of any kind. Append proof acquisition precedes the publication lock; actual runtime authority and publication serialization cover physical comparison, current guarded verification and final field assignment. The storage wrapper invokes the existing counted guarded verifier and preserves suspended-preview refusal. The small private lineage constructor is cfg-test only. Raw/mechanical, unflushed, missing undo, foreign/stale proof and existing-owner controls refuse without field changes. Success preserves the core owner and body seals/order. No production constructor, work promotion or canonical bootstrap was added.

Historical impossible two-way preference fixtures now use genuinely preferred transitions: original A23 → B26 → extended A27, and earlier equal B39 against A39 followed by stronger A40. Original durable fences, immutable A/B rows, safe shared-prefix protection, real reopen and extra unflushed suffix assertions remain. Separate mixed-target sync integration proves shorter-harder preference. Actual explicit shorter/full disconnect lifecycle tests call managed disconnect and assert invalidated/paused BASIC plus the retained physical checkpoint; they do not fabricate an empty checkpoint or invoke a manual reorg grant.

Catch-up pure seeds replay complete available actual bodies in a new genuine core. Read-only reopened state is not forced into work adoption. Live missing-body and missing-undo controls delete actual physical inputs and exercise required-data refusal unchanged; a separate raw reopened control correctly refuses earlier with `UntrustedWorkLineage` while reasserting the physical absence.

Legacy header/request-only tests use named `admit_fixture_runtime_headers_only`, asserting the populated raw core remains without earned work even after protocol processing. Actual connection/progress/invalid-body tests use `seed_genuine_fixture_history` on a new path: actual validated bodies derive complete matching coins/undo/counts/work before the unchanged strict replay transfer. There is no populated-None normalization, empty-coins repair, restored missing source or relaxed strict helper.

## Executed RED/GREEN Evidence

Every Bun/Cargo command used the pinned Bun 1.3.9 command-local PATH. Every Cargo command used `bun run scripts/command-timings.ts run --key <key> -- ...`, with Rust 1.94.1 and no overlapping Cargo jobs.

```bash
PATH='/Users/peterryszkiewicz/.bun/install/cache/@oven/bun-darwin-aarch64@1.3.9@@@1/bin:'"$PATH"
```

| Check                                        | Actual result                                                                                                                                                                                                                                      |
| -------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Initial Task 1 RED                           | Three discovered failures: opposing header/body order, unavailable-best suppression and late-parent release. First GREEN 3/3, expanded to 7/7, 9/9 and 11/11.                                                                                      |
| Typed fallback/prefix RED                    | 11 passed, 2 failed: missing active undo hidden by another viable candidate; valid prefix followed by invalid body lost progress. First fix 12/13 exposed stale eligibility; fresh requery produced 13/13.                                         |
| Extra corruption control                     | Initial 13/14 diagnostic showed malformed local coins refuse before prefix acceptance. Corrected the test premise/name to early local-corruption/no-peer-invalidity with unchanged-root refusal. This is not executed after-prefix fatal evidence. |
| Raw outbound request RED                     | One actual failure proved a raw unproved seeded header was requested before the selected-proof projection fix.                                                                                                                                     |
| Task 1 `phase165-sync-candidates`            | Final 15/15 passed, zero failed/ignored; 7.96s test execution.                                                                                                                                                                                     |
| Task 2 `phase165-sync-candidates-regression` | Final 15/15 passed, zero failed/ignored; 8.30s test execution.                                                                                                                                                                                     |
| Task 2 `phase165-sync-reorg-regression`      | Unfiltered inherited suite: 37 passed, 0 failed, 1 pre-existing ignored measurement, including long pruning. Printed 239.85s has the process caveat below.                                                                                         |
| Authority affected sweep                     | 118/118 passed, 22.75s, before the final selected projection/helper refinements; covers owner/freshness/refusal/canonical-selector controls.                                                                                                       |
| Block/request/response migrations            | Initial 10 passed/11 failed at fixture setup; final 21/21 passed, 9.34s, preserving budgets, peer/refusal and progress assertions.                                                                                                                 |
| Legacy reorg reconciliation                  | Initial 4 passed/3 failed at fixture setup; final 7/7 passed, 3.00s.                                                                                                                                                                               |
| Preflight/required-source/BASIC affected     | Final 15/15 passed, 6.04s, after final projection source; includes both relocated block_reconcile::phase158_tests cases named below.                                                                                                               |
| Fixture attachment negatives                 | 3/3 passed; raw/mechanical, unflushed/missing undo, foreign/stale guarded proof and existing-owner refusal.                                                                                                                                        |
| Network-only pure coverage                   | Clean final run: 648 unit + 2 parity + 2 property tests passed, zero failed/ignored; unit execution 17.53s.                                                                                                                                        |
| Scoped final clippy                          | Node and network all-targets/all-features with `-D warnings` passed after final source/test changes; 11.12s tool execution.                                                                                                                        |
| Workspace fmt check                          | Passed after final source changes.                                                                                                                                                                                                                 |

The relocated tests `phase158_preflight_sync_reconcile_uses_production_reorg_and_scheduled_owner` and `phase158_preflight_sync_missing_undo_is_explicit_storage_refusal_before_preview` both passed with nonzero discovery in that 15-case affected run.

The full inherited diagnostic progression was 33 failures/1 old ignored, then 2 passed/31 failed/1 ignored, then filtered iterations 21/12/1 and 29/7/1, before the final unfiltered 37/0/1. These were actual setup/authority failures, not skipped intended effects. Fault, generation, retention, protection and budget cases now reach their intended actual atomic operations through genuine preferred fixtures.

One process-control mistake limits the unfiltered run's timing evidence. The inherited Fjall suite was started while the earlier Bun guard was already active. Its test process was briefly SIGSTOPed and then immediately resumed on the parent's correction at 05:55:33Z. The exact pause start/duration was not captured; the immediately preceding owned-file write at 05:55:07Z bounds it below 26 seconds. Its printed 239.85s and timing history remain unchanged and include that pause and prior overlap. Functional outcomes are recorded, but no elapsed-performance conclusion is claimed. Later heavy suites were serialized, with no timeout changes. Parent Plan 14 supplies independent normal full-native verification.

## Pure Coverage Scope and Residuals

The saved ignored artifact `.local/open-bitcoin-dev/phase165-plan09-network-coverage.json` is **network-only**, not combined-package or whole-phase coverage. Its literal total is **10,093/10,121 lines (99.72335%, rounded)**. Eligibility is **380/380**, lib **18/18**, and the new selected-header projection at `header_store.rs:58` has executed counts throughout. Header-store aggregate is 317/320; its zero segments are old error/closure paths at 217, 238, 312 and 322, with three uncovered aggregate lines. No coverage exclusions were added.

| Existing report path under network/src | Covered/count | Deficit |
| -------------------------------------- | ------------- | ------- |
| compact_reconstruction.rs              | 329/330       | 1       |
| header_store.rs                        | 317/320       | 3       |
| inbound/permissions.rs                 | 364/367       | 3       |
| message/filter.rs                      | 171/173       | 2       |
| peer/message_dispatch.rs               | 374/381       | 7       |
| peer/policy_state.rs                   | 71/72         | 1       |
| peer/transaction_relay/orphanage.rs    | 414/418       | 4       |
| peer/transaction_relay/retry.rs        | 283/289       | 6       |
| peer/transaction_relay/scheduler.rs    | 446/447       | 1       |

These literal 28 residual lines remain visible for the parent's phase-wide coverage closure. They do not become a whole-crate 100% claim. Detailed pure controls cover reversed cached iteration, unknown stronger/equal, pending and later descendants, duplicate/copy/binding provenance, known FIFO preservation, epoch revocation, unknown-first masking, malformed ancestry, typed parameter/capacity refusals and selected request projection.

## Reviewed Guards and Artifact Checks

Current function digests were renewed only after reviewing actual source changes. The current boundary has 126 fixed Rust inputs and 410 reviewed edges. Historical tooling snapshots, corpora, adapters and receipts were preserved. Loaded-source mutations assert that their actual source anchors exist, so a no-op mutation cannot pass. Existing hash-order and unknown-first controls remain, with new cache provenance, authorization-copy, attachment, typed-fallback, prefix/requery and selected-proof/header-only/strict-seed controls.

An earlier full guard attempt had 616 passes/2 failures from stale mutation anchor spelling; actual current multiline anchors were repaired. Its next full 618/0 pass predates the later typed-fallback/request refinements and is historical evidence only. The final stable-source run passed **647 tests, zero failed, 841 expectations, 227.17s**. It ran without Cargo/Fjall overlap. The current boundary command independently passed with 126 fixed Rust inputs and 410 reviewed source edges.

Managed source checks pass with 1,521 scanned inputs, no findings and no exemptions. Breadcrumbs pass for 1,194 Rust files. All 39 approved paths exist, owned Rust/TypeScript paths satisfy the 628-line cap (largest runtime facade 627, failures 626, ownership 623), and `git diff --check` passes. No placeholders or unfinished authority paths were introduced; unknown order is an explicit bounded refusal policy. The delivered trust surfaces match the plan's six mitigated threats; no extra endpoint, authentication mechanism or schema was introduced.

## Deviations and Boundaries

Parent-approved ownership expansions followed observed failures and documented guard concerns: linkage/order separation; genuine same-live BASIC fixture attachment; fixture splits for the cap; actual typed fallback/prefix hazards; legacy header/body fixture separation; and selected-proof-only requests. These are correctness fixes and genuine setup migrations, not extra production opening capabilities. The phase158 preflight test body moved under its existing logical module with explicit cfg-test path registration; assertions were retained.

Simplification retained one existing selector/validator and one guarded mutation route. A private activation failure type and separate extension helper make error handling explicit without adding a public progress/error API or generic test callback. There are no managed mechanical exceptions, receipt setters, score inflation or freshening unrelated stale grants.

Phase 166 still owns canonical bootstrap, all-authority exact reopen acceptance and genesis BASIC; 167 owns contextual completeness; 168 owns actual pinned-daemon qualification. Four ordinary support/refusal classes and default-off networking remain unchanged. No canonical qualification receipt is claimed. Parent Plan 14 owns the full default verifier in `scripts/verify.sh` (Bash invocation) and phase-wide residual closure; affected focused evidence is this plan's completed contract.

## Task Commits

All task, RED/GREEN and metadata commits are explicitly deferred to the parent strict finalization after whole-phase native verification and independent checks. This executor staged, committed and pushed nothing and did not edit shared STATE, ROADMAP, REQUIREMENTS, config, hooks or todo files. No commit hashes are invented.

## Self-Check: PASSED

All 39 declared paths and this summary exist. All seven new/relocated Rust test artifacts have registered breadcrumb headers. Owned Rust/TypeScript files are at most 628 lines. The saved network-only coverage JSON was parsed and its 10,093/10,121 total and 380/380 eligibility claims verified. The current guard passed 647/647, with current CLI 126 inputs/410 edges; final workspace fmt, managed checks, 1,194-file breadcrumbs and diff whitespace checks passed. No stub-pattern occurrences were found in owned Rust/TypeScript paths. Frontmatter has exactly its two opening/closing delimiters and no body separator ambiguity. The index is empty; no deferred commit hash is fabricated. Full native phase finalization remains explicitly assigned to the parent.
