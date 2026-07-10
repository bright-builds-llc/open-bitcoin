# Phase 117: Parity Traceability, UAT, and Release Guardrails - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in `117-CONTEXT.md`; this log preserves alternatives considered by the yolo recommendation pass.

**Date:** 2026-07-10
**Phase:** 117-parity-traceability-uat-and-release-guardrails
**Mode:** Yolo
**Areas discussed:** Parity surface ownership, release-note location, claim-scanner semantics, checker and UAT closure

## Parity Surface Ownership

| Option | Description | Selected |
| --- | --- | --- |
| Distinct phase surfaces plus aggregate closeout | Backfill Phase 112-116 surfaces and add one Phase 117 BOUND surface while preserving exactly-once ownership. | ✓ |
| One aggregate Phase 117 surface only | Put every v2.1 requirement into one large closeout entry. | |
| Extend Phase 110 and 111 surfaces only | Avoid new phase surfaces and overload the existing entries. | |

**Auto-selected choice:** Distinct phase surfaces plus aggregate closeout.
**Notes:** This matches the parity index/checklist schema and keeps implementation ownership separate from aggregate checker coverage. Breadcrumb groups must be semantically honest, not merely mechanically present.

## Release-Note Location

| Option | Description | Selected |
| --- | --- | --- |
| Existing release-readiness root | Extend `docs/parity/release-readiness.md` and keep README as a concise pointer. | ✓ |
| New release-notes tree | Create a new changelog or per-version release document. | |
| README only | Put the complete release handoff in the repository README. | |

**Auto-selected choice:** Existing release-readiness root.
**Notes:** The repository has no first-party release-notes tree, and Phase 106 already established release-readiness as the handoff root.

## Claim-Scanner Semantics

| Option | Description | Selected |
| --- | --- | --- |
| Curated paragraph-aware scoped-claim scanner | Allow bounded/default-off v2.1 claims and reject broader public/default/archive/production claims in current claim-bearing docs. | ✓ |
| Global forbidden phrase scan | Reject phrases such as `compact block relay` everywhere. | |
| Exact required phrases only | Require approved wording but do not detect nearby overclaims. | |

**Auto-selected choice:** Curated paragraph-aware scoped-claim scanner.
**Notes:** Compact relay is now legitimately in scope, so the Phase 106 global deferral posture cannot be copied unchanged. Fixtures must prove valid deferred and scoped wording remains accepted.

## Checker And UAT Closure

| Option | Description | Selected |
| --- | --- | --- |
| Aggregate checker plus committed UAT package | Add one Phase 117 checker/test after Phase 116 and create `117-UAT.md`; optional public-network review remains non-blocking. | ✓ |
| Extend every prior phase checker | Spread closeout ownership across Phase 110-116 checker files. | |
| Documentation-only closeout | Update prose without deterministic aggregate enforcement or a committed UAT record. | |

**Auto-selected choice:** Aggregate checker plus committed UAT package.
**Notes:** The checker owns deterministic closure; `bash scripts/verify.sh` remains the final contract. Public-network review is explicitly optional and outside pre-commit/default CI.

## the agent's Discretion

- Exact checker helper and fixture structure.
- Exact paragraph-classification implementation and failure messages.
- Smallest honest breadcrumb-group splits.
- Exact doc section placement and optional UAT result wording.

## Deferred Ideas

- Broader relay, filter, public-default, archive, production, packaging, GUI, migration-apply, destructive-repair, and automatic-upload surfaces remain outside Phase 117.
