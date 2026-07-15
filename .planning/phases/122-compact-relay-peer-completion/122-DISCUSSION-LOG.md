# Phase 122: Compact Relay Peer Completion - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-07-15T15:22:57.638Z
**Phase:** 122-Compact Relay Peer Completion
**Mode:** Yolo
**Areas discussed:** Announcement correlation, Response eligibility and outcomes, Protocol verification and evidence

***

## Announcement Correlation

| Option | Description | Selected |
| --- | --- | --- |
| One latest hash per peer | Minimal peer-scoped state, but a newer tip can evict a legitimate outstanding request. | |
| Bounded per-peer hash collection with shared block storage | Correlates requests to actual peer announcements, tolerates a short tip burst, and avoids block duplication. | yes |
| Knots-style global recent-block cache only | Closest literal Knots shape, but authorizes peers that were never sent the compact block. | |
| Full block retained per peer | Direct lookup at the cost of duplicated memory and policy/storage coupling. | |

**User's choice:** Auto-selected bounded per-peer hash collection with shared block storage.
**Notes:** Record tokens only after a real compact message is produced. Use a small explicit bound aligned with Knots' recent-depth window and clear peer-owned state on disconnect.

***

## Response Eligibility And Outcomes

| Option | Description | Selected |
| --- | --- | --- |
| Per-peer provenance plus current serving and resource gates | Exactly matches HARD-01 while preserving current default-off, availability, and resource boundaries. | yes |
| Exact Knots global recent/depth policy | Includes global recent serving and old-block full-block fallback, expanding this phase beyond locally announced blocks. | |
| Any active available block without announcement provenance | Reuses block gates but permits unsolicited `getblocktxn` amplification. | |

**User's choice:** Auto-selected per-peer provenance plus current serving and resource gates.
**Notes:** Serve ordered witness-preserving `BlockTxn` responses; silently suppress benign unservable requests; disconnect on out-of-bounds indexes through typed compact misbehavior.

***

## Protocol Verification And Evidence

| Option | Description | Selected |
| --- | --- | --- |
| Rename plus one live happy-path test | Removes stale wording but leaves peer scoping, bounds, suppression, and regression enforcement unproven. | |
| Layered protocol contract with checker and parity evidence | Covers pure decisions, live shell routing, stable labels, cleanup, mutation checks, and auditable parity. | yes |
| Full public observability expansion | Reopens Phase 116/121 schemas and expands HARD-01 into unrelated operator surfaces. | |

**User's choice:** Auto-selected layered protocol contract with checker and parity evidence.
**Notes:** Rename the Phase 112 test precisely, add deterministic pure/live tests, wire a focused Phase 122 checker and mutation suite into `scripts/verify.sh`, and update parity evidence without rewriting historical artifacts.

## the agent's Discretion

- Exact Rust type and action names.
- FIFO, deque, or insertion-ordered set representation for the bounded per-peer token collection.
- Smallest module split that reuses existing action translation and serving policy boundaries.

## Deferred Ideas

- Knots-style full-block fallback for old `getblocktxn` requests.
- New public observability schemas for inbound missing-transaction serving.
