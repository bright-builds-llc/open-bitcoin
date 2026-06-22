# Phase 85: Operator Runbooks - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md - this log preserves the alternatives considered.

**Date:** 2026-06-22T11:58:54.130Z
**Phase:** 85-Operator Runbooks
**Mode:** Yolo
**Areas discussed:** Preflight Runbook, Long-Run Monitoring And No-Progress Diagnosis, Recovery And Escalation Guidance, Support-Bundle Timeline, Documentation And Verification Shape

---

## Preflight Runbook

| Option | Description | Selected |
| --- | --- | --- |
| Production-boundary preflight | Start runbooks by routing through Phase 82 production boundary, Phase 83 support matrix, and Phase 84 upgrade policy before any long-running source-built operation. | yes |
| Runtime-only checklist | Focus only on daemon/status commands and leave boundary docs as background links. | |
| Service-first checklist | Center launchd/systemd supervision before runtime evidence. | |

**User's choice:** Production-boundary preflight (recommended default)
**Notes:** Yolo mode selected the boundary-first option because RUN-01 explicitly requires production-boundary preflight and Phase 86 owns service expectations.

---

## Long-Run Monitoring And No-Progress Diagnosis

| Option | Description | Selected |
| --- | --- | --- |
| Evidence-field diagnosis | Organize long-run guidance around status fields, progress credit, no-progress reasons, recovery evidence, resource pressure, logs, metrics, reports, and unavailable reasons. | yes |
| Timeline-only diagnosis | Present a chronological operator checklist without field-level evidence requirements. | |
| Log-first diagnosis | Treat logs as the primary proof source and use status fields as supporting context. | |

**User's choice:** Evidence-field diagnosis (recommended default)
**Notes:** This preserves Phase 83's rule that elapsed time, startup, peer reachability, raw logs, and report existence are insufficient without typed fields.

---

## Recovery And Escalation Guidance

| Option | Description | Selected |
| --- | --- | --- |
| Existing recovery vocabulary | Reuse safe_retry, read_only_inspection, backup_then_rebuild, stop_and_escalate, resource pressure, stalled subsystem, and latest stop reason. | yes |
| New runbook-specific labels | Create friendlier labels only for operator runbooks. | |
| Escalation-only guidance | Avoid recovery decision tables and tell operators to collect evidence and escalate. | |

**User's choice:** Existing recovery vocabulary (recommended default)
**Notes:** Reusing existing vocabulary keeps Phase 85 aligned with v1.3 through v1.7 evidence surfaces and avoids creating a second recovery taxonomy.

---

## Support-Bundle Timeline

| Option | Description | Selected |
| --- | --- | --- |
| Redacted evidence timeline | Define preflight, command start, status snapshots, progress/no-progress events, resource/recovery events, support-bundle collection, operator action, final status, and escalation decision. | yes |
| Bundle file checklist only | List support bundle files without a timeline narrative. | |
| Full incident template | Create a larger incident report template with response commitments and support workflow ownership. | |

**User's choice:** Redacted evidence timeline (recommended default)
**Notes:** The selected option satisfies RUN-03 while preserving no automatic upload and no production support promise.

---

## Documentation And Verification Shape

| Option | Description | Selected |
| --- | --- | --- |
| Canonical runbook plus narrow checker | Add one canonical runbook document, link entrypoints, and add a deterministic Phase 85 checker if useful. | yes |
| Documentation only | Add runbook prose and rely on manual review. | |
| Broad claim scanner | Expand automation to scan all docs for production-readiness claims. | |

**User's choice:** Canonical runbook plus narrow checker (recommended default)
**Notes:** A narrow checker follows Phase 82-84 patterns. Broad claim scanning stays Phase 88 scope.

---

## the agent's Discretion

- Exact canonical runbook filename.
- Exact plan split across docs, parity metadata, checker, and verification closeout.
- Whether the narrow checker is required after research confirms existing guardrail coverage.

## Deferred Ideas

- Phase 86 owns source-built daemon and service-supervision expectations.
- Phase 87 owns the release-readiness checklist.
- Phase 88 owns broad deterministic claim guardrails.
