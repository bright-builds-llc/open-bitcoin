---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 159-2026-10-09T16-11-49
generated_at: 2026-10-09T16:15:15Z
---

# Phase 159: Authenticated BASIC Filter and Index RPCs - Context

**Gathered:** 2026-10-09 CDT
**Status:** Ready for planning
**Mode:** Yolo — one recommendation pass with two focused advisors.

Material guidance: AGENTS.md, AGENTS.bright-builds.md, placeholder-only standards-overrides.md, architecture/code-shape/testing/verification/Rust standards; both active lesson inputs fully read (7,188 bytes, 2,397 estimated tokens), no audit trigger. Main was clean and synchronized; pinned Knots initialized. Strict wrapper defers all workflow commits until clean verification; preserve hooks. Bun 1.3.9 is available at /tmp/open-bitcoin-bun-1.3.9-fresh/bun.

<domain>
## Phase Boundary

Own CFRP-01 and CFRP-02: authenticated BASIC getblockfilter and getindexinfo through the actual daemon/RPC shared durable authority, with pinned results, selection, error ordering, retained stale/pruned hash lookup and initial-sync semantics. Broader operator status and peer transport remain Phases 160–162.
</domain>

<decisions>
## Implementation Decisions

### Pinned request and error contract

- **D-01:** Use existing baseline-origin node-scoped authenticated RPC routing. Support positional and named blockhash/filtertype/index_name, omitted/null defaults, arity/help and pinned argument type/hash errors. Dedicated normalization must preserve framework type-check precedence rather than generic serde -32602 defaults.
- **D-02:** After framework checks, getblockfilter parses hash, resolves filter type, checks enabled index, resolves known block and connected validity, obtains readiness, then attempts immutable filter/header lookup. Unknown type is -5 `Unknown filtertype`; disabled BASIC is -1 `Index is not enabled for filtertype basic`; unknown block is -5 `Block not found`. Knots recognizes v0 but it is disabled/out of scope here; preserve its disabled error without implementing V0. Numeric names and uppercase BASIC are unknown.
- **D-03:** Successful stored lookup wins during catch-up and for retained stale/pruned blocks. Only failed lookup classifies never-connected as -5 `Filter not found. Block was not connected to active chain.`, initial indexing as -1 `Filter not found. Block filters are still in the process of being indexed.`, otherwise -32603 `Filter not found. This error is unexpected and indicates index corruption.` Backend failures/corruption never become successful empty filters or ordinary absence.
- **D-04:** Return exactly filter/header lowercase hex; header is uint256 display order, reversed from raw commitment bytes. Preserve exact names and codes for malformed hash length/hex and -3 type errors based on pinned local source.

### Shared read authority and provenance

- **D-05:** Prefer narrow typed read/query methods on existing ManagedNetworkHandle and the same configured Fjall authority. No independent RPC-owned index, duplicate full-history cache, detached owner, request-side history regeneration or implicit activation. Extract a pure classifier only if it clarifies actual decision inputs.
- **D-06:** Research and prove genuine ever-connected/scripts-valid provenance for known-but-unconnected and displaced-connected blocks, including missing filter rows after prune/reopen. Active membership, header/body/undo presence and an assumed fixture bit cannot replace accepted validation provenance. Preserve accepted-before-persistence facts and fail-closed recovery.
- **D-07:** Request reads must be bounded and measured. Current full-ancestry getters remain recovery validators; add a narrow integrity-preserving request path instead of walking entire history. Prefer authority-held point reads; prepare/read/checked-completion is discretionary only if measurements justify its complexity. Missing/corrupt predecessor/commitment evidence must not silently serve forged headers.

### Index summary and readiness

- **D-08:** getindexinfo returns only `basic block filter index` with `synced` and `best_block_height`. Omitted/null/empty selection returns all enabled in-scope indexes; exact name selects BASIC; unmatched name or disabled index returns {}. No invented txindex/coinstatsindex capability.
- **D-09:** synced is BasicIndexProgress::initially_synchronized(), preserving the initial-sync latch through later lag/reorg. best_block_height is processed progress, zero fallback, not safe durable checkpoint or current tip. Recover/reopen semantics follow pinned baseline and existing conservative durable reconciliation.
- **D-10:** Distinguish summary latch from getblockfilter readiness. Pinned BaseIndex returns promptly before initial completion, and later drains validation notifications. Research an equivalent bounded owner/barrier without waiting under the RPC context mutex for scheduled work or doing unbounded request-triggered indexing. Later pending work cannot be falsely diagnosed as corruption solely because synced is true.

### Authentication and evidence

- **D-11:** Reuse auth-before-JSON/context-lock HTTP boundary. Prove missing/wrong credentials on malformed requests disclose no index state; cookie/password success, batches/notifications and node scope follow existing transport. No credentials, datadir paths or raw backend details in public diagnostics.
- **D-12:** Prove production daemon configuration, actual shared dispatch, valid continuous-chain active/stale/pruned retrieval after real paired deletion and reopen, initial catch-up available/missing rows, later lag, known unconnected and connected absence, publication/backend corruption faults and concurrent lifecycle outcomes. Sparse codec fixtures alone cannot establish acceptance. Include focused parser/pure tests, exact result keys and negative precedence controls.
- **D-13:** Add parity breadcrumbs and scoped docs, review relevant READMEs and provide repo-local Cargo/Bazel UAT commands. Run native bash scripts/verify.sh, Bright Builds, source/security review and lifecycle validation before final commit/push. Keep production dependencies unchanged and public/default activation off.

### Agent Discretion

Exact types, provenance storage/recovery compatibility, read budget and barrier design, fixture decomposition and plan boundaries belong to research/planning. Roadmap UI hint is a generic indicator: this phase owns JSON RPC, no frontend; record the UI-gate applicability decision and do not invent a GUI/dashboard. No destructive repair, public-network verification or production/funds claims.
</decisions>

<canonical-refs>
## Canonical References

Downstream agents MUST read these before planning or implementation.

- `.planning/ROADMAP.md`, `.planning/REQUIREMENTS.md`, `.planning/PROJECT.md` — Phase 159, CFRP-01/02 and scope boundaries.
- `.planning/phases/154-basic-generation-and-commitment-parity/154-CONTEXT.md`, `.planning/phases/155-recoverable-index-and-pre-prune-startup-protection/155-CONTEXT.md`, `.planning/phases/156-index-owned-manual-and-automatic-prune-coordination/156-CONTEXT.md`, `.planning/phases/157-safe-activation-and-scheduled-index-catch-up/157-CONTEXT.md`, `.planning/phases/158-validated-reorg-and-retained-branch-identity/158-CONTEXT.md` — historical inputs, immutable records, fences, owner and validated reorg decisions.
- `.planning/research/ARCHITECTURE.md`, `.planning/research/PITFALLS.md`, `.planning/research/FEATURES.md` — shared authority and failure boundaries.
- `packages/bitcoin-knots/src/rpc/blockchain.cpp`, `packages/bitcoin-knots/src/rpc/node.cpp`, `packages/bitcoin-knots/src/rpc/util.cpp`, `packages/bitcoin-knots/src/rpc/server.cpp`, `packages/bitcoin-knots/src/blockfilter.cpp`, `packages/bitcoin-knots/src/index/base.cpp`, `packages/bitcoin-knots/src/index/blockfilterindex.cpp` — pinned exact RPC parsing, errors, summary, readiness and retained lookup. Local pin a9aee730466ac67d35a3c03ee24676be5e045878 is authoritative over current web guidance.
- `packages/open-bitcoin-node/src/network/runtime_authority/filter_index.rs`, `packages/open-bitcoin-node/src/chainstate/filter_index.rs`, `packages/open-bitcoin-node/src/storage/fjall_store/filters.rs`, `packages/open-bitcoin-node/src/storage/filter_index.rs`, `packages/open-bitcoin-chainstate/src/filter_index/catch_up.rs` — progress/owner/storage implementation.
- `packages/open-bitcoin-rpc/src/context.rs`, `packages/open-bitcoin-rpc/src/method.rs`, `packages/open-bitcoin-rpc/src/method/normalize.rs`, `packages/open-bitcoin-rpc/src/dispatch.rs`, `packages/open-bitcoin-rpc/src/error.rs`, `packages/open-bitcoin-rpc/src/http.rs` — actual transport and dispatch integration.
- `AGENTS.md`, `AGENTS.bright-builds.md`, `standards-overrides.md`, `standards/index.md`, `docs/parity/source-breadcrumbs.json`, `scripts/verify.sh` — repository contract.
</canonical-refs>

<code-context>
## Existing Code Insights

Reusable assets: BASIC initial-sync/processed/safe progress, immutable hash rows, validated receipt/reorg authority, configured daemon catch-up, authenticated HTTP and typed method registry. Established patterns: functional core with thin effectful adapters, finite work budgets, one serialized owner and conservative recovery. Integration gaps: existing getters walk ancestry, and scripts-valid provenance/readiness must be researched explicitly.
</code-context>

<specifics>
## Specific Ideas

Serve an already-indexed row while genesis-prefix catch-up is incomplete; preserve synced=true during subsequent accepted tip lag; reorg, actually prune paired payloads, drop/reopen Fjall and compare original stale hash commitments through authenticated HTTP. Contrast known header-only, connected missing row, malformed record and unknown block without substituting body availability for validation.
</specifics>

<deferred>
## Deferred Ideas

Peer compact-filter transport, broad operator/status/dashboard projections and full integrated milestone proof belong to Phases 160–162. V0, automatic repair/download, public defaults, archive-scale/funds/production claims remain deferred. No pending todos matched this phase.
</deferred>
