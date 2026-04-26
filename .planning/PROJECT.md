# Open Bitcoin

## What This Is

Open Bitcoin is a headless Bitcoin node and wallet implementation in Rust, built to preserve externally observable behavior from Bitcoin Knots `29.3.knots20260210` across the in-scope consensus, validation, chainstate, mempool, networking, wallet, RPC, CLI, and configuration surfaces. It is for contributors who want a reference-grade node with a cleaner, more type-safe internal architecture, auditable parity, and a strict separation between pure domain logic and effectful adapters.

## Core Value

When a behavior is in scope, Open Bitcoin must behave like the pinned Knots baseline on the outside while staying simpler and safer on the inside.

## Current State

v1.0 Headless Parity shipped on 2026-04-26. The repository now contains a headless Rust node and wallet implementation with scoped consensus, validation, chainstate, mempool, networking, wallet, RPC, CLI, config, parity-harness, benchmark, audit, and panic-guard surfaces wired and archived for the initial milestone.

The v1.0 archive lives under `.planning/milestones/`, with the detailed shipped roadmap, requirements ledger, and milestone audit preserved as historical artifacts.

## Next Milestone Goals

- Define fresh v1.1 requirements and phase scope with `/gsd-new-milestone` before new implementation work starts.
- Keep the v1.0 parity, verification, and panic-site guardrails active for any post-v1.0 changes.
- Re-evaluate deferred product surfaces such as GUI, public dashboards, and richer observability only after the next milestone's correctness goals are explicit.

## Requirements

### Validated

- v1.0 validated all 28 source-of-truth requirements across reference baseline, architecture, verification, consensus, chainstate, mempool, networking, wallet, RPC, CLI, performance, and auditability surfaces.
- The detailed v1 requirement ledger is archived at `.planning/milestones/v1.0-REQUIREMENTS.md`.
- The v1.0 audit passed with GAP-01 through GAP-04 closed and no open blockers; the audit is archived at `.planning/milestones/v1.0-MILESTONE-AUDIT.md`.

### Active

- [ ] Define the next milestone's requirements, scope, and roadmap before implementation resumes.
- [ ] Preserve v1.0 parity and architecture guardrails while evaluating post-v1.0 changes.
- [ ] Keep deferred surfaces explicitly scoped until the next milestone chooses them.

### Out of Scope

- Qt GUI parity or a faithful port of the upstream GUI code — the initial milestone is headless only.
- Line-by-line C++ parity — behavioral parity is the goal, not source-level mimicry.
- Marketing sites, public progress dashboards, or other product-adjacent packages before they support a clearer milestone — they do not move node correctness forward yet.
- Choosing a future GUI framework now — that decision belongs to the later GUI milestone.

## Context

- The repository now has first-party pure-core domain and codec crates under
  `packages/`, plus seeded parity catalog artifacts under `docs/parity/catalog/`.
- Bitcoin Knots `29.3.knots20260210` is the pinned behavioral reference baseline for the initial milestone.
- First-party code should live in well-bounded packages, with Bazelisk and Bazel/Bzlmod as the top-level build entrypoint unless a later decision replaces that choice.
- The project explicitly avoids existing Rust Bitcoin libraries in the production path and instead exports first-party Rust Bitcoin libraries from this repository.
- Verification must emphasize externally observable parity, pure-core correctness, hermetic integration testing, and contributor guardrails against accidental architectural drift.
- The reference implementation's major subsystems, quirks, known bugs, deviations, and suspected unknowns should remain visible through living project artifacts instead of tribal knowledge.
- The insertion sequence before Phase 8 is now complete: Phase 07.1 split the
  largest hotspots, Phase 07.2 clarified protocol constants, Phase 07.3
  flattened the selected consensus, chainstate, mempool, networking, and
  narrow legacy-script control-flow hotspots without changing behavior, Phase
  07.4 closed the remaining targeted `let ... else` follow-on in consensus
  code, Phase 07.5 restored contextual-header and lax-DER parity, and Phase
  07.6 closed the remaining coinbase subsidy-plus-fees reward-limit parity gap
  on both the pure-core and live chainstate paths.
- Phase 10 is complete: first-party benchmark smoke coverage, benchmark report
  artifacts, parity checklist data, deviations/unknowns notes, and release
  readiness documentation now support auditable parity review.
- Phase 12 is complete: the v1.0 milestone audit artifact gaps are closed with
  explicit Phase 11 verification, Phase 9 requirements reconciliation, roadmap
  completion cleanup, and a preserved Phase 07.5 to Phase 07.6 superseded-gap
  trail.
- v1.0 is archived: `.planning/MILESTONES.md` records the shipped milestone,
  `.planning/ROADMAP.md` is collapsed to milestone-level planning, and fresh
  requirements should be created through the next milestone workflow.

## Constraints

- **Behavioral baseline**: Match Bitcoin Knots `29.3.knots20260210` for all in-scope surfaces — parity claims must be auditable.
- **Architecture**: Follow functional core / imperative shell boundaries — pure business logic stays free of direct I/O and runtime side effects.
- **Dependency policy**: Keep dependencies minimal and security-conscious, and do not use existing Rust Bitcoin libraries in the production path — the project owns its own domain model and implementation surface.
- **Build tooling**: Use Bazelisk and Bazel with Bzlmod for first-party workspace builds — multi-package growth should remain manageable from the repo root.
- **Verification**: Enforce formatting, linting, build, testing, coverage, and architecture-policy checks in pre-commit and CI — regressions should fail early.
- **Scope**: Ship a headless node and wallet first — GUI work is explicitly deferred.

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| Use Bitcoin Knots `29.3.knots20260210` as the reference baseline | The project needs one pinned behavioral contract for parity work and regression detection | Implemented and archived in v1.0 |
| Prioritize behavioral parity over line-by-line source parity | Rust internals should be allowed to become safer and clearer without breaking external behavior | Implemented as the v1.0 parity model |
| Use functional core / imperative shell boundaries throughout first-party code | Strong boundaries improve testability, make illegal states unrepresentable, and prevent I/O drift into the pure core | Enforced by architecture policy and verification |
| Use Bazelisk and Bazel/Bzlmod for first-party workspace builds | The repository is expected to become a multi-package workspace with repeatable top-level builds | Implemented for first-party packages |
| Keep the initial milestone headless and defer any GUI to a future milestone | GUI parity would slow core correctness work and should be designed on its own terms later | Implemented; GUI remains deferred |
| Avoid third-party Rust Bitcoin libraries in the production path | The project wants full ownership of domain abstractions, invariants, and behavior | Implemented for the production path |
| Archive v1.0 before new milestone planning | The next milestone needs a clean requirements and roadmap surface while preserving historical evidence | v1.0 archive created under `.planning/milestones/` |

## Evolution

This document evolves at phase transitions and milestone boundaries.

**After each phase transition** (via `/gsd-transition`):
1. Requirements invalidated? → Move to Out of Scope with reason
2. Requirements validated? → Move to Validated with phase reference
3. New requirements emerged? → Add to Active
4. Decisions to log? → Add to Key Decisions
5. "What This Is" still accurate? → Update if drifted

**After each milestone** (via `/gsd-complete-milestone`):
1. Full review of all sections
2. Core Value check — still the right priority?
3. Audit Out of Scope — reasons still valid?
4. Update Context with current state

---
*Last updated: 2026-04-26 after v1.0 milestone archive*
