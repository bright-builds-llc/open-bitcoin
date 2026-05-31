# Phase 52: Operator Evidence Cleanup - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md - this log preserves the alternatives considered.

**Date:** 2026-05-31T23:50:17.955Z
**Phase:** 52-operator-evidence-cleanup
**Mode:** Yolo
**Areas discussed:** Support Bundle Live-Smoke Summary, Deterministic Evidence Tests, Daemon Preflight Truth, Docs And Audit References

---

## Support Bundle Live-Smoke Summary

| Option | Description | Selected |
| --- | --- | --- |
| Schema v2 nested result summary | Summarize allowlisted `result` fields from live-smoke schema v2 reports while preserving redaction. | yes |
| Top-level fields only | Keep the existing shallow allowlist behavior. | |
| Raw report embedding | Include full live-smoke input in support evidence. | |

**User's choice:** Yolo selected schema v2 nested result summary.
**Notes:** This directly closes audit debt D-02 without weakening the Phase 48 redaction boundary.

---

## Deterministic Evidence Tests

| Option | Description | Selected |
| --- | --- | --- |
| Fixture-based support bundle tests | Add schema v2 fixture assertions for nested summary fields and raw input absence. | yes |
| Manual review only | Rely on code inspection and existing tests. | |
| Public-network smoke test | Run live network evidence as part of this cleanup. | |

**User's choice:** Yolo selected deterministic fixture-based support bundle tests.
**Notes:** Public-network evidence remains opt-in and belongs to Phase 53.

---

## Daemon Preflight Truth

| Option | Description | Selected |
| --- | --- | --- |
| Refresh wording with testable helper | State that enabled startup uses the explicit opt-in bounded sync worker while preserving production-node non-claims. | yes |
| Remove the preflight line | Avoid stale wording by omitting operator-facing preflight context. | |
| Keep old wording | Leave the stale "peer transport ... not started" phrase. | |

**User's choice:** Yolo selected refreshed wording with a deterministic unit assertion.
**Notes:** This closes audit debt D-04 and keeps startup behavior honest.

---

## Docs And Audit References

| Option | Description | Selected |
| --- | --- | --- |
| Narrow docs/audit cleanup | Update only operator/audit references that mention stale support-summary or preflight debt. | yes |
| Broad release-doc rewrite | Refresh all v1.3 parity and release pages. | |
| No docs changes | Let code/tests close the debt without reader-facing cleanup. | |

**User's choice:** Yolo selected narrow docs/audit cleanup.
**Notes:** Phase 52 should not alter the milestone public-network claim boundary.

## the agent's Discretion

- Exact Rust helper names and Markdown formatting are left to the planner and executor.
- The planner may decide whether richer live-smoke summary values remain a JSON object in Markdown or are rendered as explicit bullets, as long as reviewers can inspect the required fields.

## Deferred Ideas

- Phase 53 owns live public-network evidence refresh and historical artifact caveat retirement.
