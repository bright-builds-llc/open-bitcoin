# Phase 112: BIP152 Wire Codec and Message Semantics - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in `112-CONTEXT.md` - this log preserves the alternatives considered.

**Date:** 2026-07-04T19:37:55.303Z
**Phase:** 112-BIP152 Wire Codec and Message Semantics
**Mode:** Yolo
**Areas discussed:** Message Surface, Compact Block Payloads, Block Transaction Round Trips, Malformed Payload Boundary, Runtime Scope, Verification And Parity

---

## Message Surface

| Option | Description | Selected |
| --- | --- | --- |
| Explicit BIP152 variants | Add `sendcmpct`, `cmpctblock`, `getblocktxn`, and `blocktxn` variants to `WireNetworkMessage` with dedicated payload types. | yes |
| Reuse existing block/transaction variants | Treat compact relay payloads as existing block or transaction messages. | |
| Leave as unknown commands | Defer all BIP152 command handling to later phases. | |

**User's choice:** Auto-selected explicit BIP152 variants.
**Notes:** This matches Phase 112 requirements and avoids smuggling runtime policy into existing message variants.

---

## Compact Block Payloads

| Option | Description | Selected |
| --- | --- | --- |
| Pure structural payload model | Decode header, nonce, six-byte short IDs, and prefilled transactions while leaving reconstruction to later phases. | yes |
| Full reconstruction model now | Decode and immediately model mempool reconstruction, collisions, and fallback. | |
| Opaque byte payload | Store compact blocks as raw bytes until later runtime phases. | |

**User's choice:** Auto-selected pure structural payload model.
**Notes:** Keeps the phase inside CMP-02 and RCN-01 without taking Phase 114/115 scope.

---

## Block Transaction Round Trips

| Option | Description | Selected |
| --- | --- | --- |
| Differential index codecs with witness transactions | Implement `getblocktxn` and `blocktxn` payloads using Knots-style differential indexes and existing witness transaction serialization. | yes |
| Absolute indexes | Decode request indexes as final absolute positions only. | |
| No-witness transaction path | Add separate no-witness block transaction messages now. | |

**User's choice:** Auto-selected differential index codecs with witness transactions.
**Notes:** Knots `BlockTransactionsRequest` uses differential encoding and `BlockTransactions` serializes transactions with witness data.

---

## Malformed Payload Boundary

| Option | Description | Selected |
| --- | --- | --- |
| Typed decode failures before state | Reject malformed compact payloads with typed codec/network errors before partial state is accepted. | yes |
| Peer-policy decisions in codec | Make codec decide ignore, disconnect, fallback, or misbehavior outcomes. | |
| Broad generic errors | Collapse BIP152 malformed cases into generic decode failures. | |

**User's choice:** Auto-selected typed decode failures before state.
**Notes:** This preserves functional-core boundaries and lets later peer-policy phases map malformed inputs to Knots-aligned behavior.

---

## Runtime Scope

| Option | Description | Selected |
| --- | --- | --- |
| Codec/network only | Touch pure codec and network message surfaces, leaving node runtime serving/reconstruction unchanged. | yes |
| Wire plus negotiation | Add per-peer `sendcmpct` state and compact announcement eligibility now. | |
| Wire plus reconstruction | Add mempool reconstruction and fallback now. | |

**User's choice:** Auto-selected codec/network only.
**Notes:** Phase 111 locked compact-block `getdata` as bounded but not served; Phase 113+ owns runtime policy.

---

## Verification And Parity

| Option | Description | Selected |
| --- | --- | --- |
| Focused unit and malformed fixtures | Add round-trip and malformed-input tests around each BIP152 payload with Knots breadcrumbs. | yes |
| Docs-only parity notes | Record intended behavior but defer executable tests. | |
| Public-network compact relay review | Use live peers to prove compact relay behavior in default verification. | |

**User's choice:** Auto-selected focused unit and malformed fixtures.
**Notes:** Default verification must remain deterministic and local; public-network compact relay review is out of scope for Phase 112.

---

## Claude's Discretion

- Exact Rust type/module names.
- Whether BIP152 byte helpers live in a new codec module or existing codec/network modules.
- Exact fixture construction and test names.
- Exact typed error names, as long as malformed outcomes remain precise and stable.

## Deferred Ideas

Compact relay negotiation, compact announcements, reconstruction, missing transaction scheduling, fallback, validation handoff, operator evidence rollout, parity/UAT release closeout, package relay, bloom/filter serving, compact filter serving, public serving defaults, public-network CI, archive-node claims, production full-node readiness, production-service operation, and production-funds wallet use remain deferred to later phases.
