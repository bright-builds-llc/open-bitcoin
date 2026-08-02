---
phase: 135
name: snapshot-schema-checkpointing-and-recovery
status: researched
researched: 2026-08-02
confidence: high
---

# Phase 135: Snapshot Schema, Checkpointing, and Recovery - Research

<user-constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

### Source-only snapshot contract

- **D-01:** Persist one atomic mempool-local envelope containing its own format
  version, captured lifecycle generation and capture time, canonical witness
  transaction bytes with trustworthy acceptance time, and a separate explicit
  set of authoritative unbroadcast member identities. The unbroadcast set must
  come from ManagedNetworkHandle authority in the same capture as the mempool
  records; historical origin=local or relay-requested metadata is not proof
  that initial-broadcast intent still survives.
- **D-02:** Derive txid, wtxid, fee, virtual size, topology, resource aggregates,
  serving/fanout/peer/compact indexes, and all other cache state from canonical
  transaction bytes plus current chainstate and policy. Redundant stored
  identities or derived values may be accepted only as compatibility inputs and
  must be verified rather than trusted.
- **D-03:** Keep the snapshot schema local to the mempool namespace. Support
  only explicit, tested legacy migrations. A legacy snapshot that cannot prove
  current unbroadcast membership must recover no unbroadcast entries, and a
  record without trustworthy acceptance time must not be silently assigned a
  fresh age.
- **D-04:** Unsupported versions, structurally corrupt envelopes, identity
  mismatches, and failed decode are snapshot-level typed failures: do not
  best-effort decode, overwrite, or silently clear the last stored bytes.
  Semantic record failures may drop that record and its dependent children
  while independent valid records remain recoverable.

### Checkpoint durability and coalescing

- **D-05:** Add one single-flight checkpoint coordinator over Phase 134's
  prepare → execute → complete snapshot capability. A periodic tick skips
  clean state, captures the dirty-generation high-water mark under authority,
  releases the lock before encoding/storage I/O, and coalesces newer mutations
  behind the in-flight write instead of creating concurrent snapshots.
- **D-06:** Periodic checkpoints use synchronous persistence so the documented
  successful-checkpoint boundary covers process, OS, and power loss. Clean
  shutdown quiesces mutation producers, completes or accounts for any in-flight
  write, then forces a current-generation synchronous checkpoint before it may
  claim a clean mempool shutdown.
- **D-07:** A successful stale receipt advances truthful last-durable evidence
  for its captured generation but never clears newer dirty state. Only an exact
  current-generation completion may clear dirtiness. Encoding or storage
  failure exact-aborts an unachieved capability, retains dirty state, records a
  typed failure, and remains retryable. Persistence success followed by
  completion-dispatch failure retains the achieved receipt for idempotent
  completion and must never abort an effect that already happened.
- **D-08:** Checkpoint evidence exposes current, dirty, in-flight, and
  last-durable generations; capture and completion time; trigger; persistence
  strength; outcome/failure class; and overdue state. The exact generation loss
  range is (last_durable_generation, current_generation]. Temporal age is
  measured from capture time, and no fixed wall-clock bound may be claimed while
  I/O remains pending or failing. Phase 137 may render this contract broadly but
  must not redefine it.

### Policy-aware staged recovery

- **D-09:** Recovery is a startup-only staged transition into fresh mempool and
  derived state. Validate the envelope first, verify canonical identities,
  dependency-sort records with deterministic tie-breaks, and replay them against
  current chainstate, consensus, static/effective admission policy, and their
  preserved acceptance times without exposing partially rebuilt runtime caches.
- **D-10:** Recover independent valid records even when another record is stale
  or invalid. Preserve the stable typed classes recovered,
  dropped_confirmed, dropped_duplicate, dropped_missing_parent,
  dropped_policy_incompatible, and dropped_evicted, and add a distinct
  dropped_expired classification. Report snapshot-level schema/decode/identity
  failures separately from record-level drops.
- **D-11:** Classify a record as recovered only after final post-policy and
  post-capacity membership is known. Intersect the persisted unbroadcast set
  with final surviving members that still satisfy the local relay-intent
  contract; never infer cleared membership from admission metadata.
- **D-12:** Install the final staged state through one recovery lifecycle
  projection that rebuilds all derived indexes and evidence from canonical
  survivors. Do not advance ordinary dirty generations for replaying the exact
  durable source state; any normalization or post-recovery mutation must be
  represented explicitly.
- **D-13:** Reset rolling minimum-fee state and its decay gate to the pinned
  restart baseline. Replay must not persist, reconstruct, or accidentally bump
  a rolling floor from historical pressure; acceptance age and surviving local
  unbroadcast intent are the supported durable semantics.

### Verification and scope guardrails

- **D-14:** Lead with pure codec, dependency-sort, replay, final-membership, and
  loss-window tests, then add authority/store integration cases for concurrent
  mutation during I/O, stale and duplicate receipts, encode/write/completion
  failures, shutdown quiescence, corrupt/legacy snapshots, partial recovery,
  parent-child ordering, expiry, capacity trimming, and exact unbroadcast
  restoration. Use fake time, temporary stores, and failure injection; default
  verification remains deterministic and hermetic.
- **D-15:** Extend parity catalogs, source breadcrumbs, phase structural checks,
  and claim guardrails only for concrete Phase 135 paths. Preserve the explicit
  boundary against whole-mempool rebroadcast, public/default relay, guaranteed
  propagation, public-network CI, destructive repair, and production-readiness
  claims.

### the agent's Discretion

The planner may choose exact envelope, coordinator, trigger, evidence, prepared
recovery, and install type names; the periodic interval and smallest useful
single-flight state machine; deterministic topological tie-breaks; bounded
diagnostic retention for rejected snapshots; and module splits. Prefer small
deep modules, one atomic snapshot value unless measured evidence justifies a
multi-key manifest, and extension of Phase 134 capabilities rather than a new
authority, scheduler, or effect bus.

### Deferred Ideas (OUT OF SCOPE)

- Receive-independent initial-broadcast retry scheduling, package fanout, and
  transport completion — Phase 136.
- Broad RPC, CLI, dashboard, metrics, logs, and support-bundle rendering —
  Phase 137.
- Final adversarial parity fixtures, sustained-pressure benchmarks, release
  claim guardrails, and milestone closeout — Phase 138.
- Runtime snapshot import, operator-selectable persistence strength, automatic
  destructive repair, multi-generation manifests, incremental journals,
  durable transactional outboxes, and arbitrary historical rollback.
- Public/default relay, whole-mempool rebroadcast, guaranteed propagation,
  public-network CI, production service operation, and production readiness.
</user-constraints>

<phase-requirements>
## Phase Requirements

| ID | Description | Research Support |
| --- | --- | --- |
| MPDUR-01 | Durable snapshots preserve canonical transactions, acceptance times, and surviving local unbroadcast membership without derived state. | Use the local v2 envelope and compatibility decoder described below. [VERIFIED: .planning/REQUIREMENTS.md:53] |
| MPDUR-02 | Recovery validates and topologically replays records through current chainstate and policy, rebuilds indexes, and reports typed outcomes. | Use staged replay followed by one authoritative install. [VERIFIED: .planning/REQUIREMENTS.md:54] |
| MPDUR-03 | Rolling minimum fee resets while supported age and local-unbroadcast semantics survive. | Consume a fresh Mempool and explicitly finalize its rolling state at the restart baseline before install. [VERIFIED: .planning/REQUIREMENTS.md:55] |
| MPDUR-04 | Coalesced periodic and shutdown checkpoints expose truthful durability evidence without lock-across-I/O. | Extend Phase 134's affine prepare/receipt protocol with a shell-owned single-flight coordinator. [VERIFIED: .planning/REQUIREMENTS.md:56] |
</phase-requirements>

## Summary

The repository already has the right effect boundary but not the Phase 135 data model. Snapshot preparation is authority-bound and persistence occurs outside the mutex, yet the current record stores redundant txid, wtxid, fee, vsize, and admission metadata, captures no authoritative unbroadcast set or capture time, replays stored order directly, and folds expired records into dropped_evicted. [VERIFIED: packages/open-bitcoin-node/src/storage/mempool_snapshot.rs:13-140] [VERIFIED: packages/open-bitcoin-node/src/storage/snapshot_codec/mempool.rs:20-207] [VERIFIED: packages/open-bitcoin-node/src/network/runtime_authority/lifecycle.rs:61-126]

Use one mempool-local v2 envelope inside the existing single Fjall value, keep the global storage schema unchanged, and provide one explicit v1 compatibility decoder. Validate the full envelope before semantic replay. Stage replay in a fresh Mempool, derive all identities/resources/indexes from canonical witness transactions and current chainstate/policy, classify only final membership, reset rolling pressure, then install once through the lifecycle authority. [VERIFIED: .planning/phases/135-snapshot-schema-checkpointing-and-recovery/135-CONTEXT.md:D-01-D-13]

The checkpoint coordinator should own scheduling and one in-flight/achieved-receipt state, while the managed runtime remains the only mempool authority. A successful SyncAll write is an achieved effect even if completion dispatch fails; retain that receipt and retry idempotent completion. Stale success advances last-durable evidence but cannot clear a newer dirty generation. [VERIFIED: packages/open-bitcoin-node/src/network/lifecycle_effects.rs:18] [VERIFIED: packages/open-bitcoin-node/src/network/lifecycle_effects.rs:503-627] [VERIFIED: packages/open-bitcoin-node/src/network/runtime_authority/lifecycle.rs:105-126]

**Primary recommendation:** implement schema/codec, staged recovery, authoritative install/evidence, then the coordinator/daemon wiring in that dependency order; do not introduce a journal, actor, second authority, or derived-state persistence. [VERIFIED: .planning/phases/135-snapshot-schema-checkpointing-and-recovery/135-CONTEXT.md:D-01-D-15]

## Project Constraints (from AGENTS.md)

- Preserve externally observable behavior from the pinned Bitcoin Knots 29.3.knots20260210 baseline and record intentional differences under docs/parity. [VERIFIED: AGENTS.md]
- Keep business logic in functional-core crates and filesystem, time, scheduling, service, and Fjall effects in imperative-shell adapters. [VERIFIED: AGENTS.md] [VERIFIED: standards/core/architecture.md]
- Do not use an existing Rust Bitcoin library in the production path; reuse the repository's transaction codec and identity types. [VERIFIED: AGENTS.md]
- Rust 1.94.1 and the 2024 edition are pinned; use foo.rs plus foo/ modules, no unwrap, maybe-prefixed Option names, let-else, and tracing for logs. [VERIFIED: rust-toolchain.toml] [VERIFIED: standards/languages/rust.md]
- Add parity breadcrumbs for new first-party Rust source/test files and extend phase checkers and scripts/verify.sh for new structural claims. [VERIFIED: AGENTS.md] [VERIFIED: docs/parity/source-breadcrumbs.json]
- Run Cargo/Bazel through scripts/command-timings.ts, never overlap Cargo jobs against the same target, and use bash scripts/verify.sh as the completion contract. [VERIFIED: AGENTS.md]
- Keep broad rendering and support-bundle presentation out of this phase. [VERIFIED: .planning/phases/135-snapshot-schema-checkpointing-and-recovery/135-CONTEXT.md:D-08,D-15]

## Standard Stack

### Core

| Library / facility | Version | Purpose | Why standard |
| --- | --- | --- | --- |
| Repository transaction/codec/domain types | workspace | Canonical witness bytes, txid/wtxid derivation, admission | Required by the no-external-Bitcoin-library policy. [VERIFIED: AGENTS.md] |
| serde / serde_json | 1.0.228 / 1.0.149 | Local versioned snapshot DTO | Already locked and used by snapshot_codec. [VERIFIED: packages/Cargo.lock] |
| Fjall | 3.1.4 | Atomic single-key storage and SyncAll durability | Already locked; the adapter maps Sync to SyncAll. [VERIFIED: packages/Cargo.lock:678-681] [VERIFIED: packages/open-bitcoin-node/src/storage/fjall_store.rs:528-533] |
| std BTreeMap/BTreeSet/VecDeque | Rust 1.94.1 | Deterministic graph ordering and bounded coordinator queues | No dependency is needed for the small in-snapshot DAG. [VERIFIED: rust-toolchain.toml] |
| Phase 134 lifecycle capabilities | workspace | Generation-bound prepare, abort, receipt, completion | Existing affine effect protocol already limits pending snapshots to one. [VERIFIED: packages/open-bitcoin-node/src/network/lifecycle_effects.rs:18] |

### Supporting

| Facility | Purpose | When to use |
| --- | --- | --- |
| PolicyTime/fake time | Capture age, expiry, overdue evidence | All pure and integration timing tests. [VERIFIED: packages/open-bitcoin-node/src/mempool/pool/expiry.rs:31-118] |
| tempfile-backed Fjall tests | Reopen, corrupt bytes, persistence failure | Store and shutdown integration cases. [VERIFIED: packages/open-bitcoin-node/src/storage/fjall_store/tests/snapshot_persistence.rs:120-345] |
| Bun phase checker | Structural guard for no lock-across-I/O and source-only schema | scripts/verify.sh phase boundary. [VERIFIED: scripts/check-phase134-authoritative-lifecycle.ts] |

**Installation:** no new dependency should be installed. [VERIFIED: .planning/phases/135-snapshot-schema-checkpointing-and-recovery/135-CONTEXT.md]

## Current Implementation Seams

| Seam | Current behavior | Required Phase 135 change |
| --- | --- | --- |
| storage/mempool_snapshot.rs | Record includes redundant identities, fee/vsize, and metadata; replay is stored-order. [VERIFIED: packages/open-bitcoin-node/src/storage/mempool_snapshot.rs:13-107] | Define source-only envelope and stable snapshot/record failure classes; move replay orchestration out of the persistence value. |
| snapshot_codec/mempool.rs | Witness bytes are canonical and stored identities are checked, but fee/vsize are trusted after decode; all-absent legacy metadata becomes LegacyUnknown. [VERIFIED: packages/open-bitcoin-node/src/storage/snapshot_codec/mempool.rs:97-207] | Add a local v2 format discriminator; decode missing discriminator as explicit v1; verify compatibility fields but never carry them as authority. |
| snapshot_codec.rs | The generic global SchemaVersion is currently 1 and rejects any other global version. [VERIFIED: packages/open-bitcoin-node/src/storage/snapshot_codec.rs:33-37] [VERIFIED: packages/open-bitcoin-node/src/storage/snapshot_codec.rs:214-244] | Keep global version 1; version the mempool payload locally so unrelated stores do not migrate. |
| fjall_store/mempool.rs | One SNAPSHOT_KEY is put atomically; encode/save failures abort; a completion error discards the achieved receipt from the return type. [VERIFIED: packages/open-bitcoin-node/src/storage/fjall_store/mempool.rs:75-174] | Return an achieved receipt separately from completion status so dispatch failure cannot trigger abort/rewrite. |
| runtime_authority/lifecycle.rs | Capture reserves current generation from mempool alone; completion clears dirty only if fresh, but tracks no last-durable evidence. [VERIFIED: packages/open-bitcoin-node/src/network/runtime_authority/lifecycle.rs:61-126] | Capture mempool plus exact unbroadcast plus time under one guard; add checkpoint evidence and stale-success advancement. |
| network/recovery.rs | Mutates the live network sequentially while the authority mutex remains held. [VERIFIED: packages/open-bitcoin-node/src/network/recovery.rs:80-160] [VERIFIED: packages/open-bitcoin-node/src/network/runtime_authority.rs:558-566] | Build an opaque PreparedMempoolRecovery outside the guard and install it in one short authority command. |
| lifecycle projection | Authority owns dirty generation and exact unbroadcast state across seven projections. [VERIFIED: packages/open-bitcoin-node/src/network/lifecycle_projection/authority.rs:40-145] | Add one startup-only recovery projection that replaces canonical mempool state and rebuilds all seven derived projections. |
| open-bitcoind shutdown | HTTP, inbound listener, metrics, and sync worker are stopped, but no forced mempool checkpoint is performed. [VERIFIED: packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs:80-126] | Quiesce all mutation producers, settle/force checkpoint, then and only then mark mempool shutdown clean. |

## Architecture Patterns

### Recommended Project Structure

    packages/open-bitcoin-node/src/storage/mempool_snapshot.rs
    packages/open-bitcoin-node/src/storage/mempool_snapshot/codec.rs
    packages/open-bitcoin-node/src/network/recovery.rs
    packages/open-bitcoin-node/src/network/recovery/topology.rs
    packages/open-bitcoin-node/src/network/recovery/staging.rs
    packages/open-bitcoin-node/src/network/checkpoint.rs
    packages/open-bitcoin-rpc/src/bin/open_bitcoind/checkpoint.rs
    scripts/check-phase135-snapshot-recovery.ts

This split keeps pure envelope/topology/staging logic separate from runtime authority, storage, timer, and daemon shutdown effects while following the repository's foo.rs plus foo/ convention. [VERIFIED: standards/core/architecture.md] [VERIFIED: standards/languages/rust.md]

### Pattern 1: Mempool-local source envelope

The v2 payload should contain format_version, captured_generation, captured_at, records of canonical Transaction plus Known acceptance time, and a BTreeSet of canonical unbroadcast member identities. It must not contain fee, vsize, topology, rolling fee, serving/fanout/peer/compact state, or origin/relay metadata as authority. [VERIFIED: .planning/phases/135-snapshot-schema-checkpointing-and-recovery/135-CONTEXT.md:D-01-D-03]

Decode in two phases: preflight byte-size bound; then deserialize a local tagged payload, validate version/count/time/identity/unbroadcast invariants, and only then expose a domain envelope. Preserve rejected bytes and return a typed snapshot-level error. [CITED: https://cornucopia.owasp.org/taxonomy/asvs-5.0/01-encoding-and-sanitization/05-safe-deserialization] [VERIFIED: .planning/phases/135-snapshot-schema-checkpointing-and-recovery/135-CONTEXT.md:D-04]

For v1, verify stored txid/wtxid against decoded witness bytes, ignore/recompute fee and vsize, accept only Known acceptance time, and restore an empty unbroadcast set because v1 cannot prove current membership. [VERIFIED: packages/open-bitcoin-node/src/storage/snapshot_codec/mempool.rs:117-207] [VERIFIED: .planning/phases/135-snapshot-schema-checkpointing-and-recovery/135-CONTEXT.md:D-02-D-03]

### Pattern 2: Deterministic staged recovery

Build dependencies from transaction inputs that reference another snapshot record. Use Kahn ordering with a BTreeSet ready queue keyed by txid then wtxid; propagate a failed/missing parent to descendants while allowing independent components to continue. [VERIFIED: .planning/phases/135-snapshot-schema-checkpointing-and-recovery/135-CONTEXT.md:D-09-D-10]

Replay into Mempool::new(current_config), using preserved acceptance times and current chainstate/consensus/effective policy. Exact duplicate records are classified deterministically; entries absent after final capacity enforcement become dropped_evicted, not recovered. Expiry is distinct and runs with injected startup time. [VERIFIED: packages/open-bitcoin-node/src/mempool/pool.rs:122-174] [VERIFIED: packages/open-bitcoin-node/src/mempool/pool/expiry.rs:31-118] [VERIFIED: .planning/phases/135-snapshot-schema-checkpointing-and-recovery/135-CONTEXT.md:D-09-D-11]

Finalize the staged pool by resetting all rolling-fee fields and the decay gate to the same state as a fresh Mempool; do not merely set the numeric floor because the gate/time fields also affect decay. [VERIFIED: packages/open-bitcoin-node/src/mempool/fee/rolling.rs:40-49] [VERIFIED: packages/open-bitcoin-node/src/mempool/pool/tests/sustained_pressure_cases.rs:306-314]

Restore unbroadcast as persisted identities intersected with final survivors and represented as local requested intent. All other recovered records use recovery-unknown/non-requested provenance; never derive restored membership from historical metadata or the current reconciliation helper. [VERIFIED: packages/open-bitcoin-node/src/network/lifecycle_projection/reconciliation.rs:151-174] [VERIFIED: .planning/phases/135-snapshot-schema-checkpointing-and-recovery/135-CONTEXT.md:D-11]

### Pattern 3: One authoritative install

PreparedMempoolRecovery should own the complete staged Mempool, final per-record outcomes, exact surviving unbroadcast set, captured generation, and rebuilt projection inputs. The authority command consumes it only during startup, replaces state atomically, rebuilds serving/fanout/peer/compact/evidence projections from final members, and exposes no partial cache state. [VERIFIED: packages/open-bitcoin-node/src/network/lifecycle_projection/authority.rs:204-265] [VERIFIED: .planning/phases/135-snapshot-schema-checkpointing-and-recovery/135-CONTEXT.md:D-12]

Install captured_generation as both current and last-durable, leave dirty_generation empty for an exact replay, and ensure the next ordinary mutation advances from that generation. Any normalization that changes durable source truth must explicitly create a new dirty generation. [VERIFIED: .planning/phases/135-snapshot-schema-checkpointing-and-recovery/135-CONTEXT.md:D-08,D-12]

### Pattern 4: Single-flight checkpoint state machine

Use states Idle, Persisting(prepared), and AchievedAwaitingCompletion(receipt). A periodic clean tick records skipped-clean. A dirty tick prepares under authority, releases the guard, encodes and calls Fjall SyncAll, then completes. New mutations during Persisting set only the authority's newer dirty generation and cause one follow-up after completion. [VERIFIED: packages/open-bitcoin-node/src/network/lifecycle_effects.rs:18] [VERIFIED: packages/open-bitcoin-node/src/network/runtime_authority/effects.rs:50-157]

On encode/storage failure, exact-abort the unachieved capability and retain dirty state. Once storage succeeds, never call abort: retain the achieved receipt across dispatch failure and retry completion idempotently before starting another write. Duplicate completion must not regress evidence. [VERIFIED: packages/open-bitcoin-node/src/storage/fjall_store/mempool.rs:133-174] [VERIFIED: packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/effects.rs:437-602]

Map the locked Sync strength to Fjall SyncAll. Buffer only reaches OS buffers and does not cover OS/power loss; SyncAll uses fsync. [VERIFIED: packages/open-bitcoin-node/src/storage/fjall_store.rs:528-533] [CITED: https://docs.rs/fjall/latest/fjall/enum.PersistMode.html]

Checkpoint evidence belongs to the authority and contains current, optional dirty/in-flight/last-durable generations, captured/completed times, trigger, strength, outcome/failure, and overdue. Loss truth is the generation range (last durable, current]; age saturates at zero if wall time moves backwards and is measured from capture, not completion. [VERIFIED: .planning/phases/135-snapshot-schema-checkpointing-and-recovery/135-CONTEXT.md:D-08]

### Pattern 5: Shutdown ordering

Stop HTTP admission and every mutation producer, stop inbound/sync workers, settle any achieved receipt, then force Sync checkpoint(s) until last_durable equals current. Only after that may clean-shutdown metadata be written; a failure leaves recovery required and returns/logs a typed shutdown failure. [VERIFIED: packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs:80-126] [VERIFIED: packages/open-bitcoin-node/src/storage/fjall_store.rs:405-410] [VERIFIED: .planning/phases/135-snapshot-schema-checkpointing-and-recovery/135-CONTEXT.md:D-06]

## Recommended Plan Decomposition

| Wave | Plan | Scope and proof |
| --- | --- | --- |
| 1 | 135-01 Source schema and codec | Add v2 local envelope, strict invariants/limits, explicit v1 migration, corrupt/unsupported/identity tests. No authority changes. |
| 1 | 135-02 Pure topology and recovery staging | Add deterministic dependency sort, independent-component salvage, typed outcomes, final-membership accounting, exact unbroadcast intersection, and rolling reset tests. |
| 2 | 135-03 Authoritative capture/install/evidence | Capture records plus unbroadcast/time/gen under one guard; add PreparedMempoolRecovery install; rebuild projections once; add generation/loss-window tests. Depends on 01/02. |
| 3 | 135-04 Store execution and coordinator | Preserve achieved receipts, implement single-flight/coalescing and Sync-only execution; inject encode/write/completion failures. Depends on 03. |
| 4 | 135-05 Daemon periodic and shutdown wiring | Drive ticks, quiesce producers, force exact-current checkpoint, and prevent false clean claims. Depends on 04. |
| 5 | 135-06 Parity and verification guardrails | Add breadcrumbs, parity entries, mutation-tested phase checker, verify.sh registration, focused tests, and full repository verification. |

The first two plans are independent pure-domain work and may run in parallel; all later plans should remain ordered because they consume the types and invariants established upstream. [VERIFIED: .planning/phases/135-snapshot-schema-checkpointing-and-recovery/135-CONTEXT.md:D-14]

## Don't Hand-Roll

| Problem | Don't build | Use instead | Why |
| --- | --- | --- | --- |
| Durable atomic replacement | Journal, manifest, outbox, temporary rename layer | Existing Fjall single-key put plus SyncAll | The store already owns journaling/fsync and the phase explicitly prefers one atomic value. [VERIFIED: packages/open-bitcoin-node/src/storage/fjall_store/mempool.rs:75-125] |
| Transaction identity | Parallel hash/codec implementation | Repository canonical witness codec and txid/wtxid derivation | Compatibility fields must be verified against canonical bytes. [VERIFIED: packages/open-bitcoin-node/src/storage/snapshot_codec/mempool.rs:97-155] |
| Runtime authority | Actor, mutex, scheduler, or effect bus | ManagedNetworkHandle plus Phase 134 capability ledger | A second authority would split dirty/unbroadcast truth. [VERIFIED: packages/open-bitcoin-node/src/network/runtime_authority/lifecycle.rs] |
| Recovery graph dependency | Graph crate | BTree collections and a focused Kahn sorter | The graph is startup-local and deterministic; a dependency adds no required capability. [VERIFIED: AGENTS.md] |
| Rolling-floor reconstruction | Infer historical pressure from entries | Fresh Mempool baseline/finalizer | Knots/Open Bitcoin rolling pressure is runtime state, not durable source state. [VERIFIED: packages/open-bitcoin-node/src/mempool/fee/rolling.rs:40-49] |

## Common Pitfalls and Threat Model

| Threat / pitfall | Failure | Required mitigation and verification |
| --- | --- | --- |
| Schema corruption or oversized JSON | Panic, allocation abuse, partial/default recovery, or overwrite of evidence | Bound bytes before serde, validate counts/identities/times, return typed snapshot failure, retain original bytes, and test corrupt/truncated/unknown-version/oversized cases. [CITED: https://cornucopia.owasp.org/taxonomy/asvs-5.0/01-encoding-and-sanitization/05-safe-deserialization] |
| Trusting v1 fee/vsize/identity | Derived state diverges from transaction bytes/current chainstate | Verify identity compatibility fields and recompute all other values through current admission. [VERIFIED: packages/open-bitcoin-node/src/storage/snapshot_codec/mempool.rs:117-155] |
| Stored-order replay | Child appears before parent and is falsely dropped | Pure deterministic topology sort and explicit descendant propagation tests. [VERIFIED: packages/open-bitcoin-node/src/storage/mempool_snapshot.rs:76-107] |
| Premature recovered count | An initially accepted entry is later capacity-trimmed | Derive all outcome totals only after final staged membership. [VERIFIED: .planning/phases/135-snapshot-schema-checkpointing-and-recovery/135-CONTEXT.md:D-11] |
| Expiry collapsed into eviction | Operator evidence cannot distinguish age policy from pressure | Add dropped_expired and test exact cutoff with fake time. [VERIFIED: packages/open-bitcoin-node/src/storage/mempool_snapshot.rs:125-140] |
| Unbroadcast resurrection | Historical Local/Requested metadata re-adds already-cleared intent | Restore only persisted authoritative identities intersected with final survivors; legacy restores none. [VERIFIED: .planning/phases/135-snapshot-schema-checkpointing-and-recovery/135-CONTEXT.md:D-03,D-11] |
| Stale or duplicate receipt | A late completion clears newer dirty work or regresses evidence | Bind epoch/generation/effect identity; stale advances durable high-water only; exact current alone clears; duplicate is idempotent. [VERIFIED: packages/open-bitcoin-node/src/network/lifecycle_effects.rs:503-627] |
| Completion dispatch loss | Disk effect succeeded but coordinator aborts/repeats it as unachieved | Transition to AchievedAwaitingCompletion immediately after SyncAll and retain the receipt until idempotent completion succeeds. [VERIFIED: .planning/phases/135-snapshot-schema-checkpointing-and-recovery/135-CONTEXT.md:D-07] |
| False crash-loss window | Evidence reports fixed time while I/O is pending/failing | Report exact generation interval; mark overdue/pending/failure and make no bounded wall-clock claim. [VERIFIED: .planning/phases/135-snapshot-schema-checkpointing-and-recovery/135-CONTEXT.md:D-08] |
| Lock across encoding/I/O | Admission and network lifecycle block on snapshot size/fsync | Capture under the authority, then enforce encode/store/receipt dispatch outside it with structural checker and concurrent mutation test. [VERIFIED: packages/open-bitcoin-node/src/network/runtime_authority/effects.rs:50-157] |
| Replay bumps rolling floor | Restart behavior depends on historical replay order/pressure | Consume a fresh staged pool and finalize all rolling fields to restart baseline after capacity decisions. [VERIFIED: packages/open-bitcoin-node/src/mempool/pool/tests/sustained_pressure_cases.rs:306-314] |
| False clean shutdown | Service marks clean after a failed/stale checkpoint | Quiesce first; require last_durable equals current after SyncAll before writing clean marker. [VERIFIED: .planning/phases/135-snapshot-schema-checkpointing-and-recovery/135-CONTEXT.md:D-06] |

## Pinned Bitcoin Knots Anchors

- DumpMempool copies entries and the unbroadcast set together under the mempool lock, releases the lock before file I/O, writes witness transactions with entry time, commits, and atomically renames the new file. [VERIFIED: packages/bitcoin-knots/src/node/mempool_persist.cpp:191-307]
- LoadMempool preserves entry time, uses current admission policy, counts accepted/already/failure/expired separately, and restores unbroadcast entries only if their transactions survived in the mempool. [VERIFIED: packages/bitcoin-knots/src/node/mempool_persist.cpp:77-188]
- Knots does not serialize rolling minimum-fee fields in the mempool dump; those fields are runtime-owned in CTxMemPool. [VERIFIED: packages/bitcoin-knots/src/node/mempool_persist.cpp:191-307] [VERIFIED: packages/bitcoin-knots/src/txmempool.h:327-329]
- Functional coverage asserts acceptance-time persistence and unbroadcast survival across restart. [VERIFIED: packages/bitcoin-knots/test/functional/mempool_persist.py:99-134] [VERIFIED: packages/bitcoin-knots/test/functional/mempool_persist.py:202-223]
- Open Bitcoin's periodic Sync checkpoint and typed generation receipt are project-strengthened durability semantics, not claims that Knots has the same scheduler/evidence model. [VERIFIED: packages/bitcoin-knots/src/node/mempool_persist.cpp:191-307] [VERIFIED: .planning/phases/135-snapshot-schema-checkpointing-and-recovery/135-CONTEXT.md:D-05-D-08]

## Code Examples

### Coordinator transition sketch

    match coordinator_state {
        Idle if trigger.is_periodic() && authority.is_clean() => SkippedClean,
        Idle => prepare_under_authority_then_execute_without_guard(),
        Persisting(_) => Coalesced,
        AchievedAwaitingCompletion(receipt) => retry_idempotent_completion(receipt),
    }

This is an illustrative state contract, not a proposed second authority: the coordinator owns only scheduling/effect progress, while current/dirty/unbroadcast/evidence truth remains inside ManagedNetworkHandle. [VERIFIED: .planning/phases/135-snapshot-schema-checkpointing-and-recovery/135-CONTEXT.md:D-05-D-08]

### Final outcome sketch

    recovered = accepted_ids intersect final_member_ids
    dropped_evicted = accepted_ids minus final_member_ids
    restored_unbroadcast =
        persisted_unbroadcast intersect final_member_ids intersect local_intent_ids

Compute these sets after replay and capacity enforcement; do not increment recovered during per-record admission. [VERIFIED: .planning/phases/135-snapshot-schema-checkpointing-and-recovery/135-CONTEXT.md:D-10-D-11]

## Validation Architecture

### Test Framework

| Property | Value |
| --- | --- |
| Framework | Rust built-in test harness, Bun TypeScript structural checker, Fjall temp stores [VERIFIED: packages/Cargo.toml] |
| Config | packages/Cargo.toml workspace plus scripts/verify.sh [VERIFIED: scripts/verify.sh] |
| Quick codec | bun run scripts/command-timings.ts run --key phase135-codec -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-features snapshot_codec |
| Quick recovery | bun run scripts/command-timings.ts run --key phase135-recovery -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-features recovery_cases |
| Quick daemon | bun run scripts/command-timings.ts run --key phase135-daemon -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --all-features --bin open-bitcoind checkpoint |
| Structural | bun scripts/check-phase135-snapshot-recovery.ts |
| Full suite | bash scripts/verify.sh |

The Validation Architecture section is included despite workflow.nyquist_validation=false because the phase prompt explicitly requires it and D-14 locks test-first coverage. [VERIFIED: .planning/config.json] [VERIFIED: .planning/phases/135-snapshot-schema-checkpointing-and-recovery/135-CONTEXT.md:D-14]

### Phase Requirements to Test Map

| Req ID | Behavior | Test type | Automated proof | File status |
| --- | --- | --- | --- | --- |
| MPDUR-01 | v2 round trip, v1 migration, source-only fields, exact unbroadcast, corrupt/unknown/identity failure | unit + store integration | phase135-codec plus snapshot persistence tests | Wave 0 additions required |
| MPDUR-02 | topology order, independent salvage, missing descendants, current-policy drops, final capacity classification, one install | unit + authority integration | phase135-recovery | Wave 0 additions required |
| MPDUR-03 | known ages survive; legacy unknown is not freshened; expiry distinct; rolling rate/gate/time reset | unit + restart integration | phase135-recovery | Wave 0 additions required |
| MPDUR-04 | clean skip, coalescing, concurrent mutation, stale/duplicate receipt, all failure prefixes, forced shutdown, evidence/loss interval | state-machine + Fjall + daemon integration | phase135-daemon plus structural checker | Wave 0 additions required |

### Sampling Rate

- Per task commit: run the affected quick command and the Phase 135 structural checker. [VERIFIED: standards/core/verification.md]
- Per wave merge: run all three quick commands plus the checker. [VERIFIED: standards/core/verification.md]
- Phase gate: bash scripts/verify.sh must pass, including mutation-tested checker registration and parity breadcrumbs. [VERIFIED: AGENTS.md]

### Wave 0 Gaps

- Extend storage/snapshot_codec/tests.rs with v2, v1, malformed, oversized, identity, and exact-unbroadcast fixtures.
- Extend network/tests/recovery_cases.rs with topology, partial component, expiry, capacity, rolling reset, and final-unbroadcast fixtures.
- Extend network/tests/lifecycle_projection_cases/effects.rs with achieved-receipt dispatch failure, stale durable high-water, coalescing, and loss-range fixtures.
- Extend storage/fjall_store/tests/snapshot_persistence.rs with Sync/reopen and retained-byte corruption cases.
- Add open_bitcoind checkpoint tests with fake time and producer-quiescence barriers.
- Add scripts/check-phase135-snapshot-recovery.ts and register both positive and mutation checks in scripts/verify.sh.

All gaps are new Phase 135 coverage; the underlying Rust/Bun/temp-store infrastructure already exists. [VERIFIED: packages/open-bitcoin-node/src/storage/snapshot_codec/tests.rs] [VERIFIED: packages/open-bitcoin-node/src/storage/fjall_store/tests/snapshot_persistence.rs] [VERIFIED: scripts/verify.sh]

## Security Domain

### Applicable ASVS Categories

| Category | Applies | Standard control |
| --- | --- | --- |
| Authentication/session/access control | No | Snapshot recovery is a startup-local internal path; it must not create a runtime import surface in this phase. [VERIFIED: .planning/phases/135-snapshot-schema-checkpointing-and-recovery/135-CONTEXT.md:Deferred-Ideas] |
| Input validation and safe deserialization | Yes | Bound the byte input, accept only explicit versions/shapes, validate domain invariants, fail closed at snapshot level. [CITED: https://cornucopia.owasp.org/taxonomy/asvs-5.0/01-encoding-and-sanitization/05-safe-deserialization] |
| Cryptography/integrity | Yes, existing primitive only | Recompute txid/wtxid through repository canonical hashing; do not add a custom checksum/crypto format. [VERIFIED: packages/open-bitcoin-node/src/storage/snapshot_codec/mempool.rs:117-155] |
| Data protection/operability | Yes | Do not log raw transaction payloads or rejected bytes; retain bounded low-cardinality failure evidence and leave stored bytes untouched. [CITED: https://cornucopia.owasp.org/taxonomy/asvs-5.0/14-data-protection/01-data-protection-documentation] |

### STRIDE-Oriented Threats

| Pattern | STRIDE | Mitigation |
| --- | --- | --- |
| Edited/corrupt snapshot changes identities or shape | Tampering | Strict version/identity/invariant validation; no best-effort/default decode. |
| Duplicate/stale receipt changes dirty truth | Tampering / Repudiation | Epoch, generation, effect ID, snapshot identity, idempotent completion ledger. |
| Unbroadcast metadata resurrects cleared intent | Elevation of behavior | Restore only exact persisted authoritative set intersected with final members. |
| Huge/cyclic/dependency-hostile snapshot | Denial of service | Preflight size/count limits, bounded deterministic graph processing, cycle/missing-parent typed drops. |
| Raw transaction/rejected bytes in logs | Information disclosure | Low-cardinality classes/counts/timestamps only; no raw bytes. |
| Authority held over fsync | Denial of service | Prepare and install are short critical sections; all encode/I/O outside guard, structurally checked. |

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
| --- | --- | --- | --- | --- |
| rustc | Rust tests/build | yes | 1.94.1 | none |
| cargo | Workspace tests | yes | 1.94.1 | none |
| Bun | Phase checker/timing wrapper | yes | 1.3.9 | none |
| Bazelisk/Bazel | Repository smoke build | yes | 1.28.1 / 8.6.0 | none |
| Fjall | Embedded snapshot storage | yes | locked crate 3.1.4 | none |
| Bitcoin Knots submodule | Pinned parity anchors | yes | v29.3.knots20260210 | none |

The phase has no external database or service dependency; Fjall is embedded and all required local toolchain components were probed in this checkout. [VERIFIED: local command availability audit 2026-08-02] [VERIFIED: packages/Cargo.lock:678-681]

## State of the Art

| Current approach | Phase 135 approach | Impact |
| --- | --- | --- |
| Global schema version only | Global v1 wrapper plus mempool-local v2 discriminator | Unrelated snapshots avoid forced migration. [VERIFIED: packages/open-bitcoin-node/src/storage/snapshot_codec.rs:214-244] |
| Stored-order live replay | Deterministic staged dependency replay | Parent ordering and partial independent recovery become explicit. [VERIFIED: packages/open-bitcoin-node/src/storage/mempool_snapshot.rs:76-107] |
| Recovered counted during admission | Outcomes derived from final membership | Capacity-trimmed entries cannot be falsely reported recovered. [VERIFIED: .planning/phases/135-snapshot-schema-checkpointing-and-recovery/135-CONTEXT.md:D-11] |
| Save helper loses receipt on completion error | Achieved receipt retained until idempotent completion | Successful disk effects remain truthful across dispatch failure. [VERIFIED: packages/open-bitcoin-node/src/storage/fjall_store/mempool.rs:133-174] |
| Shutdown has no forced mempool checkpoint | Quiesced exact-current Sync checkpoint | A clean claim has an auditable durability boundary. [VERIFIED: packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs:80-126] |

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
| --- | --- | --- | --- |
| A1 | Use a private five-minute periodic default initially, with fake-time tests and no public knob. [ASSUMED] | Checkpoint coordinator | Low: affects write frequency and expected freshness, not correctness; planner may choose another bounded internal interval under D-05/D-08. |

All architecture, compatibility, and durability recommendations are source-verified; only the exact internal periodic interval remains discretionary. [VERIFIED: .planning/phases/135-snapshot-schema-checkpointing-and-recovery/135-CONTEXT.md:the-agent's-Discretion]

## Open Questions (RESOLVED)

1. **Internal periodic interval**
   - What is known: it is planner discretion; all successful periodic writes must be Sync and evidence cannot claim a bound while pending/failing. [VERIFIED: .planning/phases/135-snapshot-schema-checkpointing-and-recovery/135-CONTEXT.md:D-06,D-08]
   - Resolution: Plan 06 selects a private 300-second constant, fake-time tests it, and leaves operator configurability out of scope. [RESOLVED: .planning/phases/135-snapshot-schema-checkpointing-and-recovery/135-06-PLAN.md]

No other user decision is required before planning. [VERIFIED: .planning/phases/135-snapshot-schema-checkpointing-and-recovery/135-CONTEXT.md]

## Sources

### Primary (HIGH confidence)

- AGENTS.md, AGENTS.bright-builds.md, standards/core/architecture.md, standards/core/testing.md, standards/core/verification.md, standards/languages/rust.md.
- Phase 135 CONTEXT.md, REQUIREMENTS.md, ROADMAP.md, milestone ARCHITECTURE.md and PITFALLS.md.
- Open Bitcoin mempool snapshot codec/store, runtime authority/effect ledger, lifecycle projection, recovery, rolling fee/expiry, daemon, and tests cited inline.
- Pinned Knots node/mempool_persist.cpp, txmempool.h/cpp, and functional mempool_persist.py cited inline.
- Fjall 3.1.4 local crate source and locked Cargo metadata.

### Secondary (MEDIUM confidence)

- https://docs.rs/fjall/latest/fjall/enum.PersistMode.html — persistence mode semantics; cross-checked against locked 3.1.4 source.
- https://owasp.org/www-project-application-security-verification-standard/ — ASVS 5.0.0 release and identifier convention.
- https://cornucopia.owasp.org/taxonomy/asvs-5.0/01-encoding-and-sanitization/05-safe-deserialization — safe deserialization controls.
- https://cornucopia.owasp.org/taxonomy/asvs-5.0/14-data-protection/01-data-protection-documentation — integrity/logging protection considerations.

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH — every component is already pinned and used in the repository.
- Architecture: HIGH — based on current source seams, locked Phase 134/135 decisions, and pinned Knots anchors.
- Pitfalls: HIGH — each critical failure has a current-code seam or locked requirement and an explicit test.
- Periodic interval: LOW — policy choice deliberately left to planner discretion.

**Research date:** 2026-08-02
**Valid until:** 2026-09-01
