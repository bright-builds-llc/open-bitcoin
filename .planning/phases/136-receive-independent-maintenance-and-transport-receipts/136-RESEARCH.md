# Phase 136: Receive-Independent Maintenance and Transport Receipts - Research

**Researched:** 2026-08-15
**Domain:** Initial-broadcast retry, receive-independent maintenance, TX-write receipts, ordinary topological package fanout
**Confidence:** HIGH

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

#### Unbroadcast-clear receipt

- **D-01:** Never clear unbroadcast membership on an inventory write, inventory
  queue, or INV successful-write receipt. An INV is announcement, not
  acknowledgement.
- **D-02:** The applying receipt that may clear a still-present local
  unbroadcast member is a successful write of the transaction response
  (`WireNetworkMessage::Tx`), recorded as `MempoolRetryClearCause::TransportWritten`
  through the existing Phase 134 `acknowledge_write` / `CompletePeerEmission`
  path. Failed encode, reject, disconnect, or write abort leaves the member
  retry-eligible.
- **D-03:** `EligibleServe` remains the documented Knots-equivalent semantic
  (eligible GETDATA found the transaction and a TX serve was classified). It
  must not emit a membership-clearing retry-clear before the TX write succeeds.
  Existing precedence stays `LifecycleRemoval > TransportWritten > EligibleServe`.
- **D-04:** Authoritative lifecycle removal (confirmation, replacement, expiry,
  eviction, conflict, or absence from the mempool) still clears unbroadcast
  immediately via `LifecycleRemoval`. A successful TX write never claims
  network-wide or guaranteed propagation.

#### Retry cycle timing

- **D-05:** Use one process-global retry cycle, not per-member due times. The
  first cycle is a fresh injected 10-to-15-minute window from node start or
  recovery install. Each completed tick schedules the next cycle the same way.
- **D-06:** Restart and recovery remint a fresh injected cycle. Do not persist
  or restore leftover due times; derived retry timers are not durable source
  state.
- **D-07:** First announcement of a newly accepted local transaction remains
  the existing immediate fanout path. The retry timer never replaces that first
  hop. A member admitted just after a tick waits until the next cycle for retry.
- **D-08:** Cycle length is the existing Phase 130 contract: 10-minute base plus
  injected `RetryJitterSeconds` in `0..=300`. The shell supplies
  `RetryDecisionContext { observed_at_unix_seconds, jitter }`; pure policy never
  samples time or randomness.

#### Per-tick work caps

- **D-09:** The unbroadcast set itself remains the IBR-01 membership bound
  (existing 5,000-member persisted-input / runtime cap). That cap is not the
  per-tick work budget.
- **D-10:** Each receive-independent maintenance tick has a separate inspect
  budget (max identities considered) and prepare budget (max emissions prepared).
  Prepared identities then enter the existing Phase 104 fanout path and still
  honor `PHASE104_MAX_TX_FANOUT_QUEUE_PER_PEER` and
  `PHASE104_MAX_TX_FANOUT_DRAIN_PER_PEER`.
- **D-11:** Identities past the inspect or prepare budget spill to the next
  injected cycle as still-due and unattempted. Queue-cap, rate-limit, and
  suppression labels must not be recorded as attempts for members the tick
  never prepared.
- **D-12:** Walk leftover members with a deterministic cursor so the tail is
  not starved across cycles. Do not add a parallel fanout path.

#### Parent-before-child package fanout

- **D-13:** Accepted, still-present members that were admitted together as one
  package enter the existing per-peer fanout queue in parent-before-child order.
  Drain, rate, identity (txid/wtxid), activation, and peer-eligibility controls
  stay on the existing path.
- **D-14:** Independently admitted parents or children use ordinary single-
  transaction fanout. Do not re-announce an already-present parent merely
  because a later child was accepted.
- **D-15:** Do not require emit/write order to be parent-before-child across
  peers or ticks. Rate and drain may split a pair across ticks. Do not walk
  unrelated mempool ancestors or add a package wire message.

#### Maintenance wakeup

- **D-16:** A shell-owned timer in the node/daemon runtime wakes even when no
  peer messages arrive and issues one `MaintenanceTick { now, jitter }` into
  `ManagedNetworkHandle`. Clocks and randomness stay in the shell.
- **D-17:** Do not piggyback retry scheduling on inbound or outbound message
  loops. Do not couple the retry timer to `DurableSyncRuntime` or IBD.
- **D-18:** Production samples `SystemTime` plus a fallible jitter source in
  the shell. Tests inject deterministic `now` and jitter with no sleeps. Default
  verification must stay hermetic: no wall-clock or public-network gates.
- **D-19:** Retry uses existing relay activation, peer eligibility, txid/wtxid
  selection, rate limits, bounded outboxes, serving, and successful-write
  receipts. Relay-disabled or ineligible operation may admit locally while
  emitting no public fanout and clearing unbroadcast only by lifecycle removal
  or a later eligible TX serve.

### Claude's Discretion

- Exact inspect-N and prepare-M constants, provided they are documented,
  fake-clock-assertable, smaller than the 5,000-member set cap, and do not
  silently treat PHASE104 queue/drain caps as the only IBR-02 bound.
- Exact module, command, cursor, and receipt-filter names, provided TX-only
  write completion remains the applying unbroadcast-clear receipt.
- Where the shell timer lives (`open-bitcoind` versus node runtime adapter),
  provided it is shutdown-aware, not a second policy authority, and not a
  public-default relay loop.

### Deferred Ideas (OUT OF SCOPE)

- Broad RPC, dashboard, metrics, logs, and support presentation of retry/fanout evidence — Phase 137.
- Integrated Knots parity catalog closeout, adversarial pressure, and claim guardrails — Phase 138.
- Whole-mempool or wallet-wide periodic rebroadcast — out of v2.2 scope.
- General package wire protocol or helpful ancestor re-announce beyond co-admitted package members — out of v2.2 scope.
- Public/default/production relay or guaranteed-propagation claims — permanently deferred for this milestone.

None — discussion stayed within phase scope.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| PPKG-04 | Accepted and still-present package members enter existing transaction serving and relay fanout in parent-before-child order under existing activation, peer-policy, queue, rate, and txid/wtxid controls. | `PreparedLifecycleFacts::final_present()` is admitted topological order. Package `lifecycle_delta` records only `FinallyPresent` members, so `AlreadyPresent` parents are not re-enqueued. `TxFanoutQueue` is FIFO `push_back` / `pop_front`. Plan must keep that enqueue order and must not add a second fanout path. |
| IBR-01 | Track only bounded locally submitted, relay-requested, still-present transactions for initial broadcast retry; never treat the whole mempool as a rebroadcast set. | `MempoolEntryMetadata::is_retry_eligible` already requires local + requested + current. `MAX_UNBROADCAST_MEMBERS` and `MAX_MEMPOOL_SNAPSHOT_UNBROADCAST_MEMBERS` are both 5,000. Tick walk must iterate `unbroadcast_members`, never `mempool.entries()`. |
| IBR-02 | Receive-independent maintenance schedules fresh randomized 10-to-15-minute retry cycles from injected inputs and caps work and emissions per tick. | `RetryJitterSeconds` (`0..=300`) and `RetryDecisionContext` exist. No production timer exists. Recommend inspect=256 / prepare=32, plus a deterministic cursor. Shell remints `now + 600 + jitter` on start, recovery, and after each completed tick. |
| IBR-03 | Retry uses existing relay activation, peer eligibility, txid/wtxid selection, rate limits, bounded outboxes, serving paths, and successful transport receipts rather than a parallel fanout path. | Reuse `TxFanoutQueue::enqueue_admission` / `drain_peer`, `PHASE104_*` bounds, `PrepareRelay` / `acknowledge_write` / `CompletePeerEmission`. Do not add `rebroadcast.rs` as a second announcer. Generalize outboxes so TX INV does not credit compact inventory-fallback evidence. |
| IBR-04 | Unbroadcast membership clears only at the documented eligible serve or successful-write receipt, or on authoritative lifecycle removal, and survives supported restart without claiming guaranteed propagation. | Production today emits only `LifecycleRemoval`. `EligibleServe` must stay semantic (GETDATA classified `Served`) and must not clear pre-write. Applying clear is `TransportWritten` on a fresh `WireNetworkMessage::Tx` write. Restart remints the cycle; Phase 135 already persists membership without due times. |
</phase_requirements>

## Summary

Phase 136 does not invent a new relay stack. It turns existing Phase 104 fanout, Phase 130 retry types, Phase 134 receipts, and Phase 135 durable unbroadcast membership into a receive-independent initial-broadcast retry loop, and it makes co-admitted package members leave the existing FIFO in parent-before-child order.

Three current seams will otherwise make the locked decisions fail. First, `prepare_unbroadcast_projection` re-inserts every retry-eligible `final_present` member, so a later package or maintenance delta would put a `TransportWritten`-cleared parent back into the set. Second, reconciliation currently expects unbroadcast to equal the retry-eligible canonical subset, which becomes false the moment a still-present member is cleared by a TX write. Third, `PeerEmission` is compact-coupled: `WireNetworkMessage::Tx` cannot enter it, GETDATA TX responses are encoded as Immediate bytes with no receipt, and any INV forced through `prepare_peer_emission` would increment compact inventory-fallback evidence.

**Primary recommendation:** Extend `retry.rs` and the existing fanout/receipt path; add a `ManagedNetworkHandle` maintenance tick that walks only `unbroadcast_members` with inspect/prepare budgets and a deterministic cursor; route GETDATA TX through a TX-specific `PeerEmission` write kind so only a fresh successful `Tx` write applies `TransportWritten`; put the shutdown-aware timer beside the Phase 135 checkpoint worker in `open-bitcoind`.

## Project Constraints (from .cursor/rules/)

No `.cursor/rules/` directory exists in this repository. [VERIFIED: workspace glob]

Actionable constraints that apply instead, from `AGENTS.md`, `AGENTS.bright-builds.md`, `standards/core/architecture.md`, `standards/languages/rust.md`, and `standards-overrides.md`:

- Functional core / imperative shell: clocks, jitter, sockets, and timers stay in adapters; pure policy receives injected `RetryDecisionContext`. [CITED: standards/core/architecture.md]
- New multi-file Rust modules use `foo.rs` plus `foo/`, not `foo/mod.rs`. [CITED: standards/languages/rust.md]
- Prefix optional internals with `maybe_`; prefer `let...else`; no `unwrap()` in production. [CITED: standards/languages/rust.md]
- Add parity breadcrumbs for new first-party Rust sources under `packages/open-bitcoin-*/src` or `tests`. [CITED: AGENTS.md Repo-Local Guidance]
- Verification contract is `bash scripts/verify.sh`. Default verification stays hermetic. [CITED: AGENTS.md]
- Do not use existing Rust Bitcoin libraries in the production path. [CITED: AGENTS.md]
- `standards-overrides.md` has no active override (placeholder row only). [VERIFIED: standards-overrides.md]
- No project skills are installed. [VERIFIED: AGENTS.md GSD skills block]

## Standard Stack

### Core

| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| Rust | `1.94.1` / edition 2024 | Typed retry, fanout, receipt, and cursor policy | Pinned by `rust-toolchain.toml`; local `rustc`/`cargo` report `1.94.1`. [VERIFIED: rust-toolchain.toml, `rustc --version`] |
| First-party workspace crates | `0.1.0` | Mempool eligibility, network fanout, node authority, RPC shell | Existing production path; no new Bitcoin-domain crate. [VERIFIED: packages/Cargo.toml] |
| Bitcoin Knots | `29.3.knots20260210` | Observable retry cadence and GETDATA acknowledgement baseline | Pinned submodule; `ReattemptInitialBroadcast` and `RemoveUnbroadcastTx` are the anchors. [VERIFIED: packages/bitcoin-knots/src/net_processing.cpp] |

### Supporting

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| Tokio | `1.52.1` (`macros`, `net`, `rt-multi-thread`, `signal`, `test-util`, `time`) | Daemon runtime and fake-clock tests | Shell wakeup only. Do not add Tokio to `open-bitcoin-mempool` or `open-bitcoin-network`. [VERIFIED: packages/open-bitcoin-rpc/Cargo.toml] |
| `getrandom` | `0.3.4` | Fallible production jitter | Sample in the shell; map failure to a typed degraded scheduling outcome, never a silent constant. [VERIFIED: packages/open-bitcoin-node/Cargo.toml, packages/open-bitcoin-rpc/Cargo.toml] |
| `std::time::{SystemTime, Duration}` | Rust `1.94.1` | Production `now` and wait intervals | Acquire only in `open-bitcoind` / node adapters. [VERIFIED: packages/open-bitcoin-rpc/src/bin/open_bitcoind/runtime_control.rs] |
| `std::sync::mpsc` + `thread` | Rust `1.94.1` | Shutdown-aware timer loop | Copy the Phase 135 checkpoint-worker pattern. [VERIFIED: packages/open-bitcoin-rpc/src/bin/open_bitcoind/checkpoint.rs] |
| Fjall | existing | Unchanged snapshot membership | Phase 135 already persists unbroadcast identities without due times. Do not add timer fields. [VERIFIED: packages/open-bitcoin-node/src/storage/mempool_snapshot.rs] |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Checkpoint-style `thread` + `recv_timeout` timer in `open-bitcoind` | Tokio `interval` task on the RPC runtime | Tokio is already the daemon runtime and has `test-util`, but the checkpoint worker is the proven shutdown-aware, injectible, non-sync-coupled pattern. Use the checkpoint pattern. |
| New `rebroadcast.rs` announcer | Extend `retry.rs` + existing `TxFanoutQueue` | A new announcer would violate IBR-03 / D-12. Research names are acceptable only as thin wrappers over the existing path. |
| Persist next-due timestamps | Remint on start/recovery | Persisting due times would change the Knots restart boundary and violate D-06 / Phase 135. |

**Installation:**

```bash
# No new production dependencies.
git submodule update --init --recursive
bash scripts/verify.sh
```

**Version verification:** Rust `1.94.1` (2026-03-25), Tokio `1.52.1`, getrandom `0.3.4` confirmed from Cargo manifests and local toolchain on 2026-08-15. [VERIFIED: rustc --version, Cargo.toml]

## Architecture Patterns

### Recommended Project Structure

```text
packages/open-bitcoin-network/src/peer/transaction_relay/
├── retry.rs                         # existing jitter + context; add cycle, cursor, inspect/prepare
└── fanout.rs                        # existing FIFO + PHASE104 bounds; no second queue

packages/open-bitcoin-node/src/network/
├── lifecycle_projection.rs          # change unbroadcast insert/clear + reconciliation expectation
├── runtime_authority/lifecycle.rs   # dispatch MaintenanceTick; TX-only TransportWritten on complete
├── relay_fanout.rs                  # enqueue prepared retry identities through existing path
├── relay_serving.rs                 # EligibleServe classification stays semantic
└── announcement_transport.rs        # add TX / tx-inv write kinds; stop requiring compact evidence

packages/open-bitcoin-rpc/src/bin/open_bitcoind/
├── checkpoint.rs                    # pattern to copy, not to couple
└── retry.rs                         # NEW: shutdown-aware timer, injected now/jitter, remint
```

Do not create `rebroadcast.rs` unless it is a thin alias that only calls `TxFanoutQueue` and the existing receipt path.

### Pattern 1: Command / Delta / Receipt

**What:** Shell sends injected facts; authority prepares bounded work; I/O happens outside the lock; a typed receipt mutates delivery state.
**When to use:** Every retry tick, first-hop INV drain, and GETDATA TX serve.
**Example:**

```rust
// Source: .planning/research/ARCHITECTURE.md command/delta/receipt
// Locked by 136-CONTEXT D-16 and Phase 134 D-11.
handle.maintenance_tick(RetryDecisionContext::new(now, jitter))?;
// prepare under lock -> release -> write PeerEmission -> CompletePeerEmission
```

`LifecycleCommand::Maintenance(LifecycleProjectionPlan)` already exists and is the expiry/pressure-style projector facade. Do not overload it with retry scheduling. Add a distinct handle method (name is discretion) that accepts `RetryDecisionContext` and returns owned drain work. [VERIFIED: packages/open-bitcoin-node/src/network/lifecycle_projection.rs, runtime_authority/lifecycle.rs]

### Pattern 2: Membership Insert Versus Tick Work

**What:** The 5,000-member cap is a set-size invariant. Inspect-N and prepare-M are per-tick work caps.
**When to use:** Every maintenance tick.
**Example:**

```rust
// Source: packages/open-bitcoin-network/src/peer/transaction_relay/fanout.rs
pub const PHASE104_MAX_TX_FANOUT_QUEUE_PER_PEER: usize = 1024;
pub const PHASE104_MAX_TX_FANOUT_DRAIN_PER_PEER: usize = 16;
```

Recommend documented constants `MAINTENANCE_INSPECT_BUDGET = 256` and `MAINTENANCE_PREPARE_BUDGET = 32`. Both are smaller than 5,000, and prepare 32 is larger than drain 16 so leftover-cursor tests can prove IBR-02 is not just the Phase 104 drain cap. [ASSUMED: exact numbers are discretion; planner may pick any documented pair that satisfies D-09..D-12]

### Pattern 3: FIFO Parent-Before-Child

**What:** Co-admitted `FinallyPresent` members are already recorded in topological `admitted` order. Enqueue that slice with `push_back`.
**When to use:** Package admission projection only. Do not walk unrelated ancestors.
**Example:**

```rust
// Source: packages/open-bitcoin-mempool/src/pool/package_admission/finalization.rs
for member in report.members() {
    let PackageMemberResult::FinallyPresent(result) = member else {
        continue; // AlreadyPresent parents are not re-admitted
    };
    builder.record_admitted(result.requested)?;
}
```

`prepare_fanout_projection` already iterates `facts.final_present()` and calls `enqueue_admission`. Keep that order. Independently admitted children must not re-enqueue an already-present parent (D-14). [VERIFIED: package_admission/finalization.rs, relay_fanout/lifecycle.rs]

### Anti-Patterns to Avoid

- **Walk `mempool.entries()` on a tick:** Violates IBR-01 and fingerprints as whole-mempool rebroadcast.
- **Clear on INV queue or INV write:** Violates D-01 and Knots `mempool_unbroadcast.py` intent.
- **Record `EligibleServe` as a membership-clearing retry-clear at classify time:** Violates D-03.
- **Force TX INV through current `PeerEmission::new`:** `compact_announce_evidence_reason` maps `Inv` to `CompactInventoryFallback` and rejects `Tx`. [VERIFIED: announcement_transport.rs, block_relay_evidence.rs]
- **Piggyback the retry timer on `receive_message_for_durable_serving` or `DurableSyncRuntime`:** Violates D-16/D-17. Today's `drain_relay_fanout` is invoked from the peer-admission receive path. [VERIFIED: action_translation.rs]
- **Re-insert retry-eligible `final_present` members on every projection:** Current `prepare_unbroadcast_projection` does this and would undo `TransportWritten`. [VERIFIED: lifecycle_projection/authority.rs]
- **Treat `expected_unbroadcast_members` as “all retry-eligible canonical members” after this phase:** That Phase 134 oracle becomes false once a still-present member is cleared by a TX write. [VERIFIED: lifecycle_projection/reconciliation.rs]
- **Persist next-due or cursor:** Violates D-06 and Phase 135 source-only snapshot.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| 10–15 minute jitter validation | Ad-hoc `u64` clamp | `RetryJitterSeconds::new` | Inclusive `0..=300` and `retry_jitter_out_of_range` already exist. [VERIFIED: retry.rs] |
| Per-peer announce/suppress/rate | New retry queue | `TxFanoutQueue` | PHASE104 queue 1024, drain 16, min interval 1s, txid/wtxid selection already encoded. [VERIFIED: fanout.rs] |
| Write acknowledgement | “Served” classify as done | `acknowledge_write` → `CompletePeerEmission` | Phase 134 affine write capability; stale receipts must not clear newer unbroadcast intent. [VERIFIED: 134-CONTEXT D-13, announcement_transport.rs] |
| Daemon wakeup | New scheduler crate or sync-loop hook | Copy `start_mempool_checkpoint_worker` | Injectible wait/now, shutdown channel, independent of IBD. [VERIFIED: checkpoint.rs] |
| Production randomness | Fixed public cadence | Existing `getrandom::fill` in the shell | Fallible; fingerprintable constant is a pitfall. [VERIFIED: STACK.md, node/rpc Cargo.toml] |
| Package wire / ancestor walk | `package` / `getpkgtxns` messages | Ordinary INV/TX + co-admitted FIFO | Not in the pinned tree; deferred by D-15. [VERIFIED: FEATURES.md, 136-CONTEXT] |

**Key insight:** The missing work is wiring and bounds, not a new broadcast engine. Custom retry transport would recreate the v2.1 split-authority failure.

## Common Pitfalls

### Pitfall 1: Re-inserting Cleared Still-Present Members

**What goes wrong:** A successful TX write clears unbroadcast, then a later child package or empty-looking maintenance projection puts the parent back because it is still local + requested + present.
**Why it happens:** `prepare_unbroadcast_projection` inserts every `final_present` member with `is_retry_eligible(true)` before applying `retry_clears`. [VERIFIED: lifecycle_projection/authority.rs]
**How to avoid:** Insert only newly admitted retry-eligible members that are not already cleared. Apply `retry_clears` as the sole still-present exit, and `teardown_order` / `LifecycleRemoval` as the absence exit. Never derive membership from metadata alone after the first insert.
**Warning signs:** A test that serves a parent, then admits a child, and finds the parent unbroadcast again.

### Pitfall 2: Reconciliation Still Equals Retry-Eligible

**What goes wrong:** After `TransportWritten`, production unbroadcast is a subset of retry-eligible canonical members. The current oracle reports a mismatch and looks like a projection bug.
**Why it happens:** `expected_unbroadcast_members` filters canonical identities by `is_retry_eligible(true)`. [VERIFIED: reconciliation.rs]
**How to avoid:** Change expected unbroadcast to the authoritative set itself (audit inserts/clears), or to “retry-eligible minus members with an applying `TransportWritten`/`LifecycleRemoval` fact.” Do not keep the Phase 134 equality.
**Warning signs:** Reconciliation tests that construct a local requested member and assert it must be in unbroadcast with no serve/write history.

### Pitfall 3: Compact-Coupled PeerEmission

**What goes wrong:** TX responses cannot become receipts; TX INV increments compact inventory-fallback counts; operators later read compact evidence as transaction retry success.
**Why it happens:** `PeerEmission::new` requires both `PeerEmissionWriteKind::for_message` (no `Tx` arm) and `compact_announce_evidence_reason` (`Inv` → `CompactInventoryFallback`, `Tx` → `None`). [VERIFIED: announcement_transport.rs:91-92, block_relay_evidence.rs:345-353]
**How to avoid:** Add distinct write kinds for transaction inventory and transaction response. Compact evidence functions must stay compact-only. `TransportWritten` filters `write_kind == Transaction` (or equivalent) and ignores INV.
**Warning signs:** `compact_inventory_fallback_count` moves when a local transaction is announced.

### Pitfall 4: Immediate GETDATA TX Has No Receipt

**What goes wrong:** Serving classifies `Served` and encodes `WireNetworkMessage::Tx` as `ManagedInboundResponsePlanItem::Immediate` with no `PeerEmission`. Unbroadcast never clears, or a planner is tempted to clear at classify time.
**Why it happens:** `inventory.rs` / `gate_inventory_for_durable_serving` emit Immediate TX; `InboundWireResponsePlan::resolve` encodes them with `None` capability. [VERIFIED: inventory.rs:124-138, context.rs:124-126]
**How to avoid:** Promote eligible TX serves to `PrepareRelay` + `PeerEmission` with the TX write kind. Clear only after `acknowledge_write` of that emission, and only when completion is `Applied` (fresh). Encode/write/abort failure leaves the member retry-eligible (D-02). `EligibleServe` may be recorded as non-clearing evidence later in Phase 137, not as a `retry_clears` row now.
**Warning signs:** Tests that assert unbroadcast empty after `classify_request` and before a write.

### Pitfall 5: Receive-Loop Drain Pretends to Be Immediate First Hop

**What goes wrong:** Local `sendrawtransaction` enqueues fanout but `drain_relay_fanout` today runs from the peer-admission receive path. An idle node with no inbound messages never emits the first INV until the retry timer, collapsing D-07 into D-05.
**Why it happens:** `action_translation.rs` drains after `ReceivedTransaction` singleton admission. Local submit records the queue and `rebroadcast_deferred` but does not drain to outboxes. [VERIFIED: action_translation.rs:269, admission_bridge.rs:186-206, relay_fanout.rs:355-368]
**How to avoid:** Keep admission-time enqueue as the first hop. After local/package admission *and* after a maintenance tick, the shell drains through the generalized announcement outbox, not through the inbound receive loop. The timer must not be the only drain.
**Warning signs:** First INV appears only after `mockscheduler`-equivalent 10–15 minutes in a no-receive fixture.

### Pitfall 6: Unbounded Tick After Idle Recovery

**What goes wrong:** A recovered 5,000-member set is walked and prepared in one lock hold, starving peer/RPC work.
**Why it happens:** Knots walks the whole unbroadcast set each cycle with no inspect/prepare split. Open Bitcoin’s set cap is 5,000. [VERIFIED: net_processing.cpp:1562-1574, PITFALLS.md Pitfall 6 / performance trap]
**How to avoid:** Separate inspect-N and prepare-M. Spill unattempted leftovers. Do not count queue-cap/rate-limit/suppress as attempts for unprepared identities (D-11). Advance a deterministic `BTreeSet` cursor.
**Warning signs:** One tick’s authority hold scales with `unbroadcast_members.len()`.

### Pitfall 7: Stale TX Receipt Clears Newer Intent

**What goes wrong:** A TX write started under an older generation completes after replacement/re-admission and clears the new unbroadcast marker.
**Why it happens:** Phase 134 already forbids this (`AchievedButStale` must not clear newer unbroadcast intent). [CITED: 134-CONTEXT D-13]
**How to avoid:** Apply `TransportWritten` only on `EffectCompletion::Applied` for the current authority epoch, lifecycle generation, and peer session. `AlreadyApplied` / `AchievedButStale` must be no-ops for membership.
**Warning signs:** Replay or stale-receipt tests that empty unbroadcast after a newer local admission.

## Code Examples

Verified patterns from this repository:

### Injected retry facts (already shipped)

```rust
// Source: packages/open-bitcoin-network/src/peer/transaction_relay/retry.rs
pub struct RetryDecisionContext {
    pub observed_at_unix_seconds: i64,
    pub jitter: RetryJitterSeconds,
}

impl RetryJitterSeconds {
    pub const fn new(seconds: u64) -> Result<Self, RetryJitterRangeError> {
        if seconds > 300 {
            return Err(RetryJitterRangeError);
        }
        Ok(Self(seconds))
    }
}
```

Cycle length in the shell is `600 + jitter.seconds()` seconds. Pure policy must not call `SystemTime` or `getrandom`. [VERIFIED: retry.rs; CITED: 136-CONTEXT D-08]

### Retry eligibility (enter set, not current membership)

```rust
// Source: packages/open-bitcoin-mempool/src/context.rs
pub const fn is_retry_eligible(self, is_current_member: bool) -> bool {
    is_current_member
        && matches!(self.origin, MempoolOrigin::Local)
        && matches!(self.relay_intent, RelayIntent::Requested)
}
```

Use this only as the insert gate for newly admitted members. After `TransportWritten`, a member can remain eligible by metadata and still be absent from unbroadcast. [VERIFIED: context.rs]

### Current unbroadcast projection (must change)

```rust
// Source: packages/open-bitcoin-node/src/network/lifecycle_projection/authority.rs
for member in facts.final_present() {
    if member.metadata.is_retry_eligible(true) {
        replacement.insert(member.member);
    }
}
for member in facts.teardown_order() {
    replacement.remove(member);
}
for clear in &facts.delta().retry_clears {
    replacement.remove(&clear.member);
}
```

Replace the insert loop with “newly admitted and retry-eligible only.” Keep teardown and `retry_clears` removals. [VERIFIED: authority.rs]

### Knots cycle and acknowledgement

```cpp
// Source: packages/bitcoin-knots/src/net_processing.cpp
void PeerManagerImpl::ReattemptInitialBroadcast(CScheduler& scheduler)
{
    std::set<uint256> unbroadcast_txids = m_mempool.GetUnbroadcastTxs();
    for (const auto& txid : unbroadcast_txids) {
        CTransactionRef tx = m_mempool.get(txid);
        if (tx != nullptr) {
            RelayTransaction(txid, tx->GetWitnessHash());
        } else {
            m_mempool.RemoveUnbroadcastTx(txid, true);
        }
    }
    const auto delta = 10min + FastRandomContext().randrange<std::chrono::milliseconds>(5min);
    scheduler.scheduleFromNow([&] { ReattemptInitialBroadcast(scheduler); }, delta);
}

// GETDATA TX push (~2457)
MakeAndPushMessage(pfrom, NetMsgType::TX, maybe_with_witness(*tx));
m_mempool.RemoveUnbroadcastTx(tx->GetHash());
```

Open Bitcoin matches the cadence and the “not INV” rule, but applies the clear after a successful TX write rather than at send-buffer push. That is the locked intentional difference (D-02). [VERIFIED: net_processing.cpp:1562-1579, 2452-2457]

### Shutdown-aware shell timer template

```rust
// Source: packages/open-bitcoin-rpc/src/bin/open_bitcoind/checkpoint.rs
match wait(MEMPOOL_CHECKPOINT_INTERVAL) {
    CheckpointWait::Elapsed => { /* drive periodic work */ }
    CheckpointWait::Shutdown => return drive(CheckpointDrive::Shutdown),
}
```

Copy this injectible `wait` / `now` shape. Sample jitter with `getrandom` in the production `Now`/jitter closure; tests supply fixed `now` and `RetryJitterSeconds`. Do not start this worker from `DurableSyncRuntime`. [VERIFIED: checkpoint.rs, open-bitcoind.rs:88-96]

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| `rebroadcast_deferred` evidence only, no timer | Scheduled initial-broadcast retry through existing fanout | Phase 136 (this phase) | Phase 104 D-13..D-15 deferred the timer; this phase replaces that live story |
| Clear unbroadcast when INV is queued | Clear only on TX write or lifecycle removal | Locked 2026-08-15 | Stronger than Knots send-buffer push; never weaker than “not INV” |
| Maintenance only on inbound `ReceivedTransaction` | Receive-independent shell tick | Phase 136 | Idle nodes retry; first hop must still drain without waiting for receive |
| Expected unbroadcast = all retry-eligible members | Authoritative set after insert/clear facts | Phase 136 | Required once `TransportWritten` exists |
| `PeerEmission` compact-only | Generalized TX / tx-inv write kinds | Phase 136 | Needed for IBR-03/IBR-04 without compact evidence pollution |

**Deprecated/outdated:**

- `rebroadcast_deferred` as the live scheduling claim: keep the label only when it remains true (relay disabled, or member not yet due). Do not imply Phase 104 added a timer. [CITED: 136-CONTEXT specifics; 104-CONTEXT D-13]
- Research sketch files `rebroadcast.rs` / `relay_rebroadcast.rs` as a second announcer: reuse names only as thin wrappers. [CITED: 136-CONTEXT specifics]
- Phase 134 reconciliation equality “unbroadcast == retry-eligible canonical subset.” [VERIFIED: reconciliation.rs; 134 STATE note]

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | Inspect budget 256 and prepare budget 32 are the right documented constants | Pattern 2 / IBR-02 | Too small starves retry; too large holds the authority lock. Planner may pick any pair that is documented, fake-clock-assertable, `< 5000`, and independent of PHASE104 caps. |
| A2 | First-hop INV should drain through a generalized announcement outbox after admission, not only on receive | Pitfall 5 | If a hidden immediate-drain path already exists for local submit, Plan 05/06 should reuse it instead of adding a second drain. None was found in this research. |
| A3 | `EligibleServe` stays a documented semantic label and is not written into `retry_clears` in this phase | IBR-04 / D-03 | If Phase 137 later needs an `eligible_serve` counter, it can project from serve classification without a membership-clearing delta. |

**If this table is empty:** All claims in this research were verified or cited — no user confirmation needed.

Discretion items A1–A3 do not reopen locked decisions. They only fix numbers and module seams.

## Open Questions

1. **Exact inspect-N / prepare-M**
   - What we know: Must be documented, fake-clock-assertable, `< 5000`, and not silently equal to PHASE104 drain/queue caps. [CITED: 136-CONTEXT discretion]
   - What's unclear: Product preference among valid pairs.
   - Recommendation: Ship `256` / `32` unless planning review picks another pair in the same commit that introduces the constants.

2. **Timer home**
   - What we know: Must be shutdown-aware, not a second policy authority, not a public-default relay loop, not `DurableSyncRuntime`. [CITED: D-16..D-17, discretion]
   - What's unclear: `open-bitcoind` versus a node adapter module.
   - Recommendation: `packages/open-bitcoin-rpc/src/bin/open_bitcoind/retry.rs`, mirroring `checkpoint.rs`, started from `open-bitcoind.rs` next to `start_mempool_checkpoint_worker`.

3. **Hidden first-hop drain**
   - What we know: Local submit enqueues; peer-receive drains; announcement outboxes today are block-oriented. [VERIFIED: admission_bridge.rs, action_translation.rs, sync.rs]
   - What's unclear: Whether an unreviewed inbound announcement path already drains TX INV after RPC submit.
   - Recommendation: Plan 01 should include a failing test: local accept, no inbound messages, fake clock unchanged → first INV is prepared/emitted without waiting 10 minutes. If that test already passes, reuse the existing drain; if not, add the outbox drain in the same phase.

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| Rust / Cargo | All first-party work | ✓ | 1.94.1 | — |
| Tokio (`time`, `test-util`) | Daemon shell + hermetic timer tests | ✓ | 1.52.1 | — |
| `getrandom` | Production jitter | ✓ | 0.3.4 | Typed degraded scheduling outcome; never a silent constant |
| Bun | Repo scripts / verify helpers | ✓ | 1.3.14 | — |
| Bazelisk | Repo verify Bazel smoke | ✓ | 1.28.1 / Bazel 8.6.0 | — |
| Pinned Knots tree | Parity anchors | ✓ | `29.3.knots20260210` | `git submodule update --init --recursive` |
| Public network / wall clock in default verify | None | n/a | — | Forbidden (D-18) |

**Missing dependencies with no fallback:**
- None.

**Missing dependencies with fallback:**
- None. Jitter entropy failure is a typed runtime outcome, not a missing install.

Step 2.6 external services (Postgres, Redis, Docker) are not required.

## Security Domain

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|-----------------|
| V2 Authentication | no | Unchanged RPC/auth surface (Phase 137) |
| V3 Session Management | no | Existing peer-session generation on receipts |
| V4 Access Control | no | Existing relay activation / peer eligibility |
| V5 Input Validation | yes | `RetryJitterSeconds::new` (`0..=300`); inspect/prepare newtypes; cursor stays inside `unbroadcast_members` |
| V6 Cryptography | yes | Shell `getrandom` for jitter; do not hand-roll a CSPRNG or use a fixed public cadence |

### Known Threat Patterns for initial-broadcast retry

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Whole-mempool periodic announce | Information disclosure / fingerprinting | Walk only the 5,000-cap local unbroadcast set (IBR-01) |
| Fixed 10-minute public cadence | Information disclosure | Fresh `0..=300` jitter every cycle (D-05, D-08) |
| Unbounded tick after idle recovery | Denial of service | Inspect-N / prepare-M plus leftover cursor (D-09..D-12) |
| INV treated as acknowledgement | Tampering / integrity | Clear only on fresh TX write or lifecycle removal (D-01, D-02) |
| Stale TX receipt clears newer local intent | Tampering | `Applied`-only membership mutation (134-CONTEXT D-13) |
| Compact evidence polluted by TX INV | Information disclosure / integrity | Distinct TX write kinds; compact reason stays compact-only |
| Public/default relay implied by retry | Elevation / claim creep | Keep activation default-off; no guaranteed-propagation wording |

## Recommended Plan Split

Planner should create about six plans. Do not reopen D-01..D-19.

1. **Pure retry cycle, cursor, and budgets** (`open-bitcoin-network` `retry.rs`) — process-global cycle length, inspect-N / prepare-M newtypes, deterministic `BTreeSet` cursor, leftover = unattempted. Covers IBR-01 walk rule and IBR-02 policy. No timer, no I/O.
2. **Unbroadcast insert/clear + reconciliation** — newly admitted insert only; `TransportWritten` / `LifecycleRemoval` remove; stop re-inserting still-present cleared members; rewrite `expected_unbroadcast_members`. Covers IBR-04 projection. Do not emit `EligibleServe` retry-clears.
3. **TX-specific PeerEmission write kinds** — add transaction-inventory and transaction-response kinds; keep compact evidence compact-only; INV write never clears. Prerequisite for IBR-03/IBR-04 transport.
4. **GETDATA TX receipt path** — promote Immediate TX serves to `PrepareRelay` / `acknowledge_write` / `CompletePeerEmission`; apply `TransportWritten` only on fresh `Tx` success. Covers IBR-04 receipt. Failed encode/write/abort leaves membership.
5. **Package FIFO + retry enqueue on the Phase 104 path** — prove co-admitted parent-before-child `push_back`; already-present parents not re-announced; retry prepare calls `enqueue_admission` then existing drain/rate/identity/activation. Covers PPKG-04 and IBR-03. No parallel announcer.
6. **MaintenanceTick + `open-bitcoind` timer + first-hop drain** — handle method consumes `RetryDecisionContext`; remint on start/recovery/completed tick; checkpoint-style shutdown-aware worker; drain first-hop and retry INV through generalized outboxes without receive-loop or `DurableSyncRuntime`. Covers IBR-02 wakeup, D-07, D-16..D-19. Narrow parity breadcrumbs and honest `rebroadcast_deferred` wording; leave broad operator presentation to Phase 137.

## Sources

### Primary (HIGH confidence)

- `packages/open-bitcoin-network/src/peer/transaction_relay/retry.rs` — `RetryJitterSeconds`, `RetryDecisionContext`
- `packages/open-bitcoin-network/src/peer/transaction_relay/fanout.rs` — PHASE104 bounds, FIFO, `RebroadcastDeferred`
- `packages/open-bitcoin-mempool/src/context.rs` — `is_retry_eligible`
- `packages/open-bitcoin-mempool/src/pool/lifecycle.rs` — `MempoolRetryClearCause` precedence
- `packages/open-bitcoin-mempool/src/pool/package_admission/finalization.rs` — `FinallyPresent` vs `AlreadyPresent` admitted set
- `packages/open-bitcoin-node/src/network/lifecycle_projection.rs` — `MAX_UNBROADCAST_MEMBERS = 5_000`, `LifecycleCommand::Maintenance`
- `packages/open-bitcoin-node/src/network/lifecycle_projection/authority.rs` — current unbroadcast insert/re-insert
- `packages/open-bitcoin-node/src/network/lifecycle_projection/reconciliation.rs` — retry-eligible expected set
- `packages/open-bitcoin-node/src/network/runtime_authority/lifecycle.rs` — Maintenance and `CompletePeerEmission` dispatch
- `packages/open-bitcoin-node/src/network/relay_fanout.rs` / `relay_fanout/lifecycle.rs` — enqueue/drain/first-hop
- `packages/open-bitcoin-node/src/network/relay_serving.rs` / `inventory.rs` — GETDATA classify + Immediate TX
- `packages/open-bitcoin-node/src/network/announcement_transport.rs` — compact-coupled `PeerEmission`
- `packages/open-bitcoin-node/src/network/action_translation.rs` — receive-path `drain_relay_fanout`
- `packages/open-bitcoin-rpc/src/bin/open_bitcoind/checkpoint.rs` and `open-bitcoind.rs` — timer template and startup
- `packages/bitcoin-knots/src/net_processing.cpp` — `ReattemptInitialBroadcast`, `StartScheduledTasks`, GETDATA `RemoveUnbroadcastTx`
- `packages/bitcoin-knots/src/node/transaction.cpp` — local `AddUnbroadcastTx` + immediate `RelayTransaction`
- `packages/bitcoin-knots/src/txmempool.cpp` — `RemoveUnbroadcastTx`
- `packages/bitcoin-knots/test/functional/mempool_unbroadcast.py` — persist, remint, GETDATA clear, no re-add of already-present
- `.planning/research/ARCHITECTURE.md`, `FEATURES.md`, `STACK.md`, `PITFALLS.md`, `SUMMARY.md`
- `.planning/phases/104-CONTEXT.md`, `130-CONTEXT.md`, `134-CONTEXT.md`, `135-CONTEXT.md`, `136-CONTEXT.md`

### Secondary (MEDIUM confidence)

- Architecture research (2026-07-22) recommended `rebroadcast.rs` / `relay_rebroadcast.rs` file names. Treat as historical sketch, not a second announcer. [CITED: .planning/research/ARCHITECTURE.md]
- Architecture research said Knots clears on eligible GETDATA serve; this phase locks the stronger successful-write receipt. [CITED: ARCHITECTURE.md; 136-CONTEXT D-02]

### Tertiary (LOW confidence)

- None. Exact inspect/prepare numbers are discretion, logged as A1, not an unverified web claim.

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH — pinned toolchain and existing crates; no new dependency.
- Architecture: HIGH — insertion points, Knots anchors, and three breaking seams were read in this session.
- Pitfalls: HIGH — current re-insert, reconciliation, compact `PeerEmission`, Immediate TX, and receive-path drain are verified in source.

**Research date:** 2026-08-15
**Valid until:** 2026-09-14 (30 days; internal seams, not a fast-moving ecosystem)
