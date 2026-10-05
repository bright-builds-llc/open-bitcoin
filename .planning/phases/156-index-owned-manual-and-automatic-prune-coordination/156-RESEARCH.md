---
generated_by: gsd-phase-researcher
lifecycle_mode: yolo
phase_lifecycle_id: 156-2026-10-04T20-28-36
generated_at: 2026-10-04T20:38:24Z
---

# Phase 156: Index-Owned Manual and Automatic Prune Coordination - Research

**Researched:** 2026-10-04
**Domain:** BASIC index ownership, durable lifecycle and actual Fjall prune gates
**Confidence:** HIGH for existing seams; MEDIUM for proposed implementation until behavioral verification

<user-constraints>

## User Constraints (from CONTEXT.md)

The following decision, discretion and deferred text is copied verbatim from Phase 156 CONTEXT.md. [VERIFIED: .planning/phases/156-index-owned-manual-and-automatic-prune-coordination/156-CONTEXT.md]

### Locked Decisions

### Reserved ownership

- **D-01:** Reserve the existing BASIC index lock identity across authoritative operator set/clear, store lock-map replacement and concrete deletion paths. Authenticated operators cannot create, replace, clear or weaken index-owned protection; ordinary unrelated named locks continue to work.
- **D-02:** Use explicit internal lifecycle publication for owner mutations; do not infer authorization from a caller-provided name or allow public whole-map replacement to omit the owned lock.

### Apply-time protection and automatic retention

- **D-03:** Reload and validate durable index state and protection at actual manual/automatic application, including intent creation, paired deletion and resumed deletion. Planner snapshots alone are insufficient. Refuse corrupt or inconsistent ownership before payload mutation.
- **D-04:** A stalled or failed index keeps all required bodies and non-genesis undo, even when the logical soft retention target is unattainable. Lock/protection transitions invalidate cached automatic measurements and bypass stale periodic throttling where necessary for a fresh decision.

### Durable release ordering

- **D-05:** Relax protection only when immutable BASIC records and a compatible recoverable coins/chain-metadata checkpoint prove the released prefix. Persist proof before relaxation, using the existing atomic SyncAll publisher when practical. Counters, header tip and ahead filter rows are insufficient authority.
- **D-06:** Checkpoint/protection write failures retain conservative protection and explicit failure evidence. Preserve immutable rows, valid prefixes and the existing finish-or-Repair refusal; do not repair, redownload or erase history.

### Disable and re-enable

- **D-07:** An explicit disable transition stops index work and invalidates in-flight work before releasing the owned lock. Retain immutable records and checkpoint; failure may retain extra protection, never let stale work recreate/advance active ownership after disable.
- **D-08:** Re-enable establishes conservative durable protection before new work or dependent deletion can proceed. Use explicit lifecycle/generation identity for stale-work refusal; do not ship public configuration activation or scheduled catch-up in this phase.

### Evidence and simplicity

- **D-09:** Prove all four roadmap criteria with real Fjall close/reopen, production runtime/authority callers and manual/ordinary automatic deletion. Include preplanned/stale work, bypass attempts and checkpoint/protection/disable persistence failures. Source-string guards and memory fixtures supplement behavioral evidence.
- **D-10:** Keep policy/transitions pure and adapters thin, reuse existing ownership, checkpoint and shared-store synchronization seams, add no production dependency or crate, register Rust source breadcrumbs, update parity/docs and run the full native verifier before commit/push. Perform an explicit simplification review.

### the agent's Discretion

Exact internal capability and generation types, publication guard design, fault seams, plan decomposition and bounded test fixture details are researcher/planner choices. Preserve unrelated operator lock behavior and the retained v2.4 advisories unless this safety contract requires a narrow change.

### Deferred Ideas (OUT OF SCOPE)

Public BASIC configuration and history activation preflight, scheduled catch-up (157), runtime reorg (158), RPC (159), peers (160), operator projections (161) and complete post-prune client proof (162). No pending todos matched this phase. Unrelated v2.4 advisory cleanup is excluded.

</user-constraints>

<phase-requirements>

## Phase Requirements

| ID      | Description                                                                                                                                                                                                                                                                         | Research Support                                                                                                                                                |
| ------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| CFPR-01 | An active index protects all required body/undo inputs from both manual and ordinary automatic pruning until its safe durable checkpoint permits release; operator lock set/clear cannot weaken index-owned protection, and disable/re-enable has an explicit ownership transition. | Authoritative reserved CRUD, store-owned deletion check, coins-fenced publication, lifecycle generations, cache invalidation and real-store fault matrix below. |

Requirement wording and sole Phase 156 ownership are verified in REQUIREMENTS.md and ROADMAP.md. [VERIFIED: .planning/REQUIREMENTS.md; .planning/ROADMAP.md]

</phase-requirements>

## Summary

Phase 155 already supplies immutable BASIC rows, a distinct active projection/checkpoint, a verified recovered-coins/metadata fence, atomic SyncAll checkpoint/protection publication and mandatory pre-resume startup validation. Phase 156 must close ownership and application holes around these implemented assets. The public store lock-map replacement has serialization but no reserved-owner policy; handle replace/clear accept arbitrary names; concrete paired deletion does not reload index ownership. [VERIFIED: packages/open-bitcoin-node/src/storage/fjall_store/filters/publication.rs; packages/open-bitcoin-node/src/storage/fjall_store/filters/startup.rs; packages/open-bitcoin-node/src/storage/fjall_store/prune/records.rs; packages/open-bitcoin-node/src/network/runtime_authority/prune_flush.rs; packages/open-bitcoin-node/src/storage/fjall_store/prune.rs]

The hardest compatibility issue is an explicitly disabled saved index: existing startup treats any saved state as protected and rejects a missing reserved lock. Existing v1 state stores endpoint, fence and protection, but no enabled/disabled state or work generation. Add a bounded additive ownership envelope and lifecycle-aware validation instead of deleting saved state to mean disabled. Keep public activation and scheduling out of this phase. [VERIFIED: packages/open-bitcoin-node/src/storage/filter_index.rs; packages/open-bitcoin-node/src/storage/fjall_store/filters/startup.rs; Phase 156 CONTEXT.md D-07/D-08]

**Primary recommendation:** Use one pure ownership model, one shared-store guard and one internal capability-bearing lifecycle publisher; enforce that model below the existing manual, automatic and resumed deletion owners. This is a proposed implementation, grounded in the inspected existing seams and locked decisions, not a claim that it is implemented. [VERIFIED: Phase 156 CONTEXT.md D-01..D-10; packages/open-bitcoin-node/src/storage/fjall_store.rs]

## Project Constraints (from AGENTS.md)

- Preserve pinned Knots observable behavior and auditable parity; no production Rust Bitcoin dependency, new production crate, second chainstate, GUI or implied repair. [VERIFIED: AGENTS.md Project/Constraints; .planning/REQUIREMENTS.md]
- Keep policy and transitions I/O-free in first-party chainstate; storage/runtime adapters own effects. Use typed invariants, early guards, internal optional names with `maybe_`, `foo.rs` plus `foo/`, and Arrange/Act/Assert behavior tests. [VERIFIED: AGENTS.md; standards/core/architecture.md; standards/core/code-shape.md; standards/core/testing.md; standards/languages/rust.md]
- Work stays in the active GSD lifecycle. Root owns Git finalization; this strict wrapper defers every commit until clean phase verification and the full native gate. Do not commit this research independently despite `commit_docs: true`. [VERIFIED: AGENTS.md GSD Workflow Enforcement; parent task authorization; Phase 153 CONTEXT.md D-10]
- Use `bash scripts/verify.sh` before finalization, including Bazel, coverage, managed checks and generated LOC freshness. Run ad-hoc Cargo/Bazel through `bun run scripts/command-timings.ts run --key <stable-key> -- <command>`; avoid overlapping Cargo jobs. Poll resumable sessions at least every 60 seconds and do not terminate merely for elapsed estimates. [VERIFIED: AGENTS.md Repo-Local Guidance; scripts/verify.sh]
- Register new Rust source/test breadcrumb entries, update parity documentation and relevant READMEs after substantial scope changes; preserve historical phase evidence and unrelated v2.4 advisories. [VERIFIED: AGENTS.md; .planning/milestones/v2.4-MILESTONE-AUDIT.md; Phase 156 CONTEXT.md]
- Read local instructions, sidecar, overrides and relevant managed standards before substantive work. The override file contains only placeholders; no active project skill files were found at the checked `.claude/skills` and `.agents/skills` paths. Both active lesson files were read fully during the scout. [VERIFIED: AGENTS.md; AGENTS.bright-builds.md; standards-overrides.md; filesystem probes; active lesson reads]

## Standard Stack

### Core

| Component                              | Selected version         | Purpose                                                      | Evidence                                                          |
| -------------------------------------- | ------------------------ | ------------------------------------------------------------ | ----------------------------------------------------------------- |
| Rust / standard library                | 1.94.1, edition 2024     | Typed policy, checked generations, shared Mutex/Arc          | [VERIFIED: rust-toolchain.toml; packages/Cargo.toml; rustc probe] |
| Fjall                                  | 3.1.4, existing lockfile | Existing same-database atomic batches                        | [VERIFIED: packages/Cargo.lock; local fjall-3.1.4 source]         |
| First-party chainstate/node/RPC crates | Existing workspace       | Pure ownership, shell adapters, authenticated operator tests | [VERIFIED: packages/Cargo.toml; implementation paths below]       |
| Bun                                    | Repo pin 1.3.9           | Timing, native checkers and mutation tests                   | [VERIFIED: .bun-version; pinned executable probe]                 |

Fjall's pinned API exposes atomic cross-keyspace batch commit and explicit durability selection. `SyncAll` uses fsync for data and metadata in the installed pinned source. Those APIs do not make an arbitrary read/compare against concurrent raw writers a CAS operation; existing publication source explicitly requires runtime coins/metadata serialization. \[CITED: https://docs.rs/fjall/3.1.4/fjall/struct.OwnedWriteBatch.html] [VERIFIED: local fjall-3.1.4/src/journal/writer.rs; packages/open-bitcoin-node/src/storage/fjall_store/filters/publication.rs]

### Supporting and version policy

Reuse existing Rust test harness, real temporary Fjall datadirs, Phase 155 validated history and Phase 153 ordinary automatic-retention fixtures. Add no installation step or manifest dependency. Versions are selected existing project pins, not new recommendations to install latest releases. Registry publish-date verification for Fjall 3.1.4 was attempted and returned HTTP 403; do not invent its release date. Context7 tools are unavailable in this session; exact-version official docs plus installed source were used. [VERIFIED: packages/Cargo.lock; packages/open-bitcoin-node/src/sync/tests/filter_index/recovery.rs; packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/automatic_prune.rs; tool inventory; registry probe]

### Alternatives Considered

| Proposal                          | Strongest benefit            | Decision                                                                                                                                                                                                                                           |
| --------------------------------- | ---------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Modify v1 state bytes in place    | One envelope                 | Reject silent layout change: decoder requires exact completion. Use a separate versioned owner envelope, preserving existing rows/state. [VERIFIED: storage/filter_index.rs decode_state]                                                          |
| New index mutex/service           | Local encapsulation          | Reject an independent authority. Extend the cloned store's existing shared publication guard and existing managed runtime owner. [VERIFIED: storage/fjall_store.rs; Phase 156 D-10]                                                                |
| Single atomic disable and release | Fewer commits inside storage | Accept only if work invalidation and serialized publication are proven before visibility. Prefer two explicit internal steps initially to make retained-protection failure states testable. [VERIFIED: Phase 156 D-07; existing SyncAll publisher] |

## Architecture Patterns

### Pattern 1: Additive typed owner and checked work identity

**Recommendation:** Add a small versioned owner key, e.g. `basic_filter:v1:owner`, alongside unchanged v1 records/projection/state. Parse to `Active { generation }` or `Disabled { generation }`; keep retained conservative lock as independent durable protection so disabled-after-release-failure is representable. Only internal lifecycle publication may change this envelope. [VERIFIED basis: storage/filter_index.rs STATE_KEY/codec; Phase 156 D-02/D-07/D-08]

Compatibility rules to implement and test:

| Saved artifacts                                                                  | Interpretation                                                                                      |
| -------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------- |
| No state, owner, rows, projection or reserved lock                               | Legacy absent; preserve old startup/deletion behavior                                               |
| Valid old v1 state, no owner envelope, valid reserved protection                 | Legacy active, conceptual generation zero; materialize owner before issuing work                    |
| Explicit Active owner plus state                                                 | Validate generation, checkpoint and required protection; missing/malformed protection refuses       |
| Explicit Disabled owner plus retained state/rows                                 | No work; retained valid protection may remain conservatively, or be absent after authorized release |
| Owner without state, malformed generation/mode, orphan lock or partial old state | Corruption/refusal; never infer disabled or reset                                                   |

These are proposed compatibility rules derived from the current absent-state recovery and exact v1 parser; absence alone must never erase an old index guarantee. [VERIFIED: storage/fjall_store/filters/startup.rs; open-bitcoin-chainstate/src/filter_index/recovery.rs; Phase 156 D-01/D-06]

Use private constructors for an opaque work token containing durable generation, expected checkpoint identity and preparation fence/branch identity. Before any work-owned record/checkpoint publication, reload owner state under the shared guard; require Active and the exact generation plus expected frontier. A generation alone does not prevent stale work within the same active generation from rewinding or replacing newer progress. Increment with checked arithmetic and refuse exhaustion; never wrap or infer generation from a clock/counter. Reopen remints work only after durable lifecycle recovery. [VERIFIED basis: Phase 156 D-08/D-09; existing VerifiedChainstateFence and immutable publication checks]

Route record-only `persist_basic_filter_records` through the same capability check when used as index work: rejecting only final checkpoint publication would leave a stale writer after disable. Keep a distinct explicit startup-recovery publication entrypoint, which may conservatively reconcile without pretending it is live worker completion. Do not leave the old unchecked publisher as an alternate live-work route. [VERIFIED: filters/publication.rs persist_basic_filter_records/publish_basic_filter_checkpoint; Phase 156 D-07]

### Pattern 2: Reserved CRUD enforced at the store

**Recommendation:** Handle and RPC reject attempts to set/clear `BASIC_INDEX_PRUNE_LOCK` regardless of authentication or whether it currently exists. Whole-map `sync_prune_locks` compares proposed reserved entry with the current validated owner entry under `filter_publication_guard`: unrelated operator updates may preserve the exact current entry; creation, omission, range change or invalid owner combinations refuse. Lifecycle code uses an internal guarded batch path, not this public replacement API. [VERIFIED basis: network/runtime_authority/prune_flush.rs; rpc/context/prune.rs; storage/fjall_store/prune/records.rs; Phase 156 D-01/D-02]

Preserving an identical current reserved entry is necessary for ordinary named CRUD, which currently reads and rewrites the full map. An old map must not resurrect or weaken ownership after a concurrent lifecycle transition. Compare against fresh durable ownership, not only the caller's requested name. Preserve existing other-lock replace/clear/list behavior and range/overflow checks. [VERIFIED: prune_flush.rs replace_prune_lock/clear_prune_lock; rpc/context/prune.rs]

### Pattern 3: Reload immediately before actual destruction

Manual handle application already reloads durable locks inside authority. Automatic prepare loads locks and uses them in `MeasurementKey`. Store `commit_paired_delete` presently takes only height/hash, writes intent and tombstones payloads without index validation. Its public trait forwarding and resumed intent both reach that concrete method. [VERIFIED: prune_flush.rs; automatic_prune.rs; fjall_store/prune.rs; chainstate/flush_lifecycle.rs; chainstate/fjall_store.rs]

**Recommendation:** Add one cheap validated effective-owner/protection loader, shared by manual/automatic application and concrete deletion. Acquire the publication guard before the payload mutation guard, keep it across fresh ownership/protection validation, intent publication and paired delete, and never recursively reacquire it through a public helper. Use private `*_guarded` helpers where nested batches need an already-held capability. Runtime order should remain authority → automatic-state → publication → payload. A publication helper requiring payload reads must use the same order. [VERIFIED basis: automatic_prune.rs documented lock order; fjall_store.rs shared Arc mutexes; payload_usage.rs with_payload_mutation; Phase 156 D-03]

Check `IndexInputProtection::check_prune_intent(height)` directly, as well as existing ordinary lock/keep/active-hash checks where they belong. Buffered `height_forbidden_by_lock` excludes heights zero and one; direct index protection intentionally includes them. A malformed ownership combination refuses before creating intent or changing payload usage. A legitimate protected candidate is skipped at the apply layer; a direct unauthorized concrete unlink refuses visibly. Do not fix the historical buffered formula globally. [VERIFIED: chainstate/src/prune/locks.rs; chainstate/src/filter_index.rs; flush_lifecycle/prune_apply.rs; Phase 156 D-03/D-04]

Keep the Phase 155 initialize ordering: coins recovery → lifecycle-aware index validation/reconciliation → load effective protection → resume intent → ReadyToFlush. Disabled saved indexes require startup validation of the explicit lifecycle instead of the previous universal reserved-lock requirement. Existing active live-intent checks still run before reconciliation mutation. [VERIFIED: flush_lifecycle.rs initialize; filters/startup.rs; sync/open_runtime.rs]

### Pattern 4: Records and chainstate proof authorize release

Keep `VerifiedChainstateFence` and existing atomic publisher: it rereads coins heads/best block and durable chain metadata, verifies ancestry, validates contiguous records/projection and writes checkpoint/state/full map in one SyncAll batch. Record-only append does not advance release authority. Do not relax using header tip, planned work, processed count or ahead rows. [VERIFIED: chainstate/src/filter_index.rs; filters/publication.rs; Phase 155 CONTEXT.md D-04..D-06]

**Recommendation:** Work-token comparison, fresh fence verification, immutable-row validation and owner-map update belong inside the same serialized publication critical section. Stalled/failed index keeps earliest required inputs. Existing protection can be stronger than checkpoint-derived minimum; do not weaken it as a cleanup side effect. Any actual backend commit ambiguity poisons further publication/deletion until reopen; after a successful commit but injected reply failure, reopen may observe the new durable proof, and tests must allow that truthful outcome. [VERIFIED basis: PublicationControl poisoned behavior/AfterCommit fault; IndexInputProtection::covers; Phase 156 D-05/D-06]

### Pattern 5: Disable, release, re-enable

**Recommendation:** Implement a serialized internal disable transition that stops issuing work, durably writes Disabled with incremented generation while preserving checkpoint/records/lock, then releases owned protection through a second guarded SyncAll batch. If the second step fails, stay disabled with extra retained history; never undo generation invalidation. Reopen accepts this conservative combination and never restarts work solely because rows exist. Idempotent disable retries do not reuse an old active token or perform unnecessary generation churn. [VERIFIED basis: Phase 156 D-07; current startup's state/lock invariant; pinned Knots base.cpp Stop joins sync thread]

Re-enable reconciles retained checkpoint against recovered coins/metadata, checks presence and validated historical body/undo for the required suffix, then atomically installs Active with a fresh generation plus conservative protection before returning work capability. Genesis needs body but no undo. If missing required history prevents safe re-enable, preserve the saved prefix/disabled state and refuse without acquisition. Phase 157 will consume this internal contract for public activation; do not add flags, scheduler or daemon activation now. [VERIFIED basis: BasicFilterInputs historical contract; Phase 155 recovery; Phase 156 D-08 and deferred scope; Phase 157 CFAC-02]

### Pattern 6: Changed protection bypasses stale Periodic throttling

`MeasurementKey` already includes locks, but after its equality optimization, `prepare` independently skips Periodic scans inside 60 seconds. Therefore a changed lock can still wait for the old throttle. The state currently records no separate identity for the most recent scan. [VERIFIED: automatic_prune.rs MeasurementKey/prepare]

**Recommendation:** Track the last measured protection identity, including durable owner generation/mode and effective locks. Before the periodic throttle, compare current ownership/protection and clear both reuse key and timing eligibility if changed. A handle-only invalidation hook is insufficient for cloned-store lifecycle publication; fresh loaded identity must expose changes. Keep unchanged-state scan coalescing, payload revision invalidation and independent coins-flush policy intact. Test the next Periodic turn at the same injected second after release/re-enable/ordinary lock changes. [VERIFIED basis: automatic_prune.rs; FjallNodeStore Clone; Phase 156 D-04]

### Plan-friendly file ownership and sequence

| Work package                                          | Exact existing seams                                                                                                                                                                                                    | Proposed small new modules                                                                                                         |
| ----------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------- |
| 1. Pure ownership/capability plus codec compatibility | `packages/open-bitcoin-chainstate/src/filter_index.rs`; `packages/open-bitcoin-node/src/storage/filter_index.rs`                                                                                                        | `filter_index/lifecycle.rs`, `filter_index/tests/lifecycle.rs`; node codec child if needed                                         |
| 2. Guarded lifecycle/publication and startup          | node `storage/fjall_store/filters/{publication,startup}.rs`; `storage/fjall_store.rs`                                                                                                                                   | `storage/fjall_store/filters/ownership.rs`, `lifecycle.rs`                                                                         |
| 3. Authoritative CRUD/unlink/cache wiring             | node `storage/fjall_store/prune{.rs,/records.rs}`; `chainstate/flush_lifecycle{.rs,/prune_apply.rs}`; `chainstate/fjall_store.rs`; `network/runtime_authority/{prune_flush,automatic_prune}.rs`; RPC `context/prune.rs` | authority `filter_index.rs` only for necessary internal runtime callers                                                            |
| 4. Real behavior/fault proof and evidence             | node `sync/tests/filter_index.rs` and children; automatic tests; RPC daemon automatic fixture; `docs/parity/source-breadcrumbs.json`, `docs/parity/index.json`, catalog, README, `scripts/verify.sh`                    | `sync/tests/filter_index/prune_coordination.rs`, `lifecycle.rs`; `scripts/check-phase156-prune-coordination.ts` and tested checker |

Existing paths are verified; new paths and sequencing are recommendations. Parallel edits must assign publication/startup/codec and prune/authority ownership explicitly because both work packages touch shared trait and publication seams. [VERIFIED: source file inventory; Phase 156 D-10]

### Simplification review

Keep one effective-owner loader, existing store mutex, existing paired unlink and one generation/capability vocabulary. Do not create a second full ancestry/index cache or duplicate the Phase 155 integrity scanner per candidate. Startup performs complete integrity recovery; release publication proves the prefix; actual apply reloads bounded owner/state/lock facts and validates their compatibility. If a cache is introduced later, key it by proven ownership/checkpoint revision and invalidate on publication/reopen. Preserve permitted fail-closed reorg/metadata boundaries rather than implementing Phase 158 early. [VERIFIED basis: filters.rs bounded streaming scans; Phase 155 SUMMARY decisions; Phase 156 scope]

## Don't Hand-Roll

| Problem                                               | Reuse                                                            | Why                                                                                                                           |
| ----------------------------------------------------- | ---------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| Durable checkpoint/protection ordering                | Existing SyncAll filter publisher                                | Already checks real coins/metadata fence and immutable/projection integrity. [VERIFIED: filters/publication.rs]               |
| Required input calculation                            | `IndexPrefix::input_protection`, `IndexInputProtection`          | Empty/genesis/exhaustion and safe range arithmetic already have typed semantics. [VERIFIED: chainstate/filter_index.rs]       |
| Ordinary retention, body receipts, undo/cache cleanup | Existing automatic/manual owner and paired unlink                | Preserves earned have-pruned/counters and error-path eviction. [VERIFIED: automatic_prune.rs; prune_flush.rs; prune_apply.rs] |
| Missing-history evidence                              | `BasicFilterInputs::from_historical` and store body/undo readers | Current coins cannot reconstruct historical spends. [VERIFIED: Phase 154/155 fixtures and historical input API]               |
| Test accounting                                       | Real retained-payload measurement                                | Do not use fabricated production usage to trigger automatic pruning. [VERIFIED: Phase 153 CONTEXT.md D-01/D-08]               |

## Common Pitfalls

1. **Protecting only API names:** direct whole-map replacement or concrete unlink remains a bypass. Verify both APIs plus resumed deletion and fresh reserved-name creation. [VERIFIED: current store/handle APIs; Phase 156 D-01/D-03]
1. **Checking only buffered locks:** genesis/height one are omitted by generic formula. Exercise direct unlink and startup intent at those heights. [VERIFIED: prune/locks.rs; filter_index.rs]
1. **Generation-only work fencing:** same-generation old frontier can race newer publication. Include expected checkpoint/branch and block stale record-only writes too. [VERIFIED basis: publication API accepts a checkpoint and independent records; Phase 156 D-07/D-09]
1. **Turning disabled into absent:** deleting state loses explicit ownership and makes old immutable/projection artifacts a partial-state corruption. Retain rows/state and add lifecycle-aware recovery. [VERIFIED: startup.rs for_absent_state; codec state fields]
1. **Deadlock by reusing public writers inside a held guard:** sync_prune_locks and publisher already acquire the publication mutex. Use narrowly scoped guarded internal helpers and documented order. [VERIFIED: records.rs sync_prune_locks; publication.rs]
1. **Throttle survives release:** measurement key includes locks but independent Periodic timing can suppress fresh work. Assert immediate same-second remeasurement. [VERIFIED: automatic_prune.rs prepare]
1. **Fault proof assumes failed reply means old disk state:** AfterCommit injection can return failure after new proof/lock became durable. Assert reopen state and retained inputs against actual committed proof. [VERIFIED: publication.rs finish_basic_filter_batch/after_commit_fault]
1. **Sparse fixtures promoted to consensus proof:** Phase 155 distinguishes validated spends from deletion-order chains. Reuse both with honest labels; keep complete continuous client/reorg integration for Phase 162. [VERIFIED: sync/tests/filter_index.rs module doc; recovery.rs; Phase 156 deferred scope]

## Code Examples

### Existing concrete fence and publication idiom

Source below is the verified existing API; lifecycle implementation must additionally require a current work capability before this internal publication. [VERIFIED: storage/fjall_store/filters/publication.rs; chainstate/filter_index.rs]

```rust
let fence = VerifiedChainstateFence::new(maybe_best_block, Some(&positions))
    .map_err(index_corruption)?;
store.publish_basic_filter_checkpoint(
    &fence,
    checkpoint,
    checkpoint.input_protection(),
    &records,
)?;
```

### Existing direct required-input check

Use this existing pure method in the final store gate; it handles heights zero and one. Do not substitute ordinary buffered-lock arithmetic. [VERIFIED: packages/open-bitcoin-chainstate/src/filter_index.rs]

```rust
protection
    .check_prune_intent(height)
    .map_err(index_corruption)?;
```

### Proposed internal transition order

This sequence is a proposed adapter contract, not an existing callable API. [VERIFIED basis: Phase 156 D-07/D-08; existing shared-store publication protocol]

```text
disable under managed authority:
  stop issuing work
  hold shared store publication guard
  reload + validate active owner, checkpoint and protection
  SyncAll: Disabled(next generation), preserving checkpoint and lock
  SyncAll: remove owned lock, preserving ordinary locks
  invalidate automatic measurement identity

work completion:
  hold shared publication guard
  reload owner; require Active and matching token generation/frontier/branch
  verify durable coins/metadata fence
  commit rows + projection + state + protection atomically
```

## Behavioral Evidence and Verification

Nyquist Validation Architecture is intentionally omitted: workflow.nyquist_validation is explicitly false. Behavioral proof remains required by D-09 and the native verifier. No Cargo/Bazel commands were run during research. [VERIFIED: .planning/config.json; parent task restrictions]

| Criterion                                    | Required real behavior                                                                                                                                                  | Reusable fixtures / test seams                                                                                                     |
| -------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------- |
| Manual and ordinary automatic inputs survive | Preplan deletion, initialize/stall index, apply through production handle; retain required body and undo; safe released prefix deletes                                  | node `sync/tests/filter_index/startup.rs`; `network/runtime_authority/automatic_prune/tests.rs`; daemon `tests/automatic_prune.rs` |
| CRUD cannot weaken owner                     | Authenticated set/clear reserved identity, direct store omitted/changed map, absent-owner forged creation; unrelated locks still work                                   | RPC dispatch/context prune tests; Fjall prune-record tests                                                                         |
| Durable progress owns release                | Ahead rows cannot release; checkpoint/protection/coins/metadata faults; real reopen proves safe cursor and conservative lock; direct height 0/1 attempts refuse         | Phase 155 `sync/tests/filter_index/{recovery,faults}.rs`; `filters/tests/faults.rs`                                                |
| Disable/re-enable ordered                    | Prepare work, disable, publish stale work; release-failure retains lock; reopen disabled; re-enable protects before work; stale old generation rejected after re-enable | New lifecycle tests through real store and managed owner                                                                           |
| Automatic cache observes transition          | Same-second Periodic after progress/release/re-enable; exact retained bytes, actual paired deletes/skip; unchanged identity still coalesces                             | existing automatic prune measurement tests plus real daemon fixture                                                                |

Every reused path is verified in the source inventory. Matrix rows are required proposed evidence, not reported passes. [VERIFIED: listed source files; ROADMAP.md Phase 156; Phase 156 D-09]

Use a real legal 550 MiB automatic target and actual measured payload values to exercise ordinary retention. Phase 153 fixtures already seed resource-conscious nonactive bulk payloads so small active pairs become eligible while protected/nonactive bytes keep target unreachable. Extend those fixtures carefully; do not allocate a new giant decoded chain or add an accounting override. Pair cheap pure transition tests with at least one actual daemon `flush_cycle`/Periodic path and production runtime reopen. [VERIFIED: rpc/bin/open_bitcoind/tests/automatic_prune.rs and fixtures.rs; archived v2.4 audit Actual retention and limits]

Extend concrete PublicationControl fault points for disable-state and protection-release boundaries; use channel/barrier synchronization for cloned-store race tests, not sleeps. Test before/after commit separately, generations and checkpoint contents, unchanged immutable rows, payload/undo presence, prune intent, earned deletion receipts/counters and automatic scan eligibility. Legacy absence and old active v1 recovery are controls. [VERIFIED basis: publication.rs FilterPublicationFault; existing automatic tests/writers.rs; Phase 155 faults; D-09]

Focused plan commands should be timed via the canonical wrapper, using actual package/test names; final gate remains the full native verifier. [VERIFIED: AGENTS.md; packages/Cargo.toml; scripts/verify.sh]

```bash
bun run scripts/command-timings.ts run --key phase156-chainstate -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-chainstate filter_index
bun run scripts/command-timings.ts run --key phase156-node -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node filter_index
bun run scripts/command-timings.ts run --key phase156-retention -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node automatic_prune
bash scripts/verify.sh
```

## State of the Art

| Earlier implementation                            | Phase 156 required pattern                                             | Basis                                                  |
| ------------------------------------------------- | ---------------------------------------------------------------------- | ------------------------------------------------------ |
| Serialized arbitrary named CRUD                   | Reserved owner plus operation authorization                            | [VERIFIED: current prune_flush.rs; D-01/D-02]          |
| Supplied-lock apply and unguarded concrete unlink | Durable effective protection checked at actual destruction             | [VERIFIED: prune_apply.rs; fjall_store/prune.rs; D-03] |
| Implicit saved-index ownership                    | Explicit compatible lifecycle/generation with stale completion refusal | [VERIFIED: codec/startup.rs; D-07/D-08]                |
| Lock-sensitive cache plus independent throttle    | Protection-sensitive throttle and reuse identity                       | [VERIFIED: automatic_prune.rs; D-04]                   |

No dependency upgrade or protocol modernization is needed for this scoped change. Keep the pinned baseline and document owned-lock/durability/layout distinctions without claiming public BASIC activation or serving. [VERIFIED: Phase 156 D-10; REQUIREMENTS.md scope]

## Runtime State Inventory

This additive persisted-lifecycle change requires compatibility reasoning even though it renames no existing identifier. No user datadir or external service was modified/inspected during research. [VERIFIED: parent task scope; research tool actions]

| Category            | Items found / inspection limit                                                                            | Required action                                                                                                                                                                                     |
| ------------------- | --------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Stored data         | v1 immutable rows, projection/state, full prune-lock map, live prune intent and schema-2 coins            | Code migration/compatible decode for owner absence; first owner materialization under guard; retain existing rows and checkpoint. [VERIFIED: storage/filter_index.rs; prune.rs; filters/startup.rs] |
| Live service config | No new public config or renamed service identifier in phase scope; installed external configs not queried | No service edit; Phase 157 owns activation. [VERIFIED: Phase 156 deferred scope]                                                                                                                    |
| OS-registered state | No service/OS-registration change in selected ownership contract; host registrations not queried          | No re-registration task. [VERIFIED: Phase 156 scope]                                                                                                                                                |
| Secrets/env vars    | No secret/env-name change in selected ownership contract; secret values not inspected                     | No secret migration. [VERIFIED: Phase 156 scope]                                                                                                                                                    |
| Build artifacts     | Existing Rust/Bazel outputs reflect source changes, without package/crate rename                          | Normal native rebuild; no installed package rename. [VERIFIED: D-10; workspace manifests]                                                                                                           |

## Environment Availability

| Dependency             | Available           | Observed version                         | Action                                                                                                                               |
| ---------------------- | ------------------- | ---------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------ |
| Rust                   | Yes                 | rustc 1.94.1                             | Use pin. [VERIFIED: rustc probe; rust-toolchain.toml]                                                                                |
| Bun on default PATH    | Yes, mismatched pin | 1.4.2                                    | Select verified pinned executable directory before native verification. [VERIFIED: bun probe; .bun-version]                          |
| Pinned Bun             | Yes                 | 1.3.9                                    | `/tmp/open-bitcoin-bun-1.3.9/bun-darwin-aarch64/bun`; verify again if temporary path disappears. [VERIFIED: direct executable probe] |
| Node                   | Yes                 | 24.13.0                                  | GSD tools available. [VERIFIED: node probe; successful phase-op init]                                                                |
| Bazel/Bazelisk         | Commands available  | Not run in read-only research            | Parent full verifier resolves actual versions. [VERIFIED: command -v probes; parent restriction]                                     |
| Pinned Knots submodule | Yes                 | a9aee730466ac67d35a3c03ee24676be5e045878 | Baseline source inspected. [VERIFIED: submodule git rev-parse]                                                                       |
| Fjall source           | Yes                 | 3.1.4 in Cargo cache                     | Real-store test fixture dependency available; no external daemon. [VERIFIED: Cargo.lock; local source reads]                         |

No external service or credential prerequisite was identified for these hermetic changes. The mismatched default Bun must be addressed by selecting the already-probed pin; do not silently use 1.4.2 for evidence. [VERIFIED: scoped implementation paths; .bun-version; environment probes]

## Security Domain

Security enforcement is active by default because config does not set security_enforcement false. ASVS 5.0.0 chapter numbering is used below; the older template's V2 Authentication/V4 Access Control labels belong to older numbering and must not be presented as current. This is scoped control guidance, not ASVS certification. [VERIFIED: .planning/config.json; official ASVS v5.0.0 directory listing] \[CITED: https://owasp.org/projects/asvs]

### Applicable ASVS Categories

| ASVS 5 category                  | Applies                        | Proposed control                                                                                                                                                                                                                            |
| -------------------------------- | ------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| V2 Validation and Business Logic | Yes                            | Typed lifecycle/envelopes, checked generation increment, state/lock compatibility, atomic effects and sequential transitions. \[CITED: https://raw.githubusercontent.com/OWASP/ASVS/v5.0.0/5.0/en/0x11-V2-Validation-and-Business-Logic.md] |
| V8 Authorization                 | Yes                            | Trusted store/owner enforcement; reject reserved CRUD and stale work immediately. \[CITED: https://raw.githubusercontent.com/OWASP/ASVS/v5.0.0/5.0/en/0x17-V8-Authorization.md]                                                             |
| V6 Authentication                | Existing boundary only         | Preserve authenticated RPC before dispatch; do not treat authentication as ownership authority. [VERIFIED: rpc/http.rs; context/prune.rs; ASVS directory listing]                                                                           |
| V7 Session Management            | No new session flow            | No added session/token service; internal work capability is a typed generation, not user authentication. [VERIFIED: Phase 156 scope; ASVS directory listing]                                                                                |
| V11 Cryptography                 | Existing filter integrity only | Reuse current typed filter commitments; no custom authentication crypto or new primitive. [VERIFIED: filter_index.rs; existing storage codec; ASVS directory listing]                                                                       |

### Known Threat Patterns

| Pattern                                           | STRIDE                             | Mitigation and evidence                                                                                                                                                   |
| ------------------------------------------------- | ---------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Authenticated operator impersonates index owner   | Elevation of privilege / Tampering | Reserve name at handle/RPC, preserve exact owner at whole-map store API, use explicit internal capabilities. [VERIFIED basis: D-01/D-02; inspected CRUD APIs]             |
| Stale plans/clone publication bypass current lock | Tampering                          | Shared publication guard held through actual intent/delete and lifecycle mutation; fresh durable owner facts. [VERIFIED basis: D-03; shared Arc mutex store]              |
| Stale work after disable/re-enable                | Tampering                          | Durable checked generation, expected checkpoint/branch and work-only publication gate. [VERIFIED basis: D-07/D-08]                                                        |
| Fake checkpoint releases unindexed history        | Tampering                          | Existing complete-prefix validation and recovered coins/metadata fence. [VERIFIED: filters/publication.rs]                                                                |
| Corrupt owner/map accepted as absence             | Tampering / Denial of service      | Parse/refuse without payload mutation; retained protected state and explicit reopen diagnostic. [VERIFIED basis: storage codec errors; D-06]                              |
| Guard inversion blocks maintenance                | Denial of service                  | One documented authority/state/publication/payload order, no nested public locking helper, deterministic concurrency tests. [VERIFIED basis: inspected Mutex paths; D-10] |

## Assumptions Log

No factual training-only assumptions are used. Proposed APIs, paths and lifecycle rules are explicitly recommendations grounded in verified source and locked requirements; they are not asserted as existing behavior. [VERIFIED: cited local source and Phase 156 CONTEXT.md]

| #   | Claim               | Section | Risk if Wrong |
| --- | ------------------- | ------- | ------------- |
| —   | None tagged ASSUMED | —       | —             |

## Open Questions

1. **Exact capability visibility through generic managed stores — RESOLVED:** Plan 156-03 selects specialized lifecycle methods on the concrete durable handle, deriving the fence/history from `FjallChainstateStore::inner()` under existing `mutate` authority. Store lifecycle/publication methods stay crate-scoped; a narrow safe Rust host-controller method may be public for the dependent host/daemon fixture, with no caller-selected reserved name, range or raw checkpoint and no CLI/RPC/config route. Plan 156-02 keeps `BasicFilterWorkToken` opaque, crate-scoped and bound to the same store incarnation. The read-only prune snapshot trait in Plan 156-05 returns explicit unsupported/no-index facts for generic fixtures; it does not confer lifecycle authority. This resolves the planned API design, not implementation or test completion. [VERIFIED: 156-03-PLAN.md interfaces and Task 2; 156-02-PLAN.md interfaces; 156-05-PLAN.md interfaces and Task 2]
1. **Two-step disable versus one atomic transition — RESOLVED:** Plan 156-03 Task 1 selects two ordered SyncAll batches under the existing publication guard: first stop work issuance and persist `Disabled(next generation)` while preserving records/checkpoint/owned lock; only after durable invalidation succeeds may the second batch release reserved protection. Ambiguous failures poison the live instance until reopen; Disabled-with-conservative-lock is valid recovery state, and retry completes release without unnecessary generation churn. Plan 156-07 Task 1 supplies the planned before/after fault and production reopen matrix. The one-batch alternative is not the selected plan. These are plan commitments, not observed successful transitions. [VERIFIED: 156-03-PLAN.md interfaces and Task 1; 156-07-PLAN.md Task 1]
1. **Hot-path validation cost — RESOLVED:** Plan 156-02 Task 1 selects one guarded effective-owner loader with bounded state/lock/checkpoint identity checks at application; complete forest/prefix validation remains at startup and release publication. Plan 156-05 uses that bounded snapshot at apply/per-candidate boundaries and the concrete final deletion gate. Plan 156-06 compares fresh lifecycle/generation/checkpoint/effective-lock identity before reuse and throttle gates without another full-index scan or authority. No new archive-scale benchmark target is introduced. This resolves planned responsibility and scope; actual cost and behavioral verification remain execution work. [VERIFIED: 156-02-PLAN.md Task 1; 156-05-PLAN.md interfaces and Tasks 1–2; 156-06-PLAN.md interfaces and Task 1]

## Sources

### Primary (HIGH confidence)

- Phase 156 CONTEXT.md, ROADMAP/REQUIREMENTS/STATE/PROJECT; Phases 147/148/150/153/155 CONTEXT and Phase 155 SUMMARY frontmatters — scope, durable choices and boundary evidence. [VERIFIED: local reads]
- Required AGENTS/sidecar/overrides and managed architecture/code-shape/testing/verification/local-guidance/Rust pages — constraints above. [VERIFIED: local reads]
- First-party implementation and fixture files named in the architecture/evidence sections — actual gaps, mutex/codec/fault contracts and reusable real-store tests. [VERIFIED: local reads/rg inventory]
- [Pinned Knots base.cpp](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/index/base.cpp) — Commit, Rewind, ChainStateFlushed, Stop, SetBestBlockIndex; [blockfilterindex.cpp](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/index/blockfilterindex.cpp) — CustomCommit and CustomRewind; [blockstorage.cpp](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/node/blockstorage.cpp) — buffered locks and lock persistence. [VERIFIED: pinned local submodule source]
- [Fjall 3.1.4 OwnedWriteBatch](https://docs.rs/fjall/3.1.4/fjall/struct.OwnedWriteBatch.html) — exact pinned API; installed journal writer source — SyncAll semantics. [CITED: docs.rs/fjall/3.1.4/fjall/struct.OwnedWriteBatch.html] [VERIFIED: Cargo source cache]
- [ASVS 5.0.0 V2](https://raw.githubusercontent.com/OWASP/ASVS/v5.0.0/5.0/en/0x11-V2-Validation-and-Business-Logic.md), [V8](https://raw.githubusercontent.com/OWASP/ASVS/v5.0.0/5.0/en/0x17-V8-Authorization.md), [official release directory](https://github.com/OWASP/ASVS/tree/v5.0.0/5.0/en) — scoped input/state/authorization guidance and verified chapter numbering. [CITED: official ASVS v5.0.0 sources]

### Secondary (MEDIUM confidence)

- Existing v2.5 ARCHITECTURE/PITFALLS and archived v2.4 audit — cross-phase rationale and retained caveats; current implementation was independently inspected instead of trusting older line numbers. [VERIFIED: local research/audit reads]

### Tertiary (LOW confidence)

None used; unresolved registry publish date is reported explicitly. [VERIFIED: research tool results]

## Metadata

**Confidence breakdown:** Standard stack HIGH (pins and installed APIs); architecture HIGH for existing seams, MEDIUM for proposed lifecycle/capability design; pitfalls HIGH for traced code, MEDIUM for mitigations awaiting implementation tests. [VERIFIED: evidence above]

**Research date:** 2026-10-04
**Valid until:** Recheck when any named source, pin or locked decision changes; recommended review within 30 days. [VERIFIED basis: phase-specific source/pin dependencies]

Research changes only this file; no builds, commits, external mutation, public activation or scheduled catch-up were performed. Root owns planning and final verification/finalization. [VERIFIED: parent task scope; research tool actions]
