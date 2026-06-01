---
generated_by: gsd-phase-researcher
lifecycle_mode: yolo
phase_lifecycle_id: 53-2026-06-01T02-51-22
generated_at: 2026-06-01T03:14:30Z
status: complete
---

# Phase 53: Live Evidence Refresh - Research

## Research Complete

Phase 53 is an evidence-refresh and closeout phase. The safest implementation
is to reuse the existing opt-in live-mainnet smoke runner, parse the selected
schema v2 report into a committed UAT summary, optionally generate redacted
support evidence for reviewer context, and then update only the parity/audit
surfaces needed to close D-01 and D-03.

## Key Findings

### Live-Smoke Runner

- `scripts/run-live-mainnet-smoke.ts` is the authoritative opt-in live evidence
  entrypoint. It accepts `--datadir`, `--manual-peer`, `--output-dir`,
  `--timeout-seconds`, `--poll-seconds`, and `--min-free-gib`.
- The runner writes schema v2 JSON and Markdown reports with nested
  `result.status`, `result.progressDetected`, `result.maybeNoProgressCause`,
  `result.nextAction`, `result.headerDelta`, and `result.blockDelta`.
- Phase 51 changed per-poll status snapshots to call
  `openbitcoinsyncstatus`, and the report records the command in
  `commands.status`.
- The runner exits non-zero when the result is not `passed`, so evidence runs
  that diagnose a blocker must be handled as expected UAT outcomes, not as
  deterministic verification failures.

### Historical Evidence Debt

- `.planning/v1.3-MILESTONE-AUDIT.md` leaves two Phase 53 items open:
  D-01, the skipped Phase 44 optional public-network contribution UAT, and
  D-03, the historical Phase 50 selected report caveat.
- `.planning/phases/50-public-mainnet-progress-evidence-closeout/50-UAT.md`
  records the historical selected report as `no_progress` with
  `handshake_failure`, 79 endpoint outcomes, and 24 snapshots, but those
  snapshots came from `getblockchaininfo` and showed the stale
  `rpc_getblockchaininfo` phase.
- Phase 51 and Phase 52 already amended Phase 50 UAT for future evidence:
  future smoke reports use fresh daemon sync-control snapshots, and future
  support bundles summarize schema v2 nested `result` fields.

### Support Evidence

- `packages/open-bitcoin-cli/src/operator/support.rs` treats
  `--include-live-smoke-report` as optional local evidence and now prefers
  schema v2 nested `result` summaries before falling back to old top-level
  fields.
- The support bundle remains redacted reviewer context. It should not be used
  as proof by itself and should not include raw live-smoke input, daemon tails,
  raw snapshots, options, or endpoint tables.

### Reviewer-Facing Surfaces

- `docs/parity/release-readiness.md`, `docs/parity/checklist.md`, and
  `docs/parity/index.json` already describe Phase 50 diagnosed-blocker evidence
  plus Phase 51/52 amendments. Phase 53 should update those surfaces with the
  actual new selected outcome rather than adding broad release text.
- `.planning/REQUIREMENTS.md` has a v1.3 tech-debt follow-up table with D-01
  and D-03 pending. Phase 53 should mark those rows complete only after the UAT
  summary and audit updates are written.
- `.planning/ROADMAP.md` should mark Phase 53 complete with one plan only after
  evidence refresh, docs, verification, and phase verification pass.

## Recommended Plan Shape

Use one plan with three implementation tasks:

1. Run bounded opt-in live-smoke attempts into Phase 53 local artifact paths and
   parse the selected report into a committed UAT summary.
2. Generate support evidence for the selected report when available and update
   parity/audit/requirements/roadmap docs according to the actual outcome.
3. Run deterministic verification, review stale debt markers, and write the
   Phase 53 summary and verification artifacts.

## Validation Architecture

### Deterministic Validation

- `bash scripts/test-run-live-mainnet-smoke.sh` proves the runner uses
  `openbitcoinsyncstatus` and schema v2 report fields without public network.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli open_bitcoin_support_bundle --test operator_binary`
  proves schema v2 support summaries and redaction behavior.
- `bash scripts/verify.sh` remains the repo-native deterministic contract.

### UAT Validation

Phase 53 UAT should record the exact live commands and local artifact paths. The
live commands are expected to be opt-in and may exit non-zero for an accepted
diagnosed blocker. The committed UAT must distinguish:

- `progress evidence`: `result.progressDetected=true`, positive header or block
  delta, fresh-status snapshots, contribution rows when present, and support
  summary.
- `fresh diagnosed blocker`: `result.progressDetected=false`, schema v2
  `result.status`, typed `maybeNoProgressCause`, endpoint outcomes, fresh
  `openbitcoinsyncstatus` snapshots or a clear reason they were unavailable,
  and a concrete next action.

### Closeout Checks

Before marking Phase 53 complete, grep for stale unresolved debt language:

```bash
rg -n "D-01|D-03|unresolved stale-artifact|historical Phase 50 artifact caveat|Phase 44 optional public-network UAT remains skipped" \
  .planning/v1.3-MILESTONE-AUDIT.md .planning/REQUIREMENTS.md docs/parity/release-readiness.md docs/parity/checklist.md docs/parity/index.json
```

Any remaining text must describe accepted environmental no-progress or future
operator retry risk, not unresolved Phase 53 debt.

## Risks And Constraints

- Public-network progress is environmental. The implementation must accept a
  fresh diagnosed blocker when it is typed, actionable, and generated after the
  fresh-status fix.
- Do not check generated live-smoke or support-bundle reports into git.
- Do not add public-network work to default verification.
- Do not broaden the v1.3 claim boundary beyond opt-in local evidence.

## Research Complete Marker

## RESEARCH COMPLETE
