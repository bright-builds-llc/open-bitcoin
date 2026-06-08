# Phase 65: Support Bundle and Operator Review Docs - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md; this log preserves the yolo defaults used.

**Date:** 2026-06-08T14:46:10.823Z
**Phase:** 65-Support Bundle and Operator Review Docs
**Mode:** Yolo
**Areas discussed:** Support bundle evidence contract, redaction and boundaries, operator review documentation, deterministic verification

---

## Support Bundle Evidence Contract

| Option | Description | Selected |
|--------|-------------|----------|
| Extend existing allowlisted support bundle | Reuse `SupportEvidenceBundle`, shared status snapshot, store health, metrics availability, and compact live-smoke summary | yes |
| Create a separate v1.5 report artifact | Add a new report shape apart from `support-evidence.json` and `support-evidence.md` | |
| Embed raw local evidence | Copy raw live-smoke reports, daemon tails, endpoint tables, or log files into the bundle | |

**User's choice:** Auto-selected existing allowlisted support bundle.
**Notes:** This matches Phases 59, 61, 62, and 64. The bundle remains local support evidence and does not become a raw report archive or production-node proof.

---

## Redaction And Boundaries

| Option | Description | Selected |
|--------|-------------|----------|
| Preserve strict allowlists | Add only named compact fields and test forbidden raw markers | yes |
| Redact after broad ingestion | Read broad reports/logs and rely on post-hoc scrubbing | |
| Skip new redaction tests | Trust existing redaction coverage without pinning v1.5 fields | |

**User's choice:** Auto-selected strict allowlists with regression tests.
**Notes:** Credential contents, `rpcpassword`, `rpcauth`, private keys, seed phrases, raw local reports, raw endpoint tables, daemon tails, and unbounded logs stay out of support evidence.

---

## Operator Review Documentation

| Option | Description | Selected |
|--------|-------------|----------|
| End-to-end v1.5 review flow | Document deterministic checks, optional long-run review, optional service review, support collection, and pass/fail interpretation with repo-local Cargo and Bazel commands | yes |
| Support command docs only | Update only the support bundle section without tying it to v1.5 review | |
| Installed alias only | Use `open-bitcoin` alias examples without repo-local Cargo/Bazel commands | |

**User's choice:** Auto-selected end-to-end v1.5 review flow.
**Notes:** Docs should emphasize field-based interpretation: progress deltas, final status, stop reason, recovery category, service lifecycle, restart verdicts, and next-action guidance.

---

## Deterministic Verification

| Option | Description | Selected |
|--------|-------------|----------|
| Focused Rust tests plus optional Bun checker | Test support bundle shape/redaction and enforce docs/default-verification boundaries deterministically | yes |
| Public-network verification | Add live mainnet or real service-manager checks to `bash scripts/verify.sh` | |
| Docs-only phase | Avoid code/checker updates even if support evidence fields change | |

**User's choice:** Auto-selected focused deterministic tests/checker.
**Notes:** Default verification must not run `run-live-mainnet-smoke`, `--manual-peer`, `--restart-after-progress`, `systemctl --user`, `launchctl`, or real service-manager operations.

---

## Claude's Discretion

- Planner may split work by support bundle schema/redaction, docs/checker, and focused operator-binary tests.
- Executor may add small pure summary helpers if they keep JSON allowlisting and Markdown rendering aligned.
- Executor must update parity breadcrumbs if new first-party Rust source/test files are added under `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests`.

## Deferred Ideas

- Phase 66 owns the compatibility harness operator wrapper and stable reports.
- Phase 67 owns final v1.5 release-boundary, threat-model, parity-root, and deterministic claim checks.
- Production-node support, inbound serving, relay behavior, production-funds wallet use, migration apply mode, packaging, hosted dashboards, Windows service integration, and GUI work remain future scope.
