---
phase: 128-production-compact-announcement-transport
reviewed: 2026-07-20T09:41:18Z
depth: standard
files_reviewed: 32
files_reviewed_list:
  - docs/parity/catalog/p2p.md
  - docs/parity/index.json
  - docs/parity/source-breadcrumbs.json
  - packages/open-bitcoin-network/src/peer.rs
  - packages/open-bitcoin-network/src/peer/compact_relay.rs
  - packages/open-bitcoin-network/src/peer/message_dispatch.rs
  - packages/open-bitcoin-network/src/peer/tests.rs
  - packages/open-bitcoin-node/src/lib.rs
  - packages/open-bitcoin-node/src/network.rs
  - packages/open-bitcoin-node/src/network/announcement_transport.rs
  - packages/open-bitcoin-node/src/network/block_relay_evidence.rs
  - packages/open-bitcoin-node/src/network/runtime_authority.rs
  - packages/open-bitcoin-node/src/network/tests.rs
  - packages/open-bitcoin-node/src/network/tests/announcement_transport_cases.rs
  - packages/open-bitcoin-node/src/sync.rs
  - packages/open-bitcoin-node/src/sync/block_reconcile.rs
  - packages/open-bitcoin-node/src/sync/block_response.rs
  - packages/open-bitcoin-node/src/sync/runtime_state.rs
  - packages/open-bitcoin-node/src/sync/session.rs
  - packages/open-bitcoin-node/src/sync/tests.rs
  - packages/open-bitcoin-node/src/sync/tests/production_announcement_transport_cases.rs
  - packages/open-bitcoin-node/src/sync/types.rs
  - packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/runtime_control.rs
  - packages/open-bitcoin-rpc/src/inbound_listener.rs
  - packages/open-bitcoin-rpc/src/inbound_listener/connection_runtime.rs
  - packages/open-bitcoin-rpc/src/inbound_listener/tests.rs
  - packages/open-bitcoin-rpc/tests/black_box_parity.rs
  - scripts/check-phase124-post-audit-gap-planning.ts
  - scripts/check-phase128-production-compact-announcement-transport.test.ts
  - scripts/check-phase128-production-compact-announcement-transport.ts
  - scripts/verify.sh
findings:
  critical: 0
  warning: 0
  info: 0
  total: 0
status: clean
---

# Phase 128: Code Review Report

**Reviewed:** 2026-07-20T09:41:18Z
**Depth:** standard
**Files Reviewed:** 32
**Status:** clean

## Summary

The fresh review covered the original Phase 128 source scope plus every reviewable
file changed by fixes `89d6b813` and `77ccae7d`, with the implementation diff
inspected from `f134693b..77ccae7d`.

WR-01 is resolved. Each inbound peer now owns a generation-tracked,
cancellation-safe outbox readiness cursor. Enqueue publishes readiness only
after releasing the registry lock, and the inbound loop keeps the same pinned
socket-read future alive while servicing notifications, so partial frames are
not discarded and enqueues cannot be lost between draining and waiting.
Announcements are removed under the short registry lock, written without that
lock, and credited only after `WriteWireMessageOutcome::Written`.

WR-02 is resolved. Production composition now gives inbound accepts and
outbound sync one shared `PeerIdentityAuthority`. Its atomic allocation is
unique across clones, reserves zero as the exhausted sentinel, returns the
final nonzero identifier once, and then fails closed without wrapping into a
collision. Duplicate live outbox registration fails atomically. Outbound
cleanup is guarded by explicit acquisition flags, so a failed setup cannot
unregister an existing outbox or disconnect a peer it did not connect.

No new races, missed wakeups, notification loss, peer-ID collision or
exhaustion bugs, cleanup-ownership defects, lock-across-effect violations, or
false post-write receipt evidence were found. All reviewed files meet the
project's quality standards.

This review applied the repo-local guidance in `AGENTS.md`, the Bright Builds
sidecar, and the managed architecture, code-shape, testing, verification, Rust,
and TypeScript/JavaScript standards. The tracked LOC report is generated and
was therefore excluded from the reviewable source count.

______________________________________________________________________

_Reviewed: 2026-07-20T09:41:18Z_
_Reviewer: the agent (gsd-code-reviewer)_
_Depth: standard_
