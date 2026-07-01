# Phase 103: Mempool Chainstate Lifecycle and Durable Recovery - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md - this log preserves the alternatives considered.

**Date:** 2026-07-01T12:38:00.304Z
**Phase:** 103-Mempool Chainstate Lifecycle and Durable Recovery
**Mode:** Yolo
**Areas discussed:** Mempool pressure and trimming evidence, Block connect cleanup, Disconnect and reorg reconsideration, Durable mempool persistence and recovery, Parity evidence and deterministic verification

## Mempool Pressure And Trimming Evidence

| Option | Description | Selected |
| --- | --- | --- |
| Typed pressure contract | Add pure/shared low-cardinality pressure, fee-floor, capacity, trimming, and deferred-parity evidence. | yes |
| Operator prose only | Document pressure limits without adding a typed contract. | |
| Defer all pressure evidence | Leave MEM-03 to later observability work. | |

**User's choice:** Auto-selected typed pressure contract.
**Notes:** This is required for MEM-03 and should not wait for Phase 105 observability unless only rendering remains.

## Block Connect Cleanup

| Option | Description | Selected |
| --- | --- | --- |
| Pure mempool lifecycle plus managed shell bridge | Keep cleanup in pure mempool APIs and invoke it from managed block-connect hooks. | yes |
| Managed-only ad hoc cleanup | Remove entries only from managed runtime maps. | |
| Peer-layer cleanup | Let peer/download code mutate mempool or cache state directly. | |

**User's choice:** Auto-selected pure mempool lifecycle plus managed shell bridge.
**Notes:** This preserves the repo's functional-core boundary and keeps block connect cleanup coherent across mempool indexes and managed relay caches.

## Disconnect And Reorg Reconsideration

| Option | Description | Selected |
| --- | --- | --- |
| Bounded candidate reconsideration | Reconsider eligible disconnected transactions through the Phase 102 outcome/orphan bridge under documented v2.0 bounds. | yes |
| Restore all disconnected transactions unconditionally | Reinsert every disconnected transaction without policy or bound checks. | |
| Defer reorg reconsideration | Only document the gap and leave MEM-05 unfinished. | |

**User's choice:** Auto-selected bounded candidate reconsideration.
**Notes:** The phase should avoid package-relay or full Knots reorg parity claims when descendants or package-shaped behavior remains deferred.

## Durable Mempool Persistence And Recovery

| Option | Description | Selected |
| --- | --- | --- |
| Dedicated durable mempool namespace | Add schema-versioned save/load/remove/recovery behavior in the Fjall adapter style. | yes |
| Fold into runtime metadata | Store mempool data inside generic runtime metadata. | |
| In-memory only | Leave MEM-06 to a later phase. | |

**User's choice:** Auto-selected dedicated durable mempool namespace.
**Notes:** Persistence should align with `StorageNamespace`, `SchemaVersion`, `StorageError`, `StorageRecoveryAction`, `FjallNodeStore`, and `snapshot_codec` patterns.

## Parity Evidence And Deterministic Verification

| Option | Description | Selected |
| --- | --- | --- |
| Rust tests plus deterministic checker | Add pure/integration/storage tests and a fixed-corpus checker if docs or parity roots change. | yes |
| Rust tests only | Skip phase checker even if parity docs are updated. | |
| Manual UAT only | Rely on manual review rather than deterministic checks. | |

**User's choice:** Auto-selected Rust tests plus deterministic checker.
**Notes:** Default verification must remain `bash scripts/verify.sh` and must not add public-network, service-manager, wall-clock soak, destructive repair, or production-deployment gates.

## the agent's Discretion

- Exact type names, storage key shape, module split, and plan granularity are left to the planner.
- The planner may decide whether MEM-03 needs a narrow status field in Phase 103 or whether pure/runtime evidence is enough before Phase 105 rendering.

## Deferred Ideas

- Relay serving, fanout, and rebroadcast remain Phase 104 scope.
- RPC, CLI, dashboard, metrics, logs, and support-bundle observability remain Phase 105 scope unless a narrow truth field is needed for MEM-03.
- Final parity/UAT/release-boundary closeout remains Phase 106 scope.
- Compact block relay, package relay, bloom/filter serving, public relay defaults, public-network relay CI, production full-node readiness, and production-funds wallet use remain out of v2.0 scope.
