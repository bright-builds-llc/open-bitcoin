---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 27-2026-04-28T20-48-00
generated_at: 2026-04-28T20:56:00.000Z
---

# Phase 27: Operator Runtime Benchmark Fidelity - Context

## Phase Boundary

**Goal:** Upgrade operator-runtime benchmark cases from snapshot-fixture
projection checks to runtime-collected status/dashboard evidence without losing
deterministic verification.

**Success criteria:**
1. Operator-runtime benchmark cases collect status/dashboard data through the
   real runtime collection path or an equivalent shell entrypoint instead of
   only `sample_status_snapshot()` fixtures.
2. Report metadata and benchmark validation still describe the operator-runtime
   cases accurately after the fidelity upgrade.
3. Repo-native verification keeps the benchmark/report path deterministic and
   free of public-network requirements.

**Out of scope:**
- Public-network benchmarks or any requirement for a live node in the local
  smoke path.
- New benchmark timing thresholds or release gates based on elapsed numbers.
- New benchmark groups or case IDs; the existing Phase 22 operator-runtime cases
  should stay stable so the report validator does not need a schema rethink.

## Requirements In Scope

- `VER-06`

## Canonical References

- `.planning/ROADMAP.md` — Phase 27 goal, requirement, and success criteria.
- `.planning/REQUIREMENTS.md` — current benchmark traceability state.
- `.planning/v1.1-MILESTONE-AUDIT.md` — the original warning that operator
  runtime benchmarks still used `sample_status_snapshot()` fixtures.
- `.planning/phases/22-real-sync-benchmarks-and-release-hardening/22-CONTEXT.md`
  — benchmark and verification decisions already made in Phase 22.
- `.planning/phases/22-real-sync-benchmarks-and-release-hardening/22-VERIFICATION.md`
  — current Phase 22 benchmark evidence and metadata claims.
- `packages/open-bitcoin-bench/src/cases/operator_runtime.rs` — current
  operator-runtime benchmark implementation.
- `packages/open-bitcoin-bench/src/runtime_fixtures.rs` — reusable deterministic
  fixture builders and temp-store helpers.
- `packages/open-bitcoin-bench/src/registry.rs` — benchmark group descriptions
  and metadata.
- `packages/open-bitcoin-cli/src/operator/status.rs` — shared status collection
  path.
- `packages/open-bitcoin-cli/src/operator/dashboard/mod.rs` — shared dashboard
  snapshot collection path.
- `docs/parity/benchmarks.md` — contributor-facing benchmark contract.
- `docs/parity/catalog/operator-runtime-release-hardening.md` — parity-facing
  benchmark narrative for the current runtime slice.
- `AGENTS.md` — repo-native verification contract and benchmark/doc guidance.
- `AGENTS.bright-builds.md`, `standards/index.md`,
  `standards/core/code-shape.md`, `standards/core/verification.md`,
  `standards/core/testing.md`, and `standards/languages/rust.md`.

## Repo Guidance That Materially Informs This Phase

- Use `bash scripts/verify.sh` as the repo-native verification contract.
- Keep the benchmark path deterministic, bounded, and offline in the default
  local verify flow.
- Prefer focused Rust tests and repo-owned benchmark commands over ad hoc
  one-off verification.
- Keep benchmark/report metadata truthful after the implementation changes.
- Follow the Rust and code-shape standards by keeping helper logic readable,
  guard-oriented, and inside existing files when a new module is unnecessary.

## Current State

- The `operator-runtime` benchmark group still benchmarks rendering and
  projection against `sample_status_snapshot()` in
  `packages/open-bitcoin-bench/src/cases/operator_runtime.rs`.
- The real CLI runtime already has deterministic collection helpers:
  `collect_status_snapshot()` and `collect_dashboard_snapshot()`.
- The benchmark crate already has deterministic temp-store helpers and can seed
  durable metrics snapshots without public-network access.
- The report validator only requires the existing case IDs, non-empty
  measurement metadata, and valid durability values, so the fixture labels can
  become more truthful without changing the schema.
- `docs/parity/benchmarks.md` and the benchmark registry still describe the
  operator-runtime group as shared-snapshot projection rather than
  runtime-collected evidence.

## Decisions

1. **Keep the Phase 22 case IDs stable.**
   The validator and report consumers already rely on
   `operator-runtime.status-render` and
   `operator-runtime.dashboard-projection`, so Phase 27 should upgrade fidelity
   without renaming the cases.
2. **Exercise the real collector path with deterministic fake adapters.**
   The benchmark should use a tiny fake `StatusRpcClient`, seeded local metrics
   storage, and fake service managers so it hits the shared runtime collection
   helpers without needing a live daemon.
3. **Keep the operator-runtime benchmark offline and tempdir-backed.**
   The new collection path must stay hermetic and repeatable inside both
   benchmark smoke runs and `bash scripts/verify.sh`.
4. **Refresh the benchmark narrative anywhere it still says “shared snapshot”.**
   Registry metadata and contributor-facing benchmark docs need to describe the
   new runtime-collected path accurately after the upgrade.

## Key Files And Likely Change Surfaces

- `packages/open-bitcoin-bench/src/cases/operator_runtime.rs`
- `packages/open-bitcoin-bench/src/runtime_fixtures.rs`
- `packages/open-bitcoin-bench/src/registry.rs`
- `docs/parity/benchmarks.md`
- `docs/parity/catalog/operator-runtime-release-hardening.md`
- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `docs/metrics/lines-of-code.md`

## Risks

- If the benchmark starts depending on real host state or a live daemon, the
  smoke path will stop being deterministic.
- If the fake runtime inputs do not include metrics history, the dashboard case
  could technically collect a real snapshot while still underrepresenting chart
  cost.
- If the metadata or docs keep describing the old fixture path, the benchmark
  upgrade will be technically correct but still audit-misleading.

## Implementation Notes

- Plan 01 should replace the status benchmark fixture with a runtime-collected
  snapshot path and a deterministic fake live RPC client.
- Plan 02 should route the dashboard benchmark through
  `collect_dashboard_snapshot()`, refresh operator-runtime metadata, and update
  the benchmark docs.
- Plan 03 should run the bench-focused verification path, refresh tracked LOC,
  close `VER-06`, and update the roadmap progress for the final phase.
