# Project Retrospective

*A living document updated after each milestone. Lessons feed forward into future planning.*

## Milestone: v1.0 - Headless Parity

**Shipped:** 2026-04-26
**Phases:** 22 | **Plans:** 80 | **Counted summary tasks:** 72

### What Was Built

- Headless Rust node and wallet implementation scoped to the v1.0 parity surface.
- First-party primitives, codec, consensus, chainstate, mempool, networking, wallet, RPC, CLI, and config crates under the repo workspace.
- Parity evidence through reference fixtures, cross-implementation harnesses, hermetic integration checks, property-style protocol coverage, benchmark smoke reports, and checklist documentation.
- Guardrails for pure-core architecture boundaries, panic-site classification, parity breadcrumbs, repo-native verification, and CI-facing validation.

### What Worked

- Treating Bitcoin Knots as a pinned behavioral baseline kept implementation and audit discussions concrete.
- The GSD phase and plan structure made late audit gaps traceable enough to close without weakening historical evidence.
- Keeping runtime behavior changes separate from planning artifact cleanup made Phase 12 safer to verify.

### What Was Inefficient

- Roadmap and state metadata drifted behind the actual phase summaries and needed a cleanup phase before archival.
- Some early summaries lacked modern frontmatter fields, which made the milestone audit depend on multiple evidence sources.
- Long extracted accomplishment lists needed manual curation for a useful milestone record.

### Patterns Established

- Preserve historical gap reports when they are superseded, and add an explicit closure trail instead of rewriting them.
- Keep parity claims tied to specific artifacts: requirements rows, verification reports, parity catalog pages, and executable checks.
- Use repo-owned automation for broad sweeps such as parity breadcrumbs and panic-site classification, then keep the rule in `scripts/verify.sh`.

### Key Lessons

1. Milestone archives should be created only after roadmap, requirements, and audit metadata agree with executable evidence.
2. Summary frontmatter is part of the project control plane; missing fields create avoidable audit friction later.
3. Broad parity work needs both source anchors and behavior checks, because breadcrumbs help review but do not prove parity by themselves.

### Cost Observations

- Model mix: not measured in repo artifacts.
- Sessions: multiple GSD planning, execution, UAT, security, audit, and archive turns.
- Notable: late artifact reconciliation was cheaper than changing runtime code, but would have been smaller if roadmap progress updates had stayed current after each inserted phase.

---

## Cross-Milestone Trends

### Process Evolution

| Milestone | Phases | Key Change |
| --- | ---: | --- |
| v1.0 | 22 | Established parity-first implementation, verification, audit, and archive workflow. |

### Cumulative Quality

| Milestone | Requirements | Audit Status | Verification Posture |
| --- | ---: | --- | --- |
| v1.0 | 28/28 complete | Passed with GAP-01 through GAP-04 closed | Repo-native `scripts/verify.sh`, Rust checks, coverage, architecture policy, breadcrumb guard, and panic-site guard. |

### Top Lessons

1. Keep milestone control artifacts as actively verified surfaces, not passive notes.
2. Prefer narrow, auditable parity claims over broad unsupported equivalence statements.
