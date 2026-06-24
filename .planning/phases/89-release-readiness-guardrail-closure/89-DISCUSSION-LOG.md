# Phase 89: Release Readiness Guardrail Closure - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution
> agents. Decisions are captured in CONTEXT.md.

**Date:** 2026-06-24T20:03:57.087Z
**Phase:** 89-release-readiness-guardrail-closure
**Mode:** Yolo
**Areas discussed:** Release-readiness checklist closure, deterministic
claim-guardrail corpus, fixture and verification coverage, planning metadata
hygiene

---

## Release-Readiness Checklist Closure

| Option | Description | Selected |
| --- | --- | --- |
| Add REL-02/REL-03/REL-04 rows to the canonical checklist | Keeps the existing release reviewer flow complete and auditable from one table. | Yes |
| Leave Phase 88 ownership in prose below the table | Preserves current text but does not close GAP-01. | No |
| Create a second checklist or evidence registry | Adds another source of truth and risks drift. | No |

**Chosen default:** Add REL-02, REL-03, and REL-04 rows to
`docs/parity/release-readiness.md` and update checker expectations.

**Rationale:** `.planning/v1.8-MILESTONE-AUDIT.md` identified the missing rows
as GAP-01 and as a release-readiness reviewer flow break.

---

## Deterministic Claim-Guardrail Corpus

| Option | Description | Selected |
| --- | --- | --- |
| Add the missing canonical policy docs to Phase 88 scan targets | Closes GAP-02 while preserving the curated corpus model. | Yes |
| Scan every planning and historical document | Broadens default verification and risks historical false positives. | No |
| Keep the current corpus unchanged | Leaves deferred-surface promotion possible in canonical policy docs. | No |

**Chosen default:** Add `docs/parity/upgrade-and-rollback-policy.md`,
`docs/parity/operator-runbooks.md`, and
`docs/parity/service-operation-expectations.md` to the Phase 88 corpus.

**Rationale:** The audit specifically names those documents as first-class
v1.8 release-review evidence roots that the current Phase 88 checker omits.

---

## Fixture And Verification Coverage

| Option | Description | Selected |
| --- | --- | --- |
| Add focused Phase 87 and Phase 88 fixture coverage | Proves the exact gaps cannot recur. | Yes |
| Rely on manual audit after docs edits | Lower implementation cost but weak regression protection. | No |
| Run only full verification without focused tests | Catches current state but does not document the new expected behavior. | No |

**Chosen default:** Add targeted tests for the missing release checklist rows and
for deferred-surface promotion in newly scanned policy docs, then run focused
checks and `bash scripts/verify.sh`.

**Rationale:** Phase 87 and Phase 88 already use deterministic Bun fixture tests
for release-boundary drift.

---

## Planning Metadata Hygiene

| Option | Description | Selected |
| --- | --- | --- |
| Record the stale metadata decision in Phase 89 verification | Keeps audit closeout explicit without forcing full milestone archival into this phase. | Yes |
| Refresh all milestone narrative files in Phase 89 | May be useful if required by verification, but risks expanding beyond gap closure. | No |
| Ignore stale planning metadata | Leaves the audit hygiene item ambiguous. | No |

**Chosen default:** Verification must say whether stale planning metadata was
refreshed or routed to milestone closeout.

**Rationale:** The audit lists the stale metadata as non-blocking tech debt that
should be handled during gap closure or milestone closeout.

## Claude's Discretion

- Split implementation into docs/checker closure, Phase 88 corpus/test closure,
  and verification evidence.
- Keep Phase 89 as documentation and Bun automation unless implementation
  reveals a concrete source-code gap.

## Deferred Ideas

- Full v1.8 milestone archival and broad narrative refresh remain milestone
  closeout work unless required to close Phase 89 verification.
