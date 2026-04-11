# Open Bitcoin

## What This Is

Open Bitcoin is a headless Bitcoin node and wallet implementation in Rust, built to preserve externally observable behavior from Bitcoin Knots `29.3.knots20260210` across the in-scope consensus, validation, chainstate, mempool, networking, wallet, RPC, CLI, and configuration surfaces. It is for contributors who want a reference-grade node with a cleaner, more type-safe internal architecture, auditable parity, and a strict separation between pure domain logic and effectful adapters.

## Core Value

When a behavior is in scope, Open Bitcoin must behave like the pinned Knots baseline on the outside while staying simpler and safer on the inside.

## Requirements

### Validated

- Phase 2 validated the shared pure-core domain and codec surface for typed
  amounts, hashes, scripts, transactions, blocks, and foundational P2P message
  framing (`ARCH-03`, `CONS-01`).
- Phase 2 seeded the living reference catalog under `docs/parity/catalog/`
  (`REF-03`).

### Active

- [ ] Preserve behavioral parity with the pinned Knots baseline for all in-scope node and wallet surfaces.
- [ ] Keep first-party Rust code modular, strongly typed, and organized around functional core / imperative shell boundaries.
- [ ] Prevent direct filesystem, network, clock, environment, process, thread, async-runtime, and randomness dependencies inside the pure core.
- [ ] Vendor the reference implementation under `packages/` and track intentional deviations explicitly.
- [ ] Export first-party Rust Bitcoin libraries from the workspace instead of depending on existing Rust Bitcoin production libraries.
- [ ] Enforce 100% unit-test coverage for pure-core code and catch I/O leakage automatically.
- [ ] Lock down parity with black-box tests that can run against both Knots and Open Bitcoin.
- [ ] Add fuzz/property testing for parser, serialization, and protocol surfaces where it materially reduces risk.
- [ ] Keep benchmarks, parity checklists, and reference-catalog artifacts that make verification and auditing easier.

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
| Use Bitcoin Knots `29.3.knots20260210` as the reference baseline | The project needs one pinned behavioral contract for parity work and regression detection | — Pending |
| Prioritize behavioral parity over line-by-line source parity | Rust internals should be allowed to become safer and clearer without breaking external behavior | — Pending |
| Use functional core / imperative shell boundaries throughout first-party code | Strong boundaries improve testability, make illegal states unrepresentable, and prevent I/O drift into the pure core | — Pending |
| Use Bazelisk and Bazel/Bzlmod for first-party workspace builds | The repository is expected to become a multi-package workspace with repeatable top-level builds | — Pending |
| Keep the initial milestone headless and defer any GUI to a future milestone | GUI parity would slow core correctness work and should be designed on its own terms later | — Pending |
| Avoid third-party Rust Bitcoin libraries in the production path | The project wants full ownership of domain abstractions, invariants, and behavior | — Pending |

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
*Last updated: 2026-04-11 after Phase 2 execution*
