# Phase 63: Service Supervision Lifecycle - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution
> agents. Decisions are captured in CONTEXT.md - this log preserves the
> alternatives considered.

**Date:** 2026-06-07T14:20:10.262Z
**Phase:** 63-Service Supervision Lifecycle
**Mode:** Yolo
**Areas discussed:** Service command surface, Lifecycle status contract, Launchd and systemd behavior, Operator documentation and UAT

---

## Service Command Surface

| Option | Description | Selected |
|--------|-------------|----------|
| Extend existing service commands additively | Preserve install dry-run, enable/disable/status, and add preview/start/stop/restart through the existing trait and fake manager. | yes |
| Replace service command model | Redesign service management around a new command runner and bypass the current adapter trait. | |
| Documentation-only preview | Treat existing `service install` dry-run as preview without making preview discoverable in CLI help. | |

**User's choice:** Auto-selected existing additive command model.
**Notes:** Existing service code already has a testable `ServiceManager`
boundary, generated launchd/systemd files, and fake-manager tests. The yolo
decision keeps that structure and adds missing lifecycle operations.

---

## Lifecycle Status Contract

| Option | Description | Selected |
|--------|-------------|----------|
| Normalize to Phase 63 labels | Render unmanaged, installed-stopped, running, failed, disabled, and unavailable-manager consistently across service status, status, dashboard, and JSON. | yes |
| Preserve platform-native labels | Let launchd/systemd terms and current enum names leak directly to operators. | |
| Collapse status to booleans | Only expose installed/enabled/running booleans without a stable lifecycle label. | |

**User's choice:** Auto-selected normalized Phase 63 labels.
**Notes:** Phase 62 already established explicit unavailable reasons and
cross-surface truth. Phase 63 carries that forward for service state.

---

## Launchd And Systemd Behavior

| Option | Description | Selected |
|--------|-------------|----------|
| User-scope daemon supervision | Keep launchd under LaunchAgents and systemd under `systemctl --user`, generate files for `open-bitcoind`, and avoid sudo/global install claims. | yes |
| Machine-wide service install | Add sudo/system-level units and plist placement. | |
| Operator CLI supervision | Continue generated service definitions that point at the `open-bitcoin` operator CLI binary. | |

**User's choice:** Auto-selected user-scope daemon supervision.
**Notes:** The existing runtime resolves the operator binary via
`current_exe()`. Phase 63 should introduce testable `open-bitcoind` path
resolution so service definitions supervise the daemon workflow.

---

## Operator Documentation And UAT

| Option | Description | Selected |
|--------|-------------|----------|
| Deterministic docs and tests plus opt-in UAT | Default verification stays deterministic; runbook shows repo-local Cargo and Bazel commands for manual service review. | yes |
| Live service checks in default verification | Make default verification start real services or require public-network checks. | |
| Alias-only docs | Document only installed `open-bitcoin` commands without repo-local Cargo/Bazel equivalents. | |

**User's choice:** Auto-selected deterministic verification with opt-in UAT.
**Notes:** This follows repo-local guidance and prior v1.5 decisions: public
network and service-based checks are operator UAT evidence, not default
`bash scripts/verify.sh` work.

---

## the agent's Discretion

- The planner may split work by service contract, platform adapters, operator
  surfaces, and docs.
- The executor may add small pure helpers for display-state mapping, daemon
  binary path resolution, or command rendering where that removes duplication.

## Deferred Ideas

- Phase 64 owns service-supervised restart and same-datadir resume evidence.
- Phase 65 owns support-bundle expansion.
- Phase 66 owns the compatibility harness wrapper.
- Phase 67 owns release-boundary and threat-model closeout.
