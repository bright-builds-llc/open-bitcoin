---
gsd_state_version: 1.0
milestone: v1.0
milestone_name: milestone
status: executing
stopped_at: Phase 07.6 context gathered
last_updated: "2026-04-22T10:25:45.541Z"
last_activity: 2026-04-21 -- Phase 07.5 execution started
progress:
  total_phases: 20
  completed_phases: 16
  total_plans: 53
  completed_plans: 53
  percent: 100
---

# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-04-11)

**Core value:** When a behavior is in scope, Open Bitcoin must behave like the pinned Knots baseline on the outside while staying simpler and safer on the inside.
**Current focus:** Phase 07.5 — Fix consensus parity gaps in contextual header validation and lax DER signature verification

## Current Position

Phase: 07.5 (Fix consensus parity gaps in contextual header validation and lax DER signature verification) — EXECUTING
Plan: 1 of 4
Status: Executing Phase 07.5
Last activity: 2026-04-21 -- Phase 07.5 execution started

Progress: ██████████ 100%

## Performance Metrics

**Velocity:**

- Total plans completed: 48
- Average duration: 0 min
- Total execution time: 0.0 hours

**By Phase:**

| Phase | Plans | Total | Avg/Plan |
|-------|-------|-------|----------|
| 1 | 4 | - | - |
| 02 | 4 | - | - |
| 03 | 7 | - | - |
| 03.1 | 3 | - | - |
| 03.2 | 3 | - | - |
| 03.3 | 3 | - | - |
| 03.4 | 3 | - | - |
| 04 | 3 | - | - |
| 05 | 3 | - | - |
| 06 | 4 | - | - |
| 07 | 4 | - | - |
| 07.1 | 3 | - | - |
| 07.2 | 1 | - | - |
| 07.3 | 3 | - | - |
| 07.4 | 1 | - | - |

**Recent Trend:**

- Last 5 plans: 07-01, 07-02, 07-03, 07-04, 07.2-01
- Trend: Stable

| Phase 1 P01 | 1 min | 2 tasks | 7 files |
| Phase 1 P02 | 1 min | 2 tasks | 8 files |
| Phase 1 P03 | 1 min | 3 tasks | 6 files |
| Phase 1 P04 | 1 min | 2 tasks | 5 files |
| Phase 07.1 P01 | 9m | 4 tasks | 24 files |
| Phase 07.1 P02 | 6 min | 3 tasks | 4 files |
| Phase 07.1 P03 | 15 min | 3 tasks | 9 files |
| Phase 07.2 P01 | 4 min | 2 tasks | 8 files |
| Phase 07.3 P01 | 8 min | 2 tasks | 4 files |
| Phase 07.3 P02 | 9 min | 2 tasks | 4 files |
| Phase 07.3 P03 | 6m 29s | 2 tasks | 2 files |

## Accumulated Context

### Decisions

Decisions are logged in PROJECT.md Key Decisions table.
Recent decisions affecting current work:

- [Init]: Use Bitcoin Knots `29.3.knots20260210` as the pinned behavioral baseline.
- [Init]: Keep the initial milestone headless and defer GUI work.
- [Init]: Use functional core / imperative shell boundaries with explicit I/O adapters.
- [Init]: Use Bazelisk and Bazel/Bzlmod for first-party workspace builds.
- [Phase 07.1]: Keep touched Rust entry files as module roots and move only inline test bodies into sibling tests.rs files.
- [Phase 07.1]: Treat moved fixture paths and formatter-sensitive leading newlines as task-local blocking issues and fix them inline without widening production visibility.
- [Phase 07.1]: Keep wallet.rs as the module root so the wallet crate's exported surface and navigation entrypoint stay stable while internals move underneath it. — Preserves downstream callers and lib.rs re-exports while making scan, build, and sign seams explicit.
- [Phase 07.1]: Preserve private test reachability with narrow delegate shims instead of widening production visibility. — Keeps the refactor behavior-neutral and compatible with the moved wallet test suite while satisfying clippy and coverage.
- [Phase 07.1]: Keep script.rs as the stable public consensus script entrypoint and route behavior through thin wrappers into sibling modules.
- [Phase 07.1]: Point script/tests.rs at child modules directly instead of preserving test-only helper exposure in the root file.
- [Phase 07.2]: Anchor shared serialized-size constants on the owning wire types instead of spreading duplicated `36`-byte literals across callers.
- [Phase 07.2]: Keep the Taproot `OP_SUCCESS` allowlist behavior-neutral while rewriting it in opcode-domain terms and guarding the boundary values with direct tests.
- [Phase 07.3]: Centralize block transaction validation error mapping in one private helper so both public entrypoints preserve identical txid-based debug text.
- [Phase 07.3]: Keep chainstate connect and disconnect loop order intact while moving only non-coinbase mutation into private helpers.
- [Phase 07.3]: Use local red runs for TDD in this repo when failing-test commits would violate the Rust pre-commit verification contract.
- [Phase 07.3]: Peer message handling should keep the top-level wire-message match visible while sharing only the repeated mutable peer lookup and request cleanup scaffolding.
- [Phase 07.3]: Mempool admission should keep its existing validation and prospective-state order, with replacement selection extracted as a narrow private step before state recomputation.
- [Phase 07.3]: Replacement-policy tests should assert guard-specific rejection reasons so future refactors cannot silently reorder absolute-fee, feerate, fee-bump, or new-unconfirmed-input checks.
- [Phase 07.3]: Keep the legacy script follow-on limited to a shared verify-result helper and guard-style extraction, without rewriting the opcode dispatch or multisig matching loop.
- [Phase 07.3]: Use an empty Task 2 commit when final repo-native verification passes without code changes so the plan still preserves one atomic commit per task on the main tree.

### Roadmap Evolution

- Phase 07.1 inserted after Phase 7: Codebase Maintainability Refactor Wave (URGENT)
- Phase 07.2 inserted after Phase 7: Protocol Constant Clarity Cleanup (URGENT)
- Phase 07.3 inserted after Phase 07.2: Reduce nesting with early returns (URGENT)
- Phase 07.4 inserted after Phase 07.3: Sweep the codebase for let-else opportunities (URGENT)
- Phase 07.5 inserted after Phase 07.4: Fix consensus parity gaps in contextual header validation and lax DER signature verification (URGENT)
- Phase 07.6 inserted after Phase 07.5: Enforce coinbase subsidy-plus-fees limits on the consensus and active chainstate paths (URGENT)

### Pending Todos

- 3 pending:
  - AI-agent-friendly CLI surface — see `.planning/todos/pending/2026-04-18-ai-agent-friendly-cli-surface.md`
  - Sweep panics and illegal states — see `.planning/todos/pending/2026-04-18-sweep-panics-and-illegal-states.md`
  - Reduce nesting with early returns — see `.planning/todos/pending/2026-04-18-reduce-nesting-with-early-returns.md`

### Blockers/Concerns

None yet.

## Session Continuity

Last session: 2026-04-22T10:25:45.538Z
Stopped at: Phase 07.6 context gathered
Resume file: .planning/phases/07.6-enforce-coinbase-subsidy-plus-fees-limits-on-the-consensus-a/07.6-CONTEXT.md
