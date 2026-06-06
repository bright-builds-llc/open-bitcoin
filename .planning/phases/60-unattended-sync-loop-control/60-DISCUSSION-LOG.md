# Phase 60: Unattended Sync Loop Control - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md - this log preserves the alternatives considered.

**Date:** 2026-06-06T03:04:15.615Z
**Phase:** 60-Unattended Sync Loop Control
**Mode:** Yolo
**Areas discussed:** Loop activation and policy, Stop reasons and lifecycle, Operator control surface, Verification posture

---

## Loop Activation And Policy

| Option | Description | Selected |
|--------|-------------|----------|
| Reuse explicit mainnet-ibd activation | Keep existing `sync.network_enabled = true` plus `sync.mode = "mainnet-ibd"` / CLI override as the opt-in setting. | yes |
| Add a new unattended mode flag | Introduce a second activation surface for unattended operation. | |
| Implicitly run when RPC starts | Make daemon sync default-on after RPC bind. | |

**User's choice:** Auto-selected recommended default: reuse explicit activation.
**Notes:** This preserves v1.5 release boundaries and avoids a new claim surface.

---

## Stop Reasons And Lifecycle

| Option | Description | Selected |
|--------|-------------|----------|
| Persist typed stop reasons | Use durable lifecycle/phase/stop-reason data so status can explain loop stops. | yes |
| Use stderr-only loop errors | Leave stop diagnosis only in daemon stderr. | |
| Treat all stops as failed | Collapse pause, target, no-progress, shutdown, and failures into one lifecycle. | |

**User's choice:** Auto-selected recommended default: persist typed stop reasons.
**Notes:** Phase 60 needs status-visible stop reasons for LOOP-02 and LOOP-04.

---

## Operator Control Surface

| Option | Description | Selected |
|--------|-------------|----------|
| Reuse sync pause/resume/status | Keep existing RPC/store-backed controls and make daemon loop honor them. | yes |
| Add a separate control file | Add another durable control mechanism for daemon loop policy. | |
| Require manual metadata edits | Operators inspect and edit internal store state directly. | |

**User's choice:** Auto-selected recommended default: reuse sync pause/resume/status.
**Notes:** This avoids split-brain control surfaces and preserves existing CLI/RPC behavior.

---

## Verification Posture

| Option | Description | Selected |
|--------|-------------|----------|
| Deterministic Rust tests | Use scripted transports and one-cycle helpers to prove loop policy. | yes |
| Public-network verification | Require live-mainnet long-run checks during default verification. | |
| Docs-only proof | Describe loop behavior without executable regression coverage. | |

**User's choice:** Auto-selected recommended default: deterministic Rust tests.
**Notes:** Public-network long-run checks remain opt-in UAT evidence beyond default verification.

## the agent's Discretion

- The executor may introduce a small policy type or helper if it keeps daemon loop tests finite and clear.
- The executor may add additive stop-reason variants when existing lifecycle fields are insufficient.

## Deferred Ideas

- Service supervision and supervised restart evidence remain Phase 63 and Phase 64.
- Long-run observability/support evidence remains Phase 62 and Phase 65.
