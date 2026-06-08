---
generated_by: gsd-plan-phase
lifecycle_mode: yolo
phase_lifecycle_id: 65-2026-06-08T14-45-59
generated_at: 2026-06-08T14:58:00.000Z
phase: 65
requirements: [OBS-03, OBS-04]
---

# Phase 65: Support Bundle and Operator Review Docs - Research

## RESEARCH COMPLETE

Phase 65 should be planned as a narrow support evidence and operator workflow
phase. The core runtime surfaces already exist from Phases 60-64; the highest
risk is expanding evidence collection in a way that leaks raw reports, logs,
endpoints, or credentials, or documenting optional public-network/service checks
as if they are part of deterministic verification.

## Phase Scope

### OBS-03: Redacted v1.5 Support Bundle

The existing support bundle writes a local JSON/Markdown pair through
`open-bitcoin support bundle`. It already includes:

- Redaction metadata.
- Config source and path evidence.
- Credential metadata without credential contents.
- The shared `OpenBitcoinStatusSnapshot`.
- Store-health and runtime metadata availability.
- Metrics history status and sample counts.
- Optional live-smoke summary evidence from an allowlisted projection.

The v1.5 implementation should extend this existing contract rather than create
a new support artifact. The shared status snapshot is already documented as the
source of truth for support bundles in `docs/architecture/status-snapshot.md`.

### OBS-04: Operator Review Commands

The runtime guide already has repo-local Cargo/Bazel examples for status,
service lifecycle, live-smoke review, and support bundle collection. Phase 65
should consolidate those into a v1.5 review sequence:

1. Deterministic checks from the repo root.
2. Optional public-network long-run review.
3. Optional service-managed review.
4. Support bundle collection.
5. Field-based pass/fail interpretation.

Docs must keep public-network and real service-manager flows opt-in UAT evidence
and outside `bash scripts/verify.sh`.

## Existing Implementation Surfaces

### Support Bundle

- `packages/open-bitcoin-cli/src/operator/support.rs` owns
  `SupportEvidenceBundle`, `ConfigEvidence`, `StoreHealthEvidence`,
  `MetricsHistoryEvidence`, `LiveSmokeEvidence`, and `redaction_summary()`.
- `packages/open-bitcoin-cli/src/operator/support/live_smoke.rs` already
  allowlists schema v2 live-smoke keys for first header progress, first block
  progress, restart/resume evidence, recovery diagnosis, final status, and
  resource pressure.
- `packages/open-bitcoin-cli/src/operator/support/render.rs` renders the same
  support evidence into compact Markdown.
- `packages/open-bitcoin-cli/tests/operator_binary.rs` has support bundle tests
  for JSON/Markdown generation, redaction of credential-like values, compact
  live-smoke summaries, top-level fallback compatibility, and unavailable
  missing-report behavior.

### Status, Service, Metrics, And Logs

- `packages/open-bitcoin-node/src/status.rs` defines
  `OpenBitcoinStatusSnapshot`, `ServiceStatus`, `ServiceRestartResumeStatus`,
  `SyncStatus`, `SyncResourcePressure`, `LogStatus`, and `MetricsStatus`.
- `packages/open-bitcoin-cli/src/operator/status/service_status.rs` maps durable
  runtime metadata into service restart/resume status with same-datadir,
  prior-shutdown, durable-progress, stale in-flight, recovery category, and
  next-action evidence.
- `packages/open-bitcoin-cli/src/operator/status/render.rs` and
  `packages/open-bitcoin-cli/src/operator/dashboard/model.rs` already render
  Phase 64 restart/resume evidence.
- `docs/architecture/operator-observability.md` defines bounded metrics/log
  retention and warns against raw daemon tails or unbounded support evidence.

### Deterministic Checker Pattern

Existing Bun checkers use `readFileSync`, exact required strings, and
`requireNotContains` checks against `scripts/verify.sh` to prevent live network
and real service-manager commands from entering default verification. Phase 65
should follow that pattern if docs and default-verification boundaries need a
guard.

## Recommended Plan Split

### Plan 65-01: Support Bundle Evidence Shape

Extend the support bundle allowlist and rendering for v1.5 review fields. Keep
this limited to support code and operator-binary tests. This plan should not
edit broad docs except as needed by code comments or test fixtures.

Likely files:

- `packages/open-bitcoin-cli/src/operator/support.rs`
- `packages/open-bitcoin-cli/src/operator/support/live_smoke.rs`
- `packages/open-bitcoin-cli/src/operator/support/render.rs`
- `packages/open-bitcoin-cli/tests/operator_binary.rs`

Verification:

- Focused `cargo test` for support bundle tests.
- Full repo verification later through `bash scripts/verify.sh`.

### Plan 65-02: Operator Review Docs And Deterministic Checker

Refresh `docs/operator/runtime-guide.md`,
`docs/architecture/status-snapshot.md`, and
`docs/architecture/operator-observability.md`, and add
`scripts/check-phase65-support-review.ts` if exact boundary enforcement is useful.

Likely files:

- `docs/operator/runtime-guide.md`
- `docs/architecture/status-snapshot.md`
- `docs/architecture/operator-observability.md`
- `scripts/check-phase65-support-review.ts`
- `scripts/verify.sh`

Verification:

- `bun run scripts/check-phase65-support-review.ts`
- `bash scripts/test-run-live-mainnet-smoke.sh`
- Full repo verification later through `bash scripts/verify.sh`.

### Plan 65-03: Parity Roots And Phase Verification

Update parity/catalog roots only if the support evidence contract or docs add a
new auditable v1.5 evidence path that should be discoverable from parity docs.
Then verify OBS-03 and OBS-04 and write the phase verification artifact.

Likely files:

- `docs/parity/catalog/p2p.md`
- `docs/parity/release-readiness.md` only if wording needs boundary preservation.
- `docs/parity/index.json` only if a new parity root/evidence link is introduced.
- `.planning/phases/65-support-bundle-and-operator-review-docs/65-VERIFICATION.md`

Verification:

- `bash scripts/verify.sh`
- Focused checker and support tests from prior plans.

## Pitfalls

### Pitfall 1: Raw Evidence Expansion

Support bundles must not become a tarball or raw report copier. The existing
allowlisted summary model is the right shape. Any new field must be explicitly
named and tested with forbidden raw markers.

### Pitfall 2: Bundle Existence As Success

A generated bundle means local evidence was collected. It does not prove sync
progress, service restart safety, or production readiness. Docs must point
operators to exact fields and pass/fail interpretations.

### Pitfall 3: Default Verification Drift

`scripts/verify.sh` must not run public-network live smoke, manual peers,
restart-after-progress, `systemctl --user`, `launchctl`, or real service-manager
commands. Use fixture tests and Bun checkers for deterministic proof.

### Pitfall 4: Phase 66/67 Scope Bleed

Do not implement the compatibility harness wrapper in Phase 65. Do not close the
v1.5 release boundary or threat model beyond wording needed to avoid claim drift.

## Validation Architecture

Phase 65 validation should sample the three risks most likely to break OBS-03
and OBS-04:

- **Support evidence sampling:** A fixture support bundle includes v1.5 compact
  live-smoke/restart/status evidence and proves JSON/Markdown include required
  summary fields while excluding forbidden raw material.
- **Docs command sampling:** A deterministic checker asserts the runtime guide
  contains repo-local Cargo and Bazel commands for deterministic checks,
  opt-in long-run review, service review, status review, and support bundle
  collection.
- **Boundary sampling:** The same checker asserts `scripts/verify.sh` includes
  the Phase 65 deterministic checker and excludes public-network and real
  service-manager commands.
