---
phase: 116-operator-evidence-metrics-logs-and-support-boundary
plan: 116-04
subsystem: support-checker-docs-closeout
tags:
  - operator-evidence
  - support
  - docs
  - verification
  - block-relay
requires:
  - 116-01
  - 116-02
  - 116-03
provides:
  - Sanitized support-bundle rendering for block-relay evidence.
  - Deterministic Phase 116 checker coverage and verifier wiring.
  - Operator runtime guide and architecture docs for block-relay evidence review.
affects:
  - support-bundles
  - operator-docs
  - verification
  - parity-breadcrumbs
tech-stack:
  added: []
  patterns:
    - Support JSON and Markdown use the same redacted projection instead of separate render-time derivations.
    - Phase-specific Bun checkers guard shared-contract wiring, fixed counters, doc commands, verifier order, and forbidden claims.
key-files:
  created:
    - packages/open-bitcoin-cli/src/operator/support/render/block_relay.rs
    - scripts/check-phase116-operator-block-relay-evidence.ts
    - scripts/check-phase116-operator-block-relay-evidence.test.ts
    - .planning/phases/116-operator-evidence-metrics-logs-and-support-boundary/116-04-SUMMARY.md
  modified:
    - packages/open-bitcoin-cli/src/operator/support/render.rs
    - packages/open-bitcoin-cli/src/operator/support/redaction.rs
    - packages/open-bitcoin-cli/src/operator/support/tests.rs
    - docs/architecture/status-snapshot.md
    - docs/architecture/operator-observability.md
    - docs/operator/runtime-guide.md
    - docs/parity/source-breadcrumbs.json
    - docs/metrics/lines-of-code.md
    - scripts/verify.sh
key-decisions:
  - "Support bundles allowlist shared block-relay fields and redact raw `cmpctblock`, `blocktxn`, hashes, endpoints, permission strings, credentials, and dynamic labels."
  - "The Phase 116 checker validates cross-surface evidence roots, fixed counters, doc command strings, breadcrumb coverage, verifier order, and no-claim guardrails."
  - "Runtime-guide UAT examples use explicit repo-local Cargo and Bazel commands rather than aliases."
requirements-completed:
  - OBS-04
  - OBS-05
generated_by: gsd-executor
lifecycle_mode: yolo
phase_lifecycle_id: 116-2026-07-06T03-46-36
generated_at: 2026-07-06T05:06:00Z
---

# Phase 116 Plan 04: Support Redaction, Checker, Docs, And Closeout Summary

Phase 116-04 closed the operator evidence surface by adding support-bundle sanitization, deterministic regression checks, and operator review guidance for the new block-relay contract.

## Accomplishments

- Added block-relay support rendering and redaction so JSON and Markdown share one sanitized evidence projection.
- Added `scripts/check-phase116-operator-block-relay-evidence.ts` and checker tests covering missing symbols, missing fixed counters, missing redaction needles, verifier ordering, required runtime commands, and forbidden claims.
- Updated architecture and runtime docs with Phase 116 status, metrics, logs, support semantics, and repo-local Cargo/Bazel UAT commands.
- Wired the Phase 116 checker into `scripts/verify.sh` and refreshed the tracked LOC report when verification required it.

## Verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli support -- --nocapture`
- `bun test scripts/check-phase116-operator-block-relay-evidence.test.ts`
- `bun run scripts/check-phase116-operator-block-relay-evidence.ts`
- `bash scripts/verify.sh`

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Support redaction test initially over-asserted on the global redaction summary**
- **Found during:** `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli support -- --nocapture`
- **Issue:** The test flagged intentionally listed redaction categories rather than only checking the sanitized `block_relay` subtree.
- **Fix:** Narrowed assertions to the `block_relay` JSON/Markdown sections while still asserting the global redaction safeguard marker.
- **Files modified:** `packages/open-bitcoin-cli/src/operator/support/tests.rs`

**2. [Rule 1 - Bug] Forbidden-claim matcher initially captured unrelated runtime-guide paragraphs**
- **Found during:** `bun test scripts/check-phase116-operator-block-relay-evidence.test.ts`
- **Issue:** The checker treated generic no-claim paragraphs elsewhere in `docs/operator/runtime-guide.md` as positive Phase 116 claims.
- **Fix:** Scoped the forbidden-claim scan to paragraphs that explicitly mention Phase 116 or block-relay terms.
- **Files modified:** `scripts/check-phase116-operator-block-relay-evidence.ts`

**3. [Rule 3 - Blocking issue] Full verifier required a fresh tracked LOC report**
- **Found during:** `bash scripts/verify.sh`
- **Issue:** `docs/metrics/lines-of-code.md` was stale after the Phase 116 source and script changes.
- **Fix:** Regenerated the tracked LOC report with `bun run scripts/generate-loc-report.ts --source=worktree --output=docs/metrics/lines-of-code.md`.
- **Files modified:** `docs/metrics/lines-of-code.md`

## Self-Check

- Complete: support sanitization, docs, and deterministic checker coverage are present in the working tree.
- Pending final note: the phase-level verification report is recorded in `116-VERIFICATION.md`.
