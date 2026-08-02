---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 135-2026-08-02T17-41-48
generated_at: 2026-08-02T17:41:48.380Z
---

# Phase 135: Snapshot Schema, Checkpointing, and Recovery - Context

**Gathered:** 2026-08-02
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Phase 135 makes supported restarts recover valid mempool source records and the
surviving local initial-broadcast set through current chainstate and policy. It
owns the versioned source-only snapshot envelope, generation-aware periodic and
clean-shutdown checkpoint coordination, staged topological recovery, final
membership installation, and typed checkpoint/recovery evidence.

The phase does not persist or restore rolling-fee state, topology, peer state,
serving/fanout queues, compact candidates, derived fee/resource aggregates, or
other rebuildable indexes. It does not add receive-independent retry scheduling
or transport receipts (Phase 136), broad RPC/dashboard/support presentation
(Phase 137), destructive repair, public/default relay, propagation guarantees,
public-network CI, or production-readiness claims.

</domain>

<decisions>
## Implementation Decisions

### Source-only snapshot contract

- **D-01:** Persist one atomic mempool-local envelope containing its own format
  version, captured lifecycle generation and capture time, canonical witness
  transaction bytes with trustworthy acceptance time, and a separate explicit
  set of authoritative unbroadcast member identities. The unbroadcast set must
  come from `ManagedNetworkHandle` authority in the same capture as the mempool
  records; historical `origin=local` or relay-requested metadata is not proof
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
  `prepare → execute → complete` snapshot capability. A periodic tick skips
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
  range is `(last_durable_generation, current_generation]`. Temporal age is
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
  or invalid. Preserve the stable typed classes `recovered`,
  `dropped_confirmed`, `dropped_duplicate`, `dropped_missing_parent`,
  `dropped_policy_incompatible`, and `dropped_evicted`, and add a distinct
  `dropped_expired` classification. Report snapshot-level schema/decode/identity
  failures separately from record-level drops.
- **D-11:** Classify a record as `recovered` only after final post-policy and
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

</decisions>

<canonical-refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Project and phase contract

- `AGENTS.md` — repo-local workflow, verification, parity breadcrumb, generated
  artifact, Rust, and UAT rules.
- `AGENTS.bright-builds.md` — managed workflow and cross-cutting standards.
- `standards-overrides.md` — no substantive active override applies.
- `standards/core/architecture.md` — functional-core/imperative-shell and
  illegal-state guidance.
- `standards/core/code-shape.md` — control-flow, naming, and module-size rules.
- `standards/core/operability.md` — truthful user-facing operational evidence.
- `standards/core/testing.md` — focused Arrange/Act/Assert tests.
- `standards/core/verification.md` — sync-first and repo-native verification.
- `standards/languages/rust.md` — Rust module, optional-name, invariant, and
  verification rules.
- `.planning/ROADMAP.md` § Phase 135 — fixed goal and success criteria.
- `.planning/REQUIREMENTS.md` § Durable Mempool Policy Recovery — MPDUR-01
  through MPDUR-04.
- `.planning/PROJECT.md` — v2.2 scope, parity value, architecture, and deferred
  public/default relay claims.
- `.planning/STATE.md` — current milestone continuity and accumulated workflow
  decisions.

### Milestone research and locked prior decisions

- `.planning/research/ARCHITECTURE.md` — authoritative persistence/restart flow,
  source-state boundary, and coordinator recommendation.
- `.planning/research/PITFALLS.md` — stale snapshot, false durability,
  lock-across-I/O, and recovery-order hazards.
- `.planning/research/SUMMARY.md` — synthesized v2.2 architecture and scope.
- `.planning/phases/108-durable-mempool-relay-state-recovery/108-CONTEXT.md` —
  existing recovery status, relay-cache rebuild, corruption, redaction, and
  no-propagation decisions.
- `.planning/phases/130-resource-time-and-fee-primitives/130-CONTEXT.md` —
  acceptance-time and local relay-intent metadata contracts.
- `.planning/phases/131-rolling-fee-expiry-and-descendant-eviction-core/131-CONTEXT.md`
  — rolling-floor restart baseline, expiry, capacity, and deterministic time.
- `.planning/phases/134-authoritative-cross-cache-lifecycle-integration/134-CONTEXT.md`
  — sole authority, complete projection, dirty generation, lock-free snapshot
  capability, stale receipts, and successful-prefix truth.

### Open Bitcoin implementation seams

- `packages/open-bitcoin-node/src/storage/mempool_snapshot.rs` — current source
  record, recovery classes, and order-dependent replay path.
- `packages/open-bitcoin-node/src/storage/snapshot_codec.rs` — versioned codec
  envelope and mempool DTO conversion.
- `packages/open-bitcoin-node/src/storage/fjall_store/mempool.rs` — current
  atomic save/load and prepared snapshot execution path.
- `packages/open-bitcoin-node/src/network/recovery.rs` — managed recovery,
  cache seeding, and summary evidence.
- `packages/open-bitcoin-node/src/network/runtime_authority/lifecycle.rs` —
  snapshot capture, generation binding, and stale completion behavior.
- `packages/open-bitcoin-node/src/network/runtime_authority/effects.rs` — thin
  outside-lock snapshot effect facade.
- `packages/open-bitcoin-node/src/network/lifecycle_effects.rs` — non-replayable
  snapshot capabilities, receipts, and bounded effect ledger.
- `packages/open-bitcoin-node/src/network/lifecycle_projection/authority.rs` —
  authoritative unbroadcast membership and persistence generations.
- `packages/open-bitcoin-node/src/network/lifecycle_projection/reconciliation.rs`
  — full-state reconciliation oracle for rebuilt caches.
- `packages/open-bitcoin-node/src/network/tests/recovery_cases.rs` — managed
  replay, cache rebuild, and lifecycle cleanup fixtures.
- `packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/` —
  dirty generation, exact unbroadcast, snapshot receipt, stale/duplicate, and
  reconciliation fixtures.
- `packages/open-bitcoin-node/src/storage/snapshot_codec/tests.rs` — current
  codec round-trip and schema/corruption tests.
- `packages/open-bitcoin-node/src/storage/fjall_store/tests/` — reopen,
  persistence, and failure-boundary fixtures.
- `docs/parity/source-breadcrumbs.json` — required source breadcrumbs for new
  first-party Rust files and tests.
- `scripts/check-phase103-mempool-lifecycle.ts` — existing persistence checker
  pattern.
- `scripts/check-phase108-durable-mempool-relay-state-recovery.ts` — existing
  recovery/cache/evidence checker pattern.
- `scripts/check-phase134-authoritative-lifecycle.ts` — authoritative lifecycle,
  lock-free effect, generation, and claim checker pattern.
- `scripts/verify.sh` — repository verification contract and checker order.

### Pinned Bitcoin Knots anchors

- `packages/bitcoin-knots/src/node/mempool_persist.cpp` — baseline dump/load,
  unbroadcast persistence, and restart behavior.
- `packages/bitcoin-knots/src/node/mempool_persist.h` — persistence contract and
  entrypoints.
- `packages/bitcoin-knots/src/txmempool.cpp` — replay admission, expiry,
  capacity, graph rebuild, and rolling-fee behavior.
- `packages/bitcoin-knots/src/txmempool.h` — mempool entry/index and rolling-fee
  state ownership.
- `packages/bitcoin-knots/src/validation.cpp` — chainstate validation and
  mempool admission/removal integration.
- `packages/bitcoin-knots/test/functional/mempool_persist.py` — persistence,
  unbroadcast, expiry, and restart expectations.

</canonical-refs>

<code-context>
## Existing Code Insights

### Reusable Assets

- `MempoolSnapshot`, its versioned codec, and the Fjall single-key adapter
  already provide an atomic full-snapshot baseline.
- `ManagedNetworkHandle::prepare_mempool_snapshot_write` plus consuming
  completion/abort capabilities already implement the required lock-free
  effect boundary and exact generation binding.
- `dirty_generation`, `unbroadcast_members`, and the reconciliation oracle are
  already owned by the authoritative runtime and can supply one coherent
  capture/install contract.
- `ManagedMempoolRecoverySummary` and stable low-cardinality recovery classes
  provide an evidence vocabulary to extend rather than replace.

### Established Patterns

- Pure mempool/network code decides ordering, replay, lifecycle, and final
  membership; node/Fjall/daemon shells capture time and perform I/O.
- Achieved persistence remains truthful even when completion is stale, while a
  stale receipt cannot clear newer dirty state.
- Derived caches are reconstructed from committed lifecycle facts and final
  membership instead of being independently serialized.
- Verification uses deterministic fake time, temporary stores, failure
  injection, mutation-tested TypeScript checkers, and explicit parity roots.

### Integration Points

- Extend snapshot preparation to capture canonical records and exact
  unbroadcast membership under the same authority guard.
- Add a shell-owned single-flight coordinator around the existing prepared
  snapshot write adapter and wire periodic plus shutdown triggers without
  holding authority across encoding or Fjall I/O.
- Replace stored-order direct mutation in `network/recovery.rs` with dependency
  analysis, staged replay, final classification, and one authoritative install.
- Feed narrow typed checkpoint/recovery facts into the shared status contract;
  leave broad rendering and support-bundle expansion to Phase 137.

</code-context>

<specifics>
## Specific Ideas

- Mirror Knots' transaction-record plus separate unbroadcast-set boundary while
  preserving Open Bitcoin's versioned atomic envelope and typed receipt model.
- Treat snapshot capture time, not write completion time, as the freshness age
  of durable content.
- Make final membership the only source for both `recovered` and restored
  unbroadcast truth so pressure trimming or invalid descendants cannot leak
  transient success.
- Prefer one full snapshot value and deterministic startup staging over a new
  journal, multi-key manifest, actor, or transactional outbox.

</specifics>

<deferred>
## Deferred Ideas

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

</deferred>

***

*Phase: 135-snapshot-schema-checkpointing-and-recovery*
*Context gathered: 2026-08-02*
