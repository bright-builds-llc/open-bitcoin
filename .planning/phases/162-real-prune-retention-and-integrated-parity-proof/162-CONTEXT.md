---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 162-2026-10-11T16-25-37
generated_at: 2026-10-11T16:28:27Z
---

# Phase 162: Real Prune Retention and Integrated Parity Proof - Context

**Gathered:** 2026-10-11
**Status:** Ready for planning
**Mode:** Yolo; recommended decisions selected under the invoked strict-push workflow.

<domain>
## Phase Boundary

Close CFPR-02, CFGR-01 and CFGR-02 with a reproducible continuous consensus-validated history through the existing production startup, idle scheduler, connect/reorg, manual and automatic prune, durable reopen, authenticated RPC and inbound/outbound compact-filter transport callers. Preserve all existing scope exclusions and distinguish scoped implementation completion from milestone shipment/audit.
</domain>

<decisions>
## Implementation Decisions

### Continuous history and actual production callers

- **D-01:** Extend the existing normal-maturity validated spend/fork recipe past height 1,000. Read actual runtime consensus parameters, retain normal 100-block coinbase maturity, continuous parentage, real historical and same-block spends, real durable coins and accepted connect receipts. Sparse codec-valid fixtures, MemoryCoinsView-only proofs and maturity-one histories cannot satisfy the integrated acceptance gate.
- **D-02:** Establish one identifiable scenario/history across actual configured startup, receive-independent scheduled catch-up, ordinary accepted connect, shallow reorg with displaced hash lookup, legal manual pruning, ordinary automatic retention, all-handle datadir close/reopen and client responses. Share recipes/artifacts where crate privacy requires composition, but prove provenance and every handoff; do not substitute unrelated tests for the end-to-end lifecycle.
- **D-03:** Exercise network prune-after and trailing-window rules through normal planning/apply callers. Automatic target stays the legal 550 MiB minimum. Codec-valid nonactive auxiliary rows may supply explicit storage pressure only; they are not validated-history evidence. Report their purpose and bounds separately. Prefer the simplest bounded recipe over hundreds of expensive irrelevant accepted blocks.
- **D-04:** Demonstrate real paired body/undo absence after deletion and reopen, unchanged durable coins/checkpoint identity and exact retained immutable BASIC bytes/hash/header. Include displaced records and at least one positive 1,000-height checkpoint; an empty checkpoint response is insufficient evidence.

### Client service and restrictions

- **D-05:** Use authenticated real RPC dispatch/server wiring and actual inbound/outbound socket adapters. Exercise getblockfilter/getindexinfo and all three peer families (getcfilters, getcfheaders, getcfcheckpt) after paired prune/reopen. Compare exact replies against independently pinned expected commitments, ancestry and ordering; source-only plans or prepared/enqueued replies are insufficient.
- **D-06:** Prove complete transport writes earn receipts and partial/failed writes do not. Preserve finite work, byte, queue and cleanup policies and both caller directions. General serving and effective per-peer permissions remain explicit; public listeners/defaults are not activated by tests or status collection.
- **D-07:** Retained filters never restore ordinary old-body serving or wallet eligibility. Exercise real callers for positive retained availability and negative body/wallet restrictions, with unchanged prior wallet/checkpoint state on refusal. Assert truthful NODE_COMPACT_FILTERS/NODE_NETWORK_LIMITED coexistence where relevant.

### Fault/refusal evidence and parity

- **D-08:** Cover fresh and saved/lagged missing body/undo, interrupted pending prune intent, cursor ahead of recovered durable chainstate, filter index/protection/checkpoint write failure, corrupted immutable records and partial writes with concrete Fjall/durable-coins observations. No fabricated progress, invented empty success, lost valid prefix or successful write receipt is permitted.
- **D-09:** Software fault seams must sit at actual production effect boundaries, remain test-only where possible, and preserve the existing serialized owner and safe coins-fenced cursor. Do not introduce another authority, production crate, dependency, storage engine, repair/import/download mode or unbounded fixture cache.
- **D-10:** Reuse pinned Knots source and existing independent BASIC/peer oracles. Update source breadcrumbs, parity roots and deterministic claim/mutation guards with exact evidence links. Keep Fjall layout, BASIC-only/type-0 scope and V0 exclusion, earlier missing-undo refusal, local resource-policy and modeled implicit-permission limitations explicit.
- **D-11:** Deliberately transition current-document historical guards that require Phase162-pending text, retaining their substantive safety/privacy/security checks and adding negative controls for unsupported claims. Preserve archived reports at their original evidence boundary. A completion sentence alone cannot activate CFPR-02 or CFGR requirements.

### Operator documentation and release gates

- **D-12:** Update relevant current README/operator/parity docs and give copy-pasteable repo-local Cargo and Bazel UAT commands, bounded runtime/space expectations and exact evidence limits. Reuse the Phase161 shared operator presentation; no new screen, GUI, navigation, formatter schema or wallet feature is needed.
- **D-13:** Perform an explicit simplification pass, independent source and declared-mitigation review, required native claim/mutation checks, full default `bash scripts/verify.sh` and lifecycle-valid formal verification. Record failures and reruns honestly. Use timing wrappers for ad-hoc Cargo/Bazel and serialize jobs against shared targets.
- **D-14:** Keep all nested discussion/planning/execution commits deferred until the strict wrapper's clean verification gate. Finalize the already checked-out `main` with mandatory hooks, then push its upstream. Do not bypass hooks or create a branch. Never activate requirements based on plans or stale verification.
- **D-15:** Preserve BIP37/V0, assumeutxo/dual-chainstate, archive-scale, public-default/public-network-CI, implicit repair/import/download and production-service/readiness/funds exclusions. Existing v2.4 crash-window advisories are unchanged. Phase completion does not automatically archive or ship the milestone; subsequent milestone audit remains separate.

### Agent Discretion

Exact helper/file boundaries, test-only seams and reproducible fixture composition may follow research. Favor existing modules, explicit provenance and focused assertions. No pending todos matched Phase162.
</decisions>

<canonical-refs>
## Canonical References

**Downstream agents must read relevant references before planning or implementing.**

### Scope and instructions

- `AGENTS.md`, `AGENTS.bright-builds.md`, `standards-overrides.md` — repo workflow and managed standards routing.
- `standards/core/architecture.md`, `standards/core/code-shape.md`, `standards/core/testing.md`, `standards/core/verification.md`, `standards/core/local-guidance.md`, `standards/languages/rust.md`, `standards/languages/typescript-javascript.md` — implementation and verification contracts.
- `.planning/PROJECT.md`, `.planning/ROADMAP.md`, `.planning/REQUIREMENTS.md`, `.planning/STATE.md` — scoped requirements and inherited limits.
- `.planning/phases/161-shared-operator-index-and-retention-evidence/161-CONTEXT.md` — inherited operator contract; other milestone contexts supply locked underlying behavior.

### Existing genuine lifecycle evidence and adapters

- `packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/fixtures.rs` — normal-maturity accepted spend/fork recipe.
- `packages/open-bitcoin-node/src/sync/tests/filter_index/evidence/retention.rs` — concrete checkpoint/reopen and retention evidence.
- `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/fixtures.rs` — actual daemon startup/RPC fixture; existing custom maturity does not satisfy D-01.
- `packages/open-bitcoin-node/src/sync/tests/peer_filter_transport.rs` — concrete socket evidence patterns.
- `scripts/verify.sh`, `.githooks/pre-commit`, `scripts/command-timings.ts` — native contract, mandatory hook and cooperative lock.
- `scripts/check-phase161-operator-index-evidence.ts` — historical pending-claim assumptions requiring deliberate compatible transition.

### Parity and protocol

- `docs/parity/index.json`, `docs/parity/source-breadcrumbs.json`, `docs/parity/catalog/basic-compact-filters.md`, `docs/parity/v2-5-validated-reorg.md` — current parity evidence roots.
- `packages/bitcoin-knots/src/blockfilter.cpp`, `packages/bitcoin-knots/src/index/blockfilterindex.cpp`, `packages/bitcoin-knots/src/index/base.cpp`, `packages/bitcoin-knots/src/node/blockstorage.cpp`, `packages/bitcoin-knots/src/net_processing.cpp`, `packages/bitcoin-knots/src/rpc/blockchain.cpp` — pinned behavioral source.
- `https://github.com/bitcoin/bips/blob/master/bip-0157.mediawiki` and `https://github.com/bitcoin/bips/blob/master/bip-0158.mediawiki` — primary protocol overview consulted during discussion; pinned Knots remains the implementation baseline.
</canonical-refs>

<code-context>
## Existing Code Insights

Reuse actual DurableSyncRuntime/Fjall and accepted-state ownership, configured startup/catch-up, persisted branch queries, authenticated daemon RPC and both peer adapters. Existing phase-specific evidence already covers many constituent seams; this phase strengthens continuous provenance, normal-maturity retention and complete post-prune clients. Existing automatic pressure recipes explicitly use nonactive codec-only rows. Current Phase161 claim guards pin pending prose and require a tested transition.
</code-context>

<specifics>
## Specific Ideas

Nonempty checkpoint evidence and ordinary network pruning thresholds require a continuous history above height 1,000. Storage pressure and accepted-chain evidence remain separately named. Preserve achieved-write proof and fail-closed durable cursor observations.
</specifics>

<deferred>
## Deferred Ideas

All excluded surfaces in D-15, unrelated v2.4 advisory cleanup, milestone archive/shipment and new operator UI. No folded todos or scope expansion.
</deferred>
