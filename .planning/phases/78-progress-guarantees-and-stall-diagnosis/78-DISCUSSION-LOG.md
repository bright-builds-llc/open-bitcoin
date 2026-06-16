# Phase 78: Progress Guarantees and Stall Diagnosis - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution
> agents. Decisions are captured in CONTEXT.md; this log preserves the
> alternatives considered.

**Date:** 2026-06-16T14:21:42.637Z
**Phase:** 78-progress-guarantees-and-stall-diagnosis
**Mode:** Yolo
**Areas discussed:** Progress credit contract, Stall diagnosis evidence, Soak ledger and operator surfaces, Deterministic verification

---

## Progress Credit Contract

| Option | Description | Selected |
|--------|-------------|----------|
| Validated durable active-chain credit | Credit progress only after active-chain blocks are consensus-validated, connected, and durably persisted; allow explicit stay-current evidence as useful work at tip. | yes |
| Header or peer activity credit | Treat header downloads, peer messages, and queued in-flight work as progress. | |
| Report/checkpoint activity credit | Treat ledger checkpoints or report generation as progress even without validated work. | |

**User's choice:** Auto-selected the conservative validated durable active-chain
credit model.
**Notes:** This matches PROG-01 and carries forward Phase 68 and Phase 70
decisions that better headers or peer activity are evidence, not credited
active-chain progress.

---

## Stall Diagnosis Evidence

| Option | Description | Selected |
|--------|-------------|----------|
| Extend shared typed status contracts | Add or derive expected progress windows, last useful work, last peer contribution, stalled subsystem, thresholds, and evidence basis through shared status and classifiers. | yes |
| Build soak-only stall fields | Keep stall details inside `open-bitcoin soak` reports only. | |
| Renderer-local prose | Let CLI/dashboard/support renderers independently describe stalls from existing fields. | |

**User's choice:** Auto-selected shared typed status contracts.
**Notes:** This preserves Phase 72 cross-surface truth alignment and avoids
diverging CLI, dashboard, RPC, support, soak, and docs behavior.

---

## Soak Ledger And Operator Surfaces

| Option | Description | Selected |
|--------|-------------|----------|
| Ledger/status source of truth with report projection | Carry typed progress and stall evidence through shared status and soak checkpoints; render reports as projections. | yes |
| Report-first evidence | Put richer progress/stall evidence mainly in generated reports. | |
| Support-bundle-first evidence | Defer most evidence until Phase 79 support bundles. | |

**User's choice:** Auto-selected ledger/status source of truth with report
projection.
**Notes:** This follows Phase 75's durable source-of-truth model and leaves
Phase 79 free to narrate typed facts without inventing new facts.

---

## Deterministic Verification

| Option | Description | Selected |
|--------|-------------|----------|
| Rust behavior tests plus focused Bun checker | Use deterministic Rust tests for classifiers/runtime projection and Bun checks for docs/parity/default-verification anchors. | yes |
| Public-network soak proof | Prove the phase primarily with a real public-network long soak. | |
| Checker-only proof | Use a script to assert field names and docs without behavior tests. | |

**User's choice:** Auto-selected Rust behavior tests plus a focused Bun checker.
**Notes:** This satisfies PROG-04 while preserving the repo rule that
`bash scripts/verify.sh` remains public-network-free and short-running.

---

## the agent's Discretion

- Plan boundaries across progress-credit domain/status types, stall classifier
  evidence, soak projection, operator renderers, deterministic fixtures, docs,
  and parity/checker closeout.
- Exact field names and enum-vs-adjacent-evidence tradeoffs are left to the
  planner and executor, with a preference for minimal additive shared contracts.

## Deferred Ideas

- Phase 79 support-bundle forensics and narrative reconstruction.
- Phase 80 opt-in UAT and final release-boundary closeout.
- Any production-node, inbound-serving, relay, production-wallet, migration,
  packaging, GUI, hosted-dashboard, scheduled monitor, or signed soak artifact
  scope.
