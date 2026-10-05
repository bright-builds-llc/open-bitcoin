---
phase: 156-index-owned-manual-and-automatic-prune-coordination
plan: "05"
subsystem: storage
tags: [basic-filter, prune-protection, manual-prune, fjall, ancestry, receipts]
requires:
  - phase: 156-04
    provides: Reserved operator refusal and exact guarded protection-map preservation
provides:
  - Guarded current coins/ancestry and owned protection checks before intent or paired deletion
  - Fresh immutable protection snapshots for both concrete flush sinks
  - Real manual application, clone-race, recovery and receipt evidence
affects: [156-06, 156-07, 156-08, 157, 158]
tech-stack:
  added: []
  patterns: [publication-before-payload guards, current durable ancestry proof, explicit unsupported snapshots]
key-files:
  created:
    - packages/open-bitcoin-node/src/chainstate/flush_lifecycle/fjall_sink.rs
    - packages/open-bitcoin-node/src/chainstate/flush_lifecycle/prune_apply/tests.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/tests/prune_unlink/protection.rs
    - packages/open-bitcoin-node/src/sync/tests/filter_index/prune_coordination.rs
  modified:
    - packages/open-bitcoin-node/src/storage/fjall_store/prune.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/tests/prune_unlink.rs
    - packages/open-bitcoin-node/src/chainstate.rs
    - packages/open-bitcoin-node/src/chainstate/fjall_store.rs
    - packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs
    - packages/open-bitcoin-node/src/chainstate/flush_lifecycle/prune_apply.rs
    - packages/open-bitcoin-node/src/chainstate/flush_lifecycle/tests/execute_flush.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/automatic_prune/tests/fixtures.rs
    - packages/open-bitcoin-node/src/sync/tests/filter_index.rs
    - packages/open-bitcoin-node/src/sync/tests/filter_index/startup.rs
    - packages/open-bitcoin-node/src/sync/tests/filter_index/startup/recovery.rs
    - packages/open-bitcoin-node/src/sync/tests/filter_index/faults.rs
    - packages/open-bitcoin-node/src/sync/tests/filter_index/recovery.rs
    - docs/parity/source-breadcrumbs.json
key-decisions:
  - Effective owned protection requires current recovered coins and matching saved fence/checkpoint ancestry before any prune effect.
  - Genuine absent intent may return without effects after bounded owner validation; clearing an intent requires the full applicable proof.
  - Snapshot equality includes validated ownership and current locks; snapshots never authorize concrete deletion.
patterns-established:
  - Only known transient sinks explicitly declare no index; unsupported generic sinks refuse.
  - Deliberately unsafe persisted recovery fixtures use labeled test-only raw seeds.
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 156-2026-10-04T20-28-36
generated_at: 2026-10-05T00:02:07Z
commits: []
git_finalization: pending consolidated root commit after clean whole-phase verification and full native gate
duration: 63min from first recorded TDD command
completed: 2026-10-05
---

# Phase 156 Plan 05: Fresh Prune Application Gates Summary

**Shared guarded deletion binds owned release to current recovered coins and durable ancestry, with fresh manual protection and real retained-payload/receipt proof.**

## Performance

- First recorded TDD command: 2026-10-04T22:59:54.754Z.
- Final diagnostic test completed: 2026-10-04T23:55:03.185Z.
- Implementation tasks: 3/3; task finalization remains pending root.
- Source/test/mapping paths changed: 18; this summary and ignored diagnostic evidence are separate.
- Both active lessons read completely: 5,230 global bytes plus 1,958 repository bytes, 2,397 conservative estimated tokens.

## Accomplishments

- Direct intent and paired deletion acquire the existing shared publication guard, validate fresh bounded ownership, current locks and applicable current coins/ancestry proof, then retain the guard through intent and deletion. Refusals precede payload revision invalidation. Private guarded writers avoid recursive public-lock acquisition.
- With effective owned protection, coins heads must be recovered, B must match a complete valid durable active ancestry, and the saved fence plus nonempty checkpoint endpoint must match that ancestry. Candidate height/hash must match it too. Forged low heights, unknown hashes, old branches, ahead fences, malformed/missing/weak ownership and poisoned publication refuse before effects. Disabled with retained protection uses the same gate.
- Resume validates fresh ownership before an absent-intent no-op. Any clearing or deletion additionally requires current applicable proof, current locks union supplied restrictions, active hash, keep-window and coins controls. The owned resume path reuses its already verified ancestry. It preserves finish-or-Repair behavior and actual receipt accounting.
- Added public immutable `chainstate::PruneProtectionSnapshot` with `maybe_owner()`, `locks()`, `protects_height()` and equality. The unsupported trait default returns `UnavailableNamespace`; concrete sinks forward validated current facts. Known Memory/OrderingSink/TestStore fixtures explicitly declare no index, and TestStore forwards a real snapshot when its real-store backing is present.
- Manual application reloads per candidate, adds caller restrictions, and classifies direct required inputs including zero/one as skips. The final concrete gate remains authoritative after a clone changes authority. Existing cleanup remains earned only by `DeletedLiveMate`.
- Migrated deliberately unsafe recovery intent/protection seeds to labeled cfg(test) raw helpers. Ordinary no-index initialize fixtures retain production seeding. No production raw bypass, new authority, dependency, crate, public activation or scheduler was added.

## Task Finalization Records

The strict wrapper defers all task/TDD/metadata commits. No commit, push or hook bypass occurred.

1. Task 1: concrete intent/delete/resume guards and current ancestry binding; final 39/39 verification passed after the root-owned syntax correction.
1. Task 2: immutable snapshot, fresh manual application and production reopen proof; 96/96 filter-index tests passed.
1. Task 3: classification/transient/receipt controls, coherent test extraction and fixture migration; 52/52 flush tests and 10/10 real authority controls passed. The extra default-parallel automatic control run remains an **open required root gate** below.

STATE, ROADMAP, REQUIREMENTS and config remain root-owned. CFPR-01 is not activated by this summary.

## Verification

Pinned Bun 1.3.9 was selected and reprobed from `/tmp/open-bitcoin-bun-1.3.9/bun-darwin-aarch64`. Every Cargo command was timing-wrapped and serialized. No Bazel command was run by this plan.

| Check                                      | Exact command                                                                                                                                                                                                                           | Evidence                                                                                                                                                                              |
| ------------------------------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Concrete RED                               | `bun run scripts/command-timings.ts run --key phase156-concrete-delete-red -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib storage::fjall_store::tests::prune`                                             | Expected exit 101: 26 passed, two new guard tests failed because required deletion returned DeletedLiveMate and standalone intent returned Ok.                                        |
| Snapshot RED                               | `bun run scripts/command-timings.ts run --key phase156-manual-prune-coordination-red -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib filter_index`                                                         | Expected compile failure: missing load_prune_protection on both concrete sinks.                                                                                                       |
| Classification RED                         | `bun run scripts/command-timings.ts run --key phase156-flush-regressions-red -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib flush_lifecycle`                                                              | Expected compile failure: missing direct-protection classifier.                                                                                                                       |
| Concrete final GREEN                       | `bun run scripts/command-timings.ts run --key phase156-concrete-delete -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib storage::fjall_store::tests::prune`                                                 | 39/39 passed, 1,068 filtered, 21.56s test body.                                                                                                                                       |
| Application/recovery GREEN                 | `bun run scripts/command-timings.ts run --key phase156-manual-prune-coordination -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib filter_index`                                                             | 96/96 passed, 1,011 filtered, 54.88s test body. All five new coordination cases pass.                                                                                                 |
| Flush/lifecycle GREEN                      | `bun run scripts/command-timings.ts run --key phase156-flush-regressions -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib flush_lifecycle`                                                                  | 52/52 passed, 1,055 filtered, 13.07s test body.                                                                                                                                       |
| Existing actual receipt/cache controls     | `bun run scripts/command-timings.ts run --key phase156-prune-receipt-controls -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib network::runtime_authority::tests`                                           | 10/10 passed, 1,097 filtered, 4.52s; includes error after unlink and retry/eviction.                                                                                                  |
| Extra automatic controls, default parallel | `bun run scripts/command-timings.ts run --key phase156-no-index-automatic-controls -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib network::runtime_authority::automatic_prune::tests`                     | Failed/aborted before a final count: pre-existing writer fixture open returned ENOENT, then cleanup panicked and SIGABRT terminated the process. No default-parallel pass is claimed. |
| Automatic diagnostic, serial               | `bun run scripts/command-timings.ts run --key phase156-no-index-automatic-controls -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib network::runtime_authority::automatic_prune::tests -- --test-threads=1` | 25/25 passed, 1,082 filtered, 4.84s; diagnostic only, not a substitute for default parallel.                                                                                          |
| Source/claim guard                         | `bun run scripts/check-phase155-filter-index.ts`                                                                                                                                                                                        | Passed; not substituted for durable behavior.                                                                                                                                         |
| Source provenance                          | `bun run scripts/check-parity-breadcrumbs.ts --check`                                                                                                                                                                                   | Passed for 962 Rust files; four new paths registered by exact git add -N inventory.                                                                                                   |
| Managed checks                             | `bun run scripts/bright-builds-check.ts all`                                                                                                                                                                                            | Passed, 1,281 source files and active repository lesson structure, zero findings.                                                                                                     |
| Formatting/diff                            | Scoped `rustfmt --check --edition 2024 --config skip_children=true` on all 17 owned Rust paths; `git diff --check`                                                                                                                      | Passed; unrelated module children preserved.                                                                                                                                          |

Final source was unchanged after the last GREEN sweeps. The root full native verifier, configured coverage, Bazel/provenance, lifecycle validation, final Git hook remain mandatory; default-parallel regression closure is recorded below.

## Persistence and Race Observations

Real Fjall direct and intent refusals compare complete index snapshots, actual payload bytes/presence, unchanged payload revision, retained intent, have-pruned and support counts; actual close/reopen preserves them. Active and Disabled-retained forged-height attempts refuse. Current authority defects cover nonempty heads, missing B, wrong B and corrupt metadata. The deliberately malformed H+B fixture is rejected by normal store open itself, so its durable unchanged index and undo are inspected through an actual raw Fjall reopen instead.

A channel-ordered observation inside the actual destructive path proves the publication mutex remains held after the protection check while a clone attempts map publication; paired effects and clone publication complete in order. Another clone applies an old candidate only after durable authority changes and refuses. A third pauses actual application after its snapshot: a clone lowers durable authority, final concrete deletion refuses with no accounting/intent/receipt effect, and production reopen genuinely reconciles the saved cursor.

The production manual owner first retains genesis, height-one undo and a higher required pair after a plan predates protection. Record-only rows do not release them. Real reopen retains the same inputs; genuine fenced checkpoint publication then permits only the eligible zero/one prefix, earns exactly two height receipts in one successful batch, and leaves required higher input plus all immutable filter rows intact after production reopen.

These are explicitly sparse deletion-order fixtures with valid historical coinbase inputs and dense metadata. Existing validated-spend, fork, partial-coins replay and publication faults remain in the 96-test suite. This plan does not claim consensus synchronization, public activation, complete client-after-prune behavior or hardware power-loss proof.

## Decisions and Simplification Review

Repo-local AGENTS guidance, Bright Builds sidecar, placeholder-only overrides and architecture/code-shape/testing/verification/Rust standards informed this work. One pure ownership model and existing verified fence are reused. One publication guard precedes payload accounting, and one paired-delete owner performs effects. Complete filter forest scans remain at startup/publication, not per prune candidate. Current durable metadata ancestry proof is intentionally required before applicable effects; ordinary metadata/coins transitions remain serialized by managed authority. No broader raw-writer locking or reorg orchestration was added.

The saved-fence requirement conservatively retains history even when an old fence has become incompatible but part of the checkpoint remains common. It refuses until genuine reconciliation/publication; it does not silently rewrite proof. No-index and Disabled-released deletion retain their legacy contract.

The existing generic AlreadyAbsent advisory and partial-batch support-summary undercount remain unchanged. Actual deletion callbacks preserve earned cleanup across later errors; absent/no-op candidates never earn it.

## Threat Mitigations

| Threat   | Evidence                                                                                                                                                                                            |
| -------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| T-156-15 | Guard retained from fresh proof through intent/deletion; direct 0/1, forged/unknown pairs, old-branch/ahead authority, current coins faults, same-branch extension and held-guard clone tests pass. |
| T-156-16 | Per-candidate snapshots plus final concrete proof; preplanned production manual application, stronger resume protection and snapshot-to-delete clone change pass with retained bytes/receipts.      |
| T-156-17 | Missing/weak/malformed owner/state, poison and current authority defects refuse before candidate effects, including absent mates; immutable rows/intents survive.                                   |
| T-156-18 | 52 flush tests and 10 actual authority controls preserve live-receipt cleanup and later-error behavior; unsupported and AlreadyAbsent controls earn no cleanup.                                     |

No new endpoint, authentication path, schema or filesystem trust boundary was introduced. No unresolved HIGH production finding was identified in the owned diff. Whole-phase review remains root-owned, and the parallel verification gate remains open.

## Deviations and Issues Encountered

- Root approved and updated the plan for coherent direct-test/sink children, the public type re-export, explicit no-index fixtures and narrow unsafe recovery seed migrations. Other agents' 02/03/04 changes were preserved.
- An inherited source-string test mistook pure checkpoint prefix() for a database prefix scan; the assertion now targets actual database scanning and rejects a full filter-record scan.
- The new direct fixture initially reused an empty-transaction storage block; historical-input validation correctly rejected it. The fixture now uses merkle-consistent coinbase bodies. One intermediate whole-node compile also caught missing FlushMode/FlushPolicyTime imports in coordination tests; corrected before GREEN.
- The malformed H+B observation initially assumed normal store reopen would succeed. Its existing coins integrity gate correctly refused, so the test now asserts Repair and inspects retained bytes via raw reopen. After the executor's three-correction limit, root applied the isolated two function-reference corrections required by Fjall's keyspace API; that assistance is not omitted from this record.
- The 18m24s compile wait was independently sampled by root at `rustc_metadata::dlsym_proc_macros` → dyld dlopen/mapSegments(CodeSignatureInFile) → fcntl. It was a host loader/code-validation boundary, not a test-body deadlock or directory scan. The timed advisory bundle and root sample remain in ignored local diagnostics. Commands stayed alive and were polled; no primary process, cache or host setting was changed.

## Required Root Parallel Gate — Closed

The extra default-parallel automatic suite aborted in existing `automatic_prune/tests/writers.rs`: real_fixture's Fjall open at line 41 returned Runtime ENOENT before constructing TestStore; its TempStore cleanup at line 30 then panicked. Another overlapping-writer case reported FAILED but its underlying assertion was not printed before abort. Serial 25/25 success distinguishes this observation but does not close it or justify accepting completion debt. writers.rs was not edited by this executor. Root subsequently reproduced and diagnosed timestamp-only directory collisions: a losing open removed the shared sibling path. GSD debug added atomic directory reservation with suffix retry in the existing test allocator, without production/dependency changes. The equal-clock concurrent real-store regression reproduced SIGABRT before the fix and passed afterward. Twenty default-parallel writer repetitions passed 5/5 each; three full default-parallel automatic-control repetitions passed 26/26 each. This mandatory parallel gate is now closed; the full native phase gate remains pending.

The bounded transcript extract, run ids and actual surviving path are preserved at `.local/open-bitcoin-dev/phase156/diagnostics/phase156-05-parallel-automatic-controls-failure.txt`. It is explicitly labeled an extract; the surviving directory is not claimed to be the missing path that Fjall failed to emit. No further Cargo session remains running.

## Known Stubs

None. The scoped stub scan found no introduced production TODO/FIXME, placeholder or unimplemented body. Unsupported sinks intentionally refuse; known no-index fixtures are explicit. Automatic cache coordination, broader lifecycle closure and final parity/docs remain later plans.

## Next Plan Readiness

Plan 06 may consume immutable snapshot equality, locks and direct protects_height before measurement/reuse/throttle decisions. This plan does not implement that work. Requirement activation, all shared tracking and finalization remain root-owned, the parallel fixture gate is closed by root debug evidence below.

## Self-Check: PASSED

All 18 declared source/test/mapping paths and this summary exist. The report distinguishes passed required suites, passed serial diagnostics and the initially failed and subsequently root-closed default-parallel gate; it does not assert full native success. Source provenance, managed checks, scoped Rust formatting and whitespace checks passed. No commit-existence claim applies: commits remain empty and root finalization is pending. The summary was checked before authorized scoped Markdown formatting with installed GFM/frontmatter extensions; its final check passed.
