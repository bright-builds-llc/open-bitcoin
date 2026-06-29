# Phase 100: Relay Activation Boundary and Permission Semantics - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md - this log preserves the alternatives considered.

**Date:** 2026-06-29T16:18:03.921Z
**Phase:** 100-relay-activation-boundary-and-permission-semantics
**Mode:** Yolo
**Areas discussed:** Activation contract, Peer eligibility matrix, Scoped permission effects, Evidence/docs/guardrails

***

## Activation Contract

| Option | Description | Selected |
| --- | --- | --- |
| Default-off explicit activation | Add Open Bitcoin-owned activation settings and keep all transaction relay behavior disabled by default. | yes |
| Reuse existing relay-like labels implicitly | Treat existing `relay`, `forcerelay`, and `mempool` permission labels as enough to activate behavior. | |
| Defer activation policy entirely | Leave all relay-like labels inactive until later implementation phases. | |

**User's choice:** Auto-selected default-off explicit activation.
**Notes:** This is the only option that satisfies ACT-01 while preserving v1.9 no-claim boundaries.

## Peer Eligibility Matrix

| Option | Description | Selected |
| --- | --- | --- |
| Pure typed matrix | Model default, outbound, inbound, manual, protected, and permissioned peer eligibility as a pure policy with stable outcomes. | yes |
| Adapter-local checks | Let runtime socket and mempool paths decide eligibility as each later behavior is wired. | |
| Permission-only eligibility | Treat any permissioned peer as relay-eligible. | |

**User's choice:** Auto-selected pure typed matrix.
**Notes:** Protected admission remains separate from transaction relay. Outbound/manual peers need explicit relay activation; inbound peers need activation plus scoped relay-eligible permission/class evidence.

## Scoped Permission Effects

| Option | Description | Selected |
| --- | --- | --- |
| Promote only `relay`, `forcerelay`, and `mempool` into scoped v2.0 relay policy effects | Convert current inactive labels into eligibility/policy inputs without implementing sockets or mempool mutation in Phase 100. | yes |
| Keep all relay-like labels inactive | Preserve Phase 91 behavior exactly and let later phases introduce new concepts. | |
| Activate every Knots permission in `all` | Let `all` activate relay, mempool, bloom/filter, and block/filter behavior together. | |

**User's choice:** Auto-selected scoped v2.0 relay policy effects.
**Notes:** `bloomfilter`, `blockfilters`, compact-filter-like behavior, compact blocks, package relay, and public defaults remain inactive/deferred.

## Evidence, Docs, and Guardrails

| Option | Description | Selected |
| --- | --- | --- |
| Deterministic local guardrails | Add docs/checker evidence when relevant and keep default verification public-network-free. | yes |
| Runtime-only evidence | Skip docs/checkers and rely on unit tests only. | |
| Public-network relay UAT in default verification | Prove relay with live network checks in `bash scripts/verify.sh`. | |

**User's choice:** Auto-selected deterministic local guardrails.
**Notes:** Guardrails should fail public-default relay, compact block relay, bloom/filter serving, package relay, production, wallet, and public-network CI overclaims.

## the agent's Discretion

- Exact config key names.
- Rust type and module names.
- Whether Phase 100 needs a shared status field or only docs/tests/policy evidence.
- Precise wording of stable machine labels, as long as they are low-cardinality and typed.

## Deferred Ideas

- Transaction inventory identity and download scheduling.
- Orphan handling and mempool admission.
- Mempool lifecycle, persistence, and reorg behavior.
- Relay serving, fanout, local submission, and rebroadcast.
- Operator/RPC/metrics/log/support surfaces beyond Phase 100 evidence needs.
- v2.0 parity closeout and release guardrails.
