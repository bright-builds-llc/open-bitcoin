---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 156-2026-10-04T20-28-36
generated_at: 2026-10-04T20:28:36Z
---

# Phase 156: Index-Owned Manual and Automatic Prune Coordination - Context

**Gathered:** 2026-10-04
**Status:** Ready for planning
**Mode:** Yolo

Material guidance: `AGENTS.md`, `AGENTS.bright-builds.md`, placeholder-only `standards-overrides.md`, architecture, code-shape, testing, verification, local-guidance and Rust standards. Both active lesson files were read completely (7,188 bytes, 2,397 conservative estimated tokens); no audit trigger fired. Main was clean and synchronized with origin before writing.

<domain>
## Phase Boundary

Own CFPR-01: protect every BASIC index input from actual manual and ordinary automatic deletion until recoverable filter records plus the chainstate-fenced durable checkpoint permit release. Enforce reserved ownership and explicit internal disable/re-enable ordering. Preserve the Phase 155 pre-resume startup guard. Public activation, catch-up scheduler, reorg orchestration, RPC filter lookup, peer serving and broader operator evidence belong to later phases.
</domain>

<decisions>
## Implementation Decisions

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
</decisions>

<canonical-refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Scope and prior decisions

- `.planning/ROADMAP.md` — Phase 156 criteria, dependencies and evidence gates.
- `.planning/REQUIREMENTS.md` — CFPR-01 and later-phase exclusions.
- `.planning/PROJECT.md` — single-chainstate and no-repair/no-production boundaries.
- `.planning/research/ARCHITECTURE.md` — shared owner, deletion gates and lifecycle ordering.
- `.planning/research/PITFALLS.md` — stale protection, false cursor authority and faults.
- `.planning/phases/155-recoverable-index-and-pre-prune-startup-protection/155-CONTEXT.md` — immutable storage, coins fence and pre-resume protection.
- `.planning/phases/153-automatic-prune-retention-integration/153-CONTEXT.md` — ordinary automatic retention and logical target limits.
- `.planning/phases/150-operator-prune-surfaces-and-evidence/150-CONTEXT.md` — operator lock CRUD contracts.
- `.planning/milestones/v2.4-MILESTONE-AUDIT.md` — retained advisories and honest limits.

### Baseline and implementation

- `packages/bitcoin-knots/src/index/base.cpp` — index lifecycle, committed progress and prune protection.
- `packages/bitcoin-knots/src/index/blockfilterindex.cpp` — immutable BASIC records and checkpoint commitments.
- `packages/bitcoin-knots/src/node/blockstorage.cpp` — manual/automatic pruning and locks.
- `packages/open-bitcoin-node/src/storage/fjall_store/prune.rs` — concrete paired deletion and locks.
- `packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs` — startup recovery and prune resume.
- `packages/open-bitcoin-node/src/sync/open_runtime.rs` — durable runtime startup consumer.
- `docs/parity/source-breadcrumbs.json` — required new Rust source provenance.
- `scripts/verify.sh` — full native verification and generated LOC freshness contract.

</canonical-refs>

<code-context>
## Existing Code Insights

### Reusable Assets

Existing BASIC immutable records, fenced checkpoints, atomic SyncAll publication, reserved lock identity, Fjall shared-store coordination and pre-resume startup validation. Manual authority callers already reload locks; ordinary automatic planning keys measurements by locks.

### Established Patterns

Pure decisions plus fallible shell effects, serialized chainstate/prune owner, recoverable durable coins authority, immutable rows separate from active projection, conservative failure and real-store reopen evidence.

### Integration Points

Guard both operator CRUD and low-level store replacement. Concrete paired deletion needs a final owner-state/lock check under publication synchronization. Periodic retention throttle must observe ownership changes. Internal lifecycle transitions feed later activation/catch-up without exposing those products now.
</code-context>

<specifics>
## Specific Ideas

Preplan deletion, change or stall the index, then apply through the real owner/store and prove required payload pairs survive. After durable safe progress, prove eligible pairs delete; after faults, prove extra retention. Disable with outstanding work, reject its later publication, then re-enable and reopen with protection established before work.
</specifics>

<deferred>
## Deferred Ideas

Public BASIC configuration and history activation preflight, scheduled catch-up (157), runtime reorg (158), RPC (159), peers (160), operator projections (161) and complete post-prune client proof (162). No pending todos matched this phase. Unrelated v2.4 advisory cleanup is excluded.
</deferred>
