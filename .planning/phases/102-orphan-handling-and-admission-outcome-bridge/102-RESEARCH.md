# Phase 102: Orphan Handling and Admission Outcome Bridge - Research

**Researched:** 2026-06-30 [VERIFIED: environment_context current_date]
**Domain:** Rust Bitcoin transaction relay, orphan staging, and mempool admission outcomes [VERIFIED: .planning/phases/102-orphan-handling-and-admission-outcome-bridge/102-CONTEXT.md]
**Confidence:** HIGH for codebase structure and Knots anchors; MEDIUM for exact new type names and cap constants because those remain planner choices [VERIFIED: codebase grep; ASSUMED]

<user_constraints>
## User Constraints (from CONTEXT.md)

Everything in this `<user_constraints>` block is copied from `.planning/phases/102-orphan-handling-and-admission-outcome-bridge/102-CONTEXT.md` and is the controlling user context for this phase. [VERIFIED: .planning/phases/102-orphan-handling-and-admission-outcome-bridge/102-CONTEXT.md]

### Locked Decisions

## Implementation Decisions

### Missing-Parent Staging And Parent Requests

- **D-01:** Missing-input peer transactions should become a typed orphan or candidate outcome rather than a generic mempool error that callers interpret with string matching.
- **D-02:** Orphan staging must be bounded by deterministic caps for total entries, per-peer entries, and expiry time. The planner may choose exact constant names and values, but tests must cover cap eviction and expiry without sleeping or public-network behavior.
- **D-03:** Parent request behavior should reuse Phase 101 `TxRelayId`, request, and suppression vocabulary where possible. Parent requests must be eligible typed scheduler actions, not socket writes or mempool mutations inside peer state.
- **D-04:** Orphan evidence must use fixed low-cardinality labels such as `orphaned`, `parent_requested`, `orphan_evicted`, `orphan_expired`, and `orphan_reconsidered`. Do not expose raw transaction hex, txids, wtxids, peer endpoints, permission strings, class names, credentials, or dynamic labels in shared evidence.

### Reconsideration Flow

- **D-05:** Parent acceptance should trigger reconsideration of staged children through a pure or mostly pure coordinator that takes accepted parent identity plus bounded orphan state and returns candidate admission attempts.
- **D-06:** Reconsideration must be deterministic and bounded. It should avoid recursive unbounded walks, hidden wall-clock reads, and direct socket I/O.
- **D-07:** Reconsideration after parent acceptance should produce stable evidence for accepted child, still-missing-parent child, rejected child, expired child, and evicted child paths.
- **D-08:** Disconnect cleanup should remove or mark peer-owned orphan evidence without leaking stale request state from Phase 101.

### Admission Outcome Contract

- **D-09:** Introduce one stable mempool outcome contract consumed by both peer and local transaction submissions. It should represent at least `accepted`, `rejected`, `duplicate`, `replaced`, `orphaned`, `evicted`, and `expired`.
- **D-10:** Existing `MempoolError` variants should be mapped to the new outcome contract at the mempool boundary. Callers should not pattern-match display strings to decide whether a transaction is an orphan, duplicate, rejected, or eviction case.
- **D-11:** The outcome contract should preserve enough structured data for later RPC, metrics, logs, support bundles, and relay serving without forcing Phase 102 to implement those later surfaces.
- **D-12:** Outcome names and evidence labels must stay low-cardinality and stable so later Phase 105 observability can aggregate them safely.

### Admission Policy Scope

- **D-13:** Admission tests must cover standardness, fee, RBF, ancestor/descendant limits, duplicate handling, and no partial mutation on rejection. Reuse and deepen existing `open-bitcoin-mempool` pure tests before adding adapter-heavy tests.
- **D-14:** No-partial-mutation is a hard invariant. Rejected, non-standard, low-fee, failed replacement, and limit-exceeded candidates must leave accepted mempool entries, indexes, virtual-size totals, and replacement state coherent.
- **D-15:** Replacement outcomes should distinguish ordinary rejection from successful replacement and from replacement-caused eviction. Later phases may expose more operator detail, but Phase 102 must make the internal state transition explicit.
- **D-16:** Package relay and cluster mempool behavior are out of scope. Do not broaden single-transaction orphan handling into package-relay support.

### Managed Runtime Bridge

- **D-17:** Managed runtime tests should prove peer transactions pass through the Phase 101 relay/download boundary before mempool admission. `PeerManager` and socket-facing code should not call mempool APIs directly.
- **D-18:** `ManagedPeerNetwork::process_actions` or a small child bridge module is the expected shell integration point. Keep pure scheduler/admission decisions in `open-bitcoin-network` and `open-bitcoin-mempool`; keep storage, runtime, and managed mempool mutation in `open-bitcoin-node`.
- **D-19:** Local transaction submission should use the same stable outcome contract as peer submissions, even if local and peer callers map outcomes to different later surfaces.
- **D-20:** Phase 102 may add in-memory bridge tests, but durable recovery and restart behavior are Phase 103 scope.

### Resource Governance And Boundaries

- **D-21:** Preserve Phase 94 and Phase 101 resource-governance limits under adversarial transaction download and orphan bursts. Orphan staging must not silently bypass queue, request, timeout, churn, or per-peer caps.
- **D-22:** Default verification must remain deterministic and local. Do not add public-network relay checks, sleeps, service-manager gates, wall-clock soak, or production-deployment checks to `bash scripts/verify.sh`.
- **D-23:** If docs, parity roots, or checkers are updated, preserve the v2.0 no-claim boundary: compact block relay, package relay, bloom/filter serving, public relay defaults, public-network CI, production full-node readiness, and production-funds wallet use stay deferred.
- **D-24:** New first-party Rust source or test files under `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests` need parity breadcrumb entries unless the explicit `none` breadcrumb is the only defensible source anchor.

### the agent's Discretion

The planner may choose exact type names, module split, orphan caps, expiry constants, and whether orphan staging lives in a new network transaction-relay child module, a mempool admission child module, or a small bridge type, as long as functional-core boundaries remain clear and tests prove the observable outcomes above.

### Deferred Ideas (OUT OF SCOPE)

## Deferred Ideas

Durable mempool persistence, block connect/disconnect lifecycle, long-lived mempool pressure and trimming evidence, relay serving, fanout, rebroadcast, RPC/operator/support evidence, support-bundle redaction for transaction material, release-boundary closeout, compact block relay, package relay, bloom/filter serving, public relay defaults, public-network relay CI, production full-node readiness, and production-funds wallet use remain outside Phase 102.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| DL-03 | Node stages missing-parent transactions in a bounded orphan or candidate state and requests eligible parents. [VERIFIED: .planning/REQUIREMENTS.md] | Use a bounded orphanage/coordinator keyed by transaction identity and parent outpoints, plus typed parent request actions using `TxRelayId`. [VERIFIED: packages/open-bitcoin-network/src/peer/transaction_relay.rs; VERIFIED: packages/bitcoin-knots/src/txorphanage.h] |
| DL-04 | Node reconsiders staged missing-parent transactions after parent acceptance and expires or evicts them with evidence when limits are reached. [VERIFIED: .planning/REQUIREMENTS.md] | Use accepted-parent callbacks to return bounded reconsideration candidates and injected-time expiry/eviction outputs. [VERIFIED: packages/bitcoin-knots/src/txorphanage.cpp; VERIFIED: packages/bitcoin-knots/src/node/txdownloadman_impl.cpp] |
| DL-05 | Transaction download behavior preserves v1.9 queue, request, timeout, churn, and resource-governance limits under adversarial bursts. [VERIFIED: .planning/REQUIREMENTS.md] | Reuse Phase 94 `ResourceGovernancePolicy` and Phase 101 request caps; orphan staging must not create socket writes or bypass scheduler pressure checks. [VERIFIED: packages/open-bitcoin-network/src/resource.rs; VERIFIED: packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs] |
| MEM-01 | Peer and local transaction submissions flow through one stable mempool outcome contract for accepted, rejected, duplicate, replaced, orphaned, evicted, and expired states. [VERIFIED: .planning/REQUIREMENTS.md] | Add an exported mempool outcome enum and map `AdmissionResult`/`MempoolError` into it at the mempool boundary; update local and peer bridge callers. [VERIFIED: packages/open-bitcoin-mempool/src/error.rs; VERIFIED: packages/open-bitcoin-mempool/src/types.rs; VERIFIED: packages/open-bitcoin-node/src/network.rs] |
| MEM-02 | Mempool admission tests cover standardness, fees, RBF, ancestor/descendant limits, duplicate handling, and no partial mutation on rejection. [VERIFIED: .planning/REQUIREMENTS.md] | Deepen pure mempool tests and add explicit snapshots of entries, indexes, virtual-size totals, and replacement effects around rejection paths. [VERIFIED: packages/open-bitcoin-mempool/src/pool.rs; VERIFIED: packages/open-bitcoin-mempool/src/pool/tests.rs] |
</phase_requirements>

## Summary

Phase 102 should be planned as a boundary-hardening phase, not as a relay-serving phase. [VERIFIED: .planning/phases/102-orphan-handling-and-admission-outcome-bridge/102-CONTEXT.md] The existing code already has a pure transaction download scheduler in `open-bitcoin-network`, a pure mempool admission core in `open-bitcoin-mempool`, and a managed shell in `open-bitcoin-node`; the plan should connect those three surfaces with typed outcomes and bounded orphan state. [VERIFIED: packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs; VERIFIED: packages/open-bitcoin-mempool/src/pool.rs; VERIFIED: packages/open-bitcoin-node/src/network.rs]

The safest implementation plan is to add a shared `MempoolOutcome`-style contract near `open-bitcoin-mempool`, add a pure bounded orphanage/coordinator near the Phase 101 transaction relay scheduler, and keep all actual mempool mutation and transaction storage in a focused managed-node bridge. [VERIFIED: packages/open-bitcoin-mempool/src/lib.rs; VERIFIED: packages/open-bitcoin-network/src/peer/transaction_relay.rs; VERIFIED: packages/open-bitcoin-node/src/network.rs; ASSUMED] This mirrors the repo's functional-core/imperative-shell rule and the Knots split between transaction download/orphan tracking and validation/mempool admission. [VERIFIED: standards/core/architecture.md; VERIFIED: packages/bitcoin-knots/src/node/txdownloadman_impl.cpp; VERIFIED: packages/bitcoin-knots/src/net_processing.cpp]

Knots provides strong anchors for the behavior but not a one-for-one implementation prescription. [VERIFIED: packages/bitcoin-knots/src/txorphanage.h; VERIFIED: packages/bitcoin-knots/src/txorphanage.cpp] Knots keeps at most `DEFAULT_MAX_ORPHAN_TRANSACTIONS = 100`, expires orphan transactions after `20min`, sweeps every `5min`, keys orphan storage by wtxid, tracks announcers, indexes children by previous outpoint, erases peer-owned announcers on disconnect, and schedules children for reconsideration after parent acceptance. [VERIFIED: packages/bitcoin-knots/src/net_processing.h; VERIFIED: packages/bitcoin-knots/src/txorphanage.h; VERIFIED: packages/bitcoin-knots/src/txorphanage.cpp] Open Bitcoin should keep the externally relevant invariants while using deterministic caps and fake-time tests required by the Phase 102 context. [VERIFIED: .planning/phases/102-orphan-handling-and-admission-outcome-bridge/102-CONTEXT.md; ASSUMED]

**Primary recommendation:** Plan three coordinated workstreams: mempool outcome contract and tests, pure bounded orphan staging/reconsideration, and managed bridge integration that turns peer/local submissions into stable outcomes without direct peer-layer mempool mutation. [VERIFIED: .planning/REQUIREMENTS.md; VERIFIED: packages/open-bitcoin-node/src/network.rs; ASSUMED]

## Project Constraints (from AGENTS.md)

- Read `AGENTS.md`, `AGENTS.bright-builds.md`, `standards-overrides.md`, and relevant `standards/` pages before planning or implementation. [VERIFIED: AGENTS.md]
- Use `git submodule update --init --recursive` to materialize the pinned Knots baseline under `packages/bitcoin-knots` when the submodule is not present. [VERIFIED: AGENTS.md]
- Use `rust-toolchain.toml` as the Rust source of truth; the pinned toolchain is `1.94.1`. [VERIFIED: AGENTS.md; VERIFIED: rust-toolchain.toml]
- Use `bash scripts/verify.sh` as the repo-native verification contract for first-party code; `--fast` is for local iteration only. [VERIFIED: AGENTS.md]
- During UAT, prefer explicit repo-local Cargo and Bazel commands over only naming the installed `open-bitcoin` alias. [VERIFIED: AGENTS.md]
- Use Bun for repo-owned higher-level automation scripts; this repo has no `package.json` bootstrap step. [VERIFIED: AGENTS.md]
- Treat `docs/metrics/lines-of-code.md` as tracked generated output that may update during verification. [VERIFIED: AGENTS.md]
- Record intentional in-scope behavior differences from Bitcoin Knots under `docs/parity/`. [VERIFIED: AGENTS.md]
- Add parity breadcrumbs for new first-party Rust source or test files under `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests`. [VERIFIED: AGENTS.md; VERIFIED: scripts/check-parity-breadcrumbs.ts]
- Keep pure Bitcoin domain behavior in functional-core crates and isolate filesystem, process, network, terminal, RPC, service-manager, and durable-storage effects in shell adapters. [VERIFIED: AGENTS.md; VERIFIED: standards/core/architecture.md]
- Prefer `foo.rs` plus `foo/` child modules over `foo/mod.rs` in Rust. [VERIFIED: AGENTS.md; VERIFIED: standards/languages/rust.md]
- Do not use `unwrap()` in Rust code; prefer `?`, `let...else`, structured errors, and typed states. [VERIFIED: AGENTS.md; VERIFIED: standards/languages/rust.md]
- Use Arrange, Act, Assert in non-trivial tests and test behavior rather than implementation details. [VERIFIED: AGENTS.md; VERIFIED: standards/core/testing.md]
- Before creating commits in this Rust repo, run `cargo fmt --all`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo build --all-targets --all-features`, and `cargo test --all-features`, or use the repo-native verification contract if the workflow explicitly supersedes these checks. [VERIFIED: AGENTS.md; VERIFIED: AGENTS.bright-builds.md]

## Standard Stack

### Core

| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| `open-bitcoin-mempool` | `0.1.0` workspace crate [VERIFIED: packages/Cargo.toml; VERIFIED: packages/open-bitcoin-mempool/Cargo.toml] | Owns pure mempool admission, policy, replacement, trimming, and admission tests. [VERIFIED: packages/open-bitcoin-mempool/src/pool.rs; VERIFIED: packages/open-bitcoin-mempool/src/pool/tests.rs] | Put the stable outcome contract here so peer and local submissions share one typed boundary. [VERIFIED: .planning/phases/102-orphan-handling-and-admission-outcome-bridge/102-CONTEXT.md; ASSUMED] |
| `open-bitcoin-network` | `0.1.0` workspace crate [VERIFIED: packages/Cargo.toml; VERIFIED: packages/open-bitcoin-network/Cargo.toml] | Owns pure peer, inventory, transaction relay identity, and scheduler actions. [VERIFIED: packages/open-bitcoin-network/src/peer.rs; VERIFIED: packages/open-bitcoin-network/src/peer/transaction_relay.rs] | Add bounded orphan staging and parent-request decisions here or in a small child module because this crate already owns transaction download actions. [VERIFIED: packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs; ASSUMED] |
| `open-bitcoin-node` | `0.1.0` workspace crate [VERIFIED: packages/Cargo.toml; VERIFIED: packages/open-bitcoin-node/Cargo.toml] | Owns managed runtime state, transaction storage, `ManagedMempool`, and `ManagedPeerNetwork`. [VERIFIED: packages/open-bitcoin-node/src/mempool.rs; VERIFIED: packages/open-bitcoin-node/src/network.rs] | Integrate the bridge here because shell mutation belongs in node adapters, not in peer/socket code. [VERIFIED: standards/core/architecture.md; VERIFIED: packages/open-bitcoin-node/src/network.rs] |
| `open-bitcoin-primitives` / `open-bitcoin-codec` / `open-bitcoin-consensus` / `open-bitcoin-chainstate` | `0.1.0` workspace crates [VERIFIED: packages/Cargo.toml] | Provide txid/wtxid/outpoint/transaction types, encoding, validation, and chainstate snapshots. [VERIFIED: packages/open-bitcoin-mempool/Cargo.toml; VERIFIED: packages/open-bitcoin-network/Cargo.toml] | Reuse existing first-party Bitcoin domain types; production code must not introduce external Rust Bitcoin libraries. [VERIFIED: AGENTS.md] |

### Supporting

| Tool or Module | Version | Purpose | When to Use |
|----------------|---------|---------|-------------|
| Rust toolchain | `1.94.1` [VERIFIED: rust-toolchain.toml; VERIFIED: rustc --version] | Build, lint, format, and test Rust crates. [VERIFIED: AGENTS.md] | All first-party Rust changes. [VERIFIED: AGENTS.md] |
| Bazelisk / Bazel | Bazelisk `1.28.1`, Bazel `8.6.0` [VERIFIED: bazelisk version] | Top-level Bazel smoke build and workspace growth path. [VERIFIED: AGENTS.md] | Verification through `bash scripts/verify.sh`. [VERIFIED: AGENTS.md] |
| Bun | `1.3.9` [VERIFIED: .bun-version; VERIFIED: bun --version] | Repo-owned TypeScript automation. [VERIFIED: AGENTS.md] | Parity/checker script changes when needed. [VERIFIED: AGENTS.md] |
| `packages/bitcoin-knots` | `v29.3.knots20260210` submodule revision [VERIFIED: git submodule status packages/bitcoin-knots] | Baseline behavior anchors for orphanage, txdownload, net processing, and mempool acceptance. [VERIFIED: AGENTS.md; VERIFIED: packages/bitcoin-knots/src/txorphanage.h] | Parity breadcrumb and behavior research. [VERIFIED: docs/parity/source-breadcrumbs.json] |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Outcome enum in `open-bitcoin-mempool` [ASSUMED] | Outcome enum in `open-bitcoin-node` [ASSUMED] | Node-local outcomes would not naturally cover local and peer submissions through one shared contract, which conflicts with MEM-01 and D-09. [VERIFIED: .planning/REQUIREMENTS.md; VERIFIED: .planning/phases/102-orphan-handling-and-admission-outcome-bridge/102-CONTEXT.md] |
| Pure orphan staging in `open-bitcoin-network` [ASSUMED] | Orphan staging in `open-bitcoin-node` only [ASSUMED] | Node-only staging reduces crate coupling but risks mixing scheduler decisions with shell mutation; use a node-owned bridge only for mutation and persistence of runtime state. [VERIFIED: standards/core/architecture.md; VERIFIED: packages/open-bitcoin-node/src/network.rs] |
| Deterministic orphan eviction [ASSUMED] | Knots-style random eviction [VERIFIED: packages/bitcoin-knots/src/txorphanage.cpp] | Knots uses randomized eviction after expiry sweeps, while Phase 102 requires deterministic cap and expiry tests; deterministic eviction is better for Open Bitcoin's current test contract. [VERIFIED: .planning/phases/102-orphan-handling-and-admission-outcome-bridge/102-CONTEXT.md; ASSUMED] |

**Installation:**

No new npm, Cargo, or system dependency is required for the recommended Phase 102 implementation. [VERIFIED: packages/Cargo.toml; VERIFIED: .planning/phases/102-orphan-handling-and-admission-outcome-bridge/102-CONTEXT.md; ASSUMED]

```bash
# Source anchors only, if the submodule is not already materialized.
git submodule update --init --recursive
```

**Version verification:** `npm view` is not applicable because the recommended stack adds no npm package. [VERIFIED: AGENTS.md; VERIFIED: packages/Cargo.toml] First-party crate version and edition are pinned by `packages/Cargo.toml` as workspace `version = "0.1.0"` and `edition = "2024"`. [VERIFIED: packages/Cargo.toml]

## Architecture Patterns

### Recommended Project Structure

```text
packages/open-bitcoin-mempool/src/
├── outcome.rs                       # Stable admission outcome contract. [ASSUMED]
├── pool.rs                          # Existing admission core remains the mutation boundary. [VERIFIED: packages/open-bitcoin-mempool/src/pool.rs]
└── pool/
    └── tests/
        └── outcome_cases.rs         # Focused outcome/no-partial-mutation tests if split from the large existing tests file. [ASSUMED]

packages/open-bitcoin-network/src/peer/
├── transaction_relay.rs             # Existing TxRelayId/TxDownloadAction vocabulary. [VERIFIED: packages/open-bitcoin-network/src/peer/transaction_relay.rs]
└── transaction_relay/
    ├── orphanage.rs                 # Pure bounded orphan staging and reconsideration decisions. [ASSUMED]
    └── tests/orphanage_cases.rs     # Fake-time cap, expiry, parent request, reconsideration tests. [ASSUMED]

packages/open-bitcoin-node/src/
├── mempool.rs                       # ManagedMempool returns stable outcomes. [VERIFIED: packages/open-bitcoin-node/src/mempool.rs]
└── network/
    └── admission_bridge.rs          # Shell bridge from PeerAction/local submission to mempool outcomes and orphan actions. [ASSUMED]
```

Use `foo.rs` plus `foo/` child modules for any new Rust modules. [VERIFIED: standards/languages/rust.md]

### Pattern 1: Stable Outcome Boundary

**What:** Map `AdmissionResult` and `MempoolError` into a stable enum before returning to peer or local submission callers. [VERIFIED: packages/open-bitcoin-mempool/src/types.rs; VERIFIED: packages/open-bitcoin-mempool/src/error.rs]

**When to use:** Use for every local and peer transaction submission path, including missing-input, duplicate, replacement, candidate-evicted, expiry, and orphan-eviction cases. [VERIFIED: .planning/REQUIREMENTS.md; VERIFIED: .planning/phases/102-orphan-handling-and-admission-outcome-bridge/102-CONTEXT.md]

**Example:**

```rust
// Source: proposed from Phase 102 context and current mempool API.
// [VERIFIED: packages/open-bitcoin-mempool/src/error.rs]
// [VERIFIED: packages/open-bitcoin-mempool/src/types.rs]
// [ASSUMED: exact names]
pub enum MempoolOutcome {
    Accepted { txid: Txid, evicted: Vec<Txid> },
    Replaced { txid: Txid, replaced: Vec<Txid>, evicted: Vec<Txid> },
    Duplicate { txid: Txid },
    Rejected { txid: Txid, category: RejectionCategory },
    Orphaned { txid: Txid, wtxid: Wtxid, missing_parents: Vec<Txid> },
    Evicted { txid: Txid },
    Expired { txid: Txid },
}
```

### Pattern 2: Bounded Pure Orphanage With Injected Time

**What:** Keep orphan state bounded by total count, per-peer count, expiry, and bounded reconsideration work; return typed actions instead of writing sockets. [VERIFIED: .planning/phases/102-orphan-handling-and-admission-outcome-bridge/102-CONTEXT.md; VERIFIED: packages/bitcoin-knots/src/txorphanage.cpp]

**When to use:** Use after `MempoolOutcome::Orphaned` for peer transactions, on parent acceptance, on scheduler expiry ticks, and on peer disconnect cleanup. [VERIFIED: packages/bitcoin-knots/src/node/txdownloadman_impl.cpp; VERIFIED: packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs]

**Example:**

```rust
// Source: proposed from Knots TxOrphanage behavior and Phase 102 deterministic test requirement.
// [VERIFIED: packages/bitcoin-knots/src/txorphanage.h]
// [VERIFIED: packages/bitcoin-knots/src/txorphanage.cpp]
// [VERIFIED: .planning/phases/102-orphan-handling-and-admission-outcome-bridge/102-CONTEXT.md]
// [ASSUMED: exact names]
pub struct OrphanPolicy {
    pub max_total_orphans: usize,
    pub max_orphans_per_peer: usize,
    pub ttl: Duration,
    pub max_reconsideration_per_parent: usize,
}

pub enum OrphanAction {
    RequestParent { peer_id: PeerId, parent: TxRelayId },
    Reconsider { peer_id: PeerId, transaction: Transaction },
    Evicted { txid: Txid },
    Expired { txid: Txid },
}
```

### Pattern 3: Managed Bridge as the Only Mutation Shell

**What:** Let `PeerManager` emit `PeerAction::ReceivedTransaction`, let the managed node bridge submit the transaction, and let the bridge update mempool, storage, orphanage, and wire-request actions based on stable outcomes. [VERIFIED: packages/open-bitcoin-network/src/peer.rs; VERIFIED: packages/open-bitcoin-node/src/network.rs]

**When to use:** Use inside `ManagedPeerNetwork::process_actions` or a child module called by it. [VERIFIED: .planning/phases/102-orphan-handling-and-admission-outcome-bridge/102-CONTEXT.md; VERIFIED: packages/open-bitcoin-node/src/network.rs]

**Example:**

```rust
// Source: proposed from current ManagedPeerNetwork::process_actions seam.
// [VERIFIED: packages/open-bitcoin-node/src/network.rs]
// [ASSUMED: exact names]
fn handle_peer_transaction(&mut self, peer_id: PeerId, transaction: Transaction) -> Result<Vec<WireAction>, ManagedNetworkError> {
    let outcome = self.mempool.submit_transaction_outcome(transaction.clone())?;
    let actions = self.admission_bridge.apply_peer_outcome(peer_id, transaction, outcome)?;
    Ok(actions)
}
```

### Anti-Patterns to Avoid

- **Display-string branching:** Do not parse `MempoolError` display output to detect missing inputs, duplicates, RBF failures, or evictions. [VERIFIED: packages/open-bitcoin-mempool/src/error.rs; VERIFIED: .planning/phases/102-orphan-handling-and-admission-outcome-bridge/102-CONTEXT.md]
- **Peer/socket mempool mutation:** Do not call `Mempool` or `ManagedMempool` from `PeerManager` or socket-facing message handlers. [VERIFIED: .planning/phases/102-orphan-handling-and-admission-outcome-bridge/102-CONTEXT.md; VERIFIED: packages/open-bitcoin-network/src/peer.rs]
- **Unbounded recursive reconsideration:** Do not recursively walk orphan chains without a configured work cap. [VERIFIED: .planning/phases/102-orphan-handling-and-admission-outcome-bridge/102-CONTEXT.md; VERIFIED: packages/bitcoin-knots/src/txorphanage.cpp]
- **Wall-clock sleeps in tests:** Do not sleep to test expiry; pass a fake or explicit time into expiry/reconsideration functions. [VERIFIED: .planning/phases/102-orphan-handling-and-admission-outcome-bridge/102-CONTEXT.md; VERIFIED: standards/core/testing.md]
- **Raw transaction evidence:** Do not expose transaction hex, txids, wtxids, peer endpoints, permission strings, class names, credentials, or dynamic labels in shared evidence. [VERIFIED: .planning/phases/102-orphan-handling-and-admission-outcome-bridge/102-CONTEXT.md]

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Transaction identity and inventory conversion | Custom txid/wtxid or inventory parsing. [VERIFIED: packages/open-bitcoin-network/src/peer/transaction_relay.rs] | Existing `TxRelayId` and inventory helpers. [VERIFIED: packages/open-bitcoin-network/src/peer/transaction_relay.rs] | Phase 101 already encoded txid/wtxid peer-mode behavior and mismatch suppression. [VERIFIED: packages/open-bitcoin-network/src/peer/transaction_relay.rs; VERIFIED: packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs] |
| Download request timing and caps | Direct getdata socket writes from orphan code. [VERIFIED: .planning/phases/102-orphan-handling-and-admission-outcome-bridge/102-CONTEXT.md] | Phase 101 scheduler actions and node action translation. [VERIFIED: packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs; VERIFIED: packages/open-bitcoin-node/src/network/action_translation.rs] | Existing scheduler enforces in-flight caps, duplicate suppression, delay windows, and expiry cleanup. [VERIFIED: packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs] |
| Admission policy | Parallel fee, standardness, RBF, or ancestor logic in the bridge. [VERIFIED: packages/open-bitcoin-mempool/src/pool.rs] | `Mempool::accept_transaction` plus stable outcome mapping. [VERIFIED: packages/open-bitcoin-mempool/src/pool.rs] | The mempool core already implements standardness, fee floor, RBF, graph recomputation, trimming, and candidate eviction. [VERIFIED: packages/open-bitcoin-mempool/src/pool.rs] |
| Missing-parent detection | String matching on `MempoolError` display text. [VERIFIED: packages/open-bitcoin-mempool/src/error.rs] | Typed `MissingInput` mapping plus an explicit missing-parent collection helper if multiple parent requests are needed. [VERIFIED: packages/open-bitcoin-mempool/src/error.rs; ASSUMED] | Parent requests need structured parent identities and stable outcome labels. [VERIFIED: .planning/REQUIREMENTS.md; VERIFIED: .planning/phases/102-orphan-handling-and-admission-outcome-bridge/102-CONTEXT.md] |
| Orphan expiry tests | Sleeping or relying on public network timing. [VERIFIED: .planning/phases/102-orphan-handling-and-admission-outcome-bridge/102-CONTEXT.md] | Injected `now` values and deterministic test fixtures. [VERIFIED: packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs; VERIFIED: standards/core/testing.md] | Existing scheduler tests already use deterministic expiration inputs instead of sleeps. [VERIFIED: packages/open-bitcoin-network/src/peer/transaction_relay/tests/scheduler_cases.rs] |
| Observability labels | Dynamic labels containing txids, wtxids, endpoints, or permission names. [VERIFIED: .planning/phases/102-orphan-handling-and-admission-outcome-bridge/102-CONTEXT.md] | Fixed labels `orphaned`, `parent_requested`, `orphan_evicted`, `orphan_expired`, `orphan_reconsidered`. [VERIFIED: .planning/phases/102-orphan-handling-and-admission-outcome-bridge/102-CONTEXT.md] | Phase 105 needs low-cardinality aggregation and safe evidence. [VERIFIED: .planning/REQUIREMENTS.md; VERIFIED: .planning/phases/102-orphan-handling-and-admission-outcome-bridge/102-CONTEXT.md] |

**Key insight:** The planner should avoid adding a second admission system; Phase 102 is about making the existing admission and download systems communicate through typed outcomes and bounded state. [VERIFIED: packages/open-bitcoin-mempool/src/pool.rs; VERIFIED: packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs; ASSUMED]

## Common Pitfalls

### Pitfall 1: Treating Missing Inputs As Generic Rejection

**What goes wrong:** A peer transaction with a missing parent gets recorded only as a generic `MempoolError::MissingInput`, and the bridge cannot request parents or reconsider the child later. [VERIFIED: packages/open-bitcoin-mempool/src/error.rs; VERIFIED: .planning/REQUIREMENTS.md]

**Why it happens:** Current `Mempool::accept_transaction` returns `Result<AdmissionResult, MempoolError>` and `MissingInput` currently contains one `OutPoint`. [VERIFIED: packages/open-bitcoin-mempool/src/pool.rs; VERIFIED: packages/open-bitcoin-mempool/src/error.rs]

**How to avoid:** Map missing inputs into `MempoolOutcome::Orphaned` and collect unique missing parent txids for parent request actions. [VERIFIED: .planning/phases/102-orphan-handling-and-admission-outcome-bridge/102-CONTEXT.md; ASSUMED]

**Warning signs:** Tests assert on error display text or only the first missing input when multiple parents are absent. [VERIFIED: packages/open-bitcoin-mempool/src/error.rs; ASSUMED]

### Pitfall 2: Letting Orphan Parent Requests Bypass Phase 101 Caps

**What goes wrong:** Orphan handling emits direct getdata writes or new request counters outside `TxDownloadScheduler`, bypassing existing in-flight and pressure checks. [VERIFIED: packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs; VERIFIED: packages/open-bitcoin-node/src/network/action_translation.rs]

**Why it happens:** Parent requests are a new trigger source, but they are still transaction download requests. [VERIFIED: packages/bitcoin-knots/src/node/txdownloadman_impl.cpp; VERIFIED: .planning/phases/102-orphan-handling-and-admission-outcome-bridge/102-CONTEXT.md]

**How to avoid:** Represent parent fetches as typed scheduler actions or typed orphan actions that translate through the same `TxRelayId`/getdata path and resource governance checks. [VERIFIED: packages/open-bitcoin-network/src/peer/transaction_relay.rs; VERIFIED: packages/open-bitcoin-network/src/resource.rs; ASSUMED]

**Warning signs:** New code creates `WireNetworkMessage::GetData` from peer/orphan code instead of from node action translation. [VERIFIED: packages/open-bitcoin-node/src/network/action_translation.rs; ASSUMED]

### Pitfall 3: Partial Mempool Mutation on Rejection

**What goes wrong:** Failed nonstandard, low-fee, replacement, or limit-exceeded candidates leave entries, spent-outpoint indexes, virtual-size totals, or replacement state changed. [VERIFIED: .planning/phases/102-orphan-handling-and-admission-outcome-bridge/102-CONTEXT.md]

**Why it happens:** Admission code mutates state before all validation and trimming checks pass. [VERIFIED: packages/open-bitcoin-mempool/src/pool.rs]

**How to avoid:** Preserve the current prospective-state pattern and add snapshot tests around each rejection path. [VERIFIED: packages/open-bitcoin-mempool/src/pool.rs; VERIFIED: packages/open-bitcoin-mempool/src/pool/tests.rs]

**Warning signs:** New tests inspect only the returned error/outcome and do not compare pool entry sets, spent indexes, and totals after failure. [VERIFIED: packages/open-bitcoin-mempool/src/pool/tests.rs; ASSUMED]

### Pitfall 4: Unbounded Reconsideration Chains

**What goes wrong:** Accepting one parent triggers an unbounded recursive walk of orphan descendants, which can monopolize the runtime under adversarial chains. [VERIFIED: .planning/phases/102-orphan-handling-and-admission-outcome-bridge/102-CONTEXT.md]

**Why it happens:** Orphan children can themselves be parents of other orphans. [VERIFIED: packages/bitcoin-knots/test/functional/p2p_orphan_handling.py]

**How to avoid:** Return a bounded list of reconsideration candidates per accepted parent and let subsequent accepted children trigger later bounded passes. [VERIFIED: packages/bitcoin-knots/src/txorphanage.cpp; ASSUMED]

**Warning signs:** A function called from parent acceptance loops until the orphanage is empty or has no configured max work count. [VERIFIED: .planning/phases/102-orphan-handling-and-admission-outcome-bridge/102-CONTEXT.md; ASSUMED]

### Pitfall 5: Expanding Scope Into Package Relay

**What goes wrong:** Orphan handling turns into package relay, CPFP package acceptance, compact-block reconstruction, or public relay serving. [VERIFIED: .planning/phases/102-orphan-handling-and-admission-outcome-bridge/102-CONTEXT.md]

**Why it happens:** Knots orphan handling has adjacent package and same-peer child helpers, but Phase 102 explicitly excludes package relay and cluster mempool behavior. [VERIFIED: packages/bitcoin-knots/src/txorphanage.h; VERIFIED: .planning/phases/102-orphan-handling-and-admission-outcome-bridge/102-CONTEXT.md]

**How to avoid:** Implement single-transaction staging, parent requests, reconsideration, expiry, eviction, and outcomes only. [VERIFIED: .planning/REQUIREMENTS.md]

**Warning signs:** Tests start asserting package acceptance, relay fanout, or RPC/orphan introspection. [VERIFIED: .planning/REQUIREMENTS.md; VERIFIED: .planning/phases/102-orphan-handling-and-admission-outcome-bridge/102-CONTEXT.md]

## Code Examples

Verified patterns from local sources and proposed Phase 102 shapes:

### Outcome Mapping From Current Admission Result

```rust
// Source: current AdmissionResult and MempoolError surfaces.
// [VERIFIED: packages/open-bitcoin-mempool/src/types.rs]
// [VERIFIED: packages/open-bitcoin-mempool/src/error.rs]
// [ASSUMED: exact function and enum names]
fn outcome_from_result(txid: Txid, result: Result<AdmissionResult, MempoolError>) -> MempoolOutcome {
    match result {
        Ok(admitted) if !admitted.replaced.is_empty() => MempoolOutcome::Replaced {
            txid: admitted.accepted,
            replaced: admitted.replaced,
            evicted: admitted.evicted,
        },
        Ok(admitted) => MempoolOutcome::Accepted {
            txid: admitted.accepted,
            evicted: admitted.evicted,
        },
        Err(MempoolError::DuplicateTransaction { txid }) => MempoolOutcome::Duplicate { txid },
        Err(MempoolError::MissingInput { outpoint: _ }) => {
            MempoolOutcome::Orphaned { txid, wtxid: txid.into(), missing_parents: Vec::new() }
        }
        Err(MempoolError::CandidateEvicted { txid }) => MempoolOutcome::Evicted { txid },
        Err(error) => MempoolOutcome::Rejected {
            txid,
            category: RejectionCategory::from(error),
        },
    }
}
```

The example shows the required mapping shape but should not be copied literally because `Wtxid` conversion and missing-parent collection must use existing primitives correctly. [VERIFIED: packages/open-bitcoin-primitives; ASSUMED]

### Parent Request Action Shape

```rust
// Source: current TxRelayId request vocabulary.
// [VERIFIED: packages/open-bitcoin-network/src/peer/transaction_relay.rs]
// [ASSUMED: exact variant name]
TxDownloadAction::RequestParent {
    peer_id,
    relay_id: TxRelayId::Txid(parent_txid),
}
```

If the planner avoids extending `TxDownloadAction`, use an `OrphanAction::RequestParent` that is translated by the same node action translation code and pressure checks. [VERIFIED: packages/open-bitcoin-node/src/network/action_translation.rs; ASSUMED]

### No-Partial-Mutation Test Snapshot

```rust
// Source: current mempool internals are tested from pool/tests.rs.
// [VERIFIED: packages/open-bitcoin-mempool/src/pool.rs]
// [VERIFIED: packages/open-bitcoin-mempool/src/pool/tests.rs]
// [ASSUMED: helper names]
let before = MempoolSnapshot::capture(&mempool);

let outcome = mempool.accept_transaction_outcome(candidate, chainstate.snapshot());

assert!(matches!(outcome, MempoolOutcome::Rejected { .. }));
assert_eq!(before, MempoolSnapshot::capture(&mempool));
```

The snapshot should cover accepted entry keys, spent-outpoint indexes, total virtual size, and replacement-related entry presence. [VERIFIED: packages/open-bitcoin-mempool/src/pool.rs; VERIFIED: .planning/phases/102-orphan-handling-and-admission-outcome-bridge/102-CONTEXT.md]

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Caller sees `Result<AdmissionResult, MempoolError>` and must interpret variants. [VERIFIED: packages/open-bitcoin-node/src/network.rs; VERIFIED: packages/open-bitcoin-mempool/src/error.rs] | Caller should see one stable outcome enum for accepted/rejected/duplicate/replaced/orphaned/evicted/expired. [VERIFIED: .planning/REQUIREMENTS.md] | Phase 102. [VERIFIED: .planning/ROADMAP.md] | Enables local and peer submission paths to share behavior and later RPC/metrics/support surfaces. [VERIFIED: .planning/phases/102-orphan-handling-and-admission-outcome-bridge/102-CONTEXT.md] |
| Peer `ReceivedTransaction` currently reaches `ManagedMempool::submit_transaction` in `ManagedPeerNetwork::process_actions`. [VERIFIED: packages/open-bitcoin-node/src/network.rs] | Peer transaction should pass through an admission bridge that returns typed outcomes and orphan actions. [VERIFIED: .planning/phases/102-orphan-handling-and-admission-outcome-bridge/102-CONTEXT.md; ASSUMED] | Phase 102. [VERIFIED: .planning/ROADMAP.md] | Keeps socket/peer code free of mempool mutation while preserving a single shell integration point. [VERIFIED: standards/core/architecture.md; VERIFIED: packages/open-bitcoin-node/src/network.rs] |
| Knots orphanage uses randomized eviction when over cap. [VERIFIED: packages/bitcoin-knots/src/txorphanage.cpp] | Open Bitcoin Phase 102 should use deterministic cap eviction for tests. [VERIFIED: .planning/phases/102-orphan-handling-and-admission-outcome-bridge/102-CONTEXT.md; ASSUMED] | Phase 102. [VERIFIED: .planning/ROADMAP.md] | Improves deterministic local verification while preserving bounded-resource behavior. [VERIFIED: .planning/phases/102-orphan-handling-and-admission-outcome-bridge/102-CONTEXT.md; ASSUMED] |
| Knots txdownload/orphanage integrates missing-input handling with mempool acceptance and request state. [VERIFIED: packages/bitcoin-knots/src/node/txdownloadman_impl.cpp] | Open Bitcoin should split this into pure network/orphan decisions, pure mempool outcomes, and node-shell mutation. [VERIFIED: standards/core/architecture.md; VERIFIED: packages/open-bitcoin-network/src/peer/transaction_relay.rs; VERIFIED: packages/open-bitcoin-mempool/src/pool.rs; VERIFIED: packages/open-bitcoin-node/src/network.rs] | Phase 102. [VERIFIED: .planning/ROADMAP.md] | Matches repo architecture while retaining parity anchors. [VERIFIED: AGENTS.md; VERIFIED: standards/core/architecture.md] |

**Deprecated/outdated:**

- Treating `MempoolError` display text as a caller API is not acceptable for Phase 102. [VERIFIED: .planning/phases/102-orphan-handling-and-admission-outcome-bridge/102-CONTEXT.md; VERIFIED: packages/open-bitcoin-mempool/src/error.rs]
- Adding direct socket writes from orphan code is not acceptable for Phase 102. [VERIFIED: .planning/phases/102-orphan-handling-and-admission-outcome-bridge/102-CONTEXT.md; VERIFIED: packages/open-bitcoin-node/src/network/action_translation.rs]
- Adding package-relay or cluster-mempool behavior is out of scope for Phase 102. [VERIFIED: .planning/phases/102-orphan-handling-and-admission-outcome-bridge/102-CONTEXT.md]

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | Exact names such as `MempoolOutcome`, `OrphanPolicy`, `OrphanAction`, and `admission_bridge.rs` are recommendations, not locked API names. [ASSUMED] | Summary, Architecture Patterns, Code Examples | Low: planner can choose different names while preserving the researched boundaries. |
| A2 | Deterministic orphan eviction is preferred over Knots randomized over-cap eviction for Open Bitcoin Phase 102 tests. [ASSUMED] | Summary, Standard Stack, State of the Art | Medium: if parity requires randomized eviction now, the plan must add deterministic RNG injection instead of oldest/lexicographic eviction. |
| A3 | A pure orphan staging module in `open-bitcoin-network` is the cleanest default location. [ASSUMED] | Standard Stack, Architecture Patterns | Medium: planner may decide a mempool-adjacent helper is better if missing-parent collection needs deeper mempool internals. |
| A4 | Missing-parent collection should return all unique parent txids, not only the first missing `OutPoint`. [ASSUMED] | Don't Hand-Roll, Common Pitfalls, Code Examples | Medium: parent request tests for multiple missing parents may fail or become under-specified if only one missing parent is exposed. |
| A5 | Replacement/eviction should update managed in-memory transaction indexes only to the degree needed for current outcome and duplicate/mempool-known behavior, not for Phase 104 relay serving. [ASSUMED] | Open Questions | Medium: stale indexes could affect duplicate suppression or later serving if the bridge keeps evicted/replaced txs as available. |

## Open Questions (RESOLVED)

1. **[RESOLVED] Should `MempoolError::MissingInput` stay singular or should the new outcome collect all missing parents?** [VERIFIED: packages/open-bitcoin-mempool/src/error.rs; ASSUMED]
   - What we know: The current error contains one `OutPoint`, while Phase 102 needs parent request behavior and Knots can reason about multiple missing parents. [VERIFIED: packages/open-bitcoin-mempool/src/error.rs; VERIFIED: packages/bitcoin-knots/src/node/txdownloadman_impl.cpp]
   - What's unclear: Whether to change the internal error, add a helper, or collect missing parents in the outcome wrapper. [ASSUMED]
   - Resolution: Collect all unique missing parent txids in the new outcome/orphan path while preserving existing `MempoolError::MissingInput` compatibility where useful. [ASSUMED: chosen planning answer]
   - Recommendation: Add a pure mempool helper or outcome-building path that returns all unique missing parent txids while keeping existing error compatibility if that reduces churn. [ASSUMED]

2. **[RESOLVED] Should the managed transaction store remove replaced and evicted transactions in Phase 102?** [VERIFIED: packages/open-bitcoin-node/src/network.rs; ASSUMED]
   - What we know: `AdmissionResult` already returns `replaced` and `evicted` txids, and `ManagedPeerNetwork` stores accepted transactions by txid/wtxid. [VERIFIED: packages/open-bitcoin-mempool/src/types.rs; VERIFIED: packages/open-bitcoin-node/src/network.rs]
   - What's unclear: Relay serving is Phase 104, but stale store indexes can still influence duplicate suppression and mempool-known facts. [VERIFIED: .planning/REQUIREMENTS.md; ASSUMED]
   - Resolution: Remove or clean only the managed in-memory transaction indexes required for Phase 102 duplicate/mempool-known behavior when replaced or evicted outcomes occur, leaving relay-serving cache lifecycle to Phase 104. [ASSUMED: chosen planning answer]
   - Recommendation: Update only the indexes used by Phase 102 duplicate/mempool-known behavior and document that serving/fanout remains deferred. [ASSUMED]

3. **[RESOLVED] What exact orphan caps should the first implementation use?** [VERIFIED: .planning/phases/102-orphan-handling-and-admission-outcome-bridge/102-CONTEXT.md; VERIFIED: packages/bitcoin-knots/src/net_processing.h]
   - What we know: Knots defaults to 100 orphan transactions, a 20 minute orphan expiry, and a 5 minute sweep interval. [VERIFIED: packages/bitcoin-knots/src/net_processing.h; VERIFIED: packages/bitcoin-knots/src/txorphanage.h]
   - What's unclear: Phase 102 leaves exact constant names and values to the planner. [VERIFIED: .planning/phases/102-orphan-handling-and-admission-outcome-bridge/102-CONTEXT.md]
   - Resolution: Use Knots-derived production defaults with test policies using tiny caps and short injected durations. [ASSUMED: chosen planning answer]
   - Recommendation: Use Knots-derived production defaults with test policies that set tiny caps and short injected durations. [VERIFIED: packages/bitcoin-knots/src/net_processing.h; ASSUMED]

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|-------------|-----------|---------|----------|
| Rust toolchain | Rust implementation, tests, clippy, build. [VERIFIED: AGENTS.md] | Yes [VERIFIED: rustc --version] | `rustc 1.94.1`; `cargo 1.94.1` [VERIFIED: rustc --version; VERIFIED: cargo --version] | None needed. [VERIFIED: rust-toolchain.toml] |
| Rustfmt and Clippy | Formatting and lint gate. [VERIFIED: rust-toolchain.toml; VERIFIED: AGENTS.md] | Yes [VERIFIED: rust-toolchain.toml] | Components listed in `rust-toolchain.toml`. [VERIFIED: rust-toolchain.toml] | None needed. [VERIFIED: rust-toolchain.toml] |
| Bazelisk / Bazel | Repo-native verification smoke build. [VERIFIED: AGENTS.md] | Yes [VERIFIED: bazelisk version] | Bazelisk `1.28.1`; Bazel `8.6.0`. [VERIFIED: bazelisk version] | None needed. [VERIFIED: AGENTS.md] |
| Bun | TypeScript parity/checker scripts if touched. [VERIFIED: AGENTS.md] | Yes [VERIFIED: bun --version] | `1.3.9` [VERIFIED: .bun-version; VERIFIED: bun --version] | Avoid TS checker changes if not needed. [ASSUMED] |
| `packages/bitcoin-knots` submodule | Parity anchors and breadcrumbs. [VERIFIED: AGENTS.md] | Yes [VERIFIED: git submodule status packages/bitcoin-knots] | `a9aee730466ac67d35a3c03ee24676be5e045878` at `v29.3.knots20260210`. [VERIFIED: git submodule status packages/bitcoin-knots] | Run `git submodule update --init --recursive` if missing. [VERIFIED: AGENTS.md] |
| `bash scripts/verify.sh` | Final verification. [VERIFIED: AGENTS.md] | Yes [VERIFIED: AGENTS.md; VERIFIED: repository file listing] | Repo script. [VERIFIED: AGENTS.md] | Use targeted Cargo tests only for iteration; default final gate remains `bash scripts/verify.sh`. [VERIFIED: AGENTS.md] |

**Missing dependencies with no fallback:** None found during research. [VERIFIED: rustc --version; VERIFIED: cargo --version; VERIFIED: bazelisk version; VERIFIED: bun --version; VERIFIED: git submodule status packages/bitcoin-knots]

**Missing dependencies with fallback:** None found during research. [VERIFIED: rustc --version; VERIFIED: cargo --version; VERIFIED: bazelisk version; VERIFIED: bun --version; VERIFIED: git submodule status packages/bitcoin-knots]

## Security Domain

Phase 102 remains local deterministic transaction relay and mempool-admission work; it does not add authentication, sessions, RPC exposure, public relay defaults, or production service claims. [VERIFIED: .planning/REQUIREMENTS.md; VERIFIED: .planning/phases/102-orphan-handling-and-admission-outcome-bridge/102-CONTEXT.md] The ASVS mapping below is a phase-scope security assessment, not a new compliance claim. [ASSUMED]

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|------------------|
| V2 Authentication | No for Phase 102. [VERIFIED: .planning/REQUIREMENTS.md] | No auth surface is added; keep this out of scope. [VERIFIED: .planning/phases/102-orphan-handling-and-admission-outcome-bridge/102-CONTEXT.md] |
| V3 Session Management | No for Phase 102. [VERIFIED: .planning/REQUIREMENTS.md] | No session surface is added. [VERIFIED: .planning/phases/102-orphan-handling-and-admission-outcome-bridge/102-CONTEXT.md] |
| V4 Access Control | Yes for relay eligibility and peer-bound request behavior. [VERIFIED: packages/open-bitcoin-network/src/relay.rs; VERIFIED: packages/open-bitcoin-network/src/resource.rs] | Preserve relay activation, permission-effect, and request-governance checks; do not let orphan handling bypass them. [VERIFIED: packages/open-bitcoin-network/src/relay.rs; VERIFIED: packages/open-bitcoin-network/src/resource.rs] |
| V5 Input Validation | Yes for peer transaction data and mempool policy. [VERIFIED: packages/open-bitcoin-network/src/peer.rs; VERIFIED: packages/open-bitcoin-mempool/src/pool.rs] | Use existing transaction decoding, consensus validation, policy checks, and typed outcomes. [VERIFIED: packages/open-bitcoin-codec; VERIFIED: packages/open-bitcoin-consensus; VERIFIED: packages/open-bitcoin-mempool/src/pool.rs] |
| V6 Cryptography | Yes only through existing transaction identity and validation primitives. [VERIFIED: packages/open-bitcoin-primitives; VERIFIED: packages/open-bitcoin-consensus] | Do not introduce custom hashing or signature logic in orphan/admission bridge code. [VERIFIED: AGENTS.md; VERIFIED: packages/open-bitcoin-primitives; VERIFIED: packages/open-bitcoin-consensus] |

### Known Threat Patterns for Transaction Download and Mempool Admission

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Orphan memory exhaustion through missing-parent bursts. [VERIFIED: packages/bitcoin-knots/src/txorphanage.h; VERIFIED: .planning/REQUIREMENTS.md] | Denial of Service [ASSUMED] | Bounded total/per-peer orphan caps, expiry, deterministic eviction, and no raw socket writes from orphan code. [VERIFIED: .planning/phases/102-orphan-handling-and-admission-outcome-bridge/102-CONTEXT.md] |
| Parent request amplification. [VERIFIED: packages/bitcoin-knots/src/node/txdownloadman_impl.cpp; VERIFIED: packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs] | Denial of Service [ASSUMED] | Route parent requests through `TxRelayId`, request caps, already-have/recent-reject/in-flight suppression, and resource pressure checks. [VERIFIED: packages/open-bitcoin-network/src/peer/transaction_relay.rs; VERIFIED: packages/open-bitcoin-network/src/resource.rs] |
| Txid/wtxid identity confusion. [VERIFIED: packages/bitcoin-knots/src/node/txdownloadman_impl.cpp; VERIFIED: packages/open-bitcoin-network/src/peer/transaction_relay.rs] | Spoofing / Tampering [ASSUMED] | Reuse Phase 101 `TxRelayId` and identity mismatch suppression; key orphan storage by stable transaction identity while requesting missing parents by txid. [VERIFIED: packages/open-bitcoin-network/src/peer/transaction_relay.rs; VERIFIED: packages/bitcoin-knots/src/txorphanage.cpp] |
| Partial mempool mutation on rejection. [VERIFIED: packages/open-bitcoin-mempool/src/pool.rs; VERIFIED: .planning/phases/102-orphan-handling-and-admission-outcome-bridge/102-CONTEXT.md] | Tampering [ASSUMED] | Preserve prospective-state admission and add snapshot tests for entries, indexes, totals, and replacement state. [VERIFIED: packages/open-bitcoin-mempool/src/pool.rs; VERIFIED: packages/open-bitcoin-mempool/src/pool/tests.rs] |
| Evidence leakage through labels or logs. [VERIFIED: .planning/phases/102-orphan-handling-and-admission-outcome-bridge/102-CONTEXT.md] | Information Disclosure [ASSUMED] | Use fixed low-cardinality labels and avoid raw tx material, peer endpoints, permissions, class names, credentials, and dynamic labels. [VERIFIED: .planning/phases/102-orphan-handling-and-admission-outcome-bridge/102-CONTEXT.md] |
| Runtime monopoly through recursive orphan reconsideration. [VERIFIED: packages/bitcoin-knots/test/functional/p2p_orphan_handling.py; VERIFIED: .planning/phases/102-orphan-handling-and-admission-outcome-bridge/102-CONTEXT.md] | Denial of Service [ASSUMED] | Bound reconsideration work per parent and use explicit queues instead of recursive unbounded walks. [VERIFIED: .planning/phases/102-orphan-handling-and-admission-outcome-bridge/102-CONTEXT.md; ASSUMED] |

## Sources

### Primary (HIGH confidence)

- `.planning/phases/102-orphan-handling-and-admission-outcome-bridge/102-CONTEXT.md` - user decisions, scope, deferred boundaries, canonical refs, and implementation constraints. [VERIFIED: mandatory initial read]
- `.planning/REQUIREMENTS.md` - DL-03, DL-04, DL-05, MEM-01, and MEM-02 requirement text. [VERIFIED: mandatory initial read]
- `.planning/STATE.md` - current milestone state, Phase 101 completion, deterministic verification caveats, and UAT command reminder. [VERIFIED: mandatory initial read]
- `AGENTS.md`, `AGENTS.bright-builds.md`, `standards-overrides.md`, `standards/core/*.md`, and `standards/languages/rust.md` - project and Bright Builds constraints. [VERIFIED: local file reads]
- `packages/open-bitcoin-network/src/peer/transaction_relay.rs` and `packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs` - Phase 101 transaction relay identity and scheduler boundary. [VERIFIED: local code reads]
- `packages/open-bitcoin-network/src/peer.rs`, `packages/open-bitcoin-network/src/peer/inventory_state.rs`, and `packages/open-bitcoin-network/src/resource.rs` - peer action, transaction receive, and resource-governance behavior. [VERIFIED: local code reads]
- `packages/open-bitcoin-mempool/src/error.rs`, `packages/open-bitcoin-mempool/src/types.rs`, `packages/open-bitcoin-mempool/src/pool.rs`, and `packages/open-bitcoin-mempool/src/pool/tests.rs` - current admission result, error, policy, replacement, trimming, and tests. [VERIFIED: local code reads]
- `packages/open-bitcoin-node/src/mempool.rs`, `packages/open-bitcoin-node/src/network.rs`, and `packages/open-bitcoin-node/src/network/action_translation.rs` - managed mempool and peer-to-mempool bridge seam. [VERIFIED: local code reads]
- `packages/bitcoin-knots/src/txorphanage.h`, `packages/bitcoin-knots/src/txorphanage.cpp`, `packages/bitcoin-knots/src/node/txdownloadman.h`, `packages/bitcoin-knots/src/node/txdownloadman_impl.cpp`, `packages/bitcoin-knots/src/net_processing.cpp`, `packages/bitcoin-knots/src/validation.h`, `packages/bitcoin-knots/src/consensus/validation.h`, and `packages/bitcoin-knots/src/validation.cpp` - baseline orphan, txdownload, and mempool acceptance anchors. [VERIFIED: local submodule reads]
- `packages/bitcoin-knots/test/functional/p2p_orphan_handling.py` and `packages/bitcoin-knots/test/functional/p2p_tx_download.py` - functional behavior anchors for orphan and tx download tests. [VERIFIED: local submodule reads]
- `docs/parity/source-breadcrumbs.json` and `scripts/check-parity-breadcrumbs.ts` - parity breadcrumb requirements for new/touched Rust files. [VERIFIED: local file reads]

### Secondary (MEDIUM confidence)

- None used; this research was codebase- and vendored-baseline-first. [VERIFIED: research activity]

### Tertiary (LOW confidence)

- Assumptions listed in `## Assumptions Log`; no uncited web-only claims are used. [VERIFIED: Assumptions Log]

## Metadata

**Confidence breakdown:**

- Standard stack: HIGH - First-party workspace crates, toolchain, and Knots submodule were verified locally. [VERIFIED: packages/Cargo.toml; VERIFIED: rust-toolchain.toml; VERIFIED: git submodule status packages/bitcoin-knots]
- Architecture: HIGH - Existing code and project standards already define pure network, pure mempool, and node shell boundaries. [VERIFIED: standards/core/architecture.md; VERIFIED: packages/open-bitcoin-network/src/peer/transaction_relay.rs; VERIFIED: packages/open-bitcoin-mempool/src/pool.rs; VERIFIED: packages/open-bitcoin-node/src/network.rs]
- Pitfalls: HIGH for listed risks - each risk maps to a current code seam, a locked decision, or a Knots behavior anchor. [VERIFIED: .planning/phases/102-orphan-handling-and-admission-outcome-bridge/102-CONTEXT.md; VERIFIED: packages/bitcoin-knots/src/txorphanage.cpp; VERIFIED: packages/open-bitcoin-mempool/src/pool.rs]
- Exact type names and module locations: MEDIUM - the context delegates names and module split to the planner. [VERIFIED: .planning/phases/102-orphan-handling-and-admission-outcome-bridge/102-CONTEXT.md; ASSUMED]

**Validation Architecture:** Omitted because `workflow.nyquist_validation` is `false` in `.planning/config.json`. [VERIFIED: .planning/config.json]

**Runtime State Inventory:** Omitted because Phase 102 is not a rename, rebrand, refactor-only, or migration phase. [VERIFIED: .planning/ROADMAP.md; VERIFIED: .planning/phases/102-orphan-handling-and-admission-outcome-bridge/102-CONTEXT.md]

**Research date:** 2026-06-30 [VERIFIED: environment_context current_date]
**Valid until:** 2026-07-30 for local codebase planning assumptions, or sooner if Phase 101/102 code changes before planning. [ASSUMED]
