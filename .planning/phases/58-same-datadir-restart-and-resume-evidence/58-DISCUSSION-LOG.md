# Phase 58: Same-Datadir Restart and Resume Evidence - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or
> execution agents. Decisions are captured in CONTEXT.md; this log preserves
> the alternatives considered.

**Date:** 2026-06-05T12:58:05.451Z
**Phase:** 58-same-datadir-restart-and-resume-evidence
**Mode:** Yolo
**Areas discussed:** Same-datadir restart flow, Restart evidence report schema,
Recovery diagnosis taxonomy, Deterministic restart/resume tests

---

## Same-Datadir Restart Flow

| Option | Description | Selected |
| --- | --- | --- |
| Script-managed two-session live-smoke restart | One auditable opt-in report that proves same-datadir relaunch with status snapshots and peer telemetry. | yes |
| Manual two-run operator runbook | Low code churn but fragmented, human-dependent evidence. | |
| Deterministic store-reopen resume tests | Hermetic regression guard for durable state and no duplicate block connects. | yes |
| Service-manager restart evidence | Broader platform restart policy evidence better suited to a future production-service milestone. | |

**User's choice:** Auto-selected script-managed two-session live-smoke restart
plus deterministic store-reopen tests.
**Notes:** Keep the claim narrow: explicit opt-in interruption/relaunch using
the same datadir, not service supervision or unattended production-node
operation.

---

## Restart Evidence Report Schema

| Option | Description | Selected |
| --- | --- | --- |
| `result.restartResumeEvidence` compact summary | Machine-readable restart boundary and same-datadir proof using allowlist-friendly fields. | yes |
| Reuse `firstHeaderProgress` and `firstBlockProgress` only | Minimal schema churn but ambiguous for restart proof. | |
| Top-level `restartAttempts[]` with raw snapshots | Rich local diagnostics but too broad and hard to redact. | |
| UAT-only restart note | Quick manual artifact but weak machine-readable proof. | |

**User's choice:** Auto-selected compact
`result.restartResumeEvidence`.
**Notes:** Preserve existing Phase 57 progress fields, but add a distinct
restart evidence object so Phase 59 can summarize it safely.

---

## Recovery Diagnosis Taxonomy

| Option | Description | Selected |
| --- | --- | --- |
| Layered `RecoveryDiagnosis` object | Exact Phase 58 category plus underlying peer/storage causes and storage-first precedence. | yes |
| Seven-category enum only | Simple but loses useful details unless all consumers also carry raw causes. | |
| Extend live-smoke `NoProgressCause` only | Smallest smoke-report-only path but risks inconsistent guidance across surfaces. | |
| Docs-only mapping | No contract churn but not machine-readable enough for RESUME-03. | |

**User's choice:** Auto-selected layered recovery diagnosis.
**Notes:** Categories are `peer_incompatibility`,
`public_network_unreachable`, `invalid_peer_data`, `store_corruption`,
`store_incompatibility`, `resource_exhaustion`, and
`intentional_cancellation`; underlying causes should remain visible.

---

## Deterministic Restart/Resume Tests

| Option | Description | Selected |
| --- | --- | --- |
| `DurableSyncRuntime` two-pass same-datadir fixtures | Real Fjall reopen, chainstate/header/block rehydration, and scheduler behavior without public network. | yes |
| Live-smoke restart/resume report fixtures | Operator-facing report semantics with mocked daemon/status commands. | yes |
| Durable recovery/status matrix | Focused RESUME-03 classification checks. | yes |
| Offline daemon plus local scripted peer process | Closer to process-level evidence but heavier and more brittle. | |

**User's choice:** Auto-selected runtime reopen tests, live-smoke fixture
tests, and a narrow recovery matrix.
**Notes:** Avoid a broad local peer process harness unless deterministic
fixture evidence proves insufficient.

---

## the agent's Discretion

- Exact helper names and internal schema layout for restart evidence and
  recovery diagnosis.
- Whether to split Phase 58 into multiple implementation plans or keep it as a
  single focused plan.
- Which existing restart tests can be reused versus tightened with explicit
  Phase 58 assertions.

## Deferred Ideas

- Phase 59 support-bundle allowlisting, threat model, and release-boundary
  closeout.
- Service-manager restart policy and unattended production-node operation.
- Inbound serving, transaction relay, wallet production use, migration apply
  mode, packaging, hosted dashboard, and GUI work.
