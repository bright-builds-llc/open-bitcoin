# Phase 74: Release Boundaries, Parity, and Documentation - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution
> agents. Decisions are captured in CONTEXT.md; this log preserves the
> alternatives considered during the yolo discussion pass.

**Date:** 2026-06-14T15:07:06.000Z
**Phase:** 74 - Release Boundaries, Parity, and Documentation
**Mode:** Yolo
**Areas discussed:** v1.6 release claim shape, parity roots and threat model,
deterministic release-boundary check, operator docs and README, final
traceability and archive readiness

---

## v1.6 Release Claim Shape

| Option | Description | Selected |
| --- | --- | --- |
| Explicit opt-in full-sync completion evidence | Claim source-built local review of sync-to-tip and stay-current evidence while preserving deferred production scope. | yes |
| Broad production-node readiness | Treat v1.6 as general production full-node readiness. | no |
| Documentation-only wording refresh | Update wording without a precise release claim. | no |

**User's choice:** Auto-selected explicit opt-in full-sync completion evidence.
**Notes:** This carries forward v1.6 PROJECT/REQUIREMENTS scope and prior
release-boundary phases. Broad production-node readiness remains out of scope.

---

## Parity Roots And Threat Model

| Option | Description | Selected |
| --- | --- | --- |
| Add v1.6-specific docs and roots | Create/link a v1.6 threat model and release-readiness section while preserving v1.3-v1.5 as historical evidence. | yes |
| Rewrite older milestone docs | Replace historical v1.3-v1.5 wording with current milestone wording. | no |
| Skip threat model closeout | Rely on existing parity roots without a v1.6-specific model. | no |

**User's choice:** Auto-selected v1.6-specific additive docs and roots.
**Notes:** This follows Phase 49, Phase 59, and Phase 67 precedent.

---

## Deterministic Release-Boundary Check

| Option | Description | Selected |
| --- | --- | --- |
| Add a local Bun checker | Validate v1.6 roots, docs, requirements, deferred-scope wording, and default-verification exclusions in `scripts/verify.sh`. | yes |
| Manual review only | Depend on reviewer inspection without a deterministic guard. | no |
| Public-network release gate | Require live mainnet sync evidence inside default verification. | no |

**User's choice:** Auto-selected a deterministic local Bun checker.
**Notes:** The checker must remain public-network-free, service-manager-free,
and timing-stable.

---

## Operator Docs And README

| Option | Description | Selected |
| --- | --- | --- |
| Point to authoritative evidence roots | Update README/runtime guide with current v1.6 interpretation and links while keeping Phase 73's UAT matrix authoritative. | yes |
| Duplicate the full release matrix everywhere | Repeat full release-readiness tables in README and operator docs. | no |
| Use installed alias examples | Prefer `open-bitcoin` alias commands in UAT instructions. | no |

**User's choice:** Auto-selected concise pointers plus repo-local command
forms.
**Notes:** This reflects local lessons and `AGENTS.md` repo guidance.

---

## Final Traceability And Archive Readiness

| Option | Description | Selected |
| --- | --- | --- |
| Verify all 26 v1.6 requirements | Require final traceability that every v1.6 requirement is mapped and verified before archive readiness. | yes |
| Close only REL-01 through REL-03 | Ignore prior requirement traceability in the final phase. | no |
| Check in generated live evidence | Commit local live-mainnet reports or support bundles as release evidence. | no |

**User's choice:** Auto-selected all-26 requirement traceability with local
generated evidence kept out of git.
**Notes:** Phase 74 should close REL-01 through REL-03 and confirm prior
requirements remain complete.

---

## the agent's Discretion

- Exact plan split between docs, checker, and final traceability.
- Exact v1.6 threat IDs and matrix row names.
- Whether README/runtime-guide edits are narrow pointers or slightly broader
  release closeout wording.

## Deferred Ideas

- Inbound serving, relay, production wallets, migration apply mode, packaging,
  GUI, hosted dashboards, public-network CI, and production-node readiness
  remain future milestone scopes.
