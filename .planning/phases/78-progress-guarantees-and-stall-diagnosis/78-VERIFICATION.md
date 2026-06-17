---
status: passed
phase: 78-progress-guarantees-and-stall-diagnosis
requirements: [PROG-01, PROG-02, PROG-03, PROG-04]
verified_at: 2026-06-17T11:19:49Z
generated_by: gsd-executor
lifecycle_mode: yolo
phase_lifecycle_id: 78-2026-06-16T14-21-42
generated_at: 2026-06-17T11:19:49Z
lifecycle_validated: true
---

# Phase 78 Verification Report

**Phase Goal:** Ensure long-run progress is credited only for validated,
durably connected active-chain progress or explicit stay-current evidence, and
ensure stalled paths produce compact, typed diagnosis across status, soak,
support, dashboard, logs, live-smoke, docs, and parity roots.

## Evidence Captured

- Shared status contract exposes `progress_credit`, `last_useful_work`,
  `last_peer_contribution`, `expected_progress_window`,
  `no_progress_threshold`, and `stall_diagnosis`.
- Runtime classification rejects header download, block download, peer message,
  in-flight request, retry, and report-generation activity as progress credit
  unless it is backed by validated durable active-chain progress or explicit
  stay-current evidence.
- Stall diagnosis remains typed across public-network reachability,
  incompatible or slow peer paths, validation stalls, storage/resource pressure,
  at-tip waiting, operator stop, and local shutdown.
- Status, dashboard, soak, support-bundle, structured-log, and live-smoke
  projections carry compact Phase 78 evidence without raw support bodies, raw
  snapshots, automatic uploads, service-manager gates, or broad readiness
  claims.
- Parity roots record `phase78-progress-guarantees-stall-diagnosis` across the
  checklist, index, README, P2P catalog, chainstate catalog, and operator
  runtime release-hardening catalog.
- `scripts/check-phase78-progress-guarantees.ts` and its fixture-root tests are
  wired into `scripts/verify.sh` immediately after the Phase 77 checker.
- `docs/metrics/lines-of-code.md` was regenerated from the worktree and records
  132,875 counted lines.

## Command Evidence

| Command | Result |
| --- | --- |
| `cargo fmt --manifest-path packages/Cargo.toml --all` | Passed. |
| `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase78_ --all-features` | Passed: 11 Phase 78 node tests. |
| `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli phase78_ --all-features` | Passed: 7 Phase 78 CLI tests plus filtered targets. |
| `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli support_phase78_progress_guarantee --all-features` | Passed: 3 support-bundle progress guarantee tests. |
| `bash scripts/test-run-live-mainnet-smoke.sh` | Passed with deterministic fixture coverage and no public-network run. |
| `bun test scripts/check-phase78-progress-guarantees.test.ts` | Passed: 8 tests, 0 failures. |
| `bun run scripts/check-phase78-progress-guarantees.ts` | Passed: validated Phase 78 progress guarantees and stall diagnosis boundaries. |
| `bun run scripts/check-parity-breadcrumbs.ts --check` | Passed: parity breadcrumbs verified for 266 Rust files. |
| `bash scripts/verify.sh` | Passed in 12m 13.240s. |

## Acceptance Evidence

- Required operator, architecture, and observability docs mention
  `progress_credit`, `last_useful_work`, `last_peer_contribution`,
  `expected_progress_window`, `no_progress_threshold`, and `stall_diagnosis`.
- Required diagnosis labels mention `validated_durable_active_chain`,
  `current_at_best_known_tip`, `storage_or_resource_pressure`,
  `at_tip_waiting`, `operator_stop`, and `local_shutdown`.
- Negative scans found no `raw support body`, `raw status snapshot`, broad
  readiness claim, or automatic upload language in the Phase 78 documentation
  surfaces.
- `docs/parity/index.json` parses as valid JSON and includes PROG-01 through
  PROG-04 under the Phase 78 surface.
- `scripts/verify.sh` runs the Phase 78 checker after Phase 77 and keeps the
  default verifier deterministic, public-network-free, service-manager-free,
  process-scan-free, and multi-day-wall-clock-free.

## Residual Risks

- Public-network multi-day soak remains opt-in UAT outside default
  verification.
- Real service-manager behavior remains outside deterministic default
  verification.
- Phase 78 records progress guarantees and stall diagnosis only; support-bundle
  forensic timeline expansion remains Phase 79 scope.
