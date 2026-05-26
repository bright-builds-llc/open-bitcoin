# Phase 47: Operator Sync Truth Surfaces - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md - this log preserves the alternatives considered.

**Date:** 2026-05-26T21:36:05.164Z
**Phase:** 47-operator-sync-truth-surfaces
**Mode:** Yolo
**Areas discussed:** Shared Status Contract, Surface Alignment, Metrics And Logs, Verification

---

## Shared Status Contract

| Option | Description | Selected |
| --- | --- | --- |
| Add explicit progress signal and last-progress timestamp | Make OBS-01 machine-readable and avoid renderer-local inference. | yes |
| Rely only on existing progress counters | Smaller change, but JSON consumers still infer state from several fields. |  |
| Rename existing lag/progress fields | More disruptive and risks breaking status consumers. |  |

**User's choice:** Auto-selected recommended additive status fields.
**Notes:** Existing `lag` remains the estimated lag field to avoid duplicate truth.

---

## Surface Alignment

| Option | Description | Selected |
| --- | --- | --- |
| Align human status and dashboard labels around headers/downloaded/connected | Carries Phase 46 durable progress semantics through every operator surface. | yes |
| Keep dashboard compact `blocks=headers` wording | Smaller change, but hides downloaded versus connected state. |  |
| Add a separate dashboard-only sync model | More code and a new drift point. |  |

**User's choice:** Auto-selected shared snapshot projection.
**Notes:** RPC `getblockchaininfo.blocks` stays connected-chain height.

---

## Metrics And Logs

| Option | Description | Selected |
| --- | --- | --- |
| Add downloaded/connected metrics and log fields | Lets charts and support evidence match JSON status progress dimensions. | yes |
| Leave metrics as sync height plus header height only | Preserves current shape but cannot explain partial download recovery. |  |
| Encode all details only in logs | Useful for humans but weaker for dashboard charts and automation. |  |

**User's choice:** Auto-selected explicit metrics and logs.
**Notes:** Keep existing `sync_height` as a compatibility metric.

---

## Verification

| Option | Description | Selected |
| --- | --- | --- |
| Deterministic local tests plus repo verification | Matches v1.3 rule that public-network checks stay opt-in. | yes |
| Require live mainnet smoke | Out of scope for Phase 47 and unsuitable for default verification. |  |
| Manual renderer inspection only | Insufficient for status/RPC consistency. |  |

**User's choice:** Auto-selected deterministic local verification.
**Notes:** UAT commands should use repo-local Cargo and Bazel forms.

---

## the agent's Discretion

- Exact Rust enum and helper names.
- Exact compact human/dashboard labels.

## Deferred Ideas

None.
