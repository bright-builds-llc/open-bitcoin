# Milestones: Open Bitcoin

## v1.0 Headless Parity (Shipped: 2026-04-26)

**Delivered:** A headless Rust Bitcoin node and wallet baseline with audited behavioral parity against Bitcoin Knots `29.3.knots20260210` for the scoped v1.0 surfaces.

**Phases completed:** 22 phases, 80 plans, 72 counted summary tasks

**Key accomplishments:**

- Pinned the Bitcoin Knots baseline and built first-party Rust workspace, Cargo, and Bazel/Bzlmod guardrails.
- Implemented typed primitives, serialization, consensus validation, chainstate, mempool policy, networking, wallet, RPC, CLI, and config surfaces for the scoped headless milestone.
- Added reusable parity evidence through fixtures, cross-implementation harnesses, property-style protocol coverage, benchmark smoke reports, and parity checklist documentation.
- Preserved functional-core boundaries with architecture-policy checks and kept persistence, network, process, and runtime effects in adapter-owned surfaces.
- Closed consensus parity gaps for contextual headers, lax DER handling, subsidy-plus-fees reward limits, and MoneyRange fee-boundary behavior.
- Hardened reachable panic-like production paths into typed errors and added a guard against new unclassified panic sites.
- Archived the milestone after the v1.0 audit closed GAP-01 through GAP-04 with no open blockers.

**Stats:**

- 28/28 v1 requirements complete.
- 22 phase directories retained in `.planning/phases/` for raw execution history.
- Archive artifacts written under `.planning/milestones/`.

**Archived artifacts:**

- `.planning/milestones/v1.0-ROADMAP.md`
- `.planning/milestones/v1.0-REQUIREMENTS.md`
- `.planning/milestones/v1.0-MILESTONE-AUDIT.md`

**What's next:** Define the next milestone with `/gsd-new-milestone`.

---
