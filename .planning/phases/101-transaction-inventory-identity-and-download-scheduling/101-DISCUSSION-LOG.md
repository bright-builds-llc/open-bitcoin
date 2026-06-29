# Phase 101: Transaction Inventory Identity and Download Scheduling - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md - this log preserves the alternatives considered.

**Date:** 2026-06-29T21:03:44.720Z
**Phase:** 101-Transaction Inventory Identity and Download Scheduling
**Mode:** Yolo
**Areas discussed:** Inventory identity and negotiation, Per-peer request state, Download scheduling, Received transaction cleanup, Typed actions and evidence, Tests and parity guardrails

***

## Inventory Identity And Negotiation

| Option | Description | Selected |
|--------|-------------|----------|
| Typed relay identity | Parse wire inventory into a txid/wtxid enum before pure scheduling logic. | yes |
| Keep raw inventory vectors | Continue passing `InventoryType` plus `Hash32` through peer state. | |
| Adapter-only interpretation | Let managed runtime inspect inventory tags and keep peer logic shallow. | |

**User's choice:** Auto-selected typed relay identity.
**Notes:** This matches the repo's parse-at-boundaries rule, keeps `MSG_TX` and `MSG_WTX` behavior auditable, and prevents txid/wtxid mismatch handling from becoming string or tag checks scattered across adapters.

## Per-Peer Request State

| Option | Description | Selected |
|--------|-------------|----------|
| Rich scheduler state | Track announcements, in-flight requests, timestamps, fallback peers, and cleanup reasons. | yes |
| Simple requested sets | Keep only `requested_txids` and `requested_wtxids`. | |
| Runtime-owned state | Move request lifecycle into `open-bitcoin-node`. | |

**User's choice:** Auto-selected rich scheduler state.
**Notes:** The current sets prove the seam exists but are not enough for duplicate announcement retention, timeout fallback, `notfound` cleanup, or disconnect cleanup required by INV-02, INV-03, DL-01, and DL-02.

## Download Scheduling

| Option | Description | Selected |
|--------|-------------|----------|
| Pure fake-clock scheduler | Scheduler accepts timestamps and emits typed request/fallback/expiry actions. | yes |
| Immediate getdata on inv | Keep current direct `GetData` emission from `handle_inventory`. | |
| Wall-clock runtime scheduler | Use runtime timers directly from managed networking. | |

**User's choice:** Auto-selected pure fake-clock scheduler.
**Notes:** Knots anchors use delayed and expiring transaction requests; deterministic fake-clock tests let Open Bitcoin prove those paths without sleeps, public-network checks, or socket side effects.

## Received Transaction Cleanup

| Option | Description | Selected |
|--------|-------------|----------|
| Match and cleanup by derived txid/wtxid | Derive both IDs once, clear matching requests, emit mismatch labels for non-matching responses. | yes |
| Accept any tx from peer | Treat any received `tx` as satisfying outstanding state. | |
| Delay cleanup until admission | Let mempool admission own request cleanup. | |

**User's choice:** Auto-selected match and cleanup by derived txid/wtxid.
**Notes:** This keeps request lifecycle pure and prevents mismatched transactions from accidentally satisfying unrelated in-flight requests. Mempool admission remains a later phase.

## Typed Actions And Evidence

| Option | Description | Selected |
|--------|-------------|----------|
| Fixed typed actions and labels | Emit stable request, suppression, fallback, expiry, notfound, and cleanup actions. | yes |
| Boolean request decisions | Return only request/no-request booleans. | |
| Log-first evidence | Represent scheduler outcomes primarily as log text. | |

**User's choice:** Auto-selected fixed typed actions and labels.
**Notes:** Stable typed actions are easier to test and can feed later metrics/logs/support surfaces without exposing raw transaction or peer material.

## Tests And Parity Guardrails

| Option | Description | Selected |
|--------|-------------|----------|
| Broad deterministic matrix | Cover txid/wtxid paths, mismatch, duplicates, caps, already-have, recent reject, timeout, notfound, disconnect, and received tx cleanup. | yes |
| Minimal happy-path tests | Only prove txid and wtxid requests are emitted. | |
| Public-network relay UAT | Add live relay checks to default verification. | |

**User's choice:** Auto-selected broad deterministic matrix.
**Notes:** The phase's verification contract is pure `open-bitcoin-network` tests, fake-clock expiry tests, and `bash scripts/verify.sh`. Public-network relay review stays opt-in and outside default verification.

## the agent's Discretion

- Exact Rust type names and module split.
- Exact scheduler constant names and whether values mirror Knots constants one-for-one or use locally documented equivalents.
- Whether the scheduler lives under `peer/inventory_state.rs` initially or in a new child module.

## Deferred Ideas

- Orphan staging, parent requests, and admission outcome bridges belong to Phase 102.
- Durable mempool lifecycle belongs to Phase 103.
- Relay serving, fanout, and rebroadcast belong to Phase 104.
- Operator, RPC, metrics, logs, and support evidence belong to Phase 105.
- Parity closeout, UAT, and release-boundary guardrails belong to Phase 106.
