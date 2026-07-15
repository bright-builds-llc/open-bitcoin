# Phase 123: Runtime Timing and Evidence Integrity - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md; this log preserves the alternatives considered.

**Date:** 2026-07-15
**Phase:** 123-runtime-timing-and-evidence-integrity
**Mode:** Yolo
**Areas discussed:** Deterministic idle timeout scheduling, Successful block-emission evidence, Authoritative runtime projection

***

## Deterministic Idle Timeout Scheduling

| Option | Description | Selected |
| --- | --- | --- |
| Caller-clocked maintenance pulse in the live session driver | Distinguish socket idle from EOF, retain the live session, advance an injected clock, expire timeouts, and emit fallback through the owning session. | ✓ |
| Async receive/timer selection with an injected ticker | Move the session surface to async selection and deterministic paused-time tests. | |
| Dedicated scheduler thread signaling the session owner | Add an independent timer thread and channel maintenance work back to the blocking session owner. | |

**Agent's choice:** Caller-clocked maintenance pulse in the live session driver.
**Notes:** This is the smallest thin-shell change that can emit fallback while the peer remains connected. A daemon-loop-only tick cannot use a session that the inner loop has already dropped.

## Successful Block-Emission Evidence

| Option | Description | Selected |
| --- | --- | --- |
| Typed post-write acknowledgement into `ManagedPeerNetwork` | Acknowledge an actual typed block message after each successful transport write and maintain one authoritative sanitized counter. | ✓ |
| Count when `serve_inventory` appends a block | Count construction/enqueue before the transport effect is known. | |
| Receipt-bearing wire-emitter abstraction | Generalize encode/write/acknowledge ordering across transports. | |
| Transport-local counters merged during projection | Count write success separately in each adapter and aggregate later. | |

**Agent's choice:** Typed post-write acknowledgement into `ManagedPeerNetwork`.
**Notes:** This follows the achieved-effect evidence precedent and preserves partial-batch truth without splitting counter ownership.

## Authoritative Runtime Projection

| Option | Description | Selected |
| --- | --- | --- |
| Sample one runtime-owned snapshot per sync tick | Read the authoritative runtime network once and feed the same availability-gated value to metrics and logs. | ✓ |
| Publish a runtime-owned typed evidence snapshot | Add a shared snapshot lifecycle that broader status consumers may read. | |
| Share the live `ManagedPeerNetwork` between sync and RPC | Put all consumers behind the same mutable network instance. | |

**Agent's choice:** Sample one runtime-owned snapshot per sync tick.
**Notes:** The current provider returns the right type from the wrong network instance. Direct sampling fixes provenance without a shared-mutable ownership rewrite.

## Agent's Discretion

- Exact helper and type names for idle outcomes, clock injection, post-write acknowledgement, and tick-local snapshot plumbing.
- The narrowest module split that keeps network decisions pure and transport effects in adapters.

## Deferred Ideas

- Async networking redesign, shared mutable network ownership across RPC/sync, broad cross-surface snapshot publication, and generalized delivery receipts remain outside Phase 123.
