# Quick Task 260425-csn: Refresh README parity status and docs freshness guidance

**Date:** 2026-04-25
**Mode:** quick

## Goal

Update the non-vendored README set so it reflects the current parity-ledger truth, and add a concise repo-local reminder to keep README files fresh after substantial work.

## Task 1: Refresh README Surfaces

**Files**

- `README.md`
- `packages/README.md`
- `docs/parity/README.md`
- `docs/parity/catalog/README.md`

**Action**

Rewrite stale scaffold/progress language around the current source of truth:
`docs/parity/index.json`, `docs/parity/checklist.md`, `docs/parity/release-readiness.md`, and `.planning/STATE.md`. Preserve managed blocks and exclude `packages/bitcoin-knots/**`.

**Verify**

- Stale-text audit for old status claims.
- Manual diff review for managed block preservation and source-of-truth links.

## Task 2: Add README Freshness Guidance

**Files**

- `AGENTS.md`

**Action**

Add one concise bullet under `## Repo-Local Guidance` reminding agents to check README updates after substantial feature, parity, operator-surface, or workflow changes.

**Verify**

- Confirm the bullet is outside the managed Bright Builds block.
- Run `bash scripts/verify.sh`.
