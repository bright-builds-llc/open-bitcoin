# Phase 113: Compact Relay Negotiation and Announcement Policy - Research

**Researched:** 2026-07-04 [VERIFIED: system timestamp]  
**Domain:** Bitcoin P2P compact-block negotiation, per-peer relay state, and pure announcement policy [VERIFIED: .planning/phases/113-compact-relay-negotiation-and-announcement-policy/113-CONTEXT.md]  
**Confidence:** HIGH [VERIFIED: local Open Bitcoin sources, Bitcoin Knots anchors, and BIP152 were checked in this session]

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

### Negotiation State

- **D-01:** Per-peer compact relay state must be explicit and typed: capability known/unknown, supported version, high-bandwidth preference, low-bandwidth preference, and compact-announcement eligibility must not be inferred from ad hoc booleans.
- **D-02:** `sendcmpct` version 2 is the only in-scope positive capability signal. Unsupported versions should decode as Phase 112 data but map to a stable unsupported/suppressed policy outcome instead of disconnecting by default in this phase.
- **D-03:** High-bandwidth and low-bandwidth preferences are negotiated peer state, not global activation by themselves. A peer can express a preference while still being ineligible because local activation, block-serving eligibility, header state, block availability, or resource limits fail.
- **D-04:** Negotiation state should live in pure `open-bitcoin-network` peer policy/state surfaces, with node-shell adapters only passing messages and consuming actions.

### Announcement Policy

- **D-05:** Compact block announcements are allowed only when all gates pass: local compact-relay activation, peer compact capability, high-bandwidth preference when announcing `cmpctblock`, known header continuity or acceptable tip context, validated local block availability, and resource capacity.
- **D-06:** When any compact gate fails, the policy should choose an explicit fallback action such as headers, inventory, or suppress, with a stable low-cardinality reason. Fallback must be a typed outcome so later operator evidence can summarize it without renderer-local inference.
- **D-07:** `cmpctblock` announcements should remain announcement-only in this phase. Full compact-block reconstruction, missing transaction scheduling, `getblocktxn`, `blocktxn`, and validation/connect handoff remain deferred to Phases 114 and 115.
- **D-08:** Resource gates should reuse Phase 110/111 block-serving request and in-flight policy concepts where they fit, but compact-announcement decisions should have their own labels when that avoids mixing full-block serving and compact-relay evidence.

### Scope Isolation

- **D-09:** Compact relay negotiation must remain independent from transaction relay activation, package relay, bloom/filter permissions, compact filters, public serving defaults, production-service operation, and production full-node readiness.
- **D-10:** Transaction relay or mempool participation may provide future reconstruction inputs, but in Phase 113 they must not be prerequisites for negotiation state or accidental activators for compact announcements.
- **D-11:** Peer permissions such as `download`, protected admission, inbound serving, and transaction relay eligibility may be policy inputs only where prior phases already made them scoped and bounded. They must not grant compact relay, package relay, archive serving, or public defaults by implication.

### Verification And Parity

- **D-12:** Tests must cover valid version 2 `sendcmpct`, unsupported versions, high-bandwidth toggles, low-bandwidth preference, default-disabled suppression, headers/inventory fallback, missing header or unavailable block suppression, and transaction-relay/package-relay isolation.
- **D-13:** New or touched first-party Rust source/test files under `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests` require parity breadcrumbs and `docs/parity/source-breadcrumbs.json` entries unless an explicit `none` breadcrumb is defensible.
- **D-14:** Verification remains deterministic and local through repo-native checks and `bash scripts/verify.sh`. Public-network compact-relay review remains opt-in UAT evidence only.

### Claude's Discretion

The planner may choose exact Rust type names, whether compact negotiation lives in a new `compact_relay` peer module or an existing touched peer module, and how fallback actions are named. Prefer small pure policy APIs, low-cardinality reasons, and tests that make accidental public/default/package/filter coupling impossible.

### Deferred Ideas (OUT OF SCOPE)

Compact-block reconstruction from mempool state, short-ID matching, missing transaction request scheduling, `blocktxn` response matching, fallback to full block fetch, validation/connect handoff, broad operator/RPC/CLI/dashboard/metrics/log/support evidence rollout, parity/UAT release closeout, package relay, bloom/filter serving, compact filter serving, public serving defaults, public-network CI, archive-node claims, production full-node readiness, production-service operation, and production-funds wallet use remain outside Phase 113.
</user_constraints>

<required_contract>
## Revision Contract

Phase 113 plans must apply this exact compact relay negotiation and announcement contract:

1. Supported `sendcmpct` version 2 is the only input that updates the peer's supported compact relay preference.
2. `sendcmpct { version: 2, announce: true }` records supported v2 high-bandwidth preference: high-bandwidth requested, low-bandwidth not requested.
3. `sendcmpct { version: 2, announce: false }` records supported v2 low-bandwidth preference: low-bandwidth requested, high-bandwidth not requested.
4. Unsupported `sendcmpct` versions record `last_unsupported_version` or equivalent evidence but do not overwrite the last supported v2 capability/preference. If no supported v2 preference exists, announcement policy returns unsupported/not-negotiated suppression. If a supported v2 preference exists, announcement policy continues using the last supported preference while retaining unsupported-version evidence.
5. `sendcmpct` handling itself must not update stored `CompactAnnouncementEligibility`; eligibility is refreshed only when `record_announcement_decision(&decision)` or equivalent stores the result of the latest `decide_compact_announcement` call.
6. Tests must prove this sequence: v2 high -> decision recorded as `Eligible`; v2 low -> decision recorded as `HighBandwidthNotRequested` or equivalent non-eligible state; v2 high again -> decision recorded as `Eligible` again.
7. Tests must prove v2 high -> unsupported v1/v3 -> subsequent decision still uses high-bandwidth v2 preference, while unsupported evidence remains available.
</required_contract>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| CMP-04 | Node tracks per-peer compact-block capability, high-bandwidth preference, low-bandwidth preference, and compact-block announcement eligibility deterministically. [VERIFIED: .planning/REQUIREMENTS.md] | Use a typed `CompactRelayPeerState` updated by `SendCompactMessage`, with supported version 2 as the only preference-updating capability signal, unsupported versions recorded as evidence only, and announcement eligibility refreshed only from recorded announcement decisions. [VERIFIED: packages/open-bitcoin-codec/src/compact_block.rs; packages/bitcoin-knots/src/net_processing.cpp] |
| CMP-05 | Node announces compact blocks only when activation, peer negotiation, header state, block availability, and resource limits permit it. [VERIFIED: .planning/REQUIREMENTS.md] | Add a pure announcement decision that composes `CompactRelayActivationConfig`, per-peer high-bandwidth state, header continuity facts, block-serving status/data availability, and bounded request/resource inputs. [VERIFIED: packages/open-bitcoin-network/src/block_serving.rs; packages/bitcoin-knots/src/net_processing.cpp] |
| CMP-06 | Compact-block negotiation remains independent from transaction relay, package relay, bloom/filter permissions, compact filters, and public serving defaults. [VERIFIED: .planning/REQUIREMENTS.md] | Keep compact relay state separate from `TxRelayPeerMode`, relay activation, filter permissions, and public service advertisement, with tests proving those surfaces do not activate compact announcements. [VERIFIED: packages/open-bitcoin-network/src/peer/transaction_relay.rs; packages/open-bitcoin-network/src/block_serving/tests.rs] |
</phase_requirements>

## Summary

Phase 113 should be planned as a pure network-policy phase, not as a node-shell serving or reconstruction phase. [VERIFIED: .planning/phases/113-compact-relay-negotiation-and-announcement-policy/113-CONTEXT.md] The existing code already has BIP152 wire types, explicit `CompactRelayActivationConfig`, block-serving status/resource gates, peer header preference, and transaction-relay state, so the plan should wire those through typed compact relay state instead of adding new dependencies or public runtime surfaces. [VERIFIED: packages/open-bitcoin-codec/src/compact_block.rs; packages/open-bitcoin-network/src/block_serving.rs; packages/open-bitcoin-network/src/peer.rs; packages/open-bitcoin-network/src/peer/transaction_relay.rs]

Bitcoin Knots treats `sendcmpct` version 2 as the supported compact-block version in this baseline, records whether the peer requested high-bandwidth compact blocks, sends low-bandwidth `sendcmpct` after handshake, and only pushes `cmpctblock` announcements when high-bandwidth was requested and the peer has the prior header but not the new header. [VERIFIED: packages/bitcoin-knots/src/net_processing.cpp] BIP152 defines high-bandwidth mode as `sendcmpct` boolean `1`, low-bandwidth mode as boolean `0`, `MSG_CMPCT_BLOCK == 4`, and version 2 as the witness-aware compact-block encoding. [CITED: https://raw.githubusercontent.com/bitcoin/bips/master/bip-0152.mediawiki]

**Primary recommendation:** Add a small `open-bitcoin-network` compact relay policy/state surface, update `PeerState` from `WireNetworkMessage::SendCompact`, and route `announce_block` through a typed decision that returns `CompactBlock`, `Headers`, `Inv`, or `Suppress` with fixed reasons. [VERIFIED: packages/open-bitcoin-network/src/peer.rs; packages/open-bitcoin-network/src/message.rs]

## Project Constraints

- No `.cursor/rules/` files or project-local `.cursor/skills/` / `.agents/skills/` indexes were found, so there are no additional Cursor-rule directives to enforce beyond repo and Bright Builds guidance. [VERIFIED: Glob .cursor/rules/**, .cursor/skills/**/SKILL.md, .agents/skills/**/SKILL.md]
- New or touched first-party Rust source/test files under `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests` must include parity breadcrumbs and registry coverage in `docs/parity/source-breadcrumbs.json`. [VERIFIED: AGENTS.md; docs/parity/source-breadcrumbs.json]
- Pure Bitcoin domain behavior should stay in functional-core crates and effects should stay in adapters; Phase 113 therefore belongs primarily in `open-bitcoin-network`, with `open-bitcoin-node` consuming decisions later only where needed. [VERIFIED: AGENTS.md; standards/core/architecture.md; standards/languages/rust.md]
- Verification should use the repo-native `bash scripts/verify.sh`; `--fast` is only for local iteration and the default command is the release/pre-commit contract. [VERIFIED: AGENTS.md; standards/core/verification.md]
- Rust code should avoid `unwrap()`, prefer `let...else` where it clarifies guards, name optional internals with `maybe_`, and keep unit tests focused with Arrange/Act/Assert structure. [VERIFIED: standards/languages/rust.md; standards/core/code-shape.md; standards/core/testing.md]

## Standard Stack

### Core

| Library / Surface | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| Rust workspace crates under `packages/open-bitcoin-*` | Rust `1.94.1` / Rust 2024 [VERIFIED: rust-toolchain.toml guidance in AGENTS.md; local `rustc --version`] | Implement typed peer state and pure announcement policy. | This is the pinned project toolchain and production code path. [VERIFIED: AGENTS.md] |
| `open-bitcoin-codec::SendCompactMessage` and `BIP152_COMPACT_BLOCKS_VERSION` | In-repo constant `2` [VERIFIED: packages/open-bitcoin-codec/src/compact_block.rs] | Consume decoded BIP152 `sendcmpct` payloads. | Phase 112 already owns encoding/decoding, including unsupported versions as data. [VERIFIED: packages/open-bitcoin-codec/src/compact_block.rs; packages/open-bitcoin-network/src/message/tests.rs] |
| `open-bitcoin-network::BlockRelayActivationPolicy` and `CompactRelayActivationConfig` | In-repo policy type [VERIFIED: packages/open-bitcoin-network/src/block_serving.rs] | Gate local compact-relay activation separately from block serving and transaction relay. | Existing Phase 110 activation surface already defaults compact relay to disabled. [VERIFIED: packages/open-bitcoin-network/src/block_serving/tests.rs] |
| `open-bitcoin-network::PeerManager` / `PeerState` | In-repo peer state [VERIFIED: packages/open-bitcoin-network/src/peer.rs] | Store per-peer compact relay capability and announce blocks. | `PeerState` already owns negotiated peer flags like `remote_wtxidrelay` and `remote_prefers_headers`. [VERIFIED: packages/open-bitcoin-network/src/peer.rs] |

### Supporting

| Surface | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| `BlockServingStatusFacts` / `classify_block_serving_status` | In-repo policy [VERIFIED: packages/open-bitcoin-network/src/block_serving.rs] | Represent active/recent/stale/pruned/unavailable/validated facts before payload lookup. | Use when deciding whether a local block is eligible for compact announcement. [VERIFIED: packages/open-bitcoin-node/src/network/block_serving.rs] |
| `ResourceGovernancePolicy` inputs | In-repo policy [VERIFIED: packages/open-bitcoin-network/src/block_serving.rs; packages/open-bitcoin-network/src/resource.rs] | Enforce bounded queue/request/in-flight pressure. | Use as an input to compact announcement resource gating without mixing labels with full-block serving. [VERIFIED: .planning/phases/113-compact-relay-negotiation-and-announcement-policy/113-CONTEXT.md] |
| `HeaderStore` / peer header facts | In-repo header state [VERIFIED: packages/open-bitcoin-network/src/peer.rs; packages/open-bitcoin-network/src/header_store.rs] | Determine whether the peer has prior header continuity and whether headers fallback is appropriate. | Use to model the Knots `PeerHasHeader(previous)` and `!PeerHasHeader(current)` guard at policy boundaries. [VERIFIED: packages/bitcoin-knots/src/net_processing.cpp] |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| New pure `compact_relay` policy under `open-bitcoin-network` | Inline booleans in `PeerState` and `announce_block` | Inline booleans are smaller initially but violate the locked typed-state decision and make fallback reasons harder to test. [VERIFIED: 113-CONTEXT.md; standards/core/architecture.md] |
| Reusing transaction-relay scheduler types | Existing `TxDownloadScheduler` and `TxRelayPeerMode` | Transaction relay provides a pattern but must not activate, suppress, or schedule compact relay in this phase. [VERIFIED: 113-CONTEXT.md; packages/open-bitcoin-network/src/peer/transaction_relay.rs] |
| Building compact payloads during announcement policy | `CompactBlockPayload` construction | Payload construction depends on short IDs/reconstruction-era details and is out of scope until later compact-block phases. [VERIFIED: 113-CONTEXT.md; packages/open-bitcoin-codec/src/compact_block.rs] |

**Installation:** No new package or crate dependency should be added for Phase 113. [VERIFIED: existing in-repo Rust surfaces cover the needed codec, peer state, block-serving policy, and tests]

**Version verification:** `rustc 1.94.1`, `cargo 1.94.1`, Bazelisk `1.28.1` / Bazel `8.6.0`, Bun `1.3.9`, and GNU Bash `3.2.57` are available locally. [VERIFIED: local environment audit command]

## Architecture Patterns

### Recommended Project Structure

```text
packages/open-bitcoin-network/src/
├── peer.rs                         # PeerManager state and high-level message routing [VERIFIED: existing file]
├── peer/
│   ├── compact_relay.rs            # New pure compact negotiation + announcement policy [RECOMMENDED: standards/languages/rust.md]
│   ├── inventory_state.rs          # Existing inventory/getdata/header behavior [VERIFIED: existing file]
│   └── transaction_relay.rs        # Existing tx relay pattern to keep separate [VERIFIED: existing file]
└── block_serving.rs                # Existing activation/status/resource concepts to compose [VERIFIED: existing file]
```

### Pattern 1: Typed `sendcmpct` State

**What:** Represent capability and preference as enums/structs, for example `CompactRelayPeerState { capability, high_bandwidth, low_bandwidth, maybe_unsupported_version, announcement_eligibility }`, rather than scattered booleans. Supported version 2 is the only input that updates supported preference; unsupported versions are evidence-only and must not overwrite the last supported version 2 capability/preference. [VERIFIED: 113-CONTEXT.md; standards/core/architecture.md]  
**When to use:** Use this state whenever `PeerManager::handle_message` receives `WireNetworkMessage::SendCompact`. [VERIFIED: packages/open-bitcoin-network/src/peer.rs; packages/open-bitcoin-network/src/message.rs]  
**Example:**

```rust
// Source: packages/bitcoin-knots/src/net_processing.cpp and packages/open-bitcoin-codec/src/compact_block.rs
// Recommended shape only; planner should choose final names.
pub fn apply_send_compact(
    state: CompactRelayPeerState,
    message: SendCompactMessage,
) -> CompactRelayPeerState {
    if message.version != BIP152_COMPACT_BLOCKS_VERSION {
        return state.with_unsupported_evidence(message.version);
    }

    state.with_supported_v2_preference(message.announce)
}
```

### Pattern 2: Pure Announcement Decision

**What:** Create a pure data-in/data-out decision function that returns a typed action and reason, such as `AnnounceCompactBlock`, `AnnounceHeaders`, `AnnounceInventory`, or `Suppress`. [VERIFIED: standards/core/architecture.md; 113-CONTEXT.md]  
**When to use:** Route `PeerManager::announce_block` through this function so tests can exercise every gate without storage, sockets, metrics, or mempool state. [VERIFIED: packages/open-bitcoin-network/src/peer.rs]  
**Example:**

```rust
// Source: packages/bitcoin-knots/src/net_processing.cpp; standards/core/architecture.md
pub fn decide_compact_announcement(input: CompactAnnouncementInput) -> CompactAnnouncementDecision {
    if !input.activation.compact_relay.enabled {
        return CompactAnnouncementDecision::fallback_inventory("compact_relay_disabled");
    }

    if !input.peer_state.high_bandwidth_requested() {
        return CompactAnnouncementDecision::fallback_headers("compact_high_bandwidth_not_requested");
    }

    if !input.peer_has_previous_header || input.peer_has_current_header {
        return CompactAnnouncementDecision::fallback_headers("compact_header_continuity_missing");
    }

    if !input.block_available || input.resource_limited {
        return CompactAnnouncementDecision::suppress("compact_block_unavailable");
    }

    CompactAnnouncementDecision::compact_block()
}
```

### Pattern 3: Headers/Inventory Fallback Remains Explicit

**What:** Keep the current `announce_block` fallback behavior visible: peers preferring headers receive `headers`; otherwise peers receive `inv`. [VERIFIED: packages/open-bitcoin-network/src/peer.rs]  
**When to use:** When compact gates fail for negotiation, activation, or header-state reasons, prefer a typed fallback rather than returning `None` unless the failure is a true suppression such as unavailable block or resource limit. [VERIFIED: 113-CONTEXT.md; packages/bitcoin-knots/test/functional/p2p_compactblocks.py]

### Anti-Patterns to Avoid

- **Deriving compact eligibility from transaction relay:** `remote_wtxidrelay`, relay activation, or mempool presence must not imply compact-block capability. [VERIFIED: 113-CONTEXT.md; packages/open-bitcoin-network/src/peer/transaction_relay.rs]
- **Treating unsupported `sendcmpct` as disconnect-worthy or preference-clearing in Phase 113:** Knots ignores unsupported compact versions in this path, and the phase requires stable unsupported evidence without overwriting the last supported version 2 preference. [VERIFIED: packages/bitcoin-knots/src/net_processing.cpp; 113-CONTEXT.md]
- **Building or reconstructing compact blocks in the policy:** Phase 113 is announcement policy only; reconstruction, `getblocktxn`, and `blocktxn` are deferred. [VERIFIED: 113-CONTEXT.md]
- **Adding public/operator rollout claims:** Metrics, RPC/CLI/dashboard evidence, public serving defaults, and release boundary closeout belong to later phases. [VERIFIED: .planning/ROADMAP.md; .planning/REQUIREMENTS.md]

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| BIP152 wire parsing | A second `sendcmpct` parser or manual byte checks | `SendCompactMessage` decoded by `WireNetworkMessage::SendCompact` | Phase 112 already validates payload length and preserves unsupported versions as data. [VERIFIED: packages/open-bitcoin-codec/src/compact_block.rs; packages/open-bitcoin-network/src/message/tests.rs] |
| Compact-block version constants | Magic number `2` at call sites | `BIP152_COMPACT_BLOCKS_VERSION` | Keeps version support centralized and aligned with existing codec tests. [VERIFIED: packages/open-bitcoin-codec/src/compact_block.rs] |
| Resource caps | New ad hoc caps for announcement eligibility | Existing resource governance and block-serving request/in-flight concepts, with compact-specific outcome labels | Prior phases already modeled request pressure and cleanup; Phase 113 only needs compact-specific policy labels where semantics differ. [VERIFIED: packages/open-bitcoin-network/src/block_serving.rs; 113-CONTEXT.md] |
| Transaction relay coupling | Reusing `TxDownloadScheduler` or `TxRelayPeerMode` as compact capability | A separate compact relay peer state | CMP-06 requires independence from transaction relay and package relay. [VERIFIED: .planning/REQUIREMENTS.md; packages/open-bitcoin-network/src/peer/transaction_relay.rs] |
| Public/default activation | Service-bit or permission side effects | Existing explicit `CompactRelayActivationConfig` default-off gate | Compact relay is already represented as explicit activation and defaults disabled. [VERIFIED: packages/open-bitcoin-network/src/block_serving.rs; packages/open-bitcoin-network/src/block_serving/tests.rs] |

**Key insight:** The difficult part of Phase 113 is not parsing messages or constructing compact payloads; it is preventing accidental activation through neighboring relay, permission, filter, or public-serving surfaces while preserving Knots-compatible high-bandwidth announcement semantics. [VERIFIED: 113-CONTEXT.md; packages/bitcoin-knots/src/net_processing.cpp]

## Common Pitfalls

### Pitfall 1: Conflating Capability With Eligibility

**What goes wrong:** A peer that sent `sendcmpct(announce=true, version=2)` is treated as immediately eligible for compact announcements. [VERIFIED: 113-CONTEXT.md]  
**Why it happens:** Knots stores compact capability and high-bandwidth request state, but announcement still checks header continuity and block availability before sending `cmpctblock`. [VERIFIED: packages/bitcoin-knots/src/net_processing.cpp]  
**How to avoid:** Model capability, preference, and announcement eligibility as separate fields/decision steps. [VERIFIED: standards/core/architecture.md; 113-CONTEXT.md]  
**Warning signs:** Tests only check `sendcmpct` updates one boolean and do not cover default-disabled, header-missing, or unavailable-block fallback. [VERIFIED: 113-CONTEXT.md]

### Pitfall 2: Unsupported Versions Accidentally Clear Valid Version 2 State

**What goes wrong:** After valid `sendcmpct(1,2)`, a later unsupported `sendcmpct(0,1)` disables compact announcements. [VERIFIED: packages/bitcoin-knots/test/functional/p2p_compactblocks.py]  
**Why it happens:** Code treats every decoded `sendcmpct` as authoritative instead of ignoring unsupported versions for capability. [VERIFIED: packages/bitcoin-knots/src/net_processing.cpp]  
**How to avoid:** Record unsupported-version observations as evidence without overwriting the last supported version 2 preference. [VERIFIED: packages/bitcoin-knots/test/functional/p2p_compactblocks.py; 113-CONTEXT.md]  
**Warning signs:** The test matrix lacks "supported high-bandwidth version 2, then unsupported version 1 or 3, still uses the supported high-bandwidth preference while retaining unsupported evidence." [VERIFIED: packages/bitcoin-knots/test/functional/p2p_compactblocks.py]

### Pitfall 3: Header Fallback Loses Existing `sendheaders` Behavior

**What goes wrong:** Compact fallback always emits inventory or suppresses, ignoring peers that prefer headers. [VERIFIED: packages/open-bitcoin-network/src/peer.rs]  
**Why it happens:** The new compact policy bypasses existing `remote_prefers_headers` handling. [VERIFIED: packages/open-bitcoin-network/src/peer.rs]  
**How to avoid:** Make fallback action explicit and preserve `Headers` before `Inv` where existing peer preference requires it. [VERIFIED: packages/open-bitcoin-network/src/peer.rs; packages/bitcoin-knots/test/functional/p2p_compactblocks.py]  
**Warning signs:** Existing `announce_block` tests for `SendHeaders` are rewritten instead of extended. [VERIFIED: packages/open-bitcoin-network/src/peer/tests.rs]

### Pitfall 4: Compact Relay Starts Serving Compact `getdata`

**What goes wrong:** Announcement support accidentally responds to `MSG_CMPCT_BLOCK` getdata with compact payloads. [VERIFIED: packages/open-bitcoin-node/src/network/inventory.rs]  
**Why it happens:** Both direct announcements and `MSG_CMPCT_BLOCK` responses use `cmpctblock`, but Phase 113 only covers announcement policy. [VERIFIED: 113-CONTEXT.md; BIP152 cited source]  
**How to avoid:** Keep inventory-serving suppression from Phase 111 unchanged and add guard tests if touched. [VERIFIED: packages/open-bitcoin-node/src/network/block_serving.rs; packages/open-bitcoin-node/src/network/inventory.rs]  
**Warning signs:** New tests expect `handle_getdata(CompactBlock)` to emit `WireNetworkMessage::CompactBlock`. [VERIFIED: packages/open-bitcoin-node/src/network/inventory.rs]

### Pitfall 5: Scope Leakage Through Permissions Or Public Defaults

**What goes wrong:** `download`, protected inbound, relay activation, package/filter flags, or public-serving status grant compact announcements. [VERIFIED: 113-CONTEXT.md]  
**Why it happens:** Prior block-serving and relay eligibility surfaces are reused without compact-specific activation checks. [VERIFIED: packages/open-bitcoin-network/src/block_serving.rs; packages/open-bitcoin-network/src/relay.rs]  
**How to avoid:** Put `compact_relay.enabled` and `CompactRelayPeerState` ahead of all permission-derived eligibility in the decision chain. [VERIFIED: 113-CONTEXT.md; packages/open-bitcoin-network/src/block_serving/tests.rs]  
**Warning signs:** Compact announcement tests pass only when transaction relay or inbound serving is enabled. [VERIFIED: .planning/REQUIREMENTS.md]

## Code Examples

### Knots `sendcmpct` Handling

```cpp
// Source: packages/bitcoin-knots/src/net_processing.cpp
if (msg_type == NetMsgType::SENDCMPCT) {
    bool sendcmpct_hb{false};
    uint64_t sendcmpct_version{0};
    vRecv >> sendcmpct_hb >> sendcmpct_version;

    // Only support compact block relay with witnesses
    if (sendcmpct_version != CMPCTBLOCKS_VERSION) return;

    LOCK(cs_main);
    CNodeState* nodestate = State(pfrom.GetId());
    nodestate->m_provides_cmpctblocks = true;
    nodestate->m_requested_hb_cmpctblocks = sendcmpct_hb;
    pfrom.m_bip152_highbandwidth_from = sendcmpct_hb;
    return;
}
```

### Knots Compact Announcement Gate

```cpp
// Source: packages/bitcoin-knots/src/net_processing.cpp
if (state.m_requested_hb_cmpctblocks && !PeerHasHeader(&state, pindex) && PeerHasHeader(&state, pindex->pprev)) {
    const CSerializedNetMsg& ser_cmpctblock{lazy_ser.get()};
    PushMessage(*pnode, ser_cmpctblock.Copy());
    state.pindexBestHeaderSent = pindex;
}
```

### Open Bitcoin Existing Fallback Baseline

```rust
// Source: packages/open-bitcoin-network/src/peer.rs
if peer.remote_prefers_headers {
    return Ok(Some(WireNetworkMessage::Headers(HeadersMessage {
        headers: vec![block.header.clone()],
    })));
}
Ok(Some(WireNetworkMessage::Inv(InventoryList::new(vec![
    InventoryVector {
        inventory_type: InventoryType::Block,
        object_hash: block_hash.into(),
    },
]))))
```

## State of the Art

| Old / Adjacent Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| BIP152 version 1 uses txid-based compact blocks without witness serialization. [CITED: https://raw.githubusercontent.com/bitcoin/bips/master/bip-0152.mediawiki] | Version 2 uses witness-aware compact block encoding and wtxid short IDs. [CITED: https://raw.githubusercontent.com/bitcoin/bips/master/bip-0152.mediawiki] | BIP152 version 2 section. [CITED: https://raw.githubusercontent.com/bitcoin/bips/master/bip-0152.mediawiki] | Phase 113 should only treat version 2 as supported because Phase 112 and Knots baseline use version 2. [VERIFIED: packages/open-bitcoin-codec/src/compact_block.rs; packages/bitcoin-knots/src/net_processing.cpp] |
| Low-bandwidth compact relay uses normal inv/headers announcements followed by `MSG_CMPCT_BLOCK` requests. [CITED: https://raw.githubusercontent.com/bitcoin/bips/master/bip-0152.mediawiki] | High-bandwidth relay uses direct `cmpctblock` announcements after `sendcmpct(1, version)`. [CITED: https://raw.githubusercontent.com/bitcoin/bips/master/bip-0152.mediawiki] | BIP152 intended protocol flow. [CITED: https://raw.githubusercontent.com/bitcoin/bips/master/bip-0152.mediawiki] | Phase 113 should store both preferences but only direct-announce `cmpctblock` on high-bandwidth preference and other gates. [VERIFIED: 113-CONTEXT.md; packages/bitcoin-knots/src/net_processing.cpp] |
| Compact block request/response and reconstruction are part of the full BIP152 protocol. [CITED: https://raw.githubusercontent.com/bitcoin/bips/master/bip-0152.mediawiki] | Phase 113 only implements negotiation and announcement decisions. [VERIFIED: 113-CONTEXT.md] | Project roadmap split phases 113-115. [VERIFIED: .planning/ROADMAP.md] | Planner should avoid `getblocktxn`, `blocktxn`, short-ID matching, and validation handoff tasks in this phase. [VERIFIED: .planning/ROADMAP.md; 113-CONTEXT.md] |

**Deprecated/outdated:** Treating version 1 `sendcmpct` as supported is out of scope for this Knots baseline and Open Bitcoin phase because both the local constant and Knots baseline support version 2 for witness compact blocks. [VERIFIED: packages/open-bitcoin-codec/src/compact_block.rs; packages/bitcoin-knots/src/net_processing.cpp]

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|

All claims in this research were verified against local repo files, the pinned Bitcoin Knots submodule, local environment probes, or the BIP152 text fetched during this session; no `[ASSUMED]` claims are intentionally present. [VERIFIED: source list below]

## Open Questions (RESOLVED)

1. **RESOLVED: Compact negotiation state should live in a new focused `peer/compact_relay.rs` file.**  
   - What we know: The context allows either, and Rust standards prefer named module files for new module surfaces. [VERIFIED: 113-CONTEXT.md; standards/languages/rust.md]  
   - Resolution: Use `packages/open-bitcoin-network/src/peer/compact_relay.rs` for compact relay negotiation state, announcement eligibility, action/reason types, and pure policy helpers, while keeping `peer.rs` responsible for stored peer lookup and message/decision routing only. [VERIFIED: packages/open-bitcoin-network/src/peer.rs; standards/core/code-shape.md]  
   - Why this path: The phase needs more than simple state setters because it must model capability, high-/low-bandwidth preferences, eligibility, fixed reasons, fallback actions, and tests without growing `peer.rs` further. [VERIFIED: 113-CONTEXT.md; .planning/REQUIREMENTS.md]

2. **RESOLVED: Phase 113 should expose typed compact announcement decisions, not node-shell compact payload construction.**  
   - What we know: The phase goal is pure negotiation and announcement policy, and broad operator/RPC/metrics rollout is deferred. [VERIFIED: 113-CONTEXT.md; .planning/ROADMAP.md]  
   - Resolution: Keep adapter impact minimal by returning typed `CompactAnnouncementDecision` values from `open-bitcoin-network` and recording derived `CompactAnnouncementEligibility` on `CompactRelayPeerState`; do not build `WireNetworkMessage::CompactBlock` payloads in this phase. [VERIFIED: packages/open-bitcoin-network/src/peer.rs; packages/open-bitcoin-network/src/message.rs]  
   - Why this path: Compact payload construction requires short IDs, reconstruction-era transaction inputs, and missing-transaction/fallback behavior that is explicitly deferred to Phases 114 and 115. [VERIFIED: 113-CONTEXT.md; packages/open-bitcoin-node/src/network/block_serving.rs]

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| Rust / Cargo | Rust source changes and tests | yes [VERIFIED: local environment audit] | `rustc 1.94.1`, `cargo 1.94.1` [VERIFIED: local environment audit] | None needed. [VERIFIED: AGENTS.md] |
| Bazelisk / Bazel | Repo-native smoke build through `scripts/verify.sh` | yes [VERIFIED: local environment audit] | Bazelisk `1.28.1`, Bazel `8.6.0` [VERIFIED: local environment audit] | None needed. [VERIFIED: AGENTS.md] |
| Bun | Repo-owned TypeScript automation and parity check scripts | yes [VERIFIED: local environment audit] | `1.3.9` [VERIFIED: local environment audit] | None needed. [VERIFIED: AGENTS.md] |
| Bash | `scripts/verify.sh` and thin orchestration | yes [VERIFIED: local environment audit] | GNU Bash `3.2.57` [VERIFIED: local environment audit] | None needed. [VERIFIED: AGENTS.md] |

**Missing dependencies with no fallback:** None found during the local audit. [VERIFIED: local environment audit]  
**Missing dependencies with fallback:** None found during the local audit. [VERIFIED: local environment audit]

## Security Domain

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|-----------------|
| V2 Authentication | no | This phase does not add user authentication or credentials. [VERIFIED: 113-CONTEXT.md] |
| V3 Session Management | no | This phase does not add web sessions or login state. [VERIFIED: 113-CONTEXT.md] |
| V4 Access Control | yes | Preserve explicit relay/block-serving/permission boundaries and do not let `download`, transaction relay, package relay, bloom/filter, compact filters, or public defaults grant compact announcements. [VERIFIED: 113-CONTEXT.md; packages/open-bitcoin-network/src/block_serving/tests.rs] |
| V5 Input Validation | yes | Parse wire data through existing codec/domain types and map unsupported versions to typed outcomes. [VERIFIED: packages/open-bitcoin-codec/src/compact_block.rs; packages/open-bitcoin-network/src/message.rs] |
| V6 Cryptography | limited | Do not add new cryptographic primitives in Phase 113; short-ID and witness-hash reconstruction work is deferred. [VERIFIED: 113-CONTEXT.md; .planning/ROADMAP.md] |

### Known Threat Patterns for Compact Relay Negotiation

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Resource exhaustion through announcement or request pressure | Denial of Service | Reuse bounded request/resource gates and add compact-specific suppression labels. [VERIFIED: packages/open-bitcoin-network/src/block_serving.rs; 113-CONTEXT.md] |
| Protocol confusion from unsupported compact versions | Tampering | Preserve unsupported `sendcmpct` as decoded evidence but do not mark the peer compact capable when no supported version 2 preference exists and do not overwrite the last supported version 2 preference when one exists. [VERIFIED: packages/open-bitcoin-codec/src/compact_block.rs; packages/bitcoin-knots/src/net_processing.cpp] |
| Capability escalation through transaction relay or permissions | Elevation of Privilege | Keep compact relay activation and peer compact capability separate from transaction relay, package relay, filters, and public serving defaults. [VERIFIED: 113-CONTEXT.md; .planning/REQUIREMENTS.md] |
| Information leakage through raw peer or transaction details in reasons | Information Disclosure | Use fixed low-cardinality reasons and defer operator evidence to Phase 116. [VERIFIED: 113-CONTEXT.md; .planning/ROADMAP.md] |

## Validation Guidance

Nyquist validation is disabled in `.planning/config.json`, so no separate Validation Architecture section is required. [VERIFIED: .planning/config.json] Planner tasks should still include focused Rust unit tests because the changed behavior is pure business logic. [VERIFIED: standards/core/testing.md]

Recommended targeted test areas: valid version 2 `sendcmpct`, unsupported version 1/3 evidence-only behavior, high-bandwidth enable/disable toggles, high -> low -> high recorded eligibility refresh, high -> unsupported -> high-bandwidth preference preservation, low-bandwidth capability without direct compact announcement, default-disabled compact relay suppression, headers and inventory fallback, unavailable/missing-header suppression, and transaction-relay/package/filter/public-default isolation. [VERIFIED: 113-CONTEXT.md; packages/bitcoin-knots/test/functional/p2p_compactblocks.py]

Recommended verification command for phase completion is `bash scripts/verify.sh`; targeted local iteration may use Rust crate tests first, but the repo-native command is the completion gate. [VERIFIED: AGENTS.md; standards/core/verification.md]

## Sources

### Primary (HIGH confidence)

- `AGENTS.md` - repo-local verification, parity breadcrumbs, Rust pin, and GSD workflow guidance. [VERIFIED: ReadFile]
- `AGENTS.bright-builds.md` - Bright Builds workflow and cross-cutting architecture/testing/verification rules. [VERIFIED: ReadFile]
- `standards/core/architecture.md`, `standards/core/code-shape.md`, `standards/core/testing.md`, `standards/core/verification.md`, `standards/languages/rust.md` - functional core, typed domain state, guard style, test shape, and verification expectations. [VERIFIED: ReadFile]
- `.planning/phases/113-compact-relay-negotiation-and-announcement-policy/113-CONTEXT.md` - locked scope and phase decisions. [VERIFIED: ReadFile]
- `.planning/REQUIREMENTS.md` and `.planning/ROADMAP.md` - CMP-04/CMP-05/CMP-06 and phase split. [VERIFIED: ReadFile]
- `packages/open-bitcoin-codec/src/compact_block.rs` - BIP152 payload types and version constant. [VERIFIED: ReadFile]
- `packages/open-bitcoin-network/src/message.rs`, `packages/open-bitcoin-network/src/peer.rs`, `packages/open-bitcoin-network/src/block_serving.rs` - wire message surface, peer state/announcement surface, and block-serving gates. [VERIFIED: ReadFile]
- `packages/open-bitcoin-node/src/network/block_serving.rs` and `packages/open-bitcoin-node/src/network/inventory.rs` - node-shell serving suppression and adapter boundary. [VERIFIED: ReadFile]
- `packages/bitcoin-knots/src/net_processing.cpp` and `packages/bitcoin-knots/test/functional/p2p_compactblocks.py` - Knots compact negotiation and announcement behavior anchors. [VERIFIED: ReadFile]

### Secondary (MEDIUM confidence)

- BIP152 raw mediawiki text - official protocol wording for high/low bandwidth modes, message semantics, `MSG_CMPCT_BLOCK`, and version 2 witness behavior. [CITED: https://raw.githubusercontent.com/bitcoin/bips/master/bip-0152.mediawiki]

### Tertiary (LOW confidence)

- None. [VERIFIED: no unverified web-only sources were used]

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - the phase uses existing in-repo Rust types and no new external library. [VERIFIED: packages/open-bitcoin-codec/src/compact_block.rs; packages/open-bitcoin-network/src/block_serving.rs]
- Architecture: HIGH - project and phase decisions explicitly require pure `open-bitcoin-network` state/policy and typed outcomes. [VERIFIED: 113-CONTEXT.md; standards/core/architecture.md]
- Pitfalls: HIGH - pitfalls are grounded in Knots behavior, Phase 112/110/111 existing code, and explicit CMP-04/CMP-05/CMP-06 constraints. [VERIFIED: packages/bitcoin-knots/src/net_processing.cpp; packages/bitcoin-knots/test/functional/p2p_compactblocks.py; .planning/REQUIREMENTS.md]

**Research date:** 2026-07-04 [VERIFIED: system timestamp]  
**Valid until:** 2026-08-03 for project-internal planning assumptions, unless Phase 112/113 code or the pinned Knots baseline changes first. [VERIFIED: AGENTS.md; 113-CONTEXT.md]
