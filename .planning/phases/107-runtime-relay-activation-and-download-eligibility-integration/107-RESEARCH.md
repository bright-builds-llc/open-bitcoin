# Phase 107: Runtime Relay Activation and Download Eligibility Integration - Research

**Researched:** 2026-07-03
**Domain:** Rust Bitcoin P2P transaction relay activation, peer eligibility, transaction download scheduling, and sanitized operator evidence
**Confidence:** MEDIUM

<user_constraints>
## User Constraints (from CONTEXT.md)

Source for this full section: [VERIFIED: .planning/phases/107-runtime-relay-activation-and-download-eligibility-integration/107-CONTEXT.md]

### Locked Decisions

## Implementation Decisions

### Runtime Activation Propagation

- **D-01:** `RuntimeConfig.relay` is the source of truth for daemon/runtime
  relay activation. Runtime and RPC context construction must pass the resolved
  relay config into `ManagedPeerNetwork`; default construction must remain
  default-off.
- **D-02:** Existing constructors that intentionally use default relay behavior
  may remain for tests or compatibility, but production daemon/runtime paths
  must not accidentally instantiate `ManagedPeerNetwork` with
  `RelayActivationConfig::default()` after config loading has resolved a
  different value.
- **D-03:** The activation value should be inspectable through existing shared
  status/RPC evidence without expanding baseline-compatible RPC response shapes
  with ad hoc Open Bitcoin-only fields.

### Download Eligibility Gate

- **D-04:** Transaction announcement/download scheduling must consult Phase 100
  relay eligibility before scheduling `getdata` requests. Disabled activation,
  ordinary inbound peers, protected-only peers, and peers without scoped relay
  permission should suppress downloads with stable typed evidence.
- **D-05:** Eligibility suppression should be represented as typed scheduler or
  action vocabulary, not as a swallowed branch. Downstream status, logs, metrics,
  and tests should be able to distinguish `relay_disabled`,
  `not_relay_eligible`, `inbound_serving_required`, `permission_required`, and
  `protected_not_relay` style outcomes without exposing peer ids, endpoints,
  permission strings, txids, wtxids, or raw transaction material.
- **D-06:** Suppression must not leave stale announcement, in-flight request, or
  received-transaction cleanup state. Existing duplicate, already-have,
  recent-reject, mempool-known, request-cap, timeout, `notfound`, disconnect,
  and received-transaction cleanup behavior must continue to work.

### Peer Class Matrix

- **D-07:** Tests must prove enabled and disabled relay behavior across outbound,
  inbound, manual, protected, and permissioned peer classes. Outbound and manual
  peers require explicit activation; ordinary inbound peers remain ineligible;
  permissioned inbound peers require scoped `relay`, `forcerelay`, or `mempool`
  policy inputs; protected admission alone is not relay eligibility.
- **D-08:** `forcerelay` may remain a distinct scoped policy input, but this
  phase must not turn it into unbounded broadcast, package relay, compact block
  relay, or public propagation.
- **D-09:** Service bits, public defaults, inbound listener defaults, and compact
  block/filter behavior must remain unchanged unless a future phase explicitly
  scopes that work.

### Operator Evidence

- **D-10:** RPC/status/UAT evidence should distinguish default-off relay,
  explicitly enabled relay, eligible peers, and ineligible peers using the
  shared Phase 105 sanitized status contract where practical.
- **D-11:** `sendrawtransaction` success may still mean local admission and
  queued relay evidence inside the bounded v2.0 claim. It must not imply public
  propagation, production service readiness, or production-funds wallet safety.
- **D-12:** Metrics, logs, support bundles, and operator output must continue to
  use fixed low-cardinality labels and existing redaction boundaries. No raw
  transaction hex, txids, wtxids, peer endpoints, peer ids, permission strings,
  class names, credentials, or dynamic labels should appear.

### Deterministic Guardrails

- **D-13:** Add or update deterministic checker coverage so dropped runtime
  activation config and missing download eligibility gates fail locally before
  milestone archive.
- **D-14:** If docs, parity roots, checker fixtures, or verifier wiring change,
  keep `bash scripts/verify.sh` deterministic and public-network-free. UAT may
  describe opt-in local loopback/regtest review using repo-local Cargo and Bazel
  commands.
- **D-15:** New or touched first-party Rust source/test files under
  `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests` need parity
  breadcrumbs through `docs/parity/source-breadcrumbs.json` unless an explicit
  `none` breadcrumb is defensible.

### the agent's Discretion

The planner may choose the exact type names, constructor names, scheduler action
labels, test helpers, and whether the eligibility gate lives directly in the
transaction download scheduler or immediately before scheduler entry. Prefer the
smallest pure API that preserves Phase 100 policy ownership, keeps managed
runtime adapters thin, and avoids duplicating status/redaction logic.

### Deferred Ideas (OUT OF SCOPE)

Durable mempool relay state recovery, restart replay into relay-serving indexes,
compact block relay, package relay, bloom/filter serving, broad address relay,
public relay by default, public-network relay CI, production service operation,
production full-node readiness, production-funds wallet safety, GUI, hosted
dashboards, packaging, installer, and migration apply mode remain outside Phase
107.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| ACT-01 | Operator can enable transaction relay only through explicit relay activation settings that keep public relay off by default. [VERIFIED: .planning/REQUIREMENTS.md] | `RuntimeConfig.relay` is resolved from JSONC/CLI before context construction, but `ManagedRpcContext::from_runtime_config_with_store` currently calls `ManagedPeerNetwork::new`, which hard-codes default-off relay activation. [VERIFIED: packages/open-bitcoin-rpc/src/config.rs; packages/open-bitcoin-rpc/src/config/loader/open_bitcoin_runtime.rs; packages/open-bitcoin-rpc/src/context/network.rs; packages/open-bitcoin-node/src/network/relay_serving.rs] |
| ACT-02 | Node classifies peer relay eligibility across outbound, inbound, manual, protected, and permissioned peers without changing service bits or public defaults accidentally. [VERIFIED: .planning/REQUIREMENTS.md] | `classify_relay_eligibility` already owns the peer-class matrix and stable reason labels, and `ManagedPeerNetwork::relay_serving_context_for_peer` already adapts peer state into that classifier. [VERIFIED: packages/open-bitcoin-network/src/relay.rs; packages/open-bitcoin-node/src/network/relay_serving.rs] |
| INV-02 | Node tracks per-peer txid/wtxid negotiation, already-have state, request state, and received-transaction cleanup deterministically. [VERIFIED: .planning/REQUIREMENTS.md] | The scheduler already parses txid/wtxid identity, tracks pending requests, and clears txid/wtxid state on received transaction cleanup; the eligibility gate must run before request-state mutation. [VERIFIED: packages/open-bitcoin-network/src/peer/transaction_relay.rs; packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs] |
| INV-03 | Node handles duplicate announcements, identity mismatches, `notfound`, timeout, and disconnect cleanup without stale request state. [VERIFIED: .planning/REQUIREMENTS.md] | Duplicate, identity-mismatch, `notfound`, timeout, and disconnect behavior already exists in the scheduler and must keep working after ineligible peers are suppressed without candidate or in-flight state. [VERIFIED: packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs; packages/open-bitcoin-network/src/peer/transaction_relay/tests/scheduler_cases.rs] |
| DL-01 | Node schedules transaction downloads with bounded in-flight request caps, expiry, peer fallback, and retry evidence. [VERIFIED: .planning/REQUIREMENTS.md] | `TxDownloadScheduler` already implements caps, expiry, fallback, and request action vocabulary; Phase 107 should add eligibility suppression without changing the cap/expiry/fallback model. [VERIFIED: packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs; scripts/check-phase101-transaction-inventory-download-scheduling.ts] |
| DL-02 | Node suppresses redundant transaction requests through already-have, recent-reject, in-flight, and mempool-state checks. [VERIFIED: .planning/REQUIREMENTS.md] | `TxDownloadSuppressionReason` currently covers duplicate, already-have, recent-reject, in-flight, request-cap, identity, not-transaction-inventory, and mempool-known suppressions, but it does not cover relay eligibility suppressions yet. [VERIFIED: packages/open-bitcoin-network/src/peer/transaction_relay.rs] |
| REL-03 | Local `sendrawtransaction` submissions enter mempool admission and queued relay evidence without guaranteeing public propagation. [VERIFIED: .planning/REQUIREMENTS.md] | Phase 104/105 local submission and sanitized relay evidence already exist; Phase 107 should preserve that path while making activation and download eligibility truthful in status evidence. [VERIFIED: packages/open-bitcoin-node/src/network/relay_fanout.rs; packages/open-bitcoin-rpc/src/dispatch/tests.rs; packages/open-bitcoin-node/src/status/relay_evidence.rs] |
</phase_requirements>

## Summary

Phase 107 is an integration repair, not a new relay subsystem. [VERIFIED: .planning/ROADMAP.md] The pure activation policy already exists in `open-bitcoin-network/src/relay.rs`, the pure download scheduler already exists under `open-bitcoin-network/src/peer/transaction_relay`, and the managed fanout/serving/status paths already reuse sanitized relay evidence. [VERIFIED: packages/open-bitcoin-network/src/relay.rs; packages/open-bitcoin-network/src/peer/transaction_relay.rs; packages/open-bitcoin-node/src/network/relay_fanout.rs; packages/open-bitcoin-node/src/status/relay_evidence.rs]

The broken path is concrete: runtime config resolution produces `RuntimeConfig.relay`, but `ManagedRpcContext::from_runtime_config_with_store` constructs `ManagedPeerNetwork` through the default constructor, and that constructor passes `RelayActivationConfig::default()` plus `false` inbound serving into managed network state. [VERIFIED: packages/open-bitcoin-rpc/src/config.rs; packages/open-bitcoin-rpc/src/config/loader/open_bitcoin_runtime.rs; packages/open-bitcoin-rpc/src/context/network.rs; packages/open-bitcoin-node/src/network/relay_serving.rs] A second concrete gap exists in download scheduling: `PeerManager::handle_inventory` builds `TxAnnouncementInput` and calls `TxDownloadScheduler::record_announcement` without any relay eligibility input, and `request_orphan_parent_relay` calls `request_parent` the same way. [VERIFIED: packages/open-bitcoin-network/src/peer/inventory_state.rs; packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs]

**Primary recommendation:** Pass resolved runtime relay activation into managed network construction, add relay eligibility as typed input before transaction download scheduling mutates pending state, and expose only aggregate low-cardinality activation/eligibility evidence through the shared Phase 105 status contract and deterministic Phase 107 checker. [VERIFIED: .planning/ROADMAP.md; packages/open-bitcoin-rpc/src/context/network.rs; packages/open-bitcoin-network/src/peer/inventory_state.rs; packages/open-bitcoin-node/src/status/relay_evidence.rs]

## Project Constraints (from AGENTS.md)

- Use `git submodule update --init --recursive` when the pinned Knots baseline under `packages/bitcoin-knots` is needed. [VERIFIED: AGENTS.md]
- Treat `rust-toolchain.toml` as the Rust source of truth; the repo pins Rust `1.94.1`. [VERIFIED: AGENTS.md; rust-toolchain.toml]
- Use `bash scripts/verify.sh` as the repo-native verification contract, including Bazel smoke in the full profile. [VERIFIED: AGENTS.md; scripts/verify.sh]
- Use `bash scripts/verify.sh --fast` only for local iteration and keep the default command as the pre-commit and release contract. [VERIFIED: AGENTS.md; scripts/verify.sh]
- Provide repo-local Cargo and Bazel UAT commands for operator workflows instead of relying on an installed `open-bitcoin` alias. [VERIFIED: AGENTS.md; docs/operator/runtime-guide.md]
- Use Bun for repo-owned higher-level automation scripts and TypeScript for substantial script logic. [VERIFIED: AGENTS.md; .bun-version; scripts/check-phase105-operator-relay-evidence.ts]
- Treat `docs/metrics/lines-of-code.md` as an intentionally tracked generated artifact. [VERIFIED: AGENTS.md; scripts/verify.sh]
- Record intentional behavior differences from Bitcoin Knots in `docs/parity/index.json` and companion docs under `docs/parity/`. [VERIFIED: AGENTS.md; docs/parity/index.json]
- Add parity breadcrumbs in `docs/parity/source-breadcrumbs.json` for new or touched first-party Rust source or test files under `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests`. [VERIFIED: AGENTS.md; docs/parity/source-breadcrumbs.json; scripts/check-parity-breadcrumbs.ts]
- Check whether relevant README files need updates after substantial feature, parity, operator-surface, or workflow changes. [VERIFIED: AGENTS.md]
- Preserve functional-core versus imperative-shell boundaries by keeping pure business decisions in core modules and adapter I/O in shell modules. [VERIFIED: AGENTS.bright-builds.md; standards/core/architecture.md]
- Prefer early returns, clear names, small functions, `foo.rs` plus `foo/` module layout, and no `unwrap()` in Rust production code. [VERIFIED: standards/core/code-shape.md; standards/languages/rust.md]
- Unit tests should test one concern and use Arrange, Act, Assert comments when useful. [VERIFIED: AGENTS.md; standards/core/testing.md]

## Standard Stack

### Core

| Library / Module | Version | Purpose | Why Standard |
|------------------|---------|---------|--------------|
| Rust workspace | `1.94.1`, edition `2024` | First-party implementation language and workspace model. [VERIFIED: rust-toolchain.toml; packages/Cargo.toml; rustc --version] | The repo pins this toolchain and all first-party crates live in the Cargo workspace. [VERIFIED: AGENTS.md; packages/Cargo.toml] |
| `open-bitcoin-network` | workspace path `0.1.0` | Pure peer, relay activation, transaction identity, and download scheduling logic. [VERIFIED: packages/Cargo.toml; packages/open-bitcoin-network/src/relay.rs; packages/open-bitcoin-network/src/peer/transaction_relay.rs] | Phase 100 and Phase 101 already established this crate as the pure relay/download policy owner. [VERIFIED: docs/parity/checklist.md; docs/parity/source-breadcrumbs.json] |
| `open-bitcoin-node` | workspace path `0.1.0` | Managed network adapter, relay serving cache, fanout state, and shared status projection. [VERIFIED: packages/Cargo.toml; packages/open-bitcoin-node/src/network.rs; packages/open-bitcoin-node/src/network/relay_fanout.rs; packages/open-bitcoin-node/src/status/relay_evidence.rs] | This crate owns the shell boundary that translates pure peer actions into network messages and status evidence. [VERIFIED: packages/open-bitcoin-node/src/network.rs; packages/open-bitcoin-node/src/network/action_translation.rs] |
| `open-bitcoin-rpc` | workspace path `0.1.0` | Runtime config, daemon context construction, and RPC response projection. [VERIFIED: packages/Cargo.toml; packages/open-bitcoin-rpc/src/config.rs; packages/open-bitcoin-rpc/src/context/network.rs; packages/open-bitcoin-rpc/src/dispatch/node.rs] | Runtime activation is resolved in this crate before managed network construction. [VERIFIED: packages/open-bitcoin-rpc/src/config/loader/open_bitcoin_runtime.rs] |
| Bitcoin Knots pinned baseline | `29.3.knots20260210` | Behavioral anchor for permissions, tx download, INV, and getdata parity. [VERIFIED: AGENTS.md; .planning/PROJECT.md; packages/bitcoin-knots/src/net_permissions.h; packages/bitcoin-knots/src/node/txdownloadman_impl.cpp; packages/bitcoin-knots/src/net_processing.cpp] | Phase 107 must preserve externally observable in-scope Knots behavior while staying within the bounded Open Bitcoin v2.0 claim. [VERIFIED: AGENTS.md; docs/parity/index.json] |

### Supporting

| Library / Tool | Version | Purpose | When to Use |
|----------------|---------|---------|-------------|
| `serde` / `serde_json` | `serde 1.0.228`, `serde_json 1.0.149` | Stable JSON status and RPC data shapes. [VERIFIED: packages/open-bitcoin-rpc/Cargo.toml; packages/open-bitcoin-node/src/status/relay_evidence.rs; packages/Cargo.lock] | Use for any shared activation/eligibility status contract extension. [VERIFIED: packages/open-bitcoin-node/src/status/relay_evidence.rs; packages/open-bitcoin-rpc/src/method/node.rs] |
| `jsonc-parser` | `0.32.3` | Open Bitcoin JSONC config parsing. [VERIFIED: packages/open-bitcoin-rpc/Cargo.toml; packages/open-bitcoin-rpc/src/config/open_bitcoin.rs] | Use existing config loader tests when proving `relay.enabled` propagation. [VERIFIED: packages/open-bitcoin-rpc/src/config/tests.rs] |
| `axum` / `tokio` | `axum 0.8.9`, `tokio 1.52.1` | RPC server runtime. [VERIFIED: packages/open-bitcoin-rpc/Cargo.toml; packages/Cargo.lock] | No new async/server logic is needed for Phase 107. [VERIFIED: .planning/ROADMAP.md; packages/open-bitcoin-rpc/src/dispatch/node.rs] |
| Bun | `1.3.9` | Deterministic TypeScript checkers and checker tests. [VERIFIED: .bun-version; bun --version; scripts/verify.sh] | Add the Phase 107 checker/test pair and wire it through `scripts/verify.sh`. [VERIFIED: scripts/check-phase105-operator-relay-evidence.ts; scripts/verify.sh] |
| Bazelisk / Bazel | Bazelisk `1.28.1`, Bazel `8.6.0` | Full verification smoke build and repo-local UAT command form. [VERIFIED: bazelisk version; scripts/verify.sh; AGENTS.md] | Keep UAT examples in both Cargo and Bazel forms. [VERIFIED: AGENTS.md; docs/operator/runtime-guide.md] |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Add a new relay policy model in `open-bitcoin-node` | Reuse `classify_relay_eligibility` from `open-bitcoin-network` | Reusing the Phase 100 classifier avoids divergent peer-class semantics and preserves pure policy ownership. [VERIFIED: packages/open-bitcoin-network/src/relay.rs; packages/open-bitcoin-node/src/network/relay_serving.rs; .planning/phases/107-runtime-relay-activation-and-download-eligibility-integration/107-CONTEXT.md] |
| Filter `GetData` messages in `ManagedPeerNetwork::process_actions` | Gate before `TxDownloadScheduler::record_announcement` and `request_parent` mutate state | Post-translation filtering would occur after scheduler candidate/in-flight state has already been inserted. [VERIFIED: packages/open-bitcoin-node/src/network.rs; packages/open-bitcoin-node/src/network/action_translation.rs; packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs] |
| Add baseline `getnetworkinfo` fields | Extend or reuse `openbitcoinnetworkstatus.relay` and `OpenBitcoinStatusSnapshot.mempool.relay` | `getnetworkinfo.localrelay` already represents the baseline-compatible local relay flag, while Open Bitcoin relay evidence is projected through the Open Bitcoin-specific status/RPC contract. [VERIFIED: packages/open-bitcoin-rpc/src/method/node.rs; packages/open-bitcoin-rpc/src/dispatch/node.rs; docs/architecture/status-snapshot.md] |
| Add a new package dependency | Use existing Rust modules and Bun checker pattern | The phase can be implemented with existing workspace crates, serde status contracts, and existing checker infrastructure. [VERIFIED: packages/Cargo.toml; scripts/verify.sh; scripts/check-phase101-transaction-inventory-download-scheduling.ts] |

**Installation:**

```bash
# No new package installation is recommended for Phase 107.
```

**Version verification:** Rust, Cargo, Bun, Bash, Git, and Bazelisk/Bazel versions were verified locally, and no new npm package is recommended. [VERIFIED: rustc --version; cargo --version; bun --version; bash --version; git --version; bazelisk version]

## Architecture Patterns

### Recommended Project Structure

```text
packages/open-bitcoin-network/src/
├── relay.rs                                  # Existing Phase 100 activation and peer eligibility policy
├── peer.rs                                   # PeerManager owns pure peer state and should carry relay download activation inputs
├── peer/inventory_state.rs                   # INV and orphan parent scheduling entry points
└── peer/transaction_relay/
    ├── scheduler.rs                          # Pure download scheduling and eligibility suppression before state mutation
    └── tests/scheduler_cases*.rs             # Focused scheduler coverage

packages/open-bitcoin-node/src/
├── network/relay_serving.rs                  # Managed peer eligibility adapter and constructor storage
├── network/relay_fanout.rs                   # Shared sanitized relay evidence projection
├── network/tests.rs                          # Managed getdata/runtime integration coverage
└── status/relay_evidence.rs                  # Shared status contract extension if needed

packages/open-bitcoin-rpc/src/
├── context/network.rs                        # RuntimeConfig -> ManagedPeerNetwork construction
├── context/tests.rs                          # Runtime propagation tests
└── dispatch/tests.rs                         # Open Bitcoin status/RPC evidence tests

scripts/
├── check-phase107-runtime-relay-activation-download-eligibility.ts
└── check-phase107-runtime-relay-activation-download-eligibility.test.ts
```

The structure above follows existing source ownership and checker patterns. [VERIFIED: docs/parity/source-breadcrumbs.json; scripts/verify.sh; .planning/phases/107-runtime-relay-activation-and-download-eligibility-integration/107-CONTEXT.md]

### Pattern 1: Runtime Activation Propagation

**What:** Production runtime context construction should call `ManagedPeerNetwork::new_with_relay_activation` with `config.relay` instead of calling `ManagedPeerNetwork::new`. [VERIFIED: packages/open-bitcoin-rpc/src/context/network.rs; packages/open-bitcoin-node/src/network/relay_serving.rs]

**When to use:** Use this in `ManagedRpcContext::from_runtime_config_with_store`, because that is the production-like path from resolved `RuntimeConfig` into managed network state. [VERIFIED: packages/open-bitcoin-rpc/src/context/network.rs]

**Example:**

```rust
// Source: packages/open-bitcoin-rpc/src/context/network.rs and packages/open-bitcoin-node/src/network/relay_serving.rs
let mut managed_network = ManagedPeerNetwork::new_with_relay_activation(
    MemoryChainstateStore::default(),
    local_config,
    PolicyConfig::default(),
    config.relay,
    config.inbound.enabled,
);
```

The inbound-serving boolean is resolved for Phase 107: use `config.inbound.enabled` as the deterministic managed-construction input, while optional live listener evidence stays outside default verification and public-network proof. [VERIFIED: packages/open-bitcoin-node/src/network/relay_serving.rs; packages/open-bitcoin-rpc/src/context/network.rs] [RESOLVED]

### Pattern 2: Eligibility Before Scheduler Mutation

**What:** Add relay eligibility to the scheduler input or compute it immediately before scheduler entry, then emit typed suppression actions without inserting candidates or in-flight requests. [VERIFIED: packages/open-bitcoin-network/src/peer/inventory_state.rs; packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs]

**When to use:** Use this for both transaction announcements from `handle_inventory` and orphan parent requests from `request_orphan_parent_relay`. [VERIFIED: packages/open-bitcoin-network/src/peer/inventory_state.rs]

**Example:**

```rust
// Source: packages/open-bitcoin-network/src/relay.rs and packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs
if let Some(action) = relay_eligibility_suppression(
    input.peer_id,
    relay_id,
    &input.relay_eligibility,
) {
    return vec![action];
}
```

The suppression helper should map `RelayEligibilityReason::Disabled` or `ActivationRequired` to a relay-disabled style download suppression and map `InboundServingRequired`, `PermissionRequired`, `ProtectedNotRelay`, and `PermissionEffectInactive` to stable low-cardinality ineligibility reasons. [VERIFIED: packages/open-bitcoin-network/src/relay.rs; .planning/phases/107-runtime-relay-activation-and-download-eligibility-integration/107-CONTEXT.md]

### Pattern 3: Store Activation Inputs in Pure Peer State

**What:** Give `PeerManager` a small relay download eligibility policy field, such as `{ activation: RelayActivationConfig, inbound_serving_enabled: bool }`, default it off in `PeerManager::new`, and set it from managed network constructors. [VERIFIED: packages/open-bitcoin-network/src/peer.rs; packages/open-bitcoin-node/src/network/relay_serving.rs]

**When to use:** Use this when keeping `PeerManager::handle_inventory` as the entry point for INV processing, because `ManagedPeerNetwork` currently sees transaction relay actions only after scheduling has already happened. [VERIFIED: packages/open-bitcoin-network/src/peer/inventory_state.rs; packages/open-bitcoin-node/src/network.rs]

**Example:**

```rust
// Source: packages/open-bitcoin-network/src/peer.rs and packages/open-bitcoin-network/src/relay.rs
let relay_eligibility = self.relay_download_policy.classify_peer(peer);
transaction_inputs.push(TxAnnouncementInput {
    peer_id,
    inventory: item.clone(),
    peer_mode,
    now_unix_seconds: timestamp,
    local_facts: local_facts.clone(),
    relay_eligibility,
    preferred_peer: true,
    peer_overloaded: false,
});
```

The planner should keep the classification helper pure and should derive peer class and permission effects from existing `PeerState.maybe_inbound_record` and `InboundPermissionDecision` rather than parsing permission strings again. [VERIFIED: packages/open-bitcoin-network/src/peer.rs; packages/open-bitcoin-network/src/peer/inbound_state.rs; packages/open-bitcoin-network/src/inbound/permissions.rs]

### Pattern 4: Sanitized Evidence Through Shared Status

**What:** Keep activation and eligibility evidence inside `RelayEvidenceStatus` or the existing `openbitcoinnetworkstatus.relay` projection, using aggregate counts and fixed labels only. [VERIFIED: packages/open-bitcoin-node/src/status/relay_evidence.rs; packages/open-bitcoin-rpc/src/method/node.rs; docs/architecture/status-snapshot.md]

**When to use:** Use this if tests or UAT need to distinguish default-off relay from explicitly enabled relay before any public propagation claim. [VERIFIED: .planning/ROADMAP.md; .planning/phases/107-runtime-relay-activation-and-download-eligibility-integration/107-CONTEXT.md]

**Example:**

```rust
// Source: packages/open-bitcoin-node/src/status/relay_evidence.rs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RelayActivationEvidence {
    pub enabled: bool,
}
```

If the status contract changes, update CLI status, dashboard, support rendering, metrics/log redaction tests, docs, and the Phase 105 checker expectations only through shared low-cardinality fields. [VERIFIED: packages/open-bitcoin-cli/src/operator/status/render/relay.rs; packages/open-bitcoin-cli/src/operator/dashboard/model/relay.rs; packages/open-bitcoin-cli/src/operator/support/render/relay.rs; scripts/check-phase105-operator-relay-evidence.ts]

### Anti-Patterns to Avoid

- **Managed-network post-filtering:** Do not drop `GetData` only in `process_transaction_relay_action`, because scheduler state may already contain in-flight or fallback candidates. [VERIFIED: packages/open-bitcoin-node/src/network.rs; packages/open-bitcoin-node/src/network/action_translation.rs; packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs]
- **Second eligibility model:** Do not duplicate outbound/inbound/manual/protected/permissioned semantics outside `classify_relay_eligibility`. [VERIFIED: packages/open-bitcoin-network/src/relay.rs; packages/open-bitcoin-node/src/network/relay_serving.rs]
- **Protected means relay:** Do not treat protected admission or eviction protection as transaction relay eligibility unless scoped relay, forcerelay, or mempool effects are present. [VERIFIED: packages/open-bitcoin-network/src/relay.rs; packages/open-bitcoin-network/src/inbound/permissions.rs; .planning/phases/107-runtime-relay-activation-and-download-eligibility-integration/107-CONTEXT.md]
- **Baseline RPC shape drift:** Do not add ad hoc activation fields to baseline-compatible `getnetworkinfo`; use Open Bitcoin-specific status evidence. [VERIFIED: packages/open-bitcoin-rpc/src/method/node.rs; packages/open-bitcoin-rpc/src/dispatch/node.rs; .planning/phases/107-runtime-relay-activation-and-download-eligibility-integration/107-CONTEXT.md]
- **Dynamic evidence labels:** Do not expose peer ids, endpoints, permission strings, txids, wtxids, raw transaction hex, credentials, or dynamic labels in status/logs/metrics/support bundles. [VERIFIED: .planning/phases/107-runtime-relay-activation-and-download-eligibility-integration/107-CONTEXT.md; scripts/check-phase105-operator-relay-evidence.ts]

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Relay activation and peer-class eligibility | A new local enum or ad hoc boolean branches in managed network code | `classify_relay_eligibility`, `RelayEligibilityInput`, and `RelayEligibilityDecision` | Phase 100 already owns the activation matrix and reason labels. [VERIFIED: packages/open-bitcoin-network/src/relay.rs; scripts/check-phase100-relay-activation-boundary.ts] |
| Txid/wtxid inventory identity | Manual hash/type interpretation at the gate | `TxRelayId::from_inventory_vector_for_peer` and `TxRelayPeerMode` | The scheduler already handles negotiated txid/wtxid identity and identity mismatch suppression. [VERIFIED: packages/open-bitcoin-network/src/peer/transaction_relay.rs] |
| Download request caps, expiry, fallback, and cleanup | A separate request queue outside the scheduler | `TxDownloadScheduler` and existing `TxDownloadAction` vocabulary | Phase 101 already owns request caps, expiry, fallback, `notfound`, disconnect, and received cleanup state. [VERIFIED: packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs; scripts/check-phase101-transaction-inventory-download-scheduling.ts] |
| Permission parsing | Re-reading raw permission strings in scheduler code | `InboundPermissionDecision::relay_permission_effects()` and `inactive_effects()` | The inbound permission layer already resolves scoped relay, forcerelay, mempool, and inactive effects. [VERIFIED: packages/open-bitcoin-network/src/inbound/permissions.rs] |
| Sanitized status/rendering | Surface-specific relay summaries | `RelayEvidenceStatus`, `RelayEvidenceField`, `RelayEvidenceCounters`, and existing renderers | Phase 105 already centralizes relay evidence and redaction boundaries. [VERIFIED: packages/open-bitcoin-node/src/status/relay_evidence.rs; packages/open-bitcoin-cli/src/operator/status/render/relay.rs; packages/open-bitcoin-cli/src/operator/support/render/relay.rs] |
| Deterministic phase guardrails | One-off shell checks or public-network validation | Bun TypeScript checker plus companion Bun test wired through `scripts/verify.sh` | Existing Phase 100-106 guardrails use this pattern and default verification is deterministic. [VERIFIED: scripts/verify.sh; scripts/check-phase101-transaction-inventory-download-scheduling.ts; scripts/check-phase106-parity-uat-release-boundary.ts] |

**Key insight:** The phase should connect existing policy, scheduler, and status seams; custom relay or request machinery would increase divergence from prior v2.0 evidence and make stale request-state bugs harder to detect. [VERIFIED: .planning/ROADMAP.md; packages/open-bitcoin-network/src/relay.rs; packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs]

## Common Pitfalls

### Pitfall 1: Fixing Only Test Helpers

**What goes wrong:** RPC tests can pass by manually constructing `ManagedPeerNetwork::new_with_relay_activation`, while `ManagedRpcContext::from_runtime_config` still drops runtime activation. [VERIFIED: packages/open-bitcoin-rpc/src/dispatch/tests.rs; packages/open-bitcoin-rpc/src/context/network.rs]

**Why it happens:** Current test helpers bypass the production-like runtime config construction path. [VERIFIED: packages/open-bitcoin-rpc/src/dispatch/tests.rs]

**How to avoid:** Add a context/runtime propagation test that starts from `RuntimeConfig { relay: RelayActivationConfig { enabled: true }, .. }` and observes managed relay state or status evidence through `ManagedRpcContext`. [VERIFIED: packages/open-bitcoin-rpc/src/config.rs; packages/open-bitcoin-rpc/src/context/tests.rs]

**Warning signs:** A test still calls `ManagedPeerNetwork::new_with_relay_activation` directly for runtime propagation coverage. [VERIFIED: packages/open-bitcoin-rpc/src/dispatch/tests.rs]

### Pitfall 2: Filtering After Scheduler Mutation

**What goes wrong:** A disabled or ineligible peer may fail to send `getdata` but still leave announcement candidates or in-flight state inside `TxDownloadScheduler`. [VERIFIED: packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs; packages/open-bitcoin-node/src/network/action_translation.rs]

**Why it happens:** `ManagedPeerNetwork::process_actions` translates already-created `PeerAction::TransactionRelay` actions after `PeerManager` has called scheduler methods. [VERIFIED: packages/open-bitcoin-node/src/network.rs; packages/open-bitcoin-network/src/peer/inventory_state.rs]

**How to avoid:** Perform eligibility suppression before `insert_in_flight`, `insert_candidate`, or fallback candidate insertion. [VERIFIED: packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs]

**Warning signs:** New code checks eligibility in `process_transaction_relay_action` or only around `maybe_request_inventory`. [VERIFIED: packages/open-bitcoin-node/src/network/action_translation.rs]

### Pitfall 3: Ordinary Inbound Still Downloads Transactions

**What goes wrong:** Existing tests create ordinary inbound peers and expect `getdata`; after Phase 107, those expectations contradict locked eligibility rules. [VERIFIED: packages/open-bitcoin-node/src/network/tests.rs; packages/open-bitcoin-network/src/peer/tests.rs; .planning/phases/107-runtime-relay-activation-and-download-eligibility-integration/107-CONTEXT.md]

**Why it happens:** Phase 101 predates runtime relay eligibility integration and tested transaction download mechanics without activation gates. [VERIFIED: docs/parity/checklist.md; scripts/check-phase101-transaction-inventory-download-scheduling.ts]

**How to avoid:** Split tests into default-off/ineligible suppression cases and explicit enabled eligible peer request cases. [VERIFIED: packages/open-bitcoin-network/src/relay.rs; packages/open-bitcoin-node/src/network/tests/relay_fanout_cases.rs]

**Warning signs:** A test uses `PeerManager::new(...).add_inbound_peer(...)` and still expects `request_getdata` without explicit relay activation and scoped permission. [VERIFIED: packages/open-bitcoin-network/src/peer/tests.rs; packages/open-bitcoin-network/src/peer/inbound_state.rs]

### Pitfall 4: Losing Fallback and Cleanup Semantics

**What goes wrong:** Suppressing an ineligible first announcer can block an eligible alternate announcer from requesting the transaction, or cleanup paths can retain stale state. [VERIFIED: .planning/phases/107-runtime-relay-activation-and-download-eligibility-integration/107-CONTEXT.md; packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs]

**Why it happens:** Duplicate/fallback logic and request cleanup are stateful, so adding a gate in the wrong order changes later fallback decisions. [VERIFIED: packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs; packages/open-bitcoin-network/src/peer/transaction_relay/tests/scheduler_cases.rs]

**How to avoid:** Add tests for disabled first announcer plus eligible second announcer, ineligible fallback candidate suppression, timeout fallback, `notfound`, disconnect, and received cleanup. [VERIFIED: packages/open-bitcoin-network/src/peer/transaction_relay/tests/scheduler_cases.rs; .planning/phases/107-runtime-relay-activation-and-download-eligibility-integration/107-CONTEXT.md]

**Warning signs:** New eligibility tests only assert no outgoing `getdata` and do not inspect scheduler peer snapshots or cleanup actions. [VERIFIED: packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs]

### Pitfall 5: Treating `forcerelay` As Public Propagation

**What goes wrong:** `forcerelay` could be accidentally interpreted as unbounded broadcast or public relay readiness. [VERIFIED: .planning/phases/107-runtime-relay-activation-and-download-eligibility-integration/107-CONTEXT.md]

**Why it happens:** Bitcoin Knots permissions imply specific relay behavior, but Phase 107 is bounded to scoped eligibility/download scheduling and local evidence. [VERIFIED: packages/bitcoin-knots/src/net_permissions.h; packages/bitcoin-knots/src/net_processing.cpp; .planning/PROJECT.md]

**How to avoid:** Keep `forcerelay` as `ForceRelayPolicyInput` in relay eligibility and do not change compact block, package, bloom/filter, service bits, or public defaults. [VERIFIED: packages/open-bitcoin-network/src/inbound/permissions.rs; packages/open-bitcoin-network/src/relay.rs; .planning/phases/107-runtime-relay-activation-and-download-eligibility-integration/107-CONTEXT.md]

**Warning signs:** Docs or status text claims public propagation, package relay, compact block relay, bloom/filter serving, or production service readiness. [VERIFIED: scripts/check-phase105-operator-relay-evidence.ts; scripts/check-phase106-parity-uat-release-boundary.ts]

## Code Examples

### Runtime Config To Managed Network

```rust
// Source: packages/open-bitcoin-rpc/src/context/network.rs
// Source: packages/open-bitcoin-node/src/network/relay_serving.rs
let mut managed_network = ManagedPeerNetwork::new_with_relay_activation(
    MemoryChainstateStore::default(),
    local_config,
    PolicyConfig::default(),
    config.relay,
    config.inbound.enabled,
);
```

This should replace the production-like `ManagedPeerNetwork::new(...)` call inside `ManagedRpcContext::from_runtime_config_with_store`. [VERIFIED: packages/open-bitcoin-rpc/src/context/network.rs; packages/open-bitcoin-node/src/network/relay_serving.rs]

### Relay Eligibility Input From Peer State

```rust
// Source: packages/open-bitcoin-network/src/relay.rs
// Source: packages/open-bitcoin-network/src/peer.rs
// Source: packages/open-bitcoin-network/src/inbound/permissions.rs
let relay_eligibility = classify_relay_eligibility(&RelayEligibilityInput {
    activation: self.relay_download_policy.activation,
    inbound_serving_enabled: self.relay_download_policy.inbound_serving_enabled,
    connection_class,
    relay_permission_effects,
    inactive_permission_effects,
});
```

The `connection_class`, `relay_permission_effects`, and `inactive_permission_effects` should come from existing peer role and inbound admission records. [VERIFIED: packages/open-bitcoin-network/src/peer.rs; packages/open-bitcoin-network/src/peer/inbound_state.rs; packages/open-bitcoin-network/src/inbound/permissions.rs]

### Scheduler Suppression Vocabulary

```rust
// Source: packages/open-bitcoin-network/src/relay.rs
// Source: packages/open-bitcoin-network/src/peer/transaction_relay.rs
fn relay_download_suppression_reason(
    decision: &RelayEligibilityDecision,
) -> Option<TxDownloadSuppressionReason> {
    match decision.reason {
        RelayEligibilityReason::Eligible => None,
        RelayEligibilityReason::Disabled | RelayEligibilityReason::ActivationRequired => {
            Some(TxDownloadSuppressionReason::RelayDisabled)
        }
        RelayEligibilityReason::InboundServingRequired => {
            Some(TxDownloadSuppressionReason::InboundServingRequired)
        }
        RelayEligibilityReason::PermissionRequired
        | RelayEligibilityReason::PermissionEffectInactive => {
            Some(TxDownloadSuppressionReason::PermissionRequired)
        }
        RelayEligibilityReason::ProtectedNotRelay => {
            Some(TxDownloadSuppressionReason::ProtectedNotRelay)
        }
    }
}
```

The exact variant names are at the planner's discretion, but the labels should remain fixed and low-cardinality. [VERIFIED: .planning/phases/107-runtime-relay-activation-and-download-eligibility-integration/107-CONTEXT.md]

## State of the Art

| Old Approach | Current / Recommended Approach | When Changed | Impact |
|--------------|--------------------------------|--------------|--------|
| Phase 100 defined relay activation and peer eligibility policy without scheduling integration. [VERIFIED: docs/parity/checklist.md; packages/open-bitcoin-network/src/relay.rs] | Phase 107 should reuse that policy before download scheduling and runtime managed-network construction. [VERIFIED: .planning/ROADMAP.md] | Phase 107 planning date 2026-07-03. [VERIFIED: .planning/phases/107-runtime-relay-activation-and-download-eligibility-integration/107-CONTEXT.md] | ACT-01/ACT-02 can move from policy-only to runtime-integrated evidence. [VERIFIED: .planning/REQUIREMENTS.md; .planning/ROADMAP.md] |
| Phase 101 scheduler tests allowed ordinary inbound transaction download mechanics without activation eligibility. [VERIFIED: scripts/check-phase101-transaction-inventory-download-scheduling.ts; packages/open-bitcoin-network/src/peer/tests.rs] | Phase 107 should require explicit activation and eligible peer class before `getdata` scheduling. [VERIFIED: .planning/ROADMAP.md; .planning/phases/107-runtime-relay-activation-and-download-eligibility-integration/107-CONTEXT.md] | Phase 107. [VERIFIED: .planning/ROADMAP.md] | Existing request, fallback, and cleanup tests need updated eligible-peer helpers and new ineligible suppression cases. [VERIFIED: packages/open-bitcoin-network/src/peer/transaction_relay/tests/scheduler_cases.rs; packages/open-bitcoin-node/src/network/tests.rs] |
| Phase 104 fanout/serving eligibility is already managed by `RelayEligibilityDecision`. [VERIFIED: packages/open-bitcoin-node/src/network/relay_fanout.rs; packages/open-bitcoin-node/src/network/relay_serving.rs] | Download scheduling should align with the same eligibility decision and safe label vocabulary. [VERIFIED: packages/open-bitcoin-network/src/relay.rs; packages/open-bitcoin-network/src/peer/transaction_relay/fanout.rs] | Phase 107. [VERIFIED: .planning/ROADMAP.md] | Relay serving, fanout, local submission, and download scheduling can share one policy vocabulary. [VERIFIED: packages/open-bitcoin-network/src/relay.rs; packages/open-bitcoin-node/src/network/relay_fanout.rs] |
| Phase 105 status evidence reports counters and capability fields but no explicit activation field. [VERIFIED: packages/open-bitcoin-node/src/status/relay_evidence.rs] | Phase 107 should add or reuse sanitized shared status evidence to distinguish default-off, enabled, eligible, and ineligible states. [VERIFIED: .planning/ROADMAP.md; .planning/phases/107-runtime-relay-activation-and-download-eligibility-integration/107-CONTEXT.md] | Phase 107. [VERIFIED: .planning/ROADMAP.md] | Operator/RPC/UAT evidence can prove activation without baseline RPC response drift. [VERIFIED: packages/open-bitcoin-rpc/src/method/node.rs; docs/architecture/status-snapshot.md] |

**Deprecated/outdated:**

- Treating `ManagedPeerNetwork::new` as acceptable production runtime construction after relay config resolution is outdated for Phase 107. [VERIFIED: packages/open-bitcoin-rpc/src/context/network.rs; packages/open-bitcoin-node/src/network/relay_serving.rs; .planning/phases/107-runtime-relay-activation-and-download-eligibility-integration/107-CONTEXT.md]
- Treating ordinary inbound tx announcements as requestable by default is outdated for Phase 107. [VERIFIED: packages/open-bitcoin-network/src/relay.rs; packages/open-bitcoin-network/src/peer/tests.rs; .planning/phases/107-runtime-relay-activation-and-download-eligibility-integration/107-CONTEXT.md]

## Resolved Decisions

| Decision | Resolution | Planning Impact |
|----------|------------|-----------------|
| Inbound serving eligibility source | **RESOLVED:** Phase 107 managed construction and deterministic tests use resolved `config.inbound.enabled` as the `inbound_serving_enabled` input. Live listener evidence remains outside default verification, public-network proof, and Phase 107 completion gates. | Plans should pass `config.inbound.enabled` from `ManagedRpcContext::from_runtime_config_with_store` into `ManagedPeerNetwork::new_with_relay_activation`, and UAT/checkers must keep public-network listener proof opt-in/out of default verification. |
| Public evidence granularity | **RESOLVED:** Granular scheduler eligibility labels remain typed/internal for scheduler actions and tests, while public/operator status exposes aggregate sanitized counters and fixed labels only. | Plans should preserve `relay_disabled`, `not_relay_eligible`, `inbound_serving_required`, `permission_required`, and `protected_not_relay` as typed scheduler/test vocabulary, then project public status as aggregate low-cardinality counts without peer ids, endpoints, permission strings, txids, wtxids, raw transactions, or dynamic labels. |

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | `config.inbound.enabled` is the selected Phase 107 implementation input for `inbound_serving_enabled` when constructing the managed network from runtime config. [RESOLVED] | Architecture Patterns / Runtime Activation Propagation | Future live-listener evidence may tighten eligibility in a later phase, but Phase 107 default verification and public evidence intentionally remain deterministic and public-network-free. |

## Open Questions — RESOLVED

1. **RESOLVED: Inbound serving eligibility uses resolved config for Phase 107.** [VERIFIED: packages/open-bitcoin-rpc/src/context/network.rs; packages/open-bitcoin-node/src/network/relay_serving.rs]
   - What we know: `ManagedPeerNetwork::new_with_relay_activation` already takes `inbound_serving_enabled`, and `RuntimeConfig` carries `inbound.enabled`. [VERIFIED: packages/open-bitcoin-node/src/network/relay_serving.rs; packages/open-bitcoin-rpc/src/config.rs]
   - Selected decision: Use resolved `config.inbound.enabled` for Phase 107 managed construction and deterministic tests. Live listener evidence remains out of default verification and public-network proof. [RESOLVED]
   - Planning consequence: Plan 107-02 should pass `config.inbound.enabled` through runtime construction, and docs/checkers should keep live listener/public-network evidence outside the default verifier. [RESOLVED]

2. **RESOLVED: Public evidence uses aggregate sanitized counters, not per-peer/free-form reason material.** [VERIFIED: .planning/phases/107-runtime-relay-activation-and-download-eligibility-integration/107-CONTEXT.md]
   - What we know: Tests need typed reasons such as `relay_disabled`, `not_relay_eligible`, `inbound_serving_required`, `permission_required`, and `protected_not_relay`. [VERIFIED: .planning/phases/107-runtime-relay-activation-and-download-eligibility-integration/107-CONTEXT.md]
   - Selected decision: Keep granular scheduler eligibility labels typed/internal for scheduler actions and tests, while public/operator status exposes aggregate sanitized counters and fixed labels only. [RESOLVED]
   - Planning consequence: Plan 107-03 should expose `RelayDownloadEligibilityCounters` as low-cardinality aggregate status evidence and must not expose peer ids, endpoints, permission strings, txids, wtxids, raw transactions, class names, credentials, or dynamic labels. [RESOLVED]

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|-------------|-----------|---------|----------|
| Rust compiler | Cargo workspace implementation and tests | Yes [VERIFIED: rustc --version] | `rustc 1.94.1 (e408947bf 2026-03-25)` [VERIFIED: rustc --version] | None needed. [VERIFIED: rust-toolchain.toml] |
| Cargo | Format, clippy, build, tests, UAT commands | Yes [VERIFIED: cargo --version] | `cargo 1.94.1 (29ea6fb6a 2026-03-24)` [VERIFIED: cargo --version] | None needed. [VERIFIED: scripts/verify.sh] |
| Bun | TypeScript checkers and checker tests | Yes [VERIFIED: bun --version] | `1.3.9` [VERIFIED: bun --version; .bun-version] | None needed. [VERIFIED: scripts/verify.sh] |
| Bash | Repo verification wrapper | Yes [VERIFIED: bash --version] | `GNU bash 3.2.57(1)-release` [VERIFIED: bash --version] | None needed. [VERIFIED: scripts/verify.sh] |
| Bazelisk / Bazel | Full verification smoke build and Bazel UAT form | Yes [VERIFIED: bazelisk version] | Bazelisk `1.28.1`, Bazel `8.6.0` [VERIFIED: bazelisk version] | Use Cargo-only iteration before full verification, but final verification still requires Bazel. [VERIFIED: scripts/verify.sh; AGENTS.md] |
| Git | Status/diff inspection and parent workflow commit | Yes [VERIFIED: git --version] | `git version 2.53.0` [VERIFIED: git --version] | None needed. [VERIFIED: AGENTS.md] |

**Missing dependencies with no fallback:** None found for Phase 107 planning. [VERIFIED: rustc --version; cargo --version; bun --version; bazelisk version]

**Missing dependencies with fallback:** None found for Phase 107 planning. [VERIFIED: rustc --version; cargo --version; bun --version; bazelisk version]

## Security Domain

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|------------------|
| V2 Authentication | No | Phase 107 does not change RPC authentication or credential handling. [VERIFIED: .planning/ROADMAP.md; packages/open-bitcoin-rpc/src/dispatch/node.rs] |
| V3 Session Management | No | Phase 107 does not add session state. [VERIFIED: .planning/ROADMAP.md] |
| V4 Access Control | Yes | Use typed relay eligibility from `classify_relay_eligibility` and scoped permission effects from `InboundPermissionDecision`. [VERIFIED: packages/open-bitcoin-network/src/relay.rs; packages/open-bitcoin-network/src/inbound/permissions.rs] |
| V5 Input Validation | Yes | Keep config parsing in the JSONC loader and peer permission parsing in the inbound permission module. [VERIFIED: packages/open-bitcoin-rpc/src/config/open_bitcoin.rs; packages/open-bitcoin-rpc/src/config/loader/open_bitcoin_runtime.rs; packages/open-bitcoin-network/src/inbound/permissions.rs] |
| V6 Cryptography | No | Phase 107 does not add cryptography or key handling. [VERIFIED: .planning/ROADMAP.md] |

### Known Threat Patterns for Runtime Relay Integration

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Relay activation bypass through default constructor | Elevation of Privilege | Production runtime context must pass resolved `RuntimeConfig.relay` into managed network construction, and checker coverage should fail if it regresses. [VERIFIED: packages/open-bitcoin-rpc/src/context/network.rs; packages/open-bitcoin-node/src/network/relay_serving.rs; .planning/phases/107-runtime-relay-activation-and-download-eligibility-integration/107-CONTEXT.md] |
| Unauthorized transaction downloads from ineligible peers | Elevation of Privilege / Tampering | Gate tx announcements and orphan parent requests with `RelayEligibilityDecision` before scheduler state mutation. [VERIFIED: packages/open-bitcoin-network/src/relay.rs; packages/open-bitcoin-network/src/peer/inventory_state.rs; packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs] |
| Sensitive relay material in status/logs/support | Information Disclosure | Use aggregate counters, fixed labels, and existing support redaction boundaries. [VERIFIED: packages/open-bitcoin-node/src/status/relay_evidence.rs; scripts/check-phase105-operator-relay-evidence.ts] |
| Stale in-flight requests after suppression | Denial of Service / Tampering | Suppress before candidate or in-flight insertion, and test timeout, `notfound`, disconnect, received cleanup, and fallback behavior. [VERIFIED: packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs; packages/open-bitcoin-network/src/peer/transaction_relay/tests/scheduler_cases.rs] |
| Permission confusion between protected and relay-eligible inbound peers | Elevation of Privilege | Treat protected admission as separate from relay permission effects, and require scoped `relay`, `forcerelay`, or `mempool` effects for permissioned inbound eligibility. [VERIFIED: packages/open-bitcoin-network/src/inbound/permissions.rs; packages/open-bitcoin-network/src/relay.rs; .planning/phases/107-runtime-relay-activation-and-download-eligibility-integration/107-CONTEXT.md] |

## Sources

### Primary (HIGH confidence)

- `.planning/phases/107-runtime-relay-activation-and-download-eligibility-integration/107-CONTEXT.md` - locked decisions, discretion, deferred scope, and canonical refs. [VERIFIED: local file read]
- `.planning/REQUIREMENTS.md` - Phase 107 requirement ownership for ACT-01, ACT-02, INV-02, INV-03, DL-01, DL-02, and REL-03. [VERIFIED: local file read]
- `.planning/ROADMAP.md` - Phase 107 purpose, scope, success criteria, and verification. [VERIFIED: local file read]
- `AGENTS.md`, `AGENTS.bright-builds.md`, `standards/core/*.md`, `standards/languages/rust.md`, and `standards/languages/typescript-javascript.md` - repo and Bright Builds constraints. [VERIFIED: local file read]
- `packages/open-bitcoin-network/src/relay.rs` - Phase 100 activation and eligibility policy. [VERIFIED: local file read]
- `packages/open-bitcoin-network/src/peer/transaction_relay.rs` and `packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs` - download action, suppression, request, fallback, and cleanup state. [VERIFIED: local file read]
- `packages/open-bitcoin-network/src/peer.rs`, `packages/open-bitcoin-network/src/peer/inventory_state.rs`, and `packages/open-bitcoin-network/src/inbound/permissions.rs` - PeerManager scheduling entry points and permission effect sources. [VERIFIED: local file read]
- `packages/open-bitcoin-node/src/network.rs`, `packages/open-bitcoin-node/src/network/relay_serving.rs`, and `packages/open-bitcoin-node/src/network/relay_fanout.rs` - managed construction, eligibility adaptation, fanout, and status projection. [VERIFIED: local file read]
- `packages/open-bitcoin-rpc/src/config.rs`, `packages/open-bitcoin-rpc/src/config/loader/open_bitcoin_runtime.rs`, `packages/open-bitcoin-rpc/src/context/network.rs`, and `packages/open-bitcoin-rpc/src/dispatch/node.rs` - runtime config and RPC/status projection paths. [VERIFIED: local file read]
- `scripts/verify.sh` and Phase 100-106 checker scripts - deterministic checker and verifier pattern. [VERIFIED: local file read]

### Secondary (MEDIUM confidence)

- `packages/bitcoin-knots/src/net_permissions.h`, `packages/bitcoin-knots/src/node/txdownloadman.h`, `packages/bitcoin-knots/src/node/txdownloadman_impl.cpp`, `packages/bitcoin-knots/src/net_processing.cpp`, and `packages/bitcoin-knots/test/functional/p2p_tx_download.py` - pinned baseline anchors for permission and transaction download behavior. [VERIFIED: local submodule read]
- `docs/architecture/status-snapshot.md`, `docs/architecture/operator-observability.md`, `docs/operator/runtime-guide.md`, `docs/parity/index.json`, `docs/parity/checklist.md`, and `docs/parity/source-breadcrumbs.json` - status, UAT, parity, and breadcrumb evidence contracts. [VERIFIED: local file read]

### Tertiary (LOW confidence)

- None. [VERIFIED: no web-only sources used]

## Metadata

**Confidence breakdown:**

- Standard stack: HIGH - all tools and versions were verified locally, and no new dependency is recommended. [VERIFIED: rustc --version; cargo --version; bun --version; bazelisk version; packages/Cargo.toml]
- Architecture: MEDIUM - the main seams are verified, and the Phase 107 inbound-serving source of truth is resolved to `config.inbound.enabled`; later live-listener tightening would require a future phase. [VERIFIED: packages/open-bitcoin-rpc/src/context/network.rs; packages/open-bitcoin-node/src/network/relay_serving.rs] [RESOLVED]
- Pitfalls: HIGH - pitfalls are grounded in current code paths and existing tests/checkers. [VERIFIED: packages/open-bitcoin-rpc/src/context/network.rs; packages/open-bitcoin-network/src/peer/inventory_state.rs; packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs; scripts/verify.sh]

**Research date:** 2026-07-03
**Valid until:** 2026-08-02 for local code structure; re-check if Phase 108 or another relay phase changes runtime config, scheduler, or status contracts first. [ASSUMED]
