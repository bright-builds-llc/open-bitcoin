# Phase 104: Relay Serving, Fanout, and Rebroadcast Policy - Research

**Researched:** 2026-07-01
**Domain:** P2P transaction relay serving, txid/wtxid fanout policy, mempool lifecycle coherence, local submission relay evidence
**Confidence:** HIGH

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

### Relay-Eligible Transaction Serving

- **D-01:** Peer `getdata` transaction serving should consult a typed relay
  serving cache derived from accepted local or peer mempool outcomes, not a loose
  "known transaction" map. `MSG_TX` and `MSG_WTX` requests must resolve through
  txid/wtxid-aware identity before the managed shell emits `tx` or `notfound`.
- **D-02:** Serve transactions only when the transaction is currently
  relay-eligible and present in accepted mempool-backed runtime state. Unknown,
  stale, confirmed, rejected, replaced, evicted, expired, identity-mismatched,
  and non-transaction inventory requests should emit stable typed outcomes and
  peer-facing `notfound` where appropriate.
- **D-03:** Block serving and transaction serving should remain separate
  branches. Preserve existing block `getdata` behavior while deepening
  transaction serving around mempool lifecycle state and relay eligibility.
- **D-04:** Serving evidence must be low-cardinality, for example `served`,
  `unknown`, `stale`, `confirmed`, `rejected`, `replaced`, `evicted`,
  `expired`, `identity_mismatch`, and `not_relay_eligible`. Do not expose raw
  transaction hex, txids, wtxids, peer ids, endpoints, permission strings,
  class names, credentials, or dynamic labels in shared evidence.

### Accepted-Transaction Fanout

- **D-05:** Accepted or replaced `MempoolOutcome` values should enqueue relay
  announcements to eligible peers through a pure fanout policy. The policy
  should emit typed actions for announce, suppress, queue-cap, rate-limit, and
  cleanup decisions; adapters translate those actions into `inv` messages later.
- **D-06:** Fanout eligibility must reuse Phase 100 relay activation and peer
  eligibility decisions. Outbound and manual peers require explicit relay
  activation; inbound peers require inbound serving plus scoped relay permission
  effects. Protected admission alone must not make a peer eligible for
  transaction relay.
- **D-07:** Announcements must honor each peer's negotiated identity mode:
  txid-only peers receive `InventoryType::Transaction`; wtxidrelay peers receive
  `InventoryType::WitnessTransaction`. Do not announce a transaction in an
  identity form that contradicts the peer's negotiated mode.
- **D-08:** Suppression rules should cover the origin/requesting peer,
  already-have state, recent rejects, in-flight/requested state, mempool-known
  state, relay-disabled peers, non-eligible inbound peers, queue caps, and rate
  caps. Suppression should be observable through fixed labels rather than
  dynamic transaction or peer material.
- **D-09:** Per-peer fanout queues must be bounded and fake-clock testable. Tests
  should prove cap enforcement, deterministic draining, rate limits, identity
  negotiation, and cleanup after disconnect or mempool lifecycle removal without
  sleeps or public-network behavior.

### Local Submission Relay Evidence

- **D-10:** Local `sendrawtransaction` submissions should continue to enter
  mempool admission through the shared outcome contract. When accepted or
  replaced, the managed runtime should store the transaction for serving and
  enqueue relay fanout evidence, but RPC success must not imply public
  propagation is guaranteed.
- **D-11:** Local submission evidence should distinguish accepted, queued,
  suppressed, not eligible, relay disabled, and deferred rebroadcast cases. Keep
  detailed operator/RPC/metrics/log/support presentation for Phase 105, but make
  the internal outcome and tests available here so later surfaces share one
  contract.
- **D-12:** Rejected, duplicate, orphaned, evicted, and expired local outcomes
  must not enqueue public fanout. If a duplicate accepted transaction is already
  stored, serving state may remain unchanged, but the relay evidence should not
  claim a new announcement was broadcast.

### Rebroadcast Boundary

- **D-13:** Treat transaction rebroadcast scheduling as explicitly deferred in
  Phase 104. Implement the `REL-04` route by adding bounded, testable
  `rebroadcast_deferred` evidence across docs, internal status/policy output,
  and tests rather than adding a timer-driven rebroadcast loop.
- **D-14:** The deferred rebroadcast evidence should state that Open Bitcoin can
  serve and announce newly accepted transactions within the scoped relay
  boundary, but it does not yet periodically rebroadcast wallet/local mempool
  transactions or guarantee public propagation.
- **D-15:** Do not introduce wall-clock rebroadcast timers, public-network relay
  UAT, service-manager loops, production deployment gates, wallet production
  safety claims, or compact-block/package-relay behavior while closing REL-04.

### Lifecycle Cleanup And Coherence

- **D-16:** Mempool lifecycle events from Phase 103 must clean relay serving and
  fanout state. Block connect, conflict cleanup, replacement, trimming,
  eviction, expiry, reorg reconsideration, and disconnect cleanup should not
  leave transactions serveable or queued after they are no longer eligible.
- **D-17:** Reuse the Phase 101 scheduler vocabulary and Phase 102
  `MempoolOutcome` vocabulary where possible so request, admission, serving,
  fanout, and cleanup evidence stay compatible.
- **D-18:** Keep pure relay/fanout decisions in `open-bitcoin-network` or another
  pure functional-core surface; keep mempool mutation, transaction storage, and
  message translation in `open-bitcoin-node` managed shell adapters.

### Tests, Parity, And Guardrails

- **D-19:** Tests should lead with pure serving/fanout policy cases, then managed
  network integration cases for `getdata`, accepted peer transactions, local
  `sendrawtransaction`, lifecycle cleanup, and rebroadcast-deferred evidence.
- **D-20:** New first-party Rust source or test files under
  `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests` need parity
  breadcrumbs in `docs/parity/source-breadcrumbs.json`, citing Knots anchors
  where defensible. Use explicit `none` only for Open Bitcoin-only support
  infrastructure.
- **D-21:** If docs, parity roots, or verifier wiring change, add a deterministic
  Phase 104 checker with fixture tests and wire it into `bash scripts/verify.sh`
  after Phase 103. The checker should guard REL-01 through REL-04 evidence and
  reject claims for compact blocks, package relay, bloom/filter serving, public
  relay defaults, public-network CI, production readiness, and production-funds
  wallet use.
- **D-22:** Verification stays local and deterministic. The phase closeout target
  remains `bash scripts/verify.sh`; no public-network relay, service-manager,
  wall-clock soak, destructive repair, or production-deployment gate belongs in
  default verification.

### the agent's Discretion

The planner may choose exact type names, queue constants, rate-limit constants,
module split, and whether serving/fanout policy lives in
`open-bitcoin-network::peer::transaction_relay` or a sibling pure module. Prefer
small pure APIs plus thin managed shell translation. Keep Phase 105-facing
operator/RPC/metrics/log/support presentation out of this phase except where a
minimal shared contract is needed to make REL-04's deferred evidence truthful.

### Deferred Ideas (OUT OF SCOPE)

- Periodic rebroadcast scheduling for local or wallet-originated transactions is
  deferred beyond Phase 104. Phase 104 should record `rebroadcast_deferred`
  evidence instead of implementing a timer-driven rebroadcast loop.
- Phase 105 owns rich RPC, CLI, dashboard, metrics, structured-log, and support
  bundle presentation for relay and mempool evidence.
- Phase 106 owns final parity traceability, UAT guidance, README/operator docs,
  and release-boundary guardrails across the full v2.0 milestone.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
| --- | --- | --- |
| REL-01 | Node serves only relay-eligible transactions for peer `getdata` and reports unknown/stale/confirmed/rejected/evicted correctly. [VERIFIED: .planning/REQUIREMENTS.md] | Use typed txid/wtxid identity from `TxRelayId`, serving-state outcomes backed by mempool outcome/lifecycle state, and the existing managed `ServeInventory` translation branch. [VERIFIED: packages/open-bitcoin-network/src/peer/transaction_relay.rs; packages/open-bitcoin-node/src/network/inventory.rs; packages/open-bitcoin-mempool/src/outcome.rs] |
| REL-02 | Node announces accepted transactions to eligible peers using negotiated txid/wtxid identity, per-peer queues, rate limits, suppression rules. [VERIFIED: .planning/REQUIREMENTS.md] | Reuse `RelayEligibilityDecision`, `TxRelayPeerMode`, `TxRelayId`, fake-clock scheduler style, and fixed action labels; add a pure fanout policy plus managed `inv` translation. [VERIFIED: packages/open-bitcoin-network/src/relay.rs; packages/open-bitcoin-network/src/peer/transaction_relay.rs; packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs] |
| REL-03 | Local `sendrawtransaction` submissions enter mempool admission and queued relay evidence without guaranteeing public propagation. [VERIFIED: .planning/REQUIREMENTS.md] | Prefer the existing `submit_local_transaction_outcome` path over the older `AdmissionResult` RPC bridge, then record internal relay evidence while keeping public propagation claims out of RPC success semantics. [VERIFIED: packages/open-bitcoin-node/src/network/admission_bridge.rs; packages/open-bitcoin-rpc/src/dispatch/node.rs] |
| REL-04 | Rebroadcast either implemented bounded or explicitly marked deferred across docs/status/tests. [VERIFIED: .planning/REQUIREMENTS.md] | Context locks the deferred route: add `rebroadcast_deferred` evidence in policy/status/docs/tests and avoid wall-clock rebroadcast loops. [VERIFIED: .planning/phases/104-relay-serving-fanout-and-rebroadcast-policy/104-CONTEXT.md] |
</phase_requirements>

## Summary

Phase 104 should deepen the existing transaction relay stack instead of adding a parallel relay subsystem. [VERIFIED: packages/open-bitcoin-network/src/relay.rs; packages/open-bitcoin-network/src/peer/transaction_relay.rs; packages/open-bitcoin-node/src/network.rs] The pure-core side already has default-off relay eligibility, txid/wtxid identity validation, request/download suppression vocabulary, fake-clock-friendly scheduling patterns, and stable mempool outcome labels. [VERIFIED: packages/open-bitcoin-network/src/relay.rs; packages/open-bitcoin-network/src/peer/transaction_relay.rs; packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs; packages/open-bitcoin-mempool/src/outcome.rs] The managed side already owns transaction storage, `getdata` response translation, local/peer admission, and lifecycle cleanup hooks, so Phase 104 should wire serving/fanout state there without moving I/O into pure policy code. [VERIFIED: packages/open-bitcoin-node/src/network.rs; packages/open-bitcoin-node/src/network/inventory.rs; packages/open-bitcoin-node/src/network/admission_bridge.rs; packages/open-bitcoin-node/src/network/mempool_lifecycle.rs]

Bitcoin Knots keeps transaction serving, announcement, request state, notfound handling, lifecycle cleanup, and rebroadcast-related behavior in the P2P/mempool relay layer, with txid/wtxid inventory types and bounded per-peer transaction announcement/request state. [VERIFIED: packages/bitcoin-knots/src/net_processing.cpp; packages/bitcoin-knots/src/node/txdownloadman.h; packages/bitcoin-knots/src/node/txdownloadman_impl.cpp; packages/bitcoin-knots/src/protocol.h] Open Bitcoin should preserve externally observable scope by serving only currently relay-eligible accepted mempool-backed transactions, emitting `notfound` for missing or non-serveable cases, and making accepted/queued local submission evidence truthful but non-promissory. [VERIFIED: .planning/phases/104-relay-serving-fanout-and-rebroadcast-policy/104-CONTEXT.md; .planning/REQUIREMENTS.md]

The highest-risk planning issue is lifecycle coherence: after every accepted, rejected, replaced, confirmed, evicted, expired, orphaned, disconnected, or peer-cleanup path, serving and fanout state must still agree with mempool state. [VERIFIED: packages/open-bitcoin-mempool/src/outcome.rs; packages/open-bitcoin-mempool/src/pool/lifecycle.rs; packages/open-bitcoin-node/src/network/mempool_lifecycle.rs] The second risk is claim hygiene: Phase 104 must close REL-04 by explicit `rebroadcast_deferred` evidence, not by implying unimplemented periodic rebroadcast or guaranteed public propagation. [VERIFIED: .planning/phases/104-relay-serving-fanout-and-rebroadcast-policy/104-CONTEXT.md]

**Primary recommendation:** Add a pure `open-bitcoin-network` relay serving/fanout policy using existing `RelayEligibilityDecision`, `TxRelayId`, `TxRelayPeerMode`, and `MempoolOutcome` vocabulary; keep runtime storage, transaction lookup, and `inv`/`tx`/`notfound` translation inside `open-bitcoin-node`; record `rebroadcast_deferred` as a first-class low-cardinality result. [VERIFIED: packages/open-bitcoin-network/src/relay.rs; packages/open-bitcoin-network/src/peer/transaction_relay.rs; packages/open-bitcoin-mempool/src/outcome.rs; packages/open-bitcoin-node/src/network.rs]

## Project Constraints (from AGENTS.md)

- Use `git submodule update --init --recursive` to materialize the pinned Bitcoin Knots baseline under `packages/bitcoin-knots` when Knots anchors are needed. [VERIFIED: AGENTS.md; git submodule status packages/bitcoin-knots]
- Rust `1.94.1` is pinned by `rust-toolchain.toml`; Cargo, CI, and Bazel should follow that source of truth. [VERIFIED: AGENTS.md; rust-toolchain.toml]
- Use `bash scripts/verify.sh` as the repo-native verification contract for first-party code, including the Bazel smoke build; `--fast` is local iteration only. [VERIFIED: AGENTS.md; scripts/verify.sh]
- UAT guidance should provide repo-local Cargo and Bazel commands, such as `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- ...` and `bazel run //packages/open-bitcoin-cli:open_bitcoin -- ...`. [VERIFIED: AGENTS.md]
- Bun is the canonical runtime for repo-owned higher-level automation scripts; substantial script logic should be TypeScript, with Bash kept to thin orchestration wrappers. [VERIFIED: AGENTS.md; AGENTS.bright-builds.md; standards/languages/typescript-javascript.md]
- `docs/metrics/lines-of-code.md` is an intentionally tracked generated artifact and may change when verification refreshes metrics. [VERIFIED: AGENTS.md]
- Intentional in-scope behavior differences from Bitcoin Knots must be recorded in `docs/parity/index.json` and companion docs under `docs/parity/`. [VERIFIED: AGENTS.md]
- New first-party Rust source or test files under `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests` require parity breadcrumbs in `docs/parity/source-breadcrumbs.json`; use explicit `none` only when no defensible Knots anchor exists. [VERIFIED: AGENTS.md; docs/parity/source-breadcrumbs.json]
- After substantial feature, parity, operator-surface, or workflow changes, check whether relevant README files need updates. [VERIFIED: AGENTS.md]
- Keep business logic in functional-core modules and I/O/runtime mutation in imperative-shell adapters. [VERIFIED: AGENTS.bright-builds.md; standards/core/architecture.md]
- Parse untrusted or boundary data into domain types at the boundary and make illegal states unrepresentable. [VERIFIED: AGENTS.bright-builds.md; standards/core/architecture.md]
- Prefer early returns, shallow nesting, clear names, and small modules; functions over roughly 161 lines and files over roughly 628 lines deserve extra scrutiny. [VERIFIED: standards/core/code-shape.md]
- Rust code should use `foo.rs` plus `foo/` submodules instead of new `foo/mod.rs`, avoid `unwrap()`, use `thiserror` for library errors where needed, and keep tests organized around one behavior with Arrange/Act/Assert comments when helpful. [VERIFIED: AGENTS.md; standards/languages/rust.md; standards/core/testing.md]
- Project skills were not found under `.claude/skills/` or `.agents/skills/`; the root `AGENTS.md` also reports no project skills. [VERIFIED: find .claude/skills .agents/skills -maxdepth 2 -name SKILL.md; AGENTS.md]

## Standard Stack

### Core

| Library | Version | Purpose | Why Standard |
| --- | --- | --- | --- |
| `open-bitcoin-network` | workspace crate `0.1.0`, Rust 2024 on rustc `1.94.1` | Pure peer relay policy, relay eligibility, txid/wtxid inventory identity, and transaction download scheduler vocabulary. [VERIFIED: packages/Cargo.toml; packages/open-bitcoin-network/Cargo.toml; rust-toolchain.toml] | Existing Phase 100/101 relay policy lives here, and Phase 104 decisions require pure relay/fanout decisions in this boundary. [VERIFIED: packages/open-bitcoin-network/src/relay.rs; packages/open-bitcoin-network/src/peer/transaction_relay.rs; .planning/phases/104-relay-serving-fanout-and-rebroadcast-policy/104-CONTEXT.md] |
| `open-bitcoin-mempool` | workspace crate `0.1.0`, Rust 2024 on rustc `1.94.1` | Admission outcomes, replacement/eviction/expiry labels, and lifecycle removal summaries. [VERIFIED: packages/Cargo.toml; packages/open-bitcoin-mempool/Cargo.toml; packages/open-bitcoin-mempool/src/outcome.rs; packages/open-bitcoin-mempool/src/pool/lifecycle.rs] | Phase 104 serving and fanout must be driven by accepted/replaced/evicted/expired outcome state rather than loose transaction presence. [VERIFIED: .planning/phases/104-relay-serving-fanout-and-rebroadcast-policy/104-CONTEXT.md] |
| `open-bitcoin-node` | workspace crate `0.1.0`, Rust 2024 on rustc `1.94.1` | Managed peer network runtime, transaction storage indexes, message translation, local/peer admission bridge, and lifecycle cleanup hooks. [VERIFIED: packages/Cargo.toml; packages/open-bitcoin-node/Cargo.toml; packages/open-bitcoin-node/src/network.rs; packages/open-bitcoin-node/src/network/inventory.rs; packages/open-bitcoin-node/src/network/admission_bridge.rs] | Existing managed shell already translates pure peer actions into `tx`, `inv`, `getdata`, and `notfound` messages. [VERIFIED: packages/open-bitcoin-node/src/network.rs; packages/open-bitcoin-node/src/network/action_translation.rs; packages/open-bitcoin-node/src/network/inventory.rs] |
| `open-bitcoin-rpc` | workspace crate `0.1.0`, Rust 2024 on rustc `1.94.1`; Axum `0.8.9`, Tokio `1.52.1`, serde `1.0.228`, serde_json `1.0.149` | Local `sendrawtransaction` dispatch and response contract. [VERIFIED: packages/open-bitcoin-rpc/Cargo.toml; packages/open-bitcoin-rpc/src/dispatch/node.rs; packages/open-bitcoin-rpc/src/method/node.rs] | REL-03 touches local submission evidence, but Phase 105 owns rich public presentation. [VERIFIED: .planning/REQUIREMENTS.md; .planning/phases/104-relay-serving-fanout-and-rebroadcast-policy/104-CONTEXT.md] |

### Supporting

| Library / Tool | Version | Purpose | When to Use |
| --- | --- | --- | --- |
| `bash scripts/verify.sh` | repo script | Default phase closeout verifier, including Rust checks, TypeScript checks, parity checkers, and Bazel smoke build. [VERIFIED: AGENTS.md; scripts/verify.sh] | Run before marking implementation complete; use `--profile` for verifier runtime diagnosis and `--fast` only for local iteration. [VERIFIED: AGENTS.md; scripts/verify.sh] |
| Bun | `1.3.9` | Runtime for repo-owned TypeScript phase checkers and tests. [VERIFIED: .bun-version; bun --version; AGENTS.md] | Use if Phase 104 adds a deterministic checker or checker fixture tests. [VERIFIED: scripts/check-phase103-mempool-lifecycle.ts; scripts/check-phase103-mempool-lifecycle.test.ts] |
| Bazel / Bazelisk surface | Bazel `8.6.0`, `rules_rust` `0.69.0`, Rust `1.94.1` | Top-level smoke build and Bzlmod-managed Rust build. [VERIFIED: bazel --version; MODULE.bazel] | Covered by `bash scripts/verify.sh`; do not create a separate phase-only Bazel workflow unless debugging. [VERIFIED: scripts/verify.sh] |
| Bitcoin Knots submodule | `29.3.knots20260210` at commit `a9aee730...` | Parity baseline for P2P relay, inventory serving, transaction download, lifecycle cleanup, and rebroadcast anchors. [VERIFIED: git submodule status packages/bitcoin-knots; AGENTS.md] | Use for source breadcrumbs, docs/parity evidence, and behavior guardrails. [VERIFIED: docs/parity/source-breadcrumbs.json; .planning/phases/104-relay-serving-fanout-and-rebroadcast-policy/104-CONTEXT.md] |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
| --- | --- | --- |
| Existing first-party relay and mempool crates | Add a new external Rust Bitcoin relay or mempool crate | Not acceptable for the production path because the project owns its domain model and avoids existing Rust Bitcoin libraries in production. [VERIFIED: AGENTS.md; .planning/PROJECT.md] |
| Pure policy plus managed translation | Put fanout queues directly in `ManagedPeerNetwork` with ad hoc booleans | Faster to patch but violates the functional-core/imperative-shell boundary and duplicates Phase 100/101 policy vocabulary. [VERIFIED: standards/core/architecture.md; packages/open-bitcoin-network/src/relay.rs; packages/open-bitcoin-network/src/peer/transaction_relay.rs] |
| Explicit `rebroadcast_deferred` evidence | Timer-driven wallet/local rebroadcast loop | Out of scope by locked decisions D-13 through D-15 and would create wall-clock/public-propagation claims the phase is not allowed to make. [VERIFIED: .planning/phases/104-relay-serving-fanout-and-rebroadcast-policy/104-CONTEXT.md] |

**Installation:**

```bash
# No new packages are recommended for Phase 104. [VERIFIED: packages/Cargo.toml; package.json absence]
git submodule update --init --recursive
```

**Version verification:** No `npm view` checks apply because Phase 104 should not add npm packages and this repo has no `package.json`. [VERIFIED: package.json absence; AGENTS.md] Existing versions were verified through `rust-toolchain.toml`, `.bun-version`, Cargo manifests/lockfiles, `MODULE.bazel`, and local tool probes. [VERIFIED: rust-toolchain.toml; .bun-version; packages/Cargo.toml; packages/Cargo.lock; MODULE.bazel; rustc --version; cargo --version; bun --version; bazel --version]

## Architecture Patterns

### Recommended Project Structure

```text
packages/open-bitcoin-network/src/peer/transaction_relay/
├── serving.rs                 # Pure txid/wtxid serving request classification and outcome labels.
├── fanout.rs                  # Pure accepted-outcome fanout queue, suppression, and rate-limit policy.
└── tests/
    ├── serving_cases.rs       # Pure serving policy cases for REL-01.
    └── fanout_cases.rs        # Pure fake-clock fanout cases for REL-02 and REL-04.

packages/open-bitcoin-node/src/network/
├── relay_serving.rs           # Managed lookup/translation for tx/getdata/notfound serving.
├── relay_fanout.rs            # Managed transaction storage, queue drain, and inv translation.
└── tests/
    ├── relay_serving_cases.rs # Managed getdata cases for unknown/stale/confirmed/rejected/replaced/evicted.
    └── relay_fanout_cases.rs  # Managed accepted/local submission/fanout/rebroadcast-deferred cases.

scripts/
├── check-phase104-relay-serving-fanout.ts       # Add only if docs/parity/verifier wiring changes.
└── check-phase104-relay-serving-fanout.test.ts  # Fixture tests for the checker if added.
```

This structure is recommended because `peer.rs` and `network.rs` are already near the local file-size scrutiny threshold, and existing Phase 101/103 tests use split case files for focused behavior coverage. [VERIFIED: standards/core/code-shape.md; wc -l packages/open-bitcoin-network/src/peer.rs packages/open-bitcoin-node/src/network.rs packages/open-bitcoin-node/src/network/tests.rs; packages/open-bitcoin-network/src/peer/transaction_relay/tests/scheduler_cases.rs; packages/open-bitcoin-node/src/network/tests/admission_bridge_cases.rs]

### Pattern 1: Typed Relay Serving Policy

**What:** Represent a peer transaction `getdata` request as a txid/wtxid-aware domain request, classify it against relay eligibility and serving state, and return a fixed `TxServeOutcome` before the managed shell emits `tx` or `notfound`. [VERIFIED: packages/open-bitcoin-network/src/peer/transaction_relay.rs; packages/open-bitcoin-node/src/network/inventory.rs]

**When to use:** Use this for every `MSG_TX` or `MSG_WTX` request that reaches managed inventory serving; keep block serving on the current block path. [VERIFIED: packages/open-bitcoin-network/src/peer/inventory_state.rs; packages/open-bitcoin-node/src/network/inventory.rs]

**Example:**

```rust
// Recommended new shape; source APIs verified in transaction_relay.rs.
let relay_id = TxRelayId::from_inventory_vector_for_peer(vector, peer_mode)?;
let outcome = serving_policy.classify(peer_id, relay_id, serving_state);

match outcome {
    TxServeOutcome::Served { transaction } => send_tx(peer_id, transaction),
    TxServeOutcome::Unknown
    | TxServeOutcome::Stale
    | TxServeOutcome::Confirmed
    | TxServeOutcome::Rejected
    | TxServeOutcome::Replaced
    | TxServeOutcome::Evicted
    | TxServeOutcome::Expired
    | TxServeOutcome::IdentityMismatch
    | TxServeOutcome::NotRelayEligible => send_notfound(peer_id, vector),
}
```

**Planning notes:** `TxRelayId::from_inventory_vector_for_peer` already rejects identity mismatches such as txid inventory from a wtxidrelay peer or witness inventory from a txid-only peer. [VERIFIED: packages/open-bitcoin-network/src/peer/transaction_relay.rs] `ManagedPeerNetwork::serve_inventory` already separates block and transaction lookup, so Phase 104 can deepen only the transaction branch. [VERIFIED: packages/open-bitcoin-node/src/network/inventory.rs]

### Pattern 2: Pure Fanout Policy, Managed `inv` Translation

**What:** Feed accepted/replaced `MempoolOutcome` values into a pure fanout queue policy that emits typed actions such as announce, suppress, queue cap, rate limit, cleanup, and rebroadcast deferred. [VERIFIED: packages/open-bitcoin-mempool/src/outcome.rs; .planning/phases/104-relay-serving-fanout-and-rebroadcast-policy/104-CONTEXT.md]

**When to use:** Use this when peer-admitted or locally submitted transactions become accepted/replaced and after lifecycle cleanup events remove transactions from relay eligibility. [VERIFIED: packages/open-bitcoin-node/src/network/admission_bridge.rs; packages/open-bitcoin-node/src/network/mempool_lifecycle.rs]

**Example:**

```rust
// Recommended new shape; source APIs verified in relay.rs and outcome.rs.
if matches!(outcome, MempoolOutcome::Accepted { .. } | MempoolOutcome::Replaced { .. }) {
    let actions = fanout_policy.enqueue_outcome(outcome, eligible_peers, clock.now());
    self.translate_fanout_actions(actions, outbound_messages);
}
```

**Planning notes:** Fanout eligibility should call the same Phase 100 relay eligibility classifier used by peer relay activation instead of adding new booleans. [VERIFIED: packages/open-bitcoin-network/src/relay.rs] Fanout identity should use `TxRelayPeerMode` and `InventoryType::{Transaction, WitnessTransaction}` exactly as existing announcement tests expect. [VERIFIED: packages/open-bitcoin-network/src/peer.rs; packages/open-bitcoin-network/src/peer/tests.rs]

### Pattern 3: Lifecycle-Driven Serving And Fanout Cleanup

**What:** Make all accepted, replaced, confirmed, conflicted, trimmed, evicted, expired, reorg-reconsidered, and disconnected paths call one cleanup/update API for serving and fanout state. [VERIFIED: packages/open-bitcoin-mempool/src/outcome.rs; packages/open-bitcoin-mempool/src/pool/lifecycle.rs; packages/open-bitcoin-node/src/network/mempool_lifecycle.rs]

**When to use:** Use the cleanup API whenever `remove_stored_transactions`, `apply_admitted_outcome`, `remove_evicted_outcome`, block-connect lifecycle cleanup, reorg reconsideration, or peer disconnect cleanup runs. [VERIFIED: packages/open-bitcoin-node/src/network/admission_bridge.rs; packages/open-bitcoin-node/src/network/mempool_lifecycle.rs; packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs]

**Example:**

```rust
// Recommended new managed-shell call site; source cleanup paths verified.
self.remove_stored_transactions(outcome.removed_txids());
self.relay_state.remove_transactions(outcome.removed_txids(), cleanup_reason);
self.fanout_policy.cleanup_transactions(outcome.removed_txids(), clock.now());
```

**Planning notes:** Phase 103 already removed runtime transaction indexes when block connect, conflict cleanup, replacement, eviction, expiry, or reorg reconsideration changes mempool state. [VERIFIED: packages/open-bitcoin-node/src/network/mempool_lifecycle.rs; packages/open-bitcoin-node/src/network/tests/mempool_lifecycle_cases.rs] Phase 104 should extend those same call sites rather than adding independent cleanup timing. [VERIFIED: .planning/phases/104-relay-serving-fanout-and-rebroadcast-policy/104-CONTEXT.md]

### Pattern 4: Local Submission Evidence Through Outcome Contract

**What:** Route local `sendrawtransaction` through `MempoolOutcome`-based admission so accepted/replaced submissions can store serving state and enqueue fanout evidence without saying they propagated publicly. [VERIFIED: packages/open-bitcoin-node/src/network/admission_bridge.rs; packages/open-bitcoin-rpc/src/dispatch/node.rs]

**When to use:** Use this for REL-03 and for tests that compare accepted, duplicate, rejected, orphaned, evicted, and expired local outcomes. [VERIFIED: .planning/REQUIREMENTS.md; packages/open-bitcoin-node/src/network/tests/admission_bridge_cases.rs]

**Example:**

```rust
// Existing outcome path to prefer over the older AdmissionResult-only bridge.
let outcome = self.mempool.submit_transaction_outcome(transaction.clone());
match outcome {
    MempoolOutcome::Accepted { .. } | MempoolOutcome::Replaced { .. } => {
        self.apply_admitted_outcome(&outcome, transaction);
        self.relay_evidence.record_queued_internal(&outcome);
    }
    MempoolOutcome::Rejected { .. }
    | MempoolOutcome::Duplicate { .. }
    | MempoolOutcome::Orphaned { .. }
    | MempoolOutcome::Evicted { .. }
    | MempoolOutcome::Expired { .. } => {
        self.relay_evidence.record_not_queued(&outcome);
    }
}
```

**Planning notes:** `open-bitcoin-rpc` currently calls the older `submit_local_transaction` bridge and returns txid/replaced/evicted fields. [VERIFIED: packages/open-bitcoin-rpc/src/dispatch/node.rs; packages/open-bitcoin-rpc/src/method/node.rs] The planner should either adapt that path to the outcome bridge internally or add a minimal internal relay evidence carrier that Phase 105 can present later. [VERIFIED: packages/open-bitcoin-node/src/network/admission_bridge.rs; .planning/phases/104-relay-serving-fanout-and-rebroadcast-policy/104-CONTEXT.md]

### Pattern 5: Deterministic Phase Checker For Parity/Doc Claims

**What:** If Phase 104 changes docs, parity roots, or verifier wiring, add a Bun TypeScript checker with fixture tests and wire it into `scripts/verify.sh` after Phase 103. [VERIFIED: .planning/phases/104-relay-serving-fanout-and-rebroadcast-policy/104-CONTEXT.md; scripts/verify.sh; scripts/check-phase103-mempool-lifecycle.ts]

**When to use:** Use this when adding REL-04 docs/status evidence, parity breadcrumbs, or forbidden-claim guardrails. [VERIFIED: docs/parity/source-breadcrumbs.json; scripts/check-phase103-mempool-lifecycle.ts]

**Example:**

```typescript
// Pattern copied from the Phase 103 checker style.
const REQUIRED_REQUIREMENTS = ["REL-01", "REL-02", "REL-03", "REL-04"] as const;
const FORBIDDEN_CLAIMS = [
  "compact block relay",
  "package relay",
  "public-network CI",
  "production-ready relay",
] as const;
```

**Planning notes:** The Phase 103 checker has target-file constants, required symbol checks, breadcrumb group checks, forbidden-claim checks, and fixture tests. [VERIFIED: scripts/check-phase103-mempool-lifecycle.ts; scripts/check-phase103-mempool-lifecycle.test.ts]

### Anti-Patterns to Avoid

- **Loose `known_txids` serving:** Serving directly from a known-transaction map can over-serve stale, confirmed, rejected, replaced, or evicted transactions. Use a typed serving state derived from current accepted mempool-backed runtime state. [VERIFIED: .planning/phases/104-relay-serving-fanout-and-rebroadcast-policy/104-CONTEXT.md; packages/open-bitcoin-node/src/network/inventory.rs]
- **Parallel relay eligibility booleans:** Adding one-off `can_relay` booleans risks diverging from Phase 100 default-off relay activation and scoped inbound permission semantics. Use `RelayEligibilityDecision`. [VERIFIED: packages/open-bitcoin-network/src/relay.rs; .planning/phases/100-relay-activation-boundary-and-permission-semantics/100-CONTEXT.md]
- **Immediate socket-side fanout mutation in pure code:** Pure policy should emit actions; managed node code should mutate queues/storage and translate actions into wire messages. [VERIFIED: standards/core/architecture.md; packages/open-bitcoin-node/src/network/action_translation.rs]
- **Wall-clock rebroadcast implementation:** REL-04 is locked to explicit deferral for this phase, so a timer loop would contradict D-13 through D-15. [VERIFIED: .planning/phases/104-relay-serving-fanout-and-rebroadcast-policy/104-CONTEXT.md]
- **High-cardinality evidence:** Shared evidence must not include raw transaction hex, txids, wtxids, peer ids, endpoints, permission strings, credentials, or dynamic labels. [VERIFIED: .planning/phases/104-relay-serving-fanout-and-rebroadcast-policy/104-CONTEXT.md]
- **Large-file accretion:** `peer.rs`, `network.rs`, and node network tests are already large enough that new behavior should be split into focused modules and case files. [VERIFIED: standards/core/code-shape.md; wc -l packages/open-bitcoin-network/src/peer.rs packages/open-bitcoin-node/src/network.rs packages/open-bitcoin-node/src/network/tests.rs]

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
| --- | --- | --- | --- |
| txid/wtxid inventory identity | A custom matcher over raw inventory numeric codes | `TxRelayId`, `TxRelayPeerMode`, and `InventoryType` conversions. [VERIFIED: packages/open-bitcoin-network/src/peer/transaction_relay.rs; packages/open-bitcoin-primitives/src/network.rs] | Existing code already encodes peer-mode mismatch behavior and fixed identity errors. [VERIFIED: packages/open-bitcoin-network/src/peer/transaction_relay.rs] |
| Relay eligibility | Separate booleans for outbound, inbound, protected, and relay-enabled peers | `classify_relay_eligibility` and `RelayEligibilityDecision`. [VERIFIED: packages/open-bitcoin-network/src/relay.rs] | Phase 100 already defines default-off activation, inbound-serving requirements, and protected-not-relay semantics. [VERIFIED: packages/open-bitcoin-network/src/relay.rs; .planning/phases/100-relay-activation-boundary-and-permission-semantics/100-CONTEXT.md] |
| Serving state classification | Direct lookups in `transactions_by_txid` / `transactions_by_wtxid` as the only source of truth | A typed serving cache/state machine derived from `MempoolOutcome` and lifecycle cleanup. [VERIFIED: packages/open-bitcoin-node/src/network/inventory.rs; packages/open-bitcoin-mempool/src/outcome.rs; packages/open-bitcoin-mempool/src/pool/lifecycle.rs] | The current maps prove transaction bytes are stored, not that the transaction is still relay-eligible. [VERIFIED: packages/open-bitcoin-node/src/network/inventory.rs; packages/open-bitcoin-node/src/network/mempool_lifecycle.rs] |
| Fanout queue and rate evidence | Ad hoc `Vec<InventoryVector>` mutation without labels or fake-clock control | A pure fanout policy modeled after the existing `TxDownloadScheduler` fake-clock/action pattern. [VERIFIED: packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs; packages/open-bitcoin-network/src/peer/transaction_relay/tests/scheduler_cases.rs] | Existing scheduler tests already prove deterministic caps, fallback, notfound, cleanup, and clock-driven expiry without sleeps. [VERIFIED: packages/open-bitcoin-network/src/peer/transaction_relay/tests/scheduler_cases.rs] |
| Local submission relay claims | RPC text or status that says an accepted transaction was publicly broadcast | Internal evidence labels such as accepted, queued, suppressed, not eligible, relay disabled, and `rebroadcast_deferred`. [VERIFIED: .planning/phases/104-relay-serving-fanout-and-rebroadcast-policy/104-CONTEXT.md] | REL-03 requires truthful queued relay evidence without propagation guarantees. [VERIFIED: .planning/REQUIREMENTS.md] |
| Rebroadcast | Timer-driven public relay loop | Explicit `rebroadcast_deferred` evidence across docs/status/tests. [VERIFIED: .planning/phases/104-relay-serving-fanout-and-rebroadcast-policy/104-CONTEXT.md] | Periodic rebroadcast is deferred beyond Phase 104. [VERIFIED: .planning/phases/104-relay-serving-fanout-and-rebroadcast-policy/104-CONTEXT.md] |
| Parity guardrail checking | Unstructured grep in release notes or manual review only | A deterministic Bun TypeScript checker if docs/parity/verifier wiring changes. [VERIFIED: scripts/check-phase103-mempool-lifecycle.ts; scripts/check-phase103-mempool-lifecycle.test.ts; scripts/verify.sh] | Existing phase checker pattern catches required evidence and forbidden claims in local verification. [VERIFIED: scripts/check-phase103-mempool-lifecycle.ts] |

**Key insight:** The hard problem is not serializing an `inv` or `tx` message; it is keeping serving, fanout, local submission evidence, and lifecycle cleanup consistent with relay eligibility and mempool state. [VERIFIED: packages/open-bitcoin-node/src/network/inventory.rs; packages/open-bitcoin-node/src/network/admission_bridge.rs; packages/open-bitcoin-node/src/network/mempool_lifecycle.rs; .planning/phases/104-relay-serving-fanout-and-rebroadcast-policy/104-CONTEXT.md]

## Common Pitfalls

### Pitfall 1: Serving Stale Transactions From Storage Maps

**What goes wrong:** A transaction remains in `transactions_by_txid` or `transactions_by_wtxid`, so `getdata` receives `tx` even after the transaction is confirmed, replaced, evicted, expired, or otherwise no longer relay-eligible. [VERIFIED: packages/open-bitcoin-node/src/network/inventory.rs; packages/open-bitcoin-node/src/network/mempool_lifecycle.rs]

**Why it happens:** Current managed serving checks stored bytes and returns `NotFound` only when the map lookup misses. [VERIFIED: packages/open-bitcoin-node/src/network/inventory.rs]

**How to avoid:** Make serving consult typed relay-serving state that is updated by `MempoolOutcome`, block-connect lifecycle summaries, reorg reconsideration, and explicit storage cleanup. [VERIFIED: packages/open-bitcoin-mempool/src/outcome.rs; packages/open-bitcoin-mempool/src/pool/lifecycle.rs; packages/open-bitcoin-node/src/network/mempool_lifecycle.rs]

**Warning signs:** Tests that only assert "stored tx can be served" without cases for confirmed, rejected, replaced, evicted, expired, stale, and identity-mismatch requests are incomplete for REL-01. [VERIFIED: .planning/REQUIREMENTS.md; .planning/phases/104-relay-serving-fanout-and-rebroadcast-policy/104-CONTEXT.md]

### Pitfall 2: Fanout To Ineligible Peers

**What goes wrong:** A protected inbound peer or relay-disabled peer receives transaction announcements because admission permission was confused with relay eligibility. [VERIFIED: packages/open-bitcoin-network/src/relay.rs; .planning/phases/104-relay-serving-fanout-and-rebroadcast-policy/104-CONTEXT.md]

**Why it happens:** Protected admission and relay serving are separate Phase 100 concepts. [VERIFIED: packages/open-bitcoin-network/src/relay.rs; .planning/phases/100-relay-activation-boundary-and-permission-semantics/100-CONTEXT.md]

**How to avoid:** Feed each peer through `classify_relay_eligibility` before enqueueing fanout, and record fixed suppression labels for disabled, inbound-not-serving, permission-required, and protected-not-relay cases. [VERIFIED: packages/open-bitcoin-network/src/relay.rs]

**Warning signs:** Tests assert fanout based only on connection direction or peer permission presence, rather than the classifier result. [VERIFIED: packages/open-bitcoin-network/src/relay.rs; packages/open-bitcoin-network/src/peer/tests.rs]

### Pitfall 3: Wrong Identity For Negotiated Relay Mode

**What goes wrong:** A txid-only peer receives witness transaction inventory or a wtxidrelay peer receives txid inventory, causing mismatched request/serve behavior. [VERIFIED: packages/open-bitcoin-network/src/peer/transaction_relay.rs; packages/open-bitcoin-network/src/peer/tests.rs]

**Why it happens:** The peer mode must be considered at both announcement time and `getdata` request classification time. [VERIFIED: packages/open-bitcoin-network/src/peer/transaction_relay.rs]

**How to avoid:** Use `TxRelayPeerMode::from_remote_wtxidrelay`, `TxRelayId::from_inventory_vector_for_peer`, and `InventoryType::{Transaction, WitnessTransaction}` consistently. [VERIFIED: packages/open-bitcoin-network/src/peer/transaction_relay.rs; packages/open-bitcoin-network/src/peer.rs]

**Warning signs:** New code constructs `InventoryVector` directly from a txid/wtxid without checking peer mode. [VERIFIED: packages/open-bitcoin-network/src/peer.rs; packages/open-bitcoin-network/src/peer/transaction_relay.rs]

### Pitfall 4: Conflating Queued With Propagated

**What goes wrong:** Local `sendrawtransaction` success or queued fanout evidence is described as public broadcast or guaranteed propagation. [VERIFIED: .planning/REQUIREMENTS.md; .planning/phases/104-relay-serving-fanout-and-rebroadcast-policy/104-CONTEXT.md]

**Why it happens:** Bitcoin relay has asynchronous peers, suppression, notfound/fallback, queue caps, and rate limits; Open Bitcoin Phase 104 does not implement periodic rebroadcast. [VERIFIED: packages/bitcoin-knots/src/net_processing.cpp; packages/bitcoin-knots/src/node/txdownloadman.h; .planning/phases/104-relay-serving-fanout-and-rebroadcast-policy/104-CONTEXT.md]

**How to avoid:** Use labels that distinguish accepted, queued, suppressed, not eligible, relay disabled, and `rebroadcast_deferred`; reserve rich operator wording for Phase 105. [VERIFIED: .planning/phases/104-relay-serving-fanout-and-rebroadcast-policy/104-CONTEXT.md]

**Warning signs:** Docs, RPC responses, logs, or tests contain phrases such as "broadcast guaranteed", "public propagation", "production relay", or "periodic rebroadcast" for Phase 104. [VERIFIED: .planning/phases/104-relay-serving-fanout-and-rebroadcast-policy/104-CONTEXT.md; scripts/check-phase103-mempool-lifecycle.ts]

### Pitfall 5: Adding Sleeps Or Wall-Clock Timers

**What goes wrong:** Tests become flaky or Phase 104 accidentally implements the deferred rebroadcast feature. [VERIFIED: .planning/phases/104-relay-serving-fanout-and-rebroadcast-policy/104-CONTEXT.md]

**Why it happens:** Rate limits and rebroadcast behavior are tempting to model with real time. [VERIFIED: packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs; packages/open-bitcoin-network/src/peer/transaction_relay/tests/scheduler_cases.rs]

**How to avoid:** Use fake-clock injection like the existing transaction download scheduler tests, and represent rebroadcast as a typed deferred evidence action. [VERIFIED: packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs; packages/open-bitcoin-network/src/peer/transaction_relay/tests/scheduler_cases.rs]

**Warning signs:** Tests call sleep APIs or require public-network peers. [VERIFIED: .planning/phases/104-relay-serving-fanout-and-rebroadcast-policy/104-CONTEXT.md; scripts/verify.sh]

### Pitfall 6: Missing Parity Breadcrumbs For New Rust Files

**What goes wrong:** Verification fails or parity evidence becomes unauditable after adding new first-party Rust source/test files. [VERIFIED: AGENTS.md; docs/parity/source-breadcrumbs.json]

**Why it happens:** The repo requires source breadcrumbs for new first-party Rust files under `packages/open-bitcoin-*/src` and `packages/open-bitcoin-*/tests`. [VERIFIED: AGENTS.md]

**How to avoid:** Add `docs/parity/source-breadcrumbs.json` entries for every new Phase 104 Rust file, citing Knots anchors such as `net_processing.cpp`, `txdownloadman.h`, `txdownloadman_impl.cpp`, and `protocol.h` where defensible. [VERIFIED: AGENTS.md; packages/bitcoin-knots/src/net_processing.cpp; packages/bitcoin-knots/src/node/txdownloadman.h; packages/bitcoin-knots/src/node/txdownloadman_impl.cpp; packages/bitcoin-knots/src/protocol.h]

**Warning signs:** New file paths under `packages/open-bitcoin-network/src`, `packages/open-bitcoin-node/src`, or their test directories are absent from `docs/parity/source-breadcrumbs.json`. [VERIFIED: docs/parity/source-breadcrumbs.json]

## Code Examples

Verified patterns from local source:

### Identity-Gated Transaction Inventory

```rust
// Source: packages/open-bitcoin-network/src/peer/transaction_relay.rs
let relay_id = TxRelayId::from_inventory_vector_for_peer(vector, peer_mode)?;
```

`TxRelayId::from_inventory_vector_for_peer` is the correct planning anchor for rejecting txid/wtxid identity mismatches before serving or queueing. [VERIFIED: packages/open-bitcoin-network/src/peer/transaction_relay.rs]

### Managed Translation Of Pure Relay Actions

```rust
// Source pattern: packages/open-bitcoin-node/src/network.rs
PeerAction::TransactionRelay(action) => {
    if let Some(result) = self.process_transaction_relay_action(peer_id, action) {
        outbound.push(result);
    }
}
```

Phase 104 fanout should mirror this action-translation shape: pure policy returns typed actions, and `open-bitcoin-node` translates them into targeted wire messages. [VERIFIED: packages/open-bitcoin-node/src/network.rs; packages/open-bitcoin-node/src/network/action_translation.rs]

### Outcome-Driven Admission And Cleanup

```rust
// Source pattern: packages/open-bitcoin-node/src/network/admission_bridge.rs
match &outcome {
    MempoolOutcome::Accepted { .. } | MempoolOutcome::Replaced { .. } => {
        self.apply_admitted_outcome(&outcome, transaction.clone());
    }
    MempoolOutcome::Evicted { .. } | MempoolOutcome::Expired { .. } => {
        self.remove_evicted_outcome(&outcome);
    }
    _ => {}
}
```

Phase 104 should hook serving and fanout updates into this same outcome-driven branch structure. [VERIFIED: packages/open-bitcoin-node/src/network/admission_bridge.rs]

### Fake-Clock Policy Tests

```rust
// Source pattern: packages/open-bitcoin-network/src/peer/transaction_relay/tests/scheduler_cases.rs
let mut clock = DeterministicClock::new();
let action = scheduler.record_announcement(peer_id, txid, clock.now());
clock.advance(policy.request_delay);
let requests = scheduler.next_requests(peer_id, clock.now());
```

Fanout cap, drain, rate-limit, and rebroadcast-deferred tests should use deterministic clock advancement instead of sleeps. [VERIFIED: packages/open-bitcoin-network/src/peer/transaction_relay/tests/scheduler_cases.rs]

### Phase Checker Shape

```typescript
// Source pattern: scripts/check-phase103-mempool-lifecycle.ts
const REQUIRED_REQUIREMENTS = ["REL-01", "REL-02", "REL-03", "REL-04"] as const;
const REQUIRED_FORBIDDEN_BOUNDARIES = [
  "compact block relay",
  "package relay",
  "public-network CI",
] as const;
```

If Phase 104 adds docs/parity/verifier wiring, the checker should guard required REL evidence and reject out-of-scope claims. [VERIFIED: scripts/check-phase103-mempool-lifecycle.ts; .planning/phases/104-relay-serving-fanout-and-rebroadcast-policy/104-CONTEXT.md]

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
| --- | --- | --- | --- |
| Open Bitcoin's early managed serving path can serve stored transactions from `transactions_by_txid`/`transactions_by_wtxid` and emit `NotFound` on map miss. [VERIFIED: packages/open-bitcoin-node/src/network/inventory.rs] | Phase 104 should add typed relay-serving state that distinguishes served, unknown, stale, confirmed, rejected, replaced, evicted, expired, identity mismatch, and not relay eligible. [VERIFIED: .planning/phases/104-relay-serving-fanout-and-rebroadcast-policy/104-CONTEXT.md] | Phase 104 planning target. [VERIFIED: .planning/ROADMAP.md] | REL-01 needs behavior coverage beyond raw storage lookup. [VERIFIED: .planning/REQUIREMENTS.md] |
| Phase 101 download scheduling focuses on peer `inv` intake, `getdata` requests, fallback, recent rejects, and cleanup. [VERIFIED: packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs] | Phase 104 should add outbound accepted-transaction fanout queues using the same typed identity and fixed-evidence style. [VERIFIED: .planning/phases/104-relay-serving-fanout-and-rebroadcast-policy/104-CONTEXT.md] | Phase 104 planning target. [VERIFIED: .planning/ROADMAP.md] | REL-02 must prove identity negotiation, queue bounds, rate limits, suppression, and cleanup. [VERIFIED: .planning/REQUIREMENTS.md] |
| Local RPC dispatch currently calls `submit_local_transaction` and returns txid/replaced/evicted response fields. [VERIFIED: packages/open-bitcoin-rpc/src/dispatch/node.rs; packages/open-bitcoin-rpc/src/method/node.rs] | Phase 104 should route local submissions through outcome-based admission and internal queued relay evidence while avoiding public propagation guarantees. [VERIFIED: packages/open-bitcoin-node/src/network/admission_bridge.rs; .planning/phases/104-relay-serving-fanout-and-rebroadcast-policy/104-CONTEXT.md] | Phase 104 planning target. [VERIFIED: .planning/ROADMAP.md] | REL-03 closes the admission-to-relay-evidence gap. [VERIFIED: .planning/REQUIREMENTS.md] |
| Bitcoin Knots has transaction relay serving, inventory announcement, request tracking, and wallet rebroadcast-related behavior across `net_processing`, `txdownloadman`, and wallet resend code. [VERIFIED: packages/bitcoin-knots/src/net_processing.cpp; packages/bitcoin-knots/src/node/txdownloadman.h; packages/bitcoin-knots/src/node/txdownloadman_impl.cpp; packages/bitcoin-knots/src/wallet/wallet.cpp] | Phase 104 explicitly defers periodic rebroadcast and records `rebroadcast_deferred` evidence. [VERIFIED: .planning/phases/104-relay-serving-fanout-and-rebroadcast-policy/104-CONTEXT.md] | Locked by Phase 104 discussion on 2026-07-01. [VERIFIED: .planning/phases/104-relay-serving-fanout-and-rebroadcast-policy/104-CONTEXT.md] | REL-04 can close truthfully without a timer loop or production propagation claim. [VERIFIED: .planning/REQUIREMENTS.md] |

**Deprecated/outdated for this phase:**

- Using direct stored-transaction maps as relay eligibility proof is insufficient for REL-01. [VERIFIED: packages/open-bitcoin-node/src/network/inventory.rs; .planning/REQUIREMENTS.md]
- Adding periodic rebroadcast scheduling in Phase 104 is out of scope by locked decisions D-13 through D-15. [VERIFIED: .planning/phases/104-relay-serving-fanout-and-rebroadcast-policy/104-CONTEXT.md]
- Claiming public propagation from `sendrawtransaction` success is out of scope by REL-03 and D-10. [VERIFIED: .planning/REQUIREMENTS.md; .planning/phases/104-relay-serving-fanout-and-rebroadcast-policy/104-CONTEXT.md]

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
| --- | --- | --- | --- |
| None | All factual claims in this research are sourced from local repo files, local tool probes, the pinned Bitcoin Knots submodule, or official OWASP ASVS pages. [VERIFIED: this research pass] | All | No user confirmation is needed for assumed technical facts; planning should still validate design choices against implementation constraints. [VERIFIED: this research pass] |

## Open Questions

1. **Where should the minimal Phase 105-facing relay evidence type live?**
   - What we know: Phase 104 must make internal evidence available, but rich RPC/CLI/dashboard/log/support presentation is Phase 105. [VERIFIED: .planning/phases/104-relay-serving-fanout-and-rebroadcast-policy/104-CONTEXT.md]
   - What's unclear: The exact type name and crate boundary for a minimal shared evidence carrier are left to planner discretion. [VERIFIED: .planning/phases/104-relay-serving-fanout-and-rebroadcast-policy/104-CONTEXT.md]
   - Recommendation: Put pure labels/actions in `open-bitcoin-network`; put managed aggregation/status snapshots in `open-bitcoin-node`; expose only what `open-bitcoin-rpc` needs to preserve REL-03 truthfulness. [VERIFIED: standards/core/architecture.md; packages/open-bitcoin-network/src/peer/transaction_relay.rs; packages/open-bitcoin-node/src/network.rs; packages/open-bitcoin-rpc/src/dispatch/node.rs]
2. **Is a Phase 104 checker mandatory or conditional?**
   - What we know: Context requires a checker if docs, parity roots, or verifier wiring change. [VERIFIED: .planning/phases/104-relay-serving-fanout-and-rebroadcast-policy/104-CONTEXT.md]
   - What's unclear: The implementation plan may or may not touch docs/parity/verifier beyond source breadcrumbs. [VERIFIED: docs/parity/source-breadcrumbs.json]
   - Recommendation: Plan for the checker if REL-04 docs/status evidence or parity indexes change; otherwise still add source breadcrumbs for any new Rust files. [VERIFIED: AGENTS.md; docs/parity/source-breadcrumbs.json; scripts/verify.sh]

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
| --- | --- | --- | --- | --- |
| Rust compiler | First-party Rust implementation and tests | yes [VERIFIED: rustc --version] | `rustc 1.94.1 (e408947bf 2026-03-25)` [VERIFIED: rustc --version; rust-toolchain.toml] | None needed |
| Cargo | Rust format/lint/build/test workflow | yes [VERIFIED: cargo --version] | `cargo 1.94.1 (29ea6fb6a 2026-03-24)` [VERIFIED: cargo --version; rust-toolchain.toml] | None needed |
| Bun | TypeScript phase checker and fixture tests if added | yes [VERIFIED: bun --version] | `1.3.9` [VERIFIED: bun --version; .bun-version] | None needed |
| Bazel | Repo smoke build through verifier | yes [VERIFIED: bazel --version] | `8.6.0` [VERIFIED: bazel --version; MODULE.bazel] | Use `bash scripts/verify.sh --fast` only for local iteration, not closeout. [VERIFIED: AGENTS.md; scripts/verify.sh] |
| Git | Source breadcrumbs, submodule, and diff review | yes [VERIFIED: git --version] | `2.50.1 (Apple Git-155)` [VERIFIED: git --version] | None needed |
| Bitcoin Knots submodule | Parity anchors and breadcrumbs | yes [VERIFIED: git submodule status packages/bitcoin-knots] | `29.3.knots20260210` pinned at `a9aee730...` [VERIFIED: git submodule status packages/bitcoin-knots] | Run `git submodule update --init --recursive` if missing. [VERIFIED: AGENTS.md] |
| Node.js | GSD tooling and possible TypeScript support tools | yes [VERIFIED: node --version] | `v24.13.0` [VERIFIED: node --version] | Bun is the repo-preferred runtime for repo-owned TS scripts. [VERIFIED: AGENTS.md] |
| `cargo-llvm-cov` | Coverage enforcement through verification | yes [VERIFIED: cargo llvm-cov --version] | `0.8.5` [VERIFIED: cargo llvm-cov --version] | None documented in repo guidance. [VERIFIED: scripts/verify.sh] |

**Missing dependencies with no fallback:**

- None found for Phase 104 research and expected implementation. [VERIFIED: command -v rustc cargo bun bazel git node; cargo llvm-cov --version; git submodule status packages/bitcoin-knots]

**Missing dependencies with fallback:**

- None found for Phase 104 research and expected implementation. [VERIFIED: command -v rustc cargo bun bazel git node; cargo llvm-cov --version; git submodule status packages/bitcoin-knots]

## Security Domain

Security enforcement is enabled because `.planning/config.json` does not set `security_enforcement` to `false`. [VERIFIED: .planning/config.json]

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
| --- | --- | --- |
| V1 Architecture, Design and Threat Modeling | yes | Maintain functional-core/imperative-shell boundaries and explicit relay threat controls. [CITED: https://github.com/OWASP/ASVS/tree/master/5.0/en; VERIFIED: standards/core/architecture.md] |
| V2 Validation and Business Logic | yes | Validate peer inventory identity, enforce relay eligibility, suppress ineligible fanout, and bound queues/rates. [CITED: https://github.com/OWASP/ASVS/blob/master/5.0/en/0x12-V2-Validation.md; VERIFIED: packages/open-bitcoin-network/src/relay.rs; packages/open-bitcoin-network/src/peer/transaction_relay.rs] |
| V3 Web Frontend Security | no | Phase 104 has no browser frontend surface. [VERIFIED: .planning/ROADMAP.md; .planning/phases/104-relay-serving-fanout-and-rebroadcast-policy/104-CONTEXT.md] |
| V4 API and Web Service | limited | RPC local submission evidence must avoid overclaiming propagation and keep response semantics stable. [CITED: https://github.com/OWASP/ASVS/tree/master/5.0/en; VERIFIED: packages/open-bitcoin-rpc/src/dispatch/node.rs; .planning/REQUIREMENTS.md] |
| V5 File Handling | no | Phase 104 does not add file upload or file parsing surfaces. [VERIFIED: .planning/ROADMAP.md] |
| V6 Stored Data | yes | Runtime transaction serving/fanout state must be cleaned when mempool lifecycle removes eligibility. [CITED: https://github.com/OWASP/ASVS/tree/master/5.0/en; VERIFIED: packages/open-bitcoin-node/src/network/mempool_lifecycle.rs] |
| V7 Authentication | no | Phase 104 does not add authentication mechanisms. [VERIFIED: .planning/ROADMAP.md; packages/open-bitcoin-rpc/src/dispatch/node.rs] |
| V8 Session Management | no | Phase 104 does not add browser/user sessions. [VERIFIED: .planning/ROADMAP.md] |
| V9 Access Control | yes | Peer relay eligibility is an access-control-like gate for who may receive or request transaction relay service. [CITED: https://github.com/OWASP/ASVS/tree/master/5.0/en; VERIFIED: packages/open-bitcoin-network/src/relay.rs] |
| V10 OAuth and OIDC | no | Phase 104 does not use OAuth or OIDC. [VERIFIED: .planning/ROADMAP.md] |
| V11 Cryptography | no new controls | Phase 104 should not add custom cryptography; txid/wtxid identity uses existing primitives. [CITED: https://github.com/OWASP/ASVS/tree/master/5.0/en; VERIFIED: packages/open-bitcoin-network/src/peer/transaction_relay.rs; packages/open-bitcoin-primitives/src] |
| V12 Secure Communication | limited | Peer message translation must preserve negotiated txid/wtxid relay semantics; no new transport security mechanism is introduced. [CITED: https://github.com/OWASP/ASVS/tree/master/5.0/en; VERIFIED: packages/open-bitcoin-network/src/peer/transaction_relay.rs; packages/open-bitcoin-node/src/network/action_translation.rs] |
| V13 Configuration | yes | Relay activation is default-off and queue/rate limits should be explicit, bounded, and testable. [CITED: https://github.com/OWASP/ASVS/blob/master/5.0/en/0x23-V13-Configuration.md; VERIFIED: packages/open-bitcoin-network/src/relay.rs; packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs] |
| V14 Data Protection | yes | Evidence/log/status labels must avoid raw tx hex, txids, wtxids, peer ids, endpoints, credentials, and dynamic labels. [CITED: https://github.com/OWASP/ASVS/blob/master/5.0/en/0x24-V14-Data-Protection.md; VERIFIED: .planning/phases/104-relay-serving-fanout-and-rebroadcast-policy/104-CONTEXT.md] |
| V15 Secure Coding and Configuration | yes | Keep code modular, avoid `unwrap()`, and run repo verification. [CITED: https://github.com/OWASP/ASVS/tree/master/5.0/en; VERIFIED: standards/languages/rust.md; standards/core/verification.md] |
| V16 Security Logging and Error Handling | yes | Use low-cardinality labels and generic error/evidence surfaces without sensitive transaction or peer material. [CITED: https://github.com/OWASP/ASVS/blob/master/5.0/en/0x26-V16-Security-Logging-Error-Handling.md; VERIFIED: .planning/phases/104-relay-serving-fanout-and-rebroadcast-policy/104-CONTEXT.md] |

### Known Threat Patterns for Relay Serving/Fanout

| Pattern | STRIDE | Standard Mitigation |
| --- | --- | --- |
| Peer resource exhaustion through repeated `getdata` or fanout pressure | Denial of Service | Enforce per-peer queue caps, rate limits, fixed draining, and existing download scheduler pressure controls. [VERIFIED: packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs; .planning/phases/104-relay-serving-fanout-and-rebroadcast-policy/104-CONTEXT.md] |
| Information disclosure by serving stale/rejected/confirmed transactions | Information Disclosure | Serve only currently relay-eligible accepted mempool-backed transactions and emit `notfound` for non-serveable outcomes. [VERIFIED: .planning/phases/104-relay-serving-fanout-and-rebroadcast-policy/104-CONTEXT.md; packages/open-bitcoin-node/src/network/inventory.rs] |
| Fanout to unauthorized or non-eligible peers | Elevation of Privilege / Information Disclosure | Reuse Phase 100 relay eligibility and scoped permission-effect policy. [VERIFIED: packages/open-bitcoin-network/src/relay.rs] |
| Dynamic label or log injection through evidence strings | Tampering / Information Disclosure | Emit fixed enum labels via `as_str()`-style APIs and avoid raw transaction/peer material. [VERIFIED: packages/open-bitcoin-network/src/peer/transaction_relay.rs; packages/open-bitcoin-mempool/src/outcome.rs; .planning/phases/104-relay-serving-fanout-and-rebroadcast-policy/104-CONTEXT.md] |
| Public propagation overclaim from local submission | Repudiation / Information Disclosure | Make RPC/internal evidence distinguish accepted, queued, suppressed, relay disabled, and `rebroadcast_deferred`; do not claim public broadcast. [VERIFIED: .planning/REQUIREMENTS.md; .planning/phases/104-relay-serving-fanout-and-rebroadcast-policy/104-CONTEXT.md] |
| txid/wtxid mode confusion | Tampering / Denial of Service | Validate inventory vectors with `TxRelayId::from_inventory_vector_for_peer` and announce with peer-negotiated `InventoryType`. [VERIFIED: packages/open-bitcoin-network/src/peer/transaction_relay.rs; packages/open-bitcoin-network/src/peer/tests.rs] |

## Sources

### Primary (HIGH confidence)

- `.planning/phases/104-relay-serving-fanout-and-rebroadcast-policy/104-CONTEXT.md` - locked Phase 104 scope, decisions, discretion, and deferred ideas. [VERIFIED: local file read]
- `.planning/REQUIREMENTS.md` - REL-01 through REL-04 requirement text. [VERIFIED: local file read]
- `.planning/ROADMAP.md` - Phase 104 purpose, scope, success criteria, and verification contract. [VERIFIED: local file read]
- `.planning/STATE.md` - current milestone state and verification caveats. [VERIFIED: local file read]
- `AGENTS.md`, `AGENTS.bright-builds.md`, and standards pages - repo and Bright Builds constraints. [VERIFIED: local file read]
- `packages/open-bitcoin-network/src/relay.rs` - Phase 100 relay eligibility classifier. [VERIFIED: local file read]
- `packages/open-bitcoin-network/src/peer.rs` and `packages/open-bitcoin-network/src/peer/inventory_state.rs` - peer actions, getdata handling, transaction handling, and direct announcement path. [VERIFIED: local file read]
- `packages/open-bitcoin-network/src/peer/transaction_relay.rs` and `scheduler.rs` - txid/wtxid identity, scheduler actions, suppression labels, fake-clock scheduling, and cleanup vocabulary. [VERIFIED: local file read]
- `packages/open-bitcoin-mempool/src/outcome.rs` and `packages/open-bitcoin-mempool/src/pool/lifecycle.rs` - admission outcome and lifecycle removal labels. [VERIFIED: local file read]
- `packages/open-bitcoin-node/src/network.rs`, `inventory.rs`, `admission_bridge.rs`, `action_translation.rs`, and `mempool_lifecycle.rs` - managed serving, storage, admission, action translation, and cleanup integration points. [VERIFIED: local file read]
- `packages/open-bitcoin-rpc/src/dispatch/node.rs` and `packages/open-bitcoin-rpc/src/method/node.rs` - current `sendrawtransaction` path and response contract. [VERIFIED: local file read]
- `scripts/verify.sh` and `scripts/check-phase103-mempool-lifecycle.ts` - verification and phase-checker patterns. [VERIFIED: local file read]
- `docs/parity/source-breadcrumbs.json` and `docs/parity/index.json` - source breadcrumb and parity registry requirements. [VERIFIED: local file read]
- `packages/bitcoin-knots/src/net_processing.cpp`, `src/node/txdownloadman.h`, `src/node/txdownloadman_impl.cpp`, `src/protocol.h`, `src/wallet/wallet.cpp`, and functional tests - pinned Knots relay and rebroadcast anchors. [VERIFIED: local submodule read]

### Secondary (MEDIUM confidence)

- OWASP ASVS 5.0 official repository and pages - ASVS category mapping for security-domain research. [CITED: https://github.com/OWASP/ASVS; https://github.com/OWASP/ASVS/tree/master/5.0/en; https://owasp.org/www-project-application-security-verification-standard/]
- OWASP ASVS V2, V13, V14, and V16 chapter pages - validation/business logic, configuration, data protection, and logging/error-handling controls. [CITED: https://github.com/OWASP/ASVS/blob/master/5.0/en/0x12-V2-Validation.md; https://github.com/OWASP/ASVS/blob/master/5.0/en/0x23-V13-Configuration.md; https://github.com/OWASP/ASVS/blob/master/5.0/en/0x24-V14-Data-Protection.md; https://github.com/OWASP/ASVS/blob/master/5.0/en/0x26-V16-Security-Logging-Error-Handling.md]

### Tertiary (LOW confidence)

- None. [VERIFIED: this research pass]

## Metadata

**Confidence breakdown:**

- Standard stack: HIGH - Existing repo crates and tool versions were verified from manifests, lockfiles, pinned tool files, and local version probes. [VERIFIED: packages/Cargo.toml; packages/Cargo.lock; rust-toolchain.toml; .bun-version; MODULE.bazel; rustc --version; cargo --version; bun --version; bazel --version]
- Architecture: HIGH - Recommended boundaries match existing functional-core/managed-shell split and verified integration points. [VERIFIED: standards/core/architecture.md; packages/open-bitcoin-network/src/relay.rs; packages/open-bitcoin-node/src/network.rs]
- Pitfalls: HIGH - Pitfalls come from locked Phase 104 decisions, existing code gaps, existing scheduler/lifecycle tests, and pinned Knots behavior anchors. [VERIFIED: .planning/phases/104-relay-serving-fanout-and-rebroadcast-policy/104-CONTEXT.md; packages/open-bitcoin-node/src/network/inventory.rs; packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs; packages/bitcoin-knots/src/net_processing.cpp]
- Security domain: MEDIUM - ASVS categories were verified from official OWASP sources, but exact control IDs should be rechecked if the implementation introduces new public RPC fields or operator surfaces. [CITED: https://github.com/OWASP/ASVS/tree/master/5.0/en; VERIFIED: .planning/phases/104-relay-serving-fanout-and-rebroadcast-policy/104-CONTEXT.md]

**Research date:** 2026-07-01
**Valid until:** 2026-07-31 for codebase/tooling findings; re-verify if relay architecture or ASVS source versions change before planning. [VERIFIED: current_date; local repo state]
