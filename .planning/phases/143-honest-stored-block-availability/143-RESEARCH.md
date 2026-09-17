# Phase 143: Honest Stored-Block Availability - Research

**Researched:** 2026-09-17
**Domain:** Node-shell payload-byte probe, I/O-free block-serving classifier, reserved `Pruned` label
**Confidence:** HIGH

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
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

### Deferred Ideas (OUT OF SCOPE)
- Status / RPC / CLI / dashboard / metrics / logs / support flush and
  have-bytes evidence rollout — Phase 144
- Parity-root closeout and no-claim guardrails — Phase 145
- Prune/archive product modes, assumeutxo, compact-filter serving,
  public defaults, production readiness — FUT-18 through FUT-26
- New `getblock` product RPC, if none already exists

None of these were folded into Phase 143.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| HAVL-01 | Node serves or reports a stored block as Available only when the payload bytes are present. | Replace `durable_availability: true` with a cache-or-store payload-byte probe before classification. Do not authorize Available from coins tip or header index. |
| HAVL-02 | When the payload is absent, the node refuses cleanly with Unavailable and does not emit Pruned unless prune mode actually deleted files. | Map active-non-tip-missing to `Unavailable`. Keep `Pruned` reserved. Reuse `WireNetworkMessage::NotFound`. Flip the Phase 111 missing-body test and checker name. |
| HAVL-03 | Operator evidence distinguishes payload_present, index_known, and validated_on_active_chain. | Assemble the three facts in the shell fact-assembly seam and expose them on the existing classification/report decision. Do not add Phase 144 operator surfaces. |
</phase_requirements>

## Summary

Phase 143 is an honesty fix on an already-shipped serve path, not a new storage or RPC product. The I/O-free classifier in `open-bitcoin-network` is correct: `Available` only when injected `data_availability` is `Available`. The lie is in the node shell. `managed_block_serve_input` ORs cache presence with a caller bool, and `gate_inventory_for_durable_serving` always passes `true`. That classifies an active block as Available before any durable body exists. Missing cache on an active non-tip is also mapped to `Pruned`, which Knots reserves for `m_have_pruned && !BLOCK_HAVE_DATA && nTx > 0`. This repo has no prune-mode product (FUT-18), so that mapping is a label lie.

There is no `getblock` RPC. The existing report surface is `ManagedBlockServeDecision.status_label` plus `BlockServingStatusCounters`. `getblockchaininfo` does not report per-block availability. Do not invent `getblock`. Compact announcement and compact-txn already call `managed_block_serve_input` with `durable_availability: false`; they inherit honesty once fact assembly probes cache (and durable store only on the deferred inbound gate).

**Primary recommendation:** Probe payload presence in the node shell (`blocks_by_hash.contains_key` OR `FjallNodeStore`/`DurableBlockSource` key presence), feed `data_availability: Available` only when that probe is true, never emit `Pruned` on production paths, and keep `Pruned` as a reserved unused variant whose help text cannot be read as prune-mode.

## Project Constraints (from .cursor/rules/)

No `.cursor/rules/` directory exists in this repository. [VERIFIED: workspace glob]

Actionable constraints from `AGENTS.md`, `AGENTS.bright-builds.md`, and managed standards that bind this phase:

- Functional core / imperative shell: classifier stays I/O-free; probe stays in `open-bitcoin-node` / RPC inbound adapters. [CITED: standards/core/architecture.md]
- `maybe_` prefix for optional names; `let...else` for guards; no `unwrap()`. [CITED: standards/languages/rust.md]
- New modules use `foo.rs` + `foo/`, not `foo/mod.rs`. [CITED: standards/languages/rust.md]
- File-length trigger ~628 lines (`floor(100 * tau)`); function-length trigger ~161. `inventory.rs` is 411, node `block_serving.rs` is 624, `fjall_store.rs` is 623. Prefer tests in existing `tests/block_serving.rs` rather than growing the 624-line adapter. [VERIFIED: wc -l]
- Unit-test pure mapping and fact assembly as Arrange / Act / Assert, one concern per test. [CITED: standards/core/testing.md]
- Verification is `bash scripts/verify.sh`. Do not overlap Cargo jobs against the same target directory. Run ad-hoc Cargo through `bun run scripts/command-timings.ts run --key <stable-key> -- <command>`. [CITED: AGENTS.md Repo-Local Guidance]
- New or touched first-party Rust sources need parity breadcrumbs. Knots anchors for this seam are `blockstorage.cpp`, `validation.cpp`, and `net_processing.cpp`. [CITED: AGENTS.md]
- No rust-bitcoin, no LevelDB, no new production crates. Extend Fjall `3.1.4`. [CITED: REQUIREMENTS.md Out of Scope]
- `standards-overrides.md` has no active local exceptions. [VERIFIED: standards-overrides.md]

## Standard Stack

This phase adds no libraries. Use the pinned first-party stack.

### Core

| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| Rust | 1.94.1 (2024 edition) | Language / toolchain | Pinned by `rust-toolchain.toml` and workspace. [VERIFIED: rust-toolchain.toml, rustc --version] |
| `open-bitcoin-network` | workspace | I/O-free `classify_block_serving_status` | Existing policy core; D-03 forbids moving I/O here. [VERIFIED: packages/open-bitcoin-network/src/block_serving.rs] |
| `open-bitcoin-node` | workspace | Fact assembly, cache, Fjall persist | Owns `managed_block_serve_input` and `blocks_by_hash`. [VERIFIED: packages/open-bitcoin-node/src/network/inventory.rs] |
| `open-bitcoin-rpc` | workspace | Deferred durable serve + `DurableBlockSource` | Production inbound path that currently assumes durable presence. [VERIFIED: packages/open-bitcoin-rpc/src/context.rs] |
| Fjall | 3.1.4 | Durable `block:` keyspace | Already persists bodies via `save_block` / `load_block`. [VERIFIED: packages/open-bitcoin-node/Cargo.toml] |

### Supporting

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| Bun | pin `.bun-version` 1.3.9 (local 1.3.14) | Phase 111 checker / verify scripts | Update checker terms when flipping the pruned-notfound test. [VERIFIED: .bun-version, bun --version] |
| Tokio | existing RPC runtime | Existing inbound tests | Reuse `inbound_listener/tests/block_serving.rs`; do not add a new async stack. |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Fjall `contains_key` probe | Full `load_block` decode as the probe | Decode is a stronger “readable bytes” check but does I/O+parse on every inventory classify. D-01 asks presence, not successful decode. Decode belongs at serve time (D-04). |
| New `getblock` RPC | Existing classification/evidence seam | Locked out by D-11. No `getblock` method exists. [VERIFIED: packages/open-bitcoin-rpc/src/method.rs] |
| Emit Knots-style silent drop | Existing `NotFound` | D-12 keeps NotFound. Knots `getdata` returns without `notfound` when `!BLOCK_HAVE_DATA` (`net_processing.cpp` ~2304). That wire difference is already intentional and out of scope. |

**Installation:** none. Do not add crates.

**Version verification:** Rust 1.94.1 (2026-03-25), Fjall 3.1.4 in `packages/open-bitcoin-node/Cargo.toml`, Bun pin 1.3.9.

## Architecture Patterns

### Recommended Project Structure

Keep files where they are. Do not add a new crate or operator surface.

```
packages/open-bitcoin-network/src/block_serving.rs   # I/O-free classifier; reserved Pruned
packages/open-bitcoin-node/src/network/inventory.rs  # fact assembly + probe
packages/open-bitcoin-node/src/network/block_serving.rs  # gate/serve; LookupUnavailable honesty
packages/open-bitcoin-node/src/storage/fjall_store.rs    # has_block presence helper
packages/open-bitcoin-rpc/src/context/inbound_wire.rs   # DurableBlockSource::has_block
packages/open-bitcoin-node/src/network/tests/block_serving.rs  # flip Pruned→Unavailable
```

### Pattern 1: Probe in the shell, classify in the core

**What:** The node adapter reads cache and store, then injects typed facts. `classify_block_serving_status` stays a pure function of `BlockServingStatusFacts`.
**When to use:** Every inventory, deferred durable gate, compact announcement, and compact-txn serve.
**Example:**

```rust
// Source: packages/open-bitcoin-node/src/network/inventory.rs
// Today (dishonest):
let has_local_data = self.blocks_by_hash.contains_key(&block_hash);
let data_availability = match (is_active, is_tip, has_local_data || durable_availability) {
    (true, _, true) => BlockServingDataAvailability::Available,
    (true, false, false) => BlockServingDataAvailability::Pruned,
    _ => BlockServingDataAvailability::Unavailable,
};

// Target (honest):
let payload_present = self.blocks_by_hash.contains_key(&block_hash)
    || durable_payload_present;
let data_availability = if payload_present {
    BlockServingDataAvailability::Available
} else {
    BlockServingDataAvailability::Unavailable
};
```

### Pattern 2: Three distinguishable presence facts

**What:** Assemble `payload_present`, `index_known`, and `validated_on_active_chain` at the same seam that already builds `ManagedBlockServeInput`.
**When to use:** Always, including compact reuse of `managed_block_serve_input`.
**Recommended mapping (discretion):**

| Fact | Live source | Must not authorize |
|------|-------------|--------------------|
| `payload_present` | `blocks_by_hash.contains_key` OR durable `has_block` | — (this is the only Available authorizer) |
| `index_known` | `peer_manager.header_store().contains(&hash)` | Available / serve |
| `validated_on_active_chain` | `chainstate.active_chain()` hash membership | Available / serve |

Do not use `ChainstateStore::best_block()` (coins tip) or header best-chain tip as `validated_on_active_chain` or as a serve gate. [VERIFIED: inventory.rs uses `active_chain()` today; coins `best_block` is unused on this seam]

**Type-shape recommendation (discretion):** Sit the three bools **beside** `BlockServingStatusFacts` on a shell struct (`BlockServingPresenceFacts` or fields on `ManagedBlockServeInput` / `ManagedBlockServeDecision`). Keep `BlockServingStatusFacts` as the I/O-free classifier input. Copy the three bools onto `ManagedBlockServeDecision` so HAVL-03 is testable on the existing report seam without a Phase 144 UI. Do not add them to operator JSON, CLI, dashboard, metrics, or logs.

### Pattern 3: Cache-or-store probe, decode later

**What:** Presence probe is `contains_key`. Serve-time read stays `lookup_block` / `DurableBlockSource::load_block`.
**When to use:** Classification and gate. If the later read fails, refuse Unavailable (D-04).
**Example:**

```rust
// Source: packages/open-bitcoin-node/src/storage/coins_view.rs (existing Fjall pattern)
fn coins_contains(&self, key: &[u8]) -> Result<bool, StorageError> {
    self.coins.contains_key(key).map_err(coins_backend_failure)
}

// Add the same pattern for block bodies:
// FjallNodeStore::has_block(hash) -> Result<bool, StorageError>
// using StorageNamespace::BlockIndex and block_key(hash)
```

**Probe injection recommendation (discretion):** Add `has_block` to `DurableBlockSource` next to `load_block`, and pass a `durable_payload_present: bool` that is the **result** of that probe (not a caller override). `receive_message_for_durable_serving` / `gate_inventory_for_durable_serving` must obtain the result from the live store or an injected source. `ManagedPeerNetwork` does not own `FjallNodeStore` today; the RPC context does. Thread the probe result or a `Fn(BlockHash) -> bool` into the gate. Memory tests with no store probe cache only.

### Pattern 4: Compact paths inherit through fact assembly

**What:** `announcement_transport.rs` and `action_translation.rs` already call `managed_block_serve_input(..., durable_availability: false)`.
**When to use:** Do not add a second probe implementation. Once `managed_block_serve_input` computes `payload_present` from cache (and optional durable probe when the caller supplies one), compact announcement and compact-txn inherit the same rule. Compact-txn stays cache-first; do not expand it into a durable-store product.

### Anti-Patterns to Avoid

- **Caller override bool named `durable_availability`:** The name can stay only if the value is a probe result. Do not keep `true` as an override. [VERIFIED: inventory.rs:196]
- **Classifier disk reads:** Do not add Fjall or `blocks_by_hash` to `open-bitcoin-network`. [CITED: 110 D-09/D-10, D-03]
- **Coins tip or header index as Available:** That is the HAVL-03 negative contract.
- **New wire error or `getblock`:** D-11 / D-12.
- **Documenting this as archive-node honesty or public-default serving:** D-13.
- **Growing node `block_serving.rs` past the 628-line trigger:** It is already 624 lines. Put new tests in `packages/open-bitcoin-node/src/network/tests/block_serving.rs`. [VERIFIED: wc -l]

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Payload presence | New block-file format or Knots blk*.dat reader | Existing `blocks_by_hash` + Fjall `block:` keys via `contains_key` | Bodies already persist through `save_block` / `FlushPersistSink::persist_block`. [VERIFIED: fjall_store.rs, flush_lifecycle.rs] |
| Missing-payload refuse | New RPC error / disconnect / prune bit | `WireNetworkMessage::NotFound` + `BlockServingStatusLabel::Unavailable` | Already the missing-inventory path. [VERIFIED: inventory.rs:107-113, inbound_wire.rs:110-122] |
| I/O-free policy | Disk reads in `classify_block_serving_status` | Injected `BlockServingStatusFacts` | 110 D-09/D-10. |
| Operator have-bytes UI | New status/RPC/CLI/dashboard fields | Existing `status_label` / counters on the decision | Phase 144 owns CSOBS-01/CSOBS-02. |
| getblock product | New `SupportedMethod::GetBlock` | Do not invent it | No getblock exists. [VERIFIED: method.rs] |
| Prune-mode product | Height windows, file unlink, NODE_NETWORK_LIMITED | Reserved `Pruned` variant only | FUT-18. Knots `IsBlockPruned` requires `m_have_pruned`. [VERIFIED: blockstorage.cpp:711-714] |

**Key insight:** Honesty is a fact-assembly bug, not a missing subsystem. The classifier, NotFound path, durable load, and evidence counters already exist. The work is to stop lying to them.

## Runtime State Inventory

This phase changes a live classification label, not a stored identity. After source edits, these runtime surfaces still matter:

| Category | Items Found | Action Required |
|----------|-------------|------------------|
| Stored data | Fjall `block:<64-hex>` bodies; coins `B` / header snapshot keys are unrelated to the `Pruned` label. No persisted “pruned” flag. [VERIFIED: fjall_store.rs `block_key`] | None — no data migration. Probe new and existing keys the same way. |
| Live service config | None — no n8n/Datadog/service UI stores this string. [VERIFIED: codebase search] | None |
| OS-registered state | None — no launchd/systemd/pm2 names. [VERIFIED: no matches] | None |
| Secrets/env vars | None named `durable_availability` or `Pruned`. | None |
| Build artifacts | In-memory `BlockServingStatusCounters.pruned_count` and operator JSON field `pruned_count` remain as reserved counter slots. [VERIFIED: status/block_serving.rs] | Code edit only: production paths must stop incrementing `pruned_count` for missing payload. Do not remove the field (Phase 144 / reserved label). |

**Nothing found in category:** Stored prune-mode flags, OS registrations, and secrets — verified by repo search. `pruned_count` is an in-memory counter, not a datastore.

## Common Pitfalls

### Pitfall 1: Unconditional `durable_availability: true`

**What goes wrong:** Deferred inbound serving classifies Available, gates a `DurableBlock` intent, then `resolve_block_intent` may NotFound. Classification already claimed Available.
**Why it happens:** Phase 127 deferred the disk read past the gate and used a bool shortcut.
**How to avoid:** Probe before `gate_managed_block_request`. Cache hit is enough. Cache miss must ask the store; missing key is not Available.
**Warning signs:** `gate_inventory_for_durable_serving(..., true)` or any override that ignores an empty cache.

### Pitfall 2: `Pruned` for active-non-tip missing cache

**What goes wrong:** Operators and docs read “pruned” as prune-mode. Knots only sets pruned when `m_have_pruned` is true.
**Why it happens:** `managed_block_serve_input` special-cases `(true, false, false)` as `Pruned`. Tests encode that lie (`phase111_active_chain_non_tip_missing_local_block_returns_pruned_notfound`).
**How to avoid:** Missing payload is always `Unavailable` this phase. Keep the enum. Rewrite help/docs so reserved `pruned` means “reserved for a future prune-mode delete,” not “this node pruned historical blocks.”
**Warning signs:** Production match arms that return `BlockServingDataAvailability::Pruned` or increment `pruned_count` on missing cache.

### Pitfall 3: Phase 111 checker still requires the old test name

**What goes wrong:** `scripts/check-phase111-full-block-serving-request-path.ts` `REQUIRED_TESTS` includes `phase111_active_chain_non_tip_missing_local_block_returns_pruned_notfound`. Renaming the test without updating the checker fails `verify.sh`.
**Why it happens:** Historical v2.1 checker pins exact symbols and the phrase `block_status_pruned` as a required term.
**How to avoid:** Flip the test assertion to Unavailable, rename it to `..._returns_unavailable_notfound`, and update the checker plus `check-phase111-full-block-serving-request-path.test.ts`. Keep `block_status_pruned` in `REQUIRED_TERMS` as reserved vocabulary. Rewrite “pruned active non-tip blocks” in `docs/architecture/status-snapshot.md`, `docs/operator/runtime-guide.md`, `docs/parity/catalog/p2p.md`, and `docs/parity/index.json` so they cannot be read as prune-mode. Do not claim archive-node or public-default serving while editing those files (checker scans those paths).
**Warning signs:** verify fails on missing required test name or “forbidden Phase 111 positive claim.”

### Pitfall 4: `LookupUnavailable` keeps `status_label: Available`

**What goes wrong:** Post-gate load failure already returns `BlockStatusUnavailable` + NotFound, but `completion` copies `eligible_decision.status_label` (Available). `complete_block_serve` then increments `available_count` for a refused request. That violates HAVL-01 “reports … Available only when the payload bytes are present.”
**Why it happens:** `ManagedBlockServeIntent::completion` only rewrites the outcome label. [VERIFIED: block_serving.rs:74-78]
**How to avoid:** On `LookupUnavailable`, set `status_label` to `Unavailable` (and keep `missing_inventory: true`). Do not emit `Pruned`.
**Warning signs:** `available_count` rising on NotFound durable misses.

### Pitfall 5: Using coins tip or header index to authorize a serve

**What goes wrong:** After Phase 142, coins `B` and header snapshots can exist without a body (see `durable_block_serving_context(persist_block: false)`).
**Why it happens:** Those facts look like “we know this block.”
**How to avoid:** `index_known` and `validated_on_active_chain` are report facts only. Available requires `payload_present`.
**Warning signs:** `best_block()` or `header_store.contains` in a `data_availability` match arm.

### Pitfall 6: File-length overflow on hot files

**What goes wrong:** `block_serving.rs` (node) is 624 lines; `fjall_store.rs` is 623. A few added helpers plus inline tests trip the 628 trigger.
**How to avoid:** Add `has_block` as a short method. Put new unit tests in `network/tests/block_serving.rs` and inbound durable tests in the existing RPC test module.
**Warning signs:** `bun scripts/bright-builds-check.ts` file-lengths failure.

### Pitfall 7: Treating Knots silent `getdata` drop as a wire change

**What goes wrong:** Planner “fixes” Open Bitcoin to omit NotFound because Knots returns early at `!BLOCK_HAVE_DATA`.
**Why it happens:** Parity reading without the existing 111 contract.
**How to avoid:** Keep NotFound (D-12). Document the Knots difference only if a breadcrumb comment needs it. Do not change the wire.

## Code Examples

### Current dishonest fact assembly

```rust
// Source: packages/open-bitcoin-node/src/network/inventory.rs
let has_local_data = self.blocks_by_hash.contains_key(&block_hash);
let data_availability = match (is_active, is_tip, has_local_data || durable_availability) {
    (true, _, true) => BlockServingDataAvailability::Available,
    (true, false, false) => BlockServingDataAvailability::Pruned,
    _ => BlockServingDataAvailability::Unavailable,
};
```

`gate_inventory_for_durable_serving` calls this with `durable_availability: true`. `serve_inventory` and compact paths pass `false`.

### I/O-free classifier (do not change the I/O boundary)

```rust
// Source: packages/open-bitcoin-network/src/block_serving.rs
pub fn classify_block_serving_status(
    facts: &BlockServingStatusFacts,
) -> BlockServingStatusDecision {
    let label = classify_block_serving_status_label(facts);
    let may_serve_block = label == BlockServingStatusLabel::Available;
    BlockServingStatusDecision {
        label,
        allow_storage_read: may_serve_block,
        may_serve_block,
    }
}
```

If injected `data_availability` is `Pruned`, the label is `Pruned`. Production must stop injecting `Pruned`. Classifier tests may still cover the reserved mapping.

### Knots pruned-versus-missing

```cpp
// Source: packages/bitcoin-knots/src/node/blockstorage.cpp
bool BlockManager::IsBlockPruned(const CBlockIndex& block) const
{
    AssertLockHeld(::cs_main);
    return m_have_pruned && !(block.nStatus & BLOCK_HAVE_DATA) && (block.nTx > 0);
}
```

```cpp
// Source: packages/bitcoin-knots/src/net_processing.cpp
// Pruned nodes may have deleted the block, so check whether
// it's available before trying to send.
if (!(pindex->nStatus & BLOCK_HAVE_DATA)) {
    return;
}
```

Open Bitcoin analog: `payload_present` ≡ `BLOCK_HAVE_DATA` / cache-or-store bytes. `Pruned` ≡ `IsBlockPruned`, which requires a prune-mode flag this milestone does not set.

### Durable serve already refuses missing bodies on the wire

```rust
// Source: packages/open-bitcoin-rpc/src/context/inbound_wire.rs
Some(Ok(None)) | Some(Err(_)) | None => {
    resolved.immediate_completions.push(
        intent.completion(ManagedBlockServeCompletionOutcome::LookupUnavailable),
    );
    resolved.push_encoded(
        WireNetworkMessage::NotFound(InventoryList::new(vec![intent.request().clone()])),
        ...
    );
}
```

This is the D-04 refuse. It is not enough for HAVL-01 because classification already happened.

### Existing Fjall presence helper to copy

```rust
// Source: packages/open-bitcoin-node/src/storage/coins_view.rs
fn coins_contains(&self, key: &[u8]) -> Result<bool, StorageError> {
    self.coins.contains_key(key).map_err(coins_backend_failure)
}
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Cache-only `has_local_data` | Cache OR unconditional `durable_availability` | Phase 127 durable inbound gate | Classification can claim Available with no bytes |
| Missing active non-tip → `Pruned` | Must become `Unavailable` until FUT-18 | Phase 111 historical boundary | Label currently reads as prune-mode |
| Knots `BLOCK_HAVE_DATA` then read | Same two-step: probe presence, then read | Bitcoin Core / Knots current `BlockManager` | Match the probe/read split; do not match prune-mode |

**Deprecated/outdated:**

- Caller override `durable_availability: true` as a stand-in for “we have a store.” Replace with a probe result.
- Docs that say “pruned active non-tip blocks” as if this node deleted historical files.

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | Live `HeaderStore::contains` is the `index_known` source; the Fjall block-index snapshot is not a second index. | Architecture Patterns | If reopen hydrates headers late, `index_known` could be false for a hash that exists only on disk. Planner should confirm hydrate-before-serve on the durable inbound path (existing `DurableSyncRuntime::open` already loads headers). |
| A2 | Compact-txn should stay cache-first and not gain a durable-store probe unless the caller already supplies one. | Pattern 4 | A restart-without-cache compact `getblocktxn` would refuse even if Fjall has the body. Acceptable: compact extra-txn serve is a live-cache protocol; full `getdata` uses the durable gate. |

**If this table is empty:** not applicable — two assumptions flagged.

## Open Questions

1. **How does `gate_inventory_for_durable_serving` obtain the store probe if `ManagedPeerNetwork` does not own `FjallNodeStore`?**
   - What we know: RPC `ManagedRpcContext` holds `maybe_block_source` and calls `receive_message_for_durable_serving`. [VERIFIED: context.rs:312]
   - What's unclear: whether to inject `Fn(BlockHash) -> bool` into the receive/gate API, add `has_block` on `DurableBlockSource` and call it before/during gate from a new network method, or attach an optional probe handle to the network.
   - Recommendation: Extend `DurableBlockSource` with `has_block`, and thread a probe callback into `receive_message_for_durable_serving` / `gate_inventory_for_durable_serving`. Memory tests pass a cache-only probe (or `false` for durable). Do not put Fjall types in `open-bitcoin-network`.

2. **Must `LookupUnavailable` also flip `eligibility_reason` to `StatusUnavailable`?**
   - What we know: today only the outcome label changes; `status_label` stays Available.
   - What's unclear: whether existing evidence tests assert `Available` on completion-after-miss.
   - Recommendation: Flip `status_label` to `Unavailable`. Leave eligibility as already-gated unless a test requires `StatusUnavailable`.

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| rustc / cargo | First-party tests | ✓ | 1.94.1 | — |
| Bun | Phase 111 checker | ✓ | 1.3.14 (pin 1.3.9) | — |
| Bazel | verify.sh smoke | ✓ | present | — |
| Fjall (crate) | `has_block` | ✓ | 3.1.4 (`contains_key` already used) | — |
| bitcoin-knots submodule | Breadcrumbs / IsBlockPruned | ✓ | `29.3.knots20260210` tree present | — |

**Missing dependencies with no fallback:** none.

**Missing dependencies with fallback:** none.

Step 2.6 completed. This phase is code/config only plus existing Fjall.

## Security Domain

`workflow.nyquist_validation` is false; Validation Architecture omitted. `security_enforcement` is not set to false, so this section is required.

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|-----------------|
| V2 Authentication | no | Unchanged inbound admission / RPC cookie |
| V3 Session Management | no | Unchanged peer sessions |
| V4 Access Control | yes | Existing eligibility + default-off serving; probe must not bypass activation |
| V5 Input Validation | yes | Typed `BlockHash` / inventory vectors already parsed at the boundary |
| V6 Cryptography | no | No new crypto |

### Known Threat Patterns for block serving

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Serve a fabricated body when bytes are missing | Tampering / Spoofing | Probe then read; D-04 refuse on read failure |
| Classify Available from coins tip / header index | Elevation of privilege (over-claim) | HAVL-03 negative contract |
| Leak Fjall corruption strings on the wire | Information disclosure | Existing inbound tests redact store errors as NotFound. Keep that. [VERIFIED: inbound_listener/tests/block_serving.rs] |
| Per-getdata full-block decode as a probe | Denial of service | Use `contains_key`, cache-first. Do not decode during classify. |
| Docs that claim archive / public-default serving | Repudiation of scope | D-13; Phase 111 claim scanner on the same doc files |

## Sources

### Primary (HIGH confidence)

- `packages/open-bitcoin-node/src/network/inventory.rs` — dishonest `durable_availability` OR and `Pruned` mapping
- `packages/open-bitcoin-network/src/block_serving.rs` — I/O-free classifier and reserved labels
- `packages/open-bitcoin-node/src/network/block_serving.rs` — gate/serve and `LookupUnavailable`
- `packages/open-bitcoin-rpc/src/context/inbound_wire.rs` — `DurableBlockSource::load_block` and NotFound
- `packages/open-bitcoin-rpc/src/method.rs` — no `getblock`
- `packages/bitcoin-knots/src/node/blockstorage.cpp` — `IsBlockPruned` / `CheckBlockDataAvailability`
- `packages/bitcoin-knots/src/net_processing.cpp` — `BLOCK_HAVE_DATA` before send
- `packages/bitcoin-knots/src/validation.cpp` — `fMissingData = !(nStatus & BLOCK_HAVE_DATA)`
- `scripts/check-phase111-full-block-serving-request-path.ts` — required pruned test name
- `.planning/phases/143-honest-stored-block-availability/143-CONTEXT.md` — locked decisions
- `standards/core/architecture.md`, `standards/core/testing.md`, `standards/languages/rust.md`

### Secondary (MEDIUM confidence)

- Bitcoin Core `BlockManager` docs for `CheckBlockDataAvailability` (cross-check with vendored Knots) — [CITED: doxygen.bitcoincore.org]
- Bitcoin Core PR 30410: RPC distinguishes “pruned data” vs “not fully downloaded” — confirms missing ≠ pruned. [CITED: github.com/bitcoin/bitcoin/pull/30410]

### Tertiary (LOW confidence)

- None presented as authoritative.

## Recommended Plan Slices

Planner should keep slices small and hook-passing (Phases 140–142 combined RED+GREEN because `verify.sh` runs in pre-commit).

1. **Facts and reserved `Pruned`.** Add `payload_present` / `index_known` / `validated_on_active_chain` on the shell report seam. Stop injecting `Pruned` for missing payload. Keep the enum and `as_str`. Flip network-core status-case expectations that treat missing-active as a production `Pruned` outcome only where they encode the dishonest mapping; reserved-variant tests stay.
2. **Probe replaces the override.** Add `FjallNodeStore::has_block` + `DurableBlockSource::has_block`. Thread a probe result into `managed_block_serve_input` / `gate_inventory_for_durable_serving`. Cache-or-store; no decode.
3. **Refuse and evidence honesty.** Inventory, deferred gate, compact inheritance, and `LookupUnavailable` all emit `Unavailable` + NotFound. `status_label` on miss is `Unavailable`. Existing durable inbound tests stay the wire contract.
4. **Historical checker and copy.** Rename the Phase 111 missing-body test, update the Phase 111 checker and the four claim-scanned docs so “pruned” cannot be read as prune-mode. No new operator UI. No getblock. No archive/public-default claims.

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH — pinned Rust/Fjall, no new libraries
- Architecture: HIGH — dishonest seam, probe sources, and RPC inbound path read in tree
- Pitfalls: HIGH — Phase 111 checker, `LookupUnavailable` label leak, and file-length limits verified

**Research date:** 2026-09-17
**Valid until:** 2026-10-17 (stable internal API; re-check if Phase 144 starts early)
