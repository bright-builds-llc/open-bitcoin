---
generated_by: gsd-phase-researcher
lifecycle_mode: yolo
phase_lifecycle_id: 153-2026-10-03T04-01-54
generated_at: 2026-10-03T04:07:53Z
---

# Phase 153: Automatic Prune Retention Integration - Research

**Researched:** 2026-10-02 CDT / 2026-10-03 UTC
**Domain:** Rust/Fjall retention accounting, shared authority, durable flush integration
**Confidence:** HIGH for current APIs and integration gaps; MEDIUM for proposed performance strategy

<user-constraints>
## User Constraints (from CONTEXT.md)

The following decisions, discretion and deferred ideas are copied verbatim from Phase 153 context; their provenance is the user-approved yolo discussion artifact. [VERIFIED: .planning/phases/153-automatic-prune-retention-integration/153-CONTEXT.md]

### Locked Decisions

### Authoritative usage and target

- **D-01:** Measure retained block and encoded undo payload value bytes
  from the actual durable store. Include protected/recent and nonactive
  payloads in total usage; candidate sizes refer only to current active
  height/hash pairs. Count either present mate and omit both-absent pairs.
  Do not use snapshot size, file allocation, invented averages, or decoded
  block size as the retained-byte authority.
- **D-02:** Pass configured mode, measured usage, current active tip,
  injected network prune-after height and current durable locks to the
  existing pure automatic planner. Retain its legal minimum and overflow
  behavior. Protected or nonactive bytes can keep usage above target;
  the target never permits deleting protected history.

### Ordinary lifecycle and authority

- **D-03:** Reuse the existing Periodic worker and Always shutdown flush
  path through `ManagedNetworkHandle::flush_coins`; add no independent
  deletion worker or alternate unlink owner. Automatic retention must be
  able to run when coins policy has no write due, using existing nonempty
  prune-plan semantics. A nonempty automatic plan must request the existing
  full coins/chain-metadata checkpoint, following Knots' prune-triggered
  flush rule. Keep `FlushMode::None`, disabled/manual-only,
  under-target, short-chain and fully protected cases as no-op gates.
- **D-04:** Assemble/apply against current authoritative chain facts and
  durable locks. Serialize lock updates with automatic planning/deletion,
  or use an equally strong shared guard; a stale lock snapshot must not
  allow deletion across a newly committed lock.
- **D-05:** Explicit prune configuration with a datadir must select the
  durable lifecycle even when network activation is disabled. Preserve
  opt-in networking and existing startup recovery before readiness.

### Failures and evidence

- **D-06:** Propagate accounting/storage errors and refuse without a
  fabricated plan. Apply only through paired durable unlink. Evict deleted
  hashes and forget undo on successful deletes even if a later flush fails;
  counters and have-pruned remain earned by committed deletes only.
- **D-07:** Reopen must finish or refuse interrupted pruning under current
  durable lock rules. Subsequent cycles must remeasure retained data.
  Automatic deletion must preserve serving labels and wallet eligibility;
  never introduce leftover-snapshot fallback.
- **D-08:** Behavioral tests must drive the production automatic lifecycle
  against real storage with measured bytes exceeding a legal 550 MiB
  target. Cover protected windows/locks, prune-after, no-op modes, repeated
  activity, later-flush error, restart and operator/wallet regressions.
  Keep fixtures hermetic and resource-conscious; source-string checks or
  synthetic production accounting overrides cannot prove this closure.
- **D-09:** Register new Rust source/test paths in parity breadcrumbs,
  document logical Fjall-byte versus Knots flat-file accounting differences,
  refresh contributor evidence, review and simplify the touched seams,
  then run default `bash scripts/verify.sh` including Bazel and coverage.
  Re-audit integration after closure; milestone archival stays separate.
- **D-10:** This strict wrapper defers all commits and push until clean
  phase verification and lifecycle validation. Earlier workflow commit
  steps become pending finalization records; never bypass hooks.

### Agent's Discretion

Choose the smallest store-accounting interface, bounded measurement
strategy, lock synchronization mechanism and fixture organization that
honor these decisions. No pending todos matched the phase.

### Deferred Ideas (OUT OF SCOPE)

Temporary IBD targets, archive serving, assumeutxo, BIP37, automatic repair,
public defaults, public-network release gates, production readiness and
production-funds wallet claims remain deferred.
</user-constraints>

<phase-requirements>
## Phase Requirements

| ID | Description | Research Support |
| --- | --- | --- |
| PRUN-01 | Operator can disable prune, select manual-only prune, or set an automatic target of at least 550 MiB. | Connect existing configured mode to measured store facts and ordinary production flush; durable offline activation and genuine threshold-crossing proof. [VERIFIED: .planning/REQUIREMENTS.md; config/prune.rs; runtime_authority.rs] |
| PRUN-02 | Automatic prune keeps the last 288 blocks and does not start before the network prune-after height. | Reuse existing pure planner with current active tip, injected chain parameters and serialized durable locks; exact boundary and production deletion tests. [VERIFIED: .planning/REQUIREMENTS.md; chainstate/src/prune/plan.rs; prune/range.rs] |
</phase-requirements>

## Project Constraints (from AGENTS.md)

- Preserve pinned Knots `29.3.knots20260210` behavior, record intentional differences in `docs/parity/index.json` plus companion docs, and register new Rust source/test breadcrumbs. [VERIFIED: AGENTS.md]
- Keep Bitcoin policy pure; Fjall, clocks, network and daemon effects stay in existing shell crates. No production Rust Bitcoin library or new production crate. [VERIFIED: AGENTS.md; .planning/REQUIREMENTS.md; standards/core/architecture.md]
- Rust `1.94.1` and edition 2024 are pinned; materialize the pinned Knots submodule with `git submodule update --init --recursive` if missing. Bun is canonical automation runtime; this repo has no package.json/bootstrap install requirement. [VERIFIED: AGENTS.md; rust-toolchain.toml; packages/Cargo.toml]
- Follow GSD workflow, maintain localized append-only task/lesson records, preserve managed standards, use early returns, internal `maybe` names, `foo.rs` plus `foo/`, visible error propagation, and behavior-focused Arrange/Act/Assert tests. Review oversized functions/files rather than expanding registries indefinitely. [VERIFIED: AGENTS.md; AGENTS.bright-builds.md; standards/core/code-shape.md; standards/core/testing.md; standards/languages/rust.md]
- Use `bash scripts/verify.sh` as completion/precommit contract; it includes formatting, strict Clippy, build/tests, coverage, managed checks and Bazel smoke. Route ad-hoc Cargo/Bazel through `bun run scripts/command-timings.ts run --key <stable-key> -- <command>`; do not overlap Cargo against one target directory. Poll running jobs at least every 60 seconds and collect liveness before termination. [VERIFIED: AGENTS.md; scripts/verify.sh]
- Refresh relevant README/operator documentation; tracked generated LOC freshness is required. Preserve opt-in networking, hermetic verification, dry-run-first migration and no public readiness/funds claims. [VERIFIED: AGENTS.md; .planning/PROJECT.md; .planning/REQUIREMENTS.md]
- Root sidecar and overrides were read; overrides contain only a template, no substantive exception. Both active lesson files were fully read; they reinforce repo-local operator commands, exact environmental evidence and distinguishing host/cache stalls. No project skills directories were found at `.claude/skills` or `.agents/skills`. [VERIFIED: standards-overrides.md; /Users/peterryszkiewicz/.codex/tasks/lessons.md; .codex/tasks/lessons.md; directory probes]
- Strict Phase 153 decisions override workflow commit timing: write artifacts now, defer commits/push until clean full phase verification and lifecycle validation; do not bypass hooks. [VERIFIED: 153-CONTEXT.md D-10]

## Summary

The missing consumer is narrow: `ManagedNetworkHandle::flush_coins` still passes an empty plan, although configuration, pure automatic policy, durable paired unlink and success/error cache eviction already exist. The daemon calls that method every second with Periodic and on shutdown with Always. The 50–70-minute jitter applies to coins checkpoints, not worker wakeups. Extend this path; do not create a retention owner beside it. [VERIFIED: runtime_authority.rs:532; runtime_authority/prune_flush.rs; bin/open_bitcoind/coins_flush.rs]

Two requirements make this more than adding a planner call. First, actual retained block and encoded undo values must be measured independently from candidate eligibility. Second, ordinary nonempty plans currently execute even when coins policy returns None, after which coins/chain metadata are not checkpointed. Select the existing Always behavior for nonempty automatic plans, and serialize durable RPC lock mutations with the same authority. Knots explicitly uses pruning as a full-checkpoint trigger. [VERIFIED: chainstate/flush_lifecycle.rs:299,468; context/prune.rs; packages/bitcoin-knots/src/validation.cpp:3130]

**Primary recommendation:** add store-owned snapshot-size accounting, an idle measurement fast path, serialized lock operations and network prune-after injection; construct/apply the existing planner under shared authority, checkpoint nonempty plans through the existing owner, and prove it using one real >550 MiB fixture. [VERIFIED: 153-CONTEXT.md D-01..09; local seams above]

## Standard Stack

### Core

| Component | Pinned version | Purpose | Recommendation/source |
| --- | --- | --- | --- |
| Rust / Cargo | 1.94.1; edition 2024 | Existing first-party implementation | Retain toolchain; Cargo probe matches pin. [VERIFIED: rust-toolchain.toml; packages/Cargo.toml; cargo --version] |
| Fjall | 3.1.4 | Live-value size facts, snapshots and durable paired unlink | Use installed pinned API; do not upgrade this phase. [VERIFIED: packages/Cargo.lock; installed fjall-3.1.4 source] |
| std collections/sync | Rust 1.94.1 std | Sorted facts and shared authority/measurement state | Reuse BTreeMap and Arc/Mutex conventions. [VERIFIED: prune/plan.rs; runtime_authority.rs] |
| Existing chainstate/node/RPC crates | Workspace 0.1.0 | Policy, storage/authority, daemon/RPC | Extend these crates only. [VERIFIED: packages/Cargo.toml; .planning/REQUIREMENTS.md] |

### Supporting

| Component | Pinned/current local version | Use |
| --- | --- | --- |
| Bun | 1.4.2 | Existing automation; pin and installed version agree. [VERIFIED: .bun-version; bun --version] |
| Bazel | 8.6.0 locally | Required smoke build via repo verifier. [VERIFIED: bazel --version; scripts/verify.sh] |
| cargo-llvm-cov | 0.8.5 locally | Existing core coverage contract. [VERIFIED: cargo llvm-cov --version; scripts/verify.sh] |

**Installation:** no new dependency install. Recommended dependencies are existing pinned ones, so npm registry checks do not apply. Publish dates/latest registry versions were not queried because no upgrade is recommended; all API claims target the installed lockfile version. [VERIFIED: packages/Cargo.lock; .planning/REQUIREMENTS.md]

## Architecture Patterns

### Recommended change seams

| Seam | Planned responsibility |
| --- | --- |
| `storage/fjall_store/payload_usage.rs` (new suggested module) | Exact payload-prefix total and active-pair size facts from one snapshot; optional transient generation/measurement gate. [VERIFIED: fjall_store.rs; fjall API; 153-CONTEXT.md D-01] |
| `chainstate/flush_lifecycle.rs` and `chainstate/fjall_store.rs` | Small typed sink facts capability; forward production adapter to its existing inner store. [VERIFIED: FlushPersistSink; FjallChainstateStore::inner] |
| `network/runtime_authority/prune_flush.rs` | Current tip/mode/locks, planner invocation, full-checkpoint selection and existing delete-receipt eviction. [VERIFIED: runtime_authority.rs; prune_flush.rs] |
| `context/prune.rs`, `dispatch/prune.rs` | Move lock read-modify-write under authority or shared store guard; retain request validation and response shapes. [VERIFIED: current RPC lock writers] |
| `sync/types.rs`, daemon startup | Supply network prune-after height and enable durable offline prune lifecycle. [VERIFIED: SyncNetwork; open_runtime_store; 153-CONTEXT.md D-02,D-05] |

These paths are proposed placement, not APIs already implemented. Keep existing file ownership and split new cohesive helpers rather than extending large authority/daemon files. [VERIFIED: standards/core/code-shape.md; current module boundaries]

### Pattern 1: one consistent measured store view

Use `db.snapshot()` and `fjall::Readable`: enumerate only `block:` in BlockIndex and `undo:` in Chainstate, summing each guard's `size()` into a checked u64 total. Query each active position's two keys with snapshot `size_of`; add a candidate only if at least one Option is present. A present zero-byte value is distinct from an absent pair. Do not reconstruct sizes by decoding blocks or undo. [VERIFIED: installed fjall-3.1.4/src/{db.rs,guard.rs,readable.rs,snapshot.rs}; save_block; save_undo]

The total includes recent, locked and nonactive payloads. The candidate map is height → actual block-value-plus-undo-value size for current active-chain hashes only. The planner accounts protected bytes through its total, skips their heights and may return all eligible candidates while usage still exceeds target. Missing pairs are excluded rather than contributing zero-size prune heights. [VERIFIED: 153-CONTEXT.md D-01,D-02; AutomaticPruneInput; plan_automatic_prune]

A Fjall snapshot makes the measurements consistent; it does not serialize concurrent writes. Use the authority/shared guard to protect lock publication and plan application. A snapshot's old lock map is not permission to ignore a newly committed lock. [VERIFIED: fjall-3.1.4/src/db.rs:150; current separate RPC writers; 153-CONTEXT.md D-04]

### Pattern 2: one authority transaction for retention and locks

The authority is `Arc<Mutex<ManagedPeerNetwork<S,V>>>`. Keep current active-chain positions, typed mode, planner, unlink and callback cleanup inside one `mutate` operation. New sink methods should return typed store facts/errors, with the Fjall wrapper forwarding to the same `inner()`; do not make RPC measure its metrics-store clone then pass a stale automatic plan into the owner. [VERIFIED: runtime_authority.rs:69,110; chainstate/fjall_store.rs; 153-CONTEXT.md D-04]

Use explicit authority operations for lock replacement/clear, retaining boundary validation in RPC. Each operation must hold the serialization boundary across load → modify → SyncAll commit. Route the RPC durable setters through these operations; a freshly acquired lock must take effect before the next deletion. Update same-name replacement and clear tests to exercise that owner. Alternatively, a clone-shared store mutex is acceptable only if held across the whole plan/apply cycle and every lock writer uses it; per-method locking is insufficient. Establish one acquisition order and do not reacquire a held mutex through paired-delete helpers. [VERIFIED: context/prune.rs:35,80; prune/records.rs; 153-CONTEXT.md D-04; runtime_authority mutate]

Audit all `sync_prune_locks` callers, including tests and recovery, before declaring serialization complete. Do not leave the existing RPC clone path as a bypass. Keep fixture-only in-memory adapters explicit; unsupported automatic accounting must refuse rather than return a fabricated zero total. [VERIFIED: sync_prune_locks references; default no-op FlushPersistSink unlink; audit INT-01]

### Pattern 3: prune-induced full checkpoint, unchanged cleanup owner

After creating a nonempty automatic plan, select `FlushMode::Always` for the same `flush_and_evict_pruned_blocks` call. Leave empty plans on the requested mode. Gate `FlushMode::None` before automatic work. This uses existing disk-space/coins policy instead of constructing an invented FlushDecision. Do not change the pure planner, manual byte-budget behavior or keep-window formulas. [VERIFIED: FlushMode; decide_flush; execute_flush_applying_plan; 153-CONTEXT.md D-03]

The owner orders prefix persistence → paired unlink → coins flush → chain-meta persistence. The manager forgets undo using committed-delete callbacks on both outcomes; authority eviction reads the same delete receipts even on failure. Always use that wrapper. The existing recovery contract permits finish-or-refuse when earlier durable chain metadata is stale; forcing the final checkpoint improves normal completion but does not remove crash seams. [VERIFIED: chainstate.rs:411; flush_lifecycle.rs:337,468; prune_flush.rs:28; prune.rs resume; v2.4 audit advisory WR-01]

### Pattern 4: network input and offline durable activation

`SyncNetwork` already distinguishes mainnet/testnet/signet/regtest, but has no prune-after method. Add one shell mapping at that existing chain configuration seam and inject its result to the owner. Pinned Knots uses mainnet 100000, testnet/signet 1000 and ordinary regtest 1000 (Knots fastprune is separately 100). Existing RPC duplicates the mainnet-versus-other mapping; consolidate to avoid drift. Use the active chain's actual `ChainPosition.height`/hash, not header best height or slice length. [VERIFIED: sync/types.rs:72; context/prune.rs:116; kernel/chainparams.cpp; chainstate.active_chain; classify_prune_height]

`open_runtime_store` currently returns None whenever both sync and inbound are disabled. Extend durable selection when resolved prune mode is ManualOnly/Automatic with a datadir. Do not open a network transport merely to obtain durability. Keep `DurableSyncRuntime::open_with_runtime_activation` and `initialize` recovery before authority readiness; carry mode into that authority before the flush worker starts. [VERIFIED: bin/open-bitcoind.rs:350; sync/open_runtime.rs; initialize; 153-CONTEXT.md D-05]

### Performance strategy for the one-second tick

Do not load all block/undo values or walk a full blockchain snapshot each second. Prefix `Guard::size` avoids body decoding, but an exact pass still visits every retained payload key; candidate lookup also scales with active positions. Unbounded repeated history scans while holding network authority would compete with all peer/RPC mutations. [VERIFIED: fjall guard/prefix implementations; runtime_authority Mutex; coins_flush TICK_SECS]

Recommendation: first apply cheap mode/None/tip/prune-after gates; keep transient measured-state generation in a clone-shared store object; skip exact scans when payload generation and tip/lock/mode inputs are unchanged. Serialize measurement and every relevant payload mutation using a clone-shared mutex. A writer acquires its guard before invalidation/live mutation and holds it through the persistence outcome; RAII must conservatively invalidate and release/complete on every error or early return. Exact measurement acquires the same guard and returns its facts and reusable revision together before releasing it. Revision reads used for reuse must observe only completed writers. Invalidate after paired delete even if later metadata/coins effects fail. Remeasure on reopen and before the next non-skipped retention cycle. Do not persist a new approximate-byte schema. This is a corrected synchronization recommendation for Plan 01's transient revision contract, not a claim that the current source already implements it. [VERIFIED: put_bytes/persist sequencing; paired-delete sequencing; D-06,D-07; 153-01-PLAN.md Tasks 1–3]

**Pre-write-only epochs are insufficient:** a writer can increment the epoch, pause before insertion, then allow a measurement to take the old snapshot while both its initial/final epoch reads see that new epoch. Publishing old facts under the new epoch would make the subsequent same-epoch idle fast path incorrectly skip fresh data. Add a deterministic barrier/channel regression that pauses a clone writer at this exact boundary; measurement must block/refuse, and after completion its returned facts must include the write or be visibly invalid. Repeat with a failure after live insertion. This is a concurrency counterexample derived from the plan's stated ordering, not an observed runtime test result. [VERIFIED: 153-01-PLAN.md Tasks 2–3 ordering; 153-02-PLAN.md Task 2 reuse key; static interleaving analysis]

If a begin/end protocol replaces the mutex, it must track all overlapping in-flight writers rather than simple odd/even increments, require zero writers and a stable completed revision for snapshot/publication, and use RAII completion/invalidation on failure. The simpler shared mutex is the recommendation. Keep acquisition order authority → accounting guard; store-only writers must never acquire authority while holding the accounting guard. Avoid recursive acquisition by using guarded internal helpers for nested typed/raw writes and paired unlink. [VERIFIED: current authority Mutex and nested store writer families; 153-01/02 proposed contracts; decision-based synchronization recommendation]

For ongoing ingestion, coalesce changed-generation Periodic measurement using the existing worker's supplied time: recommend a named 60-second minimum between full measurement passes; first eligible activity and Always bypass that gate. This is an implementation recommendation, not a measured SLA or a new timer/worker. Tests inject time and prove skipped cycles do not delete from stale facts, then prove eventual fresh measurement. Locks still serialize and are loaded fresh when a delete occurs. If implementation instead maintains exact per-key derived sizes, prove every writer/replacement/delete invalidates or updates that cache before relying on it. [VERIFIED: D-03 Agent's Discretion; worker time injection pattern; store write references]

The minimal cache key must include payload generation, current active tip, configured mode and lock version/fresh locks; same byte count does not mean identical candidate identity. Existing generic raw `put_bytes/remove_bytes`, `save_block`, `save_undo`, migration seeding and direct batch delete are the write families to audit. Persisted prefixes remain authority; any optimization failure must cause remeasurement/refusal, not zero usage. Generation overflow must invalidate conservatively. [VERIFIED: fjall_store.rs; fjall_store/coins.rs; fjall_store/prune.rs; D-01,D-06]

Do not claim a one-second maximum lock hold: no retained-history benchmark was run. At implementation time record exact-pass durations/key count and verify repeated unchanged ticks avoid the expensive branch; if measured contention is material, use bounded snapshot traversal with a validated publication generation or exact derived accounting rather than hiding delay. [VERIFIED: research scope and no benchmark execution; standards verification/evidence rules]

## Don't Hand-Roll

| Problem | Don't build | Use instead |
| --- | --- | --- |
| Byte sizes | Decode/reencode every historical payload or stat the datadir | Fjall snapshot `size_of` and prefix guard `size`. [VERIFIED: fjall API] |
| Retention rules | New window/lock math or generic file heuristic | Existing `plan_automatic_prune`. [VERIFIED: plan.rs; D-02] |
| Paired durability | Independent block/undo removals or second flusher | Existing `commit_paired_delete` and FlushLifecycle. [VERIFIED: prune.rs; D-06] |
| Recovery/cache cleanup | Retry wrapper that forgets committed-error deletions | Existing receipts/callbacks, initialize and resume intent. [VERIFIED: prune_flush.rs; chainstate.rs; initialize] |
| Lock coordination | Snapshot-only race detection | Existing authority operations/shared serialization boundary. [VERIFIED: D-04; RPC separate writers] |

## Common Pitfalls

1. **Accounting candidates instead of retained total.** Locked/recent/nonactive values still consume usage, while only active eligible heights may delete. Separate total from candidates and assert sidechain and undo-only controls. [VERIFIED: D-01,D-02; planner]
2. **A periodic delete without checkpoint.** Current None decision path skips coins and chain meta even after deletion; nonempty automatic plans must request Always. Check wrote_coins and reopened metadata, not only absent keys. [VERIFIED: complete_coins_write; Knots validation.cpp:3130]
3. **Stale lock publication.** Network mutex does not cover current RPC store clone writes. Verify concurrent replacement waits for the owner and a lock committed first protects the subsequent plan. [VERIFIED: context/prune.rs; runtime_authority.rs]
4. **Undo omitted or resurrected.** Prefix persistence re-saves in-memory undo before deleting; callbacks must forget committed undo even on later failure. Remeasure afterwards rather than subtracting a guessed size. [VERIFIED: flush_window; execute_flush_applying_plan; chainstate.rs callbacks]
5. **Wrong prune-after equality.** Automatic starts only at tip > prune_after_height, not at equality. Last prunable is tip.saturating_sub(288). Preserve overflow behavior for enormous MiB targets. [VERIFIED: prune/range.rs; prune/plan.rs; Knots blockstorage.cpp:399]
6. **Physical-cap claim.** Logical live values differ from Fjall physical disk space and Knots blk/rev chunk buffers. Update intentional parity documentation without promising immediate space reclamation or changing the pure planner's budget rule. [VERIFIED: D-01,D-09; Knots CalculateCurrentUsage/FindFilesToPrune; fjall Keyspace disk_space/fragmented_blob_bytes docs]
7. **Idle full-history work and premature epoch publication.** Exact logical accounting has a scan cost even without decoding; verify no expensive scan on disabled/manual-only, None, short-chain and unchanged ticks. A pre-write bump alone cannot authorize reuse: serialize snapshot/fact/revision publication against complete writer lifetimes and test a writer paused between invalidation and live insertion. [VERIFIED: tick source; prefix API; D-03; 153-01/02 static interleaving above]
8. **Transient offline authority.** A metrics/wallet store existing is not proof network authority is durable. Assert startup chooses durable runtime and worker for explicit prune mode with networking disabled. [VERIFIED: open_runtime_store; audit tech debt]
9. **False closure via fixture target override.** Minimum 550 MiB is locked. Do not use injected fake total/target as the only product proof. Drive production ordinary lifecycle using real stored values. [VERIFIED: D-08; audit INT-01]

## Code Examples

The following are implementation sketches using verified pinned APIs; surrounding helper names are proposals, not existing functions. [VERIFIED: installed fjall-3.1.4 source]

### Exact prefix total without loading values

```rust
// Sources: fjall 3.1.4 src/guard.rs and src/readable.rs; adapt error mapping locally.
use fjall::{Keyspace, Readable, Snapshot};

fn measured_prefix_bytes(
    snapshot: &Snapshot,
    keyspace: &Keyspace,
    prefix: &[u8],
) -> Result<u64, AccountingError> {
    snapshot.prefix(keyspace, prefix).try_fold(0_u64, |sum, guard| {
        let size = u64::from(guard.size()?);
        sum.checked_add(size).ok_or(AccountingError::Overflow)
    })
}
```

### Present mates and candidate facts

```rust
// Source: fjall 3.1.4 Readable::size_of; preserve absence separately from zero size.
let maybe_block_bytes = snapshot.size_of(&store.block_index, block_key(hash))?;
let maybe_undo_bytes = snapshot.size_of(&store.chainstate, undo_key(hash))?;
if maybe_block_bytes.is_some() || maybe_undo_bytes.is_some() {
    let size = u64::from(maybe_block_bytes.unwrap_or(0))
        + u64::from(maybe_undo_bytes.unwrap_or(0));
    height_sizes.insert(position.height, size);
}
```

Access to store fields/key builders belongs inside its storage module. Total accounting uses both complete prefixes even when candidates are empty. Do not export keyspace handles solely for RPC convenience. [VERIFIED: FjallNodeStore private fields; D-01; architecture standards]

## Verification Plan

Nyquist-specific Validation Architecture is intentionally omitted: `workflow.nyquist_validation` is explicitly false. The following behavior plan still satisfies Phase 153 verification needs. [VERIFIED: .planning/config.json; D-08,D-09]

| Behavior | Existing fixture seam | Required evidence |
| --- | --- | --- |
| Total/candidate sizes including undo, nonactive, half pairs, absent pairs, excluded metadata | `storage/fjall_store/tests/prune_unlink.rs`; new small accounting module tests | Actual encoded-value byte lengths equal measured facts; guard paths do not decode. [VERIFIED: existing fixtures; D-01] |
| Legal target ordinary retention | `network/runtime_authority/tests.rs`; daemon `coins_flush/tests.rs` | Configured 550 MiB, real usage above target, Periodic call without manual RPC, paired absence, have-pruned, cache/undo eviction and full checkpoint. [VERIFIED: D-08; existing seams] |
| Locks/window/prune-after/no-op gates | Existing pure automatic tests plus production authority fixture | Protected bytes survive; equality/next-height boundary; disabled/manual-only/None unchanged; fully protected succeeds above target. [VERIFIED: planner tests; D-03] |
| Lock serialization | RPC prune dispatch tests plus authority concurrency fixture | Deterministic barrier/channel schedule, no sleeps: completed lock publication precedes and protects next cycle; no stale caller-supplied locks. [VERIFIED: D-04; RPC setters] |
| Later error and retry/reopen | Authority `error_after_unlink_drops_deleted_hash_and_retry_leaves_it_gone`; flush lifecycle sink tests | Production automatic delete receipts survive metadata/coins failure; next cycle/reopen does not serve/cache/recreate deleted payload. [VERIFIED: named existing test; D-06,D-07] |
| Accounting failure | Extend existing injected flush sinks | Error before delete/no fabricated plan; existing payloads/markers remain; periodic retry and shutdown refusal visible. [VERIFIED: D-06; existing sink pattern] |
| Offline durability | Daemon startup/inbound/checkpoint fixtures | Network disabled + explicit prune + datadir chooses recovered durable authority, without transport activation. [VERIFIED: D-05; bin startup] |
| Operator/serving/wallet regressions | Existing RPC prune/config, limited-serving, Phase 152 eligibility tests | Quartet, support counts, earned labels, no cache serving, older creating-payload refusal and failed-job evidence after automatic deletion. [VERIFIED: D-07; 150/152 contexts] |
| Idle scan bound | New transient measurement gate tests | Unchanged ticks skip scans; changed payload/tip/locks cause fresh eligible measurement; Always bypasses coalescing. [VERIFIED: recommended gate; D-07] |
| Paused clone writer | Plan 01 accounting tests and Plan 02 reuse tests | Barrier/channel pauses writer after beginning invalidation but before live write; measurement blocks/refuses, then observes completed new bytes or invalid revision. Error after insertion also completes/invalidate through RAII. No stale snapshot may be published under a completed revision. [VERIFIED: corrected synchronization recommendation; 153-01/02 contracts] |

### Genuine >550 MiB fixture

Use one serialized large fixture and keep the rest small. Store reusable codec-valid blocks under distinct canonical header hashes using existing `save_block`, with encoded undo via `save_undo`; sparse ChainPosition fixtures already support an old eligible height plus high tip. Reuse a bounded 2–4 MiB body buffer across writes and retain only a few bodies in authority cache. Continue writing until the real accounting API reports >550 * 1024 * 1024; do not assume decoded body length or compression equals that threshold. Undo bytes must participate in expected sums. [VERIFIED: D-08; authority/tests.rs plant_payload and sparse tip; save_block/save_undo]

A sparse synthetic fixture tests retention integration, not consensus validation of all the intervening blocks. Keep header/active positions/coins best-block/chain meta consistent for successful reopen; open the durable runtime before populating large bodies to avoid needless hydration. Use ordinary daemon flush helpers where accessible, not only manual `flush_applying_prune_plan`. Drop all store/authority clones before deleting the temp directory; put large fixture cleanup on failure paths. [VERIFIED: existing sparse authority fixture; sync/open_runtime.rs; D-08; test cleanup helpers]

Do not make 550 MiB of encoded undo unless testing hydration memory explicitly: durable open decodes all undo records. Prefer bulk bytes in block bodies and modest, real undo values. A later error case can reuse the large fixture in the same test scenario, while unit-sized fault sinks cover precise error branches; avoid several concurrent >550 MiB fixtures. [VERIFIED: open_runtime.rs load_all_undo_records; snapshot_codec undo representation; D-08 resource-conscious requirement]

Quick targeted commands (new filter/module names below are suggested, adapt to implemented names): [VERIFIED: AGENTS.md command wrapper; Cargo workspace]

```bash
bun run scripts/command-timings.ts run --key phase153-node-tests -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node automatic_prune -- --test-threads=1
bun run scripts/command-timings.ts run --key phase153-rpc-tests -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc prune -- --test-threads=1
bash scripts/verify.sh
```

The genuine threshold test may exceed 30 seconds; do not label it a quick unit test or exclude it from default verification. Final closure also requires current parity/traceability, full verifier and an integration re-audit. No builds/tests were run by this research agent. [VERIFIED: D-08,D-09; research tool history]

## Runtime State Inventory

This is integration/refactoring of existing runtime seams, without a rename or data-format migration. Items below keep current names. [VERIFIED: D-01..10; canonical refs]

| Category | Items found | Required action |
| --- | --- | --- |
| Stored data | block/undo values, chain_meta, coins heads/best-block, prune_locks, prune_intent, have_pruned, prune_summary, leftover snapshot | Code wiring and existing runtime paired deletes; no proposed schema migration. Derive transient byte cache anew on open. [VERIFIED: fjall_store.rs; coins.rs; prune.rs; prune/records.rs] |
| Live service config | Existing JSONC prune integer/runtime mode | Use current resolved mode; no proposed service/UI-only config change. External service accounts were not inspected because phase adds none. [VERIFIED: config/prune.rs; D-02,D-05] |
| OS-registered state | No registry rename/change specified | Preserve existing daemon/service registration; no OS inventory performed or absence claim made. [VERIFIED: phase scope] |
| Secrets/env vars | No new credentials, env-var rename or secret key proposed | Preserve RPC authentication; never print secrets in failures/tests. [VERIFIED: phase scope; http.rs] |
| Build artifacts | Existing Cargo/Bazel outputs and generated LOC/parity artifacts | Rebuild via verifier, refresh tracked freshness evidence; no installed-package rename/reinstall proposed. [VERIFIED: AGENTS.md; scripts/verify.sh] |

## Environment Availability

| Dependency | Available | Version/evidence | Fallback |
| --- | --- | --- | --- |
| Rust/Cargo | Yes | Cargo 1.94.1; pinned toolchain. [VERIFIED: cargo --version; rust-toolchain.toml] | None needed |
| Installed Fjall source | Yes | Lockfile/source 3.1.4. [VERIFIED: packages/Cargo.lock; cargo registry] | Official versioned docs |
| Bun | Yes | 1.4.2. [VERIFIED: bun --version] | None needed |
| Bazel | Yes | 8.6.0. [VERIFIED: bazel --version] | No smoke-build waiver |
| cargo-llvm-cov | Yes | 0.8.5. [VERIFIED: cargo llvm-cov --version] | No coverage waiver |
| Knots baseline | Initialized | Submodule a9aee730466ac67d35a3c03ee24676be5e045878, v29.3.knots20260210. [VERIFIED: git submodule status] | Repo documented submodule init |
| Fixture disk | Present | df reported about 42 GiB free during research. [VERIFIED: df -h .] | Clean fixture promptly; do not alter operator disk thresholds |

No missing execution dependency was found by these probes. Tool availability does not establish that full verification or the large fixture will pass; previous audit's host-loading stall is historical evidence only. [VERIFIED: probes; .planning/v2.4-MILESTONE-AUDIT.md]

## Security Domain

Security enforcement is not explicitly disabled in config; include this domain. Use ASVS 5.0.0 category numbering rather than the old template's 4.x chapter numbers. This is a targeted threat map, not an ASVS compliance claim. [VERIFIED: .planning/config.json; CITED: https://owasp.org/projects/asvs]

### Applicable ASVS categories

| ASVS 5.0.0 category | Applies | Phase control |
| --- | --- | --- |
| V2 Validation and Business Logic | Yes | Typed prune mode/ranges, checked byte totals, legal eligibility and serialized lock/delete flow. [CITED: https://raw.githubusercontent.com/OWASP/ASVS/v5.0.0/5.0/en/0x11-V2-Validation-and-Business-Logic.md] [VERIFIED: prune mode/plan/locks; D-04] |
| V6 Authentication | Existing boundary | RPC HTTP authentication still precedes parsing/dispatch; no alternate mutation route. [CITED: https://raw.githubusercontent.com/OWASP/ASVS/v5.0.0/5.0/en/0x15-V6-Authentication.md] [VERIFIED: http.rs:136] |
| V7 Session Management | No new session work | Phase changes no browser/user-session mechanism; existing RPC auth remains. [CITED: https://github.com/OWASP/ASVS/blob/master/5.0/en/0x16-V7-Session-Management.md] [VERIFIED: phase scope; http.rs] |
| V8 Authorization | Existing boundary + storage ownership | Authenticated operator route, same authoritative store and protected history enforcement. [CITED: https://raw.githubusercontent.com/OWASP/ASVS/v5.0.0/5.0/en/0x17-V8-Authorization.md] [VERIFIED: dispatch/prune.rs; D-04] |
| V11 Cryptography | No new primitives | Use current storage/auth machinery; implement no new crypto. [CITED: https://raw.githubusercontent.com/OWASP/ASVS/v5.0.0/5.0/en/0x20-V11-Cryptography.md] [VERIFIED: phase scope] |

### Threat patterns

| Pattern | STRIDE | Mitigation |
| --- | --- | --- |
| Lock/delete race | Tampering | Serialize lock publication and current plan/apply; deterministic interleaving test. [VERIFIED: current separate writer seam; D-04] |
| Byte-accounting failure/overflow | Tampering / denial of service | Checked sums, exact store facts, no delete on measurement error. [VERIFIED: D-01,D-06] |
| Every-tick history work | Denial of service | Cheap gates, no body decode, unchanged-generation fastpath and bounded-frequency Periodic refresh. [VERIFIED: current tick/authority; recommended gate] |
| Committed-delete cache resurrection | Information disclosure | Existing receipt eviction and undo forgetting on both success and failure. [VERIFIED: prune_flush.rs; chainstate.rs; D-06] |
| Partial delete/crash | Tampering | Existing SyncAll paired tombstones, intent and finish-or-refuse reopen. [VERIFIED: prune.rs] |
| Raw storage detail in operator errors | Information disclosure | Preserve sanitized RPC/support errors and omit paths/keys/secret values. [VERIFIED: context/prune.rs; Phase 150 D-17; http auth] |

## State of the Art

| Existing state | Required phase state | Impact |
| --- | --- | --- |
| Configured automatic target, pure tests, no production planner caller | Measured production caller on ordinary flush | Closes audit INT-01. [VERIFIED: audit; runtime_authority.rs; D-02,D-03] |
| Knots file-info nSize+nUndoSize with chunk/IBD buffers | Existing Open Bitcoin per-hash logical value target | Preserve locked planner and explicitly document layout/accounting difference. [VERIFIED: Knots blockstorage.cpp; plan.rs; D-09] |
| RPC lock map written outside network mutex | Shared serialized publication and prune owner | Locks protect automatic deletion as well as connected manual paths. [VERIFIED: context/prune.rs; D-04] |

No deprecated library migration is required or recommended. [VERIFIED: phase scope; existing pinned stack]

## Assumptions Log

No training-only factual claims are used. Proposed helper names, measurement cadence and module placement are implementation recommendations derived from verified seams, not assertions that APIs or performance guarantees already exist. Plans 01–04 select concrete work and verification obligations; their selection is decision-based evidence, not proof of implementation or runtime success. Performance remains MEDIUM until measured. The corrected clone-shared synchronization recommendation must be reflected in Plan 01's writer/publication contract and Plan 02's reuse checks before execution; a pre-write-only epoch is explicitly rejected. [VERIFIED: 153-01..04-PLAN.md; static counterexample above; Agent's Discretion]

| # | Claim | Section | Risk if wrong |
| --- | --- | --- | --- |
| None | No claims tagged ASSUMED | All | No extra locked decision depends on training recall |

## Open Questions

1. **RESOLVED — Cost of exact measurement during rapid ingestion (planning choice):** Plan 02 Task 2 selects metadata-only snapshot measurement, unchanged-input scan skipping and a named 60-second minimum for changed Periodic inputs; first eligible activity and Always bypass coalescing. Deferred cycles never apply cached candidates. It requires pass duration/key-count evidence; Plan 03 uses one resource-conscious genuine fixture. Actual timings remain execution evidence to collect, not a claimed SLA or a planning blocker. [VERIFIED: 153-02-PLAN.md Task 2; 153-03-PLAN.md Task 2; pinned Fjall APIs]
2. **RESOLVED — All mutation invalidation paths (planned audit plus corrected synchronization):** Plan 01 Task 3 assigns generic raw writes, save_block/save_undo, migration/raw seeding and direct paired-delete writer coverage, including equal-size replacement, errors, overflow and reopen. The correction above requires clone-shared measurement/mutation synchronization with RAII completion, paired facts/revision publication and a deterministic paused-writer regression; pre-write-only epoch comparisons cannot satisfy that contract. Root must incorporate this correction in Plan 01 Tasks 1–3 and Plan 02 Task 2 before execution. Resolution means the required audit and safe contract are identified, not that the writer audit or tests have already passed. [VERIFIED: 153-01-PLAN.md Tasks 1–3; 153-02-PLAN.md Task 2; fjall_store.rs/coins.rs/prune.rs; static interleaving analysis]
3. **RESOLVED — Existing memory RPC fixture compatibility (planning choice):** Plan 01 defines unsupported accounting/lock capabilities as explicit StorageError while disabled/transient ordinary flush avoids calling them. Plan 02 Task 1 routes production lock reads and read-modify-SyncAll writes through authority, requires fixture-only memory-with-metrics cases to remain explicit, and prohibits production bypass; Plan 03 proves automatic behavior through real durable startup/storage. Executors may adapt fixture construction, but cannot use a metrics-store fallback in production or fake automatic measurements. [VERIFIED: 153-01-PLAN.md interfaces/Task 1; 153-02-PLAN.md Task 1; 153-03-PLAN.md Tasks 1–2]
4. **RESOLVED — Host verification reliability (verification obligation):** Plan 04 Task 2 selects the default full native verifier including the genuine fixture, coverage and Bazel, requires polling and liveness before termination, and keeps the task incomplete on blocked verification. No shortcut, waiver, hook bypass or premature completion is allowed; root performs lifecycle verification and re-audit afterwards. Whether the host completes this run remains execution evidence, not an invented resolved environment diagnosis. [VERIFIED: 153-04-PLAN.md Task 2/verification/output; audit environment limitation; active host-stall lesson]

## Sources

### Primary (HIGH confidence)

- Phase 153 CONTEXT, ROADMAP/REQUIREMENTS/PROJECT/STATE and v2.4 audit; prior 147/148/149/150/152 decisions. [VERIFIED: local file reads]
- Local production planner, flush lifecycle, store adapter, prune records, authority, RPC lock/dispatch and daemon sources cited above. [VERIFIED: local file reads/search]
- Pinned Knots `node/blockstorage.cpp` FindFilesToPrune/CalculateCurrentUsage, `validation.cpp` FlushStateToDisk and `kernel/chainparams.cpp` prune-after heights. [VERIFIED: pinned submodule reads]
- Installed `fjall-3.1.4/src/keyspace/mod.rs:648`, `guard.rs:55`, `db.rs:153`, `readable.rs`, `snapshot.rs`; exact API signatures and snapshot warning. [VERIFIED: installed registry source]
- [Fjall versioned Keyspace documentation](https://docs.rs/fjall/3.1.4/fjall/struct.Keyspace.html) — size_of, prefix, disk_space and reclamation distinction. [CITED: docs.rs/fjall/3.1.4/fjall/struct.Keyspace.html]
- [OWASP ASVS](https://owasp.org/projects/asvs) and official versioned 5.0.0 category pages linked above. [CITED: owasp.org/projects/asvs; official OWASP/ASVS]

Context7 tools were unavailable in this session; installed pinned library source supplied exact API evidence. Versioned Guard/Readable web fetches failed, so those claims cite installed source instead. Search-only community material was not used. [VERIFIED: tool discovery and fetch results]

### Secondary / Tertiary

None required; no secondary claims or LOW-confidence ecosystem recommendations. [VERIFIED: research tool history]

## Metadata

- **Standard stack:** HIGH — pin, installed source and local executable probes verified. [VERIFIED: sources above]
- **Architecture:** HIGH for integration seams; MEDIUM for proposed transient cache/coalescing tradeoff until measured. [VERIFIED: sources above; no benchmark]
- **Pitfalls:** HIGH — exact code paths, locked decisions and audit evidence. [VERIFIED: sources above]
- **Research date:** 2026-10-02 CDT / 2026-10-03 UTC. [VERIFIED: current clock]
- **Valid until:** recheck whenever these store/authority seams or the phase context change; otherwise review within 30 days. This is a planning review recommendation. [VERIFIED: current context/seam dependency]
- **Git:** no commit created; strict wrapper finalization remains pending until clean verification. [VERIFIED: D-10; task instruction]
