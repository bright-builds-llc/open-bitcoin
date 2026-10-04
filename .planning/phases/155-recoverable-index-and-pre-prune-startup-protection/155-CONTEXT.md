---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 155-2026-10-04T16-01-08
generated_at: 2026-10-04T16:01:08Z
---

# Phase 155: Recoverable Index and Pre-Prune Startup Protection - Context

**Gathered:** 2026-10-04
**Status:** Ready for planning
**Mode:** Yolo

Material guidance: `AGENTS.md`, `AGENTS.bright-builds.md`, placeholder-only `standards-overrides.md`, architecture/code-shape/testing/verification/local-guidance and Rust standards; both active lesson inputs read fully. Existing coins authority and startup prune ordering constrain the design.

<domain>
## Phase Boundary

Implement integrity-checked recoverable BASIC index records and safe durable progress in the existing Fjall store. Reconcile active index projection to recovered chainstate and validate index-owned protection before production startup resumes interrupted pruning. Own CFIX-02, CFIX-04 and CFPR-03 only.
</domain>

<decisions>
## Implementation Decisions

### Storage compatibility and integrity
- **D-01:** Use additive versioned index records in the existing Fjall database, preserving current schema-2 coins and old datadir compatibility. Add no production crate, dependency or storage engine.
- **D-02:** Keep immutable BASIC records keyed by block hash, with block/parent/height identity, filter bytes/hash/header and predecessor commitment. Validate encoding, bounded lengths and commitment/ancestry consistency; identical rewrites may be idempotent, conflicting immutable records must refuse.
- **D-03:** Distinguish immutable records from active height projection and durable resume authority. Preserve valid displaced or ahead records on recovery/refusal; never erase a prefix or fabricate a gap to recover.

### Chainstate fence and fault ordering
- **D-04:** Fence the safe cursor by recovered coins best-block plus compatible durable active-chain metadata and ancestry. Header tip, SyncProgress, status counters and persisted filter height alone are insufficient authority.
- **D-05:** When records precede a successful coins/metadata flush, retain those records but rewind/reconcile active projection and resume cursor to the recovered branch/checkpoint. A missing/inconsistent authority refuses explicitly.
- **D-06:** Persist index records/checkpoint/protection through concrete atomic durable batches where the same-database preconditions permit it. Separate effects must publish valid records/checkpoint before relaxing protection. Failures may retain extra history; they must not create a phantom cursor or release required input.

### Pre-prune startup protection
- **D-07:** After coins recovery establishes the durable authority, validate/reconcile saved index metadata and index-owned protection before `initialize` can call `resume_prune_intent`. Manager construction after initialize is too late.
- **D-08:** Protect the earliest still-required body/undo input conservatively using a reserved internal index identity. Missing, corrupt or unsafe protection/checkpoint combinations refuse before deletion and preserve payloads, live intent and immutable prefix. Retain the existing finish-or-Repair refusal behavior.
- **D-09:** Require the production `DurableSyncRuntime` reopen path to consume this guard. A test-installed lock after runtime construction or an unused helper does not satisfy CFPR-03. Absent index metadata preserves legacy startup behavior.

### Evidence and scope
- **D-10:** Prove successful/interrupted writes, ahead-of-chainstate rows, wrong branch, corrupt record/checkpoint/protection, and unsafe live prune intent using real Fjall close/reopen and production runtime startup. Include record/checkpoint/protection persistence fault seams; memory-only or source-string proof is supplementary.
- **D-11:** Preserve pure recovery/transition decisions outside I/O, with thin store/startup shells and behavior tests using Arrange/Act/Assert. Register all new Rust parity breadcrumbs and run the full native verifier before finalization.

### the agent's Discretion

Exact additive keyspace/envelope types, bounded record limits, explicit fault-injection seams and plan decomposition are researcher/planner choices. Prefer small modules and reuse existing typed BASIC commitments, Fjall batches, coins authority and prune locks. Do not expand into public activation, scheduler or operator product surfaces.
</decisions>

<canonical-refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Scope and safety
- `.planning/ROADMAP.md` — Phase 155 criteria and later-phase ownership.
- `.planning/REQUIREMENTS.md` — CFIX-02, CFIX-04, CFPR-03 and exclusions.
- `.planning/PROJECT.md` — single-chainstate and no-repair/no-production boundaries.
- `.planning/research/ARCHITECTURE.md` — concrete startup call chains and fenced progress/protection protocol.
- `.planning/research/PITFALLS.md` — startup deletion, ahead-of-chainstate cursor and fault hazards.
- `.planning/phases/154-basic-generation-and-commitment-parity/154-CONTEXT.md` — BASIC identities and complete historical input decisions.
- `.planning/milestones/v2.4-MILESTONE-AUDIT.md` — retained prune advisories, not authorization for unrelated cleanup.

### Baseline and first-party integration
- `packages/bitcoin-knots/src/index/base.cpp` — processed/committed progress, durable chain flush fence and prune locks.
- `packages/bitcoin-knots/src/index/blockfilterindex.cpp` — records, commitments and retained branch rows.
- `packages/bitcoin-knots/src/node/blockstorage.cpp` — pruning protection behavior.
- `packages/open-bitcoin-node/src/storage/fjall_store.rs` — additive concrete store and batch conventions.
- `packages/open-bitcoin-node/src/storage/fjall_store/coins.rs` — recovered coins checkpoint authority.
- `packages/open-bitcoin-node/src/storage/fjall_store/prune.rs` — paired deletes and resumed prune intent.
- `packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs` — initialize recovery and pre-resume ordering.
- `packages/open-bitcoin-node/src/sync/open_runtime.rs` — production durable startup consumer.
</canonical-refs>

<code-context>
## Existing Code Insights

### Reusable Assets

Phase 154's BASIC generator and typed filter hashes/headers; existing Fjall same-database batches and SyncAll durability; recovered coins best-block and chain metadata; durable prune-lock records and explicit fail-closed errors.

### Established Patterns

Pure domain decisions and thin shell adapters; immutable block-hash identity; single serialized chainstate/prune authority; legacy datadir migration is explicit and non-destructive; independent parity evidence and native verification.

### Integration Points

`initialize` recovers coins, then loads locks and immediately resumes prune intent before ReadyToFlush. `DurableSyncRuntime::open_with_runtime_activation` calls initialize before loading the managed chainstate. The guard belongs before prune resume, with production reopen proof. Index records written ahead must reconcile against coins and chain metadata rather than the later network/header state.
</code-context>

<specifics>
## Specific Ideas

Exercise a reopened store containing both a lagging saved index and a live prune intent. Required body and non-genesis undo survive or startup refuses before deleting either. Capture safe cursor and immutable rows separately after every injected boundary.
</specifics>

<deferred>
## Deferred Ideas

Phase 156 owns reserved operator CRUD, actual manual/automatic prune coordination and disable/re-enable transitions. Phase 157 owns public option parsing, activation/history preflight and scheduled catch-up. Phase 158 owns validated runtime reorg orchestration. Phases 159–162 own RPC, peers, operator surfaces and integrated retained-history proof. No pending todos matched this phase.
</deferred>
