---
generated_by: gsd-discuss-phase
lifecycle_mode: interactive
phase_lifecycle_id: 01-2026-04-11T11-36-20
generated_at: 2026-04-11T11:36:20.920Z
---

# Phase 1: Workspace, Baseline, and Guardrails - Context

**Gathered:** 2026-04-11
**Status:** Ready for planning
**Mode:** Recommended Review

<domain>
## Phase Boundary

Phase 1 establishes the pinned Knots baseline, the first-party workspace structure, and the verification and architecture guardrails that every later phase depends on. It does not implement Bitcoin node behavior yet; it creates the constraints, package boundaries, and repo-native workflow surface that later implementation phases must obey.

</domain>

<decisions>
## Implementation Decisions

### Reference vendoring
- **D-01:** Vendor Bitcoin Knots as a pinned git submodule at `packages/bitcoin-knots`.
- **D-02:** Keep the reference baseline read-only from the first-party workspace perspective; compare against it and document intentional deviations instead of blending first-party code into it.

### Workspace boundaries
- **D-03:** Start with a bounded multi-crate layout under `packages/` that separates pure-core Rust libraries from shell and adapter crates.
- **D-04:** Avoid both a single monolithic first-party crate and microcrates for every primitive in Phase 1; split only at clear domain boundaries that later phases will reuse.

### Pure-core enforcement
- **D-05:** Enforce functional core / imperative shell boundaries structurally with dedicated pure-core crates plus automated dependency-policy checks.
- **D-06:** Treat filesystem, sockets, clocks, environment variables, process execution, threads, async runtimes, and randomness as forbidden direct dependencies in pure-core crates.

### Verification workflow
- **D-07:** Expose one top-level repo verification contract for contributors, while delegating first-party package execution to Bazel targets and explicit Rust tooling where needed.
- **D-08:** Make format, lint, build, tests, coverage, and architecture-policy checks all part of the default pre-commit and CI verification path for changed areas.

### Parity and deviation tracking
- **D-09:** Seed a central parity/deviation ledger in Phase 1 with one machine-readable index and human-readable subsystem notes that can expand in later phases.
- **D-10:** Track intentional deviations explicitly from the start instead of relying on undocumented differences or commit history.

### the agent's Discretion
- Exact naming of first-party crates and Bazel targets, as long as the package layout keeps pure-core crates separate from adapters.
- Exact implementation of the architecture-policy checker, as long as it produces hard failures for forbidden pure-core dependencies.
- Exact choice of wrapper script or Bazel target name for the repo-native verification entrypoint.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Project scope and phase contract
- `.planning/PROJECT.md` — Project-wide non-negotiables for behavioral parity, functional core / imperative shell boundaries, dependency policy, and tooling direction.
- `.planning/REQUIREMENTS.md` — Phase 1 requirement IDs `REF-01`, `REF-02`, `ARCH-01`, `ARCH-02`, `ARCH-04`, `VER-01`, and `VER-02`.
- `.planning/ROADMAP.md` — Phase 1 goal, success criteria, and the downstream phases that depend on this foundation.
- `seed-prompts/2026-04-10-seed-prompt-refined-with-codex.md` — Original seed prompt defining the pinned Knots baseline, purity expectations, testing strategy, and Bazel direction.

### Repo policy and contributor workflow
- `AGENTS.md` — Repo-local instructions plus GSD-managed project and workflow guidance.
- `AGENTS.bright-builds.md` — Bright Builds default workflow and verification expectations that still apply to this phase.
- `standards-overrides.md` — Repo-specific standards override record; currently a placeholder, but Phase 1 should leave room for explicit future exceptions.
- `CONTRIBUTING.md` — Contributor workflow surface that Phase 1 verification and policy work should keep aligned with the actual repo contract.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `scripts/bright-builds-auto-update.sh`: Existing shell automation pattern for repo maintenance tasks.
- `.github/pull_request_template.md`: Existing review/documentation integration point for contributor-facing workflow expectations.
- `seed-prompts/2026-04-10-seed-prompt-refined-with-codex.md`: Detailed seed document that already captures the core architectural and verification direction for the repo.

### Established Patterns
- Root-level governance docs are Bright Builds-managed or Bright Builds-aligned and should be extended carefully rather than replaced wholesale.
- `.planning/` artifacts are already tracked in git and treated as durable project state.
- There is no first-party `packages/` tree or production Rust code yet, so Phase 1 defines the initial package and tooling structure from scratch.

### Integration Points
- `packages/` for the vendored Knots baseline and new first-party Rust crates.
- `scripts/` for repo-native verification and maintenance entrypoints.
- `.github/workflows/` for CI checks that mirror the local verification contract.
- `AGENTS.md` and `CONTRIBUTING.md` for contributor workflow documentation that needs to stay in sync with the actual enforcement mechanisms.

</code_context>

<specifics>
## Specific Ideas

- The reference baseline should live under `packages/` as a pinned upstream source, preferably a git submodule unless a stronger repo-local reason emerges during planning.
- The initial implementation remains headless; GUI work is intentionally deferred and should not influence Phase 1 structure decisions.
- Black-box parity tests should eventually run against both Knots and Open Bitcoin, so the early workspace and verification layout should leave room for dual-target test harnesses.
- The repo should make accidental I/O leakage into the pure core a hard failure rather than a convention.

</specifics>

<deferred>
## Deferred Ideas

- A GUI package or app surface for Open Bitcoin — separate future milestone.
- A public progress dashboard or marketing site — deferred until it materially supports a clearer milestone.
- Rich published benchmark or parity dashboards beyond repo-local audit artifacts — defer until the benchmark and audit layers exist.

</deferred>

---

*Phase: 01-workspace-baseline-and-guardrails*
*Context gathered: 2026-04-11*
