---
gsd_state_version: 1.0
milestone: v1.0
milestone_name: milestone
status: executing
stopped_at: Phase 12 planning complete
last_updated: "2026-04-26T14:54:30.872Z"
last_activity: 2026-04-26 -- Phase 12 planning complete
progress:
  total_phases: 22
  completed_phases: 21
  total_plans: 80
  completed_plans: 76
  percent: 95
---

# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-04-24)

**Core value:** When a behavior is in scope, Open Bitcoin must behave like the pinned Knots baseline on the outside while staying simpler and safer on the inside.
**Current focus:** Phase 12 ready to execute — Milestone Audit Artifact Closure

## Current Position

Phase: 12
Plan: 4 plans ready
Status: Ready to execute
Last activity: 2026-04-26 -- Phase 12 planning complete

Progress: █████████░ 95%

## Performance Metrics

**Velocity:**

- Total plans completed: 64
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
| 07.6 | 3 | - | - |
| 08 | 8 | - | - |
| 10 | 5 | - | - |

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
| Phase 10 P01 | 10min | 2 tasks | 12 files |
| Phase 10 P02 | 19min | 2 tasks | 20 files |
| Phase 10 P03 | 5 min | 2 tasks | 5 files |
| Phase 10 P04 | 537s | 2 tasks | 6 files |
| Phase 10 P05 | 5m 14s | 2 tasks | 5 files |
| Phase 11 P01 | 5 min | 3 tasks | 4 files |
| Phase 11 P02 | 35 min | 4 tasks | 23 files |
| Phase 11 P03 | 6 min | 4 tasks | 5 files |

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
- [Phase 10]: Use a repo-owned stable-Rust benchmark harness with serde JSON/Markdown reports instead of adding Criterion or Divan.
- [Phase 10]: Keep TDD RED runs local-only when failing commits would violate the Rust pre-commit contract.
- [Phase 10]: Treat MODULE.bazel.lock crate-universe refreshes as task-local Bazel metadata for new workspace members.
- [Phase 10]: Benchmark cases compose existing public first-party APIs instead of widening production visibility.
- [Phase 10]: Optional Knots JSON/bin inputs are recorded as report metadata only and are not read during default smoke execution.
- [Phase 10]: The benchmark CLI writes JSON and Markdown reports by default while retaining optional stdout report formatting for compatibility.
- [Phase 10]: Use scripts/run-benchmarks.sh as the contributor-facing benchmark entrypoint and forward only planned options through Bash arrays.
- [Phase 10]: Keep benchmark reports as audit and trend evidence rather than release timing gates.
- [Phase 10]: Make Knots JSON/bin inputs optional metadata enrichment while preserving mapping-only as the default comparison.
- [Phase 10]: Keep docs/parity/index.json as the checklist source of truth and Markdown files as review views.
- [Phase 10]: Keep benchmarks-audit-readiness in_progress until Plan 10-05 release-readiness work promotes it.
- [Phase 10]: Fold CLI-friendly and panic/illegal-state todos into audit risk tracking without broad implementation changes.
- [Phase 10]: Keep release readiness repo-local and deterministic by linking generated benchmark report paths instead of checking timing output into git.
- [Phase 10]: Record stale STATE.md and ROADMAP.md discrepancies in release-readiness audit notes instead of hand-rewriting unrelated planning history during Task 1.
- [Phase 10]: Promote benchmarks-audit-readiness only after regenerating benchmark smoke output and creating the release-readiness handoff.
- [Phase 11]: Treat reachable production panic-like sites as typed-error work instead of allowlisting them; keep `scripts/panic-sites.allowlist` empty unless a future invariant is locally proven.
- [Phase 11]: Guard first-party production Rust under `packages/open-bitcoin-*/src` in `bash scripts/verify.sh`, while excluding vendored Knots, build output, `tests.rs`, and inline `#[cfg(test)]` sections.
- [Phase 11]: Preserve external Bitcoin, RPC, CLI, wallet, mempool, networking, and consensus behavior while replacing internal crashes with crate-local typed errors.

### Roadmap Evolution

- Phase 07.1 inserted after Phase 7: Codebase Maintainability Refactor Wave (URGENT)
- Phase 07.2 inserted after Phase 7: Protocol Constant Clarity Cleanup (URGENT)
- Phase 07.3 inserted after Phase 07.2: Reduce nesting with early returns (URGENT)
- Phase 07.4 inserted after Phase 07.3: Sweep the codebase for let-else opportunities (URGENT)
- Phase 07.5 inserted after Phase 07.4: Fix consensus parity gaps in contextual header validation and lax DER signature verification (URGENT)
- Phase 07.6 inserted after Phase 07.5: Enforce coinbase subsidy-plus-fees limits on the consensus and active chainstate paths (URGENT)
- Phase 11 added: Panic and Illegal-State Hardening

### Pending Todos

- 0 pending.

### Blockers/Concerns

None yet.

### Quick Tasks Completed

| # | Description | Date | Commit | Directory |
|---|-------------|------|--------|-----------|
| 260424-7ow | Improve GitHub CI caching for Rust and Bazel verification | 2026-04-24 | c8a16bd | [260424-7ow-improve-github-ci-caching-for-rust-and-b](./quick/260424-7ow-improve-github-ci-caching-for-rust-and-b/) |
| 260424-jnn | Partial Clippy Panic-Lint Enforcement | 2026-04-24 | a547acd | [260424-jnn-partial-clippy-panic-lint-enforcement](./quick/260424-jnn-partial-clippy-panic-lint-enforcement/) |
| 260425-avx | Add deterministic LOC report generator and wire it into pre-commit/verify | 2026-04-25 | 9b09df8 | [260425-avx-add-deterministic-loc-report-generator-a](./quick/260425-avx-add-deterministic-loc-report-generator-a/) |
| 260425-c8x | Migrate LOC generator to TypeScript and Bun | 2026-04-25 | e8d1055 | [260425-c8x-migrate-loc-generator-to-typescript-and-](./quick/260425-c8x-migrate-loc-generator-to-typescript-and-/) |
| 260425-csn | Refresh README parity status and docs freshness guidance | 2026-04-25 | 37202da | [260425-csn-refresh-readme-parity-status-and-docs-fr](./quick/260425-csn-refresh-readme-parity-status-and-docs-fr/) |
| 260425-e1c | Fix CI Bun provisioning | 2026-04-25 | e51c539 | [260425-e1c-fix-ci-bun-provisioning](./quick/260425-e1c-fix-ci-bun-provisioning/) |
| 260425-kao | Add parity breadcrumb source anchors to first-party Rust files | 2026-04-25 | d2b67e3 | [260425-kao-add-parity-breadcrumb-source-anchors-to-](./quick/260425-kao-add-parity-breadcrumb-source-anchors-to-/) |
| 260425-kzh | Add repo rule requiring parity breadcrumbs for new Rust files | 2026-04-25 | d4e136d | [260425-kzh-add-repo-rule-requiring-parity-breadcrum](./quick/260425-kzh-add-repo-rule-requiring-parity-breadcrum/) |

## Session Continuity

Last session: 2026-04-24T12:34:12.899Z
Stopped at: Completed 10-05-PLAN.md
Resume file: None
