---
generated_by: gsd-phase-researcher
lifecycle_mode: yolo
phase_lifecycle_id: 134-2026-07-28T01-41-12
generated_at: 2026-07-28T02:09:59Z
---

# Phase 134: Authoritative Cross-Cache Lifecycle Integration - Research

<user-constraints>
## User Constraints (from CONTEXT.md)

**Source for every item in this block:** `[VERIFIED: .planning/phases/134-authoritative-cross-cache-lifecycle-integration/134-CONTEXT.md]`

### Locked Decisions

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

### Deferred Ideas (OUT OF SCOPE)

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

</user-constraints>

<phase-requirements>
## Phase Requirements

| ID | Description | Research Support |
| --- | --- | --- |
| MPLIFE-01 | `ManagedNetworkHandle` remains the sole runtime mutation authority for package admission, pressure policy, maintenance, persistence snapshots, relay queues, and transport receipts. | Route typed facade methods through one private authority reducer; structural checks prohibit direct aggregate mutation and second authorities. `[VERIFIED: .planning/REQUIREMENTS.md; 134-CONTEXT.md D-01..D-03]` |
| MPLIFE-02 | One lifecycle delta projects every package admission and removal into serving, fanout, peer request/known state, orphan/package candidates, compact reconstruction inputs, unbroadcast state, persistence dirtiness, and operator evidence. | Use a closed prepared-plan type with one explicit field per projection target and an infallible aggregate apply. `[VERIFIED: .planning/REQUIREMENTS.md; 134-CONTEXT.md D-04..D-10]` |
| MPLIFE-03 | Replacement, pressure eviction, expiry, block connection, reorg, and failed admission cannot leave stale descendants or accepted identities in any dependent cache. | Derive parent-first survivor order and descendant-first teardown before mutation; clean txid/wtxid aliases and validate with a full reconciliation oracle. `[VERIFIED: .planning/REQUIREMENTS.md; 134-CONTEXT.md D-07..D-10]` |
| MPLIFE-04 | Runtime adapters capture owned commands or snapshots under authority, release the lock before storage or network I/O, and apply bounded typed receipts in a short follow-up mutation. | Add family-specific prepare/execute/complete capabilities with epoch, generation, effect, and session/snapshot identities plus successful-prefix completion. `[VERIFIED: .planning/REQUIREMENTS.md; 134-CONTEXT.md D-11..D-15]` |

</phase-requirements>

## Summary

Phase 134 is an integration and authority-boundary phase, not a new policy phase. The mempool core already emits a deterministic `MempoolLifecycleDelta` containing admitted identities, typed removals, final membership, and retry-clear facts, but the node currently projects those facts through separate admission, expiry, connected-block, and reorg loops. Package submission is the sharpest gap: the package bridge returns its report and delta without applying the same serving/removal projection used by singleton admission. `[VERIFIED: packages/open-bitcoin-mempool/src/pool/lifecycle.rs; packages/open-bitcoin-node/src/network/admission_bridge.rs; packages/open-bitcoin-node/src/network/admission_bridge/package.rs; packages/open-bitcoin-node/src/network/mempool_lifecycle.rs]`

The correct planning unit is a sealed prepared transition: prepare the mempool patch and every dependent cache consequence while the authority is unchanged, validate all identities, bodies, ordering, final membership, effect bounds, and generation bindings, then consume it in one infallible in-memory apply. The current mempool implementation already has revision-bound `MempoolPatch` preparation and a guarded `apply_prepared`, but both package preparation and the raw patch are crate-private, so Phase 134 needs a narrow public capability rather than exposing patch internals. `[VERIFIED: packages/open-bitcoin-mempool/src/pool.rs; packages/open-bitcoin-mempool/src/pool/patch.rs; packages/open-bitcoin-mempool/src/pool/package_admission.rs]`

Effects remain separate. `ManagedNetworkHandle` prepares owned, family-specific commands under its `Arc<Mutex<_>>`, the node/RPC shell performs Fjall or socket I/O after releasing the lock, and a consuming receipt records achieved external truth without allowing stale completion to clear newer state. The existing peer-emission flow already demonstrates owned emissions, non-`Clone` receipts, outside-lock execution, and successful-prefix completion; Phase 134 should strengthen its identity and idempotency fields rather than replace it with a generic effect bus. `[VERIFIED: packages/open-bitcoin-node/src/network/runtime_authority.rs; packages/open-bitcoin-node/src/network/announcement_transport.rs; packages/open-bitcoin-rpc/src/inbound_listener/connection_runtime.rs; 134-CONTEXT.md D-11..D-15]`

**Primary recommendation:** implement one revision-bound `PreparedLifecycleTransition` and one closed-target `LifecycleProjectionPlan`, consumed only by `ManagedNetworkHandle`; then add family-specific snapshot and peer-emission capabilities whose receipts are generation-bound and idempotent. `[VERIFIED: 134-CONTEXT.md D-01..D-17; standards/core/architecture.md; standards/core/operability.md]`

## Project Constraints (from AGENTS.md)

- Preserve externally observable Bitcoin Knots `29.3.knots20260210` behavior and keep deviations auditable in `docs/parity/`. `[VERIFIED: AGENTS.md Project/Constraints and Repo-Local Guidance]`
- Materialize the pinned Knots baseline with `git submodule update --init --recursive`; use its source as the parity anchor, not an unpinned upstream branch. `[VERIFIED: AGENTS.md Repo-Local Guidance]`
- Keep pure policy and lifecycle facts in functional-core crates and filesystem, network, process, terminal, service, and durable-storage work in imperative shell adapters. `[VERIFIED: AGENTS.md Architecture/Conventions; standards/core/architecture.md]`
- Preserve the `rust-toolchain.toml` Rust `1.94.1` pin and Rust 2024 workspace edition. `[VERIFIED: AGENTS.md Repo-Local Guidance; rust-toolchain.toml; packages/Cargo.toml]`
- Add no production dependency on an existing Rust Bitcoin library; keep new dependencies minimal and security-conscious. Phase 134 needs no new production dependency. `[VERIFIED: AGENTS.md Project Constraints; local dependency audit]`
- Use `bash scripts/verify.sh` as the full verification contract; run ad-hoc Cargo or Bazel commands through `bun run scripts/command-timings.ts run --key <stable-key> -- <command>` and never overlap Cargo work against the same target directory. `[VERIFIED: AGENTS.md Repo-Local Guidance]`
- Before any future commit containing Rust changes, run format, Clippy with warnings denied, all-target/all-feature build, and all-feature tests in that order; the repo-native verifier remains the final contract. `[VERIFIED: AGENTS.md Rust Projects; standards/languages/rust.md; standards/core/verification.md]`
- Use Bun/TypeScript for the structural checker and mutation tests; Bash should remain a thin orchestration layer. `[VERIFIED: AGENTS.md Repo-Local Guidance; scripts/check-phase133-package-aware-download-orphan-bridge.ts; scripts/check-phase133-package-aware-download-orphan-bridge.test.ts]`
- Add parity breadcrumb entries for every new first-party Rust source or test file under the configured source scope. `[VERIFIED: AGENTS.md Repo-Local Guidance; docs/parity/source-breadcrumbs.json]`
- Treat `docs/metrics/lines-of-code.md` as a tracked generated artifact and review relevant READMEs after substantial feature/parity/workflow changes. `[VERIFIED: AGENTS.md Repo-Local Guidance]`
- Prefer `foo.rs` plus `foo/` module structure, propagate errors without `unwrap()`, use `thiserror` for library errors, `anyhow` for application boundaries, and `tracing` rather than `println!`. `[VERIFIED: AGENTS.md Rust Projects; standards/languages/rust.md]`
- Tests should assert behavior, isolate one concept, and use explicit Arrange/Act/Assert sections except where a trivial test remains unmistakable. `[VERIFIED: AGENTS.md Testing; standards/core/testing.md]`
- Do not add public/default relay, public-network CI, guaranteed propagation, or production-readiness claims. `[VERIFIED: AGENTS.md Project; .planning/REQUIREMENTS.md Out of Scope; 134-CONTEXT.md D-18]`

## Standard Stack

### Core

| Component | Verified version | Purpose | Why standard here |
| --- | --- | --- | --- |
| Rust | `1.94.1`, edition 2024 | Typed commands, prepared plans, consuming receipts, bounded state | Repository source of truth and existing implementation language. `[VERIFIED: rust-toolchain.toml; packages/Cargo.toml; local rustc 1.94.1]` |
| `std::sync::Arc<Mutex<_>>` | Rust stdlib | Preserve the sole synchronous runtime authority | This is the established `ManagedNetworkHandle` ownership model and is locked for Phase 134. `[VERIFIED: packages/open-bitcoin-node/src/network/runtime_authority.rs; 134-CONTEXT.md D-03]` |
| `BTreeMap` / `BTreeSet` / `VecDeque` | Rust stdlib | Deterministic projection order, identity indexes, bounded completed-effect ledger | The lifecycle builder already uses ordered collections for deterministic facts; a bounded deque/set pair supports receipt eviction without an external dependency. `[VERIFIED: packages/open-bitcoin-mempool/src/pool/lifecycle.rs; local codebase collection usage]` |
| `open-bitcoin-mempool` | workspace | Produce revision-bound mempool transitions and `MempoolLifecycleDelta` facts | This crate already owns admission, package policy, pressure, expiry, topology, and lifecycle delta invariants. `[VERIFIED: packages/open-bitcoin-mempool/src/pool.rs and pool/*.rs]` |
| `open-bitcoin-node` | workspace | Own the aggregate reducer, generations, effect preparation, and typed facades | `ManagedPeerNetwork` and `ManagedNetworkHandle` already own the production runtime aggregate. `[VERIFIED: packages/open-bitcoin-node/src/network.rs; packages/open-bitcoin-node/src/network/runtime_authority.rs]` |
| `open-bitcoin-network` | workspace | Apply peer request/known/orphan/package/compact lifecycle operations | `PeerManager`, download scheduling, orphanage, and compact partial-download state already live here. `[VERIFIED: packages/open-bitcoin-network/src/peer.rs and peer/*.rs]` |

### Supporting

| Component | Verified version | Purpose | When to use |
| --- | --- | --- | --- |
| Fjall | `3.1.4` | Execute current mempool snapshot persistence commands outside authority | Use only in the shell executor; schema/coordinator/recovery changes remain Phase 135. `[VERIFIED: packages/open-bitcoin-node/Cargo.toml; packages/open-bitcoin-node/src/storage.rs; 134-CONTEXT.md deferred ideas]` |
| Serde / serde_json | `1.0.228` / `1.0.149` | Existing snapshot serialization and stable data shapes | Reuse current snapshot encoding only; do not define a new persistence schema in this phase. `[VERIFIED: packages/open-bitcoin-node/Cargo.toml; packages/open-bitcoin-node/src/mempool_persistence.rs]` |
| Bun | `1.3.9` locally | Structural checker and independent mutation tests | Match the Phase 132/133 checker pattern and integrate with `scripts/verify.sh`. `[VERIFIED: .bun-version; local bun 1.3.9; scripts/check-phase133-package-aware-download-orphan-bridge*.ts]` |
| Bazel/Bazelisk | Bazel `8.6.0` locally | Repository smoke build through the full verifier | Use through `bash scripts/verify.sh`; no Phase 134-specific Bazel architecture is required. `[VERIFIED: local bazel 8.6.0; AGENTS.md Repo-Local Guidance; scripts/verify.sh]` |

### Alternatives Considered

| Instead of | Could use | Why not in Phase 134 |
| --- | --- | --- |
| Existing mutex authority | Actor/mailbox | Explicitly deferred; it adds cancellation, shutdown, fairness, and worker-failure semantics unrelated to lifecycle completeness. `[VERIFIED: 134-CONTEXT.md D-03 and Deferred Ideas]` |
| Family-specific effect capabilities | Generic heterogeneous `EffectBatch` | Explicitly prohibited because it weakens family-specific identity and completion invariants. `[VERIFIED: 134-CONTEXT.md D-12]` |
| Incremental prepared projection | Full cache rebuild after every transition | Prohibited on normal pressure paths; reconciliation is an audit/recovery/test oracle only. `[VERIFIED: 134-CONTEXT.md D-10]` |
| Bounded in-memory completion ledger | Durable outbox/journal | Cross-restart completion is Phase 135 scope, not Phase 134. `[VERIFIED: 134-CONTEXT.md D-15 and Deferred Ideas]` |

**Installation:** none. Reuse the pinned workspace and standard library; do not add a production crate. `[VERIFIED: local dependency audit; AGENTS.md Dependencies Philosophy]`

**Version verification:** versions above were verified from repository pins/manifests and local tool output on 2026-07-28; no registry lookup is needed because Phase 134 adds no dependency. `[VERIFIED: rust-toolchain.toml; packages/*/Cargo.toml; local CLI version probes]`

## Architecture Patterns

### Recommended Project Structure

```text
packages/
├── open-bitcoin-mempool/src/pool/
│   ├── prepared_lifecycle.rs                 # sealed revision-bound transition capability
│   ├── admission.rs                          # prepare singleton, facade consumes capability
│   ├── package_admission.rs                  # prepare package, facade consumes capability
│   ├── expiry.rs                             # prepare expiry transition
│   └── lifecycle.rs                          # committed fact vocabulary remains here
├── open-bitcoin-network/src/peer/
│   └── transaction_lifecycle.rs              # infallible prepared peer/orphan cleanup
└── open-bitcoin-node/src/network/
    ├── lifecycle_projection.rs               # closed plan, generation, prepare/apply reducer
    ├── lifecycle_effects.rs                  # family-specific commands/receipts/completion
    ├── runtime_authority.rs                  # only public mutation facade
    ├── admission_bridge.rs                   # narrow facade over shared reducer
    ├── mempool_lifecycle.rs                  # narrow facade over shared reducer
    ├── relay_serving.rs                      # infallible identity-complete operations
    ├── relay_fanout.rs                       # infallible prepared operations
    ├── compact_receive_candidates.rs         # explicit removal operations
    └── tests/
        └── lifecycle_projection_cases.rs     # complete scenario matrix
scripts/
├── check-phase134-authoritative-lifecycle.ts
└── check-phase134-authoritative-lifecycle.test.ts
```

The exact new names are planner discretion, but ownership should follow these boundaries: mempool owns the guarded core transition; network owns peer-local indexes; node owns the only cross-cache aggregate reducer and effect capabilities; RPC/storage code only executes owned commands. `[VERIFIED: 134-CONTEXT.md D-01..D-06 and agent discretion; standards/core/architecture.md]`

### Pattern 1: Sealed Prepared Core Transition

**What:** expose an opaque, consuming `PreparedMempoolTransition` that carries the base revision, prospective delta/report, and enough read-only transaction material for node projection preflight. Do not make `MempoolPatch` fields public. `[VERIFIED: packages/open-bitcoin-mempool/src/pool/patch.rs currently keeps patch construction private; 134-CONTEXT.md D-04..D-05]`

**Why:** current `submit`, `submit_package`, `expire`, and block lifecycle methods prepare and immediately apply their patch inside the mempool crate. Package pressure trimming can add removals not knowable by the node before evaluation, so node-level projection cannot be completely prepared before the core commit unless the preparation capability crosses the crate boundary. `[VERIFIED: packages/open-bitcoin-mempool/src/pool/admission.rs; package_admission.rs; expiry.rs; lifecycle.rs; prospective.rs]`

**Required API properties:**

- The capability is non-`Clone`, tied to a base `MempoolRevision`, and can be consumed once. `[VERIFIED: packages/open-bitcoin-mempool/src/pool.rs stale revision guard; 134-CONTEXT.md D-05]`
- Read-only access exposes the report/delta and affected transaction bodies needed to build projections, but not mutable patch internals. `[VERIFIED: 134-CONTEXT.md D-04..D-05]`
- `Mempool::apply_prepared_lifecycle(capability)` checks the revision before mutation and then consumes the patch. `[VERIFIED: existing `Mempool::apply_prepared` pattern in packages/open-bitcoin-mempool/src/pool.rs]`
- Existing public methods may remain compatibility facades that prepare and consume immediately, while node authority uses the split capability. `[VERIFIED: 134-CONTEXT.md D-02]`

### Pattern 2: Closed-Target Projection Plan

**What:** `LifecycleProjectionPlan` is one concrete struct with an explicit field for every authoritative target, rather than a list of callbacks or optional generic operations. `[VERIFIED: 134-CONTEXT.md D-06 and Specific Ideas]`

| Required plan field | Inputs | Infallible apply target |
| --- | --- | --- |
| serving | final-present owned bodies; final-absent txid/wtxid pairs and cause | `RelayServingCache` plus node transaction body indexes |
| fanout | parent-first final-present relay work; all absent identities | `ManagedRelayFanoutState` |
| peer lifecycle | accepted identities; descendant-first absent identities; originating peer/package identity | `PeerManager` request, known, download, orphan, and candidate state |
| compact | replacement bodies to retain; all absent wtxids to evict/retire | `CompactExtraTxnBuffer` and peer partial compact downloads |
| unbroadcast | eligible local final-present additions; lifecycle/retry clears | new bounded authoritative membership owned by `ManagedPeerNetwork` |
| persistence | next lifecycle generation and dirty-generation transition | new authoritative generation fields owned by `ManagedPeerNetwork` |
| evidence | fixed low-cardinality aggregate increments by cause/result | bounded aggregate evidence owned by `ManagedPeerNetwork` |

This list is the compile-time completeness contract and should also be mirrored as a target-name constant in the structural checker. Adding a new authoritative cache must force edits to the plan, reducer, reconciliation oracle, scenario assertions, and checker allowlist. `[VERIFIED: 134-CONTEXT.md D-06, D-17, Specific Ideas]`

### Pattern 3: Prepare First, Apply Infallibly

Preparation under `ManagedNetworkHandle` should perform these checks in order: `[VERIFIED: 134-CONTEXT.md D-05..D-09]`

1. Validate that every admitted, removed, or retry-cleared identity has exactly one consistent txid/wtxid mapping and final membership. `[VERIFIED: packages/open-bitcoin-mempool/src/pool/lifecycle.rs builder invariants; 134-CONTEXT.md D-05]`
2. Resolve and validate all final-present transaction bodies before any cache mutation; derive txid/wtxid once and store the canonical pair in the plan. `[VERIFIED: current serving storage derives identities fallibly in packages/open-bitcoin-node/src/network/inventory.rs; 134-CONTEXT.md D-05]`
3. Filter admissions strictly by `final_membership == Present`; retain their supplied/topological order. An admitted post-trim absence produces teardown only. `[VERIFIED: packages/open-bitcoin-mempool/src/pool/lifecycle.rs admitted order; 134-CONTEXT.md D-07]`
4. Derive removal teardown from the captured dependency graph, not BTree identity order or `Direct`/`Descendant` labels alone, and emit descendants before ancestors. `[VERIFIED: lifecycle removals are identity-sorted in packages/open-bitcoin-mempool/src/pool/lifecycle.rs; Knots `CTxMemPool::removeRecursive`/`RemoveStaged` in packages/bitcoin-knots/src/txmempool.cpp; 134-CONTEXT.md D-08]`
5. Precompute identity-complete operations for both txid and wtxid indexes, package fingerprints, peer request/known state, candidate retention, compact state, and unbroadcast state. `[VERIFIED: 134-CONTEXT.md D-09]`
6. Enforce each effect-family cap while constructing owned commands; cap failure aborts preparation with no mutation. `[VERIFIED: 134-CONTEXT.md D-05 and D-15]`
7. Guard the core transition revision and authority epoch, consume the core transition, then apply every target field without returning a new error. `[VERIFIED: packages/open-bitcoin-mempool/src/pool.rs; 134-CONTEXT.md D-05]`
8. Advance lifecycle/persistence generation once only when the committed delta is non-empty. `[VERIFIED: 134-CONTEXT.md D-07]`

Cache methods used during apply should accept already-derived identities and owned/validated bodies. Existing fallible helpers such as transaction storage must be split into a fallible preparation step and an infallible insertion step; swallowing errors or rolling back a partially projected aggregate is not acceptable. `[VERIFIED: packages/open-bitcoin-node/src/network/inventory.rs; standards/core/code-shape.md; 134-CONTEXT.md D-05]`

### Pattern 4: Sequential Reorg Composition

Apply each connected-block delta and each disconnected-block reacceptance as its own prepare/apply transition. Do not merge a reorg into one lifecycle builder because the builder's within-transition absent-wins rule would erase meaningful intermediate membership and ordering. `[VERIFIED: packages/open-bitcoin-node/src/network/mempool_lifecycle.rs currently processes transitions sequentially; packages/open-bitcoin-mempool/src/pool/lifecycle.rs absent-wins builder; 134-CONTEXT.md D-08]`

For every transition, assert:

- admissions/relay preparation are parent before child; `[VERIFIED: 134-CONTEXT.md D-08]`
- removals/orphan-package teardown are descendant before ancestor; `[VERIFIED: 134-CONTEXT.md D-08; packages/bitcoin-knots/src/net_processing.cpp package processing order]`
- final membership from transition N is visible while preparing transition N+1. `[VERIFIED: 134-CONTEXT.md D-08]`

### Pattern 5: Family-Specific Prepare → Execute → Complete

Use separate capabilities for snapshot persistence and peer emission. Each prepared command owns its bytes/message and carries:

- `AuthorityEpoch`; `[VERIFIED: 134-CONTEXT.md D-12]`
- lifecycle or persistence generation; `[VERIFIED: 134-CONTEXT.md D-12]`
- family-specific `EffectId`; `[VERIFIED: 134-CONTEXT.md D-12 and D-15]`
- peer-session identity for network writes or snapshot identity for persistence. `[VERIFIED: 134-CONTEXT.md D-12]`

The executor produces a non-`Clone` receipt only after the external effect succeeds. Completion first checks the bounded completed-effect ledger, then checks epoch/generation/session freshness, then applies any still-valid cache consequence. Its public result is: `[VERIFIED: 134-CONTEXT.md D-12..D-15; packages/open-bitcoin-node/src/network/announcement_transport.rs]`

| Completion | External truth | Current-state mutation |
| --- | --- | --- |
| `Applied` | effect succeeded | receipt was current and its consequence applied |
| `AchievedButStale` | effect succeeded | no newer dirty, unbroadcast, or peer-provenance state is cleared |
| `AlreadyApplied` | effect succeeded previously | duplicate/replayed completion is a no-op |

Completed-effect retention must be fixed and family-specific. Use a bounded sequence plus membership set so duplicate checks are efficient and oldest entries evict deterministically; do not make the ledger durable in this phase. `[VERIFIED: 134-CONTEXT.md D-15; agent discretion permits exact caps]`

For a batch, execute in command order. After each successful write/persist, complete that receipt before attempting or reporting a later failure. Encoding failure, disconnection, skipped command, write failure, and unsent suffix produce no receipt. `[VERIFIED: packages/open-bitcoin-rpc/src/inbound_listener/connection_runtime.rs existing successful-prefix behavior; 134-CONTEXT.md D-14]`

### Pattern 6: Generation-Based Reconciliation Oracle

Add a deterministic reconciliation report that compares canonical mempool membership against every derived cache's accepted identities and generation. Use it only at startup/recovery, explicit audit, randomized tests, and injected-failure tests. Normal admission, expiry, and pressure transitions must remain incremental. `[VERIFIED: 134-CONTEXT.md D-10]`

The oracle should report bounded counts and target labels, while test-only helpers may expose exact identities for assertions. Production aggregate evidence must not emit high-cardinality transaction identifiers. `[VERIFIED: .planning/REQUIREMENTS.md MPOBS-03; 134-CONTEXT.md D-06 and D-18]`

### Exact File and Symbol Plan Boundaries

| File/module | Symbols to add or change | Planning boundary |
| --- | --- | --- |
| `packages/open-bitcoin-mempool/src/pool/prepared_lifecycle.rs` | opaque `PreparedMempoolTransition`, read-only facts accessor, consuming apply entrypoint | Expose capability, not `MempoolPatch`; retain core policy and revision guard here. `[VERIFIED: existing private patch seam in pool.rs/patch.rs; recommended module name is planner discretion]` |
| `packages/open-bitcoin-mempool/src/pool/{admission,package_admission,expiry,lifecycle}.rs` | split prepare from compatibility submit/apply facades | Preserve existing public behavior while enabling aggregate preflight. `[VERIFIED: current immediate-apply methods in these files]` |
| `packages/open-bitcoin-node/src/network/lifecycle_projection.rs` | lifecycle command vocabulary, `LifecycleProjectionPlan`, prepare, consume/apply, generation, reconcile | This is the only cross-cache reducer. `[VERIFIED: 134-CONTEXT.md D-01..D-10]` |
| `packages/open-bitcoin-node/src/network.rs` | authoritative epoch/generation, unbroadcast membership, dirty generation, bounded evidence/completion state | Keep every mutable projection inside `ManagedPeerNetwork`. `[VERIFIED: current aggregate fields in network.rs; 134-CONTEXT.md D-02 and D-06]` |
| `packages/open-bitcoin-node/src/network/runtime_authority.rs` | typed facade methods for all lifecycle/effect prepare/complete paths | The private mutex helpers remain inaccessible to adapters. `[VERIFIED: current ManagedNetworkHandle shape; 134-CONTEXT.md D-01..D-03]` |
| `packages/open-bitcoin-node/src/network/{admission_bridge,mempool_lifecycle}.rs` | replace duplicate loops with projector facades | These modules retain orchestration-specific input/result shapes only. `[VERIFIED: duplicated loops in current files]` |
| `packages/open-bitcoin-node/src/network/{relay_serving,relay_fanout,compact_receive_candidates}.rs` | prevalidated infallible target operations and explicit identity removal | Do not let these modules derive identities or own authority. `[VERIFIED: current cache APIs; 134-CONTEXT.md D-05..D-09]` |
| `packages/open-bitcoin-network/src/peer/transaction_lifecycle.rs` | one prepared peer cleanup operation covering scheduler, known indexes, orphan/package candidates, and partial compact state | Peer-local implementation remains encapsulated but is invoked only by node reducer. `[VERIFIED: current state split across peer.rs and peer modules]` |
| `packages/open-bitcoin-node/src/network/lifecycle_effects.rs` | family command/receipt/ID types, completion enum, bounded ledgers | No generic batch and no I/O implementation under the lock. `[VERIFIED: 134-CONTEXT.md D-11..D-15]` |
| `packages/open-bitcoin-rpc/src/inbound_listener/connection_runtime.rs` and storage shell caller | execute prepared commands and complete successful prefixes | Adapters receive capabilities but never mutate aggregate state directly. `[VERIFIED: existing connection runtime pattern; 134-CONTEXT.md D-11 and D-14]` |
| `packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases.rs` | full deterministic scenario matrix and reconciliation assertions | New Rust test file requires parity breadcrumb registration. `[VERIFIED: AGENTS.md; docs/parity/source-breadcrumbs.json]` |
| `scripts/check-phase134-authoritative-lifecycle*.ts`, `scripts/verify.sh` | structural contract plus independent mutations and verifier wiring | Mirror Phase 133's checker/test pattern. `[VERIFIED: scripts/check-phase133-package-aware-download-orphan-bridge*.ts; scripts/verify.sh]` |

### Anti-Patterns to Avoid

- **Project after core commit with fallible cache calls:** a projection failure would leave canonical mempool state ahead of serving/peer/persistence state. Prepare the entire plan first. `[VERIFIED: 134-CONTEXT.md D-05; current immediate-apply seam]`
- **Drive projections from attempt-level `MempoolOutcome`:** it lacks package-wide final-membership truth; use the committed delta and prepared bodies. `[VERIFIED: packages/open-bitcoin-node/src/network/relay_fanout.rs current outcome-driven path; 134-CONTEXT.md D-07]`
- **Treat `admitted` as accepted without checking final membership:** pressure trimming can make an admitted member absent in the same transition. `[VERIFIED: packages/open-bitcoin-mempool/src/pool/package_admission/finalization.rs; 134-CONTEXT.md D-07]`
- **Use identity sort as removal dependency order:** deterministic BTree order is not descendant teardown order. Derive reverse topology explicitly. `[VERIFIED: packages/open-bitcoin-mempool/src/pool/lifecycle.rs; 134-CONTEXT.md D-08]`
- **Put sockets or Fjall calls inside `ManagedNetworkHandle::mutate`:** it lengthens the critical section and makes partial-effect truth ambiguous. `[VERIFIED: standards/core/operability.md; 134-CONTEXT.md D-11]`
- **Let stale receipts clear current intent:** achieved external truth and current cache mutation are separate outcomes. `[VERIFIED: 134-CONTEXT.md D-13]`
- **Create a second unbroadcast or dirty-generation owner in an adapter:** all mutable lifecycle state belongs to `ManagedPeerNetwork`. `[VERIFIED: 134-CONTEXT.md D-02 and D-06]`

## Don't Hand-Roll

| Problem | Do not build | Use instead | Why |
| --- | --- | --- | --- |
| Transaction/package policy or removal closure | Node-side policy replica | Existing `open-bitcoin-mempool` prepared patch and lifecycle delta | The core already owns topology, replacement, pressure, expiry, fee, and final-membership invariants. `[VERIFIED: packages/open-bitcoin-mempool/src/pool/*.rs]` |
| Cross-cache rollback | Compensating actions after a partial apply | Fallible complete preparation plus infallible consume/apply | Rollback must reproduce every index and ordering invariant and can itself fail. `[VERIFIED: 134-CONTEXT.md D-05 and Specific Ideas]` |
| General effect framework | Dynamic callbacks or heterogeneous effect bus | Family-specific owned command and receipt types | Family identities and stale rules differ; generic batching is explicitly out of scope. `[VERIFIED: 134-CONTEXT.md D-11..D-15]` |
| Unbounded idempotency | Ever-growing receipt hash set | Fixed family-specific deque/set ledger | Long-lived nodes require bounded memory; durable replay is Phase 135. `[VERIFIED: 134-CONTEXT.md D-15]` |
| Normal-path reconciliation | Rebuild every cache from mempool after each change | Incremental closed projection plus audit oracle | Full rebuild hides missing lifecycle targets and is prohibited on pressure paths. `[VERIFIED: 134-CONTEXT.md D-10]` |
| New package transport/rebroadcast | Package wire messages or whole-mempool resend | Existing ordinary transaction emission preparation only | Scheduling and package fanout transport are Phase 136; broader protocols are out of scope. `[VERIFIED: .planning/REQUIREMENTS.md Out of Scope; 134-CONTEXT.md D-18 and Deferred Ideas]` |
| New snapshot format | Phase 134 persistence schema | Existing `MempoolSnapshot` capture and Fjall save boundary | Schema, coordinator, crash window, and recovery belong to Phase 135. `[VERIFIED: packages/open-bitcoin-node/src/mempool_persistence.rs; 134-CONTEXT.md Deferred Ideas]` |

**Key insight:** atomicity here comes from making every possible failure occur before a single consuming aggregate apply, not from transactions or rollback across unrelated in-memory caches. `[VERIFIED: 134-CONTEXT.md D-05 and Specific Ideas]`

## Common Pitfalls

### Pitfall 1: Canonical Mempool Commits Before Projection Preparation

**What goes wrong:** package pressure trimming or body derivation succeeds in the core, then a node cache operation fails, leaving split truth. `[VERIFIED: current immediate-apply APIs in open-bitcoin-mempool; fallible identity derivation in network/inventory.rs]`

**Why it happens:** the raw prepared patch is crate-private and current public methods combine evaluation with application. `[VERIFIED: packages/open-bitcoin-mempool/src/pool/patch.rs; package_admission.rs]`

**How to avoid:** add the sealed prepared capability first, then require the node reducer to preflight all target operations before consuming it. `[VERIFIED: 134-CONTEXT.md D-04..D-05]`

**Warning signs:** any `?` after the core transition is applied but before generation and all cache targets are updated. `[VERIFIED: 134-CONTEXT.md D-05]`

### Pitfall 2: Package Report Success Overrides Final Membership

**What goes wrong:** a member accepted during staged evaluation but removed by post-trim pressure enters serving, fanout, known, compact, or unbroadcast state. `[VERIFIED: 134-CONTEXT.md D-07; package finalization path]`

**How to avoid:** build admissions from the ordered intersection of `delta.admitted` and `final_membership == Present`; build teardown for every affected final-absent identity. `[VERIFIED: 134-CONTEXT.md D-07]`

**Warning signs:** projection code switches on `MempoolOutcome` or package report status without reading `final_membership`. `[VERIFIED: current relay_fanout outcome API; 134-CONTEXT.md D-07]`

### Pitfall 3: Incomplete Identity Cleanup

**What goes wrong:** one alias or derived record remains serveable/known after removal. Current state spans txid and wtxid body indexes, serving maps, fanout mapping, peer known/request maps, orphan/candidate state, compact extras, and partial downloads. `[VERIFIED: packages/open-bitcoin-node/src/network.rs; inventory.rs; relay_serving.rs; relay_fanout.rs; packages/open-bitcoin-network/src/peer.rs]`

**How to avoid:** make canonical identity pairs mandatory plan inputs and expose one peer-lifecycle cleanup operation that touches every peer-local owner. `[VERIFIED: 134-CONTEXT.md D-06 and D-09]`

**Warning signs:** a removal API accepts only txid or only wtxid without a precomputed canonical pair. `[VERIFIED: 134-CONTEXT.md D-09]`

### Pitfall 4: Correct Set, Wrong Graph Order

**What goes wrong:** parent candidate cleanup occurs before child teardown or relay work queues a child before its parent. `[VERIFIED: 134-CONTEXT.md D-08; Knots package processing anchors]`

**Why it happens:** lifecycle `removed` is deterministically identity-sorted, not reverse-topological. `[VERIFIED: packages/open-bitcoin-mempool/src/pool/lifecycle.rs]`

**How to avoid:** carry explicit ordered vectors in the prepared plan and test order separately from final set equality. `[VERIFIED: 134-CONTEXT.md D-08 and D-16]`

### Pitfall 5: Reorg Delta Coalescing

**What goes wrong:** absent-wins deduplication across distinct reorg steps erases an intermediate admission or removal that should affect the next step. `[VERIFIED: 134-CONTEXT.md D-08]`

**How to avoid:** prepare and consume one delta at a time and advance generation per non-empty committed transition. `[VERIFIED: 134-CONTEXT.md D-07..D-08]`

### Pitfall 6: Receipt Proves Too Little

**What goes wrong:** a receipt proves only peer/hash or snapshot success, so a replay or stale completion mutates a newer session/generation. The current peer receipt does not carry authority epoch, lifecycle generation, effect ID, or peer-session identity. `[VERIFIED: packages/open-bitcoin-node/src/network/announcement_transport.rs]`

**How to avoid:** bind all four identity dimensions and return `Applied`, `AchievedButStale`, or `AlreadyApplied`. `[VERIFIED: 134-CONTEXT.md D-12..D-13]`

**Warning signs:** receipt types derive `Clone`, completion lacks an idempotency lookup, or a stale branch clears dirty/unbroadcast state. `[VERIFIED: 134-CONTEXT.md D-12..D-15]`

### Pitfall 7: Batch Failure Erases Successful Prefix Truth

**What goes wrong:** the shell returns on command N failure before completing receipts for successful commands 0..N-1, or credits an unsent suffix. `[VERIFIED: 134-CONTEXT.md D-14]`

**How to avoid:** encode, write, create receipt, and complete one command at a time; stop with all prior completions recorded. `[VERIFIED: existing connection runtime loop; 134-CONTEXT.md D-14]`

### Pitfall 8: Structural Checker Tests Only the Happy Source

**What goes wrong:** a checker passes the repository but cannot detect the prohibited bypass it claims to enforce. `[VERIFIED: 134-CONTEXT.md D-17; established Phase 133 checker mutation pattern]`

**How to avoid:** use separate temporary-fixture mutations for direct mutation, projector bypass, omitted target, second authority, and I/O-under-lock, and assert each fails for the intended reason. `[VERIFIED: 134-CONTEXT.md D-17; scripts/check-phase133-package-aware-download-orphan-bridge.test.ts]`

## Code Examples

The examples below are recommended shapes synthesized from the locked phase contract and existing code; exact names are planner discretion. `[VERIFIED: 134-CONTEXT.md agent discretion]`

### Sealed Prepared Transition

```rust
// Source basis:
// - packages/open-bitcoin-mempool/src/pool.rs::apply_prepared
// - packages/open-bitcoin-mempool/src/pool/patch.rs::MempoolPatch
// - 134-CONTEXT.md D-04..D-05
pub struct PreparedMempoolTransition {
    patch: MempoolPatch,
    facts: PreparedLifecycleFacts,
}

impl PreparedMempoolTransition {
    pub fn facts(&self) -> &PreparedLifecycleFacts {
        &self.facts
    }
}

impl Mempool {
    pub fn apply_prepared_lifecycle(
        &mut self,
        prepared: PreparedMempoolTransition,
    ) -> Result<MempoolLifecycleDelta, MempoolError> {
        self.apply_prepared(prepared.patch)
    }
}
```

The outer aggregate must validate its plan before calling the consuming method; the only remaining `Result` is the revision guard, which occurs before mempool mutation. `[VERIFIED: packages/open-bitcoin-mempool/src/pool.rs; 134-CONTEXT.md D-05]`

### Closed Projection Apply

```rust
// Source basis: 134-CONTEXT.md D-05..D-10.
struct LifecycleProjectionPlan {
    base_epoch: AuthorityEpoch,
    core: PreparedMempoolTransition,
    serving: PreparedServingProjection,
    fanout: PreparedFanoutProjection,
    peers: PreparedPeerLifecycleProjection,
    compact: PreparedCompactProjection,
    unbroadcast: PreparedUnbroadcastProjection,
    persistence: PreparedPersistenceProjection,
    evidence: PreparedLifecycleEvidence,
}

impl ManagedPeerNetwork {
    fn apply_lifecycle(
        &mut self,
        prepared: LifecycleProjectionPlan,
    ) -> Result<AppliedLifecycle, LifecycleApplyError> {
        if self.authority_epoch != prepared.base_epoch {
            return Err(LifecycleApplyError::StaleAuthority);
        }

        let delta = self.mempool.apply_prepared_lifecycle(prepared.core)?;
        self.relay_serving.apply_prepared(prepared.serving);
        self.relay_fanout.apply_prepared(prepared.fanout);
        self.peer_manager.apply_prepared_lifecycle(prepared.peers);
        self.compact_extra_txn.apply_prepared(prepared.compact);
        self.unbroadcast.apply_prepared(prepared.unbroadcast);
        self.persistence.apply_prepared(prepared.persistence);
        self.lifecycle_evidence.apply_prepared(prepared.evidence);

        Ok(AppliedLifecycle { delta })
    }
}
```

After the epoch/revision guards, every `apply_prepared` target method must be infallible. The implementation should structure guards before destructuring/mutation so an error cannot occur after the first state change. `[VERIFIED: 134-CONTEXT.md D-05; standards/languages/rust.md error guidance]`

### Generation-Safe Receipt Completion

```rust
// Source basis:
// - packages/open-bitcoin-node/src/network/announcement_transport.rs
// - 134-CONTEXT.md D-11..D-15
pub enum EffectCompletion {
    Applied,
    AchievedButStale,
    AlreadyApplied,
}

pub fn complete_snapshot(
    &mut self,
    receipt: SnapshotWriteReceipt,
) -> EffectCompletion {
    if self.completed_snapshots.contains(receipt.effect_id()) {
        return EffectCompletion::AlreadyApplied;
    }
    self.completed_snapshots.record(receipt.effect_id());

    if receipt.authority_epoch() != self.authority_epoch
        || receipt.generation() != self.dirty_generation
        || receipt.snapshot_id() != self.pending_snapshot_id
    {
        return EffectCompletion::AchievedButStale;
    }

    self.last_persisted_generation = receipt.generation();
    self.dirty_generation = None;
    EffectCompletion::Applied
}
```

The concrete dirty-state representation must ensure that completing generation N cannot clear dirty generation N+1. `[VERIFIED: 134-CONTEXT.md D-13]`

### Successful-Prefix Shell Execution

```rust
// Source basis:
// - packages/open-bitcoin-rpc/src/inbound_listener/connection_runtime.rs
// - 134-CONTEXT.md D-14
for emission in emissions {
    let (message, capability) = emission.into_wire_parts();
    writer.write_message(&message)?;
    let receipt = capability.acknowledge_write();
    handle.complete_peer_emission(receipt)?;
}
```

Do not pre-create receipts for a whole batch; a receipt exists only after its individual effect succeeds. `[VERIFIED: 134-CONTEXT.md D-14]`

## State of the Art

| Existing approach | Phase 134 approach | Impact |
| --- | --- | --- |
| Singleton admission manually projects a subset of the delta in `apply_admitted_transition`. `[VERIFIED: admission_bridge.rs]` | Every admission path constructs the same closed projection plan. `[VERIFIED: 134-CONTEXT.md D-01..D-07]` | Package, singleton, and peer paths cannot drift. |
| Package bridge returns report/delta but does not run the singleton projection. `[VERIFIED: admission_bridge/package.rs]` | Package facts enter the shared projector before the bridge returns. `[VERIFIED: MPLIFE-02; 134-CONTEXT.md D-01]` | Closes the current package lifecycle gap. |
| Expiry, connected-block, and reorg paths repeat removal loops. `[VERIFIED: mempool_lifecycle.rs]` | Narrow facades invoke one reducer; reorg invokes it sequentially. `[VERIFIED: 134-CONTEXT.md D-02 and D-08]` | One cleanup contract and one ordering implementation. |
| Serving insertion derives identity during mutation and can fail. `[VERIFIED: inventory.rs; relay_serving.rs]` | Preparation derives/validates identity; application only inserts/removes prevalidated records. `[VERIFIED: 134-CONTEXT.md D-05]` | No partial projection rollback. |
| Peer removal clears partial compact state for a wtxid but no single operation covers all request/known/orphan/package state. `[VERIFIED: packages/open-bitcoin-network/src/peer.rs and peer modules]` | One peer lifecycle operation updates all peer-local targets. `[VERIFIED: 134-CONTEXT.md D-06 and D-09]` | Identity-complete cleanup becomes testable. |
| Peer emission receipt binds peer/hash/reason/write kind only. `[VERIFIED: announcement_transport.rs]` | Family receipt also binds authority epoch, generation, effect ID, and peer session/snapshot. `[VERIFIED: 134-CONTEXT.md D-12]` | Stale and duplicate completion have explicit semantics. |
| Mempool snapshot can be captured/saved, but production dirty-generation completion is absent. `[VERIFIED: mempool_persistence.rs; storage.rs; call-site search]` | Phase 134 adds current-schema snapshot prepare/effect/complete truth only. `[VERIFIED: 134-CONTEXT.md D-11..D-13 and Phase 135 deferral]` | Phase 135 receives a safe authority/effect seam without prematurely defining recovery. |

**Deprecated/outdated for this phase:**

- Direct calls to duplicated admission/removal projection helpers as independent authorities must become facades over the shared reducer. `[VERIFIED: 134-CONTEXT.md D-01..D-02]`
- Receipt completion that records achieved state without epoch/generation/effect/session validation is insufficient for Phase 134. `[VERIFIED: current announcement_transport.rs; 134-CONTEXT.md D-12..D-13]`
- Rebuilding all caches after ordinary pressure/admission transitions is prohibited. `[VERIFIED: 134-CONTEXT.md D-10]`

## Assumptions Log

| # | Claim | Section | Risk if wrong |
| --- | --- | --- | --- |
| A1 | The recommended exact module/type names (`prepared_lifecycle.rs`, `LifecycleProjectionPlan`, `lifecycle_effects.rs`) are implementation suggestions, not locked public API. `[ASSUMED]` | Recommended Project Structure / Code Examples | Low: planner may rename them while preserving ownership and invariants. |
| A2 | A deque plus set is the recommended bounded in-memory completion-ledger representation. `[ASSUMED]` | Family-Specific Prepare → Execute → Complete | Low: another fixed-cap deterministic representation is acceptable if duplicate and eviction semantics remain tested. |
| A3 | Package-fingerprint cleanup uses a bounded reverse scan over the already capped active orphan/candidate collections rather than adding another mutable reverse index. `[VERIFIED: packages/open-bitcoin-network/src/peer/transaction_relay/orphanage.rs constants and bounded collections; 134-CONTEXT.md D-02, D-09]` | Open Questions (RESOLVED) | Low: the scan is bounded by existing orphan/candidate policy and avoids a second mutable projection. |

## Open Questions (RESOLVED)

1. **What are the exact smallest caps for each effect family and completion ledger?**
   - **RESOLVED:** peer preparation batches and the peer completed-effect ledger are capped at `PHASE94_MAX_PEER_QUEUED_MESSAGES` (`128`); snapshot preparation is capped at one pending write and the snapshot completed-effect ledger at two entries. `[VERIFIED: packages/open-bitcoin-network/src/resource.rs defines the existing 128-message peer cap; packages/open-bitcoin-node/src/network/announcement_transport.rs and packages/open-bitcoin-node/src/sync/session.rs already enforce it; packages/open-bitcoin-node/src/storage/fjall_store/mempool.rs persists one current `SNAPSHOT_KEY` value per call]`
   - The peer values reuse the nearest existing queue policy. The snapshot values are the smallest values that permit one in-flight current-schema write plus duplicate/stale discrimination for the immediately superseded write without creating a queue, journal, or Phase 135 coordinator. Boundary tests cover cap, cap+1, and deterministic oldest-entry eviction. `[VERIFIED: 134-CONTEXT.md D-13, D-15 and agent discretion]`

2. **Should package-fingerprint cleanup use a direct reverse index?**
   - **RESOLVED:** use a bounded reverse scan over the active orphan/same-peer candidate collections; do not add a direct member-to-fingerprint index. `[VERIFIED: packages/open-bitcoin-network/src/peer/transaction_relay/orphanage.rs caps total orphans at `PHASE102_MAX_ORPHAN_TRANSACTIONS = 100`, per-peer orphans at `25`, and reconsiderations per parent at `32`; 134-CONTEXT.md D-02 forbids a second mutable cache projection]`
   - Preparation records the exact fingerprints to retire, so apply remains infallible and does not scan. Tests cover ancestor cleanup, the configured bounds, and unchanged state on cap failure. No mempool-wide history is introduced. `[VERIFIED: 134-CONTEXT.md D-05, D-09, D-15]`

3. **Where should the current-schema snapshot executor live until Phase 135?**
   - **RESOLVED:** keep snapshot preparation and receipt completion in the shared `LifecycleCommand` / `ManagedNetworkHandle::apply_lifecycle_command` authority path, and put the outside-lock Fjall executor beside the existing persistence primitive in `packages/open-bitcoin-node/src/storage/fjall_store/mempool.rs` as `execute_prepared_mempool_snapshot_write`. `[VERIFIED: packages/open-bitcoin-node/src/storage/fjall_store/mempool.rs already owns `save_mempool_snapshot`; packages/open-bitcoin-node/src/storage.rs exposes `FjallNodeStore`; 134-CONTEXT.md D-01, D-11]`
   - The executor consumes the owned current-schema command, calls the existing encode/save path, and creates a receipt only after success. It does not add timer cadence, clean-shutdown policy, schema fields, recovery behavior, crash-window claims, or a durable completion journal. `[VERIFIED: 134-CONTEXT.md D-11..D-15 and Deferred Ideas]`

## Environment Availability

| Dependency | Required by | Available | Version | Fallback |
| --- | --- | --- | --- | --- |
| Rust/Cargo | Workspace implementation and tests | Yes | `1.94.1` | None; pinned source of truth. `[VERIFIED: local probes; rust-toolchain.toml]` |
| Bun | Structural checker/test | Yes | `1.3.9` | None needed. `[VERIFIED: local probe; .bun-version]` |
| Bazel/Bazelisk | Full verifier smoke build | Yes | Bazel `8.6.0` | Use repo-native verifier. `[VERIFIED: local probe; AGENTS.md]` |
| Pinned Knots submodule | Parity anchors | Yes in workspace | `29.3.knots20260210` project baseline | Re-run submodule initialization if absent. `[VERIFIED: packages/bitcoin-knots source tree; AGENTS.md Project]` |
| Fjall | Existing snapshot executor | Yes as workspace dependency | `3.1.4` | None needed; no external service required. `[VERIFIED: packages/open-bitcoin-node/Cargo.toml; Cargo.lock]` |

**Missing dependencies with no fallback:** none found. `[VERIFIED: local availability audit 2026-07-28]`

**Missing dependencies with fallback:** none found. `[VERIFIED: local availability audit 2026-07-28]`

The phase is hermetic and does not require a running database, public network, or external service. `[VERIFIED: Fjall is embedded in current node storage; 134-CONTEXT.md D-18]`

## Validation Architecture

Although `.planning/config.json` has `workflow.nyquist_validation: false`, this section is warranted because D-16 mandates a complete deterministic scenario matrix and D-17 mandates a mutation-tested structural checker; these are phase deliverables, not optional Nyquist scaffolding. `[VERIFIED: .planning/config.json; 134-CONTEXT.md D-16..D-17]`

### Test Framework

| Property | Value |
| --- | --- |
| Rust framework | Built-in Cargo tests in `open-bitcoin-node` and focused workspace crates. `[VERIFIED: packages/open-bitcoin-node/src/network/tests.rs; packages/Cargo.toml]` |
| Structural framework | `bun test` plus executable TypeScript checker. `[VERIFIED: scripts/check-phase133-package-aware-download-orphan-bridge*.ts]` |
| Config file | Workspace manifests plus `scripts/verify.sh`; no separate Rust test config. `[VERIFIED: repository test layout]` |
| Quick integration run | `bun run scripts/command-timings.ts run --key phase134-lifecycle-tests -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node lifecycle_projection_cases -- --test-threads=1` `[VERIFIED: AGENTS.md timing wrapper; proposed test module is Wave 0]` |
| Quick structural run | `bun test scripts/check-phase134-authoritative-lifecycle.test.ts && bun run scripts/check-phase134-authoritative-lifecycle.ts` `[VERIFIED: established Phase 133 checker convention; proposed Phase 134 filenames]` |
| Full suite | `bash scripts/verify.sh` `[VERIFIED: AGENTS.md Repo-Local Guidance]` |

### Phase Requirements → Test Map

| Req ID | Behavior | Test type | Automated command | File exists? |
| --- | --- | --- | --- | --- |
| MPLIFE-01 | Only handle facades mutate aggregate state; every path reaches shared reducer | structural + integration | Phase 134 Bun checker/test; focused node test command above | No — Wave 0. `[VERIFIED: D-17 requires new checker]` |
| MPLIFE-02 | Every delta updates all eight explicit target families and generation | integration + reconciliation oracle | focused node lifecycle test command | No — Wave 0. `[VERIFIED: D-16 requires new complete matrix]` |
| MPLIFE-03 | Replacement, pressure, expiry, block, reorg, failed admission leave no stale aliases/descendants | scenario integration + randomized/failure oracle | focused node lifecycle test command | Partial existing lifecycle/admission tests; complete cross-cache assertions absent. `[VERIFIED: network test inventory and source audit]` |
| MPLIFE-04 | Owned effects run outside lock; stale/duplicate/partial-success receipts preserve truth | integration + structural mutation | focused node test plus Phase 134 Bun checker/test | Existing peer emission tests are partial; generation/session/idempotency cases are new. `[VERIFIED: announcement_transport_cases.rs; D-12..D-17]` |

### Mandatory Deterministic Scenario Matrix

Each row must assert canonical mempool membership, serving txid/wtxid aliases, fanout order/state, peer request/known/orphan/package state, compact inputs, unbroadcast membership, dirty/lifecycle generation, bounded evidence, and reconciliation result. `[VERIFIED: 134-CONTEXT.md D-06, D-09, D-16]`

| Scenario | Critical extra assertion |
| --- | --- |
| full package admission | parent-before-child projection and relay preparation |
| partial package acceptance | only final-present survivors project, in admitted order |
| replacement | victims are absent from every alias; replacement bodies may enter bounded compact extras |
| pressure eviction | entire descendant package tears down descendant-first |
| expiry | all affected descendants and identities disappear |
| connected block | confirmations/conflicts retain typed causes and clear peer requests/orphans |
| reorg | transitions are applied sequentially, never coalesced |
| failed admission | empty delta changes no cache and does not advance generation |
| stale receipt | reports achieved external truth but clears no newer state |
| duplicate receipt | returns `AlreadyApplied` and changes nothing |
| partial I/O success | successful prefix is completed exactly once; failed/unsent suffix has no credit |

All matrix rows are locked validation requirements. `[VERIFIED: 134-CONTEXT.md D-16]`

### Structural Checker Mutation Matrix

| Independent mutation | Checker must reject |
| --- | --- |
| Add direct `ManagedPeerNetwork` mutation from an RPC/runtime adapter | mutation outside `ManagedNetworkHandle` |
| Call a cache-specific admission/removal helper without the reducer | shared-projector bypass |
| Delete one closed-plan target or its reducer application | omitted projection target |
| Add another mutable mempool/unbroadcast/generation owner | second authority |
| Add Fjall/socket/write call within an authority closure | I/O under lock |

Each mutation should be applied to a fresh temporary repository fixture and verified to fail independently; do not combine mutations into one test. `[VERIFIED: 134-CONTEXT.md D-17; Phase 133 mutation-test pattern]`

### Sampling Rate

- **Per task commit:** focused crate/module test plus the Phase 134 checker relevant to that task. `[VERIFIED: standards/core/verification.md]`
- **Per plan/wave merge:** all affected Cargo crates, checker test/checker, and `bun scripts/bright-builds-check.ts all`. `[VERIFIED: AGENTS.bright-builds.md; standards/core/verification.md]`
- **Phase gate:** `bash scripts/verify.sh` green, diff reviewed, generated LOC freshness accepted, and reconciliation/mutation matrix green. `[VERIFIED: AGENTS.md; 134-CONTEXT.md D-16..D-18]`

### Wave 0 Gaps

- [ ] `packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases.rs` plus `mod lifecycle_projection_cases;` registration — complete cross-cache matrix. `[VERIFIED: file absent in current test inventory]`
- [ ] Focused test helpers that inspect every target without exposing production high-cardinality evidence. `[VERIFIED: D-06 and D-16]`
- [ ] Deterministic stale authority, stale peer-session, stale persistence-generation, duplicate receipt, and partial-write fixtures. `[VERIFIED: D-12..D-16]`
- [ ] `scripts/check-phase134-authoritative-lifecycle.ts` and `.test.ts` with five independent mutations. `[VERIFIED: D-17; files currently absent]`
- [ ] `scripts/verify.sh` wiring adjacent to Phase 133 checker steps. `[VERIFIED: current verify.sh ends phase-specific sequence at Phase 133]`
- [ ] `docs/parity/source-breadcrumbs.json` entries for every new Rust source/test file. `[VERIFIED: AGENTS.md; breadcrumb scope config]`

## Security Domain

Security enforcement is enabled because `.planning/config.json` does not explicitly set it to `false`. `[VERIFIED: .planning/config.json; phase-researcher security rule]`

### Applicable ASVS Categories

| ASVS category | Applies | Standard control |
| --- | --- | --- |
| V2 Authentication | No direct auth feature | Preserve existing authenticated/direct-response boundaries; Phase 134 does not add public endpoints. `[VERIFIED: phase scope and D-18]` |
| V3 Session Management | Yes, peer session identity | Bind network receipts to the current peer-session identity so reconnects cannot complete stale commands. `[VERIFIED: 134-CONTEXT.md D-12..D-13]` |
| V4 Access Control | Yes, internal mutation authority | Keep aggregate mutation private to `ManagedNetworkHandle` and enforce it structurally. `[VERIFIED: MPLIFE-01; D-01..D-02 and D-17]` |
| V5 Input Validation | Yes | Validate identity pairs, bodies, final membership, package identity, revision/epoch, generation, and caps before mutation. `[VERIFIED: D-05]` |
| V6 Cryptography | No new cryptography | Reuse canonical consensus txid/wtxid derivation; do not introduce new cryptographic primitives. `[VERIFIED: current consensus identity helpers; phase scope]` |

### Known Threat Patterns

| Pattern | STRIDE | Standard mitigation |
| --- | --- | --- |
| Replayed or duplicated completion receipt | Spoofing / Tampering | Non-`Clone` family receipt, unique effect ID, bounded completed ledger, `AlreadyApplied`. `[VERIFIED: D-12..D-15]` |
| Receipt from an earlier authority epoch or peer session | Spoofing / Tampering | Bind epoch, generation, effect ID, and session/snapshot identity; return `AchievedButStale` without clearing current state. `[VERIFIED: D-12..D-13]` |
| Partial projection leaves stale serveable alias | Tampering | Complete preflight, closed target list, infallible apply, full reconciliation oracle. `[VERIFIED: D-05..D-10]` |
| Unbounded effect batch or completion history | Denial of Service | Fixed family caps and deterministic bounded ledger. `[VERIFIED: D-15]` |
| Network/storage I/O while authority lock is held | Denial of Service | Owned command capture, outside-lock executor, short typed completion. `[VERIFIED: D-03 and D-11]` |
| Orphan/package candidate survives ancestor removal | Tampering / Denial of Service | Descendant-first peer cleanup and identity-complete candidate/fingerprint retirement. `[VERIFIED: D-08..D-09]` |
| Transaction identifiers leak into shared evidence | Information Disclosure | Production evidence uses fixed low-cardinality aggregate labels/counts; exact IDs stay in authenticated direct responses/test helpers. `[VERIFIED: .planning/REQUIREMENTS.md MPOBS-03; D-06]` |
| Whole-mempool rebroadcast introduced accidentally | Information Disclosure / Denial of Service | Maintain bounded local-unbroadcast membership only; structural/claim guardrails reject broad rebroadcast behavior. `[VERIFIED: .planning/REQUIREMENTS.md IBR-01 and Out of Scope; D-18]` |

## Recommended Plan Decomposition

1. **Plan 134-01 — Prepared core capability and lifecycle plan types.** Add the sealed mempool prepare/consume boundary, authority epoch/generation newtypes, closed target plan, and preparation errors. Include revision/identity/final-membership/order tests before touching adapters. `[VERIFIED: dependency imposed by D-04..D-05 and current private patch seam]`
2. **Plan 134-02 — Infallible target operations.** Add serving, fanout, peer request/known/orphan/package, compact, unbroadcast, dirty-generation, and evidence operations that consume prevalidated target-specific plans. Add reconciliation inspection and per-target unit tests. `[VERIFIED: D-06..D-10]`
3. **Plan 134-03 — Route admission paths.** Move singleton, peer, local, and package admission through the shared reducer; assert full, partial, post-trim, replacement, pressure, and failed-admission matrix rows. `[VERIFIED: current admission bridge gap; D-01, D-07, D-16]`
4. **Plan 134-04 — Route maintenance paths.** Replace expiry, connected-block, reorg, and maintenance duplicate loops; preserve per-transition reorg sequencing and descendant-first teardown. `[VERIFIED: current mempool_lifecycle.rs duplication; D-01, D-08, D-16]`
5. **Plan 134-05 — Typed effects and completion.** Extend peer emission and current-schema snapshot preparation with family IDs, caps, epoch/generation/session/snapshot bindings, bounded ledgers, and the three completion outcomes; retain outside-lock successful-prefix shell execution. `[VERIFIED: D-11..D-15]`
6. **Plan 134-06 — Cross-cache integration and failure oracle.** Complete the deterministic scenario matrix, stale/duplicate/partial-I/O tests, randomized/failure-injection reconciliation oracle, and low-cardinality evidence assertions. `[VERIFIED: D-10 and D-16]`
7. **Plan 134-07 — Structural enforcement, parity, and full verification.** Add the mutation-tested TypeScript checker, verifier wiring, breadcrumbs/parity docs, README review, generated LOC freshness, Bright checker, and full `scripts/verify.sh`. `[VERIFIED: D-17..D-18; AGENTS.md]`

The dependency spine is 134-01 → 134-02 → {134-03, 134-04} → 134-05 → 134-06 → 134-07. Plans 134-03 and 134-04 can proceed in parallel only after the shared types and all target APIs stabilize. `[ASSUMED]`

## Sources

### Primary (HIGH confidence)

- `.planning/phases/134-authoritative-cross-cache-lifecycle-integration/134-CONTEXT.md` — locked scope, architecture, effects, validation, and deferrals. `[VERIFIED: local file]`
- `.planning/REQUIREMENTS.md` and `.planning/ROADMAP.md` — MPLIFE-01..04 and Phase 134 success criteria. `[VERIFIED: local files]`
- `AGENTS.md`, `AGENTS.bright-builds.md`, `standards-overrides.md`, and relevant `standards/` pages — workflow, architecture, Rust, testing, security-conscious dependency, and verification constraints. `[VERIFIED: local files]`
- `packages/open-bitcoin-mempool/src/pool/{lifecycle,patch,admission,package_admission,expiry,prospective}.rs` and `pool.rs` — delta invariants, revision-bound prepared patch, ordering, finalization, and current immediate-apply seams. `[VERIFIED: codebase inspection]`
- `packages/open-bitcoin-node/src/network.rs` and `network/{runtime_authority,admission_bridge,mempool_lifecycle,inventory,relay_serving,relay_fanout,compact_receive_candidates,announcement_transport}.rs` — aggregate ownership, duplicated projection paths, cache APIs, and current receipts. `[VERIFIED: codebase inspection]`
- `packages/open-bitcoin-network/src/peer.rs` and `peer/` modules — request scheduler, known identities, orphan/package candidates, compact partial downloads, and peer-local lifecycle state. `[VERIFIED: codebase inspection]`
- `packages/open-bitcoin-rpc/src/inbound_listener/connection_runtime.rs` — outside-lock write and successful-prefix completion pattern. `[VERIFIED: codebase inspection]`
- `packages/open-bitcoin-node/src/mempool_persistence.rs` and storage code — current snapshot/source boundary and Fjall save support. `[VERIFIED: codebase inspection]`
- `packages/bitcoin-knots/src/{txmempool.cpp,validation.cpp,net_processing.cpp,txorphanage.cpp,node/txdownloadman_impl.cpp,node/mempool_persist.cpp}` — pinned removal, accepted transaction/package, peer cleanup, orphan, relay, and persistence anchors. `[VERIFIED: pinned submodule inspection]`
- `scripts/check-phase133-package-aware-download-orphan-bridge*.ts` and `scripts/verify.sh` — mutation-tested structural checker and verifier integration pattern. `[VERIFIED: codebase inspection]`

### Secondary (MEDIUM confidence)

- None. The phase is repository-internal integration against a pinned source baseline, so local primary sources were sufficient. `[VERIFIED: research scope]`

### Tertiary (LOW confidence)

- None. Implementation-name and data-structure recommendations are isolated in the Assumptions Log rather than presented as externally verified facts. `[VERIFIED: this research artifact]`

## Metadata

**Confidence breakdown:**

- Standard stack: **HIGH** — no new dependency; versions and ownership were verified from repository pins, manifests, lockfile, and local executables. `[VERIFIED: local files and probes]`
- Architecture: **HIGH** — based on locked context plus direct inspection of the current authority, mempool patch, cache, peer, receipt, persistence, and Knots seams. `[VERIFIED: primary sources above]`
- Pitfalls: **HIGH** — each maps to a current code seam or explicit locked guardrail. `[VERIFIED: primary sources above]`
- Exact names/caps/index representation: **MEDIUM** — explicitly delegated to planner discretion and recorded as assumptions/open questions. `[VERIFIED: 134-CONTEXT.md agent discretion]`
- Validation: **HIGH** — scenario and mutation matrices are explicit phase decisions and fit existing repository test/checker infrastructure. `[VERIFIED: D-16..D-17; current tests/scripts]`

**Research date:** 2026-07-28

**Valid until:** 2026-08-27, or earlier if Phase 133/134 authority, cache, or receipt seams change. `[ASSUMED]`
