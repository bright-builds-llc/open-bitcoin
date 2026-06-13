# Phase 72: Operator Observability and Support Evidence - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md - this log preserves the alternatives considered.

**Date:** 2026-06-13T16:25:09.129Z
**Phase:** 72-operator-observability-and-support-evidence
**Mode:** Yolo
**Areas discussed:** Shared truth contract, Cross-surface alignment, Redacted support evidence, Operator guidance and scope, Verification posture

---

## Shared Truth Contract

| Option | Description | Selected |
| --- | --- | --- |
| Snapshot-first contract | Use `OpenBitcoinStatusSnapshot`/`SyncStatus` as the one truth source and make surfaces consume or summarize it. | yes |
| Renderer-local contracts | Let CLI, dashboard, RPC, logs, metrics, live-smoke, and support evidence each define their own interpretation. | |
| New parallel evidence model | Add a separate Phase 72 support/observability model independent of status. | |

**User's choice:** Snapshot-first contract.
**Notes:** Auto-selected because prior phases already centralized the validated active-chain, tip, stay-current, no-progress, reorg, and resource-pressure evidence in `SyncStatus`.

---

## Cross-Surface Alignment

| Option | Description | Selected |
| --- | --- | --- |
| Fill projection gaps from existing fields | Render and compare the Phase 68-71 fields that already exist before adding new domain concepts. | yes |
| Add only docs | Document intended agreement without testing or filling renderer gaps. | |
| Add a broad new dashboard/reporting surface | Build a new user surface beyond CLI/dashboard/RPC/support. | |

**User's choice:** Fill projection gaps from existing fields.
**Notes:** Auto-selected because Phase 72 is an alignment phase. Human CLI, dashboard, RPC durable status, support summaries, live-smoke summaries, metrics, and logs should agree on shared labels and unavailable reasons.

---

## Redacted Support Evidence

| Option | Description | Selected |
| --- | --- | --- |
| Compact allowlisted evidence with typed verdict | Keep support bundles redacted and compact, include the shared snapshot plus selected live-smoke fields, and derive a final verdict from evidence. | yes |
| Raw report bundle | Copy raw daemon logs, raw live-smoke reports, raw peer tables, or raw status dumps for maximum detail. | |
| Minimal bundle only | Keep existing config/status/store-health evidence without Phase 72 sync-to-tip support fields. | |

**User's choice:** Compact allowlisted evidence with typed verdict.
**Notes:** Auto-selected to preserve the existing redaction boundary while satisfying OBS-02 and OBS-04.

---

## Operator Guidance and Scope

| Option | Description | Selected |
| --- | --- | --- |
| Evidence-specific guidance | Explain whether evidence proves sync-to-tip, stay-current, diagnosed blocker, restart/resume safety, or deferred scope. | yes |
| Production readiness language | Let support and docs imply broad production-node readiness when a bundle exists. | |
| UAT command expansion now | Move the Phase 73 opt-in public-mainnet command matrix into Phase 72. | |

**User's choice:** Evidence-specific guidance.
**Notes:** Auto-selected to keep v1.6 scoped to explicit opt-in full-sync completion and avoid inbound serving, relay, production-wallet, migration-apply, packaging, GUI, hosted-dashboard, or broad production-node claims.

---

## Verification Posture

| Option | Description | Selected |
| --- | --- | --- |
| Deterministic checker plus focused tests | Add fixture/unit coverage and a Phase 72 checker wired into repo-native verification. | yes |
| Manual review only | Rely on operator inspection of rendered surfaces. | |
| Public-network default verification | Add live mainnet or service-manager checks to `bash scripts/verify.sh`. | |

**User's choice:** Deterministic checker plus focused tests.
**Notes:** Auto-selected to satisfy OBS cross-surface agreement while preserving the repo rule that default verification remains hermetic and short-running.

---

## the agent's Discretion

- Exact plan split across status/rendering, support evidence, live-smoke/log/metric projection, comparison fixtures, and docs/checker closeout.
- Exact final verdict enum names, provided they are typed and evidence-derived.
- Exact helper placement, provided shared projection logic reduces renderer-local divergence.

## Deferred Ideas

- Phase 73 owns the broader opt-in public-mainnet UAT command matrix.
- Phase 74 owns final v1.6 release-boundary and readiness closeout.
- Hosted dashboards, GUI surfaces, inbound serving, relay, production wallets, migration apply mode, signed packages, and broad production-node claims remain deferred.
