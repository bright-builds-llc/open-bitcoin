---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 114-2026-07-05T20-44-12
generated_at: 2026-07-05T20:44:40.345Z
---

# Phase 114: Compact Block Reconstruction from Mempool State - Context

**Gathered:** 2026-07-05
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Phase 114 adds pure compact-block reconstruction from current mempool candidates and bounded extra or recent-block transaction inputs using BIP152 witness-hash short IDs. The phase owns short-ID selector derivation, `PartialCompactBlock` volatile state, mempool/extra matching, stable typed outcomes for collision, duplicate, missing, malformed, and failure cases, and lifecycle hooks for mempool removal and block connect cleanup.

This phase may consume decoded `cmpctblock` payloads from Phase 112, compact relay negotiation context from Phase 113, and mempool transaction candidates through iterator inputs. It must not schedule `getblocktxn` requests, accept `blocktxn` responses, perform FillBlock validation handoff, mutate chainstate from partial compact state, add broad operator evidence rollout, enable package relay, enable bloom/filter or compact-filter serving, change public defaults, or claim production full-node readiness.

</domain>

<decisions>
## Implementation Decisions

### Short ID And Selector Surface

- **D-01:** BIP152 short-ID selector keys derive from `SHA256(header || nonce)` little-endian `u64` key halves, matching Knots `FillShortTxIDSelector`.
- **D-02:** Witness-hash short IDs use SipHash-2-4 over the 256-bit wtxid masked to six bytes (`& 0x0000_FFFF_FFFF_FFFF`), implemented in `open-bitcoin-consensus` and composed with codec selector types.
- **D-03:** Six-byte wire short IDs remain a codec newtype with explicit `short_id_match_key` for hash-map lookups; do not store short IDs as unmasked `u64` in wire encode/decode paths.

### Reconstruction State Model

- **D-04:** `PartialCompactBlock` is volatile, peer-scoped state with explicit slots for prefilled and matched transactions plus tracked wtxids for duplicate detection.
- **D-05:** `init_partial_compact_block` mirrors Knots `PartiallyDownloadedBlock::InitData`: prefilled placement, short-ID map construction with collision and bucket-overload checks, mempool scan, then bounded extra-transaction scan.
- **D-06:** Stable outcomes are `Ready { missing_indexes }`, `Invalid(...)`, and `Failed(ShortIdCollision | ShortIdBucketOverload)` with low-cardinality reason enums suitable for later operator evidence.
- **D-07:** Duplicate short-ID matches at the same slot clear the slot (mark missing) rather than silently picking a winner; extra-transaction duplicates compare witness hashes before clearing.

### Input Boundaries

- **D-08:** Mempool and extra inputs are `(Wtxid, Transaction)` iterator inputs so `open-bitcoin-network` stays independent of `open-bitcoin-mempool` crate coupling; node shell adapters supply iterators later.
- **D-09:** Transaction count is bounded by `MAX_BLOCK_WEIGHT / MIN_SERIALIZABLE_TRANSACTION_WEIGHT` and short-ID bucket size is capped at 12 entries per bucket, matching Knots reconstruction guards.
- **D-10:** Malformed structural payloads should fail before partial state is left initialized; invalid init paths call `clear()` on the partial state.

### Lifecycle Integration

- **D-11:** `on_mempool_transaction_removed` clears slots whose stored wtxid matches the removed transaction.
- **D-12:** `on_block_connected` clears all volatile partial compact-block state without touching chainstate or durable block data.
- **D-13:** Lifecycle hooks are pure methods on `PartialCompactBlock`; wire scheduling, peer in-flight maps, and validation handoff remain Phase 115.

### Scope Isolation

- **D-14:** No `getblocktxn`, `blocktxn`, FillBlock, full-block fallback, misbehavior disconnect policy, or validation/connect integration in this phase.
- **D-15:** Compact reconstruction must not activate package relay, bloom/filter serving, compact filters, or public serving defaults.
- **D-16:** New first-party Rust source/test files require parity breadcrumbs and `docs/parity/source-breadcrumbs.json` updates unless an explicit `none` breadcrumb is defensible.

### Claude's Discretion

The planner may choose exact type names, whether lifecycle helpers live beside reconstruction or in a peer submodule, and how tests build synthetic compact blocks. Prefer small pure APIs, deterministic Arrange/Act/Assert tests, and iterator-based inputs over shell effects.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Project And Phase Scope

- `AGENTS.md` — verification, parity breadcrumbs, architecture constraints.
- `AGENTS.bright-builds.md` — functional core, testing, verification workflow.
- `standards/core/architecture.md` — functional core / imperative shell boundaries.
- `standards/core/code-shape.md` — early returns, file size, readability.
- `standards/core/testing.md` — focused unit tests.
- `standards/core/verification.md` — `bash scripts/verify.sh` contract.
- `standards/languages/rust.md` — Rust module and naming rules.
- `.planning/PROJECT.md` — v2.1 scope and parity value.
- `.planning/REQUIREMENTS.md` — RCN-02, RCN-03, GOV-04 for Phase 114.
- `.planning/ROADMAP.md` — Phase 114 goal, success criteria, plan split.

### Prior Locked Decisions

- `.planning/phases/112-bip152-wire-codec-and-message-semantics/112-CONTEXT.md` — BIP152 payload codec boundary.
- `.planning/phases/113-compact-relay-negotiation-and-announcement-policy/113-CONTEXT.md` — compact negotiation and announcement-only scope.

### Existing Code Integration Points

- `packages/open-bitcoin-codec/src/compact_block.rs` — `CompactBlockPayload`, prefilled expansion, short-ID wire types.
- `packages/open-bitcoin-consensus/src/crypto.rs` — SHA256, wtxid hashing, compact short-ID composition.
- `packages/open-bitcoin-network/src/compact_reconstruction.rs` — reconstruction state and init API.
- `packages/open-bitcoin-network/src/peer/compact_relay.rs` — negotiation state consumed later by peer wiring.
- `docs/parity/source-breadcrumbs.json` — breadcrumb registry.

### Bitcoin Knots Anchors

- `packages/bitcoin-knots/src/blockencodings.h` — `PartiallyDownloadedBlock`, `CBlockHeaderAndShortTxIDs`.
- `packages/bitcoin-knots/src/blockencodings.cpp` — InitData collision, mempool, and extra-txn matching.
- `packages/bitcoin-knots/test/functional/p2p_compactblocks.py` — reconstruction behavior examples.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets

- Phase 112 codec types and prefilled index expansion helpers.
- Phase 113 compact relay negotiation remains separate from reconstruction inputs.
- `transaction_wtxid` and block header encoding in consensus/codec crates.

### Established Patterns

- Pure network policy modules with sibling `tests.rs` files.
- Iterator-based inputs to avoid shell dependencies in functional core.
- Low-cardinality typed outcome enums for later operator evidence.

### Integration Points

- Node shell will later pass mempool iterators and extra-transaction buffers into `init_partial_compact_block`.
- Peer manager will later own per-peer `PartialCompactBlock` maps and call lifecycle hooks on mempool/block events.

</code_context>

<specifics>
## Specific Ideas

- Treat missing transaction indexes as explicit `Ready` output for Phase 115 `getblocktxn` scheduling.
- Keep partial state cleared on block connect to prevent chainstate coupling.
- Use fixed outcome labels such as `short_id_collision`, `short_id_bucket_overload`, and `missing_transactions`.

</specifics>

<deferred>
## Deferred Ideas

Missing-transaction request scheduling, `blocktxn` response matching, FillBlock validation, full-block fallback, misbehavior disconnect policy, operator RPC/CLI/dashboard evidence, parity/UAT release closeout, package relay, bloom/filter serving, compact filter serving, public serving defaults, and production readiness claims remain outside Phase 114.

</deferred>

---

*Phase: 114-compact-block-reconstruction-from-mempool-state*
*Context gathered: 2026-07-05*
