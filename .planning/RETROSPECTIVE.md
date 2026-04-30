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

## Milestone: v1.1 - Operator Runtime and Real-Network Sync

**Shipped:** 2026-04-30
**Phases:** 22 | **Plans:** 69 | **Counted summary tasks:** 60

### What Was Built

- Durable Fjall-backed runtime storage, restart recovery, real-network sync foundations, and bounded metrics and structured logs for operator-facing node runtime work.
- The `open-bitcoin` operator binary with rich status output, config-path discovery, idempotent onboarding, service lifecycle commands, and a Ratatui dashboard.
- Practical wallet-runtime workflows for preview and confirm send, managed-wallet backup export, wallet freshness reporting, and scoped RPC wallet selection.
- An evidence-scoped migration and parity surface with read-only Core or Knots detection, dry-run planning, parity-ledger-backed notices, and selected-source service review truth.
- Post-audit cleanup phases that closed operator-surface truth, benchmark fidelity, configless bootstrap, and migration detection ownership debt before archive.

### What Worked

- The shared status, service, metrics, and migration contracts let later phases and cleanup work repair truthfulness without reopening the whole runtime architecture.
- Repo-native verification plus focused operator-binary tests caught regressions early enough that cleanup phases could stay narrow and auditable.
- Preserving each audit rerun as its own artifact made late closeout decisions easy to justify instead of relying on memory or informal notes.

### What Was Inefficient

- Archive tooling still preferred the older baseline audit and live roadmap shape, so milestone closeout required manual curation instead of a clean one-shot archive.
- Generated LOC bookkeeping resurfaced repeatedly whenever closeout or formatting changed the worktree after a prior refresh.
- Several optional cleanup phases were needed because operator-surface truth and evidence bookkeeping drift were discovered only after broader milestone audit passes.

### Patterns Established

- Keep gap-closure work as explicit cleanup phases with their own verification and audit trail rather than burying archive-readiness fixes inside unrelated commits.
- Treat status, dashboard, service, benchmark, and migration flows as one shared operator surface, with truthfulness checked end to end instead of per-command only.
- Use the parity ledger, requirements ledger, and milestone audits together as a control plane for operator-facing claims.

### Key Lessons

1. Archive tooling should prefer the latest passed audit artifact, not just the oldest canonical filename.
2. Generated reports that participate in verification should be refreshed as the last closeout step after formatting and archive edits settle.
3. Operator-surface milestones benefit from explicit post-audit cleanup phases because truthfulness gaps are usually cross-cutting rather than isolated to one feature.

### Cost Observations

- Model mix: not measured in repo artifacts.
- Sessions: multiple GSD execution, audit, cleanup, and archive turns across the v1.1 milestone.
- Notable: cleanup phases were cheaper than reopening the main milestone scope, but earlier end-to-end operator-flow checks would likely have avoided some late archive work.

---

## Cross-Milestone Trends

### Process Evolution

| Milestone | Phases | Key Change |
| --- | ---: | --- |
| v1.0 | 22 | Established parity-first implementation, verification, audit, and archive workflow. |
| v1.1 | 22 | Extended the workflow to terminal-first operator surfaces, milestone rerun audits, and explicit post-audit cleanup phases before archive. |

### Cumulative Quality

| Milestone | Requirements | Audit Status | Verification Posture |
| --- | ---: | --- | --- |
| v1.0 | 28/28 complete | Passed with GAP-01 through GAP-04 closed | Repo-native `scripts/verify.sh`, Rust checks, coverage, architecture policy, breadcrumb guard, and panic-site guard. |
| v1.1 | 44/44 complete | Passed after Phase 33 and Phase 34 cleanup rerun | Repo-native `scripts/verify.sh`, operator-binary coverage, benchmark smoke and report validation, and Bazel smoke builds. |

### Top Lessons

1. Keep milestone control artifacts as actively verified surfaces, not passive notes.
2. Prefer narrow, auditable parity claims over broad unsupported equivalence statements.
3. Close audit debt through explicit cleanup phases when the evidence trail matters as much as the fix itself.
