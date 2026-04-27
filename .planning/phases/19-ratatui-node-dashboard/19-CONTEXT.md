---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 19-2026-04-27T03-41-42
phase_lifecycle_id: 19-2026-04-27T09-02-20
generated_at: 2026-04-27T09:02:56.147Z
---

# Phase 19: Ratatui Node Dashboard - Context

**Gathered:** 2026-04-27
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Deliver a local terminal dashboard for operator visibility and control on top of the shared status, metrics, logs, service, sync, and wallet evidence models. This phase is scoped to Ratatui-style UI composition, live snapshot rendering, bounded charted telemetry, and gated operator actions. It does not add desktop GUI parity, external observability export, or new core sync behavior.
</domain>

<decisions>
## Implementation Decisions

### Dashboard Runtime and Data Model
- **D-01:** Keep `open-bitcoin dashboard` as an interactive terminal surface when a TTY is available, with explicit non-interactive fallback to snapshot-style output for non-TTY or scripted contexts.
- **D-02:** The dashboard SHALL consume `OpenBitcoinStatusSnapshot` as its source-of-truth state and map optional collectors (live RPC, service diagnostics) into that shape rather than building renderer-local state contracts.
- **D-03:** Use optional injected collectors/formatters so the render layer can run deterministically in tests and when live daemon data is unavailable.
- **D-04:** Reuse existing detection and path contracts from status and service surfaces instead of inventing dashboard-specific config or probe formats.

### Panels, Charts, and Refresh Behavior
- **D-05:** Show in-scope dashboard content by default in compact sections: node/system snapshot, sync and peers, mempool and wallet summary, service state, logs health signals, and bounded metric trends.
- **D-06:** Render bounded historical metrics as terminal-friendly charts/sparklines for sync progress, peers, mempool size, disk usage, and RPC health, using Phase 16+17 metric retention defaults and contracts.
- **D-07:** Default refresh should be 1-second polling in interactive mode with explicit manual refresh and graceful degrade when live data is unavailable.
- **D-08:** Preserve deterministic render behavior across unavailable fields by showing explicit unavailable states and reasons rather than suppressing sections.

### Action Surface and Safety
- **D-09:** Include both read/query actions and action entries for potentially destructive or service-affecting operations only with explicit confirm steps and clear effect summaries.
- **D-10:** Action confirmations are modal and include target, scope, and exact command-effect summary before execution.

### Rendering, Accessibility, and Testing
- **D-11:** Apply a restrained palette with readable contrast in light and dark terminals; default to no-color compatibility where color is disabled.
- **D-12:** Add non-interactive render tests and unit coverage for dashboard projection, action guards, and unavailable-data rendering, keeping automation independent of a live terminal session.

### the agent's Discretion
- Exact Ratatui widgets, key bindings, and text layout are discretionary as long as required content and safety contracts are met.
- The command-line launch flags and hidden debug knobs for rendering frequency are discretionary, provided defaults match this phase boundary.

</decisions>

<canonical_refs>
## Canonical References

Downstream agents MUST read these before planning or implementing.

### Phase scope
- `.planning/ROADMAP.md` — Phase 19 goal, depends-on relation, and success criteria.
- `.planning/REQUIREMENTS.md` — DASH-01 through DASH-04, plus `SYNC-06` context.
- `.planning/PROJECT.md` — v1.1 operator direction and terminal-first scope constraints.
- `.planning/phases/16-metrics-logs-and-sync-telemetry/16-CONTEXT.md` — baseline observability contracts that feed dashboard inputs.
- `.planning/phases/17-cli-status-and-first-run-onboarding/17-CONTEXT.md` — status command model and config compatibility boundary.
- `.planning/phases/18-service-lifecycle-integration/18-CONTEXT.md` — service status model and lifecycle output contracts.

### Architecture and implementation docs
- `docs/architecture/cli-command-architecture.md` — operator command boundaries (`dashboard` remains operator path).
- `docs/architecture/status-snapshot.md` — ownership and unavailable-field semantics for all dashboard-facing fields.
- `docs/architecture/operator-observability.md` — default metric/log retention contracts used for dashboard trend panels.
- `docs/parity/source-breadcrumbs.json` — required manifest rule for any new Rust source under `packages/open-bitcoin-*/src`.

### Existing code to reuse directly
- `packages/open-bitcoin-cli/src/operator.rs` — `OperatorCommand::Dashboard` and command args shape.
- `packages/open-bitcoin-cli/src/operator/runtime.rs` — operator dispatch entrypoint and execution pattern.
- `packages/open-bitcoin-cli/src/operator/status.rs` — status collector and snapshot projection entry.
- `packages/open-bitcoin-cli/src/operator/status/render.rs` — existing status output styling patterns and output separation.
- `packages/open-bitcoin-cli/src/operator/tests.rs` — command routing/integration behavior currently asserting dashboard is deferred.
- `packages/open-bitcoin-cli/src/args.rs` — CLI argument structures and existing command surface.
- `packages/open-bitcoin-node/src/status.rs` — shared status contract for node, service, sync, peers, mempool, wallet, metrics, and logs.
- `packages/open-bitcoin-node/src/metrics.rs` — bounded metric series and retention schema.
- `packages/open-bitcoin-node/src/logging.rs` — log status and warning/error signal contracts.
- `packages/open-bitcoin-rpc/src/config/open_bitcoin.rs` — dashboard settings section shape currently in config model.
</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `OpenBitcoinStatusSnapshot` and `OpenBitcoinStatusSnapshot` field ownership in `packages/open-bitcoin-node/src/status.rs`.
- CLI command definitions in `packages/open-bitcoin-cli/src/operator.rs` (`dashboard`, `status`, `service`, onboarding).
- Status projection and collection architecture in `packages/open-bitcoin-cli/src/operator/status.rs`.
- Existing status output patterns in `packages/open-bitcoin-cli/src/operator/status/render.rs`.

### Established Patterns
- Shared status model + optional collectors to keep pure business logic separate from rendering I/O.
- Functional core / imperative shell split already present in operator runtime and service/status collectors.
- Deterministic availability semantics (`FieldAvailability`, explicit unavailable reasons) for missing data.

### Integration Points
- `operator/runtime.rs` command dispatch should route `OperatorCommand::Dashboard` to a new dashboard execution shell.
- `operator/status.rs` and existing node snapshots should be the dashboard input source.
- `open-bitcoin-rpc` config surface should carry any dashboard-only persistence/display preferences while maintaining `bitcoin.conf` compatibility.
</code_context>

<specifics>
## Specific Ideas

- Use a compact default layout optimized for common terminal widths: key header row, three compact data regions, and an action bar at the bottom.
- Keep the action menu discoverable with explicit labels for safe vs. destructive operations.
- Show chart axis labels as simple numeric ranges instead of full analytics metadata, keeping output operator-actionable.
- Keep non-interactive fallback output close to `open-bitcoin status` so scripts can parse behavior consistency.
</specifics>

<deferred>
## Deferred Ideas

- Interactive chart zoom/pan or drill-down panels.
- Cross-node dashboard views and alert routing.
- External metrics exporters or web dashboard export formats.
- Auto-run workflows that mutate node state outside explicit confirmation.
</deferred>

---

*Phase: 19-ratatui-node-dashboard*
*Context gathered: 2026-04-27*
