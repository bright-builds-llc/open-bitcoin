# Phase 21: Drop-In Parity Audit and Migration - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution.
> Decisions are captured in `21-CONTEXT.md` — this log preserves alternatives considered.

**Date:** 2026-04-27T23:11:20.765Z
**Phase:** 21-Drop-In Parity Audit and Migration
**Mode:** Yolo
**Areas discussed:** migration command placement, dry-run scope, source selection, deviation surfacing

---

## Migration command placement

| Option | Description | Selected |
|--------|-------------|----------|
| New `open-bitcoin migrate plan` command | Keep migration planning as its own operator-owned surface with explicit source selection and dry-run output. | ✓ |
| Extend `open-bitcoin onboard` | Fold migration explanations and planning into the first-run config wizard. | |
| Reuse wallet backup only | Keep migration guidance implicit through backup and detection output without a dedicated planner. | |

**User's choice:** New `open-bitcoin migrate plan` command.
**Notes:** Onboarding is about target Open Bitcoin setup. Migration planning needs its own source-install evidence, warnings, and action list.

---

## Dry-run scope

| Option | Description | Selected |
|--------|-------------|----------|
| Dry-run only in Phase 21 | Explain benefits, tradeoffs, rollback, backups, and planned actions without mutating source installs. | ✓ |
| Preview plus partial apply | Allow selected config or service writes now while keeping wallet migration manual. | |
| Full migration apply path | Add mutation-capable switch-over for config, services, and wallets in this phase. | |

**User's choice:** Dry-run only in Phase 21.
**Notes:** This matches the milestone's explicit dry-run-first safety boundary and keeps the phase honest about the current inspection depth.

---

## Source selection semantics

| Option | Description | Selected |
|--------|-------------|----------|
| Explicit source selector plus conservative fallback | Allow `--source-datadir` to pick the source install and surface ambiguity when detection cannot prove one choice. | ✓ |
| Auto-pick first detected install | Choose one candidate automatically whenever any detection exists. | |
| Interactive picker only | Require prompting or menus to choose the source install. | |

**User's choice:** Explicit source selector plus conservative fallback.
**Notes:** Migration planning must not silently guess the source install when multiple or ambiguous candidates exist. Dry-run output should explain how to narrow the target.

---

## Deviation surfacing

| Option | Description | Selected |
|--------|-------------|----------|
| Read structured deviations from `docs/parity/index.json` | Keep one machine-readable source of truth and show only migration-relevant notices in CLI output. | ✓ |
| Parse Markdown parity docs at runtime | Mine the human-readable parity pages for migration warnings. | |
| Hardcode migration differences in Rust | Keep deviation notices separate from the parity ledger. | |

**User's choice:** Read structured deviations from `docs/parity/index.json`.
**Notes:** This keeps MIG-05 auditable and avoids drift between runtime output and the repo's documented parity story.

## the agent's Discretion

- Exact command help text and render-section naming may follow existing operator patterns.
- `onboard` may link to the migration planner later, but Phase 21 keeps the actual planning flow separate.
- A conservative "manual review" or "unsupported in this phase" action is acceptable when the code cannot yet justify a safer automatic step.

## Deferred Ideas

- Apply-mode migration execution.
- Automatic service cutover or data-copy steps.
- External-wallet import, restore, or rewrite flows.
- Broad interactive wizard UX beyond what is needed to explain the dry-run plan.
