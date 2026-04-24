# Phase 10: Benchmarks and Audit Readiness - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md - this log preserves the alternatives considered.

**Date:** 2026-04-24T10:47:33.305Z
**Phase:** 10-benchmarks-and-audit-readiness
**Mode:** Yolo
**Areas discussed:** Benchmark coverage and comparison policy, Parity checklist
shape, Audit readiness and handoff, Folded todos and risk capture

---

## Benchmark Coverage and Comparison Policy

| Option | Description | Selected |
| --- | --- | --- |
| First-party Rust benchmarks with optional Knots mapping | Build stable Rust benchmark/report surfaces, map to Knots sources where meaningful, and keep full Knots execution optional unless reliable. | yes |
| Mandatory Knots binary benchmarking | Require every local run to build and execute the pinned Knots benchmark binary. | |
| Documentation-only performance notes | Record benchmark intentions without executable first-party benchmark coverage. | |

**User's choice:** Auto-selected first-party Rust benchmarks with optional Knots mapping.
**Notes:** This best fits `PAR-02` without making local verification flaky or dependent on heavyweight C++ baseline builds.

---

## Parity Checklist Shape

| Option | Description | Selected |
| --- | --- | --- |
| JSON root plus Markdown review surface | Keep `docs/parity/index.json` authoritative and add concise Markdown checklist/review docs. | yes |
| Markdown-only checklist | Human readable but weaker for agents and CI. | |
| JSON-only status file | Machine friendly but harder for release reviewers to scan. | |

**User's choice:** Auto-selected JSON root plus Markdown review surface.
**Notes:** This preserves the existing parity catalog pattern and satisfies `AUD-01`.

---

## Audit Readiness and Handoff

| Option | Description | Selected |
| --- | --- | --- |
| Deterministic release-readiness and milestone handoff docs | Add repo-owned audit artifacts that summarize complete, deferred, unknown, evidence, and verification state. | yes |
| Public dashboard | Richer presentation but out of v1 scope and unnecessary for local audit readiness. | |
| No separate handoff | Leaves release decision context scattered across phase artifacts. | |

**User's choice:** Auto-selected deterministic release-readiness and milestone handoff docs.
**Notes:** This keeps the phase scoped to repo artifacts and avoids v2 `OBS-01` scope.

---

## Folded Todos and Risk Capture

| Option | Description | Selected |
| --- | --- | --- |
| Fold high-relevance todos into audit evidence/risk capture | Include AI-agent CLI evidence and panic/illegal-state risk tracking without broadening implementation scope. | yes |
| Implement all matched todos now | Would turn Phase 10 into broad CLI and code-quality implementation work. | |
| Ignore matched todos | Would lose relevant audit context discovered by the workflow. | |

**User's choice:** Auto-selected folded audit/risk capture for high-relevance todos.
**Notes:** The low-score early-return todo was reviewed but deferred as a separate code-quality concern.

---

## the agent's Discretion

- Exact benchmark framework/harness selection after research.
- Exact report filenames and whether benchmark smoke checks sit directly in
  `scripts/verify.sh` or behind a dedicated script.

## Deferred Ideas

- Public benchmark dashboards or published observability surfaces.
- Broad CLI feature additions beyond audit/evidence capture.
- Broad panic/illegal-state refactors.
