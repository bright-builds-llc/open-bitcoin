---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 134-2026-07-28T01-41-12
generated_at: 2026-07-28T01:49:35.254Z
---

# Phase 134: Authoritative Cross-Cache Lifecycle Integration - Context

**Gathered:** 2026-07-27
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Make `ManagedNetworkHandle` the sole mutation authority for package and mempool
lifecycle consequences. Phase 134 must project committed admission and removal
facts completely across authoritative in-memory serving, relay, peer,
orphan/package, compact, retry, persistence-dirty, and evidence state while
keeping storage and network I/O outside the authority lock.

This phase does not define the durable snapshot schema or recovery coordinator
(Phase 135), receive-independent retry scheduling or package fanout transport
(Phase 136), broad RPC/operator presentation (Phase 137), or final parity and
release guardrails (Phase 138).

</domain>

<decisions>
## Implementation Decisions

### Authoritative Mutation Boundary

- **D-01:** Add a typed lifecycle command family on `ManagedNetworkHandle`
  backed by one deterministic, I/O-free projector. All package admission,
  single-transaction admission, pressure trimming, expiry, block connection,
  reorg, maintenance, snapshot preparation, relay-queue preparation, and
  receipt completion paths must reach authoritative state through this family.
- **D-02:** Narrow caller-facing methods may remain for ergonomic and
  compatibility reasons, but they must be facades over the shared projector.
  No adapter or subsystem may own a second mempool, retry set, rolling-fee
  state, lifecycle generation, or mutable cache projection.
- **D-03:** Preserve the Phase 127 `Arc<Mutex<_>>` authority and short
  synchronous critical sections. Do not introduce a single-owner actor,
  mailbox, cancellation protocol, or shutdown redesign in this phase.
- **D-04:** Keep lifecycle facts, in-memory projection, and shell effects as
  distinct types. `MempoolLifecycleDelta` remains the committed fact
  vocabulary; a prepared projection plan describes complete in-memory
  consequences; owned effect commands describe work that leaves the lock.

### Exhaustive Lifecycle Projection

- **D-05:** Preflight every fallible input needed for projection before
  changing dependent state, including txid/wtxid derivation, transaction-body
  resolution, package identity, final-membership consistency, and bounded
  effect construction. Applying a valid prepared plan under authority must be
  infallible.
- **D-06:** The aggregate reducer must make every authoritative target
  explicit: transaction serving, ordinary relay fanout, peer request and known
  state, orphan and same-peer package candidates, compact reconstruction
  inputs, local unbroadcast membership, persistence dirtiness/generation, and
  bounded aggregate evidence.
- **D-07:** `final_membership` is authoritative. Admitted members that become
  post-trim absent never enter accepted serving, fanout, compact, unbroadcast,
  or peer-known projections. Partial-package survivors project in their
  admitted topological order. A failed admission with an empty committed delta
  changes no dependent cache and does not advance persistence generation.
- **D-08:** Preserve parent-before-child order for admission and ordinary
  relay preparation. Add an explicit descendant-before-ancestor teardown
  contract for removal, including reverse cleanup of orphan/package candidate
  state. Reorgs compose lifecycle transitions sequentially, with each
  transition's final membership applied before the next; do not merge distinct
  reorg transitions with within-transition absent-wins deduplication.
- **D-09:** Cleanup must be identity-complete. Remove or retire both txid and
  wtxid indexes, accepted package fingerprints, request/known identities,
  retained candidates, compact inputs, and unbroadcast entries whenever their
  authoritative member is absent. Replacement, pressure eviction, expiry,
  block confirmation/conflict, reorg, and post-trim absence retain their typed
  causes without leaving serveable or accepted aliases.
- **D-10:** Keep generation-based full reconciliation as a startup/recovery,
  audit, randomized-test, and failure-injection oracle. Do not use full
  mempool/cache rebuilds as the normal pressure-path projection mechanism.

### Lock-Free Effects and Typed Receipts

- **D-11:** Use bounded effect-family `prepare → execute → complete`
  capabilities. Prepare owned storage or network commands under authority,
  release the lock, execute I/O in the shell, then apply consuming typed
  receipts through one short follow-up mutation.
- **D-12:** Extend the existing non-`Clone` receipt pattern rather than adding
  one generic heterogeneous `EffectBatch`. Each family must bind its receipt
  to the authority epoch, lifecycle or persistence generation, effect
  identity, and the relevant peer-session or snapshot identity.
- **D-13:** Completion distinguishes `Applied`, `AchievedButStale`, and
  `AlreadyApplied` semantics. A successful external effect remains truthful
  even when newer authority state prevents cache mutation; a stale receipt
  must never clear newer dirty state, newer unbroadcast intent, or newer peer
  provenance.
- **D-14:** Preserve successful-prefix truth. Complete every successfully
  written or persisted prefix exactly once before reporting a later batch
  failure. Failed encoding, failed I/O, disconnected peers, skipped commands,
  and unsent suffixes receive no achieved-effect credit.
- **D-15:** Fixed family-specific batch caps and a bounded completed-effect
  sequence or ledger provide idempotency. Do not add an unbounded receipt
  history or a durable transactional outbox in Phase 134.

### Verification and Scope Guardrails

- **D-16:** Add deterministic integration coverage for package admission,
  partial acceptance, replacement, pressure eviction, expiry, block
  connection, reorg, failed admission, stale receipt, duplicate receipt, and
  partial I/O success. Each scenario must assert the complete cross-cache
  consequence and persistence-generation truth.
- **D-17:** Add a narrow structural checker with independent mutations that
  rejects direct runtime mutation outside `ManagedNetworkHandle`, bypasses of
  the shared projector, omitted projection targets, second authorities, and
  storage/network I/O while authority is held.
- **D-18:** Default verification remains deterministic and hermetic. This
  integration does not add package wire messages, whole-mempool rebroadcast,
  public/default relay, guaranteed propagation, public-network CI, or
  production-readiness claims.

### the agent's Discretion

The planner may choose exact command, prepared-plan, generation, effect,
receipt, completion, and module names; the smallest batch caps consistent with
existing resource policy; whether accepted-package cleanup uses a direct index
or a bounded reverse lookup; and how focused fixtures construct stale
authority, peer-session, and persistence generations. Prefer small deep modules
that make an omitted target or replayable receipt difficult to represent.

</decisions>

<canonical-refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Project and Phase Contract

- `AGENTS.md` — repository workflow, parity breadcrumb, generated-artifact,
  verification, and repo-local command rules.
- `AGENTS.bright-builds.md` — managed Bright Builds workflow and cross-cutting
  standards.
- `standards-overrides.md` — local standards exceptions; no active substantive
  exception applies.
- `standards/core/architecture.md` — functional-core/imperative-shell and
  illegal-state guidance.
- `standards/core/operability.md` — runtime ownership and truthful effect
  evidence.
- `standards/core/testing.md` — focused Arrange/Act/Assert test expectations.
- `standards/core/verification.md` — sync-first and repo-native verification
  requirements.
- `standards/languages/rust.md` — Rust domain, error, module, and verification
  rules.
- `.planning/ROADMAP.md` § Phase 134 — fixed goal and success criteria.
- `.planning/REQUIREMENTS.md` § Authoritative Lifecycle Integration — normative
  MPLIFE-01 through MPLIFE-04 contracts and later-phase boundaries.
- `.planning/PROJECT.md` — v2.2 scope, core value, and deferred public/default
  relay claims.
- `.planning/STATE.md` — current milestone continuity and accumulated
  decisions.

### v2.2 Research and Prior Decisions

- `.planning/research/ARCHITECTURE.md` — one authority, cross-cache integration
  seams, build order, and owned effect guidance.
- `.planning/research/PITFALLS.md` — false atomicity, graph/cache divergence,
  split authority, stale receipt, and lock-across-I/O hazards.
- `.planning/research/SUMMARY.md` — synthesized milestone boundary and
  recommended lifecycle architecture.
- `.planning/phases/123-runtime-timing-and-evidence-integrity/123-CONTEXT.md` —
  achieved-effect timing and successful-write acknowledgement.
- `.planning/phases/127-authoritative-network-state-unification/127-CONTEXT.md`
  — one shared production authority and short critical sections.
- `.planning/phases/128-production-compact-announcement-transport/128-CONTEXT.md`
  — owned emissions, non-replayable successful-write receipts, and
  successful-prefix truth.
- `.planning/phases/130-resource-time-and-fee-primitives/130-CONTEXT.md` —
  committed lifecycle-delta vocabulary, removal cause/role, and retry-clear
  precedence.
- `.planning/phases/131-rolling-fee-expiry-and-descendant-eviction-core/131-CONTEXT.md`
  — pressure, descendant eviction, expiry, and rolling-fee lifecycle
  decisions.
- `.planning/phases/132-typed-package-vocabulary-and-staged-admission/132-CONTEXT.md`
  — staged package admission, fingerprints, final membership, partial
  acceptance, and coherent deltas.
- `.planning/phases/133-package-aware-download-and-orphan-bridge/133-CONTEXT.md`
  — same-peer candidate ownership, feedback boundary, and explicit deferral of
  full lifecycle projection.

### Open Bitcoin Integration Seams

- `packages/open-bitcoin-mempool/src/pool/lifecycle.rs` — lifecycle delta,
  final membership, removal identities, retry clears, and builder invariants.
- `packages/open-bitcoin-mempool/src/pool/package_admission.rs` — authoritative
  staged package engine.
- `packages/open-bitcoin-node/src/network/runtime_authority.rs` — shared
  `ManagedNetworkHandle` and typed mutation facade.
- `packages/open-bitcoin-node/src/network.rs` — authoritative aggregate and
  dependent cache ownership.
- `packages/open-bitcoin-node/src/network/admission_bridge.rs` — current
  singleton/package admission projection and feedback paths.
- `packages/open-bitcoin-node/src/network/mempool_lifecycle.rs` — current
  block, reorg, expiry, and removal projection loops.
- `packages/open-bitcoin-node/src/network/relay_serving.rs` — txid/wtxid
  serving records and accepted-status cleanup.
- `packages/open-bitcoin-node/src/network/relay_fanout.rs` — ordinary relay and
  unbroadcast evidence state.
- `packages/open-bitcoin-node/src/network/compact_receive_candidates.rs` —
  compact reconstruction input ownership and wtxid cleanup.
- `packages/open-bitcoin-node/src/network/announcement_transport.rs` — owned
  peer emissions and consuming receipts.
- `packages/open-bitcoin-network/src/peer.rs` — peer request, known,
  orphan/package, and disconnect state.
- `packages/open-bitcoin-rpc/src/inbound_listener/connection_runtime.rs` —
  production lock-free wire execution and successful-prefix completion.
- `packages/open-bitcoin-node/src/sync.rs` — durable sync shell and shared
  authority orchestration.
- `docs/parity/source-breadcrumbs.json` — required source breadcrumbs for new
  first-party Rust files and tests.

### Pinned Bitcoin Knots Anchors

- `packages/bitcoin-knots/src/validation.cpp` — authoritative transaction and
  package admission/removal semantics.
- `packages/bitcoin-knots/src/txmempool.cpp` — mempool graph mutation,
  descendant removal, expiry, and rolling-fee consequences.
- `packages/bitcoin-knots/src/node/txdownloadman_impl.cpp` — request, known,
  rejection, and package-candidate lifecycle.
- `packages/bitcoin-knots/src/txorphanage.cpp` — bounded orphan ownership and
  candidate cleanup.
- `packages/bitcoin-knots/src/net_processing.cpp` — admission bridge, relay,
  serving, and peer consequence ordering.
- `packages/bitcoin-knots/src/node/mempool_persist.cpp` — persistence-source
  boundary inherited by later phases.

</canonical-refs>

<code-context>
## Existing Code Insights

### Reusable Assets

- `MempoolLifecycleDelta` already carries admitted members, typed removals,
  final membership, and retry clears; it should remain the committed fact
  source.
- `ManagedNetworkHandle` already owns one mutex-backed runtime and private
  read/mutate helpers with typed public operations.
- `RelayServingCache`, `ManagedRelayFanoutState`,
  `CompactExtraTxnBuffer`, and `PeerManager` already expose the concrete
  dependent state that the projector must update.
- `PeerEmission` and `PeerEmissionReceipt` already demonstrate owned commands,
  lock-free I/O, consuming success acknowledgement, and successful-prefix
  completion.

### Established Patterns

- Pure mempool and network cores make deterministic decisions; node/RPC shells
  supply time, randomness, Fjall, sockets, scheduling, and presentation.
- Exact txid/wtxid identity and final membership take precedence over
  attempt-level success labels.
- Achieved-effect evidence advances only after the corresponding write or
  durable effect succeeds.
- Default verification uses fake clocks, fixture peers, temporary stores, and
  mutation-tested TypeScript structural guards.

### Integration Points

- Replace duplicated admission, expiry, block, and reorg removal loops with one
  prepared projection/reducer seam in `open-bitcoin-node`.
- Route package reports and deltas from the Phase 133 admission bridge through
  the same seam as singleton and maintenance mutations.
- Extend peer, serving, fanout, compact, orphan/package, unbroadcast,
  persistence-dirty, and evidence modules with infallible prepared operations.
- Generalize only the receipt identities needed for Phase 134; leave actual
  maintenance scheduling and package fanout transport to Phase 136.

</code-context>

<specifics>
## Specific Ideas

- Treat projection completeness as a closed compile-time target list so adding
  an authoritative cache forces the reducer and its tests to change.
- Make fallible preflight plus infallible apply the atomicity boundary instead
  of relying on rollback across independently fallible cache mutations.
- Separate achieved external truth from whether a stale receipt may still
  mutate current cache state.
- Preserve parent-first admission and successful-write order while making
  removal teardown explicitly descendant-first.

</specifics>

<deferred>
## Deferred Ideas

- A single-owner actor and its mailbox, fairness, cancellation, shutdown, and
  worker-failure protocol.
- A generic heterogeneous effect bus spanning unrelated storage and network
  families.
- A durable transactional outbox, cross-restart completion journal, snapshot
  schema, checkpoint coordinator, and recovery behavior — Phase 135.
- Receive-independent maintenance scheduling, initial-broadcast retry,
  parent-before-child package fanout, and transport wakeups — Phase 136.
- Broad RPC, CLI, dashboard, metrics, logs, and support-bundle presentation —
  Phase 137.
- Final pinned-Knots fixture expansion, sustained-pressure benchmarks, release
  claims, and milestone guardrails — Phase 138.

</deferred>

***

*Phase: 134-authoritative-cross-cache-lifecycle-integration*
*Context gathered: 2026-07-27*
