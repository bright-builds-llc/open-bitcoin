---
phase: 37
phase_name: "Header-First Mainnet Sync Integration"
generated_by: gsd-verify-work
lifecycle_mode: yolo
phase_lifecycle_id: "37-2026-05-02T00-08-13"
generated_at: "2026-05-02T01:02:05.851Z"
status: passed
lifecycle_validated: true
---

# Phase 37 Verification

## Result

Passed. Phase 37 now keeps daemon sync on a durable header-first path: it contextually validates inbound headers, continues header batching when peers advertise more work, preserves restart progress from persisted headers, keeps header height distinct from block height, and reports invalid peer data through typed outcomes without re-enabling block download yet.

## Commands

| Command | Result | Notes |
| --- | --- | --- |
| `cargo fmt --manifest-path packages/Cargo.toml --all` | Passed | Final formatting rerun after the `network/header_sync.rs` extraction and test-harness hardening. |
| `cargo clippy --manifest-path packages/Cargo.toml --all-targets --all-features -- -D warnings` | Passed | Caught and cleared one `manual_is_multiple_of` lint while the header-validation helpers were being introduced. |
| `cargo build --manifest-path packages/Cargo.toml --all-targets --all-features` | Passed | Full workspace build stayed green after the sync/runtime and test harness changes. |
| `cargo test --manifest-path packages/Cargo.toml --all-features` | Passed | Full workspace tests and doctests passed after hardening the CLI test server’s accepted socket mode. |
| `bash scripts/verify.sh` | Passed | Final rerun covered breadcrumbs, file-length limits, panic-site policy, benchmark smoke, Bazel smoke, and coverage gates. |
| `git diff --check` | Passed | No whitespace or conflict-marker issues remain. |
| `node "$HOME/.codex/get-shit-done/bin/gsd-tools.cjs" verify lifecycle 37 --require-plans --raw` | Passed | Returned `valid` before final artifact closeout. |

## Evidence

- [`packages/open-bitcoin-node/src/sync.rs`](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/sync.rs) now routes daemon sync traffic through the header-first receive path instead of the generic eager-block path.
- [`packages/open-bitcoin-node/src/network/header_sync.rs`](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/network/header_sync.rs) builds contextual header-validation state from the persisted header tree, including median-time-past and difficulty-recovery inputs.
- [`packages/open-bitcoin-network/src/peer.rs`](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-network/src/peer.rs) supports a header-only sync policy that continues `getheaders` without scheduling block `getdata` requests.
- [`packages/open-bitcoin-network/src/header_store.rs`](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-network/src/header_store.rs) now exposes ancestry and median-time-past helpers that are covered by deterministic unit tests.
- [`packages/open-bitcoin-node/src/sync/tests.rs`](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/sync/tests.rs) proves happy-path continuation, invalid-header failure projection, restart reuse, and competing-branch takeover.
- [`packages/open-bitcoin-cli/src/client/tests.rs`](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-cli/src/client/tests.rs) no longer flakes under `verify.sh` because the accepted test socket is forced back to blocking mode before request reads.

## Residual Risks

- The daemon still does not claim block download/connect, partial-block restart recovery, or operator-surface sync controls; those remain Phase 38-39 work.
- The current header-store chain-work model is sufficient for the covered deterministic tests, but later mainnet evidence may justify more faithful cumulative-work accounting.
