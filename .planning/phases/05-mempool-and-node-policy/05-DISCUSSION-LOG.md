# Phase 5: Mempool and Node Policy - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-04-13
**Phase:** 05-mempool-and-node-policy
**Areas discussed:** crate boundary, admission scope, replacement policy, accounting model, parity closure

---

## Crate Boundary

| Option | Description | Selected |
|--------|-------------|----------|
| Extend `open-bitcoin-chainstate` | Mix mempool state into the chainstate crate | |
| New pure-core `open-bitcoin-mempool` crate | Keep mempool policy and state isolated but reusable | ✓ |
| Keep everything in `open-bitcoin-node` | Fastest path, but shell-owned policy | |

**User's choice:** Recommended default applied: new pure-core `open-bitcoin-mempool` crate.
**Notes:** This preserves the Phase 4 chainstate boundary and matches the repo's pure-core crate pattern.

---

## Admission Scope

| Option | Description | Selected |
|--------|-------------|----------|
| Minimal accept-or-reject only | No standardness or accounting, just prevout checks | |
| Targeted policy slice | Standardness, relay fee, conflict checks, ancestor or descendant limits, eviction | ✓ |
| Full package-relay parity | Include package admission, rolling minimum fee, and repair flows now | |

**User's choice:** Recommended default applied: targeted policy slice.
**Notes:** Package relay and repair remain deferred so the phase stays inside roadmap scope.

---

## Replacement Policy

| Option | Description | Selected |
|--------|-------------|----------|
| No replacement support yet | Reject all mempool conflicts | |
| Targeted RBF rules | Higher absolute fee, better feerate, incremental bump, no new unconfirmed inputs | ✓ |
| Full Knots replacement matrix | Port every helper and conflict-topology edge now | |

**User's choice:** Recommended default applied: targeted RBF rules.
**Notes:** The phase still needs explicit replacement behavior, but the first slice should stay deterministic and auditable.

---

## Accounting Model

| Option | Description | Selected |
|--------|-------------|----------|
| Incremental cache-style bookkeeping | Mirror Knots' mutable descendant caches directly | |
| Deterministic recomputation after each mutation | Rebuild parent or child and ancestor or descendant stats for correctness-first parity fixtures | ✓ |
| No ancestor or descendant stats yet | Defer accounting to a later phase | |

**User's choice:** Recommended default applied: deterministic recomputation after each mutation.
**Notes:** This favors correctness and simpler reasoning over scale in the first mempool slice.

---

## Parity Closure

| Option | Description | Selected |
|--------|-------------|----------|
| Code-only completion | No explicit parity ledger update | |
| Targeted fixtures plus parity catalog update | Close the phase with repo-owned tests and explicit deviation tracking | ✓ |
| Networking/RPC-backed harness now | Couple policy closure to later surfaces | |

**User's choice:** Recommended default applied: targeted fixtures plus parity catalog update.
**Notes:** The ledger should make remaining out-of-scope policy behavior visible instead of implying full node-surface completion.

---

## the agent's Discretion

- Exact module split inside `open-bitcoin-mempool`
- Whether the thin node wrapper is named `ManagedMempool` or similar
- Exact fixture values for relay fee, size-limit eviction, and ancestor-chain tests

## Deferred Ideas

- Package relay beyond single-transaction admission
- Rolling minimum fee and long-running fee decay behavior
- Reorg-driven mempool repair

---

*Phase: 05-mempool-and-node-policy*  
*Discussion log generated: 2026-04-13*
