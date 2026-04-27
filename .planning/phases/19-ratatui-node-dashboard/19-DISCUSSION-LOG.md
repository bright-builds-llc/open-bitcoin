# Phase 19: Ratatui Node Dashboard - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution.
> Decisions are captured in CONTEXT.md — this log preserves alternatives considered.

**Date:** 2026-04-27T03:41:42.994Z
**Phase:** 19-Ratatui Node Dashboard
**Mode:** Yolo
**Areas discussed:** Dashboard activation and runtime source, Panel architecture and data model, Graphs and refresh policy, Action menu safety model, Fallbacks, color, and testability

---

## Dashboard activation and runtime source

| Option | Description | Selected |
|--------|-------------|----------|
| Interactive dashboard with snapshot fallback | Launch interactive dashboard in TTY; print snapshot in non-interactive contexts | ✓ |
| Always interactive dashboard | Always requires TTY and fails outside interactive terminal | |
| Snapshot-only mode | Skip interactive UI; print status output only | |

**User's choice:** Interactive dashboard with snapshot fallback — launch the TUI first but degrade to non-interactive snapshot output cleanly.
**Notes:** Keeps operator workflows script-friendly and avoids failing CI or automation usage.

| Option | Description | Selected |
|--------|-------------|----------|
| Shared snapshot-first (`OpenBitcoinStatusSnapshot`) + optional collectors | Primary contract is `OpenBitcoinStatusSnapshot` with optional service/RPC enrichment | ✓ |
| Raw collector/output parsing in renderer | Build dashboard-specific collectors | |
| RPC transform only | Pull RPC output directly into UI models | |

**User's choice:** Shared snapshot-first model to preserve downstream reuse with status and logs.
**Notes:** Avoids breaking Phase 17/16 contracts and keeps unavailable semantics stable.

---

## Panel architecture and data model

| Option | Description | Selected |
|--------|-------------|----------|
| Sync/peers/mempool/wallet/service/log/health default panels | Required phase-visible sections in compact dashboard layout | ✓ |
| Status + metrics only | Defer peer, wallet, and service views | |
| Full diagnostics-first panel set including speculative widgets | Include all possible data points immediately | |

**User's choice:** Required coverage across core operator surfaces with compact defaults.
**Notes:** Aligns to DASH-01 and DASH-04 and reuses existing status fields.

| Option | Description | Selected |
|--------|-------------|----------|
| Compact status + bounded history + action area | Keeps terminal footprint predictable and readable | ✓ |
| Full-screen tabbed/graph-only dashboard | Highly visual but can sacrifice context density | |
| Single stream output mode | Skip dedicated panels | |

**User's choice:** Compact status/charts/actions composition.
**Notes:** Supports dense operator usage on narrow terminals.

---

## Graphs and refresh policy

| Option | Description | Selected |
|--------|-------------|----------|
| Bounded metric sparklines using existing series | Render required bounded series: sync, peers, mempool, disk, RPC health | ✓ |
| No charts; numeric table values only | Skip trend visuals for now | |
| Interactive zoomable charts everywhere | Full charting interactivity in terminal | |

**User's choice:** Required bounded trend charts using existing metric model.
**Notes:** Meets DASH-02 while staying lightweight.

| Option | Description | Selected |
|--------|-------------|----------|
| 1-second default refresh + manual refresh support | Responsive without overwhelming collectors | ✓ |
| 250ms polling default | Faster updates with higher terminal/cpu load | |
| Poll on keypress only | Lowest overhead but less "live" feel | |

**User's choice:** 1-second default refresh.
**Notes:** Uses explicit adaptions when live collector is unavailable.

---

## Action menu safety model

| Option | Description | Selected |
|--------|-------------|----------|
| Safe queries + confirmed destructive/service actions | Keep actions but require confirmation for risky operations | ✓ |
| All actions enabled without confirm | Faster action execution, higher risk | |
| Read-only menu only | Maximum safety, no in-dashboard operations | |

**User's choice:** Confirm-gated mixed action menu.
**Notes:** Meets DASH-03 without lowering safety.

| Option | Description | Selected |
|--------|-------------|----------|
| Modal command-summary confirmation | Show operation target/effect before execute | ✓ |
| Single-step toggle confirmation only | Binary confirmation without effect summary | |
| Optional global `--no-confirm` config | Disable confirmation for all operators | |

**User's choice:** Modal summary confirmation.
**Notes:** Explicitly documents risk before service-affecting actions.

---

## Fallbacks, color, and testability

| Option | Description | Selected |
|--------|-------------|----------|
| Restrained palette + explicit no-color fallback | Readable in light/dark terminals and when ANSI is disabled | ✓ |
| Rich high-contrast style-first theme | Visual appeal over terminal compatibility | |
| No color strategy at all | Use plain text only | |

**User's choice:** Restrained + no-color-safe palette policy.
**Notes:** Required by DASH-04 and helps broad terminal support.

| Option | Description | Selected |
|--------|-------------|----------|
| Deterministic non-interactive render tests + action guards | Snapshot/logic tests independent of live terminal | ✓ |
| Manual smoke tests only | Skip dedicated test automation for first cut | |
| Integration with heavy real-terminal snapshots | Depend on true TTY rendering in CI | |

**User's choice:** Deterministic non-interactive tests.
**Notes:** Supports `cargo test`/verification gates without terminal dependence.

## the agent's Discretion

Exact keybindings, widget choice, and spacing are deferred to planning and execution.

## Deferred Ideas

- Cross-node correlation views.
- Interactive chart zoom and drill-down.
- External observability exporter or hosted dashboard export path.

