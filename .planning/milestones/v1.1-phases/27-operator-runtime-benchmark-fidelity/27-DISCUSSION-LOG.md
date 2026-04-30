---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 27-2026-04-28T20-48-00
generated_at: 2026-04-28T20:56:00.000Z
---

# Phase 27 Discussion Log

## Prompt

Close the final milestone follow-up by upgrading the operator-runtime benchmark
cases from pure shared-snapshot projection to deterministic runtime-collected
status and dashboard evidence.

## Decisions Made

1. **Keep the existing operator-runtime case IDs.**
   The report validator and existing benchmark consumers already expect
   `operator-runtime.status-render` and
   `operator-runtime.dashboard-projection`, so fidelity should improve without
   renaming those surfaces.

2. **Collect through the real CLI runtime helpers, not a new “almost runtime”
   abstraction.**
   The benchmark should call `collect_status_snapshot()` and
   `collect_dashboard_snapshot()` using deterministic fake adapters rather than
   inventing a second benchmark-only projection path.

3. **Use fake live RPC plus seeded local metrics instead of a stopped-only
   snapshot.**
   A running collector path better reflects the intended operator-runtime cost,
   and a tiny fake `StatusRpcClient` plus seeded `FjallNodeStore` metrics data
   keeps the run offline and repeatable.

4. **Refresh the benchmark narrative where it still says “shared snapshot”.**
   The registry description and benchmark docs need to match the new
   implementation so the final milestone claim stays auditable.

## Alternatives Considered

- **Leave the benchmark cases as pure render/projection and only update the
  docs.**
  Rejected because it would not close the actual fidelity gap called out in the
  roadmap and milestone audit.

- **Benchmark the full CLI entrypoint including argument parsing and config
  resolution from scratch.**
  Rejected as broader than needed. The success criteria allow an equivalent
  shell entrypoint, but the shared collector helpers already cover the runtime
  behavior we want while staying easier to keep deterministic.

- **Use a stopped-node snapshot with no fake RPC client.**
  Rejected because it would technically use the collector path but still leave
  much of the operator-runtime collection cost unmeasured compared with the live
  collector path.

- **Rename the benchmark cases to new Phase 27-specific IDs.**
  Rejected because it would ripple into the validator and report consumers for
  little value.

## Resulting Plan Shape

- Plan 01: runtime-collected status benchmark case with deterministic fake live
  inputs
- Plan 02: runtime-collected dashboard benchmark case plus truthful benchmark
  metadata and docs
- Plan 03: verification, LOC refresh, `VER-06` closeout, and roadmap updates
