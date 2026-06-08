---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 64-2026-06-08T03-22-46
generated_at: 2026-06-08T03:22:46.768Z
---

# Phase 64: Service Restart and Same-Datadir Resume Evidence - Discussion Log

**Mode:** Yolo
**Source:** Synthesized from `.planning/ROADMAP.md`, `.planning/REQUIREMENTS.md`,
`.planning/PROJECT.md`, `.planning/STATE.md`, Phase 58 restart/resume evidence,
Phase 61 recovery taxonomy, Phase 62 truth surfaces, and Phase 63 service
lifecycle context.

## Auto-Selected Gray Areas

### Service Restart Evidence Contract

Recommended answer accepted:

- Reuse Phase 58 same-datadir evidence concepts and Phase 63 service lifecycle
  labels.
- Add service-scoped evidence for manager, datadir, prior shutdown, durable
  progress, restart outcome, stale in-flight cleanup, recovery category, and
  next action.
- Keep missing evidence explicit rather than silently successful.

### Same-Datadir Resume Safety

Recommended answer accepted:

- Prove same-datadir behavior through deterministic fixtures tied to durable
  runtime metadata, not only service file text.
- Preserve durable downloaded and connected progress while clearing stale
  in-flight work from the prior daemon session.
- Use storage recovery categories before peer retry guidance.

### Operator Surfaces

Recommended answer accepted:

- Extend service restart/status, status JSON, and dashboard surfaces without
  creating a second status vocabulary.
- Keep real launchd/systemd and public-mainnet checks opt-in UAT only.
- Document repo-local Cargo and Bazel commands for operators.

### Documentation And Verification

Recommended answer accepted:

- Keep Phase 64 framed as service-supervised restart evidence for extended
  operator review.
- Add deterministic checker coverage only for source/docs/default-verification
  boundaries, and refresh generated LOC evidence through the repo-owned
  generator if needed.

## Deferred Ideas

- Phase 65 owns support bundle expansion.
- Phase 66 owns compatibility harness wrapping.
- Phase 67 owns release-boundary closeout.
- Production service, packaging, Windows service, inbound serving, relay,
  production-funds wallet, migration apply, hosted dashboard, and GUI claims
  remain out of scope.
