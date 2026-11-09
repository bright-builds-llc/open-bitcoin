---
name: next-parity-item
description: Select or work the next Open Bitcoin parity ledger item. Use when asked what parity work comes next, to pick up the next item, or to complete it.
---

# Next parity item

Follow the repository's `AGENTS.md` and `docs/parity/README.md` for the current workflow. This skill adds selection and mode routing.

## Choose the mode

- If the user asks what to do next or requests a recommendation, select one item and explain it; stop before edits.
- If the user asks to pick up, continue, or complete parity work, select one item and carry it through the repo's verification, commit, push, and CI checks. A user-named requirement or surface takes precedence over automatic selection. Honor any narrower stop boundary.

## Select one item

1. Check local `main` against `origin/main`. Fast-forward a clean checkout before work; preserve and report any existing changes or divergence.
2. Read `docs/parity/index.json`, `docs/parity/requirements.md`, and the current `.codex/tasks/todo.md` blocks. Resume only a clearly active in-scope task, not a stale historical checkbox; otherwise consider `planned` ledger surfaces. Treat `.planning/` as read-only historical evidence, not the current task queue.
3. Choose a small, unblocked surface with approved requirement wording, pinned Knots source anchors, a testable external behavior, and a defensible evidence class. Check its known gaps and prerequisites. If none is ready, report the blocker and the evidence needed to proceed.
4. State the chosen ID, why it comes next, the expected behavior, evidence class, claim limit, and a short checkable plan. Selection alone does not change ledger status.

## Complete an authorized item

- Keep one append-only task block for multi-session work. Update behavior, focused tests, corpus or oracle evidence, source breadcrumbs, docs, and the ledger as the item requires. Seeded evidence never establishes actual pinned-Knots daemon differential qualification.
- Run focused checks and review the diff. Follow `AGENTS.md` and the parity README for the quick commit gate, full pre-push gate, direct `main` push, and CI follow-through. Create no PR or GSD lifecycle artifacts.
- Report the requirement ID, evidence added, verification results, commit, and remaining claim limits.
