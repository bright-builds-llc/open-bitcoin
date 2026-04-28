---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 22-2026-04-28T01-24-15
generated_at: 2026-04-28T01:24:15Z
---

# Phase 22: Real-Sync Benchmarks and Release Hardening - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution.
> Decisions are captured in `22-CONTEXT.md` — this log preserves alternatives considered.

**Date:** 2026-04-28T01:24:15Z
**Phase:** 22-Real-Sync Benchmarks and Release Hardening
**Mode:** Yolo
**Areas discussed:** benchmark entrypoint strategy, default verification scope, operator-doc shape, parity/readiness modeling

---

## Benchmark entrypoint strategy

| Option | Description | Selected |
|--------|-------------|----------|
| Extend `open-bitcoin-bench` and `scripts/run-benchmarks.sh` | Keep one repo-owned benchmark entrypoint and grow it from Phase 10 smoke mapping into Phase 22 runtime evidence. | ✓ |
| Add a second Phase 22 benchmark tool | Create a separate runtime benchmark binary or script just for v1.1 hardening. | |
| Keep benchmark work docs-only | Reinterpret the existing seven groups as sufficient evidence without adding runtime-backed measurements. | |

**User's choice:** Extend the existing benchmark harness.
**Notes:** Phase 22 should deepen the existing audit path instead of splitting benchmark behavior across multiple entrypoints.

---

## Default verification scope

| Option | Description | Selected |
|--------|-------------|----------|
| Deterministic smoke in `verify.sh`, richer runs opt-in | Keep the repo-native gate hermetic and bounded while allowing deeper local benchmark runs outside the default path. | ✓ |
| Run public-network or long-lived sync benchmarks by default | Treat live or slow benchmark runs as part of every local verify. | |
| Add release timing thresholds to the local gate | Fail local verification when elapsed times cross fixed numbers. | |

**User's choice:** Deterministic smoke in `verify.sh`, richer runs opt-in.
**Notes:** This preserves the existing no-public-network contract and keeps release hardening evidence-based rather than timing-threshold-based.

---

## Operator documentation shape

| Option | Description | Selected |
|--------|-------------|----------|
| Add a dedicated operator guide and update README links | Keep README concise while moving install, onboarding, service, status, dashboard, config, migration, testing, and known limitations into one operator-facing doc. | ✓ |
| Expand README only | Put all Phase 22 operator guidance into the root README. | |
| Keep architecture pages as the only documentation source | Rely on contract docs without adding a practical operator-facing guide. | |

**User's choice:** Add a dedicated operator guide and update README links.
**Notes:** The current architecture pages are good contracts, but Phase 22 needs an operator-facing release story and practical workflows.

---

## Parity and release-readiness modeling

| Option | Description | Selected |
|--------|-------------|----------|
| Add explicit v1.1 runtime-hardening surfaces to the parity ledger | Make shipped vs deferred operator/runtime claims first-class in `docs/parity/index.json`, checklist views, and release-readiness docs. | ✓ |
| Leave v1.1 scope implied by prose | Keep deferred or out-of-scope distinctions only in Markdown paragraphs. | |
| Replace the existing parity ledger | Create a second release-readiness registry separate from `docs/parity/index.json`. | |

**User's choice:** Add explicit v1.1 runtime-hardening surfaces to the parity ledger.
**Notes:** Phase 22 should strengthen the machine-readable source of truth instead of introducing a second readiness registry or burying deferrals in prose.

## the agent's Discretion

- Exact benchmark group names may evolve if they remain clear, deterministic, and traceable to the Phase 22 success criteria.
- The operator guide may live under a new `docs/operator/` subtree or another repo-local docs path if the final structure keeps the guide discoverable from `README.md`.
- Report metadata may include benchmark-profile or scenario fields as long as the default smoke report stays simple enough for local review.

## Deferred Ideas

- Public-network benchmark runs in default verification.
- Release timing thresholds that fail local verification.
- Package-manager or signed-release install claims beyond the current source-built operator workflow.
- Mutation-capable migration apply docs before a later phase explicitly implements that surface.
