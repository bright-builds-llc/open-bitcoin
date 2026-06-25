# Deferred Items

## 2026-06-25 | File-Length Gate Existing Failures

- `bash scripts/check-file-lengths.sh` fails on files outside Plan 90-05 ownership:
  - `packages/open-bitcoin-network/src/peer.rs` at 710 lines.
  - `packages/open-bitcoin-node/src/network.rs` at 701 lines.
  - `packages/open-bitcoin-rpc/src/config/loader.rs` at 633 lines.
- Plan 90-05 kept the new inbound status contract in `status/inbound.rs`; `packages/open-bitcoin-node/src/status.rs` remained below the 628-line production-file limit after the change.
- These failures were not fixed here because they are unrelated to the Plan 90-05 owned source surface and would require broader refactors.
