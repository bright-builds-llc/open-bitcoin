# Phase 116: Operator Evidence, Metrics, Logs, and Support Boundary - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-07-06
**Phase:** 116-operator-evidence-metrics-logs-and-support-boundary
**Mode:** Yolo
**Areas discussed:** Shared status contract, RPC projection, CLI/dashboard rendering, metrics/logs labels, support redaction, operator UAT docs

---

## Shared Status Contract

| Option | Description | Selected |
|--------|-------------|----------|
| Extend `BlockServingEvidenceStatus` only | Smaller diff but mixes full-block and compact-relay counters awkwardly | |
| Compose block-serving + compact-relay evidence in one shared contract | Mirrors relay evidence split between activation/counters/recovery | ✓ |
| Renderer-local summaries per surface | Faster but violates Phase 72/110 D-17 cross-surface truth | |

**User's choice:** Compose shared block-relay evidence contract consumed by all surfaces.
**Notes:** In-flight state stays aggregate-only.

---

## RPC Projection

| Option | Description | Selected |
|--------|-------------|----------|
| New dedicated RPC method | More endpoints to maintain | |
| Extend `open_bitcoin_network_status` with block-relay field | Aligns with inbound/relay/metrics layout | ✓ |

**User's choice:** Extend Open Bitcoin network status response.

---

## CLI And Dashboard

| Option | Description | Selected |
|--------|-------------|----------|
| JSON-only block-relay evidence | Leaves human operators blind | |
| Human lines + dashboard sections mirroring relay evidence | Consistent operator UX | ✓ |

**User's choice:** Add human CLI lines and dashboard sections from shared contract.

---

## Metrics And Logs

| Option | Description | Selected |
|--------|-------------|----------|
| Dynamic string labels from runtime errors | High cardinality, support leakage risk | |
| Fixed labels reused from Phases 110–115 | Stable observability and redaction | ✓ |

**User's choice:** Fixed low-cardinality labels only.

---

## Support Redaction

| Option | Description | Selected |
|--------|-------------|----------|
| Include raw compact block payloads for debugging | Violates Phase 59 threat model | |
| Allowlisted summaries through shared status + existing redaction helpers | Preserves support safety | ✓ |

**User's choice:** Allowlisted redacted summaries only.

---

## Operator UAT Docs

| Option | Description | Selected |
|--------|-------------|----------|
| Installed-alias-only commands | Conflicts with AGENTS.md repo-local UAT guidance | |
| Copy-pasteable Cargo and Bazel command forms | Matches repo conventions | ✓ |

**User's choice:** Repo-local Cargo/Bazel commands in operator docs.

---

## Claude's Discretion

Exact Rust type names, checker script naming, and module file split.

## Deferred Ideas

Phase 117 parity/UAT/release guardrails; package relay; public defaults; production readiness claims.
