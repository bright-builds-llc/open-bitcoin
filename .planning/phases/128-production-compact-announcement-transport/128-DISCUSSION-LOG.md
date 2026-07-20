# Phase 128: Production Compact Announcement Transport - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md; this log preserves the alternatives considered.

**Date:** 2026-07-20T02:02:55.970Z
**Phase:** 128-production-compact-announcement-transport
**Mode:** Yolo
**Areas discussed:** Bilateral compact negotiation, validated-block announcement trigger, post-write transport evidence

***

## Bilateral Compact Negotiation

| Option | Description | Selected |
| --- | --- | --- |
| Activation-gated post-Verack low-bandwidth offer | Send version-2 `sendcmpct(false)` after handshake establishment, keep local offer state separate from remote preference, and preserve default-off activation. | ✓ |
| Activation-gated pre-Verack low-bandwidth offer | Include the local offer in the version-response burst before handshake completion. | |
| Low-bandwidth offer plus adaptive high-bandwidth upgrades | Add bounded peer ranking, high-bandwidth promotion, and demotion behavior. | |

**User's choice:** Auto-selected the recommended post-Verack low-bandwidth offer.
**Notes:** The pinned Knots anchor sends version-2 `sendcmpct(false)` after Verack. Remote `sendcmpct(true/false)` remains the directional control for outbound compact announcements.

***

## Validated-Block Announcement Trigger

| Option | Description | Selected |
| --- | --- | --- |
| Post-durable tip event with bounded per-peer outboxes | After durable best-tip activation, snapshot live peer facts under a short lock and route peer-targeted emissions through bounded owning-session queues. | ✓ |
| Connect result returns targeted effects to current session | Return announcements only through the session currently processing the block. | |
| Dedicated announcement coordinator/actor | Introduce a single-owner command protocol for validated-block events, peer facts, queues, receipts, and shutdown. | |

**User's choice:** Auto-selected the recommended post-durable tip event with bounded per-peer outboxes.
**Notes:** This reaches concurrent inbound and outbound peers while preserving Phase 127's no-lock-across-effects boundary and existing resource/fallback policy.

***

## Post-Write Transport Evidence

| Option | Description | Selected |
| --- | --- | --- |
| Owned `PeerEmission` plus consuming write receipt | Preserve typed peer/message/block/evidence intent through transport and consume a non-replayable receipt only after a successful write. | ✓ |
| `(PeerId, WireNetworkMessage)` plus generic acknowledgement | Reuse a smaller tuple carrier and public post-write callback, re-deriving evidence after send. | |
| General receipt-bearing outbound emitter | Normalize every P2P message through a new cross-cutting emitter and receipt framework. | |

**User's choice:** Auto-selected the recommended owned carrier plus consuming receipt.
**Notes:** Successful batch prefixes receive exactly-once credit; failed writes and unsent suffixes receive none. Peer and block provenance stay internal while aggregate metrics/logs remain fixed and redacted.

## the agent's Discretion

- Exact type, queue, wakeup, and receipt names.
- Exact narrow module split and focused fixtures.
- Whether the bounded outbox is an existing-session queue extension or a small node-owned registry.

## Deferred Ideas

- Adaptive high-bandwidth inbound compact-peer selection and demotion.
- A dedicated network actor/coordinator.
- A generalized receipt emitter for all P2P messages.
- All package/filter/public-default/public-network/production-readiness surfaces remain outside Phase 128.
