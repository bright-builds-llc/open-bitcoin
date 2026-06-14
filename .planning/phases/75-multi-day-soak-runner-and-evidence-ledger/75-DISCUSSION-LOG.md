# Phase 75: Multi-Day Soak Runner and Evidence Ledger - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md; this log preserves the alternatives considered.

**Date:** 2026-06-14T23:04:34.486Z
**Phase:** 75 - Multi-Day Soak Runner and Evidence Ledger
**Mode:** Yolo
**Areas discussed:** Operator invocation and bounds contract, Durable evidence ledger shape, Stop-reason and resume taxonomy, Deterministic synthetic soak coverage

---

## Operator Invocation And Bounds Contract

| Option | Description | Selected |
| --- | --- | --- |
| New `open-bitcoin soak` operator subcommand | First-class operator UX, typed Clap boundary, run IDs, reports, and Cargo/Bazel commands, but more Rust surface. | |
| Extend `scripts/run-live-mainnet-smoke.ts` | Fastest reuse of existing opt-in runner and reports, but weak durable/resume product boundary. | |
| Add daemon flags/config only | Puts bounds near the sync loop, but gives poor ledger UX and fragments evidence. | |
| Layered combination | `open-bitcoin soak` as entrypoint, daemon/config bounds as runtime truth, live-smoke tooling as compatibility or fixture support. | yes |

**User's choice:** Auto-selected recommended option: layered combination.
**Notes:** Advisor rationale: Phase 75 covers SOAK-01 through SOAK-04 together, so the operator contract needs both stable CLI UX and daemon-owned runtime truth.

---

## Durable Evidence Ledger Shape

| Option | Description | Selected |
| --- | --- | --- |
| Datadir-owned runtime metadata | Strong resume identity, but snapshot-only state can lose timeline history. | |
| Report-directory JSON/Markdown artifacts | Portable operator artifacts, but weaker durable ownership and stale-report risk. | |
| Support-bundle extension | Useful redacted projection, but not a primary interrupted-run ledger. | |
| Hybrid ledger | Datadir run index plus append-only JSONL events, with derived JSON/Markdown reports and support summaries. | yes |

**User's choice:** Auto-selected recommended option: hybrid ledger.
**Notes:** Advisor rationale: this establishes durable identity and interrupted-run continuity without making reports or support bundles the source of truth.

---

## Stop-Reason And Resume Taxonomy

| Option | Description | Selected |
| --- | --- | --- |
| Soak-owned run outcome wrapping existing evidence | Preserves existing sync-cycle and recovery semantics while adding required run-level outcomes. | yes |
| Extend `SyncStopReason` / `SyncRecoveryCategory` directly | One shared vocabulary, but overloads sync-loop semantics with run lifecycle state. | |
| Reuse support verdicts and live-smoke statuses | Minimal taxonomy, but too broad for SOAK-03 stop distinctions. | |
| Event ledger with derived final outcome | Strong future forensics foundation, but risks pre-building Phase 79. | |

**User's choice:** Auto-selected recommended option: soak-owned run outcome wrapping existing evidence.
**Notes:** Advisor rationale: clean completion, diagnosed blocker, operator stop, resource stop, recovery stop, and unexpected termination should carry source evidence from existing status/support/live-smoke fields.

---

## Deterministic Synthetic Soak Coverage

| Option | Description | Selected |
| --- | --- | --- |
| Rust unit/integration tests with scripted clock/transport | Strong canonical behavior proof, but does not cover CLI/report rendering. | |
| Bun checker fixtures around reports | Good for schema/docs/verify guards, but weak runtime proof. | |
| Fake operator command harness | Verifies user-facing command/report flow, but can become process-flaky. | |
| Mixed layers | Rust proves state machine, thin operator harness proves CLI/report flow, Bun checker guards docs/fixtures/default-verification boundaries. | yes |

**User's choice:** Auto-selected recommended option: mixed layers.
**Notes:** Advisor rationale: Rust should prove SOAK-04 control flow with scripted inputs, while operator and Bun checks cover the user-facing and audit surfaces.

---

## the agent's Discretion

- Planning may split implementation into CLI, ledger/domain, report/support projection, deterministic Rust tests, operator docs, and checker/parity closeout.
- Implementation may introduce small pure domain types for soak bounds, run identity, events, and outcomes.

## Deferred Ideas

- Scheduled public-network soak monitors are future SOAK-05 scope.
- Signed externally comparable soak artifacts are future SOAK-06 scope.
- Deeper resource, recovery, progress, and diagnostics work belongs to Phases 76 through 79.
