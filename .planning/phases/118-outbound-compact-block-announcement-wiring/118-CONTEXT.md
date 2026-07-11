---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 118-2026-07-11T16-07-50
generated_at: 2026-07-11T16:07:50.076Z
---

# Phase 118: Outbound Compact Block Announcement Wiring - Context

**Gathered:** 2026-07-11
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Phase 118 closes the CMP-05 runtime seam identified by the v2.1 milestone audit: compact announcement decisions must produce real outbound `cmpctblock` (or headers/inventory fallback) on the announce path, without false-positive `CompactAnnounced` evidence.

Today `ManagedPeerNetwork::announce_block` calls `decide_compact_announcement_for_peer`, records evidence from the decision reason, then ignores `announcement.action` and always delegates to `PeerManager::announce_block`, which only emits Headers or Inv. No production path builds or sends `WireNetworkMessage::CompactBlock`.

This phase wires that seam only. It must not inject mempool candidates into compact receive (Phase 119), schedule compact-download timeouts or escalate compact misbehavior (Phase 120), project block-relay metrics/logs through DurableSyncRuntime (Phase 121), enable package relay, bloom/filter or compact-filter serving, change public defaults, add public-network CI gates, claim archive-node behavior, claim production full-node readiness, or claim production-funds wallet safety.

</domain>

<decisions>
## Implementation Decisions

### Action Honor Path

- **D-01:** `ManagedPeerNetwork::announce_block` must branch on `CompactAnnouncementDecision.action` instead of discarding it. `AnnounceCompactBlock` builds and returns `WireNetworkMessage::CompactBlock`; `AnnounceHeaders` / `AnnounceInventory` keep the existing headers/inv emission; `Suppress` returns no outbound message.
- **D-02:** Prefer extending the announce path in `open-bitcoin-network` peer surfaces so `PeerManager` (or a focused helper beside it) can emit CompactBlock/Headers/Inv/None from a typed action plus the validated local block. Keep `ManagedPeerNetwork` as the shell that decides, records evidence from what was actually emitted, and forwards the message — do not leave decision→wire branching only in the node adapter if the network crate already owns announce emission.

### Compact Payload Construction

- **D-03:** Add a production Block→`CompactBlockPayload` builder for the outbound announce path (not test-only fixtures). Use existing codec short-ID / prefilled helpers. For a locally validated block the announcer knows every transaction; prefer Knots-aligned announce shape (header, nonce, short IDs, coinbase/prefilled as required by BIP152 version 2) rather than stuffing every transaction as prefilled unless research shows that is the only correct local path.
- **D-04:** Payload construction failures on an `AnnounceCompactBlock` decision must not emit a false-positive compact announce. Fall back to a typed headers or inventory announce with a stable reason, or suppress with a stable reason — never record `CompactAnnounced` without a CompactBlock message.

### Evidence Correctness

- **D-05:** `CompactAnnounced` / `compact_announced_count` increments only when a `WireNetworkMessage::CompactBlock` is actually produced for send. Recording evidence from the decision reason alone before emission is the false-positive bug this phase closes.
- **D-06:** Headers fallback, inventory fallback, and suppress reasons continue to update their existing counters only when those outcomes are the path taken after action honor (and any construction fallback).

### Fallback And Suppression

- **D-07:** When the decision is already `AnnounceHeaders`, `AnnounceInventory`, or `Suppress`, preserve current Headers/Inv/None behavior and reasons. Do not invent new public defaults or couple announcement to transaction relay / package relay / filters.
- **D-08:** Existing Phase 113 policy gates remain authoritative for *when* compact announce is allowed. This phase does not reopen negotiation policy; it only makes the decided action observable on the wire.

### Verification And Parity

- **D-09:** Runtime/unit tests must prove: (1) high-bandwidth eligible path emits `WireNetworkMessage::CompactBlock`, (2) headers/inventory/suppress paths still emit Headers/Inv/None, (3) `compact_announced_count` rises only on real CompactBlock emission, (4) construction failure does not increment compact-announced evidence.
- **D-10:** New or touched first-party Rust source/test files under `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests` require parity breadcrumbs and `docs/parity/source-breadcrumbs.json` entries unless an explicit `none` breadcrumb is defensible. Prefer Knots `net_processing.cpp` / BIP152 announce anchors.
- **D-11:** Verification remains deterministic and local through repo-native checks and `bash scripts/verify.sh`. Public-network compact-relay review stays opt-in UAT only.

### Claude's Discretion

The planner/researcher may choose exact helper placement (peer module vs compact_relay vs codec), nonce selection strategy, and whether `PeerManager::announce_block` gains an action parameter versus a new `announce_block_with_action` API — prefer the smallest API change that makes action honor and evidence correctness testable. Prefer pure builders and early returns.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Project And Phase Scope

- `AGENTS.md` - repo-local verification, submodule, parity breadcrumb, UAT command, and GSD workflow guidance.
- `AGENTS.bright-builds.md` - Bright Builds workflow, functional-core, verification, and testing rules.
- `standards/core/architecture.md` - functional core / imperative shell and domain-type rules.
- `standards/core/code-shape.md` - early-return, optional-name, and file/function shape rules.
- `standards/core/testing.md` - focused unit test and Arrange/Act/Assert expectations.
- `standards/core/verification.md` - repo-native verification and clean commit gate expectations.
- `standards/languages/rust.md` - Rust module, invariant, optional naming, and verification guidance.
- `.planning/PROJECT.md` - active v2.1 scope, parity value, architecture constraints, and deferred public/production claims.
- `.planning/REQUIREMENTS.md` - CMP-05 ownership remapped to Phase 118.
- `.planning/ROADMAP.md` - Phase 118 goal, success criteria, and gap-closure framing.
- `.planning/STATE.md` - current milestone state and deterministic verification caveats.
- `.planning/v2.1-MILESTONE-AUDIT.md` - CMP-05 gap evidence: decision recorded then discarded; Headers/Inv always emitted.

### Prior Locked Decisions

- `.planning/phases/113-compact-relay-negotiation-and-announcement-policy/113-CONTEXT.md` - negotiation state, announcement policy gates, fallback reasons, and scope isolation.
- `.planning/phases/112-bip152-wire-codec-and-message-semantics/112-CONTEXT.md` - BIP152 payload types and wire message variants.
- `.planning/phases/110-block-serving-activation-and-eligibility-boundary/110-CONTEXT.md` - default-off activation and eligibility.
- `.planning/phases/117-parity-traceability-uat-and-release-guardrails/117-CONTEXT.md` - no-claim and verifier-boundary posture to preserve.

### Existing Code Integration Points

- `packages/open-bitcoin-node/src/network.rs` - `ManagedPeerNetwork::announce_block` decides then ignores action (primary seam).
- `packages/open-bitcoin-network/src/peer.rs` - `PeerManager::announce_block` Headers/Inv-only emission.
- `packages/open-bitcoin-network/src/peer/compact_relay.rs` - `CompactAnnouncementAction`, `CompactAnnouncementDecision`, `decide_compact_announcement`.
- `packages/open-bitcoin-node/src/network/block_relay_evidence.rs` - `record_announcement` / `CompactAnnounced` counter (false-positive risk).
- `packages/open-bitcoin-codec/src/compact_block.rs` - `CompactBlockPayload`, short IDs, encode/decode helpers.
- `packages/open-bitcoin-network/src/message.rs` - `WireNetworkMessage::CompactBlock`.
- `docs/parity/source-breadcrumbs.json` - required breadcrumb registry for new first-party Rust source/test files.
- `scripts/verify.sh` - repo-native verification contract.

### Bitcoin Knots Anchors

- `packages/bitcoin-knots/src/net_processing.cpp` - high-bandwidth compact block announcement and headers/inv fallback.
- `packages/bitcoin-knots/src/blockencodings.h` - BIP152 compact block structures.
- `packages/bitcoin-knots/src/blockencodings.cpp` - short ID and compact-block construction.
- `packages/bitcoin-knots/src/protocol.h` - `cmpctblock` command and inventory constants.
- `packages/bitcoin-knots/test/functional/p2p_compactblocks.py` - compact announcement behavior examples.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets

- `CompactAnnouncementAction` / `CompactAnnouncementDecision` / `decide_compact_announcement` already encode the policy outcome this phase must honor on the wire.
- `WireNetworkMessage::CompactBlock(CompactBlockPayload)` and codec encode/decode already exist from Phase 112.
- Short-ID helpers (`short_id_selector_from_header_and_nonce`, match keys) exist in the codec crate and are used by reconstruction tests.
- Block-relay evidence counters already distinguish compact announced vs headers/inventory fallback vs suppress.

### Established Patterns

- Functional-core peer policy in `open-bitcoin-network`; node adapter records sanitized evidence and returns the outbound message.
- Phase 113 tests prove decision outcomes without requiring wire CompactBlock emission — Phase 118 must add emission proofs without regressing those policy tests.
- Parity breadcrumbs required for new Rust source/test files.

### Integration Points

- Seam: `ManagedPeerNetwork::announce_block` → `decide_compact_announcement_for_peer` → (gap) → `PeerManager::announce_block`.
- Evidence seam: `record_compact_announcement_evidence(announcement.reason)` currently runs before emission and trusts the decision reason.

</code_context>

<deferred>
## Deferred Ideas

- Phase 119: mempool/extra candidate injection into compact receive and mempool-remove lifecycle hooks.
- Phase 120: compact-download timeout scheduling and misbehavior escalation beyond silent suppress.
- Phase 121: DurableSyncRuntime metrics/log projection for block-relay series.
- Package relay, bloom/filter serving, compact filters, public serving defaults, public-network CI, production full-node readiness, and production-funds wallet safety remain out of scope.

</deferred>
