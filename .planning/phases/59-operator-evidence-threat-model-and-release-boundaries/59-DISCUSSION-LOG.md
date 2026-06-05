# Phase 59: Operator Evidence, Threat Model, and Release Boundaries - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md - this log preserves the alternatives considered.

**Date:** 2026-06-05T15:10:59.825Z
**Phase:** 59-operator-evidence-threat-model-and-release-boundaries
**Mode:** Yolo
**Areas discussed:** Cross-surface operator truth, Support evidence packet, Operator docs and UAT commands, Threat model and release boundaries, Verification posture

---

## Cross-Surface Operator Truth

| Option | Description | Selected |
|--------|-------------|----------|
| Shared status truth | Use shared status snapshot and durable sync metadata as the consistency source for status, dashboard, metrics, logs, RPC, support, and live-smoke surfaces. | yes |
| Renderer-local truth | Let each surface summarize its own evidence independently. | |
| New aggregate runtime | Add a new final-evidence runtime service for v1.4 closeout. | |

**User's choice:** Yolo selected shared status truth.
**Notes:** This carries forward Phase 47, Phase 56, Phase 57, and Phase 58 truth-surface decisions and avoids adding runtime scope in a closeout phase.

---

## Support Evidence Packet

| Option | Description | Selected |
|--------|-------------|----------|
| Allowlisted v1.4 summary | Extend support summaries to include header, block, restart/resume, recovery diagnosis, peer outcome, status, metrics/log, config, and store-health summaries. | yes |
| Raw report embedding | Copy raw live-smoke reports and status snapshots into support bundles. | |
| Defer support changes | Leave support bundles at the v1.3 schema v2 summary surface. | |

**User's choice:** Yolo selected allowlisted v1.4 summary.
**Notes:** Raw report embedding conflicts with existing redaction and local-artifact boundaries. Deferring support changes would leave OBS-02 incomplete.

---

## Operator Docs And UAT Commands

| Option | Description | Selected |
|--------|-------------|----------|
| Repo-local command matrix | Document deterministic verification, manual-peer smoke, restart/resume review, support bundle generation, and pass/fail fields with exact Cargo and Bazel commands. | yes |
| Alias-oriented docs | Prefer installed `open-bitcoin` examples and name the Cargo/Bazel alternatives secondarily. | |
| Minimal docs | Link to prior phase docs without a final v1.4 closeout pass. | |

**User's choice:** Yolo selected repo-local command matrix.
**Notes:** This applies repo-local guidance and the lessons about copy-pasteable Cargo/Bazel operator commands.

---

## Threat Model And Release Boundaries

| Option | Description | Selected |
|--------|-------------|----------|
| v1.4-specific closeout | Add or refresh v1.4 threat/release sections and parity roots while preserving v1.3 as historical evidence. | yes |
| Rewrite v1.3 docs in place | Treat v1.3 threat model and release-readiness docs as the current milestone surface. | |
| Planning-only notes | Keep threat and release boundaries only in GSD artifacts. | |

**User's choice:** Yolo selected v1.4-specific closeout.
**Notes:** Reviewers need a current v1.4 surface for SEC-01 and SEC-02, but v1.3 artifacts should remain historically accurate.

---

## Verification Posture

| Option | Description | Selected |
|--------|-------------|----------|
| Deterministic gate plus opt-in UAT docs | Keep `bash scripts/verify.sh` public-network-free, add deterministic checks for support and release docs, and document live smoke as UAT only. | yes |
| Default live-network verification | Add manual-peer or restart live smoke to the default verification gate. | |
| Docs-only verification | Rely on prose review without scriptable release-boundary checks. | |

**User's choice:** Yolo selected deterministic gate plus opt-in UAT docs.
**Notes:** This carries forward SEC-03 and prior milestone boundaries.

---

## the agent's Discretion

- Exact plan count and file grouping.
- Whether v1.4 threat/release checks are implemented as new files or extensions
  to existing checker patterns.
- Exact support summary key names, provided the support bundle remains
  allowlisted, redacted, and deterministic to test.

## Deferred Ideas

- Inbound serving, transaction relay, production-funds wallet use, migration
  apply mode, packaging, hosted dashboard, GUI work, Windows service support,
  and unattended production-node operation.
- Hosted support upload or support-bundle artifact validation.
- Public-network CI or default public-network verification.
