# Phase 76: Disk and Resource Bound Enforcement - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md - this log preserves the alternatives considered.

**Date:** 2026-06-15T13:58:16.426Z
**Phase:** 76-Disk and Resource Bound Enforcement
**Mode:** Yolo
**Areas discussed:** Resource inventory and bound surfaces, enforcement and operator stop policy, retention and support evidence, deterministic verification

---

## Resource Inventory And Bound Surfaces

| Option | Description | Selected |
|--------|-------------|----------|
| Extend shared status/resource contracts | Add typed adjacent resource-bound evidence to the existing status and soak evidence flow. | yes |
| Build a soak-only resource model | Keep all resource logic inside `open-bitcoin soak` reports. | |
| Render local strings per surface | Let CLI, dashboard, docs, and support evidence each describe resource pressure independently. | |

**User's choice:** Auto-selected: extend shared status/resource contracts.
**Notes:** This carries forward Phase 71 and Phase 72 decisions that shared status is the truth contract and renderer-local summaries are too easy to diverge.

---

## Enforcement And Operator Stop Policy

| Option | Description | Selected |
|--------|-------------|----------|
| Evidence-first preflight plus typed runtime stops | Preflight unsafe starts, classify warning/stop pressure, and record `resource_stop` with source evidence. | yes |
| Warn-only enforcement | Report pressure but never stop a soak automatically. | |
| Hard-fail all unavailable measurements | Refuse operation whenever any resource measurement is unavailable. | |

**User's choice:** Auto-selected: evidence-first preflight plus typed runtime stops.
**Notes:** This satisfies RES-07 while preserving durable progress and avoiding hidden mutation. Exact numeric threshold defaults remain planner/executor discretion, but they must be documented and tested.

---

## Retention And Support Evidence

| Option | Description | Selected |
|--------|-------------|----------|
| Expose compact retention and support-bundle pressure | Reuse existing metrics/log retention policies and add compact support-bundle pressure evidence. | yes |
| Add separate retention engines | Build new Phase 76-specific pruning/retention machinery. | |
| Ignore support-bundle size until Phase 79 | Leave support-bundle size pressure for the later forensics phase. | |

**User's choice:** Auto-selected: expose compact retention and support-bundle pressure.
**Notes:** Phase 76 owns support-bundle size pressure from RES-06, while Phase 79 owns deeper forensic narratives and timelines.

---

## Deterministic Verification

| Option | Description | Selected |
|--------|-------------|----------|
| Rust behavior tests plus focused Bun checker | Prove resource behavior with deterministic Rust fixtures and use Bun only for docs/artifact/default-verification guards. | yes |
| End-to-end public-network soak | Prove behavior with a real long-running public-mainnet soak. | |
| Manual-only UAT | Document operator checks without deterministic regression coverage. | |

**User's choice:** Auto-selected: Rust behavior tests plus focused Bun checker.
**Notes:** This preserves the repo rule that `bash scripts/verify.sh` remains deterministic, public-network-free, service-manager-free, and free of multi-day waits or large local disk allocations.

---

## the agent's Discretion

- Exact threshold defaults for normal/warning/stop-required pressure, as long as they are typed, documented, tested, and tied to the explicit disk budget or configuration.
- Whether to add new enum variants or reuse existing recovery categories with more precise evidence fields.
- Plan boundaries across resource domain helpers, status/support projections, soak runtime enforcement, docs, and checker closeout.

## Deferred Ideas

- Phase 77 owns corruption and lock recovery detail.
- Phase 78 owns progress guarantees and stall diagnosis.
- Phase 79 owns full support-bundle forensics.
- Phase 80 owns opt-in soak UAT and v1.7 release-boundary closeout.
