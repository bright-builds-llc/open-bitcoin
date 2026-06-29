# Phase 101: Transaction Inventory Identity and Download Scheduling - Research

**Researched:** 2026-06-29 [VERIFIED: system current_date]
**Domain:** Rust P2P transaction relay identity, pure transaction download scheduling, and Bitcoin Knots parity [VERIFIED: .planning/ROADMAP.md; VERIFIED: .planning/phases/101-transaction-inventory-identity-and-download-scheduling/101-CONTEXT.md]
**Confidence:** HIGH for repo seams and Knots anchors; MEDIUM for exact planner-chosen scheduler type names because the phase context leaves names to the planner [VERIFIED: repo grep; VERIFIED: packages/bitcoin-knots/src/node/txdownloadman.h; ASSUMED]

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
## Implementation Decisions

### Inventory Identity And Negotiation

- **D-01:** Introduce an Open Bitcoin-owned typed transaction relay identity, for example `TxRelayId`, that distinguishes `Txid(Txid)` from `Wtxid(Wtxid)`. Raw `Hash32` plus `InventoryType` should be parsed at the wire boundary and converted into typed identity before request scheduling logic sees it.
- **D-02:** Preserve BIP339-style negotiation behavior already represented by `remote_wtxidrelay`: txid-only peers announce and request `InventoryType::Transaction`; wtxidrelay peers announce and request `InventoryType::WitnessTransaction`.
- **D-03:** Treat inventory identity mismatches as suppression decisions, not best-effort fallback. `MSG_TX` from a wtxidrelay peer and `MSG_WTX` from a txid-only peer should not create stale request state.
- **D-04:** Keep block inventory behavior separate from transaction relay scheduling. Existing header/block request paths may remain in `PeerManager`, but Phase 101 transaction logic should not blur block and transaction request accounting.

### Per-Peer Request State

- **D-05:** Replace the current bare `requested_txids` and `requested_wtxids` sets with richer pure transaction request state that records identity, announcing peer, requested peer, timestamps, expiry, and reason labels needed for duplicate suppression, `notfound`, timeout, fallback, and disconnect cleanup.
- **D-06:** Track already-have and recent-reject suppression as explicit scheduler inputs. The pure scheduler should receive local mempool/known/recent-reject facts as data and must not call mempool, storage, socket, or runtime APIs directly.
- **D-07:** Duplicate announcements for the same typed identity should be retained only when they can help fallback after timeout, `notfound`, or disconnect; they must not emit redundant `getdata` while an equivalent request is in flight.
- **D-08:** Disconnect cleanup must remove that peer's announcements and in-flight requests, then emit fallback actions when another eligible announcing peer exists.

### Download Scheduling

- **D-09:** Add a deterministic scheduler API that takes a fake-clock timestamp and emits typed request actions. Tests should be able to advance time without sleeping or touching wall-clock APIs.
- **D-10:** Keep in-flight request caps bounded and aligned with prior Phase 94 request-governance constraints. The scheduler should expose low-cardinality cap/suppression reasons instead of dynamic peer, txid, wtxid, or raw transaction labels.
- **D-11:** Model Knots-inspired scheduling delays explicitly: non-preferred peer delay, txid delay when wtxid peers exist, overloaded-peer delay, and getdata retry/expiry interval. The planner may choose exact constant names and simplified values when the tests preserve the observable behavior claimed by Phase 101.
- **D-12:** `notfound` for a requested transaction should complete or clear the matching in-flight request immediately and make the identity eligible for fallback to another announcing peer when one exists.
- **D-13:** Timeout should expire stale in-flight requests, clear the requested peer state, and choose a fallback peer if available without leaving stale request state behind.

### Received Transaction Cleanup

- **D-14:** On `tx`, derive both txid and wtxid once, mark both identities already-have, and clear any matching in-flight txid or wtxid request state for that peer.
- **D-15:** If a received transaction does not match the requested identity for that peer, emit a typed mismatch/suppression result and clean up only the state that is safe to clear. Do not treat mismatched data as satisfying an unrelated request.
- **D-16:** Phase 101 may continue returning a received transaction action for the managed network to submit later, but mempool admission semantics remain Phase 102. The new boundary should make the later admission bridge consume a stable typed transaction response instead of inspecting peer internals.

### Typed Actions And Evidence

- **D-17:** Emit typed pure actions for `request_getdata`, `suppress_duplicate`, `suppress_already_have`, `suppress_recent_reject`, `suppress_identity_mismatch`, `fallback_request`, `request_expired`, `notfound_cleanup`, `received_tx_cleanup`, and `peer_cleanup`.
- **D-18:** Adapter code may translate request actions into `WireNetworkMessage::GetData`, but socket I/O and managed runtime mutation must stay outside the scheduler.
- **D-19:** Evidence labels must be fixed and low-cardinality. Do not expose raw transaction hex, txids, wtxids, peer endpoints, permission strings, class names, credentials, or dynamic labels in planning, status, support, or log surfaces.

### Tests, Parity, And Guardrails

- **D-20:** Unit tests must cover txid and wtxid paths separately, duplicate announcements, identity mismatches, already-have suppression, recent-reject suppression, in-flight cap suppression, timeout fallback, `notfound` fallback, disconnect cleanup, and received-transaction cleanup.
- **D-21:** Use deterministic fake-clock tests for expiry and fallback. Do not add public-network relay checks, sleeps, or service-manager behavior to `bash scripts/verify.sh`.
- **D-22:** Add parity breadcrumbs for new first-party Rust source or test files under `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests`, citing concrete Knots anchors unless `none` is the only defensible breadcrumb.
- **D-23:** If docs or parity roots are updated, preserve the v2.0 no-claim boundary: transaction relay remains bounded and explicit, while compact block relay, package relay, bloom/filter serving, public relay defaults, public-network CI, production full-node readiness, and production-funds wallet use stay deferred.

### the agent's Discretion

The planner may choose the exact module split, type names, scheduler constants, and whether to keep the scheduler under `peer/inventory_state.rs` or extract a sibling transaction-relay module, as long as the result stays pure, testable, bounded, parity-auditable, and compatible with existing `PeerManager` and `ManagedPeerNetwork` integration points.

### Deferred Ideas (OUT OF SCOPE)
## Deferred Ideas

Orphan staging, parent request behavior, transaction admission outcome contracts, standardness/fee/RBF/ancestor policy, mempool pressure/trimming, mempool persistence, block connect/disconnect mempool lifecycle, relay serving/fanout, rebroadcast, RPC/operator/support evidence, release-boundary closeout, compact block relay, package relay, bloom/filter serving, public relay defaults, public-network relay CI, production full-node readiness, and production-funds wallet use remain outside Phase 101.
</user_constraints>

## Project Constraints (from AGENTS.md)

- Follow the repo-local `AGENTS.md`, then `AGENTS.bright-builds.md`, `standards-overrides.md`, and task-relevant standards pages before plan, review, implementation, or audit work. [VERIFIED: AGENTS.md; VERIFIED: AGENTS.bright-builds.md; VERIFIED: standards/index.md]
- Preserve externally observable Bitcoin Knots `29.3.knots20260210` behavior for in-scope surfaces and keep parity evidence auditable through `docs/parity/`. [VERIFIED: AGENTS.md]
- Keep pure Bitcoin domain behavior in functional-core crates and isolate filesystem, process, network, terminal, RPC, service-manager, and durable-storage effects in shell adapters. [VERIFIED: AGENTS.md; VERIFIED: standards/core/architecture.md]
- Do not use existing Rust Bitcoin libraries in the production path; the project owns its domain model and implementation surface. [VERIFIED: AGENTS.md]
- Use Rust `1.94.1` and Rust 2024 edition as pinned by `rust-toolchain.toml` and `packages/Cargo.toml`. [VERIFIED: AGENTS.md; VERIFIED: rust-toolchain.toml; VERIFIED: packages/Cargo.toml]
- Use `bash scripts/verify.sh` as the repo-native verification contract, including the Bazel smoke build. [VERIFIED: AGENTS.md; VERIFIED: scripts/verify.sh]
- New or touched first-party Rust files under `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests` need parity breadcrumbs via `docs/parity/source-breadcrumbs.json` and `scripts/check-parity-breadcrumbs.ts`. [VERIFIED: AGENTS.md; VERIFIED: scripts/check-parity-breadcrumbs.ts]
- Use `foo.rs` plus `foo/` rather than `foo/mod.rs` for new or touched multi-file Rust modules. [VERIFIED: standards/languages/rust.md]
- Prefer early returns, `let...else` for guard-style extraction, `maybe_` names for optional internals, and newtypes/enums for invariants. [VERIFIED: standards/core/code-shape.md; VERIFIED: standards/languages/rust.md]
- Unit tests for pure business logic are required, should cover one concern per test, and should clearly delineate Arrange, Act, and Assert unless trivially obvious. [VERIFIED: standards/core/testing.md]
- No project skill directories were present under `.claude/skills` or `.agents/skills`; `AGENTS.md` also states no project skills were found. [VERIFIED: find .claude .agents -maxdepth 3 -type f -name SKILL.md; VERIFIED: AGENTS.md]

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| INV-01 | Node handles transaction `inv`, `getdata`, `tx`, and `notfound` messages with typed txid and wtxid identity. [VERIFIED: .planning/REQUIREMENTS.md] | Use `TxRelayId::{Txid,Wtxid}` at the wire/scheduler boundary; Open Bitcoin already has `Txid`, `Wtxid`, `InventoryType::{Transaction,WitnessTransaction}`, and parsed `WireNetworkMessage::{Inv,GetData,NotFound,Tx}`. [VERIFIED: packages/open-bitcoin-primitives/src/hash.rs; VERIFIED: packages/open-bitcoin-primitives/src/network.rs; VERIFIED: packages/open-bitcoin-network/src/message.rs] |
| INV-02 | Node tracks per-peer txid/wtxid negotiation, already-have state, request state, and received-transaction cleanup deterministically. [VERIFIED: .planning/REQUIREMENTS.md] | `PeerState.remote_wtxidrelay`, `known_txids`, `known_wtxids`, and `handle_transaction` already exist, but request state is still bare `requested_txids` and `requested_wtxids`; replace those with scheduler state that accepts `now` as input. [VERIFIED: packages/open-bitcoin-network/src/peer.rs; VERIFIED: packages/open-bitcoin-network/src/peer/inventory_state.rs] |
| INV-03 | Node handles duplicate announcements, identity mismatches, `notfound`, timeout, and disconnect cleanup without stale request state. [VERIFIED: .planning/REQUIREMENTS.md] | Knots `TxRequestTracker` tracks candidate, requested, and completed announcements per peer/txhash and explicitly handles response, timeout, and disconnect cleanup; Open Bitcoin currently clears matching bare sets on `notfound` and `tx` only. [VERIFIED: packages/bitcoin-knots/src/txrequest.h; VERIFIED: packages/bitcoin-knots/src/txrequest.cpp; VERIFIED: packages/open-bitcoin-network/src/peer/inventory_state.rs] |
| INV-04 | Relay decisions emit stable typed actions for announcements, requests, suppressions, fallbacks, and peer cleanup. [VERIFIED: .planning/REQUIREMENTS.md] | Existing `PeerAction` already carries typed peer-manager decisions, so Phase 101 should add typed transaction relay actions rather than returning booleans or raw strings. [VERIFIED: packages/open-bitcoin-network/src/peer.rs] |
| DL-01 | Node schedules transaction downloads with bounded in-flight request caps, expiry, peer fallback, and retry evidence. [VERIFIED: .planning/REQUIREMENTS.md] | Knots uses `MAX_PEER_TX_REQUEST_IN_FLIGHT`, `MAX_PEER_TX_ANNOUNCEMENTS`, `GETDATA_TX_INTERVAL`, delayed announcements, request expiry, and fallback in `GetRequestsToSend`; Open Bitcoin has Phase 94 request caps available through `ResourceGovernancePolicy`. [VERIFIED: packages/bitcoin-knots/src/node/txdownloadman.h; VERIFIED: packages/bitcoin-knots/src/node/txdownloadman_impl.cpp; VERIFIED: packages/open-bitcoin-network/src/resource.rs] |
| DL-02 | Node suppresses redundant transaction requests through already-have, recent-reject, in-flight, and mempool-state checks. [VERIFIED: .planning/REQUIREMENTS.md] | Phase 101 should pass already-have/recent-reject/mempool facts as scheduler input; Knots checks `AlreadyHaveTx` and recent reject filters before requesting, while Open Bitcoin currently checks only known txid/wtxid sets in `handle_inventory`. [VERIFIED: .planning/phases/101-transaction-inventory-identity-and-download-scheduling/101-CONTEXT.md; VERIFIED: packages/bitcoin-knots/src/node/txdownloadman_impl.cpp; VERIFIED: packages/open-bitcoin-network/src/peer/inventory_state.rs] |
</phase_requirements>

## Summary

Phase 101 should add a first-party pure transaction relay scheduler under `open-bitcoin-network`, keeping raw `InventoryVector` parsing at the message boundary and exposing typed txid/wtxid decisions as `PeerAction`-compatible actions. [VERIFIED: .planning/phases/101-transaction-inventory-identity-and-download-scheduling/101-CONTEXT.md; VERIFIED: packages/open-bitcoin-network/src/peer/inventory_state.rs; VERIFIED: packages/open-bitcoin-network/src/peer.rs]

The current Open Bitcoin seam is narrow: `PeerState` stores `remote_wtxidrelay`, `requested_txids`, and `requested_wtxids`; `handle_inventory` immediately emits `GetData`; `NotFound` just removes hashes from those sets; and `handle_transaction` derives txid/wtxid and marks both known before returning `PeerAction::ReceivedTransaction`. [VERIFIED: packages/open-bitcoin-network/src/peer.rs; VERIFIED: packages/open-bitcoin-network/src/peer/inventory_state.rs]

The pinned Knots baseline supports a stronger design: `TxDownloadManager` and `TxRequestTracker` track peer connection info, txid/wtxid identity, candidate/requested/completed announcement states, delay windows, in-flight expiry, `notfound` response cleanup, fallback, and disconnect cleanup. [VERIFIED: packages/bitcoin-knots/src/node/txdownloadman.h; VERIFIED: packages/bitcoin-knots/src/node/txdownloadman_impl.cpp; VERIFIED: packages/bitcoin-knots/src/txrequest.h; VERIFIED: packages/bitcoin-knots/src/txrequest.cpp]

**Primary recommendation:** Build a small `TxRelayId` plus `TxDownloadScheduler` state machine in `open-bitcoin-network`, integrate it from `PeerManager::handle_inventory`, `NotFound`, `Tx`, timeout polling, and peer removal, and translate only final request actions into `WireNetworkMessage::GetData`. [VERIFIED: .planning/phases/101-transaction-inventory-identity-and-download-scheduling/101-CONTEXT.md; VERIFIED: packages/open-bitcoin-network/src/peer.rs; VERIFIED: packages/open-bitcoin-network/src/peer/inventory_state.rs; VERIFIED: packages/bitcoin-knots/src/txrequest.h]

## Standard Stack

### Core

| Library / Module | Version | Purpose | Why Standard |
|------------------|---------|---------|--------------|
| Rust toolchain | 1.94.1 | Compile and test first-party Rust crates. [VERIFIED: rustc --version; VERIFIED: rust-toolchain.toml] | The repo pins Rust 1.94.1 and Rust 2024 edition. [VERIFIED: rust-toolchain.toml; VERIFIED: packages/Cargo.toml] |
| `open-bitcoin-network` | 0.1.0 | Own pure peer decisions, `PeerManager`, `PeerAction`, message dispatch, resource caps, and transaction inventory seams. [VERIFIED: cargo metadata --manifest-path packages/Cargo.toml --no-deps; VERIFIED: packages/open-bitcoin-network/src/peer.rs] | Phase 101 verification is pure `open-bitcoin-network` tests and scheduler decisions must not perform socket I/O. [VERIFIED: .planning/ROADMAP.md; VERIFIED: .planning/phases/101-transaction-inventory-identity-and-download-scheduling/101-CONTEXT.md] |
| `open-bitcoin-primitives` | 0.1.0 | Provides `Hash32`, `Txid`, `Wtxid`, `InventoryType`, and `InventoryVector`. [VERIFIED: cargo metadata --manifest-path packages/Cargo.toml --no-deps; VERIFIED: packages/open-bitcoin-primitives/src/hash.rs; VERIFIED: packages/open-bitcoin-primitives/src/network.rs] | Existing semantic hash wrappers prevent txid/wtxid confusion without external Bitcoin crates. [VERIFIED: packages/open-bitcoin-primitives/src/hash.rs; VERIFIED: AGENTS.md] |
| `open-bitcoin-codec` | 0.1.0 | Parses and encodes network inventory vectors. [VERIFIED: cargo metadata --manifest-path packages/Cargo.toml --no-deps; VERIFIED: packages/open-bitcoin-codec/src/network.rs] | Raw type tags are already converted to `InventoryType` at the codec boundary. [VERIFIED: packages/open-bitcoin-codec/src/network.rs] |
| `open-bitcoin-consensus` | 0.1.0 | Derives `transaction_txid` and `transaction_wtxid` using witness-aware or witness-free serialization. [VERIFIED: cargo metadata --manifest-path packages/Cargo.toml --no-deps; VERIFIED: packages/open-bitcoin-consensus/src/crypto.rs] | Received `tx` cleanup requires deriving both identities exactly once. [VERIFIED: .planning/phases/101-transaction-inventory-identity-and-download-scheduling/101-CONTEXT.md; VERIFIED: packages/open-bitcoin-network/src/peer/inventory_state.rs] |
| Bitcoin Knots baseline | `29.3.knots20260210` submodule at `a9aee730466ac67d35a3c03ee24676be5e045878` | Parity anchor for transaction download behavior. [VERIFIED: git submodule status packages/bitcoin-knots] | Phase 101 behavior must preserve in-scope externally observable Knots behavior. [VERIFIED: AGENTS.md; VERIFIED: .planning/PROJECT.md via AGENTS.md] |

### Supporting

| Library / Module | Version | Purpose | When to Use |
|------------------|---------|---------|-------------|
| `ResourceGovernancePolicy` | First-party in `open-bitcoin-network` 0.1.0 | Caps inbound inventory/getdata/request pressure with low-cardinality labels. [VERIFIED: packages/open-bitcoin-network/src/resource.rs; VERIFIED: cargo metadata --manifest-path packages/Cargo.toml --no-deps] | Use for Phase 94 cap alignment and add narrower scheduler caps only when Phase 101 needs transaction download-specific semantics. [VERIFIED: .planning/phases/101-transaction-inventory-identity-and-download-scheduling/101-CONTEXT.md; VERIFIED: packages/open-bitcoin-network/src/resource.rs] |
| `open-bitcoin-node::ManagedPeerNetwork` | 0.1.0 | Adapter bridge from pure peer actions to managed mempool/storage behavior. [VERIFIED: cargo metadata --manifest-path packages/Cargo.toml --no-deps; VERIFIED: packages/open-bitcoin-node/src/network.rs] | Keep as shell translation; do not move scheduler or admission policy into managed runtime. [VERIFIED: packages/open-bitcoin-node/src/network.rs; VERIFIED: .planning/phases/101-transaction-inventory-identity-and-download-scheduling/101-CONTEXT.md] |
| Bun | 1.3.9 | Runs repo-owned TypeScript verification scripts. [VERIFIED: bun --version; VERIFIED: .bun-version; VERIFIED: scripts/verify.sh] | Needed for `check-parity-breadcrumbs.ts` and phase guardrail checks in `bash scripts/verify.sh`. [VERIFIED: scripts/verify.sh; VERIFIED: scripts/check-parity-breadcrumbs.ts] |
| Bazel/Bazelisk | Bazelisk 1.28.1, Bazel 8.6.0 | Top-level smoke build through repo verifier. [VERIFIED: bazelisk version; VERIFIED: bazel version; VERIFIED: scripts/verify.sh] | Required by default verifier after Rust and static checks. [VERIFIED: scripts/verify.sh; VERIFIED: AGENTS.md] |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| `TxRelayId` enum | Raw `Hash32` plus `InventoryType` everywhere | Raw pairs keep illegal combinations representable and contradict the locked decision to parse identity at the wire boundary. [VERIFIED: .planning/phases/101-transaction-inventory-identity-and-download-scheduling/101-CONTEXT.md; VERIFIED: standards/core/architecture.md] |
| First-party scheduler | External Rust Bitcoin relay library | Existing Rust Bitcoin libraries are forbidden in the production path, and the current repo already owns primitives/codecs needed for this phase. [VERIFIED: AGENTS.md; VERIFIED: packages/open-bitcoin-primitives/src/hash.rs; VERIFIED: packages/open-bitcoin-codec/src/network.rs] |
| Deterministic `now` input | Wall-clock sleeps or Tokio timers in scheduler tests | Phase 101 explicitly requires fake-clock expiry tests and no sleeps or public-network behavior in `scripts/verify.sh`. [VERIFIED: .planning/phases/101-transaction-inventory-identity-and-download-scheduling/101-CONTEXT.md; VERIFIED: .planning/ROADMAP.md] |
| Typed action enum | Boolean returns or log-string parsing | The phase requires stable typed actions and fixed low-cardinality evidence labels. [VERIFIED: .planning/phases/101-transaction-inventory-identity-and-download-scheduling/101-CONTEXT.md] |

**Installation:**

```bash
# No new dependency installation is recommended for Phase 101.
cargo metadata --manifest-path packages/Cargo.toml --no-deps --format-version 1
```

**Version verification:** Recommended stack versions were verified from `rustc --version`, `cargo --version`, `cargo metadata --manifest-path packages/Cargo.toml --no-deps --format-version 1`, `git submodule status packages/bitcoin-knots`, `bun --version`, `bazelisk version`, and `bazel version`. [VERIFIED: shell commands in research session]

## Architecture Patterns

### Recommended Project Structure

```text
packages/open-bitcoin-network/src/
├── peer.rs                         # PeerManager, PeerAction, message dispatch shell [VERIFIED: packages/open-bitcoin-network/src/peer.rs]
├── peer/
│   ├── inventory_state.rs          # Existing block/getdata inventory shell; keep block paths separate [VERIFIED: packages/open-bitcoin-network/src/peer/inventory_state.rs]
│   ├── transaction_relay.rs        # Recommended new pure tx relay identity and scheduler module [ASSUMED]
│   └── transaction_relay/tests.rs  # Optional focused scheduler tests if inline tests get large [ASSUMED]
├── resource.rs                     # Existing request cap and low-cardinality pressure labels [VERIFIED: packages/open-bitcoin-network/src/resource.rs]
└── message.rs                      # WireNetworkMessage boundary [VERIFIED: packages/open-bitcoin-network/src/message.rs]
```

The recommended new module name is a planner choice, but any new multi-file module should use `transaction_relay.rs` plus `transaction_relay/` rather than `transaction_relay/mod.rs`. [ASSUMED; VERIFIED: standards/languages/rust.md]

### Pattern 1: Typed Transaction Relay Identity

**What:** Represent transaction relay identity as `TxRelayId::Txid(Txid)` or `TxRelayId::Wtxid(Wtxid)` before scheduler code sees an announcement or request. [VERIFIED: .planning/phases/101-transaction-inventory-identity-and-download-scheduling/101-CONTEXT.md; VERIFIED: packages/open-bitcoin-primitives/src/hash.rs]

**When to use:** Use it for transaction `inv`, outgoing `getdata`, matching `notfound`, and matching received `tx` cleanup; do not use it for block inventory. [VERIFIED: .planning/phases/101-transaction-inventory-identity-and-download-scheduling/101-CONTEXT.md; VERIFIED: packages/open-bitcoin-network/src/peer/inventory_state.rs]

**Example:**

```rust
// Source: packages/open-bitcoin-primitives/src/hash.rs
// Source: packages/open-bitcoin-primitives/src/network.rs
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TxRelayId {
    Txid(Txid),
    Wtxid(Wtxid),
}

impl TxRelayId {
    pub const fn inventory_type(self) -> InventoryType {
        match self {
            Self::Txid(_) => InventoryType::Transaction,
            Self::Wtxid(_) => InventoryType::WitnessTransaction,
        }
    }

    pub fn object_hash(self) -> Hash32 {
        match self {
            Self::Txid(txid) => txid.into(),
            Self::Wtxid(wtxid) => wtxid.into(),
        }
    }
}
```

### Pattern 2: Wire-Boundary Negotiation Check

**What:** Convert `InventoryVector` to `TxRelayId` only when the inventory type matches the peer's negotiated `remote_wtxidrelay` mode. [VERIFIED: .planning/phases/101-transaction-inventory-identity-and-download-scheduling/101-CONTEXT.md; VERIFIED: packages/bitcoin-knots/src/net_processing.cpp]

**When to use:** Use this inside `PeerManager::handle_inventory` before inserting announcements into scheduler state. [VERIFIED: packages/open-bitcoin-network/src/peer/inventory_state.rs]

**Example:**

```rust
// Source: packages/bitcoin-knots/src/net_processing.cpp
// Source: packages/open-bitcoin-network/src/peer/inventory_state.rs
fn maybe_tx_relay_id(item: &InventoryVector, remote_wtxidrelay: bool) -> Option<TxRelayId> {
    match (item.inventory_type, remote_wtxidrelay) {
        (InventoryType::Transaction, false) => Some(TxRelayId::Txid(Txid::from(item.object_hash))),
        (InventoryType::WitnessTransaction, true) => {
            Some(TxRelayId::Wtxid(Wtxid::from(item.object_hash)))
        }
        _ => None,
    }
}
```

### Pattern 3: Pure Scheduler Input and Actions

**What:** Keep scheduling as data-in/data-out: peer id, negotiated mode, preference, already-have facts, recent-reject facts, in-flight caps, and fake-clock `now` go in; typed scheduler actions come out. [VERIFIED: .planning/phases/101-transaction-inventory-identity-and-download-scheduling/101-CONTEXT.md; VERIFIED: standards/core/architecture.md]

**When to use:** Use this for `inv`, timeout polling, `notfound`, received `tx`, and disconnect cleanup. [VERIFIED: packages/bitcoin-knots/src/txrequest.h; VERIFIED: packages/bitcoin-knots/src/node/txdownloadman.h]

**Example:**

```rust
// Source: packages/bitcoin-knots/src/txrequest.h
// Source: .planning/phases/101-transaction-inventory-identity-and-download-scheduling/101-CONTEXT.md
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TxDownloadAction {
    RequestGetData { peer_id: PeerId, relay_id: TxRelayId },
    SuppressDuplicate { relay_id: TxRelayId },
    SuppressAlreadyHave { relay_id: TxRelayId },
    SuppressRecentReject { relay_id: TxRelayId },
    SuppressIdentityMismatch { inventory_type: InventoryType },
    FallbackRequest { peer_id: PeerId, relay_id: TxRelayId },
    RequestExpired { peer_id: PeerId, relay_id: TxRelayId },
    NotFoundCleanup { peer_id: PeerId, relay_id: TxRelayId },
    ReceivedTxCleanup { peer_id: PeerId, txid: Txid, wtxid: Wtxid },
    PeerCleanup { peer_id: PeerId },
}
```

### Pattern 4: Candidate, In-Flight, Completed Cleanup

**What:** Model announcement lifecycle explicitly rather than using only `requested_txids` and `requested_wtxids`. [VERIFIED: packages/bitcoin-knots/src/txrequest.h; VERIFIED: packages/open-bitcoin-network/src/peer.rs]

**When to use:** Use it when duplicates, timeout fallback, `notfound` fallback, and disconnect fallback must leave no stale request state. [VERIFIED: .planning/ROADMAP.md; VERIFIED: packages/bitcoin-knots/test/functional/p2p_tx_download.py]

**Recommended minimal states:**

| State | Meaning | Source |
|-------|---------|--------|
| Candidate delayed | Announcement is known but not requestable until its reqtime. | [VERIFIED: packages/bitcoin-knots/src/txrequest.h; VERIFIED: packages/bitcoin-knots/src/txrequest.cpp] |
| Candidate ready | Announcement may be selected when no equivalent request is in flight. | [VERIFIED: packages/bitcoin-knots/src/txrequest.h; VERIFIED: packages/bitcoin-knots/src/txrequest.cpp] |
| In flight | `getdata` was emitted and response is awaited until expiry. | [VERIFIED: packages/bitcoin-knots/src/txrequest.h; VERIFIED: packages/bitcoin-knots/src/node/txdownloadman_impl.cpp] |
| Completed/cleared | A response, timeout, already-have fact, or disconnect made the request no longer active. | [VERIFIED: packages/bitcoin-knots/src/txrequest.h; VERIFIED: packages/bitcoin-knots/src/txrequest.cpp] |

### Anti-Patterns to Avoid

- **Raw hash scheduling:** Do not pass `Hash32` plus `InventoryType` deep into scheduler logic because the phase requires typed txid/wtxid identity before scheduling. [VERIFIED: .planning/phases/101-transaction-inventory-identity-and-download-scheduling/101-CONTEXT.md]
- **Immediate `GetData` on every valid `inv`:** Do not request duplicates while an equivalent identity is already in flight; retain useful duplicate announcers for fallback instead. [VERIFIED: .planning/phases/101-transaction-inventory-identity-and-download-scheduling/101-CONTEXT.md; VERIFIED: packages/bitcoin-knots/src/txrequest.h]
- **Mempool/storage calls inside scheduler:** Do not call mempool, durable storage, sockets, Tokio timers, or runtime APIs from the pure scheduler. [VERIFIED: .planning/phases/101-transaction-inventory-identity-and-download-scheduling/101-CONTEXT.md; VERIFIED: standards/core/architecture.md]
- **Block and transaction accounting merged together:** Do not blur block request state with transaction download state; block `getheaders`/`getdata` paths already have separate behavior. [VERIFIED: .planning/phases/101-transaction-inventory-identity-and-download-scheduling/101-CONTEXT.md; VERIFIED: packages/open-bitcoin-network/src/peer/inventory_state.rs]
- **Dynamic labels:** Do not include raw txids, wtxids, peer endpoints, permission strings, transaction hex, class names, credentials, or dynamic labels in evidence. [VERIFIED: .planning/phases/101-transaction-inventory-identity-and-download-scheduling/101-CONTEXT.md; VERIFIED: AGENTS.md]

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Txid/wtxid primitive identity | New byte arrays or strings for transaction ids | Existing `Txid`, `Wtxid`, and `Hash32` wrappers | The repo already provides semantic wrappers over 32-byte hashes. [VERIFIED: packages/open-bitcoin-primitives/src/hash.rs] |
| Wire inventory parsing | Custom byte parsing in peer code | `open-bitcoin-codec::parse_inventory_vector` and `WireNetworkMessage` decode path | The codec already parses raw inventory type tags into `InventoryType` and `Hash32`. [VERIFIED: packages/open-bitcoin-codec/src/network.rs; VERIFIED: packages/open-bitcoin-network/src/message.rs] |
| Transaction id derivation | Recomputing ad hoc hashes in peer code | `transaction_txid` and `transaction_wtxid` | The consensus crate already applies witness-free versus witness-aware transaction encoding correctly. [VERIFIED: packages/open-bitcoin-consensus/src/crypto.rs] |
| Request pressure labels | Per-tx/per-peer log labels | `ResourceGovernancePolicy` labels and fixed scheduler action variants | Existing resource labels are fixed and low-cardinality, matching Phase 101 evidence constraints. [VERIFIED: packages/open-bitcoin-network/src/resource.rs; VERIFIED: .planning/phases/101-transaction-inventory-identity-and-download-scheduling/101-CONTEXT.md] |
| Scheduler timing | Sleeps or wall-clock reads | Explicit `now_unix_seconds` or duration input passed to pure scheduler | Phase 101 requires fake-clock request expiry tests. [VERIFIED: .planning/ROADMAP.md; VERIFIED: .planning/phases/101-transaction-inventory-identity-and-download-scheduling/101-CONTEXT.md] |
| Fallback state | Only per-peer `BTreeSet<Txid>`/`BTreeSet<Wtxid>` | Per-identity announcement and in-flight records | The current sets cannot retain duplicate announcers for fallback while suppressing redundant `getdata`. [VERIFIED: packages/open-bitcoin-network/src/peer.rs; VERIFIED: packages/open-bitcoin-network/src/peer/inventory_state.rs; VERIFIED: packages/bitcoin-knots/src/txrequest.h] |

**Key insight:** This phase should hand-roll no protocol parsing, hash derivation, clock sleeping, or runtime I/O; the only first-party custom logic should be the pure transaction request state machine required by the project-owned domain model. [VERIFIED: AGENTS.md; VERIFIED: packages/open-bitcoin-primitives/src/hash.rs; VERIFIED: packages/open-bitcoin-codec/src/network.rs; VERIFIED: packages/open-bitcoin-consensus/src/crypto.rs; VERIFIED: .planning/phases/101-transaction-inventory-identity-and-download-scheduling/101-CONTEXT.md]

## Common Pitfalls

### Pitfall 1: Collapsing txid and wtxid

**What goes wrong:** A single `Hash32` key treats txid and wtxid announcements as interchangeable even when witness semantics matter. [VERIFIED: packages/open-bitcoin-primitives/src/hash.rs; VERIFIED: docs/parity/catalog/core-domain-and-serialization.md]

**Why it happens:** Existing inventory vectors are type tag plus object hash, and current Open Bitcoin request sets split only after matching `InventoryType`. [VERIFIED: packages/open-bitcoin-primitives/src/network.rs; VERIFIED: packages/open-bitcoin-network/src/peer/inventory_state.rs]

**How to avoid:** Convert to `TxRelayId` at the wire boundary and expose only typed identity to scheduler state. [VERIFIED: .planning/phases/101-transaction-inventory-identity-and-download-scheduling/101-CONTEXT.md]

**Warning signs:** Scheduler functions take `Hash32`, `InventoryVector`, or `InventoryType` instead of `TxRelayId`. [ASSUMED]

### Pitfall 2: Treating negotiation mismatches as fallback

**What goes wrong:** `MSG_TX` from a wtxidrelay peer or `MSG_WTX` from a txid-only peer can create request state that later cannot be satisfied coherently. [VERIFIED: .planning/phases/101-transaction-inventory-identity-and-download-scheduling/101-CONTEXT.md; VERIFIED: packages/bitcoin-knots/src/net_processing.cpp]

**Why it happens:** Current `handle_inventory` silently ignores mismatches without a typed suppression action, so future code could accidentally add fallback behavior while refactoring. [VERIFIED: packages/open-bitcoin-network/src/peer/inventory_state.rs]

**How to avoid:** Emit `suppress_identity_mismatch` and do not add candidate or in-flight records for the mismatched item. [VERIFIED: .planning/phases/101-transaction-inventory-identity-and-download-scheduling/101-CONTEXT.md]

**Warning signs:** A mismatched inv later appears in scheduler candidate counts or outgoing `GetData`. [ASSUMED]

### Pitfall 3: Losing fallback announcers

**What goes wrong:** Dropping duplicate announcements prevents timeout, `notfound`, or disconnect from selecting another peer. [VERIFIED: .planning/phases/101-transaction-inventory-identity-and-download-scheduling/101-CONTEXT.md; VERIFIED: packages/bitcoin-knots/test/functional/p2p_tx_download.py]

**Why it happens:** A bare per-peer requested set records what one peer was asked for but not who else announced the same identity. [VERIFIED: packages/open-bitcoin-network/src/peer.rs; VERIFIED: packages/open-bitcoin-network/src/peer/inventory_state.rs]

**How to avoid:** Keep per-identity candidate announcers while suppressing redundant `getdata` when one equivalent request is in flight. [VERIFIED: packages/bitcoin-knots/src/txrequest.h]

**Warning signs:** Duplicate inv tests assert only "no second request" and do not assert fallback after timeout/`notfound`/disconnect. [ASSUMED]

### Pitfall 4: Clearing unrelated request state on mismatched `tx`

**What goes wrong:** A peer can satisfy or erase a request by sending a different transaction identity. [VERIFIED: .planning/phases/101-transaction-inventory-identity-and-download-scheduling/101-CONTEXT.md]

**Why it happens:** Current `handle_transaction` derives txid and wtxid and removes both matching values from that peer's sets, but it does not check whether the peer had a matching in-flight request. [VERIFIED: packages/open-bitcoin-network/src/peer/inventory_state.rs]

**How to avoid:** Match received txid/wtxid against the peer's in-flight request records, emit mismatch suppression when neither matches, and clear only matching state. [VERIFIED: .planning/phases/101-transaction-inventory-identity-and-download-scheduling/101-CONTEXT.md]

**Warning signs:** A received transaction action is emitted with no accompanying cleanup classification. [ASSUMED]

### Pitfall 5: Moving admission into Phase 101

**What goes wrong:** The scheduler starts deciding standardness, orphan handling, fees, RBF, mempool persistence, or relay fanout. [VERIFIED: .planning/phases/101-transaction-inventory-identity-and-download-scheduling/101-CONTEXT.md; VERIFIED: .planning/REQUIREMENTS.md]

**Why it happens:** Current `ManagedPeerNetwork::process_actions` submits `PeerAction::ReceivedTransaction` to the mempool immediately when not already stored. [VERIFIED: packages/open-bitcoin-node/src/network.rs]

**How to avoid:** Preserve or narrow the adapter bridge for now, but keep scheduler outputs as stable typed received-transaction responses that Phase 102 can consume. [VERIFIED: .planning/phases/101-transaction-inventory-identity-and-download-scheduling/101-CONTEXT.md]

**Warning signs:** Phase 101 tests assert mempool policy outcomes rather than scheduler state/action outcomes. [VERIFIED: .planning/REQUIREMENTS.md; ASSUMED]

### Pitfall 6: Forgetting parity breadcrumbs

**What goes wrong:** New first-party Rust source or test files fail `check-parity-breadcrumbs.ts --check`. [VERIFIED: scripts/check-parity-breadcrumbs.ts; VERIFIED: scripts/verify.sh]

**Why it happens:** The checker maps every tracked `packages/open-bitcoin-*/src` and `packages/open-bitcoin-*/tests` Rust file to `docs/parity/source-breadcrumbs.json`. [VERIFIED: scripts/check-parity-breadcrumbs.ts]

**How to avoid:** Add any new scheduler source/test files to the transaction relay breadcrumb group with Knots anchors such as `txdownloadman.h`, `txdownloadman_impl.cpp`, `txrequest.h`, `txrequest.cpp`, `net_processing.cpp`, and `p2p_tx_download.py`. [VERIFIED: docs/parity/source-breadcrumbs.json; VERIFIED: packages/bitcoin-knots/src/node/txdownloadman.h; VERIFIED: packages/bitcoin-knots/src/txrequest.h; VERIFIED: packages/bitcoin-knots/test/functional/p2p_tx_download.py]

**Warning signs:** `bun run scripts/check-parity-breadcrumbs.ts --check` reports a missing mapping or expected breadcrumb block. [VERIFIED: scripts/check-parity-breadcrumbs.ts]

## Code Examples

Verified patterns from official or local sources. [VERIFIED: packages/open-bitcoin-network/src/peer.rs; VERIFIED: packages/bitcoin-knots/src/txrequest.h]

### Convert Scheduler Request To Wire `getdata`

```rust
// Source: packages/open-bitcoin-network/src/message.rs
// Source: packages/open-bitcoin-primitives/src/network.rs
fn getdata_for_request(relay_id: TxRelayId) -> WireNetworkMessage {
    WireNetworkMessage::GetData(InventoryList::new(vec![InventoryVector {
        inventory_type: relay_id.inventory_type(),
        object_hash: relay_id.object_hash(),
    }]))
}
```

### Received Transaction Cleanup Input

```rust
// Source: packages/open-bitcoin-consensus/src/crypto.rs
// Source: packages/open-bitcoin-network/src/peer/inventory_state.rs
fn received_tx_ids(transaction: &Transaction) -> Result<(Txid, Wtxid), NetworkError> {
    let txid = transaction_txid(transaction)?;
    let wtxid = transaction_wtxid(transaction)?;
    Ok((txid, wtxid))
}
```

### Timeout Polling Without Sleeps

```rust
// Source: packages/bitcoin-knots/src/txrequest.h
// Source: .planning/phases/101-transaction-inventory-identity-and-download-scheduling/101-CONTEXT.md
let actions = scheduler.expire_and_schedule(TxSchedulerTick {
    now_unix_seconds: fake_now,
    max_in_flight_per_peer: PHASE101_MAX_TX_REQUESTS_IN_FLIGHT_PER_PEER,
});
```

## State of the Art

| Old Approach | Current Approach | Source Anchor | Impact |
|--------------|------------------|---------------|--------|
| Collapse transaction inventory into type tag plus generic hash deep in peer logic | Preserve transaction identity flavor through `GenTxid`/`TxRelayId` style typed identity | [VERIFIED: packages/bitcoin-knots/src/protocol.h; VERIFIED: docs/parity/catalog/core-domain-and-serialization.md] | Prevents txid/wtxid mismatch bugs and supports BIP339-style negotiation. [VERIFIED: packages/bitcoin-knots/src/net_processing.cpp] |
| Request immediately from first valid announcer | Track candidate, requested, completed, delayed, and expired announcement states | [VERIFIED: packages/bitcoin-knots/src/txrequest.h; VERIFIED: packages/bitcoin-knots/src/txrequest.cpp] | Enables duplicate suppression, preferred-peer selection, timeout fallback, and disconnect cleanup. [VERIFIED: packages/bitcoin-knots/test/functional/p2p_tx_download.py] |
| Wall-clock integration tests for expiry | Deterministic fake-time scheduler tests | [VERIFIED: .planning/phases/101-transaction-inventory-identity-and-download-scheduling/101-CONTEXT.md; VERIFIED: packages/bitcoin-knots/test/functional/p2p_tx_download.py] | Keeps default verification deterministic and avoids sleeps. [VERIFIED: .planning/ROADMAP.md] |
| Dynamic peer/transaction evidence | Fixed action variants and low-cardinality labels | [VERIFIED: .planning/phases/101-transaction-inventory-identity-and-download-scheduling/101-CONTEXT.md; VERIFIED: packages/open-bitcoin-network/src/resource.rs] | Avoids leaking transaction or peer material into status/log/support surfaces. [VERIFIED: AGENTS.md] |

**Deprecated/outdated for this phase:**

- `PeerState::requested_txids` and `PeerState::requested_wtxids` as the only transaction request state are insufficient for fallback and cleanup behavior. [VERIFIED: packages/open-bitcoin-network/src/peer.rs; VERIFIED: .planning/phases/101-transaction-inventory-identity-and-download-scheduling/101-CONTEXT.md]
- Scheduler logic that emits `WireNetworkMessage::GetData` directly from every accepted `inv` is insufficient because the phase requires typed request, suppression, fallback, expiry, and cleanup actions. [VERIFIED: packages/open-bitcoin-network/src/peer/inventory_state.rs; VERIFIED: .planning/phases/101-transaction-inventory-identity-and-download-scheduling/101-CONTEXT.md]
- Treating `notfound` as only a set removal is insufficient because the phase requires immediate fallback when another eligible announcer exists. [VERIFIED: packages/open-bitcoin-network/src/peer.rs; VERIFIED: .planning/phases/101-transaction-inventory-identity-and-download-scheduling/101-CONTEXT.md; VERIFIED: packages/bitcoin-knots/test/functional/p2p_tx_download.py]

## Open Questions

1. **Should constants mirror Knots exactly or use smaller test-friendly values?** [VERIFIED: .planning/phases/101-transaction-inventory-identity-and-download-scheduling/101-CONTEXT.md]
   - What we know: Knots defines `MAX_PEER_TX_REQUEST_IN_FLIGHT = 100`, `MAX_PEER_TX_ANNOUNCEMENTS = 5000`, `TXID_RELAY_DELAY = 2s`, `NONPREF_PEER_TX_DELAY = 2s`, `OVERLOADED_PEER_TX_DELAY = 2s`, and `GETDATA_TX_INTERVAL = 60s`. [VERIFIED: packages/bitcoin-knots/src/node/txdownloadman.h]
   - What's unclear: The context allows simplified values if tests preserve Phase 101's claimed observable behavior. [VERIFIED: .planning/phases/101-transaction-inventory-identity-and-download-scheduling/101-CONTEXT.md]
   - Recommendation: Keep Knots names and defaults in production policy, but allow tests to construct a scheduler policy with shorter durations. [ASSUMED]

2. **Where should relay eligibility feed into scheduler input?** [VERIFIED: packages/open-bitcoin-network/src/relay.rs; VERIFIED: packages/open-bitcoin-network/src/peer.rs]
   - What we know: Phase 100 added pure relay activation/eligibility policy, and `PeerState` currently records connection role and negotiation fields but not an explicit transaction download eligibility object. [VERIFIED: packages/open-bitcoin-network/src/relay.rs; VERIFIED: packages/open-bitcoin-network/src/peer.rs]
   - What's unclear: The planner must choose whether `PeerManager` stores per-peer scheduler eligibility directly or passes it from a higher-level activation bridge. [ASSUMED]
   - Recommendation: Pass scheduler peer facts explicitly as data and avoid having scheduler call activation/config APIs. [VERIFIED: standards/core/architecture.md; ASSUMED]

3. **Should `PeerAction::ReceivedTransaction` be split in Phase 101 or left compatible until Phase 102?** [VERIFIED: packages/open-bitcoin-network/src/peer.rs; VERIFIED: packages/open-bitcoin-node/src/network.rs]
   - What we know: The context permits continuing to return a received transaction action while leaving mempool admission semantics to Phase 102. [VERIFIED: .planning/phases/101-transaction-inventory-identity-and-download-scheduling/101-CONTEXT.md]
   - What's unclear: The planner must decide whether to add a richer `ReceivedTransaction` payload now or add a sibling cleanup action consumed only by tests. [ASSUMED]
   - Recommendation: Add typed cleanup/suppression actions now and keep managed mempool admission behavior compatible unless a narrow adapter change is needed. [ASSUMED]

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | Recommended module name `peer/transaction_relay.rs` is a good planner default. | Architecture Patterns | Low; the context allows exact module split and type names at planner discretion. |
| A2 | Tests can use shorter scheduler durations while production defaults keep Knots-inspired names and values. | Open Questions | Medium; exact constants affect parity claims and should be locked by the planner. |
| A3 | Scheduler eligibility should be passed as peer facts rather than stored as a direct config dependency inside the scheduler. | Open Questions | Low; this follows functional-core guidance but integration shape is planner-owned. |
| A4 | `PeerAction::ReceivedTransaction` compatibility should be preserved unless a narrow adapter change is required. | Open Questions | Medium; Phase 102 may prefer a richer action contract, so Phase 101 should avoid overfitting. |

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|-------------|-----------|---------|----------|
| Rust compiler | Rust implementation and tests | yes | `rustc 1.94.1` | None needed. [VERIFIED: rustc --version] |
| Cargo | Cargo checks/tests and metadata | yes | `cargo 1.94.1` | None needed. [VERIFIED: cargo --version] |
| `cargo-llvm-cov` | Full `scripts/verify.sh` coverage stage | yes | `cargo-llvm-cov 0.8.5` | Use `bash scripts/verify.sh --fast` only for iteration; default verification needs coverage. [VERIFIED: cargo llvm-cov --version; VERIFIED: scripts/verify.sh] |
| Bun | TypeScript checkers and parity breadcrumb checker | yes | `1.3.9` | None needed. [VERIFIED: bun --version; VERIFIED: .bun-version] |
| Bazel/Bazelisk | Default verifier Bazel smoke build | yes | Bazelisk `1.28.1`, Bazel `8.6.0` | None needed. [VERIFIED: bazelisk version; VERIFIED: bazel version; VERIFIED: scripts/verify.sh] |
| Bash | Repo verifier shell | yes | GNU bash `3.2.57(1)` | None needed. [VERIFIED: bash --version; VERIFIED: scripts/verify.sh] |
| Node.js | GSD tooling and local automation support | yes | `v24.13.0` | None needed. [VERIFIED: node --version] |
| Bitcoin Knots submodule | Parity anchors | yes | `v29.3.knots20260210` at `a9aee730466ac67d35a3c03ee24676be5e045878` | Run `git submodule update --init --recursive` if missing. [VERIFIED: git submodule status packages/bitcoin-knots; VERIFIED: AGENTS.md] |

**Missing dependencies with no fallback:**

- None found during the research environment audit. [VERIFIED: rustc --version; VERIFIED: cargo --version; VERIFIED: cargo llvm-cov --version; VERIFIED: bun --version; VERIFIED: bazelisk version; VERIFIED: bazel version]

**Missing dependencies with fallback:**

- None found during the research environment audit. [VERIFIED: shell commands in research session]

## Security Domain

Security enforcement is enabled by default because `.planning/config.json` does not set `security_enforcement` to `false`. [VERIFIED: .planning/config.json]

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|------------------|
| V2 Authentication | no | Phase 101 does not add user authentication or credential handling. [VERIFIED: .planning/ROADMAP.md; VERIFIED: packages/open-bitcoin-network/src/peer.rs] |
| V3 Session Management | no | Phase 101 manages P2P peer state, not web user sessions. [VERIFIED: .planning/ROADMAP.md; VERIFIED: packages/open-bitcoin-network/src/peer.rs] |
| V4 Access Control | yes | Relay eligibility and permission effects should remain explicit scheduler inputs or peer facts, not implicit socket behavior. [VERIFIED: packages/open-bitcoin-network/src/relay.rs; VERIFIED: .planning/phases/101-transaction-inventory-identity-and-download-scheduling/101-CONTEXT.md] |
| V5 Input Validation | yes | Inventory tags must be parsed into typed identities and mismatches must suppress state creation. [VERIFIED: packages/open-bitcoin-codec/src/network.rs; VERIFIED: packages/open-bitcoin-primitives/src/network.rs; VERIFIED: .planning/phases/101-transaction-inventory-identity-and-download-scheduling/101-CONTEXT.md] |
| V6 Cryptography | yes | Transaction ids must use existing consensus hash derivation; do not hand-roll hashing. [VERIFIED: packages/open-bitcoin-consensus/src/crypto.rs] |

### Known Threat Patterns for P2P Transaction Download

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Inventory identity spoofing through txid/wtxid mismatch | Spoofing/Tampering | Suppress mismatched inventory before scheduler state is created. [VERIFIED: packages/bitcoin-knots/src/net_processing.cpp; VERIFIED: .planning/phases/101-transaction-inventory-identity-and-download-scheduling/101-CONTEXT.md] |
| Inventory/request flooding | Denial of Service | Apply Phase 94 request caps and scheduler-specific in-flight/announcement caps with fixed labels. [VERIFIED: packages/open-bitcoin-network/src/resource.rs; VERIFIED: packages/bitcoin-knots/src/node/txdownloadman.h] |
| Censorship by fast duplicate announcers that do not respond | Denial of Service/Tampering | Keep duplicate announcers as fallback candidates, prefer trusted peers when available, expire in-flight requests, and avoid re-requesting from the same failed announcer. [VERIFIED: packages/bitcoin-knots/src/txrequest.h; VERIFIED: packages/bitcoin-knots/test/functional/p2p_tx_download.py] |
| Stale in-flight state after `notfound`, timeout, received tx, or disconnect | Denial of Service | Convert matching requests to completed/cleared state and select fallback candidates when available. [VERIFIED: packages/bitcoin-knots/src/txrequest.h; VERIFIED: packages/bitcoin-knots/src/txrequest.cpp; VERIFIED: packages/bitcoin-knots/test/functional/p2p_tx_download.py] |
| Evidence leakage through raw txids/wtxids or peer labels | Information Disclosure | Emit fixed low-cardinality action/reason labels and avoid raw transaction or peer material in logs/status/support surfaces. [VERIFIED: .planning/phases/101-transaction-inventory-identity-and-download-scheduling/101-CONTEXT.md; VERIFIED: AGENTS.md] |

## Sources

### Primary (HIGH confidence)

- `.planning/phases/101-transaction-inventory-identity-and-download-scheduling/101-CONTEXT.md` - locked Phase 101 decisions, boundaries, tests, and deferred scope. [VERIFIED: file read]
- `.planning/REQUIREMENTS.md` - INV-01 through INV-04 and DL-01 through DL-02 ownership. [VERIFIED: file read]
- `.planning/ROADMAP.md` - Phase 101 purpose, scope, success criteria, verification, and dependency on Phase 100. [VERIFIED: file read]
- `AGENTS.md`, `AGENTS.bright-builds.md`, `standards/core/architecture.md`, `standards/core/testing.md`, `standards/core/verification.md`, `standards/languages/rust.md` - repo constraints and Bright Builds standards. [VERIFIED: file reads]
- `packages/open-bitcoin-network/src/peer.rs` - existing `PeerState`, `PeerAction`, negotiation, dispatch, and peer removal seams. [VERIFIED: file read]
- `packages/open-bitcoin-network/src/peer/inventory_state.rs` - current inventory, `getdata`, `notfound`, `tx`, and requested-inventory behavior. [VERIFIED: file read]
- `packages/open-bitcoin-primitives/src/hash.rs`, `packages/open-bitcoin-primitives/src/network.rs`, `packages/open-bitcoin-codec/src/network.rs`, `packages/open-bitcoin-consensus/src/crypto.rs` - existing primitives, inventory tags, codec parsing, and txid/wtxid derivation. [VERIFIED: file reads]
- `packages/open-bitcoin-network/src/resource.rs` - Phase 94 request caps and low-cardinality resource labels. [VERIFIED: file read]
- `packages/bitcoin-knots/src/node/txdownloadman.h`, `packages/bitcoin-knots/src/node/txdownloadman_impl.cpp`, `packages/bitcoin-knots/src/txrequest.h`, `packages/bitcoin-knots/src/txrequest.cpp`, `packages/bitcoin-knots/src/net_processing.cpp`, `packages/bitcoin-knots/src/protocol.h` - pinned Knots transaction download and protocol anchors. [VERIFIED: file reads]
- `packages/bitcoin-knots/test/functional/p2p_tx_download.py` and `packages/bitcoin-knots/test/functional/p2p_getdata.py` - baseline behavioral edge cases for fallback, mismatch, caps, recent rejects, spurious `notfound`, and invalid `getdata`. [VERIFIED: file reads]
- `scripts/verify.sh` and `scripts/check-parity-breadcrumbs.ts` - verifier and breadcrumb contract. [VERIFIED: file reads]

### Secondary (MEDIUM confidence)

- Bitcoin BIPs repository, BIP 339 (`https://github.com/bitcoin/bips/blob/master/bip-0339.mediawiki`) - wtxidrelay protocol context. [CITED: github.com/bitcoin/bips]
- Bitcoin BIPs repository, BIP 144 (`https://github.com/bitcoin/bips/blob/master/bip-0144.mediawiki`) - witness inventory context. [CITED: github.com/bitcoin/bips]
- OWASP ASVS project page (`https://owasp.org/www-project-application-security-verification-standard/`) - ASVS framing for security category review. [CITED: owasp.org]

### Tertiary (LOW confidence)

- None; unresolved planner choices are recorded in the Assumptions Log instead of being presented as facts. [VERIFIED: Assumptions Log]

## Metadata

**Confidence breakdown:**

- Standard stack: HIGH - versions and crate membership were verified with repo files and local commands. [VERIFIED: rustc --version; VERIFIED: cargo metadata --manifest-path packages/Cargo.toml --no-deps; VERIFIED: git submodule status packages/bitcoin-knots]
- Architecture: HIGH for pure-core boundary and existing seams; MEDIUM for recommended new module names because naming is planner discretion. [VERIFIED: standards/core/architecture.md; VERIFIED: packages/open-bitcoin-network/src/peer.rs; ASSUMED]
- Pitfalls: HIGH - pitfalls map to locked Phase 101 decisions, current code gaps, and Knots behavior. [VERIFIED: .planning/phases/101-transaction-inventory-identity-and-download-scheduling/101-CONTEXT.md; VERIFIED: packages/open-bitcoin-network/src/peer/inventory_state.rs; VERIFIED: packages/bitcoin-knots/src/txrequest.h]

**Research date:** 2026-06-29 [VERIFIED: system current_date]
**Valid until:** 2026-07-29 for repo-internal seams; re-check external BIP/ASVS links and tool versions if planning resumes after that date. [ASSUMED]
