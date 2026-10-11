---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 160-20261010-peer-filter-serving
generated_at: 2026-10-10T18:34:06Z
---

# Phase 160: Explicit Bounded Peer Filter Serving - Context

**Gathered:** 2026-10-10 CDT
**Status:** Ready for research and planning
**Mode:** Yolo — one recommendation pass; no interactive approval.

Material guidance: AGENTS.md, AGENTS.bright-builds.md, placeholder-only standards-overrides.md, architecture/code-shape/testing/verification/Rust standards; the parent loaded both active lesson inputs in full (7,188 bytes, no audit trigger) and synchronized clean main with origin/main before this workflow. The outer strict wrapper defers every commit/push until clean phase verification and preserves hooks.

<domain>
## Phase Boundary

Own CFNET-01 through CFNET-06: explicitly enabled, BASIC-only BIP157 peer serving for all three request/response families, branch-aware persisted reads, capability service bits and bounded real inbound/outbound transport with achieved-write evidence. Preserve the existing single durable authority, index/prune lifecycle, default-off listeners/public service and independent relay activation. Broad operator projection is Phase161; full integrated prune/reopen client proof is Phase162.
</domain>

<decisions>
## Implementation Decisions

### Activation, permissions and service bits

- **D-01:** Support explicit general BASIC peer serving through a narrowly owned configuration/daemon surface anchored to pinned `peerblockfilters`. General serving and an explicitly requested `blockfilters` permission require an enabled BASIC index; refuse invalid configuration before networking starts or destructive startup effects. Index activation alone does not enable peer serving or listeners.
- **D-02:** Derive `NODE_COMPACT_FILTERS` from configured general or permitted per-peer capability before initial catch-up completes. Observe actual version messages on inbound and outbound paths. A synchronized latch, current tip equality or a successful query is not the service-bit authority. Compose truthfully with prune-derived `NODE_NETWORK_LIMITED`; do not erase the existing network-service classification.
- **D-03:** Preserve requested-versus-expanded permission provenance. Pinned Knots distinguishes `BlockFilters_Explicit` from the `all`-expanded `BlockFilters` flag; implicit whitelist expansion omits blockfilters. Explicit `blockfilters` must enforce the BASIC prerequisite. Preserve the pinned `all` corner: `all` alone is accepted with a disabled index, grants effective per-peer BlockFilters and advertises that peer's service bit; a valid request finding no backend returns without a normal response. Combining `all` with explicitly named `blockfilters` still requires BASIC. `all` never initializes a disabled index, listener, BIP37/V0 product or public default. Do not silently equate explicit and expanded flags or infer synchronized/backend availability from this permission-derived advertisement; document this precise baseline corner. With BASIC enabled, effective `all` capability still obeys finite resource limits. Existing unpermissioned/implicit/noban/download/protected peers gain no per-peer filter privilege by implication.
- **D-04:** Keep peer-serving eligibility separate from transaction relay, block-body serving and compact-block relay. Existing per-direction permission classes and actual inbound/outbound callers must consume one typed capability decision. No global service bit may be inferred from a single permissioned peer.

### Pinned validation and branch selection

- **D-05:** Preserve `PrepareBlockFilterRequest` ordering: BASIC plus the peer's advertised service capability, known/allowed stop hash, start not above stop, inclusive range within the family limit, then index lookup. Unsupported type/unadvertised capability, invalid/disallowed stop, reversed and oversized ranges disconnect as pinned. Knots V0/type2 is unsupported on this BASIC peer surface even though its local RPC recognition differs.
- **D-06:** Legal inclusive limits are 1,000 cfilters and 2,000 cfheaders entries. Cover exact limit and limit-plus-one, height arithmetic, genesis, malformed/trailing wire payloads, unknown type and combined-invalid precedence. Do not substitute RPC errors for peer wire/disconnect behavior.
- **D-07:** Resolve ranges from the requested stop block's ancestry, including retained stale branches permitted by pinned `BlockRequestAllowed`: active membership, or scripts-valid stale provenance with the pinned best-header timestamp and proof-equivalent-age tests. Hash existence, present body, current-height lookup and retained filter existence alone do not authorize stale serving. Pruned body absence must not disqualify an otherwise permitted indexed filter.
- **D-08:** cfilters return exact ordered `(type, block hash, filter bytes)` for the entire inclusive range. cfheaders returns the requested stop hash, predecessor header at start minus one (zero only for start zero), and ordered raw filter hashes. Preserve wire/raw digest byte order separately from RPC display hex.
- **D-09:** cfcheckpt returns headers at heights 1000, 2000, and subsequent positive multiples through the stop height on that branch; genesis is excluded. Stops below1000 produce an empty valid checkpoint vector. Resource bounds must account for vector bytes, ancestry/work and queued transport; do not impose the 1,000/2,000 contiguous-range caps on checkpoint stop heights.

### Persisted reads and bounded complete preparation

- **D-10:** Use the existing configured durable `ManagedNetworkHandle` authority and immutable hash-keyed records, with narrow integrity-preserving request reads. Do not create an independent filter database/full-history cache, regenerate from block bodies/current coins, mutate indexing on request, repair/download history or wait for whole-index initial synchronization.
- **D-11:** Serve already-indexed complete permitted ranges during catch-up. Resolve and validate the complete response before emitting its first wire message. A missing record/predecessor/checkpoint in an otherwise valid request yields no fabricated empty filter and no shortened successful response; pinned index lookup failure sends no normal response. Backend corruption/fault remains a distinct fail-closed internal outcome rather than ordinary absence or success.
- **D-12:** Define measured finite per-request/per-peer/aggregate work, record-read, copy/encoded-byte and queue budgets before allocating large response collections. Count ancestry traversal and point-read integrity checks, rather than only output entries. Permissioned and protected peers remain bounded. Local exhaustion behavior and any differences from Knots are explicit parity policy, not an invented protocol limit. Exact numeric defaults and narrow API design belong to research/planning.
- **D-13:** Capture branch/lifecycle identity with immutable prepared data, prevent stale plans or duplicate completion from earning live evidence, and write sockets outside the shared authority/context lock. Keep accepted/durable progress, initial synchronization and network capability distinct. Preserve existing owner/prune generation and conservative recovery rather than adding another serialized owner.

### Transport receipts and cleanup

- **D-14:** Integrate all three families into real inbound and outbound message dispatch/transport paths. A prepared plan, encoded message, enqueue, successful policy decision or passed mock alone is not a completed serving receipt.
- **D-15:** Credit message/bytes only after the real transport achieves a complete framed write. Full-range success requires every intended message; cfilter prefix writes may be recorded accurately, but a partial frame or failed later write cannot credit that frame or full range. Retry/replay cannot double-credit an already completed receipt.
- **D-16:** Queue pressure, timeout, cancellation, disconnect, partial-write failure and lifecycle invalidation abort the unwritten suffix and release reservations/in-flight state. Preserve truthful already-written-prefix evidence without continuing stale/unwritten work. Bound total buffered suffix bytes and do not hold the authority lock while waiting for transport backpressure.

### Evidence, docs and verification

- **D-17:** Prove observed version bits, prerequisite/refusal matrix, explicit/all/implicit permissions, active/recent-stale/disallowed stale branches, exact range limits, checkpoint intervals, catch-up available/missing ranges, corruption, concurrent lifecycle change and cleanup through production callers. Use deterministic wire fixtures plus genuine validated-chain concrete Fjall/durable-coins histories. Independent pinned Knots helpers/source supply expected bytes and contracts.
- **D-18:** Include successful and forced partial/failed transport writes on both actual inbound and outbound adapter paths; source-string checks or testing only a standalone serving helper cannot establish CFNET-06. Use loopback/injected production transport seams and deterministic clocks; default verification remains hermetic/public-network-free.
- **D-19:** Register Rust source/test parity breadcrumbs, update scoped parity/docs and relevant README/UAT, and include copy-pasteable repo-local Cargo/Bazel commands. Native `bash scripts/verify.sh`, independent source/security review, explicit simplification pass and lifecycle-valid final verification gate the outer commit/push.
- **D-20:** Roadmap UI hint is protocol/header/operator terminology. No frontend/browser/UI-SPEC contract applies; this phase changes the existing headless config/protocol surface. Broader status/CLI/dashboard/support index projections remain Phase161. Record only minimal shared truthful transport evidence necessary to prove this phase.

### Agent Discretion

Research/planning may choose exact config field names within the pinned/general-serving vocabulary, typed capabilities, codec/module boundaries, measured finite budgets, transport receipt implementation, fixtures and plan decomposition. Preserve existing dependencies/crates and pure-core versus effectful-shell layering. Preserve the explicit/all/implicit baseline distinction captured above; do not expand BIP37/V0, whitelist compatibility or public defaults.
</decisions>

<canonical-refs>
## Canonical References

Downstream agents MUST read these before planning or implementation.

### Scope and inherited contracts

- `.planning/ROADMAP.md` — Phase160, dependency/evidence gates and Phase161/162 ownership.
- `.planning/REQUIREMENTS.md` — CFNET-01 through CFNET-06 and v2.5 exclusions.
- `.planning/PROJECT.md` — single-chainstate, explicit activation and claim boundaries.
- `.planning/phases/154-basic-generation-and-commitment-parity/154-CONTEXT.md` — BASIC bytes/commitments and independent oracle.
- `.planning/phases/155-recoverable-index-and-pre-prune-startup-protection/155-CONTEXT.md` — immutable records and durable fences.
- `.planning/phases/156-index-owned-manual-and-automatic-prune-coordination/156-CONTEXT.md` — reserved protection and lifecycle generations.
- `.planning/phases/157-safe-activation-and-scheduled-index-catch-up/157-CONTEXT.md` — configured catch-up and accepted-state owner.
- `.planning/phases/158-validated-reorg-and-retained-branch-identity/158-CONTEXT.md` — validated ancestry and retained stale identity.
- `.planning/phases/159-authenticated-basic-filter-and-index-rpcs/159-CONTEXT.md` — shared query integrity/provenance/readiness and independent initial-sync meaning.
- `.planning/phases/136-receive-independent-maintenance-and-transport-receipts/136-CONTEXT.md` — achieved transport receipts and partial-failure cleanup.
- `.planning/research/ARCHITECTURE.md`, `.planning/research/PITFALLS.md` — authority, bounds and failure hazards.

### Pinned behavioral source

- `packages/bitcoin-knots/src/net_processing.cpp` — InitializeNode, BlockRequestAllowed, PrepareBlockFilterRequest and three ProcessGetCF handlers.
- `packages/bitcoin-knots/src/init.cpp` — peerblockfilters prerequisite and explicit permission validation.
- `packages/bitcoin-knots/src/net_permissions.h`, `packages/bitcoin-knots/src/net_permissions.cpp`, `packages/bitcoin-knots/src/net.cpp` — explicit/all/implicit permission distinctions.
- `packages/bitcoin-knots/src/protocol.h`, `packages/bitcoin-knots/src/protocol.cpp` — service bit and wire command contracts.
- `packages/bitcoin-knots/src/blockfilter.h`, `packages/bitcoin-knots/src/index/blockfilterindex.cpp` — BASIC serialization, ancestry lookup and checkpoint intervals.
- `packages/bitcoin-knots/test/functional/p2p_blockfilters.py`, `packages/bitcoin-knots/test/functional/test_framework/messages.py` — independent protocol/helper evidence. Exact local pin `a9aee730466ac67d35a3c03ee24676be5e045878` is the behavioral authority.

### First-party integration and repository contract

- `packages/open-bitcoin-network/src/message.rs`, `packages/open-bitcoin-network/src/inbound/permissions.rs` — codec/dispatch and requested-versus-expanded permission vocabulary.
- `packages/open-bitcoin-node/src/network/runtime_authority/filter_index/query.rs`, `packages/open-bitcoin-node/src/storage/fjall_store/filters/query.rs` — shared bounded persisted reads and integrity checks.
- `packages/open-bitcoin-node/src/network/announcement_transport.rs`, `packages/open-bitcoin-rpc/src/inbound_listener/connection_runtime.rs` — achieved writes, locks and cleanup patterns.
- `packages/open-bitcoin-rpc/src/context/inbound_wire.rs`, `packages/open-bitcoin-rpc/src/config/loader/blockfilter.rs` — actual inbound routing and configured BASIC startup.
- `docs/parity/catalog/basic-compact-filters.md`, `docs/parity/index.json`, `docs/parity/source-breadcrumbs.json` — scoped behavior and auditable differences.
- `AGENTS.md`, `AGENTS.bright-builds.md`, `standards-overrides.md`, `standards/index.md`, `scripts/verify.sh` — local guidance, standards and complete native gate.
</canonical-refs>

<code-context>
## Existing Code Insights

### Reusable Assets

`BasicFilterQuery`/`BasicIndexSummary`, bounded point reads with predecessor integrity, immutable branch/hash records, validation provenance, accepted/generation authority and existing inbound write acknowledgement/peer emission receipts. Existing typed permission sets retain requested tokens separately from effective expansions.

### Established Patterns

Pure policy before effects; one durable authority; opaque completion capabilities; transport outside authority locks; measured resource governance; no fabricated successful absence; tests through ordinary daemon/receive/transport callers.

### Integration Points

The network wire enum currently lacks the three BASIC peer families. Existing blockfilters is parsed as inactive and must become conditionally active only through the new BASIC serving capability. General/per-peer service bits must reach real handshakes, and shared persisted reads must connect to both inbound and outbound transport adapters with finite cleanup-aware reservations.
</code-context>

<specifics>
## Specific Ideas

Validate a continuous spending chain and a permitted fork, retain original branch records, start a lagged index and request a fully indexed prefix while initial synchronization is false. Observe exact wire bytes and configured service bits in real handshakes. Request legal ranges crossing a missing row and prove zero response messages; then force the second cfilter write to fail and prove one written-prefix receipt, no full-range success and no unwritten suffix reservation. Check positive1000-height checkpoint boundaries independently of contiguous range caps.
</specifics>

<deferred>
## Deferred Ideas

Broad operator/status/dashboard/support rollout (Phase161); complete production-startup/prune/reopen/authenticated-RPC/all-peer-family integrated client proof (Phase162). V0/type2, BIP37, archive-scale claims, filter import, wallet acceleration, automatic repair/download, public defaults/network CI, production service/readiness and funds safety remain excluded. No pending todos matched Phase160.
</deferred>
