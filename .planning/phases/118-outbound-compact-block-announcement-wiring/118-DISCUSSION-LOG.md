# Phase 118: Outbound Compact Block Announcement Wiring - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in `118-CONTEXT.md`; this log preserves alternatives considered by the yolo recommendation pass.

**Date:** 2026-07-11
**Phase:** 118-outbound-compact-block-announcement-wiring
**Mode:** Yolo
**Areas discussed:** Action honor path, compact payload construction, evidence correctness, fallback/suppression, verification/parity

## Action Honor Path

| Option | Description | Selected |
| --- | --- | --- |
| Honor action in ManagedPeerNetwork plus PeerManager emit helper | Branch on `CompactAnnouncementAction` in the node adapter and extend peer announce emission so CompactBlock/Headers/Inv/None are produced from the typed action. | ✓ |
| Only patch ManagedPeerNetwork and leave PeerManager Headers/Inv-only | Build CompactBlock only in the node crate and keep peer announce unchanged. | |
| Reopen Phase 113 policy and change decision semantics | Treat the gap as a policy bug rather than a wiring bug. | |

**Auto-selected choice:** Honor action in ManagedPeerNetwork plus PeerManager emit helper.
**Notes:** The audit shows the decision is already correct; emission ignores it. Keep policy ownership in Phase 113 types and close the runtime seam.

## Compact Payload Construction

| Option | Description | Selected |
| --- | --- | --- |
| Production Block→CompactBlockPayload builder with BIP152 announce shape | Use codec short-ID/prefilled helpers; prefer Knots-aligned announce construction for a fully known local block. | ✓ |
| Prefill every transaction always | Skip short IDs and send the full block as prefilled entries only. | |
| Test fixtures only / no production builder | Keep using hand-built payloads in tests and leave production unwired. | |

**Auto-selected choice:** Production Block→CompactBlockPayload builder with BIP152 announce shape.
**Notes:** Audit explicitly calls out missing production builder. Prefill-everything is a last-resort fallback if research proves announce-shape construction is unsafe for this boundary.

## Evidence Correctness

| Option | Description | Selected |
| --- | --- | --- |
| Increment CompactAnnounced only after CompactBlock emission | Record evidence from the emitted outcome, not the pre-emission decision alone. | ✓ |
| Keep recording from decision reason before emission | Preserve current `record_compact_announcement_evidence(announcement.reason)` timing. | |
| Drop announce evidence entirely in this phase | Defer counters to Phase 121. | |

**Auto-selected choice:** Increment CompactAnnounced only after CompactBlock emission.
**Notes:** False-positive announce evidence is part of the CMP-05 gap. Phase 121 is metrics projection, not announce-counter correctness.

## Fallback And Suppression

| Option | Description | Selected |
| --- | --- | --- |
| Preserve Headers/Inv/Suppress paths; construction failure must not claim CompactAnnounced | Keep existing fallback actions; on builder failure fall back or suppress with stable reasons. | ✓ |
| Treat builder failure as panic/hard error | Fail the announce call without typed fallback. | |
| Always suppress when compact is decided but builder is incomplete | Never fall back to Headers/Inv on construction issues. | |

**Auto-selected choice:** Preserve Headers/Inv/Suppress paths; construction failure must not claim CompactAnnounced.
**Notes:** Matches Phase 113 typed fallback posture and avoids false-positive compact announce evidence.

## Verification And Parity

| Option | Description | Selected |
| --- | --- | --- |
| Runtime/unit proofs for emission + evidence + breadcrumbs | Prove CompactBlock emission, fallback paths, evidence gating, and parity breadcrumbs; keep verify deterministic. | ✓ |
| Docs-only gap closure | Update audit prose without wiring tests. | |
| Require public-network UAT for phase pass | Block phase completion on live mainnet compact announce. | |

**Auto-selected choice:** Runtime/unit proofs for emission + evidence + breadcrumbs.
**Notes:** Public-network review remains opt-in UAT outside default verification.

## Claude's Discretion

- Exact helper placement and announce API shape.
- Nonce selection strategy for outbound compact payloads.
- Smallest API change that keeps action honor and evidence correctness testable.

## Deferred Ideas

- Phase 119 mempool candidate injection and lifecycle hooks.
- Phase 120 timeout scheduling and misbehavior escalation.
- Phase 121 DurableSyncRuntime metrics/log projection.
- Broader relay/filter/public-default/production surfaces remain outside Phase 118.
