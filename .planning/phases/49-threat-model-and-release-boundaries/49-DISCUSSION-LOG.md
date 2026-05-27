# Phase 49: Threat Model and Release Boundaries - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md - this log preserves the alternatives considered.

**Date:** 2026-05-27T21:29:02.646Z
**Phase:** 49-threat-model-and-release-boundaries
**Mode:** Yolo
**Areas discussed:** Threat model scope, release claim boundaries, live evidence acceptance criteria, reviewer audit traceability

---

## Threat Model Scope

| Option | Description | Selected |
|--------|-------------|----------|
| Consolidated v1.3 scoped threat model | Reviewer-facing document covering SEC-01 domains with assets, trust boundaries, mitigations, evidence links, and deferred claims. | yes |
| Lightweight release-boundary addendum | Shorter release note style summary with less threat coverage. | |
| Full formal data-flow/threat-tree model | Deeper external-audit style model with diagrams and broader maintenance burden. | |
| Planning-only security closeout | GSD-local security note linked from planning artifacts. | |

**User's choice:** Auto-selected consolidated v1.3 scoped threat model.
**Notes:** The selected approach satisfies SEC-01 without implying certification or production readiness.

---

## Release Claim Boundaries

| Option | Description | Selected |
|--------|-------------|----------|
| Boundary matrix in parity/release-readiness docs | State proven claim, accepted evidence, explicit non-claim, future gate, and requirement links in existing reviewer docs. | yes |
| Threat-model-first claim register | Put claim boundaries primarily in the threat model with release-readiness links. | |
| Machine-readable release claims ledger | Add a schema/checkable claims model to parity roots. | |
| Operator-facing limitations only | Rely mainly on runtime-guide limitations. | |

**User's choice:** Auto-selected parity/release-readiness boundary matrix.
**Notes:** The support bundle remains evidence only, not a release validator or production-readiness claim.

---

## Live Evidence Acceptance Criteria

| Option | Description | Selected |
|--------|-------------|----------|
| Artifact-first reviewer contract using existing commands | Document `scripts/verify.sh`, live-smoke, support bundle, JSON/Markdown artifacts, and accepted progress-or-blocker outcomes. | yes |
| Offline live-report validator outside `verify.sh` | Add a new Bun validator for local report schema and minimum fields. | |
| Phase 50 release packet contract | Define a heavier artifact manifest with transcripts and restart/resume evidence. | |
| Hosted/manual CI workflow | Add optional hosted public-network evidence collection. | |

**User's choice:** Auto-selected artifact-first reviewer contract using existing commands.
**Notes:** Public-network behavior stays opt-in and outside the default deterministic verification gate.

---

## Reviewer Audit Traceability

| Option | Description | Selected |
|--------|-------------|----------|
| Release-readiness traceability matrix | Map requirements, roadmap phases, evidence docs, support artifacts, and non-claims through the existing parity roots. | yes |
| Dedicated `docs/parity/v1.3-traceability.md` | Add a separate one-stop traceability document and link it from roots. | |
| Support bundle traceability manifest | Expand generated support evidence with claim/evidence manifest data. | |
| Planning-only traceability | Keep traceability in Phase 49 planning artifacts only. | |

**User's choice:** Auto-selected docs-first traceability through existing parity/release-readiness roots.
**Notes:** Phase 49 should not expand support bundle schema or runtime behavior.

---

## the agent's Discretion

- Exact document split for the threat model and traceability matrix.
- Exact threat IDs, section names, and wording, provided the claim boundary is explicit.
- Whether to add a small deterministic docs/root assertion when updating `docs/parity/index.json`.

## Deferred Ideas

- Offline live-report validator outside the default verification gate.
- Hosted/manual CI live-network evidence collection.
- Future production-node, relay, wallet, migration apply, packaging, hosted dashboard, and GUI milestones.
