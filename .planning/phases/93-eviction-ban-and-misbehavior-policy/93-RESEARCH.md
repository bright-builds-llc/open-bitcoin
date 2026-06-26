# Phase 93: Eviction, Ban, and Misbehavior Policy - Research

**Researched:** 2026-06-26  
**Domain:** Bitcoin inbound peer eviction scoring, ban/discourage state, and bounded misbehavior responses  
**Confidence:** HIGH

<user_constraints>
## Locked Decisions From Context

- Eviction scoring must be pure and deterministic, using typed peer records, connection class, slot class, handshake state, activity evidence, diversity/address evidence, and Phase 91 permission effects.
- Protected peers from `forceinbound` and `noban` must not be accidentally evicted or banned; no-action outcomes still need evidence.
- Ban/discourage state must be typed, scoped to address or subnet, expiry-aware, manually reversible, and evaluated from injected timestamps.
- Misbehavior accounting should map named protocol violations to bounded responses: observe-only, disconnect, discourage, ban, and protected/no-action.
- Status, support, metrics, logs, and docs must use stable low-cardinality labels and avoid raw peer IDs, endpoint tables, permission class names, config strings, credentials, or unbounded ban tables.
- Phase 94 keeps broader resource governance: queue pressure, payload-size limits, slow handshakes, churn, reconnect throttling, and resource pressure.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
| --- | --- | --- |
| EVICT-01 | Deterministic inbound peer scoring from connection class, handshake progress, diversity/activity, and permissions. | Add a pure policy module in `open-bitcoin-network` that scores `PeerState` plus typed evidence and emits stable score components. |
| EVICT-02 | Disconnect or evict peers for caps/abuse while preserving reason codes, metrics, logs, and support evidence. | Reuse `PeerAction::Disconnect`, `PeerManager::remove_peer`, and `InboundPeerServingStatus`; add reason/event projection before any runtime socket action. |
| EVICT-03 | Durable discourage/ban policy with expiry, address/subnet scope, manual unban, and no hidden broad bans. | Introduce typed `BanScope`, `PeerBanEntry`, `PeerBanBook`, `BanDecision`, and injected-time expiry APIs; wire evidence through node status first. |
| EVICT-04 | Misbehavior accounting maps protocol violations to bounded responses without incorrectly banning or evicting permissioned peers. | Add typed `MisbehaviorKind`, `MisbehaviorPolicy`, `MisbehaviorDecision`, protected-peer checks, and tests for `noban`/protected no-action outcomes. |
</phase_requirements>

## Recommended Implementation Shape

### Pure Network Policy

Create `packages/open-bitcoin-network/src/peer_policy.rs` with parity breadcrumbs to:

- `packages/bitcoin-knots/src/net.cpp`
- `packages/bitcoin-knots/src/net_processing.cpp`
- `packages/bitcoin-knots/src/banman.h`
- `packages/bitcoin-knots/src/banman.cpp`
- `packages/bitcoin-knots/src/net_permissions.cpp`

The module should own:

- `EvictionCandidateInput`, `EvictionScore`, `EvictionScoreComponent`, `EvictionDecision`, `EvictionReason`
- `BanScope`, `PeerBanEntry`, `PeerBanBook`, `BanDecision`, `UnbanDecision`, `BanReason`
- `MisbehaviorKind`, `MisbehaviorPolicy`, `MisbehaviorObservation`, `MisbehaviorDecision`, `MisbehaviorResponse`

Keep runtime clocks and storage outside the module. All expiry checks should accept `now_unix_seconds`.

### PeerManager Integration

`PeerManager` already stores peer role, handshake state, requested inventory, `getaddr` request state, inbound record, and permission decision. Add small methods rather than embedding policy in runtime adapters:

- `eviction_decision(&self) -> EvictionDecision`
- `record_misbehavior(...) -> Result<MisbehaviorDecision, NetworkError>` if the policy needs peer lookup
- optional evidence accessors if node status should project latest decisions from `PeerManager`

If implementation becomes large, prefer keeping data and algorithms in `peer_policy.rs` and only thin `PeerManager` glue in `peer.rs`.

### Managed Node And Status Projection

Extend `packages/open-bitcoin-node/src/network/inbound.rs` and `packages/open-bitcoin-node/src/status/inbound.rs` with a bounded evidence struct, for example:

- latest eviction decision event
- active ban count, expired ban count, manual unban count
- misbehavior observation count
- protected no-action count

Status fields should use strings from policy `as_str()` methods, not ad hoc renderer strings. CLI status/support renderers should only render shared status fields.

### Docs And Deterministic Checker

Follow the Phase 90/91/92 pattern:

- update `docs/parity/catalog/p2p.md` with a Phase 93 surface paragraph and Knots anchors
- update `docs/parity/checklist.md` and `docs/parity/index.json`
- update `docs/parity/source-breadcrumbs.json` for new Rust files
- add `scripts/check-phase93-eviction-ban-policy.ts` and `.test.ts`
- wire the checker into `scripts/verify.sh`

Checker should prove required labels and docs exist while rejecting overclaims such as transaction relay, compact block relay, mempool propagation, public inbound defaults, public-network readiness, and production full-node readiness.

## Existing Integration Points

- `packages/open-bitcoin-network/src/peer.rs` - `PeerState`, `PeerManager`, `PeerAction::Disconnect`, inbound record storage, and message handling.
- `packages/open-bitcoin-network/src/inbound.rs` - `InboundPeerRecord`, `InboundHandshakeState`, `InboundAdmissionSlotClass`, and stable admission rejection labels.
- `packages/open-bitcoin-network/src/inbound/permissions.rs` - `noban` maps to `EvictionPolicyProtected` and `MisbehaviorPolicyProtected`; `forceinbound` maps to admission protection.
- `packages/open-bitcoin-node/src/network/inbound.rs` - managed projection point for admission, permission, and address-boundary evidence.
- `packages/open-bitcoin-node/src/status/inbound.rs` - shared status contract used by RPC/CLI/support.
- `packages/open-bitcoin-node/src/metrics.rs` - fixed metric enum pattern for low-cardinality counters.
- `packages/open-bitcoin-cli/src/operator/status/render/inbound.rs` and `packages/open-bitcoin-cli/src/operator/support/render/inbound.rs` - renderer-only projections.

## Risks And Guardrails

- Do not implement broad Phase 94 resource governance under the name of misbehavior. Keep timeouts, queue pressure, reconnect throttling, and payload-size governance deferred unless represented only as typed future inputs.
- Do not let `noban` or protected classes hide violations. Protected peers should produce `protected_no_action` style evidence.
- Do not expose raw peer identifiers, endpoint tables, class names, permission strings, or config literals in status/support output.
- Do not add new external crates; standard library and existing workspace crates are enough.
- Add Arrange/Act/Assert unit tests for pure policy and deterministic status projection.

## Verification Recommendation

Use targeted iteration while implementing:

```bash
cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network peer_policy
cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node inbound
bun test scripts/check-phase93-eviction-ban-policy.test.ts
```

Final gate remains:

```bash
bash scripts/verify.sh
```
