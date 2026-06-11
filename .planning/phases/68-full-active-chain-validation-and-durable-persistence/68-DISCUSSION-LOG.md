# Phase 68: Full Active-Chain Validation and Durable Persistence - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution
> agents.
> Decisions are captured in CONTEXT.md - this log preserves the alternatives
> considered.

**Date:** 2026-06-11T11:56:49.713Z
**Phase:** 68-full-active-chain-validation-and-durable-persistence
**Mode:** Yolo
**Areas discussed:** Validated progress credit, Durable persistence contract,
Runtime integration, Operator and scope boundaries, Verification posture

---

## Validated Progress Credit

| Option | Description | Selected |
| --- | --- | --- |
| Headers/downloads count as phase progress | Treat header or downloaded block advancement as enough evidence for this phase. | |
| Connected active-chain progress only | Credit block progress only after consensus validation, active-chain connection, and durable persistence. | yes |
| External tip oracle shortcut | Trust a centralized or hosted tip source to decide sync completion. | |

**User's choice:** Yolo recommendation selected connected active-chain progress
only.
**Notes:** This preserves the milestone claim that sync-to-tip evidence is
validated and first-party rather than downloaded-only or oracle-backed.

---

## Durable Persistence Contract

| Option | Description | Selected |
| --- | --- | --- |
| Complete chainstate snapshot first | Persist active chain, UTXOs, undo, headers, block index, block bodies, and runtime metadata using existing snapshot-style storage. | yes |
| Incremental persistence first | Design a more granular UTXO/undo write model before completing Phase 68. | |
| Runtime metadata only | Persist status counters without durable chainstate and UTXO/undo recovery. | |

**User's choice:** Yolo recommendation selected the complete snapshot first
path.
**Notes:** Incremental storage can be considered later if needed, but Phase 68
needs a simple restart-safe proof before optimizing the persistence strategy.

---

## Runtime Integration

| Option | Description | Selected |
| --- | --- | --- |
| Keep `DurableSyncRuntime` as the shell | Connect durable validation and progress credit inside the existing runtime shell. | yes |
| Move peer orchestration into pure core | Push network-facing runtime behavior into first-party core crates. | |
| Add a second sync runtime | Build a parallel full-sync path for Phase 68. | |

**User's choice:** Yolo recommendation selected the existing
`DurableSyncRuntime` shell.
**Notes:** This keeps functional-core behavior in core crates while avoiding a
competing daemon path.

---

## Operator And Scope Boundaries

| Option | Description | Selected |
| --- | --- | --- |
| Update only truth-contract wording | Adjust operator-facing text where connected/durable progress changes the evidence contract. | yes |
| Expand broad operator surfaces now | Close all status, support, and release docs in Phase 68. | |
| Add production-node language | Present Phase 68 as production full-node readiness. | |

**User's choice:** Yolo recommendation selected narrow truth-contract wording.
**Notes:** Later v1.6 phases own observability, UAT breadth, and release
boundary closeout. Production-node phrasing remains out of scope.

---

## Verification Posture

| Option | Description | Selected |
| --- | --- | --- |
| Deterministic Rust and repo-native verification | Prove connect, persistence, restart, and no-credit behavior with local tests plus `bash scripts/verify.sh`. | yes |
| Public-mainnet verification gate | Require a live public-network sync attempt for default phase completion. | |
| Manual-only validation | Rely on operator inspection without deterministic regression tests. | |

**User's choice:** Yolo recommendation selected deterministic local
verification.
**Notes:** Public-mainnet runs remain opt-in UAT evidence and must stay outside
default verification.

---

## the agent's Discretion

- Planner may split work across durable chainstate integration, runtime
  reconcile/connect behavior, evidence projection, and verification.
- Executor may choose complete snapshot persistence when it is the smallest
  robust restart-safe implementation.
- Executor may add small pure result types or helpers to make progress-credit
  states explicit.

## Deferred Ideas

- Stay-current operation and full tip-agreement policy belong to Phase 69.
- Reorg, peer rotation, and broader no-progress recovery belong to Phase 70.
- Long-sync resource-bound proof belongs to Phase 71.
- Cross-surface support evidence, opt-in UAT breadth, and release-boundary
  closeout belong to Phases 72 through 74.

---

*Phase: 68-full-active-chain-validation-and-durable-persistence*
*Discussion log generated: 2026-06-11T11:56:49.713Z*
