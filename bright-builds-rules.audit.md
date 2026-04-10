# Bright Builds Rules Audit Trail

<!-- bright-builds-rules-managed-file: bright-builds-rules.audit.md -->

This file records that this repository is using the Bright Builds Rules and shows where the managed adoption files came from.

## Current installation

- Source repository: `https://github.com/bright-builds-llc/bright-builds-rules`
- Version pin: `main`
- Exact commit: `bf5657e7432e21857b9bb7c3755f4bc8e3054890`
- Canonical entrypoint: `https://github.com/bright-builds-llc/bright-builds-rules/blob/main/standards/index.md`
- Managed sidecar path: `AGENTS.bright-builds.md`
- AGENTS integration mode: `append-only managed block`
- Audit manifest path: `bright-builds-rules.audit.md`
- Auto-update: `enabled`
- Auto-update reason: `trusted repo owner bright-builds-llc`
- Last operation: `update`
- Last updated (UTC): `2026-04-10T07:18:56Z`

## Managed files

- `AGENTS.md (managed block)`
- `AGENTS.bright-builds.md`
- `CONTRIBUTING.md`
- `.github/pull_request_template.md`
- `bright-builds-rules.audit.md`
- `README.md (managed badges block)`
- `scripts/bright-builds-auto-update.sh`
- `.github/workflows/bright-builds-auto-update.yml`

## Why this exists

- It provides a visible paper trail for install, update, and uninstall operations.
- The installer manages a bounded block inside `AGENTS.md`, a bounded README badge block when applicable, and marked whole-file managed surfaces such as the sidecar, audit trail, contribution guide, PR template, and optional auto-update files.
- `standards-overrides.md` remains repo-local and is preserved during update and uninstall.
- It helps humans and tools debug which standards revision a repository is pinned to.
