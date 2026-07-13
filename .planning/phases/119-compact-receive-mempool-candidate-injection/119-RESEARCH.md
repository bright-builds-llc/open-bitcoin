# Phase 119: Compact Receive Mempool Candidate Injection - Research

**Researched:** 2026-07-13
**Domain:** BIP152 compact-block receive wiring — mempool/extra candidate injection and mempool-removal lifecycle into volatile partial compact state
**Confidence:** HIGH

## Summary

Phase 119 is a **runtime seam gap-closure**, not a reconstruction rewrite. Phase 114/115 already provide typed `init_partial_compact_block` / `handle_compact_block_download` outcomes and `PartialCompactBlock::on_mempool_transaction_removed`. The audit break is that live inbound `WireNetworkMessage::CompactBlock` always reaches download with `CompactBlockReceiveFacts::default()` (empty candidates/extras), and mempool lifecycle never forwards removals into partial compact slots. [VERIFIED: `.planning/v2.1-MILESTONE-AUDIT.md`; `packages/open-bitcoin-network/src/peer/message_dispatch.rs`; `packages/open-bitcoin-node/src/network/mempool_lifecycle.rs`]

Locked decisions D-01..D-11 require the **node shell** (`ManagedPeerNetwork`) to gather mempool `(Wtxid, Transaction)` views and a **Knots-shaped bounded extra ring buffer**, pass them as `CompactBlockReceiveFacts` into `handle_compact_block_download`, keep `PeerManager` free of `open-bitcoin-mempool`, and hook removal cleanup via wtxid. Prefer intercepting `CompactBlock` in `receive_message` / `receive_sync_message` over baking mempool into `message_dispatch`. [VERIFIED: `119-CONTEXT.md`; Phase 114 D-08]

**Primary recommendation:** Intercept `CompactBlock` in the node shell, adapt `Mempool::entries()` plus a node-owned `CompactExtraTxnBuffer` (Knots `DEFAULT_BLOCK_RECONSTRUCTION_EXTRA_TXN` / size limits) into `CompactBlockReceiveFacts`, call `PeerManager::handle_compact_block_download` directly, and add a PeerManager forwarder that walks `compact_download_states[*].in_flight[*].partial` and calls `partial.on_mempool_transaction_removed` from `apply_connected_block_mempool_lifecycle` (and other wtxid-bearing removal outcomes).

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

#### Receive Candidate Supply Seam

- **D-01:** Inbound `CompactBlock` dispatch must stop always using `CompactBlockReceiveFacts::default()`. Live receive must supply mempool candidates and bounded extras into `handle_compact_block_download`.
- **D-02:** Keep `PeerManager` free of `open-bitcoin-mempool` coupling (Phase 114 D-08). Gather candidate/extra slices in the node shell (`ManagedPeerNetwork`) and pass `CompactBlockReceiveFacts` into the network download API — prefer intercepting `CompactBlock` in `receive_message` / `receive_sync_message` (or a focused helper) rather than baking mempool into `message_dispatch`.
- **D-03:** Prefer the smallest API change that makes non-empty facts reachable on the live path: e.g. call `handle_compact_block_download` directly from the shell for `CompactBlock`, or add a PeerManager entry that accepts facts without pulling mempool into the network crate. Empty-facts `handle_message` CompactBlock branch may become a test-only or deprecated path, but production receive must use the injected path.

#### Mempool And Extra Sources

- **D-04:** Mempool candidates are the current mempool's `(Wtxid, Transaction)` view at receive time — shell adapts mempool iteration into the existing `CompactBlockReceiveFacts` slice shape.
- **D-05:** Bounded extras follow Knots-shaped recent/extra compact txn inputs (bounded buffer of recent or orphan-adjacent transactions suitable for reconstruction). Prefer a dedicated bounded extra buffer owned by the node shell over unbounded history or inventing package-relay surfaces. Exact buffer size/eviction policy is Claude's Discretion within a Knots-aligned bound.
- **D-06:** Candidate and extra collection must remain read-only relative to chainstate: no chainstate mutation from partial compact state (RCN-06 preserved).

#### Mempool Removal Lifecycle Hook

- **D-07:** Hook `on_mempool_transaction_removed` (or the PeerManager-level forwarder over compact-download partial state) from mempool lifecycle when transactions leave the mempool — at minimum from `apply_connected_block_mempool_lifecycle` removals, and from other removal paths the shell already treats as mempool exits (evict/expire) when wtxid is available.
- **D-08:** Lifecycle hook clears matching volatile partial compact slots only. Do not activate package relay, bloom/filter serving, or compact filters. Do not schedule timeout ticks (Phase 120).

#### Verification And Parity

- **D-09:** Runtime/unit tests must prove: (1) live CompactBlock receive with mempool candidates reconstructs or reports Ready/missing without empty-facts only, (2) collision, duplicate, and missing outcomes remain typed on the injected path, (3) mempool removal clears matching volatile slots via the lifecycle hook, (4) package/filter/public-default surfaces stay untouched.
- **D-10:** New or touched first-party Rust source/test files under `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests` require parity breadcrumbs and `docs/parity/source-breadcrumbs.json` entries unless an explicit `none` breadcrumb is defensible. Prefer Knots `blockencodings.cpp` / `net_processing.cpp` reconstruction and extra-txn anchors.
- **D-11:** Verification remains deterministic and local through repo-native checks and `bash scripts/verify.sh`. Public-network compact-relay review stays opt-in UAT only.

### Claude's Discretion

The planner/researcher may choose exact helper placement (shell receive intercept vs PeerManager facts API), whether extras live as a small ring buffer beside ManagedPeerNetwork, how wtxid is obtained from mempool removal summaries, and how tests inject candidate sets. Prefer early returns, iterator/slice adapters, and the smallest seam that closes the audit gap without reopening Phase 114 reconstruction policy.

### Deferred Ideas (OUT OF SCOPE)

Compact-download timeout scheduling and misbehavior escalation (Phase 120), DurableSyncRuntime block-relay metrics/log projection (Phase 121), package relay, bloom/filter serving, compact filter serving, public serving defaults, public-network CI gates, archive-node claims, production full-node readiness, and production-funds wallet safety remain outside Phase 119.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| RCN-02 | Reconstruct compact blocks from current mempool state plus bounded extra/recent inputs using witness-hash short IDs | Shell injects `Mempool::entries()` + Knots-shaped extra ring into `CompactBlockReceiveFacts`; reconstruction core already matches InitData |
| RCN-03 | Detect short ID collisions, duplicate matches, missing transactions, and reconstruction failures with stable typed outcomes | Injected path reuses existing `CompactReconstructionOutcome` / download init outcomes; tests must exercise non-empty facts on live receive |
| GOV-04 | Compact relay integrates with mempool lifecycle without activating package relay or filter serving | Forward `MempoolLifecycleRemoval.wtxid` (and other wtxid-bearing exits) into PeerManager → `PartialCompactBlock::on_mempool_transaction_removed` |
</phase_requirements>

## Project Constraints (from AGENTS.md / Bright Builds)

No `.cursor/rules/` or `CLAUDE.md` present in this repo. Actionable project constraints from `AGENTS.md`, `AGENTS.bright-builds.md`, and managed standards:

- Functional core / imperative shell: keep reconstruction policy in `open-bitcoin-network`; effects and mempool adaptation in `open-bitcoin-node`. [CITED: `standards/core/architecture.md`]
- Do not use existing Rust Bitcoin libraries in the production path. [CITED: `AGENTS.md` Project Constraints]
- Prefer early returns; prefix nullable names with `maybe`; avoid `unwrap()`. [CITED: `AGENTS.bright-builds.md`; user code-styling rules]
- New first-party Rust source/test files need parity breadcrumbs via `docs/parity/source-breadcrumbs.json`. [CITED: `AGENTS.md` Repo-Local Guidance]
- Verification contract: `bash scripts/verify.sh` (default, not `--fast` for completion). [CITED: `AGENTS.md`]
- Unit tests: Arrange / Act / Assert, one concern per test. [CITED: `standards/core/testing.md`]
- `open-bitcoin-network` must not depend on `open-bitcoin-mempool` (confirmed: Cargo.toml deps are chainstate/codec/consensus/primitives only). [VERIFIED: `packages/open-bitcoin-network/Cargo.toml`]

## Standard Stack

### Core

| Library / Component | Version / Location | Purpose | Why Standard |
|---------------------|--------------------|---------|--------------|
| Rust toolchain | `1.94.1` | Language / edition 2024 | Pinned by `rust-toolchain.toml` [VERIFIED: local `rustc --version`] |
| `open-bitcoin-network` | workspace path | `CompactBlockReceiveFacts`, `handle_compact_block_download`, `PartialCompactBlock` | Existing Phase 114/115 APIs — do not replace [VERIFIED: codebase] |
| `open-bitcoin-mempool` | workspace path | `Mempool::entries()`, `MempoolLifecycleRemoval { wtxid, .. }` | Candidate + removal wtxid source [VERIFIED: codebase] |
| `open-bitcoin-node` | workspace path | `ManagedPeerNetwork` shell intercept + lifecycle | Owns mempool + PeerManager [VERIFIED: codebase] |
| Bitcoin Knots baseline | `29.3.knots20260210` under `packages/bitcoin-knots` | Parity anchors for InitData / `vExtraTxnForCompact` | Project behavioral baseline [CITED: `AGENTS.md`] |

### Supporting

| Library / Component | Version | Purpose | When to Use |
|---------------------|---------|---------|-------------|
| Bun | `1.3.9` (local) | Parity breadcrumb / verify scripts | Breadcrumb registry updates [VERIFIED: `bun --version`] |
| `scripts/verify.sh` | repo | Pre-commit / release verification | Phase completion gate [CITED: `AGENTS.md`] |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Shell CompactBlock intercept | Teach `message_dispatch` to accept facts | Would require mempool into network crate or awkward callbacks — **rejected by D-02** |
| Orphanage as sole extra source | Dedicated ring buffer | Orphanage lacks public `(wtxid, tx)` iteration API and is not a Knots `vExtraTxnForCompact` ring — use dedicated buffer, optionally **push** orphans into it [VERIFIED: `TxOrphanage` private map; Knots `net_processing.h` comment] |
| Full Knots 32768-slot buffer in tests | Tiny test buffer | Use Knots constants as production defaults; tests may construct small buffers via constructor override |

**Installation:** N/A — first-party Rust crates only; no new crates.

**Version verification:** Rust `1.94.1` confirmed via local toolchain. No npm packages.

## Architecture Patterns

### Recommended Project Structure

```
packages/open-bitcoin-node/src/network/
├── network.rs                      # receive_message / receive_sync_message CompactBlock intercept
├── compact_receive_candidates.rs   # NEW: mempool → facts adapter + CompactExtraTxnBuffer
├── mempool_lifecycle.rs            # hook removals → PeerManager forwarder
├── admission_bridge.rs             # push rejected/orphaned/replaced bodies into extra buffer
└── tests/
    └── compact_receive_cases.rs    # NEW: live receive + lifecycle runtime tests

packages/open-bitcoin-network/src/peer/
├── compact_download_state.rs       # existing handle_compact_block_download + NEW on_mempool_tx_removed forwarder
└── message_dispatch.rs             # leave empty-facts CompactBlock branch for non-shell/tests (D-03)
```

### Pattern 1: Shell CompactBlock Intercept (smallest live seam)

**What:** Match `WireNetworkMessage::CompactBlock` in `ManagedPeerNetwork::receive_message` and `receive_sync_message` before `peer_manager.handle_message`. Build facts, call `handle_compact_block_download`, then continue existing evidence/`process_actions` path.

**When to use:** Always for production receive (D-01, D-02, D-03). Both `receive_message` and `receive_sync_message` currently forward CompactBlock through `handle_message` — both need the intercept. [VERIFIED: `network.rs` lines ~219–275]

**Example:**

```rust
// Source: recommended pattern from verified ManagedPeerNetwork + CompactBlockReceiveFacts APIs
WireNetworkMessage::CompactBlock(payload) => {
    let (candidate_owned, extra_owned) = self.collect_compact_receive_owned();
    let candidate_refs: Vec<(&Wtxid, &Transaction)> = candidate_owned
        .iter()
        .map(|(wtxid, tx)| (wtxid, tx))
        .collect();
    let extra_refs: Vec<(&Wtxid, &Transaction)> = extra_owned
        .iter()
        .map(|(wtxid, tx)| (wtxid, tx))
        .collect();
    let facts = CompactBlockReceiveFacts {
        candidates: &candidate_refs,
        extra: &extra_refs,
    };
    let actions = self.peer_manager.handle_compact_block_download(
        peer_id,
        payload,
        facts,
        timestamp,
    )?;
    self.note_block_relay_observed();
    self.record_compact_download_evidence(&actions);
    // then existing process_actions / ManagedSyncMessageResult path
}
```

### Pattern 2: Owned snapshot then slice refs (Rust borrow fix)

**What:** `self.mempool` and `self.peer_manager` cannot be borrowed together. Collect owned `(Wtxid, Transaction)` snapshots first, then build `CompactBlockReceiveFacts` refs into those locals, then mutably call PeerManager.

**When to use:** Every live inject call. [VERIFIED: `ManagedPeerNetwork` owns both fields in `network.rs`]

### Pattern 3: PeerManager mempool-removal forwarder

**What:** Add `PeerManager::on_mempool_transaction_removed(&mut self, removed_wtxid: &Wtxid)` that iterates:

```text
compact_download_states.values_mut()
  → in_flight.values_mut()
  → partial.on_mempool_transaction_removed(removed_wtxid)
```

No mempool types in the network crate. Path types: `CompactDownloadPeerState.in_flight: BTreeMap<BlockHash, CompactDownloadInFlight>` where `CompactDownloadInFlight.partial: PartialCompactBlock`. [VERIFIED: `compact_download.rs`]

**When to use:** From `apply_connected_block_mempool_lifecycle` after `remove_for_connected_block`, and from admission/evict/expire paths when a **leaving** wtxid is available (see Wtxid resolution below).

### Pattern 4: Knots-shaped extra ring buffer

**What:** Node-owned ring matching Knots `vExtraTxnForCompact` / `AddToCompactExtraTransactions`.

**Knots bounds (use as named constants):** [VERIFIED: `packages/bitcoin-knots/src/net_processing.h:29–33`]

| Constant | Value | Meaning |
|----------|-------|---------|
| `DEFAULT_BLOCK_RECONSTRUCTION_EXTRA_TXN` | `32768` | Ring capacity (slots) |
| `DEFAULT_BLOCK_RECONSTRUCTION_EXTRA_TXN_SIZE` | `10_000_000` | Max aggregate dynamic usage bytes |
| `BLOCK_RECONSTRUCTION_EXTRA_TXN_PER_TXN_SIZE_LIMIT` | `100_000` | Per-tx size gate on reject path |

**Knots insert algorithm:** resize to `max_extra_txs`; overwrite at `vExtraTxnForCompactIt`; advance `% capacity`; while memusage > max size: clear next slot and advance. [VERIFIED: `net_processing.cpp` `AddToCompactExtraTransactions` ~1770–1790]

**Knots feed sources (comment + call sites):** orphan, replaced, and rejected transactions. [VERIFIED: `net_processing.h:31–32`; `net_processing.cpp` ~3031–3059]

**Recommended OB feeds (smallest shell-aligned set):**
1. Orphaned transaction body when staged in `admission_bridge`
2. Rejected transaction body when rejection outcome has a body (gate with per-tx size limit)
3. Replaced victim bodies looked up from TxServing records **before** status demotion (Knots adds replaced txs)

Do **not** invent package-relay surfaces (D-08 / deferred).

### Pattern 5: Wtxid resolution for lifecycle (D-07)

| Exit path | Wtxid source | Action |
|-----------|--------------|--------|
| `remove_for_connected_block` | `MempoolLifecycleRemoval.wtxid` | **Must hook** (D-07 minimum) [VERIFIED: `lifecycle.rs`] |
| `MempoolOutcome::Evicted` / `Expired` | Outcome carries `wtxid` | Hook via `maybe_wtxid()` |
| `MempoolOutcome::Accepted` / `Replaced` **victims** | Outcome lists **txids only** (`replaced()`, `evicted()`) | Resolve wtxid from `relay_serving.records_by_txid` / `status_wtxid_for_txid` **before** `remove_stored_transactions_with_status`; skip if unavailable |
| `MempoolOutcome::Replaced` admitted tx | `maybe_wtxid()` is the **new** still-in-mempool tx | **Do not** call removal hook with this wtxid |

[VERIFIED: `outcome.rs` `replaced()`/`evicted()` return `&[Txid]`; `relay_serving.rs` stores wtxid per record]

### Anti-Patterns to Avoid

- **Baking mempool into `message_dispatch`:** Violates D-02 / Phase 114 D-08.
- **Reimplementing InitData / short-ID matching:** Already in `compact_reconstruction.rs`.
- **Calling lifecycle hook with txid instead of wtxid:** Slots are keyed by witness hash (Phase 114 D-11). [VERIFIED: `on_mempool_transaction_removed(&Wtxid)`]
- **Calling removal hook with Replaced admitted wtxid:** That tx just entered the pool — would clear the wrong (or future) slots.
- **Mutating chainstate from partial compact state:** Forbidden by D-06 / RCN-06.
- **Scheduling `expire_compact_download_timeouts`:** Phase 120 only.
- **Treating empty-facts `handle_message` as production receive:** Live path must inject (D-01).

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Short-ID / InitData matching | Custom reconstruct in node | `handle_compact_block_download` → `init_partial_compact_block` | Already typed + tested in Phase 114/115 |
| Mempool wtxid computation | Ad-hoc hash | `MempoolEntry.wtxid` / `MempoolLifecycleRemoval.wtxid` | Stored on entry at admit time |
| Extra buffer policy | Unbounded Vec | Knots-capacity ring + byte budget | DoS / memory parity |
| Package/filter activation | New relay surfaces | No-op: leave CMP-06 / deferred surfaces untouched | GOV-04 + milestone boundary |
| Orphanage scan as extra pool | Public orphan iteration | Push into dedicated extra buffer at stage time | Orphan map is private; Knots uses separate ring |
| Victim wtxid on Replaced | New mempool outcome fields (unless tiny) | Lookup from TxServing before demotion | Victims already recorded with wtxid |

**Key insight:** The gap is **wiring**, not algorithms. Planner tasks should be thin adapters + lifecycle forwarder + runtime tests.

## Common Pitfalls

### Pitfall 1: Simultaneous borrow of mempool and peer_manager

**What goes wrong:** Compiler rejects `entries()` borrow while calling `handle_compact_block_download`.
**Why it happens:** Both live on `ManagedPeerNetwork`.
**How to avoid:** Snapshot owned `(Wtxid, Transaction)` pairs first (clone txs for the call). Accept clone cost on compact receive — Knots also walks full mempool references under lock.
**Warning signs:** Lifetime errors spanning `self.mempool` and `self.peer_manager`.

### Pitfall 2: Leaving production on empty-facts `handle_message`

**What goes wrong:** Unit tests pass with explicit facts while `receive_message` still hits `CompactBlockReceiveFacts::default()`.
**Why it happens:** Easy to only update PeerManager tests.
**How to avoid:** Runtime tests must call `ManagedPeerNetwork::receive_message(..., CompactBlock(...))` with mempool populated — not only `peer_manager.handle_compact_block_download`.
**Warning signs:** Audit break point in `message_dispatch.rs` still the only CompactBlock path used by shell.

### Pitfall 3: Hooking lifecycle with txid only

**What goes wrong:** Slots never clear; GOV-04 remains partial.
**Why it happens:** `apply_connected_block_mempool_lifecycle` currently maps `removal.txid` for TxServing cleanup only. [VERIFIED: `mempool_lifecycle.rs`]
**How to avoid:** Use `removal.wtxid` for compact forwarder; keep txid path for serving status.

### Pitfall 4: Forgetting non-block removal paths / mis-hooking Replaced

**What goes wrong:** Evict/expire leave stale matched slots; or Replaced admitted wtxid clears slots incorrectly.
**Why it happens:** D-07 minimum is connected-block lifecycle; other exits and Replaced semantics are easy to miss.
**How to avoid:** Forward Evicted/Expired via outcome wtxid; for replaced/evicted **victims**, resolve wtxid from TxServing before demotion; never hook the admitted Replaced wtxid as a removal.

### Pitfall 5: Scope creep into Phase 120/121

**What goes wrong:** Timeout ticks or metrics projection land in Phase 119.
**Why it happens:** Nearby APIs (`expire_compact_download_timeouts`, evidence helpers) look related.
**How to avoid:** Only candidate inject + removal slot clear. Note: `PeerManager::on_compact_download_block_connected` exists but is **not** called from the node shell today — do not expand Phase 119 to full block-connected compact cleanup unless needed for D-07/D-09; that cleanup is closer to GOV-03 / Phase 120. [VERIFIED: grep — no node callers]

### Pitfall 6: Parity breadcrumb omission

**What goes wrong:** `scripts/check-parity-breadcrumbs.ts` fails verify.
**How to avoid:** Breadcrumb new/touched files to Knots `blockencodings.cpp` / `net_processing.cpp` (extra-txn ring + InitData).

### Pitfall 7: RCN-03 tests only on empty-facts PeerManager path

**What goes wrong:** Collision/duplicate/missing remain proven only in network-crate unit tests; audit still marks runtime RCN-03 partial.
**How to avoid:** At least one node-shell test that injects colliding or incomplete candidate sets through live receive and asserts typed outcomes/actions.

## Code Examples

### Existing facts API (do not redesign)

```rust
// Source: packages/open-bitcoin-network/src/peer/compact_download_state.rs
pub struct CompactBlockReceiveFacts<'a> {
    pub candidates: &'a [(&'a Wtxid, &'a Transaction)],
    pub extra: &'a [(&'a Wtxid, &'a Transaction)],
}

pub fn handle_compact_block_download(
    &mut self,
    peer_id: PeerId,
    payload: CompactBlockPayload,
    facts: CompactBlockReceiveFacts<'_>,
    now_unix_seconds: i64,
) -> Result<Vec<PeerAction>, NetworkError>
```

### Mempool candidate adaptation

```rust
// Source: packages/open-bitcoin-mempool/src/pool.rs + types.rs
// Mempool::entries() -> &HashMap<Txid, MempoolEntry>
// MempoolEntry { transaction, wtxid, ... }
let owned: Vec<(Wtxid, Transaction)> = mempool
    .entries()
    .values()
    .map(|entry| (entry.wtxid, entry.transaction.clone()))
    .collect();
```

### Lifecycle removal already carries wtxid

```rust
// Source: packages/open-bitcoin-mempool/src/pool/lifecycle.rs
pub struct MempoolLifecycleRemoval {
    pub txid: Txid,
    pub wtxid: Wtxid,
    pub reason: MempoolLifecycleRemovalReason,
}
```

### Partial slot clear + PeerManager forwarder shape

```rust
// Source: packages/open-bitcoin-network/src/compact_reconstruction.rs
pub fn on_mempool_transaction_removed(&mut self, removed_wtxid: &Wtxid) {
    // clears matching txn_available / slot_wtxids entries
}

// Recommended PeerManager addition (network crate, no mempool types):
pub fn on_mempool_transaction_removed(&mut self, removed_wtxid: &Wtxid) {
    for state in self.compact_download_states.values_mut() {
        for in_flight in state.in_flight.values_mut() {
            in_flight.partial.on_mempool_transaction_removed(removed_wtxid);
        }
    }
}
```

### RCN-03 typed outcomes already present

```rust
// Source: packages/open-bitcoin-network/src/compact_reconstruction.rs
// CompactReconstructionOutcome::Ready { missing_indexes }
// CompactReconstructionOutcome::Failed(ShortIdCollision | ShortIdBucketOverload)
// CompactReconstructionOutcome::Invalid(...)
// CompactBlockTxnMisbehavior::DuplicateResponse (blocktxn path — Phase 115)
```

Live injected-path tests should assert Ready-with-missing and Failed(ShortIdCollision) when candidates cause those outcomes; duplicate blocktxn remains covered by existing Phase 115 tests unless an injected-path regression is cheap.

### Knots InitData ordering (parity model)

```cpp
// Source: packages/bitcoin-knots/src/blockencodings.cpp InitData
// 1) walk mempool (pool->txns_randomized) by GetWitnessHash short IDs
// 2) walk extra_txn (vExtraTxnForCompact), skipping nulls
// Collision with different witness hash clears slot (request missing)
```

OB already mirrors this as `candidates` then `extra` iterators in `init_partial_compact_block`. Shell must supply both slices; do not merge extras into candidates.

### Knots extra ring insert (parity model)

```cpp
// Source: packages/bitcoin-knots/src/net_processing.cpp AddToCompactExtraTransactions
// resize to max_extra_txs; overwrite at vExtraTxnForCompactIt; advance % capacity;
// while memusage > max_extra_txs_size: clear next slot and advance
```

Byte accounting: Knots uses `RecursiveDynamicUsage`. For OB, approximate with serialized/virtual size of stored txs — document as Knots-aligned bound, not byte-identical memusage. [ASSUMED: exact RecursiveDynamicUsage parity not required for RCN-02 closeout]

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Phase 114 pure reconstruct with iterator inputs | Still correct — shell never wired | 2026-07-05 | Unit-only RCN-02 |
| Phase 115 download/init with facts parameter | Live dispatch ignores facts | 2026-07-06 | Empty-facts production path |
| Phase 118 outbound announce wiring | Receive inject deferred to 119 | 2026-07-11 | Explicit deferral |
| Milestone audit empty-facts gap | Phase 119 closes feed + lifecycle | 2026-07-13 | RCN-02/03 + GOV-04 |

**Deprecated/outdated:** Treating Phase 114 checklist “done” as runtime satisfaction — audit marked RCN-02 unsatisfied at E2E. [VERIFIED: `v2.1-MILESTONE-AUDIT.md`]

## Discretion Recommendations (for planner)

| Discretion area | Recommendation | Confidence |
|-----------------|----------------|------------|
| Helper placement | Shell intercept in `receive_*` + `compact_receive_candidates.rs` helper; do **not** add mempool to network | HIGH |
| Extra buffer | New `CompactExtraTxnBuffer` on `ManagedPeerNetwork` with Knots defaults; constructor override for tests | HIGH |
| Extra byte budget | Track approximate serialized/virtual size; match Knots slot+byte gates; do not require RecursiveDynamicUsage identity | MEDIUM |
| Wtxid source | Prefer `MempoolLifecycleRemoval.wtxid`; Evicted/Expired via outcome; victims via TxServing lookup before demotion | HIGH |
| Test injection | Prefer live `receive_message` with real mempool admits; extras via buffer `push` in Arrange | HIGH |
| Empty-facts `handle_message` | Keep for PeerManager-only tests; add comment that production receive must inject via shell | HIGH |
| Task wave split | (1) network forwarder + shell intercept + mempool candidates, (2) extra ring + admission pushes, (3) lifecycle hooks + runtime tests + breadcrumbs | HIGH |

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | Exact Knots `RecursiveDynamicUsage` byte accounting is not required for RCN-02; approximate size budget is enough | Pattern 4 / Code Examples | Planner may over-scope memusage fidelity; discuss only if parity reviewers insist |

**If wrong:** Keep Knots slot count + per-txn size limit exact; treat aggregate byte budget as approximate until a later parity polish.

## Open Questions

1. **Should empty-facts CompactBlock in `message_dispatch` be annotated / deprecated in-code?**
   - What we know: D-03 allows test-only retention.
   - What's unclear: Whether to add a comment or `#[cfg(test)]` gate.
   - Recommendation: Keep callable for network-crate tests; add a short comment that production receive must inject via shell. Avoid `#[cfg(test)]` if sync/other callers still use `handle_message` for CompactBlock.

2. **How complete must extra feeds be vs Knots in Phase 119?**
   - What we know: D-05 requires a Knots-shaped bounded buffer; Knots feeds orphans/replaced/rejected.
   - What's unclear: Whether all three feeds are required for requirement closeout vs buffer existence + at least one feed + test injection.
   - Recommendation: Implement ring + push orphans/rejects/replaced-victims when bodies are available; tests may push synthetic extras. Do not block on perfect Knots memusage accounting.

3. **Forward `on_compact_download_block_connected` now?**
   - What we know: API exists; shell never calls it; D-07 focuses on mempool-removal slot clear.
   - Recommendation: Out of Phase 119 unless a D-09 test needs it; note for Phase 120 / GOV-03.

4. **Should `MempoolOutcome::Replaced` grow victim wtxid vectors?**
   - What we know: Victims are txid-only today; TxServing can resolve wtxid for stored txs.
   - Recommendation: Prefer TxServing lookup for Phase 119; only extend mempool outcome shape if lookup proves insufficient for D-09 tests.

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| Rust / Cargo | Implementation + tests | ✓ | 1.94.1 | — |
| Knots submodule sources | Parity anchors | ✓ | present under `packages/bitcoin-knots` | — |
| Bun | Breadcrumb / verify scripts | ✓ | 1.3.9 | — |
| New external services | — | N/A | — | — |

**Missing dependencies with no fallback:** None

**Missing dependencies with fallback:** None

Step 2.6: External deps limited to existing Rust toolchain and repo tools — no blocking gaps.

## Security Domain

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|-----------------|
| V2 Authentication | no | — |
| V3 Session Management | no | — |
| V4 Access Control | no | Peer eligibility / activation already gated in download init |
| V5 Input Validation | yes | Existing compact payload validation in reconstruction / codec (RCN-01) — do not weaken |
| V6 Cryptography | no new | Reuse existing wtxid / SipHash short-ID helpers |

### Known Threat Patterns for compact receive injection

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Oversized extra buffer memory growth | Denial of Service | Cap slots + byte budget to Knots defaults; per-tx size gate on reject feed |
| Untrusted CompactBlock malformed payload | Tampering | Existing InitData Invalid/Failed outcomes before chainstate mutation |
| Stale mempool match after eviction | Tampering / Integrity | Lifecycle wtxid clear on removal |
| Accidental package/filter activation | Elevation of Privilege (scope) | Explicit no-touch of package/filter/public defaults (D-08) |

## Sources

### Primary (HIGH confidence)

- `.planning/phases/119-compact-receive-mempool-candidate-injection/119-CONTEXT.md` — locked D-01..D-11
- `.planning/v2.1-MILESTONE-AUDIT.md` — RCN-02/RCN-03/GOV-04 gap evidence
- `packages/open-bitcoin-network/src/peer/message_dispatch.rs` — empty-facts CompactBlock branch
- `packages/open-bitcoin-network/src/peer/compact_download_state.rs` — `CompactBlockReceiveFacts` / `handle_compact_block_download`
- `packages/open-bitcoin-network/src/compact_download.rs` — `CompactDownloadPeerState` / `CompactDownloadInFlight.partial` / `init_compact_block_download`
- `packages/open-bitcoin-network/src/compact_reconstruction.rs` — `on_mempool_transaction_removed`, InitData scan, typed outcomes
- `packages/open-bitcoin-mempool/src/pool.rs` / `lifecycle.rs` / `types.rs` / `outcome.rs` — entries + removal wtxid + victim txid-only lists
- `packages/open-bitcoin-node/src/network.rs` / `mempool_lifecycle.rs` / `admission_bridge.rs` / `relay_serving.rs` — receive + missing hook + TxServing wtxid lookup
- `packages/bitcoin-knots/src/net_processing.h` — extra-txn defaults
- `packages/bitcoin-knots/src/net_processing.cpp` — `AddToCompactExtraTransactions`, InitData call sites
- `packages/bitcoin-knots/src/blockencodings.cpp` — `PartiallyDownloadedBlock::InitData`
- `packages/open-bitcoin-network/Cargo.toml` — no mempool dependency

### Secondary (MEDIUM confidence)

- `.planning/phases/114-compact-block-reconstruction-from-mempool-state/114-CONTEXT.md` — iterator / no-mempool-coupling policy
- `.planning/ROADMAP.md` Phase 119 success criteria
- `docs/parity/checklist.md` reconstruction surface notes

### Tertiary (LOW confidence)

- None material

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH — first-party crates and Knots anchors verified in-tree
- Architecture: HIGH — intercept + forwarder pattern matches locked decisions and existing types
- Pitfalls: HIGH — borrow conflict, empty-facts trap, wtxid hook gap, and Replaced-victim txid-only caveat verified in code

**Research date:** 2026-07-13
**Valid until:** 2026-08-12 (stable internal APIs; re-check if Phase 120 starts overlapping cleanup hooks)
