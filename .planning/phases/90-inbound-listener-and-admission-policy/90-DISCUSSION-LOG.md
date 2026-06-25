# Phase 90: Inbound Listener and Admission Policy - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md - this log preserves the alternatives considered.

**Date:** 2026-06-25T04:23:47.878Z
**Phase:** 90-Inbound Listener and Admission Policy
**Mode:** Yolo
**Areas discussed:** Activation and preflight, Admission lifecycle, Caps and outbound safety, Operator evidence and verification

---

## Activation And Preflight

| Option | Description | Selected |
|--------|-------------|----------|
| Disabled-by-default Open Bitcoin controls | Add Open Bitcoin-owned JSONC and daemon CLI controls; disabled path never binds. | yes |
| Baseline `bitcoin.conf` `listen`/`bind` compatibility now | Start accepting Knots/Core listener keys in this phase. | |
| Runtime-only hardcoded loopback listener | Implement only a test listener without operator config. | |

**User's choice:** Yolo recommended default: disabled-by-default Open Bitcoin-owned controls.
**Notes:** This matches v1.9 scope while avoiding a premature claim of full Knots listener configuration parity.

---

## Admission Lifecycle

| Option | Description | Selected |
|--------|-------------|----------|
| Pure admission policy plus thin socket shell | Model admission decisions before socket effects, then wire accept-loop behavior in `open-bitcoind`. | yes |
| Socket-first accept loop | Put most admission checks inside Tokio listener code. | |
| RPC/test-only admission | Keep using `ManagedRpcContext::add_inbound_peer` without real listener work. | |

**User's choice:** Yolo recommended default: pure admission policy plus thin socket shell.
**Notes:** This preserves the repo's functional-core boundary and reuses existing `PeerManager` handshake state.

---

## Caps And Outbound Safety

| Option | Description | Selected |
|--------|-------------|----------|
| Separate inbound caps from outbound targets | Model `max_inbound_peers` and `reserved_slots` without changing outbound sync targets. | yes |
| Shared global peer cap | Let inbound peers compete directly with outbound sync targets. | |
| Defer caps | Add listener and handshake first, leaving caps for a later phase. | |

**User's choice:** Yolo recommended default: separate inbound caps from outbound targets.
**Notes:** INB-04 requires caps and protected slots without starving outbound sync, while Phase 91 owns richer permission semantics.

---

## Operator Evidence And Verification

| Option | Description | Selected |
|--------|-------------|----------|
| Shared status/support evidence | Extend the existing status snapshot and support patterns with listener/admission fields. | yes |
| Local startup logs only | Use startup stderr/log lines as the primary evidence. | |
| New standalone evidence file only | Add a new report without integrating status, RPC, metrics, or support. | |

**User's choice:** Yolo recommended default: shared status/support evidence.
**Notes:** The existing operator surface already uses shared status contracts, unavailable reasons, redaction, metrics, logs, and support bundles.

---

## the agent's Discretion

- Exact module names and final CLI flag spellings may be refined by the planner if they preserve the Open Bitcoin-owned control boundary.
- The planner may split work into multiple plans if config, pure admission policy, runtime listener wiring, and evidence surfaces are too large for one safe implementation pass.

## Deferred Ideas

- Full Knots-compatible permissions, address relay, eviction/ban policy, DoS governance, transaction relay, compact block relay, public inbound defaults, and production-readiness claims are outside Phase 90.
