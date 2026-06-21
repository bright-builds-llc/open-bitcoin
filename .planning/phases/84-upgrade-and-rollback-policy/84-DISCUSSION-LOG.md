# Phase 84: Upgrade and Rollback Policy - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md - this log preserves the alternatives considered.

**Date:** 2026-06-21T21:33:46.234Z
**Phase:** 84-Upgrade and Rollback Policy
**Mode:** Yolo
**Areas discussed:** Upgrade preflight checklist, State and schema compatibility outcomes, Rollback and failed-upgrade boundary, Release readiness and verification drift checks

---

## Upgrade Preflight Checklist

| Option | Description | Selected |
| --- | --- | --- |
| Canonical source-built policy | Create one `docs/parity/` policy for source-built upgrade and rollback expectations, with repo-local command forms and evidence checklist. | yes |
| Runtime-guide-only extension | Add upgrade notes only to `docs/operator/runtime-guide.md`. | |
| Agent discretion without canonical doc | Let downstream planning decide whether a standalone policy is needed. | |

**User's choice:** Canonical source-built policy.
**Notes:** Auto-selected because Phase 84 success criteria require an operator-facing upgrade policy and contributor drift checks, while Phase 82/83 established `docs/parity/` as the current v1.8 policy surface.

---

## State And Schema Compatibility Outcomes

| Option | Description | Selected |
| --- | --- | --- |
| Reuse existing recovery vocabulary | Map upgrade outcomes to existing recovery states, compatibility categories, causes, and action classes. | yes |
| Add new upgrade-specific vocabulary | Define new terms for upgrade status independent of existing recovery evidence. | |
| Defer compatibility mapping to runbooks | Leave compatibility outcome mapping for Phase 85 operator runbooks. | |

**User's choice:** Reuse existing recovery vocabulary.
**Notes:** Auto-selected to avoid duplicate terminology and preserve existing status, support-bundle, storage, and recovery evidence contracts.

---

## Rollback And Failed-Upgrade Boundary

| Option | Description | Selected |
| --- | --- | --- |
| Evidence-preserving rollback guidance | Document rollback as a source-built, explicit, local-first procedure that preserves evidence and forbids hidden source datadir, wallet, service, or config mutation. | yes |
| Package-style rollback guidance | Describe package manager, signed artifact, or automatic update rollback. | |
| Repair-oriented rollback guidance | Include destructive repair or automatic rebuild as a supported rollback path. | |

**User's choice:** Evidence-preserving rollback guidance.
**Notes:** Auto-selected because package distribution, destructive repair, migration apply, and production service guarantees are explicitly deferred in v1.8.

---

## Release Readiness And Verification Drift Checks

| Option | Description | Selected |
| --- | --- | --- |
| Narrow Phase 84 checker | Add deterministic Bun checker/test coverage for upgrade policy sections, command forms, hidden-mutation boundaries, canonical links, and verifier wiring. | yes |
| Documentation only | Rely on human review and Phase 87/88 future checks. | |
| Broad claim scanner now | Implement an all-doc production-claim scanner in Phase 84. | |

**User's choice:** Narrow Phase 84 checker.
**Notes:** Auto-selected because UPG-04 asks for deterministic drift checks, while Phase 88 owns broad all-doc production-claim guardrails.

---

## the agent's Discretion

- Exact file name for the canonical policy under `docs/parity/`.
- Plan split and task sequencing.
- Whether any parity catalog link belongs in a separate plan from checker wiring.

## Deferred Ideas

- Migration apply mode, destructive repair, package-manager rollback, signed release channels, automatic update channels, production-funds wallet support, and automatic backup/restore.
