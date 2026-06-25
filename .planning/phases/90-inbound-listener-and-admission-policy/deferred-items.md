# Deferred Items

## 2026-06-25 | File-Length Gate Existing Failures

- `bash scripts/check-file-lengths.sh` fails on files outside Plan 90-05 ownership:
  - `packages/open-bitcoin-network/src/peer.rs` at 710 lines.
  - `packages/open-bitcoin-node/src/network.rs` at 701 lines.
  - `packages/open-bitcoin-rpc/src/config/loader.rs` at 633 lines.
- Plan 90-05 kept the new inbound status contract in `status/inbound.rs`; `packages/open-bitcoin-node/src/status.rs` remained below the 628-line production-file limit after the change.
- These failures were not fixed here because they are unrelated to the Plan 90-05 owned source surface and would require broader refactors.

## 2026-06-25 | Plan 90-07 Non-Owned Open Bitcoin CLI Compile Blockers

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli inbound_status -- --nocapture` fails before executing 90-07 tests on files outside Plan 90-07 ownership:
  - `packages/open-bitcoin-cli/src/operator/dashboard/model.rs` has a non-exhaustive `MetricKind` match for Phase 90 inbound metric variants.
  - `packages/open-bitcoin-cli/src/operator/dashboard/model/tests.rs` has a `PeerStatus` fixture missing the new `inbound` field.
  - `packages/open-bitcoin-cli/src/operator/runtime/support.rs` has a `PeerStatus` fixture missing the new `inbound` field.
  - `packages/open-bitcoin-cli/src/operator/soak/tests/runtime.rs` has a `PeerStatus` fixture missing the new `inbound` field.
- Plan 90-07 repaired the owned status collector, status HTTP adapter, status renderer, and owned status test fixtures only. The remaining compile blockers were not fixed here because the user explicitly constrained implementation to the 90-07 status-owned files except necessary renderer metadata.
