---
phase: 19
slug: ratatui-node-dashboard
status: verified
threats_open: 0
asvs_level: 1
created: 2026-05-03
---

# Phase 19 — Security

> Per-phase security contract: threat register, accepted risks, and audit trail.

---

## Trust Boundaries

| Boundary | Description | Data Crossing |
|----------|-------------|---------------|
| CLI args -> dashboard runtime | User flags and invocation context decide interactive vs snapshot mode and refresh settings | Output mode, tick interval, no-color preference |
| Status snapshot -> dashboard renderers | Shared node/service/wallet/sync state is projected into text and TUI surfaces | Availability-tagged status fields and bounded metric history |
| Keyboard input -> action executor | Operator keypresses drive refresh, help, status, and service-action transitions | Key events, pending confirmation state |
| Service action executor -> ServiceManager | Confirmed dashboard actions may affect the user-level service lifecycle | Install/enable/disable/uninstall commands and status queries |
| Runtime outcome -> process output | Snapshot/interactive fallbacks determine what reaches stdout/stderr | Human text, JSON snapshot output, typed error text |

---

## Threat Register

| Threat ID | Category | Component | Disposition | Mitigation | Status |
|-----------|----------|-----------|-------------|------------|--------|
| T-19-01 | Tampering | dispatch selection | mitigate | `OperatorCommand::Dashboard` routes through a single `run_dashboard(...)` entrypoint with routing tests proving the deferred placeholder path is gone. | closed |
| T-19-02 | Availability | non-interactive sessions | mitigate | Non-TTY and JSON invocations are forced onto deterministic snapshot rendering, and interactive init failure falls back to text snapshot output. | closed |
| T-19-03 | Information Disclosure | output composition | accept | Accepted risk: phase 19 reuses the existing status snapshot contract and does not add credential-like fields, but disclosure relies on the upstream snapshot contract remaining secret-safe. | closed |
| T-19-04 | Tampering | action mapping | mitigate | Unknown keys map to `None`, invalid transitions do not execute side effects, and blocked small-window mode ignores non-exit keys entirely. | closed |
| T-19-05 | Elevation of Privilege | service action execution | mitigate | Destructive actions require explicit pending/confirm transitions and execute only through the existing `execute_service_command(...)` service contract. | closed |
| T-19-06 | Denial of Service | tick/render saturation | mitigate | Metric series are bounded before rendering and the event loop refreshes on a fixed interval instead of unbounded redraw/input churn. | closed |
| T-19-09 | Tampering | unconfirmed action state | mitigate | Explicit confirm/cancel finite states, small-window blocker gating, and action tests prevent hidden or premature service mutations. | closed |
| T-19-10 | Information Disclosure | snapshot output | accept | Accepted risk: JSON/plain snapshot output is intentionally operator-visible and ANSI-free; safety depends on the shared snapshot contract continuing to exclude secrets. | closed |
| T-19-11 | Denial of Service | terminal mode failure | mitigate | Terminal-capability failures degrade to snapshot mode, and sub-threshold terminal heights render a full-screen blocker with exit-only controls. | closed |

*Status: open · closed*
*Disposition: mitigate (implementation required) · accept (documented risk) · transfer (third-party)*

---

## Accepted Risks Log

| Risk ID | Threat Ref | Rationale | Accepted By | Date |
|---------|------------|-----------|-------------|------|
| AR-19-01 | T-19-03 | The dashboard intentionally reuses the shared status snapshot contract and introduces no new secret-bearing fields; residual disclosure risk is accepted at this phase boundary rather than duplicating sanitization logic locally. | user | 2026-05-03 |
| AR-19-02 | T-19-10 | Snapshot output is deliberately machine-readable and operator-visible; residual disclosure risk is accepted as long as the shared snapshot contract remains the sole output source and stays credential-safe. | user | 2026-05-03 |

*Accepted risks do not resurface in future audit runs.*

---

## Security Audit Trail

| Audit Date | Threats Total | Closed | Open | Run By |
|------------|---------------|--------|------|--------|
| 2026-05-03 | 9 | 9 | 0 | Codex + user decision |

---

## Sign-Off

- [x] All threats have a disposition (mitigate / accept / transfer)
- [x] Accepted risks documented in Accepted Risks Log
- [x] `threats_open: 0` confirmed
- [x] `status: verified` set in frontmatter

**Approval:** verified 2026-05-03
