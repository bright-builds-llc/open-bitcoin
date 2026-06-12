# Phase 70: Reorg, Peer Rotation, and No-Progress Recovery - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md - this log preserves the alternatives considered.

**Date:** 2026-06-12T14:58:48.782Z
**Phase:** 70-reorg-peer-rotation-and-no-progress-recovery
**Mode:** Yolo
**Areas discussed:** Branch competition and active-chain selection, Durable reorg execution, Peer failure attribution and rotation, No-progress diagnosis and next actions, Verification posture

---

## Branch Competition and Active-Chain Selection

| Option | Description | Selected |
|--------|-------------|----------|
| Reuse existing cumulative-work policy | Resolve branches by chain work, then height, then block hash using the current header-store policy. | yes |
| Add a trusted external tip/checkpoint source | Use a public API, centralized peer, checkpoint, assumevalid, or assumeutxo shortcut to select the branch. | |
| Headers-only active-chain switching | Let a better header branch change active-chain status before block bodies are connected. | |

**User's choice:** Auto-selected the conservative repo-aligned default: reuse existing cumulative-work policy and require durable validated block connection before active-chain credit.
**Notes:** This carries forward Phase 68 and Phase 69 decisions that rejected trusted shortcuts and renderer-specific truth surfaces.

---

## Durable Reorg Execution

| Option | Description | Selected |
|--------|-------------|----------|
| Reuse `Chainstate::reorg` and durable reconcile | Build Phase 70 around existing connect/disconnect/undo and storage paths. | yes |
| Create a parallel reorg engine | Add a separate runtime-specific reorg path outside the first-party chainstate model. | |
| Disconnect before replacement branch is ready | Drop current active-chain blocks as soon as a better header branch appears. | |

**User's choice:** Auto-selected reuse of the existing first-party chainstate and durable reconcile path.
**Notes:** Missing active-chain blocks or undo data are storage recovery blockers, not peer retry advice.

---

## Peer Failure Attribution and Rotation

| Option | Description | Selected |
|--------|-------------|----------|
| Expand typed peer outcomes and endpoint backoff | Preserve specific failure reasons, release stale in-flight work, and rotate through eligible peers within bounded attempts. | yes |
| Collapse failures into generic network errors | Simplify status by reporting broad network failure text only. | |
| Add production peer eviction or banning | Introduce broad reputation or ban policy during this phase. | |

**User's choice:** Auto-selected typed outcomes with bounded endpoint-keyed retry/backoff and rotation.
**Notes:** Production peer eviction, address-manager governance, and relay policy stay out of scope.

---

## No-Progress Diagnosis and Next Actions

| Option | Description | Selected |
|--------|-------------|----------|
| Shared typed diagnosis | Derive no-progress causes and next actions from `StayCurrentStatus`, progress signal, recovery category, peer outcomes, and resource pressure. | yes |
| Renderer-specific messages | Let CLI, RPC, dashboard, logs, and support evidence each interpret no-progress independently. | |
| Single `no_progress` bucket | Report no-progress without distinguishing at-tip, awaiting blocks, stalled peers, backoff, reorg recovery, or storage blockers. | |

**User's choice:** Auto-selected one shared typed diagnosis contract with specific next actions.
**Notes:** Phase 72 can broaden support and cross-surface alignment after Phase 70 defines the core truth.

---

## Verification Posture

| Option | Description | Selected |
|--------|-------------|----------|
| Deterministic synthetic tests and focused checker/docs | Cover reorg, peer rotation, stale in-flight, and no-progress cases without public network or service-manager dependency. | yes |
| Public-mainnet default verification | Put live public peer or timing-dependent checks in `bash scripts/verify.sh`. | |
| Defer all verification to opt-in UAT | Implement behavior without deterministic regression coverage. | |

**User's choice:** Auto-selected deterministic tests/checkers plus opt-in public-network UAT boundaries.
**Notes:** New first-party Rust source or tests require parity breadcrumbs.

---

## the agent's Discretion

- The planner may split work by branch/reorg domain, peer rotation, no-progress projection, tests, and docs/checker closeout.
- The executor may add small pure helper types when they reduce duplicated status logic.
- Broader Phase 71 through Phase 74 surfaces should remain deferred unless a narrow update is required for REC-01 through REC-04 truthfulness.

## Deferred Ideas

- Resource-bound closeout remains Phase 71.
- Full observability and support evidence alignment remains Phase 72.
- Opt-in UAT command breadth remains Phase 73.
- Release boundary closeout remains Phase 74.
