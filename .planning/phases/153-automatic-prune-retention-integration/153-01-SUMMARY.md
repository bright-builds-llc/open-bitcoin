---
phase: 153-automatic-prune-retention-integration
plan: "01"
subsystem: database
tags: [rust, fjall, pruning, accounting, concurrency]
requires:
  - phase: 148-fjall-payload-unlink-and-have-pruned
    provides: Durable paired payload unlink and recovery
provides:
  - Exact logical retained block and encoded undo bytes from one guarded snapshot
  - Clone-shared mutation serialization and conservative accounting revisions
  - Production flush-sink accounting and durable prune-lock forwarding
affects: [153-02, 153-03, 153-04]
tech-stack:
  added: []
  patterns: [single-snapshot size accounting, clone-shared RAII payload mutex]
key-files:
  created:
    - packages/open-bitcoin-node/src/storage/fjall_store/payload_usage.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/payload_usage/tests.rs
  modified:
    - packages/open-bitcoin-node/src/storage/fjall_store.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/prune.rs
    - packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs
    - packages/open-bitcoin-node/src/chainstate/fjall_store.rs
    - docs/parity/source-breadcrumbs.json
key-decisions:
  - Measurement captures facts and their revision under the same mutex used by full payload mutation attempts.
  - New opens and attempted payload changes remain Unmeasured until accounting succeeds; overflow is permanently nonreusable.
  - Distinct open-store identities prevent comparing reopened facts with a prior open's measured revision.
patterns-established:
  - Generic raw payload writes and paired batches acquire once; compound writes never recursively acquire the payload mutex.
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 153-2026-10-03T04-01-54
generated_at: 2026-10-03T04:58:34Z
duration: 25 min
completed: 2026-10-03
---

# Phase 153 Plan 01: Retained Payload Accounting Summary

**Exact Fjall payload-value accounting and clone-shared guarded mutations supply safe facts and revisions to the production flush sink.**

## Performance

- Implementation started: 2026-10-03T04:33:23Z (first source creation; context loading preceded this).
- Completed: 2026-10-03T04:58:34Z.
- Tasks: 3/3.
- Files changed: seven assigned source/manifest files plus this summary.

## Accomplishments

- One guarded `Database::snapshot()` sums complete `block:` and `undo:` prefixes through `Guard::size`, then uses the same snapshot's `size_of` for supplied active height/hash pairs. Neither historical payload decoding nor leftover snapshots supply byte facts.
- Nonactive and recent payloads count in total. Candidates include either present mate, including a zero-length value; both-absent pairs are omitted. Metadata, coins and support records are excluded. Checked sums and backend/poison errors refuse explicitly.
- `FlushPersistSink` requests accounting, revisions and durable lock read/write capabilities. Both Fjall adapters forward to the existing store; unsupported defaults return `UnavailableNamespace`.
- The shared `Arc<Mutex<PayloadUsageState>>` covers invalidation, live mutation and persistence outcomes. Accounting holds it from before snapshot creation through fact/revision capture. Errors cannot publish old facts at a completed new revision; unwind poisons and visibly refuses further accounting and payload mutations.

## Pending Atomic Finalization Records

D-10 overrides ordinary per-task commits. Nothing was committed, pushed or staged by this executor. Root owns phase verification, requirement completion and finalization; `requirements-completed` remains empty.

| Task | Intended record | Files/responsibility | Status |
| --- | --- | --- | --- |
| 1 | `feat(153-01): define durable retention accounting contracts` | Store module/state, sink capabilities and forwarding, breadcrumb manifest | Pending finalization |
| 2 | `feat(153-01): measure retained payload values from one snapshot` | Accounting module and actual-value/half-pair/nonactive/error tests | Pending finalization |
| 3 | `fix(153-01): serialize payload mutations with accounting facts` | Generic raw mutation gateway, paired batch guard and concurrency/invalidation tests | Pending finalization |

Base HEAD remains `6dd6f18c9b16158b16391fcd288aadc94728cfcd`. Root added intent-to-add entries for the two new Rust paths so the tracked-file breadcrumb checker could inspect them; the content staging diff remained empty.

## Verification and Local RED/GREEN Evidence

All Cargo commands used `bun run scripts/command-timings.ts run --key <key> -- <command>` and ran sequentially against the shared target directory.

| Key/check | Command or evidence | Result |
| --- | --- | --- |
| `phase153-payload-usage-red` | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node payload_usage -- --test-threads=1` against refusing accounting scaffold | Meaningful RED: 0 passed, 3 failed on unsupported accounting; runtime 2.03s |
| `phase153-payload-usage` | Same filter after snapshot accounting, before production writer wiring | Accounting GREEN and writer RED: 11 passed, 4 failed on clone replacement, overflow, raw/paired invalidation and poison mutation refusal |
| `phase153-payload-invalidation` | Same filter after writer wiring | 15/15 passed, runtime 8.19s |
| `phase153-storage-regression` | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node storage::fjall_store -- --test-threads=1` | 96/96 passed, runtime 226.34s; includes all 16 new tests plus existing migration, paired unlink, recovery and snapshot tests |
| `phase153-payload-invalidation` | Accounting filter after final raw-gateway simplification | 16/16 passed, runtime 9.31s |
| `phase153-account-contract` | `cargo check --manifest-path packages/Cargo.toml -p open-bitcoin-node` | Passed after contracts/accounting and after final production refactor |
| `phase153-node-clippy` | `cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-targets --all-features -- -D warnings` | Passed after replacing seven temporary slice clones with `std::slice::from_ref` |
| `phase153-node-format` | `cargo fmt --manifest-path packages/Cargo.toml --all` | Passed |
| Breadcrumbs | `bun scripts/check-parity-breadcrumbs.ts` | Passed for 913 Rust files |
| File lengths | `bun scripts/bright-builds-check.ts file-lengths` | Passed: 1,224 files, zero findings |
| Diff | `git diff --check` and scoped source diff review | Passed; edits confined to assigned files |

The final clone-to-slice changes affect test setup syntax only; strict Clippy compiled those tests. Full native verification, build, coverage and Bazel remain Plan 04/root gates.

## Writer-Family and Guard Audit

| Family | Coverage |
| --- | --- |
| `save_block`, `save_undo` | Encoding precedes the private `put_bytes` gateway; payload-key insert plus persistence stays under one guard. Typed-writer invalidation regression passes. |
| Migration and reopen seeding | `finish_schema_one_undo_and_chain_meta` and `seed_coins_from_snapshot` route each undo through `save_undo`. Direct coins writes affect only the coins keyspace. Real migration/seeding and existing schema migration tests pass. |
| Raw insert/remove and fixture seeding | Private `put_bytes`/`remove_bytes` acquire for BlockIndex `block:` and Chainstate `undo:` only. Equal-sized replacement, removal and overflow regressions pass. |
| Paired unlink | `commit_paired_delete` acquires once around presence probes, intent persistence, paired tombstone batch and verification. Fixed intent/summary writes do not reacquire. Existing paired-unlink and reopen regressions pass. |
| Other direct batches | Header/block-index snapshots, coins batches, prune lock/summary and intent cleanup mutate nonpayload keys only. The sole `database()` production consumer is the coins-view adapter. |

Deterministic channels pause a clone writer after guard acquisition/invalidation and before live insertion. Both measurement and revision reads must wait; a direct `try_lock` probe proves the shared boundary. Tests cover changed/equal-sized replacement, two overlapping writers and an injected persistence-boundary error after an actual Fjall insertion. This error is controlled injection, not an actual disk fault. Writer unwind uses real mutex poisoning. Paused writers are released before assertions to avoid failure-path test hangs.

## Decisions and Simplification

Only equal `Current` revisions authorize reuse. `Unmeasured` and `Invalid` never do. The returned `RetainedPayloadUsage.revision` belongs to its guarded facts; callers must never attach a separately read revision to an earlier snapshot.

The explicit simplification pass moved the cohesive raw mutation gateways into `payload_usage.rs` and used direct closures instead of extra inner helpers. The parent store is 611 lines, the accounting module 217 and its tests 583. No new dependencies, persisted counters, schema or alternate storage owner were introduced. Future authority → automatic state → payload guard ordering remains Plan 02's responsibility; storage helpers never acquire authority.

Local `AGENTS.md`, its Bright Builds sidecar, overrides, architecture/code-shape/testing/verification/Rust standards and both active lesson inputs informed the implementation. Root confirmed fetch, rebase and pinned submodule bootstrap before editing. No project skills were present.

## Deviations from Plan

None. The module relocation stays within assigned files and implements the planned simplification pass. Commit timing, state ownership and empty requirement completion follow the explicit strict-wrapper instructions.

## Issues Encountered

An initial fixture incorrectly called `.len()` on the block encoder's `Result`; it was corrected before recording meaningful behavioral RED. Scoped Clippy subsequently identified seven slice clones, corrected without suppressions. App-owned Cargo diagnostics briefly contended for the target; commands were polled and liveness inspected without termination. The attempted process sample did not persist and is not claimed as evidence.

The breadcrumb checker initially excluded untracked new modules. Root's reversible index preparation resolved the tracked-file contract; the checker was left unchanged.

## Known Stubs and Threat Scan

No goal-blocking stubs were found. No security-relevant surface outside the plan's accounting/mutation trust boundaries was introduced.

## Next Plan Readiness

Plan 02 can request exact facts, use their captured revision for conservative idle reuse, and serialize durable locks through the existing authority. Lock publication/planning coordination and the genuine automatic-target lifecycle proof are still later-plan work. Full phase verification and all commits/push remain pending root finalization.

## Self-Check: PASSED

All seven assigned files and this summary exist. The unchanged base commit exists. Targeted behavioral checks, strict scoped Clippy, formatting, breadcrumbs, file lengths and diff checks passed. Atomic task commits are intentionally absent under D-10; finalization and full phase verification remain root-owned.
