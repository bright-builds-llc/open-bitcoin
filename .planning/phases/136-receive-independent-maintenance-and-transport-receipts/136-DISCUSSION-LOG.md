# Phase 136: Receive-Independent Maintenance and Transport Receipts - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-08-15
**Phase:** 136-receive-independent-maintenance-and-transport-receipts
**Mode:** Yolo
**Areas discussed:** Unbroadcast-clear receipt, Retry cycle timing, Per-tick work caps, Parent-before-child package fanout, Maintenance wakeup

---

## Unbroadcast-clear receipt

| Option | Description | Selected |
|--------|-------------|----------|
| EligibleServe at GETDATA accept (before write) | Clear when serving classifies GETDATA as serveable, matching Knots send-buffer push timing | |
| Successful TX write receipt (`TransportWritten`) | Clear only after `acknowledge_write` of `WireNetworkMessage::Tx` | ✓ |
| Both: EligibleServe semantic, TransportWritten applying receipt | Keep IBR-04 dual wording; do not emit a pre-write EligibleServe clear | ✓ (semantic + applying split) |
| Successful INV write | Clear when inventory is written | rejected |

**User's choice:** [auto] Successful TX write is the applying receipt; EligibleServe is documented semantic only and must not clear before write. LifecycleRemoval still wins. Never clear on INV.
**Notes:** Recommended because Phase 134 already completes transport only after successful write. Clearing at classify time would stop retry after a failed send.

---

## Retry cycle timing

| Option | Description | Selected |
|--------|-------------|----------|
| Process-global fresh cycle (Knots-like) | First and later cycles are 10–15 minutes from start/recovery/last tick; restart remints | ✓ |
| Per-member due from admission or last attempt | Independent due time per txid | |
| Process-global cycle with restored leftover due | Persist or reconstruct leftover wait across restart | |

**User's choice:** [auto] Process-global fresh cycle from node start or recovery install; restart remints; first announcement stays immediate existing fanout.
**Notes:** Matches `ReattemptInitialBroadcast` and `mempool_unbroadcast.py` post-restart wait. Derived timers are not durable source state.

---

## Per-tick work caps

| Option | Description | Selected |
|--------|-------------|----------|
| Reuse PHASE104 fanout queue/drain caps only | 1024/16 are the only bounds | |
| Separate tick inspect-N and prepare-M caps | Tick budget distinct from membership and drain caps | ✓ |
| Hard-cap only via 5,000-member set and allow a full-set tick | Knots-like whole-set walk | |
| Spill leftover due members unattempted | Continuation policy for a decision cap | ✓ (paired with separate caps) |

**User's choice:** [auto] Separate inspect/prepare budgets, then existing PHASE104 path; leftovers spill as still-due and unattempted with a deterministic cursor.
**Notes:** IBR-02 requires per-tick decision and emission caps. The 5,000-member cap is membership capacity, not tick work.

---

## Parent-before-child package fanout

| Option | Description | Selected |
|--------|-------------|----------|
| Enqueue parent-before-child into existing per-peer FIFO | Drain keeps current rate/identity/eligibility | ✓ |
| Require emit/write parent-before-child across peers and ticks | Hard write barrier | |
| Order only co-admitted package members | Independently admitted txs use ordinary single-tx fanout | ✓ (paired with enqueue) |
| Topologically order any still-present ancestors | Broader than PPKG-04 | |

**User's choice:** [auto] Enqueue co-admitted still-present package members parent-before-child into the existing FIFO. Independently admitted members stay on ordinary single-tx fanout. No cross-peer emit barrier.
**Notes:** PPKG-04 is enter-existing-fanout order, not a new wire protocol or ancestor re-announce.

---

## Maintenance wakeup

| Option | Description | Selected |
|--------|-------------|----------|
| Shell-owned injected timer in node/daemon runtime | Wakes with zero inbound messages; one `MaintenanceTick` | ✓ |
| Piggyback on inbound/outbound event loops | Not receive-independent | rejected |
| Couple retry timer to DurableSyncRuntime / IBD | Mixes relay policy with sync | rejected |
| Fake-clock / injected now+jitter seam | Production samples in shell; tests inject; no sleeps | ✓ (required companion) |

**User's choice:** [auto] Shell-owned timer plus fake-clock seam. Not receive-loop piggyback. Not DurableSyncRuntime.
**Notes:** IBR-02 and research ARCHITECTURE.md already specify `MaintenanceTick { now, jitter }` from the shell.

---

## Claude's Discretion

- Exact inspect-N and prepare-M constants
- Exact module, command, cursor, and receipt-filter names
- Whether the shell timer lives in `open-bitcoind` or a node runtime adapter

## Deferred Ideas

- Phase 137 operator evidence presentation
- Phase 138 parity/claim guardrails
- Whole-mempool rebroadcast, package wire protocol, public/default relay
