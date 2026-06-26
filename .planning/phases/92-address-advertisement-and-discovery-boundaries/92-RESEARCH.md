# Phase 92: Address Advertisement and Discovery Boundaries - Research

**Researched:** 2026-06-26 [VERIFIED: developer current date]
**Domain:** Bitcoin P2P local address advertisement, bounded `getaddr`/`addr` handling, and typed learned-address policy [VERIFIED: .planning/ROADMAP.md; .planning/phases/92-address-advertisement-and-discovery-boundaries/92-CONTEXT.md]
**Confidence:** HIGH [VERIFIED: local code inspection; pinned Knots source inspection]

<user_constraints>
## User Constraints (from CONTEXT.md) [VERIFIED: .planning/phases/92-address-advertisement-and-discovery-boundaries/92-CONTEXT.md]

### Locked Decisions

## Implementation Decisions

### Local Listener Advertisement

- **D-01:** Address advertisement starts from Open Bitcoin-owned `inbound.listen_addresses` and runtime-bound listener evidence created in Phase 90. Do not infer public advertisements from arbitrary local interfaces, outbound peers, DNS discovery, UPnP/NAT-PMP, external IP probes, or baseline Knots `-externalip`/`-discover` compatibility.
- **D-02:** Candidate derivation must be a pure decision that accepts typed listener endpoints, listener state, service flags, reachability/privacy configuration, and current network boundary inputs. Runtime socket code should only consume the decision output.
- **D-03:** Loopback, private, unspecified, multicast, documentation, and otherwise unroutable addresses are not advertised to public peers. Loopback may be retained only as deterministic local/UAT evidence with a stable reason such as `not_publicly_routable`.
- **D-04:** Privacy-network boundaries are explicit. Onion, I2P, CJDNS, and future non-IP reachability should be represented as deferred or unsupported address networks unless the planner adds a bounded typed placeholder with tests proving it cannot leak or relay unsupported privacy-network addresses.
- **D-05:** Version-message sender address behavior should stay conservative. Do not start sending a routable local address in `version` unless the address passes the same typed candidate policy; otherwise keep the existing zero-address behavior.

### Bounded `getaddr` Response Policy

- **D-06:** Add `getaddr` and `addr` message support only for bounded request/response behavior. This phase should not implement gossip relay, addr rebroadcast scheduling, trickle relay, unsolicited address fanout, or full addr relay peer selection.
- **D-07:** The `getaddr` response policy must be deterministic and permission-aware. Permission decisions from Phase 91 should influence whether a peer is eligible for address responses through the existing `addr`/address-response policy input, but raw class names and raw config strings must stay out of status/support output.
- **D-08:** Responses must be capped by explicit count, age, source, cache, and request-frequency rules. The cap should be small enough for deterministic tests and should not depend on wall-clock network crawling or public peers.
- **D-09:** The response cache should be typed and inspectable. Each returned address must have evidence for source, first-seen or last-seen freshness, routability classification, services, port, and whether it came from local listener advertisement or learned peer-address storage.
- **D-10:** Repeated `getaddr` requests from the same peer must not create unbounded work or change relay state. Use a stable "served once" or deterministic request-window policy, with a reason label when a later request is suppressed.

### Learned Address Management Contract

- **D-11:** Introduce a first-party typed address-management contract before durable persistence details become complicated. Required concepts include network kind, address bytes or endpoint, service flags, source, freshness timestamps, routability class, and persistence eligibility.
- **D-12:** Learned `addr` entries should be accepted only through parser and policy boundaries. Invalid ports, unsupported address networks, unroutable entries, stale timestamps, self/local loopback leakage, and over-cap batches must produce stable rejection or quarantine reasons.
- **D-13:** Persistence may be an in-memory or snapshot-backed contract in this phase, but it must expose deterministic evidence showing what would be persisted and why. Do not imply full Knots `addrman.dat`, anchor persistence, DNS seed rotation, or production peer-discovery parity unless explicitly implemented and tested.
- **D-14:** Learned-address state should integrate with existing pure network/domain crates first, then project bounded status/support evidence through shared node status surfaces. Avoid renderer-local address summaries.

### Operator Evidence, Docs, And Release Boundaries

- **D-15:** Status/support evidence should distinguish at least four concepts with stable labels: local listener advertisement candidates, suppressed advertisements, bounded `getaddr` responses, and learned address-management entries.
- **D-16:** Documentation must keep local listener advertisement, inbound `getaddr` responses, learned address storage, peer discovery, and full address relay visibly separate. Any future full-relay wording belongs to deferred/future sections.
- **D-17:** Deterministic release checks should guard the boundary by proving docs and parity catalogs mention Phase 92 address behavior without claiming full address relay, broader peer discovery, public-network defaults, or production readiness.
- **D-18:** Operator UAT commands, if added, must include repo-local Cargo and Bazel forms from `AGENTS.md`; do not rely on an installed `open-bitcoin` alias alone.

### Verification And UAT

- **D-19:** Default verification must stay deterministic, local, public-network-free, service-manager-free, and short-running. Use pure policy tests, synthetic `addr`/`getaddr` messages, loopback listener fixtures, and fixed docs/checker fixtures.
- **D-20:** Unit tests should cover local candidate classification, privacy-network suppression, `version` sender-address gating, `getaddr` response caps, permission-aware address responses, duplicate/stale/unroutable learned entries, and no full-relay side effects.
- **D-21:** Add parity breadcrumbs for any new first-party Rust source or test files under `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests`, including `docs/parity/source-breadcrumbs.json` entries. Use `none` only for Open Bitcoin-only status/support infrastructure without a defensible Knots anchor.

### the agent's Discretion

The planner may choose exact type names, module splits, and whether the first learned-address store is in-memory or snapshot-backed. Prefer a small pure address-policy/address-manager module in `open-bitcoin-network`, thin projection in `open-bitcoin-node`, config or CLI additions only when needed for scoped behavior, and docs/checkers that make non-claims explicit.

### Deferred Ideas (OUT OF SCOPE)

- Phase 93 owns eviction, disconnect, discourage, ban, expiry, unban, and misbehavior behavior.
- Phase 94 owns broader inbound DoS/resource governance beyond address response caps.
- Phase 95 owns v1.9 release-boundary docs and no-claim evidence across inbound serving.
- Future milestones own full address relay, addr rebroadcast scheduling, address gossip fanout, `addrv2` relay parity, DNS seed governance, public inbound defaults, public-network CI, and production full-node readiness.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| ADDR-01 | The node can derive local listen address candidates and advertise only configured, reachable, and privacy-safe addresses according to scoped Knots parity rules. [VERIFIED: .planning/REQUIREMENTS.md] | Use Phase 90 `InboundListenerConfig`, `InboundListenerEndpoint`, and runtime listener evidence as the only advertisement inputs, then apply a pure routability/privacy selector anchored to Knots `GetLocal`, `GetLocalAddrForPeer`, `AddLocal`, and `CNetAddr::IsRoutable`. [VERIFIED: packages/open-bitcoin-network/src/inbound.rs; packages/open-bitcoin-rpc/src/inbound_listener.rs; packages/bitcoin-knots/src/net.cpp; packages/bitcoin-knots/src/netaddress.cpp] |
| ADDR-02 | The node can answer inbound address requests within bounded cache, count, age, and permission rules without claiming full address-relay network participation. [VERIFIED: .planning/REQUIREMENTS.md] | Add `getaddr` and legacy `addr` message support to `WireNetworkMessage`, implement inbound-only deterministic response policy in the peer layer, and use Phase 91 `AddressResponsePolicyInput` only as an eligibility input. [VERIFIED: packages/open-bitcoin-network/src/message.rs; packages/open-bitcoin-network/src/peer.rs; packages/open-bitcoin-network/src/inbound/permissions.rs; packages/bitcoin-knots/src/net_processing.cpp] |
| ADDR-03 | Learned peer addresses enter a typed address-management contract with routability, source, freshness, and persistence boundaries that can be verified deterministically. [VERIFIED: .planning/REQUIREMENTS.md] | Introduce a first-party typed learned-address entry/store in `open-bitcoin-network` that records source, first/last seen, services, routability, and persistence eligibility, without implementing Knots' randomized bucketed `AddrMan` or `peers.dat`. [VERIFIED: packages/bitcoin-knots/src/protocol.h; packages/bitcoin-knots/src/addrman.cpp; packages/bitcoin-knots/src/addrdb.cpp; .planning/phases/92-address-advertisement-and-discovery-boundaries/92-CONTEXT.md] |
| ADDR-04 | Documentation and release checks distinguish local listener advertisement, inbound `getaddr` response behavior, peer discovery, and full address relay. [VERIFIED: .planning/REQUIREMENTS.md] | Extend the existing Phase 90/91 parity-doc and Bun-checker pattern with a Phase 92 surface id, required evidence labels, UAT command fragments, source breadcrumbs, and forbidden overclaim strings. [VERIFIED: docs/parity/catalog/p2p.md; docs/parity/checklist.md; docs/parity/index.json; scripts/check-phase90-inbound-listener-admission.ts; scripts/check-phase91-peer-permissions.ts] |
</phase_requirements>

## Project Constraints (from AGENTS.md)

- Prefer root `AGENTS.md` over `CLAUDE.md`; this repo has a root `AGENTS.md`. [VERIFIED: AGENTS.md]
- Read `AGENTS.bright-builds.md`, `standards-overrides.md`, and relevant standards before planning or implementation; those files define functional-core, verification, testing, and language rules. [VERIFIED: AGENTS.md; AGENTS.bright-builds.md; standards-overrides.md; standards/core/architecture.md; standards/core/code-shape.md; standards/core/testing.md; standards/core/verification.md; standards/languages/rust.md; standards/languages/typescript-javascript.md]
- Use `git submodule update --init --recursive` if the pinned Knots baseline is missing; the local submodule is present at `a9aee730466ac67d35a3c03ee24676be5e045878` tagged `v29.3.knots20260210`. [VERIFIED: AGENTS.md; git submodule status packages/bitcoin-knots]
- Use `rust-toolchain.toml` as the Rust source of truth; local `rustc` and `cargo` report `1.94.1`. [VERIFIED: AGENTS.md; rust-toolchain.toml; rustc --version; cargo --version]
- Use `bash scripts/verify.sh` as the repo-native verification contract for first-party code; `--fast` is local iteration only. [VERIFIED: AGENTS.md; scripts/verify.sh]
- During UAT, provide repo-local Cargo and Bazel command forms rather than only an installed `open-bitcoin` alias. [VERIFIED: AGENTS.md]
- Use Bun for repo-owned substantial TypeScript automation scripts; local Bun reports `1.3.9` and the repo has no `package.json` bootstrap step. [VERIFIED: AGENTS.md; .bun-version; bun --version; test ! -f package.json]
- Treat `docs/metrics/lines-of-code.md` as tracked generated output if verification refreshes it. [VERIFIED: AGENTS.md]
- Record intentional in-scope behavior differences from Bitcoin Knots in `docs/parity/index.json` and companion docs under `docs/parity/`. [VERIFIED: AGENTS.md; docs/parity/index.json; docs/parity/catalog/p2p.md]
- New first-party Rust source or test files under `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests` require parity breadcrumb comments and `docs/parity/source-breadcrumbs.json` entries. [VERIFIED: AGENTS.md; docs/parity/source-breadcrumbs.json]
- Keep pure Bitcoin domain behavior in functional-core crates and isolate filesystem, process, network, terminal, RPC, service-manager, and durable-storage effects in shell adapters. [VERIFIED: AGENTS.md; standards/core/architecture.md]
- Rust module layout should prefer `foo.rs` plus `foo/` over new `foo/mod.rs` trees. [VERIFIED: standards/languages/rust.md]
- Unit tests for pure/business logic should test behavior, one concern per test, with Arrange/Act/Assert comments when that improves clarity. [VERIFIED: AGENTS.md; standards/core/testing.md]
- Production-path code must not add existing Rust Bitcoin libraries; Open Bitcoin owns its domain model and implementation surface. [VERIFIED: AGENTS.md]
- No project-local skills were found under `.claude/skills/` or `.agents/skills/`. [VERIFIED: find .claude/skills .agents/skills -maxdepth 2 -name SKILL.md]

## Summary

Phase 92 should be planned as a narrow pure-domain address-boundary extension, not as full peer discovery or address relay. [VERIFIED: .planning/phases/92-address-advertisement-and-discovery-boundaries/92-CONTEXT.md] The existing Open Bitcoin wire layer lacks `getaddr`, `addr`, and `addrv2` variants, while Phase 90/91 already provide the listener evidence and permission policy input this phase must consume. [VERIFIED: packages/open-bitcoin-network/src/message.rs; packages/open-bitcoin-network/src/inbound.rs; packages/open-bitcoin-network/src/inbound/permissions.rs]

Knots provides useful anchors for local address advertisement, routability, `getaddr` privacy policy, and learned-address quality rules, but Knots also includes stochastic address relay queues, rolling bloom filters, randomized response caches, `AddrMan` buckets, `peers.dat`, DNS seed paths, and `addrv2` relay behavior that are explicitly outside this phase. [VERIFIED: packages/bitcoin-knots/src/net.cpp; packages/bitcoin-knots/src/net_processing.cpp; packages/bitcoin-knots/src/addrman.cpp; packages/bitcoin-knots/src/addrdb.cpp; .planning/phases/92-address-advertisement-and-discovery-boundaries/92-CONTEXT.md]

**Primary recommendation:** Implement a small `open-bitcoin-network` address policy/address book module, add legacy `getaddr`/`addr` wire support, thread bounded evidence through `open-bitcoin-node` status and CLI/support renderers, and add a Phase 92 Bun checker that proves evidence while guarding no-claim boundaries. [VERIFIED: .planning/phases/92-address-advertisement-and-discovery-boundaries/92-CONTEXT.md; packages/open-bitcoin-network/src/message.rs; packages/open-bitcoin-node/src/status/inbound.rs; scripts/check-phase91-peer-permissions.ts]

## Standard Stack

### Core

| Library/Crate | Version | Purpose | Why Standard |
|---------------|---------|---------|--------------|
| Rust workspace crates | local `0.1.0`, Rust 2024, Rust `1.94.1` | First-party domain, peer, node, RPC, and CLI changes | The workspace manifests use edition 2024 and local first-party crate version `0.1.0`, and the repo pins Rust `1.94.1`. [VERIFIED: packages/Cargo.toml; rust-toolchain.toml; cargo metadata --manifest-path packages/Cargo.toml --no-deps; rustc --version] |
| `open-bitcoin-network` | local `0.1.0` | Pure address policy, learned-address contract, `getaddr`/`addr` peer behavior | Existing `WireNetworkMessage`, `LocalPeerConfig`, `PeerManager`, inbound listener types, and Phase 91 permission effects live here. [VERIFIED: packages/open-bitcoin-network/src/message.rs; packages/open-bitcoin-network/src/peer.rs; packages/open-bitcoin-network/src/inbound.rs; packages/open-bitcoin-network/src/inbound/permissions.rs] |
| `open-bitcoin-codec` and `open-bitcoin-primitives` | local `0.1.0` | Legacy network-address codec and primitive wire data shapes | The existing 26-byte `NetworkAddress` codec already serializes services, address bytes, and big-endian port for `version`-style addresses. [VERIFIED: packages/open-bitcoin-codec/src/network.rs; packages/open-bitcoin-primitives/src/network.rs] |
| `open-bitcoin-node` | local `0.1.0` | Managed network projection, shared status, optional low-cardinality metrics | `ManagedPeerNetwork` and `InboundPeerServingStatus` are current shared projection seams for inbound peer evidence. [VERIFIED: packages/open-bitcoin-node/src/network.rs; packages/open-bitcoin-node/src/status/inbound.rs; packages/open-bitcoin-node/src/metrics.rs] |
| `open-bitcoin-rpc` | local `0.1.0` | Runtime listener evidence and optional config/CLI wiring | The RPC context builds `LocalPeerConfig`, runs inbound listener activation, and records Phase 90/91 inbound evidence. [VERIFIED: packages/open-bitcoin-rpc/src/context/network.rs; packages/open-bitcoin-rpc/src/inbound_listener.rs; packages/open-bitcoin-rpc/src/config/open_bitcoin.rs] |
| `open-bitcoin-cli` | local `0.1.0` | Human status and support-bundle rendering | Existing inbound status/support renderers consume shared status fields and already implement redaction-style presentation. [VERIFIED: packages/open-bitcoin-cli/src/operator/status/render/inbound.rs; packages/open-bitcoin-cli/src/operator/support/render/inbound.rs] |
| Pinned Bitcoin Knots source | `v29.3.knots20260210` submodule | Parity vocabulary and behavior anchors | Phase 92 references Knots `netaddress`, `net`, `net_processing`, `addrman`, `addrdb`, and functional tests as canonical anchors. [VERIFIED: .planning/phases/92-address-advertisement-and-discovery-boundaries/92-CONTEXT.md; git submodule status packages/bitcoin-knots] |

### Supporting

| Library/Tool | Version | Purpose | When to Use |
|--------------|---------|---------|-------------|
| `serde` / `serde_json` | manifest reqs `^1.0.228` / `^1.0.149` | Stable status and support JSON shapes | Use for shared inbound/address evidence structs already serialized by node/CLI surfaces. [VERIFIED: cargo metadata --manifest-path packages/Cargo.toml --no-deps; packages/open-bitcoin-node/src/status/inbound.rs] |
| `tokio` | manifest req `^1.52.1` | Existing runtime listener I/O | Use only in thin RPC/listener adapters after pure address decisions exist. [VERIFIED: cargo metadata --manifest-path packages/Cargo.toml --no-deps; packages/open-bitcoin-rpc/src/inbound_listener.rs; standards/core/architecture.md] |
| `jsonc-parser` | manifest req `^0.32.3` | Open Bitcoin JSONC config parsing | Use only if the planner adds scoped address config knobs under the existing Open Bitcoin config boundary. [VERIFIED: cargo metadata --manifest-path packages/Cargo.toml --no-deps; packages/open-bitcoin-rpc/src/config/open_bitcoin.rs] |
| Bun | `1.3.9` | Deterministic TypeScript docs/evidence checker | Use for a Phase 92 checker following Phase 90/91 patterns. [VERIFIED: .bun-version; bun --version; scripts/check-phase90-inbound-listener-admission.ts; scripts/check-phase91-peer-permissions.ts] |
| Bazel / `rules_rust` | Bazel `8.6.0`, `rules_rust` `0.69.0` | Repo smoke build and UAT command forms | Keep UAT docs and `scripts/verify.sh` aligned with the repo's Bazel surface. [VERIFIED: bazel --version; MODULE.bazel; AGENTS.md] |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| First-party pure address policy module | Knots-style global `mapLocalHost`, interface discovery, `-discover`, `-externalip`, UPnP/NAT-PMP | Rejected because D-01 starts advertisements only from Open Bitcoin-owned listener config and Phase 90 runtime-bound evidence. [VERIFIED: .planning/phases/92-address-advertisement-and-discovery-boundaries/92-CONTEXT.md; packages/bitcoin-knots/src/net.cpp] |
| Legacy bounded `getaddr`/`addr` request-response | Full `addr` relay queue, trickle scheduling, bloom known-filter, unsolicited fanout | Rejected because D-06 forbids gossip relay, rebroadcast scheduling, trickle relay, unsolicited fanout, and full relay peer selection. [VERIFIED: .planning/phases/92-address-advertisement-and-discovery-boundaries/92-CONTEXT.md; packages/bitcoin-knots/src/net_processing.cpp] |
| Typed in-memory or snapshot-backed learned-address contract | Full Knots `AddrMan`, randomized buckets, `peers.dat`, anchors | Rejected for the first implementation because D-13 requires deterministic evidence and forbids implying full `addrman.dat` or production discovery parity. [VERIFIED: .planning/phases/92-address-advertisement-and-discovery-boundaries/92-CONTEXT.md; packages/bitcoin-knots/src/addrman.cpp; packages/bitcoin-knots/src/addrdb.cpp] |
| Defer `addrv2` or add only unsupported typed placeholder | Implement `sendaddrv2`/`addrv2` relay parity | Defer unless a bounded placeholder is required, because D-04 and D-06 keep privacy networks and `addrv2` relay outside this phase. [VERIFIED: .planning/phases/92-address-advertisement-and-discovery-boundaries/92-CONTEXT.md; packages/bitcoin-knots/src/protocol.h; packages/bitcoin-knots/test/functional/p2p_addrv2_relay.py] |
| Constants for deterministic response limits | Broad operator-facing address relay tuning flags | Prefer constants in the first pass because D-08 requires deterministic small caps and no public-peer crawling. [VERIFIED: .planning/phases/92-address-advertisement-and-discovery-boundaries/92-CONTEXT.md] |

**Installation:**

```bash
# No new external Rust, npm, or Bun dependency is recommended for Phase 92.
git submodule update --init --recursive
```

The submodule command is only needed if `packages/bitcoin-knots` is missing, and no package install is required because the recommended stack uses existing workspace crates and Bun scripts. [VERIFIED: AGENTS.md; packages/Cargo.toml; .bun-version; test ! -f package.json]

**Version verification:** Package versions were verified with `cargo metadata --manifest-path packages/Cargo.toml --no-deps --format-version 1`, tool versions were verified with `rustc --version`, `cargo --version`, `bun --version`, `node --version`, `bazel --version`, `cargo llvm-cov --version`, `git --version`, and `rg --version`, and the pinned baseline was verified with `git submodule status packages/bitcoin-knots`. [VERIFIED: command outputs]

## Architecture Patterns

### Recommended Project Structure

```text
packages/open-bitcoin-network/src/
|-- address.rs                    # New pure address policy/book entrypoint.
|-- address/
|   |-- advertisement.rs           # Local listener candidate derivation.
|   |-- book.rs                    # Learned-address contract and deterministic store.
|   `-- response.rs                # Bounded getaddr response policy/cache.
|-- message.rs                     # Add GetAddr and legacy Addr payload support.
|-- message/tests.rs               # Add wire round-trips and sender-address gating tests.
`-- peer.rs                        # Thread getaddr/addr behavior through PeerManager actions.

packages/open-bitcoin-node/src/
|-- network.rs                     # Own node-side learned address store and action processing.
`-- status/inbound.rs              # Extend shared inbound status with address evidence.

packages/open-bitcoin-rpc/src/
|-- context/network.rs             # Provide listener evidence and local config inputs.
`-- inbound_listener.rs            # Consume peer responses without embedding address policy.

packages/open-bitcoin-cli/src/operator/
|-- status/render/inbound.rs       # Render shared address evidence.
`-- support/render/inbound.rs      # Render redacted shared evidence.

scripts/
|-- check-phase92-address-boundaries.ts
`-- check-phase92-address-boundaries.test.ts
```

This structure follows the repo preference for `foo.rs` plus `foo/` modules and keeps policy in `open-bitcoin-network` before projecting it through node/RPC/CLI adapters. [VERIFIED: standards/languages/rust.md; standards/core/architecture.md; packages/open-bitcoin-network/src/inbound.rs; .planning/phases/92-address-advertisement-and-discovery-boundaries/92-CONTEXT.md]

### Pattern 1: Typed Address Vocabulary Before Policy

**What:** Define project-owned types for address network kind, routability class, source, freshness, persistence eligibility, local advertisement decision, and learned-address decision. [VERIFIED: .planning/phases/92-address-advertisement-and-discovery-boundaries/92-CONTEXT.md; standards/core/architecture.md]

**When to use:** Use these types before status, peer actions, or support renderers consume any address evidence. [VERIFIED: .planning/phases/92-address-advertisement-and-discovery-boundaries/92-CONTEXT.md; docs/architecture/status-snapshot.md]

**Example:**

```rust
// Source: packages/bitcoin-knots/src/netaddress.h, packages/bitcoin-knots/src/netaddress.cpp,
// and .planning/phases/92-address-advertisement-and-discovery-boundaries/92-CONTEXT.md.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressNetworkKind {
    Ipv4,
    Ipv6,
    UnsupportedPrivacyNetwork,
    UnsupportedFutureNetwork,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoutabilityClass {
    PubliclyRoutable,
    NotPubliclyRoutable,
    Invalid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressSourceKind {
    LocalListener,
    InboundAddr,
}
```

The exact type names are recommendations, but the Phase 92 plans now lock the source labels to `source_local_listener` and `source_inbound_addr`. [RESOLVED: 92-01-PLAN.md]

### Pattern 2: Legacy `getaddr`/`addr` Wire Support Without `addrv2`

**What:** Add `WireNetworkMessage::GetAddr` with an empty payload and `WireNetworkMessage::Addr(AddressList)` for legacy v1 `addr` payloads. [VERIFIED: packages/open-bitcoin-network/src/message.rs; packages/bitcoin-knots/src/protocol.h]

**When to use:** Use for bounded request/response and learned-address parsing only; keep `addrv2` and `sendaddrv2` deferred or rejected with stable unsupported labels. [VERIFIED: .planning/phases/92-address-advertisement-and-discovery-boundaries/92-CONTEXT.md; packages/bitcoin-knots/test/functional/p2p_addrv2_relay.py]

**Example:**

```rust
// Source: packages/bitcoin-knots/src/protocol.h and packages/open-bitcoin-network/src/message.rs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddressAnnouncement {
    pub time_unix_seconds: u32,
    pub services: ServiceFlags,
    pub address: NetworkAddress,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AddressList {
    pub addresses: Vec<AddressAnnouncement>,
}
```

Knots legacy network `CAddress` serializes time as `uint32`, services as `uint64`, and then a `CService` address/port pair; Open Bitcoin already has compact-size helpers and the 26-byte network-address codec needed for the `CService` tail. [VERIFIED: packages/bitcoin-knots/src/protocol.h; packages/bitcoin-knots/src/netaddress.h; packages/open-bitcoin-codec/src/network.rs; packages/open-bitcoin-network/src/message.rs]

### Pattern 3: Pure Local Advertisement Candidate Selection

**What:** Convert listener endpoints into candidate decisions with `advertise_candidate` or `advertise_suppressed` labels and reasons such as `not_publicly_routable` or `privacy_network_deferred`. [VERIFIED: .planning/phases/92-address-advertisement-and-discovery-boundaries/92-CONTEXT.md]

**When to use:** Use before `LocalPeerConfig::version_message` chooses a sender address and before `getaddr` response policy includes local listener addresses. [VERIFIED: packages/open-bitcoin-network/src/message.rs; .planning/phases/92-address-advertisement-and-discovery-boundaries/92-CONTEXT.md]

**Example:**

```rust
// Source: packages/open-bitcoin-network/src/inbound.rs and packages/bitcoin-knots/src/net.cpp.
pub fn select_local_advertisement_candidates(
    inputs: &[ListenerAddressEvidence],
    services: ServiceFlags,
    peer_boundary: PeerAddressBoundary,
) -> Vec<LocalAdvertisementDecision> {
    inputs
        .iter()
        .map(|input| classify_listener_candidate(input, services, peer_boundary))
        .collect()
}
```

Knots selects local addresses by reachability and score while preventing privacy-network cross-advertisement; Phase 92 should copy the bounded privacy/routability principle but not copy interface discovery or peer-reported external-IP discovery. [VERIFIED: packages/bitcoin-knots/src/net.cpp; .planning/phases/92-address-advertisement-and-discovery-boundaries/92-CONTEXT.md]

### Pattern 4: Deterministic Inbound `getaddr` Response Policy

**What:** Keep per-peer request state and return either a capped `Addr` message or a stable suppression reason. [VERIFIED: .planning/phases/92-address-advertisement-and-discovery-boundaries/92-CONTEXT.md; packages/bitcoin-knots/src/net_processing.cpp]

**When to use:** Use only for inbound peers after handshake, and only through existing peer policy/action handling. [VERIFIED: packages/open-bitcoin-network/src/peer.rs; packages/bitcoin-knots/src/net_processing.cpp]

**Example:**

```rust
// Source: packages/bitcoin-knots/src/net_processing.cpp and 92-CONTEXT.md.
pub fn answer_getaddr(
    request: GetAddrRequestContext,
    cache: &AddressResponseCache,
    book: &LearnedAddressBook,
) -> GetAddrResponseDecision {
    if !request.is_inbound {
        return GetAddrResponseDecision::suppressed("not_inbound");
    }
    if request.already_served {
        return GetAddrResponseDecision::suppressed("already_served");
    }
    if !request.has_address_response_policy_input {
        return GetAddrResponseDecision::suppressed("permission_policy_denied");
    }
    cache.select_capped(book, request)
}
```

Knots ignores `getaddr` from non-inbound connections, responds once per connection, and uses `NetPermissionFlags::Addr` to select an uncached address response path; Phase 92 should use deterministic policy and evidence rather than Knots' randomized cache lifetime. [VERIFIED: packages/bitcoin-knots/src/net_processing.cpp; packages/bitcoin-knots/src/net.cpp; .planning/phases/92-address-advertisement-and-discovery-boundaries/92-CONTEXT.md]

### Pattern 5: Learned-Address Store as Contract, Not Full AddrMan

**What:** Store accepted learned addresses with source, first seen, last seen, services, routability, rejection/quarantine reason, and persistence eligibility. [VERIFIED: .planning/phases/92-address-advertisement-and-discovery-boundaries/92-CONTEXT.md; packages/bitcoin-knots/src/protocol.h; packages/bitcoin-knots/src/addrman.cpp]

**When to use:** Use after parsing `addr` messages and before any node/status projection. [VERIFIED: packages/open-bitcoin-network/src/peer.rs; packages/open-bitcoin-node/src/network.rs]

**Example:**

```rust
// Source: packages/bitcoin-knots/src/addrman.cpp and 92-CONTEXT.md.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LearnedAddressEntry {
    pub address: NetworkAddress,
    pub network_kind: AddressNetworkKind,
    pub source: AddressSourceKind,
    pub first_seen_unix_seconds: u64,
    pub last_seen_unix_seconds: u64,
    pub services: ServiceFlags,
    pub routability: RoutabilityClass,
    pub persistence_eligible: bool,
}
```

Knots `AddrMan::AddSingle` rejects non-routable addresses and updates time/services on existing entries, while `AddrInfo::IsTerrible` filters future, stale, and repeated-failure entries; Phase 92 should implement deterministic accept/reject evidence without randomized buckets or connection-attempt scoring. [VERIFIED: packages/bitcoin-knots/src/addrman.cpp; .planning/phases/92-address-advertisement-and-discovery-boundaries/92-CONTEXT.md]

### Pattern 6: Shared Status First, Renderers Second

**What:** Extend `InboundPeerServingStatus` with bounded fields for local candidates, suppressed advertisements, getaddr decisions, learned-address counts, and latest learned-address decision. [VERIFIED: packages/open-bitcoin-node/src/status/inbound.rs; .planning/phases/92-address-advertisement-and-discovery-boundaries/92-CONTEXT.md]

**When to use:** Use before changing CLI/status/support rendering so renderers remain projections of shared evidence. [VERIFIED: docs/architecture/status-snapshot.md; docs/architecture/operator-observability.md]

**Example status labels:**

```text
advertise_candidate
advertise_suppressed
not_publicly_routable
privacy_network_deferred
getaddr_served
getaddr_suppressed
learned_accepted
learned_rejected
source_local_listener
source_inbound_addr
full_relay_deferred
```

These labels are drawn from the Phase 92 context's specific ideas and should stay low-cardinality in status, logs, metrics, and support bundles. [VERIFIED: .planning/phases/92-address-advertisement-and-discovery-boundaries/92-CONTEXT.md; AGENTS.bright-builds.md]

### Anti-Patterns to Avoid

- **Interface discovery as advertisement input:** Do not scan arbitrary local interfaces or infer public addresses from outbound peers because D-01 limits inputs to configured listeners and Phase 90 listener evidence. [VERIFIED: .planning/phases/92-address-advertisement-and-discovery-boundaries/92-CONTEXT.md; packages/bitcoin-knots/src/net.cpp]
- **Version sender address eagerness:** Do not change `LocalPeerConfig::version_message` to send routable sender addresses unless the same candidate policy passes. [VERIFIED: packages/open-bitcoin-network/src/message.rs; .planning/phases/92-address-advertisement-and-discovery-boundaries/92-CONTEXT.md]
- **Full relay by accident:** Do not implement `MaybeSendAddr`, `PushAddress` fanout queues, rolling bloom known-address filters, or unsolicited relay scheduling. [VERIFIED: packages/bitcoin-knots/src/net_processing.cpp; .planning/phases/92-address-advertisement-and-discovery-boundaries/92-CONTEXT.md]
- **Full AddrMan by accident:** Do not implement randomized buckets, `peers.dat`, anchors, DNS seed rotation, or address-fetch outbound behavior in Phase 92. [VERIFIED: packages/bitcoin-knots/src/addrman.cpp; packages/bitcoin-knots/src/addrdb.cpp; packages/bitcoin-knots/test/functional/p2p_addrfetch.py; .planning/phases/92-address-advertisement-and-discovery-boundaries/92-CONTEXT.md]
- **Renderer-local address summaries:** Do not compute address classifications in CLI/support renderers because D-14 requires network/domain first and shared status projection. [VERIFIED: .planning/phases/92-address-advertisement-and-discovery-boundaries/92-CONTEXT.md; docs/architecture/status-snapshot.md]

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Local address discovery | Interface scanner, NAT mapper, UPnP/NAT-PMP, DNS/external-IP probe | Pure selector over `inbound.listen_addresses` and listener evidence | The phase locks advertisement inputs to configured listeners and runtime-bound evidence. [VERIFIED: .planning/phases/92-address-advertisement-and-discovery-boundaries/92-CONTEXT.md] |
| Full address relay | Bloom known-address filters, trickle scheduler, unsolicited fanout, peer relay selection | Bounded inbound `getaddr` response policy only | Knots' relay machinery is larger than Phase 92 and includes behavior D-06 forbids. [VERIFIED: packages/bitcoin-knots/src/net_processing.cpp; .planning/phases/92-address-advertisement-and-discovery-boundaries/92-CONTEXT.md] |
| Full AddrMan persistence | Randomized bucketed `AddrMan`, `peers.dat`, anchors | Typed in-memory or snapshot-backed learned-address contract | D-13 allows deterministic persistence evidence but forbids implying full Knots `addrman.dat` or production discovery parity. [VERIFIED: .planning/phases/92-address-advertisement-and-discovery-boundaries/92-CONTEXT.md; packages/bitcoin-knots/src/addrman.cpp; packages/bitcoin-knots/src/addrdb.cpp] |
| Privacy-network relay | Onion/I2P/CJDNS v1 serialization tricks or `addrv2` relay | Unsupported/deferred typed network placeholders | Knots treats Tor/I2P as privacy networks and `addrv2` covers longer addresses, while D-04 defers unsupported privacy networks unless no-leak tests exist. [VERIFIED: packages/bitcoin-knots/src/netaddress.h; packages/bitcoin-knots/src/protocol.h; .planning/phases/92-address-advertisement-and-discovery-boundaries/92-CONTEXT.md] |
| High-cardinality observability | Raw peer tables, raw endpoint lists, raw permission class names, raw config strings | Shared bounded status labels and redacted support evidence | Phase 91 already requires raw class/config strings to stay out of status/support, and Phase 92 inherits that for address response policy. [VERIFIED: .planning/phases/91-peer-permissions-and-connection-classes/91-CONTEXT.md; .planning/phases/92-address-advertisement-and-discovery-boundaries/92-CONTEXT.md; docs/architecture/status-snapshot.md] |
| Public discovery tests | Public peers, DNS seeds, public listener exposure, multi-day timing | Pure unit tests, synthetic messages, loopback fixtures, Bun docs checker | D-19 requires deterministic, local, public-network-free, short-running verification. [VERIFIED: .planning/phases/92-address-advertisement-and-discovery-boundaries/92-CONTEXT.md] |

**Key insight:** The hard part is not encoding `getaddr`; the hard part is preserving a narrow evidence contract so `addr` support cannot be mistaken for public peer discovery or full address relay. [VERIFIED: .planning/phases/92-address-advertisement-and-discovery-boundaries/92-CONTEXT.md; packages/bitcoin-knots/src/net_processing.cpp; docs/parity/catalog/p2p.md]

## Common Pitfalls

### Pitfall 1: Treating `NetworkAddress` as the Whole Address Contract

**What goes wrong:** Code stores only services, 16 bytes, and port, then loses source, freshness, routability, and persistence evidence. [VERIFIED: packages/open-bitcoin-primitives/src/network.rs; .planning/phases/92-address-advertisement-and-discovery-boundaries/92-CONTEXT.md]

**Why it happens:** Open Bitcoin's existing `NetworkAddress` matches the 26-byte address shape used by `version`, while legacy `addr` records add a timestamp and Phase 92 requires source/freshness evidence beyond the wire fields. [VERIFIED: packages/open-bitcoin-primitives/src/network.rs; packages/bitcoin-knots/src/protocol.h; .planning/phases/92-address-advertisement-and-discovery-boundaries/92-CONTEXT.md]

**How to avoid:** Add a separate address-announcement and learned-address entry type, then convert down to `NetworkAddress` only at the wire boundary. [VERIFIED: standards/core/architecture.md; packages/open-bitcoin-codec/src/network.rs]

**Warning signs:** Status or support output cannot explain whether an address came from a listener candidate or inbound `addr` message. [VERIFIED: .planning/phases/92-address-advertisement-and-discovery-boundaries/92-CONTEXT.md]

### Pitfall 2: Leaking Local or Privacy-Network Addresses

**What goes wrong:** Loopback/private/documentation/unspecified addresses become advertised to peers or included in `getaddr` responses. [VERIFIED: .planning/phases/92-address-advertisement-and-discovery-boundaries/92-CONTEXT.md]

**Why it happens:** Socket bindability is not the same as public routability, and Knots separately distinguishes validity, routability, privacy networks, and reachability. [VERIFIED: packages/open-bitcoin-network/src/inbound.rs; packages/bitcoin-knots/src/netaddress.h; packages/bitcoin-knots/src/netaddress.cpp]

**How to avoid:** Keep a pure classification step with explicit suppression reasons before any version sender address or `addr` response selection. [VERIFIED: .planning/phases/92-address-advertisement-and-discovery-boundaries/92-CONTEXT.md]

**Warning signs:** A `127.0.0.1`, `0.0.0.0`, private IPv4, documentation IPv4, documentation IPv6, multicast, or privacy-network placeholder appears as an advertised public candidate. [VERIFIED: .planning/phases/92-address-advertisement-and-discovery-boundaries/92-CONTEXT.md; packages/bitcoin-knots/src/netaddress.cpp]

### Pitfall 3: Accidentally Implementing Relay State

**What goes wrong:** `getaddr` handling initializes peer relay queues, known-address filters, periodic self-announcements, or fanout behavior. [VERIFIED: packages/bitcoin-knots/src/net_processing.cpp]

**Why it happens:** Knots handles `getaddr`, `addr`, address relay setup, local self-announcement, and fanout in adjacent peer-manager code. [VERIFIED: packages/bitcoin-knots/src/net_processing.cpp]

**How to avoid:** Keep Phase 92 response state as served/suppressed evidence and send only direct `Addr` responses selected by a deterministic policy. [VERIFIED: .planning/phases/92-address-advertisement-and-discovery-boundaries/92-CONTEXT.md]

**Warning signs:** New Open Bitcoin code adds periodic address timers, bloom filters, randomized replacement in send queues, or unsolicited `addr` sends. [VERIFIED: packages/bitcoin-knots/src/net_processing.cpp; .planning/phases/92-address-advertisement-and-discovery-boundaries/92-CONTEXT.md]

### Pitfall 4: Copying Knots' Random Cache Directly

**What goes wrong:** Tests become wall-clock/random dependent and default verification becomes flaky. [VERIFIED: packages/bitcoin-knots/src/net.cpp; .planning/phases/92-address-advertisement-and-discovery-boundaries/92-CONTEXT.md]

**Why it happens:** Knots uses a per-network response cache with a 21-hour plus random up-to-6-hour expiration for privacy, while Phase 92 requires deterministic tests and inspectable evidence. [VERIFIED: packages/bitcoin-knots/src/net.cpp; packages/bitcoin-knots/test/functional/p2p_getaddr_caching.py; .planning/phases/92-address-advertisement-and-discovery-boundaries/92-CONTEXT.md]

**How to avoid:** Use a deterministic cache key and explicit logical request-window or served-once policy in pure tests. [VERIFIED: .planning/phases/92-address-advertisement-and-discovery-boundaries/92-CONTEXT.md]

**Warning signs:** Unit tests depend on real elapsed hours, random expiration, or public address scraping. [VERIFIED: .planning/phases/92-address-advertisement-and-discovery-boundaries/92-CONTEXT.md]

### Pitfall 5: Letting `addr` Permission Become a Relay Claim

**What goes wrong:** A Phase 91 permission class with `addr` is interpreted as permission to relay addresses broadly. [VERIFIED: .planning/phases/91-peer-permissions-and-connection-classes/91-CONTEXT.md; .planning/phases/92-address-advertisement-and-discovery-boundaries/92-CONTEXT.md]

**Why it happens:** Knots uses `NetPermissionFlags::Addr` in both address processing and getaddr response paths, while Open Bitcoin Phase 91 exposes it only as `AddressResponsePolicyInput`. [VERIFIED: packages/bitcoin-knots/src/net_processing.cpp; packages/open-bitcoin-network/src/inbound/permissions.rs]

**How to avoid:** Treat the Phase 91 `addr` effect as eligibility for bounded response policy only and keep inactive/full-relay labels in docs/checkers. [VERIFIED: .planning/phases/91-peer-permissions-and-connection-classes/91-CONTEXT.md; scripts/check-phase91-peer-permissions.ts]

**Warning signs:** Status says "address relay support" instead of "address-response policy input" or "bounded getaddr response". [VERIFIED: docs/architecture/status-snapshot.md; scripts/check-phase91-peer-permissions.ts]

## Code Examples

Verified patterns from local source and pinned Knots anchors follow; names are illustrative unless already present in the repo. [VERIFIED: local code inspection; pinned Knots source inspection]

### Legacy `addr` Payload Shape

```rust
// Source: packages/bitcoin-knots/src/protocol.h and packages/open-bitcoin-codec/src/network.rs.
fn encode_addr_payload(list: &AddressList) -> Result<Vec<u8>, NetworkError> {
    let mut out = Vec::new();
    write_compact_size(&mut out, list.addresses.len() as u64)?;
    for address in &list.addresses {
        out.extend_from_slice(&address.time_unix_seconds.to_le_bytes());
        out.extend_from_slice(&address.services.bits().to_le_bytes());
        out.extend_from_slice(&encode_network_address(&address.address));
    }
    Ok(out)
}
```

### Version Sender Gating

```rust
// Source: packages/open-bitcoin-network/src/message.rs and 92-CONTEXT.md D-05.
pub fn version_message_with_sender_policy(
    config: &LocalPeerConfig,
    timestamp: i64,
    start_height: i32,
    maybe_sender: Option<NetworkAddress>,
) -> VersionMessage {
    let mut message = config.version_message(timestamp, start_height);
    message.sender = maybe_sender.unwrap_or_else(zero_address);
    message
}
```

### Learned Address Acceptance

```rust
// Source: packages/bitcoin-knots/src/addrman.cpp and 92-CONTEXT.md D-11 through D-13.
pub fn learn_address(
    entry: AddressAnnouncement,
    source: AddressSourceEvidence,
    now_unix_seconds: u64,
) -> LearnedAddressDecision {
    let classification = classify_network_address(&entry.address);
    if classification.routability != RoutabilityClass::PubliclyRoutable {
        return LearnedAddressDecision::rejected("not_publicly_routable");
    }
    if entry.address.port == 0 {
        return LearnedAddressDecision::rejected("invalid_port");
    }
    if is_stale_or_future(entry.time_unix_seconds, now_unix_seconds) {
        return LearnedAddressDecision::rejected("stale_or_future");
    }
    LearnedAddressDecision::accepted(entry, source, classification)
}
```

## State of the Art

| Old/Broader Approach | Current Phase 92 Approach | Anchor | Impact |
|----------------------|---------------------------|--------|--------|
| Knots advertises from a global local-host map that can be populated by discovery, binds, mappings, and peer-reported local address evidence. | Open Bitcoin should derive candidates only from configured listener endpoints and Phase 90 runtime listener evidence. | [VERIFIED: packages/bitcoin-knots/src/net.cpp; .planning/phases/92-address-advertisement-and-discovery-boundaries/92-CONTEXT.md] | Planner should avoid interface discovery, UPnP/NAT-PMP, DNS, external IP probes, and baseline flag compatibility. |
| Knots has full address relay setup with rolling bloom filters, fanout queues, periodic local self-announcement, and ADDR/ADDRV2 send scheduling. | Open Bitcoin should implement only bounded direct `getaddr`/`addr` request-response and learned-address intake. | [VERIFIED: packages/bitcoin-knots/src/net_processing.cpp; .planning/phases/92-address-advertisement-and-discovery-boundaries/92-CONTEXT.md] | Planner should not schedule unsolicited relay or fanout work. |
| Knots `getaddr` responses are capped at 1000 addresses and 23 percent of addrman and use randomized per-network cache expiration. | Open Bitcoin should choose small deterministic caps and typed inspectable cache/window evidence. | [VERIFIED: packages/bitcoin-knots/src/net_processing.cpp; packages/bitcoin-knots/src/net.cpp; .planning/phases/92-address-advertisement-and-discovery-boundaries/92-CONTEXT.md] | Planner should test count/age/source/cache/request-frequency rules with synthetic data. |
| Knots `AddrMan` is randomized, bucketed, persisted, and integrated with connection attempts and quality scoring. | Open Bitcoin should create a deterministic learned-address contract with persistence eligibility evidence. | [VERIFIED: packages/bitcoin-knots/src/addrman.cpp; packages/bitcoin-knots/src/addrdb.cpp; .planning/phases/92-address-advertisement-and-discovery-boundaries/92-CONTEXT.md] | Planner should separate address data contract from future discovery and persistence parity. |
| Knots supports `addrv2`/`sendaddrv2` for longer address networks. | Phase 92 should defer `addrv2` relay unless implementing a no-leak unsupported placeholder. | [VERIFIED: packages/bitcoin-knots/src/protocol.h; packages/bitcoin-knots/test/functional/p2p_addrv2_relay.py; .planning/phases/92-address-advertisement-and-discovery-boundaries/92-CONTEXT.md] | Planner should keep privacy-network boundaries explicit and test unsupported handling. |

**Deprecated/outdated for this phase:**

- Treating `addr` message support as full address relay support is out of scope for Phase 92. [VERIFIED: .planning/phases/92-address-advertisement-and-discovery-boundaries/92-CONTEXT.md; docs/parity/catalog/p2p.md]
- Treating public inbound listener defaults as implied by local address advertisement is out of scope for Phase 92. [VERIFIED: .planning/phases/92-address-advertisement-and-discovery-boundaries/92-CONTEXT.md; docs/parity/checklist.md]
- Treating DNS seeds, public peer crawling, or production peer-discovery parity as part of learned-address storage is out of scope for Phase 92. [VERIFIED: .planning/phases/92-address-advertisement-and-discovery-boundaries/92-CONTEXT.md; docs/parity/catalog/p2p.md]

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | Exact Rust type names in examples such as `AddressNetworkKind`, `AddressAnnouncement`, and `LearnedAddressEntry` are recommendations, but Plan 92-01 locks the shared label/reason contract. [RESOLVED: 92-01-PLAN.md] | Architecture Patterns, Code Examples | Executor may choose idiomatic Rust variant names only when `as_str()` labels remain identical. |
| A2 | Phase 92 plans choose a fixed `PHASE92_GETADDR_RESPONSE_LIMIT` of 8. [RESOLVED: 92-03-PLAN.md] | Standard Stack, Architecture Patterns | Executor must test cap, cap+1, empty, stale, duplicate, and permission-denied response paths. |
| A3 | Phase 92 plans use an in-memory learned-address store with explicit `persistence_eligible` evidence. [RESOLVED: 92-03-PLAN.md] | Architecture Patterns, Open Questions | Snapshot-backed or full AddrMan persistence remains outside Phase 92. |

## Open Questions

1. **Should the first learned-address store be in-memory or snapshot-backed?** [RESOLVED]
   - What we know: D-13 allows either in-memory or snapshot-backed persistence evidence. [VERIFIED: .planning/phases/92-address-advertisement-and-discovery-boundaries/92-CONTEXT.md]
   - Plan decision: Use an in-memory learned-address book in Plan 92-03 with explicit `persistence_eligible` evidence and no snapshot file format. [RESOLVED: 92-03-PLAN.md]
2. **What exact response cap should Phase 92 use?** [RESOLVED]
   - What we know: Knots uses 1000 addresses and 23 percent of addrman for full-network behavior, while D-08 requires a small deterministic cap. [VERIFIED: packages/bitcoin-knots/src/net_processing.cpp; .planning/phases/92-address-advertisement-and-discovery-boundaries/92-CONTEXT.md]
   - Plan decision: Use `PHASE92_GETADDR_RESPONSE_LIMIT: usize = 8` and test cap, cap+1, empty, stale, duplicate, and already-served paths. [RESOLVED: 92-03-PLAN.md]
3. **Should Phase 92 expose address metrics or status-only evidence?** [RESOLVED]
   - What we know: Existing inbound metrics are low-cardinality, and D-15 requires status/support evidence. [VERIFIED: packages/open-bitcoin-node/src/metrics.rs; .planning/phases/92-address-advertisement-and-discovery-boundaries/92-CONTEXT.md]
   - Plan decision: Keep Phase 92 to shared status/support evidence and do not add new metrics; Plan 92-05 explicitly keeps metrics unchanged. [RESOLVED: 92-05-PLAN.md]
4. **Should unsupported `addrv2` be unknown-command behavior or an explicit deferred label?** [RESOLVED]
   - What we know: `WireNetworkMessage` currently rejects unknown commands and D-04 allows typed placeholders only if tests prove no leak/relay. [VERIFIED: packages/open-bitcoin-network/src/message.rs; .planning/phases/92-address-advertisement-and-discovery-boundaries/92-CONTEXT.md]
   - Plan decision: Keep `addrv2` and `sendaddrv2` as unknown/deferred wire surfaces in Plan 92-02; privacy-network labels remain no-leak placeholders only. [RESOLVED: 92-02-PLAN.md]

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|-------------|-----------|---------|----------|
| Rust toolchain | Cargo build/test/check of first-party crates | yes | `rustc 1.94.1`, `cargo 1.94.1` | none needed [VERIFIED: rustc --version; cargo --version] |
| Bun | Phase 92 TypeScript checker/test | yes | `1.3.9` | none needed [VERIFIED: bun --version; .bun-version] |
| Node.js | GSD tooling and some script ecosystem support | yes | `v24.13.0` | none needed [VERIFIED: node --version] |
| Bazel | Repo smoke build and UAT command forms | yes | `bazel 8.6.0` | none needed [VERIFIED: bazel --version; MODULE.bazel] |
| `cargo llvm-cov` | Repo verification coverage step when invoked | yes | `cargo-llvm-cov 0.8.5` | none needed [VERIFIED: cargo llvm-cov --version] |
| Git | Submodule and planning commit operations | yes | `git version 2.53.0` | none needed [VERIFIED: git --version] |
| ripgrep | Codebase/source audits | yes | `ripgrep 15.1.0` | `grep` if unavailable, but not needed [VERIFIED: rg --version] |
| Pinned Knots submodule | Parity anchors | yes | `a9aee730466ac67d35a3c03ee24676be5e045878` / `v29.3.knots20260210` | `git submodule update --init --recursive` [VERIFIED: git submodule status packages/bitcoin-knots; AGENTS.md] |

**Missing dependencies with no fallback:** None found for research and planning. [VERIFIED: environment audit commands]

**Missing dependencies with fallback:** None found for research and planning. [VERIFIED: environment audit commands]

## Security Domain

Security enforcement is enabled by default because `.planning/config.json` does not set `security_enforcement` to `false`. [VERIFIED: .planning/config.json]

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|------------------|
| V2 Authentication | no | Phase 92 does not add authentication flows; keep existing RPC/operator authentication boundaries unchanged. [VERIFIED: .planning/phases/92-address-advertisement-and-discovery-boundaries/92-CONTEXT.md] |
| V3 Session Management | no | Phase 92 does not add sessions or cookies; no session-control change is required. [VERIFIED: .planning/phases/92-address-advertisement-and-discovery-boundaries/92-CONTEXT.md] |
| V4 Access Control | yes | Use Phase 91 `AddressResponsePolicyInput` as a bounded permission eligibility input and keep raw permission config out of status/support. [VERIFIED: packages/open-bitcoin-network/src/inbound/permissions.rs; .planning/phases/92-address-advertisement-and-discovery-boundaries/92-CONTEXT.md] |
| V5 Input Validation | yes | Parse `getaddr`/`addr` messages through typed wire parsers and reject/quarantine invalid ports, unsupported networks, stale/future timestamps, unroutable entries, self/local leakage, and over-cap batches. [VERIFIED: packages/open-bitcoin-network/src/message.rs; packages/bitcoin-knots/src/protocol.h; .planning/phases/92-address-advertisement-and-discovery-boundaries/92-CONTEXT.md] |
| V6 Cryptography | no new crypto | Do not add cryptographic primitives; continue using existing message checksum and hash utilities where already present. [VERIFIED: packages/open-bitcoin-network/src/message.rs; open_bitcoin_consensus::crypto::double_sha256 usage] |

### Known Threat Patterns for Phase 92

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Address scraping or fingerprinting via repeated `getaddr` | Information Disclosure | Inbound-only policy, served-once/window suppression, deterministic cache evidence, and capped response size. [VERIFIED: packages/bitcoin-knots/src/net_processing.cpp; packages/bitcoin-knots/src/net.cpp; .planning/phases/92-address-advertisement-and-discovery-boundaries/92-CONTEXT.md] |
| Local/privacy address leakage | Information Disclosure | Typed routability/privacy classification before version sender address and `addr` response inclusion. [VERIFIED: packages/bitcoin-knots/src/net.cpp; packages/bitcoin-knots/src/netaddress.cpp; .planning/phases/92-address-advertisement-and-discovery-boundaries/92-CONTEXT.md] |
| Address-table poisoning with stale, future, unroutable, or duplicate entries | Tampering | Learned-address parser/policy rejects or quarantines invalid entries with stable reasons. [VERIFIED: packages/bitcoin-knots/src/addrman.cpp; .planning/phases/92-address-advertisement-and-discovery-boundaries/92-CONTEXT.md] |
| Resource exhaustion through large `addr` batches or repeated `getaddr` | Denial of Service | Explicit batch caps, response caps, and repeated-request suppression in pure policy. [VERIFIED: packages/bitcoin-knots/src/net_processing.cpp; .planning/phases/92-address-advertisement-and-discovery-boundaries/92-CONTEXT.md] |
| Overclaim drift in docs/release evidence | Repudiation | Phase 92 checker should enforce required evidence labels and forbidden full-relay/discovery/public-readiness claims. [VERIFIED: scripts/check-phase91-peer-permissions.ts; docs/parity/catalog/p2p.md; .planning/phases/92-address-advertisement-and-discovery-boundaries/92-CONTEXT.md] |

## Common Verification Shape

The planner should map tasks to the repo-native verification contract, not to ad hoc public-network checks. [VERIFIED: AGENTS.md; .planning/phases/92-address-advertisement-and-discovery-boundaries/92-CONTEXT.md]

| Verification Need | Recommended Command |
|-------------------|---------------------|
| Targeted Rust unit tests for network address policy/message behavior | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network address --no-fail-fast` [VERIFIED: packages/Cargo.toml; packages/open-bitcoin-network/src/message/tests.rs] |
| Targeted status projection tests | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node inbound --no-fail-fast` [VERIFIED: packages/Cargo.toml; packages/open-bitcoin-node/src/status/inbound/tests.rs] |
| Phase 92 checker unit tests | `bun test scripts/check-phase92-address-boundaries.test.ts` [VERIFIED: scripts/check-phase91-peer-permissions.test.ts; AGENTS.md] |
| Phase 92 checker | `bun run scripts/check-phase92-address-boundaries.ts` [VERIFIED: scripts/check-phase91-peer-permissions.ts; AGENTS.md] |
| Full repo-native verification | `bash scripts/verify.sh` [VERIFIED: AGENTS.md; scripts/verify.sh] |

The exact targeted test names may change after implementation creates concrete modules and tests. [PLANNER NOTE]

## Sources

### Primary (HIGH confidence)

- `.planning/phases/92-address-advertisement-and-discovery-boundaries/92-CONTEXT.md` - locked decisions, discretion, deferred scope, canonical refs, code-context notes. [VERIFIED: file read]
- `.planning/REQUIREMENTS.md` - ADDR-01 through ADDR-04 and ADDR-FUTURE-01. [VERIFIED: file read]
- `.planning/ROADMAP.md` - Phase 92 goal and success criteria. [VERIFIED: file read]
- `.planning/STATE.md` - v1.9 milestone state and carry-forward workflow notes. [VERIFIED: file read]
- `AGENTS.md`, `AGENTS.bright-builds.md`, `standards-overrides.md`, `standards/core/*.md`, and `standards/languages/*.md` - repo constraints, verification, testing, architecture, and language rules. [VERIFIED: file read]
- `packages/open-bitcoin-network/src/message.rs`, `peer.rs`, `inbound.rs`, `inbound/permissions.rs`, and related tests - current wire, peer, listener, and permission seams. [VERIFIED: local source inspection]
- `packages/open-bitcoin-node/src/network.rs`, `network/inbound.rs`, `status/inbound.rs`, and `metrics.rs` - node projection, status, and metric seams. [VERIFIED: local source inspection]
- `packages/open-bitcoin-rpc/src/context/network.rs`, `inbound_listener.rs`, and config loader files - runtime listener and config seams. [VERIFIED: local source inspection]
- `packages/open-bitcoin-cli/src/operator/status/render/inbound.rs` and `support/render/inbound.rs` - renderer seams. [VERIFIED: local source inspection]
- `scripts/check-phase90-inbound-listener-admission.ts` and `scripts/check-phase91-peer-permissions.ts` - deterministic checker patterns. [VERIFIED: local source inspection]
- `docs/parity/catalog/p2p.md`, `docs/parity/checklist.md`, `docs/parity/index.json`, `docs/parity/source-breadcrumbs.json`, `docs/architecture/status-snapshot.md`, `docs/architecture/operator-observability.md`, and `docs/operator/runtime-guide.md` - docs/evidence/checker targets. [VERIFIED: local source inspection]
- `packages/bitcoin-knots/src/protocol.h`, `netaddress.h`, `netaddress.cpp`, `net.cpp`, `net_processing.cpp`, `addrman.h`, `addrman.cpp`, `addrdb.h`, and `addrdb.cpp` - pinned Knots anchors. [VERIFIED: pinned submodule source inspection]
- `packages/bitcoin-knots/test/functional/p2p_getaddr_caching.py`, `p2p_addrfetch.py`, `p2p_addr_relay.py`, `p2p_addrv2_relay.py`, and `feature_addrman.py` - pinned Knots behavior and out-of-scope guardrail anchors. [VERIFIED: pinned submodule source inspection]

### Secondary (MEDIUM confidence)

- None used; research was constrained to repo files and the pinned Knots submodule. [VERIFIED: tool usage]

### Tertiary (LOW confidence)

- None used. [VERIFIED: tool usage]

## Metadata

**Confidence breakdown:**

- Standard stack: HIGH - All recommended stack pieces are existing repo crates/tools or pinned submodule sources verified locally. [VERIFIED: packages/Cargo.toml; rust-toolchain.toml; .bun-version; MODULE.bazel; git submodule status packages/bitcoin-knots]
- Architecture: HIGH - The recommended module boundaries follow existing Phase 90/91 code and repo standards. [VERIFIED: packages/open-bitcoin-network/src/inbound.rs; packages/open-bitcoin-network/src/inbound/permissions.rs; standards/core/architecture.md; standards/languages/rust.md]
- Pitfalls: HIGH - Pitfalls are anchored to explicit user decisions and pinned Knots behavior that is broader than Phase 92. [VERIFIED: .planning/phases/92-address-advertisement-and-discovery-boundaries/92-CONTEXT.md; packages/bitcoin-knots/src/net_processing.cpp; packages/bitcoin-knots/src/addrman.cpp]
- Exact type names: MEDIUM - Example names remain planner-facing recommendations, while the plans now resolve storage, response cap, metrics, and `addrv2` decisions. [RESOLVED: 92-02-PLAN.md; 92-03-PLAN.md; 92-05-PLAN.md]

**Research date:** 2026-06-26 [VERIFIED: developer current date]
**Valid until:** 2026-07-26 for repo-local planning unless Knots baseline, Phase 92 context, or repo standards change first. [ESTIMATE]
