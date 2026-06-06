# Phase 62: Long-Run Sync Truth Surfaces - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md - this log preserves the alternatives considered.

**Date:** 2026-06-06T19:46:48.293Z
**Phase:** 62-long-run-sync-truth-surfaces
**Mode:** Yolo
**Areas discussed:** Shared truth contract, Bounded metrics and structured logs, Live-smoke snapshot compactness, Verification and documentation

---

## Shared Truth Contract

| Option | Description | Selected |
|--------|-------------|----------|
| Shared status-snapshot vocabulary | Use the shared status snapshot and durable sync state as the canonical source for status, dashboard, RPC, metrics, logs, and live-smoke report fields. | yes |
| Surface-local inference | Let each renderer infer lifecycle, progress, recovery, peer health, and evidence from local strings or reports. | |
| Narrow status-only consistency | Keep only status and dashboard aligned, leaving logs and live-smoke snapshots best-effort. | |

**User's choice:** Shared status-snapshot vocabulary.
**Notes:** Auto-selected because Phase 62's goal is cross-surface agreement and Phase 61 already introduced typed recovery/resource contracts.

---

## Bounded Metrics And Structured Logs

| Option | Description | Selected |
|--------|-------------|----------|
| Compact cycle summaries with existing retention | Add or tighten bounded machine-stable cycle facts for metrics and structured logs while preserving current retention policies. | yes |
| Verbose retained history | Preserve every cycle, peer outcome, snapshot, or log line for later diagnosis. | |
| Docs-only clarification | Document current behavior without adding deterministic checks for metrics/log agreement. | |

**User's choice:** Compact cycle summaries with existing retention.
**Notes:** Auto-selected because OBS-02 requires bounded long-run evidence and the repo already uses retention/allowlist patterns.

---

## Live-Smoke Snapshot Compactness

| Option | Description | Selected |
|--------|-------------|----------|
| Compact final status plus bounded snapshots | Keep opt-in live-smoke JSON/Markdown compact while preserving fields needed to compare progress, retry, stop, and recovery states. | yes |
| Full raw report retention | Embed raw daemon tails, full endpoint tables, and long snapshot history in generated evidence. | |
| No live-smoke changes | Leave live-smoke snapshots outside Phase 62 and focus only on status/RPC surfaces. | |

**User's choice:** Compact final status plus bounded snapshots.
**Notes:** Auto-selected because Phase 62 explicitly includes live-smoke snapshots but prior phases keep raw report material out of default evidence.

---

## Verification And Documentation

| Option | Description | Selected |
|--------|-------------|----------|
| Deterministic cross-surface tests and checker | Use focused Rust tests plus Bun fixture/docs checks where appropriate; keep public-network review opt-in. | yes |
| Public-network long-run verification | Prove the phase by making live mainnet review part of the default verification gate. | |
| Manual review only | Rely on operator docs and human inspection without deterministic regression checks. | |

**User's choice:** Deterministic cross-surface tests and checker.
**Notes:** Auto-selected because repo guidance requires `bash scripts/verify.sh` as the default deterministic gate and public-network checks must remain opt-in UAT.

---

## the agent's Discretion

- The planner may introduce a shared helper or checker data set if it reduces duplicate truth-field lists.
- The executor may keep the implementation in existing modules if no new module boundary is justified.
- The planner may split the work by surface cluster while preserving one shared Phase 62 field contract.

## Deferred Ideas

- Service supervision lifecycle belongs to Phase 63.
- Service-supervised same-datadir restart proof belongs to Phase 64.
- v1.5 support-bundle expansion belongs to Phase 65.
- Compatibility harness operator wrapping belongs to Phase 66.
- v1.5 release-boundary closeout belongs to Phase 67.
