# Phase 90: Inbound Listener and Admission Policy - Research

**Researched:** 2026-06-25
**Domain:** Rust Bitcoin P2P listener activation, inbound admission, status/RPC/CLI evidence
**Confidence:** HIGH

<user_constraints>
## User Constraints (from CONTEXT.md)

Source for this entire `user_constraints` block: [VERIFIED: .planning/phases/90-inbound-listener-and-admission-policy/90-CONTEXT.md]

### Locked Decisions

## Implementation Decisions

### Activation And Listener Preflight

- **D-01:** Inbound serving is disabled by default. A disabled runtime must not bind any P2P listener, create accept-loop tasks, or report listener success.
- **D-02:** Phase 90 should use Open Bitcoin-owned controls rather than silently accepting baseline `bitcoin.conf` listener keys. Add JSONC-owned config under an `inbound` section, with at least `enabled`, `listen_addresses`, `max_peers`, and `reserved_slots`. Add daemon CLI overrides with an Open Bitcoin prefix, such as `-openbitcoininbound=1` and `-openbitcoinlisten=<host:port>`, so this phase does not imply full Knots `-listen` or `-bind` compatibility.
- **D-03:** Listener preflight must be a typed, deterministic result before any socket side effect. Required outcomes include `disabled`, `no_listen_addresses`, `invalid_endpoint`, `unsafe_endpoint`, `bind_unavailable`, `already_bound`, and `ready`.
- **D-04:** Loopback endpoints are the default deterministic test/UAT target. Wildcard or public interfaces require an explicit public-exposure acknowledgement field, for example `inbound.allow_public = true`, and are never part of `bash scripts/verify.sh`.
- **D-05:** Preflight diagnostics must include the endpoint, stable reason code, human message, and next action. Error messages should name the exact config or CLI field that needs correction.

### Admission And Handshake Lifecycle

- **D-06:** Keep admission decisions in pure domain types before runtime socket effects. Introduce first-party types such as `InboundListenerConfig`, `InboundAdmissionPolicy`, `InboundAdmissionDecision`, and `InboundPeerRecord` in the network/node boundary rather than burying policy inside the Tokio accept loop.
- **D-07:** Reuse and extend the existing `PeerManager` and `ManagedPeerNetwork` inbound role support. The current `add_inbound_peer`, `ConnectionRole::Inbound`, `PeerState`, and `network_info` count paths are the starting point; Phase 90 should add enough metadata to distinguish accepted, rejected, handshaking, established, duplicate, self-connection, and disconnected inbound peers.
- **D-08:** The inbound handshake should reuse the existing message-driven version/verack path. A newly accepted inbound peer starts without `local_version_sent`, then sends local `version`, `wtxidrelay`, `verack`, and `sendheaders` only through the same `PeerAction` flow used today.
- **D-09:** Duplicate and self-connection protection is required before a peer is counted as admitted. Use stable connection keys based on remote endpoint and handshake nonce where available, reject duplicate peer IDs, and reject a remote nonce matching the local nonce as a self-connection signal.
- **D-10:** Phase 90 may parse ordinary P2P messages already supported by the core, but it must not use inbound serving as a way to claim transaction relay, compact block relay, mempool propagation, full address relay, or production network participation. Any relay-related capability should stay explicitly deferred or inert.

### Caps, Reserved Slots, And Outbound Sync Safety

- **D-11:** Inbound caps are separate from outbound sync targets. `target_outbound_peers` and existing durable sync behavior must not be reduced or starved by inbound peers.
- **D-12:** Admission policy should expose `max_inbound_peers`, `reserved_slots`, current inbound count, and current outbound count as pure inputs and outputs. If the cap is reached, the rejection reason must be stable and operator-visible.
- **D-13:** Reserved slots are an admission primitive in Phase 90, not the full permission system. They can be modeled and tested now, but Phase 91 owns Knots-aligned permission classes and richer protected-peer policy.
- **D-14:** The listener/accept loop must have a bounded shutdown path tied to `open-bitcoind` graceful shutdown. Dropping or disabling the listener should stop accepting new peers without disturbing existing outbound sync unless an explicit shutdown occurs.

### Operator Evidence, RPC, Metrics, Logs, And Support

- **D-15:** Extend operator evidence from the shared status model. Inbound listener state and admission outcomes should surface through `OpenBitcoinStatusSnapshot` or a clearly owned child contract, then render consistently in CLI status, dashboard/status JSON, support bundles, metrics, structured logs, and RPC-facing status.
- **D-16:** `getnetworkinfo` already exposes `connections`, `connections_in`, and `connections_out`; Phase 90 should keep those fields accurate and add Open Bitcoin-specific status evidence for listener/preflight/admission rather than changing baseline-shaped fields in surprising ways.
- **D-17:** Evidence labels must separate inbound serving from outbound sync. Suggested stable labels include listener state, bound endpoints, preflight reason, admitted inbound peers, rejected inbound peers, handshake state counts, duplicate/self-connection rejects, cap rejects, and latest admission event.
- **D-18:** Support bundles must preserve diagnostic usefulness without copying raw unbounded peer tables. Peer endpoint evidence should be bounded and redacted where needed, following existing support-bundle redaction patterns.

### Verification And UAT

- **D-19:** Default verification must remain deterministic, local, short-running, public-network-free, and real-service-manager-free. Use loopback listeners, injected transports, synthetic peers, and hermetic handshake fixtures for `bash scripts/verify.sh`.
- **D-20:** Unit tests should focus on pure admission policy, preflight classification, cap accounting, duplicate/self-connection rejection, and peer-state transitions with Arrange/Act/Assert structure.
- **D-21:** Integration tests may bind `127.0.0.1:0` using the existing test-harness listener pattern. They should assert that disabled config does not bind, invalid endpoints produce stable diagnostics, and enabled loopback admission increments inbound counts without changing outbound counts.
- **D-22:** Any operator UAT text must include repo-local Cargo and Bazel forms, not only an installed alias. Use commands such as `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --bin open-bitcoind -- ...` and `bazel run //packages/open-bitcoin-rpc:open_bitcoind -- ...`, plus the repo-local `open-bitcoin-cli` status forms.

### the agent's Discretion

The planner may choose exact module splits and naming if they preserve the locked boundaries above. Prefer a small pure policy module plus a thin runtime adapter over a large listener file. Prefer extending existing status/support contracts only where it keeps one shared source of truth; avoid renderer-local inbound summaries.

### Deferred Ideas (OUT OF SCOPE)

## Deferred Ideas

- Phase 91 owns peer permissions and connection classes.
- Phase 92 owns local address advertisement, `getaddr` response boundaries, and address-management contracts.
- Phase 93 owns eviction, ban, discourage, and misbehavior policy.
- Phase 94 owns inbound DoS/resource governance beyond Phase 90 admission caps.
- Phase 95 owns release-boundary and no-claim evidence across v1.9.
- Future milestones own transaction relay, compact block relay, mempool propagation, public inbound defaults, signed packaging, Windows service support, hosted dashboards, GUI, public-network CI, and production full-node readiness claims.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| INB-01 | Operators can enable inbound peer serving only through explicit config or CLI controls, with inbound serving disabled by default unless a later release boundary says otherwise. [VERIFIED: .planning/REQUIREMENTS.md] | Use Open Bitcoin JSONC `inbound` config plus daemon-only `-openbitcoin*` CLI overrides, not baseline `bitcoin.conf` `-listen` or `-bind`. [VERIFIED: docs/architecture/config-precedence.md; packages/open-bitcoin-rpc/src/config/loader.rs] |
| INB-02 | The daemon can bind and listen on configured interfaces with deterministic preflight and diagnostic errors when disabled, unavailable, unsafe, or already in use. [VERIFIED: .planning/REQUIREMENTS.md] | Use a typed preflight/activation result with stable reason codes and keep Tokio bind/listen work in a thin runtime adapter. [VERIFIED: packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs; packages/bitcoin-knots/src/net.cpp] |
| INB-03 | The node admits inbound peers through typed connection records, handshake lifecycle state, duplicate/self-connection protections, and inbound/outbound counters. [VERIFIED: .planning/REQUIREMENTS.md] | Extend `PeerManager::add_inbound_peer`, `ConnectionRole::Inbound`, `PeerState`, and `ManagedPeerNetwork::network_info`; add nonce-based self-connection checks in the existing `version` path. [VERIFIED: packages/open-bitcoin-network/src/peer.rs; packages/open-bitcoin-node/src/network.rs; packages/bitcoin-knots/src/net_processing.cpp] |
| INB-04 | The node enforces configurable inbound connection caps, reserved slots, and protected peer handling without starving the existing outbound sync workflow. [VERIFIED: .planning/REQUIREMENTS.md] | Keep inbound cap inputs separate from `target_outbound_peers` and durable sync resource pressure fields; model reserved slots without Phase 91 permissions. [VERIFIED: .planning/phases/90-inbound-listener-and-admission-policy/90-CONTEXT.md; docs/architecture/operator-observability.md] |
| INB-05 | Operator status, metrics, logs, RPC-facing status, and support evidence distinguish inbound serving from outbound sync and expose admission and handshake outcomes. [VERIFIED: .planning/REQUIREMENTS.md] | Extend `OpenBitcoinStatusSnapshot`/`PeerStatus` as the shared source, preserve `getnetworkinfo` baseline counts, and render bounded support evidence. [VERIFIED: docs/architecture/status-snapshot.md; packages/open-bitcoin-node/src/status.rs; packages/open-bitcoin-rpc/src/method/node.rs; packages/open-bitcoin-cli/src/operator/support/render.rs] |
</phase_requirements>

## Summary

Phase 90 should be implemented as a narrow listener/admission layer over the existing peer core, not as a new P2P stack. [VERIFIED: packages/open-bitcoin-network/src/peer.rs; packages/open-bitcoin-node/src/network.rs] The repo already has inbound roles, inbound peer insertion, version/verack message handling, and inbound/outbound count projection; it lacks opt-in listener config, deterministic listener preflight, endpoint admission records, self-connection rejection, cap/reserved-slot policy, and shared status evidence for listener/admission outcomes. [VERIFIED: packages/open-bitcoin-network/src/peer.rs; packages/open-bitcoin-node/src/network.rs; packages/open-bitcoin-rpc/src/config.rs; packages/open-bitcoin-node/src/status.rs]

The recommended shape is a small pure admission/preflight module plus a thin Tokio listener adapter. [VERIFIED: standards/core/architecture.md; standards/languages/rust.md] Pure code should own `InboundListenerConfig`, `InboundPreflightResult`, `InboundAdmissionPolicy`, `InboundAdmissionDecision`, and `InboundPeerRecord`; runtime code should only parse config, bind loopback/public-approved sockets, accept streams, feed decoded messages into `ManagedPeerNetwork`, and publish typed evidence. [VERIFIED: .planning/phases/90-inbound-listener-and-admission-policy/90-CONTEXT.md; packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs]

**Primary recommendation:** Implement loopback-first inbound serving through Open Bitcoin-owned JSONC/daemon CLI controls, reuse the existing `PeerAction` handshake flow, and project all listener/admission evidence through the shared status model before rendering it in CLI, RPC status, metrics/logs, and support bundles. [VERIFIED: docs/architecture/config-precedence.md; docs/architecture/status-snapshot.md; packages/open-bitcoin-network/src/peer.rs]

## Project Constraints (from AGENTS.md)

- Use `git submodule update --init --recursive` when the pinned Knots baseline must be materialized; the local Knots submodule is present at `v29.3.knots20260210`. [VERIFIED: AGENTS.md; git submodule status --recursive packages/bitcoin-knots]
- Rust `1.94.1` is the pinned Rust source of truth from `rust-toolchain.toml`; local `rustc` and `cargo` report `1.94.1`. [VERIFIED: AGENTS.md; rustc --version; cargo --version]
- Use `bash scripts/verify.sh` as the repo-native verification contract; `--fast` is local iteration only and the default command remains the pre-commit/release contract. [VERIFIED: AGENTS.md; bash scripts/verify.sh --help]
- UAT guidance must include repo-local Cargo and Bazel forms for daemon and CLI workflows, not only an installed alias. [VERIFIED: AGENTS.md]
- Bun is the canonical runtime for repo-owned higher-level automation scripts; this repo has no `package.json`, so no `bun install` bootstrap should be planned. [VERIFIED: AGENTS.md; .planning/STACK.md]
- Git hooks are installed with `bash scripts/install-git-hooks.sh`, and `bash scripts/verify.sh` self-heals missing hook installation outside CI. [VERIFIED: AGENTS.md]
- `docs/metrics/lines-of-code.md` is a tracked generated artifact and may legitimately change after verification. [VERIFIED: AGENTS.md]
- Intentional behavior differences from Bitcoin Knots belong in `docs/parity/index.json` and companion docs under `docs/parity/`. [VERIFIED: AGENTS.md; docs/parity/index.json]
- New first-party Rust source/test files under `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests` need parity breadcrumb comments and entries in `docs/parity/source-breadcrumbs.json`; use the explicit `none` breadcrumb only when no defensible Knots anchor exists. [VERIFIED: AGENTS.md; scripts/check-parity-breadcrumbs.ts; docs/parity/source-breadcrumbs.json]
- After substantial feature, parity, operator-surface, or workflow changes, check relevant READMEs for status freshness. [VERIFIED: AGENTS.md]
- Bright Builds requires functional core / imperative shell, parse-at-boundaries domain types, illegal-state modeling, early returns, `maybe_` names for optional Rust values, focused Arrange/Act/Assert unit tests, and repo-native verification before commit. [VERIFIED: AGENTS.bright-builds.md; standards/core/architecture.md; standards/core/code-shape.md; standards/core/testing.md; standards/core/verification.md; standards/languages/rust.md]
- No project-specific `.claude/skills/` or `.agents/skills/` skill indexes were found. [VERIFIED: find .claude/skills .agents/skills -maxdepth 2 -name SKILL.md]

## Standard Stack

### Core

| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| Rust workspace | 1.94.1 / edition 2024 | First-party implementation language and workspace model | The repo pins Rust `1.94.1` and Rust 2024 in the toolchain/workspace. [VERIFIED: rustc --version; packages/Cargo.toml] |
| `open-bitcoin-network` | 0.1.0 | Pure peer lifecycle, wire message parsing, handshake actions, inbound/outbound roles | Existing `PeerManager`, `PeerState`, `ConnectionRole::Inbound`, `VersionMessage`, and `PeerAction` are the correct extension points. [VERIFIED: cargo tree -p open-bitcoin-network --depth 1; packages/open-bitcoin-network/src/peer.rs; packages/open-bitcoin-network/src/message.rs] |
| `open-bitcoin-node` | 0.1.0 | Managed network wrapper, chain/mempool integration, status/metrics/log contracts | Existing `ManagedPeerNetwork::network_info` already projects inbound/outbound counts and `OpenBitcoinStatusSnapshot` is the shared evidence model. [VERIFIED: cargo tree -p open-bitcoin-node --depth 1; packages/open-bitcoin-node/src/network.rs; packages/open-bitcoin-node/src/status.rs] |
| `open-bitcoin-rpc` | 0.1.0 | Runtime config, JSON-RPC dispatch, `open-bitcoind` daemon, Tokio/Axum shell | Existing daemon startup already uses typed preflight, `tokio::net::TcpListener::bind`, graceful shutdown, and runtime config loading. [VERIFIED: cargo tree -p open-bitcoin-rpc --depth 1; packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs; packages/open-bitcoin-rpc/src/config.rs] |
| `open-bitcoin-cli` | 0.1.0 | Operator status, dashboard, support bundles, baseline-compatible RPC client | Existing status/support renderers consume `OpenBitcoinStatusSnapshot`, and baseline CLI can inspect `getnetworkinfo`. [VERIFIED: cargo tree -p open-bitcoin-cli --depth 1; packages/open-bitcoin-cli/src/operator/status.rs; packages/open-bitcoin-cli/src/operator/support/render.rs; packages/open-bitcoin-cli/BUILD.bazel] |

### Supporting

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| `tokio` | 1.52.1 | Async TCP listener/accept loop in `open-bitcoind` runtime shell | Use only in `open-bitcoin-rpc` daemon/runtime adapter code; do not introduce it into pure network/core crates. [VERIFIED: cargo tree -p open-bitcoin-rpc --depth 1; packages/open-bitcoin-rpc/BUILD.bazel] |
| `axum` | 0.8.9 | Existing JSON-RPC HTTP server runtime | Keep existing RPC server lifecycle intact while adding P2P listener tasks alongside daemon sync worker/shutdown handling. [VERIFIED: cargo tree -p open-bitcoin-rpc --depth 1; packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs] |
| `serde` / `serde_json` | 1.0.228 / 1.0.149 | Stable status/config/support JSON contracts | Use for `InboundListenerStatus`, admission events, support evidence, and status snapshot additions. [VERIFIED: cargo tree -p open-bitcoin-node --depth 1; cargo tree -p open-bitcoin-rpc --depth 1] |
| `jsonc-parser` | 0.32.3 | Open Bitcoin-owned `open-bitcoin.jsonc` parsing | Add the `inbound` JSONC section beside existing `sync` config, with `deny_unknown_fields`. [VERIFIED: cargo tree -p open-bitcoin-rpc --depth 1; packages/open-bitcoin-rpc/src/config/open_bitcoin.rs] |
| `clap` | 4.6.1 | Operator CLI parsing | Use existing operator status/support commands; daemon-specific `-openbitcoin*` flags are parsed by the daemon config loader, not by `clap`. [VERIFIED: cargo tree -p open-bitcoin-cli --depth 1; packages/open-bitcoin-cli/src/operator.rs; packages/open-bitcoin-rpc/src/config/loader.rs] |
| `open-bitcoin-test-harness` | 0.1.0 | Hermetic loopback port allocation and sandboxing | Use `PortReservation::localhost()` / `127.0.0.1:0` style integration tests for listener binding. [VERIFIED: cargo tree -p open-bitcoin-rpc --depth 1; packages/open-bitcoin-test-harness/src/isolation.rs] |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Existing first-party peer core | `rust-bitcoin`, `bdk`, or another Bitcoin P2P crate | Do not use existing Rust Bitcoin libraries in the production path; the project owns its own domain model and parity surface. [VERIFIED: AGENTS.md] |
| Open Bitcoin JSONC + daemon CLI flags | Baseline `bitcoin.conf` `-listen`, `-bind`, `-whitebind`, `-whitelist` | Phase 90 explicitly avoids claiming full Knots listener/config compatibility and keeps permission classes deferred. [VERIFIED: 90-CONTEXT.md; docs/architecture/config-precedence.md] |
| Shared `OpenBitcoinStatusSnapshot` | Renderer-local status/support DTOs | Shared snapshot is the documented source of truth for status, dashboard, support, RPC-facing status, metrics, and logs. [VERIFIED: docs/architecture/status-snapshot.md; packages/open-bitcoin-node/src/status.rs] |
| Loopback hermetic tests | Public-network bind/probe tests | Default verification must stay deterministic, public-network-free, and service-manager-free. [VERIFIED: 90-CONTEXT.md; docs/parity/release-readiness.md] |

**Installation:**

```bash
# No new dependency installation is recommended for Phase 90.
cargo tree --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --depth 1
cargo tree --manifest-path packages/Cargo.toml -p open-bitcoin-node --depth 1
cargo tree --manifest-path packages/Cargo.toml -p open-bitcoin-network --depth 1
cargo tree --manifest-path packages/Cargo.toml -p open-bitcoin-cli --depth 1
```

**Version verification:** Versions above were verified from the local locked workspace with `cargo tree`; no external package additions are recommended. [VERIFIED: cargo tree --manifest-path packages/Cargo.toml]

## Architecture Patterns

### Recommended Project Structure

```text
packages/
  open-bitcoin-network/src/
    inbound.rs              # pure listener config, endpoint parsing, admission policy, reason labels
    peer.rs                 # extend PeerState/PeerManager for inbound records and self-connection rejection
    lib.rs                  # re-export first-party inbound types
  open-bitcoin-node/src/
    network.rs              # project admission records/counts through ManagedPeerNetwork
    status.rs               # shared InboundListenerStatus / InboundAdmissionStatus under PeerStatus or child type
    metrics.rs              # add bounded numeric inbound counters only if needed by status/dashboard
  open-bitcoin-rpc/src/
    config/open_bitcoin.rs  # JSONC inbound config shape
    config/loader.rs        # daemon CLI overrides and precedence
    inbound_listener.rs     # thin Tokio listener/accept-loop adapter
    bin/open-bitcoind.rs    # start listener after preflight, join shutdown with RPC/sync workers
  open-bitcoin-cli/src/operator/
    status.rs               # collect live RPC/shared snapshot evidence
    status/render.rs        # render shared inbound fields, no local DTO
    support/                # bounded/redacted support evidence
docs/parity/source-breadcrumbs.json  # map any new Rust source/test files
```

This structure keeps pure admission decisions in first-party network/node code and socket effects in the daemon runtime shell. [VERIFIED: standards/core/architecture.md; standards/languages/rust.md; packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs]

### Pattern 1: Two-Stage Listener Activation

**What:** First classify config and endpoint safety without socket I/O, then perform a tightly scoped bind/listen activation that returns the same typed diagnostic contract before spawning accept-loop tasks. [VERIFIED: 90-CONTEXT.md; packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs; packages/bitcoin-knots/src/net.cpp]

**When to use:** Use this for `disabled`, `no_listen_addresses`, `invalid_endpoint`, `unsafe_endpoint`, `bind_unavailable`, `already_bound`, and `ready` outcomes. [VERIFIED: 90-CONTEXT.md]

**Example:**

```rust
// Source: packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs and packages/bitcoin-knots/src/net.cpp
let preflight = classify_inbound_listener_config(&runtime.inbound);
if !preflight.should_attempt_bind() {
    return record_inbound_preflight(preflight);
}

let activation = bind_inbound_listeners(preflight.ready_endpoints()).await;
if !activation.is_ready() {
    return record_inbound_preflight(activation.into_preflight_result());
}

let worker = start_inbound_accept_loop(activation, shutdown);
```

**Important nuance:** `bind_unavailable` and `already_bound` are OS-observed outcomes, so the implementation should keep them in the listener activation result while preserving a pure first-stage classifier. [ASSUMED]

### Pattern 2: Pure Admission Decision Before Peer Insertion

**What:** Admission policy should be a data-in/data-out function over endpoint key, peer id, current inbound/outbound counts, max inbound peers, reserved slots, shutdown state, and duplicate/self-connection observations. [VERIFIED: standards/core/architecture.md; 90-CONTEXT.md]

**When to use:** Use this in the accept loop before inserting into `PeerManager`, and again during `version` handling when a remote nonce becomes available. [VERIFIED: packages/open-bitcoin-network/src/peer.rs; packages/bitcoin-knots/src/net_processing.cpp]

**Example:**

```rust
// Source: packages/open-bitcoin-network/src/peer.rs and packages/bitcoin-knots/src/net_processing.cpp
let decision = admission_policy.decide(InboundAdmissionRequest {
    peer_id,
    remote_endpoint_key,
    maybe_remote_nonce,
    current_inbound,
    current_outbound,
});

match decision {
    InboundAdmissionDecision::Admit(record) => managed_network.add_inbound_peer_record(record)?,
    InboundAdmissionDecision::Reject(rejection) => record_rejection(rejection),
}
```

### Pattern 3: Reuse Existing Message-Driven Handshake

**What:** A newly accepted inbound peer should enter the existing `PeerManager` with `local_version_sent = false`, receive remote `version`, then produce `Version`, `WtxidRelay`, `Verack`, and `SendHeaders` through `PeerAction::Send`. [VERIFIED: packages/open-bitcoin-network/src/peer.rs]

**When to use:** Use this for inbound sockets after admission; do not create a separate inbound handshake engine. [VERIFIED: packages/open-bitcoin-network/src/peer.rs; docs/parity/catalog/p2p.md]

**Existing behavior to preserve:** `PeerManager::handle_version` sends local version only if `local_version_sent` is false, then sends `wtxidrelay`, `verack`, and `sendheaders` only through `PeerAction`. [VERIFIED: packages/open-bitcoin-network/src/peer.rs]

### Pattern 4: Shared Evidence Contract

**What:** Add listener/admission evidence under the shared status model rather than status/support/dashboard-specific structs. [VERIFIED: docs/architecture/status-snapshot.md; packages/open-bitcoin-node/src/status.rs]

**When to use:** Use this for listener state, bound endpoints, preflight reason, admitted/rejected counts, handshake state counts, duplicate/self rejects, cap rejects, and latest admission event. [VERIFIED: 90-CONTEXT.md]

**Example:**

```rust
// Source: packages/open-bitcoin-node/src/status.rs and packages/open-bitcoin-cli/src/operator/status/render.rs
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InboundListenerStatus {
    pub state: String,
    pub preflight_reason: String,
    pub bound_endpoints: Vec<String>,
    pub admitted_inbound_peers: u32,
    pub rejected_inbound_peers: u32,
}
```

### Anti-Patterns to Avoid

- **Binding from disabled config:** Disabled inbound serving must not bind sockets, create accept loops, or report listener success. [VERIFIED: 90-CONTEXT.md]
- **Accept-loop-owned policy:** Do not bury caps, duplicate checks, or self-connection checks inside Tokio loop branches; pure tests need to cover those decisions. [VERIFIED: standards/core/architecture.md; 90-CONTEXT.md]
- **Baseline config bleed-through:** Do not accept `bitcoin.conf` `listen`, `bind`, `whitebind`, or `whitelist` as Phase 90 controls. [VERIFIED: 90-CONTEXT.md; docs/architecture/config-precedence.md]
- **Changing `getnetworkinfo` shape aggressively:** Preserve baseline-shaped `connections`, `connections_in`, and `connections_out`; put Open Bitcoin-specific listener/admission evidence in owned status evidence. [VERIFIED: packages/open-bitcoin-rpc/src/method/node.rs; packages/bitcoin-knots/src/rpc/net.cpp; 90-CONTEXT.md]
- **Public listener in default verification:** `bash scripts/verify.sh` must not require public network interfaces or public-network peers. [VERIFIED: 90-CONTEXT.md; docs/parity/release-readiness.md]
- **Permission creep:** Do not implement Knots `NetPermissionFlags` classes, ban/discourage policy, eviction, `getaddr`, transaction relay, mempool propagation, or compact block relay in Phase 90. [VERIFIED: 90-CONTEXT.md; packages/bitcoin-knots/src/net_permissions.cpp; packages/bitcoin-knots/src/net_processing.cpp]

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Bitcoin message parsing | A new inbound-only parser | Existing `ParsedNetworkMessage::decode_wire` and `WireNetworkMessage` | The existing codec already covers `version`, `verack`, `wtxidrelay`, `sendheaders`, `ping`, `headers`, `inv`, `getdata`, `tx`, and `block`. [VERIFIED: packages/open-bitcoin-network/src/message.rs; docs/parity/catalog/p2p.md] |
| Handshake lifecycle | A second inbound handshake state machine | Existing `PeerManager::handle_message`, `handle_version`, `handle_verack`, and `PeerAction` | Current outbound/inbound state already drives version/verack actions and sync start. [VERIFIED: packages/open-bitcoin-network/src/peer.rs] |
| Config precedence | A separate daemon parser for inbound keys | Existing `RuntimeConfig`, `OpenBitcoinConfig`, and `load_runtime_config_for_args` precedence | The repo already documents `CLI > environment > Open Bitcoin JSONC > bitcoin.conf > cookies > defaults`. [VERIFIED: docs/architecture/config-precedence.md; packages/open-bitcoin-rpc/src/config/loader.rs] |
| Loopback port allocation in tests | Ad hoc fixed ports in default verification | `open-bitcoin-test-harness::PortReservation::localhost()` or `127.0.0.1:0` | Existing harness allocates loopback ports hermetically. [VERIFIED: packages/open-bitcoin-test-harness/src/isolation.rs] |
| Status/support evidence | New renderer-local DTOs | `OpenBitcoinStatusSnapshot`, `PeerStatus`, support-bundle projections | Shared status is the documented source of truth for CLI, dashboard, service, support, metrics, logs, and RPC-facing status. [VERIFIED: docs/architecture/status-snapshot.md; packages/open-bitcoin-node/src/status.rs] |
| Permission classes | Partial clone of Knots `NetPermissionFlags` | Minimal Phase 90 reserved-slot primitive only | Knots permissions include relay, mempool, address, download, force-inbound, and no-ban semantics owned by Phase 91 or later. [VERIFIED: 90-CONTEXT.md; packages/bitcoin-knots/src/net_permissions.h] |
| Ban/eviction/resource DoS | Early ban/discourage/eviction machinery | Stable admission rejection labels only | Phases 93 and 94 own eviction/ban and DoS/resource governance beyond admission caps. [VERIFIED: 90-CONTEXT.md] |

**Key insight:** The difficult part is not opening a TCP socket; it is preserving deterministic policy, truthful evidence, and parity boundaries while avoiding hidden claims about relay, permissions, public exposure, and production readiness. [VERIFIED: 90-CONTEXT.md; docs/parity/release-readiness.md]

## Common Pitfalls

### Pitfall 1: Treating Bind As The Preflight Boundary

**What goes wrong:** Code calls `TcpListener::bind` before producing a typed disabled/invalid/unsafe result. [VERIFIED: 90-CONTEXT.md]
**Why it happens:** `bind` is the first obvious place to discover endpoint errors, but disabled/no-address/public-exposure checks are pure config decisions. [VERIFIED: standards/core/architecture.md]
**How to avoid:** Use pure classification first, then a small activation stage for OS bind outcomes, then spawn accept tasks only on `ready`. [VERIFIED: packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs; packages/bitcoin-knots/src/net.cpp]
**Warning signs:** Tests can only detect errors by observing bound ports or spawned tasks. [ASSUMED]

### Pitfall 2: Counting A Peer Before Duplicate/Self Checks Settle

**What goes wrong:** `connections_in` increments for duplicate or self-connections and never gets corrected. [VERIFIED: 90-CONTEXT.md]
**Why it happens:** Existing `add_inbound_peer` inserts into `peer_ids`, and `network_info` counts that set immediately. [VERIFIED: packages/open-bitcoin-node/src/network.rs]
**How to avoid:** Add typed inbound admission/handshake states and ensure rejected duplicate/self-connection records are not counted as admitted inbound peers. [VERIFIED: 90-CONTEXT.md; packages/bitcoin-knots/src/net_processing.cpp]
**Warning signs:** A self-connection test leaves `connections_in > 0` after the remote nonce matches the local nonce. [VERIFIED: packages/bitcoin-knots/test/functional/p2p_handshake.py]

### Pitfall 3: Starving Outbound Sync With Shared Caps

**What goes wrong:** Inbound peers reduce `target_outbound_peers` or durable sync peer attempts. [VERIFIED: 90-CONTEXT.md]
**Why it happens:** A single `connected_peers` cap is simpler but violates Phase 90. [VERIFIED: 90-CONTEXT.md]
**How to avoid:** Admission policy inputs must carry separate current inbound and outbound counts, and status must continue to show outbound sync targets separately. [VERIFIED: docs/architecture/operator-observability.md; packages/open-bitcoin-node/src/network.rs]
**Warning signs:** Tests with one outbound peer and one inbound peer show lower outbound target/resource-pressure fields after inbound admission. [VERIFIED: 90-CONTEXT.md]

### Pitfall 4: Accidentally Implementing Phase 91 Permission Classes

**What goes wrong:** Phase 90 grows `noban`, `relay`, `mempool`, `download`, `addr`, or `forceinbound` behavior. [VERIFIED: packages/bitcoin-knots/src/net_permissions.h; 90-CONTEXT.md]
**Why it happens:** Knots combines admission, whitelist/whitebind, permission flags, eviction, and relay privileges in adjacent code. [VERIFIED: packages/bitcoin-knots/src/net.cpp; packages/bitcoin-knots/src/net_permissions.cpp]
**How to avoid:** Model only ordinary vs reserved slot eligibility and stable cap rejection labels; leave permission parsing/effects to Phase 91. [VERIFIED: 90-CONTEXT.md]
**Warning signs:** New config accepts `whitelist`, `whitebind`, `noban`, `forcerelay`, `mempool`, or `addr` labels. [VERIFIED: packages/bitcoin-knots/src/net_permissions.cpp]

### Pitfall 5: Renderer-Local Inbound Evidence

**What goes wrong:** CLI status, dashboard, support bundle, and RPC status each compute their own inbound summaries. [VERIFIED: docs/architecture/status-snapshot.md]
**Why it happens:** It is tempting to add text where the feature is displayed. [ASSUMED]
**How to avoid:** Add one shared status contract and render it everywhere. [VERIFIED: packages/open-bitcoin-node/src/status.rs; packages/open-bitcoin-cli/src/operator/status/render.rs; packages/open-bitcoin-cli/src/operator/support/render.rs]
**Warning signs:** Tests include inbound evidence assertions only in renderer modules, not in `OpenBitcoinStatusSnapshot` serialization. [VERIFIED: docs/architecture/status-snapshot.md]

### Pitfall 6: Public Exposure In Hermetic Tests

**What goes wrong:** Tests bind `0.0.0.0`, `::`, or non-loopback addresses by default. [VERIFIED: 90-CONTEXT.md]
**Why it happens:** Knots defaults can bind wildcard interfaces under full listener configuration. [VERIFIED: packages/bitcoin-knots/src/net.cpp; packages/bitcoin-knots/src/init.cpp]
**How to avoid:** Default Phase 90 tests use loopback and `127.0.0.1:0`; wildcard/public endpoints require `inbound.allow_public = true` and opt-in UAT wording. [VERIFIED: 90-CONTEXT.md; packages/open-bitcoin-test-harness/src/isolation.rs]
**Warning signs:** `bash scripts/verify.sh` needs a routable interface or fails on firewall/network conditions. [VERIFIED: docs/parity/release-readiness.md]

### Pitfall 7: Missing Parity Breadcrumbs

**What goes wrong:** New first-party Rust files fail the breadcrumb checker. [VERIFIED: scripts/check-parity-breadcrumbs.ts]
**Why it happens:** Adding `inbound.rs` or tests creates new paths outside the existing mapping. [VERIFIED: docs/parity/source-breadcrumbs.json]
**How to avoid:** Add breadcrumb comments and map files to `net.cpp`, `net_processing.cpp`, `net_permissions.cpp` only when they are actual anchors; use `none` only for Open Bitcoin-only status/support infrastructure. [VERIFIED: AGENTS.md; docs/parity/source-breadcrumbs.json]
**Warning signs:** `bash scripts/verify.sh` or `bun run scripts/check-parity-breadcrumbs.ts --check` reports missing mappings. [VERIFIED: scripts/check-parity-breadcrumbs.ts]

### Pitfall 8: Claiming Relay Or Production Readiness

**What goes wrong:** Docs or status text imply inbound serving means transaction relay, compact blocks, mempool propagation, public inbound defaults, or production-node readiness. [VERIFIED: 90-CONTEXT.md; docs/parity/release-readiness.md]
**Why it happens:** In Bitcoin Core/Knots, inbound service is adjacent to broader peer services. [VERIFIED: packages/bitcoin-knots/src/net_processing.cpp; packages/bitcoin-knots/src/net_permissions.h]
**How to avoid:** Use bounded wording: "opt-in loopback/default-disabled listener/admission evidence" and preserve deferred-surface language. [VERIFIED: 90-CONTEXT.md; docs/parity/catalog/p2p.md]
**Warning signs:** Phase 90 docs mention "production inbound", "public node", "relay-ready", or "full address relay". [VERIFIED: 90-CONTEXT.md]

## Code Examples

Verified patterns from local sources:

### Current Inbound Peer Entry Point

```rust
// Source: packages/open-bitcoin-network/src/peer.rs
pub fn add_inbound_peer(&mut self, peer_id: PeerId) -> Result<(), NetworkError> {
    if self.peers.contains_key(&peer_id) {
        return Err(NetworkError::PeerAlreadyExists(peer_id));
    }
    self.peers.insert(peer_id, PeerState::new(ConnectionRole::Inbound));
    Ok(())
}
```

Use this path as the extension point for richer `InboundPeerRecord` insertion rather than adding a separate peer store. [VERIFIED: packages/open-bitcoin-network/src/peer.rs]

### Current Inbound Handshake Action Flow

```rust
// Source: packages/open-bitcoin-network/src/peer.rs
if !peer.local_version_sent {
    peer.local_version_sent = true;
    actions.push(PeerAction::Send(WireNetworkMessage::Version(...)));
}
if !peer.local_verack_sent {
    peer.local_verack_sent = true;
    actions.push(PeerAction::Send(WireNetworkMessage::WtxidRelay));
    actions.push(PeerAction::Send(WireNetworkMessage::Verack));
    actions.push(PeerAction::Send(WireNetworkMessage::SendHeaders));
}
```

Do not duplicate this handshake sequence in the accept loop; the listener should feed messages into `PeerManager`. [VERIFIED: packages/open-bitcoin-network/src/peer.rs]

### Current Count Projection

```rust
// Source: packages/open-bitcoin-node/src/network.rs
match peer.role {
    ConnectionRole::Inbound => inbound_peers += 1,
    ConnectionRole::Outbound => outbound_peers += 1,
}
```

Extend this projection to use admission/handshake state so rejected peers are visible in evidence but not counted as admitted. [VERIFIED: packages/open-bitcoin-node/src/network.rs; 90-CONTEXT.md]

### Current JSONC Config Shape

```rust
// Source: packages/open-bitcoin-rpc/src/config/open_bitcoin.rs
#[serde(default, deny_unknown_fields)]
pub struct OpenBitcoinConfig {
    pub sync: SyncConfig,
}
```

Add an `inbound: InboundConfig` sibling with `Default` disabled and `deny_unknown_fields`. [VERIFIED: packages/open-bitcoin-rpc/src/config/open_bitcoin.rs; 90-CONTEXT.md]

### Current Daemon Listener Pattern

```rust
// Source: packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs
let listener = tokio::net::TcpListener::bind(bind_address).await?;
let serve_result = axum::serve(listener, http::router(state))
    .with_graceful_shutdown(shutdown_signal())
    .await;
```

Follow this shell pattern for P2P listener startup and shutdown, but keep policy decisions outside the Tokio accept loop. [VERIFIED: packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs; standards/core/architecture.md]

## Status, RPC, Metrics, Logs, And Support Evidence

- `getnetworkinfo` already serializes `connections`, `connections_in`, and `connections_out`; keep these baseline-shaped fields accurate and avoid adding Open Bitcoin-specific listener fields directly to the Knots-shaped response unless an explicitly named Open Bitcoin RPC/status surface owns them. [VERIFIED: packages/open-bitcoin-rpc/src/method/node.rs; packages/open-bitcoin-rpc/src/dispatch/node.rs; packages/bitcoin-knots/src/rpc/net.cpp]
- `OpenBitcoinStatusSnapshot` currently contains `peers: PeerStatus`; add inbound listener/admission evidence here or in a clearly owned child status contract consumed by `PeerStatus`. [VERIFIED: packages/open-bitcoin-node/src/status.rs; docs/architecture/status-snapshot.md]
- CLI live status currently maps `GetNetworkInfoResponse.connections_in/out` into `PeerCounts`; extend live status collection to include new Open Bitcoin-specific inbound evidence if the daemon exposes it through a shared status/RPC contract. [VERIFIED: packages/open-bitcoin-cli/src/operator/status.rs]
- Support bundle Markdown currently embeds the shared status snapshot and compact evidence summaries; add a bounded inbound section rather than raw peer tables. [VERIFIED: packages/open-bitcoin-cli/src/operator/support/render.rs; 90-CONTEXT.md]
- Metrics currently has `MetricKind::PeerCount`; add separate numeric inbound metrics only if they remain bounded and low-cardinality, such as admitted/rejected counts. [VERIFIED: packages/open-bitcoin-node/src/metrics.rs; docs/architecture/operator-observability.md]
- Structured logs should carry compact labels such as `inbound_preflight_reason`, `inbound_listener_state`, `admission_reject_reason`, and `inbound_handshake_state`; do not log unbounded peer tables. [VERIFIED: docs/architecture/operator-observability.md; packages/open-bitcoin-node/src/logging/writer.rs]

## Test Strategy

### Unit Tests

| Area | Required Behaviors | Suggested Location |
|------|--------------------|--------------------|
| Config parsing | `inbound.enabled` defaults false; CLI overrides JSONC; unknown JSONC fields rejected; invalid max/reserved values fail with field-specific messages. [VERIFIED: 90-CONTEXT.md; packages/open-bitcoin-rpc/src/config/tests.rs] | `packages/open-bitcoin-rpc/src/config/tests.rs` |
| Pure preflight | `disabled`, `no_listen_addresses`, `invalid_endpoint`, `unsafe_endpoint`, and `ready` labels are stable and include endpoint/message/next action. [VERIFIED: 90-CONTEXT.md] | `packages/open-bitcoin-network/src/inbound/tests.rs` or `packages/open-bitcoin-node/src/inbound/tests.rs` |
| Admission policy | Cap reached, reserved slots, duplicate endpoint, duplicate peer id, shutdown, and admit decisions are deterministic. [VERIFIED: 90-CONTEXT.md] | `packages/open-bitcoin-network/src/inbound/tests.rs` |
| Self-connection | Remote `version.nonce == LocalPeerConfig.nonce` rejects and leaves admitted count unchanged. [VERIFIED: 90-CONTEXT.md; packages/open-bitcoin-network/src/message.rs; packages/bitcoin-knots/src/net_processing.cpp] | `packages/open-bitcoin-network/src/peer/tests.rs` |
| Handshake state | Inbound peer starts without local version, then emits local `version`, `wtxidrelay`, `verack`, and `sendheaders` through `PeerAction`. [VERIFIED: packages/open-bitcoin-network/src/peer.rs] | `packages/open-bitcoin-network/src/peer/tests.rs` |
| Count projection | Inbound and outbound counts stay separate; rejected peers remain visible in evidence but do not inflate admitted counts. [VERIFIED: packages/open-bitcoin-node/src/network.rs; 90-CONTEXT.md] | `packages/open-bitcoin-node/src/network/tests.rs` |
| Status/rendering | Human and JSON status render shared inbound fields; unavailable fields preserve reasons. [VERIFIED: docs/architecture/status-snapshot.md; packages/open-bitcoin-cli/src/operator/status/render.rs] | `packages/open-bitcoin-cli/src/operator/status/tests.rs`; `packages/open-bitcoin-cli/src/operator/status/render/tests.rs` |
| Support redaction | Support evidence includes bounded admission diagnostics and omits/redacts raw endpoint tables. [VERIFIED: 90-CONTEXT.md; packages/open-bitcoin-cli/src/operator/support/render.rs] | `packages/open-bitcoin-cli/src/operator/support/tests.rs` |

### Integration Tests

- Disabled inbound config should not bind and should not spawn an accept loop. [VERIFIED: 90-CONTEXT.md]
- Invalid endpoint and unsafe wildcard/public endpoint should return stable diagnostics before bind. [VERIFIED: 90-CONTEXT.md]
- Already-bound loopback address should produce `already_bound` or `bind_unavailable` according to the final reason taxonomy, with endpoint and next action. [VERIFIED: 90-CONTEXT.md; packages/open-bitcoin-test-harness/src/isolation.rs]
- Enabled loopback listener should accept a synthetic peer, drive the existing version/verack path, increment inbound counts, and leave outbound counts/targets unchanged. [VERIFIED: 90-CONTEXT.md; packages/open-bitcoin-network/src/peer.rs; packages/open-bitcoin-node/src/network.rs]
- Shutdown should stop accepting new peers without disturbing existing outbound sync unless daemon shutdown is explicit. [VERIFIED: 90-CONTEXT.md; packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs]

### Verification Commands

```bash
# Fast local iteration while implementing.
bash scripts/verify.sh --fast

# Required repo-native verification before marking Phase 90 complete.
bash scripts/verify.sh

# Focused Rust checks for likely touched crates during iteration.
cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network
cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node
cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc
cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli

# Breadcrumb checker if new Rust files are added.
bun run scripts/check-parity-breadcrumbs.ts --check
```

These commands are available in the current environment except that full `bash scripts/verify.sh` was not run during research. [VERIFIED: bash scripts/verify.sh --help; rustc --version; bun --version]

## UAT Commands To Document After Implementation

Use loopback and explicit Open Bitcoin config for UAT; do not use public interfaces unless the operator opts in with `inbound.allow_public = true`. [VERIFIED: 90-CONTEXT.md]

```bash
# Prepare a loopback-only UAT datadir.
export OB_UAT_DIR=/tmp/open-bitcoin-inbound-uat
rm -rf "$OB_UAT_DIR"
mkdir -p "$OB_UAT_DIR"
cat > "$OB_UAT_DIR/open-bitcoin.jsonc" <<'JSONC'
{
  "inbound": {
    "enabled": true,
    "listen_addresses": ["127.0.0.1:18444"],
    "max_peers": 4,
    "reserved_slots": 1
  }
}
JSONC
```

Cargo daemon form:

```bash
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --bin open-bitcoind -- \
  -regtest \
  -datadir="$OB_UAT_DIR" \
  -openbitcoinconf="$OB_UAT_DIR/open-bitcoin.jsonc" \
  -server=1
```

Bazel daemon form:

```bash
bazel run //packages/open-bitcoin-rpc:open_bitcoind -- \
  -regtest \
  -datadir="$OB_UAT_DIR" \
  -openbitcoinconf="$OB_UAT_DIR/open-bitcoin.jsonc" \
  -server=1
```

Cargo baseline RPC inspection:

```bash
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin-cli -- \
  -regtest \
  -datadir="$OB_UAT_DIR" \
  getnetworkinfo
```

Bazel baseline RPC inspection:

```bash
bazel run //packages/open-bitcoin-cli:open_bitcoin_cli -- \
  -regtest \
  -datadir="$OB_UAT_DIR" \
  getnetworkinfo
```

Cargo operator status inspection:

```bash
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- \
  --datadir "$OB_UAT_DIR" \
  --network regtest \
  --format json \
  status
```

Bazel operator status inspection:

```bash
bazel run //packages/open-bitcoin-cli:open_bitcoin -- \
  --datadir "$OB_UAT_DIR" \
  --network regtest \
  --format json \
  status
```

The planner should add a deterministic synthetic peer command or test helper if manual UAT needs a real version/verack exchange from the command line. [ASSUMED]

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Knots `-listen`, `-bind`, wildcard binding, and permission-bearing `-whitebind` are part of a mature integrated C++ node runtime. [VERIFIED: packages/bitcoin-knots/src/init.cpp; packages/bitcoin-knots/src/net.cpp; packages/bitcoin-knots/src/net_permissions.cpp] | Phase 90 uses Open Bitcoin-owned JSONC and daemon CLI flags with disabled-by-default loopback-first listener activation. [VERIFIED: 90-CONTEXT.md] | Phase 90, 2026-06-25. [VERIFIED: 90-CONTEXT.md] | Avoids claiming full Knots config/permission parity while adding auditable inbound admission. [VERIFIED: 90-CONTEXT.md] |
| Existing Open Bitcoin has inbound peer state but no real listener. [VERIFIED: packages/open-bitcoin-network/src/peer.rs; packages/open-bitcoin-node/src/network.rs] | Add opt-in runtime listener plus pure admission policy and shared evidence. [VERIFIED: 90-CONTEXT.md] | Phase 90. [VERIFIED: .planning/ROADMAP.md] | Reuses core peer lifecycle and limits blast radius. [VERIFIED: standards/core/architecture.md] |
| Broad peer permissions are handled in Knots through `NetPermissionFlags`. [VERIFIED: packages/bitcoin-knots/src/net_permissions.h] | Phase 90 models only reserved slots and stable admission reasons; Phase 91 owns permission classes. [VERIFIED: 90-CONTEXT.md] | Phase 90/91 split. [VERIFIED: .planning/ROADMAP.md] | Prevents relay/mempool/address/ban scope creep. [VERIFIED: 90-CONTEXT.md] |

**Deprecated/outdated for this phase:**

- Treating public inbound listener exposure as a default behavior is out of scope for Phase 90. [VERIFIED: 90-CONTEXT.md]
- Treating inbound admission as proof of transaction relay, compact block relay, mempool propagation, full address relay, or production readiness is out of scope. [VERIFIED: 90-CONTEXT.md; docs/parity/release-readiness.md]
- Adding Open Bitcoin-only listener keys to `bitcoin.conf` is out of scope. [VERIFIED: docs/architecture/config-precedence.md; 90-CONTEXT.md]

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | `bind_unavailable` and `already_bound` can be represented as typed listener activation outcomes after pure config preflight, because the OS bind attempt is required to observe them. [ASSUMED] | Architecture Patterns | If the user interprets D-03 as forbidding even a scoped bind probe before all preflight outcomes, the planner must ask for clarification or split the labels into `preflight_*` and `activation_*`. |
| A2 | Phase 90 reserved slots can be modeled as a minimal ordinary/reserved slot primitive without exposing Knots permission classes. [ASSUMED] | Architecture Patterns / Test Strategy | If reserved eligibility must be operator-configurable in Phase 90, the plan needs an additional explicit non-permission config field and tests. |
| A3 | Manual UAT may need a new synthetic peer helper if operators must prove version/verack over TCP from the command line. [ASSUMED] | UAT Commands | If no helper is added, UAT can still inspect listener preflight/status, but handshake UAT may stay test-only. |

## Open Questions (RESOLVED)

1. **How should the final taxonomy separate pure preflight from OS bind activation?** RESOLVED: Use a two-stage typed contract. Plan 01 owns the pure config classifier for `disabled`, `no_listen_addresses`, `invalid_endpoint`, `unsafe_endpoint`, and `ready`; Plan 04 owns OS-observed activation diagnostics for `bind_unavailable` and `already_bound`. [VERIFIED: .planning/phases/90-inbound-listener-and-admission-policy/90-01-PLAN.md; .planning/phases/90-inbound-listener-and-admission-policy/90-04-PLAN.md]
   - What we know: Phase 90 requires stable labels including `bind_unavailable` and `already_bound`. [VERIFIED: 90-CONTEXT.md]
   - Resolution detail: Those two outcomes require an OS bind attempt, so the plans keep them out of the pure classifier and preserve both classifier and activation results in status/support evidence. [VERIFIED: standards/core/architecture.md; .planning/phases/90-inbound-listener-and-admission-policy/90-04-PLAN.md]

2. **Should Phase 90 expose a manual synthetic peer UAT helper?** RESOLVED: Do not require a standalone operator synthetic peer helper for Phase 90 execution readiness. Plan 04 must prove version/verack admission through hermetic loopback integration tests; Plan 09 must document repo-local daemon, `getnetworkinfo`, status, and support-bundle UAT commands. If execution introduces a loopback-only helper as test infrastructure, Plan 09 must document its scope, but the helper is not a required public operator surface. [VERIFIED: .planning/phases/90-inbound-listener-and-admission-policy/90-04-PLAN.md; .planning/phases/90-inbound-listener-and-admission-policy/90-09-PLAN.md]
   - What we know: Default verification can use injected transports and hermetic fixtures. [VERIFIED: 90-CONTEXT.md]
   - Resolution detail: Operator UAT remains command-based and loopback-first, while handshake proof remains in deterministic tests unless a local helper naturally falls out of Plan 04 implementation. [VERIFIED: 90-CONTEXT.md; AGENTS.md]

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|-------------|-----------|---------|----------|
| Rust / Cargo | Build, tests, implementation | yes | `rustc 1.94.1`, `cargo 1.94.1` | None needed. [VERIFIED: rustc --version; cargo --version] |
| Bun | TypeScript verification and breadcrumb checker | yes | `1.3.9` | None needed. [VERIFIED: bun --version] |
| Bazelisk/Bazel | Bazel smoke builds and UAT command forms | yes | `bazel 8.6.0` | Cargo command forms remain available for UAT, but repo verification expects Bazel smoke in full mode. [VERIFIED: bazelisk --version; AGENTS.md] |
| Bitcoin Knots submodule | Parity anchors | yes | `v29.3.knots20260210` at `a9aee730466ac67d35a3c03ee24676be5e045878` | Run `git submodule update --init --recursive` if missing. [VERIFIED: git submodule status --recursive packages/bitcoin-knots; AGENTS.md] |
| `scripts/verify.sh` | Repo-native verification | yes | Supports `--full`, `--profile`, `--fast`, `--timings` | No fallback; use this as the verification contract. [VERIFIED: bash scripts/verify.sh --help] |

**Missing dependencies with no fallback:**
- None found during research. [VERIFIED: rustc --version; bun --version; bazelisk --version; git submodule status]

**Missing dependencies with fallback:**
- None found during research. [VERIFIED: rustc --version; bun --version; bazelisk --version; git submodule status]

## Security Domain

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|------------------|
| V2 Authentication | no | Bitcoin P2P listener admission is not an authenticated app-user login surface in Phase 90; do not invent authentication. [VERIFIED: packages/bitcoin-knots/src/protocol.h; 90-CONTEXT.md] |
| V3 Session Management | no | There are no web/app sessions; peer lifecycle state is connection state and should stay typed in `InboundPeerRecord`/`PeerState`. [VERIFIED: packages/open-bitcoin-network/src/peer.rs] |
| V4 Access Control | yes | Disabled-by-default config, `allow_public` gate, inbound caps, reserved-slot primitive, and duplicate/self-connection rejection. [VERIFIED: 90-CONTEXT.md] |
| V5 Input Validation | yes | Parse endpoints into typed socket addresses, reject invalid/unsafe endpoints with stable labels, and parse wire messages through existing codec. [VERIFIED: packages/open-bitcoin-rpc/src/config/loader/rpc_address.rs; packages/open-bitcoin-network/src/message.rs; 90-CONTEXT.md] |
| V6 Cryptography | limited | Do not add new crypto; preserve existing nonce/self-connection checks and existing message checksum/codec behavior. [VERIFIED: packages/open-bitcoin-network/src/message.rs; packages/bitcoin-knots/src/net_processing.cpp] |
| V7 Error Handling/Logging | yes | Use stable reason codes, bounded structured logs, and redacted/bounded support evidence. [VERIFIED: 90-CONTEXT.md; docs/architecture/operator-observability.md; packages/open-bitcoin-cli/src/operator/support/render.rs] |
| V12 File/Resource | yes | Keep default tests loopback-only, enforce caps, and avoid unbounded peer tables or endpoint dumps. [VERIFIED: 90-CONTEXT.md; docs/architecture/operator-observability.md] |

### Known Threat Patterns For Phase 90

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Accidental public listener exposure | Information Disclosure / Elevation of Privilege | Disabled by default; public/wildcard endpoints require explicit `inbound.allow_public = true`; default verification uses loopback only. [VERIFIED: 90-CONTEXT.md] |
| Listener bind hijack or wrong endpoint | Tampering / Denial of Service | Typed preflight diagnostics with endpoint, reason, message, and next action; already-bound errors remain operator-visible. [VERIFIED: 90-CONTEXT.md; packages/bitcoin-knots/src/net.cpp] |
| Connection-slot exhaustion | Denial of Service | Pure admission caps, reserved slots, stable cap rejection reason, and separate outbound target accounting. [VERIFIED: 90-CONTEXT.md] |
| Self-connection loops | Spoofing / Denial of Service | Reject remote `version.nonce` matching local nonce and keep count unchanged. [VERIFIED: packages/bitcoin-knots/src/net_processing.cpp; packages/open-bitcoin-network/src/message.rs; 90-CONTEXT.md] |
| Duplicate endpoint/peer records | Tampering / Denial of Service | Stable endpoint keys plus duplicate peer id rejection before admission count projection. [VERIFIED: packages/open-bitcoin-network/src/peer.rs; 90-CONTEXT.md] |
| Endpoint leakage in support bundles | Information Disclosure | Redact or bound endpoint evidence and avoid raw unbounded peer tables. [VERIFIED: 90-CONTEXT.md; packages/open-bitcoin-cli/src/operator/support/render.rs] |
| Scope creep into relay/permissions | Elevation of Privilege / Tampering | Keep relay, mempool, address relay, permission classes, eviction, ban, and DoS resource governance deferred to later phases. [VERIFIED: 90-CONTEXT.md; packages/bitcoin-knots/src/net_permissions.h] |

## Sources

### Primary (HIGH confidence)

- `.planning/phases/90-inbound-listener-and-admission-policy/90-CONTEXT.md` - locked decisions, phase boundaries, canonical references, deferred scope. [VERIFIED: local file read]
- `.planning/REQUIREMENTS.md` - INB-01 through INB-05 and v1.9 deferred requirements. [VERIFIED: local file read]
- `.planning/ROADMAP.md` via `gsd-tools roadmap get-phase 90` - phase goal and requirement mapping. [VERIFIED: gsd-tools]
- `AGENTS.md`, `AGENTS.bright-builds.md`, `standards-overrides.md`, `standards/core/*`, `standards/languages/rust.md` - repo workflow, verification, testing, Rust, and architecture rules. [VERIFIED: local file read]
- `packages/open-bitcoin-network/src/peer.rs` and `packages/open-bitcoin-network/src/message.rs` - peer lifecycle, inbound role, version/verack flow, nonce fields, codec. [VERIFIED: local code read]
- `packages/open-bitcoin-node/src/network.rs`, `status.rs`, `metrics.rs` - managed peer counts, shared status, metric kinds. [VERIFIED: local code read]
- `packages/open-bitcoin-rpc/src/config*`, `context/network.rs`, `method/node.rs`, `dispatch/node.rs`, `bin/open-bitcoind.rs` - config precedence, daemon CLI parsing, getnetworkinfo counts, daemon listener/shutdown patterns. [VERIFIED: local code read]
- `packages/open-bitcoin-cli/src/operator/status*`, `support/*`, `operator.rs`, `BUILD.bazel` - status/support rendering and UAT command targets. [VERIFIED: local code read]
- `packages/open-bitcoin-test-harness/src/isolation.rs` - loopback port reservation test helper. [VERIFIED: local code read]
- `docs/architecture/config-precedence.md`, `status-snapshot.md`, `operator-observability.md`, `docs/operator/runtime-guide.md`, `docs/parity/catalog/p2p.md`, `docs/parity/release-readiness.md`, `docs/parity/source-breadcrumbs.json` - config/status/evidence/parity contracts. [VERIFIED: local docs read]
- `packages/bitcoin-knots/src/net.cpp`, `net_processing.cpp`, `net_permissions.cpp`, `net_permissions.h`, `rpc/net.cpp`, `init.cpp`, `test/functional/p2p_handshake.py` - listener, accept, bind, handshake, self-connection, permissions, `getnetworkinfo`, and Knots functional anchors. [VERIFIED: local Knots submodule read]

### Secondary (MEDIUM confidence)

- None used; research relied on repo-local code/docs and vendored Knots anchors. [VERIFIED: research process]

### Tertiary (LOW confidence)

- No web-only sources used. [VERIFIED: research process]

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - all recommended crates/tools are existing repo dependencies verified through `cargo tree`, local docs, and tool version commands. [VERIFIED: cargo tree; rustc --version; bun --version; bazelisk --version]
- Architecture: HIGH - core boundaries are locked by phase context and reinforced by existing code/standards. [VERIFIED: 90-CONTEXT.md; standards/core/architecture.md; packages/open-bitcoin-network/src/peer.rs; packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs]
- Pitfalls: HIGH - pitfalls are derived from locked decisions, existing implementation gaps, and Knots anchors. [VERIFIED: 90-CONTEXT.md; local code reads]
- UAT: MEDIUM - daemon/CLI command forms and Bazel targets are verified, but a synthetic TCP peer helper may need implementation for operator handshake UAT. [VERIFIED: AGENTS.md; packages/open-bitcoin-rpc/BUILD.bazel; packages/open-bitcoin-cli/BUILD.bazel; ASSUMED]

**Research date:** 2026-06-25
**Valid until:** 2026-07-25, or earlier if Phase 90 planning/implementation changes the status/config surfaces. [ASSUMED]
