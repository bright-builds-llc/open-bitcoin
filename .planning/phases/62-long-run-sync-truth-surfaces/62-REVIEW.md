---
status: issues_found
phase: 62-long-run-sync-truth-surfaces
generated_at: 2026-06-06T23:26:22Z
review_depth: standard
files_reviewed: 16
files_reviewed_list:
  - packages/open-bitcoin-node/src/status.rs
  - packages/open-bitcoin-node/src/status/tests.rs
  - docs/parity/source-breadcrumbs.json
  - packages/open-bitcoin-node/src/sync/types/summary.rs
  - packages/open-bitcoin-node/src/sync/runtime_state.rs
  - packages/open-bitcoin-cli/src/operator/status/render.rs
  - packages/open-bitcoin-cli/src/operator/dashboard/model.rs
  - packages/open-bitcoin-cli/src/operator/status/sync_state.rs
  - packages/open-bitcoin-rpc/src/dispatch/node.rs
  - scripts/run-live-mainnet-smoke.ts
  - scripts/test-run-live-mainnet-smoke.sh
  - docs/operator/runtime-guide.md
  - docs/architecture/status-snapshot.md
  - docs/architecture/operator-observability.md
  - scripts/check-phase62-sync-truth-surfaces.ts
  - scripts/verify.sh
findings:
  critical: 0
  warning: 2
  info: 0
  total: 2
---

# Phase 62 Code Review

## Findings

No critical issues found.

### WR-01: Structured sync logs drift from the Phase 62 truth labels

**Severity:** Warning
**File:** `packages/open-bitcoin-node/src/sync/types/summary.rs:259`, `packages/open-bitcoin-node/src/sync/types/summary.rs:274`, `scripts/check-phase62-sync-truth-surfaces.ts:149`

**Issue:** `SyncRunSummary::structured_log_records` emits `stop_reason=` and `signal=`, while the Phase 62 observability docs define the stable machine labels as `latest_stop_reason=` and `progress_signal=`. The checker declares those expected labels, but filters both out before validating `summary.rs`, so `bun run scripts/check-phase62-sync-truth-surfaces.ts` passes while the structured log surface still uses different field names. That breaks the intended same-name truth contract across status, RPC warnings, live-smoke reports, and structured logs.

**Remediation:** Rename the structured log labels to `latest_stop_reason=` and `progress_signal=` in `summary.rs`, update the Rust structured-log tests to assert those exact labels, and remove the checker exclusion so default verification guards the log surface. If legacy labels need temporary compatibility, emit both labels and document the deprecation window.

### WR-02: Live-smoke reports still hide unavailable progress and peer facts behind zeroes

**Severity:** Warning
**File:** `scripts/run-live-mainnet-smoke.ts:57`, `scripts/run-live-mainnet-smoke.ts:269`, `scripts/run-live-mainnet-smoke.ts:1261`, `scripts/run-live-mainnet-smoke.ts:1321`, `scripts/run-live-mainnet-smoke.ts:1470`, `scripts/run-live-mainnet-smoke.ts:1531`, `scripts/run-live-mainnet-smoke.ts:1534`, `scripts/run-live-mainnet-smoke.ts:2549`, `scripts/run-live-mainnet-smoke.ts:2674`

**Issue:** Phase 62 requires missing sync truth data to preserve an explicit unavailable/null reason instead of becoming zeroes. The live-smoke `SyncStatusSnapshot` and `FinalStatusSummary` types model heights, bounded counters, and outbound peer counts as required numbers, then map unavailable `sync_progress` or `peer_counts` through `?? 0`. Markdown renders those synthesized zeroes as real `headerHeight`, `outbound_peers`, and bounded counter values. A stopped or partially available status response can therefore report `0` peers, `0` heights, or `0` counters even when the status source actually said the field was unavailable.

**Remediation:** Carry `maybeSyncProgressUnavailableReason` and `maybePeerCountsUnavailableReason` through the live-smoke JSON summaries, make progress and peer numeric fields nullable when their `FieldAvailability` source is unavailable, and render `Unavailable: {reason}` in the snapshot and final-status Markdown. Add a fixture case where `sync_progress` and `peer_counts` are unavailable so `scripts/test-run-live-mainnet-smoke.sh` fails if zero substitution returns.

## Open Questions / Assumptions

- Reviewed only the Phase 62 files listed in the request; unrelated dirty orchestration files were ignored.
- Treated `.planning/phases/62-long-run-sync-truth-surfaces/62-CONTEXT.md`, the Phase 62 summaries, and the architecture/operator docs as the review contract for truth-field names and unavailable semantics.
- Public-network live mainnet smoke was not run; Phase 62 documents it as opt-in UAT rather than default verification.

## Verification Reviewed

- `bun run scripts/check-phase62-sync-truth-surfaces.ts` - passed, but WR-01 shows it misses two structured-log labels it declares.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase62 --all-features` - passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli sync_status --all-features` - passed.
- `bash scripts/test-run-live-mainnet-smoke.sh` - passed on a clean rerun; the traced rerun also passed while reviewing the quiet initial failure.

## Residual Risks

- Full `bash scripts/verify.sh` was not run manually during this review pass; normal commit hooks may still run broader checks for the review artifact.
- The review did not exercise real public-network timing, peer behavior, or datadir durability; it relied on deterministic unit tests, checker coverage, and the mock live-smoke harness.

---

_Reviewed: 2026-06-06T23:26:22Z_
_Reviewer: the agent (gsd-code-reviewer)_
_Depth: standard_
