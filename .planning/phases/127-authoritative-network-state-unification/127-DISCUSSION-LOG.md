# Phase 127: Authoritative Network State Unification - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md; this log preserves the alternatives considered.

**Date:** 2026-07-19
**Phase:** 127-authoritative-network-state-unification
**Mode:** Yolo
**Areas discussed:** Authoritative runtime ownership, Durable block serving source, Shared operator truth, Production-path guardrails

***

## Authoritative Runtime Ownership

| Option | Description | Selected |
| --- | --- | --- |
| Shared authoritative handle | One network allocation behind typed mutation and snapshot APIs; smallest architecture-preserving repair. | ✓ |
| Single-owner runtime coordinator | Commands and queues make ownership explicit but require a broad sync/inbound/RPC redesign. | |
| Authoritative core plus peer shells | Cleaner long-term decomposition but risks weakening the literal one-network phase criterion during migration. | |

**Recommended choice:** Shared authoritative handle.
**Rationale:** It directly closes the audited split authority with minimal dependency impact. Lock guards must remain internal and must not span I/O, `.await`, persistence, or serialization.

## Durable Block Serving Source

| Option | Description | Selected |
| --- | --- | --- |
| Lazy Fjall-backed block-source seam | Gate against authoritative chainstate, then load the requested durable block body on demand. | ✓ |
| Hydrate durable blocks into memory | Rebuild the in-memory inventory at startup, increasing memory use and archive-like implications. | |
| Keep cache-only serving | Minimal code change but leaves validated durable blocks unavailable after restart or cache loss. | |

**Recommended choice:** Lazy Fjall-backed block-source seam.
**Rationale:** It preserves policy-before-read behavior and bounded memory while making durable sync blocks available to inbound serving.

## Shared Operator Truth

| Option | Description | Selected |
| --- | --- | --- |
| One aggregate snapshot from the shared authority | Preserve existing schemas/renderers while changing all consumers to authoritative provenance. | ✓ |
| Versioned projection into a second network | Detects staleness but retains split mutable authority and does not satisfy the phase criterion. | |
| Redesign operator contracts | Could expose a new model but needlessly expands scope and risks compatibility regressions. | |

**Recommended choice:** One aggregate snapshot from the shared authority.
**Rationale:** Existing RPC, CLI/dashboard, metrics/log, and support contracts already encode the required redaction and low-cardinality behavior.

## Production-Path Guardrails

| Option | Description | Selected |
| --- | --- | --- |
| Focused Phase 127 tests and narrow checker | Reject duplicate production construction, cache-only serving, and non-authoritative status within this phase. | ✓ |
| Defer every guard to Phase 129 | Avoids checker work now but violates Phase 127 success criteria and permits the same integration regression. | |
| Move full milestone reconciliation into Phase 127 | Adds broad protection but crosses the approved Phase 129 boundary. | |

**Recommended choice:** Focused Phase 127 tests and narrow checker.
**Rationale:** Phase 127 must prove its repaired seams locally, while Phase 129 retains aggregate cross-phase and archival ownership.

## the agent's Discretion

- Exact shared-handle and synchronization type.
- Typed poison/runtime error mapping and snapshot API names.
- Durable block-source trait/module shape and cache interaction.
- Focused production integration fixtures and deterministic checker implementation.

## Deferred Ideas

- Single-owner actor/coordinator runtime redesign.
- Phase 128 production compact-announcement transport.
- Phase 129 aggregate integration guards and milestone reconciliation.
- Public defaults, public-network CI, archive-node behavior, production-readiness claims, package/filter relay, GUI, and hosted services.
