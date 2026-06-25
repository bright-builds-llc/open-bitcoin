# Phase 91: Peer Permissions and Connection Classes - Research

**Researched:** 2026-06-25
**Domain:** Bitcoin peer permission parsing, connection classes, bounded inbound policy, and operator evidence
**Confidence:** HIGH

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

## Implementation Decisions

### Permission Vocabulary And Parsing

- **D-01:** Use Knots permission names as the vocabulary anchor: `bloomfilter`, `blockfilters`, `noban`, `forcerelay`, `relay`, `mempool`, `download`, `addr`, `forceinbound`, `in`, `out`, and `all`.
- **D-02:** Open Bitcoin-owned config remains the entry point. Add permissions under the existing JSONC-owned `inbound` surface and Open Bitcoin-prefixed daemon CLI overrides; do not silently accept Knots `-whitelist` or `-whitebind` as full baseline compatibility.
- **D-03:** Parsing must be explicit and stable. Unsupported tokens, direction-only inputs, invalid connection direction combinations, and malformed class definitions return deterministic validation errors that name the offending field and token.
- **D-04:** Permission bundles must be typed, not plain strings. Represent parsed flags, connection direction boundaries, and effective class names with domain types so illegal states are hard to construct after parsing.
- **D-05:** `all` should expand only to an auditable parsed set. Deferred or inactive permissions remain visibly inactive; `all` must not smuggle in relay, mempool, force-relay, compact-filter, or broad block-filter behavior.

### Connection Classes And Admission Effects

- **D-06:** Introduce explicit connection classes such as ordinary inbound, permissioned inbound, protected inbound, outbound, and manual/operator configured peers as needed by the Phase 91 plan. Class names should be stable machine labels suitable for status and support evidence.
- **D-07:** `forceinbound` and `noban`-style protections may influence admission protection or eviction-candidate inputs, but only through bounded pure decisions. They should not mutate peer state hidden inside the runtime accept loop.
- **D-08:** Permissioned peers can consume the reserved admission path created in Phase 90. The planner may replace or extend `InboundAdmissionSlotClass::Reserved`, but must preserve ordinary peers being unable to consume protected capacity.
- **D-09:** Outbound sync safety remains non-negotiable. Permissioned inbound peers must not reduce `target_outbound_peers`, starve outbound sync, or count as outbound compatibility progress.

### Bounded Permission Effects

- **D-10:** v1.9 active permission effects are bounded to admission protection, eviction-policy inputs, address-response policy inputs, download-serving policy inputs, and diagnostics.
- **D-11:** `download` may influence block/header serving or max-upload-style decisions only as a policy input. It must not create a new unattended block-serving claim unless a plan adds explicit bounded tests and documentation.
- **D-12:** `addr` may influence the later Phase 92 bounded `getaddr` response policy, but Phase 91 should expose only typed policy inputs and diagnostics unless it is needed to support a Phase 91 test seam.
- **D-13:** `noban` may mark a peer as protected from eviction/ban/misbehavior responses, but Phase 93 owns actual ban, discourage, disconnect, and misbehavior semantics.
- **D-14:** `relay`, `forcerelay`, `mempool`, `bloomfilter`, `blockfilters`, and compact-filter-like permissions must be rejected, deferred, or parsed as inactive for now. They cannot initialize tx relay state, mempool query handling, force-relay rebroadcasts, compact block relay, BIP37 filtering, or compact-filter serving.

### Operator Evidence And Redaction

- **D-15:** Permission status must project through shared status/support contracts rather than renderer-local summaries. Extend `OpenBitcoinStatusSnapshot.peers.inbound` or a closely owned child contract with low-cardinality labels for permission class, active bounded effects, inactive/deferred effects, and latest permission decision.
- **D-16:** Support bundles may include bounded permission evidence and reasons, but must not leak secrets, raw config values beyond safe labels, raw peer tables, unbounded endpoints, or credential material.
- **D-17:** Structured logs and metrics should stay low-cardinality. Numeric counters may cover permissioned admits, protected peers, inactive relay-like permissions, and permission validation failures; labels must not include peer ids, raw endpoints, user labels, or raw permission config strings.

### Verification And UAT

- **D-18:** Default verification stays deterministic, local, and public-network-free. Use pure parser/policy tests, synthetic peer records, and existing Phase 90 inbound fixtures rather than public peers.
- **D-19:** Tests should prove both positive bounded effects and negative relay safeguards. At minimum, include cases for stable labels, explicit parse errors, `all` expansion boundaries, reserved/protected admission behavior, inactive relay/mempool/force-relay/compact-filter effects, status/support redaction, and no outbound sync starvation.
- **D-20:** Operator UAT docs must use repo-local Cargo and Bazel forms when commands are needed, matching the repo lesson and `AGENTS.md` guidance.

### the agent's Discretion

The planner may choose exact type names and module splits. Prefer pure parsing and policy modules in `open-bitcoin-network`, thin projection/wiring in `open-bitcoin-node`, config parsing in `open-bitcoin-rpc`, and renderer-only formatting in CLI/support modules. Keep the first implementation narrow enough to satisfy PERM-01 through PERM-04 without pulling Phase 92 or Phase 93 behavior forward.

### Deferred Ideas (OUT OF SCOPE)

- Phase 92 owns local address advertisement, bounded `getaddr` response behavior, and address-management contracts.
- Phase 93 owns actual eviction, disconnect, discourage, ban, expiry, unban, and misbehavior behavior.
- Phase 94 owns broader inbound DoS/resource governance beyond Phase 90 caps and Phase 91 permission inputs.
- Phase 95 owns v1.9 release-boundary docs and no-claim evidence across inbound serving.
- Future milestones own transaction relay, compact block relay, mempool propagation, BIP37/compact-filter serving, full address relay, public inbound defaults, public-network CI, and production full-node readiness.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| PERM-01 | Operators can define permissioned peer classes from config using Knots-aligned permission concepts, connection direction boundaries, and explicit validation errors. [VERIFIED: .planning/REQUIREMENTS.md] | Use the existing Open Bitcoin `inbound` JSONC/CLI config boundary, add typed parser/domain types in `open-bitcoin-network`, and return deterministic `ConfigError` messages from `open-bitcoin-rpc` that name field and token. [VERIFIED: packages/open-bitcoin-rpc/src/config/open_bitcoin.rs; packages/open-bitcoin-rpc/src/config/loader/inbound.rs; packages/open-bitcoin-rpc/src/config/loader/open_bitcoin_runtime.rs; packages/bitcoin-knots/src/net_permissions.cpp] |
| PERM-02 | Permission rules affect only v1.9 in-scope privileges: admission protection, eviction immunity, address response policy, download serving policy, and diagnostics. [VERIFIED: .planning/REQUIREMENTS.md] | Model each permission as active bounded effects plus inactive/deferred effects; wire active effects only into Phase 90 admission slots, eviction/address/download policy input labels, and status/support diagnostics. [VERIFIED: 91-CONTEXT.md; packages/open-bitcoin-network/src/inbound.rs; packages/bitcoin-knots/src/net.cpp; packages/bitcoin-knots/src/net_processing.cpp] |
| PERM-03 | Relay, mempool, force-relay, and compact-block-style permissions can be rejected, deferred, or parsed as inactive without enabling transaction relay, compact block relay, or mempool propagation. [VERIFIED: .planning/REQUIREMENTS.md] | Treat `relay`, `forcerelay`, `mempool`, `bloomfilter`, and `blockfilters` as inactive/deferred labels in v1.9; add negative tests around existing `WtxidRelay`, `Inv`, `Tx`, and `GetData` peer paths so permissions cannot initialize relay state. [VERIFIED: 91-CONTEXT.md; packages/open-bitcoin-network/src/peer.rs; packages/bitcoin-knots/src/net_permissions.h; packages/bitcoin-knots/src/net_processing.cpp] |
| PERM-04 | Permission effects are visible in status/support evidence without leaking secrets or hiding why a peer was admitted, protected, disconnected, discouraged, or banned. [VERIFIED: .planning/REQUIREMENTS.md] | Extend `InboundPeerServingStatus` and `InboundAdmissionEvent` or a child status contract with low-cardinality permission class/effect labels; keep support bundle redaction patterns and unavailable-reason behavior. [VERIFIED: packages/open-bitcoin-node/src/status/inbound.rs; packages/open-bitcoin-cli/src/operator/support/render/inbound.rs; packages/open-bitcoin-cli/src/operator/support/tests.rs; docs/architecture/status-snapshot.md] |
</phase_requirements>

## Summary

Phase 91 should be planned as a typed pure-domain extension of the Phase 90 inbound admission model, not as a runtime listener rewrite. [VERIFIED: 91-CONTEXT.md; packages/open-bitcoin-network/src/inbound.rs; packages/open-bitcoin-rpc/src/inbound_listener.rs] The existing runtime accept loop calls `record_inbound_admission` with `InboundAdmissionSlotClass::Ordinary`, so permission-aware admission needs a pure class-selection step before that call and a richer admission record after parsing. [VERIFIED: packages/open-bitcoin-rpc/src/inbound_listener.rs; packages/open-bitcoin-rpc/src/context/network.rs; packages/open-bitcoin-network/src/inbound.rs]

Knots permissions are broader than Phase 91 is allowed to activate. [VERIFIED: packages/bitcoin-knots/src/net_permissions.h; packages/bitcoin-knots/src/net_processing.cpp; 91-CONTEXT.md] Knots uses `noban`, `download`, `addr`, `mempool`, `relay`, `forcerelay`, `bloomfilter`, `blockfilters`, and `forceinbound` across admission, eviction, block serving, address behavior, service-bit advertisement, mempool handling, and transaction relay. [VERIFIED: packages/bitcoin-knots/src/net.cpp; packages/bitcoin-knots/src/net_processing.cpp] Open Bitcoin must therefore store active and inactive effects separately, especially for `all`, so a parsed permission set cannot silently enable existing inventory, tx, mempool, or compact-filter behavior. [VERIFIED: 91-CONTEXT.md; packages/open-bitcoin-network/src/peer.rs; packages/bitcoin-knots/test/functional/p2p_permissions.py]

The safest planning shape is: parse Open Bitcoin JSONC/CLI class definitions into domain types in `open-bitcoin-network`; resolve config and validation errors in `open-bitcoin-rpc`; let `open-bitcoin-node` count/project decisions; and keep CLI support/status renderers as pure renderers over shared status. [VERIFIED: 91-CONTEXT.md; AGENTS.md; standards/core/architecture.md; standards/languages/rust.md]

**Primary recommendation:** Implement a `PeerPermissionSet` plus `PeerConnectionClass` pure model in `open-bitcoin-network`, with explicit active/inactive effect labels, then thread it through Phase 90 admission records and shared inbound status without adding relay, mempool, filter, ban, or address-management behavior. [VERIFIED: 91-CONTEXT.md; packages/open-bitcoin-network/src/inbound.rs; packages/open-bitcoin-node/src/status/inbound.rs; packages/bitcoin-knots/src/net_permissions.h]

## Project Constraints (from AGENTS.md)

- Use `git submodule update --init --recursive` to materialize the pinned Knots baseline under `packages/bitcoin-knots`; the current submodule is present at `a9aee730466ac67d35a3c03ee24676be5e045878` tagged `v29.3.knots20260210`. [VERIFIED: AGENTS.md; git submodule status packages/bitcoin-knots]
- Use `rust-toolchain.toml` as the Rust source of truth; local `rustc` and `cargo` report `1.94.1`. [VERIFIED: AGENTS.md; rustc --version; cargo --version]
- Use `bash scripts/verify.sh` as the repo-native verification contract for first-party code, including the Bazel smoke build; `--fast` is local iteration only. [VERIFIED: AGENTS.md]
- During UAT, provide repo-local Cargo and Bazel commands rather than only an installed `open-bitcoin` alias. [VERIFIED: AGENTS.md]
- Use Bun for repo-owned higher-level automation scripts; local Bun reports `1.3.9`. [VERIFIED: AGENTS.md; bun --version]
- Treat `docs/metrics/lines-of-code.md` as tracked generated output that may legitimately change when verification regenerates it. [VERIFIED: AGENTS.md]
- Record intentional in-scope behavior differences from Bitcoin Knots in `docs/parity/index.json` and companion docs. [VERIFIED: AGENTS.md]
- New first-party Rust source or test files under `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests` require parity breadcrumb comments and `docs/parity/source-breadcrumbs.json` registration. [VERIFIED: AGENTS.md; docs/parity/source-breadcrumbs.json]
- Pure Bitcoin domain behavior should remain in functional-core crates, with filesystem, network, terminal, RPC, service-manager, and storage effects isolated in shell adapters. [VERIFIED: AGENTS.md; standards/core/architecture.md]
- Rust module layout should prefer `foo.rs` plus `foo/` over new `foo/mod.rs` trees. [VERIFIED: standards/languages/rust.md]
- Unit tests for pure/business logic must be focused and use Arrange, Act, Assert comments unless trivially obvious. [VERIFIED: AGENTS.md; standards/core/testing.md]
- Avoid existing Rust Bitcoin libraries in production code; the project owns its domain model and implementation surface. [VERIFIED: AGENTS.md]
- Project skills directories `.claude/skills/` and `.agents/skills/` contain no project `SKILL.md` files. [VERIFIED: find .claude/skills .agents/skills -maxdepth 2 -name SKILL.md]

## Standard Stack

### Core

| Library/Crate | Version | Purpose | Why Standard |
|---------------|---------|---------|--------------|
| Rust workspace crates | local `0.1.0`, Rust 2024 | Domain parsing, admission policy, node projection, RPC config, CLI rendering | Existing repo stack and all first-party package targets use Rust 2024. [VERIFIED: cargo metadata --manifest-path packages/Cargo.toml --no-deps] |
| `open-bitcoin-network` | local `0.1.0` | Pure permission vocabulary, connection classes, admission inputs, and peer-policy helpers | Current Phase 90 `InboundAdmissionPolicy`, `InboundAdmissionRequest`, `InboundAdmissionSlotClass`, and `InboundPeerRecord` already live here. [VERIFIED: packages/open-bitcoin-network/src/inbound.rs; cargo metadata] |
| `open-bitcoin-rpc` config loader | local `0.1.0` | Open Bitcoin JSONC and daemon CLI parsing/validation | Existing `InboundConfig`, CLI inbound parser, and `resolve_inbound_listener_config` own the inbound config boundary. [VERIFIED: packages/open-bitcoin-rpc/src/config/open_bitcoin.rs; packages/open-bitcoin-rpc/src/config/loader/inbound.rs; packages/open-bitcoin-rpc/src/config/loader/open_bitcoin_runtime.rs] |
| `open-bitcoin-node` | local `0.1.0` | Managed network admission counters, status contract, metrics kinds | Current node-side admission info and shared inbound status live here. [VERIFIED: packages/open-bitcoin-node/src/network/inbound.rs; packages/open-bitcoin-node/src/status/inbound.rs; packages/open-bitcoin-node/src/metrics.rs] |
| `open-bitcoin-cli` | local `0.1.0` | Human status and support bundle rendering | Existing inbound renderers already project the shared status model and redacted support evidence. [VERIFIED: packages/open-bitcoin-cli/src/operator/status/render/inbound.rs; packages/open-bitcoin-cli/src/operator/support/render/inbound.rs] |
| Pinned Bitcoin Knots | `v29.3.knots20260210` submodule | Parity anchor for permission names and deferred semantics | Phase 91 explicitly anchors vocabulary and hazards to Knots permission sources. [VERIFIED: git submodule status packages/bitcoin-knots; 91-CONTEXT.md; packages/bitcoin-knots/src/net_permissions.h] |

### Supporting

| Library/Tool | Version | Purpose | When to Use |
|--------------|---------|---------|-------------|
| `serde` / `serde_json` | manifest reqs `^1.0.228` / `^1.0.149` | Stable status/config JSON shapes | Use for shared status structs and JSONC-deserialized config structs already in the repo. [VERIFIED: cargo metadata; packages/open-bitcoin-node/src/status/inbound.rs; packages/open-bitcoin-rpc/src/config/open_bitcoin.rs] |
| `jsonc-parser` | manifest req `^0.32.3` | `open-bitcoin.jsonc` parsing | Continue using existing Open Bitcoin config parsing; do not add an independent JSON parser. [VERIFIED: cargo metadata; packages/open-bitcoin-rpc/src/config/open_bitcoin.rs] |
| `tokio` | manifest req `^1.52.1` | Existing runtime listener adapter | Use only for thin accept-loop wiring; do not put permission decisions inside Tokio tasks. [VERIFIED: cargo metadata; packages/open-bitcoin-rpc/src/inbound_listener.rs; standards/core/architecture.md] |
| Bun | `1.3.9` | Deterministic TypeScript checker if Phase 91 adds docs/guardrail scripts | Use for any Phase 91 checker following the Phase 90 checker pattern. [VERIFIED: bun --version; scripts/check-phase90-inbound-listener-admission.ts; AGENTS.md] |
| Bazelisk/Bazel | Bazelisk `1.28.1`, Bazel `8.6.0` | Repo smoke build in native verification and UAT command forms | Include Bazel command forms in UAT docs and rely on `scripts/verify.sh` for default verification. [VERIFIED: bazelisk version; bazel --version; AGENTS.md] |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Open Bitcoin JSONC `inbound.permission_classes` plus Open Bitcoin-prefixed CLI | Knots `-whitelist` / `-whitebind` | Rejected for Phase 91 planning because the user locked Open Bitcoin-owned config and explicitly disallowed silent full baseline compatibility. [VERIFIED: 91-CONTEXT.md; docs/architecture/config-precedence.md; packages/open-bitcoin-rpc/src/config/tests.rs] |
| First-party typed permission model | Raw `Vec<String>` permission labels in config/status | Rejected because D-04 requires typed bundles and Bright Builds requires parse-at-boundaries/domain types. [VERIFIED: 91-CONTEXT.md; standards/core/architecture.md; standards/languages/rust.md] |
| Literal IP address matching for v1.9 class assignment | CIDR/subnet whitelist parity | Use literal `IpAddr` matching for Phase 91 because the repo currently has no first-party subnet/CIDR primitive and inbound remote endpoints have ephemeral ports; defer CIDR compatibility to a later explicit dependency/design decision. [VERIFIED: rg SubNet/Cidr/IpNet; packages/open-bitcoin-rpc/src/inbound_listener.rs; packages/bitcoin-knots/src/net_permissions.cpp] |
| Shared inbound status child contract | Renderer-local status/support strings | Rejected because D-15 requires shared status/support contracts and current Phase 90 docs require `OpenBitcoinStatusSnapshot.peers.inbound` as source of truth. [VERIFIED: 91-CONTEXT.md; docs/architecture/status-snapshot.md] |

**Installation:**

```bash
# No new external package is recommended for Phase 91.
```

**Version verification:** Existing Rust package requirements were verified from `cargo metadata --manifest-path packages/Cargo.toml --no-deps --format-version 1`; tool versions were verified with `rustc --version`, `cargo --version`, `bun --version`, `bazelisk version`, and `bazel --version`. [VERIFIED: command outputs]

## Architecture Patterns

### Recommended Project Structure

```text
packages/open-bitcoin-network/src/
|-- inbound.rs                    # Existing module entry; re-export permission types here.
|-- inbound/
|   |-- permissions.rs            # New pure permission token/set/effect/class parser and labels.
|   `-- permission_policy.rs      # New pure admission/address/download/eviction input decisions if split is needed.
`-- peer.rs                       # Thread parsed permission evidence through peer records and policy helpers.

packages/open-bitcoin-rpc/src/config/
|-- open_bitcoin.rs               # Add JSONC config DTOs under inbound.
`-- loader/
    |-- inbound.rs                # Add Open Bitcoin-prefixed CLI parsing.
    `-- open_bitcoin_runtime.rs   # Resolve/validate into domain config.

packages/open-bitcoin-node/src/
|-- network/inbound.rs            # Count permissioned/protected admits and latest decision labels.
|-- status/inbound.rs             # Extend shared inbound status contract.
`-- metrics.rs                    # Add numeric low-cardinality counters only if status needs metrics.

packages/open-bitcoin-cli/src/operator/
|-- status/render/inbound.rs      # Render shared status fields only.
`-- support/render/inbound.rs     # Render redacted shared evidence only.
```

This structure follows the existing `inbound.rs` plus `inbound/` Rust module shape and avoids growing `packages/open-bitcoin-network/src/inbound.rs`, which is already 510 lines and close to the Bright Builds file-length refactor trigger. [VERIFIED: wc -l packages/open-bitcoin-network/src/inbound.rs; standards/languages/rust.md; standards/core/code-shape.md]

### Pattern 1: Parse Permission Classes into Domain Types

**What:** Parse config/CLI strings into `PeerPermissionToken`, `PeerPermissionSet`, `PeerPermissionEffect`, `ConnectionDirectionBoundary`, and `PeerConnectionClass` before admission policy uses them. [VERIFIED: 91-CONTEXT.md; standards/core/architecture.md]

**When to use:** Use at the Open Bitcoin JSONC/CLI boundary, then pass typed values through admission and status projection. [VERIFIED: packages/open-bitcoin-rpc/src/config/open_bitcoin.rs; packages/open-bitcoin-rpc/src/config/loader/open_bitcoin_runtime.rs]

**Example:**

```rust
// Source: packages/bitcoin-knots/src/net_permissions.cpp and 91-CONTEXT.md
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PeerPermissionToken {
    BloomFilter,
    BlockFilters,
    NoBan,
    ForceRelay,
    Relay,
    Mempool,
    Download,
    Addr,
    ForceInbound,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedPeerPermissionClass {
    pub name: PermissionClassName,
    pub directions: ConnectionDirectionBoundary,
    pub permissions: PeerPermissionSet,
    pub match_addresses: Vec<std::net::IpAddr>,
}
```

### Pattern 2: Store Active and Inactive Effects Separately

**What:** Compute active v1.9 effects and inactive/deferred effects during parsing; never infer active behavior from string labels later. [VERIFIED: 91-CONTEXT.md; packages/bitcoin-knots/src/net_permissions.h]

**When to use:** Use whenever `all`, `forcerelay`, `relay`, `mempool`, `bloomfilter`, or `blockfilters` appears in config. [VERIFIED: 91-CONTEXT.md; packages/bitcoin-knots/test/functional/p2p_permissions.py]

**Example:**

```rust
// Source: 91-CONTEXT.md D-10 through D-14
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PermissionEffectLabel {
    AdmissionProtected,
    EvictionPolicyProtected,
    AddressResponsePolicyInput,
    DownloadServingPolicyInput,
    InactiveRelay,
    InactiveForceRelay,
    InactiveMempool,
    InactiveBloomFilter,
    InactiveBlockFilters,
}
```

### Pattern 3: Keep Admission Capacity and Connection Class Separate

**What:** Keep Phase 90 capacity accounting (`Ordinary` vs `Reserved`) separate from semantic classes (`ordinary_inbound`, `permissioned_inbound`, `protected_inbound`). [VERIFIED: packages/open-bitcoin-network/src/inbound.rs; 91-CONTEXT.md]

**When to use:** Use when mapping `forceinbound` or `noban` to reserved/protected admission without losing the rule that ordinary peers cannot consume protected capacity. [VERIFIED: 91-CONTEXT.md; packages/open-bitcoin-network/src/inbound.rs]

**Example:**

```rust
// Source: packages/open-bitcoin-network/src/inbound.rs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PeerConnectionClass {
    OrdinaryInbound,
    PermissionedInbound,
    ProtectedInbound,
    Outbound,
    ManualConfigured,
}

impl PeerConnectionClass {
    pub const fn slot_class(self) -> InboundAdmissionSlotClass {
        match self {
            Self::OrdinaryInbound => InboundAdmissionSlotClass::Ordinary,
            Self::PermissionedInbound | Self::ProtectedInbound => {
                InboundAdmissionSlotClass::Reserved
            }
            Self::Outbound | Self::ManualConfigured => InboundAdmissionSlotClass::Ordinary,
        }
    }
}
```

### Pattern 4: Project Permission Evidence Through Shared Status

**What:** Extend `InboundPeerServingStatus` or a child struct with fixed low-cardinality fields such as `permissioned_inbound_peers`, `protected_inbound_peers`, `inactive_permission_effects`, and `latest_permission_decision`. [VERIFIED: packages/open-bitcoin-node/src/status/inbound.rs; 91-CONTEXT.md; docs/architecture/status-snapshot.md]

**When to use:** Use after node-side admission records include parsed class/effect evidence. [VERIFIED: packages/open-bitcoin-node/src/network/inbound.rs; packages/open-bitcoin-rpc/src/context/network.rs]

**Example:**

```rust
// Source: packages/open-bitcoin-node/src/status/inbound.rs
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InboundPermissionDecisionEvent {
    pub outcome: String,
    pub class_label: String,
    pub active_effects: Vec<String>,
    pub inactive_effects: Vec<String>,
    pub reason: String,
    pub message: String,
}
```

### Anti-Patterns to Avoid

- **Raw permission strings after parsing:** Rechecking string tokens throughout admission/status code violates D-04 and parse-at-boundaries guidance. [VERIFIED: 91-CONTEXT.md; standards/core/architecture.md]
- **Runtime accept-loop permission mutation:** Hiding `forceinbound` or `noban` effects inside `handle_inbound_stream` violates D-07; choose the class before `record_inbound_admission`. [VERIFIED: 91-CONTEXT.md; packages/open-bitcoin-rpc/src/inbound_listener.rs]
- **Overloading `getnetworkinfo`:** Detailed permission evidence belongs in `openbitcoinnetworkstatus` and `OpenBitcoinStatusSnapshot.peers.inbound`, not baseline-shaped `getnetworkinfo`. [VERIFIED: docs/architecture/status-snapshot.md; packages/open-bitcoin-rpc/src/dispatch/tests.rs]
- **Accepting Knots aliases silently:** Knots accepts `bloom`, `compactfilters`, and `cfilters`, but Phase 91 D-01 lists explicit Open Bitcoin tokens; reject aliases with stable errors unless the plan deliberately documents them. [VERIFIED: 91-CONTEXT.md; packages/bitcoin-knots/src/net_permissions.cpp]
- **CIDR parsing as a quick helper:** The repo has no existing subnet primitive; do not hand-roll CIDR matching inside Phase 91. [VERIFIED: rg SubNet/Cidr/IpNet; standards/core/architecture.md]

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Permission flag semantics | Ad hoc `Vec<String>` checks in admission/status | `PeerPermissionSet` and `PermissionEffectLabel` domain types | Knots flags have implied permissions and `all` expansion; typed sets prevent illegal active/deferred combinations. [VERIFIED: packages/bitcoin-knots/src/net_permissions.h; 91-CONTEXT.md] |
| Open Bitcoin config parsing | Independent JSON or CLI parser | Existing `InboundConfig`, `parse_inbound_cli_arg`, and `resolve_inbound_listener_config` paths | Existing config precedence, `deny_unknown_fields`, and stable `ConfigError` patterns already cover inbound settings. [VERIFIED: packages/open-bitcoin-rpc/src/config/open_bitcoin.rs; packages/open-bitcoin-rpc/src/config/loader/inbound.rs; packages/open-bitcoin-rpc/src/config/loader/open_bitcoin_runtime.rs] |
| Full Knots whitelist compatibility | `-whitelist`, `-whitebind`, legacy `-whitelistrelay`, `-whitelistforcerelay` support | Open Bitcoin-owned `inbound.permission_classes` plus Open Bitcoin-prefixed CLI overrides | User locked Open Bitcoin-owned config and no silent baseline compatibility. [VERIFIED: 91-CONTEXT.md; packages/open-bitcoin-rpc/src/config/tests.rs] |
| CIDR/subnet matcher | Custom IPv4/IPv6 prefix math | Literal `IpAddr` matching for v1.9; defer CIDR or require a separate dependency/design decision | Incorrect subnet matching is security-sensitive and no first-party subnet type exists. [VERIFIED: rg SubNet/Cidr/IpNet; packages/open-bitcoin-rpc/src/inbound_listener.rs] |
| Relay permission behavior | New transaction relay, mempool query, force-relay rebroadcast, compact-filter service | Inactive/deferred permission labels and negative tests | Phase 91 prohibits activating relay, mempool, force-relay, BIP37, compact-filter, or compact-block behavior. [VERIFIED: 91-CONTEXT.md; packages/bitcoin-knots/src/net_processing.cpp] |
| Renderer summaries | CLI/support-only permission summaries | Shared `InboundPeerServingStatus` child fields | Status, JSON, RPC extension, and support bundles must agree on the same shared evidence. [VERIFIED: 91-CONTEXT.md; docs/architecture/status-snapshot.md] |
| Ban/eviction engine | Real ban/discourage/disconnect semantics | Policy input labels only (`eviction_policy_protected`) | Phase 93 owns actual ban, discourage, disconnect, expiry, unban, and misbehavior behavior. [VERIFIED: 91-CONTEXT.md; .planning/ROADMAP.md] |

**Key insight:** Knots permission names are not a small admission feature; they touch relay, mempool, filters, address manager, block serving, ban/discourage, and eviction code, so Phase 91 must model the vocabulary without inheriting the whole behavior surface. [VERIFIED: packages/bitcoin-knots/src/net_permissions.h; packages/bitcoin-knots/src/net.cpp; packages/bitcoin-knots/src/net_processing.cpp; 91-CONTEXT.md]

## Existing Open Bitcoin Integration Points

| Area | Current State | Phase 91 Planning Implication |
|------|---------------|-------------------------------|
| Pure admission | `InboundAdmissionPolicy::decide` accepts `slot_class`, counters, endpoint keys, peer ids, local nonce, remote nonce, and shutdown flag. [VERIFIED: packages/open-bitcoin-network/src/inbound.rs] | Add permission class/effect evidence to `InboundAdmissionRequest` and `InboundPeerRecord`, or add a sibling typed field, before node insertion. [VERIFIED: packages/open-bitcoin-network/src/inbound.rs] |
| Runtime listener | `handle_inbound_stream` currently calls `record_inbound_admission(peer_id, remote_addr.to_string(), false)` and records admits/rejects. [VERIFIED: packages/open-bitcoin-rpc/src/inbound_listener.rs] | Add class resolution from remote IP before admission; keep socket I/O as thin adapter. [VERIFIED: packages/open-bitcoin-rpc/src/inbound_listener.rs; standards/core/architecture.md] |
| Node counters | `ManagedInboundAdmissionInfo` counts admitted/rejected peers, reserved admits, cap/reserved/duplicate/self/shutdown rejections, and latest rejection reason. [VERIFIED: packages/open-bitcoin-node/src/network/inbound.rs] | Extend with fixed counters for permissioned/protected admits, inactive relay-like permission observations, and validation failures only if metrics/status need them. [VERIFIED: 91-CONTEXT.md; packages/open-bitcoin-node/src/network/inbound.rs] |
| Shared status | `InboundPeerServingStatus` contains listener state, endpoint list, preflight reason, admission counts, handshake counts, rejection counters, and latest admission event. [VERIFIED: packages/open-bitcoin-node/src/status/inbound.rs] | Add permission evidence here or in a closely owned child contract; do not add renderer-local fields. [VERIFIED: 91-CONTEXT.md; docs/architecture/status-snapshot.md] |
| RPC extension | `openbitcoinnetworkstatus` returns `current_inbound_status`; `getnetworkinfo` omits detailed inbound status. [VERIFIED: packages/open-bitcoin-rpc/src/dispatch/node.rs; packages/open-bitcoin-rpc/src/dispatch/tests.rs] | Keep detailed permission evidence on the Open Bitcoin extension and shared snapshot, not baseline-shaped `getnetworkinfo`. [VERIFIED: docs/architecture/status-snapshot.md] |
| Config | `InboundConfig` has `enabled`, `listen_addresses`, `max_peers`, `reserved_slots`, and `allow_public`; it uses `deny_unknown_fields`. [VERIFIED: packages/open-bitcoin-rpc/src/config/open_bitcoin.rs] | Add `permission_classes` under `inbound`; malformed fields should fail deterministically. [VERIFIED: 91-CONTEXT.md; packages/open-bitcoin-rpc/src/config/tests.rs] |
| Baseline config rejection | Tests reject `listen`, `bind`, `whitebind`, and `whitelist` in `bitcoin.conf`. [VERIFIED: packages/open-bitcoin-rpc/src/config/tests.rs] | Preserve rejection and add tests that Knots permission keys are not accepted as baseline compatibility. [VERIFIED: 91-CONTEXT.md] |
| Peer message paths | `PeerManager` handles `WtxidRelay`, `Inv`, `GetData`, `Tx`, and `Block`, but there is no `getaddr`, `mempool`, bloom filter, or compact-filter message support. [VERIFIED: packages/open-bitcoin-network/src/peer.rs; packages/open-bitcoin-network/src/message.rs] | Add negative tests showing permission tokens do not alter transaction relay or mempool/filter handling; address permission can only be a typed policy input until Phase 92. [VERIFIED: 91-CONTEXT.md; rg GetAddr/Mempool/Bloom/Filter] |
| Support redaction | Support tests redact raw inbound endpoints into bounded summaries. [VERIFIED: packages/open-bitcoin-cli/src/operator/support/tests.rs] | Permission evidence must use class labels/effect labels, not raw peer tables, raw endpoints, user labels, or raw config strings. [VERIFIED: 91-CONTEXT.md; packages/open-bitcoin-cli/src/operator/support/tests.rs] |

## Knots Permission Semantics to Model or Defer

| Knots Concept | Knots Behavior | Open Bitcoin Phase 91 Treatment |
|---------------|----------------|---------------------------------|
| Permission vocabulary | Knots documents `bloomfilter`, `blockfilters`, `noban`, `forcerelay`, `relay`, `mempool`, `download`, `addr`, `forceinbound`; parser also recognizes direction tokens and aliases. [VERIFIED: packages/bitcoin-knots/src/net_permissions.cpp] | Support only D-01 explicit tokens; reject unsupported aliases/tokens with stable field/token errors. [VERIFIED: 91-CONTEXT.md] |
| Implied flags | `ForceRelay` includes `Relay`; `NoBan` includes `Download`; `ForceInbound` includes `NoBan`; `All` includes broad permission flags. [VERIFIED: packages/bitcoin-knots/src/net_permissions.h] | Preserve implied relationships in parsed evidence, but separate active from inactive effects so implied relay/filter/mempool behavior stays inactive. [VERIFIED: 91-CONTEXT.md] |
| Default whitelist | Knots implicit whitelist can add `relay`, `mempool`, `noban`, `download`, and `addr`, with legacy `whitelistrelay/forcerelay` interactions. [VERIFIED: packages/bitcoin-knots/src/net.cpp; packages/bitcoin-knots/test/functional/p2p_permissions.py] | Do not implement implicit defaults or legacy whitelist interactions; require explicit Open Bitcoin class definitions. [VERIFIED: 91-CONTEXT.md] |
| Directions | Knots defaults whitelist direction to inbound and rejects direction-only inputs. [VERIFIED: packages/bitcoin-knots/src/net_permissions.cpp; packages/bitcoin-knots/src/netbase.h] | Require an explicit `in` direction for active Phase 91 inbound classes; reject direction-only and unsupported `out` combinations with stable errors. [VERIFIED: 91-CONTEXT.md] |
| Admission full slots | Knots uses `ForceInbound` to attempt eviction to make room and `NoBan` to bypass banned/discouraged checks. [VERIFIED: packages/bitcoin-knots/src/net.cpp] | Use `forceinbound`/`noban` only as pure admission/reserved-slot/protected-policy inputs; do not evict or mutate hidden runtime state. [VERIFIED: 91-CONTEXT.md] |
| Eviction immunity | Knots includes `m_noban` in eviction candidates and skips punishment for `NoBan` peers. [VERIFIED: packages/bitcoin-knots/src/net.cpp; packages/bitcoin-knots/src/net_processing.cpp] | Expose `eviction_policy_protected` / `misbehavior_policy_protected` labels only; Phase 93 implements real eviction/ban/misbehavior. [VERIFIED: 91-CONTEXT.md; .planning/ROADMAP.md] |
| Download | Knots `Download` permits historical block serving beyond limits and affects minimum-chain-work behavior. [VERIFIED: packages/bitcoin-knots/src/net_processing.cpp] | Expose `download_serving_policy_input` only and test it does not add a new unattended serving claim. [VERIFIED: 91-CONTEXT.md] |
| Addr | Knots `Addr` bypasses address rate limiting/cache behavior and changes `getaddr` response source. [VERIFIED: packages/bitcoin-knots/src/net_processing.cpp] | Expose typed `address_response_policy_input`; Phase 92 owns actual address response behavior. [VERIFIED: 91-CONTEXT.md; .planning/ROADMAP.md] |
| Mempool | Knots `Mempool` permits mempool query handling in conditions where it would otherwise disconnect or ignore. [VERIFIED: packages/bitcoin-knots/src/net_processing.cpp] | Parse as inactive/deferred or reject; do not add `mempool` message handling. [VERIFIED: 91-CONTEXT.md; packages/open-bitcoin-network/src/message.rs] |
| Relay/ForceRelay | Knots `Relay` affects incoming tx rejection; `ForceRelay` can rebroadcast already-known mempool txs. [VERIFIED: packages/bitcoin-knots/src/net_processing.cpp; packages/bitcoin-knots/test/functional/p2p_permissions.py] | Parse as inactive/deferred or reject; add tests proving no transaction relay, force-relay rebroadcast, or mempool propagation is enabled. [VERIFIED: 91-CONTEXT.md; packages/open-bitcoin-network/src/peer.rs] |
| Bloom/filter permissions | Knots grants service bits for bloom and compact filters when permissioned. [VERIFIED: packages/bitcoin-knots/src/net_processing.cpp] | Parse as inactive/deferred or reject; do not advertise `NODE_BLOOM` or compact filters. [VERIFIED: 91-CONTEXT.md] |

## Common Pitfalls

### Pitfall 1: `all` Enables Deferred Behavior

**What goes wrong:** `all` expands to relay, force-relay, mempool, bloom/filter, download, addr, noban, and forceinbound, then downstream code treats every flag as active. [VERIFIED: packages/bitcoin-knots/src/net_permissions.h; packages/bitcoin-knots/test/functional/p2p_permissions.py]

**Why it happens:** Knots `All` is a broad bitset and Open Bitcoin already has peer paths for `WtxidRelay`, `Inv`, `Tx`, and `GetData`. [VERIFIED: packages/bitcoin-knots/src/net_permissions.h; packages/open-bitcoin-network/src/peer.rs]

**How to avoid:** Parse `all` into active bounded effects plus inactive/deferred labels, and assert inactive labels in status/support. [VERIFIED: 91-CONTEXT.md]

**Warning signs:** Tests only check parse success for `all` and do not assert inactive `relay`, `mempool`, `forcerelay`, `bloomfilter`, and `blockfilters` effects. [VERIFIED: 91-CONTEXT.md]

### Pitfall 2: Config Compatibility Creep

**What goes wrong:** Implementing `-whitelist`, `-whitebind`, or Knots aliases makes operators think full Knots peer-permission compatibility exists. [VERIFIED: 91-CONTEXT.md; docs/architecture/config-precedence.md]

**Why it happens:** Knots permission docs and tests focus on those baseline options. [VERIFIED: packages/bitcoin-knots/src/init.cpp; packages/bitcoin-knots/test/functional/p2p_permissions.py]

**How to avoid:** Keep Open Bitcoin-owned JSONC/CLI flags and preserve tests rejecting baseline listener/permission keys. [VERIFIED: packages/open-bitcoin-rpc/src/config/tests.rs; 91-CONTEXT.md]

**Warning signs:** New tests accept `whitelist`, `whitebind`, `whitelistrelay`, or `whitelistforcerelay` without explicitly documenting a deviation. [VERIFIED: packages/open-bitcoin-rpc/src/config/tests.rs]

### Pitfall 3: Direction Tokens Become Ambiguous

**What goes wrong:** `in`, `out`, or `in,out` are accepted without permissions or with unsupported active outbound effects. [VERIFIED: packages/bitcoin-knots/src/net_permissions.cpp; 91-CONTEXT.md]

**Why it happens:** Knots parser treats direction as tokens in the permission prefix; Open Bitcoin v1.9 only has inbound admission effects. [VERIFIED: packages/bitcoin-knots/src/net_permissions.cpp; packages/open-bitcoin-rpc/src/inbound_listener.rs]

**How to avoid:** Require permissions plus `in` for active inbound classes; reject direction-only and unsupported outbound combinations deterministically. [VERIFIED: 91-CONTEXT.md]

**Warning signs:** A class with only `out` changes outbound sync counts, target outbound peers, or manual peer behavior. [VERIFIED: 91-CONTEXT.md; packages/open-bitcoin-node/src/sync/types.rs]

### Pitfall 4: Permissioned Peers Starve Outbound Sync

**What goes wrong:** Permissioned inbound peers reduce `target_outbound_peers`, count as outbound progress, or share retry/resource budgets. [VERIFIED: 91-CONTEXT.md; packages/open-bitcoin-node/src/sync/types.rs]

**Why it happens:** "connection classes" can be misread as a shared inbound/outbound connection budget. [VERIFIED: 91-CONTEXT.md]

**How to avoid:** Keep inbound admission counters separate from sync runtime targets and add tests that outbound count/target is unchanged after permissioned inbound admits. [VERIFIED: packages/open-bitcoin-network/src/inbound.rs; packages/open-bitcoin-node/src/sync/types.rs; 91-CONTEXT.md]

**Warning signs:** Code touches `target_outbound_peers`, sync peer backoff, or `SyncRuntimeConfig` for permissioned inbound classes. [VERIFIED: packages/open-bitcoin-node/src/sync/types.rs]

### Pitfall 5: Status Leaks Config or Peer Identity

**What goes wrong:** Support bundles include raw class definitions, raw endpoints, peer ids, user labels, or full peer tables. [VERIFIED: 91-CONTEXT.md; packages/open-bitcoin-cli/src/operator/support/tests.rs]

**Why it happens:** Permission evidence needs explanation, and the easiest data is raw config or peer state. [VERIFIED: 91-CONTEXT.md]

**How to avoid:** Project stable class labels and effect labels only; keep existing endpoint redaction and unavailable-reason patterns. [VERIFIED: packages/open-bitcoin-node/src/status/inbound.rs; packages/open-bitcoin-cli/src/operator/support/tests.rs]

**Warning signs:** Metrics labels include peer ids/endpoints/class names, or support Markdown includes raw IP lists beyond bounded redaction summaries. [VERIFIED: 91-CONTEXT.md; docs/architecture/operator-observability.md]

### Pitfall 6: New Rust Files Miss Parity Breadcrumbs

**What goes wrong:** New permission modules or tests fail source-breadcrumb checks. [VERIFIED: AGENTS.md; docs/parity/source-breadcrumbs.json]

**Why it happens:** Phase 91 likely needs new Rust child modules because `inbound.rs` is already near the file-length trigger. [VERIFIED: wc -l packages/open-bitcoin-network/src/inbound.rs; standards/core/code-shape.md]

**How to avoid:** Add top-of-file parity breadcrumb blocks citing `net_permissions.h`, `net_permissions.cpp`, `p2p_permissions.py`, and any bounded `net.cpp`/`net_processing.cpp` anchors; register files in `docs/parity/source-breadcrumbs.json`. [VERIFIED: AGENTS.md; 91-CONTEXT.md; docs/parity/source-breadcrumbs.json]

**Warning signs:** New `packages/open-bitcoin-*/src` or `tests` files are absent from `docs/parity/source-breadcrumbs.json`. [VERIFIED: AGENTS.md]

## Code Examples

Verified patterns from existing code and Knots anchors:

### Current Phase 90 Admission Seam

```rust
// Source: packages/open-bitcoin-network/src/inbound.rs
pub struct InboundAdmissionRequest {
    pub peer_id: PeerId,
    pub remote_endpoint: String,
    pub slot_class: InboundAdmissionSlotClass,
    pub counters: InboundAdmissionCounters,
    pub existing_endpoint_keys: BTreeSet<String>,
    pub existing_peer_ids: BTreeSet<PeerId>,
    pub local_nonce: u64,
    pub maybe_remote_nonce: Option<u64>,
    pub is_shutdown_requested: bool,
}
```

Planner use: add permission class/effect fields here or in a sibling typed request struct before policy decisions. [VERIFIED: packages/open-bitcoin-network/src/inbound.rs]

### Current Runtime Admission Call

```rust
// Source: packages/open-bitcoin-rpc/src/context/network.rs
self.network.admit_inbound_peer(InboundAdmissionRequest {
    peer_id,
    remote_endpoint,
    slot_class: InboundAdmissionSlotClass::Ordinary,
    counters: Default::default(),
    existing_endpoint_keys: Default::default(),
    existing_peer_ids: Default::default(),
    local_nonce: 0,
    maybe_remote_nonce: None,
    is_shutdown_requested,
})
```

Planner use: replace hard-coded ordinary admission with a resolved `PeerConnectionClass` and permission evidence. [VERIFIED: packages/open-bitcoin-rpc/src/context/network.rs]

### Current Shared Inbound Status Contract

```rust
// Source: packages/open-bitcoin-node/src/status/inbound.rs
pub struct InboundAdmissionEvent {
    pub outcome: String,
    pub reason: String,
    pub slot_class: String,
    pub message: String,
}
```

Planner use: extend with permission decision evidence or add a child event while keeping shared status as source of truth. [VERIFIED: packages/open-bitcoin-node/src/status/inbound.rs; docs/architecture/status-snapshot.md]

### Knots Permission Parse Hazards

```cpp
// Source: packages/bitcoin-knots/src/net_permissions.h
// ForceRelay includes Relay; NoBan includes Download; ForceInbound includes NoBan.
// All includes BloomFilter, ForceRelay, Relay, NoBan, Mempool, Download, Addr,
// BlockFilters, and ForceInbound.
```

Planner use: encode implied relationships, then mark out-of-scope effects inactive. [VERIFIED: packages/bitcoin-knots/src/net_permissions.h; 91-CONTEXT.md]

## State of the Art

| Old/Adjacent Approach | Current Phase 91 Approach | When Changed | Impact |
|-----------------------|---------------------------|--------------|--------|
| Phase 90 reserved slots only | Permissioned/protected inbound classes can consume reserved admission capacity while ordinary peers cannot. [VERIFIED: packages/open-bitcoin-network/src/inbound.rs; 91-CONTEXT.md] | Phase 91 v1.9 boundary. [VERIFIED: .planning/ROADMAP.md] | Planner should extend Phase 90 slot policy rather than create a parallel admission path. [VERIFIED: 91-CONTEXT.md] |
| Knots full whitelist/whitebind semantics | Open Bitcoin-owned `inbound` config and Open Bitcoin-prefixed CLI only. [VERIFIED: 91-CONTEXT.md; docs/architecture/config-precedence.md] | Phase 91 locked decision D-02. [VERIFIED: 91-CONTEXT.md] | Baseline config compatibility remains intentionally incomplete and documented. [VERIFIED: docs/architecture/config-precedence.md] |
| Knots `all` activates all permission behavior | Open Bitcoin `all` must expose active/inactive effects and keep deferred behavior inert. [VERIFIED: 91-CONTEXT.md; packages/bitcoin-knots/src/net_permissions.h] | Phase 91 locked decision D-05. [VERIFIED: 91-CONTEXT.md] | Negative relay/mempool/filter tests are required, not optional. [VERIFIED: 91-CONTEXT.md] |
| Renderer-local inbound summaries | Shared `OpenBitcoinStatusSnapshot.peers.inbound` owns inbound evidence. [VERIFIED: docs/architecture/status-snapshot.md] | Phase 90 status contract. [VERIFIED: docs/architecture/status-snapshot.md] | Permission evidence must be added to shared status before CLI/support rendering. [VERIFIED: 91-CONTEXT.md] |
| Public-network/full-node claims | Deterministic local synthetic tests and loopback UAT only. [VERIFIED: .planning/STATE.md; 91-CONTEXT.md; docs/parity/release-readiness.md] | v1.9 milestone boundary. [VERIFIED: .planning/ROADMAP.md] | Default verification must not require public peers or service-manager checks. [VERIFIED: AGENTS.md; 91-CONTEXT.md] |

**Deprecated/outdated for this phase:**

- Treating Knots `-whitelist` defaults as Open Bitcoin behavior is out of scope for Phase 91. [VERIFIED: 91-CONTEXT.md; docs/architecture/config-precedence.md]
- Implementing transaction relay, mempool propagation, compact block relay, BIP37, or compact-filter serving from permission names is out of scope for Phase 91. [VERIFIED: 91-CONTEXT.md; .planning/REQUIREMENTS.md]
- Adding actual ban/discourage/disconnect behavior under `noban` is out of scope until Phase 93. [VERIFIED: 91-CONTEXT.md; .planning/ROADMAP.md]
- Adding bounded `getaddr` behavior under `addr` is out of scope until Phase 92 unless needed as a pure test seam. [VERIFIED: 91-CONTEXT.md; .planning/ROADMAP.md]

## Test Strategy for Planning

| Test Area | Required Cases | Existing Pattern |
|-----------|----------------|------------------|
| Permission parser | Accept D-01 exact tokens, reject aliases/unknown tokens, reject empty/direction-only, reject unsupported direction combinations, expand `all` with active/inactive effects. [VERIFIED: 91-CONTEXT.md; packages/bitcoin-knots/src/net_permissions.cpp] | Add pure Rust tests near `packages/open-bitcoin-network/src/inbound/permissions.rs`; use Arrange/Act/Assert. [VERIFIED: standards/core/testing.md] |
| Config validation | JSONC `inbound.permission_classes` accepts documented shape; unknown fields and malformed classes return stable `ConfigError` with field/token. [VERIFIED: 91-CONTEXT.md; packages/open-bitcoin-rpc/src/config/tests.rs] | Existing config tests assert exact errors for inbound max/reserved validation. [VERIFIED: packages/open-bitcoin-rpc/src/config/tests.rs] |
| CLI overrides | Open Bitcoin-prefixed flags can define or select permission classes if planned; baseline `whitelist`/`whitebind` remain rejected. [VERIFIED: 91-CONTEXT.md; packages/open-bitcoin-rpc/src/config/loader/inbound.rs] | Existing CLI inbound override tests. [VERIFIED: packages/open-bitcoin-rpc/src/config/tests.rs] |
| Admission | Permissioned/protected inbound can consume reserved capacity; ordinary cannot; duplicate/self/shutdown behavior remains stable; outbound count is unchanged. [VERIFIED: 91-CONTEXT.md; packages/open-bitcoin-network/src/inbound/tests.rs] | Existing `InboundAdmissionPolicy` tests cover ordinary/reserved slot behavior. [VERIFIED: packages/open-bitcoin-network/src/inbound/tests.rs] |
| Relay safeguards | `relay`, `forcerelay`, `mempool`, `bloomfilter`, `blockfilters`, and `all` do not change `WtxidRelay`, `Inv`, `Tx`, `GetData`, or service-bit behavior. [VERIFIED: 91-CONTEXT.md; packages/open-bitcoin-network/src/peer.rs; packages/open-bitcoin-network/src/message.rs] | Existing peer tests cover inventory/tx/block paths. [VERIFIED: packages/open-bitcoin-network/src/peer/tests.rs] |
| Status/RPC | `openbitcoinnetworkstatus` shows permission class, active effects, inactive effects, latest decision; `getnetworkinfo` stays baseline-shaped. [VERIFIED: 91-CONTEXT.md; packages/open-bitcoin-rpc/src/dispatch/tests.rs] | Existing dispatch tests cover inbound status and `getnetworkinfo` separation. [VERIFIED: packages/open-bitcoin-rpc/src/dispatch/tests.rs] |
| Support redaction | Support JSON/Markdown includes class/effect labels and latest decision without raw config, endpoints, peer ids, or raw peer table. [VERIFIED: 91-CONTEXT.md; packages/open-bitcoin-cli/src/operator/support/tests.rs] | Existing support tests assert endpoint redaction. [VERIFIED: packages/open-bitcoin-cli/src/operator/support/tests.rs] |
| Docs/checker | P2P catalog, source breadcrumbs, release boundary docs, and optional Phase 91 checker mention active/inactive permission boundary. [VERIFIED: AGENTS.md; docs/parity/catalog/p2p.md; scripts/check-phase90-inbound-listener-admission.ts] | Phase 90 checker pattern validates required docs and no-claim guardrails. [VERIFIED: scripts/check-phase90-inbound-listener-admission.ts] |

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|-------------|-----------|---------|----------|
| Rust toolchain | Rust code/tests/verification | yes | `rustc 1.94.1`, `cargo 1.94.1` | none needed. [VERIFIED: rustc --version; cargo --version] |
| Bitcoin Knots submodule | Parity anchors | yes | `a9aee730466ac67d35a3c03ee24676be5e045878 (v29.3.knots20260210)` | `git submodule update --init --recursive` if missing. [VERIFIED: git submodule status packages/bitcoin-knots; AGENTS.md] |
| Bun | TypeScript checkers and repo automation | yes | `1.3.9` | Avoid new checker or use existing shell wrapper only if Bun unavailable. [VERIFIED: bun --version; AGENTS.md] |
| Node.js | GSD tools and script runtime support | yes | `v24.13.0` | none needed. [VERIFIED: node --version] |
| Bazelisk/Bazel | Bazel smoke build and UAT commands | yes | Bazelisk `1.28.1`, Bazel `8.6.0` | Use Cargo-only local iteration only if Bazel unavailable; full verification still expects repo contract. [VERIFIED: bazelisk version; bazel --version; AGENTS.md] |
| `bash scripts/verify.sh` | Repo-native pre-commit/release verification | yes | repo script | No fallback for done/commit gate. [VERIFIED: AGENTS.md] |

**Missing dependencies with no fallback:**
- None found. [VERIFIED: environment probes]

**Missing dependencies with fallback:**
- None found. [VERIFIED: environment probes]

## Security Domain

Security enforcement is enabled by default because `.planning/config.json` has no `security_enforcement: false`. [VERIFIED: .planning/config.json] OWASP ASVS latest stable is 5.0.0 per the official OWASP project page/GitHub README, but this GSD template uses the V2/V3/V4/V5/V6 category shorthand. [CITED: https://owasp.org/www-project-application-security-verification-standard/; CITED: https://github.com/OWASP/ASVS]

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|------------------|
| V2 Authentication | no | No new RPC/user auth surface is planned; do not touch credential parsing. [VERIFIED: 91-CONTEXT.md; docs/architecture/config-precedence.md] |
| V3 Session Management | no | No web/session state is introduced. [VERIFIED: 91-CONTEXT.md] |
| V4 Access Control | yes | Permission classes are access-control-like network policy inputs; enforce deny-by-default unsupported tokens and typed active/inactive effects. [VERIFIED: 91-CONTEXT.md; standards/core/architecture.md] |
| V5 Input Validation | yes | Parse JSONC/CLI tokens into domain types with stable field/token errors; reject unsupported aliases, direction-only classes, malformed class definitions, and unsupported direction combinations. [VERIFIED: 91-CONTEXT.md; packages/open-bitcoin-rpc/src/config/tests.rs] |
| V6 Cryptography | no | No new cryptography is required; do not alter peer nonce/self-connection checks beyond existing typed evidence. [VERIFIED: packages/open-bitcoin-network/src/inbound.rs; packages/open-bitcoin-network/src/peer.rs] |

### Known Threat Patterns for Phase 91 Stack

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Unsupported permission token accepted as active behavior | Elevation of Privilege | Closed enum parser, stable validation errors, and inactive/deferred labels for relay-like permissions. [VERIFIED: 91-CONTEXT.md; standards/core/architecture.md] |
| `all` activates relay/mempool/filter behavior | Elevation of Privilege / Tampering | Active/inactive effect split plus negative peer-path tests. [VERIFIED: 91-CONTEXT.md; packages/open-bitcoin-network/src/peer.rs] |
| Raw config/endpoint/peer id in metrics or support evidence | Information Disclosure | Low-cardinality counters and redacted support/status projections only. [VERIFIED: 91-CONTEXT.md; docs/architecture/operator-observability.md; packages/open-bitcoin-cli/src/operator/support/tests.rs] |
| Permissioned inbound consumes outbound sync resources | Denial of Service | Separate inbound counters from `target_outbound_peers` and assert no outbound starvation. [VERIFIED: 91-CONTEXT.md; packages/open-bitcoin-node/src/sync/types.rs] |
| Hand-rolled subnet parser misclassifies peers | Spoofing / Elevation of Privilege | Use literal `IpAddr` matching for v1.9 and defer CIDR/subnet compatibility. [VERIFIED: rg SubNet/Cidr/IpNet; packages/open-bitcoin-rpc/src/inbound_listener.rs] |

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | The v1.9 class matching shape uses literal remote IP addresses rather than CIDR/subnet matching. [RESOLVED: Phase 91 decision] | Standard Stack / Architecture Patterns | If a later phase adds CIDR/subnet compatibility, it needs a separate dependency/design task and additional parser/security tests. |

## Open Questions (RESOLVED)

1. **Should Phase 91 support CIDR/subnet class matching now?**
   - What we know: Knots supports IP/network matching through `-whitelist`, but Open Bitcoin has no existing first-party subnet/CIDR type and the user locked Open Bitcoin-owned config rather than full `-whitelist` compatibility. [VERIFIED: packages/bitcoin-knots/src/net_permissions.cpp; rg SubNet/Cidr/IpNet; 91-CONTEXT.md]
   - Decision: Phase 91 uses literal `IpAddr` matches only, rejects CIDR ranges, hostnames, and socket endpoints with stable field/value errors, and defers subnet compatibility to a later explicit phase or decision. [RESOLVED: 2026-06-25]

## Sources

### Primary (HIGH confidence)

- `.planning/phases/91-peer-permissions-and-connection-classes/91-CONTEXT.md` - locked Phase 91 decisions, scope boundaries, tests, and canonical refs.
- `.planning/REQUIREMENTS.md` - PERM-01 through PERM-04 and v1.9 out-of-scope boundaries.
- `.planning/ROADMAP.md` - Phase 91 success criteria and neighboring Phase 92/93/94/95 ownership.
- `.planning/STATE.md` - v1.9 milestone constraints and deterministic verification boundary.
- `AGENTS.md`, `AGENTS.bright-builds.md`, `standards/core/*.md`, `standards/languages/rust.md` - repo-local and Bright Builds workflow, architecture, testing, verification, and Rust rules.
- `packages/bitcoin-knots/src/net_permissions.h` and `packages/bitcoin-knots/src/net_permissions.cpp` - permission flags, implied flags, parser, `all`, directions, and string labels.
- `packages/bitcoin-knots/test/functional/p2p_permissions.py` - permission label expectations, invalid input behavior, and force-relay hazard.
- `packages/bitcoin-knots/src/net.cpp` - permission use in inbound admission, whitelist merging, NoBan/ForceInbound handling, and eviction candidate inputs.
- `packages/bitcoin-knots/src/net_processing.cpp` - permission use for service bits, download, addr, mempool, relay, force-relay, and NoBan punishment behavior.
- `packages/open-bitcoin-network/src/inbound.rs`, `packages/open-bitcoin-network/src/peer.rs`, `packages/open-bitcoin-network/src/message.rs` - current pure admission and peer message seams.
- `packages/open-bitcoin-node/src/network/inbound.rs`, `packages/open-bitcoin-node/src/status/inbound.rs`, `packages/open-bitcoin-node/src/metrics.rs` - node counters, status, and metrics contracts.
- `packages/open-bitcoin-rpc/src/config/*`, `packages/open-bitcoin-rpc/src/context/network.rs`, `packages/open-bitcoin-rpc/src/inbound_listener.rs`, `packages/open-bitcoin-rpc/src/dispatch/tests.rs` - config/admission/RPC integration points.
- `packages/open-bitcoin-cli/src/operator/status/render/inbound.rs`, `packages/open-bitcoin-cli/src/operator/support/render/inbound.rs`, `packages/open-bitcoin-cli/src/operator/support/tests.rs` - renderer and redaction patterns.
- `docs/architecture/config-precedence.md`, `docs/architecture/status-snapshot.md`, `docs/architecture/operator-observability.md`, `docs/parity/catalog/p2p.md`, `docs/parity/source-breadcrumbs.json` - docs and parity constraints.

### Secondary (MEDIUM confidence)

- OWASP ASVS official project page and GitHub README - ASVS 5.0.0 current stable/version context for security-domain framing. [CITED: https://owasp.org/www-project-application-security-verification-standard/; CITED: https://github.com/OWASP/ASVS]

### Tertiary (LOW confidence)

- None used.

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - no new dependency is recommended; versions/tooling verified locally through manifests and commands. [VERIFIED: cargo metadata; rustc --version; bun --version; bazelisk version]
- Architecture: HIGH - existing Phase 90 seams and Bright Builds standards directly support the recommended module split. [VERIFIED: packages/open-bitcoin-network/src/inbound.rs; packages/open-bitcoin-rpc/src/inbound_listener.rs; standards/core/architecture.md]
- Pitfalls: HIGH - hazards are anchored to Knots source and current Open Bitcoin peer/config/status paths. [VERIFIED: packages/bitcoin-knots/src/net_permissions.h; packages/bitcoin-knots/src/net_processing.cpp; packages/open-bitcoin-network/src/peer.rs]
- Config class matching: HIGH - Phase 91 uses literal `IpAddr` matching and rejects CIDR, hostnames, and socket endpoints; subnet compatibility is deferred to a later explicit phase or decision. [RESOLVED: 2026-06-25]

**Research date:** 2026-06-25
**Valid until:** 2026-07-25 for local repo and pinned Knots findings; re-check immediately if the Knots submodule, Rust toolchain, or v1.9 phase scope changes. [VERIFIED: current_date; git submodule status packages/bitcoin-knots; rustc --version]
