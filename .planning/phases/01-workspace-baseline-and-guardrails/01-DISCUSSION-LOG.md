# Phase 1: Workspace, Baseline, and Guardrails - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-04-11
**Phase:** 01-workspace-baseline-and-guardrails
**Mode:** Recommended Review
**Areas discussed:** Reference vendoring, Workspace boundaries, Pure-core enforcement, Verification workflow, Parity and deviation tracking

---

## Reference vendoring

| Option | Description | Selected |
|--------|-------------|----------|
| Git submodule in `packages/bitcoin-knots` | Explicit upstream pin, easy commit-based updates, and clear separation from first-party code | ✓ |
| Pinned source snapshot | Vendor a tarball or copied snapshot directly into the repo | |
| Subtree-style mirror | Keep upstream history inside the repo tree with subtree mechanics | |

**User's choice:** Recommended default applied: git submodule in `packages/bitcoin-knots`
**Notes:** Applied via the recommended-review fallback because interactive selection UI was unavailable in this runtime.

---

## Workspace boundaries

| Option | Description | Selected |
|--------|-------------|----------|
| Bounded multi-crate layout | Split pure-core and adapter responsibilities into a small set of domain crates under `packages/` | ✓ |
| Single monolithic crate | Start simple with one large first-party crate and split later | |
| Fine-grained microcrates | Break nearly every domain concept into its own crate from the start | |

**User's choice:** Recommended default applied: bounded multi-crate layout
**Notes:** Chosen to balance early clarity with manageable crate overhead.

---

## Pure-core enforcement

| Option | Description | Selected |
|--------|-------------|----------|
| Structural crate boundaries plus automated policy checks | Enforce purity through dedicated pure-core crates and CI/pre-commit dependency checks | ✓ |
| Code review conventions only | Rely on contributors and reviewers to catch architectural drift | |
| Runtime-only checks | Detect I/O leakage only through tests or integration behavior | |

**User's choice:** Recommended default applied: structural boundaries plus automated policy checks
**Notes:** Static enforcement is the earliest reliable signal for the architecture this project wants to preserve.

---

## Verification workflow

| Option | Description | Selected |
|--------|-------------|----------|
| Single top-level verification contract | One repo-native command orchestrates format, lint, build, tests, coverage, and policy checks | ✓ |
| Bazel-only verification surface | Require contributors to use only raw Bazel commands | |
| Separate manual commands | Expect contributors to run individual Rust and policy tools themselves | |

**User's choice:** Recommended default applied: single top-level verification contract
**Notes:** This keeps contributor expectations simple while still allowing Bazel and Rust-specific tooling underneath.

---

## Parity and deviation tracking

| Option | Description | Selected |
|--------|-------------|----------|
| Central index plus subsystem notes | Keep one parity/deviation registry with expandable subsystem detail | ✓ |
| Prose-only notes | Track parity state in narrative docs without a central machine-readable index | |
| Subsystem notes only | Let each subsystem keep its own notes with no global parity register | |

**User's choice:** Recommended default applied: central index plus subsystem notes
**Notes:** Later phases need auditable parity status without searching multiple disconnected documents.

---

## the agent's Discretion

- Final crate names and Bazel target names
- Exact implementation mechanism for the dependency-policy checker
- Exact command or wrapper name for the repo verification entrypoint

## Deferred Ideas

- GUI package or app surface
- Public progress dashboard or marketing site
- Rich published benchmark or parity dashboards

---

*Phase: 01-workspace-baseline-and-guardrails*
*Discussion log generated: 2026-04-11*
