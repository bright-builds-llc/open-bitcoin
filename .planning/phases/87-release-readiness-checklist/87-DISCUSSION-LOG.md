# Phase 87: Release Readiness Checklist - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md - this log preserves the alternatives considered.

**Date:** 2026-06-23T01:49:01.279Z
**Phase:** 87-Release Readiness Checklist
**Mode:** Yolo
**Areas discussed:** Checklist shape, entrypoint links, no-claim boundary, verification and guardrails

---

## Checklist Shape

| Option | Description | Selected |
| --- | --- | --- |
| Extend `docs/parity/release-readiness.md` | Keep one release handoff and add a current v1.8 checklist there. | yes |
| Create a separate checklist document | Split the checklist from the existing release-readiness handoff. | |
| Leave checklist implied by prior docs | Rely on Phase 82-86 docs without a release-review matrix. | |

**User's choice:** Auto-selected recommended default: extend the existing release-readiness handoff.
**Notes:** This satisfies REL-01 without creating a second source of truth.

---

## Entrypoint Links

| Option | Description | Selected |
| --- | --- | --- |
| Compact pointers from README and parity roots | Link to the checklist without duplicating the matrix. | yes |
| Duplicate the checklist in entrypoints | Copy the full matrix into README or parity README. | |
| Keep only internal planning links | Make the checklist discoverable only through `.planning`. | |

**User's choice:** Auto-selected recommended default: compact pointers from contributor and parity entrypoints.
**Notes:** This supports REL-05 while keeping docs maintainable.

---

## No-Claim Boundary

| Option | Description | Selected |
| --- | --- | --- |
| Explicit no-claim review section | State v1.8 remains a boundary-setting milestone unless future gates pass. | yes |
| Implicit no-claim through deferred rows | Rely on existing deferred-surface prose only. | |
| Relax the boundary in release language | Allow production-readiness phrasing in v1.8. | |

**User's choice:** Auto-selected recommended default: explicit no-claim review section.
**Notes:** This supports REL-06 and preserves the Phase 82 production claim boundary.

---

## Verification And Guardrails

| Option | Description | Selected |
| --- | --- | --- |
| Add a narrow Phase 87 checker | Validate checklist, links, no-claim wording, parity roots, and verify wiring. | yes |
| Implement broad all-doc scanner now | Pull Phase 88 deterministic guardrails into Phase 87. | |
| Docs-only with no checker | Rely on manual review only. | |

**User's choice:** Auto-selected recommended default: add a narrow deterministic Phase 87 checker.
**Notes:** Phase 88 still owns broad overclaim and deferred-surface scanning.

---

## the agent's Discretion

- Plan split and exact table layout.
- Whether to update requirement traceability status when verification proves the phase evidence roots.
- Focused checker implementation details following the Phase 82 through Phase 86 Bun patterns.

## Deferred Ideas

- Phase 88 broad deterministic claim guardrails for REL-02, REL-03, and REL-04.
- Future production-readiness milestone after all evidence gates pass.
