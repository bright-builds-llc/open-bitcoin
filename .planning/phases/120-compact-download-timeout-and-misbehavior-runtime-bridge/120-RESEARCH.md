# Phase 120: Compact Download Timeout and Misbehavior Runtime Bridge - Research

**Researched:** 2026-07-13
**Domain:** Compact-block download runtime bridge (timeout tick + misbehavior escalation)
**Confidence:** HIGH

## Summary

Phase 120 is a **runtime wiring** gap-closure, not a greenfield protocol feature. Pure compact-download timeout expiry (`PeerManager::expire_compact_download_timeouts` → `expire_stale_compact_downloads`) and typed `CompactBlockTxnMisbehavior` outcomes already exist and are unit-tested under `open-bitcoin-network`. The v2.1 audit breaks are: (1) `open-bitcoin-node` never calls the expire API, so timeout → full-block `GetData` cannot fire on the live path; (2) `compact_block_txn_actions` maps `Misbehavior(_)`, `UnexpectedBlockHash`, and `NoMatchingInFlight` to an empty `Vec<PeerAction>` (silent suppress). [VERIFIED: packages/open-bitcoin-network/src/peer/compact_download_state.rs] [VERIFIED: .planning/v2.1-MILESTONE-AUDIT.md]

The planner should mirror the existing `ManagedPeerNetwork::expire_transaction_requests` **API shape** (caller-supplied `now_unix_seconds`, thin shell → PeerManager), but must **not** copy its `filter_map` that keeps only `PeerAction::TransactionRelay` — compact expiry returns `PeerAction::Send(GetData(...))`, which that filter would discard. [VERIFIED: packages/open-bitcoin-node/src/network/action_translation.rs] [VERIFIED: packages/open-bitcoin-network/src/compact_download.rs] Escalate typed misbehavior via `PeerAction::Disconnect` (and optionally peer-policy recording) using existing `MisbehaviorKind` / `MisbehaviorPolicy`, while keeping true `NoMatchingInFlight` silent to match Knots ignore behavior. [VERIFIED: packages/bitcoin-knots/src/net_processing.cpp ProcessCompactBlockTxns] [VERIFIED: packages/open-bitcoin-network/src/peer_policy.rs]

**Primary recommendation:** Add `ManagedPeerNetwork::expire_compact_download_timeouts(now)` that translates returned `Send` actions into outbound wire messages (and records `CompactDownloadCleanupCause::Timeout`), invoke it from `receive_message` / `receive_sync_message` with the message timestamp; rewrite `compact_block_txn_actions` (and Invalid init) so Knots-aligned misbehavior yields non-empty disconnect/score/suppression actions.

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

#### Timeout Tick Scheduling Seam

- **D-01:** Call `PeerManager::expire_compact_download_timeouts(now_unix_seconds)` from the node shell on a live runtime path. Prefer a `ManagedPeerNetwork` forwarder that mirrors the existing `expire_transaction_requests` pattern (thin shell → PeerManager → translate/return `PeerAction`s), not a DurableSyncRuntime-only metrics hook.
- **D-02:** The tick must be deterministic and caller-clocked: pass explicit `now_unix_seconds` from the existing receive/drive/poll path that already owns wall-clock for relay timeouts. Do not invent a background thread or Tokio timer as the primary seam; reuse the same “operator/runtime supplies now” contract used for transaction request expiry.
- **D-03:** Timeout expiration must produce live-path `PeerAction`s for full-block fallback (and suppression when policy suppresses). Ensure returned actions are translated and sent the same way other compact download actions already are — not discarded after a pure call.

#### Misbehavior Escalation Bridge

- **D-04:** Stop mapping `CompactBlockTxnHandleOutcome::Misbehavior(_)` to an empty `PeerAction` list. Typed compact misbehavior must escalate to Knots-aligned disconnect, score/discourage, or explicit suppression decisions via existing peer-policy / `PeerAction::Disconnect` / misbehavior recording surfaces.
- **D-05:** Cover GOV-02 cases called out by the audit and requirements: malformed compact blocks, invalid compact-block headers, duplicate `blocktxn`, unexpected `blocktxn`, and out-of-bounds indexes. Prefer mapping through existing `CompactBlockTxnMisbehavior` variants into `MisbehaviorKind` / disconnect reasons rather than inventing a parallel policy stack.
- **D-06:** Keep benign no-match paths suppressible when Knots would ignore them (e.g. true `NoMatchingInFlight` with no in-flight state). Do not treat every empty outcome as disconnect — only typed misbehavior and Knots-aligned unexpected/malformed cases escalate.

#### Volatile Cleanup Contract

- **D-07:** Timeout expiration must clear only volatile compact-download in-flight state for expired entries (already the intent of `expire_stale_compact_downloads`). Disconnect, timeout, and reorg cleanup must continue to remove only volatile compact-relay state — never validated chainstate or durable block data (GOV-03).
- **D-08:** If `on_compact_download_block_connected` (or equivalent block-connect volatile cleanup) is still unwired from the node shell, wire it in this phase as part of GOV-03 completeness. Do not expand into mempool/package surfaces already closed by Phase 119.

#### Verification And Scope Isolation

- **D-09:** Runtime/unit tests must prove: (1) node/shell tick calls `expire_compact_download_timeouts` and yields fallback/suppression actions on the live path, (2) typed misbehavior yields non-empty disconnect/score/suppression actions rather than silence-only, (3) disconnect/timeout/reorg cleanup still touches only volatile compact state, (4) Phase 121 DurableSyncRuntime block-relay metric/log projection, package/filter/public-default surfaces stay untouched.
- **D-10:** New or touched first-party Rust source/test files under `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests` require parity breadcrumbs and `docs/parity/source-breadcrumbs.json` entries unless an explicit `none` breadcrumb is defensible. Prefer Knots `net_processing.cpp` compact download timeout and misbehavior anchors.
- **D-11:** Verification remains deterministic and local through repo-native checks and `bash scripts/verify.sh`. Public-network compact-relay review stays opt-in UAT only.

### Claude's Discretion

The planner/researcher may choose exact tick call-site (ManagedPeerNetwork method invoked from receive loop vs sync drive helper), the precise `MisbehaviorKind` / disconnect-reason mapping table for each `CompactBlockTxnMisbehavior` variant within Knots alignment, whether escalation emits `PeerAction::Disconnect` alone or also records through `record_peer_policy_misbehavior`, and how tests advance `now_unix_seconds` to force expiry. Prefer early returns, the smallest seam that closes the audit gap, and reuse of existing action-translation / peer-policy bridges.

### Deferred Ideas (OUT OF SCOPE)

- Block-relay metrics and structured log runtime projection through `DurableSyncRuntime` — Phase 121 / OBS-03.
- Package relay, bloom/filter serving, compact filters, public defaults, public-network CI, production full-node readiness, production-funds wallet claims — out of v2.1 gap-closure scope.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| RCN-07 | Node falls back to full block fetch or suppression when reconstruction fails, responses timeout, blocks are old/far from tip, or peer/resource state becomes ineligible. | Wire `ManagedPeerNetwork::expire_compact_download_timeouts` on the live receive path so expiry emits `GetData(Block)` fallback actions; eligibility/reconstruction-failure fallbacks already exist in `init_compact_block_download`. |
| GOV-02 | Malformed compact blocks, invalid compact-block headers, duplicate/unexpected `blocktxn`, and out-of-bounds indexes produce Knots-aligned misbehavior, disconnect, or suppression. | Escalate `compact_block_txn_actions` beyond empty Vec; change Invalid init from silent Fallback-only to misbehavior/disconnect for Knots `READ_STATUS_INVALID` parity; keep `NoMatchingInFlight` silent. |
| GOV-03 | Restart, reconnect, disconnect, timeout, and reorg cleanup remove volatile compact-relay state without deleting validated chainstate or durable block data. | Timeout expire already clears only `in_flight`; wire Timeout evidence recording; confirm/wire `on_compact_download_block_connected` on the node `ReceivedBlock` path so all peers clear matching volatile slots. |
</phase_requirements>

## Project Constraints (from .cursor/rules/)

No `.cursor/rules/` directory present in this repository. [VERIFIED: glob .cursor/rules]

Actionable constraints instead come from `AGENTS.md` / Bright Builds standards (loaded for this research):

- Functional core / imperative shell — keep expiry and misbehavior **decisions** in `open-bitcoin-network`; node shell only clocks, translates, and applies effects. [CITED: standards/core/architecture.md]
- Prefer early returns / `let...else`; prefix `Option` locals with `maybe_`. [CITED: standards/core/code-shape.md] [CITED: standards/languages/rust.md]
- Unit-test pure/business logic with Arrange/Act/Assert; one concern per test. [CITED: standards/core/testing.md]
- No Rust Bitcoin libraries on the production path; verify with `bash scripts/verify.sh`; parity breadcrumbs required for new/touched first-party Rust sources. [CITED: AGENTS.md Repo-Local Guidance]
- Rust toolchain pinned at `1.94.1`. [VERIFIED: rustc --version]

## Standard Stack

### Core

| Library / Surface | Version / Pin | Purpose | Why Standard |
|-------------------|---------------|---------|--------------|
| Rust edition 2024 / rustc | 1.94.1 | Implementation language | Repo pin via `rust-toolchain.toml` [VERIFIED: rustc --version] |
| `open-bitcoin-network` | workspace | Pure compact download, PeerManager, PeerAction, peer policy | Functional core for timeout + misbehavior decisions [VERIFIED: codebase] |
| `open-bitcoin-node` (`ManagedPeerNetwork`) | workspace | Imperative shell: receive path, action translation, evidence | Live-path forwarder and effect application [VERIFIED: codebase] |
| Bitcoin Knots baseline | `29.3.knots20260210` under `packages/bitcoin-knots` | Parity anchors for Misbehaving / ignore / fallback | Behavioral baseline [CITED: AGENTS.md] |

### Supporting

| Surface | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| `MisbehaviorKind` / `MisbehaviorPolicy` / `PeerPolicyRuntimeState` | existing | Score/discourage/ban decisions + bounded evidence | Map compact misbehavior into existing policy, not a new ban book [VERIFIED: peer_policy.rs] |
| `ManagedPeerNetwork::record_peer_policy_misbehavior` | existing | Persist `MisbehaviorDecision` in runtime state | Optional accompaniment to `PeerAction::Disconnect` [VERIFIED: peer_policy.rs node] |
| `CompactDownloadCleanupCause::Timeout` evidence | existing counters | Operator cleanup/timeout counters | Record on shell expiry tick [VERIFIED: block_relay_evidence.rs] |
| `COMPACT_BLOCK_DOWNLOAD_TIMEOUT_SECONDS` | `60` | Deterministic compact in-flight age | Keep Phase 115 constant; do not retune to Knots IBD stall math in this phase [VERIFIED: compact_download.rs] |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Receive-path piggyback tick | Dedicated Tokio interval / background task | Forbidden by D-02; non-deterministic relative to injected clocks |
| DurableSyncRuntime-only tick | ManagedPeerNetwork forwarder | Forbidden by D-01; conflates Phase 121 metrics work |
| New compact-only ban book | Existing `MisbehaviorKind` + Disconnect | Parallel policy stack violates D-05 |
| Copy TX expire `filter_map` verbatim | Translate `PeerAction::Send` | TX filter would drop compact `GetData` fallbacks |

**Installation:** N/A — no new crates. Use existing workspace packages.

**Version verification:** `rustc 1.94.1` / `cargo 1.94.1` confirmed on research host. [VERIFIED: shell]

## Architecture Patterns

### Recommended Project Structure (touched seams only)

```
packages/open-bitcoin-network/src/
├── peer/compact_download_state.rs   # expire_compact_download_timeouts; compact_block_txn_actions (escalate)
├── compact_download.rs              # expire_stale; Invalid init outcome (GOV-02); timeout constant
├── compact_reconstruction.rs        # CompactBlockTxnMisbehavior variants (reuse)
└── peer_policy.rs                   # MisbehaviorKind mapping targets (reuse)

packages/open-bitcoin-node/src/network/
├── action_translation.rs            # NEW expire_compact_download_timeouts forwarder; maybe ReceivedBlock cleanup
├── network.rs                       # receive_* tick call-site
├── block_relay_evidence.rs          # Timeout cleanup recording (existing helpers)
└── tests/                           # ManagedPeerNetwork runtime proofs (D-09)
```

### Pattern 1: Caller-clocked ManagedPeerNetwork expiry forwarder

**What:** Thin shell method takes `now_unix_seconds`, calls PeerManager pure expiry, translates returned `PeerAction`s into outbound wire messages (and optional evidence), returns messages to the caller.

**When to use:** Any timeout that must be deterministic under tests and free of background timers (D-01/D-02).

**Example (target shape — do not copy TX filter):**

```rust
// Source: packages/open-bitcoin-node/src/network/action_translation.rs (expire_transaction_requests pattern)
// Compact variant MUST keep PeerAction::Send, unlike the TX filter below.
pub fn expire_transaction_requests(
    &mut self,
    now_unix_seconds: i64,
) -> ManagedResult<Vec<(PeerId, WireNetworkMessage)>> {
    Ok(self
        .peer_manager
        .expire_transaction_requests(now_unix_seconds)
        .into_iter()
        .filter_map(|(_peer_id, action)| match action {
            PeerAction::TransactionRelay(action) => process_transaction_relay_action(action),
            _ => None,
        })
        .collect())
}
```

**Recommended compact forwarder behavior:** [ASSUMED — planner discretion on exact return type]

1. Count in-flight before/after (or count expired) → `record_compact_cleanup(Timeout, n)`.
2. `let actions = self.peer_manager.expire_compact_download_timeouts(now_unix_seconds);`
3. Map `PeerAction::Send(msg)` into outbound (broadcast or peer-targeted — today expiry actions are unscoped `Send`; translate like other compact `GetData` fallbacks already handled via `process_actions` / `collect_outbound`).
4. Prefer invoking from `receive_message` / `receive_sync_message` with the same `timestamp` already passed into PeerManager (smallest live-path seam).

### Pattern 2: Pure outcome → PeerAction escalation in network crate

**What:** Keep misbehavior classification in pure handlers; map outcomes to `PeerAction::Disconnect` / Send / empty in a small matcher (`compact_block_txn_actions`).

**When to use:** GOV-02 escalation without pulling node policy into the network core’s message decode path.

**Current gap (must change):**

```rust
// Source: packages/open-bitcoin-network/src/peer/compact_download_state.rs
fn compact_block_txn_actions(outcome: CompactBlockTxnHandleOutcome) -> Vec<PeerAction> {
    match outcome {
        CompactBlockTxnHandleOutcome::Progress { actions } => {
            compact_download_actions_to_peer_actions(actions)
        }
        CompactBlockTxnHandleOutcome::Misbehavior(_)
        | CompactBlockTxnHandleOutcome::UnexpectedBlockHash
        | CompactBlockTxnHandleOutcome::NoMatchingInFlight => Vec::new(),
    }
}
```

### Pattern 3: Knots-aligned ignore vs Misbehaving

**What:** Knots `ProcessCompactBlockTxns` **returns without Misbehaving** when the peer was not expected to send `blocktxn` for that hash; invalid reconstruction / invalid InitData calls `Misbehaving` and clears in-flight. [VERIFIED: net_processing.cpp ~3358–3381, ~4459–4461]

**When to use:** Decide which empty outcomes stay silent (D-06) vs escalate (D-04/D-05).

### Anti-Patterns to Avoid

- **Copying TX expire filter for compact expiry:** Drops `GetData` fallbacks → RCN-07 still broken on live path.
- **Tokio/background timer as primary tick:** Violates D-02.
- **Touching DurableSyncRuntime::persist_metrics / block_relay_log_record:** Phase 121 / OBS-03 (D-11).
- **Treating NoMatchingInFlight as disconnect:** Diverges from Knots ignore path (D-06).
- **Leaving Invalid init as Fallback-only:** GOV-02 malformed/invalid-header cases currently become full-block fetch instead of Misbehaving. [VERIFIED: init_compact_block_download Invalid arm]
- **Mutating chainstate from timeout/misbehavior cleanup:** Only clear `compact_download_states` / `in_flight` (D-07).

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Timeout bookkeeping | New timer wheel / async task | `expire_stale_compact_downloads` + PeerManager expire | Already tested; caller clock is the contract |
| Misbehavior scoring | Compact-only ban list | `MisbehaviorKind` + `MisbehaviorPolicy` + optional `record_peer_policy_misbehavior` | Phase 96 bridge already exists |
| Action application | Ad-hoc send loops | Existing `process_actions` / `collect_outbound` / Disconnect handling | `PeerAction::Disconnect` already disconnects on live path [VERIFIED: action_translation.rs] |
| Full-block fallback encoding | Custom getdata builder | `compact_download_actions_to_peer_actions(RequestFullBlock)` | Already emits `GetData` Block inv [VERIFIED: compact_download.rs] |
| Block-connect volatile clear | New cleanup API | `PeerManager::on_compact_download_block_connected` | Exists; PeerManager `handle_block` already calls it [VERIFIED: inventory_state.rs] |

**Key insight:** This phase is almost entirely **bridging existing pure APIs into the node shell and stopping silent empty-action mapping** — not inventing new compact protocol logic.

## Common Pitfalls

### Pitfall 1: TX-style forwarder drops compact GetData

**What goes wrong:** `expire_compact_download_timeouts` returns actions but shell filters them away; tests on PeerManager still pass while node live path stays broken.
**Why it happens:** `expire_transaction_requests` only keeps `TransactionRelay`.
**How to avoid:** Assert ManagedPeerNetwork-level outbound contains `WireNetworkMessage::GetData` after advancing `now_unix_seconds`.
**Warning signs:** Only PeerManager unit tests green; no node caller or node test for expire.

### Pitfall 2: Expire without Timeout evidence / cleanup accounting

**What goes wrong:** GOV-03 timeout cleanup counters never move; audit still looks partial.
**Why it happens:** `record_compact_cleanup(Timeout, …)` exists but only disconnect currently records cleanup from the shell. [VERIFIED: grep record_compact_cleanup]
**How to avoid:** Shell forwarder records Timeout with expired count.

### Pitfall 3: Escalating NoMatchingInFlight

**What goes wrong:** Benign late/stray `blocktxn` disconnects peers Knots would ignore.
**Why it happens:** Grouping all empty outcomes together in one match arm (current code).
**How to avoid:** Split arms — escalate Misbehavior + UnexpectedBlockHash; keep NoMatchingInFlight empty.

### Pitfall 4: Invalid compact still only Fallback

**What goes wrong:** GOV-02 “malformed / invalid headers” remain unsatisfied while Duplicate/OOB escalate.
**Why it happens:** `CompactReconstructionOutcome::Invalid(_) => Fallback` in init. [VERIFIED: compact_download.rs]
**How to avoid:** Map Invalid init to misbehavior/disconnect (Knots `Misbehaving("invalid compact block")`); keep collision `Failed` as Fallback (Knots `READ_STATUS_FAILED` → getdata).

### Pitfall 5: ReceivedBlock path skips multi-peer volatile clear

**What goes wrong:** Compact completion emits `ReceivedBlock` via `process_actions` without calling `on_compact_download_block_connected`, so **other peers’** in-flight for the same hash can linger until timeout. PeerManager `handle_block` (wire `Block` message) does clear all peers. [VERIFIED: inventory_state.rs] [VERIFIED: action_translation.rs ReceivedBlock arm]
**How to avoid:** Per D-08, call `on_compact_download_block_connected(block_hash)` from the node shell when applying `ReceivedBlock` (and record `BlockConnected` cleanup if counts > 0).

### Pitfall 6: DisconnectReason vocabulary gap

**What goes wrong:** Using `ResourceLimit` or `SelfConnection` for compact misbehavior muddies diagnostics.
**Why it happens:** `DisconnectReason` today is only DuplicateVersion / SelfConnection / ResourceLimit / MissingHeaderAncestor. [VERIFIED: error.rs]
**How to avoid:** Prefer adding a compact/misbehavior-aligned `DisconnectReason` variant (discretion) **or** Disconnect + separate `MisbehaviorDecision` evidence with `MisbehaviorKind::MalformedMessage` / `HeaderViolation`.

### Pitfall 7: Phase 121 bleed

**What goes wrong:** Touching `persist_metrics` / `block_relay_log_record` while fixing timeouts.
**How to avoid:** Explicit negative test or plan acceptance criterion that those files stay untouched (D-09.4 / D-11).

## Code Examples

### Verified: PeerManager expiry already emits GetData fallback

```rust
// Source: packages/open-bitcoin-network/src/peer/tests.rs
// phase115_expire_compact_download_timeouts_requests_full_blocks
let actions = manager
    .expire_compact_download_timeouts(100 + crate::COMPACT_BLOCK_DOWNLOAD_TIMEOUT_SECONDS + 1);
assert!(matches!(
    &actions[0],
    PeerAction::Send(WireNetworkMessage::GetData(inventory))
        if inventory.inventory[0].inventory_type == InventoryType::Block
));
```

### Verified: Knots ignores unexpected blocktxn (no Misbehaving)

```cpp
// Source: packages/bitcoin-knots/src/net_processing.cpp ProcessCompactBlockTxns
if (!requested_block_from_this_peer) {
    LogDebug(BCLog::NET, "Peer %d sent us block transactions for block we weren't expecting\n", pfrom.GetId());
    return;
}
```

### Verified: Knots Misbehaving for invalid compact / non-matching txns

```cpp
// Source: packages/bitcoin-knots/src/net_processing.cpp
if (status == READ_STATUS_INVALID) {
    RemoveBlockRequest(...);
    Misbehaving(peer, "invalid compact block/non-matching block transactions");
    return;
}
// InitData invalid:
Misbehaving(*peer, "invalid compact block");
```

### Verified: Modern Knots Misbehaving sets discourage flag

```cpp
// Source: packages/bitcoin-knots/src/net_processing.cpp Misbehaving
peer.m_should_discourage = true;
```

**Mapping implication:** Prefer `MisbehaviorResponse::Discourage` (or Disconnect) over ObserveOnly for compact Misbehaving cases — use points ≥ `discourage_threshold` (default 50) or emit `PeerAction::Disconnect` directly. [VERIFIED: MisbehaviorPolicy defaults] [ASSUMED: exact points table is planner discretion within Knots discourage alignment]

### Recommended misbehavior mapping table (discretion, Knots-aligned)

| Outcome / reason | Knots analog | Open Bitcoin action |
|------------------|--------------|---------------------|
| `NoMatchingInFlight` | unexpected blocktxn log+return | empty Vec (suppress) |
| `DuplicateResponse` | prior reconstruction failed / duplicate path → Misbehaving | `Disconnect` + `MalformedMessage` |
| `OutOfBoundsIndex` | getblocktxn OOB / invalid indexes → Misbehaving | `Disconnect` + `MalformedMessage` |
| `UnexpectedBlockHash` / non-matching | “invalid compact block/non-matching block transactions” | `Disconnect` + `MalformedMessage` |
| `TooManyTransactions` / `NotInitialized` | invalid compact / bad FillBlock | `Disconnect` + `MalformedMessage` |
| Init `Invalid(NullHeader)` etc. | InitData `READ_STATUS_INVALID` | `Disconnect` + `HeaderViolation` or `MalformedMessage` (not Fallback) |
| Init `Failed(ShortIdCollision\|BucketOverload)` | `READ_STATUS_FAILED` → getdata | keep Fallback `GetData` |

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Incremental DoS score per compact fault | Knots `Misbehaving` → `m_should_discourage = true` | Knots modern net_processing | Prefer discourage/disconnect over inventing fine-grained score ladders for compact only |
| Phase 115 pure expire API | Still unwired from node | Audit v2.1 | Phase 120 must schedule |
| Silent empty PeerAction on misbehavior | Typed outcomes exist | Phase 115 | Phase 120 must escalate |

**Deprecated/outdated:**

- Treating “empty PeerAction = Knots-aligned suppress” for **typed Misbehavior** — only true for NoMatchingInFlight.

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | Best live tick call-site is `receive_message` / `receive_sync_message` piggyback with message `timestamp` | Architecture Patterns | Planner may prefer explicit public tick only (like TX expire, which itself is test-called today); still must be a node live path per success criteria |
| A2 | Exact Misbehavior points / whether to always Disconnect vs Discourage-only is discretionary if Knots discourage intent is preserved | Mapping table | Over-disconnect vs under-escalate vs peer-policy evidence gaps |
| A3 | Adding a `DisconnectReason` variant is preferred over overloading `ResourceLimit` | Pitfall 6 | Compatibility/diagnosis churn if variant naming differs |
| A4 | Witness `GetData` (`InventoryType::WitnessBlock`) can remain out of scope for this phase’s Block fallback | Standard Stack | SegWit peers may prefer witness getdata; Phase 115 already ships Block-type fallback |

**If empty:** N/A — assumptions listed above for discretion items only.

## Open Questions (RESOLVED)

1. **Tick call-site breadth** — RESOLVED: Do both. Public `ManagedPeerNetwork::expire_compact_download_timeouts` **and** call it from `receive_message` / `receive_sync_message` so any live traffic advances expiry without a new daemon loop (Plans 01).
   - What we know: TX expire forwarder exists but has **no production caller** under node/sync/rpc (only tests). [VERIFIED: ripgrep]
   - Recommendation locked by Plan 01: public API + receive_* piggyback.

2. **Invalid init: Fallback vs Misbehavior** — RESOLVED: Change Invalid → Disconnect/misbehavior escalate; keep Failed (collision) → Fallback (Plan 02). Document any remaining OB-vs-Knots differences in parity docs if needed.
   - What we know: Current code Fallbacks; Knots Misbehaves on InitData invalid; GOV-02 lists malformed/invalid headers.

3. **Peer-targeted vs unscoped Send on expire** — RESOLVED: PeerManager expire returns peer-scoped `Vec<(PeerId, PeerAction)>` pairs (like TX expire) so fallback GetData targets the peer that timed out (Plan 01).

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|-------------|-----------|---------|----------|
| rustc / cargo | Build & tests | ✓ | 1.94.1 | — |
| `packages/bitcoin-knots` submodule | Parity anchors | ✓ | present (`net_processing.cpp`) | — |
| Bun / verify.sh | Repo verification | ✓ (repo contract) | — | `bash scripts/verify.sh` |
| Tokio timer / public network | Not required | N/A | — | Do not use as primary tick |

**Missing dependencies with no fallback:** None.

**Missing dependencies with fallback:** None.

Step 2.6: External tools beyond Rust workspace are not required for this phase’s implementation (code/config only + local verify).

## Security Domain

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|------------------|
| V2 Authentication | no | — |
| V3 Session Management | no | — |
| V4 Access Control | partial | Peer role/protection already gates `MisbehaviorPolicy` protected peers |
| V5 Input Validation | yes | Typed compact decode + `CompactReconstructionInvalidReason` / misbehavior outcomes at boundary |
| V6 Cryptography | no | — |

### Known Threat Patterns for compact download bridge

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Malformed cmpctblock / blocktxn DoS | Denial of Service / Tampering | Escalate to disconnect/discourage; clear volatile in-flight only |
| Unexpected blocktxn spam | DoS | Knots ignore when no match; do not amplify into disconnect storms |
| Stale in-flight pinning | DoS | Deterministic expire → full-block fallback + clear slot |
| Accidental chainstate wipe on cleanup | Tampering | GOV-03: volatile maps only; never durable block/chainstate deletes |
| Metrics/log label explosion | Information Disclosure | Out of scope (Phase 121); keep low-cardinality evidence counters only |

## Sources

### Primary (HIGH confidence)

- `packages/open-bitcoin-network/src/peer/compact_download_state.rs` — expire API; silent misbehavior mapping
- `packages/open-bitcoin-network/src/compact_download.rs` — timeout constant, expire_stale, Invalid→Fallback, action translation
- `packages/open-bitcoin-node/src/network/action_translation.rs` — TX expire forwarder; process_actions Disconnect/ReceivedBlock
- `packages/open-bitcoin-network/src/peer/inventory_state.rs` — `handle_block` → `on_compact_download_block_connected`
- `packages/bitcoin-knots/src/net_processing.cpp` — ProcessCompactBlockTxns, CMPCTBLOCK InitData, Misbehaving, block download timeout
- `packages/bitcoin-knots/src/blockencodings.cpp` — READ_STATUS_INVALID paths
- `.planning/v2.1-MILESTONE-AUDIT.md` — RCN-07 / GOV-02 / GOV-03 gap evidence
- `.planning/phases/120-.../120-CONTEXT.md` — locked decisions D-01..D-11
- `standards/core/{architecture,code-shape,testing}.md`, `standards/languages/rust.md`, `AGENTS.md`

### Secondary (MEDIUM confidence)

- Phase 115/119 CONTEXT — prior deferrals and timeout design intent
- Phase 96 peer-policy runtime bridge patterns
- `docs/parity/*` RCN-07/GOV-02 checklist entries (claim “done” while audit marks unsatisfied — treat audit as authoritative for this phase)

### Tertiary (LOW confidence)

- Exact peer-targeting semantics for unscoped expire `Send` actions — needs planner confirmation (Open Question 3)

## Metadata

**Confidence breakdown:**

- Standard stack: HIGH — workspace crates and Knots submodule verified locally
- Architecture: HIGH — gaps and forwarder pattern verified in code; call-site piggyback is discretionary but constrained
- Pitfalls: HIGH — silent mapping, TX filter trap, Invalid→Fallback, ReceivedBlock cleanup gap verified
- Knots discourage-vs-disconnect points table: MEDIUM — modern Misbehaving is discourage-flag based; OB score thresholds are local policy

**Research date:** 2026-07-13
**Valid until:** 2026-08-12 (stable domain; re-check if PeerAction/DisconnectReason or compact_download APIs change)

## RESEARCH COMPLETE

Phase 120 research is complete. Planner can create PLAN.md files from the locked decisions, seam map, and pitfalls above.
