# Phase 71: Resource Bounds and Durable Restart/Resume - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md - this log preserves the alternatives considered.

**Date:** 2026-06-13T10:36:32.206Z
**Phase:** 71-Resource Bounds and Durable Restart/Resume
**Mode:** Yolo
**Areas discussed:** Resource bound contract, restart and interruption matrix, storage pressure and recovery guidance, deterministic long-chain verification, operator evidence and documentation

---

## Resource Bound Contract

| Option | Description | Selected |
|--------|-------------|----------|
| Extend existing typed status | Keep `SyncResourcePressure` and shared sync/status contracts as the resource envelope; add only missing typed bounded facts. | yes |
| Create a new resource subsystem | Add a parallel resource-pressure report independent of existing status. | |
| Document existing behavior only | Avoid code/test changes and describe current limits in docs. | |

**User's choice:** Auto-selected "Extend existing typed status".
**Notes:** This matches Phase 61, Phase 62, and Phase 70 decisions to avoid renderer-local strings and keep resource evidence bounded.

---

## Restart And Interruption Matrix

| Option | Description | Selected |
|--------|-------------|----------|
| Deterministic same-datadir fixtures | Use Fjall reopen, `DurableSyncRuntime`, scripted transport, and block reconcile fixtures for clean shutdown, unclean shutdown, mid-download, mid-connect, and stale in-flight cases. | yes |
| Public-mainnet restart proof | Require live public-network restart evidence to prove the phase. | |
| Documentation-only evidence | Explain restart/resume expectations without adding regression coverage. | |

**User's choice:** Auto-selected "Deterministic same-datadir fixtures".
**Notes:** Public-network restart remains optional UAT and must not enter `bash scripts/verify.sh`.

---

## Storage Pressure And Recovery Guidance

| Option | Description | Selected |
|--------|-------------|----------|
| Typed storage-first guidance | Map schema mismatch, corruption, lock contention, low disk, and storage pressure through shared recovery types and next actions. | yes |
| Renderer-specific text | Add guidance independently in CLI/docs without shared status changes. | |
| Fail without recovery detail | Surface raw errors and rely on operators to infer next actions. | |

**User's choice:** Auto-selected "Typed storage-first guidance".
**Notes:** Existing storage categories should be reused where precise; new labels are justified only if RES-03 cannot be audited otherwise.

---

## Deterministic Long-Chain Verification

| Option | Description | Selected |
|--------|-------------|----------|
| Synthetic long-chain tests | Build deterministic fixtures that exercise bounds, queues, reconnect, restart/resume, metrics/log retention, and compact support evidence without network access. | yes |
| Long public-mainnet run | Make public-network long-run behavior part of phase completion. | |
| Manual review only | Leave resource proof to human inspection. | |

**User's choice:** Auto-selected "Synthetic long-chain tests".
**Notes:** This satisfies RES-04 and preserves the default verification boundary.

---

## Operator Evidence And Documentation

| Option | Description | Selected |
|--------|-------------|----------|
| Update shared docs/checkers where contracts change | Keep operator docs, status snapshot docs, parity roots, and deterministic checkers aligned with new resource/recovery evidence. | yes |
| Defer all docs to Phase 72 | Keep Phase 71 implementation-only and document later. | |
| Expand production-node claims | Use resource proof to imply broader production readiness. | |

**User's choice:** Auto-selected "Update shared docs/checkers where contracts change".
**Notes:** Phase 71 may update contributor/operator docs for truthfulness, while broad support bundle alignment remains Phase 72 and release boundaries remain Phase 74.

---

## the agent's Discretion

- Planner may split the phase by status/resource contract, restart fixtures, storage-pressure guidance, synthetic long-chain verification, and docs/checker closeout.
- Executor may use existing files when that is the smallest robust path and must update parity breadcrumbs for new first-party Rust files.

## Deferred Ideas

None.
