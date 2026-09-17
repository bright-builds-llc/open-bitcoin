---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 143-2026-09-17T01-47-45
generated_at: 2026-09-17T01:47:56.500Z
---

# Phase 143: Honest Stored-Block Availability - Context

**Gathered:** 2026-09-17
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

The node serves or reports a stored block as Available only when the
payload bytes are present, and refuses cleanly as Unavailable when they
are not. Inventory, serve, and any existing RPC/report path must classify
from a payload-byte probe plus typed facts (`payload_present`,
`index_known`, `validated_on_active_chain`). Coins tip or header index
alone cannot authorize a serve.

This phase must not implement prune-mode product behavior, emit `Pruned`
unless prune mode actually deleted files (prune is deferred, so this
phase never emits it on production paths), claim archive-node or
production-scale historical serving, add public serving defaults, invent
a new operator evidence rollout (Phase 144), close parity-root
no-claim guardrails (Phase 145), or add assumeutxo, compact-filter
serving, or production-readiness claims.

</domain>

<decisions>
## Implementation Decisions

### durable_availability probe shape
- **D-01:** Classify a block as Available only after a payload-byte probe
  succeeds. The probe means the block body bytes are present in the live
  `blocks_by_hash` cache or the durable block store. Coins tip, header
  index membership, and `index_known` cannot authorize `Available` or a
  serve.
- **D-02:** Replace the caller-supplied `durable_availability: bool`
  shortcut. `gate_inventory_for_durable_serving` currently passes `true`
  unconditionally and that must become a real probe result. Cache
  presence counts as `payload_present`; it is not a substitute for a
  missing durable body when the cache is empty.
- **D-03:** Keep `classify_block_serving_status` I/O-free (110 D-09/D-10).
  The shell adapter probes, then feeds typed facts into the existing
  classifier. Do not move disk or cache reads into the network-core
  policy functions.
- **D-04:** Serving still requires peer eligible + status Available +
  local payload bytes (111 D-05). A successful classification that later
  fails to read bytes must refuse as Unavailable, not serve a fabricated
  body.

### Reserved Pruned label
- **D-05:** Do not emit `Pruned` or `block_status_pruned` on production
  paths in this phase. Prune-mode product behavior is deferred (FUT-18).
  Missing payload on an indexed, active, or non-tip block is
  `Unavailable`, not `Pruned`.
- **D-06:** Keep the `Pruned` enum variant reserved so a later prune
  phase can use it when prune mode actually deleted files. Help text,
  docs, and `as_str` must not be readable as "this node is in prune
  mode" or "historical blocks were pruned."
- **D-07:** Flip tests that currently expect `Pruned` for
  active-non-tip-missing-data to `Unavailable`. ROADMAP research flag
  for this phase is the shape of `durable_availability` and this reserved
  `Pruned` rule.

### Operator facts versus Phase 144 surfaces
- **D-08:** Introduce typed facts `payload_present`, `index_known`, and
  `validated_on_active_chain`. These are the HAVL-03 contract and must
  be distinguishable on the classification/report seam.
- **D-09:** `index_known` means the hash is present in a local
  header/block index. `validated_on_active_chain` means the block is a
  validated position on the active chain. Neither implies
  `payload_present`.
- **D-10:** Phase 143 wires these facts into inventory, serve, compact
  txn serve, and any existing RPC/status field that already reports
  stored-block availability. Full status / RPC / CLI / dashboard /
  metrics / logs / support rollout of flush and have-bytes evidence is
  Phase 144 (CSOBS-01/CSOBS-02). Do not build a new operator UI here.
- **D-11:** If research finds no existing `getblock` or equivalent
  stored-block RPC, do not invent a new getblock product. The "reports"
  surface is the existing classification/evidence seam plus any already
  shipped block-availability field.

### Inventory and RPC refuse contract
- **D-12:** Inventory, serve, and compact-txn paths refuse missing
  payload with the existing NotFound / missing-inventory machinery and
  an `Unavailable` status label. Do not invent a new wire error and do
  not emit `Pruned`.
- **D-13:** Public-default serving, archive-node claims, compact-filter
  serving, and production readiness stay unchanged (110 D-01/D-04/D-18
  and 111 D-13/D-16). Missing-payload refuse must not be documented as
  archive-node honesty or a public-default serving change.
- **D-14:** Verification remains `bash scripts/verify.sh`, deterministic,
  and public-network-free. Historical-serving review stays opt-in UAT
  guidance only.
- **D-15:** New or touched first-party Rust source/test files under
  `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests` need
  parity breadcrumbs in file comments and
  `docs/parity/source-breadcrumbs.json`, using `none` only when no
  defensible Knots anchor exists.

### Claude's Discretion
- Exact type names and whether the three facts extend
  `BlockServingStatusFacts` or sit beside it.
- Whether the probe is a named trait, a block-store method, or a thin
  adapter helper — as long as D-01 through D-03 hold.
- How compact-announcement inputs that already reuse
  `managed_block_serve_input` inherit the same honesty rule.
- Copy tweaks that keep reserved `Pruned` from reading as prune-mode
  without renaming the enum.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase contract
- `.planning/REQUIREMENTS.md` — HAVL-01, HAVL-02, HAVL-03; FUT-18 prune
  deferred; out-of-scope archive/public-default/production claims
- `.planning/ROADMAP.md` — Phase 143 goal, success criteria, and
  research flag: shape of `durable_availability` and reserved `Pruned`
- `.planning/PROJECT.md` — v2.3 honest-availability target; no
  archive-node or public-default claim

### Locked prior decisions
- `.planning/phases/110-block-serving-activation-and-eligibility-boundary/110-CONTEXT.md`
  — I/O-free classifier, sanitized pruned/unavailable labels, default-off
  serving, no archive claim (D-09 through D-12, D-18)
- `.planning/phases/111-full-block-serving-request-path/111-CONTEXT.md`
  — eligible + Available + local data required; historical/pruned
  bounded, not archive-node (D-05, D-06, D-12, D-13)
- `.planning/phases/142-manager-flush-lifecycle-and-restart/142-CONTEXT.md`
  — deferred honest payload-present availability and reserved `Pruned`
  to this phase

### Current dishonest seam
- `packages/open-bitcoin-node/src/network/inventory.rs` —
  `managed_block_serve_input` maps active-non-tip-missing to `Pruned`
  and ORs cache presence with a caller `durable_availability` bool;
  `gate_inventory_for_durable_serving` passes `true` unconditionally
- `packages/open-bitcoin-network/src/block_serving.rs` — I/O-free
  `classify_block_serving_status` and reserved `Pruned` /
  `BlockStatusPruned` labels
- `packages/open-bitcoin-node/src/network/block_serving.rs` — shell
  serve/gate adapter that consumes classifier facts
- `packages/open-bitcoin-node/src/network/announcement_transport.rs` —
  compact announcement reuse of `managed_block_serve_input`
- `packages/open-bitcoin-node/src/network/action_translation.rs` —
  compact-txn serve reuse of the same facts

### Knots and parity anchors
- `packages/bitcoin-knots/src/node/blockstorage.cpp` — HaveBlockData /
  pruned-versus-missing block-file availability
- `packages/bitcoin-knots/src/validation.cpp` — active-chain and
  validated-block availability
- `docs/parity/catalog/p2p.md` — block-serving / blockstorage parity
  notes
- `docs/parity/catalog/chainstate.md` — blockstorage.cpp catalog entry
- `docs/parity/source-breadcrumbs.json` — breadcrumb contract for new
  first-party Rust files

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `classify_block_serving_status` / `BlockServingStatusFacts` in
  `open-bitcoin-network` — keep as the I/O-free policy; change the facts
  the shell supplies, not the "policy owns I/O" boundary.
- `serve_managed_block_request` / `gate_managed_block_request` in
  `open-bitcoin-node/src/network/block_serving.rs` — already require
  Available before a cache read; they inherit honesty from input facts.
- Existing NotFound / missing-inventory path — reuse for missing-payload
  refuse.

### Established Patterns
- Functional core / imperative shell: probe in the node adapter, classify
  in network-core.
- Default-off block serving and sanitized low-cardinality labels from
  Phases 110–111.
- `Unavailable: {reason}` rendering on operator surfaces; Phase 143
  should emit honest labels, not a new renderer family.

### Integration Points
- `ManagedPeerNetwork::managed_block_serve_input` is the fact-assembly
  seam for inventory, durable gate, compact announcement, and compact
  txn serve.
- `gate_inventory_for_durable_serving(..., durable_availability: true)`
  is the known dishonest production call.
- Tests under `packages/open-bitcoin-node/src/network/tests/block_serving.rs`
  and `packages/open-bitcoin-network/src/block_serving/tests/status_cases.rs`
  encode the current Pruned-for-missing-active-body behavior.

</code_context>

<specifics>
## Specific Ideas

- ROADMAP research flag: pin `durable_availability` as a payload-byte
  probe result and reserve `Pruned` so help text cannot be read as
  prune-mode.
- STATE concern: Phase 143 planning should pin `durable_availability`
  and reserve `Pruned` so help text cannot be read as prune-mode.
- Current code already names the bool `durable_availability` but treats
  it as a caller override. The name can stay if the value becomes a
  probe; do not keep the override.

</specifics>

<deferred>
## Deferred Ideas

- Status / RPC / CLI / dashboard / metrics / logs / support flush and
  have-bytes evidence rollout — Phase 144
- Parity-root closeout and no-claim guardrails — Phase 145
- Prune/archive product modes, assumeutxo, compact-filter serving,
  public defaults, production readiness — FUT-18 through FUT-26
- New `getblock` product RPC, if none already exists

None of these were folded into Phase 143.

</deferred>

---

*Phase: 143-honest-stored-block-availability*
*Context gathered: 2026-09-17*
