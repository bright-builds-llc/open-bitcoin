---
phase: 162-real-prune-retention-and-integrated-parity-proof
reviewed: 2026-10-11T19:50:31Z
depth: deep
review_scope: supplemental-plan14-header-checkpoint
diff_base: a77edab276a9daa086055aa61e6143ce86bc1218
files_reviewed: 10
files_reviewed_list:
  - packages/open-bitcoin-node/src/chainstate.rs
  - packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs
  - packages/open-bitcoin-node/src/chainstate/flush_lifecycle/tests/execute_flush.rs
  - packages/open-bitcoin-node/src/chainstate/flush_lifecycle/tests/execute_flush/header_checkpoint.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/prune_flush.rs
  - packages/open-bitcoin-node/src/network/validation_history.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/validation_history.rs
  - packages/open-bitcoin-node/src/sync/runtime_state/tests.rs
  - packages/open-bitcoin-node/src/sync/runtime_state/tests/header_checkpoint.rs
  - packages/open-bitcoin-node/src/sync/runtime_state/tests/header_checkpoint/faults.rs
findings:
  critical: 0
  warning: 0
  info: 0
  total: 0
status: clean
verification_status: executor-final-reruns-pending
generated_by: gsd-code-reviewer
lifecycle_mode: yolo
phase_lifecycle_id: 162-2026-10-11T16-25-37
generated_at: 2026-10-11T19:50:31Z
---

# Phase 162: Supplemental Checkpoint Code Review

**Reviewed:** 2026-10-11T19:50:31Z
**Depth:** deep
**Files reviewed:** 10, exactly Plan 14's explicit source scope
**Status:** clean for the source snapshot below; final executor verification remains pending

## Summary

No actionable correctness, security, or maintainability findings were identified in the scoped checkpoint correction. The change addresses the diagnosed missing durable publication of admitted stale headers through ordinary managed settlement. It retains the existing flush decision, private store-bound capability, validation provenance, ordered effects, and partial-failure accounting.

This is a supplemental review of Plan 14, not a complete Phase 162 review or proof of the full retained-filter client lifecycle. Plan 15's historical guard compatibility and unchanged full prune/reopen/socket scenario remain separate mandatory gates. No requirement activation, phase completion, native verification completion, or milestone shipment follows from this report.

## Context and method

Read the repository `AGENTS.md`, `AGENTS.bright-builds.md`, `standards-overrides.md`, `standards/index.md`, and relevant architecture, code-shape, testing, verification, local-guidance, and Rust standards. The overrides file supplies no active substantive exception. These sources informed the functional-core/adapter boundary, typed errors, early-return control flow, test meaning, scope discipline, and verification limits.

Resolved and fully read only the active lesson inputs: `/Users/peterryszkiewicz/.codex/tasks/lessons.md` (5,230 bytes) and `/Users/peterryszkiewicz/Repos/open-bitcoin/.codex/tasks/lessons.md` (1,958 bytes). Combined size was 7,188 bytes and conservative estimate 2,397 tokens, within both deterministic limits. No lesson archives were loaded. No project skill directories exist at `.claude/skills/` or `.agents/skills/`.

Read the installed reviewer instructions and code-review workflow, Phase 162 context, Plans 14 and 15, and the historical stale-header diagnosis. `162-14-SUMMARY.md` was not present during review. D-14 and the explicit parent assignment defer commits and reserve shared state/Git ownership to the root; the workflow's ordinary commit step was not executed.

Read all ten source files in full, reviewed tracked diffs against the supplied baseline, and included all three untracked test children using their actual contents. None of the ten paths is Git-ignored; no `.claudeignore` is present. Traced supporting callers and storage/recovery contracts without widening the reported changed-file scope.

The parent reported two verification-driven corrections after the initial read: explicit release of the selected-pressure test's returned coins-view handle, and renaming only the new fault variants to `SnapshotBeforeCommitFailure` / `SnapshotAfterCommitFailure`. Re-read the affected hunks and refreshed every source digest below. Existing `BeforeCommit` / `AfterCommit` admission variants and test names/assertions remain intact. Root must compare these digests against its final verified freeze; any later semantic edit requires renewed review.

## Exact source snapshot

Paths are relative to `/Users/peterryszkiewicz/Repos/open-bitcoin`. Digests are SHA-256 of actual file bytes at review closure.

| File | SHA-256 |
| --- | --- |
| `packages/open-bitcoin-node/src/chainstate.rs` | `61f9b1fb5d5f3e0643ffeaa12a93f2651cd2a82ee90a5e81694ed551d3ae4a7f` |
| `packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs` | `3b31f7b4b0489992e547079e549b808e56d1f945ffd8ae70e9a73bfd3fe89324` |
| `packages/open-bitcoin-node/src/chainstate/flush_lifecycle/tests/execute_flush.rs` | `b48c83efb66841ecf9dff4ce35fae6810045d83227f9275a025b6fb5693095db` |
| `packages/open-bitcoin-node/src/chainstate/flush_lifecycle/tests/execute_flush/header_checkpoint.rs` | `70ee8bad13671a574472ee93e0fde9d51fbd3a98acaa6de5dd9479c7a9a66eb6` |
| `packages/open-bitcoin-node/src/network/runtime_authority/prune_flush.rs` | `8deb3e561d9948a4a00c35032e7b9f9e6078ac9eddcca2747c761d564782b814` |
| `packages/open-bitcoin-node/src/network/validation_history.rs` | `9083b2b04f8198d6a8c76c3af2cd86177248036ecd7ec2c8faf3cd77c251cbf0` |
| `packages/open-bitcoin-node/src/storage/fjall_store/validation_history.rs` | `23b7cff93e0d0c1f38a73cdcb8211c8aa0e2c45b102c022f6d5fb44ae23ac471` |
| `packages/open-bitcoin-node/src/sync/runtime_state/tests.rs` | `e50301dd41811c8d9b2afcecc070eac32b07183a8f3ffe3622bcf40d45ba7f28` |
| `packages/open-bitcoin-node/src/sync/runtime_state/tests/header_checkpoint.rs` | `831e8741d8e4f7d1a746a85a5f359b9b18e00ff96a069d832a4a6517715185cb` |
| `packages/open-bitcoin-node/src/sync/runtime_state/tests/header_checkpoint/faults.rs` | `708606fc9c75ea37cfc92c16e886233dcb343b3c2d79c80acd202beee1361678` |

## Cross-file evidence

| Concern | Reviewed evidence and conclusion |
| --- | --- |
| Lazy selection and exact effect order | `chainstate/flush_lifecycle.rs:420-457` checks readiness, runs the existing `decide_flush`, and returns for ordinary empty-plan no-work/refusal before the callback. The sequence is existing body/undo/raw-header prefix, fallible checkpoint, paired unlink, coins, metadata. `network/runtime_authority/prune_flush.rs:43-59` performs sink lookup and header capture entirely inside that callback. |
| Public compatibility | `chainstate/flush_lifecycle.rs:338-400` preserves both public executor signatures and delegates with a no-op callback. `chainstate.rs:445-464` preserves the direct private chainstate wrapper's no-op behavior. The new methods and capability factory remain crate-private. |
| Serialization and lock order | `prune_flush.rs:31-60` uses disjoint chainstate/peer-manager field borrows under the existing authority. `runtime_authority.rs:540-550` and `runtime_authority/automatic_prune.rs:63-113` retain authority then automatic-state ownership. Snapshot persistence takes publication then history at `storage/fjall_store/validation_history.rs:120-121`; those guards return before unlink/coins. `PendingValidatedFlush` holds values and a store clone, not a live mutex guard. No recursive handle call or nested publication guard was introduced. |
| Trusted admitted knowledge | `network/validation_history.rs:24-51` retains private snapshot fields and store binding. `storage/fjall_store/validation_history.rs:115-139` checks the same store and every admitted hash/parent/height under complete coverage. `:141-143` atomically writes only the existing Headers and BlockIndex snapshots with SyncAll durability. It does not write validation status, BASIC progress, safe cursor, prune protection, or coins authority. |
| Header-only, empty, legacy, unsupported | `header_checkpoint.rs:75-133` exercises accepted successor plus PoW-only ahead-header retention and asserts `NeverConnected` before/after reopen. `:137-175` proves failed admission cannot reach the writer. `:179-239` preserves empty behavior and legacy coverage; `:243-258` exercises unsupported memory storage. The factory returns on empty entries before snapshot writing, preserving any saved snapshot. |
| Normal cold stale authorization | `header_checkpoint.rs:407-511` uses genuine local acceptance/reorg, ordinary Always settlement, complete runtime/authority closure, cold runtime open, and the unchanged public branch-capture API. It compares exact live/cold best/stale proof facts, ScriptsValid, timestamp age 100 seconds, and proof-equivalent age 600 seconds. No private `persist_progress`, raw snapshot injection, or peer-policy change supplies this success path. |
| Old missing knowledge | `header_checkpoint.rs:335-404` models an old missing snapshot after genuine settlement and proves surviving ScriptsValid alone cannot authorize a missing stale stop. Capture remains `UnknownOrDisallowedStop`, and no repair snapshot is synthesized. Supporting branch/policy source still requires stop existence, genuine stale provenance, exact proof/time ages, and unchanged range/resource ordering. |
| Actual snapshot failures | `storage/fjall_store/validation_history.rs:148-165` places distinct faults at the actual snapshot batch boundary. Before-commit consumes the fault without writing; after-commit writes the atomic snapshot, returns an error, and poisons history until reopen. Actual backend commit failure also poisons history. Errors preserve typed storage namespace/recovery action through the private durable callback. |
| Valid prefix and truthful retry | `header_checkpoint/faults.rs:30-102` compares old snapshots, coins B, active metadata, actual safe BASIC checkpoint, immutable stale row, protection, physical mates, and prune acknowledgement on before-commit failure; retry writes the four admitted entries and survives cold reopen. `:106-179` proves after-commit durable snapshot equality with unchanged coins/meta/BASIC/protection/mates, failed settlement, poisoned history, and retry only after closed reopen. `:227-254` asserts typed Headers backend failures independently of the public authority's existing string adapter. |
| Zero work on ordinary no-op | `header_checkpoint/faults.rs:183-223` observes all four actual stage counters unchanged through None, skipped IfNeeded/Periodic, and empty-plan disk refusal; the armed snapshot fault remains available for the next selected turn. `execute_flush/header_checkpoint.rs:217-233` additionally tests not-ready behavior. These tests detect attempted work rather than merely counting successful commits. |
| Existing partial failure and lineage | `chainstate.rs:476-541` retains pending-lineage preparation, successful wrote-coins completion/confirmation, abort handling, BASIC failure notification, forgotten undo only for reported deletes, and deleted-prefix propagation. `prune_flush.rs:61-69` evicts only reported hashes on both success and error. `execute_flush/header_checkpoint.rs:237-292` asserts typed prefix failure and exact later unlink/coins/metadata sequences with truthful deletion prefixes. |
| Explicit-plan compatibility | `execute_flush/header_checkpoint.rs:297-316` deliberately preserves the existing unusual nonempty-plan None/disk-refusal behavior: selected prefix and eligible unlink can precede no coins write/refusal. The correction does not invent a second flush-policy preview or silently change this established contract. Ordinary empty-plan no-op requirements remain distinct. |
| Caller coverage | Managed periodic/pressure/Always paths and automatic retention share `flush_coins` → `automatic_prune::flush` → `flush_and_evict_pruned_blocks`. Explicit manual prune uses that same helper from `flush_applying_prune_plan`. Daemon periodic and shutdown/error settlement reach it through `coins_flush.rs::flush_cycle`; authenticated manual pruning reaches it through RPC context/dispatch. `header_checkpoint.rs:20-71` labels its short manual/automatic scenarios as wiring controls because they have no legal deletion candidate. `:261-332` exercises selected pressure using a genuine durable handle. |

## Simplification pass

The existing flush selector remains the sole decision authority. One lazy private callback is sufficient to carry network-owned header knowledge into the selected durable prefix; the no-op wrappers preserve direct/public behavior without duplicating selection or introducing another lifecycle owner. The shared capability factory also keeps ordinary sync persistence and the new settlement caller on the same trusted writer.

Moving only test readiness constructors into the existing test descendant and splitting fault tests into a focused child keep production behavior in its existing module. No additional production extraction or abstraction is needed for this correction. All scoped files remain at or below the managed 628-physical-line threshold after the test split. The selected-pressure test explicitly drops the otherwise-unused returned view at `header_checkpoint.rs:277`, then drops the handle and store before cleanup. No unresolved simplification finding was identified.

## Verification and handoff limits

- Direct reviewer evidence: complete scoped source/diff analysis, supporting call-chain/lock/error/policy inspection, ignored-path checks, refreshed SHA-256 digests, and scoped `git diff --check` with exit 0. No source files, shared trackers, registries, branches, hooks, or Git history were changed by this reviewer.
- Parent-reported execution evidence: initial 16 focused tests and 30 Phase 159 tests passed; a later expanded 18-test checkpoint run passed in 10.14 seconds. These are attributed reports, not reviewer-executed test results. The cleanup and fault-name verification corrections were reviewed afterward; strict lint, final 18 tests, and broader serial regressions were running at report creation. Their final receipts and source-digest match belong to root acceptance.
- No Cargo, Bazel, full verifier, independent full-client corpus, or formal verification was run by this reviewer. The order spy establishes effect sequencing; concrete small Fjall tests establish snapshot/error/reopen state. Neither replaces Plan 15's full legal manual/automatic paired deletion and unchanged real stale cfilter/cfheaders/positive-checkpoint socket proof.
- Root retains breadcrumb registration, final source freeze, source/mutation guard compatibility, scoped lint/regression receipts, independent mitigation review, full native/formal gates, requirement/state decisions, and strict deferred Git finalization. The historical diagnosis remains evidence of the earlier failure boundary.

_Reviewer: gsd-code-reviewer; supplemental Plan 14 source review only._
