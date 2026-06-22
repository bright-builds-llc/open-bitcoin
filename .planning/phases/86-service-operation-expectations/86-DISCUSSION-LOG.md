# Phase 86: Service Operation Expectations - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution
> agents. Decisions are captured in CONTEXT.md - this log preserves the
> alternatives considered.

**Date:** 2026-06-22T19:33:52.813Z
**Phase:** 86-Service Operation Expectations
**Mode:** Yolo
**Areas discussed:** Service support boundary, operator evidence command forms,
documentation and traceability shape, verification guardrails

---

## Service Support Boundary

| Option | Description | Selected |
| --- | --- | --- |
| Canonical service expectation doc | Create one `docs/parity/` source of truth and link to it from existing entrypoints. | yes |
| Runtime-guide-only update | Extend the existing runtime guide service section without a parity-rooted expectation doc. | |
| Broad service policy rewrite | Rework support matrix, runbooks, and runtime guide as equal sources of truth. | |

**User's choice:** Yolo selected the canonical service expectation document.
**Notes:** This matches prior v1.8 phases and avoids duplicating the service
expectation table across entrypoints.

---

## Operator Evidence Command Forms

| Option | Description | Selected |
| --- | --- | --- |
| Require repo-local Cargo and Bazel command pairs | Preserve AGENTS.md and prior lesson requirements for operator UAT commands. | yes |
| Allow installed alias examples | Shorter docs, but risks repeating the prior repo-local command lesson. | |
| Commands by reference only | Avoids duplication, but does not satisfy SVC-02 operator verification needs. | |

**User's choice:** Yolo selected repo-local Cargo and Bazel command pairs.
**Notes:** Commands should cover service lifecycle, restart/resume, status, sync
status, support bundles, logs, metrics, resource bounds, and recovery evidence.

---

## Documentation And Traceability Shape

| Option | Description | Selected |
| --- | --- | --- |
| Register `v1-8-service-operation-expectations` in parity roots | Adds machine and human traceability for SVC-01/SVC-02. | yes |
| Keep Phase 86 docs unregistered | Faster, but weakens release-readiness traceability. | |
| Add a separate service evidence manifest | More structure than the phase needs and inconsistent with v1.8 patterns. | |

**User's choice:** Yolo selected parity root registration.
**Notes:** Human entrypoints should link to the canonical doc without copying
the full table.

---

## Verification Guardrails

| Option | Description | Selected |
| --- | --- | --- |
| Narrow fixed-target Bun checker with fixture tests | Matches Phases 82 through 85 and keeps checks deterministic. | yes |
| No new checker | Leaves SVC-02 drift mostly manual. | |
| Broad all-doc claim scanner | Belongs to Phase 88, not Phase 86. | |

**User's choice:** Yolo selected a narrow Phase 86 checker and tests.
**Notes:** The checker should fail if default verification gains public-network,
real service-manager, package-manager, Windows service, long wall-clock, support
upload, or production service ownership drift.

---

## the agent's Discretion

- The planner may split the phase into canonical doc, links/roots, checker/tests,
  and closeout verification.
- The executor may keep implementation in docs and Bun automation unless a
  narrow source behavior gap appears.

## Deferred Ideas

None.
