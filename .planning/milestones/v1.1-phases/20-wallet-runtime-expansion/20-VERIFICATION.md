---
phase: 20-wallet-runtime-expansion
verified: 2026-04-27T21:27:05Z
status: passed
score: 5/5
generated_by: gsd-execute-phase
lifecycle_mode: yolo
phase_lifecycle_id: 20-2026-04-27T09-35-46
generated_at: 2026-04-27T21:27:05Z
lifecycle_validated: true
overrides_applied: 0
---

# Phase 20: Wallet Runtime Expansion Verification Report

**Phase Goal:** Expand wallet behavior into practical runtime workflows that work with durable sync state and operator-facing tools.
**Requirements:** WAL-04, WAL-05, WAL-06, WAL-07, WAL-08
**Verified:** 2026-04-27T21:27:05Z
**Status:** PASS

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
|---|-------|--------|----------|
| 1 | Wallet send now supports a safe `sendtoaddress`-style path with deterministic preview/confirm behavior, fee controls, change handling, and deterministic errors. | VERIFIED | [`packages/open-bitcoin-rpc/src/dispatch/wallet.rs`](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-rpc/src/dispatch/wallet.rs) routes `sendtoaddress` through the shared build/sign path; [`packages/open-bitcoin-cli/src/operator/wallet.rs`](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-cli/src/operator/wallet.rs) enforces preview-before-confirm; [`packages/open-bitcoin-cli/tests/operator_binary.rs`](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-cli/tests/operator_binary.rs) verifies confirm and non-confirm flows. |
| 2 | Wallet-scoped RPC/CLI selection works through `-rpcwallet` and `/wallet/<name>` for the shipped wallet method subset. | VERIFIED | [`packages/open-bitcoin-cli/src/client.rs`](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-cli/src/client.rs) routes wallet methods to wallet-scoped endpoints; [`packages/open-bitcoin-rpc/src/http.rs`](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-rpc/src/http.rs) resolves root-vs-wallet scope; [`packages/open-bitcoin-cli/tests/operator_flows.rs`](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-cli/tests/operator_flows.rs) covers routed wallet flows. |
| 3 | Practical ranged single-key descriptor behavior exists for receive/change allocation and survives the storage/runtime shell. | VERIFIED | [`packages/open-bitcoin-wallet/src/descriptor.rs`](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-wallet/src/descriptor.rs) and children model ranged single-key descriptors; [`packages/open-bitcoin-node/src/wallet_registry.rs`](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/wallet_registry.rs) and [`packages/open-bitcoin-node/src/storage/snapshot_codec/wallet.rs`](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/storage/snapshot_codec/wallet.rs) persist the normalized ranged state. |
| 4 | Wallet rescan, restart recovery, and wallet freshness/balance tracking integrate with durable sync state and survive restart. | VERIFIED | [`packages/open-bitcoin-node/src/sync.rs`](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/sync.rs) plus [`packages/open-bitcoin-node/src/sync/wallet_rescan.rs`](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/sync/wallet_rescan.rs) resume wallet rescans in bounded chunks; [`packages/open-bitcoin-node/src/status.rs`](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/status.rs) and [`packages/open-bitcoin-cli/src/operator/status.rs`](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-cli/src/operator/status.rs) expose fresh/stale/partial/scanning wallet truth. |
| 5 | Existing Core/Knots wallet candidates can be inspected for migration planning without mutation, and managed-wallet backup export stays Open Bitcoin-owned and one-way. | VERIFIED | [`packages/open-bitcoin-cli/src/operator/detect.rs`](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-cli/src/operator/detect.rs) produces format-aware read-only wallet candidate metadata; [`packages/open-bitcoin-cli/src/operator/onboarding.rs`](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-cli/src/operator/onboarding.rs) renders it with read-only cautions; [`packages/open-bitcoin-cli/src/operator/wallet.rs`](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-cli/src/operator/wallet.rs) writes one-way backup export and rejects unsafe destinations. |

**Score:** 5/5 truths verified

## Verification Evidence

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-wallet -p open-bitcoin-rpc -p open-bitcoin-node -p open-bitcoin-cli --all-features` passed.
- `cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-wallet -p open-bitcoin-rpc -p open-bitcoin-node -p open-bitcoin-cli --all-targets --all-features -- -D warnings` passed.
- `cargo build --manifest-path packages/Cargo.toml -p open-bitcoin-wallet -p open-bitcoin-rpc -p open-bitcoin-node -p open-bitcoin-cli --all-features` passed.
- `bun run scripts/check-parity-breadcrumbs.ts --check` passed.
- `bun run scripts/generate-loc-report.ts --source=worktree --output=docs/metrics/lines-of-code.md` refreshed the deterministic LOC artifact successfully.
- `bash scripts/verify.sh` passed end-to-end, including:
  - parity breadcrumb validation
  - pure-core dependency/import checks
  - production Rust file-length gate
  - panic-site validation
  - full Rust workspace test/build coverage
  - bench smoke report generation
  - Bazel smoke build
  - LLVM coverage gate

## Residual Risks

- The Phase 20 wallet surface is intentionally narrower than full Core/Knots multiwallet and migration parity; later phases still need to address richer wallet ergonomics, operator process-control surfaces, and destructive migration flows.
- The final `verify.sh` pass required a behavior-preserving follow-up refactor to satisfy repo-wide verifier gates after the feature work landed, so the closure evidence spans both direct feature commits and final cleanup/refactor commits.
