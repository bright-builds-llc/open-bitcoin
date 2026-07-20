---
phase: 128-production-compact-announcement-transport
reviewed: 2026-07-20T04:35:00Z
depth: standard
files_reviewed: 27
files_reviewed_list:
  - docs/parity/catalog/p2p.md
  - docs/parity/index.json
  - docs/parity/source-breadcrumbs.json
  - packages/open-bitcoin-network/src/peer.rs
  - packages/open-bitcoin-network/src/peer/compact_relay.rs
  - packages/open-bitcoin-network/src/peer/message_dispatch.rs
  - packages/open-bitcoin-network/src/peer/tests.rs
  - packages/open-bitcoin-node/src/network.rs
  - packages/open-bitcoin-node/src/network/announcement_transport.rs
  - packages/open-bitcoin-node/src/network/block_relay_evidence.rs
  - packages/open-bitcoin-node/src/network/runtime_authority.rs
  - packages/open-bitcoin-node/src/network/tests.rs
  - packages/open-bitcoin-node/src/network/tests/announcement_transport_cases.rs
  - packages/open-bitcoin-node/src/sync.rs
  - packages/open-bitcoin-node/src/sync/block_reconcile.rs
  - packages/open-bitcoin-node/src/sync/block_response.rs
  - packages/open-bitcoin-node/src/sync/session.rs
  - packages/open-bitcoin-node/src/sync/tests/production_announcement_transport_cases.rs
  - packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/runtime_control.rs
  - packages/open-bitcoin-rpc/src/inbound_listener.rs
  - packages/open-bitcoin-rpc/src/inbound_listener/connection_runtime.rs
  - packages/open-bitcoin-rpc/tests/black_box_parity.rs
  - scripts/check-phase124-post-audit-gap-planning.ts
  - scripts/check-phase128-production-compact-announcement-transport.test.ts
  - scripts/check-phase128-production-compact-announcement-transport.ts
  - scripts/verify.sh
findings:
  critical: 0
  warning: 2
  info: 0
  total: 2
status: issues_found
---

# Phase 128: Code Review Report

**Reviewed:** 2026-07-20T04:35:00Z
**Depth:** standard
**Files Reviewed:** 27
**Status:** issues_found

## Summary

The review covered the committed Phase 128 diff from `f3bdae46`, including bilateral negotiation, durable-tip dispatch, bounded peer outboxes, both production socket adapters, post-write evidence, tests, parity metadata, and the deterministic checker. The functional-core/effect boundary and consuming-receipt design are generally sound, and no false-positive evidence mutation was found before a successful write.

Two production concurrency defects remain. An idle inbound session has no wakeup path for a newly enqueued announcement, and inbound/outbound peer IDs are allocated from independent overlapping namespaces even though the authoritative peer map and new outbox registry use the ID as a process-wide key.

This review applied the repo-local guidance in `AGENTS.md`, the Bright Builds sidecar and standards pages for architecture, code shape, testing, verification, Rust, and TypeScript/JavaScript, with no applicable active override in `standards-overrides.md`.

## Warnings

### WR-01: Idle inbound sessions do not wake for queued announcements

**File:** `/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-rpc/src/inbound_listener/connection_runtime.rs:83-140`

**Issue:** The inbound loop drains the announcement outbox immediately before entering `read_wire_message_for_state`, then awaits that socket read without also waiting for outbox readiness. A newly durable tip enqueued while the peer is idle cannot be written until the remote peer sends another message. If it remains idle, the read waits for the established-peer timeout (currently 1,800 seconds) and then disconnects, so the queued compact/header/inventory announcement is never sent. The existing production test calls `send_all_for_peer` directly with a fake outbound session and therefore does not exercise this inbound socket path.

**Fix:** Give each peer outbox a notification primitive and select between socket readability, shutdown/timeout, and outbox notification. On notification, drain the bounded queue through the existing post-write receipt path, then resume reading. Add a loopback inbound test that completes the handshake, remains otherwise idle, enqueues a durable-tip announcement, and observes the wire message and exactly-once evidence before the idle timeout.

### WR-02: Shared authority and outbox registry use colliding peer-ID allocators

**File:** `/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-rpc/src/inbound_listener.rs:353-366`

**Issue:** The inbound listener starts its allocator at `1`, while `DurableSyncRuntime` independently starts its outbound allocator at `1` (`packages/open-bitcoin-node/src/sync.rs:160-167`). Both directions now share the same authoritative `PeerManager` and `AnnouncementOutboxRegistry`, keyed only by `PeerId`. A simultaneous first inbound and first outbound connection therefore collide. In the outbound path, `register_peer` silently reuses the inbound queue, `connect_outbound_peer` returns `PeerAlreadyExists`, and unconditional cleanup unregisters/disconnects that ID, dropping the active inbound peer and its queued announcements. The reverse ordering rejects the inbound peer as a duplicate.

**Fix:** Allocate peer IDs from one process-wide authority shared by both accept and sync paths, or use a typed/disjoint connection identity that cannot collide by direction. Make `register_peer` reject an already-registered live owner rather than silently aliasing it, and ensure failed outbound setup only cleans up resources that attempt actually acquired. Add a concurrent inbound/outbound regression test proving distinct IDs, distinct queues, and peer-scoped cleanup.

______________________________________________________________________

_Reviewed: 2026-07-20T04:35:00Z_
_Reviewer: the agent (gsd-code-reviewer)_
_Depth: standard_
