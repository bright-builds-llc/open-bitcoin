---
phase: 162-real-prune-retention-and-integrated-parity-proof
plan: "14"
subsystem: storage
tags: [rust, fjall, trusted-headers, selected-flush, stale-authorization, fault-injection]
requires:
  - phase: 162-03
    provides: Genuine configured reorg, paired retention and all-handle reopen scenario
provides:
  - Lazy private selected-prefix trusted-header checkpoint with unchanged public lifecycle APIs
  - Normal-path exact stale proof and ScriptsValid authorization through cold reopen
  - Actual atomic snapshot fault, retry, no-op and admitted-header controls
affects: [162-15, 162-04, 162-05, 162-12]
tech-stack:
  added: []
  patterns: [Selected fallible callback, disjoint authority-protected field borrows, store-bound snapshot capability]
key-files:
  created:
    - packages/open-bitcoin-node/src/chainstate/flush_lifecycle/tests/execute_flush/header_checkpoint.rs
    - packages/open-bitcoin-node/src/sync/runtime_state/tests/header_checkpoint.rs
    - packages/open-bitcoin-node/src/sync/runtime_state/tests/header_checkpoint/faults.rs
  modified:
    - packages/open-bitcoin-node/src/chainstate.rs
    - packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs
    - packages/open-bitcoin-node/src/chainstate/flush_lifecycle/tests/execute_flush.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/prune_flush.rs
    - packages/open-bitcoin-node/src/network/validation_history.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/validation_history.rs
    - packages/open-bitcoin-node/src/sync/runtime_state/tests.rs
key-decisions:
  - Capture, store lookup, admission validation and snapshot write occur only inside the selected prefix callback.
  - Direct chainstate and public lifecycle consumers retain default no-op callbacks and existing selection behavior.
  - Snapshot knowledge never upgrades validation provenance or independently grants BASIC safe progress.
  - Historical raw-source guard compatibility and unchanged integrated client rerun remain mandatory Plan15 gates.
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 162-2026-10-11T16-25-37
generated_at: 2026-10-11T19:55:44Z
duration: 23 min
completed: 2026-10-11
---

# Phase 162 Plan 14: Selected Trusted Header Checkpoint Summary

**Ordinary selected managed settlement now atomically saves admitted active, stale and header-ahead snapshots before paired unlink and coins, preserving genuine stale peer authorization through cold reopen.**

## Performance and Scope

- Tasks: 2/2 implementation tasks. Ten owned Rust paths, including three test-only children under the independently approved size amendment.
- First recorded execution clock: 2026-10-11T19:32:16Z. Final Cargo receipt ended at 19:52:32.343Z. Summary preparation clock: 19:52:59Z; final summary clock:19:55:44Z.
- Material guidance: repo AGENTS, Bright Builds sidecar and overrides, architecture/code-shape/testing/verification/Rust standards, execute-plan workflow and summary template, phase context/research/UI limits, tracked historical diagnosis and prior 01/13/02/03 handoffs.
- Both active lessons were fully loaded: 7,188 bytes and 2,397 conservative estimated tokens. Root confirmed the existing audit baseline has no trigger. No project skills were present.
- Root had already fetched origin and materialized pinned Knots before substantive work. No ref, staging, commit, branch or push operation was performed here.
- Root owns STATE/ROADMAP/REQUIREMENTS/todo, shared parity registries, generated LOC, independent reviews, full native verification, formal activation and Git finalization. Requirements remain pending.
- The scoped producer and new regressions pass. The broader flush suite is **70 passed / 1 failed**, with its exact raw-source-format assertion explicitly assigned by root to mandatory Plan15. This summary does not claim a fully green historical/native phase gate.

## Task 1: Lazy Selected Prefix and Existing Cleanup

The new crate-private lifecycle executor accepts a fallible `FnMut(&S) -> Result<(), StorageError>`. The existing readiness, policy decision and empty-plan/no-write predicates execute first. Body, undo and existing raw-header prefix persistence must succeed before the callback runs. Paired deletion, coins and metadata follow only after callback success.

Public `execute_flush` and `execute_flush_applying_plan` retain their signatures. The existing private direct chainstate wrapper delegates with a no-op callback; the network caller uses the private callback variant. Existing lineage preparation, successful completion, abort/cleanup, BASIC persistence-failure reporting, forgotten undo and truthful deleted-hash cache eviction remain in their existing owner.

Six ordered-effect controls use an actual sink and cache adapter to observe body → undo → raw headers → supplier → capture → validation → snapshot → paired unlink → coins → metadata. Readiness/no-work/None/not-due/disk refusal with no prefix leave all four supplier-stage counters zero. Prefix and callback errors propagate before any deletion or coins effect. Later unlink, coins and metadata failures report only completed deletion prefixes. Explicit nonempty-plan None and disk-refusal cases preserve their pre-existing unusual prefix/delete behavior.

The cfg(test) ready/not-ready constructors moved into the existing execute-flush test descendant. Production execution remains in its existing file; no broad production extraction or suppression was added.

## Task 2: Trusted Live Publication and Concrete Faults

Under the existing network authority, the common prune/coins helper destructures disjoint mutable chainstate and shared peer-manager field borrows. The closure receives the selected sink and obtains its optional supported history store only when invoked. It then clones nonempty live entries into the existing private store-bound capability and persists through the existing admission-checked writer.

Lock audit: authority → automatic-prune state when applicable → individual effect publication guard → history/payload. Pending flush evidence owns clones/proofs, not guards. No handle method reacquires authority. No publication/history guard spans the callback invocation; snapshot publication acquires and releases its own publication → history guards before unlink/coins. Test observation counter locks are brief and released before effects.

The snapshot batch writes Headers and BlockIndex together with SyncAll. Complete coverage still checks each hash/parent/height against admitted identity. Empty live knowledge and unsupported history stores skip publication. Legacy coverage stays UnknownLegacy; no filter/body/ledger-derived reconstruction, implicit repair or status upgrade was introduced.

Distinct test-only `SnapshotBeforeCommitFailure` and `SnapshotAfterCommitFailure` hooks sit at the actual snapshot batch commit boundary, separately from existing identity-admission BeforeCommit/AfterCommit hooks. The explicit Failure suffix avoids Clippy's common enum-postfix lint without suppression. Real batch failure also poisons history, matching uncertain publication handling.

### Semantic RED and GREEN

The historical diagnostic was converted into `phase162_header_checkpoint_reopen`. Before producer edits, its ordinary Always/reorg path failed at the required `normal snapshot` assertion: **0 passed / 1 failed**, exit101. No private persist_progress call remains in its accepted success path.

GREEN uses genuine normal maturity100 local acceptance, real shallow reorg, ordinary Always settlement and complete runtime/store/handle closure. Saved and cold stale entries survive. Cold stale and best exact proof facts equal their live values; timestamp age is100 seconds and proof-equivalent age600 seconds. ScriptsValid survives and public cold branch capture is Authorized under unchanged policy.

The final focused selector discovers and passes **18 tests**. Separate controls prove:

- Due Periodic, explicit manual empty-plan and automatic-mode short-chain dispatch reach the same selected checkpoint. These are honestly caller-wiring controls, not legal-threshold deletion evidence.
- Selected pressure uses real Fjall initialization with the existing memory-pressure input, genuine accepted genesis and normal handle IfNeeded settlement. The unused initialization view clone is explicitly dropped before cleanup.
- An admitted PoW-only header ahead survives with NeverConnected; a genuine successor after a saved snapshot is retained. No ScriptsValid or BASIC safe progress is synthesized from headers.
- A failed admitted-header publication cannot pass snapshot identity validation or reach its writer.
- Empty live capture does not consume the snapshot fault; the existing empty-coins refusal is preserved. Legacy snapshot knowledge remains intact through settlement and reopen.
- A modeled old store missing stale headers still returns UnknownOrDisallowedStop despite retained ScriptsValid. No backfill occurs.

### Snapshot Failure and Retry Evidence

The before/after ordering tests use the same small genuine history with an actual configured Enabled BASIC owner. They observe nonempty immutable stale records, actual safe checkpoint/protection, coins B, active metadata and physical body/undo mates.

Before-commit failure leaves both old snapshots, coins, metadata, protection, safe checkpoint, immutable stale row and mates unchanged. Supplier/capture/validation/writer attempts each increment once. The hook consumes itself, a normal retry writes four admitted entries to both snapshots, and closed reopen confirms their durability and ScriptsValid.

After-commit failure leaves both new snapshots durable but returns an error. Coins, metadata, safe checkpoint, immutable stale row, protection and mates remain unchanged. History publication is poisoned and another live settlement fails; complete closure/reopen derives actual durable state and permits a normal retry. No successful prune/coins receipt is returned for the failed turn.

Concrete no-op controls compare all four actual attempt counters with their baseline: None, IfNeeded below threshold, not-due Periodic and no-prefix disk refusal change none. The untouched before-commit hook fails the next selected flush, distinguishing zero attempts from merely zero successful writes. Typed direct writer controls preserve Headers BackendFailure with Restart recovery action; the durable callback carries StorageError unchanged up to the existing public handle adapter.

Earlier body/undo prefix effects are not rolled back. These changes do not strengthen the existing crash-window or whole-settlement atomicity promise.

## Verification and Receipts

All Cargo commands were serial, used pinned Bun1.3.9 PATH and Rust1.94.1, and ran through:

`bun run scripts/command-timings.ts run --key KEY -- cargo ...`

Node tests used `--manifest-path packages/Cargo.toml -p open-bitcoin-node --lib --all-features SELECTOR -- --test-threads=1`; the network rows substitute `-p open-bitcoin-network`.

| Timing key suffix after phase162-header-checkpoint- | Selector / command | Actual result | Wrapper ms |
| --- | --- | --- | --- |
| reopen-red | phase162_header_checkpoint_reopen | Semantic RED: 0 passed,1 failed | 7,919 |
| order-red | phase162_header_checkpoint_order | Compile RED: missing private executor plus test import correction | 3,630 |
| reopen-faults | phase162_header_checkpoint | Final18 passed,0 failed;10.64s test runtime | 19,912 |
| clippy | node + rpc, all-targets/all-features, -D warnings | Passed | 9,870 |
| history-regression | phase159_validation_history | 30 passed,0 failed | 25,866 |
| phase158-regression | phase158 | 102 passed,0 failed,1 intentional ignored measurement | 307,327 |
| flush-regression | flush_lifecycle | 70 passed,1 raw-source guard failure; mandatory15 | 32,759 |
| branch-regression | phase160_branch | 4 passed,0 failed | 2,417 |
| query-regression | phase159_basic_query | 10 passed,0 failed | 49,014 |
| range-regression | network phase160_range_policy | 9 passed,0 failed | 495 |
| stale-regression | network phase160_coverage_stale_stop | 1 passed,0 failed | 120 |

Exact semantic RED receipt: `.local/open-bitcoin-dev/command-timings/phase162-header-checkpoint-reopen-red/2026-10-11T19-33-26.780Z-2f593c8d-1ac1-44b2-b5cc-e7e4ac539643.json`.

Exact final GREEN receipt: `.local/open-bitcoin-dev/command-timings/phase162-header-checkpoint-reopen-faults/2026-10-11T19-50-02.693Z-c328a1a0-86b6-422e-9c87-1258ca75987b.json`.

Exact historical guard failure receipt: `.local/open-bitcoin-dev/command-timings/phase162-header-checkpoint-flush-regression/2026-10-11T19-50-22.639Z-d9f72194-ca91-4524-b1b1-df47191a1013.json`.

Additional executed checks:

- All ten owned paths: `rustfmt --edition 2024 --config skip_children=true --check` passed.
- `bun scripts/check-parity-breadcrumbs.ts --check`:1,133 Rust files verified after root registration.
- `bun scripts/bright-builds-check.ts all`:0 findings. Owned file lengths:614,628,575,316,205,166,479,176,617,259 respectively in Plan14 files_modified order.
- `bun scripts/check-phase159-filter-rpcs.ts`:125 named behavioral selectors and scoped producer boundaries passed.
- `bun scripts/check-phase160-peer-filter-serving.ts`:38 production edges,46 executable selectors and scoped claims passed.
- `git diff --check` passed; owned hunks reviewed for selection, authority, typed errors, cleanup and claim scope.

Long Phase158 execution was polled throughout; at3m45s its test process was running at93.8%CPU. It completed normally in307.19s with no interruption. A tool yield or quiet interval was not treated as a timeout.

## Deviations, Initial Failures and Simplification

1. The independently approved size amendment adds exactly one test-only faults.rs child, preserving the two tasks and all names/assertions. The formatted single runtime child reached719lines; extraction leaves617/259 without suppressions. Private named helpers keep new functions below the managed refactor trigger.
2. Initial test extraction corrected a moved SyncRuntimeConfig path. Seam compile RED exposed the expected absent executor and a missing test import. Moving cfg(test) helpers initially included one outer delimiter/doc line, causing a syntax failure observed by two already queued runs before one correction; a moved CoinsRecoveryOutcome import was also corrected. These were test/plumbing construction errors, not successful semantic evidence.
3. First concrete fixture run passed12/14 and exposed two incorrect empty-coins-success assumptions. Genuine acceptance was added before successful coins settlement; the empty control now explicitly preserves the existing refusal.
4. The old-missing-header control could not call storage-private remove_bytes from sync tests. A narrow cfg(test) owned-storage helper removes only the two header snapshots for that historical absence control; it grants no production authority.
5. Strict lint caught the four fault variants' common Commit postfix. Only the two new test-only variants received Failure suffixes; all existing admission-hook names and semantics remain.
6. The simplification/cleanup audit explicitly dropped the pressure initializer's unused view clone. The final18-test run and strict lint include this correction.
7. Historical `persist_calls_execute_flush_ifneeded` at recovery_and_persist.rs:128 checks the raw contiguous string `self.flush_lifecycle.execute_flush`. Rustfmt wraps the longer correct private executor call, causing the sole broad flush failure. Root explicitly assigned this exact existing source-guard path and full flush-lifecycle rerun to mandatory Plan15. No fake anchor, assertion weakening, historical checker edit or source-format workaround was applied here.

Three owned datadirs from the initial failed tests were removed after their test processes exited. All successful new fixtures close their owned store/runtime/handle references before cleanup. A final inventory found zero remaining Phase162 checkpoint temporary datadirs.

## Task Commits

Deferred by D-14 and parent instruction. No staging, commits, branches, hook bypass or pushes were performed. Parent retains atomic Git/final metadata responsibility after the mandatory clean gate.

## Registry and Exact Source Handoff

Root registered these actual new paths:

| New Rust path | Exact ordered pinned anchors under packages/bitcoin-knots/src/ |
| --- | --- |
| packages/open-bitcoin-node/src/chainstate/flush_lifecycle/tests/execute_flush/header_checkpoint.rs | node/chainstate.cpp; node/blockstorage.cpp; validation.cpp |
| packages/open-bitcoin-node/src/sync/runtime_state/tests/header_checkpoint.rs | chain.h; validation.cpp; headerssync.cpp |
| packages/open-bitcoin-node/src/sync/runtime_state/tests/header_checkpoint/faults.rs | chain.h; validation.cpp; headerssync.cpp |

Final source SHA256 observations:

| Owned file | SHA256 |
| --- | --- |
| chainstate.rs | 61f9b1fb5d5f3e0643ffeaa12a93f2651cd2a82ee90a5e81694ed551d3ae4a7f |
| chainstate/flush_lifecycle.rs | 3b31f7b4b0489992e547079e549b808e56d1f945ffd8ae70e9a73bfd3fe89324 |
| chainstate/flush_lifecycle/tests/execute_flush.rs | b48c83efb66841ecf9dff4ce35fae6810045d83227f9275a025b6fb5693095db |
| chainstate/flush_lifecycle/tests/execute_flush/header_checkpoint.rs | 70ee8bad13671a574472ee93e0fde9d51fbd3a98acaa6de5dd9479c7a9a66eb6 |
| network/runtime_authority/prune_flush.rs | 8deb3e561d9948a4a00c35032e7b9f9e6078ac9eddcca2747c761d564782b814 |
| network/validation_history.rs | 9083b2b04f8198d6a8c76c3af2cd86177248036ecd7ec2c8faf3cd77c251cbf0 |
| storage/fjall_store/validation_history.rs | 23b7cff93e0d0c1f38a73cdcb8211c8aa0e2c45b102c022f6d5fb44ae23ac471 |
| sync/runtime_state/tests.rs | e50301dd41811c8d9b2afcecc070eac32b07183a8f3ffe3622bcf40d45ba7f28 |
| sync/runtime_state/tests/header_checkpoint.rs | 831e8741d8e4f7d1a746a85a5f359b9b18e00ff96a069d832a4a6517715185cb |
| sync/runtime_state/tests/header_checkpoint/faults.rs | 708606fc9c75ea37cfc92c16e886233dcb343b3c2d79c80acd202beee1361678 |

Paths in this hash table are relative to `packages/open-bitcoin-node/src/`.

## Known Stubs and Threat Surface

No production stubs or unwired data sources were introduced. Empty mock collections belong to explicit negative controls. No new production endpoint, auth path, schema, dependency or trust authority was added; existing store-bound snapshot/file access is reused.

## Mandatory Plan15 and Parent Readiness

Plan15 must update the two Phase158 historical source/mutation guards and the additional existing Rust source-format guard, rerun the entire flush-lifecycle selector, and run the unchanged integrated configured client lifecycle on its original full pressure corpus before Plan04 resumes. The active/bodyless/stale RPC and actual cold stale socket assertions, independent expectations and all excluded surfaces remain frozen.

No full integrated client rerun, full default native verifier, formal requirement activation, independent review approval, Git finalization or milestone shipment is claimed here. Parent owns those gates. Old stores without stale header knowledge remain fail closed; there is no retroactive recovery claim.

Cargo is idle. No Plan15 or Plan04 execution was started by this executor.

## Self-Check: PASSED

All ten owned Rust paths exist and match the recorded SHA256 observations; all three created children are registered and covered by the passing breadcrumb check. Semantic RED and final GREEN timing receipts exist with expected101/0 statuses. Lifecycle identity, empty requirements-completed and exactly two top frontmatter delimiters were verified. Source whitespace and scoped formatting checks passed. There are no task commits to check because D-14 explicitly defers them to root. All checkpoint temporary datadirs are absent and the serial Cargo session completed. The single historical raw-source guard failure remains an explicit mandatory Plan15 gate.
