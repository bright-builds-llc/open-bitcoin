---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 23-2026-04-28T17-18-36
generated_at: 2026-04-28T17:18:36.921Z
---

# Phase 23: Service Apply Completion and Status Truthfulness - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or
> execution. Decisions are captured in `23-CONTEXT.md`.

**Date:** 2026-04-28T17:18:36.921Z  
**Phase:** 23-Service Apply Completion and Status Truthfulness  
**Mode:** Yolo  
**Areas discussed:** install apply sequencing, service-state truth source,
dashboard reuse, phase bookkeeping

---

## Install apply sequencing

| Option | Description | Selected |
|--------|-------------|----------|
| Complete the manager registration path during `install --apply` | After writing the plist or unit file, run the real launchd or systemd registration steps in the same command. | ✓ |
| Keep install write-only and require a second command | Leave `install --apply` as a file-write step and rely on `enable` later. | |
| Add a separate `start` phase to cover the missing semantics | Defer the current audit blocker to a later feature slice. | |

**Chosen direction:** `install --apply` must finish the registration path in the
same command.

**Notes:** The milestone audit called out the current write-only behavior as a
blocking operator-flow defect for both CLI and dashboard entrypoints.

---

## Service-state truth source

| Option | Description | Selected |
|--------|-------------|----------|
| Carry manager-reported enablement through the shared snapshot | Add explicit enabled-state evidence to `ServiceStateSnapshot` and let status or dashboard project from that. | ✓ |
| Infer enablement from lifecycle enum only | Keep the existing `Enabled | Running | Stopped` heuristic. | |
| Add separate dashboard-only status heuristics | Fix truthfulness only in the dashboard layer and leave CLI service status unchanged. | |

**Chosen direction:** preserve manager evidence in the snapshot model and reuse
it everywhere.

**Notes:** The old enum-only path cannot represent "failed but enabled" or
"running but not enabled" honestly.

---

## Dashboard integration

| Option | Description | Selected |
|--------|-------------|----------|
| Reuse the existing confirmation-gated shared service runtime path | Fix the shared service code and let dashboard actions inherit the behavior. | ✓ |
| Fork a dashboard-specific install path | Add a separate action implementation for dashboard service operations. | |
| Remove dashboard service actions temporarily | Avoid the inherited defect by shrinking the feature surface. | |

**Chosen direction:** keep the shared runtime path and fix it once.

**Notes:** `dashboard/action.rs` already enforces confirmation before calling
`execute_service_command()`, so the dashboard gap should close automatically
once apply semantics and status truth are corrected.

---

## Bookkeeping refresh

| Option | Description | Selected |
|--------|-------------|----------|
| Close Phase 23 with its own summaries, verification report, roadmap update, and requirements refresh | Make the gap-closure phase self-contained for later audit passes. | ✓ |
| Back-edit Phase 18 and Phase 19 summaries only | Rewrite earlier phase evidence instead of recording the repair in a new phase. | |
| Leave bookkeeping for Phase 26 | Ship the code fix now and defer service requirement bookkeeping entirely. | |

**Chosen direction:** Phase 23 records its own completion evidence and refreshes
the service-related requirements ledger.

## Claude's Discretion

- Exact parser tolerance for `launchctl print-disabled` and `systemctl
  is-enabled` output can be chosen during implementation as long as the tests
  prove the expected cases and failure modes stay safe.
- The phase may keep dashboard code unchanged if the shared service path fix is
  sufficient.

## Deferred Ideas

- Start or restart semantics beyond the existing install or enable surface.
- Richer service-log discovery from persisted plist or unit content.
- Windows service parity and packaged-service install workflows.
