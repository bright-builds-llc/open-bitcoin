---
phase: 46
phase_name: Durable Recovery and Invalid Data Handling
phase_lifecycle_id: 46-2026-05-26T17-16-33
lifecycle_mode: yolo
generated_at: 2026-05-26T17:33:21Z
generated_by: gsd-execute-phase
status: passed
lifecycle_validated: true
requirements:
  - NODE-02
  - NODE-03
  - NODE-05
---

# Phase 46 Verification

## Result

Status: PASSED

## Command Evidence

| Command | Result | Notes |
| --- | --- | --- |
| `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node sync::tests::` | PASS | Covered sync restart, invalid data, peer attribution, and existing sync regressions. |
| `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli operator::status::` | PASS | Covered CLI status rendering and snapshot projections. |
| `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc dispatch::tests::blockchain_info_uses_durable_sync_truth_when_available` | PASS | Covered RPC durable sync truth fixture after additive progress fields. |
| `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node status::tests::populated_snapshot_serializes_obs_01_fields` | PASS | Covered shared status snapshot serialization after additive progress fields. |
| `cargo fmt --manifest-path packages/Cargo.toml --all` | PASS | Required Rust formatting pass. |
| `cargo clippy --manifest-path packages/Cargo.toml --all-targets --all-features -- -D warnings` | PASS | Required Rust lint pass. |
| `cargo build --manifest-path packages/Cargo.toml --all-targets --all-features` | PASS | Required Rust build pass. |
| `cargo test --manifest-path packages/Cargo.toml --all-features` | PASS | Required full Cargo test pass. |
| `bun run scripts/generate-loc-report.ts --source=worktree --output=docs/metrics/lines-of-code.md` | PASS | Regenerated the tracked LOC artifact after `verify.sh` detected staleness. |
| `bash scripts/verify.sh` | PASS | Full repo verification completed in 1m 34.983s after LOC regeneration. |

## Acceptance Criteria

| Criterion | Status | Evidence |
| --- | --- | --- |
| Operator can restart after partial downloads, partial validation, or partial connects and see validated progress resume without duplicated block connects. | PASSED | `restart_reports_downloaded_and_connected_heights_after_partial_download` reconnects saved blocks, keeps header/download/connect progress distinct, and requests only the missing block. |
| Invalid headers or blocks are rejected with peer attribution and do not advance active chainstate. | PASSED | Existing invalid-header tests remain green; `invalid_block_body_is_peer_attributed_and_not_persisted` proves invalid blocks are uncredited, unsaved, and not connected. |
| Recovery guidance distinguishes transient network failures, incompatible stores, corrupt stores, resource exhaustion, and intentional cancellation. | PASSED | Peer failure reasons now derive durable guidance, storage recovery metadata remains priority, resource-limit health signals remain typed, and operator docs document intentional cancellation handling. |
| Durable status after recovery separates validated header, downloaded block, connected block, and error state. | PASSED | `SyncProgress` now exposes header, downloaded block, connected block, and compatibility block heights; durable state persists latest error and recovery action separately. |

## Not Run

- Live mainnet smoke was not run. It is explicitly opt-in and outside the deterministic Phase 46 verification scope.
