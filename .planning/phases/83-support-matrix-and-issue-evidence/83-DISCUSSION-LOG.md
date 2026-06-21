# Phase 83: Support Matrix and Issue Evidence - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md - this log preserves the alternatives considered.

**Date:** 2026-06-21T16:02:39.628Z
**Phase:** 83-support-matrix-and-issue-evidence
**Mode:** Yolo
**Areas discussed:** Support Matrix Scope, Issue Evidence Expectations, Residual Risk And Manual Validation, Contributor Update Boundaries

---

## Support Matrix Scope

| Option | Description | Selected |
| --- | --- | --- |
| Exact Phase 82 support terms | Use `supported`, `preview`, `opt-in UAT`, `unsupported`, and `deferred` without new maturity labels. | x |
| Expanded support vocabulary | Add more nuanced labels for community, beta, production-like, or partial support. | |
| Prose-only support descriptions | Avoid a structured matrix and describe support posture in paragraphs. | |

**User's choice:** Auto-selected exact Phase 82 support terms.
**Notes:** Recommended because Phase 82 locked the vocabulary and Phase 83 must not broaden production-readiness language.

---

## Issue Evidence Expectations

| Option | Description | Selected |
| --- | --- | --- |
| Minimal redacted evidence checklist | Request support bundles, logs/config summaries, service/resource/sync evidence, platform details, and exact repo-local reproduction commands. | x |
| Broad raw artifact request | Ask for raw logs, datadirs, wallets, peer tables, and local report directories for every issue. | |
| Support bundle only | Treat support bundle existence as sufficient issue evidence. | |

**User's choice:** Auto-selected minimal redacted evidence checklist.
**Notes:** Recommended because existing support evidence is local and redacted, and Phase 82 states artifact existence alone is not proof.

---

## Residual Risk And Manual Validation

| Option | Description | Selected |
| --- | --- | --- |
| Carry forward historical residual risks | List v1.1 through v1.7 manual validation surfaces, opt-in UAT surfaces, historical closeout caveats, and deferred gates. | x |
| Only list current v1.8 risks | Hide older residual risks once a new milestone starts. | |
| Treat historical evidence as current support | Promote v1.3 through v1.7 opt-in evidence into current production support language. | |

**User's choice:** Auto-selected carry forward historical residual risks.
**Notes:** Recommended because SUP-04 requires release reviewers to see carried-forward risks before approving v1.8 language.

---

## Contributor Update Boundaries

| Option | Description | Selected |
| --- | --- | --- |
| Evidence-gated matrix updates | Require evidence source, verifier or opt-in UAT command, residual risk, and next gate before adding or promoting support rows. | x |
| Freeform docs updates | Let contributors update support language without a structured evidence checklist. | |
| Broad scanner now | Implement the full all-doc production-claim scanner in Phase 83. | |

**User's choice:** Auto-selected evidence-gated matrix updates.
**Notes:** Recommended because SUP-03 needs update guidance now while Phase 88 owns broad deterministic guardrails.

---

## the agent's Discretion

- Choose the exact document placement for the canonical support matrix.
- Decide whether a narrow deterministic Phase 83 checker is warranted.
- Decide whether a parity root/checklist entry improves discoverability without creating a duplicate evidence registry.

## Deferred Ideas

- Phase 84 upgrade and rollback policy.
- Phase 85 operator runbooks.
- Phase 86 service operation expectations.
- Phase 87 release-readiness checklist.
- Phase 88 broad deterministic claim guardrails.
