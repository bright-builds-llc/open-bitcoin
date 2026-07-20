---
phase: 128-production-compact-announcement-transport
verified: 2026-07-20T10:02:21Z
status: passed
score: 10/10 must-haves verified
generated_by: gsd-verifier
lifecycle_mode: yolo
phase_lifecycle_id: 128-2026-07-20T01-54-33
generated_at: 2026-07-20T10:02:21Z
lifecycle_validated: true
overrides_applied: 0
---

# Phase 128: Production Compact Announcement Transport Verification Report

**Phase Goal:** Close the remaining production compact-block rollout gap by emitting local `sendcmpct` offers from real post-Verack handshakes, triggering compact/header/inventory announcement planning from newly validated durably available active-tip events, routing selected announcements through owning real production peer sessions, and crediting metrics/logs/provenance only after successful writes.
**Verified:** 2026-07-20T10:02:21Z
**Status:** passed
**Re-verification:** No — initial verification

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
| --- | --- | --- | --- |
| 1 | A production handshake emits the local low-bandwidth BIP152 version-2 `sendcmpct` offer after remote Verack and retains independently negotiated remote compact-relay state. | ✓ VERIFIED | `compact_relay.rs` gates a typed one-shot offer on activation, established handshake, and protocol version; `message_dispatch.rs` appends the offer on Verack; the real RPC loopback test observes `SendCompact(false, 2)`. |
| 2 | Duplicate Verack, disabled activation, unsupported protocol versions, and directional high/low preference changes fail closed without conflating local offer state with remote capability state. | ✓ VERIFIED | Peer state has distinct local-offer and remote-negotiation fields; focused peer tests and the Phase 128 mutation checker cover duplicate, disabled, unsupported, and high/low transitions. |
| 3 | Production announcement planning is triggered only after a validated active-tip block is durably available, collapses multi-block progress to the final tip, and excludes side-branch, duplicate, invalid, and failed-persistence paths. | ✓ VERIFIED | `block_response.rs` stores the block before queuing `DurableTipAdvanced`; `block_reconcile.rs` queues only the final connected/reorg tip; fresh direct-sync and live-reconcile tests both passed. |
| 4 | Announcement policy uses authoritative block availability and live per-peer previous/current header facts, negotiation, activation, permissions, and resource pressure. | ✓ VERIFIED | `announcement_transport.rs` snapshots active peers and bounded outboxes, derives header facts from peer provenance, and calls live status, eligibility, resource, and fallback policy rather than constants. |
| 5 | Prepared compact/header/inventory work is an owned, peer-targeted, non-cloneable emission with bounded per-peer and aggregate queue accounting, prepared outside I/O and without holding authority locks across writes. | ✓ VERIFIED | `PeerEmission` owns peer/message/hash/evidence; `PeerEmissionReceipt` is consuming and has a compile-fail clone guard; `PeerAnnouncementOutboxes` enforces caps; authority mutation is limited to preparation/completion. |
| 6 | Compact, header, and inventory selections are written through the owning real outbound or inbound peer session, and an idle inbound session wakes for newly queued work. | ✓ VERIFIED | Outbound `session.rs` and inbound `connection_runtime.rs` drain only their registered peer outbox and write encoded messages; the fresh fanout and idle-inbound loopback tests passed. |
| 7 | Only the successfully written prefix receives completion receipts exactly once; failed, rejected, dropped, disconnected, queue-full, and unsent suffix emissions receive no achieved-effect credit. | ✓ VERIFIED | Both session paths call `complete_peer_emission` only after successful send/`Written`; the consuming receipt prevents duplicate completion; fresh partial-failure tests passed. |
| 8 | Compact/header/inventory achieved-effect counters, structured logs, and header provenance are fixed-cardinality and redacted, and they reflect the exact successfully written wire variant. | ✓ VERIFIED | `block_relay_evidence.rs` maps the written message variant to fixed outcomes and records evidence only in receipt completion; success/failure tests assert exact counters and absence of peer, block, permission, credential, and transaction material. |
| 9 | Inbound and outbound production sessions cannot alias peer identity or unregister another session's outbox. | ✓ VERIFIED | A shared atomic `PeerIdentityAuthority`, duplicate-registration rejection, and ownership-scoped cleanup are wired into both directions; fresh distinct-identity and duplicate-outbox regressions passed. |
| 10 | The production seams are protected by deterministic mutation guards and auditable Knots parity evidence while remaining bounded and default-off. | ✓ VERIFIED | The live checker passed; all 19 checker mutation cases passed; parity breadcrumbs verified 389 Rust files; `scripts/verify.sh` invokes the Phase 128 checker before the final release-boundary gate. |

**Score:** 10/10 truths verified

### Required Artifacts

| Artifact | Expected | Status | Details |
| --- | --- | --- | --- |
| `packages/open-bitcoin-network/src/peer/compact_relay.rs` | Typed local offer and remote negotiation state | ✓ VERIFIED | Substantive, imported by peer state, and exercised by handshake policy. |
| `packages/open-bitcoin-network/src/peer/message_dispatch.rs` | Post-Verack offer scheduling | ✓ VERIFIED | Verack establishes the session, preserves existing actions, and appends the typed offer. |
| `packages/open-bitcoin-node/src/network/announcement_transport.rs` | Live policy preparation and owned emissions/receipts | ✓ VERIFIED | Substantive, wired through runtime authority and both session directions. |
| `packages/open-bitcoin-node/src/network/runtime_authority.rs` | Short-lock preparation and completion boundary | ✓ VERIFIED | Shared authority prepares outside I/O and consumes receipts after writes. |
| `packages/open-bitcoin-node/src/network/block_relay_evidence.rs` | Fixed achieved-effect evidence | ✓ VERIFIED | Exact wire variant selects fixed counters/log labels and provenance. |
| `packages/open-bitcoin-node/src/sync/block_response.rs` | Post-persistence direct-sync trigger | ✓ VERIFIED | Durable save and active-tip checks precede event dispatch. |
| `packages/open-bitcoin-node/src/sync/block_reconcile.rs` | Final-tip live reconcile/reorg trigger | ✓ VERIFIED | Multi-block and reorg paths collapse to the final durably persisted tip. |
| `packages/open-bitcoin-node/src/sync/session.rs` | Bounded outboxes and outbound write ownership | ✓ VERIFIED | Targeted FIFO drain, successful-prefix receipt completion, and ownership cleanup are wired. |
| `packages/open-bitcoin-rpc/src/inbound_listener/connection_runtime.rs` | Inbound write ownership and idle wakeup | ✓ VERIFIED | Cancellation-safe read/wakeup selection sends queued work and credits only `Written`. |
| `packages/open-bitcoin-node/src/sync/tests/production_announcement_transport_cases.rs` | Production fanout and partial-failure proof | ✓ VERIFIED | Both focused tests passed in this verification run. |
| `scripts/check-phase128-production-compact-announcement-transport.ts` | Deterministic seam guard | ✓ VERIFIED | Live corpus check passed. |
| `docs/parity/index.json` and `docs/parity/catalog/p2p.md` | Auditable Knots anchors and bounded claims | ✓ VERIFIED | CMP-04/CMP-05/OBS-03 evidence and exact upstream anchors are present. |

All 13/13 PLAN-declared artifacts passed `gsd-tools verify artifacts`.

### Key Link Verification

| From | To | Via | Status | Details |
| --- | --- | --- | --- | --- |
| `message_dispatch.rs` | `compact_relay.rs` | Verack asks typed local-offer policy | ✓ WIRED | Offer is appended to real peer actions. |
| `peer.rs` | production peer transport | `PeerAction::Send(SendCompact)` | ✓ WIRED | Real loopback receives the frame. |
| durable sync/reconcile | `DurableTipAdvanced` sink | save/progress persistence before dispatch | ✓ WIRED | Direct and reconcile final-tip tests passed. |
| durable-tip sink | `runtime_authority.rs` | outbox snapshot plus block preparation | ✓ WIRED | Open runtime installs the real production sink. |
| `announcement_transport.rs` | live network state | peer header/protocol/permission/resource facts | ✓ WIRED | No constant eligibility proxy found. |
| prepared emission | outbound `session.rs` | owning peer FIFO send | ✓ WIRED | Send precedes receipt completion. |
| prepared emission | inbound `connection_runtime.rs` | notification-driven owning peer send | ✓ WIRED | `Written` precedes receipt completion. |
| successful write receipt | `block_relay_evidence.rs` | consuming `complete_peer_emission` | ✓ WIRED | Failed/unsent work cannot produce a receipt. |
| `scripts/verify.sh` | Phase 128 checker and tests | default deterministic gate | ✓ WIRED | Mutation test rejects removal or reordering. |

All 9/9 PLAN-declared key links passed `gsd-tools verify key-links`.

### Data-Flow Trace (Level 4)

| Artifact | Data Variable | Source | Produces Real Data | Status |
| --- | --- | --- | --- | --- |
| Production announcement sink | `DurableTipAdvanced.block` | Newly stored active-tip block from direct sync or reconcile | Yes | ✓ FLOWING |
| Announcement preparation | peer/header/availability/resource facts | Authoritative network state plus bounded outbox snapshots | Yes | ✓ FLOWING |
| Outbound/inbound session | `PeerEmission.message` | Policy-selected compact block, headers, or inventory payload | Yes | ✓ FLOWING |
| Achieved-effect projection | `PeerEmissionReceipt` | Successful peer transport write only | Yes | ✓ FLOWING |
| Metrics/log/provenance | fixed announcement outcome | Receipt-bound written wire variant | Yes | ✓ FLOWING |

### Behavioral Spot-Checks

| Behavior | Command | Result | Status |
| --- | --- | --- | --- |
| Deterministic seam mutations | `bun test scripts/check-phase128-production-compact-announcement-transport.test.ts` | 19 passed, 0 failed | ✓ PASS |
| Live Phase 128 corpus | `bun run scripts/check-phase128-production-compact-announcement-transport.ts` | Validated | ✓ PASS |
| Parity breadcrumbs | `bun run scripts/check-parity-breadcrumbs.ts` | 389 Rust files verified | ✓ PASS |
| Compact/header/inventory fanout and partial failure | `cargo test ... -p open-bitcoin-node production_announcement_transport_cases` | 2 passed, 0 failed | ✓ PASS |
| Final durable tip from direct sync and live reconcile | `cargo test ... -p open-bitcoin-node durable_tip_` | 2 passed, 0 failed | ✓ PASS |
| Idle inbound wakeup and exact credit | `cargo test ... -p open-bitcoin-rpc idle_inbound_peer_wakes_for_queued_announcement_and_credits_once` | 1 passed, 0 failed | ✓ PASS |
| Real post-Verack compact offer | `cargo test ... -p open-bitcoin-rpc --test black_box_parity phase127_production_composition_shares_sync_serving_and_operator_authority` | 1 passed, 0 failed | ✓ PASS |
| Process-wide identity and outbox ownership | Focused RPC and node regression filters | 2 passed, 0 failed | ✓ PASS |
| Full repo-native verifier | `bash scripts/verify.sh` | Deterministic chain through Phase 128, pure-core, length, and panic-site gates passed; run was stopped during later Rust stages at orchestrator request, not by a test failure | ℹ PARTIAL |

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
| --- | --- | --- | --- | --- |
| CMP-04 | 128-01, 128-03, 128-04 | Deterministic per-peer compact capability, high/low preference, and eligibility | ✓ SATISFIED | Directional state, post-Verack offer, live eligibility, and regression tests verified. |
| CMP-05 | 128-02, 128-03, 128-04 | Compact announcement only when activation, negotiation, header state, availability, and resources permit | ✓ SATISFIED | Live policy preparation and compact/header/inventory production fanout verified. |
| OBS-03 | 128-02, 128-03, 128-04 | Fixed low-cardinality metrics and structured logs for outcome categories | ✓ SATISFIED | Receipt-only fixed counters/log projection and redaction assertions verified. |

No Phase 128 requirement is orphaned.

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
| --- | --- | --- | --- | --- |
| Phase diff | — | TODO/FIXME/placeholder/empty implementation scan | None | No blocker or user-visible stub found. The only added `console.log` is the deterministic checker success message. |
| Phase diff | — | Whitespace/error scan | None | `git diff --check` passed. |

### Disconfirmation Pass

- The strongest false-positive risk was “the outbox is populated, so delivery works.” The earlier idle-inbound wakeup gap was repaired with notification-driven cancellation-safe readiness, and a real loopback regression now proves delivery and single credit.
- The second risk was inbound/outbound peer-ID collision causing cross-delivery or incorrect cleanup. Production now shares one atomic identity authority, duplicate registration fails closed, and focused concurrent/cleanup regressions passed.
- The third risk was evidence credit at preparation or after only part of a batch. Receipts are consuming and created only after successful write; the partial-failure test proves only the written prefix is credited.
- Production transport tests manually isolate the durable event handoff, so they could be misleading alone. Separate production direct-sync and live-reconcile tests prove the upstream final durable-tip source, and the checker independently guards persistence-before-dispatch wiring.
- A dedicated sink-capture regression for progress-persistence failure is not separate, but source control flow clears the pending event and returns before dispatch. This is a test-depth note, not an unresolved goal link.

### Human Verification Required

None. Phase 128 is headless and its required behavior is deterministically observable through local wire-loopback, state, failure, and static-wiring checks. Public-network operation remains explicitly deferred/default-off and is not part of this phase contract.

### Deferred Scope Check

Phase 129 owns aggregate integration guardrails and milestone reconciliation. No failed Phase 128 truth was moved into deferred scope, and no Phase 129 item is required to make the Phase 128 production path function.

### Gaps Summary

No implementation, wiring, data-flow, requirement, or blocker anti-pattern gap remains. The four roadmap success criteria are achieved, and no verification override was needed.

***

_Verified: 2026-07-20T10:02:21Z_
_Verifier: the agent (gsd-verifier)_
