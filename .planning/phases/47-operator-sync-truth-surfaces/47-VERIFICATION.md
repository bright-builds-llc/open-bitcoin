---
phase: 47
phase_name: Operator Sync Truth Surfaces
phase_lifecycle_id: 47-2026-05-26T21-36-05
lifecycle_mode: yolo
generated_at: 2026-05-26T21:57:42Z
generated_by: gsd-execute-phase
status: passed
lifecycle_validated: true
requirements:
  - OBS-01
  - OBS-02
---

# Phase 47 Verification

## Result

Status: PASSED

## Command Evidence

| Command | Result | Notes |
| --- | --- | --- |
| `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node sync::tests::` | PASS | Covered sync summary status, metric, structured-log, peer, restart, and recovery projections. |
| `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli operator::dashboard::model::tests::` | PASS | Covered dashboard sync rows and metric chart projection. |
| `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli operator::status::` | PASS | Covered human/JSON status rendering and live RPC fallback status. |
| `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc dispatch::tests::blockchain_info_uses_durable_sync_truth_when_available` | PASS | Covered RPC-facing blockchain info from durable sync truth. |
| `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node status::tests::` | PASS | Covered shared status snapshot serialization for the expanded contract. |
| `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node metrics::tests::metric_kind_names_are_stable` | PASS | Covered stable metric kind names including downloaded and connected block heights. |
| `cargo fmt --manifest-path packages/Cargo.toml --all` | PASS | Required Rust formatting pass. |
| `cargo clippy --manifest-path packages/Cargo.toml --all-targets --all-features -- -D warnings` | PASS | Required Rust lint pass. |
| `cargo build --manifest-path packages/Cargo.toml --all-targets --all-features` | PASS | Required Rust build pass. |
| `cargo test --manifest-path packages/Cargo.toml --all-features` | PASS | Required full Cargo test pass. |
| `bun run scripts/generate-loc-report.ts --source=worktree --output=docs/metrics/lines-of-code.md` | PASS | Regenerated the tracked LOC artifact after source changes. |
| `bash scripts/verify.sh` | PASS | Full repo verification completed in 1m 35.716s. |
| `node ~/.codex/get-shit-done/bin/gsd-tools.cjs verify lifecycle 47 --require-plans --require-verification --raw` | PASS | Confirmed Phase 47 lifecycle artifacts are valid. |
| `node ~/.codex/get-shit-done/bin/gsd-tools.cjs roadmap analyze` | PASS | Confirmed Phase 47 is complete on disk and in the roadmap; next phase is 48. |

## Acceptance Criteria

| Criterion | Status | Evidence |
| --- | --- | --- |
| Operator can inspect JSON status with phase, outbound peer count, peer outcomes, best header height, best block height, progress signal, estimated lag, last successful progress, and last error. | PASSED | `SyncStatus` now carries progress signal and last successful progress; sync summary projection tests and status serialization tests cover the fields. |
| Status, dashboard, metrics, structured logs, and RPC blockchain info agree on current progress and failure state. | PASSED | CLI/dashboard/RPC tests and sync metric/log projection tests cover aligned header, downloaded, connected, signal, error, and recovery values. |
| No operator surface implies full sync until validated chainstate reaches the selected tip. | PASSED | Connected block height remains the `block_height` compatibility value, RPC fallback remains conservative, and downloaded block height is separate from connected chainstate. |
| Status remains useful during active, paused, resumed, stopped, failed, and recovering sync states. | PASSED | Existing sync-control/status tests remain green; new status fields are explicit unavailable values when durable sync truth is missing. |

## Not Run

- Live mainnet smoke was not run. It is explicitly opt-in and outside the deterministic Phase 47 verification scope.
