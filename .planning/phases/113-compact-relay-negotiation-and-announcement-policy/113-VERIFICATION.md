---
phase: 113-compact-relay-negotiation-and-announcement-policy
verified: 2026-07-05T00:16:06Z
status: passed
score: "12/12 must-haves verified"
generated_by: gsd-verifier
lifecycle_mode: yolo
phase_lifecycle_id: 113-2026-07-04T22-53-48
generated_at: 2026-07-05T00:16:06Z
lifecycle_validated: true
overrides_applied: 0
---

# Phase 113: Compact Relay Negotiation and Announcement Policy Verification Report

**Phase Goal:** Track per-peer compact-block capability and decide when compact block announcements are allowed without coupling compact relay to transaction relay or public defaults.
**Verified:** 2026-07-05T00:16:06Z
**Status:** passed
**Re-verification:** No - initial verification

Guidance consulted: `AGENTS.md`, `AGENTS.bright-builds.md`, `standards/core/architecture.md`, `standards/core/code-shape.md`, `standards/core/testing.md`, `standards/core/verification.md`, `standards/languages/rust.md`, `standards-overrides.md`, and GSD verification references.

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
| --- | --- | --- | --- |
| 1 | Peer state records compact-block capability, high-bandwidth preference, low-bandwidth preference, unsupported-version evidence, and announcement eligibility. | VERIFIED | `CompactRelayPeerState` has `capability`, `high_bandwidth_preference`, `low_bandwidth_preference`, `announcement_eligibility`, and `maybe_unsupported_version`; `PeerState` stores `compact_relay: CompactRelayPeerState`. |
| 2 | Peer compact relay state defaults to unknown and remains separate from transaction relay state. | VERIFIED | `CompactRelayPeerState::default()` sets unknown capability/preferences/eligibility and no unsupported version; `PeerState::new` initializes it separately from `remote_wtxidrelay` and transaction relay fields. |
| 3 | Supported `sendcmpct` version 2 is the only positive compact capability signal. | VERIFIED | `apply_send_compact` gates support on `BIP152_COMPACT_BLOCKS_VERSION`; unsupported versions route to `record_unsupported_version`. |
| 4 | Version 2 high/low `sendcmpct` toggles use last-supported preference semantics and clear the opposite preference. | VERIFIED | High sets high `Requested` and low `NotRequested`; low sets low `Requested` and high `NotRequested`; Phase 113 tests cover high-to-low and low-to-high. |
| 5 | Unsupported `sendcmpct` versions record evidence without disconnecting or overwriting the last supported v2 state. | VERIFIED | Unsupported handling sets `maybe_unsupported_version` and preserves existing `Supported { version: BIP152_COMPACT_BLOCKS_VERSION }`; tests assert no disconnect and preserved high-bandwidth preference. |
| 6 | `sendcmpct` handling does not update stored compact announcement eligibility. | VERIFIED | The only production assignment to `announcement_eligibility` is inside `record_announcement_decision`; negotiation tests assert eligibility remains `Unknown` after sendcmpct paths. |
| 7 | Compact announcement policy requires local activation, BIP152 v2 peer negotiation, high-bandwidth preference, header continuity, block availability, and resource capacity. | VERIFIED | `decide_compact_announcement` checks local compact activation, supported v2 capability, high-bandwidth preference, previous/current header facts, block status, and resource gate before `AnnounceCompactBlock`. |
| 8 | Every compact announcement decision derives and records current eligibility through the decision-recording path. | VERIFIED | `PeerManager::decide_compact_announcement_for_peer` builds `CompactAnnouncementInput` from stored peer state, calls `decide_compact_announcement`, and then calls `peer.compact_relay.record_announcement_decision(&decision)`. |
| 9 | Header fallback, inventory fallback, and suppression are explicit typed outcomes with stable low-cardinality reasons. | VERIFIED | `CompactAnnouncementAction` covers `AnnounceCompactBlock`, `AnnounceHeaders`, `AnnounceInventory`, and `Suppress`; `CompactAnnouncementReason::as_str()` returns fixed labels such as `compact_high_bandwidth_not_requested`, `compact_block_unavailable`, and `compact_resource_limited`. |
| 10 | Low-bandwidth compact preference remains capability state and does not authorize direct compact announcements. | VERIFIED | Tests prove `sendcmpct(false, 2)` yields headers or inventory fallback with `compact_high_bandwidth_not_requested`, not `AnnounceCompactBlock`. |
| 11 | Transaction relay, package relay, bloom/filter permissions, compact filters, public defaults, and production claims do not activate compact announcements. | VERIFIED | Scope-isolation tests cover `WtxidRelay`, block serving without compact relay, download permission, protected inbound permission, and default activation; production compact policy contains no package/filter/public/production scope terms. |
| 12 | Compact getdata serving remains suppressed and separate from direct compact announcement policy. | VERIFIED | Node regression `phase113_compact_getdata_remains_suppressed_after_negotiation_policy` asserts compact getdata returns `NotFound`, emits no `CompactBlock`, no `GetBlockTxn`/`BlockTxn`, and performs no mempool or chainstate side effects. |

**Score:** 12/12 truths verified

### Required Artifacts

| Artifact | Expected | Status | Details |
| --- | --- | --- | --- |
| `packages/open-bitcoin-network/src/peer/compact_relay.rs` | Pure compact relay negotiation state and announcement decision policy. | VERIFIED | Contains state types, `apply_send_compact`, `record_announcement_decision`, `decide_compact_announcement`, fixed reason labels, and BIP152 v2 checks. |
| `packages/open-bitcoin-network/src/peer/compact_relay/tests.rs` | Focused pure-policy tests. | VERIFIED | Covers defaults, high/low toggles, unsupported evidence, gate order, reason mapping, non-v2 supported capability rejection, and record-only eligibility mutation. |
| `packages/open-bitcoin-network/src/peer.rs` | Peer storage, `sendcmpct` routing, and decision entrypoint. | VERIFIED | Stores compact relay state per peer, applies `WireNetworkMessage::SendCompact(message)`, and records eligibility only after announcement decision. |
| `packages/open-bitcoin-network/src/peer/tests.rs` | PeerManager Phase 113 behavior and guardrail tests. | VERIFIED | Contains 28 passing `phase113_` tests for negotiation, fallback, scope isolation, eligibility refresh, and unsupported-version preservation. |
| `packages/open-bitcoin-node/src/network/tests.rs` | Node-shell compact getdata suppression regression. | VERIFIED | Contains `phase113_compact_getdata_remains_suppressed_after_negotiation_policy`. |
| `packages/open-bitcoin-network/src/lib.rs` | Public re-exports for policy consumers. | VERIFIED | Re-exports compact relay state, announcement input/decision/action/reason types, and `decide_compact_announcement`. |
| `docs/parity/source-breadcrumbs.json` | Parity breadcrumb registration for new first-party Rust files. | VERIFIED | Contains `network-compact-relay-peer-state` for `compact_relay.rs` and `compact_relay/tests.rs`; parity check passed. |

### Key Link Verification

| From | To | Via | Status | Details |
| --- | --- | --- | --- | --- |
| `WireNetworkMessage::SendCompact` | `PeerState.compact_relay` | `PeerManager::handle_message` calls `apply_send_compact` | WIRED | Sendcmpct mutates only the matched peer state and returns no actions. |
| `CompactRelayPeerState` | `decide_compact_announcement` | `CompactAnnouncementInput.peer_state` | WIRED | PeerManager copies stored peer state into the pure announcement input. |
| `CompactAnnouncementDecision` | `CompactRelayPeerState.announcement_eligibility` | `record_announcement_decision(&decision)` | WIRED | The only production assignment to eligibility is this method. |
| `BlockRelayActivationPolicy.compact_relay` | `CompactAnnouncementAction::AnnounceCompactBlock` | local activation gate | WIRED | Compact activation is checked before peer capability. |
| `ManagedPeerNetwork::receive_message(GetData CompactBlock)` | compact getdata suppression | node-shell regression | WIRED | Compact inventory requests remain `NotFound`; no compact payload/reconstruction behavior is activated. |

### Data-Flow Trace (Level 4)

| Artifact | Data Variable | Source | Produces Real Data | Status |
| --- | --- | --- | --- | --- |
| `PeerState.compact_relay` | compact relay capability/preferences | Decoded `WireNetworkMessage::SendCompact(SendCompactMessage)` | Yes | FLOWING |
| `CompactAnnouncementDecision` | action, reason, eligibility | Pure `CompactAnnouncementInput` from activation, peer state, header facts, status, and resource gate | Yes | FLOWING |
| `PeerState.compact_relay.announcement_eligibility` | stored eligibility | `CompactRelayPeerState::record_announcement_decision(&decision)` | Yes | FLOWING |

### Behavioral Spot-Checks

| Behavior | Command | Result | Status |
| --- | --- | --- | --- |
| Phase 113 peer negotiation, announcement, fallback, and isolation behavior | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network phase113_ -- --nocapture` | 28 passed, 0 failed | PASS |
| Compact getdata remains suppressed in node shell | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase113_compact_getdata_remains_suppressed_after_negotiation_policy -- --nocapture` | 1 passed, 0 failed | PASS |
| Parity breadcrumbs cover new first-party Rust files | `bun run scripts/check-parity-breadcrumbs.ts --check` | Verified 353 Rust files | PASS |

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
| --- | --- | --- | --- | --- |
| CMP-04 | `113-01-PLAN.md`, `113-02-PLAN.md` | Node tracks per-peer compact-block capability, high-bandwidth preference, low-bandwidth preference, and compact-block announcement eligibility deterministically. | SATISFIED | `CompactRelayPeerState`, per-peer storage, sendcmpct tests, and decision-recorded eligibility cover the requirement. Note: `REQUIREMENTS.md` still marks CMP-04 as Pending even though implementation evidence satisfies it. |
| CMP-05 | `113-02-PLAN.md`, `113-03-PLAN.md` | Node announces compact blocks only when activation, peer negotiation, header state, block availability, and resource limits permit it. | SATISFIED | `decide_compact_announcement` gate order plus `phase113_compact_announcement_*`, low/high toggle, header, block, and resource tests verify it. |
| CMP-06 | `113-01-PLAN.md`, `113-03-PLAN.md` | Compact-block negotiation remains independent from transaction relay, package relay, bloom/filter permissions, compact filters, and public serving defaults. | SATISFIED | Tests cover transaction relay/wtxidrelay, block serving without compact relay, download/protected inbound permissions, and default activation; production compact policy has no package/filter/public/production activators. |

No orphaned Phase 113 requirement IDs were found: `CMP-04`, `CMP-05`, and `CMP-06` are the complete Phase 113 set in `REQUIREMENTS.md` traceability and all appear in plan frontmatter.

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
| --- | --- | --- | --- | --- |
| None | - | - | - | No TODO/FIXME/placeholder/not-implemented markers or blocking stub patterns found in scoped Phase 113 files. The only empty-block scan hit is the intentional `Supported { version: BIP152_COMPACT_BLOCKS_VERSION } => {}` match arm in `decide_compact_announcement`. |

### Human Verification Required

None. Phase 113 is pure network policy plus deterministic Rust tests; no visual, real-time, external-service, public-network, performance-feel, or operator UX behavior is required to verify this phase goal.

### Deferred Items

No Phase 113 gaps were deferred. Later phases explicitly own reconstruction, missing-transaction round trips, validation handoff, operator evidence, metrics/logs/support surfaces, release docs, UAT, and no-claim checkers, but those are not missing Phase 113 deliverables.

### Gaps Summary

No goal-blocking gaps found. The implementation achieves the phase goal: per-peer compact relay negotiation state is explicit, direct compact announcements are allowed only through the typed gate policy, failed gates return stable fallback or suppression decisions, and neighboring relay/permission/public-default surfaces do not activate compact relay.

---

_Verified: 2026-07-05T00:16:06Z_
_Verifier: Claude (gsd-verifier)_
