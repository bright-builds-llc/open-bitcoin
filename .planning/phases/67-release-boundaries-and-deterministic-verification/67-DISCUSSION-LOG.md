# Phase 67: Release Boundaries and Deterministic Verification - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md - this log preserves the alternatives considered.

**Date:** 2026-06-09T00:30:52.044Z
**Phase:** 67-release-boundaries-and-deterministic-verification
**Mode:** Yolo
**Areas discussed:** Release claim shape, threat model and release readiness, deterministic verification boundary, parity roots and docs

---

## Release Claim Shape

| Option | Description | Selected |
| --- | --- | --- |
| v1.5 extended operator-review readiness | Describe v1.5 as source-built, explicit opt-in unattended mainnet operator review readiness. | yes |
| Production-node readiness | Broaden wording toward production full-node/service guarantees. | no |
| Docs-only wording refresh | Update text without adding parity roots or deterministic checks. | no |

**User's choice:** Auto-selected the conservative v1.5 operator-review readiness claim.
**Notes:** This carries forward the Phase 60 through Phase 66 boundaries and prevents scope creep into production-node, inbound-serving, relay, wallet, migration, packaging, hosted-dashboard, Windows-service, or GUI claims.

---

## Threat Model And Release Readiness

| Option | Description | Selected |
| --- | --- | --- |
| Add v1.5-specific threat/release surfaces | Add a new v1.5 threat model and release-readiness matrix while preserving v1.3/v1.4 history. | yes |
| Rewrite v1.4 as current | Mutate v1.4 docs into v1.5 docs and lose historical clarity. | no |
| Skip threat model | Depend only on prior threat models and runtime docs. | no |

**User's choice:** Auto-selected a new v1.5 threat model and release-readiness closeout.
**Notes:** The selected approach best satisfies REL-01 and avoids treating historical v1.4 evidence as the current v1.5 claim.

---

## Deterministic Verification Boundary

| Option | Description | Selected |
| --- | --- | --- |
| Add a Phase 67 deterministic checker | Use a Bun checker to guard docs, parity roots, REL traceability, and default-verification exclusions. | yes |
| Manual review only | Rely on reviewer discipline with no deterministic drift guard. | no |
| Run live checks by default | Add public-network or service-manager commands to default verification. | no |

**User's choice:** Auto-selected a deterministic local checker wired into `bash scripts/verify.sh`.
**Notes:** The checker must assert forbidden default-verification strings such as live-smoke public-network runs, manual peer probing, restart-after-progress, `systemctl --user`, and `launchctl`.

---

## Parity Roots And Docs

| Option | Description | Selected |
| --- | --- | --- |
| Update parity roots and reviewer entrypoints | Update `index.json`, checklist, README, release-readiness, P2P catalog, and runtime guide. | yes |
| Runtime guide only | Leave machine-readable parity roots stale. | no |
| Parity roots only | Leave reviewer workflow docs without current v1.5 interpretation. | no |

**User's choice:** Auto-selected full root and reviewer-doc refresh.
**Notes:** Machine-readable and human-readable parity roots should agree on surface id, REL requirements, evidence paths, known gaps, and suspected unknowns.

---

## Claude's Discretion

- Planner may choose one or two plans depending on whether docs/parity-root work and checker wiring remain cohesive.
- Executor should reuse existing checker style and preserve default verification determinism.
- No Rust changes are expected; parity breadcrumbs are only needed if new first-party Rust files are unexpectedly added.

## Deferred Ideas

- Production full-node support, inbound serving/address advertisement, transaction relay, compact block relay, production-funds wallet use, migration apply mode, signed packaging/distribution, Windows service integration, hosted dashboards, GUI parity, public-network CI, and real service-manager CI remain future milestones.
- Public-network long-run evidence remains opt-in UAT and is not promoted into `bash scripts/verify.sh`.
