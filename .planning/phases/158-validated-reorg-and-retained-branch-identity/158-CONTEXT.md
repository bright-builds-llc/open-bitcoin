---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 158-2026-10-08T02-17-15
generated_at: 2026-10-08T02:17:15Z
---

# Phase 158: Validated Reorg and Retained Branch Identity - Context

**Gathered:** 2026-10-07 CDT
**Status:** Ready for planning
**Mode:** Yolo

Material guidance: `AGENTS.md`, `AGENTS.bright-builds.md`, placeholder-only `standards-overrides.md`, architecture, code-shape, testing, verification and Rust standards. Both active lessons were loaded completely (7,188 bytes; 2,397 estimated tokens); the existing audit baseline has no new trigger. Main was clean and synchronized with origin, and the pinned Knots submodule was initialized. Defer all commits to the strict wrapper's clean verification gate; do not bypass hooks.

<domain>
## Phase Boundary

Own CFIX-03: continuous consensus-validated common-ancestor rewind and replacement filters/headers, including equal-height replacements; immutable displaced records addressable by hash after real Fjall reopen; explicit refusal of missing retained reorg body/undo inputs with conservative progress and protection. Preserve the shipped ordered catch-up owner and ordinary chainstate/prune authority.
</domain>

<decisions>
## Implementation Decisions

### Validated branch transition

- **D-01:** Extend the existing serialized staged reorg acceptance path and BASIC ordered owner. A reorg must autonomously restore branch-correct ordered work; requiring reopen or manual re-enable to resume is insufficient.
- **D-02:** Distinguish preview, accepted replacement and durable fenced replacement. Preview is not acceptance, and a success-only callback cannot capture accepted state followed by persistence failure. Preserve current mempool/chainstate ordering while preventing stale prepared index work from publishing.
- **D-03:** Identify the common ancestor by ancestry/hash, including equal-height replacements and catch-up behind the fork. Rewind active progress to the verified common prefix and derive replacement headers from that ancestor; never append from the displaced tip or select a record by height alone.
- **D-04:** Advance generation/branch identity through a trusted transition and invalidate stale prepared turns. Keep one ordered owner and bounded scheduled turns; avoid an index-owned duplicate full-history cache or a reopen/reseed workaround on each reorg.

### Retained identity and durability

- **D-05:** Keep immutable filter records keyed by block hash. Replace only the active projection/checkpoint suffix. Previously indexed displaced records and their original branch headers remain addressable after reorg, ordinary flush, actual drop and Fjall reopen.
- **D-06:** Index progress must not commit ahead of recoverable coins/chain metadata. Extend the proven validated-lineage/fence capability to accepted replacement branches; preserve accepted-unflushed behavior rather than forcing a coins flush on every connect/reorg solely to avoid crash loss.
- **D-07:** Persistence failures retain conservative protection and truthful progress. Interrupted projection/rewind writes recover a verified prefix or fail closed, without phantom advancement or destruction of immutable displaced history.

### Required inputs and refusal

- **D-08:** Validate all genuinely required retained bodies and body-bound historical undo for the reorg/index transition before preview or destructive effects. Missing body and missing undo must refuse explicitly, preserve the valid prefix and conservative protection, and leave no partial successful branch transition.
- **D-09:** Use genuine validated staged facts for replacement work, including historical and same-block spends. Never reconstruct from current coins, borrow another branch's undo/filter, skip a height, invent empty filters, silently redownload or repair. Already valid immutable records need not regenerate solely because their source payloads were subsequently pruned.
- **D-10:** Preserve index-owned reserved prune locks, generation checks and interrupted-prune startup ordering. Operator lock changes cannot weaken required reorg/index protection.

### Evidence and scope

- **D-11:** Extend continuous validated spend/fork fixtures on concrete Fjall and durable coins. Cover common-ancestor rewind, equal-height replacement, longer replacement, catch-up lag, real reopen with active/hash lookup, missing body/undo and stale-work/persistence failures. Sparse codec-valid, memory-only and source-string checks are supplementary evidence.
- **D-12:** Add required Rust source breadcrumbs and parity documentation, deterministic claim checks wired into native verification, and contributor README/UAT updates. Run the default `bash scripts/verify.sh`, independent source review, security mitigation review and formal lifecycle validation before final commit/push. UAT uses copy-pasteable repo-local Cargo/Bazel commands.

### Agent Discretion

Exact transition types, finite work limits after measurement, fault seams and plan decomposition belong to research/planning. Prefer the smallest robust change to existing core/shell seams. No new production dependency or crate. This is a headless lifecycle phase; no frontend contract is needed.
</decisions>

<canonical-refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

- `.planning/ROADMAP.md`, `.planning/REQUIREMENTS.md`, `.planning/PROJECT.md` — Phase 158, CFIX-03, evidence and exclusions.
- `.planning/research/ARCHITECTURE.md`, `.planning/research/PITFALLS.md`, `.planning/research/FEATURES.md` — milestone architecture and reorg hazards.
- `.planning/phases/154-basic-generation-and-commitment-parity/154-CONTEXT.md` — complete historical input facts.
- `.planning/phases/155-recoverable-index-and-pre-prune-startup-protection/155-CONTEXT.md` — immutable records, common-prefix recovery and durable fencing.
- `.planning/phases/156-index-owned-manual-and-automatic-prune-coordination/156-CONTEXT.md` — reserved prune ownership and generation.
- `.planning/phases/157-safe-activation-and-scheduled-index-catch-up/157-CONTEXT.md` — bounded owner, accepted facts, safe progress and scheduler.
- `packages/bitcoin-knots/src/index/base.cpp`, `packages/bitcoin-knots/src/index/blockfilterindex.cpp`, `packages/bitcoin-knots/src/validation.cpp`, `packages/bitcoin-knots/src/undo.h` — pinned rewind, retained lookup, accepted reorg and historical undo.
- `packages/open-bitcoin-node/src/chainstate.rs`, `packages/open-bitcoin-node/src/chainstate/filter_index.rs`, `packages/open-bitcoin-node/src/network/mempool_lifecycle.rs` — preview/acceptance/persistence and ordinary production consumers.
- `packages/open-bitcoin-node/src/storage/fjall_store/filters/append.rs`, `packages/open-bitcoin-node/src/storage/fjall_store/filters/lifecycle.rs` — guarded publication, retention and lifecycle.
- `AGENTS.md`, `AGENTS.bright-builds.md`, `standards-overrides.md`, `standards/index.md`, `docs/parity/source-breadcrumbs.json`, `scripts/verify.sh` — contributor contract and verification.
- https://bips.dev/157/ — ancestry-dependent filter header commitments; pinned local Knots remains behavioral authority.
</canonical-refs>

<code-context>
## Existing Code Insights

### Reusable Assets

Validated staged reorg; immutable hash rows; common-prefix recovery; pure catch-up progress and generation checks; SyncAll projection publication; durable coins/metadata fencing; reserved index prune lock; continuous validated spend fixtures.

### Established Patterns

Functional core decisions and thin Fjall/runtime adapters, serialized chainstate/prune authority, finite work budgets, conservative failure, and truthful durable versus accepted progress.

### Integration Points

The existing reorg preview invalidates BASIC ownership and validated lineage before acceptance/persistence. Research must bridge this boundary, restore branch-aware flush authority and replace conflicting active suffix projections without relaxing immutable row checks.
</code-context>

<specifics>
## Specific Ideas

Index a continuous validated spending branch, prepare an old-branch turn, accept a fork sharing a known ancestor, refuse the stale turn and derive replacement headers. Drop/reopen actual Fjall and compare active-height lookup with displaced hash lookup. Repeat with equal-height tips, lagged index progress and genuine missing reorg inputs.
</specifics>

<deferred>
## Deferred Ideas

Filter/index RPC (159), peer serving/service bits (160), broad dashboard/support projections (161), full post-prune client integration (162). No pending todos matched. V0/type 2, BIP37, archive serving, automatic repair/import/download, public defaults and production/funds claims remain excluded.
</deferred>
