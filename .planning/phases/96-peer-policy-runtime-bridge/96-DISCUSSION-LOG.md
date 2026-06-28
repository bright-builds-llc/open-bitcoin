# Phase 96: Peer Policy Runtime Bridge - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in `96-CONTEXT.md`; this log preserves alternatives considered.

**Date:** 2026-06-28T02:43:32.273Z
**Phase:** 96-peer-policy-runtime-bridge
**Mode:** Yolo
**Areas discussed:** Managed peer-policy runtime state bridge, reconnect suppression and runtime admission integration, shared operator evidence and deterministic checker boundary

---

## Managed Peer-Policy Runtime State Bridge

| Option | Description | Selected |
|--------|-------------|----------|
| Append bounded decision history in `ManagedPeerNetwork` | Small bridge from existing decisions, but not authoritative for scoped reconnect checks. | |
| Node-owned `ManagedPeerPolicyRuntimeState` | Localizes managed projection, but risks duplicating pure policy matching. | |
| `PeerManager`-owned pure `PeerPolicyState` plus thin node projection | Keeps one pure source of truth near `PeerBanBook` and `MisbehaviorPolicy`; supports scoped expiry, unban, discourage, and deterministic tests. | yes |
| Fjall-backed policy store as runtime authority | Strong durability, but too much storage/schema and claim surface for this bridge phase. | |

**Selected answer:** Use a pure `PeerManager`-owned, or equivalent `open-bitcoin-network`-owned, runtime policy state with thin managed projection.

**Notes:** The current gap is that `ManagedPeerNetwork::peer_policy_info()` projects eviction plus empty misbehavior, ban, and unban decision slices. The fix should feed actual bounded runtime policy decisions into the existing projection without turning status/support into a public banlist.

---

## Reconnect Suppression And Runtime Admission Integration

| Option | Description | Selected |
|--------|-------------|----------|
| Scoped managed peer-policy runtime state | Uses `remote_addr.ip()` plus injected `now`, avoids aggregate broad suppression, and records existing bounded reconnect labels. | yes |
| Direct durable-store lookup during accept | Durable source of truth, but adds I/O and lock/error handling to the hot accept path. | |
| Admission-policy integrated rejection | Single connection-decision pipeline, but mixes ban/discourage policy into Phase 90 admission concepts. | |

**Selected answer:** Query scoped runtime peer-policy state by remote IP and timestamp, then translate the scoped outcome into existing reconnect suppression evidence.

**Notes:** The current `ManagedRpcContext::reconnect_suppression_input_for_remote_addr` ignores `remote_addr` and `now_unix_seconds`, then derives booleans from aggregate counters. Phase 96 must prove matching and non-matching remotes so one active ban does not suppress unrelated addresses.

---

## Shared Operator Evidence And Deterministic Checker Boundary

| Option | Description | Selected |
|--------|-------------|----------|
| Snapshot-first peer-policy bridge | Reuses shared inbound status, exposes aggregate counters plus latest sanitized event, and keeps scoped state internal. | yes |
| Durable peer-policy event ledger with snapshot projection | Strong replayability, but larger storage, redaction, support-bundle, and claim risk. | |
| Resource-governance piggyback | Smallest change, but collapses peer-policy semantics into resource-governance evidence. | |

**Selected answer:** Use the shared inbound status snapshot as the public evidence boundary, with scoped state kept internal and only low-cardinality sanitized events projected.

**Notes:** Add a deterministic checker if docs/parity/verifier surfaces change. The checker should reject empty policy decision slices, aggregate-only reconnect suppression, raw peer-policy leaks, public-network verification, and production or relay claim creep.

---

## the agent's Discretion

- Exact type and module names for the pure policy state.
- Whether to add a narrow wrapper around `PeerBanBook` or a new `PeerPolicyState` type.
- Exact low-cardinality labels for new latest-event or checker evidence, as long as existing Phase 93/94 labels remain stable where possible.

## Deferred Ideas

- Durable cross-restart peer-policy event replay.
- Public banlist-style operator management.
- Phase 97 metric sample production.
- Transaction relay, mempool propagation, compact block relay, full address relay, public inbound defaults, public-network CI, production service packaging, and production full-node readiness.
