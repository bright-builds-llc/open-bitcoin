# Phase 126: Compact Relay Residual Hardening - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-07-18T16:09:20.458Z
**Phase:** 126-compact-relay-residual-hardening
**Mode:** Yolo
**Areas discussed:** Production receive invariant, Compact-block nonce source, Evidence and final reconciliation

## Production Receive Invariant

| Option | Description | Selected |
| --- | --- | --- |
| Fail-closed generic dispatch plus explicit factful receive API | Remove the implicit default-empty fallback, preserve the managed node shell as the production candidate source, and retain direct factful tests. | ✓ |
| Routed non-compact message type | Make generic compact dispatch impossible at compile time by wrapping or splitting the wire message type. | |
| Inject a compact-candidate provider port into `PeerManager` | Let generic dispatch acquire snapshots through a provider abstraction. | |

**User's choice:** Auto-selected the recommended fail-closed generic dispatch plus explicit factful receive API.
**Notes:** Explicitly supplied facts may contain empty slices when the live mempool and bounded extra buffer are genuinely empty. The selected design removes `Default`, uses a typed adapter-routing failure distinct from peer misbehavior, and preserves the `open-bitcoin-network` / `open-bitcoin-mempool` separation.

## Compact-Block Nonce Source

| Option | Description | Selected |
| --- | --- | --- |
| Call-scoped `getrandom` adapter in `open-bitcoin-node` with an injected source | Generate a fresh production `u64` in the node shell, inject fixed/failing sources in tests, and keep the consensus builder pure. | ✓ |
| Stored `CompactNonceSource` port on `ManagedPeerNetwork` | Store a reusable fixed/sequence/failing source behind the managed network API. | |
| Seeded stateful CSPRNG in the node shell | Seed and retain a Knots-like generator for repeated nonce draws. | |

**User's choice:** Auto-selected the recommended call-scoped system-entropy adapter with deterministic injection.
**Notes:** Entropy is acquired lazily through a call-scoped fallible closure only for compact announcements. Fixed, failing, and invocation-counting closures keep tests deterministic; failure uses the existing safe fallback and cannot record compact-announced achieved-effect evidence.

## Evidence And Final Reconciliation

| Option | Description | Selected |
| --- | --- | --- |
| Staged Phase 126 closeout: candidate → verified promotion → archive-ready | Keep requirements and audit pending through runtime/parity checks, then promote only after lifecycle-valid verification and the full verifier pass. | ✓ |
| Atomic post-gate projection | Move all six requirements, the audit, and archive routing in one final metadata change. | |
| Separate Phase 127 reconciliation | Split runtime/parity completion and archive authorization into different phases. | |

**User's choice:** Auto-selected the recommended staged Phase 126 closeout.
**Notes:** Extend the Phase 124 guard for candidate, verified-pre-promotion, promoted-pre-summary, and archive-ready Phase 126 states, retain the generic active-milestone traceability checker, and block archive routing on any fresh genuine gap.

## the agent's Discretion

- Exact typed routing error and named test helper for intentional empty facts.
- Exact nonce-source function signature and safe headers/inventory fallback selection.
- Plan split between runtime hardening, parity/checker work, and final metadata promotion.

## Deferred Ideas

- Compile-time message routing if production adapter count grows.
- Stored or seeded RNG state if randomness consumers or compact-announcement volume grow.
- All existing v2.1 deferred relay, filter, public-default, CI, archive-node, production-readiness, wallet, packaging, and GUI surfaces remain deferred.
