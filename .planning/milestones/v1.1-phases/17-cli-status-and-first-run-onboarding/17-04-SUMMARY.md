---
phase: 17-cli-status-and-first-run-onboarding
plan: "04"
subsystem: cli-operator-status
tags: [cli, status, json, human-output, observability, detection]

requires:
  - phase: 16-metrics-logs-and-sync-telemetry
    provides: "shared metrics, logging, and status snapshot contracts"
  - phase: 17-cli-status-and-first-run-onboarding
    provides: "operator config resolution and read-only detection evidence"
provides:
  - "OpenBitcoinStatusSnapshot-backed status collection"
  - "injected live RPC status adapter contract and fake-adapter tests"
  - "stable serde JSON status rendering and quiet human rendering"
  - "read-only detection evidence mapped into health signals"
affects: [OBS-01, OBS-02, CLI-07, operator, status, dashboard]

tech-stack:
  added:
    - "open-bitcoin-node production dependency for open-bitcoin-cli"
  patterns:
    - "status collection accepts injected RPC adapters instead of opening live network connections in tests"
    - "final JSON rendering serializes OpenBitcoinStatusSnapshot directly"
    - "human rendering is isolated in operator/status/render.rs to keep the collector file below repo size limits"

key-files:
  created:
    - packages/open-bitcoin-cli/src/operator/status/render.rs
  modified:
    - packages/open-bitcoin-cli/Cargo.toml
    - packages/open-bitcoin-cli/BUILD.bazel
    - packages/open-bitcoin-cli/src/operator/status.rs
    - packages/open-bitcoin-cli/src/operator/status/tests.rs
    - packages/open-bitcoin-cli/src/operator/tests.rs
    - docs/parity/source-breadcrumbs.json
    - docs/metrics/lines-of-code.md

key-decisions:
  - "Added open-bitcoin-node as a production CLI dependency only after status needed the shared snapshot contract."
  - "Kept RPC failures as successful unreachable snapshots with unavailable reasons rather than process failures."
  - "Split renderer helpers into a child module after the commit hook caught the file-length limit."

patterns-established:
  - "Status JSON uses serde_json on OpenBitcoinStatusSnapshot, not a CLI-specific DTO."
  - "Core/Knots detection is support evidence in health_signals, not a migration decision."
  - "Missing live fields render as explicit unavailable reasons in both human and JSON output."

requirements-completed: [OBS-01, OBS-02, CLI-07]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 17-2026-04-26T23-56-00
generated_at: 2026-04-27T01:27:04Z

duration: 14min
completed: 2026-04-27
---

# Phase 17 Plan 04 Summary

**Shared status snapshot collection and rendering for stopped, unreachable, and fake live-node evidence**

## Performance

- **Duration:** 14 min
- **Started:** 2026-04-27T01:14:59Z
- **Completed:** 2026-04-27T01:27:04Z
- **Tasks:** 3
- **Files modified:** 8

## Accomplishments

- Added `collect_status_snapshot` over `OpenBitcoinStatusSnapshot` with stopped, unreachable, and injected live RPC paths.
- Added JSON rendering through direct serde serialization of the shared snapshot and a quiet human renderer with the required support labels.
- Mapped read-only Core/Knots detection candidates into `health_signals` with source paths and uncertainty wording.
- Added redaction tests for passwords, auth headers, and cookie contents.

## Task Commits

1. **Plan 17-04: Implement shared status rendering** - `e3818f4` (feat)

## Decisions Made

- Used a `StatusRpcClient` trait so runtime wiring can supply a real adapter later while tests stay hermetic.
- Treated RPC adapter failures as `node.state == unreachable` with unavailable live fields.
- Kept service status read-only: detection can report candidate service files, but install/enable/running remain uninspected.

## Deviations from Plan

### Auto-fixed Issues

**1. File-length limit: split status renderer into child module**
- **Found during:** Commit hook for Plan 17-04
- **Issue:** `packages/open-bitcoin-cli/src/operator/status.rs` exceeded the repo production Rust file limit.
- **Fix:** Moved human/JSON renderer helpers into `packages/open-bitcoin-cli/src/operator/status/render.rs`, registered the new file in `docs/parity/source-breadcrumbs.json`, and regenerated breadcrumbs.
- **Verification:** File length is now 514 lines for `status.rs`, targeted status tests pass, breadcrumb check passes, and the full Rust gate passes.
- **Committed in:** `e3818f4`

**Total deviations:** 1 auto-fixed file-structure issue
**Impact on plan:** No behavior scope change; the split preserves the shared snapshot contract and satisfies repo code-shape policy.

## Issues Encountered

- Initial targeted tests passed with warnings; clippy cleanup removed an unused import and marked the wallet RPC response as intentionally fetched.

## Verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli operator::status::` passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node logging::` passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node metrics::` passed.
- `bun run scripts/check-parity-breadcrumbs.ts --check` passed.
- Rust pre-commit gate passed before commit: fmt, clippy, build, and `cargo test --manifest-path packages/Cargo.toml --all-features`.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

The final runtime plan can now wire the `open-bitcoin status` command to the config resolver, detector, shared snapshot collector, and renderer.

## Self-Check: PASSED

- Status code references `OpenBitcoinStatusSnapshot`.
- No final JSON DTO named `StatusJson`, `StatusDto`, or `CliStatusSnapshot` exists.
- Detection paths and uncertainty appear in both human and JSON status output.

---
*Phase: 17-cli-status-and-first-run-onboarding*
*Completed: 2026-04-27*
